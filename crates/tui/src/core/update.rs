//! The one place the state changes.

use crate::core::action::Motion;
use crate::core::state::{Overlay, Toast, View};
use crate::core::{Action, Effect, State, Timestamp};

/// How long a toast stays up.
const TOAST_LIFE: chrono::TimeDelta = chrono::TimeDelta::seconds(5);

/// No I/O, no clock, no randomness: every timestamp arrives with its action.
pub fn update(state: &mut State, at: Timestamp, action: Action) -> Vec<Effect> {
    match action {
        Action::Key(key) => match typed(state, key)
            .or_else(|| crate::core::keys::action_for(state.scope_of_keys(), key))
        {
            Some(action) => {
                if !matches!(action, Action::Quit) {
                    state.quit_armed = false;
                }
                update(state, at, action)
            }
            None => Vec::new(),
        },
        Action::Mouse(mouse) => {
            let mut effects = Vec::new();
            for action in pointed(state, mouse) {
                effects.extend(update(state, at, action));
            }
            effects
        }
        Action::Point { column, row } => {
            point(state, column, row);
            next_page(state)
        }
        Action::OpenAncestors => {
            open_ancestors(state);
            Vec::new()
        }
        Action::OpenWatches => {
            open_watches(state);
            Vec::new()
        }
        Action::Resize { width, height } => {
            state.size = (width, height);
            Vec::new()
        }
        Action::Tick => {
            state.toasts.retain(|toast| at - toast.at < TOAST_LIFE);
            Vec::new()
        }
        Action::Move(motion) => {
            move_selection(state, motion);
            next_page(state)
        }
        Action::Descend => descend(state, at),
        Action::Refresh => refresh(state),
        Action::ToggleRaw => {
            if let View::Detail(view) = state.view_mut() {
                view.raw = !view.raw;
            }
            Vec::new()
        }
        Action::ToggleExpand => {
            match state.view_mut() {
                View::Detail(view) => {
                    view.tree.toggle_selected();
                }
                View::Tree(view) => {
                    view.state.toggle_selected();
                }
                _ => {}
            }
            Vec::new()
        }
        Action::OpenFilter => {
            if matches!(state.view(), View::Collection(_)) {
                state.overlay = Some(Overlay::Filter(crate::core::filter::FilterPrompt::default()));
            }
            Vec::new()
        }
        Action::OpenPalette => {
            open_palette(state);
            Vec::new()
        }
        Action::FocusNext => {
            focus_next(state);
            Vec::new()
        }
        Action::Type(character) => {
            typing(state, Typing::Insert(character));
            Vec::new()
        }
        Action::Backspace => {
            typing(state, Typing::Delete);
            Vec::new()
        }
        Action::Submit => submit(state, at),
        Action::Confirm => {
            let Some(Overlay::Confirm(view)) = state.overlay.as_ref() else {
                return Vec::new();
            };
            if !view.satisfied() {
                let tier = view.tier;
                if let crate::catalog::ConfirmationTier::Phrase(phrase) = tier {
                    state.toasts.push(Toast {
                        at,
                        text: format!("type '{phrase}' to confirm"),
                    });
                }
                return Vec::new();
            }
            let Some(Overlay::Confirm(view)) = state.overlay.take() else {
                return Vec::new();
            };
            mutate(state, view.operation, view.args)
        }
        Action::OpenOperation { operation, args } => open(state, at, operation, args),
        Action::RunOperation { operation, args } => run(state, at, operation, args),
        Action::Edited { purpose, text } => edited(state, at, purpose, text),
        Action::Delivered {
            request: _,
            generation,
            purpose,
            outcome,
        } => delivered(state, at, generation, purpose, outcome),
        Action::Pop => {
            if state.stack.len() > 1 {
                state.stack.pop();
            }
            refetch_stale(state)
        }
        Action::JumpTo { depth } => {
            state.overlay = None;
            if depth < state.stack.len() {
                state.stack.truncate(depth + 1);
            }
            refetch_stale(state)
        }
        Action::Cancel => {
            if let View::Builder(view) = state.view_mut()
                && view.picker.is_some()
            {
                view.picker = None;
                return Vec::new();
            }
            state.overlay = None;
            Vec::new()
        }
        Action::OpenHelp => {
            let scope = state.scope_of_keys();
            state.overlay = Some(Overlay::Help(crate::core::views::help::HelpView::open(
                scope,
            )));
            Vec::new()
        }
        Action::OpenEventLog => {
            state.overlay = None;
            state.stack.push(View::EventLog(
                crate::core::views::event_log::EventLogView::default(),
            ));
            Vec::new()
        }
        Action::Quit => quit(state, at),
        Action::Launched => launch(state),
        Action::AccountsLoaded(outcome) => accounts_loaded(state, at, outcome),
        Action::AccountChosen(account) => {
            state.context.account = Some(account);
            if matches!(state.view(), View::AccountPicker(_)) && state.stack.len() > 1 {
                state.stack.pop();
            }
            replay(state, at)
        }
        Action::SessionStatus(status) => {
            state.session = status;
            match &state.session {
                crate::ports::SessionStatus::Active { .. } => {
                    if matches!(state.view(), View::Session(_)) && state.stack.len() > 1 {
                        state.stack.pop();
                        return replay(state, at);
                    }
                    Vec::new()
                }
                _ => {
                    demand_session(state);
                    Vec::new()
                }
            }
        }
        Action::SessionFailed { message } => {
            demand_session(state);
            if let View::Session(view) = state.view_mut() {
                view.error = Some(message.clone());
            }
            state.toasts.push(Toast { at, text: message });
            Vec::new()
        }
        Action::LoginStarted(authorization) => {
            if let View::Session(view) = state.view_mut() {
                view.authorization = Some(authorization);
            }
            Vec::new()
        }
        Action::LoginFinished(outcome) => match outcome {
            Ok(status) => update(state, at, Action::SessionStatus(status)),
            Err(message) => {
                if let View::Session(view) = state.view_mut() {
                    view.error = Some(message.clone());
                    view.code.reset();
                }
                state.toasts.push(Toast { at, text: message });
                Vec::new()
            }
        },
        Action::OpenRawRequest => {
            state.overlay = None;
            let mut view = crate::core::views::raw_request::RawRequestView {
                method: tui_input::Input::new("GET".to_string()),
                ..Default::default()
            };
            if let Some(selection) = state.selection()
                && let Some(link) = selection.self_link
            {
                view.path = tui_input::Input::new(link);
            }
            view.focus = crate::core::views::raw_request::RawFocus::Path;
            state.stack.push(View::RawRequest(view));
            Vec::new()
        }
        Action::Copy => match copy_text(state) {
            Some(text) => vec![Effect::Copy { text }],
            None => {
                state.toasts.push(Toast {
                    at,
                    text: "there is nothing here to copy".to_string(),
                });
                Vec::new()
            }
        },
        Action::WriteFile => match copy_text(state) {
            Some(text) => vec![Effect::Write {
                path: written_path(state),
                text,
            }],
            None => {
                state.toasts.push(Toast {
                    at,
                    text: "there is nothing here to write".to_string(),
                });
                Vec::new()
            }
        },
        Action::Copied(outcome) => {
            state.toasts.push(Toast {
                at,
                text: match outcome {
                    Ok(crate::ports::CopyRoute::Native) => "copied to the clipboard".to_string(),
                    Ok(crate::ports::CopyRoute::Osc52) => {
                        "copied through the terminal, OSC 52".to_string()
                    }
                    Ok(crate::ports::CopyRoute::TempFile(path)) => {
                        format!("no clipboard; written to {}", path.display())
                    }
                    Err(error) => format!("nothing was copied: {}", error.reason),
                },
            });
            Vec::new()
        }
        Action::Wrote(outcome) => {
            state.toasts.push(Toast {
                at,
                text: match outcome {
                    Ok(path) => format!("written to {}", path.display()),
                    Err(reason) => format!("nothing was written: {reason}"),
                },
            });
            Vec::new()
        }
        Action::WatchPolled { watch, outcome } => watch_polled(state, at, watch, outcome),
        Action::WatchFinished { watch, summary } => {
            let label = watch_label(state, watch);
            state.watches.retain(|held| held.id != watch);
            state.log.append(crate::core::event_log::LogEntry {
                at,
                method: "WATCH".to_string(),
                path: label,
                status: None,
                duration: None,
                attempt: 1,
                waited: None,
                refreshed: false,
                detail: Some(summary.clone()),
                body: None,
            });
            state.toasts.push(Toast { at, text: summary });
            vec![Effect::CancelWatch { watch }]
        }
        Action::CancelWatch(watch) => {
            let label = watch_label(state, watch);
            state.watches.retain(|held| held.id != watch);
            state.toasts.push(Toast {
                at,
                text: format!("stopped watching {label}"),
            });
            vec![Effect::CancelWatch { watch }]
        }
        Action::RequestStarted {
            request,
            attempt,
            method,
            path,
            body,
        } => {
            state.inflight.began(request, &method, &path);
            state.log.append(crate::core::event_log::LogEntry {
                at,
                method,
                path,
                status: None,
                duration: None,
                attempt,
                waited: None,
                refreshed: false,
                detail: None,
                body: state.verbose.then_some(body).flatten(),
            });
            Vec::new()
        }
        Action::RequestWaiting {
            request,
            attempt,
            status,
            wait,
        } => {
            state.inflight.waiting(request);
            let (method, path) = state.inflight.attempted(request);
            state.log.append(crate::core::event_log::LogEntry {
                at,
                method,
                path,
                status: Some(status),
                duration: None,
                attempt,
                waited: Some(wait),
                refreshed: false,
                detail: Some(format!("waiting out {status}")),
                body: None,
            });
            Vec::new()
        }
        Action::RequestRefreshed { request, attempt } => {
            let (method, path) = state.inflight.attempted(request);
            state.log.append(crate::core::event_log::LogEntry {
                at,
                method,
                path,
                status: None,
                duration: None,
                attempt,
                waited: None,
                refreshed: true,
                detail: Some("the session was refreshed".to_string()),
                body: None,
            });
            Vec::new()
        }
        Action::RequestFinished {
            request,
            attempt,
            status,
            duration,
            body,
        } => {
            state.inflight.answered(request);
            let (method, path) = state.inflight.attempted(request);
            state.log.append(crate::core::event_log::LogEntry {
                at,
                method,
                path,
                status: Some(status),
                duration: Some(duration),
                attempt,
                waited: None,
                refreshed: false,
                detail: None,
                body: state.verbose.then_some(body).flatten(),
            });
            Vec::new()
        }
        Action::RequestSettled { request } => {
            state.inflight.settled(request);
            Vec::new()
        }
    }
}

/// Where text is being entered, a printable key is text and not a binding.
fn typed(state: &State, key: ratatui::crossterm::event::KeyEvent) -> Option<Action> {
    use crate::core::keys::Scope;
    use ratatui::crossterm::event::{KeyCode, KeyModifiers};
    let entering = match state.scope_of_keys() {
        Scope::Filter | Scope::Palette | Scope::Builder | Scope::Session => true,
        // The response is read, not written, so the bindings hold there.
        Scope::RawRequest => !matches!(
            state.view(),
            View::RawRequest(crate::core::views::raw_request::RawRequestView {
                focus: crate::core::views::raw_request::RawFocus::Response,
                ..
            })
        ),
        Scope::Confirm => matches!(
            state.overlay,
            Some(Overlay::Confirm(crate::core::views::confirm::ConfirmView {
                tier: crate::catalog::ConfirmationTier::Phrase(_),
                ..
            }))
        ),
        _ => false,
    };
    if !entering || key.modifiers.contains(KeyModifiers::CONTROL) {
        return None;
    }
    match key.code {
        KeyCode::Char(character) => Some(Action::Type(character)),
        KeyCode::Backspace => Some(Action::Backspace),
        _ => None,
    }
}

/// What a printable key does to whichever field has it.
enum Typing {
    Insert(char),
    Delete,
}

/// One keystroke of text, routed to the field the scope names.
fn typing(state: &mut State, typing: Typing) {
    let request = match typing {
        Typing::Insert(character) => tui_input::InputRequest::InsertChar(character),
        Typing::Delete => tui_input::InputRequest::DeletePrevChar,
    };
    match state.overlay.as_mut() {
        Some(Overlay::Filter(prompt)) => {
            prompt.input.handle(request);
            return;
        }
        Some(Overlay::Confirm(view)) => {
            view.typed.handle(request);
            return;
        }
        Some(Overlay::Palette(view)) => {
            view.input.handle(request);
            let candidates = labels(&palette_items(state));
            if let Some(Overlay::Palette(view)) = state.overlay.as_mut() {
                view.rescore(&candidates);
            }
            return;
        }
        _ => {}
    }
    let scope = state.scope();
    if let View::Session(view) = state.view_mut() {
        view.code.handle(request);
        return;
    }
    if let View::RawRequest(view) = state.view_mut() {
        use crate::core::views::raw_request::RawFocus;
        match view.focus {
            RawFocus::Method => {
                view.method.handle(request);
            }
            RawFocus::Path => {
                view.path.handle(request);
            }
            RawFocus::Body => {
                let mut body = tui_input::Input::new(view.body.clone().unwrap_or_default());
                body.handle(request);
                let text = body.value().to_string();
                view.body = (!text.is_empty()).then_some(text);
            }
            RawFocus::Response => {}
        }
        return;
    }
    if let View::Builder(view) = state.view_mut() {
        if let Some(picker) = view.picker.as_mut() {
            picker.input.handle(request);
            let candidates: Vec<String> = view
                .addable()
                .iter()
                .map(|spec| spec.long.clone())
                .collect();
            if let Some(picker) = view.picker.as_mut() {
                picker.rescore(&candidates);
            }
            return;
        }
        let empty = builder_field(view, request);
        if empty && matches!(typing, Typing::Delete) {
            view.remove_selected();
        }
        view.reprice(scope);
    }
}

/// The selected row takes the keystroke, and says whether it was already empty.
fn builder_field(
    view: &mut crate::core::views::builder::BuilderView,
    request: tui_input::InputRequest,
) -> bool {
    use crate::core::views::builder::RowValue;
    let Some(row) = view.rows.get_mut(view.selected) else {
        return false;
    };
    match &mut row.value {
        RowValue::Text(input) | RowValue::Number(input) | RowValue::Date(input) => {
            let was_empty = input.value().is_empty();
            input.handle(request);
            was_empty
        }
        RowValue::Pick {
            picker,
            query,
            chosen,
            ..
        } => {
            let was_empty = query.value().is_empty();
            query.handle(request);
            let text = query.value().to_string();
            *chosen = picker.resolution.accepts(&text).then(|| {
                crate::core::views::builder::PickCandidate {
                    label: text.clone(),
                    value: aweber::catalog::ArgValue::new(row.spec.kind.clone(), text),
                }
            });
            was_empty
        }
        RowValue::Bool(set) => {
            if matches!(request, tui_input::InputRequest::InsertChar(' ')) {
                *set = !*set;
            }
            false
        }
        RowValue::Enum { options, chosen } => {
            if matches!(request, tui_input::InputRequest::InsertChar(' ')) && !options.is_empty() {
                *chosen = (*chosen + 1) % options.len();
            }
            false
        }
        RowValue::Json { text } => text.is_none(),
    }
}

/// `Tab` moves to the next row of whatever is being filled in.
fn focus_next(state: &mut State) {
    use crate::core::views::raw_request::RawFocus;
    match state.view_mut() {
        View::Builder(view) if !view.rows.is_empty() => {
            view.selected = (view.selected + 1) % view.rows.len();
        }
        View::RawRequest(view) => {
            view.focus = match view.focus {
                RawFocus::Method => RawFocus::Path,
                RawFocus::Path => RawFocus::Body,
                RawFocus::Body => RawFocus::Response,
                RawFocus::Response => RawFocus::Method,
            };
        }
        _ => {}
    }
}

/// Launch reads the session and resolves the account in one breath.
fn launch(state: &mut State) -> Vec<Effect> {
    let operation = aweber::catalog::Operation::ListAccounts;
    let scope = state.scope().unwrap_or(aweber::catalog::Scope {
        account_id: 0,
        account: None,
    });
    let mut effects = vec![Effect::ReadSession];
    let generation = state.next_generation();
    if let Ok(plan) =
        aweber::catalog::RequestPlan::build(operation, &scope, &aweber::catalog::Args::default())
    {
        let request = state.next_request();
        effects.push(Effect::Send {
            request,
            generation,
            plan,
            purpose: crate::core::effect::Purpose::Launch,
        });
    }
    effects
}

/// One account is the account; several open a picker.
fn accounts_loaded(
    state: &mut State,
    at: Timestamp,
    outcome: Result<Vec<crate::core::state::Account>, crate::core::failure::Failure>,
) -> Vec<Effect> {
    let accounts = match outcome {
        Ok(accounts) => accounts,
        Err(failure) => {
            if failure.kind == crate::core::failure::FailureKind::Session
                || failure.status == Some(401)
            {
                demand_session(state);
                return Vec::new();
            }
            unavailable(state, failure);
            return Vec::new();
        }
    };
    state.accounts = accounts;
    match state.accounts.len() {
        0 => Vec::new(),
        1 => {
            let account = state.accounts[0].clone();
            update(state, at, Action::AccountChosen(account))
        }
        _ => {
            let known = state.context.account.as_ref().map(|account| account.id);
            match known.and_then(|id| {
                state
                    .accounts
                    .iter()
                    .find(|account| account.id == id)
                    .cloned()
            }) {
                Some(account) => update(state, at, Action::AccountChosen(account)),
                None => {
                    state.stack.push(View::AccountPicker(
                        crate::core::views::account_picker::AccountPickerView::default(),
                    ));
                    Vec::new()
                }
            }
        }
    }
}

/// An account, as the accounts collection describes it.
fn account_of(document: &serde_json::Value) -> Option<crate::core::state::Account> {
    let id = document
        .pointer("/id")
        .and_then(serde_json::Value::as_i64)? as i32;
    Some(crate::core::state::Account {
        id,
        uuid: document
            .pointer("/uuid")
            .and_then(serde_json::Value::as_str)
            .and_then(|uuid| uuid.parse().ok()),
        name: document
            .pointer("/company")
            .or_else(|| document.pointer("/name"))
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string),
    })
}

/// Without a usable session nothing else can happen, so the Session view takes
/// over.
fn demand_session(state: &mut State) {
    let status = state.session.clone();
    if matches!(state.view(), View::Session(_)) {
        if let View::Session(view) = state.view_mut() {
            view.status = status;
        }
        return;
    }
    state.overlay = None;
    state
        .stack
        .push(View::Session(crate::core::views::session::SessionView::of(
            status,
        )));
}

/// The action that wanted a session runs once there is one.
fn replay(state: &mut State, at: Timestamp) -> Vec<Effect> {
    match state.held.take() {
        Some(action) => update(state, at, action),
        None => Vec::new(),
    }
}

/// The action a purpose stands for, to be replayed after a login.
fn held_of(purpose: &crate::core::effect::Purpose) -> Option<Action> {
    use crate::core::effect::Purpose;
    match purpose {
        Purpose::Collection { key } | Purpose::Page { key } => Some(Action::OpenOperation {
            operation: key.operation,
            args: key.args.clone(),
        }),
        Purpose::Document { operation } => Some(Action::OpenOperation {
            operation: *operation,
            args: aweber::catalog::Args::default(),
        }),
        Purpose::Mutation {
            operation, args, ..
        }
        | Purpose::Precondition { operation, args } => Some(Action::RunOperation {
            operation: *operation,
            args: args.clone(),
        }),
        Purpose::Launch => Some(Action::Launched),
        Purpose::Enrichment { .. } | Purpose::ListPicker { .. } | Purpose::Raw => None,
    }
}

/// A watch that is still running, named as the status bar names it.
fn watch_label(state: &State, watch: crate::core::WatchId) -> String {
    state
        .watches
        .iter()
        .find(|held| held.id == watch)
        .map(|held| held.label.clone())
        .unwrap_or_else(|| "the watch".to_string())
}

/// `broadcasts wait` polls until the broadcast is no longer on its way.
fn start_watch(
    state: &mut State,
    at: Timestamp,
    operation: aweber::catalog::Operation,
    args: aweber::catalog::Args,
) -> Vec<Effect> {
    /// Often enough to feel live, seldom enough to stay inside the throttle.
    const INTERVAL: std::time::Duration = std::time::Duration::from_secs(15);
    let Some(scope) = state.scope() else {
        unavailable(state, no_account());
        return Vec::new();
    };
    let plan = match aweber::catalog::RequestPlan::build(operation, &scope, &args) {
        Ok(plan) => plan,
        Err(error) => {
            state.toasts.push(Toast {
                at,
                text: error.to_string(),
            });
            return Vec::new();
        }
    };
    let label = plan.line();
    let watch = state.next_watch();
    state.watches.push(crate::core::state::Watch {
        id: watch,
        label: label.clone(),
        started_at: at,
        operation,
        args,
    });
    state.toasts.push(Toast {
        at,
        text: format!("watching {label}"),
    });
    vec![Effect::StartWatch {
        watch,
        plan,
        interval: INTERVAL,
    }]
}

/// A poll of a watch: still on its way, done, or failed.
fn watch_polled(
    state: &mut State,
    at: Timestamp,
    watch: crate::core::WatchId,
    outcome: Result<crate::core::collection::Delivered, crate::core::failure::Failure>,
) -> Vec<Effect> {
    let label = watch_label(state, watch);
    match outcome {
        Ok(delivered) => {
            let status = delivered
                .document
                .as_ref()
                .and_then(|document| document.pointer("/status"))
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown")
                .to_string();
            if underway(&status) {
                return Vec::new();
            }
            update(
                state,
                at,
                Action::WatchFinished {
                    watch,
                    summary: format!("{label} is {status}"),
                },
            )
        }
        Err(failure) => update(
            state,
            at,
            Action::WatchFinished {
                watch,
                summary: format!("{label} stopped: {}", failure.text()),
            },
        ),
    }
}

/// The pasteable line for an operation: `aweber <group> <action> …` where the
/// binary routes it, and the `aweber api` request it plans where the TUI alone
/// offers it.
fn command_line_of(
    state: &State,
    operation: aweber::catalog::Operation,
    args: &aweber::catalog::Args,
) -> String {
    if !operation.is_cli()
        && let Some(scope) = state.scope()
        && let Ok(plan) = aweber::catalog::RequestPlan::build(operation, &scope, args)
    {
        return aweber::catalog::plan_command_line(&plan);
    }
    aweber::catalog::command_line(operation, args)
}

/// The statuses a broadcast passes through before it has been sent.
fn underway(status: &str) -> bool {
    matches!(status, "draft" | "scheduled" | "queued" | "sending")
}

/// What `y` copies and `w` writes: the equivalent command line, and the
/// document it would return.
fn copy_text(state: &State) -> Option<String> {
    let text = match state.view() {
        View::Collection(view) => {
            let documents: Vec<serde_json::Value> = view
                .visible()
                .into_iter()
                .map(|row| row.document.clone())
                .collect();
            format!(
                "{}\n{}",
                command_line_of(state, view.operation, &view.args),
                pretty(&serde_json::Value::Array(documents))
            )
        }
        View::Detail(view) => {
            let line = command_line_of(state, view.operation, &view.args);
            match &view.document {
                Some(document) => format!("{line}\n{}", pretty(document)),
                None => line,
            }
        }
        View::Tree(view) => {
            let mut args = aweber::catalog::Args::default();
            args.set(
                "workflow",
                aweber::catalog::ArgValue::new(
                    aweber::catalog::ValueKind::Uuid,
                    view.workflow.to_string(),
                ),
            );
            command_line_of(state, aweber::catalog::Operation::TreeWorkflow, &args)
        }
        View::Builder(view) => command_line_of(state, view.operation, &view.args()),
        View::EventLog(_) => state.log.text(),
        View::RawRequest(view) => {
            let line = aweber::catalog::raw_command_line(
                &method_of(view.method.value()),
                view.path.value(),
                body_of(view.body.as_deref()).as_ref(),
            );
            match view
                .response
                .as_ref()
                .and_then(|response| response.document.as_ref().map(pretty))
            {
                Some(document) => format!("{line}\n{document}"),
                None => line,
            }
        }
        View::Home(_) | View::AccountPicker(_) | View::Session(_) => return None,
    };
    Some(crate::core::redact::scrub(&text))
}

/// A written file is named after the view that asked for it.
fn written_path(state: &State) -> std::path::PathBuf {
    let slug: String = state
        .view()
        .title()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            }
        })
        .collect();
    let extension = if matches!(state.view(), View::EventLog(_)) {
        "log"
    } else {
        "json"
    };
    std::env::temp_dir().join(format!("aweber-{slug}.{extension}"))
}

fn method_of(text: &str) -> reqwest::Method {
    text.trim()
        .to_ascii_uppercase()
        .parse()
        .unwrap_or(reqwest::Method::GET)
}

fn body_of(text: Option<&str>) -> Option<aweber::catalog::PlanBody> {
    serde_json::from_str(text?.trim())
        .ok()
        .map(aweber::catalog::PlanBody::Json)
}

/// What a mouse event means, as actions the keyboard can also raise: a wheel is
/// three moves, the header opens the breadcrumb palette, a watch's own cell
/// cancels that watch, and a click in the rows names the cell under it.
fn pointed(state: &State, mouse: ratatui::crossterm::event::MouseEvent) -> Vec<Action> {
    use ratatui::crossterm::event::{MouseButton, MouseEventKind};
    /// The lines one notch of the wheel moves.
    const NOTCH: usize = 3;
    let position = ratatui::layout::Position {
        x: mouse.column,
        y: mouse.row,
    };
    match mouse.kind {
        MouseEventKind::ScrollDown => vec![Action::Move(Motion::Down); NOTCH],
        MouseEventKind::ScrollUp => vec![Action::Move(Motion::Up); NOTCH],
        MouseEventKind::Down(MouseButton::Left) => {
            if state.regions.header.contains(position) {
                return vec![Action::OpenAncestors];
            }
            if let Some((watch, _)) = state
                .regions
                .watches
                .iter()
                .find(|(_, area)| area.contains(position))
            {
                return vec![Action::CancelWatch(*watch)];
            }
            if state.regions.status.contains(position) {
                return vec![Action::OpenWatches];
            }
            if state.regions.rows.contains(position) {
                return vec![Action::Point {
                    column: mouse.column,
                    row: mouse.row,
                }];
            }
            Vec::new()
        }
        _ => Vec::new(),
    }
}

/// The row, log entry, or tree node drawn at a cell becomes the selection; a
/// collection and the Event Log count from the first row drawn, and a tree
/// answers for itself through `TreeState::click_at`.
fn point(state: &mut State, column: u16, row: u16) {
    let rows = state.regions.rows;
    let position = ratatui::layout::Position { x: column, y: row };
    if !rows.contains(position) {
        return;
    }
    let from_top = (row - rows.y) as usize;
    let entries = state.log.entries().len();
    match state.view_mut() {
        View::Collection(view) => {
            let index = view.offset + from_top;
            if index < view.visible().len() {
                view.selected = index;
            }
        }
        View::EventLog(view) => {
            let index = view.offset + from_top;
            if index < entries {
                view.selected = index;
            }
        }
        View::Detail(view) if !view.raw => {
            view.tree.click_at(position);
        }
        View::Tree(view) => {
            view.state.click_at(position);
        }
        _ => {}
    }
}

/// A `q` with a mutation in flight arms the prompt and toasts; a second `q`
/// quits; anything else disarms it, and the mutation count is never touched.
fn quit(state: &mut State, at: Timestamp) -> Vec<Effect> {
    if state.inflight.mutating > 0 && !state.quit_armed {
        state.quit_armed = true;
        state.toasts.push(Toast {
            at,
            text: "a mutation is in flight — press q again to quit".to_string(),
        });
        return Vec::new();
    }
    state.quit = true;
    vec![Effect::Quit]
}

/// What the palette offers, and what choosing it means.
#[derive(Clone, Debug)]
enum Chosen {
    Operation(aweber::catalog::Operation),
    Jump(usize),
    Argument(aweber::catalog::ArgSpec),
    Candidate {
        row: usize,
        candidate: crate::core::views::builder::PickCandidate,
    },
    /// Choosing a running watch cancels it.
    Cancel(crate::core::WatchId),
    /// The views that are not operations, reachable by name like everything
    /// else.
    Act(Action),
}

/// The palette rows that open a view rather than an operation.
fn palette_views() -> Vec<(String, Chosen)> {
    vec![
        (
            "raw request".to_string(),
            Chosen::Act(Action::OpenRawRequest),
        ),
        ("event log".to_string(), Chosen::Act(Action::OpenEventLog)),
        ("help".to_string(), Chosen::Act(Action::OpenHelp)),
    ]
}

/// Every row of the palette in the current scope, label and consequence.
fn palette_items(state: &State) -> Vec<(String, Chosen)> {
    use crate::core::views::palette::PaletteScope;
    let scope = match state.overlay.as_ref() {
        Some(Overlay::Palette(view)) => &view.scope,
        _ => match state.view() {
            View::Builder(view) => match view.picker.as_ref() {
                Some(picker) => &picker.scope,
                None => return Vec::new(),
            },
            _ => return Vec::new(),
        },
    };
    match scope {
        PaletteScope::Everything => crate::catalog::entries()
            .iter()
            .map(|entry| (entry.label.clone(), Chosen::Operation(entry.operation)))
            .chain(palette_views())
            .collect(),
        PaletteScope::ForSelection(kind) => crate::catalog::for_selection(*kind)
            .into_iter()
            .map(|entry| (entry.label.clone(), Chosen::Operation(entry.operation)))
            .chain(palette_views())
            .collect(),
        PaletteScope::Session => crate::catalog::session_entries()
            .iter()
            .map(|entry| (entry.label.clone(), Chosen::Operation(entry.operation)))
            .collect(),
        PaletteScope::Ancestors => state
            .breadcrumb()
            .into_iter()
            .enumerate()
            .map(|(depth, title)| (title, Chosen::Jump(depth)))
            .collect(),
        PaletteScope::Arguments(specs) => specs
            .iter()
            .map(|spec| (spec.long.clone(), Chosen::Argument(spec.clone())))
            .collect(),
        PaletteScope::Watches => state
            .watches
            .iter()
            .map(|watch| (watch.label.clone(), Chosen::Cancel(watch.id)))
            .collect(),
        PaletteScope::Candidates { row } => match state.view() {
            View::Builder(view) => view
                .candidates(*row)
                .into_iter()
                .map(|candidate| {
                    (
                        candidate.label.clone(),
                        Chosen::Candidate {
                            row: *row,
                            candidate: candidate.clone(),
                        },
                    )
                })
                .collect(),
            _ => Vec::new(),
        },
    }
}

fn labels(items: &[(String, Chosen)]) -> Vec<String> {
    items.iter().map(|(label, _)| label.clone()).collect()
}

/// `Ctrl-p` offers whatever the view it was pressed in can act on.
fn open_palette(state: &mut State) {
    use crate::core::views::palette::{PaletteScope, PaletteView};
    if let View::Builder(view) = state.view_mut() {
        let addable = view.addable();
        let candidates: Vec<String> = addable.iter().map(|spec| spec.long.clone()).collect();
        let mut picker = PaletteView::open(PaletteScope::Arguments(addable));
        picker.rescore(&candidates);
        view.picker = Some(picker);
        return;
    }
    let scope = match state.view() {
        View::Session(_) => PaletteScope::Session,
        _ => match state.selection() {
            Some(selection) => PaletteScope::ForSelection(selection.kind),
            None => PaletteScope::Everything,
        },
    };
    state.overlay = Some(Overlay::Palette(PaletteView::open(scope)));
    let candidates = labels(&palette_items(state));
    if let Some(Overlay::Palette(view)) = state.overlay.as_mut() {
        view.rescore(&candidates);
    }
}

/// The palette over the running watches, for stopping one from any view.
fn open_watches(state: &mut State) {
    use crate::core::views::palette::{PaletteScope, PaletteView};
    state.overlay = Some(Overlay::Palette(PaletteView::open(PaletteScope::Watches)));
    let candidates = labels(&palette_items(state));
    if let Some(Overlay::Palette(view)) = state.overlay.as_mut() {
        view.rescore(&candidates);
    }
}

/// The palette over the breadcrumb, for jumping to an ancestor of the stack.
fn open_ancestors(state: &mut State) {
    use crate::core::views::palette::{PaletteScope, PaletteView};
    state.overlay = Some(Overlay::Palette(PaletteView::open(PaletteScope::Ancestors)));
    let candidates = labels(&palette_items(state));
    if let Some(Overlay::Palette(view)) = state.overlay.as_mut() {
        view.rescore(&candidates);
    }
}

/// `Enter` means whatever the field it was pressed in is for.
fn submit(state: &mut State, at: Timestamp) -> Vec<Effect> {
    if matches!(state.overlay, Some(Overlay::Filter(_)))
        && let Some(Overlay::Filter(prompt)) = state.overlay.take()
    {
        let query = prompt.input.value().to_string();
        if let View::Collection(view) = state.view_mut() {
            view.filter =
                (!query.is_empty()).then(|| crate::core::filter::Filter::apply(&query, &view.rows));
            view.selected = 0;
            view.offset = 0;
        }
        return Vec::new();
    }
    if matches!(state.overlay, Some(Overlay::Confirm(_))) {
        return update(state, at, Action::Confirm);
    }
    if matches!(state.overlay, Some(Overlay::Palette(_))) {
        let items = palette_items(state);
        let Some(Overlay::Palette(view)) = state.overlay.as_ref() else {
            return Vec::new();
        };
        let chosen = view
            .selected_index()
            .and_then(|index| items.get(index))
            .map(|(_, chosen)| chosen.clone());
        state.overlay = None;
        return match chosen {
            Some(Chosen::Operation(operation)) => chose_operation(state, at, operation),
            Some(Chosen::Jump(depth)) => update(state, at, Action::JumpTo { depth }),
            Some(Chosen::Act(action)) => update(state, at, action),
            Some(Chosen::Cancel(watch)) => update(state, at, Action::CancelWatch(watch)),
            Some(Chosen::Argument(_)) | Some(Chosen::Candidate { .. }) | None => Vec::new(),
        };
    }
    match state.view() {
        View::Builder(_) => builder_submit(state, at),
        View::Session(_) => session_submit(state),
        View::RawRequest(_) => raw_submit(state),
        _ => Vec::new(),
    }
}

/// The Session view either opens the authorization page or exchanges the code
/// that came back from it.
fn session_submit(state: &mut State) -> Vec<Effect> {
    let View::Session(view) = state.view_mut() else {
        return Vec::new();
    };
    let Some(authorization) = view.authorization.clone() else {
        return vec![Effect::ReadSession];
    };
    let code = view.code.value().trim().to_string();
    if code.is_empty() {
        return vec![Effect::OpenBrowser {
            url: authorization.url,
        }];
    }
    view.error = None;
    view.code.reset();
    vec![Effect::Login {
        code,
        verifier: authorization.verifier,
    }]
}

/// The Raw Request view sends whatever its fields say, or opens the editor on
/// its body.
fn raw_submit(state: &mut State) -> Vec<Effect> {
    let View::RawRequest(view) = state.view() else {
        return Vec::new();
    };
    let plan = aweber::catalog::RequestPlan::raw(
        method_of(view.method.value()),
        view.path.value().to_string(),
        body_of(view.body.as_deref()),
    );
    let generation = state.next_generation();
    let request = state.next_request();
    if let View::RawRequest(view) = state.view_mut() {
        view.unavailable = None;
        view.generation = generation;
    }
    vec![Effect::Send {
        request,
        generation,
        plan,
        purpose: crate::core::effect::Purpose::Raw,
    }]
}

/// A row added, or the request the rows describe.
fn builder_submit(state: &mut State, at: Timestamp) -> Vec<Effect> {
    let items = palette_items(state);
    let scope = state.scope();
    let View::Builder(view) = state.view_mut() else {
        return Vec::new();
    };
    if let Some(picker) = view.picker.as_ref() {
        let chosen = picker
            .selected_index()
            .and_then(|index| items.get(index))
            .map(|(_, chosen)| chosen.clone());
        view.picker = None;
        let mut added = false;
        match chosen {
            Some(Chosen::Argument(spec)) => {
                view.add(spec);
                view.selected = view.rows.len().saturating_sub(1);
                added = true;
            }
            Some(Chosen::Candidate { row, candidate }) => {
                if let Some(crate::core::views::builder::RowValue::Pick { query, chosen, .. }) =
                    view.rows.get_mut(row).map(|row| &mut row.value)
                {
                    *query = tui_input::Input::new(candidate.label.clone());
                    *chosen = Some(candidate);
                }
            }
            _ => {}
        }
        view.reprice(scope);
        return if added {
            load_pickers(state)
        } else {
            Vec::new()
        };
    }
    if let Some(row) = unresolved_picker(view) {
        offer_candidates(view, row);
        return Vec::new();
    }
    view.reprice(scope);
    let operation = view.operation;
    let args = view.args();
    if let Err(error) = &view.preview {
        let text = error.to_string();
        state.toasts.push(Toast { at, text });
        return Vec::new();
    }
    state.stack.pop();
    if crate::catalog::metadata(operation).kind == crate::catalog::OperationKind::Mutation {
        update(state, at, Action::RunOperation { operation, args })
    } else {
        update(state, at, Action::OpenOperation { operation, args })
    }
}

/// The selected row, when it is a picker row that has not resolved yet.
fn unresolved_picker(view: &crate::core::views::builder::BuilderView) -> Option<usize> {
    use crate::core::views::builder::RowValue;
    let row = view.rows.get(view.selected)?;
    match &row.value {
        RowValue::Pick {
            chosen: None,
            candidates,
            ..
        } if !candidates.is_empty() => Some(view.selected),
        _ => None,
    }
}

/// `Enter` on an unresolved picker row offers its candidates.
fn offer_candidates(view: &mut crate::core::views::builder::BuilderView, row: usize) {
    use crate::core::views::palette::{PaletteScope, PaletteView};
    let candidates: Vec<String> = view
        .candidates(row)
        .into_iter()
        .map(|candidate| candidate.label.clone())
        .collect();
    let mut picker = PaletteView::open(PaletteScope::Candidates { row });
    picker.rescore(&candidates);
    view.picker = Some(picker);
}

/// A chosen operation either runs as it stands or opens its builder.
fn chose_operation(
    state: &mut State,
    at: Timestamp,
    operation: aweber::catalog::Operation,
) -> Vec<Effect> {
    let args = state.prefill(operation);
    let buildable = state
        .scope()
        .is_some_and(|scope| aweber::catalog::RequestPlan::build(operation, &scope, &args).is_ok());
    if !buildable || configurable(operation) {
        let scope = state.scope();
        let generation = state.next_generation();
        let mut view = crate::core::views::builder::BuilderView::open(operation, args, generation);
        view.reprice(scope);
        state.stack.push(View::Builder(view));
        return load_pickers(state);
    }
    if crate::catalog::metadata(operation).kind == crate::catalog::OperationKind::Mutation {
        update(state, at, Action::RunOperation { operation, args })
    } else {
        update(state, at, Action::OpenOperation { operation, args })
    }
}

/// An operation with anything to fill in beyond its identifiers is configured
/// before it is sent.
fn configurable(operation: aweber::catalog::Operation) -> bool {
    use aweber::catalog::ArgRole;
    operation.specs().iter().any(|spec| {
        matches!(spec.role, ArgRole::Filter | ArgRole::JsonBody)
            && !crate::catalog::metadata(operation)
                .fills
                .iter()
                .any(|fill| fill.arg == spec.name)
    })
}

/// The argument a whole JSON body is entered as.
fn body_arg(operation: aweber::catalog::Operation) -> Option<aweber::catalog::ArgSpec> {
    let specs = operation.specs();
    ["json-body", "patch", "file"]
        .into_iter()
        .find_map(|wanted| specs.iter().find(|spec| spec.name == wanted).cloned())
        .or_else(|| {
            specs
                .iter()
                .find(|spec| spec.role == aweber::catalog::ArgRole::JsonBody)
                .cloned()
        })
}

/// A mutation asks for its body, then for confirmation, then sends.
fn run(
    state: &mut State,
    at: Timestamp,
    operation: aweber::catalog::Operation,
    args: aweber::catalog::Args,
) -> Vec<Effect> {
    state.overlay = None;
    let metadata = crate::catalog::metadata(operation);
    if let Some(kind) = metadata.editor
        && let Some(spec) = body_arg(operation)
        && !args.contains(&spec.name)
    {
        return vec![Effect::Edit {
            purpose: crate::core::effect::EditPurpose::JsonBody { operation, args },
            seed: seed(state, kind),
            extension: "json",
        }];
    }
    match metadata.tier {
        crate::catalog::ConfirmationTier::None => mutate(state, operation, args),
        tier => {
            let effect = match state.scope() {
                Some(scope) => aweber::catalog::RequestPlan::build(operation, &scope, &args)
                    .map(|plan| plan.line())
                    .unwrap_or_else(|error| error.to_string()),
                None => "no account is resolved yet".to_string(),
            };
            let _ = at;
            state.overlay = Some(Overlay::Confirm(crate::core::views::confirm::ConfirmView {
                operation,
                args,
                effect,
                tier,
                typed: tui_input::Input::default(),
            }));
            Vec::new()
        }
    }
}

/// What the editor opens with, for a body the current document already shapes.
fn seed(state: &State, kind: crate::catalog::EditKind) -> String {
    let document = state.selection().map(|selection| selection.document);
    match kind {
        crate::catalog::EditKind::Ruleset => {
            let slot = |key: &str| {
                document
                    .as_ref()
                    .and_then(|document| document.pointer(&format!("/unpublished/{key}")))
                    .cloned()
                    .unwrap_or_else(|| serde_json::Value::Array(Vec::new()))
            };
            pretty(&serde_json::json!({
                "events": slot("events"),
                "actions": slot("actions"),
            }))
        }
        crate::catalog::EditKind::JsonPatch => {
            pretty(&serde_json::json!([{"op": "replace", "path": "/name", "value": ""}]))
        }
        crate::catalog::EditKind::JsonBody => match document {
            Some(document) => pretty(&document),
            None => "{}".to_string(),
        },
    }
}

fn pretty(value: &serde_json::Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
}

/// The editor came back: parse it, keep it on failure, run it on success.
fn edited(
    state: &mut State,
    at: Timestamp,
    purpose: crate::core::effect::EditPurpose,
    text: Result<String, crate::ports::EditorError>,
) -> Vec<Effect> {
    use crate::core::effect::EditPurpose;
    let (operation, mut args) = match purpose.clone() {
        EditPurpose::JsonBody { operation, args } => (operation, args),
        EditPurpose::Ruleset {
            workflow,
            precondition,
        } => (
            aweber::catalog::Operation::UpdateWorkflowRuleset,
            workflow_args(workflow, precondition),
        ),
        EditPurpose::JsonPatch {
            workflow,
            precondition,
        } => (
            aweber::catalog::Operation::UpdateWorkflow,
            workflow_args(workflow, precondition),
        ),
    };
    let text = match text {
        Ok(text) => text,
        Err(error) => {
            state.toasts.push(Toast {
                at,
                text: match error {
                    crate::ports::EditorError::NotConfigured => {
                        "neither $VISUAL nor $EDITOR is set".to_string()
                    }
                    crate::ports::EditorError::Empty => "the editor saved nothing".to_string(),
                    crate::ports::EditorError::Failed { reason } => {
                        format!("the editor failed: {reason}")
                    }
                },
            });
            return Vec::new();
        }
    };
    if let Err(error) = serde_json::from_str::<serde_json::Value>(&text) {
        state.toasts.push(Toast {
            at,
            text: format!("that is not JSON: {error}"),
        });
        return vec![Effect::Edit {
            purpose,
            seed: text,
            extension: "json",
        }];
    }
    let Some(spec) = body_arg(operation) else {
        return Vec::new();
    };
    args.set(
        spec.name.clone(),
        aweber::catalog::ArgValue::new(spec.kind.clone(), text),
    );
    run(state, at, operation, args)
}

fn workflow_args(workflow: uuid::Uuid, precondition: Option<i64>) -> aweber::catalog::Args {
    let mut args = aweber::catalog::Args::default();
    args.set(
        "workflow",
        aweber::catalog::ArgValue::new(aweber::catalog::ValueKind::Uuid, workflow.to_string()),
    );
    if let Some(version) = precondition {
        args.set(
            aweber::catalog::PRECONDITION_ARG,
            aweber::catalog::ArgValue::new(
                aweber::catalog::ValueKind::Integer,
                version.to_string(),
            ),
        );
    }
    args
}

/// The mutation itself, once it is confirmed and complete.
fn mutate(
    state: &mut State,
    operation: aweber::catalog::Operation,
    args: aweber::catalog::Args,
) -> Vec<Effect> {
    let generation = state.next_generation();
    let target = state.selected_entity();
    let effects = send(
        state,
        operation,
        &args.clone(),
        generation,
        crate::core::effect::Purpose::Mutation {
            operation,
            args,
            target,
        },
    );
    if !effects.is_empty() {
        state.inflight.mutating += 1;
    }
    effects
}

/// What a mutation's success drops from the cache, and the views that must ask
/// again; the entity is the one the mutation was issued against, never the one
/// the selection has moved to since.
fn invalidate(
    state: &mut State,
    operation: aweber::catalog::Operation,
    target: Option<crate::core::state::EntityKey>,
) {
    use crate::catalog::Invalidation;
    let metadata = crate::catalog::metadata(operation);
    let entity = target;
    for wanted in metadata.invalidates {
        match wanted {
            Invalidation::Selection => {
                if let Some(entity) = &entity {
                    state.cache.invalidate_entity(entity);
                }
            }
            Invalidation::ParentCollection => {
                let keys: Vec<crate::core::cache::CacheKey> = state
                    .stack
                    .iter()
                    .filter_map(|view| match view {
                        View::Collection(view) => Some(crate::core::cache::CacheKey {
                            operation: view.operation,
                            args: view.args.clone(),
                        }),
                        _ => None,
                    })
                    .collect();
                for key in keys {
                    state.cache.invalidate(&key);
                    stale(state, &key);
                }
            }
            Invalidation::Operation(operation) => {
                state.cache.invalidate_operation(*operation);
                let keys: Vec<crate::core::cache::CacheKey> = state
                    .stack
                    .iter()
                    .filter_map(|view| match view {
                        View::Collection(view) if view.operation == *operation => {
                            Some(crate::core::cache::CacheKey {
                                operation: view.operation,
                                args: view.args.clone(),
                            })
                        }
                        _ => None,
                    })
                    .collect();
                for key in keys {
                    stale(state, &key);
                }
            }
        }
    }
}

/// An open collection whose contents no longer hold is emptied, so returning to
/// it asks again.
fn stale(state: &mut State, key: &crate::core::cache::CacheKey) {
    for view in &mut state.stack {
        if let View::Collection(view) = view
            && view.operation == key.operation
            && view.args == key.args
        {
            view.rows.clear();
            view.filter = None;
            view.cursor = None;
            view.end = None;
            view.total = None;
            view.selected = 0;
            view.offset = 0;
            view.loading = false;
            view.unavailable = None;
        }
    }
}

/// A collection left empty by an invalidation asks again as soon as it is on
/// top.
fn refetch_stale(state: &mut State) -> Vec<Effect> {
    let View::Collection(view) = state.view() else {
        return Vec::new();
    };
    if !view.rows.is_empty() || view.loading || view.end.is_some() || view.unavailable.is_some() {
        return Vec::new();
    }
    refresh(state)
}

/// Open an operation as the view its kind calls for.
fn open(
    state: &mut State,
    at: Timestamp,
    operation: aweber::catalog::Operation,
    args: aweber::catalog::Args,
) -> Vec<Effect> {
    use crate::catalog::OperationKind::{Collection, Document, Search, Tree, Watch};

    state.overlay = None;
    let mut merged = state.prefill(operation);
    for (name, values) in args.iter() {
        for value in values {
            merged.set(name, value.clone());
        }
    }
    let metadata = crate::catalog::metadata(operation);
    let title = crate::catalog::entry(operation).label.clone();

    match metadata.kind {
        Collection | Search => {
            let key = crate::core::cache::CacheKey {
                operation,
                args: merged.clone(),
            };
            let generation = state.next_generation();
            let mut view = crate::core::views::collection::CollectionView::opening(
                operation,
                merged.clone(),
                title,
                generation,
            );
            if let Some(cached) = state.cache.get(&key) {
                view.rows = cached.rows.clone();
                view.cursor = cached.cursor.clone();
                view.end = cached.end.clone();
                view.total = cached.total;
                view.selected = cached.selected;
                view.offset = cached.offset;
                view.loading = false;
                state.stack.push(View::Collection(view));
                return Vec::new();
            }
            state.stack.push(View::Collection(view));
            send(
                state,
                operation,
                &merged,
                generation,
                crate::core::effect::Purpose::Collection { key },
            )
        }
        Document => {
            let generation = state.next_generation();
            state.stack.push(View::Detail(
                crate::core::views::detail::DetailView::opening(
                    operation,
                    merged.clone(),
                    title,
                    generation,
                ),
            ));
            send(
                state,
                operation,
                &merged,
                generation,
                crate::core::effect::Purpose::Document { operation },
            )
        }
        Tree => {
            let generation = state.next_generation();
            let workflow = merged
                .first("workflow")
                .and_then(|value| value.raw.parse().ok())
                .unwrap_or_default();
            state
                .stack
                .push(View::Tree(crate::core::views::tree::TreeView {
                    workflow,
                    title,
                    generation,
                    state: tui_tree_widget::TreeState::default(),
                    items: Vec::new(),
                    unavailable: None,
                }));
            send(
                state,
                operation,
                &merged,
                generation,
                crate::core::effect::Purpose::Document { operation },
            )
        }
        Watch => start_watch(state, at, operation, merged),
        _ => Vec::new(),
    }
}

/// The effect that fetches an operation, or the failure of having no plan.
fn send(
    state: &mut State,
    operation: aweber::catalog::Operation,
    args: &aweber::catalog::Args,
    generation: crate::core::Generation,
    purpose: crate::core::effect::Purpose,
) -> Vec<Effect> {
    let Some(scope) = state.scope() else {
        unavailable(state, no_account());
        return Vec::new();
    };
    match aweber::catalog::RequestPlan::build(operation, &scope, args) {
        Ok(plan) => {
            let request = state.next_request();
            vec![Effect::Send {
                request,
                generation,
                plan,
                purpose,
            }]
        }
        Err(error) => {
            unavailable(
                state,
                crate::core::failure::Failure {
                    status: None,
                    method: "GET".to_string(),
                    path: crate::catalog::entry(operation).label.clone(),
                    message: error.to_string(),
                    kind: crate::core::failure::FailureKind::Decode,
                },
            );
            Vec::new()
        }
    }
}

fn no_account() -> crate::core::failure::Failure {
    crate::core::failure::Failure {
        status: None,
        method: "GET".to_string(),
        path: "no account".to_string(),
        message: "no account is resolved yet".to_string(),
        kind: crate::core::failure::FailureKind::Session,
    }
}

/// The stack index of the view that asked with this generation.
fn asked_with(state: &State, generation: crate::core::Generation) -> Option<usize> {
    state
        .stack
        .iter()
        .position(|view| view.generation() == Some(generation))
}

/// A failure delivered to the view that asked for it: inline where that view
/// holds an inline failure, a modal where that view is on top and the status is
/// not one that renders inline, and dropped where that view is gone.
fn answered_unavailable(
    state: &mut State,
    generation: crate::core::Generation,
    failure: crate::core::failure::Failure,
) {
    let Some(index) = asked_with(state, generation) else {
        return;
    };
    let inline = failure.inline();
    match &mut state.stack[index] {
        View::Collection(view) if inline => {
            view.loading = false;
            view.unavailable = Some(failure);
        }
        View::Detail(view) if inline => view.unavailable = Some(failure),
        View::Tree(view) if inline => view.unavailable = Some(failure),
        _ => {
            if index + 1 == state.stack.len() {
                state.overlay = Some(Overlay::Error(failure));
            }
        }
    }
}

/// A failure raised with no request behind it, on the view that is on top.
fn unavailable(state: &mut State, failure: crate::core::failure::Failure) {
    let inline = failure.inline();
    match state.view_mut() {
        View::Collection(view) if inline => {
            view.loading = false;
            view.unavailable = Some(failure);
        }
        View::Detail(view) if inline => view.unavailable = Some(failure),
        View::Tree(view) if inline => view.unavailable = Some(failure),
        _ => state.overlay = Some(Overlay::Error(failure)),
    }
}

/// Descending opens whatever the current row stands for.
fn descend(state: &mut State, at: Timestamp) -> Vec<Effect> {
    match state.view() {
        View::AccountPicker(view) => {
            let Some(account) = state.accounts.get(view.selected).cloned() else {
                return Vec::new();
            };
            update(state, at, Action::AccountChosen(account))
        }
        View::Home(view) => {
            let Some(group) = crate::core::views::home::HomeView::groups().get(view.selected)
            else {
                return Vec::new();
            };
            let Some(operation) = group
                .operations
                .iter()
                .copied()
                .find(|operation| {
                    matches!(
                        crate::catalog::metadata(*operation).kind,
                        crate::catalog::OperationKind::Collection
                    )
                })
                .or_else(|| group.operations.first().copied())
            else {
                return Vec::new();
            };
            open(state, at, operation, aweber::catalog::Args::default())
        }
        View::Collection(view) => {
            let operation = view.operation;
            let Some(selection) = state.selection() else {
                return Vec::new();
            };
            adopt_context(state, &selection);
            let Some(next) = document_of(selection.kind) else {
                state.toasts.push(Toast {
                    at,
                    text: format!(
                        "{} has nothing to open",
                        crate::catalog::entry(operation).label
                    ),
                });
                return Vec::new();
            };
            open(state, at, next, aweber::catalog::Args::default())
        }
        _ => Vec::new(),
    }
}

/// A list the user descends into becomes the ambient list.
fn adopt_context(state: &mut State, selection: &crate::core::state::Selection) {
    if selection.kind != crate::catalog::EntityKind::List {
        return;
    }
    let number = |pointer: &str| {
        selection
            .document
            .pointer(pointer)
            .and_then(serde_json::Value::as_i64)
    };
    let Some(id) = number("/id") else {
        return;
    };
    state.context.list = Some(crate::core::state::ListRef {
        id: id as i32,
        uuid: selection
            .document
            .pointer("/uuid")
            .and_then(serde_json::Value::as_str)
            .and_then(|uuid| uuid.parse().ok()),
        name: selection
            .document
            .pointer("/name")
            .and_then(serde_json::Value::as_str)
            .map(ToString::to_string),
        self_link: selection.self_link.clone(),
    });
}

/// The document operation of an entity kind, when it has one.
fn document_of(kind: crate::catalog::EntityKind) -> Option<aweber::catalog::Operation> {
    crate::catalog::for_selection(kind)
        .into_iter()
        .find(|entry| entry.metadata.kind == crate::catalog::OperationKind::Document)
        .map(|entry| entry.operation)
}

/// `r` drops what was cached and asks again.
fn refresh(state: &mut State) -> Vec<Effect> {
    match state.view() {
        View::Collection(view) => {
            let operation = view.operation;
            let args = view.args.clone();
            let key = crate::core::cache::CacheKey {
                operation,
                args: args.clone(),
            };
            state.cache.invalidate(&key);
            let generation = state.next_generation();
            if let View::Collection(view) = state.view_mut() {
                view.rows.clear();
                view.cursor = None;
                view.end = None;
                view.total = None;
                view.selected = 0;
                view.offset = 0;
                view.filter = None;
                view.loading = true;
                view.unavailable = None;
                view.generation = generation;
            }
            send(
                state,
                operation,
                &args,
                generation,
                crate::core::effect::Purpose::Collection { key },
            )
        }
        View::Detail(view) => {
            let operation = view.operation;
            let args = view.args.clone();
            let generation = state.next_generation();
            if let View::Detail(view) = state.view_mut() {
                view.unavailable = None;
                view.generation = generation;
            }
            send(
                state,
                operation,
                &args,
                generation,
                crate::core::effect::Purpose::Document { operation },
            )
        }
        _ => Vec::new(),
    }
}

/// Reaching the tail of a collection fetches the next page with no keystroke.
fn next_page(state: &mut State) -> Vec<Effect> {
    let View::Collection(view) = state.view() else {
        return Vec::new();
    };
    if !view.wants_next_page() {
        return Vec::new();
    }
    let (operation, args, generation) = (view.operation, view.args.clone(), view.generation);
    let Some(cursor) = view.cursor.clone() else {
        return Vec::new();
    };
    let key = crate::core::cache::CacheKey {
        operation,
        args: args.clone(),
    };
    let Some(scope) = state.scope() else {
        return Vec::new();
    };
    let Ok(plan) = aweber::catalog::RequestPlan::build(operation, &scope, &args) else {
        return Vec::new();
    };
    if let View::Collection(view) = state.view_mut() {
        view.loading = true;
    }
    let request = state.next_request();
    vec![Effect::Send {
        request,
        generation,
        plan: crate::core::collection::continue_plan(&plan, &cursor),
        purpose: crate::core::effect::Purpose::Page { key },
    }]
}

/// A response, matched to the view that asked for it.
fn delivered(
    state: &mut State,
    at: Timestamp,
    generation: crate::core::Generation,
    purpose: crate::core::effect::Purpose,
    outcome: Result<crate::core::collection::Delivered, crate::core::failure::Failure>,
) -> Vec<Effect> {
    use crate::core::effect::Purpose;

    let delivered = match outcome {
        Ok(delivered) => delivered,
        Err(failure) => return failed(state, at, generation, purpose, failure),
    };

    match purpose {
        Purpose::Collection { key } | Purpose::Page { key } => {
            let columns = crate::catalog::metadata(key.operation).columns;
            let mut rows = crate::core::collection::rows(delivered.entries, columns);
            let Some(index) = asked_with(state, generation) else {
                return Vec::new();
            };
            let View::Collection(view) = &mut state.stack[index] else {
                return Vec::new();
            };
            view.loading = false;
            view.unavailable = None;
            view.rows.append(&mut rows);
            view.total = delivered.total.or(view.total);
            view.cursor = delivered.next;
            view.end = delivered
                .capped
                .map(|offset| crate::core::collection::EndReason::OffsetCap { offset })
                .or_else(|| {
                    view.cursor
                        .is_none()
                        .then_some(crate::core::collection::EndReason::Exhausted)
                });
            if let Some(filter) = &view.filter {
                view.filter = Some(crate::core::filter::Filter::apply(
                    &filter.query.clone(),
                    &view.rows,
                ));
            }
            let cached = crate::core::cache::Cached {
                rows: view.rows.clone(),
                document: None,
                cursor: view.cursor.clone(),
                end: view.end.clone(),
                total: view.total,
                selected: view.selected,
                offset: view.offset,
            };
            state.cache.put(key, cached);
            if index + 1 == state.stack.len() {
                next_page(state)
            } else {
                Vec::new()
            }
        }
        Purpose::Document { operation } => {
            document_delivered(state, at, generation, operation, delivered)
        }
        Purpose::Mutation {
            operation,
            args,
            target,
        } => {
            state.inflight.mutating = state.inflight.mutating.saturating_sub(1);
            let line = match state.scope() {
                Some(scope) => aweber::catalog::RequestPlan::build(operation, &scope, &args)
                    .map(|plan| plan.line())
                    .unwrap_or_else(|_| crate::catalog::entry(operation).label.clone()),
                None => crate::catalog::entry(operation).label.clone(),
            };
            state.toasts.push(Toast {
                at,
                text: format!("{line} — {}", delivered.status),
            });
            invalidate(state, operation, target);
            refetch_stale(state)
        }
        Purpose::Precondition { operation, args } => rebase(state, at, operation, args, delivered),
        Purpose::Launch => {
            let accounts = delivered.entries.iter().filter_map(account_of).collect();
            update(state, at, Action::AccountsLoaded(Ok(accounts)))
        }
        Purpose::Raw => {
            if let Some(index) = asked_with(state, generation)
                && let View::RawRequest(view) = &mut state.stack[index]
            {
                view.unavailable = None;
                view.response = Some(delivered);
                view.focus = crate::core::views::raw_request::RawFocus::Response;
            }
            Vec::new()
        }
        Purpose::Enrichment { of } => {
            enrichment_delivered(state, at, generation, &of, Ok(delivered));
            Vec::new()
        }
        Purpose::ListPicker { row } => {
            picker_delivered(state, at, generation, row, Ok(delivered));
            Vec::new()
        }
    }
}

/// A failed request, by what asked for it: a mutation's failure is a modal and
/// never an inline body, a failed re-read after a `412` re-opens the editor on
/// the edit that was made, an enrichment or a picker failure is a log line, and
/// a collection's failure ends the list where it stands.
fn failed(
    state: &mut State,
    at: Timestamp,
    generation: crate::core::Generation,
    purpose: crate::core::effect::Purpose,
    failure: crate::core::failure::Failure,
) -> Vec<Effect> {
    use crate::core::effect::Purpose;

    if let Purpose::Mutation {
        operation, args, ..
    } = &purpose
    {
        state.inflight.mutating = state.inflight.mutating.saturating_sub(1);
        if let crate::core::failure::FailureKind::Precondition { .. } = failure.kind {
            return reread(state, at, *operation, args.clone(), &failure);
        }
    }
    if let Purpose::Launch = purpose {
        return update(state, at, Action::AccountsLoaded(Err(failure)));
    }
    if failure.kind == crate::core::failure::FailureKind::Session {
        state.held = held_of(&purpose);
        demand_session(state);
        if let View::Session(view) = state.view_mut() {
            view.error = Some(failure.message.clone());
        }
        return Vec::new();
    }
    match purpose {
        Purpose::Mutation { .. } => {
            blocked(state, failure);
            Vec::new()
        }
        Purpose::Precondition { operation, args } => {
            state.log.append(crate::core::event_log::LogEntry {
                at,
                method: failure.method.clone(),
                path: failure.path.clone(),
                status: failure.status,
                duration: None,
                attempt: 1,
                waited: None,
                refreshed: false,
                detail: Some("the re-read failed; the edit was kept".to_string()),
                body: None,
            });
            state.toasts.push(Toast {
                at,
                text: format!("the re-read failed: {}", failure.text()),
            });
            reopen_editor(operation, args)
        }
        Purpose::Enrichment { of } => {
            enrichment_delivered(state, at, generation, &of, Err(failure));
            Vec::new()
        }
        Purpose::ListPicker { row } => {
            picker_delivered(state, at, generation, row, Err(failure));
            Vec::new()
        }
        Purpose::Raw => {
            if let View::RawRequest(view) = state.view_mut()
                && view.generation == generation
            {
                view.unavailable = Some(failure);
            }
            Vec::new()
        }
        Purpose::Collection { .. } | Purpose::Page { .. } => {
            if let Some(index) = asked_with(state, generation)
                && let View::Collection(view) = &mut state.stack[index]
            {
                view.loading = false;
                view.end = Some(crate::core::collection::EndReason::Unavailable {
                    status: failure.status.unwrap_or_default(),
                    method: failure.method.clone(),
                    path: failure.path.clone(),
                });
            }
            answered_unavailable(state, generation, failure);
            Vec::new()
        }
        Purpose::Document { .. } => {
            answered_unavailable(state, generation, failure);
            Vec::new()
        }
        Purpose::Launch => Vec::new(),
    }
}

/// A failure that must not be mistaken for an empty view: always the modal,
/// whatever the status.
fn blocked(state: &mut State, failure: crate::core::failure::Failure) {
    state.overlay = Some(Overlay::Error(failure));
}

/// The document reached the detail or tree that asked for it wherever that view
/// sits in the stack, or is dropped where no view is waiting for it; a
/// workflow's document also states its visualization and asks for its message
/// subjects.
fn document_delivered(
    state: &mut State,
    at: Timestamp,
    generation: crate::core::Generation,
    operation: aweber::catalog::Operation,
    delivered: crate::core::collection::Delivered,
) -> Vec<Effect> {
    let zone = *at.offset();
    let document = delivered.document.clone();
    let Some(index) = asked_with(state, generation) else {
        return Vec::new();
    };
    match &mut state.stack[index] {
        View::Detail(view) => {
            view.unavailable = None;
            if let Some(document) = document.clone() {
                view.show(document, zone);
            }
        }
        View::Tree(view) => {
            view.unavailable = None;
            view.items = document
                .as_ref()
                .map(|document| crate::view::fields::tree_items(document, zone))
                .unwrap_or_default();
            if let Some(first) = view.items.first() {
                view.state.select(vec![first.identifier().clone()]);
            }
        }
        _ => return Vec::new(),
    }
    match document {
        Some(document) => enrich_workflow(state, at, index, operation, &document),
        None => Vec::new(),
    }
}

/// A workflow states its visualization from the ruleset it already holds and
/// asks for the subjects of the messages that ruleset sends, on the detail at
/// this index and under that detail's own generation.
fn enrich_workflow(
    state: &mut State,
    at: Timestamp,
    index: usize,
    operation: aweber::catalog::Operation,
    document: &serde_json::Value,
) -> Vec<Effect> {
    if crate::catalog::metadata(operation).entity != crate::catalog::EntityKind::Workflow {
        return Vec::new();
    }
    let graph = crate::core::visualization::working_graph(document);
    let reason = graph.as_ref().err().cloned();
    let value = graph
        .as_ref()
        .map(crate::core::visualization::visualization)
        .map_err(Clone::clone);
    let entity = match &mut state.stack[index] {
        View::Detail(view) => {
            view.enrich("visualization", value);
            view.entity.clone()
        }
        _ => return Vec::new(),
    };
    if let Some(reason) = reason {
        state.log.append(crate::core::event_log::LogEntry {
            at,
            method: "WALK".to_string(),
            path: "visualization".to_string(),
            status: None,
            duration: None,
            attempt: 1,
            waited: None,
            refreshed: false,
            detail: Some(reason),
            body: None,
        });
    }
    let messages: Vec<aweber::ids::MessageId> = graph
        .map(|graph| graph.message_cadences().into_keys().collect())
        .unwrap_or_default();
    let Some(entity) = entity else {
        return Vec::new();
    };
    if messages.is_empty() {
        return Vec::new();
    }
    let generation = match &state.stack[index] {
        View::Detail(view) => view.generation,
        _ => return Vec::new(),
    };
    let request = state.next_request();
    vec![Effect::Send {
        request,
        generation,
        plan: aweber::catalog::RequestPlan::message_subjects(&messages),
        purpose: crate::core::effect::Purpose::Enrichment { of: entity },
    }]
}

/// Message subjects, on the detail that is still showing the entity they belong
/// to wherever it sits in the stack; a failure marks the enrichment unavailable
/// with its reason and logs it, and neither outcome touches the document or
/// opens a modal.
fn enrichment_delivered(
    state: &mut State,
    at: Timestamp,
    generation: crate::core::Generation,
    of: &crate::core::state::EntityKey,
    outcome: Result<crate::core::collection::Delivered, crate::core::failure::Failure>,
) {
    const LABEL: &str = "message subjects";
    let reason = match &outcome {
        Ok(_) => None,
        Err(failure) => Some(failure.message.clone()),
    };
    let value = match outcome {
        Ok(delivered) => {
            let document = delivered.document.unwrap_or(serde_json::Value::Null);
            let subjects: Vec<serde_json::Value> = aweber::workflows::subjects_of(&document)
                .into_iter()
                .map(|(id, subject)| serde_json::Value::String(format!("{id} — {subject}")))
                .collect();
            Ok(serde_json::Value::Array(subjects))
        }
        Err(failure) => Err(failure.text()),
    };
    if let Some(reason) = &reason {
        state.log.append(crate::core::event_log::LogEntry {
            at,
            method: "POST".to_string(),
            path: LABEL.to_string(),
            status: None,
            duration: None,
            attempt: 1,
            waited: None,
            refreshed: false,
            detail: Some(reason.clone()),
            body: None,
        });
    }
    if let Some(index) = asked_with(state, generation)
        && let View::Detail(view) = &mut state.stack[index]
        && view.entity.as_ref() == Some(of)
    {
        view.enrich(LABEL, value);
    }
}

/// The candidates of a builder row, labelled by the picker's pointer and valued
/// by its resolution, on the builder that asked for them and no other; a
/// failure toasts and logs, and the row stays typeable.
fn picker_delivered(
    state: &mut State,
    at: Timestamp,
    generation: crate::core::Generation,
    row: usize,
    outcome: Result<crate::core::collection::Delivered, crate::core::failure::Failure>,
) {
    use crate::core::views::builder::{PickCandidate, RowValue};
    let delivered = match outcome {
        Ok(delivered) => delivered,
        Err(failure) => {
            state.log.append(crate::core::event_log::LogEntry {
                at,
                method: failure.method.clone(),
                path: failure.path.clone(),
                status: failure.status,
                duration: None,
                attempt: 1,
                waited: None,
                refreshed: false,
                detail: Some("the candidates could not be loaded".to_string()),
                body: None,
            });
            state.toasts.push(Toast {
                at,
                text: format!("no candidates: {}", failure.text()),
            });
            return;
        }
    };
    let Some(index) = asked_with(state, generation) else {
        return;
    };
    let View::Builder(view) = &mut state.stack[index] else {
        return;
    };
    let Some(builder_row) = view.rows.get_mut(row) else {
        return;
    };
    let kind = builder_row.spec.kind.clone();
    let RowValue::Pick {
        picker, candidates, ..
    } = &mut builder_row.value
    else {
        return;
    };
    *candidates = delivered
        .entries
        .iter()
        .filter_map(|entry| {
            let value = entry.pointer(picker.resolution.pointer())?;
            let raw = match value {
                serde_json::Value::String(text) => text.clone(),
                other => other.to_string(),
            };
            let label = entry
                .pointer(picker.label)
                .and_then(serde_json::Value::as_str)
                .map(ToString::to_string)
                .unwrap_or_else(|| raw.clone());
            Some(PickCandidate {
                label,
                value: aweber::catalog::ArgValue::new(kind.clone(), raw),
            })
        })
        .collect();
}

/// The candidates a builder's picker rows have not asked for yet, asked for
/// once under the builder's own generation; a row whose plan cannot be built
/// stays a typed row.
fn load_pickers(state: &mut State) -> Vec<Effect> {
    let View::Builder(view) = state.view() else {
        return Vec::new();
    };
    let generation = view.generation;
    let unloaded = view.unloaded_pickers();
    let Some(scope) = state.scope() else {
        return Vec::new();
    };
    let mut effects = Vec::new();
    for (row, picker) in unloaded {
        let Ok(plan) = aweber::catalog::RequestPlan::build(
            picker.operation,
            &scope,
            &aweber::catalog::Args::default(),
        ) else {
            continue;
        };
        let request = state.next_request();
        effects.push(Effect::Send {
            request,
            generation,
            plan,
            purpose: crate::core::effect::Purpose::ListPicker { row },
        });
    }
    effects
}

/// A lost precondition race re-reads the document rather than replaying.
fn reread(
    state: &mut State,
    at: Timestamp,
    operation: aweber::catalog::Operation,
    args: aweber::catalog::Args,
    failure: &crate::core::failure::Failure,
) -> Vec<Effect> {
    let stale_version = args
        .first(aweber::catalog::PRECONDITION_ARG)
        .and_then(|value| value.raw.parse::<i64>().ok());
    state.log.append(crate::core::event_log::LogEntry {
        at,
        method: failure.method.clone(),
        path: failure.path.clone(),
        status: failure.status,
        duration: None,
        attempt: 1,
        waited: None,
        refreshed: false,
        detail: Some(match stale_version {
            Some(version) => {
                format!("another writer changed it; the version sent was {version}")
            }
            None => "another writer changed it".to_string(),
        }),
        body: None,
    });
    state.toasts.push(Toast {
        at,
        text: "another writer changed it — re-reading".to_string(),
    });
    let Some(document) = document_of(crate::catalog::metadata(operation).entity) else {
        blocked(state, failure.clone());
        return Vec::new();
    };
    let mut reread = aweber::catalog::Args::default();
    for (name, values) in args.iter() {
        if name == aweber::catalog::PRECONDITION_ARG {
            continue;
        }
        for value in values {
            reread.set(name, value.clone());
        }
    }
    let generation = state.next_generation();
    send(
        state,
        document,
        &reread,
        generation,
        crate::core::effect::Purpose::Precondition { operation, args },
    )
}

/// The fresh document arrived: name both versions and re-open the editor with
/// the user's edit intact.
fn rebase(
    state: &mut State,
    at: Timestamp,
    operation: aweber::catalog::Operation,
    args: aweber::catalog::Args,
    delivered: crate::core::collection::Delivered,
) -> Vec<Effect> {
    let stale_version = args
        .first(aweber::catalog::PRECONDITION_ARG)
        .map(|value| value.raw.clone());
    let current = delivered
        .document
        .as_ref()
        .and_then(|document| document.pointer("/precondition_version"))
        .map(|value| match value {
            serde_json::Value::String(text) => text.clone(),
            other => other.to_string(),
        });
    state.log.append(crate::core::event_log::LogEntry {
        at,
        method: "GET".to_string(),
        path: crate::catalog::entry(operation).label.clone(),
        status: Some(delivered.status),
        duration: None,
        attempt: 1,
        waited: None,
        refreshed: false,
        detail: Some(format!(
            "precondition {} is stale; the current version is {}",
            stale_version
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            current.clone().unwrap_or_else(|| "unknown".to_string())
        )),
        body: None,
    });
    let mut next = args.clone();
    next.remove(aweber::catalog::PRECONDITION_ARG);
    if let Some(current) = current {
        next.set(
            aweber::catalog::PRECONDITION_ARG,
            aweber::catalog::ArgValue::new(aweber::catalog::ValueKind::Integer, current),
        );
    }
    reopen_editor(operation, next)
}

/// The editor opens again on the edit that was made, never on the document that
/// replaced it.
fn reopen_editor(
    operation: aweber::catalog::Operation,
    args: aweber::catalog::Args,
) -> Vec<Effect> {
    let Some(spec) = body_arg(operation) else {
        return Vec::new();
    };
    let seed = args
        .first(&spec.name)
        .map(|value| value.raw.clone())
        .unwrap_or_else(|| "{}".to_string());
    let mut next = args;
    next.remove(&spec.name);
    vec![Effect::Edit {
        purpose: crate::core::effect::EditPurpose::JsonBody {
            operation,
            args: next,
        },
        seed,
        extension: "json",
    }]
}

/// Every list moves the same way; a tree moves its own selection instead.
fn move_selection(state: &mut State, motion: Motion) {
    let region = state.regions.rows.height.max(1) as usize;
    let page = page_height(state);
    if moved_a_tree(state, motion, page) {
        return;
    }
    let length = list_length(state);
    let selected = selected_mut(state);
    let Some(selected) = selected else {
        return;
    };
    let last = length.saturating_sub(1);
    *selected = match motion {
        Motion::Down => (*selected + 1).min(last),
        Motion::Up => selected.saturating_sub(1),
        Motion::Top => 0,
        Motion::Bottom => last,
        Motion::PageDown => (*selected + page).min(last),
        Motion::PageUp => selected.saturating_sub(page),
    };
    follow(state, region);
}

/// The rows a motion covers: a `Detail` showing raw JSON is as short as its
/// enrichment rows leave it.
fn page_height(state: &State) -> usize {
    let region = state.regions.rows.height.max(1) as usize;
    match state.view() {
        View::Detail(view) if view.raw => view.body_height(region),
        _ => region,
    }
}

/// The selection stays drawn, so a click at a cell names what a key selected.
fn follow(state: &mut State, height: usize) {
    match state.view_mut() {
        View::Collection(view) => view.follow(height),
        View::EventLog(view) => view.follow(height),
        View::Detail(view) => view.follow(height),
        _ => {}
    }
}

/// A `Tree` view and a `Detail` showing its field tree drive `TreeState`
/// themselves; nothing else is measured here.
fn moved_a_tree(state: &mut State, motion: Motion, page: usize) -> bool {
    if state.overlay.is_some() {
        return false;
    }
    let tree = match state.view_mut() {
        View::Tree(view) => &mut view.state,
        View::Detail(view) if !view.raw => &mut view.tree,
        _ => return false,
    };
    match motion {
        Motion::Down => {
            tree.key_down();
        }
        Motion::Up => {
            tree.key_up();
        }
        Motion::Top => {
            tree.select_first();
        }
        Motion::Bottom => {
            tree.select_last();
        }
        Motion::PageDown => {
            tree.scroll_down(page);
        }
        Motion::PageUp => {
            tree.scroll_up(page);
        }
    }
    true
}

/// The length of the list under the cursor: a `Detail` showing raw JSON is as
/// long as that JSON, and a tree is not measured here at all.
fn list_length(state: &State) -> usize {
    match &state.overlay {
        Some(Overlay::Help(view)) => return view.rows.len(),
        Some(Overlay::Palette(view)) => return view.matches.len(),
        _ => {}
    }
    if let View::Builder(view) = state.view()
        && let Some(picker) = view.picker.as_ref()
    {
        return picker.matches.len();
    }
    match state.view() {
        View::Home(_) => crate::core::views::home::HomeView::groups().len(),
        View::AccountPicker(_) => state.accounts.len(),
        View::Collection(view) => view.visible().len(),
        View::Detail(view) => {
            if view.raw {
                view.raw_lines()
            } else {
                0
            }
        }
        View::EventLog(_) => state.log.entries().len(),
        View::Builder(view) => view.rows.len(),
        _ => 0,
    }
}

fn selected_mut(state: &mut State) -> Option<&mut usize> {
    if matches!(
        state.overlay,
        Some(Overlay::Palette(_)) | Some(Overlay::Help(_))
    ) {
        return match state.overlay.as_mut() {
            Some(Overlay::Palette(view)) => Some(&mut view.selected),
            Some(Overlay::Help(view)) => Some(&mut view.offset),
            _ => None,
        };
    }
    match state.view_mut() {
        View::Builder(view) => Some(match view.picker.as_mut() {
            Some(picker) => &mut picker.selected,
            None => &mut view.selected,
        }),
        View::Home(view) => Some(&mut view.selected),
        View::AccountPicker(view) => Some(&mut view.selected),
        View::Collection(view) => Some(&mut view.selected),
        View::Detail(view) => Some(&mut view.offset),
        View::EventLog(view) => Some(&mut view.selected),
        _ => None,
    }
}

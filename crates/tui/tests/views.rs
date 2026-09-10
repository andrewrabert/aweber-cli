//! Every view, at the floor and at a comfortable size, through `TestBackend`.

use aweber::catalog::{ArgValue, Args, Operation, ValueKind};
use aweber_tui::catalog::ConfirmationTier;
use aweber_tui::core::collection::{Continuation, EndReason};
use aweber_tui::core::event_log::LogEntry;
use aweber_tui::core::failure::{Failure, FailureKind};
use aweber_tui::core::state::{Account, ListRef, Overlay, Toast, View, Watch};
use aweber_tui::core::views;
use aweber_tui::core::{Generation, State};
use aweber_tui::harness::{Harness, fixed_now};
use aweber_tui::ports::{Authorization, SessionStatus};

/// The two sizes every view is drawn at.
const SIZES: [(u16, u16); 2] = [(80, 24), (120, 40)];

/// A state with the context resolved and nothing on the stack but home.
fn base() -> State {
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: None,
        name: Some("Acme".to_string()),
    });
    state.context.list = Some(ListRef {
        id: 7,
        uuid: None,
        name: Some("Weekly".to_string()),
        self_link: None,
    });
    state.session = SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    state
}

fn list_args() -> Args {
    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    args
}

fn subscribers() -> Vec<serde_json::Value> {
    vec![
        serde_json::json!({
            "id": 1,
            "email": "ada@example.com",
            "status": "subscribed",
            "subscribed_at": "2026-01-01T00:00:00+00:00",
        }),
        serde_json::json!({
            "id": 2,
            "email": "grace@example.com",
            "status": "unconfirmed",
            "subscribed_at": null,
        }),
    ]
}

/// A collection view over the subscribers of the ambient list.
fn collection() -> views::collection::CollectionView {
    let mut view = views::collection::CollectionView::opening(
        Operation::ListSubscribers,
        list_args(),
        "subscribers list".to_string(),
        Generation::default(),
    );
    view.rows = aweber_tui::core::collection::rows(
        subscribers(),
        aweber_tui::catalog::metadata(Operation::ListSubscribers).columns,
    );
    view.loading = false;
    view
}

fn detail() -> views::detail::DetailView {
    let mut view = views::detail::DetailView::opening(
        Operation::GetSubscriber,
        list_args(),
        "subscribers get".to_string(),
        Generation::default(),
    );
    view.show(
        serde_json::json!({
        "id": 1,
        "email": "ada@example.com",
        "status": "subscribed",
        "subscribed_at": "2026-01-01T00:00:00+00:00",
        "unsubscribed_at": null,
        "custom_fields": { "City": "Lovelace" },
            "tags": ["beta", "vip"],
        }),
        zone(),
    );
    view
}

/// The zone every snapshot reads timestamps in.
fn zone() -> chrono::FixedOffset {
    *fixed_now().offset()
}

fn forbidden() -> Failure {
    Failure {
        status: Some(403),
        method: "GET".to_string(),
        path: "/1.0/accounts/1/lists/7/subscribers".to_string(),
        message: "your token may not read subscribers".to_string(),
        kind: FailureKind::Http,
    }
}

/// Draws one state at both sizes and snapshots each.
fn snapshot(name: &str, build: impl Fn() -> State) {
    for (width, height) in SIZES {
        let state = build();
        let mut harness = Harness::new(width, height).with_state(state);
        insta::assert_snapshot!(format!("{name}-{width}x{height}"), harness.screen());
    }
}

#[test]
fn every_view_matches_its_snapshot() {
    snapshot("home", base);

    snapshot("account-picker", || {
        let mut state = base();
        state.accounts = vec![
            Account {
                id: 1,
                uuid: None,
                name: Some("Acme".to_string()),
            },
            Account {
                id: 2,
                uuid: None,
                name: None,
            },
        ];
        state.stack.push(View::AccountPicker(
            views::account_picker::AccountPickerView::default(),
        ));
        state
    });

    snapshot("collection-loading", || {
        let mut state = base();
        let mut view = collection();
        view.rows.clear();
        view.loading = true;
        state.stack.push(View::Collection(view));
        state
    });

    snapshot("collection-loaded", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        state
    });

    snapshot("collection-exhausted", || {
        let mut state = base();
        let mut view = collection();
        view.end = Some(EndReason::Exhausted);
        view.total = Some(2);
        state.stack.push(View::Collection(view));
        state
    });

    snapshot("collection-offset-cap", || {
        let mut state = base();
        let mut view = collection();
        view.end = Some(EndReason::OffsetCap { offset: 10_000 });
        view.cursor = Some(Continuation::Cursor {
            arg: "after".to_string(),
            value: "cursor".to_string(),
        });
        state.stack.push(View::Collection(view));
        state
    });

    snapshot("collection-unavailable", || {
        let mut state = base();
        let mut view = collection();
        view.rows.clear();
        view.unavailable = Some(forbidden());
        state.stack.push(View::Collection(view));
        state
    });

    snapshot("collection-empty", || {
        let mut state = base();
        let mut view = collection();
        view.rows.clear();
        view.end = Some(EndReason::Exhausted);
        state.stack.push(View::Collection(view));
        state
    });

    snapshot("collection-filtered", || {
        let mut state = base();
        let mut view = collection();
        view.filter = Some(aweber_tui::core::filter::Filter::apply("ada", &view.rows));
        state.stack.push(View::Collection(view));
        state
    });

    snapshot("detail-fields", || {
        let mut state = base();
        state.stack.push(View::Detail(detail()));
        state
    });

    snapshot("detail-expanded", || {
        let mut state = base();
        let mut view = detail();
        view.tree.open(vec!["custom_fields".to_string()]);
        view.tree.select(vec!["custom_fields".to_string()]);
        state.stack.push(View::Detail(view));
        state
    });

    snapshot("detail-enriched", || {
        let mut state = base();
        let mut view = detail();
        view.enrich(
            "message subjects",
            Ok(serde_json::json!([
                "0123456789abcdef01234567 — Welcome aboard"
            ])),
        );
        view.enrich("visualization", Ok(serde_json::json!({ "root": null })));
        state.stack.push(View::Detail(view));
        state
    });

    snapshot("detail-enrichment-unavailable", || {
        let mut state = base();
        let mut view = detail();
        view.enrich(
            "message subjects",
            Err("503 the message service is away".to_string()),
        );
        state.stack.push(View::Detail(view));
        state
    });

    snapshot("detail-raw", || {
        let mut state = base();
        let mut view = detail();
        view.raw = true;
        state.stack.push(View::Detail(view));
        state
    });

    snapshot("detail-unavailable", || {
        let mut state = base();
        let mut view = detail();
        view.document = None;
        view.unavailable = Some(forbidden());
        state.stack.push(View::Detail(view));
        state
    });

    snapshot("workflow-tree", || {
        let mut state = base();
        let document = serde_json::json!({
            "name": "Welcome",
            "unpublished": {
                "events": [{ "kind": "subscribe" }],
                "actions": [{ "kind": "send", "message_id": "m-1" }],
            },
        });
        state.stack.push(View::Tree(views::tree::TreeView {
            workflow: uuid::Uuid::nil(),
            title: "workflows tree".to_string(),
            generation: Generation::default(),
            state: tui_tree_widget::TreeState::default(),
            items: aweber_tui::view::fields::tree_items(&document, zone()),
            unavailable: None,
        }));
        state
    });

    snapshot("session-missing", || {
        let mut state = base();
        state.session = SessionStatus::Missing;
        state
            .stack
            .push(View::Session(views::session::SessionView::of(
                SessionStatus::Missing,
            )));
        state
    });

    snapshot("session-active", || {
        let mut state = base();
        state
            .stack
            .push(View::Session(views::session::SessionView::of(
                SessionStatus::Active {
                    account_id: 1,
                    expires_in_secs: 3600,
                },
            )));
        state
    });

    snapshot("session-login", || {
        let mut state = base();
        let mut view = views::session::SessionView::of(SessionStatus::Expired { account_id: 1 });
        view.authorization = Some(Authorization {
            url: "https://auth.aweber.com/oauth2/authorize?response_type=code".to_string(),
            verifier: aweber_tui::Secret::new("verifier".to_string()),
        });
        view.code = tui_input::Input::new("abc123".to_string());
        state.stack.push(View::Session(view));
        state
    });

    snapshot("raw-request", || {
        let mut state = base();
        let mut view = views::raw_request::RawRequestView {
            method: tui_input::Input::new("GET".to_string()),
            path: tui_input::Input::new("/1.0/accounts/1/lists/7".to_string()),
            ..Default::default()
        };
        view.focus = views::raw_request::RawFocus::Response;
        view.response = Some(aweber_tui::core::collection::Delivered {
            status: 200,
            document: Some(serde_json::json!({ "id": 7, "name": "Weekly" })),
            entries: Vec::new(),
            next: None,
            capped: None,
            total: None,
        });
        state.stack.push(View::RawRequest(view));
        state
    });

    snapshot("event-log", || {
        let mut state = base();
        state.log.append(LogEntry {
            at: fixed_now(),
            method: "GET".to_string(),
            path: "/1.0/accounts/1/lists".to_string(),
            status: Some(200),
            duration: Some(std::time::Duration::from_millis(42)),
            attempt: 1,
            waited: None,
            refreshed: false,
            detail: None,
            body: None,
        });
        state.log.append(LogEntry {
            at: fixed_now(),
            method: "GET".to_string(),
            path: "/1.0/accounts/1/lists".to_string(),
            status: Some(429),
            duration: None,
            attempt: 2,
            waited: Some(std::time::Duration::from_secs(2)),
            refreshed: false,
            detail: Some("waiting out 429".to_string()),
            body: None,
        });
        state
            .stack
            .push(View::EventLog(views::event_log::EventLogView::default()));
        state
    });

    snapshot("builder", || {
        let mut state = base();
        let mut view = views::builder::BuilderView::open(
            Operation::ListBroadcasts,
            state.prefill(Operation::ListBroadcasts),
            aweber_tui::core::Generation::default(),
        );
        view.reprice(state.scope());
        state.stack.push(View::Builder(view));
        state
    });

    snapshot("builder-picker", || {
        let mut state = base();
        let mut view = views::builder::BuilderView::open(
            Operation::ListBroadcasts,
            state.prefill(Operation::ListBroadcasts),
            aweber_tui::core::Generation::default(),
        );
        view.reprice(state.scope());
        let addable = view.addable();
        let candidates: Vec<String> = addable.iter().map(|spec| spec.long.clone()).collect();
        let mut picker =
            views::palette::PaletteView::open(views::palette::PaletteScope::Arguments(addable));
        picker.rescore(&candidates);
        view.picker = Some(picker);
        state.stack.push(View::Builder(view));
        state
    });

    snapshot("builder-candidates", || {
        let mut state = base();
        let mut view = views::builder::BuilderView::open(
            Operation::MoveSubscriber,
            state.prefill(Operation::MoveSubscriber),
            aweber_tui::core::Generation::default(),
        );
        view.add(
            Operation::MoveSubscriber
                .specs()
                .into_iter()
                .find(|spec| spec.name == "list-link")
                .expect("moving a subscriber names a list"),
        );
        let row = view.rows.len() - 1;
        if let views::builder::RowValue::Pick { candidates, .. } = &mut view.rows[row].value {
            *candidates = vec![
                views::builder::PickCandidate {
                    label: "Weekly".to_string(),
                    value: aweber::catalog::ArgValue::text(
                        "https://api.aweber.com/1.0/accounts/1/lists/7".to_string(),
                    ),
                },
                views::builder::PickCandidate {
                    label: "Monthly".to_string(),
                    value: aweber::catalog::ArgValue::text(
                        "https://api.aweber.com/1.0/accounts/1/lists/8".to_string(),
                    ),
                },
            ];
        }
        view.selected = row;
        view.reprice(state.scope());
        let labels: Vec<String> = view
            .candidates(row)
            .into_iter()
            .map(|candidate| candidate.label.clone())
            .collect();
        let mut picker =
            views::palette::PaletteView::open(views::palette::PaletteScope::Candidates { row });
        picker.rescore(&labels);
        view.picker = Some(picker);
        state.stack.push(View::Builder(view));
        state
    });

    snapshot("palette-watches", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        state.watches.push(Watch {
            id: aweber_tui::core::WatchId::default(),
            label: "GET /1.0/accounts/1/lists/7/broadcasts/9".to_string(),
            started_at: fixed_now(),
            operation: Operation::WaitBroadcast,
            args: list_args(),
        });
        let mut view = views::palette::PaletteView::open(views::palette::PaletteScope::Watches);
        let candidates: Vec<String> = state
            .watches
            .iter()
            .map(|watch| watch.label.clone())
            .collect();
        view.rescore(&candidates);
        state.overlay = Some(Overlay::Palette(view));
        state
    });

    snapshot("palette-everything", || {
        let mut state = base();
        let mut view = views::palette::PaletteView::open(views::palette::PaletteScope::Everything);
        let candidates: Vec<String> = aweber_tui::catalog::entries()
            .iter()
            .map(|entry| entry.label.clone())
            .collect();
        view.input = tui_input::Input::new("list".to_string());
        view.rescore(&candidates);
        state.overlay = Some(Overlay::Palette(view));
        state
    });

    snapshot("palette-for-selection", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        let kind = aweber_tui::catalog::EntityKind::Subscriber;
        let mut view =
            views::palette::PaletteView::open(views::palette::PaletteScope::ForSelection(kind));
        let candidates: Vec<String> = aweber_tui::catalog::for_selection(kind)
            .into_iter()
            .map(|entry| entry.label.clone())
            .collect();
        view.rescore(&candidates);
        state.overlay = Some(Overlay::Palette(view));
        state
    });

    snapshot("confirm-yes-no", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        state.overlay = Some(Overlay::Confirm(views::confirm::ConfirmView {
            operation: Operation::UnsubscribeSubscriber,
            args: list_args(),
            effect: "PATCH /1.0/accounts/1/lists/7/subscribers/1".to_string(),
            tier: ConfirmationTier::YesNo,
            typed: tui_input::Input::default(),
        }));
        state
    });

    snapshot("confirm-phrase", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        state.overlay = Some(Overlay::Confirm(views::confirm::ConfirmView {
            operation: Operation::DeleteSubscriber,
            args: list_args(),
            effect: "DELETE /1.0/accounts/1/lists/7/subscribers/1".to_string(),
            tier: ConfirmationTier::Phrase("delete subscriber"),
            typed: tui_input::Input::new("delete sub".to_string()),
        }));
        state
    });

    snapshot("error-modal", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        state.overlay = Some(Overlay::Error(Failure {
            status: Some(500),
            method: "GET".to_string(),
            path: "/1.0/accounts/1/lists/7/subscribers".to_string(),
            message: "the server is having a moment".to_string(),
            kind: FailureKind::Http,
        }));
        state
    });

    snapshot("help-overlay", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        state.overlay = Some(Overlay::Help(views::help::HelpView::open(
            aweber_tui::core::keys::Scope::Collection,
        )));
        state
    });

    snapshot("filter-prompt", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        state.overlay = Some(Overlay::Filter(aweber_tui::core::filter::FilterPrompt {
            input: tui_input::Input::new("ada".to_string()),
        }));
        state
    });

    snapshot("toast", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        state.toasts.push(Toast {
            at: fixed_now(),
            text: "DELETE /1.0/accounts/1/lists/7/subscribers/1 — 204".to_string(),
        });
        state
    });

    snapshot("watches-and-throttle", || {
        let mut state = base();
        state.stack.push(View::Collection(collection()));
        state.watches.push(Watch {
            id: aweber_tui::core::WatchId::default(),
            label: "GET /1.0/accounts/1/lists/7/broadcasts/9".to_string(),
            started_at: fixed_now(),
            operation: Operation::WaitBroadcast,
            args: list_args(),
        });
        state.inflight.began(
            aweber_tui::core::LogId::from(1),
            "GET",
            "/1.0/accounts/1/lists/7/broadcasts",
        );
        state.inflight.began(
            aweber_tui::core::LogId::from(2),
            "GET",
            "/1.0/accounts/1/lists/7/subscribers",
        );
        state.inflight.waiting(aweber_tui::core::LogId::from(2));
        state
    });
}

#[test]
fn a_terminal_below_the_floor_is_a_placeholder() {
    for (width, height) in [(79, 23), (1, 1)] {
        let mut harness = Harness::new(width, height).with_state(base());
        insta::assert_snapshot!(format!("too-small-{width}x{height}"), harness.screen());
    }
}

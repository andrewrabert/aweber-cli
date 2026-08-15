//! The application driven over `TestBackend` with every port stubbed.

use std::sync::{Arc, Mutex};

use ratatui::backend::TestBackend;
use ratatui::crossterm::event::{KeyEvent, MouseEvent};

use crate::core::{Action, State};
use crate::ports::{
    Authorization, BoxFuture, Clipboard, Clock, CopyError, CopyRoute, Editor, EditorError, Http,
    SessionPort, SessionStatus,
};
use crate::runtime::{App, Ports};

pub struct Harness {
    app: App,
    terminal: ratatui::Terminal<TestBackend>,
    clipboard: Arc<RecordingClipboard>,
    editor: Arc<ScriptedEditor>,
}

impl Harness {
    pub fn new(width: u16, height: u16) -> Harness {
        Harness::with_http(|_| Arc::new(DeadHttp), width, height)
    }

    /// Every request goes to the wiremock base URL through the real client,
    /// observed as the real application observes it.
    pub fn against(base_url: &str, width: u16, height: u16) -> Harness {
        let base_url = base_url.to_string();
        Harness::with_http(
            move |actions| {
                // Refreshable, so a `401` takes the refresh path the real
                // client takes; the refresh is refused, so what comes back is
                // `ApiError::Session`.
                let session = Arc::new(aweber::session::Session::new(
                    "harness-token".to_string(),
                    Arc::new(RefusedRefresh),
                ));
                let client = aweber::client::Client::with_session(&base_url, session)
                    .expect("the harness client is built")
                    .with_observer(Arc::new(crate::runtime::observer::LogObserver::new(
                        actions.clone(),
                    )));
                Arc::new(crate::runtime::http::SharedHttp::new(client))
            },
            width,
            height,
        )
    }

    fn with_http<F>(http: F, width: u16, height: u16) -> Harness
    where
        F: FnOnce(&tokio::sync::mpsc::UnboundedSender<Action>) -> Arc<dyn Http>,
    {
        let (actions, incoming) = tokio::sync::mpsc::unbounded_channel();
        let http = http(&actions);
        let clipboard = Arc::new(RecordingClipboard::default());
        let editor = Arc::new(ScriptedEditor::default());
        let ports = Ports {
            http,
            clock: Arc::new(FixedClock),
            clipboard: Arc::clone(&clipboard) as Arc<dyn Clipboard>,
            editor: Arc::clone(&editor) as Arc<dyn Editor>,
            session: Arc::new(StubSession),
        };
        let mut state = State::new(Some(crate::StoredAccount { id: 1, uuid: None }), false);
        state.size = (width, height);
        Harness {
            app: App::new(state, ports, actions, incoming),
            terminal: ratatui::Terminal::new(TestBackend::new(width, height))
                .expect("the test backend is built"),
            clipboard,
            editor,
        }
    }

    pub fn with_state(mut self, state: State) -> Harness {
        let size = self.app.state().size;
        let mut state = state;
        state.size = size;
        self.app.replace_state(state);
        self
    }

    pub fn with_editor_text(self, text: &str) -> Harness {
        *self
            .editor
            .text
            .lock()
            .expect("the scripted editor is not poisoned") = Some(text.to_string());
        self
    }

    pub fn key(&mut self, key: &str) -> &mut Harness {
        self.dispatch(Action::Key(chord(key)))
    }

    /// An action the keyboard cannot express, run through the same loop.
    pub fn dispatch(&mut self, action: Action) -> &mut Harness {
        let suspend = self.app.dispatch(action);
        self.suspend(suspend);
        self
    }

    /// The seed the scripted editor was last opened with.
    pub fn suspended_seed(&self) -> Option<String> {
        self.editor
            .seeds
            .lock()
            .expect("the scripted editor is not poisoned")
            .last()
            .cloned()
    }

    /// The loop holds the terminal for an edit; the harness does the same, then
    /// answers with whatever the scripted editor holds.
    fn suspend(&mut self, suspend: Option<crate::runtime::effects::Suspend>) {
        let Some(crate::runtime::effects::Suspend::Edit { purpose, seed, .. }) = suspend else {
            return;
        };
        self.editor
            .seeds
            .lock()
            .expect("the scripted editor is not poisoned")
            .push(seed);
        let text = self
            .editor
            .text
            .lock()
            .expect("the scripted editor is not poisoned")
            .clone()
            .ok_or(EditorError::NotConfigured);
        let _ = self.app.actions().send(Action::Edited { purpose, text });
    }

    pub fn keys(&mut self, keys: &str) -> &mut Harness {
        for key in keys.split_whitespace() {
            self.key(key);
        }
        self
    }

    pub fn mouse(&mut self, event: MouseEvent) -> &mut Harness {
        self.app.dispatch(Action::Mouse(event));
        self
    }

    /// A left click at a cell of the last drawn frame.
    pub fn click(&mut self, column: u16, row: u16) -> &mut Harness {
        self.screen();
        self.mouse(MouseEvent {
            kind: ratatui::crossterm::event::MouseEventKind::Down(
                ratatui::crossterm::event::MouseButton::Left,
            ),
            column,
            row,
            modifiers: ratatui::crossterm::event::KeyModifiers::NONE,
        })
    }

    /// A wheel notch at a cell of the last drawn frame.
    pub fn wheel(&mut self, column: u16, row: u16, down: bool) -> &mut Harness {
        self.screen();
        let kind = if down {
            ratatui::crossterm::event::MouseEventKind::ScrollDown
        } else {
            ratatui::crossterm::event::MouseEventKind::ScrollUp
        };
        self.mouse(MouseEvent {
            kind,
            column,
            row,
            modifiers: ratatui::crossterm::event::KeyModifiers::NONE,
        })
    }

    /// The requests and watches the interpreter still holds a token for.
    pub fn tracked(&self) -> usize {
        self.app.tracked()
    }

    pub async fn settle(&mut self) -> &mut Harness {
        /// An edit answers into the queue, so settling runs again; a handful of
        /// rounds is more than any flow needs.
        const ROUNDS: usize = 8;
        for _ in 0..ROUNDS {
            let suspend = self.app.settle().await;
            if suspend.is_none() {
                break;
            }
            self.suspend(suspend);
        }
        self
    }

    pub fn screen(&mut self) -> String {
        self.app
            .draw(&mut self.terminal)
            .expect("the test backend never fails to draw");
        let buffer = self.terminal.backend().buffer();
        (0..buffer.area.height)
            .map(|row| {
                (0..buffer.area.width)
                    .map(|column| buffer[(column, row)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn state(&self) -> &State {
        self.app.state()
    }

    pub fn copied(&self) -> Option<String> {
        self.clipboard
            .copied
            .lock()
            .expect("the recording clipboard is not poisoned")
            .last()
            .cloned()
    }

    /// The files a `w` really wrote, read back from where the toast says they
    /// are.
    pub fn written(&self) -> Vec<(std::path::PathBuf, String)> {
        const PREFIX: &str = "written to ";
        self.app
            .state()
            .toasts
            .iter()
            .filter_map(|toast| toast.text.strip_prefix(PREFIX))
            .map(std::path::PathBuf::from)
            .map(|path| {
                let text = std::fs::read_to_string(&path).unwrap_or_default();
                (path, text)
            })
            .collect()
    }
}

/// `2026-01-02T03:04:05+00:00`, fixed for every snapshot.
pub fn fixed_now() -> crate::core::Timestamp {
    chrono::DateTime::parse_from_rfc3339("2026-01-02T03:04:05+00:00")
        .expect("the fixed instant is well formed")
}

pub struct FixedClock;

impl Clock for FixedClock {
    fn now(&self) -> crate::core::Timestamp {
        fixed_now()
    }
}

#[derive(Default)]
pub struct ScriptedEditor {
    text: Mutex<Option<String>>,
    seeds: Mutex<Vec<String>>,
}

impl Editor for ScriptedEditor {
    fn edit(
        &self,
        seed: &str,
        _extension: &str,
    ) -> BoxFuture<'static, Result<String, EditorError>> {
        self.seeds
            .lock()
            .expect("the scripted editor is not poisoned")
            .push(seed.to_string());
        let text = self
            .text
            .lock()
            .expect("the scripted editor is not poisoned")
            .clone();
        Box::pin(async move { text.ok_or(EditorError::NotConfigured) })
    }
}

#[derive(Default)]
pub struct RecordingClipboard {
    copied: Mutex<Vec<String>>,
}

impl Clipboard for RecordingClipboard {
    fn copy(&self, text: &str) -> Result<CopyRoute, CopyError> {
        self.copied
            .lock()
            .expect("the recording clipboard is not poisoned")
            .push(text.to_string());
        Ok(CopyRoute::Native)
    }
}

pub struct StubSession;

impl SessionPort for StubSession {
    fn status(&self) -> BoxFuture<'static, SessionStatus> {
        Box::pin(async {
            SessionStatus::Active {
                account_id: 1,
                expires_in_secs: 3600,
            }
        })
    }

    fn authorization(&self) -> Authorization {
        Authorization {
            url: "https://auth.example.com/authorize".to_string(),
            verifier: crate::Secret::new("verifier".to_string()),
        }
    }

    fn login(
        &self,
        _code: String,
        _verifier: crate::Secret,
    ) -> BoxFuture<'static, Result<SessionStatus, String>> {
        Box::pin(async {
            Ok(SessionStatus::Active {
                account_id: 1,
                expires_in_secs: 3600,
            })
        })
    }

    fn logout(&self) -> BoxFuture<'static, Result<(), String>> {
        Box::pin(async { Ok(()) })
    }
}

/// A refresh grant the server will not honour.
struct RefusedRefresh;

impl aweber::session::TokenSource for RefusedRefresh {
    fn refresh(
        &self,
    ) -> aweber::session::BoxFuture<'_, Result<String, aweber::session::SessionError>> {
        Box::pin(async {
            Err(aweber::session::SessionError::Refused {
                status: 401,
                body: "the refresh grant is no longer good".to_string(),
            })
        })
    }
}

/// A port that answers nothing, for the views that ask for nothing.
struct DeadHttp;

impl Http for DeadHttp {
    fn send(
        &self,
        plan: aweber::catalog::RequestPlan,
    ) -> BoxFuture<'static, Result<aweber::client::PlanResponse, aweber::client::ApiError>> {
        let line = plan.line();
        Box::pin(async move {
            Err(aweber::client::ApiError::Http {
                status: 599,
                body: format!("the harness has no server for {line}"),
            })
        })
    }
}

/// The keys a harness script names, as chords.
fn chord(key: &str) -> KeyEvent {
    let chord: crate::core::keys::Chord = key.parse().expect("a test names a known key");
    KeyEvent::new(chord.code, chord.modifiers)
}

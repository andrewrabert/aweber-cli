//! The application, and the loop that drives it.

pub mod clipboard;
pub mod clock;
pub mod editor;
pub mod effects;
pub mod http;
pub mod input;
pub mod observer;
pub mod terminal;

use std::sync::Arc;

use crate::core::{Action, State};
use crate::view::Theme;

pub struct Ports {
    pub http: Arc<dyn crate::ports::Http>,
    pub clock: Arc<dyn crate::ports::Clock>,
    pub clipboard: Arc<dyn crate::ports::Clipboard>,
    pub editor: Arc<dyn crate::ports::Editor>,
    pub session: Arc<dyn crate::ports::SessionPort>,
}

/// State, ports, and the action queue, with no terminal of its own.
pub struct App {
    state: State,
    clock: Arc<dyn crate::ports::Clock>,
    editor: Arc<dyn crate::ports::Editor>,
    interpreter: effects::Interpreter,
    actions: tokio::sync::mpsc::UnboundedSender<Action>,
    incoming: tokio::sync::mpsc::UnboundedReceiver<Action>,
    theme: Theme,
}

impl App {
    pub fn new(
        state: State,
        ports: Ports,
        actions: tokio::sync::mpsc::UnboundedSender<Action>,
        incoming: tokio::sync::mpsc::UnboundedReceiver<Action>,
    ) -> App {
        let clock = Arc::clone(&ports.clock);
        let editor = Arc::clone(&ports.editor);
        App {
            state,
            clock,
            editor,
            interpreter: effects::Interpreter::new(ports, actions.clone()),
            actions,
            incoming,
            theme: Theme::detect(),
        }
    }

    /// Stamps the action from the clock, runs `update`, and interprets the
    /// effects.
    pub fn dispatch(&mut self, action: Action) -> Option<effects::Suspend> {
        let at = self.clock.now();
        let effects = crate::core::update(&mut self.state, at, action);
        let mut suspend = None;
        for effect in effects {
            if let Some(next) = self.interpreter.run(effect) {
                suspend = Some(next);
            }
        }
        suspend
    }

    pub fn draw<B>(&mut self, terminal: &mut ratatui::Terminal<B>) -> std::io::Result<()>
    where
        B: ratatui::backend::Backend,
        B::Error: Send + Sync + 'static,
    {
        let state = &mut self.state;
        let theme = &self.theme;
        let mut regions = None;
        terminal
            .draw(|frame| {
                regions = Some(crate::view::render(frame, state, theme));
            })
            .map_err(std::io::Error::other)?;
        if let Some(regions) = regions {
            self.state.regions = regions;
        }
        Ok(())
    }

    /// Drains queued actions and awaits outstanding work until nothing is
    /// pending.
    pub async fn settle(&mut self) -> Option<effects::Suspend> {
        /// Long enough for a spawned request to answer, short enough to end a
        /// test that has nothing outstanding.
        const QUIET: std::time::Duration = std::time::Duration::from_millis(50);
        const LONGEST: std::time::Duration = std::time::Duration::from_secs(5);
        let mut suspend = None;
        loop {
            let wait = if self.state.inflight.outstanding() > 0 {
                LONGEST
            } else {
                QUIET
            };
            match tokio::time::timeout(wait, self.incoming.recv()).await {
                Ok(Some(action)) => {
                    if let Some(next) = self.dispatch(action) {
                        suspend = Some(next);
                    }
                }
                Ok(None) => return suspend,
                Err(_) => return suspend,
            }
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    /// The requests and watches the interpreter still holds a token for.
    pub fn tracked(&self) -> usize {
        self.interpreter.tracked()
    }

    pub fn actions(&self) -> tokio::sync::mpsc::UnboundedSender<Action> {
        self.actions.clone()
    }

    /// The editor an `Edit` effect suspends into.
    pub(crate) fn editor(&self) -> Arc<dyn crate::ports::Editor> {
        Arc::clone(&self.editor)
    }

    /// The harness seeds a view by handing over the whole state.
    pub(crate) fn replace_state(&mut self, state: State) {
        self.state = state;
    }

    pub(crate) fn shutdown(&mut self) {
        self.interpreter.shutdown();
    }
}

/// `tokio::select!` over the crossterm event stream, a 4 Hz tick, a 60 Hz frame,
/// and the action queue. The reader is parked for the duration of an `$EDITOR`
/// suspend, so the editor alone holds the keyboard.
pub async fn drive(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> anyhow::Result<()> {
    let reader = input::Reader::spawn(app.actions());
    let mut ticks = tokio::time::interval(std::time::Duration::from_millis(250));
    let mut frames = tokio::time::interval(std::time::Duration::from_millis(16));
    app.draw(terminal)?;

    loop {
        let mut suspend = None;
        tokio::select! {
            _ = ticks.tick() => {
                suspend = app.dispatch(Action::Tick);
            }
            _ = frames.tick() => {
                app.draw(terminal)?;
            }
            Some(action) = app.incoming.recv() => {
                suspend = app.dispatch(action);
            }
        }
        match suspend {
            Some(effects::Suspend::Quit) => break,
            Some(effects::Suspend::Edit {
                purpose,
                seed,
                extension,
            }) => {
                let editor = app.editor();
                let text = terminal::suspended(terminal, &reader, async move {
                    editor.edit(&seed, extension).await
                })
                .await?;
                if let Some(effects::Suspend::Quit) = app.dispatch(Action::Edited { purpose, text })
                {
                    break;
                }
            }
            None => {}
        }
        if app.state().quit {
            break;
        }
        app.draw(terminal)?;
    }
    reader.cancel();
    app.shutdown();
    Ok(())
}

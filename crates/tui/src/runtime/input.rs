//! Terminal events, read on a thread that can be parked while something else
//! owns the terminal.

use std::sync::{Arc, Condvar, Mutex};

use ratatui::crossterm::event::{Event, KeyEventKind};

use crate::core::Action;

/// How long a poll waits before the loop looks at the park flag again.
const IDLE: std::time::Duration = std::time::Duration::from_millis(100);

/// Where terminal events come from, so a test can hold the reader at a poll.
pub trait EventSource: Send {
    /// Waits up to `timeout` for one event, and answers with what arrived.
    fn poll(&mut self, timeout: std::time::Duration) -> std::io::Result<Option<Event>>;
}

/// The terminal itself, polled and read through crossterm.
pub struct TerminalEvents;

impl EventSource for TerminalEvents {
    fn poll(&mut self, timeout: std::time::Duration) -> std::io::Result<Option<Event>> {
        match ratatui::crossterm::event::poll(timeout)? {
            true => ratatui::crossterm::event::read().map(Some),
            false => Ok(None),
        }
    }
}

struct Shared {
    /// A read runs only while this is held and clear.
    parked: Mutex<bool>,
    resumed: Condvar,
    token: tokio_util::sync::CancellationToken,
}

/// The terminal reader, and the means to park it.
pub struct Reader {
    shared: Arc<Shared>,
}

impl Reader {
    pub fn spawn(actions: tokio::sync::mpsc::UnboundedSender<Action>) -> Reader {
        Reader::with_source(Box::new(TerminalEvents), actions)
    }

    pub fn with_source(
        mut source: Box<dyn EventSource>,
        actions: tokio::sync::mpsc::UnboundedSender<Action>,
    ) -> Reader {
        let shared = Arc::new(Shared {
            parked: Mutex::new(false),
            resumed: Condvar::new(),
            token: tokio_util::sync::CancellationToken::new(),
        });
        let worker = Arc::clone(&shared);
        std::thread::spawn(move || read_loop(&worker, source.as_mut(), &actions));
        Reader { shared }
    }

    /// Parks the reader and returns only once no read is in progress and no
    /// further read can begin.
    pub fn pause(&self) -> Paused<'_> {
        *self
            .shared
            .parked
            .lock()
            .expect("the park flag is not poisoned") = true;
        Paused { reader: self }
    }

    pub fn parked(&self) -> bool {
        *self
            .shared
            .parked
            .lock()
            .expect("the park flag is not poisoned")
    }

    pub fn cancel(&self) {
        self.shared.token.cancel();
        self.shared.resumed.notify_all();
    }
}

/// Resumes the reader when it is dropped.
pub struct Paused<'a> {
    reader: &'a Reader,
}

impl Drop for Paused<'_> {
    fn drop(&mut self) {
        *self
            .reader
            .shared
            .parked
            .lock()
            .expect("the park flag is not poisoned") = false;
        self.reader.shared.resumed.notify_all();
    }
}

fn read_loop(
    shared: &Shared,
    source: &mut dyn EventSource,
    actions: &tokio::sync::mpsc::UnboundedSender<Action>,
) {
    while !shared.token.is_cancelled() {
        let event = {
            let mut parked = shared.parked.lock().expect("the park flag is not poisoned");
            while *parked && !shared.token.is_cancelled() {
                let (next, _) = shared
                    .resumed
                    .wait_timeout(parked, IDLE)
                    .expect("the park flag is not poisoned");
                parked = next;
            }
            if shared.token.is_cancelled() {
                return;
            }
            match source.poll(IDLE) {
                Ok(event) => event,
                Err(_) => return,
            }
        };
        if shared.token.is_cancelled() {
            continue;
        }
        if let Some(action) = event.and_then(action_of)
            && actions.send(action).is_err()
        {
            return;
        }
    }
}

/// A terminal event as an action, or nothing when the event says nothing.
fn action_of(event: Event) -> Option<Action> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => Some(Action::Key(key)),
        Event::Mouse(mouse) => Some(Action::Mouse(mouse)),
        Event::Resize(width, height) => Some(Action::Resize { width, height }),
        _ => None,
    }
}

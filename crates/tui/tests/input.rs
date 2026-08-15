//! The reader parks while something else owns the terminal.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use aweber_tui::runtime::input::{EventSource, Reader};

/// A paused reader parks, sends nothing while parked, and reads again once the
/// pause is dropped.
#[test]
fn a_paused_reader_reads_nothing() {
    let (actions, mut incoming) = tokio::sync::mpsc::unbounded_channel();
    let reader = Reader::spawn(actions);
    assert!(!reader.parked(), "a reader starts reading");

    {
        let _paused = reader.pause();
        assert!(reader.parked(), "a pause parks the reader");
        std::thread::sleep(std::time::Duration::from_millis(150));
        assert!(
            incoming.try_recv().is_err(),
            "a parked reader sends nothing"
        );
    }

    assert!(!reader.parked(), "dropping the pause resumes the reader");
    reader.cancel();
}

/// A source that records every poll that begins while the reader is parked, and
/// answers each poll with an event.
struct WatchfulSource {
    parked: Arc<AtomicBool>,
    stolen: Arc<AtomicUsize>,
}

impl EventSource for WatchfulSource {
    fn poll(
        &mut self,
        _timeout: std::time::Duration,
    ) -> std::io::Result<Option<ratatui::crossterm::event::Event>> {
        if self.parked.load(Ordering::SeqCst) {
            self.stolen.fetch_add(1, Ordering::SeqCst);
        }
        Ok(Some(ratatui::crossterm::event::Event::Key(
            ratatui::crossterm::event::KeyEvent::new(
                ratatui::crossterm::event::KeyCode::Char('x'),
                ratatui::crossterm::event::KeyModifiers::NONE,
            ),
        )))
    }
}

/// A pause taken and dropped repeatedly against a source that is always ready
/// leaves no poll ever having begun while the reader was parked, so no keystroke
/// meant for `$EDITOR` is read by the loop.
#[test]
fn no_read_begins_after_a_pause_returns() {
    let parked = Arc::new(AtomicBool::new(false));
    let stolen = Arc::new(AtomicUsize::new(0));
    let (actions, mut incoming) = tokio::sync::mpsc::unbounded_channel();
    let reader = Reader::with_source(
        Box::new(WatchfulSource {
            parked: Arc::clone(&parked),
            stolen: Arc::clone(&stolen),
        }),
        actions,
    );

    for _ in 0..200 {
        let paused = reader.pause();
        parked.store(true, Ordering::SeqCst);
        std::thread::sleep(std::time::Duration::from_micros(50));
        parked.store(false, Ordering::SeqCst);
        drop(paused);
        std::thread::sleep(std::time::Duration::from_micros(50));
    }
    reader.cancel();
    while incoming.try_recv().is_ok() {}

    assert_eq!(
        stolen.load(Ordering::SeqCst),
        0,
        "a read began while the reader was parked"
    );
}

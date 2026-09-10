//! The terminal, and the guarantee that it is given back.

use std::io::Write as _;

use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

/// Alternate screen, raw mode, mouse capture, and a panic hook that restores all
/// three.
pub fn init() -> std::io::Result<ratatui::DefaultTerminal> {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore();
        hook(info);
    }));
    enable_raw_mode()?;
    let mut out = std::io::stdout();
    execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = ratatui::backend::CrosstermBackend::new(out);
    ratatui::Terminal::new(backend)
}

pub fn restore() {
    let mut out = std::io::stdout();
    let _ = execute!(out, DisableMouseCapture, LeaveAlternateScreen);
    let _ = disable_raw_mode();
    let _ = out.flush();
}

/// Leaves the alternate screen and parks the reader for the duration of the
/// future, so the work alone holds the keyboard.
pub async fn suspended<T>(
    terminal: &mut ratatui::DefaultTerminal,
    reader: &super::input::Reader,
    work: impl std::future::Future<Output = T>,
) -> std::io::Result<T> {
    let parked = reader.pause();
    let mut out = std::io::stdout();
    execute!(out, DisableMouseCapture, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    let outcome = work.await;
    enable_raw_mode()?;
    execute!(out, EnterAlternateScreen, EnableMouseCapture)?;
    terminal.clear()?;
    drop(parked);
    Ok(outcome)
}

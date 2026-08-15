//! Below the floor the whole frame says so, and nothing else is drawn.

use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, theme: &Theme) {
    let text = format!(
        "the terminal is too small — {}x{} is the floor",
        crate::view::chrome::MIN_WIDTH,
        crate::view::chrome::MIN_HEIGHT
    );
    frame.render_widget(Paragraph::new(text).style(theme.warn), area);
}

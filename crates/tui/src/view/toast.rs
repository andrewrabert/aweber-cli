//! Toasts, newest last, along the bottom of the body.

use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

use crate::core::state::Toast;
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, toasts: &[Toast], theme: &Theme) {
    if toasts.is_empty() || area.height == 0 {
        return;
    }
    let shown = toasts.len().min(area.height as usize);
    let lines: Vec<Line<'_>> = toasts[toasts.len() - shown..]
        .iter()
        .map(|toast| Line::styled(toast.text.clone(), theme.accent))
        .collect();
    let height = lines.len() as u16;
    let area = Rect {
        x: area.x,
        y: area.y + area.height - height,
        width: area.width,
        height,
    };
    frame.render_widget(ratatui::widgets::Clear, area);
    frame.render_widget(Paragraph::new(lines), area);
}

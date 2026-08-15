//! The bindings in scope.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::core::views::help::HelpView;
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, view: &HelpView, theme: &Theme) {
    let area = crate::view::centered(area, 60, area.height.saturating_sub(4));
    let lines: Vec<Line<'static>> = view
        .rows
        .iter()
        .map(|row| {
            Line::from(vec![
                Span::styled(format!("{:<16}", row.keys), theme.accent),
                Span::styled(row.label, theme.value),
            ])
        })
        .collect();
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines).scroll((view.offset as u16, 0)).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" keys ")
                .border_style(theme.label),
        ),
        area,
    );
}

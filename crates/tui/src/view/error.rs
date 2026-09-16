//! The blocking error modal.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::core::failure::Failure;
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, failure: &Failure, theme: &Theme) {
    let area = crate::view::centered(area, 70, 9);
    let status = match failure.status {
        Some(status) => status.to_string(),
        None => "no response".to_string(),
    };
    let lines = vec![
        Line::from(vec![
            Span::styled("status  ", theme.label),
            Span::styled(status, theme.error),
        ]),
        Line::from(vec![
            Span::styled("request  ", theme.label),
            Span::styled(format!("{} {}", failure.method, failure.path), theme.value),
        ]),
        Line::from(vec![
            Span::styled("message  ", theme.label),
            Span::styled(failure.message.clone(), theme.value),
        ]),
    ];
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" the request failed ")
                    .border_style(theme.error),
            ),
        area,
    );
}

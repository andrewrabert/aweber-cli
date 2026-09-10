//! The confirmation, naming the exact HTTP effect it will cause.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::catalog::ConfirmationTier;
use crate::core::views::confirm::ConfirmView;
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, view: &ConfirmView, theme: &Theme) {
    let area = crate::view::centered(area, 66, 8);
    frame.render_widget(Clear, area);
    let mut lines = vec![
        Line::from(Span::styled(
            crate::catalog::entry(view.operation).label.clone(),
            theme.label,
        )),
        Line::from(Span::styled(view.effect.clone(), theme.value)),
        Line::default(),
    ];
    match view.tier {
        ConfirmationTier::Phrase(phrase) => {
            lines.push(Line::from(Span::styled(
                format!("type '{phrase}' to confirm"),
                theme.warn,
            )));
            lines.push(Line::from(vec![
                Span::styled("> ", theme.accent),
                Span::styled(view.typed.value().to_string(), theme.value),
            ]));
        }
        _ => lines.push(Line::from(Span::styled(
            "y to confirm, n to cancel",
            theme.warn,
        ))),
    }
    frame.render_widget(
        Paragraph::new(lines)
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" confirm ")
                    .border_style(theme.warn),
            ),
        area,
    );
}

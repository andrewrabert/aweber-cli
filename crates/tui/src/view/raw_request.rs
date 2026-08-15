//! An arbitrary method and path, and whatever came back.

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::core::views::raw_request::{RawFocus, RawRequestView};
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, view: &RawRequestView, theme: &Theme) {
    let [fields, response] =
        Layout::vertical([Constraint::Length(4), Constraint::Min(1)]).areas(area);

    let field = |label: &'static str, value: String, focused: bool| {
        Line::from(vec![
            Span::styled(if focused { "> " } else { "  " }, theme.accent),
            Span::styled(format!("{label:<8}"), theme.label),
            Span::styled(value, theme.value),
        ])
    };
    frame.render_widget(
        Paragraph::new(vec![
            field(
                "method",
                view.method.value().to_string(),
                view.focus == RawFocus::Method,
            ),
            field(
                "path",
                view.path.value().to_string(),
                view.focus == RawFocus::Path,
            ),
            field(
                "body",
                view.body.clone().unwrap_or_default(),
                view.focus == RawFocus::Body,
            ),
        ]),
        fields,
    );

    if let Some(failure) = &view.unavailable {
        frame.render_widget(
            Paragraph::new(failure.text())
                .style(theme.error)
                .wrap(ratatui::widgets::Wrap { trim: true }),
            response,
        );
        return;
    }
    match view.response.as_ref() {
        Some(delivered) => {
            let mut lines = vec![Line::from(Span::styled(
                format!("{}", delivered.status),
                theme.accent,
            ))];
            if let Some(document) = &delivered.document {
                // The Raw Request view shows whatever came back, so it is the
                // one view whose body must be scrubbed before it is drawn.
                let scrubbed: serde_json::Value =
                    serde_json::from_str(&crate::core::redact::scrub(&document.to_string()))
                        .unwrap_or_else(|_| document.clone());
                lines.extend(crate::view::json::lines(&scrubbed, theme));
            }
            frame.render_widget(Paragraph::new(lines), response);
        }
        None => frame.render_widget(
            Paragraph::new("Tab changes the field, Enter sends").style(theme.muted),
            response,
        ),
    }
}

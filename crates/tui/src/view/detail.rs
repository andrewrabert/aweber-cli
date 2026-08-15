//! One document: a field tree by default, raw JSON on `J`, the enrichment
//! rows under both.

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::core::views::detail::DetailView;
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, view: &mut DetailView, theme: &Theme) {
    if let Some(failure) = &view.unavailable {
        frame.render_widget(
            Paragraph::new(failure.text())
                .style(theme.error)
                .wrap(ratatui::widgets::Wrap { trim: true }),
            area,
        );
        return;
    }
    let Some(document) = view.document.clone() else {
        frame.render_widget(Paragraph::new("loading…").style(theme.muted), area);
        return;
    };
    let [body, enrichment] = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(view.enrichment.len() as u16),
    ])
    .areas(area);
    if view.raw {
        let lines = crate::view::json::lines(&document, theme);
        let offset = u16::try_from(view.offset).unwrap_or(u16::MAX);
        frame.render_widget(Paragraph::new(lines).scroll((offset, 0)), body);
    } else {
        let tree = tui_tree_widget::Tree::new(&view.items)
            .expect("the tree items are uniquely named")
            .highlight_style(theme.selected);
        frame.render_stateful_widget(tree, body, &mut view.tree);
    }
    let enriched: Vec<Line<'static>> = view
        .enrichment
        .iter()
        .map(|entry| match &entry.value {
            Ok(value) => Line::from(vec![
                Span::styled(format!("{}  ", entry.label), theme.label),
                Span::styled(summary(value), theme.value),
            ]),
            Err(reason) => Line::from(vec![
                Span::styled(format!("{}  ", entry.label), theme.label),
                Span::styled(format!("unavailable — {reason}"), theme.warn),
            ]),
        })
        .collect();
    frame.render_widget(Paragraph::new(enriched), enrichment);
}

/// An enrichment reads as one line, whatever shape its value has.
fn summary(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(text) => text.clone(),
        serde_json::Value::Array(entries) => entries
            .iter()
            .map(summary)
            .collect::<Vec<String>>()
            .join(", "),
        other => other.to_string(),
    }
}

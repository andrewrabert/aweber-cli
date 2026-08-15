//! The filter builder: one row per argument, and the request they describe.

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState, Paragraph};

use crate::core::views::builder::{BuilderRow, BuilderView, RowValue};
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, view: &BuilderView, theme: &Theme) {
    let [rows, preview] = Layout::vertical([Constraint::Min(1), Constraint::Length(2)]).areas(area);

    let items: Vec<ListItem<'static>> = view.rows.iter().map(|row| item(row, theme)).collect();
    let mut state = ListState::default().with_selected(Some(view.selected));
    frame.render_stateful_widget(
        List::new(items).highlight_style(theme.selected),
        rows,
        &mut state,
    );

    frame.render_widget(
        Paragraph::new(match &view.preview {
            Ok(plan) => Line::from(vec![
                Span::styled("sends ", theme.label),
                Span::styled(plan.line(), theme.value),
            ]),
            Err(error) => Line::from(vec![
                Span::styled("incomplete ", theme.label),
                Span::styled(error.to_string(), theme.warn),
            ]),
        })
        .wrap(ratatui::widgets::Wrap { trim: true }),
        preview,
    );

    if let Some(picker) = view.picker.as_ref() {
        crate::view::palette::render(frame, area, picker, theme);
    }
}

fn item(row: &BuilderRow, theme: &Theme) -> ListItem<'static> {
    ListItem::new(Line::from(vec![
        Span::styled(format!("{:<28}", row.spec.long), theme.label),
        Span::styled(shown(row), theme.value),
        Span::styled(if row.removable { "" } else { "  required" }, theme.muted),
    ]))
}

/// What a row's value reads as, whatever kind it is.
fn shown(row: &BuilderRow) -> String {
    match &row.value {
        RowValue::Text(input) | RowValue::Number(input) | RowValue::Date(input) => {
            input.value().to_string()
        }
        RowValue::Bool(set) => set.to_string(),
        RowValue::Enum { options, chosen } => options
            .get(*chosen)
            .cloned()
            .unwrap_or_else(|| "—".to_string()),
        RowValue::Pick { query, chosen, .. } => match chosen {
            Some(candidate) => candidate.label.clone(),
            None => query.value().to_string(),
        },
        RowValue::Json { text } => match text {
            Some(text) => format!("{} bytes of JSON", text.len()),
            None => "entered through $EDITOR".to_string(),
        },
    }
}

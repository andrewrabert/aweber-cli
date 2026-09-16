//! A collection: a header of columns, the loaded rows, and why it ended.

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState, Paragraph};

use crate::core::collection::EndReason;
use crate::core::views::collection::CollectionView;
use crate::view::Theme;

/// Draws the view and says which part of the area holds rows.
pub fn render(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    view: &CollectionView,
    theme: &Theme,
) -> Rect {
    let [heading, rows, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .areas(area);

    let columns = crate::catalog::metadata(view.operation).columns;
    let widths = widths(columns.len(), rows.width);
    frame.render_widget(
        Paragraph::new(Line::from(
            columns
                .iter()
                .zip(&widths)
                .map(|(column, width)| Span::styled(pad(column.label, *width), theme.label))
                .collect::<Vec<Span<'_>>>(),
        )),
        heading,
    );

    if let Some(failure) = &view.unavailable {
        frame.render_widget(
            Paragraph::new(failure.text())
                .style(theme.error)
                .wrap(ratatui::widgets::Wrap { trim: true }),
            rows,
        );
        return rows;
    }

    let visible = view.visible();
    if visible.is_empty() {
        let text = if view.loading {
            "loading…".to_string()
        } else {
            "no entries".to_string()
        };
        frame.render_widget(Paragraph::new(text).style(theme.muted), rows);
    } else {
        let items: Vec<ListItem<'_>> = visible
            .iter()
            .map(|row| {
                ListItem::new(Line::from(
                    row.cells
                        .iter()
                        .zip(&widths)
                        .map(|(cell, width)| Span::styled(pad(cell, *width), theme.value))
                        .collect::<Vec<Span<'_>>>(),
                ))
            })
            .collect();
        let mut state = ListState::default()
            .with_offset(view.offset)
            .with_selected(Some(view.selected));
        frame.render_stateful_widget(
            List::new(items).highlight_style(theme.selected),
            rows,
            &mut state,
        );
    }

    frame.render_widget(Paragraph::new(status(view, theme)), footer);
    rows
}

/// The count, the filter, the end reason, and the throbber of a fetch.
fn status(view: &CollectionView, theme: &Theme) -> Line<'static> {
    let mut spans = vec![Span::styled(view.count_label(), theme.muted)];
    if let Some(filter) = &view.filter {
        spans.push(Span::styled(
            format!("  {}", filter.label(view.rows.len())),
            theme.accent,
        ));
    }
    if view.loading {
        spans.push(Span::styled("  fetching…", theme.warn));
    }
    if let Some(end) = &view.end {
        spans.push(Span::styled(format!("  {}", ended(end)), theme.muted));
    }
    Line::from(spans)
}

fn ended(reason: &EndReason) -> String {
    match reason {
        EndReason::Exhausted => "end of the collection".to_string(),
        EndReason::OffsetCap { offset } => {
            format!("the API refuses a cursor past offset {offset}")
        }
        EndReason::Unavailable {
            status,
            method,
            path,
        } => format!("stopped: {method} {path} answered {status}"),
    }
}

/// Columns share the width evenly, with the first taking the remainder.
fn widths(columns: usize, width: u16) -> Vec<usize> {
    if columns == 0 {
        return vec![width as usize];
    }
    let each = (width as usize).saturating_sub(columns) / columns.max(1);
    let mut widths = vec![each.max(4); columns];
    if let Some(first) = widths.first_mut() {
        *first += (width as usize).saturating_sub(each * columns + columns);
    }
    widths
}

/// A cell is padded to its column, and cut where it would overrun it.
fn pad(text: &str, width: usize) -> String {
    let mut out: String = text.chars().take(width).collect();
    while out.chars().count() < width {
        out.push(' ');
    }
    out.push(' ');
    out
}

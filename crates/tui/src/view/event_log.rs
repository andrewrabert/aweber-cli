//! Every attempt of every request, oldest first.

use ratatui::layout::Rect;
use ratatui::widgets::{List, ListItem, ListState, Paragraph};

use crate::core::event_log::EventLog;
use crate::core::views::event_log::EventLogView;
use crate::view::Theme;

pub fn render(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    view: &EventLogView,
    log: &EventLog,
    theme: &Theme,
) {
    if log.entries().is_empty() {
        frame.render_widget(Paragraph::new("no requests yet").style(theme.muted), area);
        return;
    }
    let items: Vec<ListItem<'static>> = (0..log.entries().len())
        .map(|index| {
            let style = match log.entries()[index].status {
                Some(status) if status >= 400 => theme.error,
                Some(_) => theme.value,
                None => theme.muted,
            };
            ListItem::new(log.line(index)).style(style)
        })
        .collect();
    let mut state = ListState::default()
        .with_offset(view.offset)
        .with_selected(Some(view.selected));
    frame.render_stateful_widget(
        List::new(items).highlight_style(theme.selected),
        area,
        &mut state,
    );
}

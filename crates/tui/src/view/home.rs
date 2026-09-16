//! The entity groups, one per row.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState};

use crate::core::views::home::HomeView;
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, view: &HomeView, theme: &Theme) {
    let items: Vec<ListItem<'_>> = HomeView::groups()
        .iter()
        .map(|group| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:<20}", group.name), theme.value),
                Span::styled(group.about, theme.muted),
            ]))
        })
        .collect();
    let list = List::new(items).highlight_style(theme.selected);
    let mut state = ListState::default().with_selected(Some(view.selected));
    frame.render_stateful_widget(list, area, &mut state);
}

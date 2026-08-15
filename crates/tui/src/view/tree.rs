//! A workflow's ruleset, as a tree.

use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;

use crate::core::views::tree::TreeView;
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, view: &mut TreeView, theme: &Theme) {
    if let Some(failure) = &view.unavailable {
        frame.render_widget(
            Paragraph::new(failure.text())
                .style(theme.error)
                .wrap(ratatui::widgets::Wrap { trim: true }),
            area,
        );
        return;
    }
    if view.items.is_empty() {
        frame.render_widget(Paragraph::new("loading…").style(theme.muted), area);
        return;
    }
    let tree = tui_tree_widget::Tree::new(&view.items)
        .expect("the tree items are uniquely named")
        .highlight_style(theme.selected);
    frame.render_stateful_widget(tree, area, &mut view.state);
}

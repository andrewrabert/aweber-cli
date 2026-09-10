//! The fuzzy palette, with the matched characters marked.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

use crate::core::views::palette::{Match, PaletteScope, PaletteView};
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, view: &PaletteView, theme: &Theme) {
    let area = crate::view::centered(area, 64, area.height.saturating_sub(2));
    frame.render_widget(Clear, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", title(&view.scope)))
        .border_style(theme.accent);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let [prompt, rows] = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Length(1),
        ratatui::layout::Constraint::Min(1),
    ])
    .areas(inner);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("> ", theme.accent),
            Span::styled(view.input.value().to_string(), theme.value),
        ])),
        prompt,
    );

    let items: Vec<ListItem<'static>> = view
        .matches
        .iter()
        .map(|entry| ListItem::new(marked(entry, theme)))
        .collect();
    let mut state = ListState::default().with_selected(Some(view.selected));
    frame.render_stateful_widget(
        List::new(items).highlight_style(theme.selected),
        rows,
        &mut state,
    );
}

fn title(scope: &PaletteScope) -> &'static str {
    match scope {
        PaletteScope::Everything => "every operation",
        PaletteScope::ForSelection(_) => "operations for this selection",
        PaletteScope::Session => "session operations",
        PaletteScope::Ancestors => "go to",
        PaletteScope::Watches => "stop a watch",
        PaletteScope::Arguments(_) => "add an argument",
        PaletteScope::Candidates { .. } => "choose one",
    }
}

/// The characters the query matched carry the accent.
fn marked(entry: &Match, theme: &Theme) -> Line<'static> {
    entry
        .label
        .chars()
        .enumerate()
        .map(|(index, character)| {
            let style = if entry.positions.contains(&(index as u32)) {
                theme.accent
            } else {
                theme.value
            };
            Span::styled(character.to_string(), style)
        })
        .collect::<Vec<Span<'static>>>()
        .into()
}

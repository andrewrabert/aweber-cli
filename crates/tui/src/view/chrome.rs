//! The header, the status bar, and the footer every view sits between.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::core::State;
use crate::view::Theme;

pub const MIN_WIDTH: u16 = 80;
pub const MIN_HEIGHT: u16 = 24;

/// The breadcrumb of the stack and the ambient `{account, list}` context.
pub fn header(frame: &mut ratatui::Frame<'_>, area: Rect, state: &State, theme: &Theme) {
    let breadcrumb = state.breadcrumb().join(" › ");
    let mut spans = vec![Span::styled(breadcrumb, theme.accent)];
    let account = match &state.context.account {
        Some(account) => match &account.name {
            Some(name) => format!("{name} ({})", account.id),
            None => account.id.to_string(),
        },
        None => "no account".to_string(),
    };
    spans.push(Span::styled("   account ", theme.label));
    spans.push(Span::styled(account, theme.value));
    if let Some(list) = &state.context.list {
        let name = list.name.clone().unwrap_or_else(|| list.id.to_string());
        spans.push(Span::styled("   list ", theme.label));
        spans.push(Span::styled(name, theme.value));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

/// Active watches with elapsed time, in-flight count, and the throttled marker.
pub fn status_bar(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    state: &State,
    theme: &Theme,
) -> Vec<(crate::core::WatchId, Rect)> {
    let mut spans = Vec::new();
    let mut regions = Vec::new();
    let mut column = area.x;
    for watch in &state.watches {
        let text = format!("⏳ {} ", watch.label);
        let width = text.chars().count() as u16;
        regions.push((
            watch.id,
            Rect {
                x: column,
                y: area.y,
                width: width.min(area.width.saturating_sub(column - area.x)),
                height: 1,
            },
        ));
        column = column.saturating_add(width);
        spans.push(Span::styled(text, theme.warn));
    }
    if state.inflight.outstanding() > 0 {
        spans.push(Span::styled(
            format!("{} in flight ", state.inflight.outstanding()),
            theme.muted,
        ));
    }
    if state.inflight.throttled() {
        spans.push(Span::styled("throttled ", theme.warn));
    }
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
    regions
}

pub fn footer(frame: &mut ratatui::Frame<'_>, area: Rect, state: &State, theme: &Theme) {
    let mut hints = crate::core::keys::footer_hints(state.scope_of_keys());
    hints.push("Ctrl-p palette".to_owned());
    let spans = hints
        .into_iter()
        .flat_map(|hint| {
            [
                Span::styled(hint, theme.label),
                Span::styled("  ", theme.base),
            ]
        })
        .collect::<Vec<Span<'_>>>();
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}

/// The spinner of a fetch in progress, advanced by the tick it is given so the
/// frame is a function of the state alone.
pub fn throbber(frame: &mut ratatui::Frame<'_>, area: Rect, tick: usize, theme: &Theme) {
    let mut spinner = throbber_widgets_tui::ThrobberState::default();
    for _ in 0..tick {
        spinner.calc_next();
    }
    let throbber = throbber_widgets_tui::Throbber::default()
        .style(theme.accent)
        .throbber_set(throbber_widgets_tui::symbols::throbber::BRAILLE_SIX);
    frame.render_widget(throbber.to_symbol_span(&spinner), area);
}

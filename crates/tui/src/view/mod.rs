//! Rendering is a pure function of state, and it hands back the regions the
//! mouse is tested against.

pub mod account_picker;
pub mod builder;
pub mod chrome;
pub mod collection;
pub mod confirm;
pub mod detail;
pub mod error;
pub mod event_log;
pub mod fields;
pub mod help;
pub mod home;
pub mod json;
pub mod palette;
pub mod raw_request;
pub mod session;
pub mod theme;
pub mod toast;
pub mod too_small;
pub mod tree;

pub use theme::Theme;

use ratatui::layout::{Constraint, Layout, Rect};

use crate::core::State;
use crate::core::state::{Regions, View};

/// Below 80x24 the whole frame is the placeholder; no size panics. Rendering
/// borrows the state mutably, since a tree records where it drew its nodes.
pub fn render(frame: &mut ratatui::Frame<'_>, state: &mut State, theme: &Theme) -> Regions {
    let whole = frame.area();
    if state.too_small() {
        too_small::render(frame, whole, theme);
        return Regions {
            header: Rect::ZERO,
            body: whole,
            rows: Rect::ZERO,
            status: Rect::ZERO,
            footer: Rect::ZERO,
            watches: Vec::new(),
            row_height: 1,
        };
    }

    let [header, body, status, footer] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(whole);

    chrome::header(frame, header, state, theme);
    let rows = body_of(frame, body, state, theme);
    let watches = chrome::status_bar(frame, status, state, theme);
    chrome::footer(frame, footer, state, theme);
    overlay(frame, body, state, theme);
    toast::render(frame, body, &state.toasts, theme);

    Regions {
        header,
        body,
        rows,
        status,
        footer,
        watches,
        row_height: 1,
    }
}

/// The current view fills the body and says which part of it holds rows.
fn body_of(frame: &mut ratatui::Frame<'_>, area: Rect, state: &mut State, theme: &Theme) -> Rect {
    let State {
        stack,
        accounts,
        log,
        ..
    } = state;
    match stack.last_mut().expect("the stack is never empty") {
        View::Home(view) => {
            home::render(frame, area, view, theme);
            area
        }
        View::Collection(view) => collection::render(frame, area, view, theme),
        View::Detail(view) => {
            detail::render(frame, area, view, theme);
            area
        }
        View::Tree(view) => {
            tree::render(frame, area, view, theme);
            area
        }
        View::Builder(view) => {
            builder::render(frame, area, view, theme);
            area
        }
        View::AccountPicker(view) => {
            account_picker::render(frame, area, view, accounts, theme);
            area
        }
        View::Session(view) => {
            session::render(frame, area, view, theme);
            area
        }
        View::RawRequest(view) => {
            raw_request::render(frame, area, view, theme);
            area
        }
        View::EventLog(view) => {
            event_log::render(frame, area, view, log, theme);
            area
        }
    }
}

/// What sits over the view, when anything does.
fn overlay(frame: &mut ratatui::Frame<'_>, area: Rect, state: &State, theme: &Theme) {
    use crate::core::state::Overlay;
    match &state.overlay {
        Some(Overlay::Help(view)) => help::render(frame, area, view, theme),
        Some(Overlay::Palette(view)) => palette::render(frame, area, view, theme),
        Some(Overlay::Confirm(view)) => confirm::render(frame, area, view, theme),
        Some(Overlay::Error(failure)) => error::render(frame, area, failure, theme),
        Some(Overlay::Filter(prompt)) => {
            let area = centered(area, 50, 3);
            frame.render_widget(ratatui::widgets::Clear, area);
            frame.render_widget(
                ratatui::widgets::Paragraph::new(prompt.input.value().to_string()).block(
                    ratatui::widgets::Block::default()
                        .borders(ratatui::widgets::Borders::ALL)
                        .title(" filter loaded rows ")
                        .border_style(theme.accent),
                ),
                area,
            );
        }
        _ => {}
    }
}

/// A box of at most this size, in the middle of the area.
pub fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    Rect {
        x: area.x + (area.width - width) / 2,
        y: area.y + (area.height - height) / 2,
        width,
        height,
    }
}

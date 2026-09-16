//! Sixteen ANSI colours, or attributes alone under `NO_COLOR`.

use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy)]
pub struct Theme {
    pub base: Style,
    pub selected: Style,
    pub label: Style,
    pub value: Style,
    pub warn: Style,
    pub error: Style,
    pub muted: Style,
    pub accent: Style,
}

impl Theme {
    /// Sixteen ANSI colours only.
    pub fn colored() -> Theme {
        Theme {
            base: Style::default(),
            selected: Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            label: Style::default().fg(Color::Cyan),
            value: Style::default().fg(Color::White),
            warn: Style::default().fg(Color::Yellow),
            error: Style::default().fg(Color::Red),
            muted: Style::default().fg(Color::DarkGray),
            accent: Style::default().fg(Color::Magenta),
        }
    }

    /// Attributes only, for `NO_COLOR`.
    pub fn plain() -> Theme {
        Theme {
            base: Style::default(),
            selected: Style::default().add_modifier(Modifier::REVERSED),
            label: Style::default().add_modifier(Modifier::BOLD),
            value: Style::default(),
            warn: Style::default().add_modifier(Modifier::ITALIC),
            error: Style::default().add_modifier(Modifier::BOLD),
            muted: Style::default().add_modifier(Modifier::DIM),
            accent: Style::default().add_modifier(Modifier::UNDERLINED),
        }
    }

    pub fn detect() -> Theme {
        match std::env::var_os("NO_COLOR") {
            Some(value) if !value.is_empty() => Theme::plain(),
            _ => Theme::colored(),
        }
    }
}

//! What the session is, and the PKCE login that replaces it.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::core::views::session::SessionView;
use crate::ports::SessionStatus;
use crate::view::Theme;

pub fn render(frame: &mut ratatui::Frame<'_>, area: Rect, view: &SessionView, theme: &Theme) {
    let mut lines = vec![
        Line::from(vec![
            Span::styled("session  ", theme.label),
            Span::styled(described(&view.status), theme.value),
        ]),
        Line::default(),
    ];
    match &view.authorization {
        Some(authorization) => {
            lines.push(Line::from(Span::styled(
                "Enter opens the authorization page in a browser.",
                theme.muted,
            )));
            lines.push(Line::from(Span::styled(
                "Paste the code it gives back, then press Enter again.",
                theme.muted,
            )));
            lines.push(Line::default());
            lines.push(Line::from(vec![
                Span::styled("authorize  ", theme.label),
                Span::styled(authorization.url.clone(), theme.value),
            ]));
            lines.push(Line::from(vec![
                Span::styled("code       ", theme.label),
                Span::styled(view.code.value().to_string(), theme.value),
            ]));
        }
        None => lines.push(Line::from(Span::styled(
            "press Enter to begin a login",
            theme.muted,
        ))),
    }
    if let Some(error) = &view.error {
        lines.push(Line::default());
        lines.push(Line::from(Span::styled(error.clone(), theme.error)));
    }
    frame.render_widget(
        Paragraph::new(lines).wrap(ratatui::widgets::Wrap { trim: true }),
        area,
    );
}

fn described(status: &SessionStatus) -> String {
    match status {
        SessionStatus::Missing => "no credentials are stored".to_string(),
        SessionStatus::Active {
            account_id,
            expires_in_secs,
        } => format!("account {account_id}, good for {expires_in_secs}s"),
        SessionStatus::Expired { account_id } => {
            format!("account {account_id}, expired")
        }
    }
}

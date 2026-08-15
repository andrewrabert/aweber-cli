//! The accounts a token can reach.

use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListItem, ListState, Paragraph};

use crate::core::state::Account;
use crate::core::views::account_picker::AccountPickerView;
use crate::view::Theme;

pub fn render(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    view: &AccountPickerView,
    accounts: &[Account],
    theme: &Theme,
) {
    if accounts.is_empty() {
        frame.render_widget(
            Paragraph::new("this token reaches no account").style(theme.muted),
            area,
        );
        return;
    }
    let items: Vec<ListItem<'static>> = accounts
        .iter()
        .map(|account| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:<10}", account.id), theme.label),
                Span::styled(
                    account
                        .name
                        .clone()
                        .unwrap_or_else(|| "unnamed".to_string()),
                    theme.value,
                ),
            ]))
        })
        .collect();
    let mut state = ListState::default().with_selected(Some(view.selected));
    frame.render_stateful_widget(
        List::new(items).highlight_style(theme.selected),
        area,
        &mut state,
    );
}

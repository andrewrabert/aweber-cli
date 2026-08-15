//! Every operation the route table names is reachable, and no operation's
//! metadata invents an argument.

use aweber_tui::core::State;
use aweber_tui::core::state::{Overlay, View};
use aweber_tui::harness::Harness;

/// The palette, opened and typed into, as the user would.
fn palette(harness: &mut Harness, query: &str) {
    harness.key("Ctrl-p");
    for character in query.chars() {
        if character == ' ' {
            harness.key("Space");
        } else {
            harness.key(&character.to_string());
        }
    }
}

/// The labels the palette currently offers.
fn offered(state: &State) -> Vec<String> {
    match state.overlay.as_ref() {
        Some(Overlay::Palette(view)) => view
            .matches
            .iter()
            .map(|entry| entry.label.clone())
            .collect(),
        _ => Vec::new(),
    }
}

#[tokio::test]
async fn every_routed_operation_has_a_keystroke_path() {
    for entry in aweber_tui::catalog::entries() {
        let mut harness = Harness::new(80, 24);
        palette(&mut harness, &entry.label);
        let offered = offered(harness.state());
        let index = offered
            .iter()
            .position(|label| *label == entry.label)
            .unwrap_or_else(|| {
                panic!(
                    "typing '{}' offers {:?}, which does not include it",
                    entry.label, offered
                )
            });
        for _ in 0..index {
            harness.key("Down");
        }
        harness.key("Enter");
        let state = harness.state();
        assert!(
            state.stack.len() > 1
                || state.overlay.is_some()
                || !state.watches.is_empty()
                || !state.toasts.is_empty(),
            "'{}' was chosen and nothing happened",
            entry.label
        );
    }
}

#[tokio::test]
async fn the_oauth_operations_are_reachable_only_on_the_session_view() {
    let hidden: Vec<String> = aweber_tui::catalog::session_entries()
        .iter()
        .map(|entry| entry.label.clone())
        .collect();
    assert_eq!(hidden.len(), 4, "there are four hidden operations");

    let mut anywhere = Harness::new(80, 24);
    palette(&mut anywhere, "oauth");
    for label in &hidden {
        assert!(
            !offered(anywhere.state()).contains(label),
            "'{label}' is offered away from the Session view"
        );
    }

    let mut state = State::new(None, false);
    state.stack.push(View::Session(
        aweber_tui::core::views::session::SessionView::of(
            aweber_tui::ports::SessionStatus::Missing,
        ),
    ));
    let mut session = Harness::new(80, 24).with_state(state);
    palette(&mut session, "oauth");
    let offered = offered(session.state());
    for label in &hidden {
        assert!(
            offered.contains(label),
            "'{label}' is not offered on the Session view; it offers {offered:?}"
        );
    }
}

/// Every argument every entry's metadata fills has a spec of its own, the
/// reserved precondition included, so a prefilled value cannot be dropped by a
/// builder row that was never offered.
#[test]
fn metadata_matches_the_derived_specs() {
    for operation in aweber::catalog::Operation::ALL {
        let specs = operation.specs();
        for fill in aweber_tui::catalog::metadata(operation).fills {
            assert!(
                specs.iter().any(|spec| spec.name == fill.arg),
                "{operation:?} has no argument named '{}'",
                fill.arg
            );
        }
        for column in aweber_tui::catalog::metadata(operation).columns {
            assert!(
                column.pointer.is_empty() || column.pointer.starts_with('/'),
                "{operation:?} has a column pointer that is not a JSON pointer: {}",
                column.pointer
            );
        }
    }
}

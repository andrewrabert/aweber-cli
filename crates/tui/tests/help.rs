//! The help overlay and the dispatch table are one table, read two ways.

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use aweber_tui::core::keys::{self, Chord, Scope};

/// Every scope a view or an overlay can put the keyboard in.
const SCOPES: [Scope; 15] = [
    Scope::Global,
    Scope::Home,
    Scope::AccountPicker,
    Scope::Collection,
    Scope::Detail,
    Scope::Tree,
    Scope::Session,
    Scope::RawRequest,
    Scope::EventLog,
    Scope::Builder,
    Scope::Palette,
    Scope::Confirm,
    Scope::Error,
    Scope::Help,
    Scope::Filter,
];

/// The scopes whose input line owns `Left` and `Right`.
const EDITS_TEXT: [Scope; 5] = [
    Scope::Session,
    Scope::RawRequest,
    Scope::Builder,
    Scope::Palette,
    Scope::Filter,
];

fn pressed(chord: Chord) -> KeyEvent {
    KeyEvent::new(chord.code, chord.modifiers)
}

fn named(key: &str) -> KeyEvent {
    pressed(key.parse::<Chord>().expect("a test names a known key"))
}

fn action(scope: Scope, key: &str) -> Option<String> {
    keys::action_for(scope, named(key)).map(|action| format!("{action:?}"))
}

#[test]
fn help_and_the_dispatch_table_agree() {
    for scope in SCOPES {
        let rows = keys::help_rows(scope);
        for binding in keys::bindings() {
            if binding.scope != scope && binding.scope != Scope::Global {
                continue;
            }
            assert!(
                rows.iter()
                    .any(|row| row.keys == binding.keys() && row.label == binding.label),
                "{:?} in {:?} is bound but absent from help",
                binding.keys(),
                scope
            );
        }
        for row in &rows {
            assert!(
                keys::bindings().iter().any(|binding| {
                    binding.keys() == row.keys
                        && binding.label == row.label
                        && binding.scope == row.scope
                }),
                "the help row {:?} in {:?} names no binding",
                row.keys,
                scope
            );
        }
    }
}

#[test]
fn every_chord_dispatches_in_its_scope() {
    for binding in keys::bindings() {
        let wanted = format!("{:?}", binding.action);
        for chord in binding.chords {
            let reached = keys::action_for(binding.scope, pressed(*chord))
                .map(|action| format!("{action:?}"));
            assert_eq!(
                reached.as_deref(),
                Some(wanted.as_str()),
                "{chord} in {:?} does not reach {wanted}",
                binding.scope
            );
        }
    }
}

#[test]
fn no_chord_is_bound_twice_in_a_scope() {
    for scope in SCOPES {
        let mut seen: Vec<(String, &'static str)> = Vec::new();
        for binding in keys::bindings() {
            if binding.scope != scope && binding.scope != Scope::Global {
                continue;
            }
            for chord in binding.chords {
                let name = chord.to_string();
                assert!(
                    !seen.iter().any(|(bound, _)| *bound == name),
                    "{name} is bound twice in {scope:?}"
                );
                seen.push((name, binding.label));
            }
        }
    }
}

#[test]
fn the_arrow_and_page_keys_move_in_every_movable_scope() {
    for scope in keys::MOVABLE {
        for (vim, equivalent) in [
            ("j", "Down"),
            ("k", "Up"),
            ("Ctrl-d", "PgDn"),
            ("Ctrl-u", "PgUp"),
        ] {
            let moved = action(scope, vim);
            assert!(moved.is_some(), "{vim} moves nothing in {scope:?}");
            assert_eq!(
                action(scope, equivalent),
                moved,
                "{equivalent} does not move as {vim} in {scope:?}"
            );
        }
    }
}

#[test]
fn left_and_right_follow_pop_and_descend() {
    for scope in SCOPES {
        if EDITS_TEXT.contains(&scope) {
            continue;
        }
        if let Some(popped) = action(scope, "h") {
            assert_eq!(
                action(scope, "Left"),
                Some(popped),
                "Left does not pop in {scope:?}"
            );
        }
        let descended = action(scope, "Enter");
        let right = action(scope, "Right");
        match scope {
            Scope::Home
            | Scope::AccountPicker
            | Scope::Collection
            | Scope::Detail
            | Scope::Tree => assert_eq!(
                right, descended,
                "Right does not descend as Enter in {scope:?}"
            ),
            _ => assert_eq!(right, None, "Right is bound in {scope:?}"),
        }
    }
    assert_eq!(
        action(Scope::Error, "Right"),
        None,
        "Right dismisses the error modal"
    );
}

#[test]
fn the_input_line_keeps_left_and_right() {
    for scope in EDITS_TEXT {
        for key in ["Left", "Right"] {
            assert_eq!(action(scope, key), None, "{key} is bound in {scope:?}");
        }
    }
}

#[test]
fn help_lists_every_chord_of_an_action_on_one_row() {
    for scope in SCOPES {
        let rows = keys::help_rows(scope);
        for binding in keys::bindings() {
            if binding.scope != scope && binding.scope != Scope::Global {
                continue;
            }
            let named =
                rows.iter()
                    .filter(|row| row.scope == binding.scope && row.label == binding.label)
                    .filter(|row| {
                        binding.chords.iter().all(|chord| {
                            row.keys.split(" / ").any(|name| name == chord.to_string())
                        })
                    })
                    .count();
            assert_eq!(
                named, 1,
                "{:?} in {scope:?} spreads over {named} rows",
                binding.label
            );
        }
        for row in &rows {
            for name in row.keys.split(" / ") {
                assert!(
                    name.parse::<Chord>().is_ok(),
                    "the help row {:?} names {name:?}, which is no chord",
                    row.keys
                );
            }
        }
    }
}

#[test]
fn a_chord_round_trips_through_its_name() {
    for binding in keys::bindings() {
        for chord in binding.chords {
            let name = chord.to_string();
            assert_eq!(
                name.parse::<Chord>().as_ref(),
                Ok(chord),
                "{name} does not parse back to the chord help prints it for"
            );
        }
    }
    assert!("nope".parse::<Chord>().is_err());
    assert_eq!(
        "Space".parse::<Chord>(),
        Ok(Chord {
            code: KeyCode::Char(' '),
            modifiers: KeyModifiers::NONE,
        })
    );
}

#[test]
fn the_footer_shows_five_hints_at_most() {
    for scope in SCOPES {
        let hints = keys::footer_hints(scope);
        assert!(
            hints.len() <= 5,
            "{:?} offers {} hints before Ctrl-p",
            scope,
            hints.len()
        );
        let mut seen = hints.clone();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), hints.len(), "{scope:?} repeats a hint");
    }
}

#[test]
fn a_footer_hint_names_a_single_chord() {
    for scope in SCOPES {
        for hint in keys::footer_hints(scope) {
            let name = hint
                .split_whitespace()
                .next()
                .expect("a hint names a chord");
            assert!(
                !hint.contains(" / "),
                "the hint {hint:?} in {scope:?} names a whole group"
            );
            assert!(
                name.parse::<Chord>().is_ok(),
                "the hint {hint:?} in {scope:?} names no chord"
            );
        }
    }
}

//! Bindings are data, so the help overlay and the footer are generated from the
//! same table the dispatcher reads.

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::core::Action;
use crate::core::action::Motion;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Scope {
    Global,
    Home,
    AccountPicker,
    Collection,
    Detail,
    Tree,
    Session,
    RawRequest,
    EventLog,
    Builder,
    Palette,
    Confirm,
    Error,
    Help,
    Filter,
}

/// One binding per action per scope, holding every chord that reaches it.
pub struct Binding {
    /// The vim chord first, then its arrow or page equivalents.
    pub chords: &'static [Chord],
    pub scope: Scope,
    pub action: Action,
    pub label: &'static str,
    /// Shown in the footer's at-most-five hints.
    pub hint: bool,
}

impl Binding {
    /// Every chord joined by ` / `, as one help row reads.
    pub fn keys(&self) -> String {
        self.chords
            .iter()
            .map(Chord::to_string)
            .collect::<Vec<String>>()
            .join(" / ")
    }

    /// The first chord alone, as the footer reads.
    pub fn hint_key(&self) -> String {
        self.chords
            .first()
            .expect("a binding names at least one chord")
            .to_string()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Chord {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl std::fmt::Display for Chord {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.modifiers.contains(KeyModifiers::CONTROL)
            && let KeyCode::Char(character) = self.code
        {
            return write!(formatter, "Ctrl-{character}");
        }
        match self.code {
            KeyCode::Char(' ') => formatter.write_str("Space"),
            KeyCode::Char(character) => write!(formatter, "{character}"),
            KeyCode::Enter => formatter.write_str("Enter"),
            KeyCode::Esc => formatter.write_str("Esc"),
            KeyCode::Tab => formatter.write_str("Tab"),
            KeyCode::Backspace => formatter.write_str("Backspace"),
            KeyCode::Up => formatter.write_str("Up"),
            KeyCode::Down => formatter.write_str("Down"),
            KeyCode::Left => formatter.write_str("Left"),
            KeyCode::Right => formatter.write_str("Right"),
            KeyCode::PageUp => formatter.write_str("PgUp"),
            KeyCode::PageDown => formatter.write_str("PgDn"),
            other => write!(formatter, "{other:?}"),
        }
    }
}

impl std::str::FromStr for Chord {
    type Err = UnknownChord;

    fn from_str(text: &str) -> Result<Chord, UnknownChord> {
        let named = |code| Some(plain(code));
        let chord = match text {
            "Enter" => named(KeyCode::Enter),
            "Esc" => named(KeyCode::Esc),
            "Tab" => named(KeyCode::Tab),
            "Backspace" => named(KeyCode::Backspace),
            "Space" => named(KeyCode::Char(' ')),
            "Up" => named(KeyCode::Up),
            "Down" => named(KeyCode::Down),
            "Left" => named(KeyCode::Left),
            "Right" => named(KeyCode::Right),
            "PgUp" => named(KeyCode::PageUp),
            "PgDn" => named(KeyCode::PageDown),
            other => match other.strip_prefix("Ctrl-") {
                Some(rest) => sole(rest).map(|character| control(character.to_ascii_lowercase())),
                None => sole(other).map(|character| {
                    if character.is_ascii_uppercase() {
                        shifted(character)
                    } else {
                        plain(KeyCode::Char(character))
                    }
                }),
            },
        };
        chord.ok_or_else(|| UnknownChord(text.to_owned()))
    }
}

/// The single character of a chord named by a printable key.
fn sole(text: &str) -> Option<char> {
    let mut characters = text.chars();
    let character = characters.next()?;
    characters.next().is_none().then_some(character)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnknownChord(pub String);

impl std::fmt::Display for UnknownChord {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "'{}' names no key", self.0)
    }
}

impl std::error::Error for UnknownChord {}

const fn plain(code: KeyCode) -> Chord {
    Chord {
        code,
        modifiers: KeyModifiers::NONE,
    }
}

const fn control(character: char) -> Chord {
    Chord {
        code: KeyCode::Char(character),
        modifiers: KeyModifiers::CONTROL,
    }
}

const fn shifted(character: char) -> Chord {
    Chord {
        code: KeyCode::Char(character),
        modifiers: KeyModifiers::SHIFT,
    }
}

static DOWN: [Chord; 2] = [plain(KeyCode::Char('j')), plain(KeyCode::Down)];
static UP: [Chord; 2] = [plain(KeyCode::Char('k')), plain(KeyCode::Up)];
static TOP: [Chord; 1] = [plain(KeyCode::Char('g'))];
static BOTTOM: [Chord; 1] = [shifted('G')];
static PAGE_DOWN: [Chord; 2] = [control('d'), plain(KeyCode::PageDown)];
static PAGE_UP: [Chord; 2] = [control('u'), plain(KeyCode::PageUp)];
static PALETTE: [Chord; 1] = [control('p')];
static EVENT_LOG: [Chord; 1] = [control('l')];
static ANCESTORS: [Chord; 1] = [control('b')];
static WATCHES: [Chord; 1] = [control('w')];
static HELP: [Chord; 1] = [plain(KeyCode::Char('?'))];
static QUIT: [Chord; 1] = [plain(KeyCode::Char('q'))];
static YES: [Chord; 1] = [plain(KeyCode::Char('y'))];
static NO: [Chord; 1] = [plain(KeyCode::Char('n'))];
static TAB: [Chord; 1] = [plain(KeyCode::Tab)];
static RAW_JSON: [Chord; 1] = [shifted('J')];
static FILTER: [Chord; 1] = [plain(KeyCode::Char('/'))];
static REFRESH: [Chord; 1] = [plain(KeyCode::Char('r'))];
static COPY: [Chord; 1] = [plain(KeyCode::Char('y'))];
static WRITE_FILE: [Chord; 1] = [plain(KeyCode::Char('w'))];
static ARROW_DOWN: [Chord; 1] = [plain(KeyCode::Down)];
static ARROW_UP: [Chord; 1] = [plain(KeyCode::Up)];
static ENTER: [Chord; 1] = [plain(KeyCode::Enter)];
static ENTER_RIGHT: [Chord; 2] = [plain(KeyCode::Enter), plain(KeyCode::Right)];
static ESCAPE: [Chord; 1] = [plain(KeyCode::Esc)];
static POP: [Chord; 2] = [plain(KeyCode::Esc), plain(KeyCode::Char('h'))];
static POP_LEFT: [Chord; 3] = [
    plain(KeyCode::Esc),
    plain(KeyCode::Char('h')),
    plain(KeyCode::Left),
];

/// A scope whose input line owns `Left` and `Right`.
const fn edits_text(scope: Scope) -> bool {
    matches!(
        scope,
        Scope::Session | Scope::RawRequest | Scope::Builder | Scope::Palette | Scope::Filter
    )
}

/// `Right` joins `Enter` wherever `Enter` descends, expands, or submits without
/// an input line in the way.
fn enter_chords(scope: Scope) -> &'static [Chord] {
    if edits_text(scope) {
        &ENTER
    } else {
        &ENTER_RIGHT
    }
}

/// `Left` joins `Esc` and `h` wherever `h` pops.
fn pop_chords(scope: Scope) -> &'static [Chord] {
    if edits_text(scope) { &POP } else { &POP_LEFT }
}

/// The scopes a movable list occupies.
pub const MOVABLE: [Scope; 8] = [
    Scope::Home,
    Scope::AccountPicker,
    Scope::Collection,
    Scope::Detail,
    Scope::Tree,
    Scope::EventLog,
    Scope::Builder,
    Scope::Help,
];

/// `j/Down k/Up g G Ctrl-d/PgDn Ctrl-u/PgUp Enter/Right Esc/h/Left / Ctrl-p
/// Ctrl-l Ctrl-b Ctrl-w ? r y w J Tab q`, and nothing configurable.
///
/// `Ctrl-b` opens the breadcrumb palette a header click opens, and `Ctrl-w` the
/// watches palette a status-bar click opens, so no mouse action stands alone.
pub fn bindings() -> &'static [Binding] {
    static BINDINGS: std::sync::OnceLock<Vec<Binding>> = std::sync::OnceLock::new();
    BINDINGS.get_or_init(|| {
        let mut bindings = vec![
            Binding {
                chords: &PALETTE,
                scope: Scope::Global,
                action: Action::OpenPalette,
                label: "palette",
                hint: false,
            },
            Binding {
                chords: &EVENT_LOG,
                scope: Scope::Global,
                action: Action::OpenEventLog,
                label: "event log",
                hint: false,
            },
            Binding {
                chords: &ANCESTORS,
                scope: Scope::Global,
                action: Action::OpenAncestors,
                label: "breadcrumb",
                hint: false,
            },
            Binding {
                chords: &WATCHES,
                scope: Scope::Global,
                action: Action::OpenWatches,
                label: "watches",
                hint: false,
            },
            Binding {
                chords: &HELP,
                scope: Scope::Global,
                action: Action::OpenHelp,
                label: "help",
                hint: false,
            },
            Binding {
                chords: &QUIT,
                scope: Scope::Global,
                action: Action::Quit,
                label: "quit",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::Home),
                scope: Scope::Home,
                action: Action::Descend,
                label: "open",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::AccountPicker),
                scope: Scope::AccountPicker,
                action: Action::Descend,
                label: "choose",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::Collection),
                scope: Scope::Collection,
                action: Action::Descend,
                label: "open",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::Detail),
                scope: Scope::Detail,
                action: Action::ToggleExpand,
                label: "expand",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::Tree),
                scope: Scope::Tree,
                action: Action::ToggleExpand,
                label: "expand",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::Palette),
                scope: Scope::Palette,
                action: Action::Submit,
                label: "run",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::Builder),
                scope: Scope::Builder,
                action: Action::Submit,
                label: "send",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::Filter),
                scope: Scope::Filter,
                action: Action::Submit,
                label: "filter",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::Session),
                scope: Scope::Session,
                action: Action::Submit,
                label: "log in",
                hint: true,
            },
            Binding {
                chords: enter_chords(Scope::RawRequest),
                scope: Scope::RawRequest,
                action: Action::Submit,
                label: "send",
                hint: true,
            },
            Binding {
                chords: &ENTER,
                scope: Scope::Confirm,
                action: Action::Confirm,
                label: "confirm",
                hint: true,
            },
            Binding {
                chords: &YES,
                scope: Scope::Confirm,
                action: Action::Confirm,
                label: "confirm",
                hint: true,
            },
            Binding {
                chords: &NO,
                scope: Scope::Confirm,
                action: Action::Cancel,
                label: "cancel",
                hint: true,
            },
            Binding {
                chords: &TAB,
                scope: Scope::RawRequest,
                action: Action::FocusNext,
                label: "next field",
                hint: true,
            },
            Binding {
                chords: &TAB,
                scope: Scope::Builder,
                action: Action::FocusNext,
                label: "next row",
                hint: true,
            },
            Binding {
                chords: &RAW_JSON,
                scope: Scope::Detail,
                action: Action::ToggleRaw,
                label: "raw JSON",
                hint: true,
            },
            Binding {
                chords: &FILTER,
                scope: Scope::Collection,
                action: Action::OpenFilter,
                label: "filter loaded",
                hint: true,
            },
        ];
        for scope in MOVABLE {
            bindings.extend(movement(scope));
        }
        // Where a printable key is text and no vim motion is bound, the arrows
        // are the only way to move.
        bindings.push(Binding {
            chords: &ARROW_DOWN,
            scope: Scope::Palette,
            action: Action::Move(Motion::Down),
            label: "down",
            hint: false,
        });
        bindings.push(Binding {
            chords: &ARROW_UP,
            scope: Scope::Palette,
            action: Action::Move(Motion::Up),
            label: "up",
            hint: false,
        });
        for scope in [
            Scope::Home,
            Scope::AccountPicker,
            Scope::Collection,
            Scope::Detail,
            Scope::Tree,
            Scope::Session,
            Scope::RawRequest,
            Scope::EventLog,
            Scope::Builder,
        ] {
            bindings.push(Binding {
                chords: &REFRESH,
                scope,
                action: Action::Refresh,
                label: "refresh",
                hint: false,
            });
            bindings.push(Binding {
                chords: &COPY,
                scope,
                action: Action::Copy,
                label: "copy",
                hint: false,
            });
            bindings.push(Binding {
                chords: &WRITE_FILE,
                scope,
                action: Action::WriteFile,
                label: "write file",
                hint: false,
            });
        }
        for scope in [
            Scope::AccountPicker,
            Scope::Collection,
            Scope::Detail,
            Scope::Tree,
            Scope::Session,
            Scope::RawRequest,
            Scope::EventLog,
            Scope::Builder,
        ] {
            bindings.push(Binding {
                chords: pop_chords(scope),
                scope,
                action: Action::Pop,
                label: "back",
                hint: true,
            });
        }
        for scope in [
            Scope::Palette,
            Scope::Help,
            Scope::Confirm,
            Scope::Error,
            Scope::Filter,
        ] {
            bindings.push(Binding {
                chords: &ESCAPE,
                scope,
                action: Action::Cancel,
                label: "close",
                hint: true,
            });
        }
        bindings.push(Binding {
            chords: &ENTER,
            scope: Scope::Error,
            action: Action::Cancel,
            label: "dismiss",
            hint: true,
        });
        bindings
    })
}

fn movement(scope: Scope) -> Vec<Binding> {
    vec![
        Binding {
            chords: &DOWN,
            scope,
            action: Action::Move(Motion::Down),
            label: "down",
            hint: false,
        },
        Binding {
            chords: &UP,
            scope,
            action: Action::Move(Motion::Up),
            label: "up",
            hint: false,
        },
        Binding {
            chords: &TOP,
            scope,
            action: Action::Move(Motion::Top),
            label: "top",
            hint: false,
        },
        Binding {
            chords: &BOTTOM,
            scope,
            action: Action::Move(Motion::Bottom),
            label: "bottom",
            hint: false,
        },
        Binding {
            chords: &PAGE_DOWN,
            scope,
            action: Action::Move(Motion::PageDown),
            label: "page down",
            hint: false,
        },
        Binding {
            chords: &PAGE_UP,
            scope,
            action: Action::Move(Motion::PageUp),
            label: "page up",
            hint: false,
        },
    ]
}

/// A pressed key is matched in the current scope first, then globally.
pub fn action_for(scope: Scope, key: KeyEvent) -> Option<Action> {
    let chord = Chord {
        code: key.code,
        modifiers: relevant(key.modifiers, key.code),
    };
    let matching = |wanted: Scope| {
        bindings()
            .iter()
            .find(|binding| binding.scope == wanted && binding.chords.contains(&chord))
            .map(|binding| binding.action.clone())
    };
    matching(scope).or_else(|| matching(Scope::Global))
}

/// Shift is only meaningful where the binding names an uppercase character.
fn relevant(modifiers: KeyModifiers, code: KeyCode) -> KeyModifiers {
    let mut kept = KeyModifiers::NONE;
    if modifiers.contains(KeyModifiers::CONTROL) {
        kept |= KeyModifiers::CONTROL;
    }
    if modifiers.contains(KeyModifiers::SHIFT)
        && matches!(code, KeyCode::Char(character) if character.is_ascii_uppercase())
    {
        kept |= KeyModifiers::SHIFT;
    }
    kept
}

/// One row per binding in scope, listing all of its chords, so a binding cannot
/// exist without appearing in help and no chord gets a row of its own.
pub fn help_rows(scope: Scope) -> Vec<HelpRow> {
    bindings()
        .iter()
        .filter(|binding| binding.scope == scope || binding.scope == Scope::Global)
        .map(|binding| HelpRow {
            keys: binding.keys(),
            label: binding.label,
            scope: binding.scope,
        })
        .collect()
}

/// At most five, each naming one chord, with `Ctrl-p` appended by the footer.
pub fn footer_hints(scope: Scope) -> Vec<String> {
    const MOST: usize = 5;
    let mut hints: Vec<String> = Vec::new();
    for binding in bindings() {
        if !binding.hint || (binding.scope != scope && binding.scope != Scope::Global) {
            continue;
        }
        let hint = format!("{} {}", binding.hint_key(), binding.label);
        if !hints.contains(&hint) {
            hints.push(hint);
        }
        if hints.len() == MOST {
            break;
        }
    }
    hints
}

#[derive(Clone, Debug)]
pub struct HelpRow {
    pub keys: String,
    pub label: &'static str,
    pub scope: Scope,
}

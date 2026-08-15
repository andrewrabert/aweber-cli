//! Every emitted line is parsed back by the tree the binary routes with.

use aweber::catalog::{ArgRole, ArgSpec, ArgValue, Args, Operation, ValueKind};

const UUID: &str = "33333333-3333-4333-8333-333333333333";

/// The words of an emitted line, undoing the emitter's shell quoting.
fn argv(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut started = false;
    let mut quoted = false;
    let mut characters = line.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '\'' if quoted => {
                if characters.peek() == Some(&'\\') {
                    // the emitter writes a literal quote as `'\''`
                    characters.next();
                    characters.next();
                    characters.next();
                    word.push('\'');
                } else {
                    quoted = false;
                }
            }
            '\'' => {
                quoted = true;
                started = true;
            }
            character if character.is_whitespace() && !quoted => {
                if started {
                    words.push(std::mem::take(&mut word));
                    started = false;
                }
            }
            character => {
                word.push(character);
                started = true;
            }
        }
    }
    if started {
        words.push(word);
    }
    words
}

/// The shapes a `Text` argument's own validator might insist on.
const TEXTS: [&str; 6] = [
    "a value",
    UUID,
    "someone@example.com",
    "203.0.113.4",
    "https://example.com/thing",
    "7",
];

/// Whether the operation's own command takes this value for this argument.
fn accepted(operation: Operation, spec: &ArgSpec, value: &str) -> bool {
    let mut words = vec![String::new()];
    if !spec.positional {
        words.push(format!("--{}", spec.long));
    }
    words.push(value.to_string());
    match operation.command().try_get_matches_from(words) {
        Ok(_) => true,
        Err(error) => !matches!(
            error.kind(),
            clap::error::ErrorKind::InvalidValue | clap::error::ErrorKind::ValueValidation
        ),
    }
}

/// A value of the kind the argument declares, in a shape its validator takes.
fn value_of(operation: Operation, spec: &ArgSpec) -> ArgValue {
    let raw = match &spec.kind {
        ValueKind::Text => TEXTS
            .iter()
            .find(|text| accepted(operation, spec, text))
            .unwrap_or_else(|| panic!("{operation:?} takes no fixture value for {}", spec.name))
            .to_string(),
        ValueKind::Integer => "7".to_string(),
        ValueKind::Decimal => "1.5".to_string(),
        ValueKind::Bool | ValueKind::Flag => "true".to_string(),
        ValueKind::Date => "2026-01-02".to_string(),
        ValueKind::DateTime => "2026-01-02T03:04:05Z".to_string(),
        ValueKind::Uuid => UUID.to_string(),
        ValueKind::Path => "/tmp/a file".to_string(),
        ValueKind::Enum(options) => options
            .first()
            .cloned()
            .unwrap_or_else(|| "a value".to_string()),
        ValueKind::Json => r#"{"a":1}"#.to_string(),
    };
    ArgValue::new(spec.kind.clone(), raw)
}

/// Every argument an operation declares, one member of each one-of group, and
/// nothing the plan reserves for itself.
fn fixture(operation: Operation) -> (Vec<ArgSpec>, Args) {
    let mut args = Args::default();
    let mut chosen: Vec<ArgSpec> = Vec::new();
    for spec in operation.specs() {
        if spec.role == ArgRole::Reserved {
            continue;
        }
        if chosen
            .iter()
            .any(|taken| spec.alternatives.contains(&taken.name))
        {
            continue;
        }
        args.set(&spec.name, value_of(operation, &spec));
        chosen.push(spec);
    }
    (chosen, args)
}

/// `command_tree` with the groups the binary does not route added back from
/// their own clap definitions, so a TUI-only operation's line round-trips
/// through the arguments that define it.
fn every_group_tree() -> clap::Command {
    let mut tree = aweber::catalog::command_tree();
    for group in aweber::catalog::groups() {
        if !group.hidden && !group.cli {
            tree = tree.subcommand(aweber::catalog::group_command(group));
        }
    }
    tree
}

/// The words after `aweber`, which the tree parses under its own bin name.
fn parsed(line: &str) -> clap::ArgMatches {
    let words = argv(line);
    assert_eq!(words.first().map(String::as_str), Some("aweber"));
    every_group_tree()
        .try_get_matches_from(&words)
        .unwrap_or_else(|error| panic!("{line} does not parse:\n{error}"))
}

/// The matches of the operation's own subcommand.
fn leaf(matches: &clap::ArgMatches) -> &clap::ArgMatches {
    let (_, group) = matches.subcommand().expect("a line names its group");
    let (_, leaf) = group.subcommand().expect("a line names its action");
    leaf
}

/// For every operation the fixture arguments emit a line `command_tree` parses,
/// and the parse yields back the values that were emitted.
#[test]
fn every_command_line_parses_and_round_trips() {
    for operation in Operation::ALL {
        if operation.is_hidden() {
            continue;
        }
        let (specs, args) = fixture(operation);
        let line = aweber::catalog::command_line(operation, &args);
        let matches = parsed(&line);
        let leaf = leaf(&matches);
        for spec in &specs {
            if spec.kind == ValueKind::Flag {
                assert!(
                    leaf.get_flag(&spec.name),
                    "{operation:?} lost the flag {}:\n{line}",
                    spec.name
                );
                continue;
            }
            let back: Vec<String> = leaf
                .get_raw(&spec.name)
                .unwrap_or_else(|| panic!("{operation:?} lost {}:\n{line}", spec.name))
                .map(|value| value.to_string_lossy().into_owned())
                .collect();
            let sent: Vec<String> = args
                .all(&spec.name)
                .iter()
                .map(|value| value.raw.clone())
                .collect();
            assert_eq!(back, sent, "{operation:?} changed {}:\n{line}", spec.name);
        }
    }
}

/// A positional argument is emitted bare and never as `--<name>`.
#[test]
fn a_positional_argument_is_written_in_its_place() {
    let mut seen = 0;
    for operation in Operation::ALL {
        if operation.is_hidden() {
            continue;
        }
        let (specs, args) = fixture(operation);
        let line = aweber::catalog::command_line(operation, &args);
        for spec in specs.iter().filter(|spec| spec.positional) {
            seen += 1;
            assert!(
                !line.contains(&format!("--{}", spec.long)),
                "{operation:?} wrote the positional {} as a flag:\n{line}",
                spec.name
            );
            let value = args
                .first(&spec.name)
                .expect("the fixture set every required argument");
            assert!(
                argv(&line).contains(&value.raw),
                "{operation:?} did not write {} bare:\n{line}",
                spec.name
            );
        }
    }
    assert!(seen > 0, "some operation takes a positional argument");
}

/// `raw_command_line` emits an `aweber api` invocation `raw_command` parses, and
/// a body reaches it on standard input.
#[test]
fn a_raw_command_line_parses() {
    let path = "/1.0/accounts/1/lists";
    let bare = aweber::catalog::raw_command_line(&reqwest::Method::GET, path, None);
    let matches = parsed(&bare);
    let (name, api) = matches.subcommand().expect("a raw line names `api`");
    assert_eq!(name, "api");
    assert_eq!(
        api.get_one::<String>("path").map(String::as_str),
        Some(path)
    );
    assert_eq!(
        api.get_one::<String>("method").map(String::as_str),
        Some("GET")
    );

    for body in [
        aweber::catalog::PlanBody::Json(serde_json::json!({ "name": "a list" })),
        aweber::catalog::PlanBody::form(&serde_json::json!({ "name": "a list" })),
    ] {
        let line = aweber::catalog::raw_command_line(&reqwest::Method::POST, path, Some(&body));
        let (printf, call) = line
            .split_once(" | ")
            .unwrap_or_else(|| panic!("a body reaches the call on standard input:\n{line}"));
        let printed = argv(printf);
        assert_eq!(printed.first().map(String::as_str), Some("printf"));
        assert_eq!(
            printed.last().map(String::as_str),
            Some(body.encoded().as_str()),
            "the printed body is what the request carries:\n{line}"
        );
        let matches = parsed(call);
        let (name, api) = matches.subcommand().expect("a raw line names `api`");
        assert_eq!(name, "api");
        assert_eq!(
            api.get_one::<String>("input").map(String::as_str),
            Some("-"),
            "the body arrives on standard input:\n{line}"
        );
        let headers: Vec<String> = api
            .get_many::<String>("header")
            .unwrap_or_default()
            .cloned()
            .collect();
        assert_eq!(
            headers,
            vec![format!("content-type: {}", body.content_type())],
            "the line names its body's content type:\n{line}"
        );
    }
}

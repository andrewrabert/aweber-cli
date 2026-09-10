//! Credentials that never print themselves, and a scrubber for text that may
//! have carried one.

/// A value that never prints itself.
#[derive(Clone)]
pub struct Secret(String);

impl Secret {
    pub fn new(value: String) -> Secret {
        Secret(value)
    }

    pub fn expose(&self) -> &str {
        &self.0
    }
}

pub const REDACTED: &str = "«redacted»";

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(REDACTED)
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(REDACTED)
    }
}

/// The names whose value is a credential wherever it appears.
const SECRET_NAMES: [&str; 8] = [
    "access_token",
    "refresh_token",
    "client_secret",
    "code_verifier",
    "code_challenge",
    "code",
    "token",
    "authorization",
];

/// The flags whose value is a credential, in either spelling.
const SECRET_FLAGS: [&str; 9] = [
    "access-token",
    "refresh-token",
    "client-secret",
    "code-verifier",
    "code-challenge",
    "code",
    "token",
    "oauth-token",
    "oauth-consumer-key",
];

/// Replaces bearer values and every credential-bearing name in headers, query
/// strings, JSON, and command lines, whether the value follows `=`, `:`, or a
/// space.
pub fn scrub(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while !rest.is_empty() {
        let previous = text[..text.len() - rest.len()].chars().next_back();
        if let Some(consumed) = bearer_run(rest) {
            out.push_str("Bearer ");
            out.push_str(REDACTED);
            rest = &rest[consumed..];
            continue;
        }
        if flag_boundary(previous)
            && let Some((consumed, kept)) = flag_run(rest)
        {
            out.push_str(kept);
            out.push_str(REDACTED);
            rest = &rest[consumed..];
            continue;
        }
        if name_boundary(previous)
            && let Some((consumed, kept)) = named_run(rest)
        {
            out.push_str(kept);
            out.push_str(REDACTED);
            rest = &rest[consumed..];
            continue;
        }
        let character = rest.chars().next().expect("the rest is not empty");
        out.push(character);
        rest = &rest[character.len_utf8()..];
    }
    out
}

/// True where a flag can start: at the head of the text, after whitespace, or
/// after a quote.
fn flag_boundary(previous: Option<char>) -> bool {
    match previous {
        None => true,
        Some(character) => character.is_whitespace() || matches!(character, '\'' | '"'),
    }
}

/// True where a credential name can start: at the head of the text, and after
/// any character that cannot itself be part of a name.
fn name_boundary(previous: Option<char>) -> bool {
    match previous {
        None => true,
        Some(character) => !(character.is_alphanumeric() || matches!(character, '_' | '-')),
    }
}

/// The bytes of a `Bearer <token>` run at the head of `text`.
fn bearer_run(text: &str) -> Option<usize> {
    const PREFIX: usize = "Bearer ".len();
    if !starts_with_ignore_ascii_case(text, "Bearer ") {
        return None;
    }
    let length = value_length(&text[PREFIX..], false);
    (length > 0).then_some(PREFIX + length)
}

/// The bytes of a `name = value` run at the head of `text`, with the text that
/// precedes the value and is kept as it stands.
fn named_run(text: &str) -> Option<(usize, &str)> {
    for name in SECRET_NAMES {
        let quoted_name = text.starts_with('"');
        let head = usize::from(quoted_name);
        if !starts_with_ignore_ascii_case(&text[head..], name) {
            continue;
        }
        let mut cursor = head + name.len();
        if quoted_name {
            if !text[cursor..].starts_with('"') {
                continue;
            }
            cursor += 1;
        }
        cursor += whitespace(&text[cursor..]);
        let json = match text.as_bytes().get(cursor) {
            Some(b'=') => false,
            Some(b':') => true,
            _ => continue,
        };
        cursor += 1;
        cursor += whitespace(&text[cursor..]);
        let quoted_value = text[cursor..].starts_with('"');
        if quoted_value {
            cursor += 1;
        }
        let length = value_length(&text[cursor..], json && !quoted_value);
        if length == 0 {
            continue;
        }
        return Some((cursor + length, &text[..cursor]));
    }
    None
}

/// The bytes of a `--name value`, `--name=value`, or `-n value` run at the head
/// of `text`, with the text that precedes the value and is kept as it stands.
fn flag_run(text: &str) -> Option<(usize, &str)> {
    let dashes = if text.starts_with("--") {
        2
    } else if text.starts_with('-') {
        1
    } else {
        return None;
    };
    let tail = &text[dashes..];
    let name_length = tail
        .find(|c: char| c == '=' || c.is_whitespace())
        .unwrap_or(tail.len());
    let name = &tail[..name_length];
    if !SECRET_FLAGS.iter().any(|secret| same_flag(secret, name)) {
        return None;
    }
    let mut cursor = dashes + name_length;
    match text.as_bytes().get(cursor) {
        Some(b'=') => cursor += 1,
        Some(byte) if byte.is_ascii_whitespace() => cursor += whitespace(&text[cursor..]),
        _ => return None,
    }
    let quote = text[cursor..]
        .chars()
        .next()
        .filter(|character| matches!(character, '"' | '\''));
    let length = match quote {
        Some(quote) => {
            cursor += quote.len_utf8();
            text[cursor..].find(quote).unwrap_or(text.len() - cursor)
        }
        None => value_length(&text[cursor..], false),
    };
    (length > 0).then(|| (cursor + length, &text[..cursor]))
}

/// Whether `text` opens with `prefix`, ASCII case ignored, for a `prefix` that
/// may be longer than `text` and may end where a character does not.
fn starts_with_ignore_ascii_case(text: &str, prefix: &str) -> bool {
    text.len() >= prefix.len()
        && text.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
}

/// Two flag names naming one argument, `-` and `_` reading alike.
fn same_flag(secret: &str, name: &str) -> bool {
    secret.len() == name.len()
        && secret.bytes().zip(name.bytes()).all(|(left, right)| {
            let normalize = |byte: u8| {
                if byte == b'-' {
                    b'_'
                } else {
                    byte.to_ascii_lowercase()
                }
            };
            normalize(left) == normalize(right)
        })
}

fn whitespace(text: &str) -> usize {
    text.len() - text.trim_start().len()
}

/// A value runs until whatever ends it: a quote, a separator, or whitespace.
fn value_length(text: &str, allow_spaces: bool) -> usize {
    text.find(|c: char| {
        matches!(c, '"' | '&' | ',' | '}' | '\n' | '\r') || (!allow_spaces && c.is_whitespace())
    })
    .unwrap_or(text.len())
}

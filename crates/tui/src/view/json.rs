//! The same document, as JSON, syntax-coloured within the sixteen-colour
//! palette.

use ratatui::text::{Line, Span};

use crate::view::Theme;

pub fn lines(document: &serde_json::Value, theme: &Theme) -> Vec<Line<'static>> {
    let mut out = Vec::new();
    write(&mut out, document, 0, None, false, theme);
    out
}

/// One value, indented, optionally named, optionally followed by a comma.
fn write(
    out: &mut Vec<Line<'static>>,
    value: &serde_json::Value,
    depth: usize,
    key: Option<&str>,
    comma: bool,
    theme: &Theme,
) {
    let pad = "  ".repeat(depth);
    let mut opening = vec![Span::styled(pad.clone(), theme.base)];
    if let Some(key) = key {
        opening.push(Span::styled(format!("\"{key}\""), theme.label));
        opening.push(Span::styled(": ", theme.muted));
    }
    let tail = if comma { "," } else { "" };
    match value {
        serde_json::Value::Object(fields) if !fields.is_empty() => {
            opening.push(Span::styled("{", theme.muted));
            out.push(Line::from(opening));
            let last = fields.len() - 1;
            for (index, (name, member)) in fields.iter().enumerate() {
                write(out, member, depth + 1, Some(name), index != last, theme);
            }
            out.push(Line::from(vec![Span::styled(
                format!("{pad}}}{tail}"),
                theme.muted,
            )]));
        }
        serde_json::Value::Array(entries) if !entries.is_empty() => {
            opening.push(Span::styled("[", theme.muted));
            out.push(Line::from(opening));
            let last = entries.len() - 1;
            for (index, member) in entries.iter().enumerate() {
                write(out, member, depth + 1, None, index != last, theme);
            }
            out.push(Line::from(vec![Span::styled(
                format!("{pad}]{tail}"),
                theme.muted,
            )]));
        }
        other => {
            let (text, style) = match other {
                serde_json::Value::Null => ("null".to_string(), theme.muted),
                serde_json::Value::Bool(set) => (set.to_string(), theme.warn),
                serde_json::Value::String(text) => (format!("\"{text}\""), theme.value),
                serde_json::Value::Object(_) => ("{}".to_string(), theme.muted),
                serde_json::Value::Array(_) => ("[]".to_string(), theme.muted),
                number => (number.to_string(), theme.accent),
            };
            opening.push(Span::styled(text, style));
            if comma {
                opening.push(Span::styled(",", theme.muted));
            }
            out.push(Line::from(opening));
        }
    }
}

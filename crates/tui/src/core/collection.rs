//! What came back from a request, read as a page of a collection.

use aweber::catalog::{CursorStyle, RequestPlan};

#[derive(Clone, Debug)]
pub struct Delivered {
    pub status: u16,
    pub document: Option<serde_json::Value>,
    pub entries: Vec<serde_json::Value>,
    pub next: Option<Continuation>,
    /// The offset of a cursor the upstream refuses to honour, which is why
    /// `next` is empty.
    pub capped: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Continuation {
    Url(String),
    Cursor { arg: String, value: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EndReason {
    Exhausted,
    /// The offset past which the upstream cursor is refused.
    OffsetCap {
        offset: u64,
    },
    Unavailable {
        status: u16,
        method: String,
        path: String,
    },
}

#[derive(Clone, Debug)]
pub struct Row {
    pub document: serde_json::Value,
    pub cells: Vec<String>,
    pub self_link: Option<String>,
}

/// Reads `entries`, `total_size`, `next_collection_link`, and the `Link` header
/// per the plan's cursor. A `Link` cursor past
/// `aweber::pagination::MAX_OFFSET_CURSOR` is refused here, so the collection
/// ends at the cap rather than on a rejected request.
pub fn deliver(cursor: &CursorStyle, response: aweber::client::PlanResponse) -> Delivered {
    let document = response.body;
    let entries = match &document {
        Some(serde_json::Value::Array(entries)) => entries.clone(),
        Some(serde_json::Value::Object(object)) => match object.get("entries") {
            Some(serde_json::Value::Array(entries)) => entries.clone(),
            _ => Vec::new(),
        },
        _ => Vec::new(),
    };
    let total = document
        .as_ref()
        .and_then(|body| body.pointer("/total_size"))
        .and_then(serde_json::Value::as_u64);
    let mut capped = None;
    let next = match cursor {
        CursorStyle::None => None,
        CursorStyle::NextCollectionLink => document
            .as_ref()
            .and_then(|body| body.pointer("/next_collection_link"))
            .and_then(serde_json::Value::as_str)
            .map(|url| Continuation::Url(url.to_string())),
        CursorStyle::LinkHeader { parameter, arg } => {
            aweber::pagination::next_cursor(&response.headers, parameter).and_then(|value| {
                match aweber::pagination::cursor_offset(&value) {
                    Some(offset) if offset > aweber::pagination::MAX_OFFSET_CURSOR => {
                        capped = Some(offset);
                        None
                    }
                    _ => Some(Continuation::Cursor {
                        arg: (*arg).to_string(),
                        value: value.to_string(),
                    }),
                }
            })
        }
    };
    Delivered {
        status: response.status,
        document,
        entries,
        next,
        capped,
        total,
    }
}

pub fn rows(entries: Vec<serde_json::Value>, columns: &[crate::catalog::Column]) -> Vec<Row> {
    entries
        .into_iter()
        .map(|document| {
            let cells = if columns.is_empty() {
                vec![cell(&document, "")]
            } else {
                columns
                    .iter()
                    .map(|column| cell(&document, column.pointer))
                    .collect()
            };
            let self_link = document
                .pointer("/self_link")
                .and_then(serde_json::Value::as_str)
                .map(ToString::to_string);
            Row {
                document,
                cells,
                self_link,
            }
        })
        .collect()
}

/// The pointer's value as one line; an absent field reads as an em dash.
fn cell(document: &serde_json::Value, pointer: &str) -> String {
    let value = if pointer.is_empty() {
        Some(document)
    } else {
        document.pointer(pointer)
    };
    match value {
        None => "—".to_string(),
        Some(serde_json::Value::Null) => "null".to_string(),
        Some(serde_json::Value::String(text)) => text.clone(),
        Some(other) => other.to_string(),
    }
}

/// The plan that continues a collection.
pub fn continue_plan(plan: &RequestPlan, continuation: &Continuation) -> RequestPlan {
    match continuation {
        Continuation::Url(url) => RequestPlan::absolute(url),
        Continuation::Cursor { arg, value } => {
            let mut next = plan.clone();
            let name = query_name(arg);
            next.query.retain(|(key, _)| *key != name);
            next.query.push((name, value.clone()));
            next
        }
    }
}

/// The API's name for a cursor argument, as the plan builder spells it.
fn query_name(arg: &str) -> String {
    match arg {
        "links-cursor" | "after" => "after".to_string(),
        "start-token" => "start-token".to_string(),
        other => other.replace('-', "_"),
    }
}

/// Timestamps render in the injected offset beside their raw value.
pub fn local_and_raw(value: &str, offset: chrono::FixedOffset) -> Option<(String, String)> {
    let parsed = chrono::DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|at| at.with_timezone(&offset))
        .or_else(|| {
            chrono::NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S")
                .ok()
                .map(|naive| naive.and_utc().with_timezone(&offset))
        })?;
    Some((
        parsed.format("%Y-%m-%d %H:%M:%S %:z").to_string(),
        value.to_string(),
    ))
}

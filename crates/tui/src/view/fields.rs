//! A document as a tree of its own fields.

/// One value on one line; a container says how big it is.
fn scalar(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::String(text) => text.clone(),
        serde_json::Value::Array(entries) => format!("[{} entries]", entries.len()),
        serde_json::Value::Object(fields) => format!("{{{} fields}}", fields.len()),
        other => other.to_string(),
    }
}

/// Glossary labels; a campaign-service automation is a workflow, never a
/// campaign.
pub fn label(key: &str) -> &str {
    static LABELS: std::sync::OnceLock<std::sync::Mutex<Vec<&'static str>>> =
        std::sync::OnceLock::new();
    match key {
        "id" => return "id",
        "uuid" => return "uuid",
        "self_link" => return "link",
        "http_etag" => return "etag",
        "precondition_version" => return "version",
        "rule_set" | "ruleset" => return "workflow ruleset",
        "campaign_type" => return "message type",
        _ => {}
    }
    if !key.contains('_') {
        return leaked(&LABELS, key.to_string());
    }
    leaked(&LABELS, key.replace('_', " "))
}

fn leaked(
    cache: &std::sync::OnceLock<std::sync::Mutex<Vec<&'static str>>>,
    text: String,
) -> &'static str {
    let cache = cache.get_or_init(|| std::sync::Mutex::new(Vec::new()));
    let mut cache = cache.lock().expect("the label cache is not poisoned");
    if let Some(existing) = cache.iter().find(|label| **label == text) {
        return existing;
    }
    let label: &'static str = Box::leak(text.into_boxed_str());
    cache.push(label);
    label
}

/// One node per field, labelled from the glossary, nothing omitted, null and
/// absent reading differently, and a timestamp reading in `zone` beside its raw
/// value.
pub fn tree_items(
    document: &serde_json::Value,
    zone: chrono::FixedOffset,
) -> Vec<tui_tree_widget::TreeItem<'static, String>> {
    match document {
        serde_json::Value::Object(fields) => fields
            .iter()
            .map(|(key, value)| item(key.clone(), label(key).to_string(), value, zone))
            .collect(),
        serde_json::Value::Array(entries) => entries
            .iter()
            .enumerate()
            .map(|(index, value)| item(index.to_string(), format!("[{index}]"), value, zone))
            .collect(),
        other => vec![tui_tree_widget::TreeItem::new_leaf(
            scalar(other),
            scalar(other),
        )],
    }
}

fn item(
    identifier: String,
    label: String,
    value: &serde_json::Value,
    zone: chrono::FixedOffset,
) -> tui_tree_widget::TreeItem<'static, String> {
    match value {
        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
            let children = tree_items(value, zone);
            tui_tree_widget::TreeItem::new(
                identifier,
                format!("{label}  {}", scalar(value)),
                children,
            )
            .expect("every child of a document has its own name")
        }
        scalar_value => tui_tree_widget::TreeItem::new_leaf(
            identifier,
            format!("{label}  {}", timed(scalar_value, zone)),
        ),
    }
}

/// A timestamp reads in the injected zone beside the value it came as.
fn timed(value: &serde_json::Value, zone: chrono::FixedOffset) -> String {
    match value {
        serde_json::Value::String(text) => {
            match crate::core::collection::local_and_raw(text, zone) {
                Some((local, raw)) => format!("{local}  ({raw})"),
                None => text.clone(),
            }
        }
        other => scalar(other),
    }
}

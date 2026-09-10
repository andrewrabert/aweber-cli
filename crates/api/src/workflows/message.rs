use std::collections::BTreeMap;

use reqwest::Method;

use crate::client::{ApiError, ApiRequest, Client};
use crate::ids::{AccountUid, MessageId};

const MESSAGES: &str = "/internal/message/messages/batch";

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BatchOutcome {
    #[serde(default)]
    pub processed: Vec<MessageId>,
    #[serde(default)]
    pub unprocessed: Vec<MessageId>,
}

#[derive(serde::Serialize)]
struct MessageBatch<'a> {
    message_ids: &'a [MessageId],
}

#[derive(serde::Serialize)]
struct AccountMessageBatch<'a> {
    account: AccountUid,
    message_ids: &'a [MessageId],
}

pub async fn get_message_subjects(
    client: &Client,
    messages: &[MessageId],
) -> Result<BTreeMap<MessageId, String>, ApiError> {
    let document: serde_json::Value =
        ApiRequest::new(client, Method::POST, format!("{MESSAGES}/get"))
            .query("fields", "subject")
            .json_body(MessageBatch {
                message_ids: messages,
            })
            .send()
            .await?;
    Ok(subjects_of(&document))
}

pub async fn unbind_messages(
    client: &Client,
    account: AccountUid,
    messages: &[MessageId],
) -> Result<BatchOutcome, ApiError> {
    ApiRequest::new(client, Method::POST, format!("{MESSAGES}/unbind"))
        .json_body(AccountMessageBatch {
            account,
            message_ids: messages,
        })
        .send()
        .await
}

pub async fn delete_messages(
    client: &Client,
    account: AccountUid,
    messages: &[MessageId],
) -> Result<BatchOutcome, ApiError> {
    ApiRequest::new(client, Method::POST, format!("{MESSAGES}/delete"))
        .json_body(AccountMessageBatch {
            account,
            message_ids: messages,
        })
        .send()
        .await
}

/// A message the service cannot find is absent from the map.
pub fn subjects_of(document: &serde_json::Value) -> BTreeMap<MessageId, String> {
    let entries = document
        .get("messages")
        .or_else(|| document.get("entries"))
        .unwrap_or(document);
    match entries {
        serde_json::Value::Object(object) => object
            .iter()
            .filter_map(|(id, entry)| {
                Some((
                    id.parse::<MessageId>().ok()?,
                    subject_of(entry)?.to_string(),
                ))
            })
            .collect(),
        serde_json::Value::Array(items) => items
            .iter()
            .filter_map(|entry| {
                let id = entry
                    .get("id")
                    .or_else(|| entry.get("_id"))
                    .and_then(serde_json::Value::as_str)?;
                Some((
                    id.parse::<MessageId>().ok()?,
                    subject_of(entry)?.to_string(),
                ))
            })
            .collect(),
        _ => BTreeMap::new(),
    }
}

fn subject_of(entry: &serde_json::Value) -> Option<&str> {
    match entry {
        serde_json::Value::String(subject) => Some(subject),
        entry => entry.get("subject").and_then(serde_json::Value::as_str),
    }
}

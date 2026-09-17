use std::collections::BTreeMap;

use reqwest::Method;

use crate::client::{ApiError, ApiRequest, Client};
use crate::ids::{AccountUid, MessageId};

const GET_MESSAGES: &str = "/internal/message/messages/batch/get";
const UNBIND_MESSAGES: &str = "/internal/message/messages/batch/unbind";
const DELETE_MESSAGES: &str = "/internal/message/messages/batch/delete";

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub id: MessageId,
    #[serde(skip_serializing)]
    pub account_id: i32,
    #[serde(skip_serializing)]
    pub list_id: i32,
    pub subject: String,
    pub body_text: String,
    pub body_html: String,
    #[serde(skip_serializing)]
    pub body_json: serde_json::Map<String, serde_json::Value>,
    /// `body_text` was hand-edited and no longer mirrors `body_html`.
    /// The message editor uses this to decide whether to regenerate plain
    /// text from HTML (`true` = do not sync).
    #[serde(skip_serializing)]
    pub has_customized_body_text: bool,
    pub attachment_ids: Vec<String>,
    #[serde(skip_serializing)]
    pub binding: Binding,
    /// Id of the resource named by `binding`.
    #[serde(skip_serializing)]
    pub bound_resource_id: Option<String>,
    /// When copied from a broadcast, holds the source broadcast's scheduling settings.
    #[serde(skip_serializing)]
    pub copied_from: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Binding {
    Unbound,
    Broadcast,
    Followup,
    Campaign,
}

#[derive(serde::Deserialize)]
struct MessageListResponse {
    messages: Vec<Option<Message>>,
}

#[derive(serde::Deserialize)]
struct UnbindMessagesResponse {
    processed: Vec<MessageId>,
    unprocessed: Vec<MessageId>,
}

#[derive(serde::Deserialize)]
struct DeleteMessagesResponse {
    processed: Vec<MessageId>,
    unprocessed: Vec<MessageId>,
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

pub async fn get_messages(
    client: &Client,
    message_ids: &[MessageId],
) -> Result<BTreeMap<MessageId, Message>, ApiError> {
    let response: MessageListResponse =
        ApiRequest::new(client, Method::POST, GET_MESSAGES.to_owned())
            .json_body(MessageBatch { message_ids })
            .send()
            .await?;
    let found: BTreeMap<MessageId, Message> = response
        .messages
        .into_iter()
        .flatten()
        .map(|message| (message.id.clone(), message))
        .collect();
    let missing: Vec<MessageId> = message_ids
        .iter()
        .filter(|id| !found.contains_key(id))
        .cloned()
        .collect();
    if !missing.is_empty() {
        return Err(ApiError::MessagesNotFound(missing));
    }
    Ok(found)
}

pub async fn unbind_messages(
    client: &Client,
    account: AccountUid,
    message_ids: &[MessageId],
) -> Result<(), ApiError> {
    let response: UnbindMessagesResponse =
        ApiRequest::new(client, Method::POST, UNBIND_MESSAGES.to_owned())
            .json_body(AccountMessageBatch {
                account,
                message_ids,
            })
            .send()
            .await?;
    if message_ids.iter().all(|id| response.processed.contains(id)) {
        Ok(())
    } else {
        Err(ApiError::MessagesNotUnbound(response.unprocessed))
    }
}

pub async fn delete_messages(
    client: &Client,
    account: AccountUid,
    message_ids: &[MessageId],
) -> Result<(), ApiError> {
    let response: DeleteMessagesResponse =
        ApiRequest::new(client, Method::POST, DELETE_MESSAGES.to_owned())
            .json_body(AccountMessageBatch {
                account,
                message_ids,
            })
            .send()
            .await?;
    if message_ids.iter().all(|id| response.processed.contains(id)) {
        Ok(())
    } else {
        Err(ApiError::MessagesNotDeleted(response.unprocessed))
    }
}

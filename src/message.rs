use std::collections::BTreeMap;

use reqwest::Method;

use crate::client::{ApiError, ApiRequest, Client};
use crate::ids::{AccountUid, AttachmentId, BroadcastUid, MessageId, WorkflowId};

const GET_MESSAGES: &str = "/internal/message/messages/batch/get";
const UNBIND_MESSAGES: &str = "/internal/message/messages/batch/unbind";
const DELETE_MESSAGES: &str = "/internal/message/messages/batch/delete";
const MESSAGE: &str =
    "/internal/message/message-editor/v1/accounts/{account_id}/lists/{list_id}/messages/{message_id}";

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
    pub binding: Binding,
    /// Id of the resource named by `binding`.
    pub bound_resource_id: Option<String>,
    /// When copied from a broadcast, holds the source broadcast's scheduling settings.
    #[serde(skip_serializing)]
    pub copied_from: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(serde::Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Binding {
    Unbound,
    Broadcast,
    Followup,
    Campaign,
}

impl<'de> serde::Deserialize<'de> for Binding {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Binding, D::Error> {
        let text = <Option<String> as serde::Deserialize>::deserialize(deserializer)?;
        match text.as_deref() {
            None | Some("") | Some("unbound") => Ok(Binding::Unbound),
            Some("broadcast") => Ok(Binding::Broadcast),
            Some("followup") => Ok(Binding::Followup),
            Some("campaign") => Ok(Binding::Campaign),
            Some(other) => Err(serde::de::Error::custom(format!(
                "'{other}' is not a message binding"
            ))),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct LegacyMessage {
    #[serde(rename = "_id")]
    pub id: MessageId,
    pub account_id: i32,
    pub list_id: i32,
    pub subject: String,
    pub body_amp: String,
    pub body_text: String,
    pub body_html: String,
    pub body_json: serde_json::Map<String, serde_json::Value>,
    pub created_client: Option<String>,
    pub has_customized_body_text: bool,
    pub attachments: Vec<LegacyAttachment>,
    pub binding: Binding,
    pub bound_resource_id: Option<String>,
    pub modified_at: String,
    pub created_at: String,
    pub copied_from: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct LegacyAttachment {
    #[serde(rename = "_id")]
    pub id: AttachmentId,
    pub created_at: String,
    pub total_size: u64,
    pub filename: String,
}

#[derive(serde::Serialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageChanges {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_json: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_ids: Option<Vec<AttachmentId>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundResource {
    Broadcast(BroadcastUid),
    Campaign(WorkflowId),
}

#[derive(serde::Serialize)]
struct BindingChange {
    binding: Binding,
    #[serde(skip_serializing_if = "Option::is_none")]
    bound_resource_id: Option<String>,
}

fn message_path(account_id: i32, list_id: i32, message: &MessageId) -> String {
    MESSAGE
        .replace("{account_id}", &account_id.to_string())
        .replace("{list_id}", &list_id.to_string())
        .replace("{message_id}", &message.to_string())
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

pub async fn update_message(
    client: &Client,
    account_id: i32,
    list_id: i32,
    message: &MessageId,
    changes: &MessageChanges,
) -> Result<LegacyMessage, ApiError> {
    ApiRequest::new(
        client,
        Method::PATCH,
        message_path(account_id, list_id, message),
    )
    .json_body(changes)
    .send()
    .await
}

pub async fn bind_message(
    client: &Client,
    account_id: i32,
    list_id: i32,
    message: &MessageId,
    resource: &BoundResource,
) -> Result<LegacyMessage, ApiError> {
    let body = match resource {
        BoundResource::Broadcast(uid) => BindingChange {
            binding: Binding::Broadcast,
            bound_resource_id: Some(uid.to_string()),
        },
        BoundResource::Campaign(id) => BindingChange {
            binding: Binding::Campaign,
            bound_resource_id: Some(id.to_string()),
        },
    };
    ApiRequest::new(
        client,
        Method::PATCH,
        message_path(account_id, list_id, message),
    )
    .json_body(body)
    .send()
    .await
}

pub async fn unbind_message(
    client: &Client,
    account_id: i32,
    list_id: i32,
    message: &MessageId,
) -> Result<LegacyMessage, ApiError> {
    ApiRequest::new(
        client,
        Method::PATCH,
        message_path(account_id, list_id, message),
    )
    .json_body(BindingChange {
        binding: Binding::Unbound,
        bound_resource_id: None,
    })
    .send()
    .await
}

pub async fn delete_message(
    client: &Client,
    account_id: i32,
    list_id: i32,
    message: &MessageId,
) -> Result<(), ApiError> {
    ApiRequest::new(
        client,
        Method::DELETE,
        message_path(account_id, list_id, message),
    )
    .send_no_body()
    .await
}

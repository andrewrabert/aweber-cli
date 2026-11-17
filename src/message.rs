use std::collections::BTreeMap;

use reqwest::Method;

use crate::client::{ApiError, ApiRequest, Client};
use crate::ids::{AccountUid, AttachmentId, BroadcastUid, MessageId, WorkflowId};

const GET_MESSAGES: &str = "/internal/message/messages/batch/get";
const UNBIND_MESSAGES: &str = "/internal/message/messages/batch/unbind";
const DELETE_MESSAGES: &str = "/internal/message/messages/batch/delete";
const SEARCH_MESSAGES: &str = "/internal/message/messages/search";
const MESSAGES: &str =
    "/internal/message/message-editor/v1/accounts/{account_id}/lists/{list_id}/messages";
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
    pub attachment_ids: Vec<AttachmentId>,
    pub binding: Binding,
    /// Id of the resource named by `binding`.
    pub bound_resource_id: Option<String>,
    /// When copied from a broadcast, holds the source broadcast's scheduling settings.
    #[serde(skip_serializing)]
    pub copied_from: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Binding {
    Unbound,
    Broadcast,
    Followup,
    Campaign,
}

impl Binding {
    pub const ALL: [Binding; 4] = [
        Binding::Broadcast,
        Binding::Campaign,
        Binding::Followup,
        Binding::Unbound,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Binding::Unbound => "unbound",
            Binding::Broadcast => "broadcast",
            Binding::Followup => "followup",
            Binding::Campaign => "campaign",
        }
    }
}

impl std::str::FromStr for Binding {
    type Err = BindingError;

    fn from_str(text: &str) -> Result<Binding, BindingError> {
        Binding::ALL
            .into_iter()
            .find(|binding| binding.as_str() == text)
            .ok_or_else(|| BindingError {
                rejected: text.to_string(),
            })
    }
}

impl std::fmt::Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl serde::Serialize for Binding {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// The API sends `null` or `""` for an unbound message.
impl<'de> serde::Deserialize<'de> for Binding {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Binding, D::Error> {
        match <Option<String> as serde::Deserialize>::deserialize(deserializer)?.as_deref() {
            None | Some("") => Ok(Binding::Unbound),
            Some(text) => text.parse().map_err(serde::de::Error::custom),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BindingError {
    rejected: String,
}

impl std::fmt::Display for BindingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a message binding", self.rejected)
    }
}

impl std::error::Error for BindingError {}

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

/// The content of a message to create. The message starts unbound.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NewMessage {
    pub subject: String,
    pub body_text: String,
    pub body_html: String,
    pub body_json: serde_json::Map<String, serde_json::Value>,
    pub attachment_ids: Vec<AttachmentId>,
}

#[derive(serde::Serialize)]
struct NewMessageBody<'a> {
    account_id: i32,
    list_id: i32,
    subject: &'a str,
    body_text: &'a str,
    body_html: &'a str,
    body_json: &'a serde_json::Map<String, serde_json::Value>,
    attachment_ids: &'a [AttachmentId],
}

/// One entry of a list's message listing.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageSummary {
    #[serde(rename = "_id")]
    pub id: MessageId,
    pub subject: String,
    pub binding: Binding,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(serde::Deserialize)]
struct MessageSummaries {
    content: Vec<MessageSummary>,
}

/// Which messages [`find_messages`] returns. At least one field must be set.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageSearch {
    pub binding: Option<Binding>,
    pub bound_resource_id: Option<uuid::Uuid>,
    /// Every whitespace-separated word must appear in the subject.
    pub subject: Option<String>,
}

/// A field [`find_messages`] returns only on request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageField {
    AttachmentIds,
    Binding,
    BodyHtml,
    BodyJson,
    BodyText,
    BoundResourceId,
    CopiedFrom,
    HasCustomizedBodyText,
    Subject,
}

impl MessageField {
    pub const ALL: [MessageField; 9] = [
        MessageField::AttachmentIds,
        MessageField::Binding,
        MessageField::BodyHtml,
        MessageField::BodyJson,
        MessageField::BodyText,
        MessageField::BoundResourceId,
        MessageField::CopiedFrom,
        MessageField::HasCustomizedBodyText,
        MessageField::Subject,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            MessageField::AttachmentIds => "attachment_ids",
            MessageField::Binding => "binding",
            MessageField::BodyHtml => "body_html",
            MessageField::BodyJson => "body_json",
            MessageField::BodyText => "body_text",
            MessageField::BoundResourceId => "bound_resource_id",
            MessageField::CopiedFrom => "copied_from",
            MessageField::HasCustomizedBodyText => "has_customized_body_text",
            MessageField::Subject => "subject",
        }
    }
}

impl std::str::FromStr for MessageField {
    type Err = MessageFieldError;

    fn from_str(text: &str) -> Result<MessageField, MessageFieldError> {
        MessageField::ALL
            .into_iter()
            .find(|field| field.as_str() == text)
            .ok_or_else(|| MessageFieldError {
                rejected: text.to_string(),
            })
    }
}

#[derive(Clone, Debug)]
pub struct MessageFieldError {
    rejected: String,
}

impl std::fmt::Display for MessageFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a message field", self.rejected)
    }
}

impl std::error::Error for MessageFieldError {}

/// A search hit. Fields other than the ids are present only when requested.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct FoundMessage {
    pub id: MessageId,
    pub account_id: i32,
    pub list_id: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_json: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_customized_body_text: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_ids: Option<Vec<AttachmentId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binding: Option<Binding>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copied_from: Option<serde_json::Map<String, serde_json::Value>>,
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

fn messages_path(account_id: i32, list_id: i32) -> String {
    MESSAGES
        .replace("{account_id}", &account_id.to_string())
        .replace("{list_id}", &list_id.to_string())
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

pub async fn create_message(
    client: &Client,
    account_id: i32,
    list_id: i32,
    message: &NewMessage,
) -> Result<LegacyMessage, ApiError> {
    ApiRequest::new(client, Method::POST, messages_path(account_id, list_id))
        .json_body(NewMessageBody {
            account_id,
            list_id,
            subject: &message.subject,
            body_text: &message.body_text,
            body_html: &message.body_html,
            body_json: &message.body_json,
            attachment_ids: &message.attachment_ids,
        })
        .send()
        .await
}

/// The largest `limit` [`list_messages`] accepts.
pub const LIST_MESSAGES_PAGE: usize = 500;

/// Reads one page of a list's messages, newest first. `bindings` must not be
/// empty and `limit` must not exceed [`LIST_MESSAGES_PAGE`].
pub async fn list_messages(
    client: &Client,
    account_id: i32,
    list_id: i32,
    bindings: &[Binding],
    subject: Option<&str>,
    offset: usize,
    limit: usize,
) -> Result<Vec<MessageSummary>, ApiError> {
    let bindings: Vec<String> = bindings.iter().map(Binding::to_string).collect();
    let response: MessageSummaries =
        ApiRequest::new(client, Method::GET, messages_path(account_id, list_id))
            .query("binding", bindings.join(","))
            .query_opt("subject", subject)
            .query("offset", offset)
            .query("limit", limit)
            .send()
            .await?;
    Ok(response.content)
}

pub async fn find_messages(
    client: &Client,
    account: AccountUid,
    search: &MessageSearch,
    fields: &[MessageField],
) -> Result<Vec<FoundMessage>, ApiError> {
    let fields: Vec<&str> = fields.iter().map(|field| field.as_str()).collect();
    ApiRequest::new(client, Method::GET, SEARCH_MESSAGES.to_owned())
        .query("account", account)
        .query_opt("binding", search.binding)
        .query_opt("bound_resource_id", search.bound_resource_id)
        .query_opt("subject", search.subject.as_deref())
        .query_opt("fields", (!fields.is_empty()).then(|| fields.join(",")))
        .send()
        .await
}

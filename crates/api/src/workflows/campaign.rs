use reqwest::Method;

use crate::client::{ApiError, ApiRequest, Client};
use crate::ids::{AccountUid, ListUid, WorkflowId};
use crate::workflows::patch::{PatchOperation, WorkflowPatch};
use crate::workflows::ruleset::Ruleset;
use crate::workflows::values::{Timezone, UnknownStatus, WorkflowName, WorkflowStatus};

const CAMPAIGNS: &str = "/internal/campaign/campaigns";

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Workflow(serde_json::Value);

impl Workflow {
    pub fn id(&self) -> Option<WorkflowId> {
        self.text("id")?.parse().ok()
    }

    pub fn name(&self) -> Option<WorkflowName> {
        self.text("name")?.parse().ok()
    }

    pub fn status(&self) -> Result<WorkflowStatus, UnknownStatus> {
        self.text("state").unwrap_or_default().parse()
    }

    pub fn list(&self) -> Option<ListUid> {
        self.text("parent")?.parse().ok()
    }

    pub fn timezone(&self) -> Option<Timezone> {
        self.text("timezone")?.parse().ok()
    }

    pub fn sharing_enabled(&self) -> bool {
        self.0
            .get("sharing_enabled")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    }

    pub fn last_published(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.stamped("last_published_at")
            .or_else(|| self.stamped("published_at"))
    }

    pub fn updated(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.stamped("updated_at")
            .or_else(|| self.stamped("updated"))
    }

    pub fn errors(&self) -> Vec<String> {
        let reported = self
            .0
            .get("validation_problems")
            .or_else(|| self.0.get("problems"))
            .and_then(serde_json::Value::as_array);
        reported
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|entry| match entry {
                        serde_json::Value::String(text) => Some(text.clone()),
                        serde_json::Value::Object(object) => object
                            .get("detail")
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_string),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn precondition_version(&self) -> Option<PreconditionVersion> {
        self.0
            .get("precondition_version")
            .and_then(serde_json::Value::as_u64)
            .map(PreconditionVersion)
    }

    pub fn ruleset_write_op(&self) -> PatchOperation {
        let drafted = self
            .0
            .pointer("/ruleset")
            .and_then(serde_json::Value::as_object)
            .is_some_and(|ruleset| {
                ruleset.contains_key("unpublished_events")
                    || ruleset.contains_key("unpublished_actions")
            });
        if drafted {
            PatchOperation::Replace
        } else {
            PatchOperation::Add
        }
    }

    pub fn ruleset(&self) -> Result<Ruleset, RulesetError> {
        match self.0.get("ruleset") {
            None | Some(serde_json::Value::Null) => Ok(Ruleset::default()),
            Some(ruleset) => {
                serde_json::from_value(ruleset.clone()).map_err(|rejected| RulesetError {
                    rejected: rejected.to_string(),
                })
            }
        }
    }

    fn text(&self, member: &str) -> Option<&str> {
        self.0.get(member).and_then(serde_json::Value::as_str)
    }

    fn stamped(&self, member: &str) -> Option<chrono::DateTime<chrono::Utc>> {
        let text = self.text(member)?;
        chrono::DateTime::parse_from_rfc3339(text)
            .ok()
            .map(|stamped| stamped.with_timezone(&chrono::Utc))
    }
}

#[derive(serde::Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub struct PreconditionVersion(u64);

impl std::fmt::Display for PreconditionVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug)]
pub struct RulesetError {
    rejected: String,
}

impl std::fmt::Display for RulesetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the workflow's ruleset cannot be read: {}",
            self.rejected
        )
    }
}

impl std::error::Error for RulesetError {}

#[derive(serde::Serialize, Clone, Debug)]
pub struct CreateWorkflow {
    pub name: WorkflowName,
    pub owner: AccountUid,
    pub parent: ListUid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<Timezone>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct CopyWorkflow {
    #[serde(rename = "list")]
    pub target_list: ListUid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<WorkflowName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<Timezone>,
}

fn if_match<'a>(request: ApiRequest<'a>, precondition: PreconditionVersion) -> ApiRequest<'a> {
    request.header(reqwest::header::IF_MATCH, precondition.to_string())
}

pub async fn list_workflows(
    client: &Client,
    owner: AccountUid,
    parent: ListUid,
) -> Result<Vec<Workflow>, ApiError> {
    let document: serde_json::Value = ApiRequest::new(client, Method::GET, CAMPAIGNS.into())
        .query("owner", owner)
        .query("parent", parent)
        .send()
        .await?;
    let entries = match document {
        serde_json::Value::Array(entries) => entries,
        serde_json::Value::Object(mut object) => match object.remove("entries") {
            Some(serde_json::Value::Array(entries)) => entries,
            _ => Vec::new(),
        },
        _ => Vec::new(),
    };
    Ok(entries.into_iter().map(Workflow).collect())
}

pub async fn get_workflow(client: &Client, workflow: WorkflowId) -> Result<Workflow, ApiError> {
    ApiRequest::new(client, Method::GET, format!("{CAMPAIGNS}/{workflow}"))
        .query("include", "ruleset")
        .send()
        .await
}

pub async fn create_workflow(client: &Client, body: &CreateWorkflow) -> Result<Workflow, ApiError> {
    ApiRequest::new(client, Method::POST, CAMPAIGNS.into())
        .json_body(body)
        .send()
        .await
}

pub async fn update_workflow(
    client: &Client,
    workflow: WorkflowId,
    precondition: PreconditionVersion,
    patch: &WorkflowPatch,
) -> Result<Workflow, ApiError> {
    let mut request = ApiRequest::new(client, Method::PATCH, format!("{CAMPAIGNS}/{workflow}"))
        .query("include", "ruleset");
    if patch.sets_timezone() {
        request = request.query("update_times", true);
    }
    if_match(request, precondition)
        .json_body(patch)
        .send()
        .await
}

pub async fn delete_workflow(
    client: &Client,
    workflow: WorkflowId,
    precondition: PreconditionVersion,
) -> Result<(), ApiError> {
    if_match(
        ApiRequest::new(client, Method::DELETE, format!("{CAMPAIGNS}/{workflow}")),
        precondition,
    )
    .send_no_body()
    .await
}

pub async fn publish_workflow(
    client: &Client,
    workflow: WorkflowId,
    precondition: PreconditionVersion,
) -> Result<Workflow, ApiError> {
    if_match(
        ApiRequest::new(
            client,
            Method::POST,
            format!("{CAMPAIGNS}/{workflow}/publish"),
        ),
        precondition,
    )
    .send()
    .await
}

pub async fn revert_workflow(
    client: &Client,
    workflow: WorkflowId,
    precondition: PreconditionVersion,
) -> Result<Workflow, ApiError> {
    if_match(
        ApiRequest::new(
            client,
            Method::POST,
            format!("{CAMPAIGNS}/{workflow}/revert"),
        ),
        precondition,
    )
    .send()
    .await
}

pub async fn copy_workflow(
    client: &Client,
    workflow: WorkflowId,
    precondition: PreconditionVersion,
    body: &CopyWorkflow,
) -> Result<Workflow, ApiError> {
    if_match(
        ApiRequest::new(client, Method::POST, format!("{CAMPAIGNS}/{workflow}/copy")),
        precondition,
    )
    .json_body(body)
    .send()
    .await
}

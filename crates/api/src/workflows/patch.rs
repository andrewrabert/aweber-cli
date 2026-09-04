use crate::workflows::graph::{Graph, Slot};
use crate::workflows::values::{Sharing, StatusChange, Timezone, WorkflowName};

#[derive(serde::Serialize, Clone, Debug, Default)]
#[serde(transparent)]
pub struct WorkflowPatch(Vec<PatchOp>);

impl WorkflowPatch {
    pub fn edits(edit: &WorkflowEdit) -> WorkflowPatch {
        let mut ops = Vec::new();
        if let Some(name) = &edit.name {
            ops.push(PatchOp::replace(WorkflowPatchPath::Name, value_of(name)));
        }
        if let Some(sharing) = &edit.sharing {
            ops.push(PatchOp::replace(
                WorkflowPatchPath::SharingEnabled,
                value_of(sharing),
            ));
        }
        if let Some(status) = &edit.status {
            ops.push(PatchOp::replace(WorkflowPatchPath::State, value_of(status)));
        }
        if let Some(timezone) = &edit.timezone {
            ops.push(PatchOp::replace(
                WorkflowPatchPath::Timezone,
                value_of(timezone),
            ));
        }
        WorkflowPatch(ops)
    }

    pub fn with_ruleset(mut self, op: PatchOperation, graph: &Graph) -> WorkflowPatch {
        let mut events = Vec::new();
        let mut actions = Vec::new();
        for owned in graph.rules() {
            let rule = value_of(&owned.rule);
            match owned.slot {
                Slot::Event | Slot::AutomationEvent => events.push(rule),
                Slot::Action => actions.push(rule),
            }
        }
        self.0.push(PatchOp {
            op,
            path: WorkflowPatchPath::UnpublishedActions,
            value: Some(serde_json::Value::Array(actions)),
        });
        self.0.push(PatchOp {
            op,
            path: WorkflowPatchPath::UnpublishedEvents,
            value: Some(serde_json::Value::Array(events)),
        });
        self
    }

    pub(crate) fn sets_timezone(&self) -> bool {
        self.0
            .iter()
            .any(|op| op.path == WorkflowPatchPath::Timezone)
    }
}

#[derive(Clone, Debug, Default)]
pub struct WorkflowEdit {
    pub name: Option<WorkflowName>,
    pub status: Option<StatusChange>,
    pub timezone: Option<Timezone>,
    pub sharing: Option<Sharing>,
}

#[derive(serde::Serialize, Clone, Debug)]
struct PatchOp {
    op: PatchOperation,
    path: WorkflowPatchPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<serde_json::Value>,
}

impl PatchOp {
    fn replace(path: WorkflowPatchPath, value: serde_json::Value) -> PatchOp {
        PatchOp {
            op: PatchOperation::Replace,
            path,
            value: Some(value),
        }
    }
}

#[derive(serde::Serialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PatchOperation {
    Add,
    Replace,
}

#[derive(serde::Serialize, Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum WorkflowPatchPath {
    #[serde(rename = "/name")]
    Name,
    #[serde(rename = "/sharing_enabled")]
    SharingEnabled,
    #[serde(rename = "/state")]
    State,
    #[serde(rename = "/timezone")]
    Timezone,
    #[serde(rename = "/ruleset/unpublished_actions")]
    UnpublishedActions,
    #[serde(rename = "/ruleset/unpublished_events")]
    UnpublishedEvents,
}

fn value_of<T: serde::Serialize>(value: &T) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

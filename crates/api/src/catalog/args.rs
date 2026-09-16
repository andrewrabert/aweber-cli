use std::any::TypeId;
use std::collections::BTreeMap;

use crate::catalog::Operation;
use crate::catalog::plan::PRECONDITION_ARG;

/// What an argument's text means once it reaches a request.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ValueKind {
    Text,
    Integer,
    Decimal,
    Bool,
    Flag,
    Date,
    DateTime,
    Uuid,
    Path,
    Enum(Vec<String>),
    Json,
}

/// Where an argument belongs in a front end that builds requests.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ArgRole {
    Filter,
    Identifier,
    /// Pagination is driven by the collection loader, never by the user.
    Pagination,
    /// Entered through `$EDITOR`.
    JsonBody,
    /// Filled from the current document, hidden from the builder.
    Reserved,
    /// Client-side only; never reaches a request.
    Excluded,
}

#[derive(Clone, Debug)]
pub struct ArgSpec {
    pub name: String,
    /// The flag spelling, meaningless where `positional` is set.
    pub long: String,
    /// Written as a bare value in its declared place, never as `--name`.
    pub positional: bool,
    pub help: Option<String>,
    pub kind: ValueKind,
    pub role: ArgRole,
    pub required: bool,
    pub repeatable: bool,
    pub default: Option<String>,
    /// The other members of this argument's required-one-of group.
    pub alternatives: Vec<String>,
}

/// One argument's text, remembered with the kind it was entered as.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ArgValue {
    pub kind: ValueKind,
    pub raw: String,
}

impl ArgValue {
    pub fn new(kind: ValueKind, raw: impl Into<String>) -> ArgValue {
        ArgValue {
            kind,
            raw: raw.into(),
        }
    }

    pub fn text(raw: impl Into<String>) -> ArgValue {
        ArgValue::new(ValueKind::Text, raw)
    }

    /// `Bool`, `Integer`, and `Decimal` become JSON literals; everything else a string.
    pub fn json(&self) -> serde_json::Value {
        match self.kind {
            ValueKind::Bool | ValueKind::Flag => match self.raw.as_str() {
                "true" => serde_json::Value::Bool(true),
                "false" => serde_json::Value::Bool(false),
                other => serde_json::Value::String(other.to_string()),
            },
            ValueKind::Integer => self
                .raw
                .parse::<i64>()
                .map(serde_json::Value::from)
                .unwrap_or_else(|_| serde_json::Value::String(self.raw.clone())),
            ValueKind::Decimal => self
                .raw
                .parse::<f64>()
                .map(serde_json::Value::from)
                .unwrap_or_else(|_| serde_json::Value::String(self.raw.clone())),
            ValueKind::Json => serde_json::from_str(&self.raw)
                .unwrap_or_else(|_| serde_json::Value::String(self.raw.clone())),
            _ => serde_json::Value::String(self.raw.clone()),
        }
    }
}

/// The arguments a request is built from, in argument-name order.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Args(BTreeMap<String, Vec<ArgValue>>);

impl Args {
    pub fn set(&mut self, name: impl Into<String>, value: ArgValue) {
        self.0.insert(name.into(), vec![value]);
    }

    pub fn push(&mut self, name: impl Into<String>, value: ArgValue) {
        self.0.entry(name.into()).or_default().push(value);
    }

    pub fn remove(&mut self, name: &str) {
        self.0.remove(name);
    }

    pub fn first(&self, name: &str) -> Option<&ArgValue> {
        self.0.get(name).and_then(|values| values.first())
    }

    pub fn all(&self, name: &str) -> &[ArgValue] {
        self.0.get(name).map_or(&[], Vec::as_slice)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &[ArgValue])> {
        self.0
            .iter()
            .map(|(name, values)| (name.as_str(), values.as_slice()))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// `limit` is `Excluded`; the cursor and window arguments are `Pagination`;
/// `json-body`, `file`, and `patch` are `JsonBody`; `precondition-version` is
/// `Reserved`; arguments named by a required identifier group are `Identifier`;
/// the rest are `Filter`.
pub(super) fn role(name: &str, operation: Operation) -> ArgRole {
    const PAGINATION: [&str; 8] = [
        "ws-size",
        "ws-start",
        "page-size",
        "after",
        "before",
        "links-cursor",
        "send-instances-cursor",
        "start-token",
    ];
    if name == "limit" {
        return ArgRole::Excluded;
    }
    if PAGINATION.contains(&name) {
        return ArgRole::Pagination;
    }
    if matches!(name, "json-body" | "file" | "patch") {
        return ArgRole::JsonBody;
    }
    if name == PRECONDITION_ARG {
        return ArgRole::Reserved;
    }
    if identifiers(operation).contains(&name.to_string()) {
        return ArgRole::Identifier;
    }
    ArgRole::Filter
}

/// The arguments that name the entity an operation acts on.
fn identifiers(operation: Operation) -> Vec<String> {
    let command = operation.command();
    let mut named: Vec<String> = command
        .get_groups()
        .filter(|group| group.is_required_set())
        .flat_map(|group| group.get_args())
        .map(|id| id.to_string())
        .collect();
    for arg in command.get_arguments() {
        let name = arg.get_id().to_string();
        if name == "workflow" || (name.ends_with("-id") && name != "message-id") {
            named.push(name);
        }
    }
    named
}

/// Every argument of the operation's clap command, plus the reserved ones a plan needs.
pub(super) fn specs(operation: Operation) -> Vec<ArgSpec> {
    let command = operation.command();
    let groups: Vec<(Vec<String>, bool)> = command
        .get_groups()
        .map(|group| {
            (
                group.get_args().map(ToString::to_string).collect(),
                group.is_required_set(),
            )
        })
        .collect();
    let mut specs: Vec<ArgSpec> = command
        .get_arguments()
        .map(|arg| {
            let name = arg.get_id().to_string();
            let alternatives = groups
                .iter()
                .filter(|(members, required)| *required && members.contains(&name))
                .flat_map(|(members, _)| members.iter().cloned())
                .filter(|member| *member != name)
                .collect();
            let role = role(&name, operation);
            ArgSpec {
                long: arg.get_long().unwrap_or(&name).to_string(),
                positional: arg.get_long().is_none() && arg.get_short().is_none(),
                help: arg.get_help().map(ToString::to_string),
                kind: kind(arg, role),
                role,
                required: arg.is_required_set(),
                repeatable: matches!(arg.get_action(), clap::ArgAction::Append),
                default: arg
                    .get_default_values()
                    .first()
                    .map(|value| value.to_string_lossy().into_owned()),
                alternatives,
                name,
            }
        })
        .collect();
    if PRECONDITIONED.contains(&operation) {
        specs.push(ArgSpec {
            name: PRECONDITION_ARG.to_string(),
            long: PRECONDITION_ARG.to_string(),
            positional: false,
            help: Some("The version the edit was made against".to_string()),
            kind: ValueKind::Integer,
            role: ArgRole::Reserved,
            required: false,
            repeatable: false,
            default: None,
            alternatives: Vec::new(),
        });
    }
    specs
}

/// The operations whose request carries an `If-Match` precondition.
pub(super) const PRECONDITIONED: [Operation; 7] = [
    Operation::UpdateWorkflow,
    Operation::UpdateWorkflowRuleset,
    Operation::SetWorkflowState,
    Operation::DeleteWorkflow,
    Operation::PublishWorkflow,
    Operation::RevertWorkflow,
    Operation::CopyWorkflow,
];

/// The kind is read off the type the argument's own parser produces.
fn kind(arg: &clap::Arg, role: ArgRole) -> ValueKind {
    if matches!(
        arg.get_action(),
        clap::ArgAction::SetTrue | clap::ArgAction::SetFalse
    ) {
        return ValueKind::Flag;
    }
    if role == ArgRole::JsonBody {
        return ValueKind::Json;
    }
    let possible: Vec<String> = arg
        .get_possible_values()
        .iter()
        .map(|value| value.get_name().to_string())
        .collect();
    if !possible.is_empty() {
        return ValueKind::Enum(possible);
    }
    let produced = arg.get_value_parser().type_id();
    let is = |id: std::any::TypeId| produced == id;
    if is(TypeId::of::<bool>()) {
        return ValueKind::Bool;
    }
    if is(TypeId::of::<i8>())
        || is(TypeId::of::<u8>())
        || is(TypeId::of::<i16>())
        || is(TypeId::of::<u16>())
        || is(TypeId::of::<i32>())
        || is(TypeId::of::<u32>())
        || is(TypeId::of::<i64>())
        || is(TypeId::of::<u64>())
        || is(TypeId::of::<usize>())
        || is(TypeId::of::<isize>())
        || is(TypeId::of::<std::num::NonZeroU32>())
        || is(TypeId::of::<std::num::NonZeroU64>())
    {
        return ValueKind::Integer;
    }
    if is(TypeId::of::<f32>()) || is(TypeId::of::<f64>()) {
        return ValueKind::Decimal;
    }
    if is(TypeId::of::<std::path::PathBuf>()) {
        return ValueKind::Path;
    }
    if is(TypeId::of::<uuid::Uuid>()) {
        return ValueKind::Uuid;
    }
    if is(TypeId::of::<chrono::NaiveDate>()) {
        return ValueKind::Date;
    }
    if is(TypeId::of::<chrono::DateTime<chrono::Utc>>())
        || is(TypeId::of::<chrono::DateTime<chrono::FixedOffset>>())
    {
        return ValueKind::DateTime;
    }
    ValueKind::Text
}

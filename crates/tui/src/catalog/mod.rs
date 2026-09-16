//! Palette entries and builder rows are read off `aweber::catalog`; nothing
//! about an operation is written here.

mod metadata;

pub use metadata::{
    Column, ConfirmationTier, ContextSource, EditKind, EntityKind, Fill, Invalidation, Metadata,
    OperationKind, metadata,
};

use aweber::catalog::{ArgRole, ArgSpec, Operation};

pub struct Entry {
    pub operation: Operation,
    pub label: String,
    pub about: &'static str,
    pub specs: Vec<ArgSpec>,
    pub metadata: &'static Metadata,
}

fn entry_of(operation: Operation) -> Entry {
    Entry {
        operation,
        label: format!("{} {}", operation.group(), operation.action()),
        about: operation.about(),
        specs: operation.specs(),
        metadata: metadata(operation),
    }
}

/// The sixty-nine palette operations, in route-table order.
pub fn entries() -> &'static [Entry] {
    static ENTRIES: std::sync::OnceLock<Vec<Entry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| {
        aweber::catalog::groups()
            .iter()
            .filter(|group| !group.hidden)
            .flat_map(|group| group.operations.iter().copied())
            .map(entry_of)
            .collect()
    })
}

/// The four hidden `oauth` operations, offered only on the Session view.
pub fn session_entries() -> &'static [Entry] {
    static ENTRIES: std::sync::OnceLock<Vec<Entry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| {
        Operation::ALL
            .into_iter()
            .filter(|operation| operation.is_hidden())
            .map(entry_of)
            .collect()
    })
}

pub fn entry(operation: Operation) -> &'static Entry {
    static ALL: std::sync::OnceLock<Vec<Entry>> = std::sync::OnceLock::new();
    let all = ALL.get_or_init(|| Operation::ALL.into_iter().map(entry_of).collect());
    &all[Operation::ALL
        .iter()
        .position(|candidate| *candidate == operation)
        .expect("every operation is in Operation::ALL")]
}

/// The operations whose metadata accepts this selection's entity kind.
pub fn for_selection(kind: EntityKind) -> Vec<&'static Entry> {
    entries()
        .iter()
        .filter(|entry| entry.metadata.entity == kind)
        .collect()
}

/// The collection a builder row chooses from, and what a choice resolves to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Picker {
    pub operation: Operation,
    pub resolution: Resolution,
    /// The JSON pointer whose value labels a candidate.
    pub label: &'static str,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Resolution {
    /// `list-link` and `segment-link` take the entity's `self_link`.
    SelfLink,
    /// `list` and `workflow` take its uuid.
    Uuid,
    /// Every `*-id` argument takes its id.
    Id,
}

impl Resolution {
    /// The pointer a chosen candidate's value is read from.
    pub fn pointer(self) -> &'static str {
        match self {
            Resolution::SelfLink => "/self_link",
            Resolution::Uuid => "/uuid",
            Resolution::Id => "/id",
        }
    }

    /// Whether typed text is already a resolved value of this shape.
    pub fn accepts(self, text: &str) -> bool {
        match self {
            Resolution::SelfLink => text.starts_with("http://") || text.starts_with("https://"),
            Resolution::Uuid => text.parse::<uuid::Uuid>().is_ok(),
            Resolution::Id => !text.is_empty() && text.chars().all(|c| c.is_ascii_digit()),
        }
    }
}

/// The picker of an identifier argument that names another entity; every other
/// argument is a typed row.
pub fn picker(operation: Operation, argument: &str) -> Option<Picker> {
    let _ = operation;
    let (collection, resolution) = match argument {
        "list-link" => (Operation::ListLists, Resolution::SelfLink),
        "segment-link" => (Operation::ListSegments, Resolution::SelfLink),
        "list" => (Operation::ListLists, Resolution::Uuid),
        "workflow" => (Operation::ListWorkflows, Resolution::Uuid),
        "list-id" => (Operation::ListLists, Resolution::Id),
        "subscriber-id" => (Operation::ListSubscribers, Resolution::Id),
        "broadcast-id" => (Operation::ListBroadcasts, Resolution::Id),
        "segment-id" => (Operation::ListSegments, Resolution::Id),
        "tag-id" => (Operation::ListTags, Resolution::Id),
        "custom-field-id" => (Operation::ListCustomFields, Resolution::Id),
        "campaign-id" => (Operation::ListCampaigns, Resolution::Id),
        _ => return None,
    };
    Some(Picker {
        operation: collection,
        resolution,
        label: label_pointer(collection),
    })
}

/// The pointer that names a candidate of a collection.
fn label_pointer(operation: Operation) -> &'static str {
    match operation {
        Operation::ListSubscribers => "/email",
        _ => "/name",
    }
}

/// The rows a builder starts with: required arguments plus context-filled
/// identifiers.
pub fn opening_rows(operation: Operation) -> Vec<ArgSpec> {
    let filled: Vec<&str> = metadata(operation)
        .fills
        .iter()
        .map(|fill| fill.arg)
        .collect();
    operation
        .specs()
        .into_iter()
        .filter(|spec| {
            !matches!(spec.role, ArgRole::Excluded | ArgRole::Pagination)
                && (spec.required || filled.contains(&spec.name.as_str()))
        })
        .collect()
}

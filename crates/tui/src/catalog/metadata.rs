//! What the front end needs to know about an operation that clap cannot tell
//! it: the entity it acts on, the columns a collection shows, the identifiers
//! the context fills, and how loudly a mutation must be confirmed.

use aweber::catalog::Operation;

pub struct Metadata {
    pub kind: OperationKind,
    pub entity: EntityKind,
    /// The collection columns, by glossary label and JSON pointer.
    pub columns: &'static [Column],
    pub fills: &'static [Fill],
    pub tier: ConfirmationTier,
    pub invalidates: &'static [Invalidation],
    pub editor: Option<EditKind>,
    /// `broadcasts wait` alone.
    pub watch: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OperationKind {
    Collection,
    Document,
    Search,
    Mutation,
    Watch,
    Tree,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EntityKind {
    Account,
    List,
    Subscriber,
    Broadcast,
    Campaign,
    Workflow,
    CustomField,
    Tag,
    Segment,
    Integration,
    LandingPage,
    Purchase,
    WebForm,
    WebFormSplitTest,
    WebFormSplitTestComponent,
    Stat,
    Oauth,
}

pub struct Column {
    pub label: &'static str,
    pub pointer: &'static str,
}

pub struct Fill {
    pub arg: &'static str,
    pub source: ContextSource,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ContextSource {
    AccountId,
    AccountUuid,
    ListId,
    ListUuid,
    SelectionId,
    SelectionUuid,
    SelectionSelfLink,
    SelectionEmail,
    SelectionMessageId,
    SelectionEventId,
    SelectionPrecondition,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConfirmationTier {
    /// Reads.
    None,
    YesNo,
    /// Typed back verbatim.
    Phrase(&'static str),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Invalidation {
    Selection,
    ParentCollection,
    Operation(Operation),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EditKind {
    Ruleset,
    JsonPatch,
    JsonBody,
}

const NO_COLUMNS: &[Column] = &[];
const NO_FILLS: &[Fill] = &[];
const NO_INVALIDATION: &[Invalidation] = &[];

const LIST_FILL: &[Fill] = &[Fill {
    arg: "list-id",
    source: ContextSource::ListId,
}];

const LIST_UUID_FILL: &[Fill] = &[Fill {
    arg: "list",
    source: ContextSource::ListUuid,
}];

const SELECTION_ONLY: &[Invalidation] = &[Invalidation::Selection];
const PARENT_ONLY: &[Invalidation] = &[Invalidation::ParentCollection];
const SELECTION_AND_PARENT: &[Invalidation] =
    &[Invalidation::Selection, Invalidation::ParentCollection];

macro_rules! fills {
    ($(($arg:expr, $source:ident)),* $(,)?) => {
        &[$(Fill { arg: $arg, source: ContextSource::$source }),*]
    };
}

macro_rules! columns {
    ($(($label:expr, $pointer:expr)),* $(,)?) => {
        &[$(Column { label: $label, pointer: $pointer }),*]
    };
}

/// Exhaustive over `Operation`, so a new route cannot compile without its
/// metadata. `Phrase` covers every delete, `workflows revert`, and
/// `oauth revoke`; `YesNo` covers every other mutation.
pub fn metadata(operation: Operation) -> &'static Metadata {
    use ConfirmationTier::{None as NoConfirm, Phrase, YesNo};
    use EntityKind as E;
    use OperationKind::{Collection, Document, Mutation, Search, Tree, Watch};

    macro_rules! meta {
        (
            $kind:expr, $entity:expr,
            columns: $columns:expr,
            fills: $fills:expr,
            tier: $tier:expr,
            invalidates: $invalidates:expr,
            editor: $editor:expr,
            watch: $watch:expr
        ) => {{
            static METADATA: Metadata = Metadata {
                kind: $kind,
                entity: $entity,
                columns: $columns,
                fills: $fills,
                tier: $tier,
                invalidates: $invalidates,
                editor: $editor,
                watch: $watch,
            };
            &METADATA
        }};
    }

    macro_rules! read {
        ($kind:expr, $entity:expr, $columns:expr, $fills:expr) => {
            meta!(
                $kind, $entity,
                columns: $columns,
                fills: $fills,
                tier: NoConfirm,
                invalidates: NO_INVALIDATION,
                editor: Option::None,
                watch: false
            )
        };
    }

    macro_rules! write {
        ($entity:expr, $fills:expr, $tier:expr, $invalidates:expr, $editor:expr) => {
            meta!(
                Mutation, $entity,
                columns: NO_COLUMNS,
                fills: $fills,
                tier: $tier,
                invalidates: $invalidates,
                editor: $editor,
                watch: false
            )
        };
    }

    match operation {
        Operation::ListAccounts => read!(
            Collection,
            E::Account,
            columns!(("Account", "/id"), ("Company", "/company")),
            NO_FILLS
        ),
        Operation::GetAccount => read!(Document, E::Account, NO_COLUMNS, NO_FILLS),
        Operation::FindAccountSubscribers => {
            read!(Search, E::Subscriber, SUBSCRIBER_COLUMNS, NO_FILLS)
        }
        Operation::ListAccountWebformSplitTests => {
            read!(
                Collection,
                E::WebFormSplitTest,
                SPLIT_TEST_COLUMNS,
                NO_FILLS
            )
        }
        Operation::ListAccountWebforms => {
            read!(Collection, E::WebForm, WEBFORM_COLUMNS, NO_FILLS)
        }
        Operation::ListIntegrations => read!(
            Collection,
            E::Integration,
            columns!(("Integration", "/id"), ("Service", "/service_name")),
            NO_FILLS
        ),
        Operation::GetIntegration => read!(
            Document,
            E::Integration,
            NO_COLUMNS,
            fills!(("integration-id", SelectionId))
        ),
        Operation::ListLists => read!(Collection, E::List, LIST_COLUMNS, NO_FILLS),
        Operation::FindLists => read!(Search, E::List, LIST_COLUMNS, NO_FILLS),
        Operation::GetList => read!(
            Document,
            E::List,
            NO_COLUMNS,
            fills!(("list-id", SelectionId))
        ),
        Operation::ListBroadcasts => {
            read!(Collection, E::Broadcast, BROADCAST_COLUMNS, LIST_FILL)
        }
        Operation::CreateBroadcast => write!(
            E::Broadcast,
            LIST_FILL,
            YesNo,
            PARENT_ONLY,
            Some(EditKind::JsonBody)
        ),
        Operation::GetBroadcastTotal => read!(Document, E::Broadcast, NO_COLUMNS, LIST_FILL),
        Operation::GetBroadcast => read!(
            Document,
            E::Broadcast,
            NO_COLUMNS,
            fills!(("list-id", ListId), ("broadcast-id", SelectionId))
        ),
        Operation::UpdateBroadcast => write!(
            E::Broadcast,
            fills!(("list-id", ListId), ("broadcast-id", SelectionId)),
            YesNo,
            SELECTION_AND_PARENT,
            Some(EditKind::JsonBody)
        ),
        Operation::DeleteBroadcast => write!(
            E::Broadcast,
            fills!(("list-id", ListId), ("broadcast-id", SelectionId)),
            Phrase("delete broadcast"),
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::CancelBroadcast => write!(
            E::Broadcast,
            fills!(("list-id", ListId), ("broadcast-id", SelectionId)),
            YesNo,
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::GetBroadcastClicks => read!(
            Collection,
            E::Stat,
            columns!(("Link", "/url"), ("Clicks", "/count")),
            fills!(("list-id", ListId), ("broadcast-id", SelectionId))
        ),
        Operation::GetBroadcastOpens => read!(
            Collection,
            E::Stat,
            columns!(("Subscriber", "/subscriber_id"), ("Opened", "/event_time")),
            fills!(("list-id", ListId), ("broadcast-id", SelectionId))
        ),
        Operation::ScheduleBroadcast => write!(
            E::Broadcast,
            fills!(("list-id", ListId), ("broadcast-id", SelectionId)),
            YesNo,
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::WaitBroadcast => meta!(
            Watch, E::Broadcast,
            columns: NO_COLUMNS,
            fills: fills!(("list-id", ListId), ("broadcast-id", SelectionId)),
            tier: NoConfirm,
            invalidates: NO_INVALIDATION,
            editor: Option::None,
            watch: true
        ),
        Operation::ListCampaigns => read!(Collection, E::Campaign, CAMPAIGN_COLUMNS, LIST_FILL),
        Operation::ListCampaignStats => read!(
            Collection,
            E::Stat,
            STAT_COLUMNS,
            fills!(("list-id", ListId), ("campaign-id", SelectionId))
        ),
        Operation::GetCampaignStat => read!(
            Document,
            E::Stat,
            NO_COLUMNS,
            fills!(("list-id", ListId), ("campaign-id", SelectionId))
        ),
        Operation::FindCampaigns => read!(Search, E::Campaign, CAMPAIGN_COLUMNS, LIST_FILL),
        Operation::GetCampaign => read!(
            Document,
            E::Campaign,
            NO_COLUMNS,
            fills!(("list-id", ListId), ("campaign-id", SelectionId))
        ),
        Operation::ListCustomFields => read!(
            Collection,
            E::CustomField,
            columns!(("Field", "/name"), ("Id", "/id")),
            LIST_FILL
        ),
        Operation::CreateCustomField => {
            write!(E::CustomField, LIST_FILL, YesNo, PARENT_ONLY, Option::None)
        }
        Operation::GetCustomField => read!(
            Document,
            E::CustomField,
            NO_COLUMNS,
            fills!(("list-id", ListId), ("custom-field-id", SelectionId))
        ),
        Operation::DeleteCustomField => write!(
            E::CustomField,
            fills!(("list-id", ListId), ("custom-field-id", SelectionId)),
            Phrase("delete custom field"),
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::UpdateCustomField => write!(
            E::CustomField,
            fills!(("list-id", ListId), ("custom-field-id", SelectionId)),
            YesNo,
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::ListLandingPages => read!(
            Collection,
            E::LandingPage,
            columns!(("Landing page", "/name"), ("Status", "/status")),
            LIST_FILL
        ),
        Operation::GetLandingPage => read!(
            Document,
            E::LandingPage,
            NO_COLUMNS,
            fills!(("list-id", ListId), ("landing-page-id", SelectionUuid))
        ),
        Operation::CreatePurchase => {
            write!(E::Purchase, LIST_FILL, YesNo, PARENT_ONLY, Option::None)
        }
        Operation::ListSegments => read!(
            Collection,
            E::Segment,
            columns!(("Segment", "/name"), ("Id", "/id")),
            LIST_FILL
        ),
        Operation::GetSegment => read!(
            Document,
            E::Segment,
            NO_COLUMNS,
            fills!(("list-id", ListId), ("segment-id", SelectionId))
        ),
        Operation::ListSubscribers => {
            read!(Collection, E::Subscriber, SUBSCRIBER_COLUMNS, LIST_FILL)
        }
        Operation::CreateSubscriber => {
            write!(E::Subscriber, LIST_FILL, YesNo, PARENT_ONLY, Option::None)
        }
        Operation::DeleteSubscriberByEmail => write!(
            E::Subscriber,
            fills!(("list-id", ListId), ("email", SelectionEmail)),
            Phrase("delete subscriber"),
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::UpdateSubscriberByEmail => write!(
            E::Subscriber,
            fills!(("list-id", ListId), ("email", SelectionEmail)),
            YesNo,
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::FindSubscribers => {
            read!(Search, E::Subscriber, SUBSCRIBER_COLUMNS, LIST_FILL)
        }
        Operation::GetSubscriber => read!(
            Document,
            E::Subscriber,
            NO_COLUMNS,
            fills!(("list-id", ListId), ("subscriber-id", SelectionId))
        ),
        Operation::MoveSubscriber => write!(
            E::Subscriber,
            fills!(("list-id", ListId), ("subscriber-id", SelectionId)),
            YesNo,
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::DeleteSubscriber => write!(
            E::Subscriber,
            fills!(("list-id", ListId), ("subscriber-id", SelectionId)),
            Phrase("delete subscriber"),
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::UpdateSubscriber => write!(
            E::Subscriber,
            fills!(("list-id", ListId), ("subscriber-id", SelectionId)),
            YesNo,
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::UnsubscribeSubscriber => write!(
            E::Subscriber,
            fills!(("list-id", ListId), ("subscriber-id", SelectionId)),
            YesNo,
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::GetSubscriberActivity => read!(
            Collection,
            E::Stat,
            columns!(("Activity", "/type"), ("When", "/event_time")),
            fills!(("list-id", ListId), ("subscriber-id", SelectionId))
        ),
        Operation::ListTags => read!(Collection, E::Tag, columns!(("Tag", "")), LIST_FILL),
        Operation::ListWebFormSplitTests => {
            read!(
                Collection,
                E::WebFormSplitTest,
                SPLIT_TEST_COLUMNS,
                LIST_FILL
            )
        }
        Operation::GetWebFormSplitTest => read!(
            Document,
            E::WebFormSplitTest,
            NO_COLUMNS,
            fills!(("list-id", ListId), ("split-test-id", SelectionId))
        ),
        Operation::ListWebFormSplitTestComponents => read!(
            Collection,
            E::WebFormSplitTestComponent,
            columns!(("Component", "/id"), ("Weight", "/weight")),
            fills!(("list-id", ListId), ("split-test-id", SelectionId))
        ),
        Operation::GetWebFormSplitTestComponent => read!(
            Document,
            E::WebFormSplitTestComponent,
            NO_COLUMNS,
            fills!(
                ("list-id", ListId),
                ("split-test-component-id", SelectionId)
            )
        ),
        Operation::ListWebForms => read!(Collection, E::WebForm, WEBFORM_COLUMNS, LIST_FILL),
        Operation::GetWebForm => read!(
            Document,
            E::WebForm,
            NO_COLUMNS,
            fills!(("list-id", ListId), ("webform-id", SelectionId))
        ),
        Operation::GetBroadcastLinkAnalytics => read!(
            Collection,
            E::Stat,
            columns!(("Link", "/url"), ("Count", "/count")),
            fills!(("broadcast-id", SelectionUuid))
        ),
        Operation::ListWorkflows => {
            read!(Collection, E::Workflow, WORKFLOW_COLUMNS, LIST_UUID_FILL)
        }
        Operation::GetWorkflow => read!(
            Document,
            E::Workflow,
            NO_COLUMNS,
            fills!(("workflow", SelectionUuid))
        ),
        Operation::TreeWorkflow => meta!(
            Tree, E::Workflow,
            columns: NO_COLUMNS,
            fills: fills!(("workflow", SelectionUuid)),
            tier: NoConfirm,
            invalidates: NO_INVALIDATION,
            editor: Option::None,
            watch: false
        ),
        Operation::CreateWorkflow => write!(
            E::Workflow,
            LIST_UUID_FILL,
            YesNo,
            &[Invalidation::Operation(Operation::ListWorkflows)],
            Option::None
        ),
        Operation::UpdateWorkflow => write!(
            E::Workflow,
            fills!(
                ("workflow", SelectionUuid),
                ("precondition-version", SelectionPrecondition)
            ),
            YesNo,
            SELECTION_AND_PARENT,
            Some(EditKind::JsonPatch)
        ),
        Operation::UpdateWorkflowRuleset => write!(
            E::Workflow,
            fills!(
                ("workflow", SelectionUuid),
                ("precondition-version", SelectionPrecondition)
            ),
            YesNo,
            SELECTION_ONLY,
            Some(EditKind::Ruleset)
        ),
        Operation::PublishWorkflow => write!(
            E::Workflow,
            fills!(
                ("workflow", SelectionUuid),
                ("precondition-version", SelectionPrecondition)
            ),
            YesNo,
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::RevertWorkflow => write!(
            E::Workflow,
            fills!(
                ("workflow", SelectionUuid),
                ("precondition-version", SelectionPrecondition)
            ),
            Phrase("revert workflow"),
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::SetWorkflowState => write!(
            E::Workflow,
            fills!(
                ("workflow", SelectionUuid),
                ("precondition-version", SelectionPrecondition)
            ),
            YesNo,
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::CopyWorkflow => write!(
            E::Workflow,
            fills!(
                ("workflow", SelectionUuid),
                ("precondition-version", SelectionPrecondition)
            ),
            YesNo,
            &[Invalidation::Operation(Operation::ListWorkflows)],
            Option::None
        ),
        Operation::DeleteWorkflow => write!(
            E::Workflow,
            fills!(
                ("workflow", SelectionUuid),
                ("precondition-version", SelectionPrecondition)
            ),
            Phrase("delete workflow"),
            SELECTION_AND_PARENT,
            Option::None
        ),
        Operation::GetWorkflowStats => read!(
            Collection,
            E::Stat,
            columns!(("Message", "/message_id"), ("Sent", "/sent")),
            fills!(("workflow", SelectionUuid))
        ),
        Operation::GetWorkflowMessageStats => read!(
            Collection,
            E::Stat,
            columns!(("Link", "/url"), ("Clicks", "/clicks")),
            fills!(("message-id", SelectionMessageId))
        ),
        Operation::GetWorkflowEventHistory => read!(
            Collection,
            E::Stat,
            columns!(("Event", "/event_id"), ("When", "/occurred_at")),
            fills!(("workflow", SelectionUuid), ("event-id", SelectionEventId))
        ),
        Operation::OauthGetAccessToken => {
            write!(E::Oauth, NO_FILLS, YesNo, NO_INVALIDATION, Option::None)
        }
        Operation::OauthGetRequestToken => {
            write!(E::Oauth, NO_FILLS, YesNo, NO_INVALIDATION, Option::None)
        }
        Operation::OauthRevoke => write!(
            E::Oauth,
            NO_FILLS,
            Phrase("revoke token"),
            NO_INVALIDATION,
            Option::None
        ),
        Operation::OauthToken => {
            write!(E::Oauth, NO_FILLS, YesNo, NO_INVALIDATION, Option::None)
        }
    }
}

const LIST_COLUMNS: &[Column] = columns!(
    ("List", "/name"),
    ("Id", "/id"),
    ("Subscribers", "/total_subscribed_subscribers")
);

const SUBSCRIBER_COLUMNS: &[Column] = columns!(
    ("Email", "/email"),
    ("Status", "/status"),
    ("Subscribed", "/subscribed_at")
);

const BROADCAST_COLUMNS: &[Column] = columns!(
    ("Subject", "/subject"),
    ("Status", "/status"),
    ("Scheduled", "/scheduled_at")
);

const CAMPAIGN_COLUMNS: &[Column] = columns!(
    ("Campaign", "/subject"),
    ("Type", "/campaign_type"),
    ("Id", "/id")
);

const WORKFLOW_COLUMNS: &[Column] =
    columns!(("Workflow", "/name"), ("State", "/state"), ("Id", "/id"));

const STAT_COLUMNS: &[Column] = columns!(("Stat", "/name"), ("Value", "/value"));

const WEBFORM_COLUMNS: &[Column] = columns!(("Web form", "/name"), ("Id", "/id"));

const SPLIT_TEST_COLUMNS: &[Column] = columns!(("Split test", "/name"), ("Status", "/status"));

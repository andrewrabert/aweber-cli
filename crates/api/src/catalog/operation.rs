use crate::catalog::args::{self, ArgSpec};
use crate::catalog::commands;
use crate::catalog::groups;

/// Every operation the API client can perform, whether or not the CLI routes it.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Operation {
    ListAccounts,
    GetAccount,
    FindAccountSubscribers,
    ListAccountWebformSplitTests,
    ListAccountWebforms,
    ListIntegrations,
    GetIntegration,
    ListLists,
    FindLists,
    GetList,
    ListBroadcasts,
    CreateBroadcast,
    GetBroadcastTotal,
    GetBroadcast,
    UpdateBroadcast,
    DeleteBroadcast,
    CancelBroadcast,
    GetBroadcastClicks,
    GetBroadcastOpens,
    ScheduleBroadcast,
    WaitBroadcast,
    ListCampaigns,
    ListCampaignStats,
    GetCampaignStat,
    FindCampaigns,
    GetCampaign,
    ListCustomFields,
    CreateCustomField,
    GetCustomField,
    DeleteCustomField,
    UpdateCustomField,
    ListLandingPages,
    GetLandingPage,
    CreatePurchase,
    ListSegments,
    GetSegment,
    ListSubscribers,
    CreateSubscriber,
    DeleteSubscriberByEmail,
    UpdateSubscriberByEmail,
    FindSubscribers,
    GetSubscriber,
    MoveSubscriber,
    DeleteSubscriber,
    UpdateSubscriber,
    GetSubscriberActivity,
    ListTags,
    ListWebFormSplitTests,
    GetWebFormSplitTest,
    ListWebFormSplitTestComponents,
    GetWebFormSplitTestComponent,
    ListWebForms,
    GetWebForm,
    GetBroadcastLinkAnalytics,
    UnsubscribeSubscriber,
    ListWorkflows,
    GetWorkflow,
    TreeWorkflow,
    CreateWorkflow,
    UpdateWorkflow,
    UpdateWorkflowRuleset,
    PublishWorkflow,
    RevertWorkflow,
    SetWorkflowState,
    CopyWorkflow,
    DeleteWorkflow,
    GetWorkflowStats,
    GetWorkflowMessageStats,
    GetWorkflowEventHistory,
    OauthGetAccessToken,
    OauthGetRequestToken,
    OauthRevoke,
    OauthToken,
}

impl Operation {
    pub const ALL: [Operation; 73] = [
        Operation::ListAccounts,
        Operation::GetAccount,
        Operation::FindAccountSubscribers,
        Operation::ListAccountWebformSplitTests,
        Operation::ListAccountWebforms,
        Operation::ListIntegrations,
        Operation::GetIntegration,
        Operation::ListLists,
        Operation::FindLists,
        Operation::GetList,
        Operation::ListBroadcasts,
        Operation::CreateBroadcast,
        Operation::GetBroadcastTotal,
        Operation::GetBroadcast,
        Operation::UpdateBroadcast,
        Operation::DeleteBroadcast,
        Operation::CancelBroadcast,
        Operation::GetBroadcastClicks,
        Operation::GetBroadcastOpens,
        Operation::ScheduleBroadcast,
        Operation::WaitBroadcast,
        Operation::ListCampaigns,
        Operation::ListCampaignStats,
        Operation::GetCampaignStat,
        Operation::FindCampaigns,
        Operation::GetCampaign,
        Operation::ListCustomFields,
        Operation::CreateCustomField,
        Operation::GetCustomField,
        Operation::DeleteCustomField,
        Operation::UpdateCustomField,
        Operation::ListLandingPages,
        Operation::GetLandingPage,
        Operation::CreatePurchase,
        Operation::ListSegments,
        Operation::GetSegment,
        Operation::ListSubscribers,
        Operation::CreateSubscriber,
        Operation::DeleteSubscriberByEmail,
        Operation::UpdateSubscriberByEmail,
        Operation::FindSubscribers,
        Operation::GetSubscriber,
        Operation::MoveSubscriber,
        Operation::DeleteSubscriber,
        Operation::UpdateSubscriber,
        Operation::GetSubscriberActivity,
        Operation::ListTags,
        Operation::ListWebFormSplitTests,
        Operation::GetWebFormSplitTest,
        Operation::ListWebFormSplitTestComponents,
        Operation::GetWebFormSplitTestComponent,
        Operation::ListWebForms,
        Operation::GetWebForm,
        Operation::GetBroadcastLinkAnalytics,
        Operation::UnsubscribeSubscriber,
        Operation::ListWorkflows,
        Operation::GetWorkflow,
        Operation::TreeWorkflow,
        Operation::CreateWorkflow,
        Operation::UpdateWorkflow,
        Operation::UpdateWorkflowRuleset,
        Operation::PublishWorkflow,
        Operation::RevertWorkflow,
        Operation::SetWorkflowState,
        Operation::CopyWorkflow,
        Operation::DeleteWorkflow,
        Operation::GetWorkflowStats,
        Operation::GetWorkflowMessageStats,
        Operation::GetWorkflowEventHistory,
        Operation::OauthGetAccessToken,
        Operation::OauthGetRequestToken,
        Operation::OauthRevoke,
        Operation::OauthToken,
    ];

    /// The route table's group for this operation.
    pub fn group(self) -> &'static str {
        groups::route(self).0
    }

    /// The route table's action name for this operation.
    pub fn action(self) -> &'static str {
        groups::route(self).1
    }

    /// The four `oauth` operations are absent from the clap tree.
    pub fn is_hidden(self) -> bool {
        groups::group_of(self).hidden
    }

    /// Whether the binary defines `aweber <group> <action>` as the catalog does.
    /// The `workflows` operations are the exception: the binary defines its own
    /// `workflows` commands over the same endpoints, and only the TUI runs these
    /// as they stand.
    pub fn is_cli(self) -> bool {
        groups::group_of(self).cli
    }

    /// The `about` line of the operation's clap command, looked up by the
    /// operation itself and never by its position in a table.
    pub fn about(self) -> &'static str {
        static ABOUT: std::sync::OnceLock<std::collections::BTreeMap<Operation, &'static str>> =
            std::sync::OnceLock::new();
        ABOUT.get_or_init(|| {
            Operation::ALL
                .iter()
                .map(|operation| {
                    let about = commands::command(*operation)
                        .get_about()
                        .map(ToString::to_string)
                        .unwrap_or_default();
                    (*operation, &*Box::leak(about.into_boxed_str()))
                })
                .collect()
        })[&self]
    }

    pub fn command(self) -> clap::Command {
        commands::command(self)
    }

    /// clap introspection plus the reserved arguments a plan needs.
    pub fn specs(self) -> Vec<ArgSpec> {
        args::specs(self)
    }
}

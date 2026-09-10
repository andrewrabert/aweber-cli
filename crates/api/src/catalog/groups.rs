use crate::catalog::Operation;

/// A resource group of the route table.
pub struct Group {
    pub name: &'static str,
    pub about: &'static str,
    pub long_about: Option<&'static str>,
    /// Absent from the clap tree and from the TUI palette; offered on the
    /// Session view alone.
    pub hidden: bool,
    /// The binary defines `aweber <group> <action>` as the catalog does. Where
    /// it does not, the group stays in the TUI palette and the binary defines
    /// its own, richer commands under the group's name.
    pub cli: bool,
    pub operations: &'static [Operation],
}

struct Table {
    name: &'static str,
    about: &'static str,
    long_about: Option<&'static str>,
    hidden: bool,
    cli: bool,
    routes: &'static [(&'static str, Operation)],
}

static TABLE: &[Table] = &[
    Table {
        name: "lists",
        about: "Manage subscriber lists",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListLists),
            ("get", Operation::GetList),
            ("find", Operation::FindLists),
        ],
    },
    Table {
        name: "subscribers",
        about: "Manage subscribers",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListSubscribers),
            ("get", Operation::GetSubscriber),
            ("create", Operation::CreateSubscriber),
            ("update", Operation::UpdateSubscriber),
            ("update-by-email", Operation::UpdateSubscriberByEmail),
            ("delete", Operation::DeleteSubscriber),
            ("delete-by-email", Operation::DeleteSubscriberByEmail),
            ("unsubscribe", Operation::UnsubscribeSubscriber),
            ("find", Operation::FindSubscribers),
            ("move", Operation::MoveSubscriber),
            ("activity", Operation::GetSubscriberActivity),
        ],
    },
    Table {
        name: "broadcasts",
        about: "Manage broadcasts (email campaigns)",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListBroadcasts),
            ("get", Operation::GetBroadcast),
            ("create", Operation::CreateBroadcast),
            ("update", Operation::UpdateBroadcast),
            ("delete", Operation::DeleteBroadcast),
            ("schedule", Operation::ScheduleBroadcast),
            ("cancel", Operation::CancelBroadcast),
            ("wait", Operation::WaitBroadcast),
            ("total", Operation::GetBroadcastTotal),
            ("clicks", Operation::GetBroadcastClicks),
            ("opens", Operation::GetBroadcastOpens),
            ("link-analytics", Operation::GetBroadcastLinkAnalytics),
        ],
    },
    Table {
        name: "campaigns",
        about: "Manage a single follow-up or broadcast message on a list",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListCampaigns),
            ("get", Operation::GetCampaign),
            ("find", Operation::FindCampaigns),
            ("stats", Operation::ListCampaignStats),
            ("stat", Operation::GetCampaignStat),
        ],
    },
    // The campaign service's endpoints one request at a time, for the TUI. The
    // binary's `aweber workflows` commands are richer than one request each and
    // are defined in the `aweber-cli` crate instead.
    Table {
        name: "workflows",
        about: "Manage automation workflows (unrelated to `aweber campaigns`)",
        long_about: Some(
            "Manage automation workflows (unrelated to `aweber campaigns`).\n\n\
             These commands call undocumented, unversioned and unsupported \
             endpoints that can change or disappear without notice.",
        ),
        hidden: false,
        cli: false,
        routes: &[
            ("list", Operation::ListWorkflows),
            ("get", Operation::GetWorkflow),
            ("tree", Operation::TreeWorkflow),
            ("create", Operation::CreateWorkflow),
            ("update", Operation::UpdateWorkflow),
            ("update-ruleset", Operation::UpdateWorkflowRuleset),
            ("publish", Operation::PublishWorkflow),
            ("revert", Operation::RevertWorkflow),
            ("set-state", Operation::SetWorkflowState),
            ("copy", Operation::CopyWorkflow),
            ("delete", Operation::DeleteWorkflow),
            ("stats", Operation::GetWorkflowStats),
            ("message-stats", Operation::GetWorkflowMessageStats),
            ("event-history", Operation::GetWorkflowEventHistory),
        ],
    },
    Table {
        name: "account",
        about: "Manage your AWeber account",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListAccounts),
            ("get", Operation::GetAccount),
            ("find-subscribers", Operation::FindAccountSubscribers),
            ("webforms", Operation::ListAccountWebforms),
            (
                "webform-split-tests",
                Operation::ListAccountWebformSplitTests,
            ),
        ],
    },
    Table {
        name: "custom-fields",
        about: "Manage custom fields",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListCustomFields),
            ("get", Operation::GetCustomField),
            ("create", Operation::CreateCustomField),
            ("update", Operation::UpdateCustomField),
            ("delete", Operation::DeleteCustomField),
        ],
    },
    Table {
        name: "tags",
        about: "Manage tags",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[("list", Operation::ListTags)],
    },
    Table {
        name: "segments",
        about: "Manage segments",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListSegments),
            ("get", Operation::GetSegment),
        ],
    },
    Table {
        name: "integrations",
        about: "Manage integrations",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListIntegrations),
            ("get", Operation::GetIntegration),
        ],
    },
    Table {
        name: "landing-pages",
        about: "Manage landing pages",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListLandingPages),
            ("get", Operation::GetLandingPage),
        ],
    },
    Table {
        name: "purchases",
        about: "Record purchases",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[("create", Operation::CreatePurchase)],
    },
    Table {
        name: "webforms",
        about: "Manage webforms",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListWebForms),
            ("get", Operation::GetWebForm),
        ],
    },
    Table {
        name: "webform-split-tests",
        about: "Manage webform split tests",
        long_about: None,
        hidden: false,
        cli: true,
        routes: &[
            ("list", Operation::ListWebFormSplitTests),
            ("get", Operation::GetWebFormSplitTest),
            ("components", Operation::ListWebFormSplitTestComponents),
            ("component", Operation::GetWebFormSplitTestComponent),
        ],
    },
    Table {
        name: "oauth",
        about: "Exchange OAuth credentials directly",
        long_about: None,
        hidden: true,
        cli: true,
        routes: &[
            ("access-token", Operation::OauthGetAccessToken),
            ("request-token", Operation::OauthGetRequestToken),
            ("revoke", Operation::OauthRevoke),
            ("token", Operation::OauthToken),
        ],
    },
];

/// Every group of the route table, the hidden `oauth` group last.
pub fn groups() -> &'static [Group] {
    static GROUPS: std::sync::OnceLock<Vec<Group>> = std::sync::OnceLock::new();
    GROUPS.get_or_init(|| {
        TABLE
            .iter()
            .map(|table| Group {
                name: table.name,
                about: table.about,
                long_about: table.long_about,
                hidden: table.hidden,
                cli: table.cli,
                operations: Box::leak(
                    table
                        .routes
                        .iter()
                        .map(|(_, operation)| *operation)
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                ),
            })
            .collect()
    })
}

pub fn resolve(group: &str, action: &str) -> Option<Operation> {
    TABLE
        .iter()
        .find(|table| table.name == group)
        .and_then(|table| table.routes.iter().find(|(name, _)| *name == action))
        .map(|(_, operation)| *operation)
}

/// The group name and action name that route to this operation.
pub(super) fn route(operation: Operation) -> (&'static str, &'static str) {
    TABLE
        .iter()
        .find_map(|table| {
            table
                .routes
                .iter()
                .find(|(_, routed)| *routed == operation)
                .map(|(action, _)| (table.name, *action))
        })
        .expect("every operation is routed exactly once")
}

pub(super) fn group_of(operation: Operation) -> &'static Group {
    groups()
        .iter()
        .find(|group| group.operations.contains(&operation))
        .expect("every operation belongs to a group")
}

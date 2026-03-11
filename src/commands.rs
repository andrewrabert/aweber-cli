use aweber::cli::{Cli, CliCommand};

/// A single route: an action name within a group, mapped to a CliCommand variant.
struct Route {
    action: &'static str,
    command: CliCommand,
}

/// A resource group containing one or more routes.
struct Group {
    name: &'static str,
    about: &'static str,
    routes: &'static [Route],
}

/// The complete route table. Groups are ordered by usage frequency (most-used first).
/// Every non-OAuth CliCommand variant must appear exactly once.
static GROUPS: &[Group] = &[
    Group {
        name: "lists",
        about: "Manage subscriber lists",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListLists,
            },
            Route {
                action: "get",
                command: CliCommand::GetList,
            },
            Route {
                action: "find",
                command: CliCommand::FindLists,
            },
        ],
    },
    Group {
        name: "subscribers",
        about: "Manage subscribers",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListSubscribers,
            },
            Route {
                action: "get",
                command: CliCommand::GetSubscriber,
            },
            Route {
                action: "create",
                command: CliCommand::CreateSubscriber,
            },
            Route {
                action: "update",
                command: CliCommand::UpdateSubscriberByEmail,
            },
            Route {
                action: "delete",
                command: CliCommand::DeleteSubscriberByEmail,
            },
            Route {
                action: "find",
                command: CliCommand::FindSubscribers,
            },
            Route {
                action: "move",
                command: CliCommand::MoveSubscriber,
            },
            Route {
                action: "unsubscribe",
                command: CliCommand::DeleteSubscriber,
            },
            Route {
                action: "update-by-email",
                command: CliCommand::UpdateSubscriber,
            },
            Route {
                action: "activity",
                command: CliCommand::GetSubscriberActivity,
            },
        ],
    },
    Group {
        name: "broadcasts",
        about: "Manage broadcasts (email campaigns)",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListBroadcasts,
            },
            Route {
                action: "get",
                command: CliCommand::GetBroadcast,
            },
            Route {
                action: "create",
                command: CliCommand::CreateBroadcast,
            },
            Route {
                action: "update",
                command: CliCommand::UpdateBroadcast,
            },
            Route {
                action: "delete",
                command: CliCommand::DeleteBroadcast,
            },
            Route {
                action: "schedule",
                command: CliCommand::ScheduleBroadcast,
            },
            Route {
                action: "cancel",
                command: CliCommand::CancelBroadcast,
            },
            Route {
                action: "total",
                command: CliCommand::GetBroadcastTotal,
            },
            Route {
                action: "clicks",
                command: CliCommand::GetBroadcastClicks,
            },
            Route {
                action: "opens",
                command: CliCommand::GetBroadcastOpens,
            },
            Route {
                action: "link-analytics",
                command: CliCommand::GetBroadcastLinkAnalytics,
            },
        ],
    },
    Group {
        name: "campaigns",
        about: "Manage campaigns",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListCampaigns,
            },
            Route {
                action: "get",
                command: CliCommand::GetCampaign,
            },
            Route {
                action: "find",
                command: CliCommand::FindCampaigns,
            },
            // TODO: verify stats/stat naming against Swagger API semantics
            Route {
                action: "stats",
                command: CliCommand::ListCampaignStats,
            },
            Route {
                action: "stat",
                command: CliCommand::GetCampaignStat,
            },
        ],
    },
    Group {
        name: "account",
        about: "Manage your AWeber account",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListAccounts,
            },
            Route {
                action: "get",
                command: CliCommand::GetAccount,
            },
            Route {
                action: "find-subscribers",
                command: CliCommand::FindAccountSubscribers,
            },
            Route {
                action: "webforms",
                command: CliCommand::ListAccountWebforms,
            },
            Route {
                action: "webform-split-tests",
                command: CliCommand::ListAccountWebformSplitTests,
            },
        ],
    },
    Group {
        name: "custom-fields",
        about: "Manage custom fields",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListCustomFields,
            },
            Route {
                action: "get",
                command: CliCommand::GetCustomField,
            },
            Route {
                action: "create",
                command: CliCommand::CreateCustomField,
            },
            Route {
                action: "update",
                command: CliCommand::UpdateCustomField,
            },
            Route {
                action: "delete",
                command: CliCommand::DeleteCustomField,
            },
        ],
    },
    Group {
        name: "tags",
        about: "Manage tags",
        routes: &[Route {
            action: "list",
            command: CliCommand::ListTags,
        }],
    },
    Group {
        name: "segments",
        about: "Manage segments",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListSegments,
            },
            Route {
                action: "get",
                command: CliCommand::GetSegment,
            },
        ],
    },
    Group {
        name: "integrations",
        about: "Manage integrations",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListIntegrations,
            },
            Route {
                action: "get",
                command: CliCommand::GetIntegration,
            },
        ],
    },
    Group {
        name: "landing-pages",
        about: "Manage landing pages",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListLandingPages,
            },
            Route {
                action: "get",
                command: CliCommand::GetLandingPage,
            },
        ],
    },
    Group {
        name: "purchases",
        about: "Record purchases",
        routes: &[Route {
            action: "create",
            command: CliCommand::CreatePurchase,
        }],
    },
    Group {
        name: "webforms",
        about: "Manage webforms",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListWebForms,
            },
            Route {
                action: "get",
                command: CliCommand::GetWebForm,
            },
        ],
    },
    Group {
        name: "webform-split-tests",
        about: "Manage webform split tests",
        routes: &[
            Route {
                action: "list",
                command: CliCommand::ListWebFormSplitTests,
            },
            Route {
                action: "get",
                command: CliCommand::GetWebFormSplitTest,
            },
            Route {
                action: "components",
                command: CliCommand::ListWebFormSplitTestComponents,
            },
            Route {
                action: "component",
                command: CliCommand::GetWebFormSplitTestComponent,
            },
        ],
    },
];

/// Build the clap command tree with nested resource-group subcommands.
///
/// Returns a `clap::Command` where each resource group is a top-level subcommand
/// containing action subcommands. The auth subcommand is also included.
pub fn build_command_tree() -> clap::Command {
    let auth_cmd = clap::Command::new("auth")
        .about("Manage authentication")
        .subcommand_required(true)
        .subcommand(clap::Command::new("login").about("Log in to AWeber"))
        .subcommand(clap::Command::new("logout").about("Log out and remove stored credentials"))
        .subcommand(clap::Command::new("status").about("Show authentication status"));

    let mut app = clap::Command::new("aweber-cli")
        .bin_name("aweber")
        .about("AWeber API CLI")
        .version(env!("AWEBER_VERSION"))
        .arg(
            clap::Arg::new("base-url")
                .long("base-url")
                .env("AWEBER_API_URL")
                .default_value("https://api.aweber.com/1.0")
                .help("AWeber API base URL"),
        )
        .arg(
            clap::Arg::new("token")
                .long("token")
                .env("AWEBER_TOKEN")
                .help("OAuth2 access token (overrides stored credentials)"),
        )
        .arg(
            clap::Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .global(true)
                .help("Print request URLs to stderr"),
        )
        .subcommand_required(true)
        .subcommand(auth_cmd);

    for group in GROUPS {
        let mut group_cmd = clap::Command::new(group.name)
            .about(group.about)
            .subcommand_required(true);

        for route in group.routes {
            let subcmd = Cli::get_command(route.command).name(route.action.to_string());
            group_cmd = group_cmd.subcommand(subcmd);
        }

        app = app.subcommand(group_cmd);
    }

    app
}

/// Resolve a (group_name, action_name) pair to the corresponding CliCommand.
///
/// Returns `None` if the group or action is not found.
pub fn resolve_command(group: &str, action: &str) -> Option<CliCommand> {
    GROUPS
        .iter()
        .find(|g| g.name == group)
        .and_then(|g| g.routes.iter().find(|r| r.action == action))
        .map(|r| r.command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// The 4 OAuth commands that are intentionally hidden from the CLI.
    const HIDDEN_COMMANDS: &[CliCommand] = &[
        CliCommand::OauthGetAccessToken,
        CliCommand::OauthGetRequestToken,
        CliCommand::OauthRevoke,
        CliCommand::OauthToken,
    ];

    #[test]
    fn all_non_oauth_commands_are_routed_exactly_once() {
        // Collect all commands from the route table.
        let mut routed: Vec<CliCommand> = Vec::new();
        for group in GROUPS {
            for route in group.routes {
                routed.push(route.command);
            }
        }

        // Verify no duplicates.
        // CliCommand does not derive Hash/Eq (generated code, cannot modify),
        // so use the Debug string as a stable unique identifier.
        let unique: HashSet<_> = routed.iter().copied().map(|c| format!("{c:?}")).collect();
        assert_eq!(
            routed.len(),
            unique.len(),
            "route table contains duplicate commands"
        );

        // Verify count is exactly 53.
        assert_eq!(routed.len(), 53, "expected 53 routed commands");

        // Verify every non-OAuth CliCommand variant is present.
        let hidden_set: HashSet<String> =
            HIDDEN_COMMANDS.iter().map(|c| format!("{c:?}")).collect();
        let routed_set: HashSet<String> = routed.iter().map(|c| format!("{c:?}")).collect();

        for cmd in CliCommand::iter() {
            let name = format!("{cmd:?}");
            if hidden_set.contains(&name) {
                assert!(
                    !routed_set.contains(&name),
                    "OAuth command {name} should not be in route table"
                );
            } else {
                assert!(
                    routed_set.contains(&name),
                    "non-OAuth command {name} is missing from route table"
                );
            }
        }
    }

    #[test]
    fn resolve_command_finds_known_routes() {
        assert!(matches!(
            resolve_command("lists", "list"),
            Some(CliCommand::ListLists)
        ));
        assert!(matches!(
            resolve_command("broadcasts", "create"),
            Some(CliCommand::CreateBroadcast)
        ));
        assert!(matches!(
            resolve_command("subscribers", "activity"),
            Some(CliCommand::GetSubscriberActivity)
        ));
    }

    #[test]
    fn resolve_command_returns_none_for_unknown() {
        assert!(resolve_command("nonexistent", "list").is_none());
        assert!(resolve_command("lists", "nonexistent").is_none());
    }
}

use crate::auth;
use crate::cli::{CliCommand, WorkflowCommand};

const WORKFLOWS: &str = "workflows";

/// One `aweber workflows` action.
struct Route {
    action: &'static str,
    command: WorkflowCommand,
}

/// The `workflows` group is the binary's own: each action reads and edits a
/// workflow through `aweber::workflows` over several requests, where the
/// catalog offers the TUI one request per operation under the same group name.
/// Every `WorkflowCommand` appears exactly once.
static WORKFLOW_ROUTES: &[Route] = &[
    Route {
        action: "list",
        command: WorkflowCommand::List,
    },
    Route {
        action: "show",
        command: WorkflowCommand::Show,
    },
    Route {
        action: "create",
        command: WorkflowCommand::Create,
    },
    Route {
        action: "update",
        command: WorkflowCommand::Update,
    },
    Route {
        action: "add-step",
        command: WorkflowCommand::AddStep,
    },
    Route {
        action: "update-step",
        command: WorkflowCommand::UpdateStep,
    },
    Route {
        action: "publish",
        command: WorkflowCommand::Publish,
    },
    Route {
        action: "delete",
        command: WorkflowCommand::Delete,
    },
];

/// Build the clap command tree with nested resource-group subcommands.
///
/// Every group but `workflows` is the catalog's, in route-table order;
/// `workflows` takes the catalog group's place with the binary's own actions.
pub fn build_command_tree() -> clap::Command {
    let login_cmd = clap::Command::new("login").about("Log in to AWeber").arg(
        clap::Arg::new("client-id")
            .long("client-id")
            .env("AWEBER_CLIENT_ID")
            .default_value(auth::DEFAULT_CLIENT_ID)
            .help("OAuth2 client ID"),
    );

    let auth_cmd = clap::Command::new("auth")
        .about("Manage authentication")
        .subcommand_required(true)
        .subcommand(login_cmd)
        .subcommand(clap::Command::new("logout").about("Log out and remove stored credentials"))
        .subcommand(clap::Command::new("status").about("Show authentication status"));

    let mut app = clap::Command::new("aweber-cli")
        .bin_name("aweber")
        .about("AWeber API CLI")
        .version(env!("AWEBER_VERSION"))
        .arg(
            clap::Arg::new("credentials-file")
                .short('c')
                .long("credentials-file")
                .env("AWEBER_CREDENTIALS_FILE")
                .help("Path to the credentials JSON file"),
        )
        .arg(
            clap::Arg::new("token")
                .long("token")
                .env("AWEBER_TOKEN")
                .help("OAuth2 access token (overrides stored credentials)"),
        )
        .arg(
            clap::Arg::new("api-url")
                .long("api-url")
                .env("AWEBER_API_URL")
                .default_value(aweber::oauth::DEFAULT_API_URL)
                .help("AWeber API base URL"),
        )
        .arg(
            clap::Arg::new("auth-url")
                .long("auth-url")
                .env("AWEBER_AUTH_URL")
                .default_value(aweber::oauth::DEFAULT_AUTH_URL)
                .help("AWeber auth base URL"),
        )
        .arg(
            clap::Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .global(true)
                .help("Print request and response details to stderr"),
        )
        .subcommand_required(true)
        .subcommand(auth_cmd)
        .subcommand(aweber::catalog::raw_command())
        .subcommand(clap::Command::new("tui").about("Browse the API in a full-screen terminal UI"));

    for group in aweber::catalog::groups() {
        if group.hidden {
            continue;
        }
        let group_cmd = match (group.cli, group.name) {
            (true, _) => aweber::catalog::group_command(group),
            (false, WORKFLOWS) => workflows_command(group),
            (false, other) => panic!("neither the catalog nor aweber routes the `{other}` group"),
        };
        app = app.subcommand(group_cmd);
    }

    app
}

/// `aweber workflows`, in the catalog group's place and with the binary's own
/// actions.
fn workflows_command(group: &aweber::catalog::Group) -> clap::Command {
    let mut group_cmd = clap::Command::new(group.name)
        .about("Manage automation workflows")
        .long_about(
            "Manage automation workflows.\n\n\
             These commands use an undocumented, unversioned and unsupported API \
             surface that can change or disappear without notice.",
        )
        .subcommand_required(true);
    for route in WORKFLOW_ROUTES {
        group_cmd = group_cmd.subcommand(route.command.command().name(route.action));
    }
    group_cmd
}

/// Resolve a (group_name, action_name) pair to the command the binary runs.
///
/// Returns `None` if the group or action is not routed, including the hidden
/// `oauth` operations and the catalog's TUI-only `workflows` operations.
pub fn resolve_command(group: &str, action: &str) -> Option<CliCommand> {
    if group == WORKFLOWS {
        return WORKFLOW_ROUTES
            .iter()
            .find(|route| route.action == action)
            .map(|route| CliCommand::Workflow(route.command));
    }
    aweber::catalog::resolve(group, action)
        .filter(|operation| !operation.is_hidden() && operation.is_cli())
        .map(CliCommand::Operation)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn every_workflow_command_is_routed_exactly_once() {
        let routed: Vec<String> = WORKFLOW_ROUTES
            .iter()
            .map(|route| format!("{:?}", route.command))
            .collect();
        let unique: BTreeSet<&String> = routed.iter().collect();
        assert_eq!(
            routed.len(),
            unique.len(),
            "a workflow command is routed twice"
        );
        for command in WorkflowCommand::ALL {
            assert!(
                routed.contains(&format!("{command:?}")),
                "{command:?} is not routed"
            );
        }
    }

    #[test]
    fn the_workflows_group_is_the_one_group_the_catalog_leaves_to_the_binary() {
        let left: Vec<&str> = aweber::catalog::groups()
            .iter()
            .filter(|group| !group.hidden && !group.cli)
            .map(|group| group.name)
            .collect();
        assert_eq!(left, vec![WORKFLOWS]);
    }

    #[test]
    fn the_tree_parses_every_routed_command() {
        let tree = build_command_tree();
        for group in aweber::catalog::groups()
            .iter()
            .filter(|group| !group.hidden)
        {
            let group_cmd = tree
                .get_subcommands()
                .find(|candidate| candidate.get_name() == group.name)
                .unwrap_or_else(|| panic!("`aweber {}` is in the tree", group.name));
            for action in group_cmd.get_subcommands() {
                assert!(
                    resolve_command(group.name, action.get_name()).is_some(),
                    "`aweber {} {}` is in the tree but does not resolve",
                    group.name,
                    action.get_name()
                );
            }
        }
    }

    #[test]
    fn resolve_command_finds_known_routes() {
        assert_eq!(
            resolve_command("lists", "list"),
            Some(CliCommand::Operation(aweber::catalog::Operation::ListLists))
        );
        assert_eq!(
            resolve_command("broadcasts", "create"),
            Some(CliCommand::Operation(
                aweber::catalog::Operation::CreateBroadcast
            ))
        );
        assert_eq!(
            resolve_command("workflows", "show"),
            Some(CliCommand::Workflow(WorkflowCommand::Show))
        );
    }

    #[test]
    fn resolve_command_returns_none_for_unknown() {
        assert!(resolve_command("nonexistent", "list").is_none());
        assert!(resolve_command("lists", "nonexistent").is_none());
        // The catalog's `workflows get` is the TUI's, not a CLI command.
        assert!(resolve_command("workflows", "get").is_none());
        assert!(resolve_command("oauth", "token").is_none());
    }
}

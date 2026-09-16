use anyhow::Context as _;
use aweber::types;

pub(crate) struct Cli {
    pub(crate) client: aweber::client::Client,
    pub(crate) account_id: i32,
    account: tokio::sync::OnceCell<aweber::ids::AccountUid>,
}

impl Cli {
    pub(crate) fn new(
        client: aweber::client::Client,
        account_id: i32,
        account: Option<aweber::ids::AccountUid>,
    ) -> Self {
        let cell = tokio::sync::OnceCell::new();
        if let Some(account) = account {
            cell.set(account).expect("a fresh cell is empty");
        }
        Self {
            client,
            account_id,
            account: cell,
        }
    }

    pub(crate) async fn account_uid(&self) -> anyhow::Result<aweber::ids::AccountUid> {
        self.account
            .get_or_try_init(|| async {
                let document = aweber::endpoints::get_account(&self.client, self.account_id)
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"))
                    .context("failed to look up the account uid")?;
                aweber::endpoints::account_uid(&document).context("account has no uid")
            })
            .await
            .copied()
    }

    // -----------------------------------------------------------------------
    // `aweber workflows` clap definitions; every other command's definition is
    // `aweber::catalog`'s
    // -----------------------------------------------------------------------
    fn workflow_positional() -> clap::Arg {
        clap::Arg::new("workflow")
            .required(true)
            .value_parser(clap::value_parser!(
                crate::workflows::request::WorkflowSource
            ))
            .help("The workflow id, or its name with --list")
    }

    fn workflow_list_arg(required: bool) -> clap::Arg {
        clap::Arg::new("list")
            .long("list")
            .required(required)
            .value_parser(clap::value_parser!(String))
            .help("The list name or unique list ID")
    }

    fn workflow_property_args() -> [clap::Arg; 6] {
        [
            clap::Arg::new("timezone")
                .long("timezone")
                .value_parser(clap::value_parser!(aweber::workflows::Timezone))
                .help("The workflow timezone, e.g. America/New_York"),
            clap::Arg::new("sharing")
                .long("sharing")
                .value_parser(clap::value_parser!(aweber::workflows::Sharing))
                .help("Let anyone holding the sharing code copy this workflow (yes|no)"),
            clap::Arg::new("starter")
                .long("starter")
                .value_parser(clap::value_parser!(crate::workflows::request::StarterKind))
                .help("What starts a subscriber (new-subscriber|tag)"),
            clap::Arg::new("starter-tag")
                .long("starter-tag")
                .value_parser(clap::value_parser!(aweber::workflows::Tag))
                .help("The tag that starts a subscriber"),
            clap::Arg::new("add-exit-tag")
                .long("add-exit-tag")
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(aweber::workflows::Tag))
                .help("Remove the subscriber if they get this tag"),
            clap::Arg::new("remove-exit-tag")
                .long("remove-exit-tag")
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(aweber::workflows::Tag))
                .help("Stop this tag from removing a subscriber"),
        ]
    }

    fn workflow_placement_args() -> [clap::Arg; 4] {
        [
            clap::Arg::new("before")
                .long("before")
                .value_parser(clap::value_parser!(aweber::ids::RuleId))
                .help("Place the step just before that step"),
            clap::Arg::new("after")
                .long("after")
                .value_parser(clap::value_parser!(aweber::ids::RuleId))
                .help("Place the step just after that step"),
            clap::Arg::new("branch")
                .long("branch")
                .num_args(2)
                .value_names(["SPLIT", "BRANCH"])
                .value_parser(clap::value_parser!(crate::workflows::request::BranchArg))
                .help("Place the step in that branch of that split"),
            clap::Arg::new("inside")
                .long("inside")
                .value_name("STEP-ID")
                .value_parser(clap::value_parser!(aweber::ids::RuleId))
                .help("Place the step inside that feed step's loop"),
        ]
    }

    fn workflow_automation_args() -> [clap::Arg; 9] {
        [
            clap::Arg::new("when-opened-apply-tag")
                .long("when-opened-apply-tag")
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(aweber::workflows::Tag))
                .help("On open, apply this tag"),
            clap::Arg::new("when-opened-remove-tag")
                .long("when-opened-remove-tag")
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(aweber::workflows::Tag))
                .help("On open, remove this tag"),
            clap::Arg::new("when-opened-exit")
                .long("when-opened-exit")
                .action(clap::ArgAction::SetTrue)
                .help("On open, exit the workflow"),
            clap::Arg::new("no-open-automation")
                .long("no-open-automation")
                .action(clap::ArgAction::SetTrue)
                .help("Remove the open automation"),
            clap::Arg::new("when-clicked-apply-tag")
                .long("when-clicked-apply-tag")
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(aweber::workflows::Tag))
                .help("On click, apply this tag"),
            clap::Arg::new("when-clicked-remove-tag")
                .long("when-clicked-remove-tag")
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(aweber::workflows::Tag))
                .help("On click, remove this tag"),
            clap::Arg::new("when-clicked-exit")
                .long("when-clicked-exit")
                .action(clap::ArgAction::SetTrue)
                .help("On click, exit the workflow"),
            clap::Arg::new("link")
                .long("link")
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(aweber::workflows::LinkUrl))
                .help("Limit the click rule to these links"),
            clap::Arg::new("remove-click-rule")
                .long("remove-click-rule")
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(u64).range(1..))
                .help("Delete click rule n"),
        ]
    }

    fn workflow_wait_args() -> [clap::Arg; 4] {
        [
            clap::Arg::new("for")
                .long("for")
                .value_parser(clap::value_parser!(aweber::workflows::Delay))
                .help("Wait this long, e.g. 45m, 4h, 2d, 1w, 6mo"),
            clap::Arg::new("send-on")
                .long("send-on")
                .value_parser(clap::value_parser!(aweber::workflows::SendDays))
                .help("The days a subscriber may move on"),
            clap::Arg::new("send-at")
                .long("send-at")
                .value_parser(clap::value_parser!(aweber::workflows::SendTime))
                .help("The time of day on those days"),
            clap::Arg::new("subscriber-timezone")
                .long("subscriber-timezone")
                .value_parser(clap::value_parser!(aweber::workflows::TimezoneSource))
                .help("Resolve this step's time in the subscriber's own timezone (yes|no)"),
        ]
    }

    fn workflow_feed_args() -> [clap::Arg; 3] {
        [
            clap::Arg::new("url")
                .long("url")
                .value_parser(clap::value_parser!(aweber::workflows::FeedUrl))
                .help("The RSS feed url"),
            clap::Arg::new("check-every")
                .long("check-every")
                .value_parser(clap::value_parser!(aweber::workflows::Recurrence))
                .help("How often the feed is checked: 1h, 1d, 1w, 1mo"),
            clap::Arg::new("check-times")
                .long("check-times")
                .value_name("N")
                .value_parser(clap::value_parser!(u32).range(1..))
                .help("Stop checking after this many checks; steps may then follow the feed"),
        ]
    }

    fn workflow_split_args() -> [clap::Arg; 4] {
        [
            clap::Arg::new("link-contains")
                .long("link-contains")
                .value_name("TEXT")
                .value_parser(clap::value_parser!(aweber::workflows::LinkFragment))
                .conflicts_with("link")
                .help("Limit the click test to links containing this text"),
            clap::Arg::new("when-tagged")
                .long("when-tagged")
                .value_name("TAG")
                .value_parser(clap::value_parser!(aweber::workflows::Tag))
                .conflicts_with_all(["when-opened", "when-clicked"])
                .help("The tag the split tests the subscriber for"),
            clap::Arg::new("when-opened")
                .long("when-opened")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("when-clicked")
                .help("Test whether any message the workflow sent was opened"),
            clap::Arg::new("when-clicked")
                .long("when-clicked")
                .action(clap::ArgAction::SetTrue)
                .help("Test whether any message the workflow sent was clicked"),
        ]
    }

    pub(crate) fn cli_list_workflows() -> clap::Command {
        clap::Command::new("")
            .arg(Self::workflow_list_arg(true))
            .arg(
                clap::Arg::new("status")
                    .long("status")
                    .action(clap::ArgAction::Append)
                    .value_parser(clap::value_parser!(aweber::workflows::WorkflowStatus))
                    .help("Only workflows with this status"),
            )
            .arg(
                clap::Arg::new("starter-tag")
                    .long("starter-tag")
                    .action(clap::ArgAction::Append)
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .help("Only workflows started by this tag"),
            )
            .about("List the workflows on a list")
    }

    pub(crate) fn cli_show_workflow() -> clap::Command {
        clap::Command::new("")
            .arg(Self::workflow_positional())
            .arg(Self::workflow_list_arg(false))
            .arg(
                clap::Arg::new("published")
                    .long("published")
                    .action(clap::ArgAction::SetTrue)
                    .help("Show the published version instead of the working copy"),
            )
            .arg(
                clap::Arg::new("draft")
                    .long("draft")
                    .action(clap::ArgAction::SetTrue)
                    .help("Show only the unpublished changes"),
            )
            .arg(
                clap::Arg::new("no-stats")
                    .long("no-stats")
                    .action(clap::ArgAction::SetTrue)
                    .help("Omit the stats fields"),
            )
            .about("Show a workflow, its steps and its exit tags")
    }

    pub(crate) fn cli_create_workflow() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("name")
                    .required(true)
                    .value_parser(clap::value_parser!(aweber::workflows::WorkflowName))
                    .help("The workflow name"),
            )
            .arg(Self::workflow_list_arg(true))
            .arg(
                clap::Arg::new("from")
                    .long("from")
                    .value_parser(clap::value_parser!(
                        crate::workflows::request::WorkflowSource
                    ))
                    .help(
                        "Duplicate this workflow, sharing code or name instead of starting empty",
                    ),
            )
            .args(Self::workflow_property_args())
            .about("Create a workflow")
    }

    pub(crate) fn cli_update_workflow() -> clap::Command {
        clap::Command::new("")
            .arg(Self::workflow_positional())
            .arg(Self::workflow_list_arg(false))
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .value_parser(clap::value_parser!(aweber::workflows::WorkflowName))
                    .help("Rename the workflow"),
            )
            .arg(
                clap::Arg::new("status")
                    .long("status")
                    .value_parser(clap::value_parser!(aweber::workflows::StatusChange))
                    .help("Set the running status (active|paused|draining|stopped)"),
            )
            .args(Self::workflow_property_args())
            .about("Update a workflow's name, status and properties")
    }

    fn add_step_workflow_positional() -> clap::Arg {
        clap::Arg::new("workflow")
            .required(true)
            .value_name("WORKFLOW")
            .value_parser(clap::value_parser!(
                crate::workflows::request::WorkflowSource
            ))
            .help("Workflow name or id")
    }

    fn add_step_shared_args() -> [clap::Arg; 5] {
        [
            clap::Arg::new("list")
                .long("list")
                .global(true)
                .display_order(100)
                .value_name("LIST")
                .value_parser(clap::value_parser!(String))
                .help("The workflow's list (required when WORKFLOW is a name)"),
            clap::Arg::new("before")
                .long("before")
                .global(true)
                .display_order(101)
                .value_name("STEP-ID")
                .value_parser(clap::value_parser!(aweber::ids::RuleId))
                .help("Place the new step just before that step"),
            clap::Arg::new("after")
                .long("after")
                .global(true)
                .display_order(102)
                .value_name("STEP-ID")
                .value_parser(clap::value_parser!(aweber::ids::RuleId))
                .help("Place the new step just after that step"),
            clap::Arg::new("branch")
                .long("branch")
                .global(true)
                .display_order(103)
                .num_args(2)
                .value_names(["SPLIT-STEP-ID", "yes|no"])
                .value_parser(clap::value_parser!(crate::workflows::request::BranchArg))
                .help("Append to that branch of that split"),
            clap::Arg::new("inside")
                .long("inside")
                .global(true)
                .display_order(103)
                .value_name("STEP-ID")
                .value_parser(clap::value_parser!(aweber::ids::RuleId))
                .help("Place the step inside that feed step's loop"),
        ]
    }

    fn add_step_message_command() -> clap::Command {
        clap::Command::new("message")
            .arg(
                clap::Arg::new("message-id")
                    .required(true)
                    .value_name("MESSAGE-ID")
                    .value_parser(clap::value_parser!(aweber::ids::MessageId))
                    .help("The message this step sends"),
            )
            .arg(
                clap::Arg::new("when-opened-apply-tag")
                    .long("when-opened-apply-tag")
                    .action(clap::ArgAction::Append)
                    .value_name("TAG")
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .help("On open, apply this tag"),
            )
            .arg(
                clap::Arg::new("when-opened-remove-tag")
                    .long("when-opened-remove-tag")
                    .action(clap::ArgAction::Append)
                    .value_name("TAG")
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .help("On open, remove this tag"),
            )
            .arg(
                clap::Arg::new("when-opened-exit")
                    .long("when-opened-exit")
                    .action(clap::ArgAction::SetTrue)
                    .help("On open, exit the workflow"),
            )
            .arg(
                clap::Arg::new("no-open-automation")
                    .long("no-open-automation")
                    .action(clap::ArgAction::SetTrue)
                    .help("Remove the open automation"),
            )
            .arg(
                clap::Arg::new("when-clicked-apply-tag")
                    .long("when-clicked-apply-tag")
                    .action(clap::ArgAction::Append)
                    .value_name("TAG")
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .help("On click, apply this tag"),
            )
            .arg(
                clap::Arg::new("when-clicked-remove-tag")
                    .long("when-clicked-remove-tag")
                    .action(clap::ArgAction::Append)
                    .value_name("TAG")
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .help("On click, remove this tag"),
            )
            .arg(
                clap::Arg::new("when-clicked-exit")
                    .long("when-clicked-exit")
                    .action(clap::ArgAction::SetTrue)
                    .help("On click, exit the workflow"),
            )
            .arg(
                clap::Arg::new("link")
                    .long("link")
                    .action(clap::ArgAction::Append)
                    .value_name("URL")
                    .value_parser(clap::value_parser!(aweber::workflows::LinkUrl))
                    .help("Limit the click rule to these links (repeatable) [default: every link]"),
            )
            .arg(
                clap::Arg::new("remove-click-rule")
                    .long("remove-click-rule")
                    .action(clap::ArgAction::Append)
                    .value_name("N")
                    .value_parser(clap::value_parser!(u64).range(1..))
                    .help("Delete click rule n"),
            )
            .about("Add a message step")
    }

    fn add_step_wait_command() -> clap::Command {
        clap::Command::new("wait")
            .arg(
                clap::Arg::new("for")
                    .long("for")
                    .value_name("DURATION")
                    .value_parser(clap::value_parser!(aweber::workflows::Delay))
                    .help("Wait this long, e.g. 45m, 4h, 2d, 1w, 6mo"),
            )
            .arg(
                clap::Arg::new("send-on")
                    .long("send-on")
                    .value_name("DAYS")
                    .value_parser(clap::value_parser!(aweber::workflows::SendDays))
                    .help(
                        "The days a subscriber may move on: comma-separated sun, mon, tue, wed, \
                         thu, fri, sat, or weekdays, or every-day",
                    ),
            )
            .arg(
                clap::Arg::new("send-at")
                    .long("send-at")
                    .value_name("HH:MM")
                    .value_parser(clap::value_parser!(aweber::workflows::SendTime))
                    .help("The time of day on those days: 24-hour, minutes 00 or 30"),
            )
            .arg(
                clap::Arg::new("subscriber-timezone")
                    .long("subscriber-timezone")
                    .value_name("yes|no")
                    .value_parser(clap::value_parser!(aweber::workflows::TimezoneSource))
                    .help(
                        "Resolve this step's time in the subscriber's own timezone instead of the \
                         workflow's",
                    ),
            )
            .about("Add a wait step")
    }

    fn add_step_tag_command() -> clap::Command {
        clap::Command::new("tag")
            .arg(
                clap::Arg::new("apply")
                    .long("apply")
                    .action(clap::ArgAction::Append)
                    .value_name("TAG")
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .help("Apply this tag (repeatable)"),
            )
            .arg(
                clap::Arg::new("remove")
                    .long("remove")
                    .action(clap::ArgAction::Append)
                    .value_name("TAG")
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .help("Remove this tag (repeatable)"),
            )
            .about("Add a tag step")
    }

    fn add_step_feed_command() -> clap::Command {
        clap::Command::new("feed")
            .arg(
                clap::Arg::new("message-id")
                    .required(true)
                    .value_name("MESSAGE-ID")
                    .value_parser(clap::value_parser!(aweber::ids::MessageId))
                    .help("The message this step sends"),
            )
            .arg(
                clap::Arg::new("url")
                    .long("url")
                    .required(true)
                    .value_name("RSS-URL")
                    .value_parser(clap::value_parser!(aweber::workflows::FeedUrl))
                    .help("The feed's url"),
            )
            .arg(
                clap::Arg::new("check-every")
                    .long("check-every")
                    .required(true)
                    .value_name("INTERVAL")
                    .value_parser(clap::value_parser!(aweber::workflows::Recurrence))
                    .help("How often to check the feed"),
            )
            .arg(
                clap::Arg::new("check-times")
                    .long("check-times")
                    .value_name("N")
                    .value_parser(clap::value_parser!(u32).range(1..))
                    .help("Stop checking after this many checks; steps may then follow the feed"),
            )
            .about("Add a feed step")
    }

    fn add_step_split_command() -> clap::Command {
        clap::Command::new("split")
            .arg(
                clap::Arg::new("when-tagged")
                    .long("when-tagged")
                    .value_name("TAG")
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .conflicts_with_all(["when-opened", "when-clicked"])
                    .help("The tag the subscriber is tested for"),
            )
            .arg(
                clap::Arg::new("when-opened")
                    .long("when-opened")
                    .action(clap::ArgAction::SetTrue)
                    .conflicts_with("when-clicked")
                    .help("Test whether any message the workflow sent was opened"),
            )
            .arg(
                clap::Arg::new("when-clicked")
                    .long("when-clicked")
                    .action(clap::ArgAction::SetTrue)
                    .help("Test whether any message the workflow sent was clicked"),
            )
            .arg(
                clap::Arg::new("link")
                    .long("link")
                    .value_name("URL")
                    .value_parser(clap::value_parser!(aweber::workflows::LinkUrl))
                    .help("Limit the click test to this link [default: every link]"),
            )
            .arg(
                clap::Arg::new("link-contains")
                    .long("link-contains")
                    .value_name("TEXT")
                    .value_parser(clap::value_parser!(aweber::workflows::LinkFragment))
                    .conflicts_with("link")
                    .help("Limit the click test to links containing this text"),
            )
            .about("Add a split step")
    }

    pub(crate) fn cli_add_workflow_step() -> clap::Command {
        clap::Command::new("")
            .arg(Self::add_step_workflow_positional())
            .args(Self::add_step_shared_args())
            .subcommand_required(true)
            .subcommand_value_name("KIND")
            .subcommand_help_heading("KIND")
            .disable_help_subcommand(true)
            .subcommand(Self::add_step_message_command())
            .subcommand(Self::add_step_wait_command())
            .subcommand(Self::add_step_tag_command())
            .subcommand(Self::add_step_feed_command())
            .subcommand(Self::add_step_split_command())
            .about("Add a step to a workflow")
    }

    pub(crate) fn cli_update_workflow_step() -> clap::Command {
        clap::Command::new("")
            .arg(Self::workflow_positional())
            .arg(
                clap::Arg::new("step-id")
                    .required(true)
                    .value_parser(clap::value_parser!(aweber::ids::RuleId))
                    .help("The step's id, as `show` reports it"),
            )
            .arg(Self::workflow_list_arg(false))
            .arg(
                clap::Arg::new("message")
                    .long("message")
                    .value_parser(clap::value_parser!(aweber::ids::MessageId))
                    .help("Swap the step's message"),
            )
            .args(Self::workflow_wait_args())
            .arg(
                clap::Arg::new("apply")
                    .long("apply")
                    .action(clap::ArgAction::Append)
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .help("A tag the step applies"),
            )
            .arg(
                clap::Arg::new("remove")
                    .long("remove")
                    .num_args(0..=1)
                    .action(clap::ArgAction::Append)
                    .value_parser(clap::value_parser!(aweber::workflows::Tag))
                    .help("A tag the step removes, or, with no value, delete the step"),
            )
            .args(Self::workflow_feed_args())
            .args(Self::workflow_split_args())
            .args(Self::workflow_automation_args())
            .args(Self::workflow_placement_args())
            .group(
                clap::ArgGroup::new("change")
                    .args([
                        "message",
                        "for",
                        "send-on",
                        "send-at",
                        "subscriber-timezone",
                        "apply",
                        "remove",
                        "url",
                        "check-every",
                        "check-times",
                        "when-tagged",
                        "when-opened",
                        "when-clicked",
                        "when-opened-apply-tag",
                        "when-opened-remove-tag",
                        "when-opened-exit",
                        "no-open-automation",
                        "when-clicked-apply-tag",
                        "when-clicked-remove-tag",
                        "when-clicked-exit",
                        "link",
                        "remove-click-rule",
                        "before",
                        "after",
                        "branch",
                        "inside",
                    ])
                    .required(true)
                    .multiple(true),
            )
            .about("Change, move or delete a workflow step")
    }

    pub(crate) fn cli_publish_workflow() -> clap::Command {
        clap::Command::new("")
            .arg(Self::workflow_positional())
            .arg(Self::workflow_list_arg(false))
            .arg(
                clap::Arg::new("discard")
                    .long("discard")
                    .action(clap::ArgAction::SetTrue)
                    .help("Throw away the unpublished changes instead of publishing"),
            )
            .about("Publish a workflow's working copy")
    }

    pub(crate) fn cli_delete_workflow() -> clap::Command {
        clap::Command::new("")
            .arg(Self::workflow_positional())
            .arg(Self::workflow_list_arg(false))
            .about("Delete a workflow and return its messages to Drafts")
    }

    // -----------------------------------------------------------------------
    // execute dispatch
    // -----------------------------------------------------------------------
    pub(crate) async fn execute(
        &self,
        command: CliCommand,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match command {
            CliCommand::Operation(operation) => self.execute_operation(operation, matches).await,
            CliCommand::Workflow(command) => self.execute_workflow(command, matches).await,
        }
    }

    async fn execute_workflow(
        &self,
        command: WorkflowCommand,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match command {
            WorkflowCommand::List => self.execute_list_workflows(matches).await,
            WorkflowCommand::Show => self.execute_show_workflow(matches).await,
            WorkflowCommand::Create => self.execute_create_workflow(matches).await,
            WorkflowCommand::Update => self.execute_update_workflow(matches).await,
            WorkflowCommand::AddStep => self.execute_add_workflow_step(matches).await,
            WorkflowCommand::UpdateStep => self.execute_update_workflow_step(matches).await,
            WorkflowCommand::Publish => self.execute_publish_workflow(matches).await,
            WorkflowCommand::Delete => self.execute_delete_workflow(matches).await,
        }
    }

    async fn execute_operation(
        &self,
        operation: aweber::catalog::Operation,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        use aweber::catalog::Operation;
        match operation {
            Operation::ListAccounts => self.execute_list_accounts(matches).await,
            Operation::GetAccount => self.execute_get_account(matches).await,
            Operation::FindAccountSubscribers => {
                self.execute_find_account_subscribers(matches).await
            }
            Operation::ListAccountWebformSplitTests => {
                self.execute_list_account_webform_split_tests(matches).await
            }
            Operation::ListAccountWebforms => self.execute_list_account_webforms(matches).await,
            Operation::ListIntegrations => self.execute_list_integrations(matches).await,
            Operation::GetIntegration => self.execute_get_integration(matches).await,
            Operation::ListLists => self.execute_list_lists(matches).await,
            Operation::FindLists => self.execute_find_lists(matches).await,
            Operation::GetList => self.execute_get_list(matches).await,
            Operation::ListBroadcasts => self.execute_list_broadcasts(matches).await,
            Operation::CreateBroadcast => self.execute_create_broadcast(matches).await,
            Operation::GetBroadcastTotal => self.execute_get_broadcast_total(matches).await,
            Operation::GetBroadcast => self.execute_get_broadcast(matches).await,
            Operation::UpdateBroadcast => self.execute_update_broadcast(matches).await,
            Operation::DeleteBroadcast => self.execute_delete_broadcast(matches).await,
            Operation::CancelBroadcast => self.execute_cancel_broadcast(matches).await,
            Operation::GetBroadcastClicks => self.execute_get_broadcast_clicks(matches).await,
            Operation::GetBroadcastOpens => self.execute_get_broadcast_opens(matches).await,
            Operation::ScheduleBroadcast => self.execute_schedule_broadcast(matches).await,
            Operation::WaitBroadcast => self.execute_wait_broadcast(matches).await,
            Operation::ListCampaigns => self.execute_list_campaigns(matches).await,
            Operation::ListCampaignStats => self.execute_list_campaign_stats(matches).await,
            Operation::GetCampaignStat => self.execute_get_campaign_stat(matches).await,
            Operation::FindCampaigns => self.execute_find_campaigns(matches).await,
            Operation::GetCampaign => self.execute_get_campaign(matches).await,
            Operation::ListCustomFields => self.execute_list_custom_fields(matches).await,
            Operation::CreateCustomField => self.execute_create_custom_field(matches).await,
            Operation::GetCustomField => self.execute_get_custom_field(matches).await,
            Operation::DeleteCustomField => self.execute_delete_custom_field(matches).await,
            Operation::UpdateCustomField => self.execute_update_custom_field(matches).await,
            Operation::ListLandingPages => self.execute_list_landing_pages(matches).await,
            Operation::GetLandingPage => self.execute_get_landing_page(matches).await,
            Operation::CreatePurchase => self.execute_create_purchase(matches).await,
            Operation::ListSegments => self.execute_list_segments(matches).await,
            Operation::GetSegment => self.execute_get_segment(matches).await,
            Operation::ListSubscribers => self.execute_list_subscribers(matches).await,
            Operation::CreateSubscriber => self.execute_create_subscriber(matches).await,
            Operation::DeleteSubscriberByEmail => {
                self.execute_delete_subscriber_by_email(matches).await
            }
            Operation::UpdateSubscriberByEmail => {
                self.execute_update_subscriber_by_email(matches).await
            }
            Operation::FindSubscribers => self.execute_find_subscribers(matches).await,
            Operation::GetSubscriber => self.execute_get_subscriber(matches).await,
            Operation::MoveSubscriber => self.execute_move_subscriber(matches).await,
            Operation::DeleteSubscriber => self.execute_delete_subscriber(matches).await,
            Operation::UpdateSubscriber => self.execute_update_subscriber(matches).await,
            Operation::GetSubscriberActivity => self.execute_get_subscriber_activity(matches).await,
            Operation::ListTags => self.execute_list_tags(matches).await,
            Operation::ListWebFormSplitTests => {
                self.execute_list_web_form_split_tests(matches).await
            }
            Operation::GetWebFormSplitTest => self.execute_get_web_form_split_test(matches).await,
            Operation::ListWebFormSplitTestComponents => {
                self.execute_list_web_form_split_test_components(matches)
                    .await
            }
            Operation::GetWebFormSplitTestComponent => {
                self.execute_get_web_form_split_test_component(matches)
                    .await
            }
            Operation::ListWebForms => self.execute_list_web_forms(matches).await,
            Operation::GetWebForm => self.execute_get_web_form(matches).await,
            Operation::GetBroadcastLinkAnalytics => {
                self.execute_get_broadcast_link_analytics(matches).await
            }
            Operation::UnsubscribeSubscriber => self.execute_unsubscribe_subscriber(matches).await,
            // The catalog's one-request workflow operations are the TUI's; the
            // `workflows` group the binary routes is `WorkflowCommand`.
            Operation::ListWorkflows
            | Operation::GetWorkflow
            | Operation::TreeWorkflow
            | Operation::CreateWorkflow
            | Operation::UpdateWorkflow
            | Operation::UpdateWorkflowRuleset
            | Operation::PublishWorkflow
            | Operation::RevertWorkflow
            | Operation::SetWorkflowState
            | Operation::CopyWorkflow
            | Operation::DeleteWorkflow
            | Operation::GetWorkflowStats
            | Operation::GetWorkflowMessageStats
            | Operation::GetWorkflowEventHistory => anyhow::bail!(
                "`aweber {} {}` is offered in the TUI alone",
                operation.group(),
                operation.action()
            ),
            Operation::OauthGetAccessToken => self.execute_oauth_get_access_token(matches).await,
            Operation::OauthGetRequestToken => self.execute_oauth_get_request_token(matches).await,
            Operation::OauthRevoke => self.execute_oauth_revoke(matches).await,
            Operation::OauthToken => self.execute_oauth_token(matches).await,
        }
    }
    // -----------------------------------------------------------------------
    // execute_* methods - call aweber::endpoints::* directly
    // -----------------------------------------------------------------------

    async fn resolve_list_id(&self, matches: &clap::ArgMatches) -> anyhow::Result<i32> {
        if let Some(&id) = matches.try_get_one::<i32>("list-id").ok().flatten() {
            return Ok(id);
        }
        let name = matches
            .try_get_one::<String>("list")
            .ok()
            .flatten()
            .context("a list is required: pass --list-id or --list")?;
        let result = aweber::endpoints::find_lists(
            &self.client,
            self.account_id,
            Some(name),
            None,
            None,
            None,
        )
        .await
        .context("failed to look up list by name")?;
        let wanted = name.to_lowercase();
        let mut matches_iter = result.entries.iter().filter(|l| {
            [l.name.as_deref(), l.unique_list_id.as_deref()]
                .into_iter()
                .flatten()
                .any(|candidate| candidate.to_lowercase() == wanted)
        });
        let list = matches_iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("no list is named or identified by '{name}'"))?;
        if matches_iter.next().is_some() {
            anyhow::bail!("multiple lists match '{name}', use --list-id instead");
        }
        list.id
            .map(|id| id as i32)
            .ok_or_else(|| anyhow::anyhow!("list '{name}' found but has no ID"))
    }

    pub(crate) async fn resolve_list_uid(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<aweber::ids::ListUid> {
        let list_id = self.resolve_list_id(matches).await?;
        let document: serde_json::Value = aweber::client::ApiRequest::new(
            &self.client,
            reqwest::Method::GET,
            format!("/1.0/accounts/{}/lists/{list_id}", self.account_id),
        )
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))
        .context("failed to look up the list uid")?;
        document
            .get("uuid")
            .and_then(serde_json::Value::as_str)
            .and_then(|member| member.parse().ok())
            .with_context(|| format!("list {list_id} has no uid"))
    }

    async fn resolve_subscriber_id(
        &self,
        matches: &clap::ArgMatches,
        list_id: i32,
    ) -> anyhow::Result<i32> {
        if let Some(&id) = matches.get_one::<i32>("subscriber-id") {
            return Ok(id);
        }
        let email = matches.get_one::<String>("email").unwrap();
        let result = aweber::endpoints::find_subscribers(
            &self.client,
            self.account_id,
            list_id,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(email),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .context("failed to look up subscriber by email")?;
        let mut matches_iter = result
            .entries
            .iter()
            .filter(|s| s.email.as_deref() == Some(email));
        let subscriber = matches_iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("no subscriber found with email '{email}'"))?;
        if matches_iter.next().is_some() {
            anyhow::bail!(
                "multiple subscribers found with email '{email}', use --subscriber-id instead"
            );
        }
        subscriber
            .id
            .map(|id| id as i32)
            .ok_or_else(|| anyhow::anyhow!("subscriber '{email}' found but has no ID"))
    }

    async fn resolve_custom_field_id(
        &self,
        matches: &clap::ArgMatches,
        list_id: i32,
    ) -> anyhow::Result<i32> {
        if let Some(&id) = matches.get_one::<i32>("custom-field-id") {
            return Ok(id);
        }
        let name = matches.get_one::<String>("custom-field").unwrap();
        let result = aweber::endpoints::list_custom_fields(
            &self.client,
            self.account_id,
            list_id,
            None,
            None,
        )
        .await
        .context("failed to look up custom field by name")?;
        let mut matches_iter = result
            .entries
            .iter()
            .filter(|cf| cf.name.as_ref() == Some(name));
        let field = matches_iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("no custom field found with name '{name}'"))?;
        if matches_iter.next().is_some() {
            anyhow::bail!(
                "multiple custom fields found with name '{name}', use --custom-field-id instead"
            );
        }
        field
            .id
            .map(|id| id as i32)
            .ok_or_else(|| anyhow::anyhow!("custom field '{name}' found but has no ID"))
    }

    pub(crate) async fn execute_list_accounts(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let ws_size = matches.get_one::<std::num::NonZeroU32>("ws-size").copied();
        let ws_start = matches.get_one::<i32>("ws-start").copied();
        let result = aweber::endpoints::get_accounts(&self.client, ws_size, ws_start).await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_account(
        &self,
        _matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = aweber::endpoints::get_account(&self.client, self.account_id).await;
        self.print_result(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn execute_find_account_subscribers(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let tags = matches
            .get_one::<String>("tags")
            .map(|s| serde_json::to_string(&[s]).unwrap());
        let tags_not_in = matches
            .get_one::<String>("tags-not-in")
            .map(|s| serde_json::to_string(&[s]).unwrap());
        let result = aweber::endpoints::find_account_subscribers(
            &self.client,
            self.account_id,
            matches
                .get_one::<types::GetAccountsFindsubscribersAdTracking>("ad-tracking")
                .map(|v| v.as_str()),
            matches.get_one::<i32>("area-code").copied(),
            matches
                .get_one::<types::GetAccountsFindsubscribersCity>("city")
                .map(|v| v.as_str()),
            matches
                .get_one::<types::GetAccountsFindsubscribersCountry>("country")
                .map(|v| v.as_str()),
            matches
                .get_one::<String>("custom-fields")
                .map(|s| s.as_str()),
            matches.get_one::<i32>("dma-code").copied(),
            matches
                .get_one::<types::GetAccountsFindsubscribersEmail>("email")
                .map(|v| v.as_str()),
            matches
                .get_one::<i32>("last-followup-message-number-sent")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("last-followup-message-sent-at")
                .copied(),
            matches.get_one::<f64>("latitude").copied(),
            matches.get_one::<f64>("longitude").copied(),
            matches
                .get_one::<types::GetAccountsFindsubscribersMiscNotes>("misc-notes")
                .map(|v| v.as_str()),
            matches
                .get_one::<types::GetAccountsFindsubscribersName>("name")
                .map(|v| v.as_str()),
            matches
                .get_one::<types::GetAccountsFindsubscribersPostalCode>("postal-code")
                .map(|v| v.as_str()),
            matches
                .get_one::<types::GetAccountsFindsubscribersRegion>("region")
                .map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsFindsubscribersStatus>("status"),
            matches
                .get_one::<chrono::NaiveDate>("subscribed-after")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("subscribed-at")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("subscribed-before")
                .copied(),
            matches.get_one::<types::GetAccountsFindsubscribersSubscriptionMethod>(
                "subscription-method",
            ),
            tags.as_deref(),
            tags_not_in.as_deref(),
            matches.get_one::<types::GetAccountsFindsubscribersUnsubscribeMethod>(
                "unsubscribe-method",
            ),
            matches
                .get_one::<chrono::NaiveDate>("unsubscribed-after")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("unsubscribed-at")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("unsubscribed-before")
                .copied(),
            matches.get_one::<chrono::NaiveDate>("verified-at").copied(),
            matches.get_one::<types::GetAccountsFindsubscribersWsShow>("ws-show"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_list_account_webform_split_tests(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = aweber::endpoints::list_account_webform_split_tests(
            &self.client,
            self.account_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_list_account_webforms(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = aweber::endpoints::list_account_webforms(
            &self.client,
            self.account_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_list_integrations(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = aweber::endpoints::list_integrations(
            &self.client,
            self.account_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_integration(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let integration_id = *matches.get_one::<i32>("integration-id").unwrap();
        let result =
            aweber::endpoints::get_integration(&self.client, self.account_id, integration_id).await;
        self.print_result(result)
    }

    pub(crate) async fn execute_list_lists(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = aweber::endpoints::list_lists(
            &self.client,
            self.account_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_find_lists(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = aweber::endpoints::find_lists(
            &self.client,
            self.account_id,
            matches
                .get_one::<types::GetAccountsListsFindName>("name")
                .map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsListsFindWsShow>("ws-show"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_list(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::get_list(&self.client, self.account_id, list_id).await;
        self.print_result(result)
    }

    pub(crate) async fn execute_list_broadcasts(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let ws_size = matches.get_one::<std::num::NonZeroU32>("ws-size").copied();
        let ws_start = matches.get_one::<i32>("ws-start").copied();
        use types::GetAccountsListsBroadcastsStatus::*;
        let statuses = match matches.get_one::<types::GetAccountsListsBroadcastsStatus>("status") {
            Some(&status) => vec![status],
            None => vec![Draft, Scheduled, Sent],
        };
        let mut limit = Limit::from_matches(matches);
        for status in &statuses {
            if limit.is_exhausted() {
                break;
            }
            let result = aweber::endpoints::list_broadcasts(
                &self.client,
                self.account_id,
                list_id,
                Some(status),
                ws_size,
                ws_start,
            )
            .await;
            self.print_paginated_ndjson(result, &mut limit).await?;
        }
        Ok(())
    }
    pub(crate) async fn execute_create_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::CreateBroadcast>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) = matches.get_one::<String>("body-amp") {
                body.insert("body_amp".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("body-html") {
                body.insert("body_html".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("body-text") {
                body.insert("body_text".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<bool>("click-tracking-enabled") {
                body.insert("click_tracking_enabled".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("exclude-lists") {
                body.insert("exclude_lists".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("facebook-integration") {
                body.insert("facebook_integration".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("include-lists") {
                body.insert("include_lists".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<bool>("is-archived") {
                body.insert("is_archived".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<bool>("notify-on-send") {
                body.insert("notify_on_send".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("subject") {
                body.insert("subject".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("twitter-integration") {
                body.insert("twitter_integration".into(), serde_json::json!(v));
            }
            serde_json::from_value::<types::CreateBroadcast>(serde_json::Value::Object(body))?
        };
        let result =
            aweber::endpoints::create_broadcast(&self.client, self.account_id, list_id, &body)
                .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_get_broadcast_total(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::get_broadcast_total(
            &self.client,
            self.account_id,
            list_id,
            matches
                .get_one::<types::GetAccountsListsBroadcastsTotalStatus>("status")
                .unwrap(),
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_get_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result =
            aweber::endpoints::get_broadcast(&self.client, self.account_id, list_id, broadcast_id)
                .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_update_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::UpdateBroadcast>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) = matches.get_one::<String>("body-amp") {
                body.insert("body_amp".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("body-html") {
                body.insert("body_html".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("body-text") {
                body.insert("body_text".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<bool>("click-tracking-enabled") {
                body.insert("click_tracking_enabled".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("exclude-lists") {
                body.insert("exclude_lists".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("facebook-integration") {
                body.insert("facebook_integration".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("include-lists") {
                body.insert("include_lists".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<bool>("is-archived") {
                body.insert("is_archived".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<bool>("notify-on-send") {
                body.insert("notify_on_send".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("segment-link") {
                body.insert("segment_link".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("subject") {
                body.insert("subject".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("twitter-integration") {
                body.insert("twitter_integration".into(), serde_json::json!(v));
            }
            serde_json::from_value::<types::UpdateBroadcast>(serde_json::Value::Object(body))?
        };
        let result = aweber::endpoints::update_broadcast(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
            &body,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_delete_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result = aweber::endpoints::delete_broadcast(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
        )
        .await;
        self.print_void(result)
    }

    pub(crate) async fn execute_cancel_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result = aweber::endpoints::cancel_broadcast(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_get_broadcast_clicks(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result = aweber::endpoints::get_broadcast_clicks(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
            matches.get_one::<String>("after").map(|s| s.as_str()),
            matches.get_one::<String>("before").map(|s| s.as_str()),
            matches.get_one::<bool>("detailed").copied(),
            matches
                .get_one::<std::num::NonZeroU64>("page-size")
                .copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_broadcast_opens(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result = aweber::endpoints::get_broadcast_opens(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
            matches.get_one::<String>("after").map(|s| s.as_str()),
            matches.get_one::<String>("before").map(|s| s.as_str()),
            matches
                .get_one::<std::num::NonZeroU64>("page-size")
                .copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_schedule_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::ScheduleBroadcast>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) = matches.get_one::<chrono::DateTime<chrono::Utc>>("scheduled-for") {
                body.insert("scheduled_for".into(), serde_json::json!(v.to_rfc3339()));
            }
            serde_json::from_value::<types::ScheduleBroadcast>(serde_json::Value::Object(body))?
        };
        let result = aweber::endpoints::schedule_broadcast(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
            &body,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_wait_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let interval = *matches.get_one::<u64>("interval").unwrap();

        loop {
            let broadcast = aweber::endpoints::get_broadcast(
                &self.client,
                self.account_id,
                list_id,
                broadcast_id,
            )
            .await?;

            match broadcast.status {
                Some(types::BroadcastStatus::Sent) => {
                    return self.print_result(Ok::<_, aweber::client::ApiError>(broadcast));
                }
                Some(
                    ref status @ (types::BroadcastStatus::Sending
                    | types::BroadcastStatus::Scheduled),
                ) => {
                    eprintln!(
                        "Broadcast {broadcast_id} status: {status} — polling again in {interval}s"
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
                }
                Some(ref status) => {
                    anyhow::bail!(
                        "Broadcast {broadcast_id} has status: {status} — cannot wait for completion"
                    );
                }
                None => {
                    anyhow::bail!(
                        "Broadcast {broadcast_id} has no status — cannot wait for completion"
                    );
                }
            }
        }
    }

    pub(crate) async fn execute_list_campaigns(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::list_campaigns(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_list_campaign_stats(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let campaign_id = *matches.get_one::<i32>("campaign-id").unwrap();
        let result = aweber::endpoints::list_campaign_stats(
            &self.client,
            self.account_id,
            list_id,
            campaign_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_campaign_stat(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let campaign_id = *matches.get_one::<i32>("campaign-id").unwrap();
        let stats_id = matches
            .get_one::<types::GetAccountsListsCampaignsBcampaignidStats2StatsId>("stats-id")
            .unwrap();
        let result = aweber::endpoints::get_campaign_stat(
            &self.client,
            self.account_id,
            list_id,
            campaign_id,
            stats_id,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_find_campaigns(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::find_campaigns(
            &self.client,
            self.account_id,
            list_id,
            matches
                .get_one::<types::GetAccountsListsCampaignsFindCampaignType>("campaign-type")
                .unwrap(),
            matches.get_one::<types::GetAccountsListsCampaignsFindWsShow>("ws-show"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_campaign(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let campaign_id = *matches.get_one::<i32>("campaign-id").unwrap();
        let campaign_type = matches
            .get_one::<types::GetAccountsListsCampaignsCampaigntypecampaignidCampaignType>(
                "campaign-type",
            )
            .unwrap();
        let result = aweber::endpoints::get_campaign(
            &self.client,
            self.account_id,
            list_id,
            &campaign_type.to_string(),
            campaign_id,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_list_custom_fields(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::list_custom_fields(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_create_custom_field(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::PostAccountsListsCustomFieldsBody>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) = matches.get_one::<String>("name") {
                body.insert("name".into(), serde_json::json!(v));
            }
            body.insert("ws.op".into(), serde_json::json!("create"));
            serde_json::from_value::<types::PostAccountsListsCustomFieldsBody>(
                serde_json::Value::Object(body),
            )?
        };
        let result =
            aweber::endpoints::create_custom_field(&self.client, self.account_id, list_id, &body)
                .await;
        match result {
            Err(e) if e.api_message_is("name: Must be unique") => {
                let name = matches
                    .get_one::<String>("name")
                    .map(|s| s.as_str())
                    .unwrap_or("unknown");
                Err(e).context(format!("custom field '{name}' already exists"))
            }
            other => self.print_void(other),
        }
    }

    pub(crate) async fn execute_get_custom_field(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let custom_field_id = self.resolve_custom_field_id(matches, list_id).await?;
        let result = aweber::endpoints::get_custom_field(
            &self.client,
            self.account_id,
            list_id,
            custom_field_id,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_delete_custom_field(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let custom_field_id = self.resolve_custom_field_id(matches, list_id).await?;
        let result = aweber::endpoints::delete_custom_field(
            &self.client,
            self.account_id,
            list_id,
            custom_field_id,
        )
        .await;
        self.print_void(result)
    }

    pub(crate) async fn execute_update_custom_field(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let custom_field_id = self.resolve_custom_field_id(matches, list_id).await?;
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::PatchAccountsListsCustomFieldsBody>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) = matches.get_one::<bool>("is-subscriber-updateable") {
                body.insert("is_subscriber_updateable".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("name") {
                body.insert("name".into(), serde_json::json!(v));
            }
            serde_json::from_value::<types::PatchAccountsListsCustomFieldsBody>(
                serde_json::Value::Object(body),
            )?
        };
        let result = aweber::endpoints::update_custom_field(
            &self.client,
            self.account_id,
            list_id,
            custom_field_id,
            &body,
        )
        .await;
        self.print_result(result)
    }
    pub(crate) async fn execute_list_landing_pages(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::list_landing_pages(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_landing_page(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let landing_page_id = *matches.get_one::<uuid::Uuid>("landing-page-id").unwrap();
        let result = aweber::endpoints::get_landing_page(
            &self.client,
            self.account_id,
            list_id,
            landing_page_id,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_create_purchase(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::Purchase>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) = matches.get_one::<types::PurchaseAdTracking>("ad-tracking") {
                body.insert("ad_tracking".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<String>("currency") {
                body.insert("currency".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<types::PurchaseEmail>("email") {
                body.insert("email".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<String>("event-note") {
                body.insert("event_note".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("event-time") {
                body.insert("event_time".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<types::PurchaseIpAddress>("ip-address") {
                body.insert("ip_address".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::PurchaseMiscNotes>("misc-notes") {
                body.insert("misc_notes".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::PurchaseName>("name") {
                body.insert("name".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<String>("product-name") {
                body.insert("product_name".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("url") {
                body.insert("url".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<f64>("value") {
                body.insert("value".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<String>("vendor") {
                body.insert("vendor".into(), serde_json::json!(v));
            }
            serde_json::from_value::<types::Purchase>(serde_json::Value::Object(body))?
        };
        let result =
            aweber::endpoints::create_purchase(&self.client, self.account_id, list_id, &body).await;
        self.print_void(result)
    }

    pub(crate) async fn execute_list_segments(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::list_segments(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_segment(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let segment_id = *matches.get_one::<i32>("segment-id").unwrap();
        let result =
            aweber::endpoints::get_segment(&self.client, self.account_id, list_id, segment_id)
                .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_list_subscribers(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::list_subscribers(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<types::GetAccountsListsSubscribersSortOrder>("sort-order"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_create_subscriber(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::AddSubscriberRequestBody>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) =
                matches.get_one::<types::AddSubscriberRequestBodyAdTracking>("ad-tracking")
            {
                body.insert("ad_tracking".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyEmail>("email") {
                body.insert("email".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) =
                matches.get_one::<types::AddSubscriberRequestBodyIpAddress>("ip-address")
            {
                body.insert("ip_address".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<i64>("last-followup-message-number-sent") {
                body.insert(
                    "last_followup_message_number_sent".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) =
                matches.get_one::<types::AddSubscriberRequestBodyMiscNotes>("misc-notes")
            {
                body.insert("misc_notes".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyName>("name") {
                body.insert("name".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyStrictCustomFields>(
                "strict-custom-fields",
            ) {
                body.insert(
                    "strict_custom_fields".into(),
                    serde_json::json!(v.to_string()),
                );
            }
            if let Some(v) =
                matches.get_one::<types::AddSubscriberRequestBodyUpdateExisting>("update-existing")
            {
                body.insert("update_existing".into(), serde_json::json!(v.to_string()));
            }
            serde_json::from_value::<types::AddSubscriberRequestBody>(serde_json::Value::Object(
                body,
            ))?
        };
        match aweber::endpoints::create_subscriber(&self.client, self.account_id, list_id, &body)
            .await
        {
            Err(e) if e.api_message_is("email: Subscriber already subscribed.") => {
                Err(e).context("subscriber already on list")
            }
            other => self.print_void(other),
        }
    }

    pub(crate) async fn execute_delete_subscriber_by_email(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_email = matches
            .get_one::<types::DeleteAccountsListsSubscribersSubscriberEmail>("email")
            .unwrap();
        let result = aweber::endpoints::delete_subscriber_by_email(
            &self.client,
            self.account_id,
            list_id,
            subscriber_email,
        )
        .await;
        self.print_void(result)
    }
    pub(crate) async fn execute_update_subscriber_by_email(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_email = matches
            .get_one::<types::PatchAccountsListsSubscribersSubscriberEmail>("email")
            .unwrap();
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<serde_json::Value>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) =
                matches.get_one::<types::UpdateSubscriberRequestBodyAdTracking>("ad-tracking")
            {
                body.insert("ad_tracking".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<String>("new-email") {
                body.insert("email".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<i64>("last-followup-message-number-sent") {
                body.insert(
                    "last_followup_message_number_sent".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = matches.get_one::<String>("misc-notes") {
                body.insert("misc_notes".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyName>("name") {
                body.insert("name".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyStatus>("status") {
                body.insert("status".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches
                .get_one::<types::UpdateSubscriberRequestBodyStrictCustomFields>(
                    "strict-custom-fields",
                )
            {
                body.insert(
                    "strict_custom_fields".into(),
                    serde_json::json!(v.to_string()),
                );
            }
            if let Some(vals) = matches.get_many::<String>("custom-field") {
                let cf: serde_json::Map<String, serde_json::Value> = vals
                    .map(|s| match s.split_once('=') {
                        Some((k, v)) => (k.to_string(), serde_json::json!(v)),
                        None => (s.to_string(), serde_json::Value::Null),
                    })
                    .collect();
                body.insert("custom_fields".into(), serde_json::Value::Object(cf));
            }
            serde_json::Value::Object(body)
        };
        let result = aweber::endpoints::update_subscriber_by_email(
            &self.client,
            self.account_id,
            list_id,
            subscriber_email,
            &body,
        )
        .await;
        self.print_result(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn execute_find_subscribers(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let tags = matches
            .get_one::<String>("tags")
            .map(|s| serde_json::to_string(&[s]).unwrap());
        let tags_not_in = matches
            .get_one::<String>("tags-not-in")
            .map(|s| serde_json::to_string(&[s]).unwrap());
        let result = aweber::endpoints::find_subscribers(
            &self.client,
            self.account_id,
            list_id,
            matches
                .get_one::<types::GetAccountsListsSubscribersFindAdTracking>("ad-tracking")
                .map(|v| v.as_str()),
            matches.get_one::<i32>("area-code").copied(),
            matches
                .get_one::<types::GetAccountsListsSubscribersFindCity>("city")
                .map(|v| v.as_str()),
            matches
                .get_one::<types::GetAccountsListsSubscribersFindCountry>("country")
                .map(|v| v.as_str()),
            matches
                .get_one::<String>("custom-fields")
                .map(|s| s.as_str()),
            matches.get_one::<i32>("dma-code").copied(),
            matches
                .get_one::<types::GetAccountsListsSubscribersFindEmail>("email")
                .map(|v| v.as_str()),
            matches
                .get_one::<i32>("last-followup-message-number-sent")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("last-followup-message-sent-at")
                .copied(),
            matches.get_one::<f64>("latitude").copied(),
            matches.get_one::<f64>("longitude").copied(),
            matches
                .get_one::<types::GetAccountsListsSubscribersFindMiscNotes>("misc-notes")
                .map(|v| v.as_str()),
            matches
                .get_one::<types::GetAccountsListsSubscribersFindName>("name")
                .map(|v| v.as_str()),
            matches
                .get_one::<types::GetAccountsListsSubscribersFindPostalCode>("postal-code")
                .map(|v| v.as_str()),
            matches
                .get_one::<types::GetAccountsListsSubscribersFindRegion>("region")
                .map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsListsSubscribersFindSortKey>("sort-key"),
            matches.get_one::<types::GetAccountsListsSubscribersFindSortOrder>("sort-order"),
            matches.get_one::<types::GetAccountsListsSubscribersFindStatus>("status"),
            matches
                .get_one::<chrono::NaiveDate>("subscribed-after")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("subscribed-at")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("subscribed-before")
                .copied(),
            matches.get_one::<types::GetAccountsListsSubscribersFindSubscriptionMethod>(
                "subscription-method",
            ),
            tags.as_deref(),
            tags_not_in.as_deref(),
            matches.get_one::<types::GetAccountsListsSubscribersFindUnsubscribeMethod>(
                "unsubscribe-method",
            ),
            matches
                .get_one::<chrono::NaiveDate>("unsubscribed-after")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("unsubscribed-at")
                .copied(),
            matches
                .get_one::<chrono::NaiveDate>("unsubscribed-before")
                .copied(),
            matches.get_one::<chrono::NaiveDate>("verified-at").copied(),
            matches.get_one::<types::GetAccountsListsSubscribersFindWsShow>("ws-show"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_subscriber(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_id = self.resolve_subscriber_id(matches, list_id).await?;
        let result = aweber::endpoints::get_subscriber(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_move_subscriber(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_id = self.resolve_subscriber_id(matches, list_id).await?;
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::MoveSubscriberRequestBody>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) = matches.get_one::<bool>("enforce-custom-field-mapping") {
                body.insert("enforce_custom_field_mapping".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<i64>("last-followup-message-number-sent") {
                body.insert(
                    "last_followup_message_number_sent".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = matches.get_one::<String>("list-link") {
                body.insert("list_link".into(), serde_json::json!(v));
            }
            body.insert("ws.op".into(), serde_json::json!("move"));
            serde_json::from_value::<types::MoveSubscriberRequestBody>(serde_json::Value::Object(
                body,
            ))?
        };
        let result = aweber::endpoints::move_subscriber(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
            &body,
        )
        .await;
        self.print_void(result)
    }

    pub(crate) async fn execute_delete_subscriber(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_id = self.resolve_subscriber_id(matches, list_id).await?;
        let result = aweber::endpoints::delete_subscriber(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
        )
        .await;
        self.print_void(result)
    }

    pub(crate) async fn execute_update_subscriber(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_id = self.resolve_subscriber_id(matches, list_id).await?;
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<serde_json::Value>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) =
                matches.get_one::<types::UpdateSubscriberRequestBodyAdTracking>("ad-tracking")
            {
                body.insert("ad_tracking".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<String>("new-email") {
                body.insert("email".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<i64>("last-followup-message-number-sent") {
                body.insert(
                    "last_followup_message_number_sent".into(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = matches.get_one::<String>("misc-notes") {
                body.insert("misc_notes".into(), serde_json::json!(v));
            }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyName>("name") {
                body.insert("name".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyStatus>("status") {
                body.insert("status".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches
                .get_one::<types::UpdateSubscriberRequestBodyStrictCustomFields>(
                    "strict-custom-fields",
                )
            {
                body.insert(
                    "strict_custom_fields".into(),
                    serde_json::json!(v.to_string()),
                );
            }
            if let Some(vals) = matches.get_many::<String>("custom-field") {
                let cf: serde_json::Map<String, serde_json::Value> = vals
                    .map(|s| match s.split_once('=') {
                        Some((k, v)) => (k.to_string(), serde_json::json!(v)),
                        None => (s.to_string(), serde_json::Value::Null),
                    })
                    .collect();
                body.insert("custom_fields".into(), serde_json::Value::Object(cf));
            }
            serde_json::Value::Object(body)
        };
        let result = aweber::endpoints::update_subscriber(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
            &body,
        )
        .await;
        self.print_result(result)
    }
    pub(crate) async fn execute_unsubscribe_subscriber(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_id = self.resolve_subscriber_id(matches, list_id).await?;
        let result = aweber::endpoints::update_subscriber(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
            &serde_json::json!({ "status": "unsubscribed" }),
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_get_subscriber_activity(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_id = self.resolve_subscriber_id(matches, list_id).await?;
        let result = aweber::endpoints::get_subscriber_activity(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_list_tags(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::list_tags(&self.client, self.account_id, list_id).await;
        self.print_result(result)
    }

    pub(crate) async fn execute_list_web_form_split_tests(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::list_web_form_split_tests(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_web_form_split_test(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let split_test_id = *matches.get_one::<i32>("split-test-id").unwrap();
        let result = aweber::endpoints::get_web_form_split_test(
            &self.client,
            self.account_id,
            list_id,
            split_test_id,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_list_web_form_split_test_components(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let split_test_id = *matches.get_one::<i32>("split-test-id").unwrap();
        let result = aweber::endpoints::list_web_form_split_test_components(
            &self.client,
            self.account_id,
            list_id,
            split_test_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_web_form_split_test_component(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let split_test_id = *matches.get_one::<i32>("split-test-id").unwrap();
        let split_test_component_id: i32 = matches
            .get_one::<String>("split-test-component-id")
            .unwrap()
            .parse()
            .context("split-test-component-id must be an integer")?;
        let result = aweber::endpoints::get_web_form_split_test_component(
            &self.client,
            self.account_id,
            list_id,
            split_test_id,
            split_test_component_id,
        )
        .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_list_web_forms(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = aweber::endpoints::list_web_forms(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, &mut Limit::from_matches(matches))
            .await
    }

    pub(crate) async fn execute_get_web_form(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let webform_id = *matches.get_one::<i32>("webform-id").unwrap();
        let result =
            aweber::endpoints::get_web_form(&self.client, self.account_id, list_id, webform_id)
                .await;
        self.print_result(result)
    }

    pub(crate) async fn execute_get_broadcast_link_analytics(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let account = self.account_uid().await?;
        let broadcast_uuid: uuid::Uuid = matches
            .get_one::<types::GetBroadcastLinksAnalyticsBroadcastId>("broadcast-id")
            .expect("--broadcast-id is required")
            .as_str()
            .parse()
            .context("--broadcast-id must be a UUID")?;
        let filter = matches
            .get_one::<types::GetBroadcastLinksAnalyticsFilter>("filter")
            .unwrap();
        let before = matches.get_one::<i64>("before").copied();
        let max_count = matches.get_one::<u64>("max-count").copied();
        let min_count = matches.get_one::<u64>("min-count").copied();
        let page_size = matches
            .get_one::<std::num::NonZeroU64>("page-size")
            .copied();
        let sort_asc = matches.get_one::<bool>("sort-asc").copied();
        let sort_by = matches.get_one::<types::GetBroadcastLinksAnalyticsSortBy>("sort-by");
        let mut limit = Limit::from_matches(matches);
        let mut after = matches.get_one::<String>("after").cloned();
        loop {
            let page = aweber::endpoints::get_broadcast_link_analytics(
                &self.client,
                &account,
                broadcast_uuid,
                filter,
                after.as_deref(),
                before,
                max_count,
                min_count,
                page_size,
                sort_asc,
                sort_by,
            )
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
            let budget = limit.remaining();
            self.print_ndjson(&page.entries, &mut limit)?;
            let already_warned = budget.is_some_and(|budget| budget < page.entries.len());
            match page.next_cursor {
                Some(_) if limit.is_exhausted() => {
                    if !already_warned {
                        limit.warn_truncated(true);
                    }
                    return Ok(());
                }
                Some(cursor) => after = Some(cursor.to_string()),
                None => return Ok(()),
            }
        }
    }

    pub(crate) async fn execute_oauth_get_access_token(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::PostOauthAccessTokenBody>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) = matches.get_one::<types::OauthCallback>("oauth-callback") {
                body.insert("oauth_callback".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::OauthConsumerKey>("oauth-consumer-key") {
                body.insert(
                    "oauth_consumer_key".into(),
                    serde_json::json!(v.to_string()),
                );
            }
            if let Some(v) = matches.get_one::<types::OauthNonce>("oauth-nonce") {
                body.insert("oauth_nonce".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::OauthSignature>("oauth-signature") {
                body.insert("oauth_signature".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) =
                matches.get_one::<types::OauthSignatureMethod>("oauth-signature-method")
            {
                body.insert(
                    "oauth_signature_method".into(),
                    serde_json::json!(v.to_string()),
                );
            }
            if let Some(v) = matches.get_one::<types::OauthTimestamp>("oauth-timestamp") {
                body.insert("oauth_timestamp".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::OauthToken>("oauth-token") {
                body.insert("oauth_token".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::OauthVersion>("oauth-version") {
                body.insert("oauth_version".into(), serde_json::json!(v.to_string()));
            }
            serde_json::from_value::<types::PostOauthAccessTokenBody>(serde_json::Value::Object(
                body,
            ))?
        };
        let result = aweber::endpoints::oauth_get_access_token(&self.client, &body).await;
        self.print_result(result)
    }

    pub(crate) async fn execute_oauth_get_request_token(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let body = if let Some(path) = matches.get_one::<std::path::PathBuf>("json-body") {
            let txt = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str::<types::PostOauthRequestTokenBody>(&txt)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            let mut body = serde_json::Map::new();
            if let Some(v) = matches.get_one::<types::OauthCallback>("oauth-callback") {
                body.insert("oauth_callback".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::OauthConsumerKey>("oauth-consumer-key") {
                body.insert(
                    "oauth_consumer_key".into(),
                    serde_json::json!(v.to_string()),
                );
            }
            if let Some(v) = matches.get_one::<types::OauthNonce>("oauth-nonce") {
                body.insert("oauth_nonce".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::OauthSignature>("oauth-signature") {
                body.insert("oauth_signature".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) =
                matches.get_one::<types::OauthSignatureMethod>("oauth-signature-method")
            {
                body.insert(
                    "oauth_signature_method".into(),
                    serde_json::json!(v.to_string()),
                );
            }
            if let Some(v) = matches.get_one::<types::OauthTimestamp>("oauth-timestamp") {
                body.insert("oauth_timestamp".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::OauthToken>("oauth-token") {
                body.insert("oauth_token".into(), serde_json::json!(v.to_string()));
            }
            if let Some(v) = matches.get_one::<types::OauthVersion>("oauth-version") {
                body.insert("oauth_version".into(), serde_json::json!(v.to_string()));
            }
            serde_json::from_value::<types::PostOauthRequestTokenBody>(serde_json::Value::Object(
                body,
            ))?
        };
        let result = aweber::endpoints::oauth_get_request_token(&self.client, &body).await;
        self.print_result(result)
    }

    pub(crate) async fn execute_oauth_revoke(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let authorization = matches
            .get_one::<String>("authorization")
            .map(|s| s.as_str());
        let path = matches.get_one::<std::path::PathBuf>("json-body").unwrap();
        let txt = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let body = serde_json::from_str::<types::PostOauth2RevokeBody>(&txt)
            .with_context(|| format!("failed to parse {}", path.display()))?;
        let result = aweber::endpoints::oauth2_revoke(&self.client, authorization, &body).await;
        self.print_void(result)
    }

    pub(crate) async fn execute_oauth_token(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let authorization = matches
            .get_one::<String>("authorization")
            .map(|s| s.as_str());
        let path = matches.get_one::<std::path::PathBuf>("json-body").unwrap();
        let txt = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let body = serde_json::from_str::<types::PostOauth2TokenBody>(&txt)
            .with_context(|| format!("failed to parse {}", path.display()))?;
        let result = aweber::endpoints::oauth2_token(&self.client, authorization, &body).await;
        self.print_result(result)
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn print_result<T: serde::Serialize>(
        &self,
        result: Result<T, aweber::client::ApiError>,
    ) -> anyhow::Result<()> {
        use std::io::IsTerminal;
        match result {
            Ok(value) => {
                if std::io::stdout().is_terminal() {
                    let json = serde_json::to_value(&value)?;
                    println!("{}", colored_json::to_colored_json_auto(&json)?);
                } else {
                    println!("{}", serde_json::to_string_pretty(&value)?);
                }
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("{e}")),
        }
    }

    pub(crate) fn print_ndjson<T: serde::Serialize>(
        &self,
        items: &[T],
        limit: &mut Limit,
    ) -> anyhow::Result<()> {
        use std::io::{IsTerminal, Write};
        let pretty = std::io::stdout().is_terminal();
        let stdout = std::io::stdout();
        let mut out = std::io::BufWriter::new(stdout.lock());
        let mut written = 0usize;
        for item in items {
            if !limit.consume() {
                break;
            }
            if pretty {
                let json = serde_json::to_value(item)?;
                writeln!(out, "{}", colored_json::to_colored_json_auto(&json)?)?;
            } else {
                serde_json::to_writer(&mut out, item)?;
                writeln!(out)?;
            }
            written += 1;
        }
        out.flush()?;
        limit.warn_truncated(written < items.len());
        Ok(())
    }

    async fn print_paginated_ndjson<C>(
        &self,
        first_page: Result<C, aweber::client::ApiError>,
        limit: &mut Limit,
    ) -> anyhow::Result<()>
    where
        C: types::PaginatedCollection + serde::de::DeserializeOwned,
        C::Item: serde::Serialize,
    {
        use std::io::{IsTerminal, Write};
        let pretty = std::io::stdout().is_terminal();
        let mut page = match first_page {
            Ok(p) => p,
            Err(e) => return Err(anyhow::anyhow!("{e}")),
        };
        let stdout = std::io::stdout();
        let mut out = std::io::BufWriter::new(stdout.lock());
        loop {
            let entries = page.take_entries();
            let total = entries.len();
            let mut written = 0usize;
            for item in entries {
                if !limit.consume() {
                    break;
                }
                if pretty {
                    let json = serde_json::to_value(&item)?;
                    writeln!(out, "{}", colored_json::to_colored_json_auto(&json)?)?;
                } else {
                    serde_json::to_writer(&mut out, &item)?;
                    writeln!(out)?;
                }
                written += 1;
            }
            out.flush()?;
            if written < total {
                limit.warn_truncated(true);
                return Ok(());
            }
            match page.next_collection_link() {
                Some(_) if limit.is_exhausted() => {
                    limit.warn_truncated(true);
                    break;
                }
                Some(url) => {
                    let url = url.to_string();
                    page = self
                        .client
                        .get_url(&url)
                        .await
                        .map_err(|e| anyhow::anyhow!("fetching next page: {e}"))?;
                }
                None => break,
            }
        }
        Ok(())
    }

    fn print_void(&self, result: Result<(), aweber::client::ApiError>) -> anyhow::Result<()> {
        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("{e}")),
        }
    }
}

pub(crate) struct Limit {
    remaining: Option<usize>,
    suppress_warning: bool,
}

impl Limit {
    pub(crate) fn from_matches(matches: &clap::ArgMatches) -> Limit {
        let remaining = matches
            .try_get_one::<usize>("limit")
            .ok()
            .flatten()
            .copied();
        Limit {
            suppress_warning: remaining == Some(0),
            remaining,
        }
    }

    pub(crate) fn remaining(&self) -> Option<usize> {
        self.remaining
    }

    pub(crate) fn is_exhausted(&self) -> bool {
        self.remaining == Some(0)
    }

    pub(crate) fn consume(&mut self) -> bool {
        match &mut self.remaining {
            Some(0) => false,
            Some(remaining) => {
                *remaining -= 1;
                true
            }
            None => true,
        }
    }

    pub(crate) fn warn_truncated(&self, more_available: bool) {
        if more_available && !self.suppress_warning && self.is_exhausted() {
            eprintln!("Warning: results truncated by --limit; more results available");
        }
    }
}

/// What `aweber <group> <action>` resolves to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum CliCommand {
    /// One request of the catalog, run from its clap arguments.
    Operation(aweber::catalog::Operation),
    /// A `workflows` command the binary defines over several requests.
    Workflow(WorkflowCommand),
}

/// The `aweber workflows` actions, each reading and editing a workflow through
/// `aweber::workflows` rather than issuing one catalog request.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorkflowCommand {
    List,
    Show,
    Create,
    Update,
    AddStep,
    UpdateStep,
    Publish,
    Delete,
}

impl WorkflowCommand {
    #[cfg(test)]
    pub(crate) const ALL: [WorkflowCommand; 8] = [
        WorkflowCommand::List,
        WorkflowCommand::Show,
        WorkflowCommand::Create,
        WorkflowCommand::Update,
        WorkflowCommand::AddStep,
        WorkflowCommand::UpdateStep,
        WorkflowCommand::Publish,
        WorkflowCommand::Delete,
    ];

    /// The clap command that defines the action's arguments.
    pub(crate) fn command(self) -> clap::Command {
        match self {
            WorkflowCommand::List => Cli::cli_list_workflows(),
            WorkflowCommand::Show => Cli::cli_show_workflow(),
            WorkflowCommand::Create => Cli::cli_create_workflow(),
            WorkflowCommand::Update => Cli::cli_update_workflow(),
            WorkflowCommand::AddStep => Cli::cli_add_workflow_step(),
            WorkflowCommand::UpdateStep => Cli::cli_update_workflow_step(),
            WorkflowCommand::Publish => Cli::cli_publish_workflow(),
            WorkflowCommand::Delete => Cli::cli_delete_workflow(),
        }
    }
}

use crate::catalog::{Group, Operation};
use crate::types;

/// `aweber api <path> -X <method> --input <file> -H <key:value> --json`
pub fn raw_command() -> clap::Command {
    clap::Command::new("api")
        .about("Make an authenticated API request")
        .arg(
            clap::Arg::new("path")
                .required(true)
                .help("API path (e.g., /1.0/accounts/12345/lists)"),
        )
        .arg(
            clap::Arg::new("method")
                .short('X')
                .long("method")
                .default_value("GET")
                .help("HTTP method"),
        )
        .arg(
            clap::Arg::new("input")
                .long("input")
                .help("Request body file (use - for stdin)"),
        )
        .arg(
            clap::Arg::new("header")
                .short('H')
                .long("header")
                .action(clap::ArgAction::Append)
                .help("Extra header (key:value, repeatable)"),
        )
        .arg(
            clap::Arg::new("json")
                .long("json")
                .action(clap::ArgAction::SetTrue)
                .help("Output JSON object with status, headers, and body"),
        )
}

/// `aweber` with the routed groups and `api`, without the global flags,
/// `auth`, `tui`, or `workflows`, which the binary adds.
pub fn command_tree() -> clap::Command {
    let mut app = clap::Command::new("aweber-cli")
        .bin_name("aweber")
        .about("AWeber API CLI")
        .subcommand_required(true)
        .subcommand(raw_command());
    for group in crate::catalog::groups() {
        if group.hidden || !group.cli {
            continue;
        }
        app = app.subcommand(group_command(group));
    }
    app
}

/// One routed group as a clap subcommand, its actions in route-table order.
pub fn group_command(group: &Group) -> clap::Command {
    let mut group_cmd = clap::Command::new(group.name)
        .about(group.about)
        .subcommand_required(true);
    if let Some(long_about) = group.long_about {
        group_cmd = group_cmd.long_about(long_about);
    }
    for operation in group.operations {
        group_cmd = group_cmd.subcommand(command(*operation).name(operation.action()));
    }
    group_cmd
}

/// The clap command that defines an operation's arguments.
pub(super) fn command(operation: Operation) -> clap::Command {
    match operation {
        Operation::ListAccounts => cli_list_accounts(),
        Operation::GetAccount => cli_get_account(),
        Operation::FindAccountSubscribers => cli_find_account_subscribers(),
        Operation::ListAccountWebformSplitTests => cli_list_account_webform_split_tests(),
        Operation::ListAccountWebforms => cli_list_account_webforms(),
        Operation::ListIntegrations => cli_list_integrations(),
        Operation::GetIntegration => cli_get_integration(),
        Operation::ListLists => cli_list_lists(),
        Operation::FindLists => cli_find_lists(),
        Operation::GetList => cli_get_list(),
        Operation::ListBroadcasts => cli_list_broadcasts(),
        Operation::CreateBroadcast => cli_create_broadcast(),
        Operation::GetBroadcastTotal => cli_get_broadcast_total(),
        Operation::GetBroadcast => cli_get_broadcast(),
        Operation::UpdateBroadcast => cli_update_broadcast(),
        Operation::DeleteBroadcast => cli_delete_broadcast(),
        Operation::CancelBroadcast => cli_cancel_broadcast(),
        Operation::GetBroadcastClicks => cli_get_broadcast_clicks(),
        Operation::GetBroadcastOpens => cli_get_broadcast_opens(),
        Operation::ScheduleBroadcast => cli_schedule_broadcast(),
        Operation::WaitBroadcast => cli_wait_broadcast(),
        Operation::ListCampaigns => cli_list_campaigns(),
        Operation::ListCampaignStats => cli_list_campaign_stats(),
        Operation::GetCampaignStat => cli_get_campaign_stat(),
        Operation::FindCampaigns => cli_find_campaigns(),
        Operation::GetCampaign => cli_get_campaign(),
        Operation::ListCustomFields => cli_list_custom_fields(),
        Operation::CreateCustomField => cli_create_custom_field(),
        Operation::GetCustomField => cli_get_custom_field(),
        Operation::DeleteCustomField => cli_delete_custom_field(),
        Operation::UpdateCustomField => cli_update_custom_field(),
        Operation::ListLandingPages => cli_list_landing_pages(),
        Operation::GetLandingPage => cli_get_landing_page(),
        Operation::CreatePurchase => cli_create_purchase(),
        Operation::ListSegments => cli_list_segments(),
        Operation::GetSegment => cli_get_segment(),
        Operation::ListSubscribers => cli_list_subscribers(),
        Operation::CreateSubscriber => cli_create_subscriber(),
        Operation::DeleteSubscriberByEmail => cli_delete_subscriber_by_email(),
        Operation::UpdateSubscriberByEmail => cli_update_subscriber_by_email(),
        Operation::FindSubscribers => cli_find_subscribers(),
        Operation::GetSubscriber => cli_get_subscriber(),
        Operation::MoveSubscriber => cli_move_subscriber(),
        Operation::DeleteSubscriber => cli_delete_subscriber(),
        Operation::UpdateSubscriber => cli_update_subscriber(),
        Operation::GetSubscriberActivity => cli_get_subscriber_activity(),
        Operation::ListTags => cli_list_tags(),
        Operation::ListWebFormSplitTests => cli_list_web_form_split_tests(),
        Operation::GetWebFormSplitTest => cli_get_web_form_split_test(),
        Operation::ListWebFormSplitTestComponents => cli_list_web_form_split_test_components(),
        Operation::GetWebFormSplitTestComponent => cli_get_web_form_split_test_component(),
        Operation::ListWebForms => cli_list_web_forms(),
        Operation::GetWebForm => cli_get_web_form(),
        Operation::GetBroadcastLinkAnalytics => cli_get_broadcast_link_analytics(),
        Operation::UnsubscribeSubscriber => cli_unsubscribe_subscriber(),
        Operation::ListWorkflows => cli_list_workflows(),
        Operation::GetWorkflow => cli_get_workflow(),
        Operation::TreeWorkflow => cli_tree_workflow(),
        Operation::CreateWorkflow => cli_create_workflow(),
        Operation::UpdateWorkflow => cli_update_workflow(),
        Operation::UpdateWorkflowRuleset => cli_update_workflow_ruleset(),
        Operation::PublishWorkflow => cli_publish_workflow(),
        Operation::RevertWorkflow => cli_revert_workflow(),
        Operation::SetWorkflowState => cli_set_workflow_state(),
        Operation::CopyWorkflow => cli_copy_workflow(),
        Operation::DeleteWorkflow => cli_delete_workflow(),
        Operation::GetWorkflowStats => cli_get_workflow_stats(),
        Operation::GetWorkflowMessageStats => cli_get_workflow_message_stats(),
        Operation::GetWorkflowEventHistory => cli_get_workflow_event_history(),
        Operation::OauthGetAccessToken => cli_oauth_get_access_token(),
        Operation::OauthGetRequestToken => cli_oauth_get_request_token(),
        Operation::OauthRevoke => cli_oauth_revoke(),
        Operation::OauthToken => cli_oauth_token(),
    }
}

fn limit_arg() -> clap::Arg {
    clap::Arg::new("limit")
        .long("limit")
        .value_parser(clap::value_parser!(usize))
        .help("Maximum total number of entries to output")
}

fn list_id_args() -> [clap::Arg; 2] {
    [
        clap::Arg::new("list-id")
            .long("list-id")
            .value_parser(clap::value_parser!(i32))
            .help("The list ID"),
        clap::Arg::new("list")
            .long("list")
            .value_parser(clap::value_parser!(String))
            .help("The list name or unique list ID (looked up via the API)"),
    ]
}

fn list_id_group() -> clap::ArgGroup {
    clap::ArgGroup::new("list-identifier")
        .args(["list-id", "list"])
        .required(true)
}

fn subscriber_id_args() -> [clap::Arg; 2] {
    [
        clap::Arg::new("subscriber-id")
            .long("subscriber-id")
            .value_parser(clap::value_parser!(i32))
            .help("The subscriber ID"),
        clap::Arg::new("email")
            .long("email")
            .value_parser(clap::value_parser!(String))
            .help("The subscriber's email address (looked up via the API)"),
    ]
}

fn subscriber_id_group() -> clap::ArgGroup {
    clap::ArgGroup::new("subscriber-identifier")
        .args(["subscriber-id", "email"])
        .required(true)
}

fn custom_field_id_args() -> [clap::Arg; 2] {
    [
        clap::Arg::new("custom-field-id")
            .long("custom-field-id")
            .value_parser(clap::value_parser!(i32))
            .help("The custom field ID"),
        clap::Arg::new("custom-field")
            .long("custom-field")
            .value_parser(clap::value_parser!(String))
            .help("The custom field name (looked up via the API)"),
    ]
}

fn custom_field_id_group() -> clap::ArgGroup {
    clap::ArgGroup::new("custom-field-identifier")
        .args(["custom-field-id", "custom-field"])
        .required(true)
}

fn cli_list_accounts() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get accounts")
}
fn cli_get_account() -> clap::Command {
    clap::Command::new("").about("Get account")
}
fn cli_find_account_subscribers() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: GetAccountsFindsubscribersAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("area-code") . long ("area-code") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The subscriber's area code"))
            .arg (clap::Arg::new ("city") . long ("city") . value_parser (clap::value_parser! (types :: GetAccountsFindsubscribersCity)) . required (false) . help ("The subscriber's city"))
            .arg (clap::Arg::new ("country") . long ("country") . value_parser (clap::value_parser! (types :: GetAccountsFindsubscribersCountry)) . required (false) . help ("The subscriber's country"))
            .arg (clap::Arg::new ("custom-fields") . long ("custom-fields") . value_parser (clap::value_parser! (String)) . required (false) . help ("The JSON encoded custom field key value pairs"))
            .arg (clap::Arg::new ("dma-code") . long ("dma-code") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The subscriber's designated market area code (usa and canada only)"))
            .arg (clap::Arg::new ("email") . long ("email") . value_parser (clap::value_parser! (types :: GetAccountsFindsubscribersEmail)) . required (false) . help ("The subscriber's email address"))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber"))
            .arg (clap::Arg::new ("last-followup-message-sent-at") . long ("last-followup-message-sent-at") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day when the last followup message was sent to the subscriber"))
            .arg (clap::Arg::new ("latitude") . long ("latitude") . value_parser (clap::value_parser! (f64)) . required (false) . help ("The subscriber's geographical latitude"))
            .arg (clap::Arg::new ("longitude") . long ("longitude") . value_parser (clap::value_parser! (f64)) . required (false) . help ("The subscriber's geographical longitude"))
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (types :: GetAccountsFindsubscribersMiscNotes)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: GetAccountsFindsubscribersName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("postal-code") . long ("postal-code") . value_parser (clap::value_parser! (types :: GetAccountsFindsubscribersPostalCode)) . required (false) . help ("The subscriber's postal or zip code"))
            .arg (clap::Arg::new ("region") . long ("region") . value_parser (clap::value_parser! (types :: GetAccountsFindsubscribersRegion)) . required (false) . help ("The subscriber's state or region abbreviation"))
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsFindsubscribersStatus :: Subscribed . to_string () , types :: GetAccountsFindsubscribersStatus :: Unsubscribed . to_string () , types :: GetAccountsFindsubscribersStatus :: Unconfirmed . to_string () ,]) , | s | types :: GetAccountsFindsubscribersStatus :: try_from (s) . unwrap ())) . required (false) . help ("The subscriber's status"))
            .arg (clap::Arg::new ("subscribed-after") . long ("subscribed-after") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day on or after the subscriber subscribed"))
            .arg (clap::Arg::new ("subscribed-at") . long ("subscribed-at") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day in which the subscriber subscribed"))
            .arg (clap::Arg::new ("subscribed-before") . long ("subscribed-before") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day on or before the subscriber subscribed"))
            .arg (clap::Arg::new ("subscription-method") . long ("subscription-method") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsFindsubscribersSubscriptionMethod :: Api . to_string () , types :: GetAccountsFindsubscribersSubscriptionMethod :: Email . to_string () , types :: GetAccountsFindsubscribersSubscriptionMethod :: Import . to_string () , types :: GetAccountsFindsubscribersSubscriptionMethod :: Webform . to_string () ,]) , | s | types :: GetAccountsFindsubscribersSubscriptionMethod :: try_from (s) . unwrap ())) . required (false) . help ("How the subscriber was subscribed"))
            .arg (clap::Arg::new ("tags") . long ("tags") . value_parser (clap::value_parser! (String)) . required (false) . help ("A tag to match. All tags must match for the subscriber to match."))
            .arg (clap::Arg::new ("tags-not-in") . long ("tags-not-in") . value_parser (clap::value_parser! (String)) . required (false) . help ("A tag to exclude. Checks that all tags are not matched to a subscriber."))
            .arg (clap::Arg::new ("unsubscribe-method") . long ("unsubscribe-method") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsFindsubscribersUnsubscribeMethod :: UnsubscribeLink . to_string () , types :: GetAccountsFindsubscribersUnsubscribeMethod :: CustomerCp . to_string () , types :: GetAccountsFindsubscribersUnsubscribeMethod :: Undeliverable . to_string () , types :: GetAccountsFindsubscribersUnsubscribeMethod :: ApiUnsubscribe . to_string () , types :: GetAccountsFindsubscribersUnsubscribeMethod :: ApiMove . to_string () ,]) , | s | types :: GetAccountsFindsubscribersUnsubscribeMethod :: try_from (s) . unwrap ())) . required (false) . help ("How the subscriber unsubscribed"))
            .arg (clap::Arg::new ("unsubscribed-after") . long ("unsubscribed-after") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day on or after the subscriber unsubscribed"))
            .arg (clap::Arg::new ("unsubscribed-at") . long ("unsubscribed-at") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day in which the subscriber unsubscribed"))
            .arg (clap::Arg::new ("unsubscribed-before") . long ("unsubscribed-before") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day on or before the subscriber unsubscribed"))
            .arg (clap::Arg::new ("verified-at") . long ("verified-at") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day in which the subscriber confirmed their email address"))
            .arg (clap::Arg::new ("ws-show") . long ("ws-show") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsFindsubscribersWsShow :: TotalSize . to_string () ,]) , | s | types :: GetAccountsFindsubscribersWsShow :: try_from (s) . unwrap ())) . required (false) . help ("A flag to show the total size only - expecting \\\"total_size\\\", when added the response will be an integer"))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(limit_arg())
            .about ("Find subscribers for account")
}
fn cli_list_account_webform_split_tests() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get split tests for account")
}
fn cli_list_account_webforms() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get webforms for account")
}
fn cli_list_integrations() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get integrations")
}
fn cli_get_integration() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("integration-id")
                .long("integration-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The integration ID"),
        )
        .about("Get integration")
}
fn cli_list_lists() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get lists")
}
fn cli_find_lists() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: GetAccountsListsFindName)) . required (false) . help ("Name or unique list ID of the list"))
            .arg (clap::Arg::new ("ws-show") . long ("ws-show") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsFindWsShow :: TotalSize . to_string () ,]) , | s | types :: GetAccountsListsFindWsShow :: try_from (s) . unwrap ())) . required (false) . help ("A flag to show the total size only - expecting \\\"total_size\\\", when added the response will be an integer"))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .about ("Find lists")
}
fn cli_get_list() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .about("Get list")
}
fn cli_list_broadcasts() -> clap::Command {
    clap::Command::new ("")
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsBroadcastsStatus :: Draft . to_string () , types :: GetAccountsListsBroadcastsStatus :: Scheduled . to_string () , types :: GetAccountsListsBroadcastsStatus :: Sent . to_string () ,]) , | s | types :: GetAccountsListsBroadcastsStatus :: try_from (s) . unwrap ())) . required (false) . help ("Filter by status (draft, scheduled, sent). If omitted, all statuses are retrieved. **(Please be aware that `draft` only returns API created Broadcast drafts)**"))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(limit_arg())
            .about ("Get broadcasts")
}
fn cli_create_broadcast() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("body-amp") . long ("body-amp") . value_parser (clap::value_parser! (String)) . required (false) . help ("<b>[Please read <a href=\"https://help.aweber.com/hc/en-us/articles/360025741194\" target=\"_blank\">our KB article before using this field.]</b>The content of the message in AMP format."))
            .arg (clap::Arg::new ("body-html") . long ("body-html") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The content of the message in html format. If body_text is not provided, it will be auto-generated. If body_text is not provided, body_html must be provided."))
            .arg (clap::Arg::new ("body-text") . long ("body-text") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The content of the message in plain text, used when HTML is not supported. If body_html is not provided, the broadcast will be sent using only the body_text. If body_text is not provided, body_html must be provided."))
            .arg (clap::Arg::new ("click-tracking-enabled") . long ("click-tracking-enabled") . value_parser (clap::value_parser! (bool)) . required (false) . help ("Enables links in the email message to be tracked"))
            .arg (clap::Arg::new ("exclude-lists") . long ("exclude-lists") . value_parser (clap::value_parser! (String)) . required (false) . help ("JSON encoded list of [Lists](#tag/Lists) URLs to exclude in the delivery of this broadcast. Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing excluded_lists."))
            .arg (clap::Arg::new ("facebook-integration") . long ("facebook-integration") . value_parser (clap::value_parser! (String)) . required (false) . help ("URL to the [Facebook broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be posted to this Facebook integration  - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"))
            .arg (clap::Arg::new ("include-lists") . long ("include-lists") . value_parser (clap::value_parser! (String)) . required (false) . help ("JSON encoded list of [Lists](#tag/Lists) URLs to include in the delivery of this broadcast. Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing included_lists."))
            .arg (clap::Arg::new ("is-archived") . long ("is-archived") . value_parser (clap::value_parser! (bool)) . required (false) . help ("Whether the broadcast enabled sharing via an archive url"))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("notify-on-send") . long ("notify-on-send") . value_parser (clap::value_parser! (bool)) . required (false) . help ("If true, notify when stats are available on a sent broadcast message"))
            .arg (clap::Arg::new ("subject") . long ("subject") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The broadcast subject line. Subject must not be empty nor contain only whitespace."))
            .arg (clap::Arg::new ("twitter-integration") . long ("twitter-integration") . value_parser (clap::value_parser! (String)) . required (false) . help ("URL to the [Twitter broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be tweeted - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Create broadcast")
}
fn cli_get_broadcast_total() -> clap::Command {
    clap::Command::new ("")
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsBroadcastsTotalStatus :: Draft . to_string () , types :: GetAccountsListsBroadcastsTotalStatus :: Scheduled . to_string () , types :: GetAccountsListsBroadcastsTotalStatus :: Sent . to_string () ,]) , | s | types :: GetAccountsListsBroadcastsTotalStatus :: try_from (s) . unwrap ())) . required (true) . help ("The status of the broadcasts to retrieve. **(Please be aware that `draft` only returns API created Broadcast drafts)**"))
            .about ("Get total broadcasts")
}
fn cli_get_broadcast() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("broadcast-id")
                .long("broadcast-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The broadcast ID"),
        )
        .args(list_id_args())
        .group(list_id_group())
        .about("Get broadcast")
}
fn cli_update_broadcast() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("body-amp") . long ("body-amp") . value_parser (clap::value_parser! (String)) . required (false) . help ("<b>[Please read <a href=\"https://help.aweber.com/hc/en-us/articles/360025741194\" target=\"_blank\">our KB article before using this field.]</b>The content of the message in AMP format."))
            .arg (clap::Arg::new ("body-html") . long ("body-html") . value_parser (clap::value_parser! (String)) . required (false) . help ("The content of the message in html format. If body_text is not provided, it will be auto-generated."))
            .arg (clap::Arg::new ("body-text") . long ("body-text") . value_parser (clap::value_parser! (String)) . required (false) . help ("The content of the message in plain text, used when HTML is not supported. If body_html is not provided, the broadcast will be sent using only the body_text."))
            .arg (clap::Arg::new ("broadcast-id") . long ("broadcast-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The broadcast ID"))
            .arg (clap::Arg::new ("click-tracking-enabled") . long ("click-tracking-enabled") . value_parser (clap::value_parser! (bool)) . required (false) . help ("Enables links in the email message to be tracked."))
            .arg (clap::Arg::new ("exclude-lists") . long ("exclude-lists") . value_parser (clap::value_parser! (String)) . required (false) . help ("JSON encoded list of [Lists](#tag/Lists) URLs to exclude in the delivery of this broadcast. Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing excluded_lists."))
            .arg (clap::Arg::new ("facebook-integration") . long ("facebook-integration") . value_parser (clap::value_parser! (String)) . required (false) . help ("URL to the [Facebook broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be posted to this Facebook integration  - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"))
            .arg (clap::Arg::new ("include-lists") . long ("include-lists") . value_parser (clap::value_parser! (String)) . required (false) . help ("JSON encoded list of [Lists](#tag/Lists) URLs to include in the delivery of this broadcast. Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing included_lists."))
            .arg (clap::Arg::new ("is-archived") . long ("is-archived") . value_parser (clap::value_parser! (bool)) . required (false) . help ("Whether the broadcast enabled sharing via an archive url."))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("notify-on-send") . long ("notify-on-send") . value_parser (clap::value_parser! (bool)) . required (false) . help ("If true, notify when stats are available on a sent broadcast message, defaults to true."))
            .arg (clap::Arg::new ("segment-link") . long ("segment-link") . value_parser (clap::value_parser! (String)) . required (false) . help ("URL to the [Segment](#tag/Segments) to send this broadcast to.  Use the `self_link` of the segment here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>/segments/<segment_id>`. If not specified, the broadcast will be sent to the \"Active Subscribers\" segment."))
            .arg (clap::Arg::new ("subject") . long ("subject") . value_parser (clap::value_parser! (String)) . required (false) . help ("The broadcast subject line. Subject must not be empty nor contain only whitespace."))
            .arg (clap::Arg::new ("twitter-integration") . long ("twitter-integration") . value_parser (clap::value_parser! (String)) . required (false) . help ("URL to the [Twitter broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be tweeted - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Update broadcast")
}
fn cli_delete_broadcast() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("broadcast-id")
                .long("broadcast-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The broadcast ID"),
        )
        .args(list_id_args())
        .group(list_id_group())
        .about("Delete broadcast")
}
fn cli_cancel_broadcast() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("broadcast-id")
                .long("broadcast-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The broadcast ID"),
        )
        .args(list_id_args())
        .group(list_id_group())
        .about("Cancel scheduled broadcast")
}
fn cli_get_broadcast_clicks() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("after") . long ("after") . value_parser (clap::value_parser! (String)) . required (false) . help ("The pagination key when paging forward. Cannot be combined with `before`."))
            .arg (clap::Arg::new ("before") . long ("before") . value_parser (clap::value_parser! (String)) . required (false) . help ("The pagination key when paging in reverse. Cannot be combined with `after`."))
            .arg (clap::Arg::new ("broadcast-id") . long ("broadcast-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The broadcast ID"))
            .arg (clap::Arg::new ("detailed") . long ("detailed") . value_parser (clap::value_parser! (bool)) . required (false) . help ("When true, returns individual click events with URLs instead of aggregated click data per subscriber"))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("page-size") . long ("page-size") . value_parser (clap::value_parser! (std::num::NonZeroU64)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg(limit_arg())
            .about ("Get broadcast clicks")
}
fn cli_get_broadcast_opens() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("after")
                .long("after")
                .value_parser(clap::value_parser!(String))
                .required(false)
                .help("The pagination key when paging forward. Cannot be combined with `before`."),
        )
        .arg(
            clap::Arg::new("before")
                .long("before")
                .value_parser(clap::value_parser!(String))
                .required(false)
                .help(
                    "The pagination key when paging in reverse. Cannot be combined with `after`.",
                ),
        )
        .arg(
            clap::Arg::new("broadcast-id")
                .long("broadcast-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The broadcast ID"),
        )
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("page-size")
                .long("page-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU64))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(limit_arg())
        .about("Get broadcast opens")
}
fn cli_schedule_broadcast() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("broadcast-id")
                .long("broadcast-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The broadcast ID"),
        )
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("scheduled-for")
                .long("scheduled-for")
                .value_parser(clap::value_parser!(chrono::DateTime<chrono::Utc>))
                .required_unless_present("json-body")
                .help("Scheduled time for sending broadcast message, ISO-8601 formatted."),
        )
        .arg(
            clap::Arg::new("json-body")
                .long("json-body")
                .value_name("JSON-FILE")
                .required(false)
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .help("Path to a file that contains the full json body."),
        )
        .about("Schedule broadcast")
}
fn cli_wait_broadcast() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("broadcast-id")
                .long("broadcast-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The broadcast ID"),
        )
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("interval")
                .long("interval")
                .value_parser(clap::value_parser!(u64))
                .default_value("30")
                .help("Polling interval in seconds"),
        )
        .about("Wait for a broadcast to finish sending")
}
fn cli_list_campaigns() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get campaigns")
}
fn cli_list_campaign_stats() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("campaign-id")
                .long("campaign-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The campaign ID"),
        )
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get broadcast statistics")
}
fn cli_get_campaign_stat() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("campaign-id") . long ("campaign-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The campaign ID"))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("stats-id") . long ("stats-id") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalClicks . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: UniqueClicks . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalOpens . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: UniqueOpens . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalSales . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalSalesDollars . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalUnsubscribed . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlyOpens . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlyClicks . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlyWebhits . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlySales . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlyUnsubscribed . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailyOpens . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailyClicks . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailyWebhits . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailySales . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailyUnsubscribed . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: ClicksByLink . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: WebhitsByLink . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: OpensBySubscriber . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: SalesBySubscriber . to_string () ,]) , | s | types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: try_from (s) . unwrap ())) . required (true) . help ("\n\n>The statistic's ID.\n>\n>The datatype of the ID may be different for each Stat.\n>\n>Below is a list of the statistic IDs that can be passed in.\n>\n>__Aggregate Statistics__\n>\n>| Stat ID | Description |\n>|---------|-------------|\n>| total_clicks | Total number of times a subscriber clicked any link appearing in your campaign except the unsubscribe link (includes multiple clicks of the same link) |\n>| unique_clicks | Total number of subscribers who clicked any link in your campaign |\n>| total_opens | Total number of times your campaign was opened by any subscriber your campaign was sent to (including multiple opens by the same subscriber) |\n>| unique_opens | Total number of subscribers who opened your campaign |\n>| total_sales | Total number of sales made by subscribers who received your campaign |\n>| total_sales_dollars | Total monetary value of sales made by subscribers who received your campaign |\n>| total_unsubscribed | Total number of subscribers who unsubscribed by clicking the unsubscribe link in your campaign |\n>\n>__Time Related Statistics__\n>\n>| Stat ID | Description |\n>|---------|-------------|\n>| hourly_clicks | Hourly breakdown of unique and total clicks for the first 24 hours after a campaign was sent |\n>| hourly_opens | Hourly breakdown of unique and total opens for the first 24 hours after a campaign was sent |\n>| hourly_sales | Hourly breakdown of sales for the first 24 hours after a campaign was sent |\n>| hourly_unsubscribed | Hourly breakdown of subscribers who unsubscribed by clicking the unsubscribed link for the first 24 hours after a campaign was sent |\n>| hourly_webhits | Hourly breakdown of webhits to your website from subscribers sent this message for the first 24 hours after a campaign was sent |\n>| daily_clicks | Daily breakdown of unique and total clicks for the first 14 days after a campaign was sent |\n>| daily_opens | Daily breakdown of unique and total opens for the first 14 days after a campaign was sent |\n>| daily_sales | Daily breakdown of sales for the first 14 days after a campaign was sent |\n>| daily_unsubscribed | Daily breakdown of subscribers who unsuscribed by clicking the unsubscribed link for the first 14 days after a campaign was sent |\n>| daily_webhits | Daily breakdown of webhits to your website from subscribers sent this message for the first 14 days after a campaign was sent |\n>\n>__Top 10 URL Statistics__\n>\n>| Stat ID | Description |\n>|---------|-------------|\n>| clicks_by_link | Top 10 links that were clicked (ranked by total_clicked) |\n>| webhits_by_link | Top 10 webhits by click (ranked by total clicks) |\n>\n>__Top 10 Subscriber Statistics__\n>(Requires access to subscriber data)\n>\n>| Stat ID | Description |\n>|---------|-------------|\n>| opens_by_subscriber | Top 10 subscribers that opened your message (ranked by total opens) |\n>| sales_by_subscriber | Top 10 subscribers that made a sale from your message (ranked by total sales dollars) |\n"))
            .about ("Get broadcast statistic")
}
fn cli_find_campaigns() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("campaign-type") . long ("campaign-type") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsCampaignsFindCampaignType :: B . to_string () , types :: GetAccountsListsCampaignsFindCampaignType :: F . to_string () ,]) , | s | types :: GetAccountsListsCampaignsFindCampaignType :: try_from (s) . unwrap ())) . required (true) . help ("The campaign type (b - broadcast, f - followup)"))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("ws-show") . long ("ws-show") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsCampaignsFindWsShow :: TotalSize . to_string () ,]) , | s | types :: GetAccountsListsCampaignsFindWsShow :: try_from (s) . unwrap ())) . required (false) . help ("A flag to show the total size only - expecting \\\"total_size\\\", when added the response will be an integer"))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(limit_arg())
            .about ("Find campaigns")
}
fn cli_get_campaign() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("campaign-id") . long ("campaign-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The campaign ID"))
            .arg (clap::Arg::new ("campaign-type") . long ("campaign-type") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsCampaignsCampaigntypecampaignidCampaignType :: B . to_string () , types :: GetAccountsListsCampaignsCampaigntypecampaignidCampaignType :: F . to_string () ,]) , | s | types :: GetAccountsListsCampaignsCampaigntypecampaignidCampaignType :: try_from (s) . unwrap ())) . required (true) . help ("The campaign type (b - broadcast, f - followup)"))
            .args(list_id_args())
            .group(list_id_group())
            .about ("Get campaign")
}
fn cli_list_custom_fields() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get custom fields")
}
fn cli_create_custom_field() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("name")
                .long("name")
                .value_parser(clap::value_parser!(String))
                .required_unless_present("json-body")
                .help("The name of the custom field"),
        )
        .arg(
            clap::Arg::new("json-body")
                .long("json-body")
                .value_name("JSON-FILE")
                .required(false)
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .help("Path to a file that contains the full json body."),
        )
        .about("Add custom field")
}
fn cli_get_custom_field() -> clap::Command {
    clap::Command::new("")
        .args(custom_field_id_args())
        .group(custom_field_id_group())
        .args(list_id_args())
        .group(list_id_group())
        .about("Get custom field")
}
fn cli_delete_custom_field() -> clap::Command {
    clap::Command::new("")
        .args(custom_field_id_args())
        .group(custom_field_id_group())
        .args(list_id_args())
        .group(list_id_group())
        .about("Delete custom field")
}
fn cli_update_custom_field() -> clap::Command {
    clap::Command::new("")
        .args(custom_field_id_args())
        .group(custom_field_id_group())
        .arg(
            clap::Arg::new("is-subscriber-updateable")
                .long("is-subscriber-updateable")
                .value_parser(clap::value_parser!(bool))
                .required(false)
                .help("Whether the subscriber is allowed to update the custom field"),
        )
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("name")
                .long("name")
                .value_parser(clap::value_parser!(String))
                .required(false)
                .help("The name of the custom field"),
        )
        .arg(
            clap::Arg::new("json-body")
                .long("json-body")
                .value_name("JSON-FILE")
                .required(false)
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .help("Path to a file that contains the full json body."),
        )
        .about("Update custom field")
}
fn cli_list_landing_pages() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            ::clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(::clap::value_parser!(::std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            ::clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(::clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get landing pages")
}
fn cli_get_landing_page() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("landing-page-id")
                .long("landing-page-id")
                .value_parser(clap::value_parser!(::uuid::Uuid))
                .required(true)
                .help("The landing page ID"),
        )
        .args(list_id_args())
        .group(list_id_group())
        .about("Get landing page")
}
fn cli_create_purchase() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: PurchaseAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("currency") . long ("currency") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html)."))
            .arg (clap::Arg::new ("email") . long ("email") . value_parser (clap::value_parser! (types :: PurchaseEmail)) . required_unless_present ("json-body") . help ("The subscriber's email address"))
            .arg (clap::Arg::new ("event-note") . long ("event-note") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("A custom note associated with this specific tracked event"))
            .arg (clap::Arg::new ("event-time") . long ("event-time") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The timestamp of when the event occurred"))
            .arg (clap::Arg::new ("ip-address") . long ("ip-address") . value_parser (clap::value_parser! (types :: PurchaseIpAddress)) . required_unless_present ("json-body") . help ("The subscriber's IP address. This must be a public IP address."))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (types :: PurchaseMiscNotes)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: PurchaseName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("product-name") . long ("product-name") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("A custom description for the page or event"))
            .arg (clap::Arg::new ("url") . long ("url") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The URL for the tracked event"))
            .arg (clap::Arg::new ("value") . long ("value") . value_parser (clap::value_parser! (f64)) . required_unless_present ("json-body"))
            .arg (clap::Arg::new ("vendor") . long ("vendor") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The sales tracking url profile for the web page"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Create a purchase")
}
fn cli_list_segments() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            ::clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(::clap::value_parser!(::std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            ::clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(::clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get segments")
}
fn cli_get_segment() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            ::clap::Arg::new("segment-id")
                .long("segment-id")
                .value_parser(::clap::value_parser!(i32))
                .required(true)
                .help("The segment ID"),
        )
        .about("Get segment")
}
fn cli_list_subscribers() -> clap::Command {
    clap::Command::new ("")
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("sort-order") . long ("sort-order") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsSubscribersSortOrder :: Asc . to_string () , types :: GetAccountsListsSubscribersSortOrder :: Desc . to_string () ,]) , | s | types :: GetAccountsListsSubscribersSortOrder :: try_from (s) . unwrap ())) . required (false) . help ("The collection will be sorted by the order in which the subscribers were added to the list. To specify the order, use the value `asc` for ascending or `desc` for descending."))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(limit_arg())
            .about ("Get subscribers")
}
fn cli_create_subscriber() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("email") . long ("email") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyEmail)) . required_unless_present ("json-body") . help ("The subscriber's email address"))
            .arg (clap::Arg::new ("ip-address") . long ("ip-address") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyIpAddress)) . required (false) . help ("The subscriber's IP address. This field is used to determine the following Geo Location fields: area_code, city, country, dma_code, latitude, longitude, postal_code, and region. IP address can only be specified when Subscribers are initially created. Internal, private, or reserved IP addresses are not acceptable."))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i64)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0 (default), the Subscriber should receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyMiscNotes)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("strict-custom-fields") . long ("strict-custom-fields") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: AddSubscriberRequestBodyStrictCustomFields :: True . to_string () , types :: AddSubscriberRequestBodyStrictCustomFields :: False . to_string () ,]) , | s | types :: AddSubscriberRequestBodyStrictCustomFields :: try_from (s) . unwrap ())) . required (false) . help ("If this parameter is present and set to `true`, then custom field names are matched case sensitively.  Enabling this option also causes the operation to fail if a custom field is included that is not defined for the list."))
            .arg (clap::Arg::new ("update-existing") . long ("update-existing") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: AddSubscriberRequestBodyUpdateExisting :: True . to_string () , types :: AddSubscriberRequestBodyUpdateExisting :: False . to_string () ,]) , | s | types :: AddSubscriberRequestBodyUpdateExisting :: try_from (s) . unwrap ())) . required (false) . help ("If this parameter is present and set to `true`, then if a subscriber is already present on the list, the subscriber will be updated.  **Note:** \n- Only the fields defined in the <a href='#tag/Subscribers/paths/~1accounts~1{accountId}~1lists~1{listId}~1subscribers~1{subscriberId}/patch'>patch endpoint will be updated.\n- Any tags in the request will be **appended** to the existing Subscriber."))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Add subscriber")
}
fn cli_delete_subscriber_by_email() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("email")
                .long("email")
                .value_parser(clap::value_parser!(
                    types::DeleteAccountsListsSubscribersSubscriberEmail
                ))
                .required(true)
                .help("The subscriber's email address"),
        )
        .about("Delete subscriber by email")
}
fn cli_update_subscriber_by_email() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: UpdateSubscriberRequestBodyAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("custom-field") . long ("custom-field") . value_parser (clap::value_parser! (String)) . required (false) . action (clap::ArgAction::Append) . value_name ("KEY[=VALUE]") . help ("Set a custom field (KEY=VALUE, KEY= for empty string, KEY for null)"))
            .arg (clap::Arg::new ("new-email") . long ("new-email") . value_parser (clap::value_parser! (String)) . required (false) . help ("Set the subscriber's email address"))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i64)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0, the Subscriber will receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (String)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: UpdateSubscriberRequestBodyName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: UpdateSubscriberRequestBodyStatus :: Subscribed . to_string () , types :: UpdateSubscriberRequestBodyStatus :: Unsubscribed . to_string () ,]) , | s | types :: UpdateSubscriberRequestBodyStatus :: try_from (s) . unwrap ())) . required (false) . help ("The subscriber's status. **Note** you cannot set a subscriber's status to \"unconfirmed\"."))
            .arg (clap::Arg::new ("strict-custom-fields") . long ("strict-custom-fields") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: UpdateSubscriberRequestBodyStrictCustomFields :: True . to_string () , types :: UpdateSubscriberRequestBodyStrictCustomFields :: False . to_string () ,]) , | s | types :: UpdateSubscriberRequestBodyStrictCustomFields :: try_from (s) . unwrap ())) . required (false) . help ("If this parameter is present and set to `true`, then custom field names are matched case sensitively.  Enabling this option also causes the operation to fail if a custom field is included that is not defined for the list."))
            .arg (clap::Arg::new ("email") . long ("email") . value_parser (clap::value_parser! (types :: PatchAccountsListsSubscribersSubscriberEmail)) . required (true) . help ("The subscriber's email address"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Update subscriber by email")
}
fn cli_find_subscribers() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: GetAccountsListsSubscribersFindAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("area-code") . long ("area-code") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The subscriber's area code"))
            .arg (clap::Arg::new ("city") . long ("city") . value_parser (clap::value_parser! (types :: GetAccountsListsSubscribersFindCity)) . required (false) . help ("The subscriber's city"))
            .arg (clap::Arg::new ("country") . long ("country") . value_parser (clap::value_parser! (types :: GetAccountsListsSubscribersFindCountry)) . required (false) . help ("The subscriber's country"))
            .arg (clap::Arg::new ("custom-fields") . long ("custom-fields") . value_parser (clap::value_parser! (String)) . required (false) . help ("The JSON encoded custom field key value pairs"))
            .arg (clap::Arg::new ("dma-code") . long ("dma-code") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The subscriber's designated market area code (usa and canada only)"))
            .arg (clap::Arg::new ("email") . long ("email") . value_parser (clap::value_parser! (types :: GetAccountsListsSubscribersFindEmail)) . required (false) . help ("The subscriber's email address"))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber"))
            .arg (clap::Arg::new ("last-followup-message-sent-at") . long ("last-followup-message-sent-at") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day when the last followup message was sent to the subscriber"))
            .arg (clap::Arg::new ("latitude") . long ("latitude") . value_parser (clap::value_parser! (f64)) . required (false) . help ("The subscriber's geographical latitude"))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("longitude") . long ("longitude") . value_parser (clap::value_parser! (f64)) . required (false) . help ("The subscriber's geographical longitude"))
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (types :: GetAccountsListsSubscribersFindMiscNotes)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: GetAccountsListsSubscribersFindName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("postal-code") . long ("postal-code") . value_parser (clap::value_parser! (types :: GetAccountsListsSubscribersFindPostalCode)) . required (false) . help ("The subscriber's postal or zip code"))
            .arg (clap::Arg::new ("region") . long ("region") . value_parser (clap::value_parser! (types :: GetAccountsListsSubscribersFindRegion)) . required (false) . help ("The subscriber's state or region abbreviation"))
            .arg (clap::Arg::new ("sort-key") . long ("sort-key") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsSubscribersFindSortKey :: SubscribedAt . to_string () , types :: GetAccountsListsSubscribersFindSortKey :: UnsubscribedAt . to_string () ,]) , | s | types :: GetAccountsListsSubscribersFindSortKey :: try_from (s) . unwrap ())) . required (false) . help ("The collection will be sorted by the field key specified. If no key is spcified the search will default to the order in which the subscribers were added to the list."))
            .arg (clap::Arg::new ("sort-order") . long ("sort-order") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsSubscribersFindSortOrder :: Asc . to_string () , types :: GetAccountsListsSubscribersFindSortOrder :: Desc . to_string () ,]) , | s | types :: GetAccountsListsSubscribersFindSortOrder :: try_from (s) . unwrap ())) . required (false) . help ("The collection will be sorted in the order specified by the key entered for `sort_key`. To specify the order, use the value `asc` for ascending or `desc` for descending."))
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsSubscribersFindStatus :: Subscribed . to_string () , types :: GetAccountsListsSubscribersFindStatus :: Unsubscribed . to_string () , types :: GetAccountsListsSubscribersFindStatus :: Unconfirmed . to_string () ,]) , | s | types :: GetAccountsListsSubscribersFindStatus :: try_from (s) . unwrap ())) . required (false) . help ("The subscriber's status"))
            .arg (clap::Arg::new ("subscribed-after") . long ("subscribed-after") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day on or after the subscriber subscribed"))
            .arg (clap::Arg::new ("subscribed-at") . long ("subscribed-at") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day in which the subscriber subscribed"))
            .arg (clap::Arg::new ("subscribed-before") . long ("subscribed-before") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day on or before the subscriber subscribed"))
            .arg (clap::Arg::new ("subscription-method") . long ("subscription-method") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsSubscribersFindSubscriptionMethod :: Api . to_string () , types :: GetAccountsListsSubscribersFindSubscriptionMethod :: Email . to_string () , types :: GetAccountsListsSubscribersFindSubscriptionMethod :: Import . to_string () , types :: GetAccountsListsSubscribersFindSubscriptionMethod :: Webform . to_string () ,]) , | s | types :: GetAccountsListsSubscribersFindSubscriptionMethod :: try_from (s) . unwrap ())) . required (false) . help ("How the subscriber was subscribed"))
            .arg (clap::Arg::new ("tags") . long ("tags") . value_parser (clap::value_parser! (String)) . required (false) . help ("A tag to match. All tags must match for the subscriber to match."))
            .arg (clap::Arg::new ("tags-not-in") . long ("tags-not-in") . value_parser (clap::value_parser! (String)) . required (false) . help ("A tag to exclude. Checks that all tags are not matched to a subscriber."))
            .arg (clap::Arg::new ("unsubscribe-method") . long ("unsubscribe-method") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsSubscribersFindUnsubscribeMethod :: UnsubscribeLink . to_string () , types :: GetAccountsListsSubscribersFindUnsubscribeMethod :: CustomerCp . to_string () , types :: GetAccountsListsSubscribersFindUnsubscribeMethod :: Undeliverable . to_string () , types :: GetAccountsListsSubscribersFindUnsubscribeMethod :: ApiUnsubscribe . to_string () , types :: GetAccountsListsSubscribersFindUnsubscribeMethod :: ApiMove . to_string () ,]) , | s | types :: GetAccountsListsSubscribersFindUnsubscribeMethod :: try_from (s) . unwrap ())) . required (false) . help ("How the subscriber unsubscribed"))
            .arg (clap::Arg::new ("unsubscribed-after") . long ("unsubscribed-after") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day on or after the subscriber unsubscribed"))
            .arg (clap::Arg::new ("unsubscribed-at") . long ("unsubscribed-at") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day in which the subscriber unsubscribed"))
            .arg (clap::Arg::new ("unsubscribed-before") . long ("unsubscribed-before") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day on or before the subscriber unsubscribed"))
            .arg (clap::Arg::new ("verified-at") . long ("verified-at") . value_parser (clap::value_parser! (chrono::NaiveDate)) . required (false) . help ("The day in which the subscriber confirmed their email address"))
            .arg (clap::Arg::new ("ws-show") . long ("ws-show") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsSubscribersFindWsShow :: TotalSize . to_string () ,]) , | s | types :: GetAccountsListsSubscribersFindWsShow :: try_from (s) . unwrap ())) . required (false) . help ("A flag to show the total size only - expecting \\\"total_size\\\", when added the response will be an integer"))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(limit_arg())
            .about ("Find subscribers for list")
}
fn cli_get_subscriber() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .args(subscriber_id_args())
        .group(subscriber_id_group())
        .about("Get subscriber")
}
fn cli_move_subscriber() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("enforce-custom-field-mapping") . long ("enforce-custom-field-mapping") . value_parser (clap::value_parser! (bool)) . required (false) . help ("If set to true, this will cause the move of a subscriber to fail if the custom fields from the origin list do not match (case insensitively) to the target list"))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i64)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0, the Subscriber will receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("list-link") . long ("list-link") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The link to the destination [List](#tag/Lists/paths/~1accounts~1{accountId}~1lists~1{listId}/get)"))
            .args(subscriber_id_args())
            .group(subscriber_id_group())
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Move subscriber")
}
fn cli_delete_subscriber() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .args(subscriber_id_args())
        .group(subscriber_id_group())
        .about("Delete subscriber by ID")
}
fn cli_update_subscriber() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: UpdateSubscriberRequestBodyAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("custom-field") . long ("custom-field") . value_parser (clap::value_parser! (String)) . required (false) . action (clap::ArgAction::Append) . value_name ("KEY[=VALUE]") . help ("Set a custom field (KEY=VALUE, KEY= for empty string, KEY for null)"))
            .arg (clap::Arg::new ("new-email") . long ("new-email") . value_parser (clap::value_parser! (String)) . required (false) . help ("Set the subscriber's email address"))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i64)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0, the Subscriber will receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."))
            .args(list_id_args())
            .group(list_id_group())
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (String)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: UpdateSubscriberRequestBodyName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: UpdateSubscriberRequestBodyStatus :: Subscribed . to_string () , types :: UpdateSubscriberRequestBodyStatus :: Unsubscribed . to_string () ,]) , | s | types :: UpdateSubscriberRequestBodyStatus :: try_from (s) . unwrap ())) . required (false) . help ("The subscriber's status. **Note** you cannot set a subscriber's status to \"unconfirmed\"."))
            .arg (clap::Arg::new ("strict-custom-fields") . long ("strict-custom-fields") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: UpdateSubscriberRequestBodyStrictCustomFields :: True . to_string () , types :: UpdateSubscriberRequestBodyStrictCustomFields :: False . to_string () ,]) , | s | types :: UpdateSubscriberRequestBodyStrictCustomFields :: try_from (s) . unwrap ())) . required (false) . help ("If this parameter is present and set to `true`, then custom field names are matched case sensitively.  Enabling this option also causes the operation to fail if a custom field is included that is not defined for the list."))
            .args(subscriber_id_args())
            .group(subscriber_id_group())
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Update subscriber by ID")
}
fn cli_get_subscriber_activity() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .args(subscriber_id_args())
        .group(subscriber_id_group())
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get subscriber activity")
}
fn cli_list_tags() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .about("Get tags for list")
}
fn cli_list_web_form_split_tests() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get split tests for list")
}
fn cli_get_web_form_split_test() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("split-test-id")
                .long("split-test-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The webform split test ID"),
        )
        .about("Get split test for list")
}
fn cli_list_web_form_split_test_components() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("split-test-id")
                .long("split-test-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The webform split test ID"),
        )
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get split test components")
}
fn cli_get_web_form_split_test_component() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("split-test-component-id")
                .long("split-test-component-id")
                .value_parser(clap::value_parser!(String))
                .required(true)
                .help("The webform split test component ID"),
        )
        .arg(
            clap::Arg::new("split-test-id")
                .long("split-test-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The webform split test ID"),
        )
        .about("Get split test component")
}
fn cli_list_web_forms() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("ws-size")
                .long("ws-size")
                .value_parser(clap::value_parser!(std::num::NonZeroU32))
                .required(false)
                .help("The pagination total entries to retrieve"),
        )
        .arg(
            clap::Arg::new("ws-start")
                .long("ws-start")
                .value_parser(clap::value_parser!(i32))
                .required(false)
                .help("The pagination starting offset"),
        )
        .arg(limit_arg())
        .about("Get webforms for list")
}
fn cli_get_web_form() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .arg(
            clap::Arg::new("webform-id")
                .long("webform-id")
                .value_parser(clap::value_parser!(i32))
                .required(true)
                .help("The webform ID"),
        )
        .about("Get webform for list")
}
fn cli_get_broadcast_link_analytics() -> clap::Command {
    clap::Command::new ("")
            .arg (clap::Arg::new ("after") . long ("after") . value_parser (clap::value_parser! (String)) . required (false) . help ("specifies the IDs for pagination, for results from after onward"))
            .arg (clap::Arg::new ("before") . long ("before") . value_parser (clap::value_parser! (i64)) . required (false) . help ("specifies the IDs for pagination, for results from before onward"))
            .arg (clap::Arg::new ("broadcast-id") . long ("broadcast-id") . value_parser (clap::value_parser! (types :: GetBroadcastLinksAnalyticsBroadcastId)) . required (true) . help ("Broadcast UUID. Can be found using the [Get broadcasts](#tag/Broadcasts/paths/~1accounts~1{accountId}~1lists~1{listId}~1broadcasts/get) endpoint."))
            .arg (clap::Arg::new ("filter") . long ("filter") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetBroadcastLinksAnalyticsFilter :: Clicks . to_string () , types :: GetBroadcastLinksAnalyticsFilter :: Pageviews . to_string () ,]) , | s | types :: GetBroadcastLinksAnalyticsFilter :: try_from (s) . unwrap ())) . required (true) . help ("Type of link data to retrieve"))
            .arg (clap::Arg::new ("max-count") . long ("max-count") . value_parser (clap::value_parser! (u64)) . required (false) . help ("Maximum count threshold for unique links"))
            .arg (clap::Arg::new ("min-count") . long ("min-count") . value_parser (clap::value_parser! (u64)) . required (false) . help ("Minimum count threshold for unique links"))
            .arg (clap::Arg::new ("page-size") . long ("page-size") . value_parser (clap::value_parser! (std::num::NonZeroU64)) . required (false) . help ("specifies the max number of items in a single page"))
            .arg (clap::Arg::new ("sort-asc") . long ("sort-asc") . value_parser (clap::value_parser! (bool)) . required (false) . help ("Whether to sort in ascending order (true) or descending order (false)"))
            .arg (clap::Arg::new ("sort-by") . long ("sort-by") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetBroadcastLinksAnalyticsSortBy :: Unique . to_string () , types :: GetBroadcastLinksAnalyticsSortBy :: Total . to_string () ,]) , | s | types :: GetBroadcastLinksAnalyticsSortBy :: try_from (s) . unwrap ())) . required (false) . help ("Field to sort the results by"))
            .arg(limit_arg())
            .about ("Broadcast Links Analytics")
}
fn workflow_args() -> [clap::Arg; 2] {
    [
        clap::Arg::new("workflow")
            .required(true)
            .value_parser(clap::value_parser!(String))
            .help("The workflow UUID, or its name with --list"),
        clap::Arg::new("list")
            .long("list")
            .value_parser(clap::value_parser!(String))
            .help("The list name or unique list ID (scopes name resolution)"),
    ]
}

fn workflow_list_arg(required: bool) -> clap::Arg {
    clap::Arg::new("list")
        .long("list")
        .required(required)
        .value_parser(clap::value_parser!(String))
        .help("The list name or unique list ID")
}

fn workflow_page_size_arg() -> clap::Arg {
    clap::Arg::new("page-size")
        .long("page-size")
        .value_parser(clap::value_parser!(u8).range(1..=100))
        .help("Entries per request (1-100)")
}

fn workflow_timezone_arg() -> clap::Arg {
    clap::Arg::new("timezone")
        .long("timezone")
        .value_parser(clap::value_parser!(String))
        .help("IANA timezone name")
}

fn workflow_update_timezone_arg() -> clap::Arg {
    clap::Arg::new("timezone")
        .long("timezone")
        .value_parser(clap::value_parser!(String))
        .help("IANA timezone name")
        .long_help(
            "IANA timezone name.\n\n\
                 Passing --timezone also sends update_times=true. The server rewrites \
                 every wait action's timezone and shifts each rrule's BYHOUR to keep \
                 the same wall-clock time, and it writes the rewritten actions into \
                 the unpublished ruleset slot, so `workflows publish` is what makes \
                 them live.",
        )
}

fn cli_list_workflows() -> clap::Command {
    clap::Command::new("")
        .arg(workflow_list_arg(true))
        .arg(limit_arg())
        .about("List the workflows on a list")
}

fn cli_get_workflow() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .about("Get a workflow with its message subjects and visualization")
}

fn cli_tree_workflow() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .about("Render a workflow as a text tree")
}

fn cli_create_workflow() -> clap::Command {
    clap::Command::new("")
        .arg(workflow_list_arg(true))
        .arg(
            clap::Arg::new("name")
                .long("name")
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("The workflow name"),
        )
        .arg(workflow_timezone_arg())
        .about("Create a workflow")
}

fn cli_update_workflow() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .arg(
            clap::Arg::new("name")
                .long("name")
                .value_parser(clap::value_parser!(String))
                .help("The workflow name"),
        )
        .arg(workflow_update_timezone_arg())
        .arg(
            clap::Arg::new("sharing-enabled")
                .long("sharing-enabled")
                .value_parser(clap::value_parser!(bool))
                .help("Whether the workflow may be shared"),
        )
        .arg(
            clap::Arg::new("auto-extend-enabled")
                .long("auto-extend-enabled")
                .value_parser(clap::value_parser!(bool))
                .help("Whether the workflow auto-extends"),
        )
        .arg(
            clap::Arg::new("patch")
                .long("patch")
                .value_parser(clap::value_parser!(String))
                .help("A JSON Patch document (use - for stdin)"),
        )
        .about("Update a workflow's settings")
}

fn cli_update_workflow_ruleset() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .arg(
            clap::Arg::new("file")
                .long("file")
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("A ruleset with events and actions (use - for stdin)"),
        )
        .about("Replace a workflow's unpublished ruleset")
}

fn cli_publish_workflow() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .about("Publish a workflow's unpublished ruleset")
}

fn cli_revert_workflow() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .about("Discard a workflow's unpublished changes")
}

fn cli_set_workflow_state() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .arg(
            clap::Arg::new("state")
                .long("state")
                .required(true)
                .value_parser(clap::builder::TypedValueParser::map(
                    clap::builder::PossibleValuesParser::new([
                        "active", "paused", "draining", "stopped",
                    ]),
                    |s: String| {
                        s.parse::<crate::workflows::StatusChange>()
                            .expect("clap restricted the value")
                    },
                ))
                .help("The state to set"),
        )
        .about("Set a workflow's state")
}

fn cli_copy_workflow() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("workflow")
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("The workflow UUID, or its name resolved within --list"),
        )
        .arg(workflow_list_arg(true))
        .arg(
            clap::Arg::new("name")
                .long("name")
                .value_parser(clap::value_parser!(String))
                .help("The copy's name (at most 500 characters)"),
        )
        .arg(workflow_timezone_arg())
        .about("Copy a workflow onto a list")
        .long_about(
            "Copy a workflow onto a list.\n\n\
                 The copy is unusable until it leaves the loading state. The returned \
                 state is loading when the source has a ruleset and draft when it does \
                 not.\n\n\
                 A webfeed source is refused with 400. A copy whose target list belongs \
                 to another account needs sharing_enabled on the source and is refused \
                 with 403 without it.",
        )
}

fn cli_delete_workflow() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .arg(
            clap::Arg::new("delete-messages")
                .long("delete-messages")
                .action(clap::ArgAction::SetTrue)
                .help("Delete the workflow's messages instead of leaving them in Drafts"),
        )
        .about("Delete a workflow")
}

fn cli_get_workflow_stats() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .about("Get a workflow's message totals and recurring events")
}

fn cli_get_workflow_message_stats() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .arg(
            clap::Arg::new("message-id")
                .long("message-id")
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("The 24 hex digit message id"),
        )
        .arg(workflow_page_size_arg())
        .arg(
            clap::Arg::new("links-cursor")
                .long("links-cursor")
                .value_parser(clap::value_parser!(String))
                .help("Continue the link statistics from this cursor"),
        )
        .arg(
            clap::Arg::new("send-instances-cursor")
                .long("send-instances-cursor")
                .value_parser(clap::value_parser!(String))
                .help("Continue the send instances from this cursor"),
        )
        .about("Get one workflow message's link and send statistics")
}

fn cli_get_workflow_event_history() -> clap::Command {
    clap::Command::new("")
        .args(workflow_args())
        .arg(
            clap::Arg::new("event-id")
                .long("event-id")
                .required(true)
                .value_parser(clap::value_parser!(uuid::Uuid))
                .help("The recurring event's UUID"),
        )
        .arg(workflow_page_size_arg())
        .arg(
            clap::Arg::new("start-token")
                .long("start-token")
                .value_parser(clap::value_parser!(i64))
                .help("Continue from this Unix-seconds token"),
        )
        .arg(limit_arg())
        .about("Get a recurring event's history")
}

fn cli_unsubscribe_subscriber() -> clap::Command {
    clap::Command::new("")
        .args(list_id_args())
        .group(list_id_group())
        .args(subscriber_id_args())
        .group(subscriber_id_group())
        .about("Unsubscribe a subscriber, keeping their history")
}

fn cli_oauth_get_access_token() -> clap::Command {
    clap::Command::new("")
        .arg(
            ::clap::Arg::new("oauth-callback")
                .long("oauth-callback")
                .value_parser(::clap::value_parser!(types::OauthCallback))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-consumer-key")
                .long("oauth-consumer-key")
                .value_parser(::clap::value_parser!(types::OauthConsumerKey))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-nonce")
                .long("oauth-nonce")
                .value_parser(::clap::value_parser!(types::OauthNonce))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-signature")
                .long("oauth-signature")
                .value_parser(::clap::value_parser!(types::OauthSignature))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-signature-method")
                .long("oauth-signature-method")
                .value_parser(::clap::value_parser!(types::OauthSignatureMethod))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-timestamp")
                .long("oauth-timestamp")
                .value_parser(::clap::value_parser!(types::OauthTimestamp))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-token")
                .long("oauth-token")
                .value_parser(::clap::value_parser!(types::OauthToken))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-version")
                .long("oauth-version")
                .value_parser(::clap::value_parser!(types::OauthVersion))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("json-body")
                .long("json-body")
                .value_name("JSON-FILE")
                .required(false)
                .value_parser(::clap::value_parser!(std::path::PathBuf))
                .help("Path to a file that contains the full json body."),
        )
        .about("Get an access token")
}
fn cli_oauth_get_request_token() -> clap::Command {
    clap::Command::new("")
        .arg(
            ::clap::Arg::new("oauth-callback")
                .long("oauth-callback")
                .value_parser(::clap::value_parser!(types::OauthCallback))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-consumer-key")
                .long("oauth-consumer-key")
                .value_parser(::clap::value_parser!(types::OauthConsumerKey))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-nonce")
                .long("oauth-nonce")
                .value_parser(::clap::value_parser!(types::OauthNonce))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-signature")
                .long("oauth-signature")
                .value_parser(::clap::value_parser!(types::OauthSignature))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-signature-method")
                .long("oauth-signature-method")
                .value_parser(::clap::value_parser!(types::OauthSignatureMethod))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-timestamp")
                .long("oauth-timestamp")
                .value_parser(::clap::value_parser!(types::OauthTimestamp))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-token")
                .long("oauth-token")
                .value_parser(::clap::value_parser!(types::OauthToken))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("oauth-version")
                .long("oauth-version")
                .value_parser(::clap::value_parser!(types::OauthVersion))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("json-body")
                .long("json-body")
                .value_name("JSON-FILE")
                .required(false)
                .value_parser(::clap::value_parser!(std::path::PathBuf))
                .help("Path to a file that contains the full json body."),
        )
        .about("Get a request token")
}
fn cli_oauth_revoke() -> clap::Command {
    clap::Command::new("")
        .arg(
            ::clap::Arg::new("authorization")
                .long("authorization")
                .value_parser(::clap::value_parser!(::std::string::String))
                .required(false),
        )
        .arg(
            ::clap::Arg::new("json-body")
                .long("json-body")
                .value_name("JSON-FILE")
                .required(true)
                .value_parser(::clap::value_parser!(std::path::PathBuf))
                .help("Path to a file that contains the full json body."),
        )
        .about("Revoke a token")
}
fn cli_oauth_token() -> clap::Command {
    clap::Command::new("")
        .arg(
            clap::Arg::new("authorization")
                .long("authorization")
                .value_parser(clap::value_parser!(String))
                .required(false),
        )
        .arg(
            clap::Arg::new("json-body")
                .long("json-body")
                .value_name("JSON-FILE")
                .required(true)
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .help("Path to a file that contains the full json body."),
        )
        .about("Get a token")
}

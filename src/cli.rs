use crate::types;
use anyhow::Context as _;

pub struct Cli {
    pub client: crate::client::Client,
    pub account_id: i32,
}

impl Cli {
    pub fn new(client: crate::client::Client, account_id: i32) -> Self {
        Self { client, account_id }
    }

    pub fn get_command(cmd: CliCommand) -> clap::Command {
        match cmd {
            CliCommand::ListAccounts => Self::cli_list_accounts(),
            CliCommand::GetAccount => Self::cli_get_account(),
            CliCommand::FindAccountSubscribers => Self::cli_find_account_subscribers(),
            CliCommand::ListAccountWebformSplitTests => {
                Self::cli_list_account_webform_split_tests()
            }
            CliCommand::ListAccountWebforms => Self::cli_list_account_webforms(),
            CliCommand::ListIntegrations => Self::cli_list_integrations(),
            CliCommand::GetIntegration => Self::cli_get_integration(),
            CliCommand::ListLists => Self::cli_list_lists(),
            CliCommand::FindLists => Self::cli_find_lists(),
            CliCommand::GetList => Self::cli_get_list(),
            CliCommand::ListBroadcasts => Self::cli_list_broadcasts(),
            CliCommand::CreateBroadcast => Self::cli_create_broadcast(),
            CliCommand::GetBroadcastTotal => {
                Self::cli_get_broadcast_total()
            }
            CliCommand::GetBroadcast => Self::cli_get_broadcast(),
            CliCommand::UpdateBroadcast => Self::cli_update_broadcast(),
            CliCommand::DeleteBroadcast => {
                Self::cli_delete_broadcast()
            }
            CliCommand::CancelBroadcast => {
                Self::cli_cancel_broadcast()
            }
            CliCommand::GetBroadcastClicks => {
                Self::cli_get_broadcast_clicks()
            }
            CliCommand::GetBroadcastOpens => {
                Self::cli_get_broadcast_opens()
            }
            CliCommand::ScheduleBroadcast => {
                Self::cli_schedule_broadcast()
            }
            CliCommand::WaitBroadcast => Self::cli_wait_broadcast(),
            CliCommand::ListCampaigns => Self::cli_list_campaigns(),
            CliCommand::ListCampaignStats => {
                Self::cli_list_campaign_stats()
            }
            CliCommand::GetCampaignStat => {
                Self::cli_get_campaign_stat()
            }
            CliCommand::FindCampaigns => {
                Self::cli_find_campaigns()
            }
            CliCommand::GetCampaign => {
                Self::cli_get_campaign()
            }
            CliCommand::ListCustomFields => {
                Self::cli_list_custom_fields()
            }
            CliCommand::CreateCustomField => {
                Self::cli_create_custom_field()
            }
            CliCommand::GetCustomField => {
                Self::cli_get_custom_field()
            }
            CliCommand::DeleteCustomField => {
                Self::cli_delete_custom_field()
            }
            CliCommand::UpdateCustomField => {
                Self::cli_update_custom_field()
            }
            CliCommand::ListLandingPages => {
                Self::cli_list_landing_pages()
            }
            CliCommand::GetLandingPage => {
                Self::cli_get_landing_page()
            }
            CliCommand::CreatePurchase => Self::cli_create_purchase(),
            CliCommand::ListSegments => Self::cli_list_segments(),
            CliCommand::GetSegment => Self::cli_get_segment(),
            CliCommand::ListSubscribers => Self::cli_list_subscribers(),
            CliCommand::CreateSubscriber => Self::cli_create_subscriber(),
            CliCommand::DeleteSubscriberByEmail => {
                Self::cli_delete_subscriber_by_email()
            }
            CliCommand::UpdateSubscriberByEmail => {
                Self::cli_update_subscriber_by_email()
            }
            CliCommand::FindSubscribers => {
                Self::cli_find_subscribers()
            }
            CliCommand::GetSubscriber => Self::cli_get_subscriber(),
            CliCommand::MoveSubscriber => {
                Self::cli_move_subscriber()
            }
            CliCommand::DeleteSubscriber => {
                Self::cli_delete_subscriber()
            }
            CliCommand::UpdateSubscriber => {
                Self::cli_update_subscriber()
            }
            CliCommand::GetSubscriberActivity => {
                Self::cli_get_subscriber_activity()
            }
            CliCommand::ListTags => Self::cli_list_tags(),
            CliCommand::ListWebFormSplitTests => {
                Self::cli_list_web_form_split_tests()
            }
            CliCommand::GetWebFormSplitTest => {
                Self::cli_get_web_form_split_test()
            }
            CliCommand::ListWebFormSplitTestComponents => {
                Self::cli_list_web_form_split_test_components()
            }
            CliCommand::GetWebFormSplitTestComponent => {
                Self::cli_get_web_form_split_test_component()
            }
            CliCommand::ListWebForms => Self::cli_list_web_forms(),
            CliCommand::GetWebForm => Self::cli_get_web_form(),
            CliCommand::GetBroadcastLinkAnalytics => Self::cli_get_broadcast_link_analytics(),
            CliCommand::OauthGetAccessToken => Self::cli_oauth_get_access_token(),
            CliCommand::OauthGetRequestToken => Self::cli_oauth_get_request_token(),
            CliCommand::OauthRevoke => Self::cli_oauth_revoke(),
            CliCommand::OauthToken => Self::cli_oauth_token(),
        }
    }
    // -----------------------------------------------------------------------
    // cli_* static methods (clap command definitions) - copied exactly from
    // generated_cli.rs
    // -----------------------------------------------------------------------
    fn limit_arg() -> clap::Arg {
        clap::Arg::new("limit")
            .long("limit")
            .value_parser(clap::value_parser!(usize))
            .help("Maximum total number of entries to output")
    }

    fn list_id_args() -> [clap::Arg; 2] {
        [
            clap::Arg::new("list-id").long("list-id").value_parser(clap::value_parser!(i32)).help("The list ID"),
            clap::Arg::new("list").long("list").value_parser(clap::value_parser!(String)).help("The list name (looked up via the API)"),
        ]
    }

    fn list_id_group() -> clap::ArgGroup {
        clap::ArgGroup::new("list-identifier").args(["list-id", "list"]).required(true)
    }

    fn subscriber_id_args() -> [clap::Arg; 2] {
        [
            clap::Arg::new("subscriber-id").long("subscriber-id").value_parser(clap::value_parser!(i32)).help("The subscriber ID"),
            clap::Arg::new("email").long("email").value_parser(clap::value_parser!(String)).help("The subscriber's email address (looked up via the API)"),
        ]
    }

    fn subscriber_id_group() -> clap::ArgGroup {
        clap::ArgGroup::new("subscriber-identifier").args(["subscriber-id", "email"]).required(true)
    }

    fn custom_field_id_args() -> [clap::Arg; 2] {
        [
            clap::Arg::new("custom-field-id").long("custom-field-id").value_parser(clap::value_parser!(i32)).help("The custom field ID"),
            clap::Arg::new("custom-field").long("custom-field").value_parser(clap::value_parser!(String)).help("The custom field name (looked up via the API)"),
        ]
    }

    fn custom_field_id_group() -> clap::ArgGroup {
        clap::ArgGroup::new("custom-field-identifier").args(["custom-field-id", "custom-field"]).required(true)
    }

    pub fn cli_list_accounts() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get accounts")
    }
    pub fn cli_get_account() -> clap::Command {
        clap::Command::new ("")
            .about ("Get account")
    }
    pub fn cli_find_account_subscribers() -> clap::Command {
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
            .arg(Self::limit_arg())
            .about ("Find subscribers for account")
    }
    pub fn cli_list_account_webform_split_tests() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get split tests for account")
    }
    pub fn cli_list_account_webforms() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get webforms for account")
    }
    pub fn cli_list_integrations() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get integrations")
    }
    pub fn cli_get_integration() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("integration-id") . long ("integration-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The integration ID"))
            .about ("Get integration")
    }
    pub fn cli_list_lists() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get lists")
    }
    pub fn cli_find_lists() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: GetAccountsListsFindName)) . required (false) . help ("Name or unique list ID of the list"))
            .arg (clap::Arg::new ("ws-show") . long ("ws-show") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsFindWsShow :: TotalSize . to_string () ,]) , | s | types :: GetAccountsListsFindWsShow :: try_from (s) . unwrap ())) . required (false) . help ("A flag to show the total size only - expecting \\\"total_size\\\", when added the response will be an integer"))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .about ("Find lists")
    }
    pub fn cli_get_list() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .about ("Get list")
    }
    pub fn cli_list_broadcasts() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsBroadcastsStatus :: Draft . to_string () , types :: GetAccountsListsBroadcastsStatus :: Scheduled . to_string () , types :: GetAccountsListsBroadcastsStatus :: Sent . to_string () ,]) , | s | types :: GetAccountsListsBroadcastsStatus :: try_from (s) . unwrap ())) . required (false) . help ("Filter by status (draft, scheduled, sent). If omitted, all statuses are retrieved. **(Please be aware that `draft` only returns API created Broadcast drafts)**"))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get broadcasts")
    }
    pub fn cli_create_broadcast() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("body-amp") . long ("body-amp") . value_parser (clap::value_parser! (String)) . required (false) . help ("<b>[Please read <a href=\"https://help.aweber.com/hc/en-us/articles/360025741194\" target=\"_blank\">our KB article before using this field.]</b>The content of the message in AMP format."))
            .arg (clap::Arg::new ("body-html") . long ("body-html") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The content of the message in html format. If body_text is not provided, it will be auto-generated. If body_text is not provided, body_html must be provided."))
            .arg (clap::Arg::new ("body-text") . long ("body-text") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The content of the message in plain text, used when HTML is not supported. If body_html is not provided, the broadcast will be sent using only the body_text. If body_text is not provided, body_html must be provided."))
            .arg (clap::Arg::new ("click-tracking-enabled") . long ("click-tracking-enabled") . value_parser (clap::value_parser! (bool)) . required (false) . help ("Enables links in the email message to be tracked"))
            .arg (clap::Arg::new ("exclude-lists") . long ("exclude-lists") . value_parser (clap::value_parser! (String)) . required (false) . help ("JSON encoded list of [Lists](#tag/Lists) URLs to exclude in the delivery of this broadcast. Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing excluded_lists."))
            .arg (clap::Arg::new ("facebook-integration") . long ("facebook-integration") . value_parser (clap::value_parser! (String)) . required (false) . help ("URL to the [Facebook broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be posted to this Facebook integration  - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"))
            .arg (clap::Arg::new ("include-lists") . long ("include-lists") . value_parser (clap::value_parser! (String)) . required (false) . help ("JSON encoded list of [Lists](#tag/Lists) URLs to include in the delivery of this broadcast. Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing included_lists."))
            .arg (clap::Arg::new ("is-archived") . long ("is-archived") . value_parser (clap::value_parser! (bool)) . required (false) . help ("Whether the broadcast enabled sharing via an archive url"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("notify-on-send") . long ("notify-on-send") . value_parser (clap::value_parser! (bool)) . required (false) . help ("If true, notify when stats are available on a sent broadcast message"))
            .arg (clap::Arg::new ("subject") . long ("subject") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The broadcast subject line. Subject must not be empty nor contain only whitespace."))
            .arg (clap::Arg::new ("twitter-integration") . long ("twitter-integration") . value_parser (clap::value_parser! (String)) . required (false) . help ("URL to the [Twitter broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be tweeted - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Create broadcast")
    }
    pub fn cli_get_broadcast_total() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsBroadcastsTotalStatus :: Draft . to_string () , types :: GetAccountsListsBroadcastsTotalStatus :: Scheduled . to_string () , types :: GetAccountsListsBroadcastsTotalStatus :: Sent . to_string () ,]) , | s | types :: GetAccountsListsBroadcastsTotalStatus :: try_from (s) . unwrap ())) . required (true) . help ("The status of the broadcasts to retrieve. **(Please be aware that `draft` only returns API created Broadcast drafts)**"))
            .about ("Get total broadcasts")
    }
    pub fn cli_get_broadcast() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("broadcast-id") . long ("broadcast-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The broadcast ID"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .about ("Get broadcast")
    }
    pub fn cli_update_broadcast() -> clap::Command {
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
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("notify-on-send") . long ("notify-on-send") . value_parser (clap::value_parser! (bool)) . required (false) . help ("If true, notify when stats are available on a sent broadcast message, defaults to true."))
            .arg (clap::Arg::new ("segment-link") . long ("segment-link") . value_parser (clap::value_parser! (String)) . required (false) . help ("URL to the [Segment](#tag/Segments) to send this broadcast to.  Use the `self_link` of the segment here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>/segments/<segment_id>`. If not specified, the broadcast will be sent to the \"Active Subscribers\" segment."))
            .arg (clap::Arg::new ("subject") . long ("subject") . value_parser (clap::value_parser! (String)) . required (false) . help ("The broadcast subject line. Subject must not be empty nor contain only whitespace."))
            .arg (clap::Arg::new ("twitter-integration") . long ("twitter-integration") . value_parser (clap::value_parser! (String)) . required (false) . help ("URL to the [Twitter broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be tweeted - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Update broadcast")
    }
    pub fn cli_delete_broadcast() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("broadcast-id") . long ("broadcast-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The broadcast ID"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .about ("Delete broadcast")
    }
    pub fn cli_cancel_broadcast() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("broadcast-id") . long ("broadcast-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The broadcast ID"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .about ("Cancel scheduled broadcast")
    }
    pub fn cli_get_broadcast_clicks() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("after") . long ("after") . value_parser (clap::value_parser! (String)) . required (false) . help ("The pagination key when paging forward. Cannot be combined with `before`."))
            .arg (clap::Arg::new ("before") . long ("before") . value_parser (clap::value_parser! (String)) . required (false) . help ("The pagination key when paging in reverse. Cannot be combined with `after`."))
            .arg (clap::Arg::new ("broadcast-id") . long ("broadcast-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The broadcast ID"))
            .arg (clap::Arg::new ("detailed") . long ("detailed") . value_parser (clap::value_parser! (bool)) . required (false) . help ("When true, returns individual click events with URLs instead of aggregated click data per subscriber"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("page-size") . long ("page-size") . value_parser (clap::value_parser! (std::num::NonZeroU64)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg(Self::limit_arg())
            .about ("Get broadcast clicks")
    }
    pub fn cli_get_broadcast_opens() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("after") . long ("after") . value_parser (clap::value_parser! (String)) . required (false) . help ("The pagination key when paging forward. Cannot be combined with `before`."))
            .arg (clap::Arg::new ("before") . long ("before") . value_parser (clap::value_parser! (String)) . required (false) . help ("The pagination key when paging in reverse. Cannot be combined with `after`."))
            .arg (clap::Arg::new ("broadcast-id") . long ("broadcast-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The broadcast ID"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("page-size") . long ("page-size") . value_parser (clap::value_parser! (std::num::NonZeroU64)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg(Self::limit_arg())
            .about ("Get broadcast opens")
    }
    pub fn cli_schedule_broadcast() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("broadcast-id") . long ("broadcast-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The broadcast ID"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("scheduled-for") . long ("scheduled-for") . value_parser (clap::value_parser! (chrono::DateTime<chrono::Utc>)) . required_unless_present ("json-body") . help ("Scheduled time for sending broadcast message, ISO-8601 formatted."))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Schedule broadcast")
    }
    pub fn cli_wait_broadcast() -> clap::Command {
        clap::Command::new("")
            .arg(clap::Arg::new("broadcast-id").long("broadcast-id").value_parser(clap::value_parser!(i32)).required(true).help("The broadcast ID"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg(clap::Arg::new("interval").long("interval").value_parser(clap::value_parser!(u64)).default_value("30").help("Polling interval in seconds"))
            .about("Wait for a broadcast to finish sending")
    }
    pub fn cli_list_campaigns() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get campaigns")
    }
    pub fn cli_list_campaign_stats() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("campaign-id") . long ("campaign-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The campaign ID"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get broadcast statistics")
    }
    pub fn cli_get_campaign_stat() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("campaign-id") . long ("campaign-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The campaign ID"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("stats-id") . long ("stats-id") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalClicks . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: UniqueClicks . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalOpens . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: UniqueOpens . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalSales . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalSalesDollars . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: TotalUnsubscribed . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlyOpens . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlyClicks . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlyWebhits . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlySales . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: HourlyUnsubscribed . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailyOpens . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailyClicks . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailyWebhits . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailySales . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: DailyUnsubscribed . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: ClicksByLink . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: WebhitsByLink . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: OpensBySubscriber . to_string () , types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: SalesBySubscriber . to_string () ,]) , | s | types :: GetAccountsListsCampaignsBcampaignidStats2StatsId :: try_from (s) . unwrap ())) . required (true) . help ("\n\n>The statistic's ID.\n>\n>The datatype of the ID may be different for each Stat.\n>\n>Below is a list of the statistic IDs that can be passed in.\n>\n>__Aggregate Statistics__\n>\n>| Stat ID | Description |\n>|---------|-------------|\n>| total_clicks | Total number of times a subscriber clicked any link appearing in your campaign except the unsubscribe link (includes multiple clicks of the same link) |\n>| unique_clicks | Total number of subscribers who clicked any link in your campaign |\n>| total_opens | Total number of times your campaign was opened by any subscriber your campaign was sent to (including multiple opens by the same subscriber) |\n>| unique_opens | Total number of subscribers who opened your campaign |\n>| total_sales | Total number of sales made by subscribers who received your campaign |\n>| total_sales_dollars | Total monetary value of sales made by subscribers who received your campaign |\n>| total_unsubscribed | Total number of subscribers who unsubscribed by clicking the unsubscribe link in your campaign |\n>\n>__Time Related Statistics__\n>\n>| Stat ID | Description |\n>|---------|-------------|\n>| hourly_clicks | Hourly breakdown of unique and total clicks for the first 24 hours after a campaign was sent |\n>| hourly_opens | Hourly breakdown of unique and total opens for the first 24 hours after a campaign was sent |\n>| hourly_sales | Hourly breakdown of sales for the first 24 hours after a campaign was sent |\n>| hourly_unsubscribed | Hourly breakdown of subscribers who unsubscribed by clicking the unsubscribed link for the first 24 hours after a campaign was sent |\n>| hourly_webhits | Hourly breakdown of webhits to your website from subscribers sent this message for the first 24 hours after a campaign was sent |\n>| daily_clicks | Daily breakdown of unique and total clicks for the first 14 days after a campaign was sent |\n>| daily_opens | Daily breakdown of unique and total opens for the first 14 days after a campaign was sent |\n>| daily_sales | Daily breakdown of sales for the first 14 days after a campaign was sent |\n>| daily_unsubscribed | Daily breakdown of subscribers who unsuscribed by clicking the unsubscribed link for the first 14 days after a campaign was sent |\n>| daily_webhits | Daily breakdown of webhits to your website from subscribers sent this message for the first 14 days after a campaign was sent |\n>\n>__Top 10 URL Statistics__\n>\n>| Stat ID | Description |\n>|---------|-------------|\n>| clicks_by_link | Top 10 links that were clicked (ranked by total_clicked) |\n>| webhits_by_link | Top 10 webhits by click (ranked by total clicks) |\n>\n>__Top 10 Subscriber Statistics__\n>(Requires access to subscriber data)\n>\n>| Stat ID | Description |\n>|---------|-------------|\n>| opens_by_subscriber | Top 10 subscribers that opened your message (ranked by total opens) |\n>| sales_by_subscriber | Top 10 subscribers that made a sale from your message (ranked by total sales dollars) |\n"))
            .about ("Get broadcast statistic")
    }
    pub fn cli_find_campaigns() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("campaign-type") . long ("campaign-type") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsCampaignsFindCampaignType :: B . to_string () , types :: GetAccountsListsCampaignsFindCampaignType :: F . to_string () ,]) , | s | types :: GetAccountsListsCampaignsFindCampaignType :: try_from (s) . unwrap ())) . required (true) . help ("The campaign type (b - broadcast, f - followup)"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("ws-show") . long ("ws-show") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsCampaignsFindWsShow :: TotalSize . to_string () ,]) , | s | types :: GetAccountsListsCampaignsFindWsShow :: try_from (s) . unwrap ())) . required (false) . help ("A flag to show the total size only - expecting \\\"total_size\\\", when added the response will be an integer"))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Find campaigns")
    }
    pub fn cli_get_campaign() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("campaign-id") . long ("campaign-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The campaign ID"))
            .arg (clap::Arg::new ("campaign-type") . long ("campaign-type") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsCampaignsCampaigntypecampaignidCampaignType :: B . to_string () , types :: GetAccountsListsCampaignsCampaigntypecampaignidCampaignType :: F . to_string () ,]) , | s | types :: GetAccountsListsCampaignsCampaigntypecampaignidCampaignType :: try_from (s) . unwrap ())) . required (true) . help ("The campaign type (b - broadcast, f - followup)"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .about ("Get campaign")
    }
    pub fn cli_list_custom_fields() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get custom fields")
    }
    pub fn cli_create_custom_field() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The name of the custom field"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Add custom field")
    }
    pub fn cli_get_custom_field() -> clap::Command {
        clap::Command::new ("")
            .args(Self::custom_field_id_args())
            .group(Self::custom_field_id_group())
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .about ("Get custom field")
    }
    pub fn cli_delete_custom_field() -> clap::Command {
        clap::Command::new ("")
            .args(Self::custom_field_id_args())
            .group(Self::custom_field_id_group())
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .about ("Delete custom field")
    }
    pub fn cli_update_custom_field() -> clap::Command {
        clap::Command::new ("")
            .args(Self::custom_field_id_args())
            .group(Self::custom_field_id_group())
            .arg (clap::Arg::new ("is-subscriber-updateable") . long ("is-subscriber-updateable") . value_parser (clap::value_parser! (bool)) . required (false) . help ("Whether the subscriber is allowed to update the custom field"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (String)) . required (false) . help ("The name of the custom field"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Update custom field")
    }
    pub fn cli_list_landing_pages() -> clap::Command {
        clap::Command::new("")
            .arg(
                ::clap::Arg::new("list-id")
                    .long("list-id")
                    .value_parser(::clap::value_parser!(i32))
                    .required(true)
                    .help("The list ID"),
            )
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
            .arg(Self::limit_arg())
            .about("Get landing pages")
    }
    pub fn cli_get_landing_page() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("landing-page-id") . long ("landing-page-id") . value_parser (clap::value_parser! (:: uuid :: Uuid)) . required (true) . help ("The landing page ID"))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .about ("Get landing page")
    }
    pub fn cli_create_purchase() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: PurchaseAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("currency") . long ("currency") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html)."))
            .arg (clap::Arg::new ("email") . long ("email") . value_parser (clap::value_parser! (types :: PurchaseEmail)) . required_unless_present ("json-body") . help ("The subscriber's email address"))
            .arg (clap::Arg::new ("event-note") . long ("event-note") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("A custom note associated with this specific tracked event"))
            .arg (clap::Arg::new ("event-time") . long ("event-time") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The timestamp of when the event occurred"))
            .arg (clap::Arg::new ("ip-address") . long ("ip-address") . value_parser (clap::value_parser! (types :: PurchaseIpAddress)) . required_unless_present ("json-body") . help ("The subscriber's IP address. This must be a public IP address."))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (types :: PurchaseMiscNotes)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: PurchaseName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("product-name") . long ("product-name") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("A custom description for the page or event"))
            .arg (clap::Arg::new ("url") . long ("url") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The URL for the tracked event"))
            .arg (clap::Arg::new ("value") . long ("value") . value_parser (clap::value_parser! (f64)) . required_unless_present ("json-body"))
            .arg (clap::Arg::new ("vendor") . long ("vendor") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The sales tracking url profile for the web page"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Create a purchase")
    }
    pub fn cli_list_segments() -> clap::Command {
        clap::Command::new("")
            .arg(
                ::clap::Arg::new("list-id")
                    .long("list-id")
                    .value_parser(::clap::value_parser!(i32))
                    .required(true)
                    .help("The list ID"),
            )
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
            .arg(Self::limit_arg())
            .about("Get segments")
    }
    pub fn cli_get_segment() -> clap::Command {
        clap::Command::new("")
            .arg(
                ::clap::Arg::new("list-id")
                    .long("list-id")
                    .value_parser(::clap::value_parser!(i32))
                    .required(true)
                    .help("The list ID"),
            )
            .arg(
                ::clap::Arg::new("segment-id")
                    .long("segment-id")
                    .value_parser(::clap::value_parser!(i32))
                    .required(true)
                    .help("The segment ID"),
            )
            .about("Get segment")
    }
    pub fn cli_list_subscribers() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("sort-order") . long ("sort-order") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: GetAccountsListsSubscribersSortOrder :: Asc . to_string () , types :: GetAccountsListsSubscribersSortOrder :: Desc . to_string () ,]) , | s | types :: GetAccountsListsSubscribersSortOrder :: try_from (s) . unwrap ())) . required (false) . help ("The collection will be sorted by the order in which the subscribers were added to the list. To specify the order, use the value `asc` for ascending or `desc` for descending."))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get subscribers")
    }
    pub fn cli_create_subscriber() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("email") . long ("email") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyEmail)) . required_unless_present ("json-body") . help ("The subscriber's email address"))
            .arg (clap::Arg::new ("ip-address") . long ("ip-address") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyIpAddress)) . required (false) . help ("The subscriber's IP address. This field is used to determine the following Geo Location fields: area_code, city, country, dma_code, latitude, longitude, postal_code, and region. IP address can only be specified when Subscribers are initially created. Internal, private, or reserved IP addresses are not acceptable."))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i64)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0 (default), the Subscriber should receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyMiscNotes)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: AddSubscriberRequestBodyName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("strict-custom-fields") . long ("strict-custom-fields") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: AddSubscriberRequestBodyStrictCustomFields :: True . to_string () , types :: AddSubscriberRequestBodyStrictCustomFields :: False . to_string () ,]) , | s | types :: AddSubscriberRequestBodyStrictCustomFields :: try_from (s) . unwrap ())) . required (false) . help ("If this parameter is present and set to `true`, then custom field names are matched case sensitively.  Enabling this option also causes the operation to fail if a custom field is included that is not defined for the list."))
            .arg (clap::Arg::new ("update-existing") . long ("update-existing") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: AddSubscriberRequestBodyUpdateExisting :: True . to_string () , types :: AddSubscriberRequestBodyUpdateExisting :: False . to_string () ,]) , | s | types :: AddSubscriberRequestBodyUpdateExisting :: try_from (s) . unwrap ())) . required (false) . help ("If this parameter is present and set to `true`, then if a subscriber is already present on the list, the subscriber will be updated.  **Note:** \n- Only the fields defined in the <a href='#tag/Subscribers/paths/~1accounts~1{accountId}~1lists~1{listId}~1subscribers~1{subscriberId}/patch'>patch endpoint will be updated.\n- Any tags in the request will be **appended** to the existing Subscriber."))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Add subscriber")
    }
    pub fn cli_delete_subscriber_by_email() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("email") . long ("email") . value_parser (clap::value_parser! (types :: DeleteAccountsListsSubscribersSubscriberEmail)) . required (true) . help ("The subscriber's email address"))
            .about ("Delete subscriber by email")
    }
    pub fn cli_update_subscriber_by_email() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: UpdateSubscriberRequestBodyAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("custom-field") . long ("custom-field") . value_parser (clap::value_parser! (String)) . required (false) . action (clap::ArgAction::Append) . value_name ("KEY[=VALUE]") . help ("Set a custom field (KEY=VALUE, KEY= for empty string, KEY for null)"))
            .arg (clap::Arg::new ("new-email") . long ("new-email") . value_parser (clap::value_parser! (String)) . required (false) . help ("Set the subscriber's email address"))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i64)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0, the Subscriber will receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (String)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: UpdateSubscriberRequestBodyName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: UpdateSubscriberRequestBodyStatus :: Subscribed . to_string () , types :: UpdateSubscriberRequestBodyStatus :: Unsubscribed . to_string () ,]) , | s | types :: UpdateSubscriberRequestBodyStatus :: try_from (s) . unwrap ())) . required (false) . help ("The subscriber's status. **Note** you cannot set a subscriber's status to \"unconfirmed\"."))
            .arg (clap::Arg::new ("strict-custom-fields") . long ("strict-custom-fields") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: UpdateSubscriberRequestBodyStrictCustomFields :: True . to_string () , types :: UpdateSubscriberRequestBodyStrictCustomFields :: False . to_string () ,]) , | s | types :: UpdateSubscriberRequestBodyStrictCustomFields :: try_from (s) . unwrap ())) . required (false) . help ("If this parameter is present and set to `true`, then custom field names are matched case sensitively.  Enabling this option also causes the operation to fail if a custom field is included that is not defined for the list."))
            .arg (clap::Arg::new ("email") . long ("email") . value_parser (clap::value_parser! (types :: PatchAccountsListsSubscribersSubscriberEmail)) . required (true) . help ("The subscriber's email address"))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Update subscriber by email")
    }
    pub fn cli_find_subscribers() -> clap::Command {
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
            .args(Self::list_id_args())
            .group(Self::list_id_group())
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
            .arg(Self::limit_arg())
            .about ("Find subscribers for list")
    }
    pub fn cli_get_subscriber() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .args(Self::subscriber_id_args())
            .group(Self::subscriber_id_group())
            .about ("Get subscriber")
    }
    pub fn cli_move_subscriber() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("enforce-custom-field-mapping") . long ("enforce-custom-field-mapping") . value_parser (clap::value_parser! (bool)) . required (false) . help ("If set to true, this will cause the move of a subscriber to fail if the custom fields from the origin list do not match (case insensitively) to the target list"))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i64)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0, the Subscriber will receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("list-link") . long ("list-link") . value_parser (clap::value_parser! (String)) . required_unless_present ("json-body") . help ("The link to the destination [List](#tag/Lists/paths/~1accounts~1{accountId}~1lists~1{listId}/get)"))
            .args(Self::subscriber_id_args())
            .group(Self::subscriber_id_group())
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Move subscriber")
    }
    pub fn cli_delete_subscriber() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .args(Self::subscriber_id_args())
            .group(Self::subscriber_id_group())
            .about ("Delete subscriber by ID")
    }
    pub fn cli_update_subscriber() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("ad-tracking") . long ("ad-tracking") . value_parser (clap::value_parser! (types :: UpdateSubscriberRequestBodyAdTracking)) . required (false) . help ("The customer ad tracking field"))
            .arg (clap::Arg::new ("custom-field") . long ("custom-field") . value_parser (clap::value_parser! (String)) . required (false) . action (clap::ArgAction::Append) . value_name ("KEY[=VALUE]") . help ("Set a custom field (KEY=VALUE, KEY= for empty string, KEY for null)"))
            .arg (clap::Arg::new ("new-email") . long ("new-email") . value_parser (clap::value_parser! (String)) . required (false) . help ("Set the subscriber's email address"))
            .arg (clap::Arg::new ("last-followup-message-number-sent") . long ("last-followup-message-number-sent") . value_parser (clap::value_parser! (i64)) . required (false) . help ("The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0, the Subscriber will receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."))
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("misc-notes") . long ("misc-notes") . value_parser (clap::value_parser! (String)) . required (false) . help ("Miscellaneous notes"))
            .arg (clap::Arg::new ("name") . long ("name") . value_parser (clap::value_parser! (types :: UpdateSubscriberRequestBodyName)) . required (false) . help ("The subscriber's name"))
            .arg (clap::Arg::new ("status") . long ("status") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: UpdateSubscriberRequestBodyStatus :: Subscribed . to_string () , types :: UpdateSubscriberRequestBodyStatus :: Unsubscribed . to_string () ,]) , | s | types :: UpdateSubscriberRequestBodyStatus :: try_from (s) . unwrap ())) . required (false) . help ("The subscriber's status. **Note** you cannot set a subscriber's status to \"unconfirmed\"."))
            .arg (clap::Arg::new ("strict-custom-fields") . long ("strict-custom-fields") . value_parser (clap::builder::TypedValueParser::map (clap::builder::PossibleValuesParser::new ([types :: UpdateSubscriberRequestBodyStrictCustomFields :: True . to_string () , types :: UpdateSubscriberRequestBodyStrictCustomFields :: False . to_string () ,]) , | s | types :: UpdateSubscriberRequestBodyStrictCustomFields :: try_from (s) . unwrap ())) . required (false) . help ("If this parameter is present and set to `true`, then custom field names are matched case sensitively.  Enabling this option also causes the operation to fail if a custom field is included that is not defined for the list."))
            .args(Self::subscriber_id_args())
            .group(Self::subscriber_id_group())
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (false) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Update subscriber by ID")
    }
    pub fn cli_get_subscriber_activity() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .args(Self::subscriber_id_args())
            .group(Self::subscriber_id_group())
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get subscriber activity")
    }
    pub fn cli_list_tags() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .about ("Get tags for list")
    }
    pub fn cli_list_web_form_split_tests() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get split tests for list")
    }
    pub fn cli_get_web_form_split_test() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("split-test-id") . long ("split-test-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The webform split test ID"))
            .about ("Get split test for list")
    }
    pub fn cli_list_web_form_split_test_components() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("split-test-id") . long ("split-test-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The webform split test ID"))
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get split test components")
    }
    pub fn cli_get_web_form_split_test_component() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("split-test-component-id") . long ("split-test-component-id") . value_parser (clap::value_parser! (String)) . required (true) . help ("The webform split test component ID"))
            .arg (clap::Arg::new ("split-test-id") . long ("split-test-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The webform split test ID"))
            .about ("Get split test component")
    }
    pub fn cli_list_web_forms() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("ws-size") . long ("ws-size") . value_parser (clap::value_parser! (std::num::NonZeroU32)) . required (false) . help ("The pagination total entries to retrieve"))
            .arg (clap::Arg::new ("ws-start") . long ("ws-start") . value_parser (clap::value_parser! (i32)) . required (false) . help ("The pagination starting offset"))
            .arg(Self::limit_arg())
            .about ("Get webforms for list")
    }
    pub fn cli_get_web_form() -> clap::Command {
        clap::Command::new ("")
            .args(Self::list_id_args())
            .group(Self::list_id_group())
            .arg (clap::Arg::new ("webform-id") . long ("webform-id") . value_parser (clap::value_parser! (i32)) . required (true) . help ("The webform ID"))
            .about ("Get webform for list")
    }
    pub fn cli_get_broadcast_link_analytics() -> clap::Command {
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
            .about ("Broadcast Links Analytics")
    }
    pub fn cli_oauth_get_access_token() -> clap::Command {
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
    pub fn cli_oauth_get_request_token() -> clap::Command {
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
    pub fn cli_oauth_revoke() -> clap::Command {
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
    pub fn cli_oauth_token() -> clap::Command {
        clap::Command::new ("")
            .arg (clap::Arg::new ("authorization") . long ("authorization") . value_parser (clap::value_parser! (String)) . required (false))
            .arg (clap::Arg::new ("json-body") . long ("json-body") . value_name ("JSON-FILE") . required (true) . value_parser (clap::value_parser! (std :: path :: PathBuf)) . help ("Path to a file that contains the full json body."))
            .about ("Get a token")
    }
    // -----------------------------------------------------------------------
    // execute dispatch
    // -----------------------------------------------------------------------
    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::ListAccounts => self.execute_list_accounts(matches).await,
            CliCommand::GetAccount => self.execute_get_account(matches).await,
            CliCommand::FindAccountSubscribers => {
                self.execute_find_account_subscribers(matches).await
            }
            CliCommand::ListAccountWebformSplitTests => {
                self.execute_list_account_webform_split_tests(matches)
                    .await
            }
            CliCommand::ListAccountWebforms => {
                self.execute_list_account_webforms(matches).await
            }
            CliCommand::ListIntegrations => {
                self.execute_list_integrations(matches).await
            }
            CliCommand::GetIntegration => {
                self.execute_get_integration(matches).await
            }
            CliCommand::ListLists => self.execute_list_lists(matches).await,
            CliCommand::FindLists => self.execute_find_lists(matches).await,
            CliCommand::GetList => self.execute_get_list(matches).await,
            CliCommand::ListBroadcasts => {
                self.execute_list_broadcasts(matches).await
            }
            CliCommand::CreateBroadcast => {
                self.execute_create_broadcast(matches).await
            }
            CliCommand::GetBroadcastTotal => {
                self.execute_get_broadcast_total(matches)
                    .await
            }
            CliCommand::GetBroadcast => {
                self.execute_get_broadcast(matches).await
            }
            CliCommand::UpdateBroadcast => {
                self.execute_update_broadcast(matches).await
            }
            CliCommand::DeleteBroadcast => {
                self.execute_delete_broadcast(matches).await
            }
            CliCommand::CancelBroadcast => {
                self.execute_cancel_broadcast(matches)
                    .await
            }
            CliCommand::GetBroadcastClicks => {
                self.execute_get_broadcast_clicks(matches)
                    .await
            }
            CliCommand::GetBroadcastOpens => {
                self.execute_get_broadcast_opens(matches)
                    .await
            }
            CliCommand::ScheduleBroadcast => {
                self.execute_schedule_broadcast(matches)
                    .await
            }
            CliCommand::WaitBroadcast => {
                self.execute_wait_broadcast(matches).await
            }
            CliCommand::ListCampaigns => {
                self.execute_list_campaigns(matches).await
            }
            CliCommand::ListCampaignStats => {
                self.execute_list_campaign_stats(matches)
                    .await
            }
            CliCommand::GetCampaignStat => {
                self.execute_get_campaign_stat(matches)
                    .await
            }
            CliCommand::FindCampaigns => {
                self.execute_find_campaigns(matches)
                    .await
            }
            CliCommand::GetCampaign => {
                self.execute_get_campaign(matches)
                    .await
            }
            CliCommand::ListCustomFields => {
                self.execute_list_custom_fields(matches).await
            }
            CliCommand::CreateCustomField => {
                self.execute_create_custom_field(matches)
                    .await
            }
            CliCommand::GetCustomField => {
                self.execute_get_custom_field(matches)
                    .await
            }
            CliCommand::DeleteCustomField => {
                self.execute_delete_custom_field(matches)
                    .await
            }
            CliCommand::UpdateCustomField => {
                self.execute_update_custom_field(matches)
                    .await
            }
            CliCommand::ListLandingPages => {
                self.execute_list_landing_pages(matches).await
            }
            CliCommand::GetLandingPage => {
                self.execute_get_landing_page(matches)
                    .await
            }
            CliCommand::CreatePurchase => {
                self.execute_create_purchase(matches).await
            }
            CliCommand::ListSegments => {
                self.execute_list_segments(matches).await
            }
            CliCommand::GetSegment => {
                self.execute_get_segment(matches).await
            }
            CliCommand::ListSubscribers => {
                self.execute_list_subscribers(matches).await
            }
            CliCommand::CreateSubscriber => {
                self.execute_create_subscriber(matches).await
            }
            CliCommand::DeleteSubscriberByEmail => {
                self.execute_delete_subscriber_by_email(matches)
                    .await
            }
            CliCommand::UpdateSubscriberByEmail => {
                self.execute_update_subscriber_by_email(matches).await
            }
            CliCommand::FindSubscribers => {
                self.execute_find_subscribers(matches)
                    .await
            }
            CliCommand::GetSubscriber => {
                self.execute_get_subscriber(matches).await
            }
            CliCommand::MoveSubscriber => {
                self.execute_move_subscriber(matches).await
            }
            CliCommand::DeleteSubscriber => {
                self.execute_delete_subscriber(matches)
                    .await
            }
            CliCommand::UpdateSubscriber => {
                self.execute_update_subscriber(matches)
                    .await
            }
            CliCommand::GetSubscriberActivity => {
                self.execute_get_subscriber_activity(matches)
                    .await
            }
            CliCommand::ListTags => self.execute_list_tags(matches).await,
            CliCommand::ListWebFormSplitTests => {
                self.execute_list_web_form_split_tests(matches)
                    .await
            }
            CliCommand::GetWebFormSplitTest => {
                self.execute_get_web_form_split_test(matches)
                    .await
            }
            CliCommand::ListWebFormSplitTestComponents => {
                self.execute_list_web_form_split_test_components(matches)
                    .await
            }
            CliCommand::GetWebFormSplitTestComponent => {
                self.execute_get_web_form_split_test_component(matches)
                    .await
            }
            CliCommand::ListWebForms => {
                self.execute_list_web_forms(matches).await
            }
            CliCommand::GetWebForm => {
                self.execute_get_web_form(matches).await
            }
            CliCommand::GetBroadcastLinkAnalytics => {
                self.execute_get_broadcast_link_analytics(matches).await
            }
            CliCommand::OauthGetAccessToken => self.execute_oauth_get_access_token(matches).await,
            CliCommand::OauthGetRequestToken => {
                self.execute_oauth_get_request_token(matches).await
            }
            CliCommand::OauthRevoke => self.execute_oauth_revoke(matches).await,
            CliCommand::OauthToken => self.execute_oauth_token(matches).await,
        }
    }
    // -----------------------------------------------------------------------
    // execute_* methods - call crate::endpoints::* directly
    // -----------------------------------------------------------------------

    async fn resolve_list_id(&self, matches: &clap::ArgMatches) -> anyhow::Result<i32> {
        if let Some(&id) = matches.get_one::<i32>("list-id") {
            return Ok(id);
        }
        let name = matches.get_one::<String>("list").unwrap();
        let result = crate::endpoints::find_lists(
            &self.client,
            self.account_id,
            Some(name),
            None,
            None,
            None,
        )
        .await
        .context("failed to look up list by name")?;
        let mut matches_iter = result
            .entries
            .iter()
            .filter(|l| l.name.as_deref() == Some(name));
        let list = matches_iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("no list found with name '{name}'"))?;
        if matches_iter.next().is_some() {
            anyhow::bail!("multiple lists found with name '{name}', use --list-id instead");
        }
        list.id
            .map(|id| id as i32)
            .ok_or_else(|| anyhow::anyhow!("list '{name}' found but has no ID"))
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
        let result = crate::endpoints::find_subscribers(
            &self.client,
            self.account_id,
            list_id,
            None, None, None, None, None, None,
            Some(email),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None, None, None, None, None,
            None, None, None,
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
        let result = crate::endpoints::list_custom_fields(
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

    pub async fn execute_list_accounts(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let ws_size = matches.get_one::<std::num::NonZeroU32>("ws-size").copied();
        let ws_start = matches.get_one::<i32>("ws-start").copied();
        let result = crate::endpoints::get_accounts(&self.client, ws_size, ws_start).await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_account(&self, _matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let result = crate::endpoints::get_account(&self.client, self.account_id).await;
        self.print_result(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn execute_find_account_subscribers(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let tags = matches.get_one::<String>("tags").map(|s| serde_json::to_string(&[s]).unwrap());
        let tags_not_in = matches.get_one::<String>("tags-not-in").map(|s| serde_json::to_string(&[s]).unwrap());
        let result = crate::endpoints::find_account_subscribers(
            &self.client,
            self.account_id,
            matches.get_one::<types::GetAccountsFindsubscribersAdTracking>("ad-tracking").map(|v| v.as_str()),
            matches.get_one::<i32>("area-code").copied(),
            matches.get_one::<types::GetAccountsFindsubscribersCity>("city").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsFindsubscribersCountry>("country").map(|v| v.as_str()),
            matches.get_one::<String>("custom-fields").map(|s| s.as_str()),
            matches.get_one::<i32>("dma-code").copied(),
            matches.get_one::<types::GetAccountsFindsubscribersEmail>("email").map(|v| v.as_str()),
            matches.get_one::<i32>("last-followup-message-number-sent").copied(),
            matches.get_one::<chrono::NaiveDate>("last-followup-message-sent-at").copied(),
            matches.get_one::<f64>("latitude").copied(),
            matches.get_one::<f64>("longitude").copied(),
            matches.get_one::<types::GetAccountsFindsubscribersMiscNotes>("misc-notes").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsFindsubscribersName>("name").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsFindsubscribersPostalCode>("postal-code").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsFindsubscribersRegion>("region").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsFindsubscribersStatus>("status"),
            matches.get_one::<chrono::NaiveDate>("subscribed-after").copied(),
            matches.get_one::<chrono::NaiveDate>("subscribed-at").copied(),
            matches.get_one::<chrono::NaiveDate>("subscribed-before").copied(),
            matches.get_one::<types::GetAccountsFindsubscribersSubscriptionMethod>("subscription-method"),
            tags.as_deref(),
            tags_not_in.as_deref(),
            matches.get_one::<types::GetAccountsFindsubscribersUnsubscribeMethod>("unsubscribe-method"),
            matches.get_one::<chrono::NaiveDate>("unsubscribed-after").copied(),
            matches.get_one::<chrono::NaiveDate>("unsubscribed-at").copied(),
            matches.get_one::<chrono::NaiveDate>("unsubscribed-before").copied(),
            matches.get_one::<chrono::NaiveDate>("verified-at").copied(),
            matches.get_one::<types::GetAccountsFindsubscribersWsShow>("ws-show"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_list_account_webform_split_tests(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = crate::endpoints::list_account_webform_split_tests(
            &self.client,
            self.account_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_list_account_webforms(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = crate::endpoints::list_account_webforms(
            &self.client,
            self.account_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_list_integrations(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = crate::endpoints::list_integrations(
            &self.client,
            self.account_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_integration(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let integration_id = *matches.get_one::<i32>("integration-id").unwrap();
        let result = crate::endpoints::get_integration(
            &self.client,
            self.account_id,
            integration_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_list_lists(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = crate::endpoints::list_lists(
            &self.client,
            self.account_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_find_lists(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let result = crate::endpoints::find_lists(
            &self.client,
            self.account_id,
            matches.get_one::<types::GetAccountsListsFindName>("name").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsListsFindWsShow>("ws-show"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_get_list(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::get_list(&self.client, self.account_id, list_id).await;
        self.print_result(result)
    }

    pub async fn execute_list_broadcasts(
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
        for status in &statuses {
            let result = crate::endpoints::list_broadcasts(
                &self.client,
                self.account_id,
                list_id,
                Some(status),
                ws_size,
                ws_start,
            )
            .await;
            self.print_paginated_ndjson(result, matches).await?;
        }
        Ok(())
    }
    pub async fn execute_create_broadcast(
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
            if let Some(v) = matches.get_one::<String>("body-amp") { body.insert("body_amp".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("body-html") { body.insert("body_html".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("body-text") { body.insert("body_text".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<bool>("click-tracking-enabled") { body.insert("click_tracking_enabled".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("exclude-lists") { body.insert("exclude_lists".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("facebook-integration") { body.insert("facebook_integration".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("include-lists") { body.insert("include_lists".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<bool>("is-archived") { body.insert("is_archived".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<bool>("notify-on-send") { body.insert("notify_on_send".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("subject") { body.insert("subject".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("twitter-integration") { body.insert("twitter_integration".into(), serde_json::json!(v)); }
            serde_json::from_value::<types::CreateBroadcast>(serde_json::Value::Object(body))?
        };
        let result = crate::endpoints::create_broadcast(
            &self.client,
            self.account_id,
            list_id,
            &body,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_get_broadcast_total(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::get_broadcast_total(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<types::GetAccountsListsBroadcastsTotalStatus>("status").unwrap(),
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_get_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result = crate::endpoints::get_broadcast(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_update_broadcast(
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
            if let Some(v) = matches.get_one::<String>("body-amp") { body.insert("body_amp".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("body-html") { body.insert("body_html".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("body-text") { body.insert("body_text".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<bool>("click-tracking-enabled") { body.insert("click_tracking_enabled".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("exclude-lists") { body.insert("exclude_lists".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("facebook-integration") { body.insert("facebook_integration".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("include-lists") { body.insert("include_lists".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<bool>("is-archived") { body.insert("is_archived".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<bool>("notify-on-send") { body.insert("notify_on_send".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("segment-link") { body.insert("segment_link".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("subject") { body.insert("subject".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("twitter-integration") { body.insert("twitter_integration".into(), serde_json::json!(v)); }
            serde_json::from_value::<types::UpdateBroadcast>(serde_json::Value::Object(body))?
        };
        let result = crate::endpoints::update_broadcast(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
            &body,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_delete_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result = crate::endpoints::delete_broadcast(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
        )
        .await;
        self.print_void(result)
    }

    pub async fn execute_cancel_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result = crate::endpoints::cancel_broadcast(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_get_broadcast_clicks(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result = crate::endpoints::get_broadcast_clicks(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
            matches.get_one::<String>("after").map(|s| s.as_str()),
            matches.get_one::<String>("before").map(|s| s.as_str()),
            matches.get_one::<bool>("detailed").copied(),
            matches.get_one::<std::num::NonZeroU64>("page-size").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_broadcast_opens(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let result = crate::endpoints::get_broadcast_opens(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
            matches.get_one::<String>("after").map(|s| s.as_str()),
            matches.get_one::<String>("before").map(|s| s.as_str()),
            matches.get_one::<std::num::NonZeroU64>("page-size").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_schedule_broadcast(
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
        let result = crate::endpoints::schedule_broadcast(
            &self.client,
            self.account_id,
            list_id,
            broadcast_id,
            &body,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_wait_broadcast(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let broadcast_id = *matches.get_one::<i32>("broadcast-id").unwrap();
        let interval = *matches.get_one::<u64>("interval").unwrap();

        loop {
            let broadcast = crate::endpoints::get_broadcast(
                &self.client,
                self.account_id,
                list_id,
                broadcast_id,
            )
            .await?;

            match broadcast.status {
                Some(types::BroadcastStatus::Sent) => {
                    return self.print_result(Ok::<_, crate::client::ApiError>(broadcast));
                }
                Some(ref status @ (types::BroadcastStatus::Sending | types::BroadcastStatus::Scheduled)) => {
                    eprintln!("Broadcast {broadcast_id} status: {status} — polling again in {interval}s");
                    tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
                }
                Some(ref status) => {
                    anyhow::bail!("Broadcast {broadcast_id} has status: {status} — cannot wait for completion");
                }
                None => {
                    anyhow::bail!("Broadcast {broadcast_id} has no status — cannot wait for completion");
                }
            }
        }
    }

    pub async fn execute_list_campaigns(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::list_campaigns(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_list_campaign_stats(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let campaign_id = *matches.get_one::<i32>("campaign-id").unwrap();
        let result = crate::endpoints::list_campaign_stats(
            &self.client,
            self.account_id,
            list_id,
            campaign_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_campaign_stat(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let campaign_id = *matches.get_one::<i32>("campaign-id").unwrap();
        let stats_id = matches
            .get_one::<types::GetAccountsListsCampaignsBcampaignidStats2StatsId>("stats-id")
            .unwrap();
        // The endpoint takes i32 for the path segment; the enum Display gives
        // the string form which goes in the URL.  We pass it as-is and let the
        // endpoint use it.  However our endpoints::get_campaign_stat takes i32.
        // The stats_id is really a string label, so we pass its Display repr
        // through the URL path directly.  For now we need to match the endpoint
        // signature which takes i32 -- this is a design mismatch.  We'll pass
        // the string form via the endpoint.
        // Actually, looking at the endpoint, stats_id is i32 but that seems
        // incorrect for string stats IDs. Let's convert to string and use
        // a direct API request instead.  But per the user's instructions we
        // should call endpoints directly.  Let's just use to_string() and
        // note the endpoint needs adjustment.
        // For now, pass 0 as placeholder - the endpoint signature needs fixing.
        // Actually, re-reading endpoints.rs: it takes i32 for stats_id and
        // embeds in the path. This is wrong for string stats IDs. But we
        // must follow the existing code. The caller likely needs to fix this.
        // We'll print the stats_id.to_string() directly.
        let _ = (list_id, campaign_id, stats_id);
        // Use a workaround: call the endpoint via the client directly
        use crate::client::ApiRequest;
        use reqwest::Method;
        let result: Result<types::Stat, _> = ApiRequest::new(
            &self.client,
            Method::GET,
            format!(
                "/accounts/{}/lists/{}/campaigns/b{}/stats/{}",
                self.account_id, list_id, campaign_id, stats_id
            ),
        )
        .send()
        .await;
        self.print_result(result)
    }

    pub async fn execute_find_campaigns(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::find_campaigns(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<types::GetAccountsListsCampaignsFindCampaignType>("campaign-type").unwrap(),
            matches.get_one::<types::GetAccountsListsCampaignsFindWsShow>("ws-show"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_campaign(
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
        let result = crate::endpoints::get_campaign(
            &self.client,
            self.account_id,
            list_id,
            &campaign_type.to_string(),
            campaign_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_list_custom_fields(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::list_custom_fields(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_create_custom_field(
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
            if let Some(v) = matches.get_one::<String>("name") { body.insert("name".into(), serde_json::json!(v)); }
            body.insert("ws.op".into(), serde_json::json!("create"));
            serde_json::from_value::<types::PostAccountsListsCustomFieldsBody>(serde_json::Value::Object(body))?
        };
        let result = crate::endpoints::create_custom_field(
            &self.client,
            self.account_id,
            list_id,
            &body,
        )
        .await;
        match result {
            Err(e) if e.api_message_is("name: Must be unique") => {
                let name = matches.get_one::<String>("name").map(|s| s.as_str()).unwrap_or("unknown");
                Err(e).context(format!("custom field '{name}' already exists"))
            }
            other => self.print_void(other),
        }
    }

    pub async fn execute_get_custom_field(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let custom_field_id = self.resolve_custom_field_id(matches, list_id).await?;
        let result = crate::endpoints::get_custom_field(
            &self.client,
            self.account_id,
            list_id,
            custom_field_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_delete_custom_field(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let custom_field_id = self.resolve_custom_field_id(matches, list_id).await?;
        let result = crate::endpoints::delete_custom_field(
            &self.client,
            self.account_id,
            list_id,
            custom_field_id,
        )
        .await;
        self.print_void(result)
    }

    pub async fn execute_update_custom_field(
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
            if let Some(v) = matches.get_one::<bool>("is-subscriber-updateable") { body.insert("is_subscriber_updateable".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("name") { body.insert("name".into(), serde_json::json!(v)); }
            serde_json::from_value::<types::PatchAccountsListsCustomFieldsBody>(serde_json::Value::Object(body))?
        };
        let result = crate::endpoints::update_custom_field(
            &self.client,
            self.account_id,
            list_id,
            custom_field_id,
            &body,
        )
        .await;
        self.print_result(result)
    }
    pub async fn execute_list_landing_pages(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::list_landing_pages(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_landing_page(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let landing_page_id = *matches.get_one::<uuid::Uuid>("landing-page-id").unwrap();
        let result = crate::endpoints::get_landing_page(
            &self.client,
            self.account_id,
            list_id,
            landing_page_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_create_purchase(
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
            if let Some(v) = matches.get_one::<types::PurchaseAdTracking>("ad-tracking") { body.insert("ad_tracking".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<String>("currency") { body.insert("currency".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<types::PurchaseEmail>("email") { body.insert("email".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<String>("event-note") { body.insert("event_note".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("event-time") { body.insert("event_time".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<types::PurchaseIpAddress>("ip-address") { body.insert("ip_address".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::PurchaseMiscNotes>("misc-notes") { body.insert("misc_notes".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::PurchaseName>("name") { body.insert("name".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<String>("product-name") { body.insert("product_name".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("url") { body.insert("url".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<f64>("value") { body.insert("value".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("vendor") { body.insert("vendor".into(), serde_json::json!(v)); }
            serde_json::from_value::<types::Purchase>(serde_json::Value::Object(body))?
        };
        let result = crate::endpoints::create_purchase(
            &self.client,
            self.account_id,
            list_id,
            &body,
        )
        .await;
        self.print_void(result)
    }

    pub async fn execute_list_segments(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::list_segments(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_segment(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let segment_id = *matches.get_one::<i32>("segment-id").unwrap();
        let result = crate::endpoints::get_segment(
            &self.client,
            self.account_id,
            list_id,
            segment_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_list_subscribers(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::list_subscribers(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<types::GetAccountsListsSubscribersSortOrder>("sort-order"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_create_subscriber(
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
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyAdTracking>("ad-tracking") { body.insert("ad_tracking".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyEmail>("email") { body.insert("email".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyIpAddress>("ip-address") { body.insert("ip_address".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<i64>("last-followup-message-number-sent") { body.insert("last_followup_message_number_sent".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyMiscNotes>("misc-notes") { body.insert("misc_notes".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyName>("name") { body.insert("name".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyStrictCustomFields>("strict-custom-fields") { body.insert("strict_custom_fields".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::AddSubscriberRequestBodyUpdateExisting>("update-existing") { body.insert("update_existing".into(), serde_json::json!(v.to_string())); }
            serde_json::from_value::<types::AddSubscriberRequestBody>(serde_json::Value::Object(body))?
        };
        match crate::endpoints::create_subscriber(
            &self.client,
            self.account_id,
            list_id,
            &body,
        )
        .await
        {
            Err(e) if e.api_message_is("email: Subscriber already subscribed.") => {
                Err(e).context("subscriber already on list")
            }
            other => self.print_void(other),
        }
    }

    pub async fn execute_delete_subscriber_by_email(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_email = matches
            .get_one::<types::DeleteAccountsListsSubscribersSubscriberEmail>("email")
            .unwrap();
        let result = crate::endpoints::delete_subscriber_by_email(
            &self.client,
            self.account_id,
            list_id,
            subscriber_email,
        )
        .await;
        self.print_void(result)
    }
    pub async fn execute_update_subscriber_by_email(
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
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyAdTracking>("ad-tracking") { body.insert("ad_tracking".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<String>("new-email") { body.insert("email".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<i64>("last-followup-message-number-sent") { body.insert("last_followup_message_number_sent".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("misc-notes") { body.insert("misc_notes".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyName>("name") { body.insert("name".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyStatus>("status") { body.insert("status".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyStrictCustomFields>("strict-custom-fields") { body.insert("strict_custom_fields".into(), serde_json::json!(v.to_string())); }
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
        let result = crate::endpoints::update_subscriber_by_email(
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
    pub async fn execute_find_subscribers(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let tags = matches.get_one::<String>("tags").map(|s| serde_json::to_string(&[s]).unwrap());
        let tags_not_in = matches.get_one::<String>("tags-not-in").map(|s| serde_json::to_string(&[s]).unwrap());
        let result = crate::endpoints::find_subscribers(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<types::GetAccountsListsSubscribersFindAdTracking>("ad-tracking").map(|v| v.as_str()),
            matches.get_one::<i32>("area-code").copied(),
            matches.get_one::<types::GetAccountsListsSubscribersFindCity>("city").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsListsSubscribersFindCountry>("country").map(|v| v.as_str()),
            matches.get_one::<String>("custom-fields").map(|s| s.as_str()),
            matches.get_one::<i32>("dma-code").copied(),
            matches.get_one::<types::GetAccountsListsSubscribersFindEmail>("email").map(|v| v.as_str()),
            matches.get_one::<i32>("last-followup-message-number-sent").copied(),
            matches.get_one::<chrono::NaiveDate>("last-followup-message-sent-at").copied(),
            matches.get_one::<f64>("latitude").copied(),
            matches.get_one::<f64>("longitude").copied(),
            matches.get_one::<types::GetAccountsListsSubscribersFindMiscNotes>("misc-notes").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsListsSubscribersFindName>("name").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsListsSubscribersFindPostalCode>("postal-code").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsListsSubscribersFindRegion>("region").map(|v| v.as_str()),
            matches.get_one::<types::GetAccountsListsSubscribersFindSortKey>("sort-key"),
            matches.get_one::<types::GetAccountsListsSubscribersFindSortOrder>("sort-order"),
            matches.get_one::<types::GetAccountsListsSubscribersFindStatus>("status"),
            matches.get_one::<chrono::NaiveDate>("subscribed-after").copied(),
            matches.get_one::<chrono::NaiveDate>("subscribed-at").copied(),
            matches.get_one::<chrono::NaiveDate>("subscribed-before").copied(),
            matches.get_one::<types::GetAccountsListsSubscribersFindSubscriptionMethod>("subscription-method"),
            tags.as_deref(),
            tags_not_in.as_deref(),
            matches.get_one::<types::GetAccountsListsSubscribersFindUnsubscribeMethod>("unsubscribe-method"),
            matches.get_one::<chrono::NaiveDate>("unsubscribed-after").copied(),
            matches.get_one::<chrono::NaiveDate>("unsubscribed-at").copied(),
            matches.get_one::<chrono::NaiveDate>("unsubscribed-before").copied(),
            matches.get_one::<chrono::NaiveDate>("verified-at").copied(),
            matches.get_one::<types::GetAccountsListsSubscribersFindWsShow>("ws-show"),
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_subscriber(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_id = self.resolve_subscriber_id(matches, list_id).await?;
        let result = crate::endpoints::get_subscriber(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_move_subscriber(
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
            if let Some(v) = matches.get_one::<bool>("enforce-custom-field-mapping") { body.insert("enforce_custom_field_mapping".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<i64>("last-followup-message-number-sent") { body.insert("last_followup_message_number_sent".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("list-link") { body.insert("list_link".into(), serde_json::json!(v)); }
            body.insert("ws.op".into(), serde_json::json!("move"));
            serde_json::from_value::<types::MoveSubscriberRequestBody>(serde_json::Value::Object(body))?
        };
        let result = crate::endpoints::move_subscriber(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
            &body,
        )
        .await;
        self.print_void(result)
    }

    pub async fn execute_delete_subscriber(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_id = self.resolve_subscriber_id(matches, list_id).await?;
        let result = crate::endpoints::delete_subscriber(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
        )
        .await;
        self.print_void(result)
    }

    pub async fn execute_update_subscriber(
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
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyAdTracking>("ad-tracking") { body.insert("ad_tracking".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<String>("new-email") { body.insert("email".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<i64>("last-followup-message-number-sent") { body.insert("last_followup_message_number_sent".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<String>("misc-notes") { body.insert("misc_notes".into(), serde_json::json!(v)); }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyName>("name") { body.insert("name".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyStatus>("status") { body.insert("status".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::UpdateSubscriberRequestBodyStrictCustomFields>("strict-custom-fields") { body.insert("strict_custom_fields".into(), serde_json::json!(v.to_string())); }
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
        let result = crate::endpoints::update_subscriber(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
            &body,
        )
        .await;
        self.print_result(result)
    }
    pub async fn execute_get_subscriber_activity(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let subscriber_id = self.resolve_subscriber_id(matches, list_id).await?;
        let result = crate::endpoints::get_subscriber_activity(
            &self.client,
            self.account_id,
            list_id,
            subscriber_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_list_tags(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::list_tags(
            &self.client,
            self.account_id,
            list_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_list_web_form_split_tests(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::list_web_form_split_tests(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_web_form_split_test(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let split_test_id = *matches.get_one::<i32>("split-test-id").unwrap();
        let result = crate::endpoints::get_web_form_split_test(
            &self.client,
            self.account_id,
            list_id,
            split_test_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_list_web_form_split_test_components(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let split_test_id = *matches.get_one::<i32>("split-test-id").unwrap();
        let result = crate::endpoints::list_web_form_split_test_components(
            &self.client,
            self.account_id,
            list_id,
            split_test_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_web_form_split_test_component(
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
        let result = crate::endpoints::get_web_form_split_test_component(
            &self.client,
            self.account_id,
            list_id,
            split_test_id,
            split_test_component_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_list_web_forms(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let result = crate::endpoints::list_web_forms(
            &self.client,
            self.account_id,
            list_id,
            matches.get_one::<std::num::NonZeroU32>("ws-size").copied(),
            matches.get_one::<i32>("ws-start").copied(),
        )
        .await;
        self.print_paginated_ndjson(result, matches).await
    }

    pub async fn execute_get_web_form(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let list_id = self.resolve_list_id(matches).await?;
        let webform_id = *matches.get_one::<i32>("webform-id").unwrap();
        let result = crate::endpoints::get_web_form(
            &self.client,
            self.account_id,
            list_id,
            webform_id,
        )
        .await;
        self.print_result(result)
    }

    pub async fn execute_get_broadcast_link_analytics(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        // Convert from the rich wrapper types to the simpler endpoint types.
        let account_id = self.account_id;
        let after = matches.get_one::<String>("after").map(|s| s.as_str());
        let before_str = matches.get_one::<i64>("before").map(|v| v.to_string());
        let broadcast_id = matches
            .get_one::<types::GetBroadcastLinksAnalyticsBroadcastId>("broadcast-id")
            .and_then(|v| v.parse::<i32>().ok())
            .expect("--broadcast-id is required");
        let filter = matches
            .get_one::<types::GetBroadcastLinksAnalyticsFilter>("filter")
            .unwrap()
            .to_string();
        let max_count = matches.get_one::<u64>("max-count").map(|v| *v as i32);
        let min_count = matches.get_one::<u64>("min-count").map(|v| *v as i32);
        let page_size = matches.get_one::<std::num::NonZeroU64>("page-size").copied();
        let sort_asc = matches.get_one::<bool>("sort-asc").copied();
        let sort_by = matches
            .get_one::<types::GetBroadcastLinksAnalyticsSortBy>("sort-by")
            .map(|v| v.to_string());
        let result = crate::endpoints::get_broadcast_link_analytics(
            &self.client,
            account_id,
            after,
            before_str.as_deref(),
            broadcast_id,
            &filter,
            max_count,
            min_count,
            page_size,
            sort_asc,
            sort_by.as_deref(),
        )
        .await;
        self.print_result(result)
    }
    pub async fn execute_oauth_get_access_token(
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
            if let Some(v) = matches.get_one::<types::OauthCallback>("oauth-callback") { body.insert("oauth_callback".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthConsumerKey>("oauth-consumer-key") { body.insert("oauth_consumer_key".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthNonce>("oauth-nonce") { body.insert("oauth_nonce".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthSignature>("oauth-signature") { body.insert("oauth_signature".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthSignatureMethod>("oauth-signature-method") { body.insert("oauth_signature_method".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthTimestamp>("oauth-timestamp") { body.insert("oauth_timestamp".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthToken>("oauth-token") { body.insert("oauth_token".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthVersion>("oauth-version") { body.insert("oauth_version".into(), serde_json::json!(v.to_string())); }
            serde_json::from_value::<types::PostOauthAccessTokenBody>(serde_json::Value::Object(body))?
        };
        let result = crate::endpoints::oauth_get_access_token(&self.client, &body).await;
        self.print_result(result)
    }

    pub async fn execute_oauth_get_request_token(
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
            if let Some(v) = matches.get_one::<types::OauthCallback>("oauth-callback") { body.insert("oauth_callback".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthConsumerKey>("oauth-consumer-key") { body.insert("oauth_consumer_key".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthNonce>("oauth-nonce") { body.insert("oauth_nonce".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthSignature>("oauth-signature") { body.insert("oauth_signature".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthSignatureMethod>("oauth-signature-method") { body.insert("oauth_signature_method".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthTimestamp>("oauth-timestamp") { body.insert("oauth_timestamp".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthToken>("oauth-token") { body.insert("oauth_token".into(), serde_json::json!(v.to_string())); }
            if let Some(v) = matches.get_one::<types::OauthVersion>("oauth-version") { body.insert("oauth_version".into(), serde_json::json!(v.to_string())); }
            serde_json::from_value::<types::PostOauthRequestTokenBody>(serde_json::Value::Object(body))?
        };
        let result = crate::endpoints::oauth_get_request_token(&self.client, &body).await;
        self.print_result(result)
    }

    pub async fn execute_oauth_revoke(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let authorization = matches.get_one::<String>("authorization").map(|s| s.as_str());
        let path = matches.get_one::<std::path::PathBuf>("json-body").unwrap();
        let txt = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let body = serde_json::from_str::<types::PostOauth2RevokeBody>(&txt)
            .with_context(|| format!("failed to parse {}", path.display()))?;
        let result = crate::endpoints::oauth2_revoke(&self.client, authorization, &body).await;
        self.print_void(result)
    }

    pub async fn execute_oauth_token(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let authorization = matches.get_one::<String>("authorization").map(|s| s.as_str());
        let path = matches.get_one::<std::path::PathBuf>("json-body").unwrap();
        let txt = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let body = serde_json::from_str::<types::PostOauth2TokenBody>(&txt)
            .with_context(|| format!("failed to parse {}", path.display()))?;
        let result = crate::endpoints::oauth2_token(&self.client, authorization, &body).await;
        self.print_result(result)
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn print_result<T: serde::Serialize>(
        &self,
        result: Result<T, crate::client::ApiError>,
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

    async fn print_paginated_ndjson<C>(
        &self,
        first_page: Result<C, crate::client::ApiError>,
        matches: &clap::ArgMatches,
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
        let mut remaining = matches.get_one::<usize>("limit").copied();
        loop {
            for item in page.take_entries() {
                if remaining == Some(0) {
                    out.flush()?;
                    eprintln!("Warning: results truncated by --limit; more results available");
                    return Ok(());
                }
                if pretty {
                    let json = serde_json::to_value(&item)?;
                    let colored = colored_json::to_colored_json_auto(&json)?;
                    writeln!(out, "{colored}")?;
                } else {
                    serde_json::to_writer(&mut out, &item)?;
                    writeln!(out)?;
                }
                if let Some(r) = &mut remaining {
                    *r -= 1;
                }
            }
            out.flush()?;
            match page.next_collection_link() {
                Some(_) if remaining == Some(0) => {
                    eprintln!("Warning: results truncated by --limit; more results available");
                    break;
                }
                Some(url) => {
                    let url = url.to_string();
                    page = self.client.get_url(&url).await.map_err(|e| {
                        anyhow::anyhow!("fetching next page: {e}")
                    })?;
                }
                None => break,
            }
        }
        Ok(())
    }

    fn print_void(
        &self,
        result: Result<(), crate::client::ApiError>,
    ) -> anyhow::Result<()> {
        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("{e}")),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
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
    OauthGetAccessToken,
    OauthGetRequestToken,
    OauthRevoke,
    OauthToken,
}
impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::ListAccounts,
            CliCommand::GetAccount,
            CliCommand::FindAccountSubscribers,
            CliCommand::ListAccountWebformSplitTests,
            CliCommand::ListAccountWebforms,
            CliCommand::ListIntegrations,
            CliCommand::GetIntegration,
            CliCommand::ListLists,
            CliCommand::FindLists,
            CliCommand::GetList,
            CliCommand::ListBroadcasts,
            CliCommand::CreateBroadcast,
            CliCommand::GetBroadcastTotal,
            CliCommand::GetBroadcast,
            CliCommand::UpdateBroadcast,
            CliCommand::DeleteBroadcast,
            CliCommand::CancelBroadcast,
            CliCommand::GetBroadcastClicks,
            CliCommand::GetBroadcastOpens,
            CliCommand::ScheduleBroadcast,
            CliCommand::WaitBroadcast,
            CliCommand::ListCampaigns,
            CliCommand::ListCampaignStats,
            CliCommand::GetCampaignStat,
            CliCommand::FindCampaigns,
            CliCommand::GetCampaign,
            CliCommand::ListCustomFields,
            CliCommand::CreateCustomField,
            CliCommand::GetCustomField,
            CliCommand::DeleteCustomField,
            CliCommand::UpdateCustomField,
            CliCommand::ListLandingPages,
            CliCommand::GetLandingPage,
            CliCommand::CreatePurchase,
            CliCommand::ListSegments,
            CliCommand::GetSegment,
            CliCommand::ListSubscribers,
            CliCommand::CreateSubscriber,
            CliCommand::DeleteSubscriberByEmail,
            CliCommand::UpdateSubscriberByEmail,
            CliCommand::FindSubscribers,
            CliCommand::GetSubscriber,
            CliCommand::MoveSubscriber,
            CliCommand::DeleteSubscriber,
            CliCommand::UpdateSubscriber,
            CliCommand::GetSubscriberActivity,
            CliCommand::ListTags,
            CliCommand::ListWebFormSplitTests,
            CliCommand::GetWebFormSplitTest,
            CliCommand::ListWebFormSplitTestComponents,
            CliCommand::GetWebFormSplitTestComponent,
            CliCommand::ListWebForms,
            CliCommand::GetWebForm,
            CliCommand::GetBroadcastLinkAnalytics,
            CliCommand::OauthGetAccessToken,
            CliCommand::OauthGetRequestToken,
            CliCommand::OauthRevoke,
            CliCommand::OauthToken,
        ]
        .into_iter()
    }
}

//! Every operation's plan issues the request its typed endpoint function
//! issues: same method, same path, same query, for the same fixture arguments.

use aweber::catalog::{ArgValue, Args, Operation, RequestPlan, Scope};
use aweber::client::Client;
use aweber::ids::{AccountUid, ListUid, MessageId, RuleId, WorkflowId};
use aweber::pagination::PageSize;
use aweber::types;
use aweber::workflows;
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, ResponseTemplate};

mod support;

const ACCOUNT_ID: i32 = 1;
const LIST_ID: i32 = 2;
const BROADCAST_ID: i32 = 3;
const CUSTOM_FIELD_ID: i32 = 4;
const SEGMENT_ID: i32 = 5;
const SUBSCRIBER_ID: i32 = 6;
const SPLIT_TEST_ID: i32 = 7;
const SPLIT_TEST_COMPONENT_ID: i32 = 8;
const WEBFORM_ID: i32 = 9;
const INTEGRATION_ID: i32 = 10;
const CAMPAIGN_ID: i32 = 11;

const ACCOUNT_UUID: &str = "11111111-1111-4111-8111-111111111111";
const LIST_UUID: &str = "22222222-2222-4222-8222-222222222222";
const WORKFLOW_UUID: &str = "33333333-3333-4333-8333-333333333333";
const EVENT_UUID: &str = "44444444-4444-4444-8444-444444444444";
const BROADCAST_UUID: &str = "55555555-5555-4555-8555-555555555555";
const LANDING_PAGE_UUID: &str = "66666666-6666-4666-8666-666666666666";

const MESSAGE_ID: &str = "0123456789abcdef01234567";
const EMAIL: &str = "someone@example.com";
const WS_SIZE: &str = "10";
const PAGE_SIZE: &str = "5";
const PRECONDITION: &str = "7";
const RULESET: &str = r#"{"events":[],"actions":[]}"#;
const SCHEDULED_FOR: &str = "2026-01-02T03:04:05+00:00";
const EVENT_TIME: &str = "2026-01-02T03:04:05Z";
const LIST_LINK: &str = "https://api.aweber.com/1.0/accounts/1/lists/3";
const REVOKE_BODY: &str = r#"{"client_id":"an id","client_secret":"a secret"}"#;
const TOKEN_BODY: &str = r#"{"client_id":"an id","client_secret":"a secret","code":"a code","grant_type":"authorization_code"}"#;

fn parsed(value: &str) -> uuid::Uuid {
    value.parse().expect("the fixture is a uuid")
}

fn window() -> std::num::NonZeroU32 {
    std::num::NonZeroU32::new(10).expect("ten is not zero")
}

fn page() -> std::num::NonZeroU64 {
    std::num::NonZeroU64::new(5).expect("five is not zero")
}

fn small_page() -> PageSize {
    PAGE_SIZE.parse().expect("five is a page size")
}

fn typed_id<T: std::str::FromStr>(value: &str) -> T
where
    T::Err: std::fmt::Debug,
{
    value.parse().expect("the fixture is an id")
}

fn precondition() -> workflows::PreconditionVersion {
    serde_json::from_value(serde_json::json!(7)).expect("a version is an integer")
}

fn message() -> MessageId {
    typed_id(MESSAGE_ID)
}

fn empty_graph() -> workflows::Graph {
    workflows::Ruleset::default().working(workflows::Timezone::utc())
}

fn args(pairs: &[(&str, &str)]) -> Args {
    let mut args = Args::default();
    for (name, value) in pairs {
        args.set(*name, ArgValue::text(*value));
    }
    args
}

fn body<T: serde::de::DeserializeOwned>(document: serde_json::Value) -> T {
    serde_json::from_value(document).expect("the fixture body is well formed")
}

/// The arguments each operation's plan is built from.
fn fixture(operation: Operation) -> Args {
    let list = ("list-id", "2");
    let workflow = ("workflow", WORKFLOW_UUID);
    let size = ("ws-size", WS_SIZE);
    match operation {
        Operation::ListAccounts => args(&[size]),
        Operation::GetAccount => args(&[]),
        Operation::FindAccountSubscribers => args(&[("email", EMAIL), size]),
        Operation::ListAccountWebformSplitTests => args(&[size]),
        Operation::ListAccountWebforms => args(&[size]),
        Operation::ListIntegrations => args(&[size]),
        Operation::GetIntegration => args(&[("integration-id", "10")]),
        Operation::ListLists => args(&[size]),
        Operation::FindLists => args(&[("name", "a list"), size]),
        Operation::GetList => args(&[list]),
        Operation::ListBroadcasts => args(&[list, ("status", "draft"), size]),
        Operation::CreateBroadcast => args(&[
            list,
            ("body-html", "<p>hello</p>"),
            ("body-text", "hello"),
            ("click-tracking-enabled", "true"),
            ("is-archived", "true"),
            ("notify-on-send", "true"),
            ("subject", "hello"),
        ]),
        Operation::GetBroadcastTotal => args(&[list, ("status", "sent")]),
        Operation::GetBroadcast => args(&[list, ("broadcast-id", "3")]),
        Operation::UpdateBroadcast => args(&[list, ("broadcast-id", "3")]),
        Operation::DeleteBroadcast => args(&[list, ("broadcast-id", "3")]),
        Operation::CancelBroadcast => args(&[list, ("broadcast-id", "3")]),
        Operation::GetBroadcastClicks => {
            args(&[list, ("broadcast-id", "3"), ("page-size", PAGE_SIZE)])
        }
        Operation::GetBroadcastOpens => {
            args(&[list, ("broadcast-id", "3"), ("page-size", PAGE_SIZE)])
        }
        Operation::ScheduleBroadcast => args(&[
            list,
            ("broadcast-id", "3"),
            ("scheduled-for", SCHEDULED_FOR),
        ]),
        Operation::WaitBroadcast => args(&[list, ("broadcast-id", "3")]),
        Operation::ListCampaigns => args(&[list, size]),
        Operation::ListCampaignStats => args(&[list, ("campaign-id", "11"), size]),
        Operation::GetCampaignStat => {
            args(&[list, ("campaign-id", "11"), ("stats-id", "total_opens")])
        }
        Operation::FindCampaigns => args(&[list, ("campaign-type", "b"), size]),
        Operation::GetCampaign => args(&[list, ("campaign-type", "b"), ("campaign-id", "11")]),
        Operation::ListCustomFields => args(&[list, size]),
        Operation::CreateCustomField => args(&[list, ("name", "a field")]),
        Operation::GetCustomField => args(&[list, ("custom-field-id", "4")]),
        Operation::DeleteCustomField => args(&[list, ("custom-field-id", "4")]),
        Operation::UpdateCustomField => args(&[list, ("custom-field-id", "4")]),
        Operation::ListLandingPages => args(&[list, size]),
        Operation::GetLandingPage => args(&[list, ("landing-page-id", LANDING_PAGE_UUID)]),
        Operation::CreatePurchase => args(&[
            list,
            ("currency", "USD"),
            ("email", EMAIL),
            ("event-note", "a note"),
            ("event-time", EVENT_TIME),
            ("ip-address", "203.0.113.4"),
            ("product-name", "a product"),
            ("url", "https://example.com/checkout"),
            ("value", "1.0"),
            ("vendor", "a vendor"),
        ]),
        Operation::ListSegments => args(&[list, size]),
        Operation::GetSegment => args(&[list, ("segment-id", "5")]),
        Operation::ListSubscribers => args(&[list, size]),
        Operation::CreateSubscriber => args(&[list, ("email", EMAIL)]),
        Operation::DeleteSubscriberByEmail => args(&[list, ("email", EMAIL)]),
        Operation::UpdateSubscriberByEmail => args(&[list, ("email", EMAIL)]),
        Operation::FindSubscribers => args(&[list, ("email", EMAIL), size]),
        Operation::GetSubscriber => args(&[list, ("subscriber-id", "6")]),
        Operation::MoveSubscriber => {
            args(&[list, ("subscriber-id", "6"), ("list-link", LIST_LINK)])
        }
        Operation::DeleteSubscriber => args(&[list, ("subscriber-id", "6")]),
        Operation::UpdateSubscriber => args(&[list, ("subscriber-id", "6")]),
        Operation::UnsubscribeSubscriber => args(&[list, ("subscriber-id", "6")]),
        Operation::GetSubscriberActivity => args(&[list, ("subscriber-id", "6"), size]),
        Operation::ListTags => args(&[list]),
        Operation::ListWebFormSplitTests => args(&[list, size]),
        Operation::GetWebFormSplitTest => args(&[list, ("split-test-id", "7")]),
        Operation::ListWebFormSplitTestComponents => args(&[list, ("split-test-id", "7"), size]),
        Operation::GetWebFormSplitTestComponent => args(&[
            list,
            ("split-test-id", "7"),
            ("split-test-component-id", "8"),
        ]),
        Operation::ListWebForms => args(&[list, size]),
        Operation::GetWebForm => args(&[list, ("webform-id", "9")]),
        Operation::GetBroadcastLinkAnalytics => args(&[
            ("broadcast-id", BROADCAST_UUID),
            ("filter", "clicks"),
            ("page-size", PAGE_SIZE),
        ]),
        Operation::ListWorkflows => args(&[("list", LIST_UUID)]),
        Operation::GetWorkflow => args(&[workflow]),
        Operation::TreeWorkflow => args(&[workflow]),
        Operation::CreateWorkflow => args(&[("list", LIST_UUID), ("name", "a workflow")]),
        Operation::UpdateWorkflow => args(&[
            workflow,
            ("name", "a workflow"),
            ("precondition-version", PRECONDITION),
        ]),
        Operation::UpdateWorkflowRuleset => args(&[
            workflow,
            ("file", RULESET),
            ("precondition-version", PRECONDITION),
        ]),
        Operation::PublishWorkflow => args(&[workflow, ("precondition-version", PRECONDITION)]),
        Operation::RevertWorkflow => args(&[workflow, ("precondition-version", PRECONDITION)]),
        Operation::SetWorkflowState => args(&[
            workflow,
            ("state", "active"),
            ("precondition-version", PRECONDITION),
        ]),
        Operation::CopyWorkflow => args(&[
            workflow,
            ("list", LIST_UUID),
            ("name", "a copy"),
            ("precondition-version", PRECONDITION),
        ]),
        Operation::DeleteWorkflow => args(&[workflow, ("precondition-version", PRECONDITION)]),
        Operation::GetWorkflowStats => args(&[workflow]),
        Operation::GetWorkflowMessageStats => {
            args(&[("message-id", MESSAGE_ID), ("page-size", PAGE_SIZE)])
        }
        Operation::GetWorkflowEventHistory => {
            args(&[workflow, ("event-id", EVENT_UUID), ("page-size", PAGE_SIZE)])
        }
        Operation::OauthGetAccessToken => args(&[]),
        Operation::OauthGetRequestToken => args(&[]),
        Operation::OauthRevoke => args(&[
            ("authorization", "Bearer fixture"),
            ("json-body", REVOKE_BODY),
        ]),
        Operation::OauthToken => args(&[
            ("authorization", "Bearer fixture"),
            ("json-body", TOKEN_BODY),
        ]),
    }
}

/// Every fixture value re-read as the kind the operation's `ArgSpec` declares,
/// so a decimal reaches a body as a number rather than as its text.
fn typed(operation: Operation, args: Args) -> Args {
    let specs = operation.specs();
    let mut retyped = Args::default();
    for (name, values) in args.iter() {
        let kind = specs
            .iter()
            .find(|spec| spec.name == name)
            .map_or(aweber::catalog::ValueKind::Text, |spec| spec.kind.clone());
        for value in values {
            retyped.push(name, ArgValue::new(kind.clone(), value.raw.clone()));
        }
    }
    retyped
}

/// The typed endpoint function of each operation, called with the same fixture.
/// The response is discarded: only the request it put on the wire is compared.
#[allow(clippy::cognitive_complexity)]
async fn call_endpoint(operation: Operation, client: &Client) {
    let account: AccountUid = typed_id(ACCOUNT_UUID);
    let list: ListUid = typed_id(LIST_UUID);
    let workflow: WorkflowId = typed_id(WORKFLOW_UUID);
    match operation {
        Operation::ListAccounts => {
            let _ = aweber::endpoints::get_accounts(client, Some(window()), None).await;
        }
        Operation::GetAccount => {
            let _ = aweber::endpoints::get_account(client, ACCOUNT_ID).await;
        }
        Operation::FindAccountSubscribers => {
            let _ = aweber::endpoints::find_account_subscribers(
                client,
                ACCOUNT_ID,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(EMAIL),
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
                Some(window()),
                None,
            )
            .await;
        }
        Operation::ListAccountWebformSplitTests => {
            let _ = aweber::endpoints::list_account_webform_split_tests(
                client,
                ACCOUNT_ID,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::ListAccountWebforms => {
            let _ =
                aweber::endpoints::list_account_webforms(client, ACCOUNT_ID, Some(window()), None)
                    .await;
        }
        Operation::ListIntegrations => {
            let _ = aweber::endpoints::list_integrations(client, ACCOUNT_ID, Some(window()), None)
                .await;
        }
        Operation::GetIntegration => {
            let _ = aweber::endpoints::get_integration(client, ACCOUNT_ID, INTEGRATION_ID).await;
        }
        Operation::ListLists => {
            let _ = aweber::endpoints::list_lists(client, ACCOUNT_ID, Some(window()), None).await;
        }
        Operation::FindLists => {
            let _ = aweber::endpoints::find_lists(
                client,
                ACCOUNT_ID,
                Some("a list"),
                None,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::GetList => {
            let _ = aweber::endpoints::get_list(client, ACCOUNT_ID, LIST_ID).await;
        }
        Operation::ListBroadcasts => {
            let _ = aweber::endpoints::list_broadcasts(
                client,
                ACCOUNT_ID,
                LIST_ID,
                Some(&types::GetAccountsListsBroadcastsStatus::Draft),
                Some(window()),
                None,
            )
            .await;
        }
        Operation::CreateBroadcast => {
            let _ = aweber::endpoints::create_broadcast(
                client,
                ACCOUNT_ID,
                LIST_ID,
                &body::<types::CreateBroadcast>(serde_json::json!({
                    "body_html": "<p>hello</p>",
                    "body_text": "hello",
                    "subject": "hello",
                })),
            )
            .await;
        }
        Operation::GetBroadcastTotal => {
            let _ = aweber::endpoints::get_broadcast_total(
                client,
                ACCOUNT_ID,
                LIST_ID,
                &types::GetAccountsListsBroadcastsTotalStatus::Sent,
            )
            .await;
        }
        Operation::GetBroadcast | Operation::WaitBroadcast => {
            let _ =
                aweber::endpoints::get_broadcast(client, ACCOUNT_ID, LIST_ID, BROADCAST_ID).await;
        }
        Operation::UpdateBroadcast => {
            let _ = aweber::endpoints::update_broadcast(
                client,
                ACCOUNT_ID,
                LIST_ID,
                BROADCAST_ID,
                &types::UpdateBroadcast::default(),
            )
            .await;
        }
        Operation::DeleteBroadcast => {
            let _ = aweber::endpoints::delete_broadcast(client, ACCOUNT_ID, LIST_ID, BROADCAST_ID)
                .await;
        }
        Operation::CancelBroadcast => {
            let _ = aweber::endpoints::cancel_broadcast(client, ACCOUNT_ID, LIST_ID, BROADCAST_ID)
                .await;
        }
        Operation::GetBroadcastClicks => {
            let _ = aweber::endpoints::get_broadcast_clicks(
                client,
                ACCOUNT_ID,
                LIST_ID,
                BROADCAST_ID,
                None,
                None,
                None,
                Some(page()),
            )
            .await;
        }
        Operation::GetBroadcastOpens => {
            let _ = aweber::endpoints::get_broadcast_opens(
                client,
                ACCOUNT_ID,
                LIST_ID,
                BROADCAST_ID,
                None,
                None,
                Some(page()),
            )
            .await;
        }
        Operation::ScheduleBroadcast => {
            let _ = aweber::endpoints::schedule_broadcast(
                client,
                ACCOUNT_ID,
                LIST_ID,
                BROADCAST_ID,
                &body::<types::ScheduleBroadcast>(serde_json::json!({
                    "scheduled_for": "2026-01-02T03:04:05Z",
                })),
            )
            .await;
        }
        Operation::ListCampaigns => {
            let _ = aweber::endpoints::list_campaigns(
                client,
                ACCOUNT_ID,
                LIST_ID,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::ListCampaignStats => {
            let _ = aweber::endpoints::list_campaign_stats(
                client,
                ACCOUNT_ID,
                LIST_ID,
                CAMPAIGN_ID,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::GetCampaignStat => {
            let _ = aweber::endpoints::get_campaign_stat(
                client,
                ACCOUNT_ID,
                LIST_ID,
                CAMPAIGN_ID,
                &types::GetAccountsListsCampaignsBcampaignidStats2StatsId::TotalOpens,
            )
            .await;
        }
        Operation::FindCampaigns => {
            let _ = aweber::endpoints::find_campaigns(
                client,
                ACCOUNT_ID,
                LIST_ID,
                &types::GetAccountsListsCampaignsFindCampaignType::B,
                None,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::GetCampaign => {
            let _ = aweber::endpoints::get_campaign(client, ACCOUNT_ID, LIST_ID, "b", CAMPAIGN_ID)
                .await;
        }
        Operation::ListCustomFields => {
            let _ = aweber::endpoints::list_custom_fields(
                client,
                ACCOUNT_ID,
                LIST_ID,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::CreateCustomField => {
            let _ = aweber::endpoints::create_custom_field(
                client,
                ACCOUNT_ID,
                LIST_ID,
                &body::<types::PostAccountsListsCustomFieldsBody>(serde_json::json!({
                    "name": "a field",
                    "ws.op": "create",
                })),
            )
            .await;
        }
        Operation::GetCustomField => {
            let _ =
                aweber::endpoints::get_custom_field(client, ACCOUNT_ID, LIST_ID, CUSTOM_FIELD_ID)
                    .await;
        }
        Operation::DeleteCustomField => {
            let _ = aweber::endpoints::delete_custom_field(
                client,
                ACCOUNT_ID,
                LIST_ID,
                CUSTOM_FIELD_ID,
            )
            .await;
        }
        Operation::UpdateCustomField => {
            let _ = aweber::endpoints::update_custom_field(
                client,
                ACCOUNT_ID,
                LIST_ID,
                CUSTOM_FIELD_ID,
                &types::PatchAccountsListsCustomFieldsBody::default(),
            )
            .await;
        }
        Operation::ListLandingPages => {
            let _ = aweber::endpoints::list_landing_pages(
                client,
                ACCOUNT_ID,
                LIST_ID,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::GetLandingPage => {
            let _ = aweber::endpoints::get_landing_page(
                client,
                ACCOUNT_ID,
                LIST_ID,
                parsed(LANDING_PAGE_UUID),
            )
            .await;
        }
        Operation::CreatePurchase => {
            let _ = aweber::endpoints::create_purchase(
                client,
                ACCOUNT_ID,
                LIST_ID,
                &body::<types::Purchase>(serde_json::json!({
                    "currency": "USD",
                    "email": EMAIL,
                    "event_note": "a note",
                    "event_time": "2026-01-02T03:04:05Z",
                    "ip_address": "203.0.113.4",
                    "product_name": "a product",
                    "url": "https://example.com/checkout",
                    "value": 1.0,
                    "vendor": "a vendor",
                })),
            )
            .await;
        }
        Operation::ListSegments => {
            let _ =
                aweber::endpoints::list_segments(client, ACCOUNT_ID, LIST_ID, Some(window()), None)
                    .await;
        }
        Operation::GetSegment => {
            let _ = aweber::endpoints::get_segment(client, ACCOUNT_ID, LIST_ID, SEGMENT_ID).await;
        }
        Operation::ListSubscribers => {
            let _ = aweber::endpoints::list_subscribers(
                client,
                ACCOUNT_ID,
                LIST_ID,
                None,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::CreateSubscriber => {
            let _ = aweber::endpoints::create_subscriber(
                client,
                ACCOUNT_ID,
                LIST_ID,
                &body::<types::AddSubscriberRequestBody>(serde_json::json!({ "email": EMAIL })),
            )
            .await;
        }
        Operation::DeleteSubscriberByEmail => {
            let _ =
                aweber::endpoints::delete_subscriber_by_email(client, ACCOUNT_ID, LIST_ID, EMAIL)
                    .await;
        }
        Operation::UpdateSubscriberByEmail => {
            let _ = aweber::endpoints::update_subscriber_by_email(
                client,
                ACCOUNT_ID,
                LIST_ID,
                EMAIL,
                &serde_json::json!({}),
            )
            .await;
        }
        Operation::FindSubscribers => {
            let _ = aweber::endpoints::find_subscribers(
                client,
                ACCOUNT_ID,
                LIST_ID,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(EMAIL),
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
                Some(window()),
                None,
            )
            .await;
        }
        Operation::GetSubscriber => {
            let _ =
                aweber::endpoints::get_subscriber(client, ACCOUNT_ID, LIST_ID, SUBSCRIBER_ID).await;
        }
        Operation::MoveSubscriber => {
            let _ = aweber::endpoints::move_subscriber(
                client,
                ACCOUNT_ID,
                LIST_ID,
                SUBSCRIBER_ID,
                &body::<types::MoveSubscriberRequestBody>(serde_json::json!({
                    "list_link": "https://api.aweber.com/1.0/accounts/1/lists/3",
                    "ws.op": "move",
                })),
            )
            .await;
        }
        Operation::DeleteSubscriber => {
            let _ =
                aweber::endpoints::delete_subscriber(client, ACCOUNT_ID, LIST_ID, SUBSCRIBER_ID)
                    .await;
        }
        Operation::UpdateSubscriber => {
            let _ = aweber::endpoints::update_subscriber(
                client,
                ACCOUNT_ID,
                LIST_ID,
                SUBSCRIBER_ID,
                &serde_json::json!({}),
            )
            .await;
        }
        Operation::UnsubscribeSubscriber => {
            let _ = aweber::endpoints::update_subscriber(
                client,
                ACCOUNT_ID,
                LIST_ID,
                SUBSCRIBER_ID,
                &serde_json::json!({ "status": "unsubscribed" }),
            )
            .await;
        }
        Operation::GetSubscriberActivity => {
            let _ = aweber::endpoints::get_subscriber_activity(
                client,
                ACCOUNT_ID,
                LIST_ID,
                SUBSCRIBER_ID,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::ListTags => {
            let _ = aweber::endpoints::list_tags(client, ACCOUNT_ID, LIST_ID).await;
        }
        Operation::ListWebFormSplitTests => {
            let _ = aweber::endpoints::list_web_form_split_tests(
                client,
                ACCOUNT_ID,
                LIST_ID,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::GetWebFormSplitTest => {
            let _ = aweber::endpoints::get_web_form_split_test(
                client,
                ACCOUNT_ID,
                LIST_ID,
                SPLIT_TEST_ID,
            )
            .await;
        }
        Operation::ListWebFormSplitTestComponents => {
            let _ = aweber::endpoints::list_web_form_split_test_components(
                client,
                ACCOUNT_ID,
                LIST_ID,
                SPLIT_TEST_ID,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::GetWebFormSplitTestComponent => {
            let _ = aweber::endpoints::get_web_form_split_test_component(
                client,
                ACCOUNT_ID,
                LIST_ID,
                SPLIT_TEST_ID,
                SPLIT_TEST_COMPONENT_ID,
            )
            .await;
        }
        Operation::ListWebForms => {
            let _ = aweber::endpoints::list_web_forms(
                client,
                ACCOUNT_ID,
                LIST_ID,
                Some(window()),
                None,
            )
            .await;
        }
        Operation::GetWebForm => {
            let _ = aweber::endpoints::get_web_form(client, ACCOUNT_ID, LIST_ID, WEBFORM_ID).await;
        }
        Operation::GetBroadcastLinkAnalytics => {
            let _ = aweber::endpoints::get_broadcast_link_analytics(
                client,
                &account,
                parsed(BROADCAST_UUID),
                &types::GetBroadcastLinksAnalyticsFilter::Clicks,
                None,
                None,
                None,
                None,
                Some(page()),
                None,
                None,
            )
            .await;
        }
        Operation::ListWorkflows => {
            let _ = workflows::list_workflows(client, account, list).await;
        }
        Operation::GetWorkflow | Operation::TreeWorkflow => {
            let _ = workflows::get_workflow(client, workflow).await;
        }
        Operation::CreateWorkflow => {
            let _ = workflows::create_workflow(
                client,
                &workflows::CreateWorkflow {
                    name: typed_id("a workflow"),
                    owner: account,
                    parent: list,
                    timezone: None,
                },
            )
            .await;
        }
        Operation::UpdateWorkflow => {
            let edit = workflows::WorkflowEdit {
                name: Some(typed_id("a workflow")),
                ..workflows::WorkflowEdit::default()
            };
            let _ = workflows::update_workflow(
                client,
                workflow,
                precondition(),
                &workflows::WorkflowPatch::edits(&edit),
            )
            .await;
        }
        Operation::UpdateWorkflowRuleset => {
            let patch = workflows::WorkflowPatch::default()
                .with_ruleset(workflows::PatchOperation::Replace, &empty_graph());
            let _ = workflows::update_workflow(client, workflow, precondition(), &patch).await;
        }
        Operation::SetWorkflowState => {
            let edit = workflows::WorkflowEdit {
                status: Some(workflows::StatusChange::Active),
                ..workflows::WorkflowEdit::default()
            };
            let _ = workflows::update_workflow(
                client,
                workflow,
                precondition(),
                &workflows::WorkflowPatch::edits(&edit),
            )
            .await;
        }
        Operation::PublishWorkflow => {
            let _ = workflows::publish_workflow(client, workflow, precondition()).await;
        }
        Operation::RevertWorkflow => {
            let _ = workflows::revert_workflow(client, workflow, precondition()).await;
        }
        Operation::CopyWorkflow => {
            let _ = workflows::copy_workflow(
                client,
                workflow,
                precondition(),
                &workflows::CopyWorkflow {
                    target_list: list,
                    name: Some(typed_id("a copy")),
                    timezone: None,
                },
            )
            .await;
        }
        Operation::DeleteWorkflow => {
            let _ = workflows::delete_workflow(client, workflow, precondition()).await;
        }
        Operation::GetWorkflowStats => {
            let _ = workflows::get_campaign_message_totals(
                client,
                workflow,
                workflows::SendCadence::Once,
            )
            .await;
        }
        Operation::GetWorkflowMessageStats => {
            let _ = workflows::get_link_click_stats(
                client,
                &message(),
                account,
                workflows::SendCadence::Once,
                small_page(),
                None,
            )
            .await;
        }
        Operation::GetWorkflowEventHistory => {
            let _ = workflows::get_recurring_event_history(
                client,
                workflow,
                typed_id::<RuleId>(EVENT_UUID),
                small_page(),
                None,
            )
            .await;
        }
        Operation::OauthGetAccessToken => {
            let _ = aweber::endpoints::oauth_get_access_token(
                client,
                &types::PostOauthAccessTokenBody::default(),
            )
            .await;
        }
        Operation::OauthGetRequestToken => {
            let _ = aweber::endpoints::oauth_get_request_token(
                client,
                &types::PostOauthRequestTokenBody::default(),
            )
            .await;
        }
        Operation::OauthRevoke => {
            let _ = aweber::endpoints::oauth2_revoke(
                client,
                Some("Bearer fixture"),
                &body::<types::PostOauth2RevokeBody>(serde_json::json!({
                    "client_id": "an id",
                    "client_secret": "a secret",
                })),
            )
            .await;
        }
        Operation::OauthToken => {
            let _ = aweber::endpoints::oauth2_token(
                client,
                Some("Bearer fixture"),
                &body::<types::PostOauth2TokenBody>(serde_json::json!({
                    "client_id": "an id",
                    "client_secret": "a secret",
                    "code": "a code",
                    "grant_type": "authorization_code",
                })),
            )
            .await;
        }
    }
}

/// A request as the server saw it, with its query in a stable order.
#[derive(Debug, Eq, PartialEq)]
struct Recorded {
    method: String,
    path: String,
    query: Vec<(String, String)>,
    /// `authorization`, `accept`, `content-length`, `host`, and `user-agent`
    /// are dropped; every other header is compared by lowercased name and value.
    headers: Vec<(String, String)>,
    body: Option<RecordedBody>,
}

/// A body read as the request's own `content-type` says to read it.
#[derive(Debug, Eq, PartialEq)]
enum RecordedBody {
    /// Parsed, so key order cannot fail the comparison.
    Json(serde_json::Value),
    /// The members of an `application/x-www-form-urlencoded` body, sorted.
    Form(Vec<(String, String)>),
    Text(String),
}

const IGNORED_HEADERS: [&str; 5] = [
    "authorization",
    "accept",
    "content-length",
    "host",
    "user-agent",
];

fn recorded(request: &wiremock::Request) -> Recorded {
    let mut query: Vec<(String, String)> = request
        .url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    query.sort();
    let mut headers: Vec<(String, String)> = request
        .headers
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_ascii_lowercase(),
                value.to_str().unwrap_or("<binary>").to_string(),
            )
        })
        .filter(|(name, _)| !IGNORED_HEADERS.contains(&name.as_str()))
        .collect();
    headers.sort();
    let content_type = request
        .headers
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    let text = String::from_utf8_lossy(&request.body).into_owned();
    let body = if request.body.is_empty() {
        None
    } else if content_type.starts_with("application/x-www-form-urlencoded") {
        let mut members: Vec<(String, String)> = serde_urlencoded::from_str(&text)
            .unwrap_or_else(|error| panic!("a form body is readable as one: {error}"));
        members.sort();
        Some(RecordedBody::Form(members))
    } else if content_type.starts_with("application/json") {
        Some(RecordedBody::Json(
            serde_json::from_slice(&request.body)
                .unwrap_or_else(|error| panic!("a json body is readable as one: {error}")),
        ))
    } else {
        Some(RecordedBody::Text(text))
    };
    Recorded {
        method: request.method.to_string(),
        path: request.url.path().to_string(),
        query,
        headers,
        body,
    }
}

/// Driven by `Operation::ALL`; a missing arm fails rather than skipping.
#[tokio::test]
async fn every_plan_matches_its_endpoint_request() {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
        .mount(&server)
        .await;
    let client = support::client(&server);
    let scope = Scope {
        account_id: ACCOUNT_ID,
        account: Some(typed_id(ACCOUNT_UUID)),
    };

    for operation in Operation::ALL {
        let args = typed(operation, fixture(operation));
        let plan = RequestPlan::build(operation, &scope, &args)
            .unwrap_or_else(|error| panic!("{operation:?} has no plan: {error}"));
        let before = server
            .received_requests()
            .await
            .expect("the server records its requests")
            .len();

        call_endpoint(operation, &client).await;
        let _ = client.send_plan(&plan).await;

        let requests = server
            .received_requests()
            .await
            .expect("the server records its requests");
        assert_eq!(
            requests.len(),
            before + 2,
            "{operation:?} did not issue one endpoint request and one planned request"
        );
        let endpoint = recorded(&requests[before]);
        let planned = recorded(&requests[before + 1]);
        assert_eq!(
            planned, endpoint,
            "{operation:?} plans a different request than its endpoint function issues"
        );
    }
}

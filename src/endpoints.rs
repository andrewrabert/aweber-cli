use crate::client::{ApiError, ApiRequest, Client};
use crate::types;
use reqwest::Method;

// ---------------------------------------------------------------------------
// Accounts
// ---------------------------------------------------------------------------

pub async fn get_accounts(
    client: &Client,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::Accounts, ApiError> {
    ApiRequest::new(client, Method::GET, "/1.0/accounts".into())
        .query_opt("ws.size", ws_size)
        .query_opt("ws.start", ws_start)
        .send()
        .await
}

pub async fn get_account(client: &Client, account_id: i32) -> Result<types::Account, ApiError> {
    ApiRequest::new(client, Method::GET, format!("/1.0/accounts/{account_id}"))
        .send()
        .await
}

// ---------------------------------------------------------------------------
// Account-level find subscribers
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
pub async fn find_account_subscribers(
    client: &Client,
    account_id: i32,
    ad_tracking: Option<&str>,
    area_code: Option<i32>,
    city: Option<&str>,
    country: Option<&str>,
    custom_fields: Option<&str>,
    dma_code: Option<i32>,
    email: Option<&str>,
    last_followup_message_number_sent: Option<i32>,
    last_followup_message_sent_at: Option<chrono::NaiveDate>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    misc_notes: Option<&str>,
    name: Option<&str>,
    postal_code: Option<&str>,
    region: Option<&str>,
    status: Option<&types::GetAccountsFindsubscribersStatus>,
    subscribed_after: Option<chrono::NaiveDate>,
    subscribed_at: Option<chrono::NaiveDate>,
    subscribed_before: Option<chrono::NaiveDate>,
    subscription_method: Option<&types::GetAccountsFindsubscribersSubscriptionMethod>,
    tags: Option<&str>,
    tags_not_in: Option<&str>,
    unsubscribe_method: Option<&types::GetAccountsFindsubscribersUnsubscribeMethod>,
    unsubscribed_after: Option<chrono::NaiveDate>,
    unsubscribed_at: Option<chrono::NaiveDate>,
    unsubscribed_before: Option<chrono::NaiveDate>,
    verified_at: Option<chrono::NaiveDate>,
    ws_show: Option<&types::GetAccountsFindsubscribersWsShow>,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::FindSubscribersAccount, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}"),
    )
    .query("ws.op", "findSubscribers")
    .query_opt("ad_tracking", ad_tracking)
    .query_opt("area_code", area_code)
    .query_opt("city", city)
    .query_opt("country", country)
    .query_opt("custom_fields", custom_fields)
    .query_opt("dma_code", dma_code)
    .query_opt("email", email)
    .query_opt(
        "last_followup_message_number_sent",
        last_followup_message_number_sent,
    )
    .query_opt(
        "last_followup_message_sent_at",
        last_followup_message_sent_at,
    )
    .query_opt("latitude", latitude)
    .query_opt("longitude", longitude)
    .query_opt("misc_notes", misc_notes)
    .query_opt("name", name)
    .query_opt("postal_code", postal_code)
    .query_opt("region", region)
    .query_opt("status", status)
    .query_opt("subscribed_after", subscribed_after)
    .query_opt("subscribed_at", subscribed_at)
    .query_opt("subscribed_before", subscribed_before)
    .query_opt("subscription_method", subscription_method)
    .query_opt("tags", tags)
    .query_opt("tags_not_in", tags_not_in)
    .query_opt("unsubscribe_method", unsubscribe_method)
    .query_opt("unsubscribed_after", unsubscribed_after)
    .query_opt("unsubscribed_at", unsubscribed_at)
    .query_opt("unsubscribed_before", unsubscribed_before)
    .query_opt("verified_at", verified_at)
    .query_opt("ws.show", ws_show)
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Account-level webforms / split tests
// ---------------------------------------------------------------------------

pub async fn list_account_webform_split_tests(
    client: &Client,
    account_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::WebFormSplitTests, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}"),
    )
    .query("ws.op", "getWebFormSplitTests")
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn list_account_webforms(
    client: &Client,
    account_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::Webforms, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}"),
    )
    .query("ws.op", "getWebForms")
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Integrations
// ---------------------------------------------------------------------------

pub async fn list_integrations(
    client: &Client,
    account_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::Integrations, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/integrations"),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn get_integration(
    client: &Client,
    account_id: i32,
    integration_id: i32,
) -> Result<types::Integration, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/integrations/{integration_id}"),
    )
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Lists
// ---------------------------------------------------------------------------

pub async fn list_lists(
    client: &Client,
    account_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::Lists, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists"),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn find_lists(
    client: &Client,
    account_id: i32,
    name: Option<&str>,
    ws_show: Option<&types::GetAccountsListsFindWsShow>,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::FindLists, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists"),
    )
    .query("ws.op", "find")
    .query_opt("name", name)
    .query_opt("ws.show", ws_show)
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn get_list(
    client: &Client,
    account_id: i32,
    list_id: i32,
) -> Result<types::List, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}"),
    )
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Broadcasts
// ---------------------------------------------------------------------------

pub async fn list_broadcasts(
    client: &Client,
    account_id: i32,
    list_id: i32,
    status: Option<&types::GetAccountsListsBroadcastsStatus>,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::Broadcasts, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts"),
    )
    .query_opt("status", status)
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn create_broadcast(
    client: &Client,
    account_id: i32,
    list_id: i32,
    body: &types::CreateBroadcast,
) -> Result<types::Broadcast, ApiError> {
    ApiRequest::new(
        client,
        Method::POST,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts"),
    )
    .form_body(body)
    .send()
    .await
}

pub async fn get_broadcast_total(
    client: &Client,
    account_id: i32,
    list_id: i32,
    status: &types::GetAccountsListsBroadcastsTotalStatus,
) -> Result<types::GetAccountsListsBroadcastsTotalResponse, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts/total"),
    )
    .query("status", status)
    .send()
    .await
}

pub async fn get_broadcast(
    client: &Client,
    account_id: i32,
    list_id: i32,
    broadcast_id: i32,
) -> Result<types::Broadcast, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts/{broadcast_id}"),
    )
    .send()
    .await
}

pub async fn update_broadcast(
    client: &Client,
    account_id: i32,
    list_id: i32,
    broadcast_id: i32,
    body: &types::UpdateBroadcast,
) -> Result<types::Broadcast, ApiError> {
    ApiRequest::new(
        client,
        Method::PUT,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts/{broadcast_id}"),
    )
    .form_body(body)
    .send()
    .await
}

pub async fn delete_broadcast(
    client: &Client,
    account_id: i32,
    list_id: i32,
    broadcast_id: i32,
) -> Result<(), ApiError> {
    ApiRequest::new(
        client,
        Method::DELETE,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts/{broadcast_id}"),
    )
    .send_no_body()
    .await
}

pub async fn cancel_broadcast(
    client: &Client,
    account_id: i32,
    list_id: i32,
    broadcast_id: i32,
) -> Result<types::PostAccountsListsBroadcastsCancelResponse, ApiError> {
    ApiRequest::new(
        client,
        Method::POST,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts/{broadcast_id}/cancel"),
    )
    .send()
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn get_broadcast_clicks(
    client: &Client,
    account_id: i32,
    list_id: i32,
    broadcast_id: i32,
    after: Option<&str>,
    before: Option<&str>,
    detailed: Option<bool>,
    page_size: Option<std::num::NonZeroU64>,
) -> Result<types::BroadcastClicks, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts/{broadcast_id}/clicks"),
    )
    .query_opt("after", after)
    .query_opt("before", before)
    .query_opt("detailed", detailed)
    .query_opt("page_size", page_size)
    .send()
    .await
}

pub async fn get_broadcast_opens(
    client: &Client,
    account_id: i32,
    list_id: i32,
    broadcast_id: i32,
    after: Option<&str>,
    before: Option<&str>,
    page_size: Option<std::num::NonZeroU64>,
) -> Result<types::BroadcastOpens, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts/{broadcast_id}/opens"),
    )
    .query_opt("after", after)
    .query_opt("before", before)
    .query_opt("page_size", page_size)
    .send()
    .await
}

pub async fn schedule_broadcast(
    client: &Client,
    account_id: i32,
    list_id: i32,
    broadcast_id: i32,
    body: &types::ScheduleBroadcast,
) -> Result<types::PostAccountsListsBroadcastsScheduleResponse, ApiError> {
    ApiRequest::new(
        client,
        Method::POST,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/broadcasts/{broadcast_id}/schedule"),
    )
    .form_body(body)
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Campaigns
// ---------------------------------------------------------------------------

pub async fn list_campaigns(
    client: &Client,
    account_id: i32,
    list_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::ListCampaigns, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/campaigns"),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn list_campaign_stats(
    client: &Client,
    account_id: i32,
    list_id: i32,
    campaign_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::Stats, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/campaigns/b{campaign_id}/stats"),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn get_campaign_stat(
    client: &Client,
    account_id: i32,
    list_id: i32,
    campaign_id: i32,
    stats_id: i32,
) -> Result<types::Stat, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!(
            "/1.0/accounts/{account_id}/lists/{list_id}/campaigns/b{campaign_id}/stats/{stats_id}"
        ),
    )
    .send()
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn find_campaigns(
    client: &Client,
    account_id: i32,
    list_id: i32,
    campaign_type: &types::GetAccountsListsCampaignsFindCampaignType,
    ws_show: Option<&types::GetAccountsListsCampaignsFindWsShow>,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::FindCampaigns, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/campaigns"),
    )
    .query("ws.op", "find")
    .query("campaign_type", campaign_type)
    .query_opt("ws.show", ws_show)
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn get_campaign(
    client: &Client,
    account_id: i32,
    list_id: i32,
    campaign_type: &str,
    campaign_id: i32,
) -> Result<types::Campaign, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!(
            "/1.0/accounts/{account_id}/lists/{list_id}/campaigns/{campaign_type}{campaign_id}"
        ),
    )
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Custom Fields
// ---------------------------------------------------------------------------

pub async fn list_custom_fields(
    client: &Client,
    account_id: i32,
    list_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::CustomFields, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/custom_fields"),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn create_custom_field(
    client: &Client,
    account_id: i32,
    list_id: i32,
    body: &types::PostAccountsListsCustomFieldsBody,
) -> Result<(), ApiError> {
    ApiRequest::new(
        client,
        Method::POST,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/custom_fields"),
    )
    .json_body(body)
    .send_no_body()
    .await
}

pub async fn get_custom_field(
    client: &Client,
    account_id: i32,
    list_id: i32,
    custom_field_id: i32,
) -> Result<types::CustomField, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/custom_fields/{custom_field_id}"),
    )
    .send()
    .await
}

pub async fn delete_custom_field(
    client: &Client,
    account_id: i32,
    list_id: i32,
    custom_field_id: i32,
) -> Result<(), ApiError> {
    ApiRequest::new(
        client,
        Method::DELETE,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/custom_fields/{custom_field_id}"),
    )
    .send_no_body()
    .await
}

pub async fn update_custom_field(
    client: &Client,
    account_id: i32,
    list_id: i32,
    custom_field_id: i32,
    body: &types::PatchAccountsListsCustomFieldsBody,
) -> Result<types::CustomField, ApiError> {
    ApiRequest::new(
        client,
        Method::PATCH,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/custom_fields/{custom_field_id}"),
    )
    .json_body(body)
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Landing Pages
// ---------------------------------------------------------------------------

pub async fn list_landing_pages(
    client: &Client,
    account_id: i32,
    list_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::LandingPages, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/landing_pages"),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn get_landing_page(
    client: &Client,
    account_id: i32,
    list_id: i32,
    landing_page_id: uuid::Uuid,
) -> Result<types::LandingPage, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/landing_pages/{landing_page_id}"),
    )
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Purchases
// ---------------------------------------------------------------------------

pub async fn create_purchase(
    client: &Client,
    account_id: i32,
    list_id: i32,
    body: &types::Purchase,
) -> Result<(), ApiError> {
    ApiRequest::new(
        client,
        Method::POST,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/purchases"),
    )
    .json_body(body)
    .send_no_body()
    .await
}

// ---------------------------------------------------------------------------
// Segments
// ---------------------------------------------------------------------------

pub async fn list_segments(
    client: &Client,
    account_id: i32,
    list_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::Segments, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/segments"),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn get_segment(
    client: &Client,
    account_id: i32,
    list_id: i32,
    segment_id: i32,
) -> Result<types::Segment, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/segments/{segment_id}"),
    )
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Subscribers (collection-level / by email)
// ---------------------------------------------------------------------------

pub async fn list_subscribers(
    client: &Client,
    account_id: i32,
    list_id: i32,
    sort_order: Option<&types::GetAccountsListsSubscribersSortOrder>,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::Subscribers, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/subscribers"),
    )
    .query_opt("sort_order", sort_order)
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn create_subscriber(
    client: &Client,
    account_id: i32,
    list_id: i32,
    body: &types::AddSubscriberRequestBody,
) -> Result<(), ApiError> {
    ApiRequest::new(
        client,
        Method::POST,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/subscribers"),
    )
    .json_body(body)
    .send_no_body()
    .await
}

pub async fn delete_subscriber_by_email(
    client: &Client,
    account_id: i32,
    list_id: i32,
    subscriber_email: &str,
) -> Result<(), ApiError> {
    ApiRequest::new(
        client,
        Method::DELETE,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/subscribers"),
    )
    .query_opt("subscriber_email", Some(subscriber_email))
    .send_no_body()
    .await
}

pub async fn update_subscriber_by_email(
    client: &Client,
    account_id: i32,
    list_id: i32,
    subscriber_email: &str,
    body: &serde_json::Value,
) -> Result<types::Subscriber, ApiError> {
    ApiRequest::new(
        client,
        Method::PATCH,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/subscribers"),
    )
    .query_opt("subscriber_email", Some(subscriber_email))
    .json_body(body)
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Subscribers find (by list)
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
pub async fn find_subscribers(
    client: &Client,
    account_id: i32,
    list_id: i32,
    ad_tracking: Option<&str>,
    area_code: Option<i32>,
    city: Option<&str>,
    country: Option<&str>,
    custom_fields: Option<&str>,
    dma_code: Option<i32>,
    email: Option<&str>,
    last_followup_message_number_sent: Option<i32>,
    last_followup_message_sent_at: Option<chrono::NaiveDate>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    misc_notes: Option<&str>,
    name: Option<&str>,
    postal_code: Option<&str>,
    region: Option<&str>,
    sort_key: Option<&types::GetAccountsListsSubscribersFindSortKey>,
    sort_order: Option<&types::GetAccountsListsSubscribersFindSortOrder>,
    status: Option<&types::GetAccountsListsSubscribersFindStatus>,
    subscribed_after: Option<chrono::NaiveDate>,
    subscribed_at: Option<chrono::NaiveDate>,
    subscribed_before: Option<chrono::NaiveDate>,
    subscription_method: Option<&types::GetAccountsListsSubscribersFindSubscriptionMethod>,
    tags: Option<&str>,
    tags_not_in: Option<&str>,
    unsubscribe_method: Option<&types::GetAccountsListsSubscribersFindUnsubscribeMethod>,
    unsubscribed_after: Option<chrono::NaiveDate>,
    unsubscribed_at: Option<chrono::NaiveDate>,
    unsubscribed_before: Option<chrono::NaiveDate>,
    verified_at: Option<chrono::NaiveDate>,
    ws_show: Option<&types::GetAccountsListsSubscribersFindWsShow>,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::FindSubscribers, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/subscribers"),
    )
    .query("ws.op", "find")
    .query_opt("ad_tracking", ad_tracking)
    .query_opt("area_code", area_code)
    .query_opt("city", city)
    .query_opt("country", country)
    .query_opt("custom_fields", custom_fields)
    .query_opt("dma_code", dma_code)
    .query_opt("email", email)
    .query_opt(
        "last_followup_message_number_sent",
        last_followup_message_number_sent,
    )
    .query_opt(
        "last_followup_message_sent_at",
        last_followup_message_sent_at,
    )
    .query_opt("latitude", latitude)
    .query_opt("longitude", longitude)
    .query_opt("misc_notes", misc_notes)
    .query_opt("name", name)
    .query_opt("postal_code", postal_code)
    .query_opt("region", region)
    .query_opt("sort_key", sort_key)
    .query_opt("sort_order", sort_order)
    .query_opt("status", status)
    .query_opt("subscribed_after", subscribed_after)
    .query_opt("subscribed_at", subscribed_at)
    .query_opt("subscribed_before", subscribed_before)
    .query_opt("subscription_method", subscription_method)
    .query_opt("tags", tags)
    .query_opt("tags_not_in", tags_not_in)
    .query_opt("unsubscribe_method", unsubscribe_method)
    .query_opt("unsubscribed_after", unsubscribed_after)
    .query_opt("unsubscribed_at", unsubscribed_at)
    .query_opt("unsubscribed_before", unsubscribed_before)
    .query_opt("verified_at", verified_at)
    .query_opt("ws.show", ws_show)
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Subscribers (by ID)
// ---------------------------------------------------------------------------

pub async fn get_subscriber(
    client: &Client,
    account_id: i32,
    list_id: i32,
    subscriber_id: i32,
) -> Result<types::GetAccountsListsSubscribers2Response, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/subscribers/{subscriber_id}"),
    )
    .send()
    .await
}

pub async fn move_subscriber(
    client: &Client,
    account_id: i32,
    list_id: i32,
    subscriber_id: i32,
    body: &types::MoveSubscriberRequestBody,
) -> Result<(), ApiError> {
    ApiRequest::new(
        client,
        Method::POST,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/subscribers/{subscriber_id}"),
    )
    .form_body(body)
    .send_no_body()
    .await
}

pub async fn delete_subscriber(
    client: &Client,
    account_id: i32,
    list_id: i32,
    subscriber_id: i32,
) -> Result<(), ApiError> {
    ApiRequest::new(
        client,
        Method::DELETE,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/subscribers/{subscriber_id}"),
    )
    .send_no_body()
    .await
}

pub async fn update_subscriber(
    client: &Client,
    account_id: i32,
    list_id: i32,
    subscriber_id: i32,
    body: &serde_json::Value,
) -> Result<types::Subscriber, ApiError> {
    ApiRequest::new(
        client,
        Method::PATCH,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/subscribers/{subscriber_id}"),
    )
    .json_body(body)
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Subscriber Activity
// ---------------------------------------------------------------------------

pub async fn get_subscriber_activity(
    client: &Client,
    account_id: i32,
    list_id: i32,
    subscriber_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::SubscriberGetActivity, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!(
            "/1.0/accounts/{account_id}/lists/{list_id}/subscribers/{subscriber_id}"
        ),
    )
    .query("ws.op", "getActivity")
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Tags
// ---------------------------------------------------------------------------

pub async fn list_tags(
    client: &Client,
    account_id: i32,
    list_id: i32,
) -> Result<Vec<String>, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/tags"),
    )
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Web Form Split Tests
// ---------------------------------------------------------------------------

pub async fn list_web_form_split_tests(
    client: &Client,
    account_id: i32,
    list_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::WebFormSplitTests, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/web_form_split_tests"),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn get_web_form_split_test(
    client: &Client,
    account_id: i32,
    list_id: i32,
    split_test_id: i32,
) -> Result<types::WebformSplitTest, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/web_form_split_tests/{split_test_id}"),
    )
    .send()
    .await
}

pub async fn list_web_form_split_test_components(
    client: &Client,
    account_id: i32,
    list_id: i32,
    split_test_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::WebformSplitTestComponents, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!(
            "/1.0/accounts/{account_id}/lists/{list_id}/web_form_split_tests/{split_test_id}/components"
        ),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn get_web_form_split_test_component(
    client: &Client,
    account_id: i32,
    list_id: i32,
    split_test_id: i32,
    split_test_component_id: i32,
) -> Result<types::WebformSplitTestComponent, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!(
            "/1.0/accounts/{account_id}/lists/{list_id}/web_form_split_tests/{split_test_id}/components/{split_test_component_id}"
        ),
    )
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Web Forms
// ---------------------------------------------------------------------------

pub async fn list_web_forms(
    client: &Client,
    account_id: i32,
    list_id: i32,
    ws_size: Option<std::num::NonZeroU32>,
    ws_start: Option<i32>,
) -> Result<types::Webforms, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/web_forms"),
    )
    .query_opt("ws.size", ws_size)
    .query_opt("ws.start", ws_start)
    .send()
    .await
}

pub async fn get_web_form(
    client: &Client,
    account_id: i32,
    list_id: i32,
    webform_id: i32,
) -> Result<types::Webform, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("/1.0/accounts/{account_id}/lists/{list_id}/web_forms/{webform_id}"),
    )
    .send()
    .await
}

// ---------------------------------------------------------------------------
// Broadcast Link Analytics
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
pub async fn get_broadcast_link_analytics(
    client: &Client,
    account_id: i32,
    after: Option<&str>,
    before: Option<&str>,
    broadcast_id: i32,
    filter: &str,
    max_count: Option<i32>,
    min_count: Option<i32>,
    page_size: Option<std::num::NonZeroU64>,
    sort_asc: Option<bool>,
    sort_by: Option<&str>,
) -> Result<Vec<types::GetBroadcastLinksAnalyticsResponseItem>, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        "/1.0/analytics/reports/broadcasts-links".into(),
    )
    .query("account_id", account_id)
    .query_opt("after", after)
    .query_opt("before", before)
    .query("broadcast_id", broadcast_id)
    .query("filter", filter)
    .query_opt("max_count", max_count)
    .query_opt("min_count", min_count)
    .query_opt("page_size", page_size)
    .query_opt("sort_asc", sort_asc)
    .query_opt("sort_by", sort_by)
    .send()
    .await
}

// ---------------------------------------------------------------------------
// OAuth (kept for completeness, rarely used directly from CLI)
// ---------------------------------------------------------------------------

pub async fn oauth_get_access_token(
    client: &Client,
    body: &types::PostOauthAccessTokenBody,
) -> Result<types::PostOauthAccessTokenResponse, ApiError> {
    ApiRequest::new(client, Method::POST, "/oauth/access_token".into())
        .form_body(body)
        .send()
        .await
}

pub async fn oauth_get_request_token(
    client: &Client,
    body: &types::PostOauthRequestTokenBody,
) -> Result<types::PostOauthRequestTokenResponse, ApiError> {
    ApiRequest::new(client, Method::POST, "/oauth/request_token".into())
        .form_body(body)
        .send()
        .await
}

pub async fn oauth2_revoke(
    client: &Client,
    authorization: Option<&str>,
    body: &types::PostOauth2RevokeBody,
) -> Result<(), ApiError> {
    let mut req = ApiRequest::new(client, Method::POST, "/oauth2/revoke".into());
    if let Some(auth) = authorization {
        req = req.header(reqwest::header::AUTHORIZATION, auth.to_string());
    }
    req.json_body(body).send_no_body().await
}

pub async fn oauth2_token(
    client: &Client,
    authorization: Option<&str>,
    body: &types::PostOauth2TokenBody,
) -> Result<types::PostOauth2TokenResponse, ApiError> {
    let mut req = ApiRequest::new(client, Method::POST, "/oauth2/token".into());
    if let Some(auth) = authorization {
        req = req.header(reqwest::header::AUTHORIZATION, auth.to_string());
    }
    req.json_body(body).send().await
}

// ---------------------------------------------------------------------------
// Message Editor API (cookie auth via control panel)
// ---------------------------------------------------------------------------

fn meapi_path(account_id: i32, list_id: i32) -> String {
    format!("/message-editor/v1/accounts/{account_id}/lists/{list_id}/messages")
}

fn cookie_header(session: &str) -> String {
    format!("AUTORESPSID={session}")
}

/// Add cookie + CSRF headers required by mutating message-editor requests.
fn meapi_csrf(req: ApiRequest, session: &str, account_id: i32) -> ApiRequest {
    req.header(reqwest::header::COOKIE, cookie_header(session))
        .header(reqwest::header::AUTHORIZATION, format!("aweber {session}"))
        .header(
            reqwest::header::HeaderName::from_static("x-aweber-customer-id"),
            account_id.to_string(),
        )
}

pub async fn list_messages(
    client: &Client,
    session: &str,
    account_id: i32,
    list_id: i32,
    binding: Option<&str>,
    subject: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<types::MessageList, ApiError> {
    ApiRequest::new(client, Method::GET, meapi_path(account_id, list_id))
        .header(reqwest::header::COOKIE, cookie_header(session))
        .query_opt("binding", binding)
        .query_opt("subject", subject)
        .query_opt("limit", limit)
        .query_opt("offset", offset)
        .send()
        .await
}

pub async fn get_message(
    client: &Client,
    session: &str,
    account_id: i32,
    list_id: i32,
    message_id: &str,
) -> Result<serde_json::Value, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("{}/{message_id}", meapi_path(account_id, list_id)),
    )
    .header(reqwest::header::COOKIE, cookie_header(session))
    .send()
    .await
}

pub async fn create_message(
    client: &Client,
    session: &str,
    account_id: i32,
    list_id: i32,
    body: &types::CreateMessage,
) -> Result<serde_json::Value, ApiError> {
    let req = ApiRequest::new(client, Method::POST, meapi_path(account_id, list_id));
    meapi_csrf(req, session, account_id)
        .json_body(body)
        .send()
        .await
}

pub async fn update_message(
    client: &Client,
    session: &str,
    account_id: i32,
    list_id: i32,
    message_id: &str,
    body: &types::UpdateMessage,
) -> Result<serde_json::Value, ApiError> {
    let req = ApiRequest::new(
        client,
        Method::PATCH,
        format!("{}/{message_id}", meapi_path(account_id, list_id)),
    );
    meapi_csrf(req, session, account_id)
        .json_body(body)
        .send()
        .await
}

pub async fn delete_message(
    client: &Client,
    session: &str,
    account_id: i32,
    list_id: i32,
    message_id: &str,
) -> Result<(), ApiError> {
    let req = ApiRequest::new(
        client,
        Method::DELETE,
        format!("{}/{message_id}", meapi_path(account_id, list_id)),
    );
    meapi_csrf(req, session, account_id)
        .send_no_body()
        .await
}

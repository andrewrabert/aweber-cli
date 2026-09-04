use reqwest::Method;

use crate::client::{ApiError, ApiRequest, Client};
use crate::ids::{AccountUid, MessageId, RuleId, WorkflowId};
use crate::pagination::{self, Cursor, Page, PageSize};
use crate::workflows::values::{MessageTotals, SendCadence};

const REPORTS: &str = "/internal/analytics-view/reports";

pub async fn get_campaign_message_stats(
    client: &Client,
    message: &MessageId,
    account: AccountUid,
    cadence: SendCadence,
) -> Result<MessageTotals, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("{REPORTS}/campaign-message-stats/{message}"),
    )
    .query("account_id", account)
    .query("recurring", cadence)
    .send()
    .await
}

pub async fn get_campaign_message_totals(
    client: &Client,
    workflow: WorkflowId,
    cadence: SendCadence,
) -> Result<Vec<serde_json::Value>, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("{REPORTS}/campaign-message-totals/{workflow}"),
    )
    .query("recurring", cadence)
    .send()
    .await
}

pub async fn get_recurring_events(
    client: &Client,
    workflow: WorkflowId,
) -> Result<Vec<serde_json::Value>, ApiError> {
    ApiRequest::new(
        client,
        Method::GET,
        format!("{REPORTS}/recurring-events/{workflow}"),
    )
    .send()
    .await
}

pub async fn get_link_click_stats(
    client: &Client,
    message: &MessageId,
    account: AccountUid,
    cadence: SendCadence,
    page_size: PageSize,
    after: Option<&Cursor>,
) -> Result<Page<serde_json::Value>, ApiError> {
    let response = ApiRequest::new(
        client,
        Method::GET,
        format!("{REPORTS}/campaign-message-click-stats/{message}"),
    )
    .query("account_id", account)
    .query("recurring", cadence)
    .query("page_size", page_size)
    .query_opt("after", after)
    .send_with_headers::<Vec<serde_json::Value>>()
    .await?;
    Ok(Page {
        entries: response.body,
        next_cursor: pagination::next_cursor(&response.headers, "after")
            .and_then(pagination::within_offset_cap),
    })
}

pub async fn get_recurring_message_stats(
    client: &Client,
    message: &MessageId,
    account: AccountUid,
    page_size: PageSize,
    after: Option<&Cursor>,
) -> Result<Page<serde_json::Value>, ApiError> {
    let response = ApiRequest::new(
        client,
        Method::GET,
        format!("{REPORTS}/recurring-message-stats/{message}"),
    )
    .query("account_id", account)
    .query("page_size", page_size)
    .query_opt("after", after)
    .send_with_headers::<Vec<serde_json::Value>>()
    .await?;
    Ok(Page {
        entries: response.body,
        next_cursor: pagination::next_cursor(&response.headers, "after"),
    })
}

pub async fn get_recurring_event_history(
    client: &Client,
    workflow: WorkflowId,
    event: RuleId,
    page_size: PageSize,
    start_token: Option<&Cursor>,
) -> Result<Page<serde_json::Value>, ApiError> {
    let response = ApiRequest::new(
        client,
        Method::GET,
        format!("{REPORTS}/recurring-events/{workflow}/events/{event}"),
    )
    .query("page-size", page_size)
    .query_opt("start-token", start_token)
    .send_with_headers::<Vec<serde_json::Value>>()
    .await?;
    Ok(Page {
        entries: response.body,
        next_cursor: pagination::next_cursor(&response.headers, "start-token"),
    })
}

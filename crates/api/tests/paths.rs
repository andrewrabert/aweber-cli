use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod support;

async fn server() -> MockServer {
    MockServer::start().await
}

#[tokio::test]
async fn campaign_stat_requests_the_versioned_path() {
    let server = server().await;
    Mock::given(method("GET"))
        .and(path(
            "/1.0/accounts/1/lists/2/campaigns/b3/stats/total_opens",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .expect(1)
        .mount(&server)
        .await;

    aweber::endpoints::get_campaign_stat(
        &support::client(&server),
        1,
        2,
        3,
        &aweber::types::GetAccountsListsCampaignsBcampaignidStats2StatsId::TotalOpens,
    )
    .await
    .expect("the mocked stat is returned");
}

#[tokio::test]
async fn get_campaign_requests_the_versioned_path() {
    let server = server().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/2/campaigns/b3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .expect(1)
        .mount(&server)
        .await;

    aweber::endpoints::get_campaign(&support::client(&server), 1, 2, "b", 3)
        .await
        .expect("the mocked campaign is returned");
}

#[tokio::test]
async fn subscriber_activity_requests_the_versioned_path() {
    let server = server().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/2/subscribers/3"))
        .and(query_param("ws.op", "getActivity"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .expect(1)
        .mount(&server)
        .await;

    aweber::endpoints::get_subscriber_activity(&support::client(&server), 1, 2, 3, None, None)
        .await
        .expect("the mocked activity collection is returned");
}

#[tokio::test]
async fn split_test_components_request_the_versioned_path() {
    let server = server().await;
    Mock::given(method("GET"))
        .and(path(
            "/1.0/accounts/1/lists/2/web_form_split_tests/3/components",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .expect(1)
        .mount(&server)
        .await;

    aweber::endpoints::list_web_form_split_test_components(
        &support::client(&server),
        1,
        2,
        3,
        None,
        None,
    )
    .await
    .expect("the mocked component collection is returned");
}

#[tokio::test]
async fn split_test_component_requests_the_versioned_path() {
    let server = server().await;
    Mock::given(method("GET"))
        .and(path(
            "/1.0/accounts/1/lists/2/web_form_split_tests/3/components/4",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .expect(1)
        .mount(&server)
        .await;

    aweber::endpoints::get_web_form_split_test_component(&support::client(&server), 1, 2, 3, 4)
        .await
        .expect("the mocked component is returned");
}

#[tokio::test]
async fn link_analytics_requests_the_beta_path_with_uuids() {
    let server = server().await;
    let account: aweber::ids::AccountUid = "11111111-1111-4111-8111-111111111111".parse().unwrap();
    let broadcast = uuid::Uuid::parse_str("22222222-2222-4222-8222-222222222222").unwrap();
    Mock::given(method("GET"))
        .and(path("/2.0-beta/analytics/reports/broadcasts-links"))
        .and(query_param("account_id", account.to_string()))
        .and(query_param("broadcast_id", broadcast.to_string()))
        .and(query_param("filter", "clicks"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .expect(1)
        .mount(&server)
        .await;

    let page = aweber::endpoints::get_broadcast_link_analytics(
        &support::client(&server),
        &account,
        broadcast,
        &aweber::types::GetBroadcastLinksAnalyticsFilter::Clicks,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .expect("the mocked link rows are returned");
    assert!(page.entries.is_empty());
    assert_eq!(page.next_cursor, None);
}

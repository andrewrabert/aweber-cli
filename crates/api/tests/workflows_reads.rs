#![cfg(feature = "workflows")]

mod support;

use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

const WORKFLOW: &str = "33333333-3333-4333-8333-333333333333";
const EVENT: &str = "44444444-4444-4444-8444-444444444444";
const OWNER: &str = "55555555-5555-4555-8555-555555555555";
const PARENT: &str = "66666666-6666-4666-8666-666666666666";

#[tokio::test]
async fn get_workflow_sends_include_ruleset() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/internal/campaign/campaigns/{WORKFLOW}")))
        .and(query_param("include", "ruleset"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": WORKFLOW,
            "name": "Welcome",
            "state": "draft",
            "precondition_version": 7,
            "ruleset": {},
        })))
        .expect(1)
        .mount(&server)
        .await;

    let workflow =
        aweber::workflows::get_workflow(&support::client(&server), WORKFLOW.parse().unwrap())
            .await
            .expect("the mocked workflow is returned");
    assert_eq!(
        workflow.name().map(|name| name.to_string()),
        Some("Welcome".to_string())
    );
    assert_eq!(
        workflow.ruleset_write_op(),
        aweber::workflows::PatchOperation::Add
    );
    assert!(workflow
        .ruleset()
        .expect("an empty ruleset reads")
        .working(aweber::workflows::Timezone::utc())
        .steps()
        .is_empty());
    assert_eq!(
        workflow.precondition_version().map(|v| v.to_string()),
        Some("7".to_string())
    );
}

#[tokio::test]
async fn list_workflows_sends_owner_and_parent_only() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/internal/campaign/campaigns"))
        .and(query_param("owner", OWNER))
        .and(query_param("parent", PARENT))
        .and(query_param_is_missing("include"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!([{ "id": WORKFLOW }])),
        )
        .expect(1)
        .mount(&server)
        .await;

    let workflows = aweber::workflows::list_workflows(
        &support::client(&server),
        OWNER.parse().unwrap(),
        PARENT.parse().unwrap(),
    )
    .await
    .expect("the mocked workflow list is returned");
    assert_eq!(workflows.len(), 1);
}

#[tokio::test]
async fn event_history_extracts_the_start_token_cursor() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!(
            "/internal/analytics-view/reports/recurring-events/{WORKFLOW}/events/{EVENT}"
        )))
        .and(query_param("page-size", "25"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header(
                    "link",
                    format!(
                        "<https://api.aweber.com/analytics-view/reports/recurring-events/\
                         {WORKFLOW}/events/{EVENT}/?start-token=1700000500>; rel=\"next\""
                    )
                    .as_str(),
                )
                .set_body_json(serde_json::json!([{ "window": 1 }])),
        )
        .expect(1)
        .mount(&server)
        .await;

    let page = aweber::workflows::get_recurring_event_history(
        &support::client(&server),
        WORKFLOW.parse().unwrap(),
        EVENT.parse().unwrap(),
        "25".parse().unwrap(),
        None,
    )
    .await
    .expect("the mocked window page is returned");
    assert_eq!(page.entries.len(), 1);
    assert_eq!(page.next_cursor, Some("1700000500".parse().unwrap()));
}

#![cfg(feature = "workflows")]

mod support;

use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const WORKFLOW: &str = "33333333-3333-4333-8333-333333333333";
const LIST: &str = "66666666-6666-4666-8666-666666666666";

fn document() -> serde_json::Value {
    serde_json::json!({
        "id": WORKFLOW,
        "name": "Welcome",
        "state": "draft",
        "precondition_version": 12,
    })
}

fn empty_graph() -> aweber::workflows::Graph {
    aweber::workflows::Ruleset::default().working(aweber::workflows::Timezone::utc())
}

async fn precondition(server: &MockServer) -> aweber::workflows::PreconditionVersion {
    Mock::given(method("GET"))
        .and(path(format!("/internal/campaign/campaigns/{WORKFLOW}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(document()))
        .mount(server)
        .await;
    aweber::workflows::get_workflow(&support::client(server), WORKFLOW.parse().unwrap())
        .await
        .expect("the mocked workflow is returned")
        .precondition_version()
        .expect("the document carries a precondition version")
}

#[tokio::test]
async fn every_mutation_sends_if_match_as_a_bare_integer() {
    let server = MockServer::start().await;
    let version = precondition(&server).await;
    let workflow: aweber::ids::WorkflowId = WORKFLOW.parse().unwrap();
    for (verb, suffix) in [
        ("PATCH", ""),
        ("DELETE", ""),
        ("POST", "/publish"),
        ("POST", "/revert"),
        ("POST", "/copy"),
    ] {
        Mock::given(method(verb))
            .and(path(format!(
                "/internal/campaign/campaigns/{WORKFLOW}{suffix}"
            )))
            .and(header("if-match", "12"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": WORKFLOW,
                "precondition_version": 13,
            })))
            .expect(1)
            .mount(&server)
            .await;
    }

    let client = support::client(&server);
    let patch = aweber::workflows::WorkflowPatch::edits(&aweber::workflows::WorkflowEdit {
        name: Some("Renamed".parse().unwrap()),
        ..aweber::workflows::WorkflowEdit::default()
    });
    aweber::workflows::update_workflow(&client, workflow, version, &patch)
        .await
        .expect("the patch is accepted");
    aweber::workflows::delete_workflow(&client, workflow, version)
        .await
        .expect("the delete is accepted");
    aweber::workflows::publish_workflow(&client, workflow, version)
        .await
        .expect("the publish is accepted");
    aweber::workflows::revert_workflow(&client, workflow, version)
        .await
        .expect("the revert is accepted");
    aweber::workflows::copy_workflow(
        &client,
        workflow,
        version,
        &aweber::workflows::CopyWorkflow {
            target_list: LIST.parse().unwrap(),
            name: Some("Copied".parse().unwrap()),
            timezone: None,
        },
    )
    .await
    .expect("the copy is accepted");
}

#[tokio::test]
async fn timezone_update_sends_update_times_true() {
    let server = MockServer::start().await;
    let version = precondition(&server).await;
    Mock::given(method("PATCH"))
        .and(path(format!("/internal/campaign/campaigns/{WORKFLOW}")))
        .and(query_param("update_times", "true"))
        .and(query_param("include", "ruleset"))
        .respond_with(ResponseTemplate::new(200).set_body_json(document()))
        .expect(1)
        .mount(&server)
        .await;

    let patch = aweber::workflows::WorkflowPatch::edits(&aweber::workflows::WorkflowEdit {
        timezone: Some("America/New_York".parse().unwrap()),
        ..aweber::workflows::WorkflowEdit::default()
    });
    aweber::workflows::update_workflow(
        &support::client(&server),
        WORKFLOW.parse().unwrap(),
        version,
        &patch,
    )
    .await
    .expect("the timezone patch is accepted");
}

#[tokio::test]
async fn the_first_ruleset_write_sends_add_and_later_writes_replace() {
    for (ruleset, expected) in [
        (serde_json::json!({}), "add"),
        (
            serde_json::json!({
                "unpublished_events": [],
                "unpublished_actions": [],
            }),
            "replace",
        ),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/internal/campaign/campaigns/{WORKFLOW}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": WORKFLOW,
                "precondition_version": 12,
                "ruleset": ruleset,
            })))
            .mount(&server)
            .await;
        let client = support::client(&server);
        let workflow_id: aweber::ids::WorkflowId = WORKFLOW.parse().unwrap();
        let workflow = aweber::workflows::get_workflow(&client, workflow_id)
            .await
            .expect("the mocked workflow is returned");
        let version = workflow
            .precondition_version()
            .expect("the document carries a precondition version");

        Mock::given(method("PATCH"))
            .and(path(format!("/internal/campaign/campaigns/{WORKFLOW}")))
            .and(body_json(serde_json::json!([
                {
                    "op": expected,
                    "path": "/ruleset/unpublished_actions",
                    "value": [],
                },
                {
                    "op": expected,
                    "path": "/ruleset/unpublished_events",
                    "value": [],
                },
            ])))
            .respond_with(ResponseTemplate::new(200).set_body_json(document()))
            .expect(1)
            .mount(&server)
            .await;

        let patch =
            aweber::workflows::WorkflowPatch::edits(&aweber::workflows::WorkflowEdit::default())
                .with_ruleset(workflow.ruleset_write_op(), &empty_graph());
        aweber::workflows::update_workflow(&client, workflow_id, version, &patch)
            .await
            .expect("the ruleset patch is accepted");
    }
}

#[tokio::test]
async fn a_412_surfaces_as_a_precondition_failure() {
    let server = MockServer::start().await;
    let version = precondition(&server).await;
    Mock::given(method("POST"))
        .and(path(format!(
            "/internal/campaign/campaigns/{WORKFLOW}/publish"
        )))
        .respond_with(ResponseTemplate::new(412).set_body_json(serde_json::json!({
            "detail": "Precondition `If-Match` does not match version",
        })))
        .expect(1)
        .mount(&server)
        .await;

    let error = aweber::workflows::publish_workflow(
        &support::client(&server),
        WORKFLOW.parse().unwrap(),
        version,
    )
    .await
    .expect_err("a stale precondition fails");
    assert!(matches!(
        error,
        aweber::client::ApiError::Http { status: 412, .. }
    ));
}

// The harness is shared with the workflows tests, which use far more of it.
#[allow(dead_code)]
mod support;

use support::fixtures::MESSAGE;
use support::Harness;

const ROUTE: &str = "/internal/message/messages/batch/get";

#[tokio::test]
async fn get_emits_only_the_public_message_fields() {
    let harness = Harness::start().await;
    harness
        .accept(
            "POST",
            ROUTE,
            serde_json::json!({ "messages": [{
                "id": MESSAGE,
                "account_id": 1,
                "list_id": 2,
                "subject": "Hello",
                "body_text": "Hi",
                "body_html": "<p>Hi</p>",
                "body_json": { "blocks": [] },
                "has_customized_body_text": true,
                "attachment_ids": [],
                "binding": "unbound",
                "bound_resource_id": null,
                "copied_from": null,
            }] }),
        )
        .await;

    let output = harness
        .command()
        .args(["messages", "get", MESSAGE])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        Harness::json(&output),
        serde_json::json!([{
            "id": MESSAGE,
            "subject": "Hello",
            "body_text": "Hi",
            "body_html": "<p>Hi</p>",
            "attachment_ids": [],
        }])
    );
}

#[tokio::test]
async fn get_fails_when_the_service_withholds_a_message() {
    let harness = Harness::start().await;
    harness
        .accept("POST", ROUTE, serde_json::json!({ "messages": [null] }))
        .await;

    let output = harness
        .command()
        .args(["messages", "get", MESSAGE])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        format!("error: message {MESSAGE} was not found")
    );
}

#[tokio::test]
async fn get_fails_when_the_service_refuses() {
    let harness = Harness::start().await;
    harness.refuse("POST", ROUTE, 401).await;

    let output = harness
        .command()
        .args(["messages", "get", MESSAGE])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(output.stdout.is_empty(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).starts_with("error: "),
        "{output:?}"
    );
}

#[tokio::test]
async fn get_rejects_a_malformed_message_id_before_any_request() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["messages", "get", "not-a-message-id"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("is not a 24 hex digit message id"),
        "{output:?}"
    );
    assert_eq!(harness.calls("POST", ROUTE).await, 0);
}

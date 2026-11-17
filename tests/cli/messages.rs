// The harness is shared with the workflows tests, which use far more of it.
#[allow(dead_code)]
mod support;

use support::fixtures::{MESSAGE, SECOND_MESSAGE};
use support::{Harness, ACCOUNT, WORKFLOW};

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
            "binding": "unbound",
            "bound_resource_id": null,
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

const LIST_ROUTE: &str = "/internal/message/message-editor/v1/accounts/1/lists/2/messages";
const SEARCH_ROUTE: &str = "/internal/message/messages/search";

fn query(request: &wiremock::Request) -> Vec<(String, String)> {
    request
        .url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect()
}

fn pair(key: &str, value: &str) -> (String, String) {
    (key.to_string(), value.to_string())
}

fn legacy_message() -> serde_json::Value {
    serde_json::json!({
        "_id": MESSAGE,
        "account_id": 1,
        "list_id": 2,
        "subject": "Hello",
        "body_amp": "",
        "body_text": "",
        "body_html": "",
        "body_json": {},
        "created_client": null,
        "has_customized_body_text": false,
        "attachments": [],
        "binding": "unbound",
        "bound_resource_id": null,
        "modified_at": "2026-11-17T00:00:00",
        "created_at": "2026-11-17T00:00:00",
        "copied_from": null,
    })
}

#[tokio::test]
async fn create_sends_empty_bodies_when_only_a_subject_is_given() {
    let harness = Harness::start().await;
    harness
        .expect(
            "POST",
            LIST_ROUTE,
            serde_json::json!({
                "account_id": 1,
                "list_id": 2,
                "subject": "Hello",
                "body_text": "",
                "body_html": "",
                "body_json": {},
                "attachment_ids": [],
            }),
            legacy_message(),
        )
        .await;

    let output = harness
        .command()
        .args(["messages", "create", "--list-id", "2", "--subject", "Hello"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(Harness::json(&output), legacy_message());
}

#[tokio::test]
async fn create_sends_the_given_content() {
    let harness = Harness::start().await;
    let mut document = tempfile::NamedTempFile::new().expect("a temporary file");
    std::io::Write::write_all(&mut document, br#"{"blocks": []}"#).expect("the file is written");
    harness
        .expect(
            "POST",
            LIST_ROUTE,
            serde_json::json!({
                "account_id": 1,
                "list_id": 2,
                "subject": "Hello",
                "body_text": "Hi",
                "body_html": "<p>Hi</p>",
                "body_json": { "blocks": [] },
                "attachment_ids": [SECOND_MESSAGE],
            }),
            legacy_message(),
        )
        .await;

    let output = harness
        .command()
        .args(["messages", "create", "--list-id", "2", "--subject", "Hello"])
        .args(["--body-text", "Hi", "--body-html", "<p>Hi</p>"])
        .arg("--body-json")
        .arg(document.path())
        .args(["--attachment", SECOND_MESSAGE])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
}

#[tokio::test]
async fn create_requires_a_subject() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["messages", "create", "--list-id", "2"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert_eq!(harness.calls("POST", LIST_ROUTE).await, 0);
}

fn summary(subject: &str) -> serde_json::Value {
    serde_json::json!({
        "_id": MESSAGE,
        "subject": subject,
        "binding": "unbound",
        "created_at": "2026-11-17T00:00:00",
        "modified_at": "2026-11-17T00:00:00",
    })
}

#[tokio::test]
async fn list_reads_unbound_messages_by_default() {
    let harness = Harness::start().await;
    harness
        .accept(
            "GET",
            LIST_ROUTE,
            serde_json::json!({ "content": [summary("Hello")] }),
        )
        .await;

    let output = harness
        .command()
        .args(["messages", "list", "--list-id", "2"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        Harness::json(&output),
        serde_json::json!([summary("Hello")])
    );
    let requests = harness.server_requests("GET", LIST_ROUTE).await;
    assert_eq!(requests.len(), 1);
    assert_eq!(
        query(&requests[0]),
        [
            pair("binding", "unbound"),
            pair("offset", "0"),
            pair("limit", "500"),
        ]
    );
}

#[tokio::test]
async fn list_sends_the_filters_and_the_page() {
    let harness = Harness::start().await;
    harness
        .accept("GET", LIST_ROUTE, serde_json::json!({ "content": [] }))
        .await;

    let output = harness
        .command()
        .args(["messages", "list", "--list-id", "2"])
        .args(["--binding", "broadcast", "--binding", "campaign"])
        .args(["--subject", "news", "--offset", "10", "--limit", "5"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(Harness::json(&output), serde_json::json!([]));
    let requests = harness.server_requests("GET", LIST_ROUTE).await;
    assert_eq!(requests.len(), 1);
    assert_eq!(
        query(&requests[0]),
        [
            pair("binding", "broadcast,campaign"),
            pair("subject", "news"),
            pair("offset", "10"),
            pair("limit", "5"),
        ]
    );
}

#[tokio::test]
async fn list_reads_further_pages_until_one_comes_back_short() {
    let harness = Harness::start().await;
    harness
        .accept(
            "GET",
            LIST_ROUTE,
            serde_json::json!({ "content": [summary("Hello")] }),
        )
        .await;

    let output = harness
        .command()
        .args(["messages", "list", "--list-id", "2", "--limit", "1"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(harness.calls("GET", LIST_ROUTE).await, 1);
}

#[tokio::test]
async fn find_sends_the_account_and_the_search() {
    let harness = Harness::start().await;
    harness
        .accept(
            "GET",
            SEARCH_ROUTE,
            serde_json::json!([{
                "id": MESSAGE,
                "account_id": 1,
                "list_id": 2,
                "subject": "Hello",
                "binding": "campaign",
            }]),
        )
        .await;

    let output = harness
        .command()
        .args(["messages", "find", "--binding", "campaign"])
        .args(["--bound-resource-id", WORKFLOW, "--subject", "Hello there"])
        .args(["--fields", "subject", "--fields", "binding"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        Harness::json(&output),
        serde_json::json!([{
            "id": MESSAGE,
            "account_id": 1,
            "list_id": 2,
            "subject": "Hello",
            "binding": "campaign",
        }])
    );
    let requests = harness.server_requests("GET", SEARCH_ROUTE).await;
    assert_eq!(requests.len(), 1);
    assert_eq!(
        query(&requests[0]),
        [
            pair("account", ACCOUNT),
            pair("binding", "campaign"),
            pair("bound_resource_id", WORKFLOW),
            pair("subject", "Hello there"),
            pair("fields", "subject,binding"),
        ]
    );
}

#[tokio::test]
async fn find_emits_only_the_ids_when_no_fields_are_requested() {
    let harness = Harness::start().await;
    harness
        .accept(
            "GET",
            SEARCH_ROUTE,
            serde_json::json!([{ "id": MESSAGE, "account_id": 1, "list_id": 2 }]),
        )
        .await;

    let output = harness
        .command()
        .args(["messages", "find", "--subject", "Hello"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        Harness::json(&output),
        serde_json::json!([{ "id": MESSAGE, "account_id": 1, "list_id": 2 }])
    );
    let requests = harness.server_requests("GET", SEARCH_ROUTE).await;
    assert_eq!(
        query(&requests[0]),
        [pair("account", ACCOUNT), pair("subject", "Hello")]
    );
}

#[tokio::test]
async fn find_requires_a_search_term() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["messages", "find", "--fields", "subject"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert_eq!(harness.calls("GET", SEARCH_ROUTE).await, 0);
}

#![cfg(feature = "workflows")]

mod support;

use wiremock::matchers::{body_json, method, path, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT: &str = "77777777-7777-4777-8777-777777777777";
const FIRST: &str = "aaaaaaaaaaaaaaaaaaaaaaaa";
const SECOND: &str = "bbbbbbbbbbbbbbbbbbbbbbbb";

fn messages() -> Vec<aweber::ids::MessageId> {
    vec![
        FIRST.parse().expect("24 hex digits"),
        SECOND.parse().expect("24 hex digits"),
    ]
}

fn first() -> aweber::ids::MessageId {
    FIRST.parse().expect("24 hex digits")
}

fn welcome() -> serde_json::Value {
    serde_json::json!({
        "id": FIRST,
        "account_id": 1,
        "list_id": 2,
        "subject": "Welcome",
        "body_text": "Hi",
        "body_html": "<p>Hi</p>",
        "body_json": { "blocks": [] },
        "has_customized_body_text": true,
        "attachment_ids": ["cccccccccccccccccccccccc"],
        "binding": "campaign",
        "bound_resource_id": ACCOUNT,
        "copied_from": { "id": 7 },
    })
}

#[tokio::test]
async fn batch_get_sends_no_fields_and_no_account_param() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/internal/message/messages/batch/get"))
        .and(query_param_is_missing("fields"))
        .and(query_param_is_missing("account"))
        .and(body_json(serde_json::json!({
            "message_ids": [FIRST],
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "messages": [welcome()],
        })))
        .expect(1)
        .mount(&server)
        .await;

    let contents = aweber::message::get_messages(&support::client(&server), &[first()])
        .await
        .expect("the mocked contents are returned");
    assert_eq!(contents.len(), 1);
    let first = &contents[&first()];
    assert_eq!(first.account_id, 1);
    assert_eq!(first.list_id, 2);
    assert_eq!(first.subject, "Welcome");
    assert_eq!(first.body_html, "<p>Hi</p>");
    assert_eq!(first.body_text, "Hi");
    assert_eq!(
        first.body_json,
        serde_json::json!({ "blocks": [] })
            .as_object()
            .cloned()
            .unwrap()
    );
    assert_eq!(
        first.attachment_ids,
        ["cccccccccccccccccccccccc".to_string()]
    );
    assert_eq!(first.binding, aweber::message::Binding::Campaign);
    assert_eq!(first.bound_resource_id, Some(ACCOUNT.to_string()));
    assert_eq!(
        first.copied_from,
        serde_json::json!({ "id": 7 }).as_object().cloned()
    );
    assert!(first.has_customized_body_text);
}

#[tokio::test]
async fn batch_get_rejects_a_message_with_missing_fields() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/internal/message/messages/batch/get"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "messages": [{ "id": FIRST, "subject": "Welcome" }],
        })))
        .mount(&server)
        .await;

    aweber::message::get_messages(&support::client(&server), &[first()])
        .await
        .expect_err("a message without its body fields does not deserialize");
}

#[tokio::test]
async fn batch_get_reports_a_null_message_as_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/internal/message/messages/batch/get"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "messages": [welcome(), null],
        })))
        .mount(&server)
        .await;

    let error = aweber::message::get_messages(&support::client(&server), &messages())
        .await
        .expect_err("the null message is missing");
    match error {
        aweber::client::ApiError::MessagesNotFound(missing) => {
            assert_eq!(
                missing,
                vec![SECOND.parse::<aweber::ids::MessageId>().unwrap()]
            );
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[tokio::test]
async fn batch_unbind_sends_the_account_and_message_ids() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/internal/message/messages/batch/unbind"))
        .and(body_json(serde_json::json!({
            "account": ACCOUNT,
            "message_ids": [FIRST, SECOND],
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "processed": [FIRST],
            "unprocessed": [SECOND],
        })))
        .expect(1)
        .mount(&server)
        .await;

    let error = aweber::message::unbind_messages(
        &support::client(&server),
        ACCOUNT.parse().unwrap(),
        &messages(),
    )
    .await
    .expect_err("one message was not unbound");
    match error {
        aweber::client::ApiError::MessagesNotUnbound(unprocessed) => {
            assert_eq!(
                unprocessed,
                vec![SECOND.parse::<aweber::ids::MessageId>().unwrap()]
            );
        }
        other => panic!("unexpected error: {other}"),
    }
}

#[tokio::test]
async fn batch_delete_sends_the_account_and_message_ids() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/internal/message/messages/batch/delete"))
        .and(body_json(serde_json::json!({
            "account": ACCOUNT,
            "message_ids": [FIRST, SECOND],
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "processed": [FIRST, SECOND],
            "unprocessed": [],
        })))
        .expect(1)
        .mount(&server)
        .await;

    aweber::message::delete_messages(
        &support::client(&server),
        ACCOUNT.parse().unwrap(),
        &messages(),
    )
    .await
    .expect("every message was deleted");
}

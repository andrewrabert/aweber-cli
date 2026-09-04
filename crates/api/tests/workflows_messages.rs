#![cfg(feature = "workflows")]

mod support;

use wiremock::matchers::{body_json, method, path, query_param};
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

#[tokio::test]
async fn batch_get_sends_message_ids_and_reads_subjects() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/internal/message/messages/batch/get"))
        .and(query_param("fields", "subject"))
        .and(body_json(serde_json::json!({
            "message_ids": [FIRST, SECOND],
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "messages": [
                { "id": FIRST, "subject": "Welcome" },
                { "id": SECOND, "subject": "Day two" },
            ],
        })))
        .expect(1)
        .mount(&server)
        .await;

    let subjects = aweber::workflows::get_message_subjects(&support::client(&server), &messages())
        .await
        .expect("the mocked subjects are returned");
    assert_eq!(
        subjects
            .into_iter()
            .map(|(id, subject)| (id.to_string(), subject))
            .collect::<Vec<_>>(),
        vec![
            (FIRST.to_string(), "Welcome".to_string()),
            (SECOND.to_string(), "Day two".to_string()),
        ]
    );
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

    let outcome = aweber::workflows::unbind_messages(
        &support::client(&server),
        ACCOUNT.parse().unwrap(),
        &messages(),
    )
    .await
    .expect("the mocked outcome is returned");
    assert_eq!(outcome.processed.len(), 1);
    assert_eq!(outcome.unprocessed.len(), 1);
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

    let outcome = aweber::workflows::delete_messages(
        &support::client(&server),
        ACCOUNT.parse().unwrap(),
        &messages(),
    )
    .await
    .expect("the mocked outcome is returned");
    assert_eq!(outcome.processed.len(), 2);
    assert!(outcome.unprocessed.is_empty());
}

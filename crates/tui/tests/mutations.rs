//! A write that fails blocks; a read that fails does not.

use aweber::catalog::{ArgValue, Args, Operation, ValueKind};
use aweber_tui::core::state::{Account, Overlay, View};
use aweber_tui::core::{Action, State};
use aweber_tui::harness::Harness;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// The crypto provider wiremock's TLS stack expects, installed once.
fn crypto() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

fn harness(server: &MockServer) -> Harness {
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: Some("00000000-0000-0000-0000-000000000000".parse().unwrap()),
        name: Some("Acme".to_string()),
    });
    state.session = aweber_tui::ports::SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    Harness::against(&server.uri(), 80, 24).with_state(state)
}

fn integer(value: &str) -> ArgValue {
    ArgValue::new(ValueKind::Integer, value)
}

/// A `404` on a delete opens the blocking modal and leaves the collection it was
/// run from on screen.
#[tokio::test]
async fn a_mutation_failure_blocks() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "entries": [{ "id": 7, "name": "Weekly" }],
            "total_size": 1,
        })))
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/1.0/accounts/1/lists/7/subscribers/3"))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_json(serde_json::json!({ "error": { "message": "no such subscriber" } })),
        )
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness
        .dispatch(Action::OpenOperation {
            operation: Operation::ListLists,
            args: Args::default(),
        })
        .settle()
        .await;

    let mut args = Args::default();
    args.set("list-id", integer("7"));
    args.set("subscriber-id", integer("3"));
    harness
        .dispatch(Action::RunOperation {
            operation: Operation::DeleteSubscriber,
            args,
        })
        .settle()
        .await;
    for character in "delete subscriber".chars() {
        if character == ' ' {
            harness.key("Space");
        } else {
            harness.key(&character.to_string());
        }
    }
    harness.key("Enter").settle().await;

    match &harness.state().overlay {
        Some(Overlay::Error(failure)) => {
            assert_eq!(failure.status, Some(404), "the modal states the status");
        }
        _ => panic!("a failed mutation blocks"),
    }
    assert!(
        matches!(harness.state().view(), View::Collection(_)),
        "the collection it was run from is still on screen"
    );
    assert_eq!(
        harness.state().inflight.mutating,
        0,
        "the failure released the mutation count"
    );
}

/// A `403` on a read still renders inline.
#[tokio::test]
async fn a_read_failure_stays_inline() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_json(serde_json::json!({ "error": { "message": "not yours" } })),
        )
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness
        .dispatch(Action::OpenOperation {
            operation: Operation::ListLists,
            args: Args::default(),
        })
        .settle()
        .await;

    assert!(
        harness.state().overlay.is_none(),
        "a read failure opens no modal"
    );
    let View::Collection(view) = harness.state().view() else {
        panic!("the collection is on top");
    };
    assert_eq!(
        view.unavailable.as_ref().and_then(|failure| failure.status),
        Some(403),
        "the collection says why it is empty"
    );
}

/// A re-read that itself fails re-opens the editor on the edit that was made.
#[tokio::test]
async fn a_failed_re_read_keeps_the_edit() {
    crypto();
    let server = MockServer::start().await;
    let workflow = uuid::Uuid::nil();
    Mock::given(method("PATCH"))
        .and(path(format!("/internal/campaign/campaigns/{workflow}")))
        .respond_with(
            ResponseTemplate::new(412)
                .set_body_json(serde_json::json!({ "error": { "message": "stale version" } })),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("/internal/campaign/campaigns/{workflow}")))
        .respond_with(
            ResponseTemplate::new(500)
                .set_body_json(serde_json::json!({ "error": { "message": "gone away" } })),
        )
        .mount(&server)
        .await;

    let edit = r#"[{"op":"replace","path":"/name","value":"Renamed"}]"#;
    let mut harness = harness(&server).with_editor_text(edit);
    let mut args = Args::default();
    args.set(
        "workflow",
        ArgValue::new(ValueKind::Uuid, workflow.to_string()),
    );
    args.set(aweber::catalog::PRECONDITION_ARG, integer("4"));
    args.set("patch", ArgValue::new(ValueKind::Json, edit.to_string()));

    harness
        .dispatch(Action::RunOperation {
            operation: Operation::UpdateWorkflow,
            args,
        })
        .settle()
        .await;
    harness.key("y").settle().await;

    assert_eq!(
        harness.suspended_seed().as_deref(),
        Some(edit),
        "the editor re-opened on the edit that was made"
    );
    assert!(
        harness
            .state()
            .log
            .entries()
            .iter()
            .any(|entry| entry.status == Some(412)),
        "the lost race is in the Event Log:\n{}",
        harness.state().log.text()
    );
}

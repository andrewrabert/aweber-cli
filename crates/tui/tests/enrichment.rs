//! A workflow says more than it was sent, and says so without blocking.

use aweber::catalog::{ArgValue, Args, Operation, ValueKind};
use aweber_tui::core::collection::Delivered;
use aweber_tui::core::effect::Purpose;
use aweber_tui::core::state::{Account, View};
use aweber_tui::core::{Action, Effect, RequestId, State, update};
use aweber_tui::harness::{Harness, fixed_now};
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

/// The same state the harness is seeded with, driven through the reducer alone.
fn state() -> State {
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
    state
}

const WORKFLOW: &str = "33333333-3333-4333-8333-333333333333";
const MESSAGE: &str = "0123456789abcdef01234567";

/// A workflow whose one action sends one message.
fn workflow() -> serde_json::Value {
    serde_json::json!({
        "id": WORKFLOW,
        "name": "Welcome",
        "state": "draft",
        "timezone": "UTC",
        "ruleset": {
            "events": [{
                "id": "11111111-1111-4111-8111-111111111111",
                "parents": [],
                "type": "subscribe.v1",
                "filter": null,
                "title": null,
                "metadata": "subscribe.v1",
            }],
            "actions": [{
                "id": "22222222-2222-4222-8222-222222222222",
                "parents": ["11111111-1111-4111-8111-111111111111"],
                "definition": {
                    "function": "ruleset.email.action.compose_v1",
                    "kwargs": {
                        "account": "<event:account>",
                        "list": "<event:list>",
                        "meapi_id": MESSAGE,
                        "message": format!("<message:new message={MESSAGE}>"),
                        "recipient": "<event:recipient>",
                    },
                },
                "title": null,
                "metadata": "send-message",
                "recurring": false,
            }],
        },
    })
}

/// A ruleset the reader refuses: a set-branch rule with no branches.
fn unwalkable() -> serde_json::Value {
    serde_json::json!({
        "id": WORKFLOW,
        "name": "Broken",
        "state": "draft",
        "timezone": "UTC",
        "ruleset": {
            "events": [{
                "id": "11111111-1111-4111-8111-111111111111",
                "parents": [],
                "type": "subscribe.v1",
                "filter": null,
                "title": null,
                "metadata": "subscribe.v1",
            }],
            "actions": [{
                "id": "22222222-2222-4222-8222-222222222222",
                "parents": ["11111111-1111-4111-8111-111111111111"],
                "definition": {
                    "function": "rulesengine.action.set_branch",
                    "kwargs": {},
                },
                "title": null,
                "metadata": "set-branch",
                "recurring": false,
            }],
        },
    })
}

async fn get_workflow(server: &MockServer, document: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path(format!("/internal/campaign/campaigns/{WORKFLOW}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(document))
        .mount(server)
        .await;
}

fn open_workflow() -> Action {
    let mut args = Args::default();
    args.set("workflow", ArgValue::new(ValueKind::Uuid, WORKFLOW));
    Action::OpenOperation {
        operation: Operation::GetWorkflow,
        args,
    }
}

/// The enrichment rows of the detail on top.
fn enrichment(harness: &Harness) -> Vec<(&'static str, Result<serde_json::Value, String>)> {
    match harness.state().view() {
        View::Detail(view) => view
            .enrichment
            .iter()
            .map(|entry| (entry.label, entry.value.clone()))
            .collect(),
        _ => panic!("the detail is on top"),
    }
}

/// A workflow detail states its message subjects and its visualization.
#[tokio::test]
async fn a_workflow_detail_is_enriched() {
    crypto();
    let server = MockServer::start().await;
    get_workflow(&server, workflow()).await;
    Mock::given(method("POST"))
        .and(path("/internal/message/messages/batch/get"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "messages": { MESSAGE: { "subject": "Welcome aboard" } },
        })))
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness.dispatch(open_workflow()).settle().await;

    let rows = enrichment(&harness);
    let subjects = rows
        .iter()
        .find(|(label, _)| *label == "message subjects")
        .expect("the subjects were asked for");
    assert!(
        format!("{:?}", subjects.1).contains("Welcome aboard"),
        "the subject of the message it sends is stated: {:?}",
        subjects.1
    );
    let visualization = rows
        .iter()
        .find(|(label, _)| *label == "visualization")
        .expect("the visualization was walked");
    assert!(
        visualization.1.is_ok(),
        "a walkable ruleset states its visualization"
    );
    assert!(harness.state().overlay.is_none(), "nothing blocked");
}

/// A failed subject read marks that row unavailable with its reason, logs it,
/// leaves the document on screen, and opens no modal.
#[tokio::test]
async fn a_failed_enrichment_degrades() {
    crypto();
    let server = MockServer::start().await;
    get_workflow(&server, workflow()).await;
    Mock::given(method("POST"))
        .and(path("/internal/message/messages/batch/get"))
        .respond_with(
            ResponseTemplate::new(503)
                .set_body_json(serde_json::json!({ "error": { "message": "no editor today" } })),
        )
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness.dispatch(open_workflow()).settle().await;

    let rows = enrichment(&harness);
    let subjects = rows
        .iter()
        .find(|(label, _)| *label == "message subjects")
        .expect("the subjects were asked for");
    assert!(
        subjects.1.is_err(),
        "the row states its reason in place of its value"
    );
    assert!(harness.state().overlay.is_none(), "nothing blocked");
    let View::Detail(view) = harness.state().view() else {
        panic!("the detail is on top");
    };
    assert!(view.document.is_some(), "the document is still on screen");
    assert!(
        harness
            .state()
            .log
            .entries()
            .iter()
            .any(|entry| entry.path == "message subjects"),
        "the failure reached the Event Log:\n{}",
        harness.state().log.text()
    );
}

/// An unwalkable ruleset marks the visualization unavailable and nothing else.
#[tokio::test]
async fn an_unwalkable_ruleset_degrades() {
    crypto();
    let server = MockServer::start().await;
    get_workflow(&server, unwalkable()).await;

    let mut harness = harness(&server);
    harness.dispatch(open_workflow()).settle().await;

    let rows = enrichment(&harness);
    let visualization = rows
        .iter()
        .find(|(label, _)| *label == "visualization")
        .expect("the visualization was attempted");
    assert!(
        visualization.1.is_err(),
        "the row states why it could not be walked"
    );
    assert!(harness.state().overlay.is_none(), "nothing blocked");
    let View::Detail(view) = harness.state().view() else {
        panic!("the detail is on top");
    };
    assert!(view.document.is_some(), "the document is still on screen");
    assert!(
        harness
            .state()
            .log
            .entries()
            .iter()
            .any(|entry| entry.path == "visualization"),
        "the failure reached the Event Log:\n{}",
        harness.state().log.text()
    );
}

/// Subjects delivered while another view covers the workflow detail that asked
/// for them land on that detail.
#[tokio::test]
async fn an_enrichment_reaches_a_buried_detail() {
    let mut state = state();
    let effects = update(&mut state, fixed_now(), open_workflow());
    let generation = effects
        .iter()
        .find_map(|effect| match effect {
            Effect::Send { generation, .. } => Some(*generation),
            _ => None,
        })
        .expect("the workflow was asked for");

    let effects = update(
        &mut state,
        fixed_now(),
        Action::Delivered {
            request: RequestId::default(),
            generation,
            purpose: Purpose::Document {
                operation: Operation::GetWorkflow,
            },
            outcome: Ok(Delivered {
                status: 200,
                document: Some(workflow()),
                entries: Vec::new(),
                next: None,
                capped: None,
                total: None,
            }),
        },
    );
    let (enrichment_generation, of) = effects
        .iter()
        .find_map(|effect| match effect {
            Effect::Send {
                generation,
                purpose: Purpose::Enrichment { of },
                ..
            } => Some((*generation, of.clone())),
            _ => None,
        })
        .expect("the subjects were asked for");

    update(&mut state, fixed_now(), Action::OpenEventLog);
    assert!(
        matches!(state.view(), View::EventLog(_)),
        "the Event Log covers the detail"
    );

    update(
        &mut state,
        fixed_now(),
        Action::Delivered {
            request: RequestId::default(),
            generation: enrichment_generation,
            purpose: Purpose::Enrichment { of },
            outcome: Ok(Delivered {
                status: 200,
                document: Some(serde_json::json!({
                    "messages": { MESSAGE: { "subject": "Welcome aboard" } },
                })),
                entries: Vec::new(),
                next: None,
                capped: None,
                total: None,
            }),
        },
    );

    update(&mut state, fixed_now(), Action::Pop);
    let View::Detail(view) = state.view() else {
        panic!("the detail is on top");
    };
    let subjects = view
        .enrichment
        .iter()
        .find(|entry| entry.label == "message subjects")
        .expect("the subjects reached the buried detail");
    assert!(
        format!("{:?}", subjects.value).contains("Welcome aboard"),
        "the subjects never reached the buried detail: {:?}",
        subjects.value
    );
}

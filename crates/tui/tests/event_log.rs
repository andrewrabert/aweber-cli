//! Every attempt is attributed to the request that made it.

use aweber_tui::core::state::{Account, View};
use aweber_tui::core::{Action, LogId, State};
use aweber_tui::harness::Harness;
use wiremock::matchers::{method, path, path_regex};
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
        uuid: None,
        name: None,
    });
    state.session = aweber_tui::ports::SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    Harness::against(&server.uri(), 80, 24).with_state(state)
}

fn integer(value: &str) -> aweber::catalog::ArgValue {
    aweber::catalog::ArgValue::new(aweber::catalog::ValueKind::Integer, value)
}

/// A document read of one list, so four of them are four requests.
fn read(list: &str) -> Action {
    let mut args = aweber::catalog::Args::default();
    args.set("list-id", integer(list));
    Action::OpenOperation {
        operation: aweber::catalog::Operation::GetList,
        args,
    }
}

/// Four requests in flight at once each log their own method and path, and no
/// entry is blank.
#[tokio::test]
async fn concurrent_requests_are_attributed_to_themselves() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/1\.0/accounts/1/lists/\d+$"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(std::time::Duration::from_millis(40))
                .set_body_json(serde_json::json!({ "id": 1, "name": "a list" })),
        )
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    for list in ["11", "12", "13", "14"] {
        harness.dispatch(read(list));
    }
    assert_eq!(
        harness.state().inflight.outstanding(),
        0,
        "nothing has started yet"
    );
    harness.settle().await;

    let entries = harness.state().log.entries().to_vec();
    assert!(
        entries
            .iter()
            .all(|entry| !entry.method.is_empty() && !entry.path.is_empty()),
        "no entry is blank:\n{}",
        harness.state().log.text()
    );
    for list in ["11", "12", "13", "14"] {
        let suffix = format!("/1.0/accounts/1/lists/{list}");
        assert!(
            entries
                .iter()
                .filter(|entry| entry.path.ends_with(&suffix))
                .count()
                >= 2,
            "{suffix} logged its own start and finish:\n{}",
            harness.state().log.text()
        );
    }
    assert_eq!(
        harness.state().inflight.outstanding(),
        0,
        "every request answered"
    );
}

/// A retried request logs both attempts under one entry's method and path.
#[tokio::test]
async fn a_retry_keeps_its_method_and_path() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "0"))
        .up_to_n_times(1)
        .with_priority(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({ "id": 7, "name": "a" })),
        )
        .with_priority(2)
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness.dispatch(read("7")).settle().await;

    let entries = harness.state().log.entries().to_vec();
    let mine: Vec<_> = entries
        .iter()
        .filter(|entry| entry.path.ends_with("/1.0/accounts/1/lists/7"))
        .collect();
    assert!(
        mine.iter().any(|entry| entry.attempt == 2),
        "the retry is logged as a second attempt:\n{}",
        harness.state().log.text()
    );
    assert!(
        mine.iter().all(|entry| entry.method == "GET"),
        "every attempt kept its method:\n{}",
        harness.state().log.text()
    );
    assert!(
        mine.iter().any(|entry| entry.waited.is_some()),
        "the wait it took is logged:\n{}",
        harness.state().log.text()
    );
    assert!(
        matches!(harness.state().view(), View::Detail(_)),
        "the retried request answered"
    );
}

/// Every settled request is forgotten, so a long session's in-flight table stays
/// as small as what is in flight.
#[tokio::test]
async fn a_settled_request_is_forgotten() {
    let mut harness = Harness::new(80, 24);
    let request = LogId::default();
    harness
        .dispatch(Action::RequestStarted {
            request,
            attempt: 1,
            method: "GET".to_string(),
            path: "/first".to_string(),
            body: None,
        })
        .dispatch(Action::RequestFinished {
            request,
            attempt: 1,
            status: 200,
            duration: std::time::Duration::from_millis(1),
            body: None,
        })
        .dispatch(Action::RequestSettled { request });
    assert_eq!(harness.state().inflight.outstanding(), 0);

    harness
        .dispatch(Action::RequestStarted {
            request,
            attempt: 1,
            method: "POST".to_string(),
            path: "/second".to_string(),
            body: None,
        })
        .dispatch(Action::RequestFinished {
            request,
            attempt: 1,
            status: 201,
            duration: std::time::Duration::from_millis(1),
            body: None,
        });

    let entries = harness.state().log.entries().to_vec();
    let last = entries.last().expect("the second request was logged");
    assert_eq!(
        (last.method.as_str(), last.path.as_str()),
        ("POST", "/second"),
        "the settled request was forgotten rather than answered for:\n{}",
        harness.state().log.text()
    );
    assert!(
        entries.iter().all(|entry| !entry.method.is_empty()),
        "no entry is blank:\n{}",
        harness.state().log.text()
    );
}

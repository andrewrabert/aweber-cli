//! The whole loop, against a server, with only the terminal stubbed.

use aweber_tui::core::state::{Account, Overlay, View};
use aweber_tui::core::{Action, State};
use aweber_tui::harness::Harness;
use wiremock::matchers::{method, path, path_regex, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// The crypto provider wiremock's TLS stack expects, installed once.
fn crypto() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

/// A harness against the server, with the account already resolved.
fn harness(server: &MockServer) -> Harness {
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: None,
        name: Some("Acme".to_string()),
    });
    state.session = aweber_tui::ports::SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    Harness::against(&server.uri(), 80, 24).with_state(state)
}

fn collection(entries: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "entries": entries,
        "total_size": entries.as_array().map(Vec::len).unwrap_or(0),
    }))
}

/// The palette, opened and typed into, as the user would.
async fn choose(harness: &mut Harness, label: &str) {
    harness.key("Ctrl-p");
    for character in label.chars() {
        if character == ' ' {
            harness.key("Space");
        } else {
            harness.key(&character.to_string());
        }
    }
    harness.key("Enter").settle().await;
    // An operation with filters to configure opens its builder first; sending
    // it is one more Enter.
    if matches!(harness.state().view(), View::Builder(_)) {
        harness.key("Enter").settle().await;
    }
}

#[tokio::test]
async fn browsing_walks_the_stack_against_wiremock() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists"))
        .and(query_param_is_missing("ws.start"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "entries": [{ "id": 7, "name": "Weekly" }],
            "total_size": 2,
            "next_collection_link": format!("{}/1.0/accounts/1/lists?ws.start=1", server.uri()),
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists"))
        .and(query_param("ws.start", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "entries": [{ "id": 8, "name": "Monthly" }],
            "total_size": 2,
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/1\.0/accounts/1/lists/\d+/subscribers$"))
        .respond_with(collection(serde_json::json!([
            { "id": 42, "email": "one@example.com", "status": "subscribed" }
        ])))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7/subscribers/42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 42,
            "email": "one@example.com",
            "status": "subscribed",
        })))
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    choose(&mut harness, "lists list").await;
    let View::Collection(view) = harness.state().view() else {
        panic!("the lists are on top")
    };
    assert_eq!(
        view.rows.len(),
        2,
        "both pages arrived without a keystroke: {}",
        view.count_label()
    );
    assert_eq!(view.count_label(), "2 of 2");

    harness.key("Enter").settle().await;
    assert!(
        matches!(harness.state().view(), View::Detail(_)),
        "Enter opens the list"
    );
    assert_eq!(
        harness.state().context.list.as_ref().map(|list| list.id),
        Some(7),
        "the list became the ambient list"
    );

    choose(&mut harness, "subscribers list").await;
    let View::Collection(view) = harness.state().view() else {
        panic!("the subscribers are on top")
    };
    assert_eq!(view.rows.len(), 1);

    harness.key("Enter").settle().await;
    let View::Detail(view) = harness.state().view() else {
        panic!("the subscriber is on top")
    };
    assert_eq!(
        view.document
            .as_ref()
            .and_then(|document| document.pointer("/email"))
            .and_then(serde_json::Value::as_str),
        Some("one@example.com")
    );
    assert_eq!(harness.state().breadcrumb().len(), 5);
}

#[tokio::test]
async fn a_mutation_refreshes_the_open_collection() {
    crypto();
    let server = MockServer::start().await;
    let broadcasts = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7/broadcasts"))
        .respond_with(collection(serde_json::json!([
            { "id": 9, "subject": "Hello", "status": "scheduled" }
        ])))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/1.0/accounts/1/lists/7/broadcasts/9/cancel"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let _ = broadcasts;

    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: None,
        name: None,
    });
    state.context.list = Some(aweber_tui::core::state::ListRef {
        id: 7,
        uuid: None,
        name: Some("Weekly".to_string()),
        self_link: None,
    });
    let mut harness = Harness::against(&server.uri(), 80, 24).with_state(state);

    choose(&mut harness, "broadcasts list").await;
    let View::Collection(view) = harness.state().view() else {
        panic!("the broadcasts are on top")
    };
    assert_eq!(view.rows.len(), 1);

    choose(&mut harness, "broadcasts cancel").await;
    assert!(
        matches!(harness.state().overlay, Some(Overlay::Confirm(_))),
        "a mutation confirms first"
    );
    harness.key("y").settle().await;
    assert!(
        harness
            .state()
            .toasts
            .iter()
            .any(|toast| toast.text.contains("204")),
        "the empty-bodied mutation still says what happened: {:?}",
        harness
            .state()
            .toasts
            .iter()
            .map(|toast| toast.text.clone())
            .collect::<Vec<String>>()
    );
    let View::Collection(view) = harness.state().view() else {
        panic!("the broadcasts are still on top")
    };
    assert_eq!(
        view.rows.len(),
        1,
        "the invalidated collection asked the server again"
    );
    assert!(
        server
            .received_requests()
            .await
            .unwrap_or_default()
            .iter()
            .filter(|request| request.url.path() == "/1.0/accounts/1/lists/7/broadcasts")
            .count()
            >= 2,
        "the collection was fetched twice"
    );
}

#[tokio::test]
async fn a_precondition_race_never_replays_silently() {
    crypto();
    let server = MockServer::start().await;
    let workflow = "11111111-1111-4111-8111-111111111111";
    Mock::given(method("PATCH"))
        .and(path_regex(r"^/internal/campaign/campaigns/.*"))
        .respond_with(ResponseTemplate::new(412).set_body_json(serde_json::json!({
            "error": { "message": "another writer changed it" }
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/internal/campaign/campaigns/.*"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": workflow,
            "name": "Welcome",
            "precondition_version": 9,
        })))
        .mount(&server)
        .await;

    let mut harness = harness(&server).with_editor_text(r#"{"events":[],"actions":[]}"#);
    let mut args = aweber::catalog::Args::default();
    args.set(
        "workflow",
        aweber::catalog::ArgValue::new(aweber::catalog::ValueKind::Uuid, workflow),
    );
    args.set(
        aweber::catalog::PRECONDITION_ARG,
        aweber::catalog::ArgValue::new(aweber::catalog::ValueKind::Integer, "3"),
    );
    args.set(
        "file",
        aweber::catalog::ArgValue::new(
            aweber::catalog::ValueKind::Json,
            r#"{"events":[{"kind":"tag"}],"actions":[]}"#,
        ),
    );
    harness
        .dispatch(Action::RunOperation {
            operation: aweber::catalog::Operation::UpdateWorkflowRuleset,
            args,
        })
        .settle()
        .await;
    harness.key("y").settle().await;

    let log = harness.state().log.text();
    assert!(
        log.contains("another writer changed it"),
        "the log says a writer got there first:\n{log}"
    );
    assert!(
        log.contains("precondition 3 is stale; the current version is 9"),
        "the log names both versions:\n{log}"
    );
    assert!(
        harness
            .suspended_seed()
            .is_some_and(|seed| seed.contains("tag")),
        "the editor re-opened with the user's edit intact"
    );
}

#[tokio::test]
async fn a_dead_session_lands_on_the_session_view() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": { "message": "the token is no good" }
        })))
        .up_to_n_times(1)
        .with_priority(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists"))
        .respond_with(collection(
            serde_json::json!([{ "id": 7, "name": "Weekly" }]),
        ))
        .with_priority(2)
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    choose(&mut harness, "lists list").await;
    assert!(
        matches!(harness.state().view(), View::Session(_)),
        "an unrecoverable session takes the screen"
    );
    assert!(harness.state().held.is_some(), "the action is held");

    harness
        .dispatch(Action::LoginFinished(Ok(
            aweber_tui::ports::SessionStatus::Active {
                account_id: 1,
                expires_in_secs: 3600,
            },
        )))
        .settle()
        .await;
    assert!(
        matches!(harness.state().view(), View::Collection(_)),
        "the held action was replayed after the login"
    );
    assert!(harness.state().held.is_none());
}

#[tokio::test]
async fn a_watch_completes_from_another_view() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7/broadcasts/9"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 9,
            "subject": "Hello",
            "status": "sent",
        })))
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    let mut args = aweber::catalog::Args::default();
    args.set(
        "list-id",
        aweber::catalog::ArgValue::new(aweber::catalog::ValueKind::Integer, "7"),
    );
    args.set(
        "broadcast-id",
        aweber::catalog::ArgValue::new(aweber::catalog::ValueKind::Integer, "9"),
    );
    harness
        .dispatch(Action::OpenOperation {
            operation: aweber::catalog::Operation::WaitBroadcast,
            args,
        })
        .settle()
        .await;
    assert!(
        harness
            .state()
            .toasts
            .iter()
            .any(|toast| toast.text.contains("is sent")),
        "a finished watch says so from wherever the user is: {:?}",
        harness
            .state()
            .toasts
            .iter()
            .map(|toast| toast.text.clone())
            .collect::<Vec<String>>()
    );
    assert!(
        harness.state().watches.is_empty(),
        "a finished watch leaves the status bar"
    );
}

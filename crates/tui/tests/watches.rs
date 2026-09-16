//! A watch is the user's to stop, from the palette or from its own cell.

use aweber::catalog::{ArgValue, Args, Operation, ValueKind};
use aweber_tui::core::state::Account;
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
        uuid: None,
        name: Some("Acme".to_string()),
    });
    state.session = aweber_tui::ports::SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    Harness::against(&server.uri(), 80, 24).with_state(state)
}

fn watch(broadcast: &str) -> Action {
    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    args.set("broadcast-id", ArgValue::new(ValueKind::Integer, broadcast));
    Action::OpenOperation {
        operation: Operation::WaitBroadcast,
        args,
    }
}

/// A broadcast that never leaves the queue, so the watch keeps running.
async fn queued(server: &MockServer, broadcast: &str) {
    Mock::given(method("GET"))
        .and(path(format!(
            "/1.0/accounts/1/lists/7/broadcasts/{broadcast}"
        )))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({ "id": 1, "status": "queued" })),
        )
        .mount(server)
        .await;
}

/// `Ctrl-w` lists the running watches and choosing one stops it, from any view.
#[tokio::test]
async fn a_watch_is_cancelled_from_the_palette() {
    crypto();
    let server = MockServer::start().await;
    queued(&server, "9").await;

    let mut harness = harness(&server);
    harness.dispatch(watch("9")).settle().await;
    assert_eq!(harness.state().watches.len(), 1, "the watch is running");

    harness.key("Ctrl-w");
    assert!(
        matches!(
            harness.state().overlay,
            Some(aweber_tui::core::state::Overlay::Palette(_))
        ),
        "Ctrl-w opens the palette over the watches"
    );
    harness.key("Enter").settle().await;

    assert!(
        harness.state().watches.is_empty(),
        "choosing the watch stopped it"
    );
    assert_eq!(harness.tracked(), 0, "its task was cancelled with it");
    assert!(
        harness
            .state()
            .toasts
            .iter()
            .any(|toast| toast.text.starts_with("stopped watching")),
        "the user is told which watch stopped"
    );
}

/// A click on a watch in the status bar stops that watch and no other.
#[tokio::test]
async fn a_watch_is_cancelled_by_its_own_cell() {
    crypto();
    let server = MockServer::start().await;
    queued(&server, "9").await;
    queued(&server, "10").await;

    let mut harness = harness(&server);
    harness.dispatch(watch("9")).settle().await;
    harness.dispatch(watch("10")).settle().await;
    assert_eq!(harness.state().watches.len(), 2);

    harness.screen();
    let (second, area) = harness.state().regions.watches[1];
    harness.click(area.x, area.y);

    let running: Vec<_> = harness.state().watches.iter().map(|held| held.id).collect();
    assert_eq!(running.len(), 1, "one watch was stopped");
    assert!(!running.contains(&second), "the one under the cell stopped");
}

/// A cancelled watch leaves no token behind, and neither does a request that has
/// answered.
#[tokio::test]
async fn nothing_outlives_what_asked_for_it() {
    crypto();
    let server = MockServer::start().await;
    queued(&server, "9").await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({ "entries": [], "total_size": 0 })),
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
    assert_eq!(
        harness.tracked(),
        0,
        "a request that answered holds nothing"
    );

    harness.dispatch(watch("9")).settle().await;
    assert_eq!(harness.tracked(), 1, "the watch is the only task left");

    let running = harness.state().watches[0].id;
    harness
        .dispatch(Action::CancelWatch(running))
        .settle()
        .await;
    assert_eq!(harness.tracked(), 0, "the cancelled watch left nothing");
}

/// A watch cancelled while its poll is in flight leaves nothing outstanding and
/// no throttle marker, so the status bar states what is running.
#[tokio::test]
async fn a_cancelled_poll_leaves_nothing_in_flight() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7/broadcasts/9"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(std::time::Duration::from_secs(30))
                .set_body_json(serde_json::json!({ "id": 1, "status": "queued" })),
        )
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness.dispatch(watch("9"));
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let running = harness.state().watches[0].id;

    harness
        .dispatch(Action::CancelWatch(running))
        .settle()
        .await;

    assert_eq!(harness.tracked(), 0, "the cancelled watch left nothing");
    assert_eq!(
        harness.state().inflight.outstanding(),
        0,
        "the cancelled poll is still counted as in flight"
    );
    assert!(
        !harness.state().inflight.throttled(),
        "the cancelled poll is still throttling"
    );
}

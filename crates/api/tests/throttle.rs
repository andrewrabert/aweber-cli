use std::sync::{Arc, Mutex};
use std::time::Duration;

use aweber::client::{Client, RequestEvent, RequestKey, RequestObserver, RetryPolicy, Sleeper};
use aweber::session::BoxFuture;
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Default)]
struct Recorder {
    waits: Mutex<Vec<Duration>>,
}

impl Recorder {
    fn waits(&self) -> Vec<Duration> {
        self.waits.lock().unwrap().clone()
    }
}

impl Sleeper for Recorder {
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        self.waits.lock().unwrap().push(duration);
        Box::pin(std::future::ready(()))
    }
}

#[derive(Default)]
struct Events {
    seen: Mutex<Vec<(RequestKey, String)>>,
}

impl Events {
    fn seen(&self) -> Vec<(RequestKey, String)> {
        self.seen.lock().unwrap().clone()
    }

    fn lines(&self) -> Vec<String> {
        self.seen()
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
    }

    fn keys(&self) -> Vec<RequestKey> {
        let mut keys: Vec<RequestKey> = self.seen().into_iter().map(|(key, _)| key).collect();
        keys.dedup();
        keys.sort();
        keys.dedup();
        keys
    }

    /// The lines one key saw, in the order the observer saw them.
    fn under(&self, key: RequestKey) -> Vec<String> {
        self.seen()
            .into_iter()
            .filter(|(seen, _)| *seen == key)
            .map(|(_, line)| line)
            .collect()
    }
}

impl RequestObserver for Events {
    fn observe(&self, request: RequestKey, event: RequestEvent) {
        let line = match event {
            RequestEvent::Started { attempt, url, .. } => format!("started {attempt} {url}"),
            RequestEvent::Finished {
                attempt, status, ..
            } => format!("finished {attempt} {status}"),
            RequestEvent::Waiting {
                attempt,
                status,
                wait,
            } => format!("waiting {attempt} {status} {}", wait.as_secs()),
            RequestEvent::Refreshed { attempt } => format!("refreshed {attempt}"),
            RequestEvent::Failed { attempt, .. } => format!("failed {attempt}"),
            RequestEvent::Settled { attempts } => format!("settled {attempts}"),
        };
        self.seen.lock().unwrap().push((request, line));
    }
}

fn client(server: &MockServer, sleeper: Arc<dyn Sleeper>) -> Client {
    static PROVIDER: std::sync::Once = std::sync::Once::new();
    PROVIDER.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
    Client::new(&server.uri()).with_retry_policy(RetryPolicy {
        attempts: 3,
        base_backoff: Duration::from_millis(500),
        max_backoff: Duration::from_secs(30),
        sleeper,
    })
}

#[tokio::test]
async fn a_429_honours_retry_after() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "2")
                .set_body_string("slow down"),
        )
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": 1})))
        .mount(&server)
        .await;
    let recorder = Arc::new(Recorder::default());
    let client = client(&server, recorder.clone());

    let body: serde_json::Value = client
        .get_url(&format!("{}/1.0/accounts", server.uri()))
        .await
        .unwrap();

    assert_eq!(body, serde_json::json!({"id": 1}));
    assert_eq!(recorder.waits(), vec![Duration::from_secs(2)]);
}

#[tokio::test]
async fn three_429s_exhaust_the_budget() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "2")
                .set_body_string("slow down"),
        )
        .mount(&server)
        .await;
    let recorder = Arc::new(Recorder::default());
    let client = client(&server, recorder.clone());

    let error = client
        .get_url::<serde_json::Value>(&format!("{}/1.0/accounts", server.uri()))
        .await
        .unwrap_err();

    assert_eq!(error.status(), Some(429));
    assert_eq!(server.received_requests().await.unwrap().len(), 3);
    assert_eq!(recorder.waits().len(), 2);
}

#[tokio::test]
async fn every_retry_is_observed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "2")
                .set_body_string("slow down"),
        )
        .mount(&server)
        .await;
    let events = Arc::new(Events::default());
    let client = client(&server, Arc::new(Recorder::default())).with_observer(events.clone());

    let _ = client
        .get_url::<serde_json::Value>(&format!("{}/1.0/accounts", server.uri()))
        .await;

    let url = format!("{}/1.0/accounts", server.uri());
    assert_eq!(
        events.lines(),
        vec![
            format!("started 1 {url}"),
            "finished 1 429".to_string(),
            "waiting 1 429 2".to_string(),
            format!("started 2 {url}"),
            "finished 2 429".to_string(),
            "waiting 2 429 2".to_string(),
            format!("started 3 {url}"),
            "finished 3 429".to_string(),
            "settled 3".to_string(),
        ]
    );
    assert_eq!(events.keys().len(), 1);
}

#[tokio::test]
async fn a_retry_after_date_is_honoured() {
    let server = MockServer::start().await;
    let at = chrono::Utc::now() + chrono::Duration::seconds(5);
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", at.to_rfc2822().as_str())
                .set_body_string("slow down"),
        )
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": 1})))
        .mount(&server)
        .await;
    let recorder = Arc::new(Recorder::default());
    let client = client(&server, recorder.clone());

    let body: serde_json::Value = client
        .get_url(&format!("{}/1.0/accounts", server.uri()))
        .await
        .unwrap();

    assert_eq!(body, serde_json::json!({"id": 1}));
    let waits = recorder.waits();
    assert_eq!(waits.len(), 1);
    assert!(
        waits[0] > Duration::from_secs(2) && waits[0] <= Duration::from_secs(5),
        "an HTTP-date five seconds out waited {:?}",
        waits[0]
    );
}

#[tokio::test]
async fn an_outlandish_retry_after_is_capped() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "86400")
                .set_body_string("slow down"),
        )
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": 1})))
        .mount(&server)
        .await;
    let recorder = Arc::new(Recorder::default());
    let client = client(&server, recorder.clone());

    let _: serde_json::Value = client
        .get_url(&format!("{}/1.0/accounts", server.uri()))
        .await
        .unwrap();

    assert_eq!(recorder.waits(), vec![Duration::from_secs(30)]);
}

/// A backoff that never ends, so a request dropped while it waits is dropped
/// inside the sleep.
struct Endless;

impl Sleeper for Endless {
    fn sleep(&self, _duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(std::future::pending())
    }
}

#[tokio::test]
async fn a_dropped_request_settles() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"id": 1}))
                .set_delay(Duration::from_secs(30)),
        )
        .mount(&server)
        .await;
    let events = Arc::new(Events::default());
    let client = client(&server, Arc::new(Recorder::default())).with_observer(events.clone());

    let url = format!("{}/1.0/accounts", server.uri());
    let dropped = tokio::time::timeout(
        Duration::from_millis(200),
        client.get_url::<serde_json::Value>(&url),
    )
    .await;

    assert!(dropped.is_err(), "the send answered instead of dropping");
    assert_eq!(
        events.lines(),
        vec![format!("started 1 {url}"), "settled 1".to_string()]
    );
    assert_eq!(events.keys().len(), 1);
}

#[tokio::test]
async fn a_request_dropped_in_a_backoff_settles() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "2")
                .set_body_string("slow down"),
        )
        .mount(&server)
        .await;
    let events = Arc::new(Events::default());
    let client = client(&server, Arc::new(Endless)).with_observer(events.clone());

    let url = format!("{}/1.0/accounts", server.uri());
    let dropped = tokio::time::timeout(
        Duration::from_millis(200),
        client.get_url::<serde_json::Value>(&url),
    )
    .await;

    assert!(dropped.is_err(), "the send answered instead of dropping");
    assert_eq!(
        events.lines(),
        vec![
            format!("started 1 {url}"),
            "finished 1 429".to_string(),
            "waiting 1 429 2".to_string(),
            "settled 1".to_string(),
        ]
    );
}

#[tokio::test]
async fn concurrent_requests_carry_their_own_keys() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": 1})))
        .mount(&server)
        .await;
    let events = Arc::new(Events::default());
    let client = client(&server, Arc::new(Recorder::default())).with_observer(events.clone());

    let lists = format!("{}/1.0/lists", server.uri());
    let accounts = format!("{}/1.0/accounts", server.uri());
    let (one, two) = tokio::join!(
        client.get_url::<serde_json::Value>(&lists),
        client.get_url::<serde_json::Value>(&accounts),
    );
    one.unwrap();
    two.unwrap();

    let keys = events.keys();
    assert_eq!(keys.len(), 2);
    for key in keys {
        let lines = events.under(key);
        let started: Vec<_> = lines
            .iter()
            .filter(|line| line.starts_with("started "))
            .collect();
        assert_eq!(started.len(), 1, "one key, one first attempt: {lines:?}");
        assert_eq!(lines.last().map(String::as_str), Some("settled 1"));
    }
}

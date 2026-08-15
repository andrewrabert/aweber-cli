use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use aweber::client::{ApiError, Client};
use aweber::session::{BoxFuture, Session, SessionError, TokenSource};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

struct Rotating {
    calls: AtomicUsize,
    token: String,
}

impl Rotating {
    fn new(token: &str) -> Arc<Rotating> {
        Arc::new(Rotating {
            calls: AtomicUsize::new(0),
            token: token.to_string(),
        })
    }

    fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

impl TokenSource for Rotating {
    fn refresh(&self) -> BoxFuture<'_, Result<String, SessionError>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(self.token.clone())
        })
    }
}

struct Refused;

impl TokenSource for Refused {
    fn refresh(&self) -> BoxFuture<'_, Result<String, SessionError>> {
        Box::pin(async {
            Err(SessionError::Refused {
                status: 400,
                body: "{\"error\":\"invalid_grant\"}".to_string(),
            })
        })
    }
}

fn client(server: &MockServer, session: Arc<Session>) -> Client {
    static PROVIDER: std::sync::Once = std::sync::Once::new();
    PROVIDER.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
    Client::with_session(&server.uri(), session).unwrap()
}

async fn unauthorized_then_ok(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/1.0/accounts"))
        .and(header("authorization", "Bearer old"))
        .respond_with(ResponseTemplate::new(401).set_body_string("{}"))
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts"))
        .and(header("authorization", "Bearer new"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": 1})))
        .mount(server)
        .await;
}

#[tokio::test]
async fn a_401_refreshes_and_retries_once() {
    let server = MockServer::start().await;
    unauthorized_then_ok(&server).await;
    let source = Rotating::new("new");
    let session = Arc::new(Session::new("old".to_string(), source.clone()));
    let client = client(&server, session);

    let body: serde_json::Value = client
        .get_url(&format!("{}/1.0/accounts", server.uri()))
        .await
        .unwrap();

    assert_eq!(body, serde_json::json!({"id": 1}));
    assert_eq!(source.calls(), 1);
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

#[tokio::test]
async fn concurrent_401s_await_one_refresh() {
    let server = MockServer::start().await;
    unauthorized_then_ok(&server).await;
    let source = Rotating::new("new");
    let session = Arc::new(Session::new("old".to_string(), source.clone()));
    let client = client(&server, session);
    let url = format!("{}/1.0/accounts", server.uri());

    let (first, second) = tokio::join!(
        client.get_url::<serde_json::Value>(&url),
        client.get_url::<serde_json::Value>(&url)
    );

    assert_eq!(first.unwrap(), serde_json::json!({"id": 1}));
    assert_eq!(second.unwrap(), serde_json::json!({"id": 1}));
    assert_eq!(source.calls(), 1);
}

#[tokio::test]
async fn a_second_401_is_an_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(401).set_body_string("denied"))
        .mount(&server)
        .await;
    let source = Rotating::new("new");
    let session = Arc::new(Session::new("old".to_string(), source.clone()));
    let client = client(&server, session);

    let error = client
        .get_url::<serde_json::Value>(&format!("{}/1.0/accounts", server.uri()))
        .await
        .unwrap_err();

    assert_eq!(error.status(), Some(401));
    assert_eq!(source.calls(), 1);
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

#[tokio::test]
async fn a_failed_refresh_is_a_session_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(401).set_body_string("denied"))
        .mount(&server)
        .await;
    let session = Arc::new(Session::new("old".to_string(), Arc::new(Refused)));
    let client = client(&server, session);

    let error = client
        .get_url::<serde_json::Value>(&format!("{}/1.0/accounts", server.uri()))
        .await
        .unwrap_err();

    match error {
        ApiError::Session(SessionError::Refused { status, .. }) => assert_eq!(status, 400),
        other => panic!("expected a session error, got {other:?}"),
    }
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn the_final_attempt_still_refreshes() {
    let server = MockServer::start().await;
    unauthorized_then_ok(&server).await;
    let source = Rotating::new("new");
    let session = Arc::new(Session::new("old".to_string(), source.clone()));
    let client = client(&server, session).with_retry_policy(aweber::client::RetryPolicy {
        attempts: 1,
        ..aweber::client::RetryPolicy::default()
    });

    let body: serde_json::Value = client
        .get_url(&format!("{}/1.0/accounts", server.uri()))
        .await
        .unwrap();

    assert_eq!(body, serde_json::json!({"id": 1}));
    assert_eq!(source.calls(), 1);
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

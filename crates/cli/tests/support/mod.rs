pub mod fixtures;

use std::io::Write as _;

use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub const ACCOUNT_ID: &str = "1";
pub const ACCOUNT: &str = "11111111-1111-4111-8111-111111111111";
pub const LIST: &str = "66666666-6666-4666-8666-666666666666";
pub const LIST_NAME: &str = "Main";
pub const WORKFLOW: &str = "33333333-3333-4333-8333-333333333333";

pub struct Harness {
    server: MockServer,
    credentials: tempfile::NamedTempFile,
}

impl Harness {
    pub async fn start() -> Harness {
        let server = MockServer::start().await;
        let mut credentials = tempfile::NamedTempFile::new().expect("a temporary file");
        let document = serde_json::json!({
            "access_token": "test-token",
            "refresh_token": "test-refresh",
            "expires_at": 4_102_444_800u64,
            "account_id": ACCOUNT_ID,
            "account": ACCOUNT,
            "api_url": server.uri(),
        });
        credentials
            .write_all(document.to_string().as_bytes())
            .expect("the credentials are written");
        credentials.flush().expect("the credentials are flushed");
        Harness {
            server,
            credentials,
        }
    }

    pub async fn start_with_stale_credentials() -> Harness {
        let harness = Harness::start().await;
        let document = serde_json::json!({
            "access_token": "test-token",
            "refresh_token": "test-refresh",
            "expires_at": 4_102_444_800u64,
            "account_id": ACCOUNT_ID,
            "account_uuid": ACCOUNT,
            "api_url": harness.server.uri(),
        });
        std::fs::write(harness.credentials.path(), document.to_string())
            .expect("the credentials are rewritten");
        Mock::given(method("GET"))
            .and(path(format!("/1.0/accounts/{ACCOUNT_ID}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "uuid": ACCOUNT,
            })))
            .mount(&harness.server)
            .await;
        harness
    }

    pub async fn with_list(&self, name: &str, list: &str) -> &Harness {
        Mock::given(method("GET"))
            .and(path(format!("/1.0/accounts/{ACCOUNT_ID}/lists")))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "entries": [{ "id": 42, "name": name }],
            })))
            .mount(&self.server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/1.0/accounts/{ACCOUNT_ID}/lists/42")))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 42,
                "name": name,
                "uuid": list,
            })))
            .mount(&self.server)
            .await;
        self
    }

    pub async fn with_workflows(&self, entries: serde_json::Value) -> &Harness {
        Mock::given(method("GET"))
            .and(path("/internal/campaign/campaigns"))
            .respond_with(ResponseTemplate::new(200).set_body_json(entries))
            .mount(&self.server)
            .await;
        self
    }

    pub async fn with_workflow(&self, document: serde_json::Value) -> &Harness {
        Mock::given(method("GET"))
            .and(path(format!("/internal/campaign/campaigns/{WORKFLOW}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(document))
            .mount(&self.server)
            .await;
        self
    }

    pub async fn with_subjects(&self, subjects: serde_json::Value) -> &Harness {
        Mock::given(method("POST"))
            .and(path("/internal/message/messages/batch/get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(subjects))
            .mount(&self.server)
            .await;
        self
    }

    pub async fn expect(
        &self,
        verb: &str,
        route: &str,
        body: serde_json::Value,
        response: serde_json::Value,
    ) -> &Harness {
        Mock::given(method(verb))
            .and(path(route.to_string()))
            .and(body_json(body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .expect(1)
            .mount(&self.server)
            .await;
        self
    }

    pub async fn accept(&self, verb: &str, route: &str, response: serde_json::Value) -> &Harness {
        Mock::given(method(verb))
            .and(path(route.to_string()))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.server)
            .await;
        self
    }

    pub async fn refuse(&self, verb: &str, route: &str, status: u16) -> &Harness {
        Mock::given(method(verb))
            .and(path(route.to_string()))
            .respond_with(
                ResponseTemplate::new(status).set_body_json(serde_json::json!({
                    "detail": "refused",
                })),
            )
            .mount(&self.server)
            .await;
        self
    }

    pub async fn bodies(&self, verb: &str, route: &str) -> Vec<serde_json::Value> {
        self.server
            .received_requests()
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|request| request.method.as_str().eq_ignore_ascii_case(verb))
            .filter(|request| request.url.path() == route)
            .filter_map(|request| serde_json::from_slice(&request.body).ok())
            .collect()
    }

    pub async fn calls(&self, verb: &str, route: &str) -> usize {
        self.server
            .received_requests()
            .await
            .unwrap_or_default()
            .into_iter()
            .filter(|request| request.method.as_str().eq_ignore_ascii_case(verb))
            .filter(|request| request.url.path() == route)
            .count()
    }

    pub fn command(&self) -> assert_cmd::Command {
        let mut command =
            assert_cmd::Command::cargo_bin("aweber").expect("the aweber binary is built");
        command.arg("--credentials-file").arg(
            self.credentials
                .path()
                .to_str()
                .expect("the temporary path is utf-8"),
        );
        command
    }

    pub fn json(output: &std::process::Output) -> serde_json::Value {
        serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
            panic!(
                "stdout is one JSON object ({e}): {}",
                String::from_utf8_lossy(&output.stdout)
            )
        })
    }
}

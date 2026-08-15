//! A failed request, said in the terms the screen needs: what was asked for,
//! what came back, and whether it belongs inline or in a modal.

#[derive(Clone, Debug)]
pub struct Failure {
    pub status: Option<u16>,
    pub method: String,
    pub path: String,
    /// `/error/message` from the body when it carries one.
    pub message: String,
    pub kind: FailureKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FailureKind {
    Http,
    /// The refresh failed; the Session view takes over and the action is held.
    Session,
    Transport,
    Decode,
    /// A lost precondition race, naming the version that was sent.
    Precondition {
        stale: Option<i64>,
    },
}

impl Failure {
    pub fn from_error(
        method: &reqwest::Method,
        path: &str,
        error: &aweber::client::ApiError,
    ) -> Failure {
        let (status, message, kind) = match error {
            aweber::client::ApiError::Http { status, body } => {
                let message = api_message(body).unwrap_or_else(|| summarize(body));
                let kind = if *status == 412 {
                    FailureKind::Precondition { stale: None }
                } else {
                    FailureKind::Http
                };
                (Some(*status), message, kind)
            }
            aweber::client::ApiError::Request(error) => {
                (None, error.to_string(), FailureKind::Transport)
            }
            aweber::client::ApiError::Deserialize { source, .. } => {
                (None, source.to_string(), FailureKind::Decode)
            }
            aweber::client::ApiError::Session(error) => {
                (None, error.to_string(), FailureKind::Session)
            }
        };
        Failure {
            status,
            method: method.to_string(),
            path: path.to_string(),
            message: crate::core::redact::scrub(&message),
            kind,
        }
    }

    /// 403 and 404 render inline rather than as a modal.
    pub fn inline(&self) -> bool {
        matches!(self.status, Some(403) | Some(404))
    }

    pub fn text(&self) -> String {
        match self.status {
            Some(status) => format!("{} {} — {status} {}", self.method, self.path, self.message),
            None => format!("{} {} — {}", self.method, self.path, self.message),
        }
    }
}

fn api_message(body: &str) -> Option<String> {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()?
        .pointer("/error/message")
        .and_then(serde_json::Value::as_str)
        .map(ToString::to_string)
}

/// A body with no `/error/message` is shown as its first line.
fn summarize(body: &str) -> String {
    let line = body.lines().next().unwrap_or_default().trim();
    if line.is_empty() {
        "no message".to_string()
    } else {
        line.to_string()
    }
}

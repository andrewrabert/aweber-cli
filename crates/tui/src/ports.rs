//! Everything outside the process the core is allowed to reach, named as traits
//! so the core is exercised without a terminal, a clock, or a network.

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

pub trait Http: Send + Sync + 'static {
    /// At most four requests are in flight at once.
    fn send(
        &self,
        plan: aweber::catalog::RequestPlan,
    ) -> BoxFuture<'static, Result<aweber::client::PlanResponse, aweber::client::ApiError>>;
}

pub trait Clock: Send + Sync + 'static {
    fn now(&self) -> chrono::DateTime<chrono::FixedOffset>;
}

pub trait Clipboard: Send + Sync + 'static {
    /// Native, then OSC 52, then a temp file whose path comes back in the route.
    fn copy(&self, text: &str) -> Result<CopyRoute, CopyError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CopyRoute {
    Native,
    Osc52,
    TempFile(std::path::PathBuf),
}

#[derive(Clone, Debug)]
pub struct CopyError {
    pub reason: String,
}

impl std::fmt::Display for CopyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.reason)
    }
}

pub trait Editor: Send + Sync + 'static {
    /// The terminal is already suspended when this is called.
    fn edit(&self, seed: &str, extension: &str) -> BoxFuture<'static, Result<String, EditorError>>;
}

#[derive(Clone, Debug)]
pub enum EditorError {
    NotConfigured,
    Failed { reason: String },
    Empty,
}

impl std::fmt::Display for EditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorError::NotConfigured => f.write_str("no editor is configured"),
            EditorError::Failed { reason } => write!(f, "the editor failed: {reason}"),
            EditorError::Empty => f.write_str("the editor saved nothing"),
        }
    }
}

pub trait SessionPort: Send + Sync + 'static {
    fn status(&self) -> BoxFuture<'static, SessionStatus>;

    /// A fresh PKCE pair and the URL that authorizes it.
    fn authorization(&self) -> Authorization;

    /// Exchanges the code, persists the grant, and rotates the live client's token.
    fn login(
        &self,
        code: String,
        verifier: crate::Secret,
    ) -> BoxFuture<'static, Result<SessionStatus, String>>;

    fn logout(&self) -> BoxFuture<'static, Result<(), String>>;
}

#[derive(Clone, Debug)]
pub struct Authorization {
    pub url: String,
    pub verifier: crate::Secret,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SessionStatus {
    Missing,
    Active {
        account_id: i32,
        expires_in_secs: u64,
    },
    Expired {
        account_id: i32,
    },
}

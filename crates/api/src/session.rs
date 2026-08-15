use std::sync::{Arc, RwLock};

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

/// The generation of an access token, incremented by every completed refresh.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Generation(u64);

/// The refresh grant and its persistence, owned by the front end.
pub trait TokenSource: Send + Sync + 'static {
    /// Rotate the stored refresh token and persist the new pair before returning
    /// the fresh access token.
    fn refresh(&self) -> BoxFuture<'_, Result<String, SessionError>>;
}

struct Live {
    access_token: Option<String>,
    generation: Generation,
}

/// The live access token plus the means to replace it.
pub struct Session {
    live: RwLock<Live>,
    source: Option<Arc<dyn TokenSource>>,
    gate: tokio::sync::Mutex<()>,
}

impl Session {
    pub fn new(access_token: String, source: Arc<dyn TokenSource>) -> Session {
        Session {
            live: RwLock::new(Live {
                access_token: Some(access_token),
                generation: Generation(0),
            }),
            source: Some(source),
            gate: tokio::sync::Mutex::new(()),
        }
    }

    /// A token that cannot be refreshed: `--token`, and the unauthenticated auth calls.
    pub fn fixed(access_token: Option<String>) -> Session {
        Session {
            live: RwLock::new(Live {
                access_token,
                generation: Generation(0),
            }),
            source: None,
            gate: tokio::sync::Mutex::new(()),
        }
    }

    pub fn token(&self) -> (Generation, Option<String>) {
        let live = self.live.read().expect("session lock poisoned");
        (live.generation, live.access_token.clone())
    }

    /// Replace the access token, retiring every generation seen before now.
    pub fn set_token(&self, access_token: String) -> Generation {
        let mut live = self.live.write().expect("session lock poisoned");
        live.access_token = Some(access_token);
        live.generation = Generation(live.generation.0 + 1);
        live.generation
    }

    pub fn is_refreshable(&self) -> bool {
        self.source.is_some()
    }

    /// Exchange the refresh grant for a new access token.
    ///
    /// Concurrent callers await the one refresh in progress; a caller whose
    /// generation is already superseded takes the new token without refreshing.
    pub async fn refresh(&self, seen: Generation) -> Result<String, SessionError> {
        let _guard = self.gate.lock().await;
        if let (generation, Some(token)) = self.token()
            && generation != seen
        {
            return Ok(token);
        }
        let source = self.source.as_ref().ok_or(SessionError::NoRefreshToken)?;
        let access_token = source.refresh().await?;
        self.set_token(access_token.clone());
        Ok(access_token)
    }
}

#[derive(Debug)]
pub enum SessionError {
    NoRefreshToken,
    Refused { status: u16, body: String },
    Transport(String),
    Persist(String),
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::NoRefreshToken => write!(f, "no refresh token available"),
            SessionError::Refused { status, body } => {
                write!(f, "token refresh refused: HTTP {status}: {body}")
            }
            SessionError::Transport(e) => write!(f, "token refresh failed: {e}"),
            SessionError::Persist(e) => write!(f, "could not persist refreshed tokens: {e}"),
        }
    }
}

impl std::error::Error for SessionError {}

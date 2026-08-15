use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::auth::DEFAULT_CLIENT_ID;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Credentials {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64,
    pub account_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<aweber::ids::AccountUid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
}

impl Credentials {
    pub(crate) fn client_id(&self) -> &str {
        self.client_id.as_deref().unwrap_or(DEFAULT_CLIENT_ID)
    }

    pub(crate) fn auth_url(&self) -> &str {
        self.auth_url
            .as_deref()
            .unwrap_or(aweber::oauth::DEFAULT_AUTH_URL)
    }
}

/// The stored account the credentials file names.
pub(crate) struct Stored {
    pub access_token: String,
    pub account_id: String,
    pub account: Option<aweber::ids::AccountUid>,
}

/// The credentials file, serializing its own rewrites.
#[derive(Clone)]
pub(crate) struct Store {
    inner: std::sync::Arc<StoreInner>,
}

struct StoreInner {
    path: PathBuf,
    rewrite: tokio::sync::Mutex<()>,
}

impl Store {
    pub(crate) fn new(override_path: Option<&Path>) -> Result<Store> {
        Ok(Store {
            inner: std::sync::Arc::new(StoreInner {
                path: path(override_path)?,
                rewrite: tokio::sync::Mutex::new(()),
            }),
        })
    }

    pub(crate) fn load(&self) -> Result<Credentials> {
        let json =
            std::fs::read_to_string(&self.inner.path).context("could not read credentials file")?;
        Ok(serde_json::from_str(&json)?)
    }

    pub(crate) fn save(&self, creds: &Credentials) -> Result<()> {
        write(creds, &self.inner.path)
    }

    pub(crate) fn forget(&self) -> Result<()> {
        match std::fs::remove_file(&self.inner.path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(anyhow::Error::new(e).context("could not delete credentials file")),
        }
    }

    /// Pre-emptively refresh when the stored expiry is near, then hand back a
    /// session that can refresh again from inside a request.
    pub(crate) async fn open(
        &self,
        token: Option<&str>,
    ) -> Result<(
        std::sync::Arc<aweber::session::Session>,
        Option<aweber_tui::StoredAccount>,
    )> {
        if let Some(token) = token {
            return Ok((
                std::sync::Arc::new(aweber::session::Session::fixed(Some(token.to_string()))),
                None,
            ));
        }

        let creds = self
            .load()
            .context("not logged in — run `aweber auth login` first")?;

        const EXPIRY_BUFFER_SECS: u64 = 60;
        let stored = if creds.expires_at > now_secs() + EXPIRY_BUFFER_SECS {
            Stored {
                access_token: creds.access_token,
                account_id: creds.account_id,
                account: creds.account,
            }
        } else {
            let access_token = self.rotate().await.context("token refresh failed")?;
            let creds = self.load()?;
            Stored {
                access_token,
                account_id: creds.account_id,
                account: creds.account,
            }
        };

        let session = std::sync::Arc::new(aweber::session::Session::new(
            stored.access_token.clone(),
            std::sync::Arc::new(self.clone()),
        ));
        let account = aweber_tui::StoredAccount {
            id: stored
                .account_id
                .parse()
                .context("invalid account_id in stored credentials")?,
            uuid: stored.account,
        };
        Ok((session, Some(account)))
    }

    /// What the stored credentials say the session is, without a request.
    pub(crate) fn status(&self) -> aweber_tui::ports::SessionStatus {
        let Ok(creds) = self.load() else {
            return aweber_tui::ports::SessionStatus::Missing;
        };
        let Ok(account_id) = creds.account_id.parse() else {
            return aweber_tui::ports::SessionStatus::Missing;
        };
        let now = now_secs();
        if creds.expires_at > now {
            aweber_tui::ports::SessionStatus::Active {
                account_id,
                expires_in_secs: creds.expires_at - now,
            }
        } else {
            aweber_tui::ports::SessionStatus::Expired { account_id }
        }
    }

    /// Exchange the stored refresh grant and persist the rotated pair.
    async fn rotate(&self) -> Result<String, aweber::session::SessionError> {
        let _guard = self.inner.rewrite.lock().await;
        let creds = self
            .load()
            .map_err(|e| aweber::session::SessionError::Persist(e.to_string()))?;
        let client = aweber::client::Client::new(creds.auth_url());
        let tokens =
            aweber::oauth::refresh_tokens(&client, creds.client_id(), &creds.refresh_token)
                .await
                .map_err(|e| match e {
                    aweber::client::ApiError::Http { status, body } => {
                        aweber::session::SessionError::Refused { status, body }
                    }
                    other => aweber::session::SessionError::Transport(other.to_string()),
                })?;
        let new_creds = Credentials {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_at: now_secs() + tokens.expires_in,
            account_id: creds.account_id,
            account: creds.account,
            client_id: creds.client_id,
            api_url: creds.api_url,
            auth_url: creds.auth_url,
        };
        self.save(&new_creds)
            .map_err(|e| aweber::session::SessionError::Persist(e.to_string()))?;
        Ok(new_creds.access_token)
    }
}

impl aweber::session::TokenSource for Store {
    fn refresh(
        &self,
    ) -> aweber::session::BoxFuture<'_, Result<String, aweber::session::SessionError>> {
        Box::pin(self.rotate())
    }
}

pub(crate) fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub(crate) fn path(override_path: Option<&Path>) -> Result<PathBuf> {
    match override_path {
        Some(path) => Ok(path.to_path_buf()),
        None => {
            let config_dir = dirs::config_dir().context("could not determine config directory")?;
            Ok(config_dir.join("aweber").join("credentials.json"))
        }
    }
}

pub(crate) fn load(override_path: Option<&Path>) -> Result<Credentials> {
    let path = path(override_path)?;
    let json = std::fs::read_to_string(&path).context("could not read credentials file")?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn save(creds: &Credentials, override_path: Option<&Path>) -> Result<()> {
    write(creds, &path(override_path)?)
}

fn write(creds: &Credentials, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("could not create config directory")?;
    }
    let json = serde_json::to_string_pretty(creds)?;
    std::fs::write(path, &json).context("could not write credentials file")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
            .context("could not set credentials file permissions")?;
    }

    Ok(())
}

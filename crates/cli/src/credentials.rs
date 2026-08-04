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

pub(crate) struct Session {
    pub access_token: String,
    pub account_id: String,
    pub api_url: Option<String>,
    #[allow(dead_code)]
    pub auth_url: Option<String>,
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
    let path = path(override_path)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("could not create config directory")?;
    }
    let json = serde_json::to_string_pretty(creds)?;
    std::fs::write(&path, &json).context("could not write credentials file")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
            .context("could not set credentials file permissions")?;
    }

    Ok(())
}

/// Load a valid session from stored credentials, refreshing if expired.
pub(crate) async fn load_session(override_path: Option<&Path>) -> Result<Session> {
    let creds = load(override_path).context("not logged in — run `aweber auth login` first")?;

    const EXPIRY_BUFFER_SECS: u64 = 60;
    if creds.expires_at > now_secs() + EXPIRY_BUFFER_SECS {
        return Ok(Session {
            access_token: creds.access_token,
            account_id: creds.account_id,
            api_url: creds.api_url,
            auth_url: creds.auth_url,
        });
    }

    let client = aweber::client::Client::new(creds.auth_url());
    let tokens = aweber::oauth::refresh_tokens(&client, creds.client_id(), &creds.refresh_token)
        .await
        .context("token refresh failed")?;
    let new_creds = Credentials {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at: now_secs() + tokens.expires_in,
        account_id: creds.account_id,
        client_id: creds.client_id,
        api_url: creds.api_url,
        auth_url: creds.auth_url,
    };
    save(&new_creds, override_path)?;
    Ok(Session {
        access_token: new_creds.access_token,
        account_id: new_creds.account_id,
        api_url: new_creds.api_url,
        auth_url: new_creds.auth_url,
    })
}

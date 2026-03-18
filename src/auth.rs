use anyhow::{Context, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_CLIENT_ID: &str = "lZ68iB3i3ZKdCI4q9Uwkqx1c4ykiFe3c";
const REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";
const SCOPES: &str = "account.read list.read list.write subscriber.read subscriber.write subscriber.read-extended email.read email.write landing-page.read";

pub const DEFAULT_API_URL: &str = "https://api.aweber.com";
pub const DEFAULT_AUTH_URL: &str = "https://auth.aweber.com";

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    access_token: String,
    refresh_token: String,
    expires_at: u64,
    account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_url: Option<String>,
}

impl Credentials {
    fn client_id(&self) -> &str {
        self.client_id.as_deref().unwrap_or(DEFAULT_CLIENT_ID)
    }

    fn auth_url(&self) -> &str {
        self.auth_url.as_deref().unwrap_or(DEFAULT_AUTH_URL)
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}

fn credentials_path(override_path: Option<&Path>) -> Result<PathBuf> {
    match override_path {
        Some(path) => Ok(path.to_path_buf()),
        None => {
            let config_dir =
                dirs::config_dir().context("could not determine config directory")?;
            Ok(config_dir.join("aweber").join("credentials.json"))
        }
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn save_credentials(creds: &Credentials, override_path: Option<&Path>) -> Result<()> {
    let path = credentials_path(override_path)?;
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

fn load_credentials(override_path: Option<&Path>) -> Result<Credentials> {
    let path = credentials_path(override_path)?;
    let json = std::fs::read_to_string(&path).context("could not read credentials file")?;
    let creds: Credentials = serde_json::from_str(&json)?;
    Ok(creds)
}

fn generate_code_verifier() -> String {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    let mut bytes = [0u8; 32];
    for chunk in bytes.chunks_mut(8) {
        let hash = RandomState::new().build_hasher().finish().to_le_bytes();
        chunk.copy_from_slice(&hash[..chunk.len()]);
    }
    URL_SAFE_NO_PAD.encode(bytes)
}

fn generate_code_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}

async fn exchange_code(
    code: &str,
    code_verifier: &str,
    api_url: &str,
    auth_url: &str,
    client_id: &str,
) -> Result<Credentials> {
    let url = format!("{auth_url}/oauth2/token");
    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", REDIRECT_URI),
            ("client_id", client_id),
            ("code_verifier", code_verifier),
        ])
        .send()
        .await
        .context("failed to contact token endpoint")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("token exchange failed ({status}): {body}");
    }

    let token_resp: TokenResponse = resp
        .json()
        .await
        .context("failed to parse token response")?;
    Ok(Credentials {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token,
        expires_at: now_secs() + token_resp.expires_in,
        account_id: String::new(),
        client_id: (client_id != DEFAULT_CLIENT_ID).then(|| client_id.to_string()),
        api_url: (api_url != DEFAULT_API_URL).then(|| api_url.to_string()),
        auth_url: (auth_url != DEFAULT_AUTH_URL).then(|| auth_url.to_string()),
    })
}

async fn fetch_account_id(access_token: &str, api_url: &str) -> Result<String> {
    #[derive(Deserialize)]
    struct Account {
        id: i64,
    }
    #[derive(Deserialize)]
    struct Accounts {
        entries: Vec<Account>,
    }

    let url = format!("{api_url}/1.0/accounts");
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await
        .context("failed to fetch accounts")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("failed to fetch accounts ({status}): {body}");
    }

    let accounts: Accounts = resp
        .json()
        .await
        .context("failed to parse accounts response")?;
    let account = accounts
        .entries
        .first()
        .context("no accounts found for this token")?;
    Ok(account.id.to_string())
}

async fn refresh(creds: &Credentials) -> Result<Credentials> {
    let url = format!("{}/oauth2/token", creds.auth_url());
    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", &creds.refresh_token),
            ("client_id", creds.client_id()),
        ])
        .send()
        .await
        .context("failed to contact token endpoint")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("token refresh failed ({status}): {body}");
    }

    let token_resp: TokenResponse = resp
        .json()
        .await
        .context("failed to parse token response")?;
    Ok(Credentials {
        access_token: token_resp.access_token,
        refresh_token: token_resp.refresh_token,
        expires_at: now_secs() + token_resp.expires_in,
        account_id: creds.account_id.clone(),
        client_id: creds.client_id.clone(),
        api_url: creds.api_url.clone(),
        auth_url: creds.auth_url.clone(),
    })
}

pub async fn login(
    creds_path: Option<&Path>,
    api_url: &str,
    auth_url: &str,
    client_id: &str,
) -> Result<()> {
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);
    let scope_encoded = SCOPES.replace(' ', "%20");
    let authorize_url = format!(
        "{auth_url}/oauth2/authorize?response_type=code&client_id={client_id}&redirect_uri={REDIRECT_URI}&scope={scope_encoded}&code_challenge={code_challenge}&code_challenge_method=S256"
    );

    println!("Open this URL in your browser to authorize:\n");
    println!("  {authorize_url}\n");

    if open::that(&authorize_url).is_err() {
        println!("(Could not open browser automatically. Copy the URL above.)");
    }

    println!("After authorizing, paste the code below.\n");
    use std::io::Write;
    print!("Authorization code: ");
    std::io::stdout()
        .flush()
        .context("failed to flush stdout")?;

    let mut code = String::new();
    std::io::stdin()
        .read_line(&mut code)
        .context("failed to read authorization code")?;
    let code = code.trim();

    if code.is_empty() {
        anyhow::bail!("no authorization code provided");
    }

    let mut creds = exchange_code(code, &code_verifier, api_url, auth_url, client_id).await?;
    creds.account_id = fetch_account_id(&creds.access_token, api_url).await?;
    save_credentials(&creds, creds_path)?;

    println!(
        "\nLogged in successfully (account {}). Token expires in 2 hours and will auto-refresh.",
        creds.account_id
    );
    Ok(())
}

pub fn logout(creds_path: Option<&Path>) -> Result<()> {
    let path = credentials_path(creds_path)?;
    match std::fs::remove_file(&path) {
        Ok(()) => println!("Logged out."),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => println!("Not logged in."),
        Err(e) => return Err(anyhow::Error::new(e).context("could not delete credentials file")),
    }
    Ok(())
}

pub fn status(creds_path: Option<&Path>) -> Result<()> {
    match load_credentials(creds_path) {
        Ok(creds) => {
            let now = now_secs();
            if creds.expires_at > now {
                let remaining = creds.expires_at - now;
                let mins = remaining / 60;
                println!(
                    "Logged in (account {}). Token expires in {mins} minutes.",
                    creds.account_id
                );
            } else {
                println!(
                    "Logged in (account {}). Token expired (will auto-refresh on next API call).",
                    creds.account_id
                );
            }
        }
        Err(_) => {
            println!("Not logged in. Run `aweber auth login` to authenticate.");
        }
    }
    Ok(())
}

pub struct Session {
    pub access_token: String,
    pub account_id: String,
    pub api_url: Option<String>,
    pub auth_url: Option<String>,
}

/// Load a valid session from stored credentials, refreshing if expired.
pub async fn load_session(creds_path: Option<&Path>) -> Result<Session> {
    let creds =
        load_credentials(creds_path).context("not logged in — run `aweber auth login` first")?;

    const EXPIRY_BUFFER_SECS: u64 = 60;
    if creds.expires_at > now_secs() + EXPIRY_BUFFER_SECS {
        return Ok(Session {
            access_token: creds.access_token,
            account_id: creds.account_id,
            api_url: creds.api_url,
            auth_url: creds.auth_url,
        });
    }

    let new_creds = refresh(&creds).await?;
    save_credentials(&new_creds, creds_path)?;
    Ok(Session {
        access_token: new_creds.access_token,
        account_id: new_creds.account_id,
        api_url: new_creds.api_url,
        auth_url: new_creds.auth_url,
    })
}

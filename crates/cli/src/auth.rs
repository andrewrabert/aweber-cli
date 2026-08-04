use std::io::Write as _;
use std::path::Path;

use anyhow::{Context, Result};

use crate::credentials::{self, Credentials};

pub(crate) const DEFAULT_CLIENT_ID: &str = "lZ68iB3i3ZKdCI4q9Uwkqx1c4ykiFe3c";

/// Resolve the account id of the token backing this client.
pub(crate) async fn fetch_account_id(client: &aweber::client::Client) -> Result<i32> {
    let accounts = aweber::endpoints::get_accounts(client, None, None)
        .await
        .context("failed to fetch accounts")?;
    let account = accounts
        .entries
        .first()
        .context("no accounts found for this token")?;
    Ok(account.id.context("account missing id field")? as i32)
}

fn bearer_client(baseurl: &str, access_token: &str) -> Result<aweber::client::Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {access_token}").parse().unwrap(),
    );
    Ok(aweber::client::Client::new_with_client(
        baseurl,
        reqwest::Client::builder()
            .default_headers(headers)
            .build()?,
    ))
}

pub(crate) async fn login(
    creds_path: Option<&Path>,
    api_url: &str,
    auth_url: &str,
    client_id: &str,
) -> Result<()> {
    let pkce = aweber::oauth::Pkce::generate();
    let authorize_url = aweber::oauth::authorize_url(auth_url, client_id, &pkce.challenge());

    println!("Open this URL in your browser to authorize:\n");
    println!("  {authorize_url}\n");

    if open::that(&authorize_url).is_err() {
        println!("(Could not open browser automatically. Copy the URL above.)");
    }

    println!("After authorizing, paste the code below.\n");
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

    let auth_client = aweber::client::Client::new(auth_url);
    let tokens = aweber::oauth::exchange_code(&auth_client, client_id, code, pkce.verifier())
        .await
        .context("token exchange failed")?;

    let account_id = fetch_account_id(&bearer_client(api_url, &tokens.access_token)?).await?;

    let creds = Credentials {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        expires_at: credentials::now_secs() + tokens.expires_in,
        account_id: account_id.to_string(),
        client_id: (client_id != DEFAULT_CLIENT_ID).then(|| client_id.to_string()),
        api_url: (api_url != aweber::oauth::DEFAULT_API_URL).then(|| api_url.to_string()),
        auth_url: (auth_url != aweber::oauth::DEFAULT_AUTH_URL).then(|| auth_url.to_string()),
    };
    credentials::save(&creds, creds_path)?;

    println!(
        "\nLogged in successfully (account {}). Token expires in 2 hours and will auto-refresh.",
        creds.account_id
    );
    Ok(())
}

pub(crate) fn logout(creds_path: Option<&Path>) -> Result<()> {
    let path = credentials::path(creds_path)?;
    match std::fs::remove_file(&path) {
        Ok(()) => println!("Logged out."),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => println!("Not logged in."),
        Err(e) => return Err(anyhow::Error::new(e).context("could not delete credentials file")),
    }
    Ok(())
}

pub(crate) fn status(creds_path: Option<&Path>) -> Result<()> {
    match credentials::load(creds_path) {
        Ok(creds) => {
            let now = credentials::now_secs();
            if creds.expires_at > now {
                let mins = (creds.expires_at - now) / 60;
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

use std::path::Path;

use anyhow::Context as _;

mod auth;
mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    let app = commands::build_command_tree();
    let matches = app.get_matches();

    let creds_file = matches
        .get_one::<String>("credentials-file")
        .map(|s| Path::new(s.as_str()));

    let api_url = matches
        .get_one::<String>("api-url")
        .expect("api-url has default");
    let auth_url = matches
        .get_one::<String>("auth-url")
        .expect("auth-url has default");

    // Handle auth commands (no credentials needed)
    if let Some(("auth", auth_matches)) = matches.subcommand() {
        match auth_matches.subcommand() {
            Some(("login", login_matches)) => {
                let client_id = login_matches
                    .get_one::<String>("client-id")
                    .expect("client-id has default");

                return auth::login(creds_file, api_url, auth_url, client_id).await;
            }
            Some(("logout", _)) => return auth::logout(creds_file),
            Some(("status", _)) => return auth::status(creds_file),
            _ => unreachable!(),
        }
    }

    // Resolve token and session fallbacks
    let (token, account_id, api_url, _auth_url) =
        match matches.get_one::<String>("token").cloned() {
            Some(t) => (t, None, api_url.clone(), auth_url.clone()),
            None => {
                let session = auth::load_session(creds_file).await?;
                let parsed: i32 = session
                    .account_id
                    .parse()
                    .context("invalid account_id in stored credentials")?;
                // Stored URLs from credentials take effect when CLI arg is the default
                let api_url = session.api_url.unwrap_or_else(|| api_url.clone());
                let auth_url = session.auth_url.unwrap_or_else(|| auth_url.clone());
                (session.access_token, Some(parsed), api_url, auth_url)
            }
        };

    let client = aweber::client::Client::new_with_client(
        &api_url,
        reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {token}").parse().unwrap(),
                );
                headers
            })
            .build()?,
    );

    let verbose = matches.get_flag("verbose");
    let client = client.with_verbose(verbose);

    let account_id = match account_id {
        Some(id) => id,
        None => {
            let accounts: aweber::types::Accounts =
                aweber::endpoints::get_accounts(&client, None, None)
                    .await
                    .context("failed to fetch accounts")?;
            let account = accounts
                .entries
                .first()
                .context("no accounts found for this token")?;
            account
                .id
                .context("account missing id field")? as i32
        }
    };

    let cli = aweber::cli::Cli::new(client, account_id);

    let (group_name, group_matches) = matches.subcommand().expect("subcommand is required");
    let (action_name, action_matches) = group_matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("no action specified for '{group_name}'"))?;

    let cli_cmd = commands::resolve_command(group_name, action_name)
        .ok_or_else(|| anyhow::anyhow!("unknown command: {group_name} {action_name}"))?;

    match cli.execute(cli_cmd, action_matches).await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

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

    // Handle auth commands (no credentials needed)
    if let Some(("auth", auth_matches)) = matches.subcommand() {
        match auth_matches.subcommand() {
            Some(("login", login_matches)) => {
                let client_id = login_matches
                    .get_one::<String>("client-id")
                    .expect("client-id has default");
                let api_url = login_matches
                    .get_one::<String>("api-url")
                    .expect("api-url has default");
                let auth_url = login_matches
                    .get_one::<String>("auth-url")
                    .expect("auth-url has default");

                return auth::login(creds_file, api_url, auth_url, client_id).await;
            }
            Some(("logout", _)) => return auth::logout(creds_file),
            Some(("status", _)) => return auth::status(creds_file),
            _ => unreachable!(),
        }
    }

    // Resolve token and account_id
    let (token, account_id, api_url) = match matches.get_one::<String>("token").cloned() {
        Some(t) => (t, None, auth::DEFAULT_API_URL.to_string()),
        None => {
            let session = auth::load_session(creds_file).await?;
            let parsed: i32 = session
                .account_id
                .parse()
                .context("invalid account_id in stored credentials")?;
            (session.access_token, Some(parsed), session.api_url)
        }
    };

    let account_id = account_id.ok_or_else(|| {
        anyhow::anyhow!(
            "account ID not available — log in with `aweber auth login` or use a stored session"
        )
    })?;

    let base_url = format!("{api_url}/1.0");
    let client = aweber::client::Client::new_with_client(
        &base_url,
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

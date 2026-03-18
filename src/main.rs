use std::io::{IsTerminal, Read, Write};
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

    // Handle api command (needs auth but not account_id)
    if let Some(("api", api_matches)) = matches.subcommand() {
        return handle_api_command(&client, api_matches).await;
    }

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

async fn handle_api_command(
    client: &aweber::client::Client,
    matches: &clap::ArgMatches,
) -> anyhow::Result<()> {
    let path = matches
        .get_one::<String>("path")
        .expect("path is required");
    let method: reqwest::Method = matches
        .get_one::<String>("method")
        .expect("method has default")
        .parse()
        .context("invalid HTTP method")?;

    let body = match matches.get_one::<String>("input") {
        Some(input) if input == "-" => {
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf)?;
            Some(buf)
        }
        Some(file) => Some(std::fs::read(file).context("failed to read input file")?),
        None => None,
    };

    let headers: Vec<(reqwest::header::HeaderName, String)> = matches
        .get_many::<String>("header")
        .unwrap_or_default()
        .map(|h| {
            let (key, value) = h
                .split_once(':')
                .context("header must be key:value")?;
            let name = key
                .parse::<reqwest::header::HeaderName>()
                .map_err(|e| anyhow::anyhow!("invalid header name '{key}': {e}"))?;
            Ok((name, value.trim_start().to_string()))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let resp = client
        .raw_request(method, path, &headers, body.as_deref())
        .await?;

    let color = std::io::stderr().is_terminal();
    let pretty = std::io::stdout().is_terminal();

    print_response_headers(&resp, color);

    if pretty {
        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&resp.body) {
            let formatted = colored_json::to_colored_json_auto(&json)?;
            println!("{formatted}");
        } else {
            std::io::stdout().write_all(&resp.body)?;
        }
    } else {
        std::io::stdout().write_all(&resp.body)?;
    }

    if !(200..300).contains(&resp.status) {
        std::process::exit(1);
    }
    Ok(())
}

fn print_response_headers(resp: &aweber::client::RawResponse, color: bool) {
    if color {
        eprintln!(
            "\x1b[34m{}\x1b[0m \x1b[36m{}\x1b[0m",
            resp.http_version, resp.status
        );
        for (key, value) in &resp.headers {
            eprintln!("\x1b[37m{key}:\x1b[0m \x1b[36m{value}\x1b[0m");
        }
    } else {
        eprintln!("{} {}", resp.http_version, resp.status);
        for (key, value) in &resp.headers {
            eprintln!("{key}: {value}");
        }
    }
    eprintln!();
}

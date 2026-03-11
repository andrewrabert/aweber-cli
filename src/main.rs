use anyhow::Context as _;

mod auth;
mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // First pass: handle cases that don't need credentials
    let args: Vec<String> = std::env::args().collect();

    // Let clap handle --help / -h / missing subcommand directly (no credentials needed)
    let needs_early_parse = args.len() <= 1
        || args
            .iter()
            .any(|a| a == "--help" || a == "-h" || a == "--version" || a == "-V");
    if needs_early_parse {
        let app = commands::build_command_tree();
        let _matches = app.get_matches(); // will print help and exit
        unreachable!();
    }

    // Handle `messages validate` without credentials (client-side only)
    if args.len() >= 3 && args[1] == "messages" && args[2] == "validate" {
        let app = commands::build_command_tree();
        let matches = app.get_matches_from(&args);
        let (_, group_matches) = matches.subcommand().expect("subcommand is required");
        let (_, action_matches) = group_matches.subcommand().expect("action is required");
        let body_json_raw = action_matches
            .get_one::<String>("body-json")
            .expect("body-json is required");
        let content = aweber::cli::read_content(body_json_raw)?;
        let value: serde_json::Value =
            serde_json::from_str(&content).context("invalid JSON in body-json")?;
        return match aweber::validate::validate_body_json(&value) {
            Ok(()) => {
                println!("body_json is valid");
                Ok(())
            }
            Err(errors) => {
                for e in &errors {
                    eprintln!("validation error: {e}");
                }
                anyhow::bail!("{} validation error(s)", errors.len());
            }
        };
    }

    if args[1] == "auth" {
        let app = commands::build_command_tree();
        let matches = app.get_matches();
        if let Some(("auth", auth_matches)) = matches.subcommand() {
            match auth_matches.subcommand() {
                Some(("login", _)) => return auth::login().await,
                Some(("logout", _)) => return auth::logout(),
                Some(("status", _)) => return auth::status(),
                _ => unreachable!(),
            }
        }
    }

    // Resolve token and account_id from the same auth context
    let (token, account_id) = match std::env::var("AWEBER_TOKEN").ok().or_else(|| {
        args.iter()
            .position(|a| a == "--token")
            .and_then(|i| args.get(i + 1).cloned())
    }) {
        Some(t) => (t, None),
        None => {
            let (t, id) = auth::load_session().await?;
            let parsed: i32 = id
                .parse()
                .context("invalid account_id in stored credentials")?;
            (t, Some(parsed))
        }
    };

    let account_id = account_id.ok_or_else(|| {
        anyhow::anyhow!(
            "account ID not available — log in with `aweber auth login` or use a stored session"
        )
    })?;

    let app = commands::build_command_tree();
    let matches = app.get_matches_from(args);

    let base_url = matches
        .get_one::<String>("base-url")
        .expect("base-url has default");

    let client = aweber::client::Client::new_with_client(
        base_url,
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
    let mut cli = aweber::cli::Cli::new(client, account_id);

    // Set up cookie-auth client for message editor commands if --session is provided.
    if let Some(session) = matches.get_one::<String>("session") {
        let cp_url = matches
            .get_one::<String>("cp-url")
            .expect("cp-url has default");
        let session_client = aweber::client::Client::new(cp_url).with_verbose(verbose);
        cli = cli.with_session(session_client, session.clone());
    }

    let (group_name, group_matches) = matches.subcommand().expect("subcommand is required");
    let (action_name, action_matches) = group_matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("no action specified for '{group_name}'"))?;

    let cli_cmd = commands::resolve_command(group_name, action_name)
        .ok_or_else(|| anyhow::anyhow!("unknown command: {group_name} {action_name}"))?;

    cli.execute(cli_cmd, action_matches).await
}

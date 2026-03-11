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
    let cli = aweber::cli::Cli::new(client, account_id);

    let (group_name, group_matches) = matches.subcommand().expect("subcommand is required");
    let (action_name, action_matches) = group_matches
        .subcommand()
        .ok_or_else(|| anyhow::anyhow!("no action specified for '{group_name}'"))?;

    let cli_cmd = commands::resolve_command(group_name, action_name)
        .ok_or_else(|| anyhow::anyhow!("unknown command: {group_name} {action_name}"))?;

    cli.execute(cli_cmd, action_matches).await
}

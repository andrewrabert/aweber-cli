//! A terminal UI over the whole AWeber API surface.

pub mod catalog;
pub mod core;
pub mod harness;
pub mod ports;
pub mod runtime;
pub mod view;

pub use core::redact::Secret;

pub struct Options {
    pub client: aweber::client::Client,
    pub session: std::sync::Arc<dyn ports::SessionPort>,
    pub stored_account: Option<StoredAccount>,
    /// Routes request and response bodies into the Event Log.
    pub verbose: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct StoredAccount {
    pub id: i32,
    /// The account's uid, read from the credentials' `uuid` member.
    pub uuid: Option<aweber::ids::AccountUid>,
}

/// Enters the alternate screen, runs the loop, and restores the terminal on
/// return, error, and panic.
pub async fn run(options: Options) -> anyhow::Result<()> {
    let (actions, incoming) = tokio::sync::mpsc::unbounded_channel();
    let client = options
        .client
        .with_observer(std::sync::Arc::new(runtime::observer::LogObserver::new(
            actions.clone(),
        )))
        .with_bodies_observed(options.verbose);
    let ports = runtime::Ports {
        http: std::sync::Arc::new(runtime::http::SharedHttp::new(client)),
        clock: std::sync::Arc::new(runtime::clock::SystemClock::new()),
        clipboard: std::sync::Arc::new(runtime::clipboard::SystemClipboard),
        editor: std::sync::Arc::new(runtime::editor::SystemEditor),
        session: options.session,
    };
    let state = core::State::new(options.stored_account, options.verbose);
    let mut app = runtime::App::new(state, ports, actions.clone(), incoming);
    let _ = actions.send(core::Action::Launched);

    let mut terminal = runtime::terminal::init()?;
    let outcome = runtime::drive(&mut terminal, &mut app).await;
    runtime::terminal::restore();
    outcome
}

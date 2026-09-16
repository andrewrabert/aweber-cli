use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

use crate::credentials::{self, Credentials, Store};

/// Runs the TUI, landing on the Session view when no credentials are usable.
pub(crate) async fn launch(
    credentials_file: Option<&Path>,
    api_url: &str,
    auth_url: &str,
    token: Option<&str>,
    verbose: bool,
) -> Result<()> {
    let store = Store::new(credentials_file)?;
    let (session, stored_account) = match store.open(token).await {
        Ok(opened) => opened,
        Err(_) => (
            Arc::new(aweber::session::Session::fixed(
                token.map(ToString::to_string),
            )),
            None,
        ),
    };
    let api_url = store
        .load()
        .ok()
        .and_then(|creds| creds.api_url)
        .unwrap_or_else(|| api_url.to_string());
    let client = aweber::client::Client::with_session(&api_url, session)?;
    let front = Arc::new(FileSession {
        store: store.clone(),
        api_url: api_url.clone(),
        auth_url: auth_url.to_string(),
    });
    aweber_tui::run(aweber_tui::Options {
        client,
        session: front,
        stored_account,
        verbose,
    })
    .await
}

/// The credentials file behind the Session view's login and logout.
struct FileSession {
    store: Store,
    api_url: String,
    auth_url: String,
}

impl FileSession {
    fn client_id(&self) -> String {
        self.store
            .load()
            .ok()
            .map(|creds| creds.client_id().to_string())
            .unwrap_or_else(|| crate::auth::DEFAULT_CLIENT_ID.to_string())
    }

    fn auth_url(&self) -> String {
        self.store
            .load()
            .ok()
            .and_then(|creds| creds.auth_url)
            .unwrap_or_else(|| self.auth_url.clone())
    }
}

impl aweber_tui::ports::SessionPort for FileSession {
    fn status(&self) -> aweber_tui::ports::BoxFuture<'static, aweber_tui::ports::SessionStatus> {
        let status = self.store.status();
        Box::pin(async move { status })
    }

    fn authorization(&self) -> aweber_tui::ports::Authorization {
        let pkce = aweber::oauth::Pkce::generate();
        aweber_tui::ports::Authorization {
            url: aweber::oauth::authorize_url(
                &self.auth_url(),
                &self.client_id(),
                &pkce.challenge(),
            ),
            verifier: aweber_tui::Secret::new(pkce.verifier().to_string()),
        }
    }

    fn login(
        &self,
        code: String,
        verifier: aweber_tui::Secret,
    ) -> aweber_tui::ports::BoxFuture<'static, Result<aweber_tui::ports::SessionStatus, String>>
    {
        let store = self.store.clone();
        let client_id = self.client_id();
        let auth_url = self.auth_url();
        let api_url = self.api_url.clone();
        Box::pin(async move {
            let auth_client = aweber::client::Client::new(&auth_url);
            let tokens =
                aweber::oauth::exchange_code(&auth_client, &client_id, &code, verifier.expose())
                    .await
                    .map_err(|error| error.to_string())?;
            let api_client =
                aweber::client::Client::with_bearer_token(&api_url, &tokens.access_token)
                    .map_err(|error| error.to_string())?;
            let account = crate::auth::fetch_account(&api_client)
                .await
                .map_err(|error| error.to_string())?;
            let account_id = account.id.ok_or_else(|| "account missing id".to_string())?;
            let creds = Credentials {
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
                expires_at: credentials::now_secs() + tokens.expires_in,
                account_id: account_id.to_string(),
                account: aweber::endpoints::account_uid(&account),
                client_id: (client_id != crate::auth::DEFAULT_CLIENT_ID).then_some(client_id),
                api_url: (api_url != aweber::oauth::DEFAULT_API_URL).then_some(api_url),
                auth_url: (auth_url != aweber::oauth::DEFAULT_AUTH_URL).then_some(auth_url),
            };
            store.save(&creds).map_err(|error| error.to_string())?;
            Ok(store.status())
        })
    }

    fn logout(&self) -> aweber_tui::ports::BoxFuture<'static, Result<(), String>> {
        let store = self.store.clone();
        Box::pin(async move { store.forget().map_err(|error| error.to_string()) })
    }
}

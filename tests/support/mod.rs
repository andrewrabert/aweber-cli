use std::sync::Once;

static PROVIDER: Once = Once::new();

pub fn client(server: &wiremock::MockServer) -> aweber::client::Client {
    PROVIDER.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
    aweber::client::Client::new(&server.uri())
}

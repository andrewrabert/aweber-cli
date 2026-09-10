use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::client::{ApiError, ApiRequest, Client};

pub const DEFAULT_API_URL: &str = "https://api.aweber.com";
pub const DEFAULT_AUTH_URL: &str = "https://auth.aweber.com";
pub const REDIRECT_URI: &str = "urn:ietf:wg:oauth:2.0:oob";
pub const SCOPES: &str = "account.read list.read list.write subscriber.read subscriber.write subscriber.read-extended email.read email.write landing-page.read";

const TOKEN_PATH: &str = "/oauth2/token";

/// A PKCE code verifier and its derived challenge.
pub struct Pkce {
    verifier: String,
}

impl Pkce {
    pub fn generate() -> Pkce {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        let mut bytes = [0u8; 32];
        for chunk in bytes.chunks_mut(8) {
            let hash = RandomState::new().build_hasher().finish().to_le_bytes();
            chunk.copy_from_slice(&hash[..chunk.len()]);
        }
        Pkce {
            verifier: URL_SAFE_NO_PAD.encode(bytes),
        }
    }

    pub fn verifier(&self) -> &str {
        &self.verifier
    }

    pub fn challenge(&self) -> String {
        URL_SAFE_NO_PAD.encode(Sha256::digest(self.verifier.as_bytes()))
    }
}

/// Tokens returned by the OAuth2 token endpoint.
#[derive(Debug, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

/// Build the authorization URL the user visits to grant access.
pub fn authorize_url(auth_url: &str, client_id: &str, code_challenge: &str) -> String {
    let scope = SCOPES.replace(' ', "%20");
    let redirect_uri = REDIRECT_URI.replace(':', "%3A");
    format!(
        "{auth_url}/oauth2/authorize?response_type=code&client_id={client_id}&redirect_uri={redirect_uri}&scope={scope}&code_challenge={code_challenge}&code_challenge_method=S256"
    )
}

#[derive(Serialize)]
struct AuthorizationCodeGrant<'a> {
    grant_type: &'static str,
    code: &'a str,
    redirect_uri: &'static str,
    client_id: &'a str,
    code_verifier: &'a str,
}

#[derive(Serialize)]
struct RefreshTokenGrant<'a> {
    grant_type: &'static str,
    refresh_token: &'a str,
    client_id: &'a str,
}

/// Exchange an authorization code for tokens.
pub async fn exchange_code(
    client: &Client,
    client_id: &str,
    code: &str,
    code_verifier: &str,
) -> Result<Tokens, ApiError> {
    ApiRequest::new(client, reqwest::Method::POST, TOKEN_PATH.to_string())
        .form_body(AuthorizationCodeGrant {
            grant_type: "authorization_code",
            code,
            redirect_uri: REDIRECT_URI,
            client_id,
            code_verifier,
        })
        .send()
        .await
}

/// Exchange a refresh token for a fresh set of tokens.
pub async fn refresh_tokens(
    client: &Client,
    client_id: &str,
    refresh_token: &str,
) -> Result<Tokens, ApiError> {
    ApiRequest::new(client, reqwest::Method::POST, TOKEN_PATH.to_string())
        .form_body(RefreshTokenGrant {
            grant_type: "refresh_token",
            refresh_token,
            client_id,
        })
        .send()
        .await
}

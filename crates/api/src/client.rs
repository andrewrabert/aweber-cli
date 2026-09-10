use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::session::{Generation, Session, SessionError};

#[derive(Clone)]
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
    session: Arc<Session>,
    observer: Option<Arc<dyn RequestObserver>>,
    bodies_observed: bool,
    retry: RetryPolicy,
}

impl Client {
    pub fn new(baseurl: &str) -> Self {
        Self::new_with_client(baseurl, default_http_client())
    }

    pub fn with_bearer_token(baseurl: &str, access_token: &str) -> Result<Client, reqwest::Error> {
        let mut client = Self::new_with_client(
            baseurl,
            reqwest::ClientBuilder::new()
                .connect_timeout(Duration::from_secs(15))
                .timeout(Duration::from_secs(15))
                .build()?,
        );
        client.session = Arc::new(Session::fixed(Some(access_token.to_string())));
        Ok(client)
    }

    /// Build a client whose requests carry the session's token and refresh it on `401`.
    pub fn with_session(baseurl: &str, session: Arc<Session>) -> Result<Client, reqwest::Error> {
        let mut client = Self::new_with_client(
            baseurl,
            reqwest::ClientBuilder::new()
                .connect_timeout(Duration::from_secs(15))
                .timeout(Duration::from_secs(15))
                .build()?,
        );
        client.session = session;
        Ok(client)
    }

    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
            session: Arc::new(Session::fixed(None)),
            observer: None,
            bodies_observed: false,
            retry: RetryPolicy::default(),
        }
    }

    pub fn with_observer(mut self, observer: Arc<dyn RequestObserver>) -> Client {
        self.observer = Some(observer);
        self
    }

    /// Bodies reach the observer only when this is set.
    pub fn with_bodies_observed(mut self, observed: bool) -> Client {
        self.bodies_observed = observed;
        self
    }

    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Client {
        self.retry = policy;
        self
    }

    pub fn session(&self) -> &Arc<Session> {
        &self.session
    }

    fn observe(&self, request: RequestKey, event: RequestEvent) {
        if let Some(observer) = &self.observer {
            observer.observe(request, event);
        }
    }

    /// Run one logical request under one key, settling it however it ends.
    async fn execute(&self, attempt_spec: &Attempt) -> Result<Raw, ApiError> {
        let request = RequestKey::allocate();
        let mut settlement = Settlement::new(self.observer.clone(), request);
        self.attempts(request, attempt_spec, settlement.attempts())
            .await
    }

    /// Carry the session's token, refresh once on `401`, wait out `429`, and
    /// report every attempt to the observer.
    async fn attempts(
        &self,
        request: RequestKey,
        attempt_spec: &Attempt,
        made: &mut u8,
    ) -> Result<Raw, ApiError> {
        let mut refreshed = false;
        let mut attempt: u8 = 0;
        loop {
            attempt += 1;
            *made = attempt;
            let (generation, token) = self.session.token();
            let built = attempt_spec.build(&self.client, token.as_deref());
            self.observe(
                request,
                RequestEvent::Started {
                    attempt,
                    method: attempt_spec.method.clone(),
                    url: attempt_spec.line(),
                    body: self.observed_body(attempt_spec),
                },
            );
            let started = Instant::now();
            let response = match built.send().await {
                Ok(response) => response,
                Err(e) => {
                    self.observe(
                        request,
                        RequestEvent::Failed {
                            attempt,
                            duration: started.elapsed(),
                            error: e.to_string(),
                        },
                    );
                    return Err(ApiError::Request(e));
                }
            };
            let status = response.status().as_u16();
            let version = http_version(response.version());
            let headers = response.headers().clone();
            let bytes = match response.bytes().await {
                Ok(bytes) => bytes.to_vec(),
                Err(e) => {
                    self.observe(
                        request,
                        RequestEvent::Failed {
                            attempt,
                            duration: started.elapsed(),
                            error: e.to_string(),
                        },
                    );
                    return Err(ApiError::Request(e));
                }
            };
            self.observe(
                request,
                RequestEvent::Finished {
                    attempt,
                    status,
                    duration: started.elapsed(),
                    body: self
                        .bodies_observed
                        .then(|| String::from_utf8_lossy(&bytes).into_owned()),
                },
            );

            let last = attempt >= self.retry.attempts + u8::from(refreshed);
            if status == 401 && !refreshed && self.session.is_refreshable() {
                refreshed = true;
                self.refresh(generation).await?;
                self.observe(request, RequestEvent::Refreshed { attempt });
                continue;
            }
            if status == 429 && !last {
                let wait = retry_after(&headers, chrono::Utc::now())
                    .unwrap_or_else(|| self.retry.base_backoff * 2u32.pow(u32::from(attempt) - 1))
                    .min(self.retry.max_backoff);
                self.observe(
                    request,
                    RequestEvent::Waiting {
                        attempt,
                        status,
                        wait,
                    },
                );
                self.retry.sleeper.sleep(wait).await;
                continue;
            }
            return Ok(Raw {
                status,
                version,
                headers,
                bytes,
            });
        }
    }

    async fn refresh(&self, seen: Generation) -> Result<(), ApiError> {
        self.session
            .refresh(seen)
            .await
            .map(|_| ())
            .map_err(ApiError::Session)
    }

    fn observed_body(&self, attempt: &Attempt) -> Option<String> {
        if !self.bodies_observed {
            return None;
        }
        attempt.body.as_ref().map(|body| match body {
            OutBody::Json(v) => serde_json::to_string_pretty(v).expect("body is serializable"),
            OutBody::Form(text) => text.clone(),
            OutBody::Bytes(b) => String::from_utf8_lossy(b).into_owned(),
        })
    }
}

fn default_http_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(15))
        .build()
        .unwrap()
}

fn http_version(version: reqwest::Version) -> &'static str {
    match version {
        reqwest::Version::HTTP_09 => "HTTP/0.9",
        reqwest::Version::HTTP_10 => "HTTP/1.0",
        reqwest::Version::HTTP_11 => "HTTP/1.1",
        reqwest::Version::HTTP_2 => "HTTP/2",
        reqwest::Version::HTTP_3 => "HTTP/3",
        _ => "HTTP/?",
    }
}

/// `Retry-After` in either of its two forms: a count of seconds, or an HTTP-date.
fn retry_after(
    headers: &reqwest::header::HeaderMap,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Duration> {
    let value = headers
        .get(reqwest::header::RETRY_AFTER)?
        .to_str()
        .ok()?
        .trim();
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    let at = chrono::DateTime::parse_from_rfc2822(value).ok()?;
    (at.with_timezone(&chrono::Utc) - now).to_std().ok()
}

/// How many attempts one logical request may take, and how long it waits between them.
#[derive(Clone)]
pub struct RetryPolicy {
    pub attempts: u8,
    pub base_backoff: Duration,
    /// No wait exceeds this, whatever `Retry-After` asks for.
    pub max_backoff: Duration,
    pub sleeper: Arc<dyn Sleeper>,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            attempts: 3,
            base_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(30),
            sleeper: Arc::new(TokioSleeper),
        }
    }
}

pub trait Sleeper: Send + Sync + 'static {
    fn sleep(&self, duration: Duration) -> crate::session::BoxFuture<'static, ()>;
}

/// The sleeper of `RetryPolicy::default()`.
pub struct TokioSleeper;

impl Sleeper for TokioSleeper {
    fn sleep(&self, duration: Duration) -> crate::session::BoxFuture<'static, ()> {
        Box::pin(tokio::time::sleep(duration))
    }
}

/// Which logical request an event belongs to: one key per call of a send path,
/// carried by every attempt of it, allocated by the client.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RequestKey(u64);

impl RequestKey {
    fn allocate() -> RequestKey {
        static ISSUED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        RequestKey(ISSUED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

/// The guard the attempt loop holds, so a request that is dropped mid-flight
/// settles like one that answered.
struct Settlement {
    observer: Option<Arc<dyn RequestObserver>>,
    request: RequestKey,
    attempts: u8,
}

impl Settlement {
    fn new(observer: Option<Arc<dyn RequestObserver>>, request: RequestKey) -> Settlement {
        Settlement {
            observer,
            request,
            attempts: 0,
        }
    }

    /// The counter every attempt stamps the number it is into.
    fn attempts(&mut self) -> &mut u8 {
        &mut self.attempts
    }
}

impl Drop for Settlement {
    fn drop(&mut self) {
        if let Some(observer) = &self.observer {
            observer.observe(
                self.request,
                RequestEvent::Settled {
                    attempts: self.attempts,
                },
            );
        }
    }
}

pub trait RequestObserver: Send + Sync + 'static {
    fn observe(&self, request: RequestKey, event: RequestEvent);
}

#[derive(Clone, Debug)]
pub enum RequestEvent {
    Started {
        attempt: u8,
        method: reqwest::Method,
        url: String,
        body: Option<String>,
    },
    /// The wait taken before the next attempt, from `Retry-After` when the
    /// response carried it.
    Waiting {
        attempt: u8,
        status: u16,
        wait: Duration,
    },
    Refreshed {
        attempt: u8,
    },
    Finished {
        attempt: u8,
        status: u16,
        duration: Duration,
        body: Option<String>,
    },
    Failed {
        attempt: u8,
        duration: Duration,
        error: String,
    },
    /// The last event under this key: no further attempt will be made.
    Settled {
        attempts: u8,
    },
}

/// Error type for API requests.
#[derive(Debug)]
pub enum ApiError {
    /// HTTP error response with status code and body.
    Http { status: u16, body: String },
    /// Transport or connection error.
    Request(reqwest::Error),
    /// Failed to deserialize the response body.
    Deserialize {
        source: serde_json::Error,
        body: String,
    },
    /// The refresh itself failed; distinct from an API failure.
    Session(SessionError),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Http { status, body } => write!(f, "HTTP {status}: {body}"),
            ApiError::Request(e) => write!(f, "request error: {e}"),
            ApiError::Deserialize { source, body } => {
                write!(f, "deserialize error: {source}\nbody: {body}")
            }
            ApiError::Session(e) => write!(f, "session error: {e}"),
        }
    }
}

impl ApiError {
    /// Check if the AWeber API error message matches the given string.
    pub fn api_message_is(&self, expected: &str) -> bool {
        let ApiError::Http { body, .. } = self else {
            return false;
        };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(body) else {
            return false;
        };
        json.pointer("/error/message").and_then(|v| v.as_str()) == Some(expected)
    }

    pub fn status(&self) -> Option<u16> {
        match self {
            ApiError::Http { status, .. } => Some(*status),
            _ => None,
        }
    }
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiError::Request(e) => Some(e),
            ApiError::Deserialize { source, .. } => Some(source),
            ApiError::Session(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        ApiError::Request(e)
    }
}

#[derive(Clone)]
enum OutBody {
    Json(serde_json::Value),
    Form(String),
    Bytes(Vec<u8>),
}

/// One logical request, rebuilt for every attempt.
struct Attempt {
    method: reqwest::Method,
    url: String,
    query: Vec<(String, String)>,
    headers: Vec<(reqwest::header::HeaderName, String)>,
    body: Option<OutBody>,
    accept_json: bool,
}

impl Attempt {
    fn build(&self, client: &reqwest::Client, token: Option<&str>) -> reqwest::RequestBuilder {
        let mut req = client.request(self.method.clone(), &self.url);
        if self.accept_json {
            req = req.header(reqwest::header::ACCEPT, "application/json");
        }
        for (k, v) in &self.query {
            req = req.query(&[(k, v)]);
        }
        for (name, value) in &self.headers {
            req = req.header(name, value);
        }
        if let Some(token) = token {
            let mut value: reqwest::header::HeaderValue = format!("Bearer {token}")
                .parse()
                .expect("bearer token is not a valid header value");
            value.set_sensitive(true);
            req = req.header(reqwest::header::AUTHORIZATION, value);
        }
        match &self.body {
            Some(OutBody::Json(v)) => req.json(v),
            Some(OutBody::Form(text)) => req
                .header(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(text.clone()),
            Some(OutBody::Bytes(b)) => req.body(b.clone()),
            None => req,
        }
    }

    /// `GET https://api.aweber.com/1.0/accounts?ws.size=100`
    fn line(&self) -> String {
        if self.query.is_empty() {
            self.url.clone()
        } else {
            let qs: Vec<_> = self.query.iter().map(|(k, v)| format!("{k}={v}")).collect();
            format!("{}?{}", self.url, qs.join("&"))
        }
    }
}

/// A response as it came off the wire.
struct Raw {
    status: u16,
    version: &'static str,
    headers: reqwest::header::HeaderMap,
    bytes: Vec<u8>,
}

impl Raw {
    /// Return the body on success, an `ApiError::Http` on failure.
    fn into_body(self) -> Result<String, ApiError> {
        let body = String::from_utf8_lossy(&self.bytes).into_owned();
        if (200..300).contains(&self.status) {
            Ok(body)
        } else {
            Err(ApiError::Http {
                status: self.status,
                body,
            })
        }
    }
}

/// A builder for API requests that handles path, query params, and body.
pub struct ApiRequest<'a> {
    client: &'a Client,
    method: reqwest::Method,
    path: String,
    query: Vec<(&'static str, String)>,
    body: Option<OutBody>,
    extra_headers: Vec<(reqwest::header::HeaderName, String)>,
}

impl<'a> ApiRequest<'a> {
    pub fn new(client: &'a Client, method: reqwest::Method, path: String) -> Self {
        Self {
            client,
            method,
            path,
            query: Vec::new(),
            body: None,
            extra_headers: Vec::new(),
        }
    }

    /// Add a required query parameter.
    pub fn query<V: std::fmt::Display>(mut self, key: &'static str, value: V) -> Self {
        self.query.push((key, value.to_string()));
        self
    }

    /// Add an optional query parameter (skipped if None).
    pub fn query_opt<V: std::fmt::Display>(mut self, key: &'static str, value: Option<V>) -> Self {
        if let Some(v) = value {
            self.query.push((key, v.to_string()));
        }
        self
    }

    /// Set the JSON request body.
    pub fn json_body(mut self, body: impl serde::Serialize) -> Self {
        self.body = Some(OutBody::Json(
            serde_json::to_value(body).expect("failed to serialize body"),
        ));
        self
    }

    /// Set the form-urlencoded request body.
    pub fn form_body(mut self, body: impl serde::Serialize) -> Self {
        let document = serde_json::to_value(body).expect("failed to serialize body");
        self.body = Some(OutBody::Form(
            serde_urlencoded::to_string(&document).expect("a form body is a flat object"),
        ));
        self
    }

    /// Add an extra header.
    pub fn header(mut self, name: reqwest::header::HeaderName, value: String) -> Self {
        self.extra_headers.push((name, value));
        self
    }

    fn attempt(&self) -> Attempt {
        Attempt {
            method: self.method.clone(),
            url: format!("{}{}", self.client.baseurl, self.path),
            query: self
                .query
                .iter()
                .map(|(k, v)| ((*k).to_string(), v.clone()))
                .collect(),
            headers: self.extra_headers.clone(),
            body: self.body.clone(),
            accept_json: true,
        }
    }

    /// Send the request and deserialize the response.
    pub async fn send<T: serde::de::DeserializeOwned>(self) -> Result<T, ApiError> {
        let body = self.client.execute(&self.attempt()).await?.into_body()?;
        serde_json::from_str(&body).map_err(|e| ApiError::Deserialize { source: e, body })
    }

    pub async fn send_with_headers<T: serde::de::DeserializeOwned>(
        self,
    ) -> Result<ApiResponse<T>, ApiError> {
        let raw = self.client.execute(&self.attempt()).await?;
        let headers = raw.headers.clone();
        let body = raw.into_body()?;
        let body =
            serde_json::from_str(&body).map_err(|e| ApiError::Deserialize { source: e, body })?;
        Ok(ApiResponse { body, headers })
    }

    /// Send the request, ignoring the response body (for DELETE, etc.).
    pub async fn send_no_body(self) -> Result<(), ApiError> {
        self.client.execute(&self.attempt()).await?.into_body()?;
        Ok(())
    }
}

pub struct ApiResponse<T> {
    pub body: T,
    pub headers: reqwest::header::HeaderMap,
}

/// Response from a raw API request.
pub struct RawResponse {
    pub status: u16,
    pub http_version: &'static str,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Client {
    /// GET an absolute URL and deserialize the response.
    pub async fn get_url<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, ApiError> {
        let attempt = Attempt {
            method: reqwest::Method::GET,
            url: url.to_string(),
            query: Vec::new(),
            headers: Vec::new(),
            body: None,
            accept_json: true,
        };
        let body = self.execute(&attempt).await?.into_body()?;
        serde_json::from_str(&body).map_err(|e| ApiError::Deserialize { source: e, body })
    }

    /// Send a raw API request, returning the full response.
    pub async fn raw_request(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: &[(reqwest::header::HeaderName, String)],
        body: Option<&[u8]>,
    ) -> Result<RawResponse, ApiError> {
        let url = if path.starts_with(&self.baseurl) {
            path.to_string()
        } else {
            format!("{}{}", self.baseurl, path)
        };
        let attempt = Attempt {
            method,
            url,
            query: Vec::new(),
            headers: headers.to_vec(),
            body: body.map(|b| OutBody::Bytes(b.to_vec())),
            accept_json: false,
        };
        let raw = self.execute(&attempt).await?;
        let resp_headers = raw
            .headers
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
            .collect();
        Ok(RawResponse {
            status: raw.status,
            http_version: raw.version,
            headers: resp_headers,
            body: raw.bytes,
        })
    }
}

/// A deserialized document with the status and headers that carried it.
pub struct PlanResponse {
    pub status: u16,
    pub headers: reqwest::header::HeaderMap,
    pub body: Option<serde_json::Value>,
}

#[cfg(feature = "catalog")]
impl Client {
    /// Send the request a plan describes, deserializing whatever body comes back.
    pub async fn send_plan(
        &self,
        plan: &crate::catalog::RequestPlan,
    ) -> Result<PlanResponse, ApiError> {
        let url = if plan.path.starts_with("http://") || plan.path.starts_with("https://") {
            plan.path.clone()
        } else {
            format!("{}{}", self.baseurl, plan.path)
        };
        let attempt = Attempt {
            method: plan.method.clone(),
            url,
            query: plan.query.clone(),
            headers: plan.headers.clone(),
            body: plan.body.as_ref().map(|body| match body {
                crate::catalog::PlanBody::Json(document) => OutBody::Json(document.clone()),
                crate::catalog::PlanBody::Form(_) => OutBody::Form(body.encoded()),
            }),
            accept_json: true,
        };
        let raw = self.execute(&attempt).await?;
        let status = raw.status;
        let headers = raw.headers.clone();
        let body = raw.into_body()?;
        let body = if body.trim().is_empty() {
            None
        } else {
            Some(
                serde_json::from_str(&body)
                    .map_err(|e| ApiError::Deserialize { source: e, body })?,
            )
        };
        Ok(PlanResponse {
            status,
            headers,
            body,
        })
    }
}

/// Percent-encode a path segment.
pub fn encode_path(s: &str) -> String {
    s.replace('%', "%25")
        .replace(' ', "%20")
        .replace('/', "%2F")
        .replace('?', "%3F")
        .replace('#', "%23")
}

use std::fmt;

#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
    pub(crate) verbose: bool,
}

impl Client {
    pub fn new(baseurl: &str) -> Self {
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(std::time::Duration::from_secs(15))
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap();
        Self::new_with_client(baseurl, client)
    }

    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
            verbose: false,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
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
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Http { status, body } => write!(f, "HTTP {status}: {body}"),
            ApiError::Request(e) => write!(f, "request error: {e}"),
            ApiError::Deserialize { source, body } => {
                write!(f, "deserialize error: {source}\nbody: {body}")
            }
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
}

impl std::error::Error for ApiError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ApiError::Request(e) => Some(e),
            ApiError::Deserialize { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        ApiError::Request(e)
    }
}

enum Body {
    Json(serde_json::Value),
    Form(serde_json::Value),
}

impl Body {
    fn value(&self) -> &serde_json::Value {
        match self {
            Body::Json(v) | Body::Form(v) => v,
        }
    }
}

/// A builder for API requests that handles path, query params, and body.
pub struct ApiRequest<'a> {
    client: &'a Client,
    method: reqwest::Method,
    path: String,
    query: Vec<(&'static str, String)>,
    body: Option<Body>,
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
    pub fn query_opt<V: std::fmt::Display>(
        mut self,
        key: &'static str,
        value: Option<V>,
    ) -> Self {
        if let Some(v) = value {
            self.query.push((key, v.to_string()));
        }
        self
    }

    /// Set the JSON request body.
    pub fn json_body(mut self, body: impl serde::Serialize) -> Self {
        self.body = Some(Body::Json(
            serde_json::to_value(body).expect("failed to serialize body"),
        ));
        self
    }

    /// Set the form-urlencoded request body.
    pub fn form_body(mut self, body: impl serde::Serialize) -> Self {
        self.body = Some(Body::Form(
            serde_json::to_value(body).expect("failed to serialize body"),
        ));
        self
    }

    /// Add an extra header.
    pub fn header(mut self, name: reqwest::header::HeaderName, value: String) -> Self {
        self.extra_headers.push((name, value));
        self
    }

    fn build_request(self) -> (reqwest::RequestBuilder, bool) {
        let verbose = self.client.verbose;
        let url = format!("{}{}", self.client.baseurl, self.path);
        if verbose {
            if self.query.is_empty() {
                eprintln!("{} {url}", self.method);
            } else {
                let qs: Vec<_> = self.query.iter().map(|(k, v)| format!("{k}={v}")).collect();
                eprintln!("{} {url}?{}", self.method, qs.join("&"));
            }
        }
        let mut req = self.client.client.request(self.method, &url);
        req = req.header(reqwest::header::ACCEPT, "application/json");
        for (k, v) in &self.query {
            req = req.query(&[(k, v)]);
        }
        for (name, value) in &self.extra_headers {
            req = req.header(name, value);
        }
        if let Some(body) = self.body {
            if verbose {
                eprintln!("{}", serde_json::to_string_pretty(body.value()).unwrap());
            }
            req = match body {
                Body::Json(v) => req.json(&v),
                Body::Form(v) => req.form(&v),
            };
        }
        (req, verbose)
    }

    /// Send the request and deserialize the response.
    pub async fn send<T: serde::de::DeserializeOwned>(self) -> Result<T, ApiError> {
        let (req, verbose) = self.build_request();
        let body = handle_response(req.send().await?, verbose).await?;
        serde_json::from_str(&body).map_err(|e| ApiError::Deserialize {
            source: e,
            body,
        })
    }

    /// Send the request, ignoring the response body (for DELETE, etc.).
    pub async fn send_no_body(self) -> Result<(), ApiError> {
        let (req, verbose) = self.build_request();
        handle_response(req.send().await?, verbose).await?;
        Ok(())
    }
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
        if self.verbose {
            eprintln!("GET {url}");
        }
        let response = self
            .client
            .get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .send()
            .await?;
        let body = handle_response(response, self.verbose).await?;
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
        let url = format!("{}{}", self.baseurl, path);
        if self.verbose {
            eprintln!("{} {url}", method);
        }
        let mut req = self.client.request(method, &url);
        for (name, value) in headers {
            req = req.header(name, value);
        }
        if let Some(body) = body {
            if self.verbose {
                eprintln!("{}", String::from_utf8_lossy(body));
            }
            req = req.body(body.to_vec());
        }
        let response = req.send().await?;
        let status = response.status().as_u16();
        let http_version = match response.version() {
            reqwest::Version::HTTP_09 => "HTTP/0.9",
            reqwest::Version::HTTP_10 => "HTTP/1.0",
            reqwest::Version::HTTP_11 => "HTTP/1.1",
            reqwest::Version::HTTP_2 => "HTTP/2",
            reqwest::Version::HTTP_3 => "HTTP/3",
            _ => "HTTP/?",
        };
        let resp_headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
            .collect();
        if self.verbose {
            eprintln!("< {status}");
        }
        let bytes = response.bytes().await?.to_vec();
        if self.verbose && !bytes.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&bytes));
        }
        Ok(RawResponse {
            status,
            http_version,
            headers: resp_headers,
            body: bytes,
        })
    }
}

/// Read the response body, log it if verbose, and return it on success or an error on failure.
async fn handle_response(
    response: reqwest::Response,
    verbose: bool,
) -> Result<String, ApiError> {
    let status = response.status().as_u16();
    let is_success = (200..300).contains(&status);
    let body = if is_success {
        response.text().await?
    } else {
        response.text().await.unwrap_or_default()
    };
    if verbose {
        eprintln!("< {status}");
        if !body.is_empty() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                eprintln!("{}", serde_json::to_string_pretty(&json).unwrap());
            } else {
                eprintln!("{body}");
            }
        }
    }
    if is_success {
        Ok(body)
    } else {
        Err(ApiError::Http { status, body })
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

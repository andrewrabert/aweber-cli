use aweber::client::{RequestEvent, RequestKey, RequestObserver};

/// `--verbose` request tracing on stderr, in the format the CLI prints today.
///
/// A retry prints its attempt number and the wait taken; the request key and
/// the settlement are ignored, since one command traces one request at a time.
pub(crate) struct Verbose {
    bodies: bool,
}

impl Verbose {
    pub(crate) fn new(bodies: bool) -> Verbose {
        Verbose { bodies }
    }
}

impl RequestObserver for Verbose {
    fn observe(&self, _request: RequestKey, event: RequestEvent) {
        if !self.bodies {
            return;
        }
        match event {
            RequestEvent::Started {
                method, url, body, ..
            } => {
                eprintln!("{method} {url}");
                if let Some(body) = body {
                    eprintln!("{body}");
                }
            }
            RequestEvent::Finished { status, body, .. } => {
                eprintln!("< {status}");
                if let Some(body) = body.filter(|b| !b.is_empty()) {
                    match serde_json::from_str::<serde_json::Value>(&body) {
                        Ok(json) => {
                            eprintln!("{}", serde_json::to_string_pretty(&json).unwrap());
                        }
                        Err(_) => eprintln!("{body}"),
                    }
                }
            }
            RequestEvent::Waiting {
                attempt,
                status,
                wait,
            } => {
                eprintln!(
                    "< attempt {attempt} got {status}; waiting {:.3}s before retrying",
                    wait.as_secs_f64()
                );
            }
            RequestEvent::Refreshed { attempt } => {
                eprintln!("< attempt {attempt} refreshed the access token; retrying");
            }
            RequestEvent::Failed { attempt, error, .. } => {
                eprintln!("< attempt {attempt} failed: {error}");
            }
            RequestEvent::Settled { .. } => {}
        }
    }
}

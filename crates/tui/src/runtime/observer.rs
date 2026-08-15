//! Turns request events into actions, so every attempt lands in the Event Log
//! under the key the client stamped it with, however many requests are in
//! flight.

use aweber::client::{RequestEvent, RequestKey, RequestObserver};

use crate::core::{Action, LogId};

pub struct LogObserver {
    actions: tokio::sync::mpsc::UnboundedSender<Action>,
}

impl LogObserver {
    pub fn new(actions: tokio::sync::mpsc::UnboundedSender<Action>) -> LogObserver {
        LogObserver { actions }
    }
}

impl RequestObserver for LogObserver {
    fn observe(&self, key: RequestKey, event: RequestEvent) {
        let request = LogId::of(key);
        let action = match event {
            RequestEvent::Started {
                attempt,
                method,
                url,
                body,
            } => Action::RequestStarted {
                request,
                attempt,
                method: method.to_string(),
                path: url,
                body,
            },
            RequestEvent::Waiting {
                attempt,
                status,
                wait,
            } => Action::RequestWaiting {
                request,
                attempt,
                status,
                wait,
            },
            RequestEvent::Refreshed { attempt } => Action::RequestRefreshed { request, attempt },
            RequestEvent::Finished {
                attempt,
                status,
                duration,
                body,
            } => Action::RequestFinished {
                request,
                attempt,
                status,
                duration,
                body,
            },
            RequestEvent::Failed {
                attempt,
                duration,
                error,
            } => Action::RequestFinished {
                request,
                attempt,
                status: 0,
                duration,
                body: Some(error),
            },
            RequestEvent::Settled { .. } => Action::RequestSettled { request },
        };
        let _ = self.actions.send(action);
    }
}

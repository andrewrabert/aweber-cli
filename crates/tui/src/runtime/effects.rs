//! Every effect the core asks for, run somewhere the core cannot see.

use std::collections::HashMap;

use crate::core::{Action, Effect, RequestId, WatchId};

/// The effects the loop must run itself, holding the terminal.
pub enum Suspend {
    Edit {
        purpose: crate::core::effect::EditPurpose,
        seed: String,
        extension: &'static str,
    },
    Quit,
}

pub struct Interpreter {
    ports: super::Ports,
    actions: tokio::sync::mpsc::UnboundedSender<Action>,
    requests: HashMap<RequestId, tokio_util::sync::CancellationToken>,
    watches: HashMap<WatchId, tokio_util::sync::CancellationToken>,
}

impl Interpreter {
    pub fn new(
        ports: super::Ports,
        actions: tokio::sync::mpsc::UnboundedSender<Action>,
    ) -> Interpreter {
        Interpreter {
            ports,
            actions,
            requests: HashMap::new(),
            watches: HashMap::new(),
        }
    }

    /// A request that has answered drops its own token before the next effect
    /// runs.
    pub fn run(&mut self, effect: Effect) -> Option<Suspend> {
        self.prune();
        match effect {
            Effect::Send {
                request,
                generation,
                plan,
                purpose,
            } => {
                self.send(request, generation, plan, purpose);
                None
            }
            Effect::StartWatch {
                watch,
                plan,
                interval,
            } => {
                self.watch(watch, plan, interval);
                None
            }
            Effect::CancelWatch { watch } => {
                if let Some(token) = self.watches.remove(&watch) {
                    token.cancel();
                }
                None
            }
            Effect::Copy { text } => {
                let outcome = self.ports.clipboard.copy(&text);
                let _ = self.actions.send(Action::Copied(outcome));
                None
            }
            Effect::Write { path, text } => {
                let outcome = std::fs::write(&path, text.as_bytes())
                    .map(|()| path)
                    .map_err(|error| error.to_string());
                let _ = self.actions.send(Action::Wrote(outcome));
                None
            }
            Effect::Edit {
                purpose,
                seed,
                extension,
            } => Some(Suspend::Edit {
                purpose,
                seed,
                extension,
            }),
            Effect::OpenBrowser { url } => {
                let _ = open::that_detached(url);
                None
            }
            Effect::Login { code, verifier } => {
                let session = std::sync::Arc::clone(&self.ports.session);
                let actions = self.actions.clone();
                tokio::spawn(async move {
                    let outcome = session.login(code, verifier).await;
                    let _ = actions.send(Action::LoginFinished(outcome));
                });
                None
            }
            Effect::Logout => {
                let session = std::sync::Arc::clone(&self.ports.session);
                let actions = self.actions.clone();
                tokio::spawn(async move {
                    match session.logout().await {
                        Ok(()) => {
                            let _ = actions
                                .send(Action::SessionStatus(crate::ports::SessionStatus::Missing));
                        }
                        Err(message) => {
                            let _ = actions.send(Action::SessionFailed { message });
                        }
                    }
                });
                None
            }
            Effect::ReadSession => {
                let session = std::sync::Arc::clone(&self.ports.session);
                let actions = self.actions.clone();
                tokio::spawn(async move {
                    let status = session.status().await;
                    // The login the Session view offers is part of what the
                    // session is, so reading one answers with both, the status
                    // first because it is what opens the view.
                    let _ = actions.send(Action::SessionStatus(status));
                    let _ = actions.send(Action::LoginStarted(session.authorization()));
                });
                None
            }
            Effect::Quit => Some(Suspend::Quit),
        }
    }

    fn send(
        &mut self,
        request: RequestId,
        generation: crate::core::Generation,
        plan: aweber::catalog::RequestPlan,
        purpose: crate::core::effect::Purpose,
    ) {
        let token = tokio_util::sync::CancellationToken::new();
        self.requests.insert(request, token.clone());
        let http = std::sync::Arc::clone(&self.ports.http);
        let actions = self.actions.clone();
        let method = plan.method.clone();
        let line = plan.line();
        let cursor = plan.cursor.clone();
        tokio::spawn(async move {
            let _guard = token.clone().drop_guard();
            let outcome = tokio::select! {
                () = token.cancelled() => return,
                outcome = http.send(plan) => outcome,
            };
            let outcome = match outcome {
                Ok(response) => Ok(crate::core::collection::deliver(&cursor, response)),
                Err(error) => Err(crate::core::failure::Failure::from_error(
                    &method, &line, &error,
                )),
            };
            let _ = actions.send(Action::Delivered {
                request,
                generation,
                purpose,
                outcome,
            });
        });
    }

    fn watch(
        &mut self,
        watch: WatchId,
        plan: aweber::catalog::RequestPlan,
        interval: std::time::Duration,
    ) {
        let token = tokio_util::sync::CancellationToken::new();
        self.watches.insert(watch, token.clone());
        let http = std::sync::Arc::clone(&self.ports.http);
        let actions = self.actions.clone();
        let method = plan.method.clone();
        let line = plan.line();
        let cursor = plan.cursor.clone();
        tokio::spawn(async move {
            let _guard = token.clone().drop_guard();
            loop {
                let outcome = tokio::select! {
                    () = token.cancelled() => return,
                    outcome = http.send(plan.clone()) => outcome,
                };
                let outcome = match outcome {
                    Ok(response) => Ok(crate::core::collection::deliver(&cursor, response)),
                    Err(error) => Err(crate::core::failure::Failure::from_error(
                        &method, &line, &error,
                    )),
                };
                if actions
                    .send(Action::WatchPolled { watch, outcome })
                    .is_err()
                {
                    return;
                }
                tokio::select! {
                    () = token.cancelled() => return,
                    () = tokio::time::sleep(interval) => {}
                }
            }
        });
    }

    /// The requests and watches still holding a token.
    pub fn tracked(&self) -> usize {
        self.requests
            .values()
            .chain(self.watches.values())
            .filter(|token| !token.is_cancelled())
            .count()
    }

    /// What has been cancelled, whether by its own task answering or by the
    /// core, stops being tracked.
    fn prune(&mut self) {
        self.requests.retain(|_, token| !token.is_cancelled());
        self.watches.retain(|_, token| !token.is_cancelled());
    }

    /// A superseded page or a cancelled watch drops its token; no task outlives
    /// the process.
    pub fn shutdown(&mut self) {
        for (_, token) in self.requests.drain() {
            token.cancel();
        }
        for (_, token) in self.watches.drain() {
            token.cancel();
        }
    }
}

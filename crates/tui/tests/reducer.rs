//! The reducer, driven directly: no ports, no terminal, no clock.

use aweber::catalog::{ArgValue, Args, Operation, ValueKind};
use aweber_tui::core::cache::{Cache, CacheKey, Cached};
use aweber_tui::core::collection::Delivered;
use aweber_tui::core::effect::Purpose;
use aweber_tui::core::failure::{Failure, FailureKind};
use aweber_tui::core::state::{Account, Overlay, View};
use aweber_tui::core::{Action, Effect, Generation, State, update};
use aweber_tui::harness::fixed_now;
use aweber_tui::ports::SessionStatus;

/// A state with one account resolved and nothing else.
fn state() -> State {
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: None,
        name: Some("Acme".to_string()),
    });
    state.session = SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    state
}

fn act(state: &mut State, action: Action) -> Vec<Effect> {
    update(state, fixed_now(), action)
}

fn open(state: &mut State, operation: Operation) -> Vec<Effect> {
    act(
        state,
        Action::OpenOperation {
            operation,
            args: Args::default(),
        },
    )
}

/// The generation and request a `Send` effect carries.
fn sent(effects: &[Effect]) -> (aweber_tui::core::RequestId, Generation) {
    effects
        .iter()
        .find_map(|effect| match effect {
            Effect::Send {
                request,
                generation,
                ..
            } => Some((*request, *generation)),
            _ => None,
        })
        .expect("a request was sent")
}

fn page(entries: Vec<serde_json::Value>, next: bool) -> Delivered {
    let mut document = serde_json::json!({ "entries": entries.clone() });
    if next {
        document["next_collection_link"] =
            serde_json::json!("https://api.example.com/1.0/accounts/1/lists?ws.start=1");
    }
    Delivered {
        status: 200,
        document: Some(document),
        entries,
        next: next.then(|| {
            aweber_tui::core::collection::Continuation::Url(
                "https://api.example.com/1.0/accounts/1/lists?ws.start=1".to_string(),
            )
        }),
        capped: None,
        total: None,
    }
}

#[test]
fn the_reducer_moves_the_stack_and_the_context() {
    let mut state = state();
    assert_eq!(state.breadcrumb(), vec!["home".to_string()]);

    let effects = open(&mut state, Operation::ListLists);
    assert_eq!(state.stack.len(), 2, "opening a collection pushes a view");
    let (_, generation) = sent(&effects);

    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Collection {
                key: CacheKey {
                    operation: Operation::ListLists,
                    args: Args::default(),
                },
            },
            outcome: Ok(page(
                vec![serde_json::json!({ "id": 7, "name": "Weekly" })],
                false,
            )),
        },
    );
    let View::Collection(view) = state.view() else {
        panic!("the collection is on top")
    };
    assert_eq!(view.rows.len(), 1);
    assert_eq!(view.count_label(), "loaded 1");

    act(&mut state, Action::Descend);
    assert_eq!(
        state.context.list.as_ref().map(|list| list.id),
        Some(7),
        "descending into a list adopts it as the ambient list"
    );

    act(&mut state, Action::Pop);
    assert_eq!(state.stack.len(), 2, "popping returns to the collection");
    act(&mut state, Action::Pop);
    assert_eq!(state.breadcrumb(), vec!["home".to_string()]);
    act(&mut state, Action::Pop);
    assert_eq!(state.stack.len(), 1, "home is never popped");
}

#[test]
fn a_superseded_page_is_discarded() {
    let mut state = state();
    let effects = open(&mut state, Operation::ListLists);
    let (_, first) = sent(&effects);
    let effects = act(&mut state, Action::Refresh);
    let (_, second) = sent(&effects);
    assert!(second > first, "a refresh takes a fresh generation");

    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation: first,
            purpose: Purpose::Collection {
                key: CacheKey {
                    operation: Operation::ListLists,
                    args: Args::default(),
                },
            },
            outcome: Ok(page(vec![serde_json::json!({ "id": 1 })], false)),
        },
    );
    let View::Collection(view) = state.view() else {
        panic!("the collection is on top")
    };
    assert!(
        view.rows.is_empty(),
        "the superseded page never reached the view"
    );
}

#[test]
fn a_cached_collection_asks_for_nothing() {
    let mut state = state();
    let key = CacheKey {
        operation: Operation::ListLists,
        args: state.prefill(Operation::ListLists),
    };
    let mut cache = Cache::default();
    cache.put(
        key.clone(),
        Cached {
            rows: aweber_tui::core::collection::rows(
                vec![serde_json::json!({ "id": 3, "name": "Cached" })],
                aweber_tui::catalog::metadata(Operation::ListLists).columns,
            ),
            document: None,
            cursor: None,
            end: None,
            total: Some(1),
            selected: 0,
            offset: 0,
        },
    );
    state.cache = cache;
    let effects = open(&mut state, Operation::ListLists);
    assert!(effects.is_empty(), "a cache hit sends nothing");
    let View::Collection(view) = state.view() else {
        panic!("the collection is on top")
    };
    assert_eq!(view.count_label(), "1 of 1");
}

#[test]
fn an_empty_bodied_mutation_still_toasts_and_invalidates() {
    let mut state = state();
    open(&mut state, Operation::ListLists);
    let key = CacheKey {
        operation: Operation::ListLists,
        args: state.prefill(Operation::ListLists),
    };
    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    args.set("broadcast-id", ArgValue::new(ValueKind::Integer, "9"));

    let effects = act(
        &mut state,
        Action::RunOperation {
            operation: Operation::CancelBroadcast,
            args: args.clone(),
        },
    );
    assert!(effects.is_empty(), "a mutation asks before it sends");
    assert!(
        matches!(state.overlay, Some(Overlay::Confirm(_))),
        "the confirmation names the effect"
    );

    let effects = act(&mut state, Action::Confirm);
    let (_, generation) = sent(&effects);
    assert_eq!(state.inflight.mutating, 1);

    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Mutation {
                operation: Operation::CancelBroadcast,
                args,
                target: None,
            },
            outcome: Ok(Delivered {
                status: 204,
                document: None,
                entries: Vec::new(),
                next: None,
                capped: None,
                total: None,
            }),
        },
    );
    assert_eq!(state.inflight.mutating, 0);
    assert_eq!(state.toasts.len(), 1, "success is announced");
    assert!(state.toasts[0].text.contains("204"));
    assert!(
        state.cache.get(&key).is_none(),
        "the parent collection was dropped from the cache"
    );
}

#[test]
fn a_phrase_confirmation_refuses_the_wrong_words() {
    let mut state = state();
    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    args.set("broadcast-id", ArgValue::new(ValueKind::Integer, "9"));
    act(
        &mut state,
        Action::RunOperation {
            operation: Operation::DeleteBroadcast,
            args,
        },
    );
    let effects = act(&mut state, Action::Confirm);
    assert!(effects.is_empty(), "an unsatisfied phrase sends nothing");
    assert!(matches!(state.overlay, Some(Overlay::Confirm(_))));
    for character in "delete broadcast".chars() {
        act(&mut state, Action::Type(character));
    }
    let effects = act(&mut state, Action::Confirm);
    assert!(!effects.is_empty(), "the typed phrase releases the request");
    assert!(state.overlay.is_none());
}

#[test]
fn a_failure_opens_a_modal_and_a_403_does_not() {
    let mut state = state();
    let effects = open(&mut state, Operation::ListLists);
    let (_, generation) = sent(&effects);
    let key = CacheKey {
        operation: Operation::ListLists,
        args: state.prefill(Operation::ListLists),
    };
    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Collection { key: key.clone() },
            outcome: Err(Failure {
                status: Some(403),
                method: "GET".to_string(),
                path: "/1.0/accounts/1/lists".to_string(),
                message: "forbidden".to_string(),
                kind: FailureKind::Http,
            }),
        },
    );
    assert!(state.overlay.is_none(), "a 403 stays inline");
    let View::Collection(view) = state.view() else {
        panic!("the collection is on top")
    };
    assert!(view.unavailable.is_some());

    let effects = act(&mut state, Action::Refresh);
    let (_, generation) = sent(&effects);
    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Collection { key },
            outcome: Err(Failure {
                status: Some(500),
                method: "GET".to_string(),
                path: "/1.0/accounts/1/lists".to_string(),
                message: "boom".to_string(),
                kind: FailureKind::Http,
            }),
        },
    );
    assert!(
        matches!(state.overlay, Some(Overlay::Error(_))),
        "anything else is a blocking modal"
    );
}

#[test]
fn a_dead_session_holds_the_action_and_replays_it() {
    let mut state = state();
    let effects = open(&mut state, Operation::ListLists);
    let (_, generation) = sent(&effects);
    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Collection {
                key: CacheKey {
                    operation: Operation::ListLists,
                    args: Args::default(),
                },
            },
            outcome: Err(Failure {
                status: None,
                method: "GET".to_string(),
                path: "/1.0/accounts/1/lists".to_string(),
                message: "the refresh failed".to_string(),
                kind: FailureKind::Session,
            }),
        },
    );
    assert!(
        matches!(state.view(), View::Session(_)),
        "a dead session takes the screen"
    );
    assert!(state.held.is_some(), "the action that needed it is held");

    let effects = act(
        &mut state,
        Action::LoginFinished(Ok(SessionStatus::Active {
            account_id: 1,
            expires_in_secs: 3600,
        })),
    );
    assert!(
        !matches!(state.view(), View::Session(_)),
        "logging in returns to the view"
    );
    assert!(!effects.is_empty(), "the held action was replayed");
    assert!(state.held.is_none());
}

#[test]
fn reaching_the_tail_fetches_the_next_page_with_no_keystroke() {
    let mut state = state();
    let effects = open(&mut state, Operation::ListLists);
    let (_, generation) = sent(&effects);
    let key = CacheKey {
        operation: Operation::ListLists,
        args: state.prefill(Operation::ListLists),
    };
    let effects = act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Collection { key },
            outcome: Ok(page(vec![serde_json::json!({ "id": 1 })], true)),
        },
    );
    assert!(
        effects
            .iter()
            .any(|effect| matches!(effect, Effect::Send { .. })),
        "a cursor at the tail is followed at once"
    );
}

#[test]
fn a_local_filter_says_it_only_covers_what_is_loaded() {
    let mut state = state();
    let effects = open(&mut state, Operation::ListLists);
    let (_, generation) = sent(&effects);
    let key = CacheKey {
        operation: Operation::ListLists,
        args: state.prefill(Operation::ListLists),
    };
    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Collection { key },
            outcome: Ok(page(
                vec![
                    serde_json::json!({ "id": 1, "name": "Weekly" }),
                    serde_json::json!({ "id": 2, "name": "Monthly" }),
                ],
                false,
            )),
        },
    );
    act(&mut state, Action::OpenFilter);
    for character in "week".chars() {
        act(&mut state, Action::Type(character));
    }
    act(&mut state, Action::Submit);
    let View::Collection(view) = state.view() else {
        panic!("the collection is on top")
    };
    let filter = view.filter.as_ref().expect("the filter was applied");
    assert_eq!(filter.label(view.rows.len()), "filter 1 of 2 loaded");
}

/// A `Link` cursor past the offset cap ends the collection stating the offset,
/// and no further page is asked for.
#[tokio::test]
async fn a_capped_cursor_ends_the_collection() {
    let mut state = state();
    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    args.set("broadcast-id", ArgValue::new(ValueKind::Integer, "9"));
    let effects = act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::GetBroadcastClicks,
            args: args.clone(),
        },
    );
    let (_, generation) = sent(&effects);
    let capped = format!(
        "https://api.example.com/clicks?after=abc%2C{}",
        aweber::pagination::MAX_OFFSET_CURSOR + 1
    );
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::LINK,
        format!("<{capped}>; rel=\"next\"")
            .parse()
            .expect("the link header is well formed"),
    );
    let response = aweber::client::PlanResponse {
        status: 200,
        headers,
        body: Some(serde_json::json!({ "entries": [{ "url": "https://a", "count": 1 }] })),
    };
    let cursor = aweber::catalog::CursorStyle::LinkHeader {
        parameter: "after",
        arg: "after",
    };
    let delivered = aweber_tui::core::collection::deliver(&cursor, response);
    assert_eq!(
        delivered.capped,
        Some(aweber::pagination::MAX_OFFSET_CURSOR + 1),
        "the refused cursor is named by its offset"
    );

    let key = CacheKey {
        operation: Operation::GetBroadcastClicks,
        args,
    };
    let effects = act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Collection { key },
            outcome: Ok(delivered),
        },
    );
    assert!(effects.is_empty(), "no further page is asked for");
    let View::Collection(view) = state.view() else {
        panic!("the collection is on top");
    };
    assert_eq!(
        view.end,
        Some(aweber_tui::core::collection::EndReason::OffsetCap {
            offset: aweber::pagination::MAX_OFFSET_CURSOR + 1
        })
    );
    assert!(view.cursor.is_none(), "the refused cursor is not kept");
}

/// `q` with a mutation in flight toasts and quits on the second press, and the
/// mutation's own answer still decrements the count that armed it.
#[tokio::test]
async fn quitting_under_a_mutation_prompts_without_losing_the_count() {
    let mut state = state();
    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    args.set("broadcast-id", ArgValue::new(ValueKind::Integer, "9"));
    act(
        &mut state,
        Action::RunOperation {
            operation: Operation::CancelBroadcast,
            args: args.clone(),
        },
    );
    let effects = act(&mut state, Action::Confirm);
    let (_, generation) = sent(&effects);
    assert_eq!(state.inflight.mutating, 1);

    let effects = act(&mut state, Action::Quit);
    assert!(effects.is_empty(), "the first q asks rather than quits");
    assert!(state.quit_armed, "the prompt is armed");
    assert_eq!(state.inflight.mutating, 1, "the count is never touched");
    assert!(!state.quit);

    let effects = act(&mut state, Action::Quit);
    assert!(matches!(effects.as_slice(), [Effect::Quit]));
    assert!(state.quit);

    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Mutation {
                operation: Operation::CancelBroadcast,
                args,
                target: None,
            },
            outcome: Ok(Delivered {
                status: 204,
                document: None,
                entries: Vec::new(),
                next: None,
                capped: None,
                total: None,
            }),
        },
    );
    assert_eq!(
        state.inflight.mutating, 0,
        "the mutation that armed the prompt decremented the count"
    );
}

/// The purpose a `Send` effect carries.
fn purpose_of(effects: &[Effect]) -> Purpose {
    effects
        .iter()
        .find_map(|effect| match effect {
            Effect::Send { purpose, .. } => Some(purpose.clone()),
            _ => None,
        })
        .expect("a request was sent")
}

fn cached_naming(list: &str) -> (CacheKey, Cached) {
    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, list));
    (
        CacheKey {
            operation: Operation::ListBroadcasts,
            args,
        },
        Cached {
            rows: Vec::new(),
            document: None,
            cursor: None,
            end: None,
            total: None,
            selected: 0,
            offset: 0,
        },
    )
}

/// A selection moved while a mutation is in flight leaves the mutation
/// invalidating the entity it was run against, and the entity moved to cached.
#[test]
fn a_mutation_invalidates_the_entity_it_was_run_against() {
    let mut state = state();
    let effects = open(&mut state, Operation::ListLists);
    let (_, generation) = sent(&effects);
    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose: Purpose::Collection {
                key: CacheKey {
                    operation: Operation::ListLists,
                    args: Args::default(),
                },
            },
            outcome: Ok(page(
                vec![
                    serde_json::json!({ "id": 7, "name": "Weekly" }),
                    serde_json::json!({ "id": 8, "name": "Daily" }),
                ],
                false,
            )),
        },
    );

    let (run_against, run_against_cached) = cached_naming("7");
    let (moved_to, moved_to_cached) = cached_naming("8");
    state.cache.put(run_against.clone(), run_against_cached);
    state.cache.put(moved_to.clone(), moved_to_cached);

    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    args.set("broadcast-id", ArgValue::new(ValueKind::Integer, "9"));
    act(
        &mut state,
        Action::RunOperation {
            operation: Operation::CancelBroadcast,
            args,
        },
    );
    let effects = act(&mut state, Action::Confirm);
    let (_, generation) = sent(&effects);
    let purpose = purpose_of(&effects);

    act(
        &mut state,
        Action::Move(aweber_tui::core::action::Motion::Down),
    );
    assert_eq!(
        state.selected_entity().map(|entity| entity.id),
        Some("8".to_string()),
        "the selection moved while the mutation was in flight"
    );

    act(
        &mut state,
        Action::Delivered {
            request: aweber_tui::core::RequestId::default(),
            generation,
            purpose,
            outcome: Ok(Delivered {
                status: 204,
                document: None,
                entries: Vec::new(),
                next: None,
                capped: None,
                total: None,
            }),
        },
    );

    assert!(
        state.cache.get(&run_against).is_none(),
        "the entity the mutation was run against is still cached"
    );
    assert!(
        state.cache.get(&moved_to).is_some(),
        "the entity the selection moved to was dropped from the cache"
    );
}

/// One request answering while another waits out a backoff leaves the throttle
/// marker standing.
#[tokio::test]
async fn a_finished_request_clears_only_its_own_throttle() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/1.0/accounts/1/lists/7"))
        .respond_with(
            wiremock::ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({ "id": 7, "name": "Weekly" })),
        )
        .mount(&server)
        .await;

    let waiting = aweber_tui::core::LogId::default();
    let mut state = state();
    state
        .inflight
        .began(waiting, "GET", "/1.0/accounts/1/lists/9");
    state.inflight.waiting(waiting);
    let mut harness =
        aweber_tui::harness::Harness::against(&server.uri(), 80, 24).with_state(state);

    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    harness
        .dispatch(Action::OpenOperation {
            operation: Operation::GetList,
            args,
        })
        .settle()
        .await;

    assert!(
        matches!(harness.state().view(), View::Detail(_)),
        "another request answered"
    );
    assert!(
        harness.state().inflight.throttled(),
        "another request's answer cleared the wait"
    );

    harness.dispatch(Action::RequestFinished {
        request: waiting,
        attempt: 2,
        status: 200,
        duration: std::time::Duration::from_millis(5),
        body: None,
    });
    assert!(
        !harness.state().inflight.throttled(),
        "the request's own answer clears its wait"
    );
}

/// A request that started, waited out a backoff, and settled without answering
/// leaves nothing outstanding and no throttle marker.
#[test]
fn a_settlement_balances_a_request_that_never_answered() {
    let mut state = state();
    let at = fixed_now();
    let request = aweber_tui::core::LogId::from(1);
    update(
        &mut state,
        at,
        Action::RequestStarted {
            request,
            attempt: 1,
            method: "GET".to_string(),
            path: "/1.0/accounts/1/lists".to_string(),
            body: None,
        },
    );
    update(
        &mut state,
        at,
        Action::RequestWaiting {
            request,
            attempt: 1,
            status: 429,
            wait: std::time::Duration::from_secs(2),
        },
    );
    assert_eq!(state.inflight.outstanding(), 1);
    assert!(state.inflight.throttled());

    update(&mut state, at, Action::RequestSettled { request });

    assert_eq!(
        state.inflight.outstanding(),
        0,
        "a dropped request is still outstanding"
    );
    assert!(
        !state.inflight.throttled(),
        "a dropped request is still throttling"
    );
}

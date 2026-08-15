//! An answer belongs to the generation that asked for it, or to nothing.

use aweber::catalog::{ArgValue, Args, Operation, ValueKind};
use aweber_tui::core::cache::CacheKey;
use aweber_tui::core::collection::Delivered;
use aweber_tui::core::effect::Purpose;
use aweber_tui::core::failure::{Failure, FailureKind};
use aweber_tui::core::state::{Account, View};
use aweber_tui::core::{Action, Effect, Generation, RequestId, State, update};
use aweber_tui::harness::fixed_now;
use aweber_tui::ports::SessionStatus;

fn state() -> State {
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: Some("00000000-0000-0000-0000-000000000000".parse().unwrap()),
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

/// The generation a `Send` effect carries.
fn sent(effects: &[Effect]) -> Generation {
    effects
        .iter()
        .find_map(|effect| match effect {
            Effect::Send { generation, .. } => Some(*generation),
            _ => None,
        })
        .expect("a request was sent")
}

fn page(entries: Vec<serde_json::Value>, next: bool) -> Delivered {
    Delivered {
        status: 200,
        document: Some(serde_json::json!({ "entries": entries.clone() })),
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

fn forbidden() -> Failure {
    Failure {
        status: Some(403),
        method: "GET".to_string(),
        path: "/1.0/accounts/1/lists".to_string(),
        message: "not yours".to_string(),
        kind: FailureKind::Http,
    }
}

fn delivered(document: serde_json::Value) -> Delivered {
    Delivered {
        status: 200,
        document: Some(document),
        entries: Vec::new(),
        next: None,
        capped: None,
        total: None,
    }
}

fn answer(
    state: &mut State,
    generation: Generation,
    purpose: Purpose,
    document: serde_json::Value,
) {
    act(
        state,
        Action::Delivered {
            request: RequestId::default(),
            generation,
            purpose,
            outcome: Ok(delivered(document)),
        },
    );
}

fn subscriber_args() -> Args {
    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    args.set("subscriber-id", ArgValue::new(ValueKind::Integer, "3"));
    args
}

fn workflow_args() -> Args {
    let mut args = Args::default();
    args.set(
        "workflow",
        ArgValue::new(ValueKind::Uuid, uuid::Uuid::nil().to_string()),
    );
    args
}

/// The generation the view on top is waiting for.
fn generation_of(state: &State) -> Generation {
    match state.view() {
        View::Detail(view) => view.generation,
        View::Tree(view) => view.generation,
        View::RawRequest(view) => view.generation,
        _ => panic!("that view has no generation"),
    }
}

/// The generation before the first one ever handed out.
fn superseded() -> Generation {
    Generation::default()
}

/// A document, a tree, and a raw response from a superseded generation are all
/// dropped, and the view keeps what it has.
#[tokio::test]
async fn a_superseded_answer_never_reaches_a_view() {
    let mut state = state();
    act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::GetSubscriber,
            args: subscriber_args(),
        },
    );
    let stale = superseded();
    assert!(
        stale < generation_of(&state),
        "the answer is from an older generation"
    );
    answer(
        &mut state,
        stale,
        Purpose::Document {
            operation: Operation::GetSubscriber,
        },
        serde_json::json!({ "id": 3, "email": "stale@example.com" }),
    );
    let View::Detail(view) = state.view() else {
        panic!("the detail is on top");
    };
    assert!(view.document.is_none(), "the older answer was dropped");

    act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::TreeWorkflow,
            args: workflow_args(),
        },
    );
    let stale = superseded();
    assert!(
        stale < generation_of(&state),
        "the answer is from an older generation"
    );
    answer(
        &mut state,
        stale,
        Purpose::Document {
            operation: Operation::TreeWorkflow,
        },
        serde_json::json!({ "name": "stale" }),
    );
    let View::Tree(view) = state.view() else {
        panic!("the tree is on top");
    };
    assert!(view.items.is_empty(), "the older answer was dropped");

    act(&mut state, Action::OpenRawRequest);
    act(&mut state, Action::Submit);
    let stale = superseded();
    assert!(
        stale < generation_of(&state),
        "the answer is from an older generation"
    );
    answer(
        &mut state,
        stale,
        Purpose::Raw,
        serde_json::json!({ "stale": true }),
    );
    let View::RawRequest(view) = state.view() else {
        panic!("the raw request is on top");
    };
    assert!(view.response.is_none(), "the older answer was dropped");
}

/// The answer of the current generation is shown.
#[tokio::test]
async fn the_current_generation_is_shown() {
    let mut state = state();
    act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::GetSubscriber,
            args: subscriber_args(),
        },
    );
    let current = generation_of(&state);
    answer(
        &mut state,
        current,
        Purpose::Document {
            operation: Operation::GetSubscriber,
        },
        serde_json::json!({ "id": 3, "email": "ada@example.com" }),
    );
    let View::Detail(view) = state.view() else {
        panic!("the detail is on top");
    };
    assert!(view.document.is_some(), "the answer reached the view");
    assert!(!view.items.is_empty(), "its field tree was built");
    assert_eq!(
        view.entity.as_ref().map(|entity| entity.id.as_str()),
        Some("3"),
        "the view knows what an enrichment would name"
    );

    act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::TreeWorkflow,
            args: workflow_args(),
        },
    );
    let current = generation_of(&state);
    answer(
        &mut state,
        current,
        Purpose::Document {
            operation: Operation::TreeWorkflow,
        },
        serde_json::json!({ "name": "Welcome" }),
    );
    let View::Tree(view) = state.view() else {
        panic!("the tree is on top");
    };
    assert!(!view.items.is_empty(), "the answer reached the view");

    act(&mut state, Action::OpenRawRequest);
    act(&mut state, Action::Submit);
    let current = generation_of(&state);
    answer(
        &mut state,
        current,
        Purpose::Raw,
        serde_json::json!({ "ok": true }),
    );
    let View::RawRequest(view) = state.view() else {
        panic!("the raw request is on top");
    };
    assert!(view.response.is_some(), "the answer reached the view");
}

/// A `403` answering a request another view made leaves the collection on top
/// with its rows, its count, and no failure body.
#[tokio::test]
async fn a_dead_requests_failure_never_blanks_the_view_on_top() {
    let mut state = state();
    let effects = act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::ListLists,
            args: Args::default(),
        },
    );
    let collection = sent(&effects);
    act(
        &mut state,
        Action::Delivered {
            request: RequestId::default(),
            generation: collection,
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

    act(
        &mut state,
        Action::Delivered {
            request: RequestId::default(),
            generation: superseded(),
            purpose: Purpose::Document {
                operation: Operation::GetSubscriber,
            },
            outcome: Err(forbidden()),
        },
    );

    let View::Collection(view) = state.view() else {
        panic!("the collection is on top");
    };
    assert_eq!(view.rows.len(), 1, "the rows are still there");
    assert_eq!(view.count_label(), "loaded 1");
    assert!(
        view.unavailable.is_none(),
        "a dead request's failure blanked the view"
    );
    assert!(
        state.overlay.is_none(),
        "a dead request's failure opened a modal"
    );
}

/// A page delivered while a detail covers the collection that asked for it
/// lands on that collection, so popping back shows the page, the fetch is over,
/// and the tail asks for the next page.
#[tokio::test]
async fn a_page_reaches_the_collection_that_asked_for_it() {
    let mut state = state();
    let effects = act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::ListLists,
            args: Args::default(),
        },
    );
    let collection = sent(&effects);
    act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::GetSubscriber,
            args: subscriber_args(),
        },
    );
    assert!(
        matches!(state.view(), View::Detail(_)),
        "the detail covers the collection"
    );

    act(
        &mut state,
        Action::Delivered {
            request: RequestId::default(),
            generation: collection,
            purpose: Purpose::Collection {
                key: CacheKey {
                    operation: Operation::ListLists,
                    args: Args::default(),
                },
            },
            outcome: Ok(page(
                vec![serde_json::json!({ "id": 7, "name": "Weekly" })],
                true,
            )),
        },
    );

    act(&mut state, Action::Pop);
    let View::Collection(view) = state.view() else {
        panic!("the collection is on top");
    };
    assert_eq!(view.rows.len(), 1, "the page reached the collection");
    assert!(!view.loading, "the fetch is over");

    let effects = act(
        &mut state,
        Action::Move(aweber_tui::core::action::Motion::Down),
    );
    assert!(
        effects
            .iter()
            .any(|effect| matches!(effect, Effect::Send { .. })),
        "the tail asks for the next page"
    );
}

/// A delivered page clears the failure body the collection was showing.
#[tokio::test]
async fn a_delivered_page_clears_the_unavailable_body() {
    let mut state = state();
    let effects = act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::ListLists,
            args: Args::default(),
        },
    );
    let collection = sent(&effects);
    act(
        &mut state,
        Action::Delivered {
            request: RequestId::default(),
            generation: collection,
            purpose: Purpose::Collection {
                key: CacheKey {
                    operation: Operation::ListLists,
                    args: Args::default(),
                },
            },
            outcome: Err(forbidden()),
        },
    );
    let View::Collection(view) = state.view() else {
        panic!("the collection is on top");
    };
    assert!(view.unavailable.is_some(), "the failure landed inline");

    act(
        &mut state,
        Action::Delivered {
            request: RequestId::default(),
            generation: collection,
            purpose: Purpose::Page {
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
        panic!("the collection is on top");
    };
    assert!(
        view.unavailable.is_none(),
        "the delivered page left the failure body standing"
    );
    assert_eq!(view.rows.len(), 1, "the page is shown");
}

/// A document delivered while the Event Log covers the detail that asked for it
/// lands on that detail, so popping back shows the document rather than
/// `loading…`.
#[tokio::test]
async fn a_document_reaches_the_detail_that_asked_for_it() {
    let mut state = state();
    let effects = act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::GetSubscriber,
            args: subscriber_args(),
        },
    );
    let detail = sent(&effects);
    act(&mut state, Action::OpenEventLog);
    assert!(
        matches!(state.view(), View::EventLog(_)),
        "the Event Log covers the detail"
    );

    answer(
        &mut state,
        detail,
        Purpose::Document {
            operation: Operation::GetSubscriber,
        },
        serde_json::json!({ "id": 3, "email": "buried@example.com" }),
    );

    act(&mut state, Action::Pop);
    let View::Detail(view) = state.view() else {
        panic!("the detail is on top");
    };
    assert!(
        view.document.is_some(),
        "the document never reached the buried detail"
    );
}

/// A ruleset delivered while another view covers the tree that asked for it
/// lands on that tree.
#[tokio::test]
async fn a_ruleset_reaches_the_tree_that_asked_for_it() {
    let mut state = state();
    let effects = act(
        &mut state,
        Action::OpenOperation {
            operation: Operation::TreeWorkflow,
            args: workflow_args(),
        },
    );
    let tree = sent(&effects);
    act(&mut state, Action::OpenEventLog);
    assert!(
        matches!(state.view(), View::EventLog(_)),
        "the Event Log covers the tree"
    );

    answer(
        &mut state,
        tree,
        Purpose::Document {
            operation: Operation::TreeWorkflow,
        },
        serde_json::json!({ "name": "buried" }),
    );

    act(&mut state, Action::Pop);
    let View::Tree(view) = state.view() else {
        panic!("the tree is on top");
    };
    assert!(
        !view.items.is_empty(),
        "the ruleset never reached the buried tree"
    );
}

/// A raw response delivered while another view covers the Raw Request view that
/// sent it lands on that view.
#[tokio::test]
async fn a_raw_response_reaches_the_view_that_sent_it() {
    let mut state = state();
    act(&mut state, Action::OpenRawRequest);
    act(&mut state, Action::Submit);
    let raw = generation_of(&state);
    act(&mut state, Action::OpenEventLog);
    assert!(
        matches!(state.view(), View::EventLog(_)),
        "the Event Log covers the Raw Request view"
    );

    answer(
        &mut state,
        raw,
        Purpose::Raw,
        serde_json::json!({ "buried": true }),
    );

    act(&mut state, Action::Pop);
    let View::RawRequest(view) = state.view() else {
        panic!("the raw request is on top");
    };
    assert!(
        view.response.is_some(),
        "the response never reached the buried view"
    );
}

/// Candidates answering a builder that has been closed never reach the builder
/// that replaced it.
#[tokio::test]
async fn a_superseded_picker_answer_never_reaches_a_builder() {
    use aweber_tui::core::views::builder::{BuilderView, RowValue};

    let mut state = state();
    let first = state.next_generation();
    let view = BuilderView::open(
        Operation::MoveSubscriber,
        state.prefill(Operation::MoveSubscriber),
        first,
    );
    let row = view
        .rows
        .iter()
        .position(|row| matches!(row.value, RowValue::Pick { .. }))
        .expect("the operation has a picker row");
    state.stack.push(View::Builder(view));

    act(&mut state, Action::Pop);
    let second = state.next_generation();
    state.stack.push(View::Builder(BuilderView::open(
        Operation::MoveSubscriber,
        state.prefill(Operation::MoveSubscriber),
        second,
    )));
    assert!(
        second != first,
        "the replacement asks under its own generation"
    );

    act(
        &mut state,
        Action::Delivered {
            request: RequestId::default(),
            generation: first,
            purpose: Purpose::ListPicker { row },
            outcome: Ok(Delivered {
                status: 200,
                document: None,
                entries: vec![serde_json::json!({
                    "id": 9,
                    "name": "Stale",
                    "self_link": "https://api.example.com/1.0/accounts/1/lists/9",
                })],
                next: None,
                capped: None,
                total: None,
            }),
        },
    );

    let View::Builder(view) = state.view() else {
        panic!("the replacement builder is on top");
    };
    assert!(
        view.candidates(row).is_empty(),
        "a cancelled builder's candidates reached the builder that replaced it"
    );
}

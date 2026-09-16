//! What a key and what a pointer reach: trees move and expand, raw JSON
//! scrolls, and the mouse selects what is drawn under it.

use aweber::catalog::{ArgValue, Args, Operation, ValueKind};
use aweber_tui::core::state::{Account, View};
use aweber_tui::core::{Action, State};
use aweber_tui::harness::Harness;
use wiremock::matchers::{method, path, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// The crypto provider wiremock's TLS stack expects, installed once.
fn crypto() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

fn harness(server: &MockServer) -> Harness {
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: None,
        name: Some("Acme".to_string()),
    });
    state.session = aweber_tui::ports::SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    Harness::against(&server.uri(), 80, 24).with_state(state)
}

fn integer(value: &str) -> ArgValue {
    ArgValue::new(ValueKind::Integer, value)
}

/// A subscriber deep enough to have something to expand.
fn subscriber() -> serde_json::Value {
    serde_json::json!({
        "id": 3,
        "email": "ada@example.com",
        "status": "subscribed",
        "custom_fields": { "City": "Lovelace", "Plan": "gold" },
        "tags": ["beta", "vip"],
    })
}

fn open_subscriber() -> Action {
    let mut args = Args::default();
    args.set("list-id", integer("7"));
    args.set("subscriber-id", integer("3"));
    Action::OpenOperation {
        operation: Operation::GetSubscriber,
        args,
    }
}

/// The identifiers of the tree the detail currently has selected.
fn detail_selection(harness: &Harness) -> Vec<String> {
    match harness.state().view() {
        View::Detail(view) => view.tree.selected().to_vec(),
        other => panic!("the detail is on top, not {:?}", other.title()),
    }
}

/// `j`, `Ctrl-d`, and `G` move a detail's field tree, and `Enter` expands the
/// node they land on.
#[tokio::test]
async fn a_detail_moves_and_expands() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7/subscribers/3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(subscriber()))
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness.dispatch(open_subscriber()).settle().await;
    harness.screen();
    assert_eq!(
        detail_selection(&harness),
        vec!["id".to_string()],
        "the tree opens on its first row"
    );

    harness.key("j");
    assert_eq!(detail_selection(&harness), vec!["email".to_string()]);
    harness.key("Ctrl-d");
    harness.key("G");
    assert_eq!(
        detail_selection(&harness),
        vec!["tags".to_string()],
        "G lands on the last row"
    );

    harness.key("g").keys("j j j").key("Enter");
    let View::Detail(view) = harness.state().view() else {
        panic!("the detail is on top");
    };
    assert!(
        view.tree
            .opened()
            .contains(&vec!["custom_fields".to_string()]),
        "Enter expanded the node the motion landed on"
    );
}

/// `J` scrolls the raw JSON of a document taller than the screen, and the last
/// line is reachable.
#[tokio::test]
async fn raw_json_scrolls_to_the_end() {
    crypto();
    let server = MockServer::start().await;
    let mut document = serde_json::Map::new();
    for index in 0..80 {
        document.insert(format!("field_{index:02}"), serde_json::json!(index));
    }
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7/subscribers/3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::Value::Object(document)))
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness.dispatch(open_subscriber()).settle().await;
    harness.key("J");
    harness.screen();
    let lines = match harness.state().view() {
        View::Detail(view) => view.raw_lines(),
        _ => panic!("the detail is on top"),
    };
    assert!(lines > 24, "the document is taller than the screen");

    harness.key("G");
    let View::Detail(view) = harness.state().view() else {
        panic!("the detail is on top");
    };
    let height = harness.state().regions.rows.height as usize;
    assert_eq!(
        view.offset,
        lines.saturating_sub(height.max(1)),
        "the last line is reachable and the scroll stays inside the document"
    );

    harness.key("g");
    let View::Detail(view) = harness.state().view() else {
        panic!("the detail is on top");
    };
    assert_eq!(view.offset, 0, "g returns to the first line");
}

/// `G` on an enriched document's raw JSON draws its last line, the enrichment
/// rows taking no lines off the reach of the scroll.
#[tokio::test]
async fn an_enriched_documents_last_line_is_reachable() {
    crypto();
    let server = MockServer::start().await;
    let mut document = serde_json::Map::new();
    for index in 0..80 {
        document.insert(format!("field_{index:02}"), serde_json::json!(index));
    }
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7/subscribers/3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::Value::Object(document)))
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness.dispatch(open_subscriber()).settle().await;
    harness.key("J");
    harness.screen();

    let region = harness.state().regions.rows.height as usize;
    let (lines, body) = match harness.state().view() {
        View::Detail(view) => (view.raw_lines(), view.body_height(region)),
        _ => panic!("the detail is on top"),
    };
    assert_eq!(body, region, "an unenriched document owns the whole region");

    harness.key("G");
    let View::Detail(view) = harness.state().view() else {
        panic!("the detail is on top");
    };
    let plain = view.offset;
    assert_eq!(plain, lines.saturating_sub(region));

    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: None,
        name: Some("Acme".to_string()),
    });
    state.session = aweber_tui::ports::SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    let mut args = Args::default();
    args.set("list-id", integer("7"));
    args.set("subscriber-id", integer("3"));
    let mut detail = aweber_tui::core::views::detail::DetailView::opening(
        Operation::GetSubscriber,
        args,
        "subscribers get".to_string(),
        aweber_tui::core::Generation::default(),
    );
    let mut document = serde_json::Map::new();
    for index in 0..80 {
        document.insert(format!("field_{index:02}"), serde_json::json!(index));
    }
    detail.show(
        serde_json::Value::Object(document),
        chrono::FixedOffset::east_opt(0).expect("utc is an offset"),
    );
    detail.raw = true;
    for label in ["message subjects", "visualization"] {
        detail.enrich(label, Ok(serde_json::json!("something")));
    }
    state.stack.push(View::Detail(detail));

    let mut harness = Harness::against(&server.uri(), 80, 24).with_state(state);
    harness.screen();
    harness.key("G");
    let View::Detail(view) = harness.state().view() else {
        panic!("the detail is on top");
    };
    let region = harness.state().regions.rows.height as usize;
    assert_eq!(
        view.body_height(region),
        region - 2,
        "the enrichment rows take their lines off the JSON"
    );
    assert_eq!(
        view.offset,
        lines.saturating_sub(region - 2),
        "the last line is reachable with the enrichment rows drawn"
    );
    assert!(
        view.offset > plain,
        "an enriched document scrolls further than an unenriched one"
    );
}

/// `j` and `k` move the workflow tree's selection, and `Enter` opens the node.
#[tokio::test]
async fn a_workflow_tree_moves_and_expands() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path_regex(r"^/internal/campaign/campaigns/.*$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "Welcome",
            "unpublished": {
                "events": [{ "id": "11111111-1111-4111-8111-111111111111" }],
                "actions": [{ "id": "22222222-2222-4222-8222-222222222222" }],
            },
        })))
        .mount(&server)
        .await;

    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: Some("00000000-0000-0000-0000-000000000000".parse().unwrap()),
        name: None,
    });
    state.session = aweber_tui::ports::SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    let mut harness = Harness::against(&server.uri(), 80, 24).with_state(state);

    let mut args = Args::default();
    args.set(
        "workflow",
        ArgValue::new(ValueKind::Uuid, uuid::Uuid::nil().to_string()),
    );
    harness
        .dispatch(Action::OpenOperation {
            operation: Operation::TreeWorkflow,
            args,
        })
        .settle()
        .await;
    harness.screen();

    let first = match harness.state().view() {
        View::Tree(view) => view.state.selected().to_vec(),
        _ => panic!("the tree is on top"),
    };
    assert!(!first.is_empty(), "the tree opens on a row");

    harness.key("j");
    let second = match harness.state().view() {
        View::Tree(view) => view.state.selected().to_vec(),
        _ => panic!("the tree is on top"),
    };
    assert_ne!(second, first, "j moved the selection");
    harness.key("k");
    let back = match harness.state().view() {
        View::Tree(view) => view.state.selected().to_vec(),
        _ => panic!("the tree is on top"),
    };
    assert_eq!(back, first, "k moved it back");

    harness.key("Enter");
    let View::Tree(view) = harness.state().view() else {
        panic!("the tree is on top");
    };
    assert!(
        view.state.opened().contains(&back),
        "Enter opened the selected node"
    );
}

/// A wheel notch scrolls a collection, a click selects the row under the cell,
/// and both leave the selection where the keys would have put it.
#[tokio::test]
async fn the_mouse_scrolls_and_selects() {
    crypto();
    let server = MockServer::start().await;
    let entries: Vec<serde_json::Value> = (0..40)
        .map(|index| serde_json::json!({ "id": index, "name": format!("list {index:02}") }))
        .collect();
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "entries": entries,
            "total_size": 40,
        })))
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness
        .dispatch(Action::OpenOperation {
            operation: Operation::ListLists,
            args: Args::default(),
        })
        .settle()
        .await;
    harness.screen();
    let rows = harness.state().regions.rows;

    harness.wheel(rows.x + 1, rows.y + 1, true);
    let View::Collection(view) = harness.state().view() else {
        panic!("the collection is on top");
    };
    assert_eq!(view.selected, 3, "one notch is three rows");

    harness.wheel(rows.x + 1, rows.y + 1, false);
    let View::Collection(view) = harness.state().view() else {
        panic!("the collection is on top");
    };
    assert_eq!(view.selected, 0, "a notch back is three rows back");

    harness.click(rows.x + 1, rows.y + 4);
    let View::Collection(view) = harness.state().view() else {
        panic!("the collection is on top");
    };
    assert_eq!(
        view.selected, 4,
        "the click named the row drawn under the cell"
    );

    harness.keys("j j j j");
    let View::Collection(view) = harness.state().view() else {
        panic!("the collection is on top");
    };
    assert_eq!(
        view.selected, 8,
        "the keys carry on from where the click was"
    );
}

/// A click on a tree node selects that node and nothing else.
#[tokio::test]
async fn a_click_selects_a_tree_node() {
    crypto();
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists/7/subscribers/3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(subscriber()))
        .mount(&server)
        .await;

    let mut harness = harness(&server);
    harness.dispatch(open_subscriber()).settle().await;
    harness.screen();
    let rows = harness.state().regions.rows;

    harness.click(rows.x + 2, rows.y + 2);
    assert_eq!(
        detail_selection(&harness),
        vec!["status".to_string()],
        "the third node drawn is the one the click named"
    );
}

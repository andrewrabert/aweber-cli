//! A row that names another entity resolves it, and never sends a name in its
//! place.

use aweber_tui::core::State;
use aweber_tui::core::state::{Account, View};
use aweber_tui::core::views::builder::RowValue;
use aweber_tui::harness::Harness;
use wiremock::matchers::{method, path};
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
        uuid: Some("00000000-0000-0000-0000-000000000000".parse().unwrap()),
        name: Some("Acme".to_string()),
    });
    state.session = aweber_tui::ports::SessionStatus::Active {
        account_id: 1,
        expires_in_secs: 3600,
    };
    Harness::against(&server.uri(), 80, 24).with_state(state)
}

/// The lists a `list-link` row chooses from.
async fn lists(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/1.0/accounts/1/lists"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "entries": [{
                "id": 7,
                "uuid": "11111111-1111-4111-8111-111111111111",
                "name": "Weekly",
                "self_link": "https://api.aweber.com/1.0/accounts/1/lists/7",
            }],
            "total_size": 1,
        })))
        .mount(server)
        .await;
}

/// The builder of the operation under test.
fn builder(harness: &Harness) -> &aweber_tui::core::views::builder::BuilderView {
    match harness.state().view() {
        View::Builder(view) => view,
        _ => panic!("the builder is on top"),
    }
}

/// The index of the row an argument is entered in.
fn row_of(harness: &Harness, name: &str) -> usize {
    builder(harness)
        .rows
        .iter()
        .position(|row| row.spec.name == name)
        .unwrap_or_else(|| panic!("{name} has a row"))
}

/// The palette, opened and typed into, as the user would.
async fn choose(harness: &mut Harness, label: &str) {
    harness.key("Ctrl-p");
    for character in label.chars() {
        if character == ' ' {
            harness.key("Space");
        } else {
            harness.key(&character.to_string());
        }
    }
    harness.key("Enter").settle().await;
}

/// A row the builder does not start with, added from its argument palette.
async fn add_row(harness: &mut Harness, name: &str) {
    choose(harness, name).await;
    harness.settle().await;
}

/// A `list-link` row loads its candidates, and choosing one sends the list's
/// `self_link`.
#[tokio::test]
async fn a_link_row_sends_a_resolved_link() {
    crypto();
    let server = MockServer::start().await;
    lists(&server).await;

    let mut harness = harness(&server);
    choose(&mut harness, "subscribers move").await;
    add_row(&mut harness, "list-link").await;

    let row = row_of(&harness, "list-link");
    assert_eq!(
        builder(&harness).candidates(row).len(),
        1,
        "the row loaded its candidates once it appeared"
    );

    let RowValue::Pick { chosen, .. } = &builder(&harness).rows[row].value else {
        panic!("a list-link row is a picker row");
    };
    assert!(chosen.is_none(), "nothing is chosen yet");

    while builder(&harness).selected != row {
        harness.key("Tab");
    }
    harness.key("Enter");
    harness.key("Enter");

    let RowValue::Pick { chosen, .. } = &builder(&harness).rows[row].value else {
        panic!("a list-link row is a picker row");
    };
    let chosen = chosen.as_ref().expect("the candidate was chosen");
    assert_eq!(
        chosen.value.raw, "https://api.aweber.com/1.0/accounts/1/lists/7",
        "the list's self_link is what goes out"
    );
    assert_eq!(
        builder(&harness)
            .args()
            .first("list-link")
            .map(|value| value.raw.as_str()),
        Some("https://api.aweber.com/1.0/accounts/1/lists/7")
    );
}

/// A name typed into a picker row is not sent as a link, and the preview says
/// the argument is missing.
#[tokio::test]
async fn a_typed_name_is_never_sent_as_a_link() {
    crypto();
    let server = MockServer::start().await;
    lists(&server).await;

    let mut harness = harness(&server);
    choose(&mut harness, "subscribers move").await;
    add_row(&mut harness, "list-link").await;

    let row = row_of(&harness, "list-link");
    while builder(&harness).selected != row {
        harness.key("Tab");
    }
    harness.keys("W e e k l y");

    let RowValue::Pick { query, chosen, .. } = &builder(&harness).rows[row].value else {
        panic!("a list-link row is a picker row");
    };
    assert_eq!(query.value(), "Weekly", "the name was typed");
    assert!(chosen.is_none(), "a name is not a link");
    assert!(
        builder(&harness).args().first("list-link").is_none(),
        "nothing unresolved is sent"
    );
    assert!(
        builder(&harness).preview.is_err(),
        "the preview says the argument is missing"
    );
}

/// A uuid typed into a `workflow` row is taken as resolved.
#[tokio::test]
async fn a_typed_uuid_resolves_itself() {
    crypto();
    let server = MockServer::start().await;
    lists(&server).await;

    let mut harness = harness(&server);
    choose(&mut harness, "workflows get").await;

    let row = row_of(&harness, "workflow");
    while builder(&harness).selected != row {
        harness.key("Tab");
    }
    let uuid = "11111111-1111-4111-8111-111111111111";
    for character in uuid.chars() {
        harness.key(&character.to_string());
    }

    let RowValue::Pick { chosen, .. } = &builder(&harness).rows[row].value else {
        panic!("a workflow row is a picker row");
    };
    assert_eq!(
        chosen
            .as_ref()
            .map(|candidate| candidate.value.raw.as_str()),
        Some(uuid),
        "a typed uuid is already resolved"
    );
}

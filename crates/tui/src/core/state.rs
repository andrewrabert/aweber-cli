//! Everything the application knows, and nothing it does.

use aweber::catalog::{ArgValue, Args, Operation, Scope, ValueKind};

use crate::catalog::{ContextSource, EntityKind};
use crate::core::views;

pub struct State {
    pub context: Context,
    pub stack: Vec<View>,
    pub overlay: Option<Overlay>,
    pub toasts: Vec<Toast>,
    pub watches: Vec<Watch>,
    pub log: crate::core::event_log::EventLog,
    pub cache: crate::core::cache::Cache,
    pub inflight: Inflight,
    pub regions: Regions,
    pub size: (u16, u16),
    pub session: crate::ports::SessionStatus,
    pub accounts: Vec<Account>,
    pub verbose: bool,
    /// The action to replay once login succeeds.
    pub held: Option<crate::core::Action>,
    /// A `q` pressed while a mutation was in flight, awaiting its second press.
    pub quit_armed: bool,
    pub quit: bool,
    requests: crate::core::RequestId,
    generations: crate::core::Generation,
    watch_ids: crate::core::WatchId,
}

impl State {
    pub fn new(stored: Option<crate::StoredAccount>, verbose: bool) -> State {
        let account = stored.map(|stored| Account {
            id: stored.id,
            uuid: stored.uuid,
            name: None,
        });
        State {
            context: Context {
                account,
                list: None,
            },
            stack: vec![View::Home(views::home::HomeView::default())],
            overlay: None,
            toasts: Vec::new(),
            watches: Vec::new(),
            log: crate::core::event_log::EventLog::default(),
            cache: crate::core::cache::Cache::default(),
            inflight: Inflight::default(),
            regions: Regions::default(),
            size: (80, 24),
            session: crate::ports::SessionStatus::Missing,
            accounts: Vec::new(),
            verbose,
            held: None,
            quit_armed: false,
            quit: false,
            requests: crate::core::RequestId::default(),
            generations: crate::core::Generation::default(),
            watch_ids: crate::core::WatchId::default(),
        }
    }

    pub fn breadcrumb(&self) -> Vec<String> {
        self.stack.iter().map(View::title).collect()
    }

    pub fn scope(&self) -> Option<Scope> {
        self.context.account.as_ref().map(|account| Scope {
            account_id: account.id,
            account: account.uuid,
        })
    }

    pub fn view(&self) -> &View {
        self.stack.last().expect("the stack is never empty")
    }

    pub fn view_mut(&mut self) -> &mut View {
        self.stack.last_mut().expect("the stack is never empty")
    }

    pub fn scope_of_keys(&self) -> crate::core::keys::Scope {
        use crate::core::keys::Scope as K;
        match &self.overlay {
            Some(Overlay::Palette(_)) => return K::Palette,
            Some(Overlay::Help(_)) => return K::Help,
            Some(Overlay::Confirm(_)) => return K::Confirm,
            Some(Overlay::Error(_)) => return K::Error,
            Some(Overlay::Filter(_)) => return K::Filter,
            None => {}
        }
        match self.view() {
            View::Home(_) => K::Home,
            View::AccountPicker(_) => K::AccountPicker,
            View::Collection(_) => K::Collection,
            View::Detail(_) => K::Detail,
            View::Tree(_) => K::Tree,
            View::Session(_) => K::Session,
            View::RawRequest(_) => K::RawRequest,
            View::EventLog(_) => K::EventLog,
            View::Builder(_) => K::Builder,
        }
    }

    pub fn selection(&self) -> Option<Selection> {
        match self.view() {
            View::Collection(view) => {
                let row = view.selected_row()?;
                Some(Selection {
                    kind: crate::catalog::metadata(view.operation).entity,
                    document: row.document.clone(),
                    self_link: row.self_link.clone(),
                })
            }
            View::Detail(view) => {
                let document = view.document.clone()?;
                let self_link = document
                    .pointer("/self_link")
                    .and_then(serde_json::Value::as_str)
                    .map(ToString::to_string);
                Some(Selection {
                    kind: crate::catalog::metadata(view.operation).entity,
                    document,
                    self_link,
                })
            }
            _ => None,
        }
    }

    /// The entity the selection names, captured when a mutation is issued so
    /// its success invalidates what it was run against.
    pub fn selected_entity(&self) -> Option<EntityKey> {
        let selection = self.selection()?;
        let id = selection
            .document
            .pointer("/id")
            .or_else(|| selection.document.pointer("/uuid"))?;
        Some(EntityKey {
            kind: selection.kind,
            id: match id {
                serde_json::Value::String(text) => text.clone(),
                other => other.to_string(),
            },
        })
    }

    pub fn next_request(&mut self) -> crate::core::RequestId {
        self.requests.next()
    }

    pub fn next_generation(&mut self) -> crate::core::Generation {
        self.generations.next()
    }

    pub fn next_watch(&mut self) -> crate::core::WatchId {
        self.watch_ids.next()
    }

    /// Fills an operation's identifier arguments from the context and the
    /// selection.
    pub fn prefill(&self, operation: Operation) -> Args {
        let selection = self.selection();
        let mut args = Args::default();
        for fill in crate::catalog::metadata(operation).fills {
            if let Some(value) = self.filled(fill.source, selection.as_ref()) {
                args.set(fill.arg, value);
            }
        }
        args
    }

    fn filled(&self, source: ContextSource, selection: Option<&Selection>) -> Option<ArgValue> {
        let document = selection.map(|selection| &selection.document);
        let text = |pointer: &str| {
            document?
                .pointer(pointer)
                .map(|value| match value {
                    serde_json::Value::String(text) => text.clone(),
                    other => other.to_string(),
                })
                .filter(|text| text != "null")
        };
        match source {
            ContextSource::AccountId => self
                .context
                .account
                .as_ref()
                .map(|account| ArgValue::new(ValueKind::Integer, account.id.to_string())),
            ContextSource::AccountUuid => self
                .context
                .account
                .as_ref()
                .and_then(|account| account.uuid)
                .map(|uuid| ArgValue::new(ValueKind::Uuid, uuid.to_string())),
            ContextSource::ListId => self
                .context
                .list
                .as_ref()
                .map(|list| ArgValue::new(ValueKind::Integer, list.id.to_string())),
            ContextSource::ListUuid => self
                .context
                .list
                .as_ref()
                .and_then(|list| list.uuid)
                .map(|uuid| ArgValue::new(ValueKind::Uuid, uuid.to_string())),
            ContextSource::SelectionId => {
                text("/id").map(|id| ArgValue::new(ValueKind::Integer, id))
            }
            ContextSource::SelectionUuid => text("/uuid")
                .or_else(|| text("/id"))
                .map(|id| ArgValue::new(ValueKind::Uuid, id)),
            ContextSource::SelectionSelfLink => selection
                .and_then(|selection| selection.self_link.clone())
                .map(ArgValue::text),
            ContextSource::SelectionEmail => text("/email").map(ArgValue::text),
            ContextSource::SelectionMessageId => text("/message_id").map(ArgValue::text),
            ContextSource::SelectionEventId => text("/event_id").map(ArgValue::text),
            ContextSource::SelectionPrecondition => text("/precondition_version")
                .map(|version| ArgValue::new(ValueKind::Integer, version)),
        }
    }

    pub fn too_small(&self) -> bool {
        self.size.0 < crate::view::chrome::MIN_WIDTH
            || self.size.1 < crate::view::chrome::MIN_HEIGHT
    }
}

#[derive(Clone, Debug, Default)]
pub struct Context {
    pub account: Option<Account>,
    pub list: Option<ListRef>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Account {
    pub id: i32,
    /// The uid the internal services name the account by, from its `uuid` member.
    pub uuid: Option<aweber::ids::AccountUid>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListRef {
    pub id: i32,
    pub uuid: Option<uuid::Uuid>,
    pub name: Option<String>,
    pub self_link: Option<String>,
}

pub enum View {
    Home(views::home::HomeView),
    AccountPicker(views::account_picker::AccountPickerView),
    Collection(views::collection::CollectionView),
    Detail(views::detail::DetailView),
    Tree(views::tree::TreeView),
    Session(views::session::SessionView),
    RawRequest(views::raw_request::RawRequestView),
    EventLog(views::event_log::EventLogView),
    Builder(views::builder::BuilderView),
}

impl View {
    /// The breadcrumb name of this view.
    pub fn title(&self) -> String {
        match self {
            View::Home(_) => "home".to_string(),
            View::AccountPicker(_) => "accounts".to_string(),
            View::Collection(view) => view.title.clone(),
            View::Detail(view) => view.title.clone(),
            View::Tree(view) => view.title.clone(),
            View::Session(_) => "session".to_string(),
            View::RawRequest(_) => "raw request".to_string(),
            View::EventLog(_) => "event log".to_string(),
            View::Builder(view) => crate::catalog::entry(view.operation).label.clone(),
        }
    }

    /// The generation an answer must carry to reach this view; `Home`,
    /// `AccountPicker`, `Session`, and `EventLog` ask for nothing and have none.
    pub fn generation(&self) -> Option<crate::core::Generation> {
        match self {
            View::Collection(view) => Some(view.generation),
            View::Detail(view) => Some(view.generation),
            View::Tree(view) => Some(view.generation),
            View::RawRequest(view) => Some(view.generation),
            View::Builder(view) => Some(view.generation),
            View::Home(_) | View::AccountPicker(_) | View::Session(_) | View::EventLog(_) => None,
        }
    }
}

pub enum Overlay {
    Palette(views::palette::PaletteView),
    Help(views::help::HelpView),
    Confirm(views::confirm::ConfirmView),
    /// Blocking: carries status, `/error/message`, method, and path.
    Error(crate::core::failure::Failure),
    Filter(crate::core::filter::FilterPrompt),
}

#[derive(Clone, Debug)]
pub struct Toast {
    pub at: crate::core::Timestamp,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct Watch {
    pub id: crate::core::WatchId,
    pub label: String,
    pub started_at: crate::core::Timestamp,
    pub operation: Operation,
    pub args: Args,
}

/// Every logical request that has started and not settled, and the mutations
/// among what is outstanding.
#[derive(Clone, Debug, Default)]
pub struct Inflight {
    pub mutating: usize,
    /// The started-and-unsettled requests, each with the method and path of its
    /// latest attempt and whether it is waiting out a backoff.
    started: std::collections::HashMap<crate::core::LogId, Attempted>,
}

#[derive(Clone, Debug)]
struct Attempted {
    method: String,
    path: String,
    waiting: bool,
}

impl Inflight {
    /// The first attempt of a request makes it outstanding; a later attempt
    /// restates the method and path the log attributes to it.
    pub fn began(&mut self, request: crate::core::LogId, method: &str, path: &str) {
        let attempted = self.started.entry(request).or_insert_with(|| Attempted {
            method: String::new(),
            path: String::new(),
            waiting: false,
        });
        attempted.method = method.to_string();
        attempted.path = path.to_string();
    }

    /// The request is waiting out a backoff until its next attempt answers or it
    /// settles.
    pub fn waiting(&mut self, request: crate::core::LogId) {
        if let Some(attempted) = self.started.get_mut(&request) {
            attempted.waiting = true;
        }
    }

    /// An attempt that answered clears that request's wait and no other's.
    pub fn answered(&mut self, request: crate::core::LogId) {
        if let Some(attempted) = self.started.get_mut(&request) {
            attempted.waiting = false;
        }
    }

    /// Balances `began` however the request ended: answered, failed, or dropped
    /// before it could answer.
    pub fn settled(&mut self, request: crate::core::LogId) {
        self.started.remove(&request);
    }

    /// The method and path of the latest attempt, empty where the request is no
    /// longer in flight.
    pub fn attempted(&self, request: crate::core::LogId) -> (String, String) {
        match self.started.get(&request) {
            Some(attempted) => (attempted.method.clone(), attempted.path.clone()),
            None => (String::new(), String::new()),
        }
    }

    pub fn outstanding(&self) -> usize {
        self.started.len()
    }

    pub fn throttled(&self) -> bool {
        self.started.values().any(|attempted| attempted.waiting)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Regions {
    pub header: ratatui::layout::Rect,
    pub body: ratatui::layout::Rect,
    pub rows: ratatui::layout::Rect,
    pub status: ratatui::layout::Rect,
    pub footer: ratatui::layout::Rect,
    pub watches: Vec<(crate::core::WatchId, ratatui::layout::Rect)>,
    pub row_height: u16,
}

#[derive(Clone, Debug)]
pub struct Selection {
    pub kind: EntityKind,
    pub document: serde_json::Value,
    pub self_link: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EntityKey {
    pub kind: EntityKind,
    pub id: String,
}

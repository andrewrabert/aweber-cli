use crate::catalog::Operation;
use crate::catalog::args::{ArgValue, Args};

/// The argument that carries the version an edit was made against.
pub const PRECONDITION_ARG: &str = "precondition-version";

/// A request body together with the encoding its endpoint requires.
#[derive(Clone, Debug, PartialEq)]
pub enum PlanBody {
    Json(serde_json::Value),
    /// `application/x-www-form-urlencoded`, in the order the members were built.
    Form(Vec<(String, String)>),
}

impl PlanBody {
    /// A flat object as form members: a string, number, or boolean member is its
    /// own text, a null is the empty string, and anything else is its compact
    /// JSON text.
    pub fn form(document: &serde_json::Value) -> PlanBody {
        let members = match document {
            serde_json::Value::Object(map) => map
                .iter()
                .map(|(key, value)| (key.clone(), member(value)))
                .collect(),
            _ => Vec::new(),
        };
        PlanBody::Form(members)
    }

    pub fn content_type(&self) -> &'static str {
        match self {
            PlanBody::Json(_) => "application/json",
            PlanBody::Form(_) => "application/x-www-form-urlencoded",
        }
    }

    /// The bytes the request carries.
    pub fn encoded(&self) -> String {
        match self {
            PlanBody::Json(document) => document.to_string(),
            PlanBody::Form(members) => {
                serde_urlencoded::to_string(members).expect("form members are strings")
            }
        }
    }
}

/// The text a form member carries.
fn member(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(text) => text.clone(),
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(_) | serde_json::Value::Number(_) => value.to_string(),
        other => other.to_string(),
    }
}

/// One HTTP request, ready to send.
#[derive(Clone, Debug)]
pub struct RequestPlan {
    pub method: reqwest::Method,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub headers: Vec<(reqwest::header::HeaderName, String)>,
    pub body: Option<PlanBody>,
    pub cursor: CursorStyle,
}

/// How a collection's next page is asked for.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CursorStyle {
    None,
    /// `/next_collection_link` in the body, followed as an absolute URL.
    NextCollectionLink,
    /// The named query parameter of the `Link` header's `rel="next"` entry,
    /// resent as `arg`.
    LinkHeader {
        parameter: &'static str,
        arg: &'static str,
    },
}

/// The account every path is built under.
#[derive(Copy, Clone, Debug)]
pub struct Scope {
    pub account_id: i32,
    /// The uid the internal services name an owner by, when the credentials
    /// carry it.
    pub account: Option<crate::ids::AccountUid>,
}

#[derive(Clone, Debug)]
pub enum PlanError {
    MissingArgument { name: String },
    InvalidValue { name: String, reason: String },
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanError::MissingArgument { name } => write!(f, "{name} is required"),
            PlanError::InvalidValue { name, reason } => write!(f, "{name} is invalid: {reason}"),
        }
    }
}

impl std::error::Error for PlanError {}

impl RequestPlan {
    pub fn absolute(url: &str) -> RequestPlan {
        RequestPlan {
            method: reqwest::Method::GET,
            path: url.to_string(),
            query: Vec::new(),
            headers: Vec::new(),
            body: None,
            cursor: CursorStyle::NextCollectionLink,
        }
    }

    pub fn raw(method: reqwest::Method, path: String, body: Option<PlanBody>) -> RequestPlan {
        RequestPlan {
            method,
            path,
            query: Vec::new(),
            headers: Vec::new(),
            body,
            cursor: CursorStyle::None,
        }
    }

    /// `POST /internal/message/messages/batch/get?fields=subject`
    pub fn message_subjects(messages: &[crate::ids::MessageId]) -> RequestPlan {
        let ids: Vec<serde_json::Value> = messages
            .iter()
            .map(|id| serde_json::Value::String(id.to_string()))
            .collect();
        RequestPlan {
            method: reqwest::Method::POST,
            path: "/internal/message/messages/batch/get".to_string(),
            query: vec![("fields".to_string(), "subject".to_string())],
            headers: Vec::new(),
            body: Some(PlanBody::Json(
                serde_json::json!({ "message_ids": serde_json::Value::Array(ids) }),
            )),
            cursor: CursorStyle::None,
        }
    }

    /// `GET /1.0/accounts/1/lists?ws.size=100`
    pub fn line(&self) -> String {
        let mut line = format!("{} {}", self.method, self.path);
        if !self.query.is_empty() {
            let pairs: Vec<String> = self
                .query
                .iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect();
            line.push('?');
            line.push_str(&pairs.join("&"));
        }
        line
    }
}

/// The builder every arm of `RequestPlan::build` writes into.
struct Builder<'a> {
    operation: Operation,
    scope: &'a Scope,
    args: &'a Args,
    method: reqwest::Method,
    path: String,
    query: Vec<(String, String)>,
    headers: Vec<(reqwest::header::HeaderName, String)>,
    body: Option<serde_json::Value>,
    cursor: CursorStyle,
}

impl<'a> Builder<'a> {
    fn new(operation: Operation, scope: &'a Scope, args: &'a Args) -> Builder<'a> {
        Builder {
            operation,
            scope,
            args,
            method: reqwest::Method::GET,
            path: String::new(),
            query: Vec::new(),
            headers: Vec::new(),
            body: None,
            cursor: CursorStyle::None,
        }
    }

    fn account(&self) -> i32 {
        self.scope.account_id
    }

    fn account_uid(&self) -> Result<crate::ids::AccountUid, PlanError> {
        self.scope.account.ok_or(PlanError::MissingArgument {
            name: "account-uid".to_string(),
        })
    }

    fn required(&self, name: &str) -> Result<&'a ArgValue, PlanError> {
        self.args.first(name).ok_or(PlanError::MissingArgument {
            name: name.to_string(),
        })
    }

    fn text(&self, name: &str) -> Result<&'a str, PlanError> {
        Ok(self.required(name)?.raw.as_str())
    }

    fn method(mut self, method: reqwest::Method) -> Builder<'a> {
        self.method = method;
        self
    }

    fn path(mut self, path: impl Into<String>) -> Builder<'a> {
        self.path = path.into();
        self
    }

    /// `/1.0/accounts/{account_id}` and everything under it.
    fn account_path(self, tail: &str) -> Builder<'a> {
        let account = self.account();
        self.path(format!("/1.0/accounts/{account}{tail}"))
    }

    /// `/1.0/accounts/{account_id}/lists/{list_id}` and everything under it.
    fn list_path(self, tail: &str) -> Result<Builder<'a>, PlanError> {
        let list = self.text("list-id")?.to_string();
        Ok(self.account_path(&format!("/lists/{list}{tail}")))
    }

    fn fixed(mut self, key: &str, value: &str) -> Builder<'a> {
        self.query.push((key.to_string(), value.to_string()));
        self
    }

    /// Send the named arguments as query parameters under their API names.
    fn pass(mut self, names: &[&str]) -> Builder<'a> {
        for name in names {
            for value in self.args.all(name) {
                self.query.push((query_name(name), value.raw.clone()));
            }
        }
        self
    }

    fn require_query(mut self, names: &[&str]) -> Result<Builder<'a>, PlanError> {
        for name in names {
            let value = self.required(name)?.raw.clone();
            self.query.push((query_name(name), value));
        }
        Ok(self)
    }

    /// A body of the named arguments, each under its API name.
    fn body_of(mut self, names: &[&str]) -> Builder<'a> {
        let mut map = serde_json::Map::new();
        for name in names {
            if let Some(value) = self.args.first(name) {
                map.insert(query_name(name), value.json());
            }
        }
        self.body = Some(serde_json::Value::Object(map));
        self
    }

    /// The whole body, entered as JSON through an editor.
    fn json_body(mut self, name: &str) -> Result<Builder<'a>, PlanError> {
        let value = self.required(name)?;
        self.body =
            Some(
                serde_json::from_str(&value.raw).map_err(|e| PlanError::InvalidValue {
                    name: name.to_string(),
                    reason: e.to_string(),
                })?,
            );
        Ok(self)
    }

    fn body(mut self, body: serde_json::Value) -> Builder<'a> {
        self.body = Some(body);
        self
    }

    /// The whole body from `--json-body` when it is present, otherwise the
    /// named arguments.
    fn body_or_json(self, names: &[&str]) -> Result<Builder<'a>, PlanError> {
        if self.args.contains("json-body") {
            self.json_body("json-body")
        } else {
            Ok(self.body_of(names))
        }
    }

    /// One more member of the object body.
    fn with_body(mut self, name: &str, value: serde_json::Value) -> Builder<'a> {
        let mut map = match self.body.take() {
            Some(serde_json::Value::Object(map)) => map,
            _ => serde_json::Map::new(),
        };
        map.insert(name.to_string(), value);
        self.body = Some(serde_json::Value::Object(map));
        self
    }

    /// A JSON Patch replacing the pointer of every argument that is present.
    fn patch_body(mut self, pairs: &[(&str, &str)]) -> Builder<'a> {
        let ops: Vec<serde_json::Value> = pairs
            .iter()
            .filter_map(|(name, pointer)| {
                self.args.first(name).map(|value| {
                    serde_json::json!({
                        "op": "replace",
                        "path": pointer,
                        "value": value.json(),
                    })
                })
            })
            .collect();
        self.body = Some(serde_json::Value::Array(ops));
        self
    }

    /// A JSON Patch replacing the workflow's unpublished ruleset slots from a
    /// draft entered as JSON, actions before events as
    /// `workflows::WorkflowPatch::with_ruleset` writes them.
    fn ruleset_body(self, name: &str) -> Result<Builder<'a>, PlanError> {
        let value = self.required(name)?;
        let draft: serde_json::Value =
            serde_json::from_str(&value.raw).map_err(|error| PlanError::InvalidValue {
                name: name.to_string(),
                reason: error.to_string(),
            })?;
        let slot = |key: &str| {
            draft
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::Value::Array(Vec::new()))
        };
        Ok(self.body(serde_json::json!([
            {
                "op": "replace",
                "path": UNPUBLISHED_ACTIONS,
                "value": slot("actions"),
            },
            {
                "op": "replace",
                "path": UNPUBLISHED_EVENTS,
                "value": slot("events"),
            },
        ])))
    }

    /// The `Authorization` header the OAuth2 endpoints take verbatim.
    fn authorization(mut self) -> Builder<'a> {
        if let Some(value) = self.args.first("authorization") {
            self.headers
                .push((reqwest::header::AUTHORIZATION, value.raw.clone()));
        }
        self
    }

    fn cursor(mut self, cursor: CursorStyle) -> Builder<'a> {
        self.cursor = cursor;
        self
    }

    /// The window pagination every `1.0` collection shares.
    fn window(self) -> Builder<'a> {
        self.pass(&["ws-size", "ws-start"])
            .cursor(CursorStyle::NextCollectionLink)
    }

    /// `If-Match`, from the version the edit was made against.
    fn precondition(mut self) -> Builder<'a> {
        if let Some(version) = self.args.first(PRECONDITION_ARG) {
            self.headers
                .push((reqwest::header::IF_MATCH, version.raw.clone()));
        }
        self
    }

    fn workflow(&self) -> Result<&'a str, PlanError> {
        self.text("workflow")
    }

    fn finish(self) -> RequestPlan {
        debug_assert!(!self.path.is_empty(), "{:?} has no path", self.operation);
        let operation = self.operation;
        RequestPlan {
            method: self.method,
            path: self.path,
            query: self.query,
            headers: self.headers,
            body: self.body.map(|document| {
                if FORM_ENCODED.contains(&operation) {
                    PlanBody::form(&document)
                } else {
                    PlanBody::Json(document)
                }
            }),
            cursor: self.cursor,
        }
    }
}

/// The operations the API takes as a form rather than as JSON.
const FORM_ENCODED: [Operation; 6] = [
    Operation::CreateBroadcast,
    Operation::UpdateBroadcast,
    Operation::ScheduleBroadcast,
    Operation::MoveSubscriber,
    Operation::OauthGetAccessToken,
    Operation::OauthGetRequestToken,
];

/// The API's name for a CLI argument: `ws-size` is `ws.size`, `sort-order` is
/// `sort_order`.
fn query_name(name: &str) -> String {
    match name {
        "ws-size" => "ws.size".to_string(),
        "ws-start" => "ws.start".to_string(),
        "ws-show" => "ws.show".to_string(),
        "page-size" => "page_size".to_string(),
        other => other.replace('-', "_"),
    }
}

const CAMPAIGNS: &str = "/internal/campaign/campaigns";
const REPORTS: &str = "/internal/analytics-view/reports";
/// The JSON pointers of a workflow's unpublished ruleset slots.
const UNPUBLISHED_ACTIONS: &str = "/ruleset/unpublished_actions";
const UNPUBLISHED_EVENTS: &str = "/ruleset/unpublished_events";

impl RequestPlan {
    /// Every `Operation` is matched; identifiers must already be resolved to ids.
    pub fn build(
        operation: Operation,
        scope: &Scope,
        args: &Args,
    ) -> Result<RequestPlan, PlanError> {
        const DELETE: reqwest::Method = reqwest::Method::DELETE;
        const PATCH: reqwest::Method = reqwest::Method::PATCH;
        const POST: reqwest::Method = reqwest::Method::POST;
        const PUT: reqwest::Method = reqwest::Method::PUT;
        let b = Builder::new(operation, scope, args);
        let plan = match operation {
            Operation::ListAccounts => b.path("/1.0/accounts").window(),
            Operation::GetAccount => b.account_path(""),
            Operation::FindAccountSubscribers => b
                .account_path("")
                .fixed("ws.op", "findSubscribers")
                .pass(SUBSCRIBER_FILTERS)
                .window(),
            Operation::ListAccountWebformSplitTests => b
                .account_path("")
                .fixed("ws.op", "getWebFormSplitTests")
                .window(),
            Operation::ListAccountWebforms => {
                b.account_path("").fixed("ws.op", "getWebForms").window()
            }
            Operation::ListIntegrations => b.account_path("/integrations").window(),
            Operation::GetIntegration => {
                let integration = b.text("integration-id")?.to_string();
                b.account_path(&format!("/integrations/{integration}"))
            }
            Operation::ListLists => b.account_path("/lists").window(),
            Operation::FindLists => b
                .account_path("/lists")
                .fixed("ws.op", "find")
                .pass(&["name", "ws-show"])
                .window(),
            Operation::GetList => b.list_path("")?,
            Operation::ListBroadcasts => b.list_path("/broadcasts")?.pass(&["status"]).window(),
            Operation::CreateBroadcast => b
                .method(POST)
                .list_path("/broadcasts")?
                .body_or_json(BROADCAST_FIELDS)?,
            Operation::GetBroadcastTotal => b
                .list_path("/broadcasts/total")?
                .require_query(&["status"])?,
            Operation::GetBroadcast => {
                let broadcast = b.text("broadcast-id")?.to_string();
                b.list_path(&format!("/broadcasts/{broadcast}"))?
            }
            Operation::UpdateBroadcast => {
                let broadcast = b.text("broadcast-id")?.to_string();
                b.method(PUT)
                    .list_path(&format!("/broadcasts/{broadcast}"))?
                    .body_or_json(BROADCAST_FIELDS)?
            }
            Operation::DeleteBroadcast => {
                let broadcast = b.text("broadcast-id")?.to_string();
                b.method(DELETE)
                    .list_path(&format!("/broadcasts/{broadcast}"))?
            }
            Operation::CancelBroadcast => {
                let broadcast = b.text("broadcast-id")?.to_string();
                b.method(POST)
                    .list_path(&format!("/broadcasts/{broadcast}/cancel"))?
            }
            Operation::GetBroadcastClicks => {
                let broadcast = b.text("broadcast-id")?.to_string();
                b.list_path(&format!("/broadcasts/{broadcast}/clicks"))?
                    .pass(&["after", "before", "detailed", "page-size"])
                    .cursor(CursorStyle::LinkHeader {
                        parameter: "after",
                        arg: "after",
                    })
            }
            Operation::GetBroadcastOpens => {
                let broadcast = b.text("broadcast-id")?.to_string();
                b.list_path(&format!("/broadcasts/{broadcast}/opens"))?
                    .pass(&["after", "before", "page-size"])
                    .cursor(CursorStyle::LinkHeader {
                        parameter: "after",
                        arg: "after",
                    })
            }
            Operation::ScheduleBroadcast => {
                let broadcast = b.text("broadcast-id")?.to_string();
                b.method(POST)
                    .list_path(&format!("/broadcasts/{broadcast}/schedule"))?
                    .body_or_json(&["scheduled-for"])?
            }
            Operation::WaitBroadcast => {
                let broadcast = b.text("broadcast-id")?.to_string();
                b.list_path(&format!("/broadcasts/{broadcast}"))?
            }
            Operation::ListCampaigns => b.list_path("/campaigns")?.window(),
            Operation::ListCampaignStats => {
                let campaign = b.text("campaign-id")?.to_string();
                b.list_path(&format!("/campaigns/b{campaign}/stats"))?
                    .window()
            }
            Operation::GetCampaignStat => {
                let campaign = b.text("campaign-id")?.to_string();
                let stat = b.text("stats-id")?.to_string();
                b.list_path(&format!("/campaigns/b{campaign}/stats/{stat}"))?
            }
            Operation::FindCampaigns => b
                .list_path("/campaigns")?
                .fixed("ws.op", "find")
                .require_query(&["campaign-type"])?
                .pass(&["ws-show"])
                .window(),
            Operation::GetCampaign => {
                let kind = b.text("campaign-type")?.to_string();
                let campaign = b.text("campaign-id")?.to_string();
                b.list_path(&format!("/campaigns/{kind}{campaign}"))?
            }
            Operation::ListCustomFields => b.list_path("/custom_fields")?.window(),
            Operation::CreateCustomField => b
                .method(POST)
                .list_path("/custom_fields")?
                .body_or_json(&["name"])?
                .with_body("ws.op", serde_json::Value::String("create".to_string())),
            Operation::GetCustomField => {
                let field = b.text("custom-field-id")?.to_string();
                b.list_path(&format!("/custom_fields/{field}"))?
            }
            Operation::DeleteCustomField => {
                let field = b.text("custom-field-id")?.to_string();
                b.method(DELETE)
                    .list_path(&format!("/custom_fields/{field}"))?
            }
            Operation::UpdateCustomField => {
                let field = b.text("custom-field-id")?.to_string();
                b.method(PATCH)
                    .list_path(&format!("/custom_fields/{field}"))?
                    .body_or_json(&["name", "is-subscriber-updateable"])?
            }
            Operation::ListLandingPages => b.list_path("/landing_pages")?.window(),
            Operation::GetLandingPage => {
                let page = b.text("landing-page-id")?.to_string();
                b.list_path(&format!("/landing_pages/{page}"))?
            }
            Operation::CreatePurchase => b
                .method(POST)
                .list_path("/purchases")?
                .body_or_json(PURCHASE_FIELDS)?,
            Operation::ListSegments => b.list_path("/segments")?.window(),
            Operation::GetSegment => {
                let segment = b.text("segment-id")?.to_string();
                b.list_path(&format!("/segments/{segment}"))?
            }
            Operation::ListSubscribers => {
                b.list_path("/subscribers")?.pass(&["sort-order"]).window()
            }
            Operation::CreateSubscriber => b
                .method(POST)
                .list_path("/subscribers")?
                .body_or_json(NEW_SUBSCRIBER_FIELDS)?,
            Operation::DeleteSubscriberByEmail => {
                let email = b.text("email")?.to_string();
                b.method(DELETE)
                    .list_path("/subscribers")?
                    .fixed("subscriber_email", &email)
            }
            Operation::UpdateSubscriberByEmail => {
                let email = b.text("email")?.to_string();
                b.method(PATCH)
                    .list_path("/subscribers")?
                    .fixed("subscriber_email", &email)
                    .body_or_json(SUBSCRIBER_EDITS)?
            }
            Operation::FindSubscribers => b
                .list_path("/subscribers")?
                .fixed("ws.op", "find")
                .pass(SUBSCRIBER_FILTERS)
                .pass(&["sort-key", "sort-order"])
                .window(),
            Operation::GetSubscriber => {
                let subscriber = b.text("subscriber-id")?.to_string();
                b.list_path(&format!("/subscribers/{subscriber}"))?
            }
            Operation::MoveSubscriber => {
                let subscriber = b.text("subscriber-id")?.to_string();
                b.method(POST)
                    .list_path(&format!("/subscribers/{subscriber}"))?
                    .body_or_json(MOVE_FIELDS)?
                    .with_body("ws.op", serde_json::Value::String("move".to_string()))
            }
            Operation::DeleteSubscriber => {
                let subscriber = b.text("subscriber-id")?.to_string();
                b.method(DELETE)
                    .list_path(&format!("/subscribers/{subscriber}"))?
            }
            Operation::UpdateSubscriber => {
                let subscriber = b.text("subscriber-id")?.to_string();
                b.method(PATCH)
                    .list_path(&format!("/subscribers/{subscriber}"))?
                    .body_or_json(SUBSCRIBER_EDITS)?
            }
            Operation::UnsubscribeSubscriber => {
                let subscriber = b.text("subscriber-id")?.to_string();
                b.method(PATCH)
                    .list_path(&format!("/subscribers/{subscriber}"))?
                    .body(serde_json::json!({ "status": "unsubscribed" }))
            }
            Operation::GetSubscriberActivity => {
                let subscriber = b.text("subscriber-id")?.to_string();
                b.list_path(&format!("/subscribers/{subscriber}"))?
                    .fixed("ws.op", "getActivity")
                    .window()
            }
            Operation::ListTags => b.list_path("/tags")?,
            Operation::ListWebFormSplitTests => b.list_path("/web_form_split_tests")?.window(),
            Operation::GetWebFormSplitTest => {
                let test = b.text("split-test-id")?.to_string();
                b.list_path(&format!("/web_form_split_tests/{test}"))?
            }
            Operation::ListWebFormSplitTestComponents => {
                let test = b.text("split-test-id")?.to_string();
                b.list_path(&format!("/web_form_split_tests/{test}/components"))?
                    .window()
            }
            Operation::GetWebFormSplitTestComponent => {
                let test = b.text("split-test-id")?.to_string();
                let component = b.text("split-test-component-id")?.to_string();
                b.list_path(&format!(
                    "/web_form_split_tests/{test}/components/{component}"
                ))?
            }
            Operation::ListWebForms => b.list_path("/web_forms")?.window(),
            Operation::GetWebForm => {
                let form = b.text("webform-id")?.to_string();
                b.list_path(&format!("/web_forms/{form}"))?
            }
            Operation::GetBroadcastLinkAnalytics => {
                let account = b.account_uid()?;
                b.path("/2.0-beta/analytics/reports/broadcasts-links")
                    .fixed("account_id", &account.to_string())
                    .require_query(&["broadcast-id", "filter"])?
                    .pass(&[
                        "after",
                        "before",
                        "max-count",
                        "min-count",
                        "page-size",
                        "sort-asc",
                        "sort-by",
                    ])
                    .cursor(CursorStyle::LinkHeader {
                        parameter: "after",
                        arg: "after",
                    })
            }
            Operation::ListWorkflows => {
                let account = b.account_uid()?;
                let list = b.text("list")?.to_string();
                b.path(CAMPAIGNS)
                    .fixed("owner", &account.to_string())
                    .fixed("parent", &list)
            }
            Operation::GetWorkflow | Operation::TreeWorkflow => {
                let workflow = b.workflow()?.to_string();
                b.path(format!("{CAMPAIGNS}/{workflow}"))
                    .fixed("include", "ruleset")
            }
            Operation::CreateWorkflow => {
                let account = b.account_uid()?;
                let list = b.text("list")?.to_string();
                b.method(POST)
                    .path(CAMPAIGNS)
                    .body_of(&["name", "timezone"])
                    .with_body("owner", serde_json::Value::String(account.to_string()))
                    .with_body("parent", serde_json::Value::String(list))
            }
            Operation::UpdateWorkflow => {
                let workflow = b.workflow()?.to_string();
                let times = b.args.contains("timezone");
                let mut plan = b
                    .method(PATCH)
                    .path(format!("{CAMPAIGNS}/{workflow}"))
                    .fixed("include", "ruleset")
                    .precondition();
                if times {
                    plan = plan.fixed("update_times", "true");
                }
                if plan.args.contains("patch") {
                    plan.json_body("patch")?
                } else {
                    plan.patch_body(&[
                        ("name", "/name"),
                        ("timezone", "/timezone"),
                        ("sharing-enabled", "/sharing_enabled"),
                        ("auto-extend-enabled", "/auto_extend_enabled"),
                    ])
                }
            }
            Operation::UpdateWorkflowRuleset => {
                let workflow = b.workflow()?.to_string();
                b.method(PATCH)
                    .path(format!("{CAMPAIGNS}/{workflow}"))
                    .fixed("include", "ruleset")
                    .precondition()
                    .ruleset_body("file")?
            }
            Operation::PublishWorkflow => {
                let workflow = b.workflow()?.to_string();
                b.method(POST)
                    .path(format!("{CAMPAIGNS}/{workflow}/publish"))
                    .precondition()
            }
            Operation::RevertWorkflow => {
                let workflow = b.workflow()?.to_string();
                b.method(POST)
                    .path(format!("{CAMPAIGNS}/{workflow}/revert"))
                    .precondition()
            }
            Operation::SetWorkflowState => {
                let workflow = b.workflow()?.to_string();
                b.method(PATCH)
                    .path(format!("{CAMPAIGNS}/{workflow}"))
                    .fixed("include", "ruleset")
                    .precondition()
                    .patch_body(&[("state", "/state")])
            }
            Operation::CopyWorkflow => {
                let workflow = b.workflow()?.to_string();
                let list = b.text("list")?.to_string();
                b.method(POST)
                    .path(format!("{CAMPAIGNS}/{workflow}/copy"))
                    .precondition()
                    .body_of(&["name", "timezone"])
                    .with_body("list", serde_json::Value::String(list))
            }
            Operation::DeleteWorkflow => {
                let workflow = b.workflow()?.to_string();
                b.method(DELETE)
                    .path(format!("{CAMPAIGNS}/{workflow}"))
                    .precondition()
            }
            Operation::GetWorkflowStats => {
                let workflow = b.workflow()?.to_string();
                let recurring = b
                    .args
                    .first("recurring")
                    .map(|value| value.raw.clone())
                    .unwrap_or_else(|| "false".to_string());
                b.path(format!("{REPORTS}/campaign-message-totals/{workflow}"))
                    .fixed("recurring", &recurring)
            }
            Operation::GetWorkflowMessageStats => {
                let account = b.account_uid()?;
                let message = b.text("message-id")?.to_string();
                let recurring = b
                    .args
                    .first("recurring")
                    .map(|value| value.raw.clone())
                    .unwrap_or_else(|| "false".to_string());
                let mut plan = b
                    .path(format!("{REPORTS}/campaign-message-click-stats/{message}"))
                    .fixed("account_id", &account.to_string())
                    .fixed("recurring", &recurring)
                    .pass(&["page-size"])
                    .cursor(CursorStyle::LinkHeader {
                        parameter: "after",
                        arg: "links-cursor",
                    });
                if let Some(cursor) = plan.args.first("links-cursor") {
                    let cursor = cursor.raw.clone();
                    plan = plan.fixed("after", &cursor);
                }
                plan
            }
            Operation::GetWorkflowEventHistory => {
                let workflow = b.workflow()?.to_string();
                let event = b.text("event-id")?.to_string();
                let size = b
                    .args
                    .first("page-size")
                    .map(|value| value.raw.clone())
                    .unwrap_or_else(|| "100".to_string());
                let mut plan = b
                    .path(format!(
                        "{REPORTS}/recurring-events/{workflow}/events/{event}"
                    ))
                    .fixed("page-size", &size)
                    .cursor(CursorStyle::LinkHeader {
                        parameter: "start-token",
                        arg: "start-token",
                    });
                if let Some(token) = plan.args.first("start-token") {
                    let token = token.raw.clone();
                    plan = plan.fixed("start-token", &token);
                }
                plan
            }
            Operation::OauthGetAccessToken => b
                .method(POST)
                .path("/oauth/access_token")
                .body_or_json(OAUTH_FIELDS)?,
            Operation::OauthGetRequestToken => b
                .method(POST)
                .path("/oauth/request_token")
                .body_or_json(OAUTH_FIELDS)?,
            Operation::OauthRevoke => b
                .method(POST)
                .path("/oauth2/revoke")
                .authorization()
                .body_or_json(&[])?,
            Operation::OauthToken => b
                .method(POST)
                .path("/oauth2/token")
                .authorization()
                .body_or_json(&[])?,
        };
        Ok(plan.finish())
    }
}

const SUBSCRIBER_FILTERS: &[&str] = &[
    "ad-tracking",
    "area-code",
    "city",
    "country",
    "custom-fields",
    "dma-code",
    "email",
    "last-followup-message-number-sent",
    "last-followup-message-sent-at",
    "latitude",
    "longitude",
    "misc-notes",
    "name",
    "postal-code",
    "region",
    "status",
    "subscribed-after",
    "subscribed-at",
    "subscribed-before",
    "subscription-method",
    "tags",
    "tags-not-in",
    "unsubscribe-method",
    "unsubscribed-after",
    "unsubscribed-at",
    "unsubscribed-before",
    "verified-at",
    "ws-show",
];

const BROADCAST_FIELDS: &[&str] = &[
    "body-amp",
    "body-html",
    "body-text",
    "click-tracking-enabled",
    "exclude-lists",
    "facebook-integration",
    "include-lists",
    "is-archived",
    "notify-on-send",
    "segment-link",
    "subject",
    "twitter-integration",
];

const PURCHASE_FIELDS: &[&str] = &[
    "ad-tracking",
    "currency",
    "email",
    "event-note",
    "event-time",
    "ip-address",
    "misc-notes",
    "name",
    "product-name",
    "url",
    "value",
    "vendor",
];

const NEW_SUBSCRIBER_FIELDS: &[&str] = &[
    "ad-tracking",
    "email",
    "ip-address",
    "last-followup-message-number-sent",
    "misc-notes",
    "name",
    "strict-custom-fields",
    "update-existing",
];

const SUBSCRIBER_EDITS: &[&str] = &[
    "ad-tracking",
    "last-followup-message-number-sent",
    "misc-notes",
    "name",
    "status",
    "strict-custom-fields",
];

const MOVE_FIELDS: &[&str] = &[
    "enforce-custom-field-mapping",
    "last-followup-message-number-sent",
    "list-link",
];

const OAUTH_FIELDS: &[&str] = &[
    "oauth-callback",
    "oauth-consumer-key",
    "oauth-nonce",
    "oauth-signature",
    "oauth-signature-method",
    "oauth-timestamp",
    "oauth-token",
    "oauth-version",
];

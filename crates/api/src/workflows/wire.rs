use crate::ids::{BranchId, MessageId, RuleId};
use crate::workflows::graph::{
    Branch, Condition, ConditionField, ConditionTest, OwnedRule, Slot, Starter, Step, StepKind,
    Tested,
};
use crate::workflows::values::{
    FeedUrl, LinkMatch, Recurrence, SendCadence, Tag, Timezone, TimezoneSource, WaitTiming,
};

const EVENT_ACCOUNT: &str = "<event:account>";
const EVENT_LIST: &str = "<event:list>";
const EVENT_RECIPIENT: &str = "<event:recipient>";
const EVENT_SUBSCRIBER: &str = "<event:subscriber>";
const EVENT_MESSAGE: &str = "<event:message>";
const EVENT_REMOTE_IP: &str = "<event:remote_ip>";
const SUBSCRIBER: &str = "<subscriber>";
const RULESET: &str = "<ruleset>";
const ACTION: &str = "<action>";
const MESSAGES_SENT: &str = "<messages:sent>";

const TYPE_SUBSCRIBE: &str = "subscribe.v1";
const TYPE_TAG: &str = "tag.v1";
const TYPE_WAIT_COMPLETE: &str = "wait_complete.v1";
const TYPE_OPEN: &str = "open.v2";
const TYPE_CLICK: &str = "click.v2";

const MODE_ALL: &str = "all";
const MODE_ANY: &str = "any";
const EQUALS: &str = "==";
const DIFFERS: &str = "!=";

const MINIMUM_NEW_ITEMS: u64 = 1;

#[derive(Clone, Debug)]
pub(super) struct PreservedRule {
    arrived: serde_json::Value,
    rule: Rule,
}

impl PreservedRule {
    pub(super) fn rule(&self) -> &Rule {
        &self.rule
    }

    fn from_value(arrived: serde_json::Value) -> Result<PreservedRule, ReadError> {
        let rule = read(&arrived)?;
        Ok(PreservedRule { arrived, rule })
    }

    fn emitted(value: serde_json::Value) -> PreservedRule {
        PreservedRule::from_value(value).expect("an emitted rule reads back")
    }
}

impl PartialEq for PreservedRule {
    fn eq(&self, other: &PreservedRule) -> bool {
        self.arrived == other.arrived
    }
}

impl Eq for PreservedRule {}

impl serde::Serialize for PreservedRule {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.arrived.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for PreservedRule {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<PreservedRule, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        PreservedRule::from_value(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug)]
pub(super) struct ReadError(String);

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ReadError {}

#[derive(Clone, Debug)]
pub(super) struct Rule {
    pub(super) id: RuleId,
    pub(super) parents: Vec<RuleId>,
    pub(super) branch: Option<BranchId>,
    pub(super) recurring: bool,
    pub(super) body: Body,
}

#[derive(Clone, Debug)]
pub(super) enum Body {
    Event(Event),
    Action(Action),
}

#[derive(Clone, Debug)]
pub(super) enum Event {
    Subscribe {
        conditions: Vec<Condition>,
    },
    Tagged {
        tags: Vec<Tag>,
        except: Vec<Tag>,
        reentry: bool,
    },
    WaitComplete {
        correlation: String,
    },
    CheckFeedComplete {
        correlation: String,
        url: Option<FeedUrl>,
    },
    Opened,
    Clicked {
        links: Vec<LinkMatch>,
    },
    Reserved,
}

#[derive(Clone, Debug)]
pub(super) enum Action {
    Send {
        message: Option<MessageId>,
    },
    Wait {
        correlation: String,
        timing: WaitTiming,
        timezone_source: TimezoneSource,
        deleted: bool,
    },
    CheckFeed {
        correlation: String,
        check_every: Recurrence,
    },
    Tag {
        applied: Vec<Tag>,
        removed: Vec<Tag>,
    },
    SetBranch {
        tested: Tested,
        branches: Vec<RuleBranch>,
    },
    Stop,
}

#[derive(Clone, Debug)]
pub(super) struct RuleBranch {
    pub(super) id: Option<BranchId>,
    pub(super) affirms: Option<bool>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FunctionName(&'static str);

impl FunctionName {
    const COMPOSE: FunctionName = FunctionName("ruleset.email.action.compose_v1");
    const WAIT: FunctionName = FunctionName("ruleset.schedule.action.wait_v1");
    const MODIFY_TAGS: FunctionName = FunctionName("ruleset.tag.action.modify_tags_v1");
    const SET_BRANCH: FunctionName = FunctionName("rulesengine.action.set_branch");
    const SET_BRANCH_V1: FunctionName = FunctionName("ruleset.schedule.action.set_branch_v1");
    const STOP: FunctionName = FunctionName("rulesengine.action.stop");
    const EVENT_VALUE: FunctionName = FunctionName("rulesengine.filter.event_value");
    const EVENT_VALUE_IN: FunctionName = FunctionName("rulesengine.filter.event_value_in");
    const EVENT_VALUE_IN_URLS: FunctionName =
        FunctionName("rulesengine.filter.event_value_in_urls");
    const REENTRY_ALLOWED: FunctionName = FunctionName("rulesengine.filter.reentry_allowed");
    const ANY_TAGS: FunctionName = FunctionName("ruleset.tag.filter.any_tags_v1");
    const FEED_CHANGED: FunctionName = FunctionName("ruleset.rss.filter.state_changed_v1");
    const ANY_OPENS: FunctionName = FunctionName("ruleset.analytics.filter.any_message_opens_v1");
    const ANY_CLICKS: FunctionName = FunctionName("ruleset.analytics.filter.any_message_clicks_v1");
    const ANY_CLICK_URLS: FunctionName =
        FunctionName("ruleset.analytics.filter.any_message_click_urls_v1");
    const ANY_CLICK_URL_CONTAINS: FunctionName =
        FunctionName("ruleset.analytics.filter.any_message_click_url_contains_v1");
    const MESSAGEAPI_ID: FunctionName = FunctionName("ruleset.messagemap.filter.messageapi_id_v1");
    const GEOIP_COUNTRY: FunctionName = FunctionName("ruleset.geoip.filter.geoip_country_v1");
    const ADTRACKING: FunctionName = FunctionName("ruleset.recipient.filter.adtracking_v1");
    const CUSTOM_FIELD: FunctionName = FunctionName("ruleset.recipient.filter.custom_field_v1");

    fn named(text: &str, function: FunctionName) -> bool {
        text == function.0
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Dispatch {
    Subscribe,
    TagEvent,
    WaitComplete,
    CheckFeedComplete,
    Open,
    Click,
    MessageNotOpened,
    LinkNotClicked,
    SendMessage,
    SetBranch,
    Tag,
    Wait,
    CheckFeed,
    Stop,
}

const DISPATCHES: [Dispatch; 14] = [
    Dispatch::Subscribe,
    Dispatch::TagEvent,
    Dispatch::WaitComplete,
    Dispatch::CheckFeedComplete,
    Dispatch::Open,
    Dispatch::Click,
    Dispatch::MessageNotOpened,
    Dispatch::LinkNotClicked,
    Dispatch::SendMessage,
    Dispatch::SetBranch,
    Dispatch::Tag,
    Dispatch::Wait,
    Dispatch::CheckFeed,
    Dispatch::Stop,
];

impl Dispatch {
    fn word(self) -> &'static str {
        match self {
            Dispatch::Subscribe => "subscribe.v1",
            Dispatch::TagEvent => "tag.v1",
            Dispatch::WaitComplete => "wait_complete.v1",
            Dispatch::CheckFeedComplete => "check-feed-complete",
            Dispatch::Open => "open.v2",
            Dispatch::Click => "click.v2",
            Dispatch::MessageNotOpened => "message_not_opened.v1",
            Dispatch::LinkNotClicked => "link_not_clicked.v1",
            Dispatch::SendMessage => "send-message",
            Dispatch::SetBranch => "set-branch",
            Dispatch::Tag => "tag",
            Dispatch::Wait => "wait",
            Dispatch::CheckFeed => "check-feed",
            Dispatch::Stop => "stop",
        }
    }

    fn parse(text: &str) -> Option<Dispatch> {
        DISPATCHES
            .into_iter()
            .find(|dispatch| dispatch.word() == text)
    }
}

#[derive(Clone, Debug)]
pub(super) struct Position {
    pub(super) parents: Vec<RuleId>,
    pub(super) branch: Option<BranchId>,
    pub(super) cadence: SendCadence,
}

type Object = serde_json::Map<String, serde_json::Value>;

struct Criterion<'a> {
    function: &'a str,
    operator: &'a str,
    expect: &'a serde_json::Value,
    kwargs: Object,
}

fn read(value: &serde_json::Value) -> Result<Rule, ReadError> {
    let object = value
        .as_object()
        .ok_or_else(|| ReadError("a rule is not an object".to_string()))?;
    let id: RuleId = object
        .get("id")
        .and_then(serde_json::Value::as_str)
        .and_then(|text| text.parse().ok())
        .ok_or_else(|| ReadError("a rule carries no rule id".to_string()))?;
    let fail = |what: String| ReadError(format!("rule {id}: {what}"));
    let mut parents = Vec::new();
    for entry in listed(object, "parents") {
        let parent: RuleId = entry
            .as_str()
            .and_then(|text| text.parse().ok())
            .ok_or_else(|| fail("a parent is not a rule id".to_string()))?;
        parents.push(parent);
    }
    let branch = match object.get("branch") {
        None | Some(serde_json::Value::Null) => None,
        Some(value) => Some(
            value
                .as_str()
                .and_then(|text| text.parse().ok())
                .ok_or_else(|| fail("branch is not a branch id".to_string()))?,
        ),
    };
    let recurring = object
        .get("recurring")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let word = match object.get("metadata").and_then(serde_json::Value::as_str) {
        Some(word) if !word.is_empty() => word,
        _ => object
            .get("type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| fail("carries neither metadata nor type".to_string()))?,
    };
    let dispatch = Dispatch::parse(word)
        .ok_or_else(|| fail(format!("'{word}' is not a rule the builder writes")))?;
    let body = read_body(object, dispatch).map_err(|error| fail(error.0))?;
    Ok(Rule {
        id,
        parents,
        branch,
        recurring,
        body,
    })
}

fn read_body(object: &Object, dispatch: Dispatch) -> Result<Body, ReadError> {
    Ok(match dispatch {
        Dispatch::Subscribe => Body::Event(Event::Subscribe {
            conditions: read_conditions(&criteria(object)?)?,
        }),
        Dispatch::TagEvent => Body::Event(read_tagged(&criteria(object)?)?),
        Dispatch::WaitComplete => Body::Event(Event::WaitComplete {
            correlation: read_correlation(&criteria(object)?)?,
        }),
        Dispatch::CheckFeedComplete => {
            let criteria = criteria(object)?;
            Body::Event(Event::CheckFeedComplete {
                correlation: read_correlation(&criteria)?,
                url: read_feed_url(&criteria)?,
            })
        }
        Dispatch::Open => Body::Event(Event::Opened),
        Dispatch::Click => Body::Event(Event::Clicked {
            links: read_links(&criteria(object)?)?,
        }),
        Dispatch::MessageNotOpened | Dispatch::LinkNotClicked => Body::Event(Event::Reserved),
        Dispatch::SendMessage => {
            let kwargs = kwargs_named(object, &[FunctionName::COMPOSE])?;
            let message = match kwargs.get("meapi_id") {
                None | Some(serde_json::Value::Null) => None,
                Some(value) => Some(
                    value
                        .as_str()
                        .and_then(|text| text.parse().ok())
                        .ok_or_else(|| ReadError("meapi_id is not a message id".to_string()))?,
                ),
            };
            Body::Action(Action::Send { message })
        }
        Dispatch::Wait => {
            let kwargs = kwargs_named(object, &[FunctionName::WAIT])?;
            Body::Action(Action::Wait {
                correlation: text(&kwargs, "id")?,
                timing: read_timing(&kwargs)?,
                timezone_source: match kwargs.get("use_subscriber_timezone") {
                    None | Some(serde_json::Value::Null) => TimezoneSource::Workflow,
                    Some(value) => serde_json::from_value(value.clone())
                        .map_err(|error| ReadError(error.to_string()))?,
                },
                deleted: object
                    .get("is_deleted")
                    .and_then(serde_json::Value::as_bool)
                    .unwrap_or(false),
            })
        }
        Dispatch::CheckFeed => {
            let kwargs = kwargs_named(object, &[FunctionName::WAIT])?;
            let rule = listed(&kwargs, "rrules")
                .into_iter()
                .next()
                .ok_or_else(|| ReadError("a check-feed carries no recurrence rule".to_string()))?;
            Body::Action(Action::CheckFeed {
                correlation: text(&kwargs, "id")?,
                check_every: serde_json::from_value(rule)
                    .map_err(|error| ReadError(error.to_string()))?,
            })
        }
        Dispatch::Tag => {
            let kwargs = kwargs_named(object, &[FunctionName::MODIFY_TAGS])?;
            Body::Action(Action::Tag {
                applied: read_tags(&kwargs, "add_labels")?,
                removed: read_tags(&kwargs, "remove_labels")?,
            })
        }
        Dispatch::SetBranch => {
            let kwargs = kwargs_named(
                object,
                &[FunctionName::SET_BRANCH, FunctionName::SET_BRANCH_V1],
            )?;
            Body::Action(read_set_branch(&kwargs)?)
        }
        Dispatch::Stop => {
            kwargs_named(object, &[FunctionName::STOP])?;
            Body::Action(Action::Stop)
        }
    })
}

fn criteria(object: &Object) -> Result<Vec<Criterion<'_>>, ReadError> {
    let filter = match object.get("filter") {
        None | Some(serde_json::Value::Null) => return Ok(Vec::new()),
        Some(filter) => filter
            .as_object()
            .ok_or_else(|| ReadError("filter is not an object".to_string()))?,
    };
    read_criteria(filter)
}

fn read_criteria(filter: &Object) -> Result<Vec<Criterion<'_>>, ReadError> {
    let mut found = Vec::new();
    for entry in listed_ref(filter, "criteria") {
        let object = entry
            .as_object()
            .ok_or_else(|| ReadError("a criterion is not an object".to_string()))?;
        found.push(Criterion {
            function: object
                .get("function")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| ReadError("a criterion names no function".to_string()))?,
            operator: object
                .get("operator")
                .and_then(serde_json::Value::as_str)
                .unwrap_or(EQUALS),
            expect: object.get("expect").unwrap_or(&serde_json::Value::Null),
            kwargs: object
                .get("kwargs")
                .and_then(serde_json::Value::as_object)
                .cloned()
                .unwrap_or_default(),
        });
    }
    Ok(found)
}

fn read_conditions(criteria: &[Criterion<'_>]) -> Result<Vec<Condition>, ReadError> {
    let mut conditions = Vec::new();
    for criterion in criteria {
        let field = if FunctionName::named(criterion.function, FunctionName::GEOIP_COUNTRY) {
            ConditionField::Country
        } else if FunctionName::named(criterion.function, FunctionName::ADTRACKING) {
            ConditionField::AdTracking
        } else if FunctionName::named(criterion.function, FunctionName::EVENT_VALUE) {
            match text(&criterion.kwargs, "key")?.as_str() {
                "source" => ConditionField::Source,
                key => {
                    return Err(ReadError(format!(
                        "'{key}' is not a subscribe condition the builder writes"
                    )));
                }
            }
        } else if FunctionName::named(criterion.function, FunctionName::CUSTOM_FIELD) {
            ConditionField::Custom {
                name: text(&criterion.kwargs, "field")?,
            }
        } else {
            return Err(ReadError(format!(
                "'{}' is not a subscribe condition the builder writes",
                criterion.function
            )));
        };
        let value = match criterion.expect {
            serde_json::Value::Null => None,
            serde_json::Value::String(text) => Some(text.clone()),
            other => Some(other.to_string()),
        };
        let test = match (criterion.operator, value) {
            (EQUALS, Some(value)) => ConditionTest::Is { value },
            (DIFFERS, Some(value)) => ConditionTest::IsNot { value },
            (EQUALS, None) => ConditionTest::Undefined,
            (DIFFERS, None) => ConditionTest::Defined,
            (operator, _) => {
                return Err(ReadError(format!(
                    "'{operator}' is not a subscribe condition operator the builder writes"
                )));
            }
        };
        conditions.push(Condition { field, test });
    }
    Ok(conditions)
}

fn read_tagged(criteria: &[Criterion<'_>]) -> Result<Event, ReadError> {
    let mut tags: Option<Vec<Tag>> = None;
    let mut labelled: Option<Vec<Tag>> = None;
    let mut except = Vec::new();
    let mut reentry = false;
    for criterion in criteria {
        if FunctionName::named(criterion.function, FunctionName::EVENT_VALUE_IN) {
            labelled = Some(read_tags(&criterion.kwargs, "values")?);
        } else if FunctionName::named(criterion.function, FunctionName::ANY_TAGS) {
            let listed = read_tags(&criterion.kwargs, "labels")?;
            if criterion.expect.as_bool() == Some(false) {
                except = listed;
            } else {
                tags = Some(listed);
            }
        } else if FunctionName::named(criterion.function, FunctionName::REENTRY_ALLOWED) {
            reentry = true;
        } else {
            return Err(ReadError(format!(
                "'{}' is not a tag event criterion the builder writes",
                criterion.function
            )));
        }
    }
    Ok(Event::Tagged {
        tags: tags.or(labelled).unwrap_or_default(),
        except,
        reentry,
    })
}

fn read_correlation(criteria: &[Criterion<'_>]) -> Result<String, ReadError> {
    criteria
        .iter()
        .find(|criterion| {
            FunctionName::named(criterion.function, FunctionName::EVENT_VALUE)
                && criterion
                    .kwargs
                    .get("key")
                    .and_then(serde_json::Value::as_str)
                    == Some("id")
        })
        .and_then(|criterion| criterion.expect.as_str())
        .map(str::to_string)
        .ok_or_else(|| ReadError("a completion event names no wait id".to_string()))
}

fn read_feed_url(criteria: &[Criterion<'_>]) -> Result<Option<FeedUrl>, ReadError> {
    let changed = criteria
        .iter()
        .find(|criterion| FunctionName::named(criterion.function, FunctionName::FEED_CHANGED))
        .ok_or_else(|| ReadError("a check-feed-complete names no feed".to_string()))?;
    let url = text(&changed.kwargs, "url")?;
    if url.is_empty() {
        return Ok(None);
    }
    url.parse()
        .map(Some)
        .map_err(|error: crate::workflows::values::FeedUrlError| ReadError(error.to_string()))
}

fn read_links(criteria: &[Criterion<'_>]) -> Result<Vec<LinkMatch>, ReadError> {
    let mut links = Vec::new();
    for criterion in criteria {
        if !FunctionName::named(criterion.function, FunctionName::EVENT_VALUE_IN_URLS) {
            continue;
        }
        for entry in listed(&criterion.kwargs, "urls") {
            let url = entry
                .as_str()
                .and_then(|text| text.parse().ok())
                .ok_or_else(|| ReadError("a click rule's url is not a link".to_string()))?;
            links.push(LinkMatch::Url(url));
        }
    }
    Ok(links)
}

const ZERO_DURATION: &str = "PT0S";

fn read_timing(kwargs: &Object) -> Result<WaitTiming, ReadError> {
    let duration = kwargs
        .get("delay")
        .cloned()
        .ok_or_else(|| ReadError("delay is not text".to_string()))?;
    let delay = if zero_duration(&duration) {
        None
    } else {
        Some(serde_json::from_value(duration).map_err(|error| ReadError(error.to_string()))?)
    };
    let mut schedules = Vec::new();
    for rule in listed(kwargs, "rrules") {
        let schedule: Recurrence =
            serde_json::from_value(rule).map_err(|error| ReadError(error.to_string()))?;
        schedules.push(schedule);
    }
    Ok(WaitTiming { delay, schedules })
}

fn zero_duration(duration: &serde_json::Value) -> bool {
    duration.as_str().is_some_and(|text| {
        text.starts_with('P')
            && text
                .chars()
                .filter(char::is_ascii_digit)
                .all(|digit| digit == '0')
            && text.chars().any(|character| character.is_ascii_digit())
    })
}

fn read_set_branch(kwargs: &Object) -> Result<Action, ReadError> {
    let mut branches = Vec::new();
    let mut tested = None;
    for declared in listed(kwargs, "branches") {
        let Some(object) = declared.as_object() else {
            branches.push(RuleBranch {
                id: None,
                affirms: None,
            });
            continue;
        };
        let id = object
            .get("id")
            .and_then(serde_json::Value::as_str)
            .and_then(|text| text.parse().ok());
        let criteria = read_criteria(object)?;
        let first = criteria.first();
        if tested.is_none()
            && let Some(criterion) = first
        {
            tested = Some(read_tested(criterion)?);
        }
        branches.push(RuleBranch {
            id,
            affirms: first.and_then(|criterion| criterion.expect.as_bool()),
        });
    }
    let tested = tested.ok_or_else(|| ReadError("a split declares no test".to_string()))?;
    Ok(Action::SetBranch { tested, branches })
}

fn read_tested(criterion: &Criterion<'_>) -> Result<Tested, ReadError> {
    if FunctionName::named(criterion.function, FunctionName::ANY_TAGS) {
        return Ok(Tested::HasTag {
            tags: read_tags(&criterion.kwargs, "labels")?,
        });
    }
    if FunctionName::named(criterion.function, FunctionName::ANY_OPENS) {
        return Ok(Tested::Opened);
    }
    if FunctionName::named(criterion.function, FunctionName::ANY_CLICKS) {
        return Ok(Tested::Clicked { link: None });
    }
    if FunctionName::named(criterion.function, FunctionName::ANY_CLICK_URLS) {
        let link = listed(&criterion.kwargs, "click_urls")
            .first()
            .and_then(serde_json::Value::as_str)
            .and_then(|text| text.parse().ok())
            .map(LinkMatch::Url);
        return Ok(Tested::Clicked { link });
    }
    if FunctionName::named(criterion.function, FunctionName::ANY_CLICK_URL_CONTAINS) {
        let link = listed(&criterion.kwargs, "fragments")
            .first()
            .and_then(serde_json::Value::as_str)
            .and_then(|text| text.parse().ok())
            .map(LinkMatch::Fragment);
        return Ok(Tested::Clicked { link });
    }
    Err(ReadError(format!(
        "'{}' is not a split test the builder writes",
        criterion.function
    )))
}

fn read_tags(kwargs: &Object, key: &str) -> Result<Vec<Tag>, ReadError> {
    let mut tags = Vec::new();
    for entry in listed(kwargs, key) {
        let tag: Tag = entry
            .as_str()
            .and_then(|text| text.parse().ok())
            .ok_or_else(|| ReadError(format!("{key} holds a value that is not a tag")))?;
        tags.push(tag);
    }
    Ok(tags)
}

fn kwargs_named(object: &Object, functions: &[FunctionName]) -> Result<Object, ReadError> {
    let definition = object
        .get("definition")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| ReadError("an action carries no definition".to_string()))?;
    let function = definition
        .get("function")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| ReadError("a definition names no function".to_string()))?;
    if !functions
        .iter()
        .any(|named| FunctionName::named(function, *named))
    {
        return Err(ReadError(format!(
            "'{function}' is not the function this action's metadata names"
        )));
    }
    Ok(definition
        .get("kwargs")
        .and_then(serde_json::Value::as_object)
        .cloned()
        .unwrap_or_default())
}

fn text(kwargs: &Object, key: &str) -> Result<String, ReadError> {
    kwargs
        .get(key)
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| ReadError(format!("{key} is not text")))
}

pub(super) fn identity(preserved: &PreservedRule) -> RuleId {
    preserved.rule.id
}

pub(super) fn cadence(rules: &[OwnedRule]) -> SendCadence {
    let recurring = rules.iter().any(|owned| owned.rule.rule.recurring);
    if recurring {
        SendCadence::Recurring
    } else {
        SendCadence::Once
    }
}

pub(super) fn declared_branch(rules: &[OwnedRule], branch: Branch) -> Option<BranchId> {
    let declared = rules.iter().find_map(|owned| match &owned.rule.rule.body {
        Body::Action(Action::SetBranch { branches, .. }) => Some(branches),
        _ => None,
    })?;
    declared
        .get(usize::from(matches!(branch, Branch::No)))
        .and_then(|declared| declared.id)
}

pub(super) fn emit_step(
    step: &Step,
    at: &Position,
    timezone: &Timezone,
) -> (RuleId, Vec<OwnedRule>) {
    let recurring = matches!(at.cadence, SendCadence::Recurring);
    let anchor = RuleId::new();
    let rules = match &step.kind {
        StepKind::Message { message } => {
            let mut owned = vec![owned(
                Slot::Action,
                action_rule(
                    anchor,
                    &at.parents,
                    compose_definition(message.as_ref()),
                    Dispatch::SendMessage,
                    Some(recurring),
                    at.branch,
                ),
            )];
            owned.extend(automation_rules(
                step,
                anchor,
                at,
                recurring,
                message.as_ref(),
            ));
            owned
        }
        StepKind::Wait {
            timing,
            timezone_source,
            ..
        } => {
            let correlation = uuid::Uuid::new_v4().to_string();
            let wait = action_rule(
                anchor,
                &at.parents,
                definition_of(
                    FunctionName::WAIT,
                    wait_kwargs(&correlation, timing, timezone, *timezone_source),
                ),
                Dispatch::Wait,
                Some(recurring),
                at.branch,
            );
            let completion = event_rule(
                RuleId::new(),
                &[anchor],
                TYPE_WAIT_COMPLETE,
                Some(filter_object(
                    MODE_ANY,
                    vec![correlation_criterion(&correlation)],
                    None,
                )),
                Dispatch::WaitComplete,
                Some(recurring),
                at.branch,
            );
            vec![owned(Slot::Action, wait), owned(Slot::Event, completion)]
        }
        StepKind::Tag { applied, removed } => vec![owned(
            Slot::Action,
            action_rule(
                anchor,
                &at.parents,
                definition_of(
                    FunctionName::MODIFY_TAGS,
                    modify_tags_kwargs(applied, removed),
                ),
                Dispatch::Tag,
                Some(recurring),
                at.branch,
            ),
        )],
        StepKind::Feed {
            url, check_every, ..
        } => {
            let correlation = uuid::Uuid::new_v4().to_string();
            let completion = RuleId::new();
            let check = action_rule(
                anchor,
                &at.parents,
                definition_of(
                    FunctionName::WAIT,
                    check_feed_kwargs(&correlation, check_every, timezone),
                ),
                Dispatch::CheckFeed,
                None,
                at.branch,
            );
            let ready = event_rule(
                completion,
                &[anchor],
                TYPE_WAIT_COMPLETE,
                Some(filter_object(
                    MODE_ALL,
                    vec![
                        correlation_criterion(&correlation),
                        feed_criterion(url.as_ref()),
                    ],
                    None,
                )),
                Dispatch::CheckFeedComplete,
                Some(true),
                at.branch,
            );
            vec![owned(Slot::Action, check), owned(Slot::Event, ready)]
        }
        StepKind::Split { tested, .. } => {
            let memoize = uuid::Uuid::new_v4().to_string();
            let branches = serde_json::Value::Array(
                [true, false]
                    .into_iter()
                    .map(|expect| {
                        filter_object(
                            MODE_ALL,
                            vec![split_criterion(tested, expect, &memoize)],
                            Some(BranchId::new()),
                        )
                    })
                    .collect(),
            );
            let mut kwargs = Object::new();
            kwargs.insert("branches".to_string(), branches);
            kwargs.insert("ruleset".to_string(), token(RULESET));
            vec![owned(
                Slot::Action,
                action_rule(
                    anchor,
                    &at.parents,
                    definition_of(FunctionName::SET_BRANCH, kwargs),
                    Dispatch::SetBranch,
                    None,
                    at.branch,
                ),
            )]
        }
    };
    (anchor, rules)
}

pub(super) fn rewrite(
    preserved: &PreservedRule,
    step: &Step,
    click: Option<usize>,
) -> PreservedRule {
    let Some(object) = preserved.arrived.as_object() else {
        return preserved.clone();
    };
    let mut rewritten = object.clone();
    match &step.kind {
        StepKind::Message { message } => {
            if let Some(kwargs) = kwargs_of(&mut rewritten, FunctionName::COMPOSE) {
                write_compose(kwargs, message.as_ref());
            }
            rewrite_automation(&mut rewritten, step, message.as_ref(), click);
        }
        StepKind::Wait {
            timing,
            timezone_source,
            deleted,
        } => {
            if let Some(kwargs) = kwargs_of(&mut rewritten, FunctionName::WAIT) {
                write_timing(kwargs, timing);
                kwargs.insert(
                    "use_subscriber_timezone".to_string(),
                    value_of(timezone_source),
                );
            }
            if *deleted {
                rewritten.insert("is_deleted".to_string(), serde_json::Value::Bool(true));
            } else {
                rewritten.remove("is_deleted");
            }
        }
        StepKind::Tag { applied, removed } => {
            if let Some(kwargs) = kwargs_of(&mut rewritten, FunctionName::MODIFY_TAGS) {
                kwargs.insert("add_labels".to_string(), tag_list(applied));
                kwargs.insert("remove_labels".to_string(), tag_list(removed));
            }
        }
        StepKind::Feed {
            url, check_every, ..
        } => {
            if let Some(kwargs) = kwargs_of(&mut rewritten, FunctionName::WAIT) {
                kwargs.insert(
                    "rrules".to_string(),
                    serde_json::Value::Array(vec![value_of(check_every)]),
                );
            }
            rewrite_criteria(&mut rewritten, FunctionName::FEED_CHANGED, |kwargs| {
                kwargs.insert("url".to_string(), feed_url(url.as_ref()));
            });
        }
        StepKind::Split { tested, .. } => rewrite_split(&mut rewritten, tested),
    }
    PreservedRule::emitted(serde_json::Value::Object(rewritten))
}

pub(super) fn reposition(preserved: &PreservedRule, at: &Position) -> PreservedRule {
    let Some(object) = preserved.arrived.as_object() else {
        return preserved.clone();
    };
    let mut rewritten = object.clone();
    rewritten.insert("parents".to_string(), id_list(&at.parents));
    match at.branch {
        Some(branch) => {
            rewritten.insert(
                "branch".to_string(),
                serde_json::Value::String(branch.to_string()),
            );
        }
        None => {
            rewritten.remove("branch");
        }
    }
    PreservedRule::emitted(serde_json::Value::Object(rewritten))
}

pub(super) fn emit_starter(starter: &Starter, preserved: Option<&PreservedRule>) -> PreservedRule {
    let id = preserved.map(identity).unwrap_or_else(RuleId::new);
    match starter {
        Starter::NewSubscriber { conditions } => {
            let criteria: Vec<serde_json::Value> =
                conditions.iter().map(condition_criterion).collect();
            event_rule(
                id,
                &[],
                TYPE_SUBSCRIBE,
                filter_or_none(MODE_ALL, criteria),
                Dispatch::Subscribe,
                None,
                None,
            )
        }
        Starter::Tag {
            tags,
            except,
            reentry,
        } => {
            let mut criteria = Vec::new();
            if !tags.is_empty() {
                criteria.push(label_criterion(tags));
                criteria.push(any_tags_criterion(tags, true));
            }
            if !except.is_empty() {
                criteria.push(any_tags_criterion(except, false));
            }
            if *reentry {
                criteria.push(reentry_criterion());
            }
            event_rule(
                id,
                &[],
                TYPE_TAG,
                filter_or_none(MODE_ALL, criteria),
                Dispatch::TagEvent,
                None,
                None,
            )
        }
    }
}

pub(super) fn emit_exit(
    tags: &[Tag],
    event: Option<&PreservedRule>,
    stop: Option<&PreservedRule>,
) -> Vec<OwnedRule> {
    if tags.is_empty() {
        return Vec::new();
    }
    let trigger = event.map(identity).unwrap_or_else(RuleId::new);
    let halt = stop.map(identity).unwrap_or_else(RuleId::new);
    vec![
        owned(
            Slot::Event,
            event_rule(
                trigger,
                &[],
                TYPE_TAG,
                Some(filter_object(MODE_ALL, vec![label_criterion(tags)], None)),
                Dispatch::TagEvent,
                None,
                None,
            ),
        ),
        owned(
            Slot::Action,
            action_rule(
                halt,
                &[trigger],
                definition_of(FunctionName::STOP, stop_kwargs()),
                Dispatch::Stop,
                None,
                None,
            ),
        ),
    ]
}

fn owned(slot: Slot, rule: PreservedRule) -> OwnedRule {
    OwnedRule { slot, rule }
}

fn event_rule(
    id: RuleId,
    parents: &[RuleId],
    kind: &str,
    filter: Option<serde_json::Value>,
    dispatch: Dispatch,
    recurring: Option<bool>,
    branch: Option<BranchId>,
) -> PreservedRule {
    let mut object = Object::new();
    object.insert("id".to_string(), serde_json::Value::String(id.to_string()));
    object.insert("parents".to_string(), id_list(parents));
    object.insert("type".to_string(), token(kind));
    object.insert(
        "filter".to_string(),
        filter.unwrap_or(serde_json::Value::Null),
    );
    object.insert("title".to_string(), serde_json::Value::Null);
    object.insert("metadata".to_string(), token(dispatch.word()));
    if let Some(recurring) = recurring {
        object.insert("recurring".to_string(), serde_json::Value::Bool(recurring));
    }
    if let Some(branch) = branch {
        object.insert(
            "branch".to_string(),
            serde_json::Value::String(branch.to_string()),
        );
    }
    PreservedRule::emitted(serde_json::Value::Object(object))
}

fn action_rule(
    id: RuleId,
    parents: &[RuleId],
    definition: serde_json::Value,
    dispatch: Dispatch,
    recurring: Option<bool>,
    branch: Option<BranchId>,
) -> PreservedRule {
    let mut object = Object::new();
    object.insert("id".to_string(), serde_json::Value::String(id.to_string()));
    object.insert("parents".to_string(), id_list(parents));
    object.insert("definition".to_string(), definition);
    object.insert("title".to_string(), serde_json::Value::Null);
    object.insert("metadata".to_string(), token(dispatch.word()));
    if let Some(recurring) = recurring {
        object.insert("recurring".to_string(), serde_json::Value::Bool(recurring));
    }
    if let Some(branch) = branch {
        object.insert(
            "branch".to_string(),
            serde_json::Value::String(branch.to_string()),
        );
    }
    PreservedRule::emitted(serde_json::Value::Object(object))
}

fn automation_rules(
    step: &Step,
    anchor: RuleId,
    at: &Position,
    recurring: bool,
    message: Option<&MessageId>,
) -> Vec<OwnedRule> {
    let mut emitted = Vec::new();
    if let Some(rule) = &step.automations.on_open {
        let event = RuleId::new();
        emitted.push(owned(
            Slot::AutomationEvent,
            event_rule(
                event,
                &[anchor],
                TYPE_OPEN,
                Some(filter_object(
                    MODE_ALL,
                    vec![message_criterion(message, recurring)],
                    None,
                )),
                Dispatch::Open,
                Some(recurring),
                at.branch,
            ),
        ));
        emitted.extend(effect_rules(
            event,
            &rule.applied,
            &rule.removed,
            rule.exit,
            recurring,
            at.branch,
        ));
    }
    for rule in &step.automations.on_click {
        let event = RuleId::new();
        let mut criteria = vec![message_criterion(message, recurring)];
        if !rule.links.is_empty() {
            criteria.push(destination_criterion(&rule.links));
        }
        emitted.push(owned(
            Slot::AutomationEvent,
            event_rule(
                event,
                &[anchor],
                TYPE_CLICK,
                Some(filter_object(MODE_ALL, criteria, None)),
                Dispatch::Click,
                Some(recurring),
                at.branch,
            ),
        ));
        emitted.extend(effect_rules(
            event,
            &rule.applied,
            &rule.removed,
            rule.exit,
            recurring,
            at.branch,
        ));
    }
    emitted
}

fn effect_rules(
    event: RuleId,
    applied: &[Tag],
    removed: &[Tag],
    exit: bool,
    recurring: bool,
    branch: Option<BranchId>,
) -> Vec<OwnedRule> {
    let mut emitted = Vec::new();
    if !applied.is_empty() || !removed.is_empty() {
        emitted.push(owned(
            Slot::Action,
            action_rule(
                RuleId::new(),
                &[event],
                definition_of(
                    FunctionName::MODIFY_TAGS,
                    modify_tags_kwargs(applied, removed),
                ),
                Dispatch::Tag,
                Some(recurring),
                branch,
            ),
        ));
    }
    if exit {
        emitted.push(owned(
            Slot::Action,
            action_rule(
                RuleId::new(),
                &[event],
                definition_of(FunctionName::STOP, stop_kwargs()),
                Dispatch::Stop,
                Some(recurring),
                branch,
            ),
        ));
    }
    emitted
}

fn definition_of(function: FunctionName, kwargs: Object) -> serde_json::Value {
    let mut definition = Object::new();
    definition.insert("function".to_string(), token(function.0));
    definition.insert("kwargs".to_string(), serde_json::Value::Object(kwargs));
    serde_json::Value::Object(definition)
}

fn compose_definition(message: Option<&MessageId>) -> serde_json::Value {
    let mut kwargs = Object::new();
    kwargs.insert("account".to_string(), token(EVENT_ACCOUNT));
    kwargs.insert("list".to_string(), token(EVENT_LIST));
    write_compose(&mut kwargs, message);
    kwargs.insert("recipient".to_string(), token(EVENT_RECIPIENT));
    definition_of(FunctionName::COMPOSE, kwargs)
}

fn write_compose(kwargs: &mut Object, message: Option<&MessageId>) {
    match message {
        Some(message) => {
            kwargs.insert("meapi_id".to_string(), value_of(message));
        }
        None => {
            kwargs.remove("meapi_id");
        }
    }
    kwargs.insert("message".to_string(), token(&new_message(message)));
}

fn wait_kwargs(
    correlation: &str,
    timing: &WaitTiming,
    timezone: &Timezone,
    source: TimezoneSource,
) -> Object {
    let mut kwargs = Object::new();
    kwargs.insert("id".to_string(), token(correlation));
    kwargs.insert("account".to_string(), token(EVENT_ACCOUNT));
    kwargs.insert("list".to_string(), token(EVENT_LIST));
    kwargs.insert("recipient".to_string(), token(EVENT_RECIPIENT));
    kwargs.insert("ruleset".to_string(), token(RULESET));
    write_timing(&mut kwargs, timing);
    kwargs.insert("timezone".to_string(), value_of(timezone));
    kwargs.insert("action".to_string(), token(ACTION));
    kwargs.insert("use_subscriber_timezone".to_string(), value_of(&source));
    kwargs
}

fn write_timing(kwargs: &mut Object, timing: &WaitTiming) {
    kwargs.insert(
        "delay".to_string(),
        match &timing.delay {
            Some(delay) => value_of(delay),
            None => token(ZERO_DURATION),
        },
    );
    kwargs.insert(
        "rrules".to_string(),
        serde_json::Value::Array(timing.schedules.iter().map(value_of).collect()),
    );
}

fn check_feed_kwargs(correlation: &str, check_every: &Recurrence, timezone: &Timezone) -> Object {
    let mut kwargs = Object::new();
    kwargs.insert("id".to_string(), token(correlation));
    kwargs.insert("account".to_string(), token(EVENT_ACCOUNT));
    kwargs.insert("list".to_string(), token(EVENT_LIST));
    kwargs.insert("recipient".to_string(), token(EVENT_RECIPIENT));
    kwargs.insert("ruleset".to_string(), token(RULESET));
    kwargs.insert("delay".to_string(), token(ZERO_DURATION));
    kwargs.insert(
        "rrules".to_string(),
        serde_json::Value::Array(vec![value_of(check_every)]),
    );
    kwargs.insert("timezone".to_string(), value_of(timezone));
    kwargs.insert("action".to_string(), token(ACTION));
    kwargs.insert(
        "use_subscriber_timezone".to_string(),
        value_of(&TimezoneSource::Workflow),
    );
    kwargs
}

fn modify_tags_kwargs(applied: &[Tag], removed: &[Tag]) -> Object {
    let mut kwargs = Object::new();
    kwargs.insert("account".to_string(), token(EVENT_ACCOUNT));
    kwargs.insert("list".to_string(), token(EVENT_LIST));
    kwargs.insert("recipient".to_string(), token(SUBSCRIBER));
    kwargs.insert("add_labels".to_string(), tag_list(applied));
    kwargs.insert("remove_labels".to_string(), tag_list(removed));
    kwargs
}

fn stop_kwargs() -> Object {
    let mut kwargs = Object::new();
    kwargs.insert("ruleset".to_string(), token(RULESET));
    kwargs.insert("subscriber".to_string(), token(SUBSCRIBER));
    kwargs
}

fn filter_or_none(mode: &str, criteria: Vec<serde_json::Value>) -> Option<serde_json::Value> {
    if criteria.is_empty() {
        return None;
    }
    Some(filter_object(mode, criteria, None))
}

fn filter_object(
    mode: &str,
    criteria: Vec<serde_json::Value>,
    id: Option<BranchId>,
) -> serde_json::Value {
    let mut filter = Object::new();
    filter.insert("type".to_string(), token(mode));
    filter.insert("criteria".to_string(), serde_json::Value::Array(criteria));
    if let Some(id) = id {
        filter.insert("id".to_string(), serde_json::Value::String(id.to_string()));
    }
    serde_json::Value::Object(filter)
}

fn criterion_object(
    function: FunctionName,
    operator: &str,
    expect: serde_json::Value,
    kwargs: Object,
    memoize: Option<&str>,
) -> serde_json::Value {
    let mut criterion = Object::new();
    criterion.insert("function".to_string(), token(function.0));
    criterion.insert("operator".to_string(), token(operator));
    criterion.insert("expect".to_string(), expect);
    criterion.insert("kwargs".to_string(), serde_json::Value::Object(kwargs));
    if let Some(memoize) = memoize {
        criterion.insert("memoize_id".to_string(), token(memoize));
    }
    serde_json::Value::Object(criterion)
}

fn condition_criterion(condition: &Condition) -> serde_json::Value {
    let mut kwargs = Object::new();
    let function = match &condition.field {
        ConditionField::Country => {
            kwargs.insert("remote_ip".to_string(), token(EVENT_REMOTE_IP));
            FunctionName::GEOIP_COUNTRY
        }
        ConditionField::AdTracking => {
            write_recipient(&mut kwargs);
            FunctionName::ADTRACKING
        }
        ConditionField::Source => {
            kwargs.insert("key".to_string(), token("source"));
            FunctionName::EVENT_VALUE
        }
        ConditionField::Custom { name } => {
            write_recipient(&mut kwargs);
            kwargs.insert("field".to_string(), token(name));
            FunctionName::CUSTOM_FIELD
        }
    };
    let (operator, expect) = match &condition.test {
        ConditionTest::Is { value } => (EQUALS, token(value)),
        ConditionTest::IsNot { value } => (DIFFERS, token(value)),
        ConditionTest::Defined => (DIFFERS, serde_json::Value::Null),
        ConditionTest::Undefined => (EQUALS, serde_json::Value::Null),
    };
    criterion_object(function, operator, expect, kwargs, None)
}

fn write_recipient(kwargs: &mut Object) {
    kwargs.insert("account".to_string(), token(EVENT_ACCOUNT));
    kwargs.insert("list".to_string(), token(EVENT_LIST));
    kwargs.insert("recipient".to_string(), token(EVENT_RECIPIENT));
}

fn correlation_criterion(correlation: &str) -> serde_json::Value {
    let mut kwargs = Object::new();
    kwargs.insert("key".to_string(), token("id"));
    criterion_object(
        FunctionName::EVENT_VALUE,
        EQUALS,
        token(correlation),
        kwargs,
        None,
    )
}

fn message_criterion(message: Option<&MessageId>, recurring: bool) -> serde_json::Value {
    let mut kwargs = Object::new();
    if recurring {
        kwargs.insert("message".to_string(), token(EVENT_MESSAGE));
        return criterion_object(
            FunctionName::MESSAGEAPI_ID,
            EQUALS,
            message.map(value_of).unwrap_or(serde_json::Value::Null),
            kwargs,
            None,
        );
    }
    kwargs.insert("key".to_string(), token("message"));
    criterion_object(
        FunctionName::EVENT_VALUE,
        EQUALS,
        token(&new_message(message)),
        kwargs,
        None,
    )
}

fn destination_criterion(links: &[LinkMatch]) -> serde_json::Value {
    let mut kwargs = Object::new();
    kwargs.insert("key".to_string(), token("destination"));
    kwargs.insert("urls".to_string(), link_list(links));
    criterion_object(
        FunctionName::EVENT_VALUE_IN_URLS,
        EQUALS,
        serde_json::Value::Bool(true),
        kwargs,
        None,
    )
}

fn label_criterion(tags: &[Tag]) -> serde_json::Value {
    let mut kwargs = Object::new();
    kwargs.insert("key".to_string(), token("label"));
    kwargs.insert("values".to_string(), tag_list(tags));
    criterion_object(
        FunctionName::EVENT_VALUE_IN,
        EQUALS,
        serde_json::Value::Bool(true),
        kwargs,
        None,
    )
}

fn any_tags_criterion(tags: &[Tag], expect: bool) -> serde_json::Value {
    let mut kwargs = Object::new();
    write_recipient(&mut kwargs);
    kwargs.insert("labels".to_string(), tag_list(tags));
    criterion_object(
        FunctionName::ANY_TAGS,
        EQUALS,
        serde_json::Value::Bool(expect),
        kwargs,
        None,
    )
}

fn reentry_criterion() -> serde_json::Value {
    let mut kwargs = Object::new();
    kwargs.insert("ruleset".to_string(), token(RULESET));
    kwargs.insert("subscriber".to_string(), token(SUBSCRIBER));
    criterion_object(
        FunctionName::REENTRY_ALLOWED,
        EQUALS,
        serde_json::Value::Bool(true),
        kwargs,
        None,
    )
}

fn feed_criterion(url: Option<&FeedUrl>) -> serde_json::Value {
    let mut kwargs = Object::new();
    kwargs.insert("account".to_string(), token(EVENT_ACCOUNT));
    kwargs.insert("list".to_string(), token(EVENT_LIST));
    kwargs.insert("subscriber".to_string(), token(EVENT_SUBSCRIBER));
    kwargs.insert("ruleset".to_string(), token(RULESET));
    kwargs.insert("url".to_string(), feed_url(url));
    kwargs.insert(
        "categories".to_string(),
        serde_json::Value::Array(Vec::new()),
    );
    kwargs.insert(
        "min_new_items".to_string(),
        serde_json::Value::Number(MINIMUM_NEW_ITEMS.into()),
    );
    criterion_object(
        FunctionName::FEED_CHANGED,
        EQUALS,
        serde_json::Value::Bool(true),
        kwargs,
        None,
    )
}

fn feed_url(url: Option<&FeedUrl>) -> serde_json::Value {
    url.map(value_of).unwrap_or_else(|| token(""))
}

fn split_criterion(tested: &Tested, expect: bool, memoize: &str) -> serde_json::Value {
    let mut kwargs = Object::new();
    let function = match tested {
        Tested::HasTag { tags } => {
            write_recipient(&mut kwargs);
            kwargs.insert("labels".to_string(), tag_list(tags));
            FunctionName::ANY_TAGS
        }
        Tested::Opened => {
            write_watched(&mut kwargs);
            FunctionName::ANY_OPENS
        }
        Tested::Clicked { link: None } => {
            write_watched(&mut kwargs);
            FunctionName::ANY_CLICKS
        }
        Tested::Clicked { link: Some(link) } => {
            write_watched(&mut kwargs);
            let (key, function) = match link {
                LinkMatch::Fragment(_) => ("fragments", FunctionName::ANY_CLICK_URL_CONTAINS),
                LinkMatch::Url(_) => ("click_urls", FunctionName::ANY_CLICK_URLS),
            };
            kwargs.insert(key.to_string(), link_list(std::slice::from_ref(link)));
            function
        }
    };
    criterion_object(
        function,
        EQUALS,
        serde_json::Value::Bool(expect),
        kwargs,
        Some(memoize),
    )
}

fn write_watched(kwargs: &mut Object) {
    kwargs.insert("account".to_string(), token(EVENT_ACCOUNT));
    kwargs.insert("subscriber".to_string(), token(EVENT_SUBSCRIBER));
    kwargs.insert("messages".to_string(), token(MESSAGES_SENT));
}

fn rewrite_automation(
    rewritten: &mut Object,
    step: &Step,
    message: Option<&MessageId>,
    click: Option<usize>,
) {
    let recurring = rewritten
        .get("recurring")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let names_message = rewritten
        .get("type")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|kind| kind == TYPE_OPEN || kind == TYPE_CLICK);
    if !names_message {
        return;
    }
    if let Some(filter) = rewritten.get_mut("filter").and_then(criteria_of) {
        for criterion in filter.iter_mut() {
            let Some(object) = criterion.as_object_mut() else {
                continue;
            };
            let function = object
                .get("function")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string);
            let Some(function) = function else {
                continue;
            };
            if FunctionName::named(&function, FunctionName::EVENT_VALUE) && !recurring {
                object.insert("expect".to_string(), token(&new_message(message)));
            }
            if FunctionName::named(&function, FunctionName::MESSAGEAPI_ID) {
                object.insert(
                    "expect".to_string(),
                    message.map(value_of).unwrap_or(serde_json::Value::Null),
                );
            }
        }
    }
    let Some(rule) = click.and_then(|index| step.automations.on_click.get(index)) else {
        return;
    };
    let links = rule.links.clone();
    if links.is_empty() {
        return;
    }
    rewrite_criteria(rewritten, FunctionName::EVENT_VALUE_IN_URLS, |kwargs| {
        kwargs.insert("urls".to_string(), link_list(&links));
    });
}

fn rewrite_split(rewritten: &mut Object, tested: &Tested) {
    let function = match named_function(rewritten) {
        Some(function)
            if FunctionName::named(&function, FunctionName::SET_BRANCH)
                || FunctionName::named(&function, FunctionName::SET_BRANCH_V1) =>
        {
            function
        }
        _ => return,
    };
    let Some(kwargs) = kwargs_of_text(rewritten, &function) else {
        return;
    };
    let Some(branches) = kwargs
        .get_mut("branches")
        .and_then(|value| value.as_array_mut())
    else {
        return;
    };
    for (index, branch) in branches.iter_mut().enumerate() {
        let Some(criteria) = criteria_of(branch) else {
            continue;
        };
        let memoize = criteria
            .first()
            .and_then(serde_json::Value::as_object)
            .and_then(|object| object.get("memoize_id"))
            .and_then(serde_json::Value::as_str)
            .map(str::to_string);
        let expect = index == 0;
        *criteria = vec![split_criterion(
            tested,
            expect,
            memoize.as_deref().unwrap_or_default(),
        )];
    }
}

fn rewrite_criteria(
    rewritten: &mut Object,
    function: FunctionName,
    mut edit: impl FnMut(&mut Object),
) {
    let Some(filter) = rewritten.get_mut("filter") else {
        return;
    };
    let Some(criteria) = criteria_of(filter) else {
        return;
    };
    for criterion in criteria.iter_mut() {
        let Some(object) = criterion.as_object_mut() else {
            continue;
        };
        let named = object
            .get("function")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|text| FunctionName::named(text, function));
        if !named {
            continue;
        }
        if let Some(kwargs) = object
            .get_mut("kwargs")
            .and_then(|value| value.as_object_mut())
        {
            edit(kwargs);
        }
    }
}

fn named_function(rewritten: &Object) -> Option<String> {
    let text = rewritten
        .get("definition")?
        .as_object()?
        .get("function")?
        .as_str()?;
    Some(text.to_string())
}

fn criteria_of(filter: &mut serde_json::Value) -> Option<&mut Vec<serde_json::Value>> {
    filter.as_object_mut()?.get_mut("criteria")?.as_array_mut()
}

fn kwargs_of(rewritten: &mut Object, function: FunctionName) -> Option<&mut Object> {
    kwargs_of_text(rewritten, function.0)
}

fn kwargs_of_text<'a>(rewritten: &'a mut Object, function: &str) -> Option<&'a mut Object> {
    let definition = rewritten.get_mut("definition")?.as_object_mut()?;
    let named = definition
        .get("function")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|text| text == function);
    if !named {
        return None;
    }
    definition.get_mut("kwargs")?.as_object_mut()
}

fn listed(object: &Object, key: &str) -> Vec<serde_json::Value> {
    object
        .get(key)
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default()
}

fn listed_ref<'a>(object: &'a Object, key: &str) -> &'a [serde_json::Value] {
    object
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map_or(&[], Vec::as_slice)
}

fn id_list(ids: &[RuleId]) -> serde_json::Value {
    serde_json::Value::Array(
        ids.iter()
            .map(|id| serde_json::Value::String(id.to_string()))
            .collect(),
    )
}

fn tag_list(tags: &[Tag]) -> serde_json::Value {
    serde_json::Value::Array(tags.iter().map(value_of).collect())
}

fn link_list(links: &[LinkMatch]) -> serde_json::Value {
    serde_json::Value::Array(
        links
            .iter()
            .map(|link| serde_json::Value::String(link.to_string()))
            .collect(),
    )
}

fn token(text: &str) -> serde_json::Value {
    serde_json::Value::String(text.to_string())
}

fn value_of<T: serde::Serialize>(value: &T) -> serde_json::Value {
    serde_json::to_value(value).unwrap_or(serde_json::Value::Null)
}

fn new_message(message: Option<&MessageId>) -> String {
    match message {
        Some(message) => format!("<message:new message={message}>"),
        None => "<message:new message=undefined>".to_string(),
    }
}

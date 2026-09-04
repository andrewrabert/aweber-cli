use std::collections::BTreeMap;

use aweber::ids::MessageId;
use aweber::workflows as domain;
use serde::Serialize;

pub(crate) struct WorkflowView<'a> {
    pub(crate) workflow: &'a domain::Workflow,
    pub(crate) starter: Option<&'a domain::Starter>,
    pub(crate) changes: domain::ChangeCount,
}

#[derive(Serialize)]
pub(crate) struct ListDocument {
    workflows: Vec<ListedWorkflow>,
}

#[derive(Serialize)]
struct ListedWorkflow {
    id: Option<String>,
    name: Option<String>,
    status: Option<String>,
    unpublished_changes: domain::ChangeCount,
    starter: Option<Starter>,
    updated: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct WorkflowDocument {
    id: Option<String>,
    name: Option<String>,
    list: Option<String>,
    status: Option<String>,
    unpublished_changes: domain::ChangeCount,
    starter: Option<Starter>,
    timezone: Option<String>,
    sharing: Sharing,
    last_published: Option<String>,
    updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<String>>,
}

#[derive(Serialize)]
struct Sharing {
    enabled: bool,
    code: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct ShowDocument {
    about: WorkflowDocument,
    steps: Vec<Step>,
    exit_tags: Vec<String>,
}

#[derive(Serialize)]
pub(crate) struct DeleteDocument {
    #[serde(flatten)]
    workflow: WorkflowDocument,
    messages_returned: u64,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
enum Starter {
    NewSubscriber {
        conditions: Vec<Condition>,
    },
    Tag {
        tags: Vec<String>,
        except: Vec<String>,
        reentry: bool,
    },
}

#[derive(Serialize)]
struct Condition {
    #[serde(flatten)]
    field: ConditionField,
    #[serde(flatten)]
    test: ConditionTest,
}

#[derive(Serialize)]
#[serde(tag = "field", rename_all = "kebab-case")]
enum ConditionField {
    Source,
    AdTracking,
    Country,
    Custom { name: String },
}

#[derive(Serialize)]
#[serde(tag = "test", rename_all = "kebab-case")]
enum ConditionTest {
    Is {
        #[serde(rename = "value")]
        text: String,
    },
    IsNot {
        #[serde(rename = "value")]
        text: String,
    },
    Defined,
    Undefined,
}

struct Enrichment<'a> {
    subjects: &'a BTreeMap<MessageId, String>,
    stats: Option<&'a BTreeMap<MessageId, domain::MessageTotals>>,
}

#[derive(Serialize)]
struct Step {
    id: String,
    #[serde(flatten)]
    kind: StepKind,
    automations: Automations,
    #[serde(skip_serializing_if = "Option::is_none")]
    stats: Option<Option<Stats>>,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
enum StepKind {
    Message {
        message: Option<Message>,
    },
    Wait {
        duration: Option<String>,
        schedules: Vec<Recurrence>,
        timezone_source: String,
        deleted: bool,
    },
    Tag {
        applied: Vec<String>,
        removed: Vec<String>,
    },
    Feed {
        url: Option<String>,
        check_every: Recurrence,
        inside: Vec<Step>,
    },
    Split {
        tested: Tested,
        yes: Vec<Step>,
        no: Vec<Step>,
    },
}

#[derive(Serialize)]
struct Message {
    id: String,
    subject: Option<String>,
}

#[derive(Serialize)]
struct Recurrence {
    every: String,
    days: Option<Vec<String>>,
    day_of_month: Option<String>,
    at: Option<String>,
    ends: Ends,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Ends {
    Never,
    After(u32),
    Until(String),
}

#[derive(Serialize)]
#[serde(tag = "on", rename_all = "lowercase")]
enum Tested {
    Tagged { tags: Vec<String> },
    Opened,
    Clicked { link: Option<String> },
}

#[derive(Serialize)]
struct Automations {
    on_open: Option<OpenRule>,
    on_click: Vec<ClickRule>,
}

#[derive(Serialize)]
struct OpenRule {
    applied: Vec<String>,
    removed: Vec<String>,
    exit: bool,
}

#[derive(Serialize)]
struct ClickRule {
    links: Vec<String>,
    applied: Vec<String>,
    removed: Vec<String>,
    exit: bool,
}

#[derive(Serialize)]
struct Stats {
    sends: u64,
    opens: u64,
    open_rate: domain::Rate,
    clicks: u64,
    click_rate: domain::Rate,
    bounces: u64,
}

pub(crate) fn list_document(views: &[WorkflowView<'_>]) -> ListDocument {
    let mut rows: Vec<&WorkflowView<'_>> = views.iter().collect();
    rows.sort_by_key(|row| std::cmp::Reverse(row.workflow.updated()));
    ListDocument {
        workflows: rows
            .into_iter()
            .map(|view| ListedWorkflow {
                id: text(view.workflow.id()),
                name: text(view.workflow.name()),
                status: status(view),
                unpublished_changes: view.changes,
                starter: view.starter.map(Starter::from),
                updated: moment(view.workflow.updated()),
            })
            .collect(),
    }
}

pub(crate) fn workflow_document(view: &WorkflowView<'_>) -> WorkflowDocument {
    WorkflowDocument {
        id: text(view.workflow.id()),
        name: text(view.workflow.name()),
        list: text(view.workflow.list()),
        status: status(view),
        unpublished_changes: view.changes,
        starter: view.starter.map(Starter::from),
        timezone: text(view.workflow.timezone()),
        sharing: Sharing {
            enabled: view.workflow.sharing_enabled(),
            code: text(view.workflow.id()),
        },
        last_published: moment(view.workflow.last_published()),
        updated: moment(view.workflow.updated()),
        errors: None,
    }
}

pub(crate) fn show_document(
    view: &WorkflowView<'_>,
    steps: &[domain::Step],
    exit_tags: &[domain::Tag],
    subjects: &BTreeMap<MessageId, String>,
    stats: Option<&BTreeMap<MessageId, domain::MessageTotals>>,
) -> ShowDocument {
    let mut about = workflow_document(view);
    about.errors = Some(view.workflow.errors());
    let enrichment = Enrichment {
        subjects,
        stats: stats.filter(|totals| !totals.is_empty()),
    };
    ShowDocument {
        about,
        steps: enrichment.enrich_all(steps),
        exit_tags: words(exit_tags),
    }
}

pub(crate) fn publish_document(view: &WorkflowView<'_>) -> WorkflowDocument {
    let mut document = workflow_document(view);
    document.errors = Some(view.workflow.errors());
    document
}

pub(crate) fn delete_document(
    view: &WorkflowView<'_>,
    unbound: &domain::BatchOutcome,
) -> DeleteDocument {
    DeleteDocument {
        workflow: workflow_document(view),
        messages_returned: unbound.processed.len() as u64,
    }
}

pub(crate) fn print(document: &impl Serialize) -> anyhow::Result<()> {
    use std::io::IsTerminal;
    let document = serde_json::to_value(document)?;
    let rendered = if std::io::stdout().is_terminal() {
        colored_json::to_colored_json_auto(&document)?
    } else {
        serde_json::to_string_pretty(&document)?
    };
    println!("{rendered}");
    Ok(())
}

impl Enrichment<'_> {
    fn enrich(&self, step: &domain::Step) -> Step {
        let kind = match &step.kind {
            domain::StepKind::Message { message } => StepKind::Message {
                message: message.as_ref().map(|message| Message {
                    id: message.to_string(),
                    subject: self.subjects.get(message).cloned(),
                }),
            },
            domain::StepKind::Wait {
                timing,
                timezone_source,
                deleted,
            } => StepKind::Wait {
                duration: text(timing.delay.as_ref()),
                schedules: timing.schedules.iter().map(Recurrence::from).collect(),
                timezone_source: timezone_source.to_string(),
                deleted: *deleted,
            },
            domain::StepKind::Tag { applied, removed } => StepKind::Tag {
                applied: words(applied),
                removed: words(removed),
            },
            domain::StepKind::Feed {
                url,
                check_every,
                inside,
            } => StepKind::Feed {
                url: text(url.as_ref()),
                check_every: Recurrence::from(check_every),
                inside: self.enrich_all(inside),
            },
            domain::StepKind::Split { tested, yes, no } => StepKind::Split {
                tested: Tested::from(tested),
                yes: self.enrich_all(yes),
                no: self.enrich_all(no),
            },
        };
        let stats = self.stats.map(|stats| {
            sent_message(&step.kind)
                .and_then(|message| stats.get(message))
                .and_then(Stats::measured)
        });
        Step {
            id: step.id.to_string(),
            kind,
            automations: Automations::from(&step.automations),
            stats,
        }
    }

    fn enrich_all(&self, steps: &[domain::Step]) -> Vec<Step> {
        steps.iter().map(|step| self.enrich(step)).collect()
    }
}

impl Stats {
    fn measured(totals: &domain::MessageTotals) -> Option<Stats> {
        match (totals.open_rate(), totals.click_rate()) {
            (Some(open_rate), Some(click_rate)) => Some(Stats {
                sends: totals.total_sent,
                opens: totals.total_opens,
                open_rate,
                clicks: totals.total_clicks,
                click_rate,
                bounces: totals.total_bounces,
            }),
            _ => None,
        }
    }
}

impl From<&domain::Starter> for Starter {
    fn from(starter: &domain::Starter) -> Starter {
        match starter {
            domain::Starter::NewSubscriber { conditions } => Starter::NewSubscriber {
                conditions: conditions.iter().map(Condition::from).collect(),
            },
            domain::Starter::Tag {
                tags,
                except,
                reentry,
            } => Starter::Tag {
                tags: words(tags),
                except: words(except),
                reentry: *reentry,
            },
        }
    }
}

impl From<&domain::Condition> for Condition {
    fn from(condition: &domain::Condition) -> Condition {
        Condition {
            field: match &condition.field {
                domain::ConditionField::Source => ConditionField::Source,
                domain::ConditionField::AdTracking => ConditionField::AdTracking,
                domain::ConditionField::Country => ConditionField::Country,
                domain::ConditionField::Custom { name } => {
                    ConditionField::Custom { name: name.clone() }
                }
            },
            test: match &condition.test {
                domain::ConditionTest::Is { value } => ConditionTest::Is {
                    text: value.clone(),
                },
                domain::ConditionTest::IsNot { value } => ConditionTest::IsNot {
                    text: value.clone(),
                },
                domain::ConditionTest::Defined => ConditionTest::Defined,
                domain::ConditionTest::Undefined => ConditionTest::Undefined,
            },
        }
    }
}

impl From<&domain::Recurrence> for Recurrence {
    fn from(recurrence: &domain::Recurrence) -> Recurrence {
        Recurrence {
            every: recurrence.to_string(),
            days: recurrence
                .days()
                .map(|days| days.to_string().split(',').map(str::to_string).collect()),
            day_of_month: text(recurrence.day_of_month()),
            at: text(recurrence.at()),
            ends: Ends::from(recurrence.ends()),
        }
    }
}

impl From<domain::Ends> for Ends {
    fn from(ends: domain::Ends) -> Ends {
        match ends {
            domain::Ends::Never => Ends::Never,
            domain::Ends::After { checks } => Ends::After(checks),
            domain::Ends::Until { day } => Ends::Until(day.to_string()),
        }
    }
}

impl From<&domain::Tested> for Tested {
    fn from(tested: &domain::Tested) -> Tested {
        match tested {
            domain::Tested::HasTag { tags } => Tested::Tagged { tags: words(tags) },
            domain::Tested::Opened => Tested::Opened,
            domain::Tested::Clicked { link } => Tested::Clicked {
                link: text(link.as_ref()),
            },
        }
    }
}

impl From<&domain::Automations> for Automations {
    fn from(automations: &domain::Automations) -> Automations {
        Automations {
            on_open: automations.on_open.as_ref().map(|rule| OpenRule {
                applied: words(&rule.applied),
                removed: words(&rule.removed),
                exit: rule.exit,
            }),
            on_click: automations
                .on_click
                .iter()
                .map(|rule| ClickRule {
                    links: words(&rule.links),
                    applied: words(&rule.applied),
                    removed: words(&rule.removed),
                    exit: rule.exit,
                })
                .collect(),
        }
    }
}

fn sent_message(kind: &domain::StepKind) -> Option<&MessageId> {
    match kind {
        domain::StepKind::Message {
            message: Some(message),
        } => Some(message),
        _ => None,
    }
}

fn status(view: &WorkflowView<'_>) -> Option<String> {
    view.workflow.status().ok().map(|status| status.to_string())
}

fn words<T: std::fmt::Display>(values: &[T]) -> Vec<String> {
    values
        .iter()
        .map(std::string::ToString::to_string)
        .collect()
}

fn text<T: std::fmt::Display>(value: Option<T>) -> Option<String> {
    value.map(|value| value.to_string())
}

fn moment(at: Option<chrono::DateTime<chrono::Utc>>) -> Option<String> {
    at.map(|at| at.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
}

pub(crate) mod object;
pub(crate) mod request;

use std::collections::BTreeMap;

use aweber::ids::{ListUid, MessageId, WorkflowId};
use aweber::workflows::{
    self, Automations, BatchOutcome, Graph, GraphError, MessageCadence, MessageTotals,
    PreconditionVersion, SendCadence, StatusChange, Step, Timezone, Workflow, WorkflowEdit,
    WorkflowPatch, WorkflowStatus,
};

use crate::cli::Cli;
use crate::workflows::object::WorkflowView;
use crate::workflows::request::{
    AddStepRequest, AutomationFlags, CreateRequest, ListRequest, PublishRequest, ShowRequest,
    ShowVersion, StepChange, UpdateRequest, UpdateStepRequest, WorkflowSource,
};

pub(crate) struct Failure {
    lines: Vec<String>,
    code: i32,
}

impl Failure {
    pub(crate) fn api(message: impl std::fmt::Display) -> Failure {
        Failure {
            lines: vec![message.to_string()],
            code: 1,
        }
    }

    pub(crate) fn blocked(workflow: &Workflow) -> Failure {
        Failure {
            lines: workflow.errors(),
            code: 1,
        }
    }

    pub(crate) fn usage(message: impl std::fmt::Display) -> Failure {
        Failure {
            lines: vec![message.to_string()],
            code: 2,
        }
    }

    pub(crate) fn report(self) -> ! {
        for line in &self.lines {
            eprintln!("error: {line}");
        }
        std::process::exit(self.code)
    }
}

impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.lines.join("\n"))
    }
}

impl std::fmt::Debug for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Failure {}

struct Reading {
    workflow: Workflow,
    working: Graph,
    published: Graph,
}

impl Reading {
    fn of(workflow: Workflow) -> Result<Reading, Failure> {
        let zone = workflow.timezone().unwrap_or_else(Timezone::utc);
        let ruleset = workflow.ruleset().map_err(Failure::api)?;
        Ok(Reading {
            working: ruleset.working(zone.clone()),
            published: ruleset.published(zone),
            workflow,
        })
    }

    fn view(&self) -> WorkflowView<'_> {
        WorkflowView {
            workflow: &self.workflow,
            starter: self.working.starter(),
            changes: self.working.changes_against(&self.published),
        }
    }

    fn precondition(&self) -> Result<PreconditionVersion, Failure> {
        self.workflow
            .precondition_version()
            .ok_or_else(|| Failure::api("the workflow document carries no precondition version"))
    }
}

fn graph_failure(workflow: WorkflowId, error: GraphError) -> Failure {
    match error {
        GraphError::NotScheduled { .. } => {
            Failure::usage(format!("{error}; pass --send-on and --send-at together"))
        }
        GraphError::UnknownStep { .. } => Failure::api(format!("{workflow} {error}")),
        other => Failure::api(other),
    }
}

fn cadence_of(cadence: MessageCadence) -> Option<SendCadence> {
    match cadence {
        MessageCadence::Once => Some(SendCadence::Once),
        MessageCadence::Recurring => Some(SendCadence::Recurring),
        MessageCadence::Mixed => None,
    }
}

fn sent_messages(steps: &[Step]) -> Vec<MessageId> {
    let mut found = Vec::new();
    walk_messages(steps, &mut found);
    found
}

fn walk_messages(steps: &[Step], found: &mut Vec<MessageId>) {
    for step in steps {
        match &step.kind {
            workflows::StepKind::Message {
                message: Some(message),
            } => {
                if !found.contains(message) {
                    found.push(message.clone());
                }
            }
            workflows::StepKind::Feed { inside, .. } => walk_messages(inside, found),
            workflows::StepKind::Split { yes, no, .. } => {
                walk_messages(yes, found);
                walk_messages(no, found);
            }
            _ => {}
        }
    }
}

impl Cli {
    pub(crate) async fn resolve_workflow(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<WorkflowId> {
        let source = matches
            .get_one::<WorkflowSource>("workflow")
            .expect("the workflow positional is required")
            .clone();
        let name = match &source {
            WorkflowSource::Id(id) => return Ok(*id),
            WorkflowSource::Name(name) => name,
        };
        if matches.get_one::<String>("list").is_none() {
            return Err(Failure::usage(format!(
                "'{name}' is not a workflow id, so --list is required"
            ))
            .into());
        }
        let list = self.resolve_list_uid(matches).await?;
        self.resolve_source(&source, list).await
    }

    async fn resolve_source(
        &self,
        source: &WorkflowSource,
        list: ListUid,
    ) -> anyhow::Result<WorkflowId> {
        let name = match source {
            WorkflowSource::Id(id) => return Ok(*id),
            WorkflowSource::Name(name) => name,
        };
        let owner = self.account_uid().await?;
        let entries = workflows::list_workflows(&self.client, owner, list)
            .await
            .map_err(Failure::api)?;
        let found: Vec<WorkflowId> = entries
            .iter()
            .filter(|entry| {
                entry.name().as_ref().map(ToString::to_string) == Some(name.to_string())
            })
            .filter_map(Workflow::id)
            .collect();
        match found.len() {
            0 => Err(Failure::api(format!("no workflow named '{name}' on list {list}")).into()),
            1 => Ok(found[0]),
            count => Err(Failure::api(format!(
                "{count} workflows are named '{name}'; pass the workflow id instead"
            ))
            .into()),
        }
    }

    async fn read(&self, workflow: WorkflowId) -> anyhow::Result<Reading> {
        let document = workflows::get_workflow(&self.client, workflow)
            .await
            .map_err(Failure::api)?;
        Ok(Reading::of(document)?)
    }

    async fn write(
        &self,
        workflow: WorkflowId,
        reading: &Reading,
        patch: &WorkflowPatch,
    ) -> anyhow::Result<Reading> {
        let updated =
            workflows::update_workflow(&self.client, workflow, reading.precondition()?, patch)
                .await
                .map_err(Failure::api)?;
        Ok(Reading::of(updated)?)
    }

    async fn subjects(&self, steps: &[Step]) -> BTreeMap<MessageId, String> {
        let messages = sent_messages(steps);
        if messages.is_empty() {
            return BTreeMap::new();
        }
        workflows::get_message_subjects(&self.client, &messages)
            .await
            .unwrap_or_default()
    }

    async fn message_stats(
        &self,
        graph: &Graph,
        steps: &[Step],
    ) -> anyhow::Result<BTreeMap<MessageId, MessageTotals>> {
        let account = self.account_uid().await?;
        let cadences = graph.message_cadences();
        let mut totals = BTreeMap::new();
        for message in sent_messages(steps) {
            let Some(cadence) = cadences.get(&message).copied().and_then(cadence_of) else {
                continue;
            };
            if let Ok(read) =
                workflows::get_campaign_message_stats(&self.client, &message, account, cadence)
                    .await
            {
                totals.insert(message, read);
            }
        }
        Ok(totals)
    }

    pub(crate) async fn execute_list_workflows(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let request = ListRequest::try_from(matches)?;
        let list = self.resolve_list_uid(matches).await?;
        let owner = self.account_uid().await?;
        let entries = workflows::list_workflows(&self.client, owner, list)
            .await
            .map_err(Failure::api)?;
        let mut readings = Vec::new();
        for entry in entries {
            readings.push(Reading::of(entry)?);
        }
        let views: Vec<WorkflowView<'_>> = readings
            .iter()
            .filter(|reading| keeps_status(reading, &request.statuses))
            .filter(|reading| keeps_starter_tag(reading, &request))
            .map(Reading::view)
            .collect();
        object::print(&object::list_document(&views))?;
        Ok(())
    }

    pub(crate) async fn execute_show_workflow(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let request = ShowRequest::try_from(matches)?;
        let workflow = self.resolve_workflow(matches).await?;
        let reading = self.read(workflow).await?;
        let graph = match request.version {
            ShowVersion::Published => &reading.published,
            _ => &reading.working,
        };
        let steps = match request.version {
            ShowVersion::Draft => {
                let changed = reading.working.changed_steps(&reading.published);
                if changed.is_empty() {
                    return Err(
                        Failure::api(format!("{workflow} has no unpublished changes")).into(),
                    );
                }
                changed
            }
            _ => graph.steps(),
        };
        let subjects = self.subjects(&steps).await;
        let stats = if request.stats {
            Some(self.message_stats(graph, &steps).await?)
        } else {
            None
        };
        let document = object::show_document(
            &reading.view(),
            &steps,
            graph.exit_tags(),
            &subjects,
            stats.as_ref(),
        );
        object::print(&document)?;
        Ok(())
    }

    pub(crate) async fn execute_create_workflow(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let request = CreateRequest::try_from(matches)?;
        let list = self.resolve_list_uid(matches).await?;
        let owner = self.account_uid().await?;
        let created = match &request.from {
            Some(source) => {
                let from = self.resolve_source(source, list).await?;
                let reading = self.read(from).await?;
                workflows::copy_workflow(
                    &self.client,
                    from,
                    reading.precondition()?,
                    &workflows::CopyWorkflow {
                        target_list: list,
                        name: Some(request.name.clone()),
                        timezone: request.properties.timezone.clone(),
                    },
                )
                .await
                .map_err(Failure::api)?
            }
            None => workflows::create_workflow(
                &self.client,
                &workflows::CreateWorkflow {
                    name: request.name.clone(),
                    owner,
                    parent: list,
                    timezone: request.properties.timezone.clone(),
                },
            )
            .await
            .map_err(Failure::api)?,
        };
        let id = created
            .id()
            .ok_or_else(|| Failure::api("the created workflow carries no id"))?;
        let reading = Reading::of(created)?;
        let reading = match properties_patch(&reading, &request.properties, WorkflowEdit::default())
        {
            Some(patch) => self.write(id, &reading, &patch).await?,
            None => reading,
        };
        object::print(&object::workflow_document(&reading.view()))?;
        Ok(())
    }

    pub(crate) async fn execute_update_workflow(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let request = UpdateRequest::try_from(matches)?;
        let workflow = self.resolve_workflow(matches).await?;
        let reading = self.read(workflow).await?;
        let current = reading.workflow.status().ok();
        let mut edit = WorkflowEdit {
            name: request.name.clone(),
            ..WorkflowEdit::default()
        };
        if let Some(status) = request.status {
            if status == StatusChange::Active
                && current == Some(WorkflowStatus::Draft)
                && reading.workflow.last_published().is_none()
            {
                return Err(Failure::api(format!("{workflow} has never been published")).into());
            }
            let asserted = current
                .and_then(|current| StatusChange::try_from(current).ok())
                .is_some_and(|current| current == status);
            if !asserted {
                edit.status = Some(status);
            }
        }
        let reading = match properties_patch(&reading, &request.properties, edit) {
            Some(patch) => self.write(workflow, &reading, &patch).await?,
            None => reading,
        };
        object::print(&object::workflow_document(&reading.view()))?;
        Ok(())
    }

    pub(crate) async fn execute_add_workflow_step(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let request = AddStepRequest::try_from(matches)?;
        let workflow = self.resolve_workflow(matches).await?;
        let reading = self.read(workflow).await?;
        let mut graph = reading.working.clone();
        let feed = matches!(request.kind, workflows::StepKind::Feed { .. });
        let added = graph
            .add_step(
                request.kind,
                if feed {
                    Automations::default()
                } else {
                    request.automations.clone().unwrap_or_default()
                },
                request.placement,
            )
            .map_err(|e| graph_failure(workflow, e))?;
        if let Some(message) = request.inside_message {
            graph
                .add_step(
                    workflows::StepKind::Message {
                        message: Some(message),
                    },
                    request.automations.unwrap_or_default(),
                    workflows::Placement::Inside { feed: added },
                )
                .map_err(|e| graph_failure(workflow, e))?;
        }
        let patch = WorkflowPatch::edits(&WorkflowEdit::default())
            .with_ruleset(reading.workflow.ruleset_write_op(), &graph);
        let written = self.write(workflow, &reading, &patch).await?;
        object::print(&object::workflow_document(&written.view()))?;
        Ok(())
    }

    pub(crate) async fn execute_update_workflow_step(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let request = UpdateStepRequest::try_from(matches)?;
        let flags = AutomationFlags::from_matches(matches)?;
        let workflow = self.resolve_workflow(matches).await?;
        let reading = self.read(workflow).await?;
        let mut graph = reading.working.clone();
        match request.change {
            StepChange::Remove => graph
                .remove_step(request.step, &reading.published)
                .map_err(|e| graph_failure(workflow, e))?,
            StepChange::Edit {
                mut edit,
                placement,
            } => {
                if let Some(flags) = flags {
                    let current = graph
                        .step(request.step)
                        .map(|step| step.automations)
                        .unwrap_or_else(Automations::default);
                    edit.automations = Some(flags.apply(&current));
                }
                graph
                    .edit_step(request.step, edit)
                    .map_err(|e| graph_failure(workflow, e))?;
                if let Some(placement) = placement {
                    graph
                        .move_step(request.step, placement)
                        .map_err(|e| graph_failure(workflow, e))?;
                }
            }
        }
        let patch = WorkflowPatch::edits(&WorkflowEdit::default())
            .with_ruleset(reading.workflow.ruleset_write_op(), &graph);
        let written = self.write(workflow, &reading, &patch).await?;
        object::print(&object::workflow_document(&written.view()))?;
        Ok(())
    }

    pub(crate) async fn execute_publish_workflow(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let request = PublishRequest::try_from(matches)?;
        let workflow = self.resolve_workflow(matches).await?;
        let reading = self.read(workflow).await?;
        let precondition = reading.precondition()?;
        if !request.discard && !reading.workflow.errors().is_empty() {
            return Err(Failure::blocked(&reading.workflow).into());
        }
        let result = if request.discard {
            workflows::revert_workflow(&self.client, workflow, precondition).await
        } else {
            workflows::publish_workflow(&self.client, workflow, precondition).await
        }
        .map_err(Failure::api)?;
        let written = Reading::of(result)?;
        object::print(&object::publish_document(&written.view()))?;
        Ok(())
    }

    pub(crate) async fn execute_delete_workflow(
        &self,
        matches: &clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let workflow = self.resolve_workflow(matches).await?;
        let reading = self.read(workflow).await?;
        let precondition = reading.precondition()?;
        let messages = sent_messages(&reading.working.steps());
        let mut unbound = BatchOutcome {
            processed: Vec::new(),
            unprocessed: Vec::new(),
        };
        if !messages.is_empty() {
            let account = self.account_uid().await?;
            match workflows::unbind_messages(&self.client, account, &messages).await {
                Ok(outcome) => {
                    if !outcome.unprocessed.is_empty() {
                        warn_untouched(&outcome.unprocessed);
                    }
                    unbound = outcome;
                }
                Err(_) => warn_untouched(&messages),
            }
        }
        workflows::delete_workflow(&self.client, workflow, precondition)
            .await
            .map_err(Failure::api)?;
        object::print(&object::delete_document(&reading.view(), &unbound))?;
        Ok(())
    }
}

fn warn_untouched(messages: &[MessageId]) {
    let named = messages
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    eprintln!("Warning: these messages were left in the workflow: {named}");
}

fn keeps_status(reading: &Reading, statuses: &[WorkflowStatus]) -> bool {
    if statuses.is_empty() {
        return true;
    }
    reading
        .workflow
        .status()
        .is_ok_and(|status| statuses.contains(&status))
}

fn keeps_starter_tag(reading: &Reading, request: &ListRequest) -> bool {
    if request.starter_tags.is_empty() {
        return true;
    }
    match reading.working.starter() {
        Some(workflows::Starter::Tag { tags, .. }) => {
            tags.iter().any(|tag| request.starter_tags.contains(tag))
        }
        _ => false,
    }
}

fn properties_patch(
    reading: &Reading,
    properties: &request::Properties,
    mut edit: WorkflowEdit,
) -> Option<WorkflowPatch> {
    edit.timezone = properties.timezone.clone();
    edit.sharing = properties.sharing;
    let touches_graph = properties.starter.is_some()
        || !properties.add_exit_tags.is_empty()
        || !properties.remove_exit_tags.is_empty();
    let touches_document = edit.name.is_some()
        || edit.status.is_some()
        || edit.timezone.is_some()
        || edit.sharing.is_some();
    if !touches_graph && !touches_document {
        return None;
    }
    let patch = WorkflowPatch::edits(&edit);
    if !touches_graph {
        return Some(patch);
    }
    let mut graph = reading.working.clone();
    if let Some(starter) = properties.starter.clone() {
        graph.set_starter(carried(starter, reading.working.starter()));
    }
    for tag in &properties.add_exit_tags {
        graph.add_exit_tag(tag.clone());
    }
    for tag in &properties.remove_exit_tags {
        graph.remove_exit_tag(tag);
    }
    Some(patch.with_ruleset(reading.workflow.ruleset_write_op(), &graph))
}

fn carried(
    requested: workflows::Starter,
    current: Option<&workflows::Starter>,
) -> workflows::Starter {
    match (requested, current) {
        (
            workflows::Starter::NewSubscriber { .. },
            Some(workflows::Starter::NewSubscriber { conditions }),
        ) => workflows::Starter::NewSubscriber {
            conditions: conditions.clone(),
        },
        (
            workflows::Starter::Tag { tags, .. },
            Some(workflows::Starter::Tag {
                except, reentry, ..
            }),
        ) => workflows::Starter::Tag {
            tags,
            except: except.clone(),
            reentry: *reentry,
        },
        (requested, _) => requested,
    }
}

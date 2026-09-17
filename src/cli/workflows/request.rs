use aweber::ids::{MessageId, RuleId, WorkflowId};
use aweber::workflows::{
    Automations, Branch, ClickRule, Delay, Ends, FeedUrl, LinkFragment, LinkMatch, LinkUrl,
    OpenRule, Placement, Recurrence, SendDays, SendTime, Sharing, Starter, StatusChange, Step,
    StepEdit, StepKind, StepName, Tag, Tested, Timezone, TimezoneSource, WaitEdit, WaitTiming,
    WorkflowName, WorkflowNameError, WorkflowStatus,
};

use crate::workflows::Failure;

#[derive(Clone, Debug)]
pub(crate) enum WorkflowSource {
    Id(WorkflowId),
    Name(WorkflowName),
}

impl std::str::FromStr for WorkflowSource {
    type Err = WorkflowNameError;

    fn from_str(text: &str) -> Result<WorkflowSource, WorkflowNameError> {
        match text.parse::<WorkflowId>() {
            Ok(id) => Ok(WorkflowSource::Id(id)),
            Err(_) => text.parse().map(WorkflowSource::Name),
        }
    }
}

impl std::fmt::Display for WorkflowSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowSource::Id(id) => std::fmt::Display::fmt(id, f),
            WorkflowSource::Name(name) => std::fmt::Display::fmt(name, f),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum StarterKind {
    NewSubscriber,
    Tag,
}

impl std::str::FromStr for StarterKind {
    type Err = UnknownStarterKind;

    fn from_str(text: &str) -> Result<StarterKind, UnknownStarterKind> {
        match text {
            "new-subscriber" => Ok(StarterKind::NewSubscriber),
            "tag" => Ok(StarterKind::Tag),
            _ => Err(UnknownStarterKind {
                rejected: text.to_string(),
            }),
        }
    }
}

impl std::fmt::Display for StarterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            StarterKind::NewSubscriber => "new-subscriber",
            StarterKind::Tag => "tag",
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct UnknownStarterKind {
    rejected: String,
}

impl std::fmt::Display for UnknownStarterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a starter", self.rejected)
    }
}

impl std::error::Error for UnknownStarterKind {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum BranchArg {
    Split(RuleId),
    Branch(Branch),
}

impl std::str::FromStr for BranchArg {
    type Err = UnknownBranchArg;

    fn from_str(text: &str) -> Result<BranchArg, UnknownBranchArg> {
        if let Ok(split) = text.parse::<RuleId>() {
            return Ok(BranchArg::Split(split));
        }
        text.parse()
            .map(BranchArg::Branch)
            .map_err(|_| UnknownBranchArg {
                rejected: text.to_string(),
            })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct UnknownBranchArg {
    rejected: String,
}

impl std::fmt::Display for UnknownBranchArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a step id, yes or no", self.rejected)
    }
}

impl std::error::Error for UnknownBranchArg {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum ShowVersion {
    Working,
    Published,
    Draft,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Properties {
    pub(crate) timezone: Option<Timezone>,
    pub(crate) sharing: Option<Sharing>,
    pub(crate) starter: Option<Starter>,
    pub(crate) add_exit_tags: Vec<Tag>,
    pub(crate) remove_exit_tags: Vec<Tag>,
}

pub(crate) struct ListRequest {
    pub(crate) statuses: Vec<WorkflowStatus>,
    pub(crate) starter_tags: Vec<Tag>,
}

pub(crate) struct ShowRequest {
    pub(crate) version: ShowVersion,
    pub(crate) stats: bool,
}

pub(crate) struct CreateRequest {
    pub(crate) name: WorkflowName,
    pub(crate) from: Option<WorkflowSource>,
    pub(crate) properties: Properties,
}

pub(crate) struct UpdateRequest {
    pub(crate) name: Option<WorkflowName>,
    pub(crate) status: Option<StatusChange>,
    pub(crate) properties: Properties,
}

pub(crate) struct AddStepRequest {
    pub(crate) kind: StepKind,
    pub(crate) automations: Option<Automations>,
    pub(crate) placement: Placement,
    pub(crate) inside_message: Option<MessageId>,
}

#[allow(clippy::large_enum_variant)]
pub(crate) enum StepChange {
    Remove,
    Edit {
        edit: StepEdit,
        placement: Option<Placement>,
    },
}

pub(crate) struct UpdateStepRequest {
    pub(crate) step: RuleId,
    pub(crate) change: StepChange,
}

pub(crate) struct PublishRequest {
    pub(crate) discard: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct AutomationFlags {
    pub(crate) open_applied: Vec<Tag>,
    pub(crate) open_removed: Vec<Tag>,
    pub(crate) open_exit: bool,
    pub(crate) no_open: bool,
    pub(crate) click_applied: Vec<Tag>,
    pub(crate) click_removed: Vec<Tag>,
    pub(crate) click_exit: bool,
    pub(crate) links: Vec<LinkMatch>,
    pub(crate) removed_click_rules: Vec<usize>,
}

const AUTOMATION_FLAGS: [&str; 9] = [
    "when-opened-apply-tag",
    "when-opened-remove-tag",
    "when-opened-exit",
    "no-open-automation",
    "when-clicked-apply-tag",
    "when-clicked-remove-tag",
    "when-clicked-exit",
    "link",
    "remove-click-rule",
];

impl AutomationFlags {
    pub(crate) fn from_matches(
        matches: &clap::ArgMatches,
    ) -> Result<Option<AutomationFlags>, Failure> {
        if !AUTOMATION_FLAGS.iter().any(|flag| named(matches, flag)) {
            return Ok(None);
        }
        Ok(Some(AutomationFlags {
            open_applied: tags(matches, "when-opened-apply-tag"),
            open_removed: tags(matches, "when-opened-remove-tag"),
            open_exit: flag(matches, "when-opened-exit"),
            no_open: flag(matches, "no-open-automation"),
            click_applied: tags(matches, "when-clicked-apply-tag"),
            click_removed: tags(matches, "when-clicked-remove-tag"),
            click_exit: flag(matches, "when-clicked-exit"),
            links: link_matches(matches),
            removed_click_rules: many::<u64>(matches, "remove-click-rule")
                .into_iter()
                .map(|nth| nth as usize)
                .collect(),
        }))
    }

    pub(crate) fn apply(&self, current: &Automations) -> Automations {
        let on_open = if self.no_open {
            None
        } else if self.names_an_open_rule() {
            Some(OpenRule {
                applied: self.open_applied.clone(),
                removed: self.open_removed.clone(),
                exit: self.open_exit,
            })
        } else {
            current.on_open.clone()
        };
        let mut on_click: Vec<ClickRule> = current
            .on_click
            .iter()
            .enumerate()
            .filter(|(index, _)| !self.removed_click_rules.contains(&(index + 1)))
            .map(|(_, rule)| rule.clone())
            .collect();
        if self.names_a_click_rule() {
            on_click.push(ClickRule {
                links: self.links.clone(),
                applied: self.click_applied.clone(),
                removed: self.click_removed.clone(),
                exit: self.click_exit,
            });
        }
        Automations { on_open, on_click }
    }

    fn names_an_open_rule(&self) -> bool {
        !self.open_applied.is_empty() || !self.open_removed.is_empty() || self.open_exit
    }

    fn names_a_click_rule(&self) -> bool {
        !self.click_applied.is_empty() || !self.click_removed.is_empty() || self.click_exit
    }
}

impl std::convert::TryFrom<&clap::ArgMatches> for Properties {
    type Error = Failure;

    fn try_from(matches: &clap::ArgMatches) -> Result<Properties, Failure> {
        let kind = matches.get_one::<StarterKind>("starter").copied();
        let starter_tag = matches.get_one::<Tag>("starter-tag").cloned();
        let starter = match (kind, starter_tag) {
            (Some(StarterKind::NewSubscriber), None) => Some(Starter::NewSubscriber {
                conditions: Vec::new(),
            }),
            (Some(StarterKind::Tag), Some(tag)) => Some(Starter::Tag {
                tags: vec![tag],
                except: Vec::new(),
                reentry: false,
            }),
            (Some(StarterKind::Tag), None) => {
                return Err(Failure::usage("--starter tag requires --starter-tag"));
            }
            (Some(StarterKind::NewSubscriber), Some(_)) | (None, Some(_)) => {
                return Err(Failure::usage("--starter-tag requires --starter tag"));
            }
            (None, None) => None,
        };
        let add_exit_tags = tags(matches, "add-exit-tag");
        let remove_exit_tags = tags(matches, "remove-exit-tag");
        if let Some(both) = add_exit_tags
            .iter()
            .find(|tag| remove_exit_tags.contains(tag))
        {
            return Err(Failure::usage(format!(
                "'{both}' is both added and removed"
            )));
        }
        Ok(Properties {
            timezone: matches.get_one::<Timezone>("timezone").cloned(),
            sharing: matches.get_one::<Sharing>("sharing").copied(),
            starter,
            add_exit_tags,
            remove_exit_tags,
        })
    }
}

impl std::convert::TryFrom<&clap::ArgMatches> for ListRequest {
    type Error = Failure;

    fn try_from(matches: &clap::ArgMatches) -> Result<ListRequest, Failure> {
        Ok(ListRequest {
            statuses: many::<WorkflowStatus>(matches, "status"),
            starter_tags: tags(matches, "starter-tag"),
        })
    }
}

impl std::convert::TryFrom<&clap::ArgMatches> for ShowRequest {
    type Error = Failure;

    fn try_from(matches: &clap::ArgMatches) -> Result<ShowRequest, Failure> {
        let published = flag(matches, "published");
        let draft = flag(matches, "draft");
        if published && draft {
            return Err(Failure::usage(
                "--published cannot be combined with --draft",
            ));
        }
        let version = match (published, draft) {
            (true, _) => ShowVersion::Published,
            (_, true) => ShowVersion::Draft,
            _ => ShowVersion::Working,
        };
        Ok(ShowRequest {
            version,
            stats: !flag(matches, "no-stats"),
        })
    }
}

impl std::convert::TryFrom<&clap::ArgMatches> for CreateRequest {
    type Error = Failure;

    fn try_from(matches: &clap::ArgMatches) -> Result<CreateRequest, Failure> {
        Ok(CreateRequest {
            name: matches
                .get_one::<WorkflowName>("name")
                .cloned()
                .expect("the name positional is required"),
            from: matches.get_one::<WorkflowSource>("from").cloned(),
            properties: Properties::try_from(matches)?,
        })
    }
}

impl std::convert::TryFrom<&clap::ArgMatches> for UpdateRequest {
    type Error = Failure;

    fn try_from(matches: &clap::ArgMatches) -> Result<UpdateRequest, Failure> {
        let name = matches.get_one::<WorkflowName>("name").cloned();
        let status = matches.get_one::<StatusChange>("status").copied();
        let properties = Properties::try_from(matches)?;
        let named = name.is_some()
            || status.is_some()
            || properties.timezone.is_some()
            || properties.sharing.is_some()
            || properties.starter.is_some()
            || !properties.add_exit_tags.is_empty()
            || !properties.remove_exit_tags.is_empty();
        if !named {
            return Err(Failure::usage(
                "nothing to update; pass --name, --status, --timezone, --sharing, \
                 --starter, --starter-tag, --add-exit-tag or --remove-exit-tag",
            ));
        }
        Ok(UpdateRequest {
            name,
            status,
            properties,
        })
    }
}

impl std::convert::TryFrom<&clap::ArgMatches> for AddStepRequest {
    type Error = Failure;

    fn try_from(matches: &clap::ArgMatches) -> Result<AddStepRequest, Failure> {
        let (word, kind_matches) = matches
            .subcommand()
            .expect("clap requires a kind subcommand");
        let name: StepName = word.parse().expect("clap names one of the five kinds");
        let automations = match name {
            StepName::Message | StepName::Feed => AutomationFlags::from_matches(kind_matches)?
                .map(|flags| flags.apply(&Automations::default())),
            _ => None,
        };
        let inside_message = match name {
            StepName::Feed => Some(message_id(kind_matches)),
            _ => None,
        };
        Ok(AddStepRequest {
            kind: added_kind(name, kind_matches)?,
            automations,
            placement: placement(matches)?.unwrap_or(Placement::End),
            inside_message,
        })
    }
}

impl std::convert::TryFrom<&clap::ArgMatches> for UpdateStepRequest {
    type Error = Failure;

    fn try_from(matches: &clap::ArgMatches) -> Result<UpdateStepRequest, Failure> {
        let step = *matches
            .get_one::<RuleId>("step-id")
            .expect("the step-id positional is required");
        let removals = tags(matches, "remove");
        if named(matches, "remove") && removals.is_empty() {
            if REMOVE_CONFLICTS.iter().any(|flag| named(matches, flag)) {
                return Err(Failure::usage(
                    "--remove cannot be combined with any other flag",
                ));
            }
            return Ok(UpdateStepRequest {
                step,
                change: StepChange::Remove,
            });
        }
        let mut edit = StepEdit {
            message: matches.get_one::<MessageId>("message").cloned(),
            timezone_source: matches
                .get_one::<TimezoneSource>("subscriber-timezone")
                .copied(),
            url: matches.get_one::<FeedUrl>("url").cloned(),
            check_every: matches.get_one::<Recurrence>("check-every").cloned(),
            ends: matches
                .get_one::<u32>("check-times")
                .map(|checks| Ends::After { checks: *checks }),
            ..StepEdit::default()
        };
        edit.timing = edited_wait(matches)?;
        if named(matches, "apply") {
            edit.applied = Some(tags(matches, "apply"));
        }
        if named(matches, "remove") {
            edit.removed = Some(removals);
        }
        if named(matches, "when-tagged")
            || named(matches, "when-opened")
            || named(matches, "when-clicked")
        {
            edit.tested = tested(matches);
        }
        Ok(UpdateStepRequest {
            step,
            change: StepChange::Edit {
                edit,
                placement: placement(matches)?,
            },
        })
    }
}

impl std::convert::TryFrom<&clap::ArgMatches> for PublishRequest {
    type Error = Failure;

    fn try_from(matches: &clap::ArgMatches) -> Result<PublishRequest, Failure> {
        Ok(PublishRequest {
            discard: flag(matches, "discard"),
        })
    }
}

const REMOVE_CONFLICTS: [&str; 19] = [
    "message",
    "for",
    "send-on",
    "send-at",
    "subscriber-timezone",
    "apply",
    "url",
    "check-every",
    "when-tagged",
    "when-opened",
    "when-clicked",
    "link",
    "link-contains",
    "when-opened-apply-tag",
    "when-opened-remove-tag",
    "when-opened-exit",
    "no-open-automation",
    "when-clicked-apply-tag",
    "when-clicked-remove-tag",
];

fn message_id(matches: &clap::ArgMatches) -> MessageId {
    matches
        .get_one::<MessageId>("message-id")
        .cloned()
        .expect("clap requires the message id positional")
}

fn added_kind(name: StepName, matches: &clap::ArgMatches) -> Result<StepKind, Failure> {
    Ok(match name {
        StepName::Message => StepKind::Message {
            message: Some(message_id(matches)),
        },
        StepName::Wait => {
            let (timing, timezone_source) = added_wait(matches)?;
            StepKind::Wait {
                timing,
                timezone_source,
                deleted: false,
            }
        }
        StepName::Tag => StepKind::Tag {
            applied: tags(matches, "apply"),
            removed: tags(matches, "remove"),
        },
        StepName::Feed => StepKind::Feed {
            url: Some(
                matches
                    .get_one::<FeedUrl>("url")
                    .cloned()
                    .expect("clap requires --url for a feed step"),
            ),
            check_every: matches
                .get_one::<Recurrence>("check-every")
                .cloned()
                .expect("clap requires --check-every for a feed step")
                .ending(match matches.get_one::<u32>("check-times") {
                    Some(checks) => Ends::After { checks: *checks },
                    None => Ends::Never,
                }),
            inside: Vec::new(),
        },
        StepName::Split => StepKind::Split {
            tested: tested(matches).ok_or_else(|| {
                Failure::usage(format!(
                    "{name} requires --when-tagged, --when-opened or --when-clicked"
                ))
            })?,
            yes: Vec::<Step>::new(),
            no: Vec::<Step>::new(),
        },
    })
}

fn added_wait(matches: &clap::ArgMatches) -> Result<(WaitTiming, TimezoneSource), Failure> {
    let source = matches
        .get_one::<TimezoneSource>("subscriber-timezone")
        .copied()
        .unwrap_or(TimezoneSource::Workflow);
    let duration = matches.get_one::<Delay>("for").copied();
    let days = matches.get_one::<SendDays>("send-on").cloned();
    let at = matches.get_one::<SendTime>("send-at").copied();
    if duration.is_some() && (days.is_some() || at.is_some()) {
        return Err(Failure::usage(
            "--for cannot be combined with --send-on or --send-at",
        ));
    }
    match (duration, days, at) {
        (Some(delay), None, None) => Ok((
            WaitTiming {
                delay: Some(delay),
                schedules: Vec::new(),
            },
            source,
        )),
        (None, Some(days), Some(at)) => Ok((
            WaitTiming {
                delay: None,
                schedules: vec![Recurrence::daily(days, at)],
            },
            source,
        )),
        _ => Err(Failure::usage(
            "a wait step takes --for, or --send-on and --send-at together",
        )),
    }
}

fn edited_wait(matches: &clap::ArgMatches) -> Result<Option<WaitEdit>, Failure> {
    let duration = matches.get_one::<Delay>("for").copied();
    let days = matches.get_one::<SendDays>("send-on").cloned();
    let at = matches.get_one::<SendTime>("send-at").copied();
    if duration.is_some() && (days.is_some() || at.is_some()) {
        return Err(Failure::usage(
            "--for cannot be combined with --send-on or --send-at",
        ));
    }
    Ok(match (duration, days, at) {
        (Some(delay), None, None) => Some(WaitEdit::Timing(WaitTiming {
            delay: Some(delay),
            schedules: Vec::new(),
        })),
        (None, Some(days), Some(at)) => Some(WaitEdit::Timing(WaitTiming {
            delay: None,
            schedules: vec![Recurrence::daily(days, at)],
        })),
        (None, Some(days), None) => Some(WaitEdit::Days(days)),
        (None, None, Some(at)) => Some(WaitEdit::At(at)),
        _ => None,
    })
}

fn tested(matches: &clap::ArgMatches) -> Option<Tested> {
    let tagged = matches
        .try_get_one::<Tag>("when-tagged")
        .ok()
        .flatten()
        .cloned();
    if let Some(tag) = tagged {
        return Some(Tested::HasTag { tags: vec![tag] });
    }
    if flag(matches, "when-opened") {
        return Some(Tested::Opened);
    }
    if !flag(matches, "when-clicked") {
        return None;
    }
    Some(Tested::Clicked {
        link: split_link(matches),
    })
}

fn split_link(matches: &clap::ArgMatches) -> Option<LinkMatch> {
    let fragment = matches
        .try_get_one::<LinkFragment>("link-contains")
        .ok()
        .flatten()
        .cloned();
    match fragment {
        Some(fragment) => Some(LinkMatch::Fragment(fragment)),
        None => link_matches(matches).into_iter().next(),
    }
}

fn placement(matches: &clap::ArgMatches) -> Result<Option<Placement>, Failure> {
    let before = matches.get_one::<RuleId>("before").copied();
    let after = matches.get_one::<RuleId>("after").copied();
    let branch = branch_of(matches);
    let inside = matches.get_one::<RuleId>("inside").copied();
    let named = [
        before.is_some(),
        after.is_some(),
        branch.is_some(),
        inside.is_some(),
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    if named > 1 {
        return Err(Failure::usage(
            "--before, --after, --branch and --inside are mutually exclusive",
        ));
    }
    Ok(match (before, after, branch, inside) {
        (Some(step), _, _, _) => Some(Placement::Before { step }),
        (_, Some(step), _, _) => Some(Placement::After { step }),
        (_, _, Some((split, branch)), _) => Some(Placement::Branch { split, branch }),
        (_, _, _, Some(feed)) => Some(Placement::Inside { feed }),
        _ => None,
    })
}

fn branch_of(matches: &clap::ArgMatches) -> Option<(RuleId, Branch)> {
    let values: Vec<BranchArg> = many(matches, "branch");
    let mut split = None;
    let mut side = None;
    for value in values {
        match value {
            BranchArg::Split(id) => split = Some(id),
            BranchArg::Branch(branch) => side = Some(branch),
        }
    }
    Some((split?, side?))
}

fn link_matches(matches: &clap::ArgMatches) -> Vec<LinkMatch> {
    many::<LinkUrl>(matches, "link")
        .into_iter()
        .map(LinkMatch::Url)
        .collect()
}

fn tags(matches: &clap::ArgMatches, id: &str) -> Vec<Tag> {
    many(matches, id)
}

fn many<T: Clone + Send + Sync + 'static>(matches: &clap::ArgMatches, id: &str) -> Vec<T> {
    matches
        .try_get_many::<T>(id)
        .ok()
        .flatten()
        .map(|values| values.cloned().collect())
        .unwrap_or_default()
}

fn flag(matches: &clap::ArgMatches, id: &str) -> bool {
    matches.try_get_one::<bool>(id).ok().flatten() == Some(&true)
}

fn named(matches: &clap::ArgMatches, id: &str) -> bool {
    if matches.try_contains_id(id).is_err() {
        return false;
    }
    matches.value_source(id) == Some(clap::parser::ValueSource::CommandLine)
}

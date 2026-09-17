use crate::ids::{MessageId, RuleId};
use crate::workflows::values::{
    ChangeCount, Ends, FeedUrl, LinkMatch, Recurrence, SendCadence, SendDays, SendTime, Tag,
    Timezone, TimezoneSource, WaitTiming,
};
use crate::workflows::wire::{self, Body, Event, Position, PreservedRule};

#[derive(Clone, Debug)]
pub struct Graph {
    pub(super) starter: Option<StarterEntry>,
    pub(super) lane: Vec<StepEntry>,
    pub(super) exit: ExitRule,
    pub(super) timezone: Timezone,
    pub(super) reserved: Vec<OwnedRule>,
}

#[derive(Clone, Debug)]
pub(super) struct StepEntry {
    pub(super) step: Step,
    pub(super) rules: Vec<OwnedRule>,
    pub(super) branches: [Vec<StepEntry>; 2],
    pub(super) inside: Vec<StepEntry>,
}

#[derive(Clone, Debug)]
pub(super) struct OwnedRule {
    pub(super) slot: Slot,
    pub(super) rule: PreservedRule,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum Slot {
    Event,
    AutomationEvent,
    Action,
}

#[derive(Clone, Debug)]
pub(super) struct StarterEntry {
    pub(super) starter: Starter,
    pub(super) event: PreservedRule,
}

#[derive(Clone, Debug, Default)]
pub(super) struct ExitRule {
    pub(super) tags: Vec<Tag>,
    pub(super) event: Option<PreservedRule>,
    pub(super) stop: Option<PreservedRule>,
}

impl Graph {
    pub fn starter(&self) -> Option<&Starter> {
        self.starter.as_ref().map(|entry| &entry.starter)
    }

    pub fn steps(&self) -> Vec<Step> {
        filled(&self.lane)
    }

    pub fn step(&self, id: RuleId) -> Option<Step> {
        found(&self.lane, id)
    }

    pub fn exit_tags(&self) -> &[Tag] {
        &self.exit.tags
    }

    pub fn message_cadences(&self) -> std::collections::BTreeMap<MessageId, MessageCadence> {
        let mut cadences: std::collections::BTreeMap<MessageId, MessageCadence> =
            std::collections::BTreeMap::new();
        for (message, cadence) in sends(&self.lane) {
            cadences
                .entry(message)
                .and_modify(|held| {
                    if *held != cadence {
                        *held = MessageCadence::Mixed;
                    }
                })
                .or_insert(cadence);
        }
        cadences
    }

    pub fn set_starter(&mut self, starter: Starter) {
        let event = wire::emit_starter(&starter, self.starter.as_ref().map(|entry| &entry.event));
        self.starter = Some(StarterEntry { starter, event });
    }

    pub fn add_exit_tag(&mut self, tag: Tag) -> TagAdded {
        if self.exit.tags.contains(&tag) {
            return TagAdded::AlreadyPresent;
        }
        self.exit.tags.push(tag);
        self.write_exit();
        TagAdded::Added
    }

    pub fn remove_exit_tag(&mut self, tag: &Tag) -> TagRemoved {
        let held = self.exit.tags.iter().position(|held| held == tag);
        match held {
            None => TagRemoved::Absent,
            Some(index) => {
                self.exit.tags.remove(index);
                self.write_exit();
                TagRemoved::Removed
            }
        }
    }

    pub fn add_step(
        &mut self,
        kind: StepKind,
        automations: Automations,
        at: Placement,
    ) -> Result<RuleId, GraphError> {
        if let Placement::Inside { feed } = at {
            if !matches!(kind, StepKind::Message { .. }) {
                return Err(GraphError::LoopTakesMessages { feed });
            }
        }
        let step = Step {
            id: RuleId::new(),
            kind,
            automations,
        };
        let (lane, index) = self.slot_for(at)?;
        let position = self.position_at(lane.clone(), index);
        let (anchor, rules) = wire::emit_step(&step, &position, &self.timezone.clone());
        let entry = StepEntry {
            step: Step { id: anchor, ..step },
            rules,
            branches: [Vec::new(), Vec::new()],
            inside: Vec::new(),
        };
        self.insert(&lane, index, entry);
        self.reparent(&lane, index + 1);
        Ok(anchor)
    }

    pub fn edit_step(&mut self, id: RuleId, edit: StepEdit) -> Result<(), GraphError> {
        let path = self
            .locate(id)
            .ok_or(GraphError::UnknownStep { step: id })?;
        let entry = self
            .entry_mut(&path)
            .ok_or(GraphError::UnknownStep { step: id })?;
        let mut step = entry.step.clone();
        apply(&mut step, &edit, id)?;
        entry.step = step.clone();
        let mut clicks = 0;
        for owned in entry.rules.iter_mut() {
            let click = match &owned.rule.rule().body {
                Body::Event(Event::Clicked { .. }) => {
                    clicks += 1;
                    Some(clicks - 1)
                }
                _ => None,
            };
            owned.rule = wire::rewrite(&owned.rule, &step, click);
        }
        Ok(())
    }

    pub fn move_step(&mut self, id: RuleId, to: Placement) -> Result<(), GraphError> {
        let path = self
            .locate(id)
            .ok_or(GraphError::UnknownStep { step: id })?;
        if matches!(path.lane, Lane::Inside { .. }) {
            return Err(GraphError::InsideLoop { step: id });
        }
        if let Placement::Inside { feed } = to {
            let entry = self
                .entry(&path)
                .ok_or(GraphError::UnknownStep { step: id })?;
            if !matches!(entry.step.kind, StepKind::Message { .. }) {
                return Err(GraphError::LoopTakesMessages { feed });
            }
        }
        let entry = self
            .take(&path)
            .ok_or(GraphError::UnknownStep { step: id })?;
        self.reparent(&path.lane, path.index);
        let (lane, index) = match self.slot_for(to) {
            Ok(slot) => slot,
            Err(refusal) => {
                self.insert(&path.lane, path.index, entry);
                return Err(refusal);
            }
        };
        let position = self.position_at(lane.clone(), index);
        let mut moved = entry;
        for owned in moved.rules.iter_mut() {
            if owned.slot == Slot::Action && wire::identity(&owned.rule) == id {
                owned.rule = wire::reposition(&owned.rule, &position);
            }
        }
        self.insert(&lane, index, moved);
        self.reparent(&lane, index + 1);
        Ok(())
    }

    pub fn remove_step(&mut self, id: RuleId, published: &Graph) -> Result<(), GraphError> {
        let path = self
            .locate(id)
            .ok_or(GraphError::UnknownStep { step: id })?;
        if matches!(path.lane, Lane::Inside { .. }) {
            return Err(GraphError::InsideLoop { step: id });
        }
        let waits = self
            .entry(&path)
            .is_some_and(|entry| matches!(entry.step.kind, StepKind::Wait { .. }));
        if waits && published.step(id).is_some() {
            let entry = self
                .entry_mut(&path)
                .ok_or(GraphError::UnknownStep { step: id })?;
            if let StepKind::Wait { deleted, .. } = &mut entry.step.kind {
                *deleted = true;
            }
            let step = entry.step.clone();
            for owned in entry.rules.iter_mut() {
                owned.rule = wire::rewrite(&owned.rule, &step, None);
            }
            return Ok(());
        }
        self.take(&path)
            .ok_or(GraphError::UnknownStep { step: id })?;
        self.reparent(&path.lane, path.index);
        Ok(())
    }

    pub fn changes_against(&self, published: &Graph) -> ChangeCount {
        let mine = flattened(&self.lane);
        let theirs = flattened(&published.lane);
        let mut edits = 0;
        for entry in &mine {
            let matched = theirs.iter().find(|other| other.step.id == entry.step.id);
            match matched {
                Some(other) if rules_of(other) == rules_of(entry) => {}
                _ => edits += 1,
            }
        }
        for entry in &theirs {
            if !mine.iter().any(|other| other.step.id == entry.step.id) {
                edits += 1;
            }
        }
        let ordering: Vec<RuleId> = mine.iter().map(|entry| entry.step.id).collect();
        let published_order: Vec<RuleId> = theirs.iter().map(|entry| entry.step.id).collect();
        if ordering != published_order {
            edits += 1;
        }
        if self.starter() != published.starter() {
            edits += 1;
        }
        if self.exit.tags != published.exit.tags {
            edits += 1;
        }
        ChangeCount::over(edits)
    }

    pub fn changed_steps(&self, published: &Graph) -> Vec<Step> {
        let theirs = flattened(&published.lane);
        flattened(&self.lane)
            .into_iter()
            .filter(|entry| {
                let matched = theirs.iter().find(|other| other.step.id == entry.step.id);
                match matched {
                    Some(other) => rules_of(other) != rules_of(entry),
                    None => true,
                }
            })
            .map(|entry| entry.step.clone())
            .collect()
    }

    pub(super) fn rules(&self) -> Vec<OwnedRule> {
        let mut emitted = Vec::new();
        if let Some(starter) = &self.starter {
            emitted.push(OwnedRule {
                slot: Slot::Event,
                rule: starter.event.clone(),
            });
        }
        collect(&self.lane, &mut emitted);
        emitted.extend(self.reserved.iter().cloned());
        emitted.extend(wire::emit_exit(
            &self.exit.tags,
            self.exit.event.as_ref(),
            self.exit.stop.as_ref(),
        ));
        emitted
    }

    fn write_exit(&mut self) {
        let emitted = wire::emit_exit(
            &self.exit.tags,
            self.exit.event.as_ref(),
            self.exit.stop.as_ref(),
        );
        self.exit.event = emitted
            .iter()
            .find(|owned| owned.slot == Slot::Event)
            .map(|owned| owned.rule.clone());
        self.exit.stop = emitted
            .iter()
            .find(|owned| owned.slot == Slot::Action)
            .map(|owned| owned.rule.clone());
    }

    fn slot_for(&self, at: Placement) -> Result<(Lane, usize), GraphError> {
        match at {
            Placement::End => Ok((Lane::Main, self.lane.len())),
            Placement::Before { step } => {
                let path = self.locate(step).ok_or(GraphError::UnknownStep { step })?;
                Ok((path.lane, path.index))
            }
            Placement::After { step } => {
                let path = self.locate(step).ok_or(GraphError::UnknownStep { step })?;
                let forever = self.entry(&path).is_some_and(|entry| {
                    matches!(
                        &entry.step.kind,
                        StepKind::Feed { check_every, .. }
                            if matches!(check_every.ends(), Ends::Never)
                    )
                });
                if forever {
                    return Err(GraphError::AfterForeverLoop { feed: step });
                }
                Ok((path.lane, path.index + 1))
            }
            Placement::Inside { feed } => {
                let path = self
                    .locate(feed)
                    .ok_or(GraphError::UnknownStep { step: feed })?;
                let entry = self
                    .entry(&path)
                    .ok_or(GraphError::UnknownStep { step: feed })?;
                if !matches!(entry.step.kind, StepKind::Feed { .. }) {
                    return Err(GraphError::WrongKind {
                        step: feed,
                        kind: StepName::Feed,
                    });
                }
                let length = entry.inside.len();
                Ok((
                    Lane::Inside {
                        feed,
                        outer: Box::new(path.lane),
                    },
                    length,
                ))
            }
            Placement::Branch { split, branch } => {
                let path = self
                    .locate(split)
                    .ok_or(GraphError::UnknownStep { step: split })?;
                let entry = self
                    .entry(&path)
                    .ok_or(GraphError::UnknownStep { step: split })?;
                if !matches!(entry.step.kind, StepKind::Split { .. }) {
                    return Err(GraphError::NotASplit { step: split });
                }
                let length = entry.branches[side_of(branch)].len();
                Ok((
                    Lane::Branch {
                        split,
                        branch,
                        outer: Box::new(path.lane),
                    },
                    length,
                ))
            }
        }
    }

    fn position_at(&self, lane: Lane, index: usize) -> Position {
        if let Lane::Inside { feed, .. } = &lane {
            let completion = self
                .locate(*feed)
                .and_then(|path| self.entry(&path).cloned())
                .as_ref()
                .and_then(trailing_event);
            return Position {
                parents: completion.into_iter().collect(),
                branch: self.branch_of(&lane),
                cadence: SendCadence::Recurring,
            };
        }
        let entries = self.entries(&lane);
        let mut parents = Vec::new();
        let cadence = SendCadence::Once;
        if let Some(entries) = entries {
            for entry in entries[..index.min(entries.len())].iter().rev() {
                if let Some(id) = trailing_event(entry) {
                    parents.push(id);
                    break;
                }
            }
        }
        if parents.is_empty() {
            if let Some(starter) = &self.starter {
                parents.push(wire::identity(&starter.event));
            }
        }
        Position {
            parents,
            branch: self.branch_of(&lane),
            cadence,
        }
    }

    fn branch_of(&self, lane: &Lane) -> Option<crate::ids::BranchId> {
        let Lane::Branch { split, branch, .. } = lane else {
            return None;
        };
        let path = self.locate(*split)?;
        let entry = self.entry(&path)?;
        wire::declared_branch(&entry.rules, *branch)
    }

    fn reparent(&mut self, lane: &Lane, index: usize) {
        let position = self.position_at(lane.clone(), index);
        let Some(entries) = self.entries_mut(lane) else {
            return;
        };
        let Some(entry) = entries.get_mut(index) else {
            return;
        };
        let anchor = entry.step.id;
        for owned in entry.rules.iter_mut() {
            if owned.slot == Slot::Action && wire::identity(&owned.rule) == anchor {
                owned.rule = wire::reposition(&owned.rule, &position);
            }
        }
    }

    fn insert(&mut self, lane: &Lane, index: usize, entry: StepEntry) {
        if let Some(entries) = self.entries_mut(lane) {
            let at = index.min(entries.len());
            entries.insert(at, entry);
        }
    }

    fn take(&mut self, path: &Path) -> Option<StepEntry> {
        let entries = self.entries_mut(&path.lane)?;
        if path.index >= entries.len() {
            return None;
        }
        Some(entries.remove(path.index))
    }

    fn locate(&self, id: RuleId) -> Option<Path> {
        locate_in(&self.lane, Lane::Main, id)
    }

    fn entry(&self, path: &Path) -> Option<&StepEntry> {
        self.entries(&path.lane)?.get(path.index)
    }

    fn entry_mut(&mut self, path: &Path) -> Option<&mut StepEntry> {
        let index = path.index;
        self.entries_mut(&path.lane)?.get_mut(index)
    }

    fn entries(&self, lane: &Lane) -> Option<&Vec<StepEntry>> {
        match lane {
            Lane::Main => Some(&self.lane),
            Lane::Branch {
                split,
                branch,
                outer,
            } => {
                let outer = self.entries(outer)?;
                let entry = outer.iter().find(|entry| entry.step.id == *split)?;
                entry.branches.get(side_of(*branch))
            }
            Lane::Inside { feed, outer } => {
                let outer = self.entries(outer)?;
                let entry = outer.iter().find(|entry| entry.step.id == *feed)?;
                Some(&entry.inside)
            }
        }
    }

    fn entries_mut(&mut self, lane: &Lane) -> Option<&mut Vec<StepEntry>> {
        match lane {
            Lane::Main => Some(&mut self.lane),
            Lane::Branch {
                split,
                branch,
                outer,
            } => {
                let split = *split;
                let side = side_of(*branch);
                let outer = outer.clone();
                let entries = self.entries_mut(&outer)?;
                let entry = entries.iter_mut().find(|entry| entry.step.id == split)?;
                entry.branches.get_mut(side)
            }
            Lane::Inside { feed, outer } => {
                let feed = *feed;
                let outer = outer.clone();
                let entries = self.entries_mut(&outer)?;
                let entry = entries.iter_mut().find(|entry| entry.step.id == feed)?;
                Some(&mut entry.inside)
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Lane {
    Main,
    Branch {
        split: RuleId,
        branch: Branch,
        outer: Box<Lane>,
    },
    Inside {
        feed: RuleId,
        outer: Box<Lane>,
    },
}

const BRANCHES: [Branch; 2] = [Branch::Yes, Branch::No];

fn side_of(branch: Branch) -> usize {
    usize::from(matches!(branch, Branch::No))
}

#[derive(Clone, Debug)]
struct Path {
    lane: Lane,
    index: usize,
}

fn locate_in(entries: &[StepEntry], lane: Lane, id: RuleId) -> Option<Path> {
    for (index, entry) in entries.iter().enumerate() {
        if entry.step.id == id {
            return Some(Path { lane, index });
        }
        for (branch, entries) in BRANCHES.into_iter().zip(entry.branches.iter()) {
            let inner = Lane::Branch {
                split: entry.step.id,
                branch,
                outer: Box::new(lane.clone()),
            };
            if let Some(path) = locate_in(entries, inner, id) {
                return Some(path);
            }
        }
        let inner = Lane::Inside {
            feed: entry.step.id,
            outer: Box::new(lane.clone()),
        };
        if let Some(path) = locate_in(&entry.inside, inner, id) {
            return Some(path);
        }
    }
    None
}

fn filled(entries: &[StepEntry]) -> Vec<Step> {
    entries.iter().map(fill).collect()
}

fn fill(entry: &StepEntry) -> Step {
    let kind = match &entry.step.kind {
        StepKind::Split { tested, .. } => StepKind::Split {
            tested: tested.clone(),
            yes: filled(&entry.branches[0]),
            no: filled(&entry.branches[1]),
        },
        StepKind::Feed {
            url, check_every, ..
        } => StepKind::Feed {
            url: url.clone(),
            check_every: check_every.clone(),
            inside: filled(&entry.inside),
        },
        kind => kind.clone(),
    };
    Step {
        id: entry.step.id,
        kind,
        automations: entry.step.automations.clone(),
    }
}

fn found(entries: &[StepEntry], id: RuleId) -> Option<Step> {
    for entry in entries {
        if entry.step.id == id {
            return Some(fill(entry));
        }
        for branch in &entry.branches {
            if let Some(step) = found(branch, id) {
                return Some(step);
            }
        }
        if let Some(step) = found(&entry.inside, id) {
            return Some(step);
        }
    }
    None
}

fn flattened(entries: &[StepEntry]) -> Vec<&StepEntry> {
    let mut found = Vec::new();
    for entry in entries {
        found.push(entry);
        for branch in &entry.branches {
            found.extend(flattened(branch));
        }
        found.extend(flattened(&entry.inside));
    }
    found
}

fn collect(entries: &[StepEntry], emitted: &mut Vec<OwnedRule>) {
    for entry in entries {
        emitted.extend(entry.rules.iter().cloned());
        for branch in &entry.branches {
            collect(branch, emitted);
        }
        collect(&entry.inside, emitted);
    }
}

fn rules_of(entry: &StepEntry) -> Vec<&PreservedRule> {
    entry.rules.iter().map(|owned| &owned.rule).collect()
}

fn sends(entries: &[StepEntry]) -> Vec<(MessageId, MessageCadence)> {
    let mut found = Vec::new();
    for entry in entries {
        if let StepKind::Message {
            message: Some(message),
        } = &entry.step.kind
        {
            found.push((message.clone(), cadence_of(entry)));
        }
        for branch in &entry.branches {
            found.extend(sends(branch));
        }
        for inner in sends(&entry.inside) {
            found.push((inner.0, MessageCadence::Recurring));
        }
    }
    found
}

fn cadence_of(entry: &StepEntry) -> MessageCadence {
    match wire::cadence(&entry.rules) {
        SendCadence::Recurring => MessageCadence::Recurring,
        SendCadence::Once => MessageCadence::Once,
    }
}

fn trailing_event(entry: &StepEntry) -> Option<RuleId> {
    entry
        .rules
        .iter()
        .rev()
        .find(|owned| owned.slot == Slot::Event)
        .map(|owned| wire::identity(&owned.rule))
}

fn apply(step: &mut Step, edit: &StepEdit, id: RuleId) -> Result<(), GraphError> {
    if let Some(change) = &edit.message {
        match &mut step.kind {
            StepKind::Message { message } => *message = Some(change.clone()),
            _ => return Err(GraphError::NotAMessage { step: id }),
        }
    }
    if let Some(change) = &edit.timing {
        let StepKind::Wait { timing, .. } = &mut step.kind else {
            return Err(GraphError::WrongKind {
                step: id,
                kind: StepName::Wait,
            });
        };
        match change {
            WaitEdit::Timing(whole) => *timing = whole.clone(),
            WaitEdit::Days(days) => {
                let first = timing
                    .schedules
                    .first_mut()
                    .ok_or(GraphError::NotScheduled { step: id })?;
                *first = first.clone().with_days(days.clone());
            }
            WaitEdit::At(at) => {
                let first = timing
                    .schedules
                    .first_mut()
                    .ok_or(GraphError::NotScheduled { step: id })?;
                *first = first.clone().with_time(*at);
            }
        }
    }
    if let Some(change) = &edit.timezone_source {
        let StepKind::Wait {
            timezone_source, ..
        } = &mut step.kind
        else {
            return Err(GraphError::WrongKind {
                step: id,
                kind: StepName::Wait,
            });
        };
        *timezone_source = *change;
    }
    if edit.applied.is_some() || edit.removed.is_some() {
        let StepKind::Tag { applied, removed } = &mut step.kind else {
            return Err(GraphError::WrongKind {
                step: id,
                kind: StepName::Tag,
            });
        };
        if let Some(listed) = &edit.applied {
            *applied = listed.clone();
        }
        if let Some(listed) = &edit.removed {
            *removed = listed.clone();
        }
    }
    if edit.url.is_some() || edit.check_every.is_some() || edit.ends.is_some() {
        let StepKind::Feed {
            url, check_every, ..
        } = &mut step.kind
        else {
            return Err(GraphError::WrongKind {
                step: id,
                kind: StepName::Feed,
            });
        };
        if let Some(named) = &edit.url {
            *url = Some(named.clone());
        }
        if let Some(every) = &edit.check_every {
            *check_every = every.clone();
        }
        if let Some(ends) = edit.ends {
            *check_every = check_every.clone().ending(ends);
        }
    }
    if let Some(tests) = &edit.tested {
        let StepKind::Split { tested, .. } = &mut step.kind else {
            return Err(GraphError::NotASplit { step: id });
        };
        *tested = tests.clone();
    }
    if let Some(automations) = &edit.automations {
        step.automations = automations.clone();
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Starter {
    NewSubscriber {
        conditions: Vec<Condition>,
    },
    Tag {
        tags: Vec<Tag>,
        except: Vec<Tag>,
        reentry: bool,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    pub field: ConditionField,
    pub test: ConditionTest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConditionField {
    Source,
    AdTracking,
    Country,
    Custom { name: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConditionTest {
    Is { value: String },
    IsNot { value: String },
    Defined,
    Undefined,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Step {
    pub id: RuleId,
    pub kind: StepKind,
    pub automations: Automations,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StepKind {
    Message {
        message: Option<MessageId>,
    },
    Wait {
        timing: WaitTiming,
        timezone_source: TimezoneSource,
        deleted: bool,
    },
    Tag {
        applied: Vec<Tag>,
        removed: Vec<Tag>,
    },
    Feed {
        url: Option<FeedUrl>,
        check_every: Recurrence,
        inside: Vec<Step>,
    },
    Split {
        tested: Tested,
        yes: Vec<Step>,
        no: Vec<Step>,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StepName {
    Message,
    Wait,
    Tag,
    Feed,
    Split,
}

const STEP_NAMES: [StepName; 5] = [
    StepName::Message,
    StepName::Wait,
    StepName::Tag,
    StepName::Feed,
    StepName::Split,
];

impl StepName {
    fn word(self) -> &'static str {
        match self {
            StepName::Message => "message",
            StepName::Wait => "wait",
            StepName::Tag => "tag",
            StepName::Feed => "feed",
            StepName::Split => "split",
        }
    }
}

impl std::str::FromStr for StepName {
    type Err = StepNameError;

    fn from_str(text: &str) -> Result<StepName, StepNameError> {
        STEP_NAMES
            .into_iter()
            .find(|name| name.word() == text)
            .ok_or_else(|| StepNameError {
                rejected: text.to_string(),
            })
    }
}

impl std::convert::TryFrom<&str> for StepName {
    type Error = StepNameError;

    fn try_from(text: &str) -> Result<StepName, StepNameError> {
        text.parse()
    }
}

impl std::fmt::Display for StepName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.word())
    }
}

#[derive(Clone, Debug)]
pub struct StepNameError {
    rejected: String,
}

impl std::fmt::Display for StepNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a step kind", self.rejected)
    }
}

impl std::error::Error for StepNameError {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageCadence {
    Once,
    Recurring,
    Mixed,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TagAdded {
    Added,
    AlreadyPresent,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TagRemoved {
    Removed,
    Absent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tested {
    HasTag { tags: Vec<Tag> },
    Opened,
    Clicked { link: Option<LinkMatch> },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Automations {
    pub on_open: Option<OpenRule>,
    pub on_click: Vec<ClickRule>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenRule {
    pub applied: Vec<Tag>,
    pub removed: Vec<Tag>,
    pub exit: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClickRule {
    pub links: Vec<LinkMatch>,
    pub applied: Vec<Tag>,
    pub removed: Vec<Tag>,
    pub exit: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Placement {
    End,
    Before { step: RuleId },
    After { step: RuleId },
    Branch { split: RuleId, branch: Branch },
    Inside { feed: RuleId },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Branch {
    Yes,
    No,
}

impl std::str::FromStr for Branch {
    type Err = UnknownBranch;

    fn from_str(text: &str) -> Result<Branch, UnknownBranch> {
        match text {
            "yes" => Ok(Branch::Yes),
            "no" => Ok(Branch::No),
            _ => Err(UnknownBranch {
                rejected: text.to_string(),
            }),
        }
    }
}

impl std::fmt::Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Branch::Yes => "yes",
            Branch::No => "no",
        })
    }
}

#[derive(Clone, Debug)]
pub struct UnknownBranch {
    rejected: String,
}

impl std::fmt::Display for UnknownBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a branch", self.rejected)
    }
}

impl std::error::Error for UnknownBranch {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WaitEdit {
    Timing(WaitTiming),
    Days(SendDays),
    At(SendTime),
}

#[derive(Clone, Debug, Default)]
pub struct StepEdit {
    pub message: Option<MessageId>,
    pub timing: Option<WaitEdit>,
    pub timezone_source: Option<TimezoneSource>,
    pub applied: Option<Vec<Tag>>,
    pub removed: Option<Vec<Tag>>,
    pub url: Option<FeedUrl>,
    pub check_every: Option<Recurrence>,
    pub ends: Option<Ends>,
    pub tested: Option<Tested>,
    pub automations: Option<Automations>,
}

#[derive(Clone, Debug)]
pub enum GraphError {
    UnknownStep { step: RuleId },
    NotASplit { step: RuleId },
    NotAMessage { step: RuleId },
    WrongKind { step: RuleId, kind: StepName },
    NotScheduled { step: RuleId },
    InsideLoop { step: RuleId },
    LoopTakesMessages { feed: RuleId },
    AfterForeverLoop { feed: RuleId },
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::UnknownStep { step } => write!(f, "has no step {step}"),
            GraphError::NotASplit { step } => write!(f, "{step} is not a split step"),
            GraphError::NotAMessage { step } => write!(f, "{step} is not a message step"),
            GraphError::WrongKind { step, kind } => {
                write!(f, "{step} is not a {kind} step")
            }
            GraphError::NotScheduled { step } => {
                write!(f, "{step} waits for a duration")
            }
            GraphError::InsideLoop { step } => {
                write!(f, "{step} is inside a feed loop; remove the feed step")
            }
            GraphError::LoopTakesMessages { feed } => {
                write!(f, "{feed} takes only message steps inside its loop")
            }
            GraphError::AfterForeverLoop { feed } => {
                write!(f, "{feed} checks forever; nothing follows it")
            }
        }
    }
}

impl std::error::Error for GraphError {}

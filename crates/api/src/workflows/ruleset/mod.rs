use crate::ids::{BranchId, RuleId};
use crate::workflows::graph::{
    Automations, ClickRule, ExitRule, Graph, OpenRule, OwnedRule, Slot, Starter, StarterEntry,
    Step, StepEntry, StepKind,
};
use crate::workflows::values::Timezone;
use crate::workflows::wire::{Action, Body, Event, PreservedRule};

#[derive(Clone, Debug, Default)]
pub struct Ruleset {
    events: Vec<PreservedRule>,
    actions: Vec<PreservedRule>,
    unpublished_events: Option<Vec<PreservedRule>>,
    unpublished_actions: Option<Vec<PreservedRule>>,
}

impl Ruleset {
    pub fn working(&self, timezone: Timezone) -> Graph {
        let drafted = self.unpublished_events.is_some() || self.unpublished_actions.is_some();
        if !drafted {
            return self.published(timezone);
        }
        let events = self.unpublished_events.clone().unwrap_or_default();
        let actions = self.unpublished_actions.clone().unwrap_or_default();
        project(events.iter().chain(actions.iter()), timezone)
    }

    pub fn published(&self, timezone: Timezone) -> Graph {
        project(self.events.iter().chain(self.actions.iter()), timezone)
    }
}

#[derive(serde::Deserialize)]
struct Slots {
    #[serde(default)]
    events: Option<Vec<PreservedRule>>,
    #[serde(default)]
    actions: Option<Vec<PreservedRule>>,
    #[serde(default)]
    unpublished_events: Option<Vec<PreservedRule>>,
    #[serde(default)]
    unpublished_actions: Option<Vec<PreservedRule>>,
}

impl<'de> serde::Deserialize<'de> for Ruleset {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Ruleset, D::Error> {
        let slots = Slots::deserialize(deserializer)?;
        Ok(Ruleset {
            events: slots.events.unwrap_or_default(),
            actions: slots.actions.unwrap_or_default(),
            unpublished_events: slots.unpublished_events,
            unpublished_actions: slots.unpublished_actions,
        })
    }
}

struct Held<B> {
    id: RuleId,
    parents: Vec<RuleId>,
    branch: Option<BranchId>,
    recurring: bool,
    slot: Slot,
    body: B,
    preserved: PreservedRule,
}

struct Slotted {
    branch: Option<BranchId>,
    entry: StepEntry,
}

type Consumed = std::collections::BTreeSet<RuleId>;

fn project<'a>(rules: impl Iterator<Item = &'a PreservedRule>, timezone: Timezone) -> Graph {
    let mut events: Vec<Held<Event>> = Vec::new();
    let mut actions: Vec<Held<Action>> = Vec::new();
    for preserved in rules {
        let rule = preserved.rule();
        match rule.body.clone() {
            Body::Event(event) => {
                let automation = matches!(event, Event::Opened | Event::Clicked { .. });
                events.push(Held {
                    id: rule.id,
                    parents: rule.parents.clone(),
                    branch: rule.branch,
                    recurring: rule.recurring,
                    slot: if automation {
                        Slot::AutomationEvent
                    } else {
                        Slot::Event
                    },
                    body: event,
                    preserved: preserved.clone(),
                });
            }
            Body::Action(action) => actions.push(Held {
                id: rule.id,
                parents: rule.parents.clone(),
                branch: rule.branch,
                recurring: rule.recurring,
                slot: Slot::Action,
                body: action,
                preserved: preserved.clone(),
            }),
        }
    }

    let mut consumed = Consumed::new();
    let starter = read_starter(&events, &mut consumed);
    let exit = read_exit(&events, &actions, &mut consumed);
    let reserved = read_reserved(&events, &actions, &mut consumed);

    let mut slotted: Vec<Slotted> = Vec::new();
    for event in &events {
        if event.slot == Slot::AutomationEvent || matches!(event.body, Event::Reserved) {
            continue;
        }
        for held in &actions {
            if consumed.contains(&held.id) || !held.parents.contains(&event.id) {
                continue;
            }
            if let Some(entry) = read_step(held, &events, &actions, &mut consumed) {
                slotted.push(Slotted {
                    branch: held.branch,
                    entry,
                });
            }
        }
    }

    Graph {
        starter,
        lane: assemble(slotted),
        exit,
        timezone,
        reserved,
    }
}

fn read_starter(events: &[Held<Event>], consumed: &mut Consumed) -> Option<StarterEntry> {
    let held = events.first()?;
    let starter = match &held.body {
        Event::Subscribe { conditions } => Starter::NewSubscriber {
            conditions: conditions.clone(),
        },
        Event::Tagged {
            tags,
            except,
            reentry,
        } => Starter::Tag {
            tags: tags.clone(),
            except: except.clone(),
            reentry: *reentry,
        },
        Event::WaitComplete { .. }
        | Event::CheckFeedComplete { .. }
        | Event::Opened
        | Event::Clicked { .. }
        | Event::Reserved => return None,
    };
    consumed.insert(held.id);
    Some(StarterEntry {
        starter,
        event: held.preserved.clone(),
    })
}

fn read_exit(
    events: &[Held<Event>],
    actions: &[Held<Action>],
    consumed: &mut Consumed,
) -> ExitRule {
    for held in events {
        if !held.parents.is_empty() || consumed.contains(&held.id) {
            continue;
        }
        let Event::Tagged { tags, .. } = &held.body else {
            continue;
        };
        let stop = children(held.id, actions)
            .into_iter()
            .find(|child| matches!(child.body, Action::Stop));
        consumed.insert(held.id);
        if let Some(stop) = &stop {
            consumed.insert(stop.id);
        }
        return ExitRule {
            tags: tags.clone(),
            event: Some(held.preserved.clone()),
            stop: stop.map(|held| held.preserved.clone()),
        };
    }
    ExitRule::default()
}

fn read_reserved(
    events: &[Held<Event>],
    actions: &[Held<Action>],
    consumed: &mut Consumed,
) -> Vec<OwnedRule> {
    let mut held_rules = Vec::new();
    for held in events {
        if !matches!(held.body, Event::Reserved) {
            continue;
        }
        consumed.insert(held.id);
        held_rules.push(owned(held));
        for child in children(held.id, actions) {
            consumed.insert(child.id);
            held_rules.push(owned(child));
        }
    }
    held_rules
}

fn read_step(
    held: &Held<Action>,
    events: &[Held<Event>],
    actions: &[Held<Action>],
    consumed: &mut Consumed,
) -> Option<StepEntry> {
    let entry = match &held.body {
        Action::Send { message } => {
            consumed.insert(held.id);
            let mut rules = vec![owned(held)];
            let automations = read_automations(held.id, events, actions, consumed, &mut rules);
            StepEntry {
                step: Step {
                    id: held.id,
                    kind: StepKind::Message {
                        message: message.clone(),
                    },
                    automations,
                },
                rules,
                branches: [Vec::new(), Vec::new()],
                inside: Vec::new(),
            }
        }
        Action::Wait {
            correlation,
            timing,
            timezone_source,
            deleted,
        } => {
            consumed.insert(held.id);
            let mut rules = vec![owned(held)];
            let completion = events.iter().find(|event| {
                event.parents.contains(&held.id)
                    && matches!(&event.body, Event::WaitComplete { correlation: named } if named == correlation)
            });
            if let Some(completion) = completion {
                consumed.insert(completion.id);
                rules.push(owned(completion));
            }
            entry(
                held.id,
                StepKind::Wait {
                    timing: timing.clone(),
                    timezone_source: *timezone_source,
                    deleted: *deleted,
                },
                rules,
                Automations::default(),
            )
        }
        Action::CheckFeed {
            correlation,
            check_every,
        } => {
            consumed.insert(held.id);
            let mut rules = vec![owned(held)];
            let completion = events.iter().find(|event| {
                event.parents.contains(&held.id)
                    && matches!(&event.body, Event::CheckFeedComplete { correlation: named, .. } if named == correlation)
            });
            let mut url = None;
            let mut inside: Vec<StepEntry> = Vec::new();
            if let Some(completion) = completion {
                consumed.insert(completion.id);
                rules.push(owned(completion));
                if let Event::CheckFeedComplete { url: named, .. } = &completion.body {
                    url = named.clone();
                }
                for child in children(completion.id, actions) {
                    let Action::Send { message } = &child.body else {
                        continue;
                    };
                    if !child.recurring {
                        continue;
                    }
                    consumed.insert(child.id);
                    let mut owned_rules = vec![owned(child)];
                    let automations =
                        read_automations(child.id, events, actions, consumed, &mut owned_rules);
                    inside.push(entry(
                        child.id,
                        StepKind::Message {
                            message: message.clone(),
                        },
                        owned_rules,
                        automations,
                    ));
                }
            }
            let mut feed = entry(
                held.id,
                StepKind::Feed {
                    url,
                    check_every: check_every.clone(),
                    inside: Vec::new(),
                },
                rules,
                Automations::default(),
            );
            feed.inside = inside;
            feed
        }
        Action::Tag { .. } if automated(held, events) => return None,
        Action::Tag { applied, removed } => {
            consumed.insert(held.id);
            entry(
                held.id,
                StepKind::Tag {
                    applied: applied.clone(),
                    removed: removed.clone(),
                },
                vec![owned(held)],
                Automations::default(),
            )
        }
        Action::SetBranch { tested, .. } => {
            consumed.insert(held.id);
            entry(
                held.id,
                StepKind::Split {
                    tested: tested.clone(),
                    yes: Vec::new(),
                    no: Vec::new(),
                },
                vec![owned(held)],
                Automations::default(),
            )
        }
        Action::Stop => return None,
    };
    Some(entry)
}

fn read_automations(
    anchor: RuleId,
    events: &[Held<Event>],
    actions: &[Held<Action>],
    consumed: &mut Consumed,
    rules: &mut Vec<OwnedRule>,
) -> Automations {
    let mut automations = Automations::default();
    for held in events {
        if !held.parents.contains(&anchor) || consumed.contains(&held.id) {
            continue;
        }
        let links = match &held.body {
            Event::Opened => None,
            Event::Clicked { links } => Some(links.clone()),
            Event::Subscribe { .. }
            | Event::Tagged { .. }
            | Event::WaitComplete { .. }
            | Event::CheckFeedComplete { .. }
            | Event::Reserved => continue,
        };
        consumed.insert(held.id);
        rules.push(owned(held));
        let mut applied = Vec::new();
        let mut removed = Vec::new();
        let mut exit = false;
        for child in children(held.id, actions) {
            consumed.insert(child.id);
            rules.push(owned(child));
            match &child.body {
                Action::Tag {
                    applied: added,
                    removed: dropped,
                } => {
                    applied.extend(added.iter().cloned());
                    removed.extend(dropped.iter().cloned());
                }
                Action::Stop => exit = true,
                Action::Send { .. }
                | Action::Wait { .. }
                | Action::CheckFeed { .. }
                | Action::SetBranch { .. } => {}
            }
        }
        match links {
            None => {
                automations.on_open = Some(OpenRule {
                    applied,
                    removed,
                    exit,
                });
            }
            Some(links) => automations.on_click.push(ClickRule {
                links,
                applied,
                removed,
                exit,
            }),
        }
    }
    automations
}

fn assemble(slotted: Vec<Slotted>) -> Vec<StepEntry> {
    let mut lane: Vec<StepEntry> = Vec::new();
    for held in slotted {
        match held.branch {
            None => lane.push(held.entry),
            Some(branch) => match lane_for(&mut lane, branch) {
                Some(entries) => entries.push(held.entry),
                None => lane.push(held.entry),
            },
        }
    }
    lane
}

fn lane_for(entries: &mut [StepEntry], branch: BranchId) -> Option<&mut Vec<StepEntry>> {
    let owning = entries
        .iter()
        .position(|entry| declared_side(entry, branch).is_some());
    if let Some(index) = owning {
        let entry = entries.get_mut(index)?;
        let side = declared_side(entry, branch)?;
        return entry.branches.get_mut(side);
    }
    for entry in entries.iter_mut() {
        for nested in entry.branches.iter_mut() {
            if let Some(found) = lane_for(nested, branch) {
                return Some(found);
            }
        }
    }
    None
}

fn declared_side(entry: &StepEntry, branch: BranchId) -> Option<usize> {
    let declared = entry
        .rules
        .iter()
        .find_map(|owned| match &owned.rule.rule().body {
            Body::Action(Action::SetBranch { branches, .. }) => Some(branches),
            _ => None,
        })?;
    let at = declared
        .iter()
        .position(|declared| declared.id == Some(branch))?;
    Some(
        match declared.get(at).and_then(|declared| declared.affirms) {
            Some(true) => 0,
            Some(false) => 1,
            None => at,
        },
    )
}

fn children(parent: RuleId, actions: &[Held<Action>]) -> Vec<&Held<Action>> {
    actions
        .iter()
        .filter(|held| held.parents.contains(&parent))
        .collect()
}

fn automated(held: &Held<Action>, events: &[Held<Event>]) -> bool {
    held.parents.iter().any(|parent| {
        events.iter().any(|event| {
            event.id == *parent && matches!(event.body, Event::Opened | Event::Clicked { .. })
        })
    })
}

fn owned<B>(held: &Held<B>) -> OwnedRule {
    OwnedRule {
        slot: held.slot,
        rule: held.preserved.clone(),
    }
}

fn entry(id: RuleId, kind: StepKind, rules: Vec<OwnedRule>, automations: Automations) -> StepEntry {
    StepEntry {
        step: Step {
            id,
            kind,
            automations,
        },
        rules,
        branches: [Vec::new(), Vec::new()],
        inside: Vec::new(),
    }
}

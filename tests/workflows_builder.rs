#![cfg(feature = "workflows")]

use aweber::workflows::{
    Condition, ConditionField, ConditionTest, Ends, Graph, Recurrence, Starter, Step, StepKind,
    Tag, Tested, Timezone, TimezoneSource, Workflow,
};

fn fixture(name: &str) -> Graph {
    let path = format!(
        "{}/tests/fixtures/builder/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let ruleset: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{path} reads")),
    )
    .unwrap_or_else(|_| panic!("{path} holds a ruleset"));
    let workflow: Workflow = serde_json::from_value(serde_json::json!({
        "id": "33333333-3333-4333-8333-333333333333",
        "timezone": "America/New_York",
        "ruleset": ruleset,
    }))
    .expect("a workflow document");
    workflow
        .ruleset()
        .expect("every rule is one the builder writes")
        .working(Timezone::utc())
}

fn kinds(steps: &[Step]) -> Vec<String> {
    steps.iter().map(kind_of).collect()
}

fn kind_of(step: &Step) -> String {
    match &step.kind {
        StepKind::Message { .. } => "message".to_string(),
        StepKind::Wait { .. } => "wait".to_string(),
        StepKind::Tag { .. } => "tag".to_string(),
        StepKind::Feed { .. } => "feed".to_string(),
        StepKind::Split { yes, no, .. } => format!(
            "split(yes: [{}], no: [{}])",
            kinds(yes).join(", "),
            kinds(no).join(", ")
        ),
    }
}

fn days_of(recurrence: &Recurrence) -> String {
    recurrence.days().expect("days").to_string()
}

fn time_of(recurrence: &Recurrence) -> String {
    recurrence.at().expect("a time").to_string()
}

fn tag(word: &str) -> Tag {
    word.parse().expect("a tag")
}

fn tags(words: &[&str]) -> Vec<Tag> {
    words.iter().map(|word| tag(word)).collect()
}

#[test]
fn a_new_subscriber_lane_reads_as_a_wait_a_message_and_a_tag() {
    let graph = fixture("new_subscriber_lane");
    let steps = graph.steps();
    assert_eq!(kinds(&steps), ["wait", "message", "tag"]);
    assert_eq!(
        graph.starter(),
        Some(&Starter::NewSubscriber {
            conditions: Vec::new()
        })
    );
    assert!(graph.exit_tags().is_empty());
    let StepKind::Wait { timing, .. } = &steps[0].kind else {
        panic!("the first step is a wait");
    };
    assert_eq!(timing.delay.expect("a delay").to_string(), "2d");
    assert!(timing.schedules.is_empty());
    let StepKind::Tag { applied, removed } = &steps[2].kind else {
        panic!("the third step is a tag");
    };
    assert_eq!(*applied, tags(&["welcomed"]));
    assert_eq!(*removed, tags(&["cold"]));
}

#[test]
fn a_subscribe_starter_carries_every_condition_the_builder_wrote() {
    let graph = fixture("subscribe_conditions");
    assert_eq!(kinds(&graph.steps()), ["message"]);
    assert_eq!(
        graph.starter(),
        Some(&Starter::NewSubscriber {
            conditions: vec![
                Condition {
                    field: ConditionField::Source,
                    test: ConditionTest::Is {
                        value: "webform".to_string()
                    },
                },
                Condition {
                    field: ConditionField::Country,
                    test: ConditionTest::Is {
                        value: "US".to_string()
                    },
                },
                Condition {
                    field: ConditionField::Custom {
                        name: "plan".to_string()
                    },
                    test: ConditionTest::IsNot {
                        value: "trial".to_string()
                    },
                },
            ]
        })
    );
    assert!(graph.exit_tags().is_empty());
}

#[test]
fn a_tag_starter_carries_its_tags_its_except_tags_and_reentry() {
    let graph = fixture("tag_starter");
    assert_eq!(kinds(&graph.steps()), ["message"]);
    assert_eq!(
        graph.starter(),
        Some(&Starter::Tag {
            tags: tags(&["vip", "gold"]),
            except: tags(&["churned"]),
            reentry: true,
        })
    );
    assert!(graph.exit_tags().is_empty());
}

#[test]
fn an_exit_rule_owns_its_tag_event_and_its_stop() {
    let graph = fixture("exit_tags");
    assert_eq!(kinds(&graph.steps()), ["message"]);
    assert_eq!(
        graph.starter(),
        Some(&Starter::NewSubscriber {
            conditions: Vec::new()
        })
    );
    assert_eq!(graph.exit_tags(), tags(&["gone", "bounced"]));
}

#[test]
fn a_feed_loop_reads_as_one_feed_owning_the_sends_automations() {
    let graph = fixture("feed_loop");
    let steps = graph.steps();
    assert_eq!(kinds(&steps), ["feed"]);
    assert_eq!(
        graph.starter(),
        Some(&Starter::NewSubscriber {
            conditions: Vec::new()
        })
    );
    assert!(graph.exit_tags().is_empty());
    let StepKind::Feed {
        url,
        check_every,
        inside,
    } = &steps[0].kind
    else {
        panic!("the only step is a feed");
    };
    assert_eq!(kinds(inside), ["message"]);
    assert_eq!(
        url.as_ref().map(ToString::to_string),
        Some("https://example.com/feed.xml".to_string())
    );
    assert_eq!(check_every.to_string(), "1h");
    let automations = &inside[0].automations;
    let on_open = automations.on_open.as_ref().expect("an open rule");
    assert_eq!(on_open.applied, tags(&["opened"]));
    assert!(on_open.removed.is_empty());
    assert!(!on_open.exit);
    assert_eq!(automations.on_click.len(), 1);
    assert_eq!(
        automations.on_click[0]
            .links
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>(),
        ["https://example.com/post"]
    );
    assert!(automations.on_click[0].exit);
}

#[test]
fn a_tag_split_owns_the_steps_of_both_branches() {
    let graph = fixture("tag_split");
    let steps = graph.steps();
    assert_eq!(
        kinds(&steps),
        ["split(yes: [message], no: [tag])".to_string()]
    );
    assert_eq!(
        graph.starter(),
        Some(&Starter::NewSubscriber {
            conditions: Vec::new()
        })
    );
    assert!(graph.exit_tags().is_empty());
    let StepKind::Split { tested, .. } = &steps[0].kind else {
        panic!("the only step is a split");
    };
    assert_eq!(
        *tested,
        Tested::HasTag {
            tags: tags(&["vip", "gold"])
        }
    );
}

#[test]
fn a_split_inside_a_branch_keeps_its_own_branches() {
    let graph = fixture("nested_split");
    let steps = graph.steps();
    assert_eq!(
        kinds(&steps),
        ["split(yes: [split(yes: [message], no: [])], no: [tag])".to_string()]
    );
    let StepKind::Split { tested, yes, .. } = &steps[0].kind else {
        panic!("the only step is a split");
    };
    assert_eq!(*tested, Tested::Opened);
    let StepKind::Split { tested, .. } = &yes[0].kind else {
        panic!("the yes branch holds a split");
    };
    assert_eq!(
        *tested,
        Tested::HasTag {
            tags: tags(&["vip"])
        }
    );
    assert!(graph.exit_tags().is_empty());
}

#[test]
fn a_message_the_builder_has_not_chosen_reads_as_a_message_of_none() {
    let graph = fixture("draft_message");
    let steps = graph.steps();
    assert_eq!(kinds(&steps), ["message"]);
    assert_eq!(steps[0].kind, StepKind::Message { message: None });
    assert_eq!(
        graph.starter(),
        Some(&Starter::NewSubscriber {
            conditions: Vec::new()
        })
    );
    assert!(graph.exit_tags().is_empty());
}

#[test]
fn a_wait_carries_its_delay_its_schedules_and_its_timezone_source() {
    let graph = fixture("scheduled_wait");
    let steps = graph.steps();
    assert_eq!(kinds(&steps), ["wait"]);
    let StepKind::Wait {
        timing,
        timezone_source,
        ..
    } = &steps[0].kind
    else {
        panic!("the only step is a wait");
    };
    assert_eq!(timing.delay.expect("a delay").to_string(), "3d");
    assert_eq!(timing.schedules.len(), 2);
    assert_eq!(days_of(&timing.schedules[0]), "mon,wed");
    assert_eq!(time_of(&timing.schedules[0]), "09:30");
    assert_eq!(days_of(&timing.schedules[1]), "sat");
    assert_eq!(time_of(&timing.schedules[1]), "12:00");
    assert_eq!(*timezone_source, TimezoneSource::Subscriber);
    assert!(graph.exit_tags().is_empty());
}

#[test]
fn a_finite_feed_loop_owns_its_sends_and_the_tag_after_it_follows() {
    let graph = fixture("finite_feed_loop");
    let steps = graph.steps();
    assert_eq!(kinds(&steps), ["feed", "tag"]);
    let StepKind::Feed {
        check_every,
        inside,
        ..
    } = &steps[0].kind
    else {
        panic!("the first step is a feed");
    };
    assert_eq!(kinds(inside), ["message", "message"]);
    assert!(inside[0].automations.on_open.is_none());
    assert_eq!(
        inside[1]
            .automations
            .on_open
            .as_ref()
            .expect("an open rule")
            .applied,
        tags(&["opened"])
    );
    assert_eq!(check_every.ends(), Ends::After { checks: 10 });
}

#[test]
fn a_removed_published_wait_reads_as_deleted() {
    let path = format!(
        "{}/tests/fixtures/builder/deleted_wait.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let ruleset: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("the fixture reads"))
            .expect("the fixture holds a ruleset");
    let workflow: Workflow = serde_json::from_value(serde_json::json!({
        "id": "33333333-3333-4333-8333-333333333333",
        "timezone": "America/New_York",
        "ruleset": ruleset,
    }))
    .expect("a workflow document");
    let ruleset = workflow.ruleset().expect("every rule reads");
    let working = ruleset.working(Timezone::utc());
    let published = ruleset.published(Timezone::utc());
    assert_eq!(kinds(&working.steps()), ["message", "wait", "message"]);
    assert_eq!(kinds(&published.steps()), ["message", "wait", "message"]);
    let StepKind::Wait { deleted, .. } = &working.steps()[1].kind else {
        panic!("the second step is a wait");
    };
    assert!(deleted, "the working copy holds the wait as deleted");
    let StepKind::Wait { deleted, .. } = &published.steps()[1].kind else {
        panic!("the second step is a wait");
    };
    assert!(!deleted, "the published version holds it undeleted");
}

#[test]
fn reserved_events_and_their_actions_are_held_and_emitted_unchanged() {
    let graph = fixture("reserved_event");
    let steps = graph.steps();
    assert_eq!(kinds(&steps), ["message"]);
    assert!(steps[0].automations.on_open.is_none());
    assert!(steps[0].automations.on_click.is_empty());
    let patch = aweber::workflows::WorkflowPatch::default()
        .with_ruleset(aweber::workflows::PatchOperation::Replace, &graph);
    let text = serde_json::to_value(&patch)
        .expect("a patch writes back")
        .to_string();
    assert!(text.contains("message_not_opened.v1"), "{text}");
    assert!(text.contains("link_not_clicked.v1"), "{text}");
    assert!(text.contains("ignored"), "{text}");
}

#[test]
fn a_wait_with_a_delay_and_two_schedules_reads_both() {
    let graph = fixture("legacy_wait");
    let steps = graph.steps();
    assert_eq!(kinds(&steps), ["wait"]);
    let StepKind::Wait { timing, .. } = &steps[0].kind else {
        panic!("the only step is a wait");
    };
    assert_eq!(timing.delay.expect("a delay").to_string(), "2d");
    assert_eq!(timing.schedules.len(), 2);
    assert_eq!(days_of(&timing.schedules[0]), "mon,wed");
    assert_eq!(time_of(&timing.schedules[0]), "09:00");
    assert_eq!(days_of(&timing.schedules[1]), "sat");
    assert_eq!(time_of(&timing.schedules[1]), "17:30");
}

#[test]
fn recurrence_parts_the_builder_can_write_round_trip() {
    let graph = fixture("custom_recurrence");
    let steps = graph.steps();
    assert_eq!(kinds(&steps), ["wait", "feed"]);
    let StepKind::Wait { timing, .. } = &steps[0].kind else {
        panic!("the first step is a wait");
    };
    assert_eq!(timing.delay.expect("a delay").to_string(), "1h");
    assert_eq!(
        timing.schedules[0]
            .day_of_month()
            .expect("a day of the month")
            .to_string(),
        "last"
    );
    assert_eq!(time_of(&timing.schedules[0]), "08:15");
    let StepKind::Feed { check_every, .. } = &steps[1].kind else {
        panic!("the second step is a feed");
    };
    assert_eq!(
        check_every.ends(),
        Ends::Until {
            day: "2026-12-31".parse().expect("a date")
        }
    );
    assert_eq!(days_of(check_every), "tue");
}

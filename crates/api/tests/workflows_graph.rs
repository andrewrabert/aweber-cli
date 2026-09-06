#![cfg(feature = "workflows")]

use aweber::workflows::{
    Automations, Graph, GraphError, MessageCadence, Placement, SendDays, StepEdit, StepKind,
    StepName, Tag, Tested, Timezone, WaitEdit, Workflow,
};

const STARTER: &str = "e0000000-0000-4000-8000-000000000001";
const WAIT_ACTION: &str = "a0000000-0000-4000-8000-000000000001";
const WAIT_EVENT: &str = "e0000000-0000-4000-8000-000000000002";
const FEED_ACTION: &str = "a0000000-0000-4000-8000-000000000005";
const FEED_EVENT: &str = "e0000000-0000-4000-8000-000000000003";
const SEND_ACTION: &str = "a0000000-0000-4000-8000-000000000002";
const LOOP_SEND: &str = "a0000000-0000-4000-8000-000000000009";
const CORRELATION: &str = "c0000000-0000-4000-8000-000000000001";
const MESSAGE: &str = "aaaaaaaaaaaaaaaaaaaaaaaa";
const SPLIT_ACTION: &str = "a0000000-0000-4000-8000-00000000000a";
const YES_BRANCH: &str = "b0000000-0000-4000-8000-000000000001";
const NO_BRANCH: &str = "b0000000-0000-4000-8000-000000000002";
const MEMOIZE: &str = "d0000000-0000-4000-8000-000000000001";

fn workflow(document: serde_json::Value) -> Workflow {
    serde_json::from_value(document).expect("a workflow document")
}

fn graph(events: serde_json::Value, actions: serde_json::Value) -> Graph {
    workflow(serde_json::json!({
        "id": "33333333-3333-4333-8333-333333333333",
        "timezone": "America/New_York",
        "ruleset": { "events": events, "actions": actions },
    }))
    .ruleset()
    .expect("the ruleset reads")
    .working(Timezone::utc())
}

fn subscribe_starter(id: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "parents": [],
        "type": "subscribe.v1",
        "filter": null,
        "title": null,
        "metadata": "subscribe.v1",
    })
}

fn send(id: &str, parent: &str, message: &str, recurring: bool) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "parents": [parent],
        "definition": {
            "function": "ruleset.email.action.compose_v1",
            "kwargs": {
                "account": "<event:account>",
                "list": "<event:list>",
                "meapi_id": message,
                "message": format!("<message:new message={message}>"),
                "recipient": "<event:recipient>",
            },
        },
        "title": null,
        "metadata": "send-message",
        "recurring": recurring,
    })
}

fn wait(
    id: &str,
    event_id: &str,
    parent: &str,
    delay: &str,
    schedule: &[&str],
) -> [serde_json::Value; 2] {
    [
        serde_json::json!({
            "id": id,
            "parents": [parent],
            "definition": {
                "function": "ruleset.schedule.action.wait_v1",
                "kwargs": {
                    "id": CORRELATION,
                    "account": "<event:account>",
                    "list": "<event:list>",
                    "recipient": "<event:recipient>",
                    "ruleset": "<ruleset>",
                    "delay": delay,
                    "rrules": schedule,
                    "timezone": "America/New_York",
                    "action": "<action>",
                    "use_subscriber_timezone": false,
                },
            },
            "title": null,
            "metadata": "wait",
            "recurring": false,
        }),
        serde_json::json!({
            "id": event_id,
            "parents": [id],
            "type": "wait_complete.v1",
            "filter": {
                "type": "any",
                "criteria": [{
                    "function": "rulesengine.filter.event_value",
                    "operator": "==",
                    "expect": CORRELATION,
                    "kwargs": { "key": "id" },
                }],
            },
            "title": null,
            "metadata": "wait_complete.v1",
            "recurring": false,
        }),
    ]
}

fn check_feed(id: &str, event_id: &str, parent: &str, message: &str) -> [serde_json::Value; 3] {
    [
        serde_json::json!({
            "id": id,
            "parents": [parent],
            "definition": {
                "function": "ruleset.schedule.action.wait_v1",
                "kwargs": {
                    "id": CORRELATION,
                    "account": "<event:account>",
                    "list": "<event:list>",
                    "recipient": "<event:recipient>",
                    "ruleset": "<ruleset>",
                    "delay": "PT0S",
                    "rrules": ["FREQ=DAILY;COUNT=5475;BYDAY=SU;BYHOUR=8;BYMINUTE=0"],
                    "timezone": "America/New_York",
                    "action": "<action>",
                    "use_subscriber_timezone": false,
                },
            },
            "title": null,
            "metadata": "check-feed",
        }),
        serde_json::json!({
            "id": event_id,
            "parents": [id],
            "type": "wait_complete.v1",
            "filter": {
                "type": "all",
                "criteria": [
                    {
                        "function": "rulesengine.filter.event_value",
                        "operator": "==",
                        "expect": CORRELATION,
                        "kwargs": { "key": "id" },
                    },
                    {
                        "function": "ruleset.rss.filter.state_changed_v1",
                        "operator": "==",
                        "expect": true,
                        "kwargs": {
                            "account": "<event:account>",
                            "list": "<event:list>",
                            "subscriber": "<event:subscriber>",
                            "ruleset": "<ruleset>",
                            "url": "https://example.com/feed.xml",
                            "categories": [],
                            "min_new_items": 1,
                        },
                    },
                ],
            },
            "title": null,
            "metadata": "check-feed-complete",
            "recurring": true,
        }),
        send(LOOP_SEND, event_id, message, true),
    ]
}

fn tag_split(id: &str, parent: &str, tag: &str) -> serde_json::Value {
    let branch = |branch_id: &str, expect: bool| {
        serde_json::json!({
            "type": "all",
            "criteria": [{
                "function": "ruleset.tag.filter.any_tags_v1",
                "operator": "==",
                "expect": expect,
                "kwargs": {
                    "account": "<event:account>",
                    "list": "<event:list>",
                    "recipient": "<event:recipient>",
                    "labels": [tag],
                },
                "memoize_id": MEMOIZE,
            }],
            "title": null,
            "metadata": null,
            "id": branch_id,
        })
    };
    serde_json::json!({
        "id": id,
        "parents": [parent],
        "definition": {
            "function": "rulesengine.action.set_branch",
            "kwargs": {
                "ruleset": "<ruleset>",
                "branches": [branch(YES_BRANCH, true), branch(NO_BRANCH, false)],
            },
        },
        "title": null,
        "metadata": "set-branch",
    })
}

fn branched(mut rule: serde_json::Value, branch: &str) -> serde_json::Value {
    rule["branch"] = serde_json::json!(branch);
    rule
}

#[test]
fn a_message_sent_in_and_out_of_a_feed_loop_reads_as_mixed() {
    let [feed, feed_event, looped] = check_feed(FEED_ACTION, FEED_EVENT, STARTER, MESSAGE);
    let projected = graph(
        serde_json::json!([subscribe_starter(STARTER), feed_event]),
        serde_json::json!([send(SEND_ACTION, STARTER, MESSAGE, false), feed, looped]),
    );
    let cadences = projected.message_cadences();
    let message = MESSAGE.parse().expect("24 hex digits");
    assert_eq!(cadences.get(&message), Some(&MessageCadence::Mixed));
}

#[test]
fn a_tag_split_before_any_message_reads_as_a_split_on_that_tag() {
    let projected = graph(
        serde_json::json!([subscribe_starter(STARTER)]),
        serde_json::json!([
            tag_split(SPLIT_ACTION, STARTER, "everything-hunky-dory"),
            branched(send(SEND_ACTION, STARTER, MESSAGE, false), YES_BRANCH),
        ]),
    );
    let steps = projected.steps();
    assert_eq!(steps.len(), 1);
    let StepKind::Split { tested, yes, no } = &steps[0].kind else {
        panic!("a tag split reads as a split, not {:?}", steps[0].kind);
    };
    let tag: Tag = "everything-hunky-dory".parse().expect("a tag");
    assert_eq!(*tested, Tested::HasTag { tags: vec![tag] });
    assert_eq!(yes.len(), 1, "the true branch owns the send");
    assert!(no.is_empty());
}

#[test]
fn a_wait_with_two_schedules_reads_as_one_wait_of_two_schedules() {
    let [action, event] = wait(
        WAIT_ACTION,
        WAIT_EVENT,
        STARTER,
        "PT0S",
        &[
            "FREQ=DAILY;BYDAY=MO;BYHOUR=9;BYMINUTE=0",
            "FREQ=DAILY;BYDAY=TU;BYHOUR=9;BYMINUTE=0",
        ],
    );
    let projected = graph(
        serde_json::json!([subscribe_starter(STARTER), event]),
        serde_json::json!([action]),
    );
    let steps = projected.steps();
    assert_eq!(steps.len(), 1);
    let StepKind::Wait { timing, .. } = &steps[0].kind else {
        panic!("a wait reads as a wait, not {:?}", steps[0].kind);
    };
    assert_eq!(timing.delay, None, "a zero delay is no delay");
    assert_eq!(timing.schedules.len(), 2);
    assert_eq!(timing.schedules[0].days().expect("days").to_string(), "mon");
    assert_eq!(timing.schedules[1].days().expect("days").to_string(), "tue");
    assert_eq!(
        timing.schedules[0].at().expect("a time").to_string(),
        "09:00"
    );
    assert_eq!(steps[0].id.to_string(), WAIT_ACTION);
}

#[test]
fn a_partial_schedule_edit_of_a_duration_wait_is_not_scheduled() {
    let [action, event] = wait(WAIT_ACTION, WAIT_EVENT, STARTER, "P2D", &[]);
    let mut projected = graph(
        serde_json::json!([subscribe_starter(STARTER), event]),
        serde_json::json!([action]),
    );
    let step = projected.steps()[0].id;
    let days: SendDays = "sun".parse().expect("a day set");
    let refusal = projected.edit_step(
        step,
        StepEdit {
            timing: Some(WaitEdit::Days(days)),
            ..StepEdit::default()
        },
    );
    assert!(matches!(
        refusal,
        Err(aweber::workflows::GraphError::NotScheduled { .. })
    ));
}

fn feed_graph() -> Graph {
    let [feed, feed_event, looped] = check_feed(FEED_ACTION, FEED_EVENT, STARTER, MESSAGE);
    graph(
        serde_json::json!([subscribe_starter(STARTER), feed_event]),
        serde_json::json!([feed, looped]),
    )
}

fn wait_graph() -> Graph {
    let [action, event] = wait(WAIT_ACTION, WAIT_EVENT, STARTER, "P2D", &[]);
    graph(
        serde_json::json!([subscribe_starter(STARTER), event]),
        serde_json::json!([action]),
    )
}

#[test]
fn a_message_added_inside_a_feed_loop_is_recurring_under_its_completion() {
    let mut projected = feed_graph();
    let feed = projected.steps()[0].id;
    let message = projected
        .add_step(
            StepKind::Message {
                message: Some(MESSAGE.parse().expect("24 hex digits")),
            },
            Automations::default(),
            Placement::Inside { feed },
        )
        .expect("a message goes inside a feed loop");
    let StepKind::Feed { inside, .. } = &projected.steps()[0].kind else {
        panic!("the only step is a feed");
    };
    assert_eq!(inside.len(), 2);
    assert_eq!(inside[1].id, message);
    assert_eq!(
        projected.message_cadences().values().next(),
        Some(&MessageCadence::Recurring)
    );
}

#[test]
fn nothing_follows_a_forever_feed() {
    let mut projected = feed_graph();
    let feed = projected.steps()[0].id;
    let refusal = projected.add_step(
        StepKind::Tag {
            applied: Vec::new(),
            removed: Vec::new(),
        },
        Automations::default(),
        Placement::After { step: feed },
    );
    assert!(matches!(refusal, Err(GraphError::AfterForeverLoop { .. })));
}

#[test]
fn a_feed_loop_takes_only_messages() {
    let mut projected = feed_graph();
    let feed = projected.steps()[0].id;
    let refusal = projected.add_step(
        StepKind::Tag {
            applied: Vec::new(),
            removed: Vec::new(),
        },
        Automations::default(),
        Placement::Inside { feed },
    );
    assert!(matches!(refusal, Err(GraphError::LoopTakesMessages { .. })));
    assert_eq!(StepName::Feed.to_string(), "feed");
}

#[test]
fn a_wait_cannot_be_moved_into_a_feed_loop() {
    let [wait_action, wait_event] = wait(WAIT_ACTION, WAIT_EVENT, STARTER, "P2D", &[]);
    let [feed, feed_event, looped] = check_feed(FEED_ACTION, FEED_EVENT, WAIT_EVENT, MESSAGE);
    let mut projected = graph(
        serde_json::json!([subscribe_starter(STARTER), wait_event, feed_event]),
        serde_json::json!([wait_action, feed, looped]),
    );
    let before: Vec<_> = projected
        .steps()
        .iter()
        .map(|step| format!("{:?}", step.kind))
        .collect();
    let wait_id = projected.steps()[0].id;
    let feed_id = projected.steps()[1].id;
    let refusal = projected.move_step(wait_id, Placement::Inside { feed: feed_id });
    assert!(matches!(
        refusal,
        Err(GraphError::LoopTakesMessages { feed }) if feed == feed_id
    ));
    let after: Vec<_> = projected
        .steps()
        .iter()
        .map(|step| format!("{:?}", step.kind))
        .collect();
    assert_eq!(before, after, "the refused move leaves the wait in place");
}

#[test]
fn removing_a_published_wait_keeps_it_deleted() {
    let published = wait_graph();
    let mut working = wait_graph();
    let step = working.steps()[0].id;
    working
        .remove_step(step, &published)
        .expect("a published wait is removed");
    let StepKind::Wait { deleted, .. } = &working.steps()[0].kind else {
        panic!("the wait is kept until publish");
    };
    assert!(deleted, "a published wait is kept with deleted set");

    let mut drafted = wait_graph();
    let step = drafted.steps()[0].id;
    drafted
        .remove_step(step, &graph(serde_json::json!([]), serde_json::json!([])))
        .expect("an unpublished wait is dropped");
    assert!(drafted.steps().is_empty());
}

#[test]
fn a_message_inside_a_loop_is_removed_with_its_feed() {
    let mut projected = feed_graph();
    let StepKind::Feed { inside, .. } = &projected.steps()[0].kind else {
        panic!("the only step is a feed");
    };
    let step = inside[0].id;
    let published = feed_graph();
    let refusal = projected.remove_step(step, &published);
    assert!(matches!(refusal, Err(GraphError::InsideLoop { .. })));
}

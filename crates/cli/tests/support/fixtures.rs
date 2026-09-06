use serde_json::{json, Value};

pub const STARTER: &str = "e0000000-0000-4000-8000-000000000001";
pub const WAIT_ACTION: &str = "a0000000-0000-4000-8000-000000000001";
pub const WAIT_EVENT: &str = "e0000000-0000-4000-8000-000000000002";
pub const CORRELATION: &str = "c0000000-0000-4000-8000-000000000001";
pub const SEND_ACTION: &str = "a0000000-0000-4000-8000-000000000002";
const SECOND_SEND: &str = "a0000000-0000-4000-8000-000000000009";
pub const TAG_ACTION: &str = "a0000000-0000-4000-8000-000000000003";
pub const SPLIT_ACTION: &str = "a0000000-0000-4000-8000-000000000004";
pub const FEED_ACTION: &str = "a0000000-0000-4000-8000-000000000005";
pub const FEED_EVENT: &str = "e0000000-0000-4000-8000-000000000003";
pub const OPEN_EVENT: &str = "e0000000-0000-4000-8000-000000000004";
pub const CLICK_EVENT: &str = "e0000000-0000-4000-8000-000000000005";
pub const OPEN_TAG: &str = "a0000000-0000-4000-8000-000000000006";
pub const CLICK_STOP: &str = "a0000000-0000-4000-8000-000000000007";
pub const EXIT_EVENT: &str = "e0000000-0000-4000-8000-000000000006";
pub const EXIT_STOP: &str = "a0000000-0000-4000-8000-000000000008";
pub const YES_BRANCH: &str = "b0000000-0000-4000-8000-000000000001";
pub const NO_BRANCH: &str = "b0000000-0000-4000-8000-000000000002";
pub const MEMOIZE: &str = "d0000000-0000-4000-8000-000000000001";
pub const MESSAGE: &str = "aaaaaaaaaaaaaaaaaaaaaaaa";
pub const SECOND_MESSAGE: &str = "bbbbbbbbbbbbbbbbbbbbbbbb";
pub const TIMEZONE: &str = "America/New_York";

pub fn document(ruleset: Value) -> Value {
    json!({
        "id": super::WORKFLOW,
        "name": "Welcome",
        "parent": super::LIST,
        "state": "draft",
        "timezone": TIMEZONE,
        "sharing_enabled": true,
        "updated_at": "2026-08-01T00:00:00Z",
        "precondition_version": 12,
        "ruleset": ruleset,
    })
}

pub fn published(events: Value, actions: Value) -> Value {
    json!({ "events": events, "actions": actions })
}

pub fn with_unpublished(published: Value, events: Value, actions: Value) -> Value {
    let mut ruleset = published;
    let object = ruleset.as_object_mut().expect("the published pair");
    object.insert("unpublished_events".into(), events);
    object.insert("unpublished_actions".into(), actions);
    ruleset
}

pub fn subscribe_starter(id: &str) -> Value {
    json!({
        "id": id,
        "parents": [],
        "type": "subscribe.v1",
        "filter": null,
        "title": null,
        "metadata": "subscribe.v1",
    })
}

pub fn tag_starter(id: &str, tag: &str) -> Value {
    json!({
        "id": id,
        "parents": [],
        "type": "tag.v1",
        "filter": {
            "type": "all",
            "criteria": [
                {
                    "function": "rulesengine.filter.event_value_in",
                    "operator": "==",
                    "expect": true,
                    "kwargs": { "key": "label", "values": [tag] },
                },
                {
                    "function": "ruleset.tag.filter.any_tags_v1",
                    "operator": "==",
                    "expect": true,
                    "kwargs": {
                        "account": "<event:account>",
                        "list": "<event:list>",
                        "recipient": "<event:recipient>",
                        "labels": [tag],
                    },
                },
            ],
        },
        "title": null,
        "metadata": "tag.v1",
    })
}

pub fn send(id: &str, parent: &str, message: &str, recurring: bool) -> Value {
    json!({
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

pub fn wait(id: &str, event_id: &str, parent: &str, delay: &str, schedule: &[&str]) -> [Value; 2] {
    [
        json!({
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
                    "timezone": TIMEZONE,
                    "action": "<action>",
                    "use_subscriber_timezone": false,
                },
            },
            "title": null,
            "metadata": "wait",
            "recurring": false,
        }),
        json!({
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

pub fn tag(id: &str, parent: &str, applied: &[&str], removed: &[&str]) -> Value {
    json!({
        "id": id,
        "parents": [parent],
        "definition": {
            "function": "ruleset.tag.action.modify_tags_v1",
            "kwargs": {
                "account": "<event:account>",
                "list": "<event:list>",
                "recipient": "<subscriber>",
                "add_labels": applied,
                "remove_labels": removed,
            },
        },
        "title": null,
        "metadata": "tag",
        "recurring": false,
    })
}

pub fn check_feed(
    id: &str,
    event_id: &str,
    parent: &str,
    url: &str,
    rule: &str,
    messages: &[(&str, &str)],
) -> Vec<Value> {
    let mut rules = vec![
        json!({
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
                    "rrules": [rule],
                    "timezone": TIMEZONE,
                    "action": "<action>",
                    "use_subscriber_timezone": false,
                },
            },
            "title": null,
            "metadata": "check-feed",
        }),
        json!({
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
                            "url": url,
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
    ];
    for (send_id, message) in messages {
        rules.push(send(send_id, event_id, message, true));
    }
    rules
}

pub fn deleted_wait(id: &str, event_id: &str, parent: &str, delay: &str) -> [Value; 2] {
    let [mut action, event] = wait(id, event_id, parent, delay, &[]);
    action
        .as_object_mut()
        .expect("a rule object")
        .insert("is_deleted".into(), json!(true));
    [action, event]
}

pub fn split(id: &str, parent: &str, function: &str, opened: bool) -> Value {
    let criterion = |expect: bool| {
        json!({
            "function": if opened {
                "ruleset.analytics.filter.any_message_opens_v1"
            } else {
                "ruleset.analytics.filter.any_message_clicks_v1"
            },
            "operator": "==",
            "expect": expect,
            "kwargs": {
                "account": "<event:account>",
                "subscriber": "<event:subscriber>",
                "messages": "<messages:sent>",
            },
            "memoize_id": MEMOIZE,
        })
    };
    json!({
        "id": id,
        "parents": [parent],
        "definition": {
            "function": function,
            "kwargs": {
                "branches": [
                    { "type": "all", "id": YES_BRANCH, "criteria": [criterion(true)] },
                    { "type": "all", "id": NO_BRANCH, "criteria": [criterion(false)] },
                ],
                "ruleset": "<ruleset>",
            },
        },
        "title": null,
        "metadata": "set-branch",
    })
}

pub fn open_automation(id: &str, tag_id: &str, send_id: &str, message: &str) -> [Value; 2] {
    [
        json!({
            "id": id,
            "parents": [send_id],
            "type": "open.v2",
            "filter": {
                "type": "all",
                "criteria": [{
                    "function": "rulesengine.filter.event_value",
                    "operator": "==",
                    "expect": format!("<message:new message={message}>"),
                    "kwargs": { "key": "message" },
                }],
            },
            "title": null,
            "metadata": "open.v2",
            "recurring": false,
        }),
        tag(tag_id, id, &["opened"], &[]),
    ]
}

pub fn click_automation(
    id: &str,
    stop_id: &str,
    send_id: &str,
    message: &str,
    urls: &[&str],
) -> [Value; 2] {
    [
        json!({
            "id": id,
            "parents": [send_id],
            "type": "click.v2",
            "filter": {
                "type": "all",
                "criteria": [
                    {
                        "function": "rulesengine.filter.event_value",
                        "operator": "==",
                        "expect": format!("<message:new message={message}>"),
                        "kwargs": { "key": "message" },
                    },
                    {
                        "function": "rulesengine.filter.event_value_in_urls",
                        "operator": "==",
                        "expect": true,
                        "kwargs": { "key": "destination", "urls": urls },
                    },
                ],
            },
            "title": null,
            "metadata": "click.v2",
            "recurring": false,
        }),
        stop(stop_id, id, true),
    ]
}

pub fn stop(id: &str, parent: &str, recurring: bool) -> Value {
    let mut rule = json!({
        "id": id,
        "parents": [parent],
        "definition": {
            "function": "rulesengine.action.stop",
            "kwargs": { "ruleset": "<ruleset>", "subscriber": "<subscriber>" },
        },
        "title": null,
        "metadata": "stop",
    });
    if recurring {
        rule.as_object_mut()
            .expect("a rule object")
            .insert("recurring".into(), json!(true));
    }
    rule
}

pub fn exit_on_tag(id: &str, stop_id: &str, tags: &[&str]) -> [Value; 2] {
    [
        json!({
            "id": id,
            "parents": [],
            "type": "tag.v1",
            "filter": {
                "type": "all",
                "criteria": [{
                    "function": "rulesengine.filter.event_value_in",
                    "operator": "==",
                    "expect": true,
                    "kwargs": { "key": "label", "values": tags },
                }],
            },
            "title": null,
            "metadata": "tag.v1",
        }),
        stop(stop_id, id, false),
    ]
}

pub fn draft_send(id: &str, parent: &str) -> Value {
    json!({
        "id": id,
        "parents": [parent],
        "definition": {
            "function": "ruleset.email.action.compose_v1",
            "kwargs": {
                "account": "<event:account>",
                "list": "<event:list>",
                "meapi_id": null,
                "message": "<message:new>",
                "recipient": "<event:recipient>",
            },
        },
        "title": null,
        "metadata": "send-message",
        "recurring": false,
    })
}

pub fn lane_ruleset() -> Value {
    let [wait_action, wait_event] = wait(WAIT_ACTION, WAIT_EVENT, STARTER, "P2D", &[]);
    published(
        json!([subscribe_starter(STARTER), wait_event]),
        json!([wait_action, send(SEND_ACTION, WAIT_EVENT, MESSAGE, false)]),
    )
}

pub fn scheduled_wait_ruleset() -> Value {
    let [wait_action, wait_event] = wait(
        WAIT_ACTION,
        WAIT_EVENT,
        STARTER,
        "PT0S",
        &["FREQ=DAILY;BYDAY=MO,WE;BYHOUR=9;BYMINUTE=30"],
    );
    published(
        json!([subscribe_starter(STARTER), wait_event]),
        json!([wait_action]),
    )
}

pub fn branching_ruleset() -> Value {
    published(
        json!([subscribe_starter(STARTER)]),
        json!([
            send(SEND_ACTION, STARTER, MESSAGE, false),
            split(SPLIT_ACTION, STARTER, "rulesengine.action.set_branch", true),
            branched(tag(TAG_ACTION, STARTER, &["yes"], &[]), YES_BRANCH),
        ]),
    )
}

pub fn set_branch_v1_ruleset() -> Value {
    published(
        json!([subscribe_starter(STARTER)]),
        json!([
            send(SEND_ACTION, STARTER, MESSAGE, false),
            split(
                SPLIT_ACTION,
                STARTER,
                "ruleset.schedule.action.set_branch_v1",
                false,
            ),
        ]),
    )
}

pub fn feed_ruleset() -> Value {
    let rules = check_feed(
        FEED_ACTION,
        FEED_EVENT,
        STARTER,
        "https://example.com/feed.xml",
        "FREQ=DAILY;BYDAY=SU,MO,TU,WE,TH,FR,SA;BYHOUR=8;BYMINUTE=0;COUNT=5475",
        &[(SEND_ACTION, MESSAGE)],
    );
    published(
        json!([subscribe_starter(STARTER), rules[1]]),
        json!([rules[0], rules[2]]),
    )
}

pub fn mixed_cadence_ruleset() -> Value {
    let rules = check_feed(
        FEED_ACTION,
        FEED_EVENT,
        STARTER,
        "https://example.com/feed.xml",
        "FREQ=DAILY;BYDAY=SU,MO,TU,WE,TH,FR,SA;BYHOUR=8;BYMINUTE=0;COUNT=5475",
        &[(SEND_ACTION, MESSAGE)],
    );
    published(
        json!([subscribe_starter(STARTER), rules[1]]),
        json!([
            send(SECOND_SEND, STARTER, MESSAGE, false),
            rules[0],
            rules[2],
        ]),
    )
}

pub fn automations_ruleset() -> Value {
    let [opened, opened_tag] = open_automation(OPEN_EVENT, OPEN_TAG, SEND_ACTION, MESSAGE);
    let [clicked, clicked_stop] = click_automation(
        CLICK_EVENT,
        CLICK_STOP,
        SEND_ACTION,
        MESSAGE,
        &["https://example.com/page"],
    );
    published(
        json!([subscribe_starter(STARTER), opened, clicked]),
        json!([
            send(SEND_ACTION, STARTER, MESSAGE, false),
            opened_tag,
            clicked_stop,
        ]),
    )
}

pub fn tagged_ruleset() -> Value {
    let [exit_event, exit_stop] = exit_on_tag(EXIT_EVENT, EXIT_STOP, &["gone"]);
    published(
        json!([tag_starter(STARTER, "vip"), exit_event]),
        json!([send(SEND_ACTION, STARTER, MESSAGE, false), exit_stop]),
    )
}

pub fn draft_message_ruleset() -> Value {
    published(
        json!([subscribe_starter(STARTER)]),
        json!([draft_send(SEND_ACTION, STARTER)]),
    )
}

pub fn multi_schedule_wait_ruleset() -> Value {
    let [wait_action, wait_event] = wait(
        WAIT_ACTION,
        WAIT_EVENT,
        STARTER,
        "PT0S",
        &[
            "FREQ=DAILY;BYDAY=MO;BYHOUR=9;BYMINUTE=0",
            "FREQ=DAILY;BYDAY=TU;BYHOUR=9;BYMINUTE=0",
        ],
    );
    published(
        json!([subscribe_starter(STARTER), wait_event]),
        json!([wait_action]),
    )
}

pub fn draft_patch(op: &str, events: Value, actions: Value) -> Value {
    json!([
        { "op": op, "path": "/ruleset/unpublished_actions", "value": actions },
        { "op": op, "path": "/ruleset/unpublished_events", "value": events },
    ])
}

fn branched(rule: Value, branch: &str) -> Value {
    let mut rule = rule;
    rule.as_object_mut()
        .expect("a rule object")
        .insert("branch".into(), json!(branch));
    rule
}

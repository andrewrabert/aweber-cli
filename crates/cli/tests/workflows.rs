mod support;

use support::fixtures;
use support::{Harness, LIST_NAME, WORKFLOW};

fn workflow_route() -> String {
    format!("/internal/campaign/campaigns/{WORKFLOW}")
}

#[tokio::test]
async fn list_emits_one_object_of_workflows_newest_first() {
    let harness = Harness::start().await;
    harness.with_list(LIST_NAME, support::LIST).await;
    harness
        .with_workflows(serde_json::json!({
            "entries": [
                {
                    "id": "33333333-3333-4333-8333-333333333333",
                    "name": "Older",
                    "state": "draft",
                    "updated_at": "2026-01-01T00:00:00Z",
                },
                {
                    "id": "44444444-4444-4444-8444-444444444444",
                    "name": "Newer",
                    "state": "active",
                    "updated_at": "2026-08-01T00:00:00Z",
                },
            ],
        }))
        .await;

    let output = harness
        .command()
        .args(["workflows", "list", "--list", LIST_NAME])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let document = Harness::json(&output);
    let rows = document["workflows"].as_array().expect("an array");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["name"], "Newer");
    assert_eq!(rows[1]["name"], "Older");
    assert_eq!(rows[0]["unpublished_changes"], 0);
}

#[tokio::test]
async fn show_emits_about_steps_and_exit_tags() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .with_subjects(serde_json::json!({
            "messages": [{ "id": fixtures::MESSAGE, "subject": "Hello" }],
        }))
        .await;
    harness
        .accept(
            "GET",
            &format!(
                "/internal/analytics-view/reports/campaign-message-stats/{}",
                fixtures::MESSAGE
            ),
            serde_json::json!({
                "total_sent": 10,
                "total_opens": 6,
                "unique_opens": 5,
                "total_clicks": 3,
                "unique_clicks": 2,
                "total_bounces": 1,
            }),
        )
        .await;

    let output = harness
        .command()
        .args(["workflows", "show", WORKFLOW])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let document = Harness::json(&output);
    assert_eq!(document["about"]["name"], "Welcome");
    assert_eq!(document["about"]["status"], "draft");
    assert_eq!(document["about"]["errors"], serde_json::json!([]));
    assert_eq!(document["about"]["starter"]["kind"], "new-subscriber");
    assert_eq!(document["exit_tags"], serde_json::json!([]));
    let steps = document["steps"].as_array().expect("an array");
    assert_eq!(steps.len(), 2);
    assert_eq!(steps[0]["kind"], "wait");
    assert_eq!(steps[0]["duration"], "2d");
    assert_eq!(steps[0]["timezone_source"], "workflow");
    assert_eq!(steps[1]["kind"], "message");
    assert_eq!(steps[1]["message"]["subject"], "Hello");
}

#[tokio::test]
async fn show_reports_sends_opens_open_rate_clicks_click_rate_and_bounces() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;
    harness
        .accept(
            "GET",
            &format!(
                "/internal/analytics-view/reports/campaign-message-stats/{}",
                fixtures::MESSAGE
            ),
            serde_json::json!({
                "total_sent": 4,
                "total_opens": 3,
                "unique_opens": 2,
                "total_clicks": 2,
                "unique_clicks": 1,
                "total_bounces": 1,
            }),
        )
        .await;

    let output = harness
        .command()
        .args(["workflows", "show", WORKFLOW])
        .output()
        .expect("the binary runs");
    let document = Harness::json(&output);
    let stats = &document["steps"][1]["stats"];
    assert_eq!(stats["sends"], 4);
    assert_eq!(stats["opens"], 3);
    assert_eq!(stats["open_rate"], 0.5);
    assert_eq!(stats["clicks"], 2);
    assert_eq!(stats["click_rate"], 0.25);
    assert_eq!(stats["bounces"], 1);
    assert_eq!(document["steps"][0]["stats"], serde_json::Value::Null);
}

#[tokio::test]
async fn show_of_a_never_sent_message_emits_a_null_stats_member() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;
    harness
        .accept(
            "GET",
            &format!(
                "/internal/analytics-view/reports/campaign-message-stats/{}",
                fixtures::MESSAGE
            ),
            serde_json::json!({ "total_sent": 0 }),
        )
        .await;

    let output = harness
        .command()
        .args(["workflows", "show", WORKFLOW])
        .output()
        .expect("the binary runs");
    let document = Harness::json(&output);
    assert_eq!(document["steps"][1]["stats"], serde_json::Value::Null);
}

#[tokio::test]
async fn show_no_stats_omits_every_stats_field() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let output = harness
        .command()
        .args(["workflows", "show", WORKFLOW, "--no-stats"])
        .output()
        .expect("the binary runs");
    let document = Harness::json(&output);
    for step in document["steps"].as_array().expect("an array") {
        assert!(step.get("stats").is_none(), "{step}");
    }
    assert_eq!(
        harness
            .calls(
                "GET",
                &format!(
                    "/internal/analytics-view/reports/campaign-message-stats/{}",
                    fixtures::MESSAGE
                )
            )
            .await,
        0
    );
}

#[tokio::test]
async fn show_published_reads_the_published_version() {
    let harness = Harness::start().await;
    let ruleset = fixtures::with_unpublished(
        fixtures::lane_ruleset(),
        serde_json::json!([fixtures::subscribe_starter(fixtures::STARTER)]),
        serde_json::json!([]),
    );
    harness.with_workflow(fixtures::document(ruleset)).await;
    harness.with_subjects(serde_json::json!({})).await;
    harness
        .accept(
            "GET",
            &format!(
                "/internal/analytics-view/reports/campaign-message-stats/{}",
                fixtures::MESSAGE
            ),
            serde_json::json!({ "total_sent": 0 }),
        )
        .await;

    let published = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--published"])
            .output()
            .expect("the binary runs"),
    );
    assert_eq!(published["steps"].as_array().expect("an array").len(), 2);

    let working = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW])
            .output()
            .expect("the binary runs"),
    );
    assert_eq!(working["steps"].as_array().expect("an array").len(), 0);
}

#[tokio::test]
async fn show_draft_without_unpublished_changes_exits_1() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let output = harness
        .command()
        .args(["workflows", "show", WORKFLOW, "--draft"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        format!("error: {WORKFLOW} has no unpublished changes")
    );
}

#[tokio::test]
async fn show_published_with_draft_exits_2() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;

    let output = harness
        .command()
        .args(["workflows", "show", WORKFLOW, "--published", "--draft"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        "error: --published cannot be combined with --draft"
    );
}

#[tokio::test]
async fn show_reads_a_boolean_expect_split_and_a_tagged_starter() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::branching_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let output = harness
        .command()
        .args(["workflows", "show", WORKFLOW, "--no-stats"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let document = Harness::json(&output);
    let steps = document["steps"].as_array().expect("an array");
    let split = steps
        .iter()
        .find(|step| step["kind"] == "split")
        .expect("a split step");
    assert_eq!(split["tested"]["on"], "opened");
    assert!(split["tested"].get("step").is_none(), "{split}");
}

#[tokio::test]
async fn show_reads_a_branch_scoped_step_off_its_branch_field() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::branching_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    let split = document["steps"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|step| step["kind"] == "split")
        .expect("a split step")
        .clone();
    assert_eq!(split["yes"].as_array().expect("an array").len(), 1);
    assert_eq!(split["yes"][0]["kind"], "tag");
    assert_eq!(split["no"], serde_json::json!([]));
}

#[tokio::test]
async fn show_of_an_unknown_server_status_emits_a_null_status() {
    let harness = Harness::start().await;
    let mut document = fixtures::document(fixtures::lane_ruleset());
    document["state"] = serde_json::json!("hibernating");
    harness.with_workflow(document).await;
    harness.with_subjects(serde_json::json!({})).await;

    let emitted = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    assert_eq!(emitted["about"]["status"], serde_json::Value::Null);
}

#[tokio::test]
async fn show_of_a_duration_wait_emits_duration_and_timezone_source() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    let wait = &document["steps"][0];
    assert_eq!(wait["duration"], "2d");
    assert_eq!(wait["timezone_source"], "workflow");
    assert_eq!(wait["deleted"], false);
    assert_eq!(wait["schedules"], serde_json::json!([]));
}

#[tokio::test]
async fn show_of_a_scheduled_wait_emits_send_days_send_at_and_timezone_source() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::scheduled_wait_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    let wait = &document["steps"][0];
    assert_eq!(
        wait["schedules"][0]["days"],
        serde_json::json!(["mon", "wed"])
    );
    assert_eq!(wait["schedules"][0]["at"], "09:30");
    assert_eq!(wait["deleted"], false);
    assert_eq!(wait["timezone_source"], "workflow");
    assert_eq!(wait["duration"], serde_json::Value::Null);
}

#[tokio::test]
async fn show_of_a_wait_with_two_schedules_emits_both_schedules() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::multi_schedule_wait_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW])
            .output()
            .expect("the binary runs"),
    );
    let steps = document["steps"].as_array().expect("an array");
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0]["kind"], "wait");
    let schedules = steps[0]["schedules"].as_array().expect("an array");
    assert_eq!(schedules.len(), 2);
    assert_eq!(schedules[0]["days"], serde_json::json!(["mon"]));
    assert_eq!(schedules[1]["days"], serde_json::json!(["tue"]));
}

#[tokio::test]
async fn show_of_a_feed_step_emits_its_url_and_interval() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::feed_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    let feed = &document["steps"][0];
    assert_eq!(feed["kind"], "feed");
    assert_eq!(feed["url"], "https://example.com/feed.xml");
    assert_eq!(feed["check_every"]["every"], "1d");
    assert_eq!(feed["check_every"]["ends"], "never");
    assert_eq!(feed["inside"][0]["kind"], "message");
}

#[tokio::test]
async fn show_of_a_message_step_reports_its_automations() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::automations_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    let automations = &document["steps"][0]["automations"];
    assert_eq!(
        automations["on_open"]["applied"],
        serde_json::json!(["opened"])
    );
    assert_eq!(automations["on_click"][0]["exit"], true);
    assert_eq!(
        automations["on_click"][0]["links"],
        serde_json::json!(["https://example.com/page"])
    );
}

#[tokio::test]
async fn show_of_a_tagged_workflow_reports_its_starter_and_exit_tags() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::tagged_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    assert_eq!(document["about"]["starter"]["kind"], "tag");
    assert_eq!(
        document["about"]["starter"]["tags"],
        serde_json::json!(["vip"])
    );
    assert_eq!(document["exit_tags"], serde_json::json!(["gone"]));
}

#[tokio::test]
async fn a_name_without_list_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "show", "Welcome"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        "error: 'Welcome' is not a workflow id, so --list is required"
    );
}

#[tokio::test]
async fn an_unmatched_name_exits_1() {
    let harness = Harness::start().await;
    harness.with_list(LIST_NAME, support::LIST).await;
    harness
        .with_workflows(serde_json::json!({ "entries": [] }))
        .await;

    let output = harness
        .command()
        .args(["workflows", "show", "Missing", "--list", LIST_NAME])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        format!(
            "error: no workflow named 'Missing' on list {}",
            support::LIST
        )
    );
}

#[tokio::test]
async fn an_ambiguous_name_exits_1() {
    let harness = Harness::start().await;
    harness.with_list(LIST_NAME, support::LIST).await;
    harness
        .with_workflows(serde_json::json!({
            "entries": [
                { "id": "33333333-3333-4333-8333-333333333333", "name": "Twin" },
                { "id": "44444444-4444-4444-8444-444444444444", "name": "Twin" },
            ],
        }))
        .await;

    let output = harness
        .command()
        .args(["workflows", "show", "Twin", "--list", LIST_NAME])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        "error: 2 workflows are named 'Twin'; pass the workflow id instead"
    );
}

#[tokio::test]
async fn a_malformed_flag_value_exits_2_naming_the_flag() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "show", WORKFLOW, "--list"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));

    let output = harness
        .command()
        .args(["workflows", "add-step", WORKFLOW, "wait", "--for", "1d12h"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--for"), "{stderr}");
}

#[tokio::test]
async fn a_credentials_file_written_before_the_account_rename_still_loads() {
    let harness = Harness::start_with_stale_credentials().await;
    harness.with_list(LIST_NAME, support::LIST).await;
    harness
        .with_workflows(serde_json::json!({ "entries": [] }))
        .await;

    let output = harness
        .command()
        .args(["workflows", "list", "--list", LIST_NAME])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let document = Harness::json(&output);
    assert_eq!(document["workflows"], serde_json::json!([]));
}

#[tokio::test]
async fn the_group_help_carries_the_unsupported_api_notice() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "--help"])
        .output()
        .expect("the binary runs");
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(
        help.contains(
            "These commands use an undocumented, unversioned and unsupported API surface \
             that can change or disappear without notice."
        ),
        "{help}"
    );
}

#[tokio::test]
async fn the_group_help_names_eight_commands() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "--help"])
        .output()
        .expect("the binary runs");
    let help = String::from_utf8_lossy(&output.stdout);
    for named in [
        "list",
        "show",
        "create",
        "update",
        "add-step",
        "update-step",
        "publish",
        "delete",
    ] {
        assert!(help.contains(named), "{named} is missing from {help}");
    }
}

#[tokio::test]
async fn no_retired_spelling_resolves_as_a_command() {
    let harness = Harness::start().await;
    for retired in [
        "get",
        "tree",
        "stats",
        "message-stats",
        "event-history",
        "update-ruleset",
        "validate",
        "revert",
        "set-state",
        "copy",
        "copy-details",
    ] {
        let output = harness
            .command()
            .args(["workflows", retired, WORKFLOW])
            .output()
            .expect("the binary runs");
        assert_eq!(
            output.status.code(),
            Some(2),
            "{retired} still resolves: {output:?}"
        );
    }
}

#[tokio::test]
async fn no_command_of_the_group_takes_a_page_size_flag() {
    let harness = Harness::start().await;
    for action in [
        "list",
        "show",
        "create",
        "update",
        "add-step",
        "update-step",
        "publish",
        "delete",
    ] {
        let output = harness
            .command()
            .args(["workflows", action, "--help"])
            .output()
            .expect("the binary runs");
        let help = String::from_utf8_lossy(&output.stdout);
        assert!(!help.contains("--page-size"), "{action}: {help}");
        assert!(!help.contains("--limit"), "{action}: {help}");
    }
}

#[tokio::test]
async fn create_emits_the_workflow_object() {
    let harness = Harness::start().await;
    harness.with_list(LIST_NAME, support::LIST).await;
    harness
        .expect(
            "POST",
            "/internal/campaign/campaigns",
            serde_json::json!({
                "name": "Fresh",
                "owner": support::ACCOUNT,
                "parent": support::LIST,
            }),
            fixtures::document(fixtures::published(
                serde_json::json!([]),
                serde_json::json!([]),
            )),
        )
        .await;

    let output = harness
        .command()
        .args(["workflows", "create", "Fresh", "--list", LIST_NAME])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let document = Harness::json(&output);
    assert_eq!(document["id"], WORKFLOW);
    assert_eq!(document["list"], support::LIST);
    assert_eq!(document["sharing"]["code"], WORKFLOW);
}

#[tokio::test]
async fn create_from_duplicates_the_source_and_applies_property_flags() {
    let harness = Harness::start().await;
    harness.with_list(LIST_NAME, support::LIST).await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .expect(
            "POST",
            &format!("{}/copy", workflow_route()),
            serde_json::json!({
                "list": support::LIST,
                "name": "Copy",
            }),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args([
            "workflows",
            "create",
            "Copy",
            "--list",
            LIST_NAME,
            "--from",
            WORKFLOW,
            "--sharing",
            "yes",
        ])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let patches = harness.bodies("PATCH", &workflow_route()).await;
    assert_eq!(
        patches,
        vec![serde_json::json!([
            { "op": "replace", "path": "/sharing_enabled", "value": true },
        ])]
    );
}

#[tokio::test]
async fn update_without_flags_exits_2_listing_the_flags() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        "error: nothing to update; pass --name, --status, --timezone, --sharing, \
         --starter, --starter-tag, --add-exit-tag or --remove-exit-tag"
    );
}

#[tokio::test]
async fn update_status_active_on_an_unpublished_draft_exits_1() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW, "--status", "active"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        format!("error: {WORKFLOW} has never been published")
    );
    assert_eq!(harness.calls("PATCH", &workflow_route()).await, 0);
}

#[tokio::test]
async fn update_status_asserting_the_current_status_makes_no_call() {
    let harness = Harness::start().await;
    let mut document = fixtures::document(fixtures::lane_ruleset());
    document["state"] = serde_json::json!("paused");
    harness.with_workflow(document).await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW, "--status", "paused"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(harness.calls("PATCH", &workflow_route()).await, 0);
    let emitted = Harness::json(&output);
    assert_eq!(emitted["status"], "paused");
}

#[tokio::test]
async fn update_status_naming_no_status_at_all_exits_2_without_naming_one() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW, "--status", "hibernating"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("hibernating"), "{stderr}");
    for status in ["active", "paused", "draining", "stopped"] {
        assert!(!stderr.contains(status), "{stderr}");
    }
}

#[tokio::test]
async fn update_status_naming_an_unsettable_status_exits_2_naming_it() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW, "--status", "archived"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("archived"), "{stderr}");
}

#[tokio::test]
async fn update_timezone_sends_update_times_with_the_timezone_op() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args([
            "workflows",
            "update",
            WORKFLOW,
            "--timezone",
            "Europe/Berlin",
        ])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        harness.bodies("PATCH", &workflow_route()).await,
        vec![serde_json::json!([
            { "op": "replace", "path": "/timezone", "value": "Europe/Berlin" },
        ])]
    );
}

#[tokio::test]
async fn update_the_same_tag_added_and_removed_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args([
            "workflows",
            "update",
            WORKFLOW,
            "--add-exit-tag",
            "gone",
            "--remove-exit-tag",
            "gone",
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr.trim(), "error: 'gone' is both added and removed");
}

#[tokio::test]
async fn update_starter_tag_without_its_kind_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW, "--starter-tag", "vip"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr.trim(), "error: --starter-tag requires --starter tag");
}

#[tokio::test]
async fn update_starter_tag_without_the_tag_flag_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW, "--starter", "tag"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr.trim(), "error: --starter tag requires --starter-tag");
}

#[tokio::test]
async fn update_an_unrecognized_starter_value_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW, "--starter", "form"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("form"), "{stderr}");
    assert!(!stderr.contains("landing"), "{stderr}");
}

#[tokio::test]
async fn update_add_exit_tag_emits_the_tag_v1_event_and_its_stop_action() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW, "--add-exit-tag", "gone"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let patch = patch.first().expect("one patch").clone();
    let events = patch
        .as_array()
        .expect("an array")
        .iter()
        .find(|op| op["path"] == "/ruleset/unpublished_events")
        .expect("the events op")["value"]
        .clone();
    let exit = events
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["type"] == "tag.v1")
        .expect("the exit event")
        .clone();
    assert_eq!(exit["parents"], serde_json::json!([]));
    assert_eq!(
        exit["filter"]["criteria"][0]["function"],
        "rulesengine.filter.event_value_in"
    );
    assert_eq!(
        exit["filter"]["criteria"][0]["kwargs"]["values"],
        serde_json::json!(["gone"])
    );
    let actions = patch
        .as_array()
        .expect("an array")
        .iter()
        .find(|op| op["path"] == "/ruleset/unpublished_actions")
        .expect("the actions op")["value"]
        .clone();
    let stop = actions
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "stop")
        .expect("the stop action")
        .clone();
    assert_eq!(stop["definition"]["function"], "rulesengine.action.stop");
    assert_eq!(stop["parents"], serde_json::json!([exit["id"].clone()]));
}

#[tokio::test]
async fn update_removing_the_last_exit_tag_drops_both_exit_rules() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::tagged_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::tagged_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args(["workflows", "update", WORKFLOW, "--remove-exit-tag", "gone"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let patch = patch.first().expect("one patch").clone();
    for op in patch.as_array().expect("an array") {
        let rules = op["value"].as_array().expect("an array");
        assert!(!rules.iter().any(|rule| rule["metadata"] == "stop"), "{op}");
        assert!(
            !rules
                .iter()
                .any(|rule| rule["type"] == "tag.v1" && rule["id"] != fixtures::STARTER),
            "{op}"
        );
    }
}

#[tokio::test]
async fn update_exit_tags_are_idempotent_and_combinable() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::tagged_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::tagged_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args([
            "workflows",
            "update",
            WORKFLOW,
            "--add-exit-tag",
            "gone",
            "--remove-exit-tag",
            "absent",
        ])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let patch = patch.first().expect("one patch").clone();
    let events = patch
        .as_array()
        .expect("an array")
        .iter()
        .find(|op| op["path"] == "/ruleset/unpublished_events")
        .expect("the events op")["value"]
        .clone();
    let exit = events
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["type"] == "tag.v1" && rule["id"] != fixtures::STARTER)
        .expect("the exit event")
        .clone();
    assert_eq!(
        exit["filter"]["criteria"][0]["kwargs"]["values"],
        serde_json::json!(["gone"])
    );
}

#[tokio::test]
async fn add_step_appends_to_the_end_of_the_main_lane() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "message",
            fixtures::SECOND_MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let actions = patch.first().expect("one patch").as_array().expect("ops")[0]["value"].clone();
    let sends: Vec<&serde_json::Value> = actions
        .as_array()
        .expect("an array")
        .iter()
        .filter(|rule| rule["metadata"] == "send-message")
        .collect();
    assert_eq!(sends.len(), 2);
    assert_eq!(
        sends[1]["definition"]["kwargs"]["meapi_id"],
        fixtures::SECOND_MESSAGE
    );
}

#[tokio::test]
async fn add_step_message_emits_one_compose_action() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::published(
            serde_json::json!([fixtures::subscribe_starter(fixtures::STARTER)]),
            serde_json::json!([]),
        )))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "message",
            fixtures::MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    assert_eq!(ops[0]["path"], "/ruleset/unpublished_actions");
    let actions = ops[0]["value"].as_array().expect("an array");
    assert_eq!(actions.len(), 1);
    let sent = &actions[0];
    assert_eq!(
        sent["definition"]["function"],
        "ruleset.email.action.compose_v1"
    );
    assert_eq!(
        sent["definition"]["kwargs"],
        serde_json::json!({
            "account": "<event:account>",
            "list": "<event:list>",
            "meapi_id": fixtures::MESSAGE,
            "message": format!("<message:new message={}>", fixtures::MESSAGE),
            "recipient": "<event:recipient>",
        })
    );
    assert_eq!(sent["title"], serde_json::Value::Null);
    assert_eq!(sent["metadata"], "send-message");
    assert_eq!(sent["recurring"], false);
    assert_eq!(sent["parents"], serde_json::json!([fixtures::STARTER]));
}

#[tokio::test]
async fn add_step_wait_for_emits_the_duration_and_no_recurrence_rule() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::published(
            serde_json::json!([fixtures::subscribe_starter(fixtures::STARTER)]),
            serde_json::json!([]),
        )))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args(["workflows", "add-step", WORKFLOW, "wait", "--for", "3d"])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let actions = ops[0]["value"].as_array().expect("an array");
    let kwargs = &actions[0]["definition"]["kwargs"];
    assert_eq!(
        actions[0]["definition"]["function"],
        "ruleset.schedule.action.wait_v1"
    );
    assert_eq!(kwargs["delay"], "P3D");
    assert_eq!(kwargs["rrules"], serde_json::json!([]));
    assert_eq!(kwargs["timezone"], fixtures::TIMEZONE);
    assert_eq!(kwargs["use_subscriber_timezone"], false);
    assert_eq!(kwargs["action"], "<action>");
    let events = ops[1]["value"].as_array().expect("an array");
    let completion = events
        .iter()
        .find(|rule| rule["metadata"] == "wait_complete.v1")
        .expect("the completion event");
    assert_eq!(completion["filter"]["type"], "any");
    assert_eq!(
        completion["filter"]["criteria"][0]["expect"],
        kwargs["id"].clone()
    );
}

#[tokio::test]
async fn add_step_wait_send_on_and_send_at_emit_the_zero_duration_and_one_rule() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::published(
            serde_json::json!([fixtures::subscribe_starter(fixtures::STARTER)]),
            serde_json::json!([]),
        )))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "wait",
            "--send-on",
            "weekdays",
            "--send-at",
            "09:30",
            "--subscriber-timezone",
            "yes",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let kwargs = &ops[0]["value"][0]["definition"]["kwargs"];
    assert_eq!(kwargs["delay"], "PT0S");
    assert_eq!(
        kwargs["rrules"],
        serde_json::json!(["FREQ=DAILY;BYDAY=MO,TU,WE,TH,FR;BYHOUR=9;BYMINUTE=30"])
    );
    assert_eq!(kwargs["use_subscriber_timezone"], true);
}

#[tokio::test]
async fn add_step_wait_with_for_and_a_schedule_flag_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "wait",
            "--for",
            "3d",
            "--send-at",
            "09:30",
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        "error: --for cannot be combined with --send-on or --send-at"
    );
}

#[tokio::test]
async fn add_step_wait_with_one_schedule_flag_alone_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "wait",
            "--send-on",
            "weekdays",
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        "error: a wait step takes --for, or --send-on and --send-at together"
    );
}

#[tokio::test]
async fn add_step_wait_with_no_timing_flag_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "add-step", WORKFLOW, "wait"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        "error: a wait step takes --for, or --send-on and --send-at together"
    );
}

#[tokio::test]
async fn add_step_tag_emits_the_modify_tags_action() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::published(
            serde_json::json!([fixtures::subscribe_starter(fixtures::STARTER)]),
            serde_json::json!([]),
        )))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "tag",
            "--apply",
            "vip",
            "--remove",
            "trial",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let action = &ops[0]["value"][0];
    assert_eq!(
        action["definition"]["function"],
        "ruleset.tag.action.modify_tags_v1"
    );
    assert_eq!(
        action["definition"]["kwargs"],
        serde_json::json!({
            "account": "<event:account>",
            "list": "<event:list>",
            "recipient": "<subscriber>",
            "add_labels": ["vip"],
            "remove_labels": ["trial"],
        })
    );
    assert_eq!(action["metadata"], "tag");
    assert_eq!(action["recurring"], false);
}

#[tokio::test]
async fn add_step_feed_emits_the_check_feed_pair_and_a_recurring_send_inside() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::published(
            serde_json::json!([fixtures::subscribe_starter(fixtures::STARTER)]),
            serde_json::json!([]),
        )))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::feed_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "feed",
            fixtures::MESSAGE,
            "--url",
            "https://example.com/feed.xml",
            "--check-every",
            "1d",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let actions = ops[0]["value"].as_array().expect("an array");
    assert_eq!(actions[0]["metadata"], "check-feed");
    assert!(actions[0].get("recurring").is_none(), "{}", actions[0]);
    assert_eq!(actions[1]["metadata"], "send-message");
    assert_eq!(actions[1]["recurring"], true);
    let events = ops[1]["value"].as_array().expect("an array");
    let completion = events
        .iter()
        .find(|rule| rule["metadata"] == "check-feed-complete")
        .expect("the completion event");
    assert_eq!(completion["type"], "wait_complete.v1");
    assert_eq!(completion["recurring"], true);
    assert_eq!(completion["filter"]["type"], "all");
    assert_eq!(
        completion["filter"]["criteria"][1]["kwargs"]["url"],
        "https://example.com/feed.xml"
    );
}

#[tokio::test]
async fn add_step_split_emits_the_branch_expect_pair_and_a_shared_memoize_id() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::branching_ruleset()),
        )
        .await;

    harness
        .command()
        .args(["workflows", "add-step", WORKFLOW, "split", "--when-opened"])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let split = ops[0]["value"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "set-branch")
        .expect("the split action")
        .clone();
    assert_eq!(
        split["definition"]["function"],
        "rulesengine.action.set_branch"
    );
    let branches = split["definition"]["kwargs"]["branches"]
        .as_array()
        .expect("an array");
    assert_eq!(branches.len(), 2);
    assert_eq!(branches[0]["criteria"][0]["expect"], true);
    assert_eq!(branches[1]["criteria"][0]["expect"], false);
    assert_eq!(
        branches[0]["criteria"][0]["memoize_id"],
        branches[1]["criteria"][0]["memoize_id"]
    );
    assert_eq!(
        branches[0]["criteria"][0]["function"],
        "ruleset.analytics.filter.any_message_opens_v1"
    );
    assert!(split.get("recurring").is_none(), "{split}");
}

#[tokio::test]
async fn add_step_split_when_clicked_without_links_emits_any_message_clicks() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::branching_ruleset()),
        )
        .await;

    harness
        .command()
        .args(["workflows", "add-step", WORKFLOW, "split", "--when-clicked"])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let split = ops[0]["value"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "set-branch")
        .expect("the split action")
        .clone();
    assert_eq!(
        split["definition"]["kwargs"]["branches"][0]["criteria"][0]["function"],
        "ruleset.analytics.filter.any_message_clicks_v1"
    );
}

#[tokio::test]
async fn add_step_split_when_clicked_with_link_contains_emits_the_fragment_criterion() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::branching_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "split",
            "--when-clicked",
            "--link-contains",
            "/promo",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let split = ops[0]["value"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "set-branch")
        .expect("the split action")
        .clone();
    let criterion = &split["definition"]["kwargs"]["branches"][0]["criteria"][0];
    assert_eq!(
        criterion["function"],
        "ruleset.analytics.filter.any_message_click_url_contains_v1"
    );
    assert_eq!(
        criterion["kwargs"]["fragments"],
        serde_json::json!(["/promo"])
    );
    assert!(
        criterion["kwargs"].get("click_urls").is_none(),
        "{criterion}"
    );
}

#[tokio::test]
async fn add_step_with_two_placement_flags_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "message",
            fixtures::MESSAGE,
            "--before",
            fixtures::SEND_ACTION,
            "--after",
            fixtures::WAIT_ACTION,
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        "error: --before, --after, --branch and --inside are mutually exclusive"
    );
}

#[tokio::test]
async fn add_step_of_a_kind_that_takes_no_message_id_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "tag",
            fixtures::MESSAGE,
            "--apply",
            "vip",
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unexpected argument"), "{stderr}");
    assert!(stderr.contains(fixtures::MESSAGE), "{stderr}");
}

#[tokio::test]
async fn add_step_message_without_a_message_id_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "add-step", WORKFLOW, "message"])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("<MESSAGE-ID>"), "{stderr}");
}

#[tokio::test]
async fn add_step_with_a_flag_of_another_kind_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "message",
            fixtures::MESSAGE,
            "--apply",
            "vip",
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unexpected argument '--apply'"), "{stderr}");
}

#[tokio::test]
async fn add_step_takes_its_shared_flags_on_either_side_of_the_kind_word() {
    let harness = Harness::start().await;
    harness.with_list(LIST_NAME, support::LIST).await;
    harness
        .with_workflows(serde_json::json!({
            "entries": [{ "id": WORKFLOW, "name": "Welcome" }],
        }))
        .await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    for order in [
        vec![
            "workflows",
            "add-step",
            "Welcome",
            "--list",
            LIST_NAME,
            "message",
            fixtures::SECOND_MESSAGE,
        ],
        vec![
            "workflows",
            "add-step",
            "Welcome",
            "message",
            fixtures::SECOND_MESSAGE,
            "--list",
            LIST_NAME,
        ],
    ] {
        let output = harness
            .command()
            .args(&order)
            .output()
            .expect("the binary runs");
        assert!(output.status.success(), "{order:?}: {output:?}");
    }
    assert_eq!(harness.calls("PATCH", &workflow_route()).await, 2);
    let patches = harness.bodies("PATCH", &workflow_route()).await;
    let appended = |patch: &serde_json::Value| {
        let actions = patch.as_array().expect("ops")[0]["value"].clone();
        let last = actions
            .as_array()
            .expect("an array")
            .iter()
            .rfind(|rule| rule["metadata"] == "send-message")
            .expect("a send")
            .clone();
        (last["definition"].clone(), last["parents"].clone())
    };
    assert_eq!(appended(&patches[0]), appended(&patches[1]));
    assert_eq!(
        appended(&patches[0]).0["kwargs"]["meapi_id"],
        fixtures::SECOND_MESSAGE
    );
}

#[tokio::test]
async fn add_step_without_a_kind_word_exits_2_naming_the_five_kinds() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "add-step", WORKFLOW])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    for kind in ["message", "wait", "tag", "feed", "split"] {
        assert!(stderr.contains(kind), "{kind} is missing from {stderr}");
    }
}

#[tokio::test]
async fn add_step_message_carries_its_automation_rules() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::published(
            serde_json::json!([fixtures::subscribe_starter(fixtures::STARTER)]),
            serde_json::json!([]),
        )))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::automations_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "message",
            fixtures::MESSAGE,
            "--when-opened-apply-tag",
            "opened",
            "--when-clicked-exit",
            "--link",
            "https://example.com/page",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let actions = ops[0]["value"].as_array().expect("an array");
    assert!(actions.iter().any(|rule| rule["metadata"] == "tag"));
    assert!(actions.iter().any(|rule| rule["metadata"] == "stop"));
    let events = ops[1]["value"].as_array().expect("an array");
    assert!(events.iter().any(|rule| rule["type"] == "open.v2"));
    let clicked = events
        .iter()
        .find(|rule| rule["type"] == "click.v2")
        .expect("the click event");
    assert_eq!(
        clicked["filter"]["criteria"][1]["kwargs"]["urls"],
        serde_json::json!(["https://example.com/page"])
    );
}

#[tokio::test]
async fn add_step_patches_both_unpublished_arrays_in_one_call() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "message",
            fixtures::SECOND_MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(harness.calls("PATCH", &workflow_route()).await, 1);
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let sent = patch.first().expect("one patch").clone();
    let ops = sent.as_array().expect("ops");
    assert_eq!(ops.len(), 2);
    assert_eq!(ops[0]["op"], "add");
    assert_eq!(ops[0]["path"], "/ruleset/unpublished_actions");
    assert_eq!(ops[1]["path"], "/ruleset/unpublished_events");
    assert_eq!(
        sent,
        fixtures::draft_patch("add", ops[1]["value"].clone(), ops[0]["value"].clone())
    );
}

#[tokio::test]
async fn show_of_a_mixed_cadence_message_omits_its_stats() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::mixed_cadence_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;
    harness
        .accept(
            "GET",
            &format!(
                "/internal/analytics-view/reports/campaign-message-stats/{}",
                fixtures::MESSAGE
            ),
            serde_json::json!({ "total_sent": 9 }),
        )
        .await;

    let output = harness
        .command()
        .args(["workflows", "show", WORKFLOW])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let document = Harness::json(&output);
    for step in document["steps"].as_array().expect("an array") {
        assert!(step.get("stats").is_none(), "{step}");
    }
    assert_eq!(
        harness
            .calls(
                "GET",
                &format!(
                    "/internal/analytics-view/reports/campaign-message-stats/{}",
                    fixtures::MESSAGE
                )
            )
            .await,
        0
    );
}

#[tokio::test]
async fn add_step_re_emits_every_other_rule_verbatim() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "message",
            fixtures::SECOND_MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let original = fixtures::lane_ruleset();
    for rule in original["actions"].as_array().expect("an array") {
        assert!(
            ops[0]["value"].as_array().expect("an array").contains(rule),
            "{rule} was rewritten"
        );
    }
    for rule in original["events"].as_array().expect("an array") {
        assert!(
            ops[1]["value"].as_array().expect("an array").contains(rule),
            "{rule} was rewritten"
        );
    }
}

#[tokio::test]
async fn update_step_rewrites_only_the_kwargs_the_flags_name() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::WAIT_ACTION,
            "--for",
            "5d",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let wait = ops[0]["value"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "wait")
        .expect("the wait action")
        .clone();
    let kwargs = &wait["definition"]["kwargs"];
    assert_eq!(kwargs["delay"], "P5D");
    assert_eq!(kwargs["id"], fixtures::CORRELATION);
    assert_eq!(kwargs["action"], "<action>");
    assert_eq!(kwargs["timezone"], fixtures::TIMEZONE);
}

#[tokio::test]
async fn update_step_send_on_alone_changes_a_scheduled_waits_days() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::scheduled_wait_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::scheduled_wait_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::WAIT_ACTION,
            "--send-on",
            "sun",
        ])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let kwargs = ops[0]["value"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "wait")
        .expect("the wait action")["definition"]["kwargs"]
        .clone();
    assert_eq!(
        kwargs["rrules"],
        serde_json::json!(["FREQ=DAILY;BYDAY=SU;BYHOUR=9;BYMINUTE=30"])
    );
}

#[tokio::test]
async fn update_step_send_on_alone_on_a_duration_wait_exits_2() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;

    let output = harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::WAIT_ACTION,
            "--send-on",
            "sun",
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        format!(
            "error: {} waits for a duration; pass --send-on and --send-at together",
            fixtures::WAIT_ACTION
        )
    );
    assert_eq!(harness.calls("PATCH", &workflow_route()).await, 0);
}

#[tokio::test]
async fn update_step_for_puts_a_scheduled_wait_in_the_duration_form() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::scheduled_wait_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::scheduled_wait_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::WAIT_ACTION,
            "--for",
            "6h",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let kwargs = ops[0]["value"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "wait")
        .expect("the wait action")["definition"]["kwargs"]
        .clone();
    assert_eq!(kwargs["delay"], "PT6H");
    assert_eq!(kwargs["rrules"], serde_json::json!([]));
}

#[tokio::test]
async fn update_step_subscriber_timezone_alone_changes_the_timezone_source() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::WAIT_ACTION,
            "--subscriber-timezone",
            "yes",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let kwargs = ops[0]["value"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "wait")
        .expect("the wait action")["definition"]["kwargs"]
        .clone();
    assert_eq!(kwargs["use_subscriber_timezone"], true);
    assert_eq!(kwargs["delay"], "P2D");
}

#[tokio::test]
async fn update_step_message_rewrites_the_send_id_and_its_template() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::SEND_ACTION,
            "--message",
            fixtures::SECOND_MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let sent = ops[0]["value"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "send-message")
        .expect("the send action")
        .clone();
    assert_eq!(
        sent["definition"]["kwargs"]["meapi_id"],
        fixtures::SECOND_MESSAGE
    );
    assert_eq!(
        sent["definition"]["kwargs"]["message"],
        format!("<message:new message={}>", fixtures::SECOND_MESSAGE)
    );
}

#[tokio::test]
async fn update_step_remove_deletes_the_step() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::SEND_ACTION,
            "--remove",
        ])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    assert!(
        !ops[0]["value"]
            .as_array()
            .expect("an array")
            .iter()
            .any(|rule| rule["metadata"] == "send-message"),
        "{}",
        ops[0]
    );
}

#[tokio::test]
async fn update_step_remove_with_another_flag_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::SEND_ACTION,
            "--remove",
            "--message",
            fixtures::SECOND_MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        "error: --remove cannot be combined with any other flag"
    );
}

#[tokio::test]
async fn update_step_with_no_flags_exits_2() {
    let harness = Harness::start().await;

    let output = harness
        .command()
        .args(["workflows", "update-step", WORKFLOW, fixtures::SEND_ACTION])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(2));
}

#[tokio::test]
async fn a_step_command_with_an_unknown_step_id_exits_1() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;

    let unknown = "99999999-9999-4999-8999-999999999999";
    let output = harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            unknown,
            "--message",
            fixtures::SECOND_MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr.trim(),
        format!("error: {WORKFLOW} has no step {unknown}")
    );
    assert_eq!(harness.calls("PATCH", &workflow_route()).await, 0);
}

#[tokio::test]
async fn a_step_command_on_a_scheduled_set_branch_re_emits_that_function() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::set_branch_v1_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::set_branch_v1_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::SEND_ACTION,
            "--message",
            fixtures::SECOND_MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let split = ops[0]["value"]
        .as_array()
        .expect("an array")
        .iter()
        .find(|rule| rule["metadata"] == "set-branch")
        .expect("the split action")
        .clone();
    assert_eq!(
        split["definition"]["function"],
        "ruleset.schedule.action.set_branch_v1"
    );
}

#[tokio::test]
async fn publish_emits_the_workflow_object_and_errors() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "POST",
            &format!("{}/publish", workflow_route()),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args(["workflows", "publish", WORKFLOW])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let document = Harness::json(&output);
    assert_eq!(document["errors"], serde_json::json!([]));
    assert_eq!(document["id"], WORKFLOW);
}

#[tokio::test]
async fn publish_blocked_writes_one_error_line_each_and_exits_1() {
    let harness = Harness::start().await;
    let mut document = fixtures::document(fixtures::lane_ruleset());
    document["validation_problems"] = serde_json::json!([
        "The workflow has no starter",
        "A message step has no message",
    ]);
    harness.with_workflow(document).await;

    let output = harness
        .command()
        .args(["workflows", "publish", WORKFLOW])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        stderr,
        "error: The workflow has no starter\nerror: A message step has no message\n"
    );
    assert_eq!(
        harness
            .calls("POST", &format!("{}/publish", workflow_route()))
            .await,
        0
    );
}

#[tokio::test]
async fn publish_discard_throws_the_unpublished_changes_away() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "POST",
            &format!("{}/revert", workflow_route()),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    let output = harness
        .command()
        .args(["workflows", "publish", WORKFLOW, "--discard"])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        harness
            .calls("POST", &format!("{}/revert", workflow_route()))
            .await,
        1
    );
    assert_eq!(
        harness
            .calls("POST", &format!("{}/publish", workflow_route()))
            .await,
        0
    );
}

#[tokio::test]
async fn publish_and_delete_work_on_a_draft_message_workflow() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::draft_message_ruleset()))
        .await;
    harness
        .accept(
            "POST",
            &format!("{}/publish", workflow_route()),
            fixtures::document(fixtures::draft_message_ruleset()),
        )
        .await;
    harness
        .accept("DELETE", &workflow_route(), serde_json::json!({}))
        .await;

    assert!(harness
        .command()
        .args(["workflows", "publish", WORKFLOW])
        .output()
        .expect("the binary runs")
        .status
        .success());
    assert!(harness
        .command()
        .args(["workflows", "delete", WORKFLOW])
        .output()
        .expect("the binary runs")
        .status
        .success());
}

#[tokio::test]
async fn delete_emits_messages_returned() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "POST",
            "/internal/message/messages/batch/unbind",
            serde_json::json!({
                "processed": [fixtures::MESSAGE],
                "unprocessed": [],
            }),
        )
        .await;
    harness
        .accept("DELETE", &workflow_route(), serde_json::json!({}))
        .await;

    let output = harness
        .command()
        .args(["workflows", "delete", WORKFLOW])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let document = Harness::json(&output);
    assert_eq!(document["messages_returned"], 1);
}

#[tokio::test]
async fn delete_of_a_workflow_with_no_ruleset_reports_zero_messages() {
    let harness = Harness::start().await;
    let mut document = fixtures::document(fixtures::lane_ruleset());
    document["ruleset"] = serde_json::Value::Null;
    harness.with_workflow(document).await;
    harness
        .accept("DELETE", &workflow_route(), serde_json::json!({}))
        .await;

    let output = harness
        .command()
        .args(["workflows", "delete", WORKFLOW])
        .output()
        .expect("the binary runs");
    assert!(output.status.success(), "{output:?}");
    let emitted = Harness::json(&output);
    assert_eq!(emitted["messages_returned"], 0);
    assert_eq!(
        harness
            .calls("POST", "/internal/message/messages/batch/unbind")
            .await,
        0
    );
}

#[tokio::test]
async fn delete_of_a_workflow_that_has_been_active_exits_1() {
    let harness = Harness::start().await;
    let mut document = fixtures::document(fixtures::lane_ruleset());
    document["state"] = serde_json::json!("active");
    harness.with_workflow(document).await;
    harness
        .accept(
            "POST",
            "/internal/message/messages/batch/unbind",
            serde_json::json!({ "processed": [], "unprocessed": [] }),
        )
        .await;
    harness.refuse("DELETE", &workflow_route(), 400).await;

    let output = harness
        .command()
        .args(["workflows", "delete", WORKFLOW])
        .output()
        .expect("the binary runs");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.starts_with("error: "), "{stderr}");
}

#[tokio::test]
async fn every_command_writes_exactly_one_json_object_to_stdout() {
    let harness = Harness::start().await;
    harness.with_list(LIST_NAME, support::LIST).await;
    harness
        .with_workflows(serde_json::json!({ "entries": [] }))
        .await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;
    harness
        .accept(
            "GET",
            &format!(
                "/internal/analytics-view/reports/campaign-message-stats/{}",
                fixtures::MESSAGE
            ),
            serde_json::json!({ "total_sent": 0 }),
        )
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;
    harness
        .accept(
            "POST",
            "/internal/campaign/campaigns",
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;
    harness
        .accept(
            "POST",
            &format!("{}/publish", workflow_route()),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;
    harness
        .accept(
            "POST",
            "/internal/message/messages/batch/unbind",
            serde_json::json!({ "processed": [], "unprocessed": [] }),
        )
        .await;
    harness
        .accept("DELETE", &workflow_route(), serde_json::json!({}))
        .await;

    let invocations: Vec<Vec<&str>> = vec![
        vec!["workflows", "list", "--list", LIST_NAME],
        vec!["workflows", "show", WORKFLOW],
        vec!["workflows", "create", "Fresh", "--list", LIST_NAME],
        vec!["workflows", "update", WORKFLOW, "--name", "Renamed"],
        vec![
            "workflows",
            "add-step",
            WORKFLOW,
            "message",
            fixtures::SECOND_MESSAGE,
        ],
        vec![
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::SEND_ACTION,
            "--message",
            fixtures::SECOND_MESSAGE,
        ],
        vec!["workflows", "publish", WORKFLOW],
        vec!["workflows", "delete", WORKFLOW],
    ];
    for args in invocations {
        let output = harness
            .command()
            .args(&args)
            .output()
            .expect("the binary runs");
        assert!(output.status.success(), "{args:?}: {output:?}");
        let document = Harness::json(&output);
        assert!(document.is_object(), "{args:?}: {document}");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout)
                .matches('\n')
                .count(),
            serde_json::to_string_pretty(&document)
                .expect("the document renders")
                .matches('\n')
                .count()
                + 1,
            "{args:?} wrote more than one object"
        );
    }
}

#[tokio::test]
async fn add_step_message_inside_a_feed_appends_a_recurring_send() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::feed_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::feed_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "--inside",
            fixtures::FEED_ACTION,
            "message",
            fixtures::SECOND_MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let actions = ops[0]["value"].as_array().expect("an array");
    let added = actions
        .iter()
        .find(|rule| rule["definition"]["kwargs"]["meapi_id"] == fixtures::SECOND_MESSAGE)
        .expect("the added send");
    assert_eq!(added["metadata"], "send-message");
    assert_eq!(added["recurring"], true);
    assert_eq!(added["parents"][0], fixtures::FEED_EVENT);
}

#[tokio::test]
async fn add_step_feed_with_check_times_writes_a_finite_count() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::published(
            serde_json::json!([fixtures::subscribe_starter(fixtures::STARTER)]),
            serde_json::json!([]),
        )))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::feed_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "add-step",
            WORKFLOW,
            "feed",
            fixtures::MESSAGE,
            "--url",
            "https://example.com/feed.xml",
            "--check-every",
            "1h",
            "--check-times",
            "10",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let actions = ops[0]["value"].as_array().expect("an array");
    assert_eq!(actions[0]["metadata"], "check-feed");
    assert_eq!(
        actions[0]["definition"]["kwargs"]["rrules"][0],
        "FREQ=HOURLY;COUNT=10"
    );
}

#[tokio::test]
async fn update_step_remove_on_a_published_wait_writes_is_deleted() {
    let harness = Harness::start().await;
    harness
        .with_workflow(fixtures::document(fixtures::lane_ruleset()))
        .await;
    harness
        .accept(
            "PATCH",
            &workflow_route(),
            fixtures::document(fixtures::lane_ruleset()),
        )
        .await;

    harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::WAIT_ACTION,
            "--remove",
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let actions = ops[0]["value"].as_array().expect("an array");
    let wait = actions
        .iter()
        .find(|rule| rule["metadata"] == "wait")
        .expect("the wait action is kept");
    assert_eq!(wait["is_deleted"], true);
}

#[tokio::test]
async fn show_of_a_removed_published_wait_emits_deleted() {
    let harness = Harness::start().await;
    let [wait_action, wait_event] = fixtures::deleted_wait(
        fixtures::WAIT_ACTION,
        fixtures::WAIT_EVENT,
        fixtures::STARTER,
        "P2D",
    );
    let ruleset = fixtures::with_unpublished(
        fixtures::lane_ruleset(),
        serde_json::json!([fixtures::subscribe_starter(fixtures::STARTER), wait_event]),
        serde_json::json!([
            wait_action,
            fixtures::send(
                fixtures::SEND_ACTION,
                fixtures::WAIT_EVENT,
                fixtures::MESSAGE,
                false
            )
        ]),
    );
    harness.with_workflow(fixtures::document(ruleset)).await;
    harness.with_subjects(serde_json::json!({})).await;

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    let wait = &document["steps"][0];
    assert_eq!(wait["kind"], "wait");
    assert_eq!(wait["deleted"], true);

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--published", "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    assert_eq!(document["steps"][0]["deleted"], false);
}

#[tokio::test]
async fn show_emits_a_reserved_events_workflow_and_update_step_preserves_it() {
    let harness = Harness::start().await;
    let reserved = serde_json::json!({
        "id": "e0000000-0000-4000-8000-000000000007",
        "parents": [fixtures::SEND_ACTION],
        "type": "message_not_opened.v1",
        "filter": null,
        "title": null,
        "metadata": "message_not_opened.v1",
    });
    let ruleset = fixtures::published(
        serde_json::json!([
            fixtures::subscribe_starter(fixtures::STARTER),
            reserved.clone(),
        ]),
        serde_json::json!([fixtures::send(
            fixtures::SEND_ACTION,
            fixtures::STARTER,
            fixtures::MESSAGE,
            false,
        )]),
    );
    harness
        .with_workflow(fixtures::document(ruleset.clone()))
        .await;
    harness.with_subjects(serde_json::json!({})).await;
    harness
        .accept("PATCH", &workflow_route(), fixtures::document(ruleset))
        .await;

    let document = Harness::json(
        &harness
            .command()
            .args(["workflows", "show", WORKFLOW, "--no-stats"])
            .output()
            .expect("the binary runs"),
    );
    let steps = document["steps"].as_array().expect("an array");
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0]["kind"], "message");

    harness
        .command()
        .args([
            "workflows",
            "update-step",
            WORKFLOW,
            fixtures::SEND_ACTION,
            "--message",
            fixtures::SECOND_MESSAGE,
        ])
        .output()
        .expect("the binary runs");
    let patch = harness.bodies("PATCH", &workflow_route()).await;
    let ops = patch.first().expect("one patch").as_array().expect("ops");
    let events = ops[1]["value"].as_array().expect("an array");
    assert!(
        events
            .iter()
            .any(|rule| rule["type"] == "message_not_opened.v1"),
        "the reserved event is written back unchanged"
    );
}

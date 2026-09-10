//! A workflow's working graph as the detail's `visualization` row states it:
//! the starter, each step's kind and what it acts on, and the exit tags.

use aweber::workflows::{
    ClickRule, Condition, ConditionField, ConditionTest, Graph, OpenRule, Starter, Step, StepKind,
    Tested, Timezone, Workflow,
};
use serde_json::Value;

/// The working graph of a workflow document, or why its ruleset cannot be read.
pub fn working_graph(document: &Value) -> Result<Graph, String> {
    let workflow: Workflow =
        serde_json::from_value(document.clone()).map_err(|error| error.to_string())?;
    let ruleset = workflow.ruleset().map_err(|error| error.to_string())?;
    Ok(ruleset.working(workflow.timezone().unwrap_or_else(Timezone::utc)))
}

pub fn visualization(graph: &Graph) -> Value {
    serde_json::json!({
        "starter": graph.starter().map(starter),
        "steps": steps(&graph.steps()),
        "exit_tags": words(graph.exit_tags()),
    })
}

fn starter(starter: &Starter) -> Value {
    match starter {
        Starter::NewSubscriber { conditions } => serde_json::json!({
            "on": "new subscriber",
            "conditions": conditions.iter().map(condition).collect::<Vec<String>>(),
        }),
        Starter::Tag {
            tags,
            except,
            reentry,
        } => serde_json::json!({
            "on": "tag",
            "tags": words(tags),
            "except": words(except),
            "reentry": reentry,
        }),
    }
}

fn condition(condition: &Condition) -> String {
    let field = match &condition.field {
        ConditionField::Source => "source".to_string(),
        ConditionField::AdTracking => "ad tracking".to_string(),
        ConditionField::Country => "country".to_string(),
        ConditionField::Custom { name } => name.clone(),
    };
    let test = match &condition.test {
        ConditionTest::Is { value } => format!("is {value}"),
        ConditionTest::IsNot { value } => format!("is not {value}"),
        ConditionTest::Defined => "is defined".to_string(),
        ConditionTest::Undefined => "is undefined".to_string(),
    };
    format!("{field} {test}")
}

fn steps(steps: &[Step]) -> Value {
    Value::Array(steps.iter().map(step).collect())
}

fn step(step: &Step) -> Value {
    let mut fields = serde_json::Map::new();
    fields.insert("id".to_string(), Value::String(step.id.to_string()));
    let kind = match &step.kind {
        StepKind::Message { message } => serde_json::json!({
            "step": "message",
            "message": message.as_ref().map(ToString::to_string),
        }),
        StepKind::Wait {
            timing,
            timezone_source,
            deleted,
        } => serde_json::json!({
            "step": "wait",
            "delay": timing.delay.as_ref().map(ToString::to_string),
            "schedules": words(&timing.schedules),
            "timezone": timezone_source.to_string(),
            "deleted": deleted,
        }),
        StepKind::Tag { applied, removed } => serde_json::json!({
            "step": "tag",
            "applied": words(applied),
            "removed": words(removed),
        }),
        StepKind::Feed {
            url,
            check_every,
            inside,
        } => serde_json::json!({
            "step": "feed",
            "url": url.as_ref().map(ToString::to_string),
            "check_every": check_every.to_string(),
            "inside": steps(inside),
        }),
        StepKind::Split { tested, yes, no } => serde_json::json!({
            "step": "split",
            "tested": tested_text(tested),
            "yes": steps(yes),
            "no": steps(no),
        }),
    };
    if let Value::Object(kind) = kind {
        fields.extend(kind);
    }
    if let Some(rule) = &step.automations.on_open {
        fields.insert("on_open".to_string(), open_rule(rule));
    }
    if !step.automations.on_click.is_empty() {
        fields.insert(
            "on_click".to_string(),
            Value::Array(step.automations.on_click.iter().map(click_rule).collect()),
        );
    }
    Value::Object(fields)
}

fn tested_text(tested: &Tested) -> String {
    match tested {
        Tested::HasTag { tags } => format!("has tag {}", words(tags).join(", ")),
        Tested::Opened => "opened".to_string(),
        Tested::Clicked { link: Some(link) } => format!("clicked {link}"),
        Tested::Clicked { link: None } => "clicked".to_string(),
    }
}

fn open_rule(rule: &OpenRule) -> Value {
    serde_json::json!({
        "applied": words(&rule.applied),
        "removed": words(&rule.removed),
        "exit": rule.exit,
    })
}

fn click_rule(rule: &ClickRule) -> Value {
    serde_json::json!({
        "links": words(&rule.links),
        "applied": words(&rule.applied),
        "removed": words(&rule.removed),
        "exit": rule.exit,
    })
}

fn words<T: std::fmt::Display>(values: &[T]) -> Vec<String> {
    values.iter().map(ToString::to_string).collect()
}

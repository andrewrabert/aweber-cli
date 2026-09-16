use crate::catalog::Operation;
use crate::catalog::args::{ArgRole, Args, ValueKind};

/// The `aweber …` invocation equivalent to the plan; flags first, then
/// positionals in their declared order, and arguments only, never credentials.
pub fn command_line(operation: Operation, args: &Args) -> String {
    let specs = operation.specs();
    let mut line = format!("aweber {} {}", operation.group(), operation.action());
    for (name, values) in args.iter() {
        let spec = specs.iter().find(|spec| spec.name == name);
        if spec.is_some_and(|spec| spec.role == ArgRole::Reserved || spec.positional) {
            continue;
        }
        let long = spec.map_or(name, |spec| spec.long.as_str());
        for value in values {
            if value.kind == ValueKind::Flag {
                if value.raw != "false" {
                    line.push_str(&format!(" --{long}"));
                }
                continue;
            }
            line.push_str(&format!(" --{long} {}", quote(&value.raw)));
        }
    }
    for spec in specs.iter().filter(|spec| spec.positional) {
        for value in args.all(&spec.name) {
            line.push_str(&format!(" {}", quote(&value.raw)));
        }
    }
    line
}

/// `aweber api '/1.0/accounts/1/lists' -X GET`, and with a body,
/// `printf '%s' '…' | aweber api '/1.0/…' -X POST -H 'content-type: …' --input -`,
/// the content type being the body's own.
pub fn raw_command_line(
    method: &reqwest::Method,
    path: &str,
    body: Option<&crate::catalog::PlanBody>,
) -> String {
    let call = format!("aweber api {} -X {}", quote(path), quote(method.as_str()));
    match body {
        None => call,
        Some(body) => format!(
            "printf '%s' {} | {call} -H {} --input -",
            quote(&body.encoded()),
            quote(&format!("content-type: {}", body.content_type())),
        ),
    }
}

/// The `aweber api` invocation that issues a built plan: its query on the path
/// and its headers as `-H`, for an operation `aweber` does not route by name.
pub fn plan_command_line(plan: &crate::catalog::RequestPlan) -> String {
    let mut path = plan.path.clone();
    if !plan.query.is_empty() {
        let pairs: Vec<String> = plan
            .query
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect();
        path.push('?');
        path.push_str(&pairs.join("&"));
    }
    let mut line = raw_command_line(&plan.method, &path, plan.body.as_ref());
    for (name, value) in &plan.headers {
        line.push_str(&format!(" -H {}", quote(&format!("{name}: {value}"))));
    }
    line
}

/// Shell quoting, so an emitted line can be pasted as it stands.
fn quote(value: &str) -> String {
    let safe = !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/' | ':' | '@'));
    if safe {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', r"'\''"))
    }
}

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    let temp = tempfile::tempdir().expect("temporary directory");
    Command::new(env!("CARGO_BIN_EXE_aweber"))
        .args(args)
        .env_remove("AWEBER_TOKEN")
        .env("AWEBER_API_URL", "http://127.0.0.1:1")
        .env("AWEBER_AUTH_URL", "http://127.0.0.1:1")
        .env("AWEBER_CREDENTIALS_FILE", temp.path().join("absent.json"))
        .output()
        .expect("run aweber")
}

#[test]
fn reads_the_guide_without_login() {
    let output = run(&["guide"]);
    assert!(output.status.success(), "{output:?}");
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout, include_bytes!("../../GUIDE.md"));
}

#[test]
fn lists_sections_and_matches_a_section_case_insensitively() {
    let list = run(&["guide", "--list"]);
    assert!(list.status.success(), "{list:?}");
    let names = String::from_utf8(list.stdout).expect("UTF-8 section names");
    assert!(names.lines().any(|name| name == "workflows"));
    assert!(names.lines().any(|name| name == "broadcasts"));

    let output = run(&["guide", "WoRkFlOwS"]);
    assert!(output.status.success(), "{output:?}");
    let text = String::from_utf8(output.stdout).expect("UTF-8 section text");
    assert_eq!(text.lines().next(), Some("## workflows"));
    assert_eq!(
        text.lines().filter(|line| line.starts_with("## ")).count(),
        1
    );
}

#[test]
fn exits_with_usage_error_for_an_unknown_section() {
    let output = run(&["guide", "missing"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let message = String::from_utf8(output.stderr).expect("UTF-8 usage error");
    assert!(message.contains("missing"));
    assert!(message.contains("workflows"));
}

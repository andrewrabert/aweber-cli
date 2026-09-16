//! No credential leaves the process, by any of the ways out.

use aweber::catalog::{ArgValue, Args, Operation, ValueKind};
use aweber_tui::Secret;
use aweber_tui::core::event_log::LogEntry;
use aweber_tui::core::state::{Account, View};
use aweber_tui::core::views::detail::DetailView;
use aweber_tui::core::{Generation, State};
use aweber_tui::harness::{Harness, fixed_now};

const TOKEN: &str = "aw-secret-access-token-value";
const REFRESH: &str = "aw-secret-refresh-token-value";
const VERIFIER: &str = "aw-secret-code-verifier-value";
const CLIENT_SECRET: &str = "aw-secret-client-secret-value";
const MULTIBYTE: &str = "aw-secret-tökén-välue-ünicode";

/// Every credential that must never be printed.
const SECRETS: [&str; 5] = [TOKEN, REFRESH, VERIFIER, CLIENT_SECRET, MULTIBYTE];

fn assert_clean(what: &str, text: &str) {
    for secret in SECRETS {
        assert!(
            !text.contains(secret),
            "{what} carries a credential:\n{text}"
        );
    }
}

/// A document as an unlucky API might return one.
fn leaky() -> serde_json::Value {
    serde_json::json!({
        "id": 7,
        "name": "Weekly",
        "access_token": TOKEN,
        "refresh_token": REFRESH,
        "client_secret": CLIENT_SECRET,
        "code_verifier": VERIFIER,
    })
}

#[test]
fn a_secret_never_prints_itself() {
    let secret = Secret::new(TOKEN.to_string());
    assert_eq!(secret.expose(), TOKEN);
    assert_clean("a debug-printed secret", &format!("{secret:?}"));
    assert_clean("a displayed secret", &format!("{secret}"));
}

#[test]
fn the_event_log_scrubs_every_field() {
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), true);
    state.log.append(LogEntry {
        at: fixed_now(),
        method: "GET".to_string(),
        path: format!("/1.0/accounts/1/lists?access_token={TOKEN}"),
        status: Some(200),
        duration: None,
        attempt: 1,
        waited: None,
        refreshed: false,
        detail: Some(format!("Authorization: Bearer {TOKEN}")),
        body: Some(leaky().to_string()),
    });
    assert_clean("the event log", &state.log.text());
    assert_clean("a log line", &state.log.line(0));
}

#[tokio::test]
async fn credentials_never_leave_the_process() {
    let mut args = Args::default();
    args.set("list-id", ArgValue::new(ValueKind::Integer, "7"));
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), true);
    state.context.account = Some(Account {
        id: 1,
        uuid: None,
        name: None,
    });
    let mut detail = DetailView::opening(
        Operation::GetList,
        args,
        "lists get".to_string(),
        Generation::default(),
    );
    detail.document = Some(leaky());
    state.stack.push(View::Detail(detail));
    state.log.append(LogEntry {
        at: fixed_now(),
        method: "GET".to_string(),
        path: format!("/1.0/accounts/1/lists/7?access_token={TOKEN}"),
        status: Some(200),
        duration: None,
        attempt: 1,
        waited: None,
        refreshed: false,
        detail: None,
        body: Some(leaky().to_string()),
    });

    let mut harness = Harness::new(80, 24).with_state(state);
    harness.key("y").settle().await;
    let copied = harness.copied().expect("something was copied");
    assert_clean("the clipboard", &copied);
    assert!(
        copied.contains("aweber lists get"),
        "the copy emits the equivalent command line:\n{copied}"
    );

    harness.key("w").settle().await;
    let written = harness.written();
    assert_eq!(written.len(), 1, "one file was written");
    assert_clean("the written file", &written[0].1);
    std::fs::remove_file(&written[0].0).ok();

    harness.key("Ctrl-l").settle().await;
    assert_clean("the event log view", &harness.screen());
    harness.key("y").settle().await;
    assert_clean(
        "the copied event log",
        &harness.copied().expect("the log was copied"),
    );
}

/// `--oauth-token value`, `--client-secret=value`, and
/// `-H Authorization: Bearer value` are all scrubbed, and the flag itself
/// survives.
#[test]
fn a_credential_given_as_a_flag_is_scrubbed() {
    let forms = [
        (
            format!("aweber oauth access-token --oauth-token {TOKEN} --list-id 7"),
            "--oauth-token",
        ),
        (
            format!("aweber oauth token --client-secret={CLIENT_SECRET} --list-id 7"),
            "--client-secret",
        ),
        (
            format!("aweber oauth token --code-verifier '{VERIFIER}' --list-id 7"),
            "--code-verifier",
        ),
        (
            format!("aweber api /1.0 -H 'Authorization: Bearer {REFRESH}'"),
            "-H",
        ),
    ];
    for (line, flag) in forms {
        let scrubbed = aweber_tui::core::redact::scrub(&line);
        assert_clean("an emitted command line", &scrubbed);
        assert!(
            scrubbed.contains(flag),
            "{flag} survives its value:\n{scrubbed}"
        );
    }
}

/// The command line an `oauth` operation emits carries the flag and not the
/// value.
#[tokio::test]
async fn an_emitted_oauth_command_line_carries_no_token() {
    let mut args = Args::default();
    args.set("oauth-token", ArgValue::new(ValueKind::Text, TOKEN));
    args.set(
        "oauth-consumer-key",
        ArgValue::new(ValueKind::Text, REFRESH),
    );
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: None,
        name: None,
    });
    let mut detail = DetailView::opening(
        Operation::OauthGetAccessToken,
        args,
        "oauth get-access-token".to_string(),
        Generation::default(),
    );
    detail.document = Some(serde_json::json!({ "id": 1 }));
    state.stack.push(View::Detail(detail));

    let mut harness = Harness::new(80, 24).with_state(state);
    harness.key("y").settle().await;
    let copied = harness.copied().expect("the command line was copied");
    assert_clean("the emitted oauth command line", &copied);
    assert!(
        copied.contains("--oauth-token"),
        "the flag survives its value:\n{copied}"
    );
}

/// A character straddling the length of a credential name is scrubbed rather
/// than cut, at every offset a character can start at.
#[test]
fn multibyte_text_beside_a_credential_name_is_scrubbed() {
    const NAMES: [&str; 8] = [
        "access_token",
        "refresh_token",
        "client_secret",
        "code_verifier",
        "code_challenge",
        "code",
        "token",
        "authorization",
    ];
    const FLAGS: [&str; 10] = [
        "--access-token",
        "--refresh-token",
        "--client-secret",
        "--code-verifier",
        "--code-challenge",
        "--code",
        "--token",
        "--oauth-token",
        "--oauth-consumer-key",
        "-token",
    ];
    const TAIL: &str = "ötökén-välue-ünicodeö";

    for name in NAMES.iter().chain(FLAGS.iter()).chain(["Bearer "].iter()) {
        for cut in 0..=name.len() {
            let head = &name[..cut];
            for form in [
                format!("{head}{TAIL}"),
                format!("\"{head}{TAIL}\": \"{TAIL}\""),
                format!("{head}{TAIL}={TAIL}"),
            ] {
                aweber_tui::core::redact::scrub(&form);
            }
        }
    }

    for name in NAMES {
        for form in [
            format!("{name}={TAIL}{TOKEN}"),
            format!("\"{name}\": \"{TAIL}{TOKEN}\""),
        ] {
            assert_clean(
                "a multibyte credential value",
                &aweber_tui::core::redact::scrub(&form),
            );
        }
    }

    for flag in FLAGS {
        for form in [
            format!("aweber oauth token {flag}={TAIL}{TOKEN} --list-id 7"),
            format!("aweber oauth token {flag} {TAIL}{TOKEN} --list-id 7"),
        ] {
            assert_clean(
                "a multibyte flag value",
                &aweber_tui::core::redact::scrub(&form),
            );
        }
    }

    assert_clean(
        "a multibyte bearer value",
        &aweber_tui::core::redact::scrub(&format!("Authorization: Bearer {TAIL}{TOKEN}")),
    );
}

/// A credential of multibyte characters is redacted whole as a query value, a
/// JSON value, a bearer value, and a flag value in all three flag forms.
#[test]
fn a_credential_of_multibyte_characters_is_scrubbed() {
    let forms = [
        format!("/1.0/accounts/1/lists?access_token={MULTIBYTE}&page=2"),
        format!("{{\"access_token\": \"{MULTIBYTE}\", \"id\": 7}}"),
        format!("Authorization: Bearer {MULTIBYTE}"),
        format!("aweber oauth token --oauth-token {MULTIBYTE} --list-id 7"),
        format!("aweber oauth token --client-secret={MULTIBYTE} --list-id 7"),
        format!("aweber oauth token --code-verifier '{MULTIBYTE}' --list-id 7"),
    ];
    for form in forms {
        assert_clean(
            "a multibyte credential",
            &aweber_tui::core::redact::scrub(&form),
        );
    }
}

/// `start-token=abc` and `--start-token abc` survive whole while `token=abc`,
/// `?token=abc`, and `"token": "abc"` are scrubbed.
#[test]
fn a_secret_name_inside_a_word_is_not_a_secret() {
    for kept in [
        "start-token=abc",
        "--start-token abc",
        "--start-token=abc",
        "/reports?start-token=abc&page=2",
    ] {
        assert_eq!(
            aweber_tui::core::redact::scrub(kept),
            kept,
            "'{kept}' is a pagination cursor, not a credential"
        );
    }
    for scrubbed in [
        "token=abc",
        "?token=abc",
        "\"token\": \"abc\"",
        "--token abc",
    ] {
        let out = aweber_tui::core::redact::scrub(scrubbed);
        assert!(!out.contains("abc"), "'{scrubbed}' kept its value: {out}");
    }
}

#[tokio::test]
async fn an_emitted_command_line_never_carries_a_credential() {
    let mut state = State::new(Some(aweber_tui::StoredAccount { id: 1, uuid: None }), false);
    state.context.account = Some(Account {
        id: 1,
        uuid: None,
        name: None,
    });
    let mut harness = Harness::new(80, 24).with_state(state);
    harness.key("Ctrl-p");
    for character in "raw request".chars() {
        if character == ' ' {
            harness.key("Space");
        } else {
            harness.key(&character.to_string());
        }
    }
    harness.key("Enter");
    assert!(
        matches!(harness.state().view(), View::RawRequest(_)),
        "the palette opens the Raw Request view"
    );
    for character in format!("/1.0/accounts/1/lists?access_token={TOKEN}").chars() {
        harness.key(&character.to_string());
    }
    harness.keys("Tab Tab").key("y").settle().await;
    assert_clean(
        "the raw request command line",
        &harness.copied().expect("the raw request was copied"),
    );
}

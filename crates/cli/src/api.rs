use std::io::{IsTerminal, Read as _, Write as _};

use anyhow::Context as _;
use base64::Engine as _;

use aweber::client::{Client, RawResponse};

/// Run the raw-API passthrough, exiting with status 1 on a non-2xx response.
pub(crate) async fn run(client: &Client, matches: &clap::ArgMatches) -> anyhow::Result<()> {
    let path = matches.get_one::<String>("path").expect("path is required");
    let method: reqwest::Method = matches
        .get_one::<String>("method")
        .expect("method has default")
        .parse()
        .context("invalid HTTP method")?;

    let body = match matches.get_one::<String>("input") {
        Some(input) if input == "-" => {
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf)?;
            Some(buf)
        }
        Some(file) => Some(std::fs::read(file).context("failed to read input file")?),
        None => None,
    };

    let headers: Vec<(reqwest::header::HeaderName, String)> = matches
        .get_many::<String>("header")
        .unwrap_or_default()
        .map(|h| {
            let (key, value) = h.split_once(':').context("header must be key:value")?;
            let name = key
                .parse::<reqwest::header::HeaderName>()
                .map_err(|e| anyhow::anyhow!("invalid header name '{key}': {e}"))?;
            Ok((name, value.trim_start().to_string()))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let resp = client
        .raw_request(method, path, &headers, body.as_deref())
        .await?;

    if matches.get_flag("json") {
        print_json_response(&resp)?;
    } else {
        let color = std::io::stderr().is_terminal();
        let pretty = std::io::stdout().is_terminal();

        print_response_headers(&resp, color);

        if pretty {
            if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&resp.body) {
                let formatted = colored_json::to_colored_json_auto(&json)?;
                println!("{formatted}");
            } else {
                std::io::stdout().write_all(&resp.body)?;
            }
        } else {
            std::io::stdout().write_all(&resp.body)?;
        }
    }

    if !(200..300).contains(&resp.status) {
        std::process::exit(1);
    }
    Ok(())
}

fn print_response_headers(resp: &RawResponse, color: bool) {
    if color {
        eprintln!(
            "\x1b[34m{}\x1b[0m \x1b[36m{}\x1b[0m",
            resp.http_version, resp.status
        );
        for (key, value) in &resp.headers {
            eprintln!("\x1b[37m{key}:\x1b[0m \x1b[36m{value}\x1b[0m");
        }
    } else {
        eprintln!("{} {}", resp.http_version, resp.status);
        for (key, value) in &resp.headers {
            eprintln!("{key}: {value}");
        }
    }
    eprintln!();
}

fn encode_body(body: &[u8]) -> (String, &'static str) {
    if let Ok(text) = std::str::from_utf8(body) {
        return (text.to_string(), "text");
    }
    let encoded = base64::engine::general_purpose::STANDARD.encode(body);
    (encoded, "base64")
}

fn collect_headers(headers: &[(String, String)]) -> serde_json::Value {
    let mut grouped: indexmap::IndexMap<&str, Vec<&str>> = indexmap::IndexMap::new();
    for (k, v) in headers {
        grouped.entry(k).or_default().push(v);
    }
    let map: serde_json::Map<String, serde_json::Value> = grouped
        .into_iter()
        .map(|(k, values)| {
            let value = if values.len() == 1 {
                serde_json::Value::String(values[0].to_string())
            } else {
                serde_json::Value::Array(
                    values
                        .into_iter()
                        .map(|v| serde_json::Value::String(v.to_string()))
                        .collect(),
                )
            };
            (k.to_string(), value)
        })
        .collect();
    serde_json::Value::Object(map)
}

fn print_json_response(resp: &RawResponse) -> anyhow::Result<()> {
    let (body_value, body_encoding) = encode_body(&resp.body);

    let mut envelope = serde_json::Map::new();
    envelope.insert("status".into(), resp.status.into());
    envelope.insert("headers".into(), collect_headers(&resp.headers));
    envelope.insert("body".into(), body_value.into());
    envelope.insert("body_encoding".into(), body_encoding.into());

    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::Value::Object(envelope))?
    );
    Ok(())
}

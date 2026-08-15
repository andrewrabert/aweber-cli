use aweber::pagination::{next_cursor, within_offset_cap};

fn headers(link: &str) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::LINK, link.parse().unwrap());
    headers
}

#[test]
fn next_cursor_reads_only_the_query_parameter() {
    let headers = headers(
        "<https://api.aweber.com/service/reports/recurring-events/c/events/e/\
         ?start-token=1700000000>; rel=\"next\"",
    );
    assert_eq!(
        next_cursor(&headers, "start-token"),
        Some("1700000000".parse().unwrap())
    );
}

#[test]
fn next_cursor_ignores_a_link_without_rel_next() {
    let headers = headers("<https://api.aweber.com/reports?after=abc>; rel=\"prev\"");
    assert_eq!(next_cursor(&headers, "after"), None);
}

/// A `next` cursor whose percent escape is followed by a multibyte character
/// decodes to the bytes it names, and the process survives.
#[test]
fn a_cursor_with_a_multibyte_escape_decodes() {
    let headers =
        headers("<https://api.aweber.com/reports?after=t%C3%B6k%C3%A9n%2c9>; rel=\"next\"");
    assert_eq!(
        next_cursor(&headers, "after"),
        Some("tökén,9".parse().unwrap())
    );
}

#[test]
fn offset_cursor_past_the_cap_resolves_to_none() {
    assert_eq!(
        within_offset_cap("clicks,9999".parse().unwrap()),
        Some("clicks,9999".parse().unwrap())
    );
    assert_eq!(
        within_offset_cap("clicks,10000".parse().unwrap()),
        Some("clicks,10000".parse().unwrap())
    );
    assert_eq!(within_offset_cap("clicks,10001".parse().unwrap()), None);
}

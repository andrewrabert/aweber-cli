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

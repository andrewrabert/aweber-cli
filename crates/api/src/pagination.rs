pub const MAX_OFFSET_CURSOR: u64 = 10_000;

const DEFAULT_PAGE_SIZE: u8 = 100;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cursor(String);

impl std::str::FromStr for Cursor {
    type Err = CursorError;

    fn from_str(text: &str) -> Result<Cursor, CursorError> {
        if text.is_empty() {
            return Err(CursorError);
        }
        Ok(Cursor(text.to_string()))
    }
}

impl std::convert::TryFrom<&str> for Cursor {
    type Error = CursorError;

    fn try_from(text: &str) -> Result<Cursor, CursorError> {
        text.parse()
    }
}

impl std::fmt::Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug)]
pub struct CursorError;

impl std::fmt::Display for CursorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("a continuation token is never empty")
    }
}

impl std::error::Error for CursorError {}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PageSize(std::num::NonZeroU8);

impl PageSize {
    pub fn default_size() -> PageSize {
        PageSize(std::num::NonZeroU8::new(DEFAULT_PAGE_SIZE).unwrap_or(std::num::NonZeroU8::MIN))
    }
}

impl std::str::FromStr for PageSize {
    type Err = PageSizeError;

    fn from_str(text: &str) -> Result<PageSize, PageSizeError> {
        text.parse().map(PageSize).map_err(|_| PageSizeError {
            rejected: text.to_string(),
        })
    }
}

impl std::convert::TryFrom<&str> for PageSize {
    type Error = PageSizeError;

    fn try_from(text: &str) -> Result<PageSize, PageSizeError> {
        text.parse()
    }
}

impl std::fmt::Display for PageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug)]
pub struct PageSizeError {
    rejected: String,
}

impl std::fmt::Display for PageSizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a page size", self.rejected)
    }
}

impl std::error::Error for PageSizeError {}

pub struct Page<T> {
    pub entries: Vec<T>,
    pub next_cursor: Option<Cursor>,
}

pub fn next_cursor(headers: &reqwest::header::HeaderMap, parameter: &str) -> Option<Cursor> {
    headers
        .get_all(reqwest::header::LINK)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(split_links)
        .find_map(|link| next_target(link))
        .and_then(|target| query_parameter(target, parameter))
        .and_then(|text| text.parse().ok())
}

/// The offset of a `<first-key>,<offset>` cursor, when it carries one.
pub fn cursor_offset(cursor: &Cursor) -> Option<u64> {
    cursor.0.rsplit_once(',')?.1.trim().parse::<u64>().ok()
}

/// `<first-key>,<offset>` cursors resolve to `None` past [`MAX_OFFSET_CURSOR`].
pub fn within_offset_cap(cursor: Cursor) -> Option<Cursor> {
    match cursor_offset(&cursor) {
        Some(offset) if offset > MAX_OFFSET_CURSOR => None,
        _ => Some(cursor),
    }
}

fn split_links(header: &str) -> impl Iterator<Item = &str> {
    let mut depth = 0usize;
    let mut quoted = false;
    header
        .split(move |c| {
            match c {
                '"' => quoted = !quoted,
                '<' if !quoted => depth += 1,
                '>' if !quoted => depth = depth.saturating_sub(1),
                _ => {}
            }
            c == ',' && depth == 0 && !quoted
        })
        .map(str::trim)
}

fn next_target(link: &str) -> Option<&str> {
    let mut parts = link.split(';').map(str::trim);
    let target = parts.next()?;
    let target = target.strip_prefix('<')?.strip_suffix('>')?;
    parts
        .filter_map(|param| param.split_once('='))
        .any(|(name, value)| {
            name.trim().eq_ignore_ascii_case("rel")
                && value.trim().trim_matches('"').eq_ignore_ascii_case("next")
        })
        .then_some(target)
}

fn query_parameter(target: &str, parameter: &str) -> Option<String> {
    let query = target.split_once('?')?.1;
    let query = query.split('#').next().unwrap_or(query);
    query
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .find(|(name, _)| *name == parameter)
        .map(|(_, value)| percent_decode(value))
}

/// The value of one hexadecimal digit, for a byte that is one.
fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                out.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                match (hex_digit(bytes[index + 1]), hex_digit(bytes[index + 2])) {
                    (Some(high), Some(low)) => {
                        out.push((high << 4) | low);
                        index += 3;
                    }
                    _ => {
                        out.push(b'%');
                        index += 1;
                    }
                }
            }
            byte => {
                out.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

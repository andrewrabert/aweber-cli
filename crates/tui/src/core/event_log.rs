//! Every attempt of every request, append-only and scrubbed on the way in.

#[derive(Clone, Debug, Default)]
pub struct EventLog {
    entries: Vec<LogEntry>,
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub at: crate::core::Timestamp,
    pub method: String,
    pub path: String,
    pub status: Option<u16>,
    pub duration: Option<std::time::Duration>,
    pub attempt: u8,
    pub waited: Option<std::time::Duration>,
    pub refreshed: bool,
    pub detail: Option<String>,
    /// Present only under `--verbose`.
    pub body: Option<String>,
}

impl EventLog {
    /// Append-only, and every field is scrubbed of credentials on the way in.
    pub fn append(&mut self, entry: LogEntry) {
        let scrub = crate::core::redact::scrub;
        self.entries.push(LogEntry {
            path: scrub(&entry.path),
            detail: entry.detail.as_deref().map(scrub),
            body: entry.body.as_deref().map(scrub),
            ..entry
        });
    }

    pub fn entries(&self) -> &[LogEntry] {
        &self.entries
    }

    pub fn text(&self) -> String {
        (0..self.entries.len())
            .map(|index| self.line(index))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn line(&self, index: usize) -> String {
        let Some(entry) = self.entries.get(index) else {
            return String::new();
        };
        let mut line = format!(
            "{} {} {}",
            entry.at.format("%H:%M:%S"),
            entry.method,
            entry.path
        );
        if let Some(status) = entry.status {
            line.push_str(&format!(" {status}"));
        }
        if let Some(duration) = entry.duration {
            line.push_str(&format!(" {}ms", duration.as_millis()));
        }
        if entry.attempt > 1 {
            line.push_str(&format!(" attempt {}", entry.attempt));
        }
        if let Some(waited) = entry.waited {
            line.push_str(&format!(" waited {}ms", waited.as_millis()));
        }
        if entry.refreshed {
            line.push_str(" refreshed");
        }
        if let Some(detail) = &entry.detail {
            line.push_str(&format!(" — {detail}"));
        }
        if let Some(body) = &entry.body {
            line.push_str(&format!("\n{body}"));
        }
        line
    }
}

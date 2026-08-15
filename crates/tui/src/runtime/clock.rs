//! The wall clock, in the offset the terminal's user lives in.

pub struct SystemClock {
    offset: chrono::FixedOffset,
}

impl SystemClock {
    /// The local offset is resolved once, at construction.
    pub fn new() -> SystemClock {
        SystemClock {
            offset: *chrono::Local::now().offset(),
        }
    }
}

impl Default for SystemClock {
    fn default() -> SystemClock {
        SystemClock::new()
    }
}

impl crate::ports::Clock for SystemClock {
    fn now(&self) -> chrono::DateTime<chrono::FixedOffset> {
        chrono::Utc::now().with_timezone(&self.offset)
    }
}

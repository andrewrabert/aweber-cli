//! The pure core: state, the actions that change it, and the effects that
//! change anything else.

pub mod action;
pub mod cache;
pub mod collection;
pub mod effect;
pub mod event_log;
pub mod failure;
pub mod filter;
pub mod keys;
pub mod redact;
pub mod state;
pub mod update;
pub mod views;
pub mod visualization;

pub use action::Action;
pub use effect::Effect;
pub use state::State;
pub use update::update;

pub type Timestamp = chrono::DateTime<chrono::FixedOffset>;

/// What a send is cancelled by.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct RequestId(u64);

/// What the Event Log attributes an attempt to: the key the client stamped
/// every attempt of one logical request with.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct LogId(u64);

impl LogId {
    pub fn of(key: aweber::client::RequestKey) -> LogId {
        LogId(key.get())
    }
}

/// The id under its own name, so distinct requests can be named without the
/// client allocating them.
impl From<u64> for LogId {
    fn from(id: u64) -> LogId {
        LogId(id)
    }
}

impl std::fmt::Display for LogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Generation(u64);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct WatchId(u64);

impl RequestId {
    pub(crate) fn next(&mut self) -> RequestId {
        self.0 += 1;
        *self
    }
}

impl Generation {
    pub(crate) fn next(&mut self) -> Generation {
        self.0 += 1;
        *self
    }
}

impl WatchId {
    pub(crate) fn next(&mut self) -> WatchId {
        self.0 += 1;
        *self
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

//! Everything that can happen to the state, whether a keystroke or a response.

use crate::core::{collection, effect, failure, state};

#[derive(Clone, Debug)]
pub enum Action {
    Key(ratatui::crossterm::event::KeyEvent),
    Mouse(ratatui::crossterm::event::MouseEvent),
    Resize {
        width: u16,
        height: u16,
    },
    Tick,

    Move(Motion),
    /// The row, field, or tree node under a cell; the movement keys reach the
    /// same ones.
    Point {
        column: u16,
        row: u16,
    },
    Descend,
    Pop,
    /// The palette over the breadcrumb, on `Ctrl-b` or a click on the header.
    OpenAncestors,
    /// The palette over the running watches, on `Ctrl-w`.
    OpenWatches,
    /// The palette can jump to any ancestor of the stack.
    JumpTo {
        depth: usize,
    },
    Refresh,
    ToggleRaw,
    ToggleExpand,
    FocusNext,
    OpenPalette,
    OpenHelp,
    OpenEventLog,
    OpenFilter,
    OpenRawRequest,
    Copy,
    WriteFile,
    Confirm,
    Cancel,
    Quit,

    Type(char),
    Backspace,
    Submit,

    Launched,
    AccountsLoaded(Result<Vec<state::Account>, failure::Failure>),
    AccountChosen(state::Account),
    SessionStatus(crate::ports::SessionStatus),
    SessionFailed {
        message: String,
    },
    LoginStarted(crate::ports::Authorization),
    LoginFinished(Result<crate::ports::SessionStatus, String>),

    OpenOperation {
        operation: aweber::catalog::Operation,
        args: aweber::catalog::Args,
    },
    RunOperation {
        operation: aweber::catalog::Operation,
        args: aweber::catalog::Args,
    },
    Delivered {
        request: crate::core::RequestId,
        generation: crate::core::Generation,
        purpose: effect::Purpose,
        outcome: Result<collection::Delivered, failure::Failure>,
    },

    RequestStarted {
        request: crate::core::LogId,
        attempt: u8,
        method: String,
        path: String,
        body: Option<String>,
    },
    RequestWaiting {
        request: crate::core::LogId,
        attempt: u8,
        status: u16,
        wait: std::time::Duration,
    },
    RequestRefreshed {
        request: crate::core::LogId,
        attempt: u8,
    },
    RequestFinished {
        request: crate::core::LogId,
        attempt: u8,
        status: u16,
        duration: std::time::Duration,
        body: Option<String>,
    },
    /// No further attempt will be made: the request stops being outstanding and
    /// its method and path are forgotten.
    RequestSettled {
        request: crate::core::LogId,
    },

    Edited {
        purpose: effect::EditPurpose,
        text: Result<String, crate::ports::EditorError>,
    },
    Copied(Result<crate::ports::CopyRoute, crate::ports::CopyError>),
    Wrote(Result<std::path::PathBuf, String>),

    WatchPolled {
        watch: crate::core::WatchId,
        outcome: Result<collection::Delivered, failure::Failure>,
    },
    WatchFinished {
        watch: crate::core::WatchId,
        summary: String,
    },
    CancelWatch(crate::core::WatchId),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Motion {
    Down,
    Up,
    Top,
    Bottom,
    PageDown,
    PageUp,
}

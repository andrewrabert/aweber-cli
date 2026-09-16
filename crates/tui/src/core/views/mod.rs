//! One module per view, each holding only what that view remembers.

pub mod account_picker;
pub mod builder;
pub mod collection;
pub mod confirm;
pub mod detail;
pub mod event_log;
pub mod help;
pub mod home;
pub mod palette;
pub mod raw_request;
pub mod session;
pub mod tree;

/// The first row a list draws so that the selection is on screen.
pub(crate) fn follow(selected: usize, offset: usize, height: usize) -> usize {
    let height = height.max(1);
    if selected < offset {
        return selected;
    }
    if selected >= offset + height {
        return selected + 1 - height;
    }
    offset
}

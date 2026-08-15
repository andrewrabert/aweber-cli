//! A window onto the append-only log the state owns.

#[derive(Clone, Debug, Default)]
pub struct EventLogView {
    pub selected: usize,
    /// The first entry drawn, so a click at a cell names the entry under it.
    pub offset: usize,
}

impl EventLogView {
    pub fn follow(&mut self, height: usize) {
        self.offset = super::follow(self.selected, self.offset, height);
    }
}

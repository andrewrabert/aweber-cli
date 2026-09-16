//! A collection, its loaded rows, and why it ended.

use aweber::catalog::{Args, Operation};

pub struct CollectionView {
    pub operation: Operation,
    pub args: Args,
    pub title: String,
    pub rows: Vec<crate::core::collection::Row>,
    pub filter: Option<crate::core::filter::Filter>,
    pub selected: usize,
    pub offset: usize,
    pub generation: crate::core::Generation,
    pub cursor: Option<crate::core::collection::Continuation>,
    pub end: Option<crate::core::collection::EndReason>,
    pub total: Option<u64>,
    pub loading: bool,
    pub unavailable: Option<crate::core::failure::Failure>,
}

impl CollectionView {
    pub fn opening(
        operation: Operation,
        args: Args,
        title: String,
        generation: crate::core::Generation,
    ) -> CollectionView {
        CollectionView {
            operation,
            args,
            title,
            rows: Vec::new(),
            filter: None,
            selected: 0,
            offset: 0,
            generation,
            cursor: None,
            end: None,
            total: None,
            loading: true,
            unavailable: None,
        }
    }

    pub fn visible(&self) -> Vec<&crate::core::collection::Row> {
        match &self.filter {
            None => self.rows.iter().collect(),
            Some(filter) => filter
                .kept
                .iter()
                .filter_map(|i| self.rows.get(*i))
                .collect(),
        }
    }

    pub fn selected_row(&self) -> Option<&crate::core::collection::Row> {
        self.visible().get(self.selected).copied()
    }

    /// The first row drawn, so a click at a cell names the row under it.
    pub fn follow(&mut self, height: usize) {
        self.offset = super::follow(self.selected, self.offset, height);
    }

    /// "loaded 214" until the API supplies a total, then "214 of 900".
    pub fn count_label(&self) -> String {
        let loaded = self.rows.len();
        match self.total {
            Some(total) => format!("{loaded} of {total}"),
            None => format!("loaded {loaded}"),
        }
    }

    /// True once the selection is within a page of the tail.
    pub fn wants_next_page(&self) -> bool {
        const LOOKAHEAD: usize = 20;
        self.cursor.is_some() && !self.loading && self.selected + LOOKAHEAD >= self.visible().len()
    }
}

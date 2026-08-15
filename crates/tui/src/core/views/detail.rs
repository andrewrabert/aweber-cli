//! One document, as a field tree or as raw JSON.

use aweber::catalog::{Args, Operation};

pub struct DetailView {
    pub operation: Operation,
    pub args: Args,
    pub title: String,
    /// A response from an older generation is dropped rather than shown.
    pub generation: crate::core::Generation,
    /// What an enrichment names when it comes back.
    pub entity: Option<crate::core::state::EntityKey>,
    pub document: Option<serde_json::Value>,
    pub raw: bool,
    pub tree: tui_tree_widget::TreeState<String>,
    pub items: Vec<tui_tree_widget::TreeItem<'static, String>>,
    /// The first line of the raw JSON drawn, used only while `raw` is set.
    pub offset: usize,
    pub enrichment: Vec<Enrichment>,
    pub unavailable: Option<crate::core::failure::Failure>,
}

impl DetailView {
    pub fn opening(
        operation: Operation,
        args: Args,
        title: String,
        generation: crate::core::Generation,
    ) -> DetailView {
        DetailView {
            operation,
            args,
            title,
            generation,
            entity: None,
            document: None,
            raw: false,
            tree: tui_tree_widget::TreeState::default(),
            items: Vec::new(),
            offset: 0,
            enrichment: Vec::new(),
            unavailable: None,
        }
    }

    /// Builds the field tree in the given zone, selects its first row, and takes
    /// the document's id as the view's entity.
    pub fn show(&mut self, document: serde_json::Value, zone: chrono::FixedOffset) {
        self.items = crate::view::fields::tree_items(&document, zone);
        self.tree = tui_tree_widget::TreeState::default();
        if let Some(first) = self.items.first() {
            self.tree.select(vec![first.identifier().clone()]);
        }
        self.entity = document
            .pointer("/uuid")
            .or_else(|| document.pointer("/id"))
            .map(|id| crate::core::state::EntityKey {
                kind: crate::catalog::metadata(self.operation).entity,
                id: match id {
                    serde_json::Value::String(text) => text.clone(),
                    other => other.to_string(),
                },
            });
        self.document = Some(document);
        self.offset = 0;
    }

    /// An enrichment replaces the one of the same label, so a retry cannot
    /// double a row.
    pub fn enrich(&mut self, label: &'static str, value: Result<serde_json::Value, String>) {
        match self
            .enrichment
            .iter_mut()
            .find(|entry| entry.label == label)
        {
            Some(entry) => entry.value = value,
            None => self.enrichment.push(Enrichment { label, value }),
        }
    }

    pub fn raw_lines(&self) -> usize {
        self.document
            .as_ref()
            .map(|document| {
                serde_json::to_string_pretty(document)
                    .unwrap_or_else(|_| document.to_string())
                    .lines()
                    .count()
            })
            .unwrap_or(0)
    }

    /// The height the raw JSON is drawn in, the enrichment rows taken off the
    /// region the two share.
    pub fn body_height(&self, region: usize) -> usize {
        region.saturating_sub(self.enrichment.len()).max(1)
    }

    /// Keeps the raw-JSON scroll inside the document, clamped against the
    /// height the JSON is drawn in rather than against the whole region.
    pub fn follow(&mut self, region: usize) {
        let last = self.raw_lines().saturating_sub(self.body_height(region));
        self.offset = self.offset.min(last);
    }
}

/// An enrichment that failed states its reason in place of its value.
pub struct Enrichment {
    pub label: &'static str,
    pub value: Result<serde_json::Value, String>,
}

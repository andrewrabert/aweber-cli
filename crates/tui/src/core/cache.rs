//! Per-view results, keyed by operation and arguments, retained for the session.

use std::collections::BTreeMap;

use aweber::catalog::{Args, Operation};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CacheKey {
    pub operation: Operation,
    pub args: Args,
}

#[derive(Clone, Debug, Default)]
pub struct Cache {
    entries: BTreeMap<CacheKey, Cached>,
}

#[derive(Clone, Debug)]
pub struct Cached {
    pub rows: Vec<crate::core::collection::Row>,
    pub document: Option<serde_json::Value>,
    pub cursor: Option<crate::core::collection::Continuation>,
    pub end: Option<crate::core::collection::EndReason>,
    pub total: Option<u64>,
    pub selected: usize,
    pub offset: usize,
}

impl Cache {
    pub fn get(&self, key: &CacheKey) -> Option<&Cached> {
        self.entries.get(key)
    }

    pub fn put(&mut self, key: CacheKey, value: Cached) {
        self.entries.insert(key, value);
    }

    pub fn invalidate(&mut self, key: &CacheKey) {
        self.entries.remove(key);
    }

    /// Drops the entity's own entries and the collection it belongs to.
    pub fn invalidate_entity(&mut self, entity: &crate::core::state::EntityKey) {
        self.entries.retain(|key, cached| {
            let names_entity = key
                .args
                .iter()
                .any(|(_, values)| values.iter().any(|value| value.raw == entity.id));
            let holds_entity = cached.rows.iter().any(|row| {
                row.document
                    .pointer("/id")
                    .map(identifier)
                    .is_some_and(|id| id == entity.id)
            });
            !names_entity && !holds_entity
        });
    }

    pub fn invalidate_operation(&mut self, operation: Operation) {
        self.entries.retain(|key, _| key.operation != operation);
    }
}

fn identifier(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

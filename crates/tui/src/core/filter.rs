//! A filter over the rows already loaded, and never presented as more.

#[derive(Default)]
pub struct FilterPrompt {
    pub input: tui_input::Input,
}

#[derive(Clone, Debug)]
pub struct Filter {
    pub query: String,
    pub kept: Vec<usize>,
}

impl Filter {
    /// Matches only rows already loaded.
    pub fn apply(query: &str, rows: &[crate::core::collection::Row]) -> Filter {
        let needle = query.to_lowercase();
        let kept = rows
            .iter()
            .enumerate()
            .filter(|(_, row)| {
                needle.is_empty()
                    || row
                        .cells
                        .iter()
                        .any(|cell| cell.to_lowercase().contains(&needle))
            })
            .map(|(index, _)| index)
            .collect();
        Filter {
            query: query.to_string(),
            kept,
        }
    }

    /// "filter 3 of 214 loaded"
    pub fn label(&self, loaded: usize) -> String {
        format!("filter {} of {loaded} loaded", self.kept.len())
    }
}

//! The bindings in scope, generated from the dispatch table.

pub struct HelpView {
    pub rows: Vec<crate::core::keys::HelpRow>,
    pub offset: usize,
}

impl HelpView {
    pub fn open(scope: crate::core::keys::Scope) -> HelpView {
        HelpView {
            rows: crate::core::keys::help_rows(scope),
            offset: 0,
        }
    }
}

//! A workflow's ruleset as a tree.

pub struct TreeView {
    pub workflow: uuid::Uuid,
    pub title: String,
    /// A response from an older generation is dropped rather than shown.
    pub generation: crate::core::Generation,
    pub state: tui_tree_widget::TreeState<String>,
    pub items: Vec<tui_tree_widget::TreeItem<'static, String>>,
    pub unavailable: Option<crate::core::failure::Failure>,
}

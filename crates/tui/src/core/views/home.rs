//! The entity groups of the route table, and nothing else.

#[derive(Clone, Debug, Default)]
pub struct HomeView {
    pub selected: usize,
}

impl HomeView {
    /// The non-hidden groups of the route table.
    pub fn groups() -> &'static [&'static aweber::catalog::Group] {
        static GROUPS: std::sync::OnceLock<Vec<&'static aweber::catalog::Group>> =
            std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            aweber::catalog::groups()
                .iter()
                .filter(|group| !group.hidden)
                .collect()
        })
    }
}

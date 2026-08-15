//! The fuzzy palette, over whatever the current scope offers.

use aweber::catalog::ArgSpec;

pub struct PaletteView {
    pub input: tui_input::Input,
    pub scope: PaletteScope,
    pub matches: Vec<Match>,
    pub selected: usize,
}

pub enum PaletteScope {
    /// No selection: every routed operation.
    Everything,
    /// A selection: the operations valid for its entity kind, identifiers
    /// pre-filled.
    ForSelection(crate::catalog::EntityKind),
    /// The four hidden `oauth` operations plus login and logout.
    Session,
    Ancestors,
    /// The running watches; choosing one cancels it.
    Watches,
    Arguments(Vec<ArgSpec>),
    /// The loaded candidates of a builder row.
    Candidates {
        row: usize,
    },
}

pub struct Match {
    pub index: usize,
    pub score: u32,
    pub label: String,
    pub positions: Vec<u32>,
}

impl PaletteView {
    pub fn open(scope: PaletteScope) -> PaletteView {
        PaletteView {
            input: tui_input::Input::default(),
            scope,
            matches: Vec::new(),
            selected: 0,
        }
    }

    pub fn rescore(&mut self, candidates: &[String]) {
        let mut matcher =
            nucleo_matcher::Matcher::new(nucleo_matcher::Config::DEFAULT.match_paths());
        let query = self.input.value().to_string();
        let pattern = nucleo_matcher::pattern::Pattern::parse(
            &query,
            nucleo_matcher::pattern::CaseMatching::Ignore,
            nucleo_matcher::pattern::Normalization::Smart,
        );
        let mut haystack = Vec::new();
        self.matches = candidates
            .iter()
            .enumerate()
            .filter_map(|(index, label)| {
                let utf32 = nucleo_matcher::Utf32Str::new(label, &mut haystack);
                let mut positions = Vec::new();
                let score = pattern.indices(utf32, &mut matcher, &mut positions)?;
                positions.sort_unstable();
                positions.dedup();
                Some(Match {
                    index,
                    score,
                    label: label.clone(),
                    positions,
                })
            })
            .collect();
        self.matches.sort_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then(left.index.cmp(&right.index))
        });
        self.selected = self.selected.min(self.matches.len().saturating_sub(1));
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.matches.get(self.selected).map(|entry| entry.index)
    }
}

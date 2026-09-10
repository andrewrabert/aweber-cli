//! The progressive filter builder: typed rows, removable, previewed.

use aweber::catalog::ValueKind;
use aweber::catalog::{ArgRole, ArgSpec, ArgValue, Args, Operation, PlanError, RequestPlan};

pub struct BuilderView {
    pub operation: Operation,
    pub rows: Vec<BuilderRow>,
    pub selected: usize,
    pub picker: Option<super::palette::PaletteView>,
    pub preview: Result<RequestPlan, PlanError>,
    /// The generation every picker request of this builder carries, so a
    /// cancelled builder's candidates never reach the builder that replaced it.
    pub generation: crate::core::Generation,
}

pub struct BuilderRow {
    pub spec: ArgSpec,
    pub value: RowValue,
    pub removable: bool,
}

pub enum RowValue {
    Text(tui_input::Input),
    Number(tui_input::Input),
    Date(tui_input::Input),
    Bool(bool),
    Enum {
        options: Vec<String>,
        chosen: usize,
    },
    /// Resolves a name to a `self_link`, a uuid, or an id through a picker.
    Pick {
        picker: crate::catalog::Picker,
        query: tui_input::Input,
        /// Loaded once, when the row appears; filtered locally after that.
        candidates: Vec<PickCandidate>,
        /// Typed text becomes a choice only where the resolution accepts it as
        /// already resolved.
        chosen: Option<PickCandidate>,
    },
    /// Entered through `$EDITOR` only.
    Json {
        text: Option<String>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PickCandidate {
    pub label: String,
    pub value: ArgValue,
}

impl BuilderView {
    pub fn open(
        operation: Operation,
        prefilled: Args,
        generation: crate::core::Generation,
    ) -> BuilderView {
        let rows = crate::catalog::opening_rows(operation)
            .into_iter()
            .map(|spec| {
                let seed = prefilled.first(&spec.name).cloned();
                BuilderRow {
                    value: row_value(operation, &spec, seed),
                    removable: !spec.required,
                    spec,
                }
            })
            .collect();
        BuilderView {
            operation,
            rows,
            selected: 0,
            picker: None,
            preview: Err(PlanError::MissingArgument {
                name: "account".to_string(),
            }),
            generation,
        }
    }

    /// The request the rows currently describe.
    pub fn reprice(&mut self, scope: Option<aweber::catalog::Scope>) {
        self.preview = match scope {
            Some(scope) => RequestPlan::build(self.operation, &scope, &self.args()),
            None => Err(PlanError::MissingArgument {
                name: "account".to_string(),
            }),
        };
    }

    pub fn args(&self) -> Args {
        let mut args = Args::default();
        for row in &self.rows {
            if let Some(value) = row_arg(row) {
                args.set(row.spec.name.clone(), value);
            }
        }
        args
    }

    pub fn add(&mut self, spec: ArgSpec) {
        let value = row_value(self.operation, &spec, None);
        self.rows.push(BuilderRow {
            spec,
            value,
            removable: true,
        });
    }

    pub fn remove_selected(&mut self) {
        if self
            .rows
            .get(self.selected)
            .is_some_and(|row| row.removable)
        {
            self.rows.remove(self.selected);
            self.selected = self.selected.min(self.rows.len().saturating_sub(1));
        }
    }

    /// The rows whose candidates have not been asked for yet.
    pub fn unloaded_pickers(&self) -> Vec<(usize, crate::catalog::Picker)> {
        self.rows
            .iter()
            .enumerate()
            .filter_map(|(index, row)| match &row.value {
                RowValue::Pick {
                    picker, candidates, ..
                } if candidates.is_empty() => Some((index, *picker)),
                _ => None,
            })
            .collect()
    }

    /// The candidates a picker row offers, narrowed by what has been typed;
    /// `Enter` on an unresolved picker row offers them instead of submitting.
    pub fn candidates(&self, row: usize) -> Vec<&PickCandidate> {
        let Some(BuilderRow {
            value: RowValue::Pick {
                query, candidates, ..
            },
            ..
        }) = self.rows.get(row)
        else {
            return Vec::new();
        };
        let typed = query.value().to_lowercase();
        candidates
            .iter()
            .filter(|candidate| candidate.label.to_lowercase().contains(&typed))
            .collect()
    }

    /// The rows not yet present, offered by the fuzzy picker.
    pub fn addable(&self) -> Vec<ArgSpec> {
        self.operation
            .specs()
            .into_iter()
            .filter(|spec| {
                !matches!(spec.role, ArgRole::Excluded | ArgRole::Pagination)
                    && !self.rows.iter().any(|row| row.spec.name == spec.name)
            })
            .collect()
    }
}

fn row_value(operation: Operation, spec: &ArgSpec, seed: Option<ArgValue>) -> RowValue {
    let text = seed
        .as_ref()
        .map(|value| value.raw.clone())
        .or_else(|| spec.default.clone())
        .unwrap_or_default();
    if let Some(picker) = crate::catalog::picker(operation, &spec.name) {
        return RowValue::Pick {
            picker,
            query: tui_input::Input::new(text.clone()),
            candidates: Vec::new(),
            chosen: picker.resolution.accepts(&text).then(|| PickCandidate {
                label: text.clone(),
                value: ArgValue::new(spec.kind.clone(), text),
            }),
        };
    }
    match &spec.kind {
        ValueKind::Bool | ValueKind::Flag => RowValue::Bool(text == "true"),
        ValueKind::Integer | ValueKind::Decimal => RowValue::Number(tui_input::Input::new(text)),
        ValueKind::Date | ValueKind::DateTime => RowValue::Date(tui_input::Input::new(text)),
        ValueKind::Json => RowValue::Json {
            text: seed.map(|value| value.raw),
        },
        ValueKind::Enum(options) => {
            let chosen = options
                .iter()
                .position(|option| *option == text)
                .unwrap_or(0);
            RowValue::Enum {
                options: options.clone(),
                chosen,
            }
        }
        _ => RowValue::Text(tui_input::Input::new(text)),
    }
}

fn row_arg(row: &BuilderRow) -> Option<ArgValue> {
    let kind = row.spec.kind.clone();
    match &row.value {
        RowValue::Text(input) | RowValue::Number(input) | RowValue::Date(input) => {
            let text = input.value();
            (!text.is_empty()).then(|| ArgValue::new(kind, text))
        }
        RowValue::Bool(set) => Some(ArgValue::new(kind, set.to_string())),
        RowValue::Enum { options, chosen } => options
            .get(*chosen)
            .map(|option| ArgValue::new(kind, option.clone())),
        RowValue::Pick { chosen, .. } => chosen.as_ref().map(|candidate| candidate.value.clone()),
        RowValue::Json { text } => text.clone().map(|text| ArgValue::new(kind, text)),
    }
}

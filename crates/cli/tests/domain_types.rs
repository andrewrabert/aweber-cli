use std::path::{Path, PathBuf};

const SCANNED: [&str; 2] = ["../api/src/workflows", "src/workflows"];

const EDGES: [&str; 5] = [
    "ruleset.rs",
    "wire.rs",
    "patch.rs",
    "campaign.rs",
    "object.rs",
];

const BANNED: [&str; 11] = [
    "i32",
    "&str",
    "String",
    "uuid::Uuid",
    "serde_json::Value",
    "Vec<String>",
    "serde_json::Map",
    "Cow<'_, str>",
    "f64",
    "Option<bool>",
    "&[String]",
];

const BANNED_METHODS: [&str; 13] = [
    "as_str",
    "into_inner",
    "get",
    "as_uuid",
    "to_uuid",
    "is_",
    "has_",
    "is_zero",
    "is_empty",
    "header_value",
    "new_v4",
    "from_uuid",
    "mint",
];

const WIRE_TYPES: [&str; 10] = [
    "Rule",
    "RuleFilter",
    "Criterion",
    "Definition",
    "RuleBranch",
    "FunctionName",
    "Dispatch",
    "PreservedRule",
    "serde_json::Value",
    "serde_json::Map",
];

const WIRE_READERS: [&str; 1] = ["read"];

const VISIBILITIES: [&str; 3] = ["pub ", "pub(crate) ", "pub(super) "];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn scanned_files() -> Vec<PathBuf> {
    let mut found = Vec::new();
    for tree in SCANNED {
        collect(&root().join(tree), &mut found);
    }
    found.sort();
    found
}

fn collect(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, found);
        } else if path.extension().is_some_and(|kind| kind == "rs")
            && path.file_name().is_some_and(|name| name != "schema.rs")
        {
            found.push(path);
        }
    }
}

fn is_edge(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    EDGES.contains(&name)
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

fn declarations(source: &str) -> Vec<&str> {
    source
        .lines()
        .map(str::trim)
        .filter(|line| VISIBILITIES.iter().any(|named| line.starts_with(named)))
        .collect()
}

fn debt() -> Vec<String> {
    let text = read(&root().join("tests/domain_types_debt.txt"));
    text.lines()
        .map(|line| {
            line.split('#')
                .next()
                .unwrap_or_default()
                .trim()
                .to_string()
        })
        .filter(|line| !line.is_empty())
        .collect()
}

fn entry_parts(entry: &str) -> Option<(&str, &str)> {
    let (claimed, rest) = entry.split_once(".rs::")?;
    Some((claimed, rest))
}

fn claimed_by_debt(path: &Path) -> bool {
    debt().iter().any(|entry| {
        entry_parts(entry).is_some_and(|(claimed, _)| {
            path.to_string_lossy()
                .ends_with(format!("{claimed}.rs").trim_start_matches("crates/"))
        })
    })
}

fn excused(path: &Path, line: &str) -> bool {
    debt().iter().any(|entry| {
        let Some((claimed, named)) = entry_parts(entry) else {
            return false;
        };
        let file = format!("{claimed}.rs");
        let last = named.rsplit("::").next().unwrap_or(named);
        path.to_string_lossy()
            .ends_with(file.trim_start_matches("crates/"))
            && line.contains(last)
    })
}

#[test]
fn no_workflows_signature_names_a_base_type() {
    let mut found = Vec::new();
    for path in scanned_files() {
        if is_edge(&path) {
            continue;
        }
        let source = read(&path);
        for line in declarations(&source) {
            if !line.contains("fn ") {
                continue;
            }
            for named in BANNED {
                if line.contains(named) && !excused(&path, line) {
                    found.push(format!("{}: {line}", path.display()));
                }
            }
        }
    }
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn no_type_outside_an_edge_names_its_representation() {
    let mut found = Vec::new();
    for path in scanned_files() {
        if is_edge(&path) {
            continue;
        }
        let source = read(&path);
        for named in BANNED_METHODS {
            let declared = if named.ends_with('_') {
                format!("fn {named}")
            } else {
                format!("fn {named}(")
            };
            if source.contains(&declared) {
                found.push(format!("{}: {declared}", path.display()));
            }
        }
    }
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn no_interior_module_names_the_wire() {
    let mut found = Vec::new();
    for path in scanned_files() {
        if is_edge(&path) {
            continue;
        }
        let source = read(&path);
        for named in WIRE_TYPES {
            if named.starts_with("serde_json") || named == "PreservedRule" {
                continue;
            }
            let wired = format!("wire::{named}");
            if source.contains(&wired) {
                found.push(format!("{}: {wired}", path.display()));
            }
        }
        for named in WIRE_READERS {
            let wired = format!("wire::{named}");
            if source.contains(&wired) {
                found.push(format!("{}: {wired}", path.display()));
            }
        }
        if !claimed_by_debt(&path) {
            for line in source.lines() {
                let names_json =
                    line.contains("serde_json::Value") || line.contains("serde_json::Map");
                if names_json {
                    found.push(format!("{}: {}", path.display(), line.trim()));
                }
            }
        }
    }
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn no_id_type_offers_a_public_fresh_id_constructor() {
    let source = read(&root().join("../api/src/ids.rs"));
    assert_eq!(source.matches("pub(crate) fn new()").count(), 2, "{source}");
    assert!(!source.contains("pub fn new("), "{source}");
    assert_eq!(
        source.matches("#[cfg(feature = \"workflows\")]").count(),
        2,
        "only the two `new` impl blocks are gated"
    );
}

#[test]
fn no_id_type_or_its_constructor_names_a_representation() {
    let mut found = Vec::new();
    for tree in ["../api/src", "src"] {
        let mut files = Vec::new();
        collect(&root().join(tree), &mut files);
        for path in files {
            let source = read(&path);
            for named in ["fn new_v4", "fn from_uuid", "AccountUuid", "ListUuid"] {
                if source.contains(named) {
                    found.push(format!("{}: {named}", path.display()));
                }
            }
        }
    }
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn no_field_outside_the_patch_carve_out_is_named_for_the_wire() {
    let mut found = Vec::new();
    for path in scanned_files() {
        if path.ends_with("patch.rs") {
            continue;
        }
        let source = read(&path);
        for line in source.lines().map(str::trim) {
            let field = line.starts_with("value:")
                || line.starts_with("data:")
                || line.starts_with("input:")
                || line.starts_with("pub value:")
                || line.starts_with("pub data:")
                || line.starts_with("pub input:");
            if field {
                found.push(format!("{}: {line}", path.display()));
            }
        }
    }
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn the_debt_file_holds_no_stale_entry() {
    let entries = debt();
    assert!(!entries.is_empty(), "the debt file names its entries");
    for entry in entries {
        let (claimed, name) =
            entry_parts(&entry).unwrap_or_else(|| panic!("'{entry}' is <path>::<name>"));
        let name = name.rsplit("::").next().unwrap_or(name);
        let path = root()
            .join("..")
            .join(format!("{claimed}.rs").trim_start_matches("crates/"));
        let source = read(&path);
        assert!(
            source.contains(name),
            "'{entry}' no longer resolves in {}",
            path.display()
        );
    }
}

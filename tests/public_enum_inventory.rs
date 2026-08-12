use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const INVENTORY: &str = include_str!("fixtures/i02-c01-public-enums.tsv");
const CLOSED_ENUMS: [&str; 2] = [
    "src/conformance.rs:CssSupportStatus",
    "src/syntax.rs:CssImportance",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvolutionPolicy {
    Evolving,
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SourceEnum {
    non_exhaustive: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Token {
    text: String,
}

fn parse_inventory(input: &str) -> Result<BTreeMap<String, EvolutionPolicy>, String> {
    let mut lines = input.lines();
    if lines.next() != Some("path:item\tpolicy") {
        return Err("inventory header must be exactly `path:item\\tpolicy`".to_owned());
    }

    let mut inventory = BTreeMap::new();
    for (offset, line) in lines.enumerate() {
        let line_number = offset + 2;
        let Some((id, policy)) = line.split_once('\t') else {
            return Err(format!("inventory line {line_number} has no policy column"));
        };
        if id.is_empty() || !id.contains(':') || policy.contains('\t') {
            return Err(format!("inventory line {line_number} is malformed"));
        }
        let policy = match policy {
            "evolving" => EvolutionPolicy::Evolving,
            "closed" => EvolutionPolicy::Closed,
            other => return Err(format!("{id}: unknown evolution policy `{other}`")),
        };
        if inventory.insert(id.to_owned(), policy).is_some() {
            return Err(format!("{id}: duplicate inventory item"));
        }
    }
    Ok(inventory)
}

fn rust_files_under(directory: &Path) -> Vec<PathBuf> {
    fn visit(directory: &Path, files: &mut Vec<PathBuf>) {
        let mut entries = fs::read_dir(directory)
            .unwrap_or_else(|error| panic!("cannot read {}: {error}", directory.display()))
            .map(|entry| entry.expect("source directory entry"))
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                visit(&path, files);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                files.push(path);
            }
        }
    }

    let mut files = Vec::new();
    visit(directory, &mut files);
    files
}

fn tokenize_rust(source: &str) -> Vec<Token> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index].is_ascii_whitespace() {
            index += 1;
            continue;
        }
        if bytes[index..].starts_with(b"//") {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            continue;
        }
        if bytes[index..].starts_with(b"/*") {
            index += 2;
            let mut depth = 1usize;
            while index < bytes.len() && depth != 0 {
                if bytes[index..].starts_with(b"/*") {
                    depth += 1;
                    index += 2;
                } else if bytes[index..].starts_with(b"*/") {
                    depth -= 1;
                    index += 2;
                } else {
                    index += 1;
                }
            }
            continue;
        }

        let raw_start = match bytes[index] {
            b'r' => Some(index),
            b'b' if bytes.get(index + 1) == Some(&b'r') => Some(index + 1),
            _ => None,
        };
        if let Some(raw_start) = raw_start {
            let mut cursor = raw_start + 1;
            let mut hashes = 0usize;
            while bytes.get(cursor) == Some(&b'#') {
                hashes += 1;
                cursor += 1;
            }
            if bytes.get(cursor) == Some(&b'"') {
                cursor += 1;
                while cursor < bytes.len() {
                    if bytes[cursor] == b'"'
                        && bytes.get(cursor + 1..cursor + 1 + hashes) == Some(&vec![b'#'; hashes])
                    {
                        index = cursor + 1 + hashes;
                        break;
                    }
                    cursor += 1;
                }
                if cursor >= bytes.len() {
                    index = bytes.len();
                }
                continue;
            }
        }

        let quote_index = if bytes[index] == b'"' {
            Some(index)
        } else if bytes[index] == b'b' && bytes.get(index + 1) == Some(&b'"') {
            Some(index + 1)
        } else {
            None
        };
        if let Some(quote_index) = quote_index {
            index = quote_index + 1;
            while index < bytes.len() {
                if bytes[index] == b'\\' {
                    index += 2;
                } else if bytes[index] == b'"' {
                    index += 1;
                    break;
                } else {
                    index += 1;
                }
            }
            continue;
        }

        if bytes[index].is_ascii_alphabetic() || bytes[index] == b'_' {
            let start = index;
            index += 1;
            while index < bytes.len()
                && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
            {
                index += 1;
            }
            tokens.push(Token {
                text: source[start..index].to_owned(),
            });
        } else {
            tokens.push(Token {
                text: char::from(bytes[index]).to_string(),
            });
            index += 1;
        }
    }

    tokens
}

fn public_enums_in_source(path: &str, source: &str) -> BTreeMap<String, SourceEnum> {
    let tokens = tokenize_rust(source);
    let mut enums = BTreeMap::new();
    for index in 0..tokens.len().saturating_sub(2) {
        if tokens[index].text != "pub" || tokens[index + 1].text != "enum" {
            continue;
        }
        let name = &tokens[index + 2].text;
        let start = tokens[..index]
            .iter()
            .rposition(|token| matches!(token.text.as_str(), ";" | "}" | "{"))
            .map_or(0, |position| position + 1);
        let non_exhaustive = tokens[start..index]
            .iter()
            .any(|token| token.text == "non_exhaustive");
        let id = format!("{path}:{name}");
        assert!(
            enums
                .insert(id.clone(), SourceEnum { non_exhaustive })
                .is_none(),
            "{id}: duplicate public enum declaration"
        );
    }
    enums
}

fn discover_owned_public_enums(root: &Path) -> BTreeMap<String, SourceEnum> {
    let mut enums = BTreeMap::new();
    for path in rust_files_under(&root.join("src")) {
        let relative = path
            .strip_prefix(root)
            .expect("source beneath manifest directory")
            .to_string_lossy()
            .replace('\\', "/");
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
        for (id, declaration) in public_enums_in_source(&relative, &source) {
            assert!(enums.insert(id.clone(), declaration).is_none(), "{id}");
        }
    }
    enums
}

fn validate_inventory(
    inventory: &BTreeMap<String, EvolutionPolicy>,
    source: &BTreeMap<String, SourceEnum>,
) -> Result<(), String> {
    let mut failures = Vec::new();
    for id in source.keys() {
        if !inventory.contains_key(id) {
            failures.push(format!("{id}: omitted from inventory"));
        }
    }
    for id in inventory.keys() {
        if !source.contains_key(id) {
            failures.push(format!("{id}: extra inventory item"));
        }
    }

    let declared_closed = inventory
        .iter()
        .filter_map(|(id, policy)| (*policy == EvolutionPolicy::Closed).then_some(id.as_str()))
        .collect::<BTreeSet<_>>();
    let required_closed = CLOSED_ENUMS.into_iter().collect::<BTreeSet<_>>();
    for id in required_closed.difference(&declared_closed) {
        failures.push(format!("{id}: required closed exception is not closed"));
    }
    for id in declared_closed.difference(&required_closed) {
        failures.push(format!("{id}: unauthorized closed exception"));
    }

    for (id, policy) in inventory {
        let Some(declaration) = source.get(id) else {
            continue;
        };
        match policy {
            EvolutionPolicy::Evolving if !declaration.non_exhaustive => {
                failures.push(format!("{id}: evolving enum lacks #[non_exhaustive]"));
            }
            EvolutionPolicy::Closed if declaration.non_exhaustive => {
                failures.push(format!("{id}: closed enum has #[non_exhaustive]"));
            }
            EvolutionPolicy::Evolving | EvolutionPolicy::Closed => {}
        }
    }

    failures.sort();
    failures.dedup();
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}

#[test]
fn inventory_closes_over_every_owned_public_enum_and_evolution_policy() {
    let inventory = parse_inventory(INVENTORY).expect("valid checked-in public enum inventory");
    let source = discover_owned_public_enums(Path::new(env!("CARGO_MANIFEST_DIR")));
    validate_inventory(&inventory, &source).unwrap_or_else(|failure| panic!("{failure}"));
}

#[test]
fn inventory_mutations_report_stable_item_identity() {
    let inventory = parse_inventory(INVENTORY).expect("valid checked-in public enum inventory");
    let source = discover_owned_public_enums(Path::new(env!("CARGO_MANIFEST_DIR")));

    let omitted_id = "src/syntax.rs:CssMediaType";
    let mut omitted = inventory.clone();
    assert_eq!(omitted.remove(omitted_id), Some(EvolutionPolicy::Evolving));
    assert!(
        validate_inventory(&omitted, &source)
            .expect_err("omission mutation must fail")
            .contains(omitted_id)
    );

    let extra_id = "src/syntax.rs:CssInventedInventoryMutation";
    let mut extra = inventory.clone();
    assert_eq!(
        extra.insert(extra_id.to_owned(), EvolutionPolicy::Evolving),
        None
    );
    assert!(
        validate_inventory(&extra, &source)
            .expect_err("extra mutation must fail")
            .contains(extra_id)
    );

    let wrong_exception_id = "src/syntax.rs:CssMediaType";
    let mut wrong_exception = inventory;
    assert_eq!(
        wrong_exception.insert(wrong_exception_id.to_owned(), EvolutionPolicy::Closed),
        Some(EvolutionPolicy::Evolving)
    );
    assert!(
        validate_inventory(&wrong_exception, &source)
            .expect_err("wrong-exception mutation must fail")
            .contains(wrong_exception_id)
    );
}

#[test]
fn source_scanner_includes_nested_macro_and_feature_gated_public_enums() {
    let source = r#"
        mod nested {
            #[non_exhaustive]
            pub enum CssNested { Value }
        }
        macro_rules! generated {
            () => {
                #[non_exhaustive]
                pub enum CssGenerated { Value }
            };
        }
        #[cfg(feature = "app-strict")]
        #[non_exhaustive]
        pub enum CssFeatureGated { Value }
    "#;
    let discovered = public_enums_in_source("src/synthetic.rs", source);
    assert_eq!(
        discovered.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "src/synthetic.rs:CssFeatureGated",
            "src/synthetic.rs:CssGenerated",
            "src/synthetic.rs:CssNested",
        ]
    );
    assert!(discovered.values().all(|item| item.non_exhaustive));
}

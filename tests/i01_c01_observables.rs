use std::collections::{BTreeMap, BTreeSet};

use surgeist_css::{
    CssDeclaration, CssDeclarationContextRef, CssErrorCode, CssImportance, CssPropertyNameRef,
    CssRecoveryAction, CssRecoveryDiagnostic, CssRule, CssScopedRule, ErrorKind, parse_sheet,
    parse_style_attribute,
};

#[path = "catalog_inventory/vectors.rs"]
mod catalog_vectors;
#[path = "i01_c01_observables/cases.rs"]
mod i01_cases;

use i01_cases::{
    Case, EntryPoint, FeatureMode, STABLE_CASE_OWNERS, focused_cases, non_property_cases,
};

const EXACT_CASE_COUNT: usize = 974;
const EXACT_DEFAULT_CASE_COUNT: usize = 962;
const EXACT_APP_STRICT_CASE_COUNT: usize = 974;
const EXACT_IDENTITY_FINGERPRINT: u64 = 0x16c4_697e_8010_aa3c;
const EXACT_OWNER_COUNTS: [(&str, usize); 16] = [
    ("app_strict_parity", 12),
    ("authored_declaration_values", 10),
    ("catalog_inventory", 358),
    ("conformance_catalog", 72),
    ("coupled_declarations", 6),
    ("declaration_importance", 8),
    ("initiative_i01_audit", 8),
    ("nested_structural_recovery", 12),
    ("property_schema", 358),
    ("public_surface", 15),
    ("source_coordinates", 6),
    ("specialized_list_recovery", 24),
    ("structural_recovery_adversarial", 24),
    ("structured_errors", 19),
    ("style_attribute_recovery", 19),
    ("stylesheet_recovery", 23),
];

const FIXTURE: &str = include_str!("fixtures/i01-c01-observables.tsv");
const HEADER: &str = "case_id\towner\tentry\tfeature\tinput\tclean\tretained\tvalues\tauthored_declarations\tdiagnostics";

#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    case_id: String,
    owner: String,
    entry: String,
    feature: String,
    input: String,
    clean: String,
    retained: String,
    values: String,
    authored_declarations: String,
    diagnostics: String,
}

impl Row {
    fn fields(&self) -> [&str; 10] {
        [
            &self.case_id,
            &self.owner,
            &self.entry,
            &self.feature,
            &self.input,
            &self.clean,
            &self.retained,
            &self.values,
            &self.authored_declarations,
            &self.diagnostics,
        ]
    }
}

fn parse_fixture(source: &str) -> Result<Vec<Row>, String> {
    let mut lines = source.lines();
    let header = lines.next().ok_or("fixture is empty")?;
    if header != HEADER {
        return Err(format!("unknown or reordered columns: `{header}`"));
    }

    let mut rows: Vec<Row> = Vec::new();
    let mut ids = BTreeSet::new();
    for (line_index, line) in lines.enumerate() {
        if line.is_empty() {
            return Err(format!("blank row at fixture line {}", line_index + 2));
        }
        let raw = line.split('\t').collect::<Vec<_>>();
        if raw.len() != 10 {
            return Err(format!(
                "fixture line {} has {} columns, expected 10",
                line_index + 2,
                raw.len()
            ));
        }
        let fields = raw
            .into_iter()
            .map(unescape)
            .collect::<Result<Vec<_>, _>>()?;
        if fields
            .iter()
            .enumerate()
            .any(|(index, field)| index != 4 && field.is_empty())
        {
            return Err(format!(
                "absent required observable at fixture line {}",
                line_index + 2
            ));
        }
        let row = Row {
            case_id: fields[0].clone(),
            owner: fields[1].clone(),
            entry: fields[2].clone(),
            feature: fields[3].clone(),
            input: fields[4].clone(),
            clean: fields[5].clone(),
            retained: fields[6].clone(),
            values: fields[7].clone(),
            authored_declarations: fields[8].clone(),
            diagnostics: fields[9].clone(),
        };
        if !ids.insert(row.case_id.clone()) {
            return Err(format!("duplicate case ID `{}`", row.case_id));
        }
        if let Some(previous) = rows.last()
            && previous.case_id >= row.case_id
        {
            return Err(format!(
                "noncanonical case order: `{}` before `{}`",
                previous.case_id, row.case_id
            ));
        }
        match row.entry.as_str() {
            "sheet" | "style" => {}
            value => return Err(format!("{}: unknown entry point `{value}`", row.case_id)),
        }
        match row.feature.as_str() {
            "both" | "default" | "app-strict" => {}
            value => return Err(format!("{}: unknown feature mode `{value}`", row.case_id)),
        }
        match row.clean.as_str() {
            "true" | "false" => {}
            value => return Err(format!("{}: invalid clean state `{value}`", row.case_id)),
        }
        validate_authored_field(&row)?;
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("fixture has no cases".to_owned());
    }
    Ok(rows)
}

fn validate_authored_field(row: &Row) -> Result<(), String> {
    let retained = row
        .retained
        .split('~')
        .filter_map(|item| item.strip_prefix("property:"))
        .collect::<Vec<_>>();
    let authored = parse_authored_declarations(&row.authored_declarations, &row.case_id)?;
    let authored_ids = authored
        .iter()
        .map(|declaration| declaration.id.as_str())
        .collect::<Vec<_>>();
    if retained != authored_ids {
        return Err(format!(
            "{}: retained/authored declaration identity mismatch",
            row.case_id
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AuthoredDeclaration<'a> {
    id: String,
    value_capability: &'a str,
    value: &'a str,
    importance_capability: &'a str,
    importance: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FrozenSemanticValue<'a> {
    id: &'a str,
    payload: &'a str,
    importance: &'a str,
}

fn parse_semantic_values<'a>(
    field: &'a str,
    case_id: &str,
) -> Result<Vec<FrozenSemanticValue<'a>>, String> {
    if field == "-" {
        return Ok(Vec::new());
    }
    field
        .split('~')
        .map(|item| {
            let (id, observation) = item
                .split_once('=')
                .ok_or_else(|| format!("{case_id}: malformed semantic value `{item}`"))?;
            let (payload, importance) = observation
                .rsplit_once('@')
                .ok_or_else(|| format!("{case_id}: missing semantic importance in `{item}`"))?;
            if !matches!(importance, "normal" | "important") {
                return Err(format!(
                    "{case_id}: invalid semantic importance `{importance}`"
                ));
            }
            Ok(FrozenSemanticValue {
                id,
                payload,
                importance,
            })
        })
        .collect()
}

struct FrozenDeclarationCursor<'a> {
    case_id: &'a str,
    semantic_values: Vec<FrozenSemanticValue<'a>>,
    authored_declarations: Vec<AuthoredDeclaration<'a>>,
    semantic_index: usize,
    authored_index: usize,
    wrapper_comparisons: usize,
    prior_public_comparisons: usize,
}

impl<'a> FrozenDeclarationCursor<'a> {
    fn new(row: &'a Row) -> Self {
        Self {
            case_id: &row.case_id,
            semantic_values: parse_semantic_values(&row.values, &row.case_id)
                .expect("frozen semantic-value expectation"),
            authored_declarations: parse_authored_declarations(
                &row.authored_declarations,
                &row.case_id,
            )
            .expect("frozen authored-declaration expectation"),
            semantic_index: 0,
            authored_index: 0,
            wrapper_comparisons: 0,
            prior_public_comparisons: 0,
        }
    }

    fn next(
        &mut self,
        includes_semantic_value: bool,
    ) -> (Option<FrozenSemanticValue<'a>>, AuthoredDeclaration<'a>) {
        let authored = self
            .authored_declarations
            .get(self.authored_index)
            .unwrap_or_else(|| panic!("{}: unexpected retained declaration", self.case_id))
            .clone();
        self.authored_index += 1;
        let semantic = includes_semantic_value.then(|| {
            let value = *self
                .semantic_values
                .get(self.semantic_index)
                .unwrap_or_else(|| panic!("{}: missing frozen semantic value", self.case_id));
            self.semantic_index += 1;
            value
        });
        (semantic, authored)
    }

    fn finish(&self) {
        assert_eq!(
            self.semantic_index,
            self.semantic_values.len(),
            "{} semantic value count",
            self.case_id
        );
        assert_eq!(
            self.authored_index,
            self.authored_declarations.len(),
            "{} authored declaration count",
            self.case_id
        );
    }
}

fn parse_authored_declarations<'a>(
    field: &'a str,
    case_id: &str,
) -> Result<Vec<AuthoredDeclaration<'a>>, String> {
    if field == "-" {
        return Ok(Vec::new());
    }
    field
        .split('~')
        .map(|item| {
            let (id, observation) = item
                .split_once('=')
                .ok_or_else(|| format!("{case_id}: malformed authored declaration `{item}`"))?;
            let (value_observation, importance_observation) = observation
                .rsplit_once('@')
                .ok_or_else(|| format!("{case_id}: missing authored importance in `{item}`"))?;
            let (value_capability, value) = value_observation
                .split_once(':')
                .ok_or_else(|| format!("{case_id}: missing value capability in `{item}`"))?;
            let (importance_capability, importance) = importance_observation
                .split_once(':')
                .ok_or_else(|| format!("{case_id}: missing importance capability in `{item}`"))?;
            if !matches!(value_capability, "public" | "deferred-i01") {
                return Err(format!(
                    "{case_id}: unknown authored-value capability `{value_capability}`"
                ));
            }
            if !matches!(importance_capability, "public" | "keyframe-grammar") {
                return Err(format!(
                    "{case_id}: unknown importance capability `{importance_capability}`"
                ));
            }
            if !matches!(importance, "normal" | "important")
                || (importance_capability == "keyframe-grammar" && importance != "normal")
            {
                return Err(format!(
                    "{case_id}: invalid authored importance `{importance_observation}`"
                ));
            }
            Ok(AuthoredDeclaration {
                id: id.to_owned(),
                value_capability,
                value,
                importance_capability,
                importance,
            })
        })
        .collect()
}

fn unescape(field: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut chars = field.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        match chars.next() {
            Some('\\') => output.push('\\'),
            Some('t') => output.push('\t'),
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some(escaped) => return Err(format!("malformed escape `\\{escaped}`")),
            None => return Err("trailing fixture escape".to_owned()),
        }
    }
    Ok(output)
}

fn expected_cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for vector in catalog_vectors::PROPERTY_POSITIVE_VECTORS {
        cases.push(Case::new(
            format!("catalog.property.{}.positive", vector.id),
            format!("catalog_inventory::{}/positive", vector.id),
            EntryPoint::Style,
            FeatureMode::Both,
            format!("{}: {}", vector.canonical_name, vector.authored_value),
        ));
    }
    for vector in catalog_vectors::PROPERTY_NEGATIVE_VECTORS {
        cases.push(Case::new(
            format!("catalog.property.{}.boundary", vector.id),
            format!("catalog_inventory::{}/negative", vector.id),
            EntryPoint::Style,
            FeatureMode::Both,
            format!("{}: {}", vector.canonical_name, vector.authored_value),
        ));
    }
    for vector in catalog_vectors::PROPERTY_POSITIVE_VECTORS {
        cases.push(Case::new(
            format!("focused.property-schema.{}.ordinary", vector.id),
            format!("property_schema::property_schema_parser_identity_matches_every_frozen_name/{}/ordinary", vector.id),
            EntryPoint::Sheet,
            FeatureMode::Both,
            format!(".test {{ {}: {}; }}", vector.canonical_name.to_ascii_uppercase(), vector.authored_value),
        ));
        cases.push(Case::new(
            format!("focused.property-schema.{}.important", vector.id),
            format!("property_schema::property_schema_parser_identity_matches_every_frozen_name/{}/important", vector.id),
            EntryPoint::Sheet,
            FeatureMode::Both,
            format!(".test {{ {}: {} !important; }}", vector.canonical_name.to_ascii_uppercase(), if vector.canonical_name == "all" { "inherit" } else { vector.authored_value }),
        ));
    }
    cases.extend(non_property_cases());
    cases.extend(focused_cases());
    cases.sort_by(|left, right| left.id.cmp(&right.id));
    cases
}

fn identity_pairs_from_rows(rows: &[Row]) -> Vec<(String, String)> {
    rows.iter()
        .map(|row| (row.owner.clone(), row.case_id.clone()))
        .collect()
}

fn identity_pairs_from_cases(cases: &[Case]) -> Vec<(String, String)> {
    cases
        .iter()
        .map(|case| (case.owner.clone(), case.id.clone()))
        .collect()
}

fn literal_identity_pairs() -> Result<Vec<(String, String)>, String> {
    STABLE_CASE_OWNERS
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(index, line)| {
            let (case_id, owner) = line
                .split_once('\t')
                .ok_or_else(|| format!("malformed stable identity at line {}", index + 1))?;
            if case_id.is_empty() || owner.is_empty() || owner.contains('\t') {
                return Err(format!("malformed stable identity `{line}`"));
            }
            Ok((owner.to_owned(), case_id.to_owned()))
        })
        .collect()
}

fn validate_exact_identity_closure(mut identities: Vec<(String, String)>) -> Result<(), String> {
    if identities.len() != EXACT_CASE_COUNT {
        return Err(format!(
            "exact I01 case count: expected {EXACT_CASE_COUNT}, got {}",
            identities.len()
        ));
    }
    identities.sort();
    let mut case_ids = BTreeSet::new();
    let mut owner_case_pairs = BTreeSet::new();
    let mut owners = BTreeMap::new();
    for (owner, case_id) in &identities {
        if !case_ids.insert(case_id.as_str()) {
            return Err(format!("case identity collision: `{case_id}`"));
        }
        if !owner_case_pairs.insert((owner.as_str(), case_id.as_str())) {
            return Err(format!(
                "owner/case identity collision: `{owner}` / `{case_id}`"
            ));
        }
        let owner_id = owner
            .split_once("::")
            .map_or(owner.as_str(), |(owner_id, _)| owner_id);
        *owners.entry(owner_id).or_insert(0usize) += 1;
    }
    let exact_owners = EXACT_OWNER_COUNTS.into_iter().collect::<BTreeMap<_, _>>();
    if owners != exact_owners {
        return Err("exact I01 owner/cardinality closure mismatch".to_owned());
    }
    let fingerprint = identity_fingerprint(&identities);
    if fingerprint != EXACT_IDENTITY_FINGERPRINT {
        return Err(format!(
            "stable owner/case mapping changed: expected {EXACT_IDENTITY_FINGERPRINT:#018x}, got {fingerprint:#018x}"
        ));
    }
    Ok(())
}

fn identity_fingerprint(identities: &[(String, String)]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for (owner, case_id) in identities {
        for byte in owner
            .bytes()
            .chain([0])
            .chain(case_id.bytes())
            .chain(*b"\n")
        {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

#[test]
fn i01_observable_fixture_has_the_exact_independent_case_union() {
    let rows = parse_fixture(FIXTURE).expect("valid I01 observable fixture");
    let expected = expected_cases();
    let literal_identities = literal_identity_pairs().expect("literal stable owner/case mapping");
    validate_exact_identity_closure(literal_identities.clone())
        .expect("literal exact owner/case closure");
    validate_exact_identity_closure(identity_pairs_from_rows(&rows))
        .expect("fixture exact owner/case closure");
    validate_exact_identity_closure(identity_pairs_from_cases(&expected))
        .expect("source-case exact owner/case closure");
    let literal_identities = literal_identities.into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        identity_pairs_from_rows(&rows)
            .into_iter()
            .collect::<BTreeSet<_>>(),
        literal_identities,
        "fixture owner/case mapping"
    );
    assert_eq!(
        identity_pairs_from_cases(&expected)
            .into_iter()
            .collect::<BTreeSet<_>>(),
        literal_identities,
        "executable source-case owner/case mapping"
    );
    assert_eq!(
        catalog_vectors::PROPERTY_POSITIVE_VECTORS.len(),
        179,
        "imported positive catalog-vector count changed"
    );
    assert_eq!(
        catalog_vectors::PROPERTY_NEGATIVE_VECTORS.len(),
        179,
        "imported boundary catalog-vector count changed"
    );
    assert_eq!(rows.len(), expected.len(), "exact I01 case union size");
    for (row, case) in rows.iter().zip(expected.iter()) {
        assert_eq!(row.case_id, case.id, "case-union identity");
        assert_eq!(row.owner, case.owner, "{} owner", case.id);
        assert_eq!(row.entry, case.entry.as_str(), "{} entry", case.id);
        assert_eq!(row.feature, case.feature.as_str(), "{} feature", case.id);
        assert_eq!(row.input, case.input, "{} authored input", case.id);
    }
}

#[test]
fn i01_observable_fixture_matches_every_public_report() {
    let rows = parse_fixture(FIXTURE).expect("valid I01 observable fixture");
    let mut executed = 0usize;
    let mut wrapper_comparisons = 0usize;
    let mut prior_public_comparisons = 0usize;
    for row in rows {
        if row.feature == "app-strict" && !cfg!(feature = "app-strict") {
            continue;
        }
        let actual = observe(&row);
        assert_eq!(actual.clean, row.clean, "{} clean report", row.case_id);
        assert_eq!(
            actual.retained, row.retained,
            "{} retained syntax",
            row.case_id
        );
        assert_eq!(
            actual.diagnostics, row.diagnostics,
            "{} diagnostics",
            row.case_id
        );
        wrapper_comparisons += actual.wrapper_comparisons;
        prior_public_comparisons += actual.prior_public_comparisons;
        executed += 1;
        #[cfg(feature = "app-strict")]
        assert_strict_parity(&row);
    }
    let expected_executed = if cfg!(feature = "app-strict") {
        EXACT_APP_STRICT_CASE_COUNT
    } else {
        EXACT_DEFAULT_CASE_COUNT
    };
    assert_eq!(executed, expected_executed, "executed public-report rows");
    assert_eq!(
        wrapper_comparisons,
        if cfg!(feature = "app-strict") {
            751
        } else {
            746
        },
        "all pre-authored deferred ordinary slices compare through wrappers"
    );
    assert_eq!(
        prior_public_comparisons,
        if cfg!(feature = "app-strict") { 35 } else { 34 },
        "prior public authored-slice comparisons remain intact"
    );
}

#[test]
fn i01_observable_reader_rejects_union_and_observable_mutations() {
    let rows = parse_fixture(FIXTURE).expect("valid fixture");
    assert!(
        parse_fixture(&FIXTURE.replacen(HEADER, &format!("{HEADER}\textra"), 1))
            .expect_err("unknown column must fail")
            .contains("unknown or reordered columns")
    );
    let duplicate = format!(
        "{HEADER}\n{}\n{}\n",
        render_row(&rows[0]),
        render_row(&rows[0])
    );
    assert!(
        parse_fixture(&duplicate)
            .expect_err("duplicate ID must fail")
            .contains(&rows[0].case_id)
    );
    let malformed = format!("{HEADER}\n{}\\q\n", render_row(&rows[0]));
    assert!(
        parse_fixture(&malformed)
            .expect_err("malformed escape must fail")
            .contains("malformed escape")
    );
    let noncanonical = format!(
        "{HEADER}\n{}\n{}\n",
        render_row(&rows[1]),
        render_row(&rows[0])
    );
    assert!(
        parse_fixture(&noncanonical)
            .expect_err("noncanonical order must fail")
            .contains("noncanonical case order")
    );
    let removed = rows
        .iter()
        .skip(1)
        .map(render_row)
        .collect::<Vec<_>>()
        .join("\n");
    let removed = format!("{HEADER}\n{removed}\n");
    let removed_rows = parse_fixture(&removed).expect("well-formed incomplete fixture");
    assert_ne!(
        removed_rows.len(),
        expected_cases().len(),
        "removed case must fail exact union"
    );
    let removed_expected = expected_cases().into_iter().skip(1).collect::<Vec<_>>();
    assert!(
        validate_exact_identity_closure(identity_pairs_from_rows(&removed_rows)).is_err(),
        "coordinated TSV/source omission must fail literal closure"
    );
    assert!(
        validate_exact_identity_closure(identity_pairs_from_cases(&removed_expected)).is_err(),
        "coordinated source/TSV omission must fail literal closure"
    );

    let mut collided = identity_pairs_from_rows(&rows);
    collided[1].1 = collided[0].1.clone();
    assert!(
        validate_exact_identity_closure(collided)
            .expect_err("case identity collision must fail")
            .contains(&rows[0].case_id)
    );

    let mut missing = rows.clone();
    missing[0].retained.clear();
    let missing = format!(
        "{HEADER}\n{}\n",
        missing
            .iter()
            .map(render_row)
            .collect::<Vec<_>>()
            .join("\n")
    );

    let declaration_index = rows
        .iter()
        .position(|row| row.authored_declarations != "-")
        .expect("fixture must contain retained declarations");
    let mut missing_authored = rows.clone();
    missing_authored[declaration_index]
        .authored_declarations
        .clear();
    let missing_authored = format!(
        "{HEADER}\n{}\n",
        missing_authored
            .iter()
            .map(render_row)
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        parse_fixture(&missing_authored)
            .expect_err("missing authored declaration observable must fail")
            .contains("absent required observable"),
        "responsible case: {}",
        rows[declaration_index].case_id
    );

    let keyframe_index = rows
        .iter()
        .position(|row| {
            row.authored_declarations
                .contains("@keyframe-grammar:normal")
        })
        .expect("fixture must contain keyframe declaration observables");
    let mut missing_keyframe = rows.clone();
    let mut keyframe_declarations = missing_keyframe[keyframe_index]
        .authored_declarations
        .split('~')
        .collect::<Vec<_>>();
    keyframe_declarations.pop().expect("keyframe declaration");
    missing_keyframe[keyframe_index].authored_declarations =
        nonempty(keyframe_declarations.join("~"));
    let missing_keyframe = format!(
        "{HEADER}\n{}\n",
        missing_keyframe
            .iter()
            .map(render_row)
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        parse_fixture(&missing_keyframe)
            .expect_err("missing keyframe declaration observable must fail")
            .contains(&rows[keyframe_index].case_id)
    );
    assert!(
        parse_fixture(&missing)
            .expect_err("missing observable must fail")
            .contains("absent required observable"),
        "responsible case: {}",
        rows[0].case_id
    );

    let repeated_index = rows
        .iter()
        .position(|row| row.diagnostics.split('~').count() > 1)
        .expect("fixture must contain repeated diagnostics");
    let mut repeated = rows.clone();
    repeated[repeated_index].diagnostics = repeated[repeated_index]
        .diagnostics
        .split_once('~')
        .expect("repeated diagnostic")
        .0
        .to_owned();
    assert_ne!(
        observe(&repeated[repeated_index]).diagnostics,
        repeated[repeated_index].diagnostics,
        "{} repeated diagnostic removal must fail",
        repeated[repeated_index].case_id
    );
}

fn render_row(row: &Row) -> String {
    row.fields().map(escape).join("\t")
}

fn escape(field: &str) -> String {
    field
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

struct Observation {
    clean: String,
    retained: String,
    diagnostics: String,
    wrapper_comparisons: usize,
    prior_public_comparisons: usize,
}

fn observe(expected: &Row) -> Observation {
    let mut frozen = FrozenDeclarationCursor::new(expected);
    let (clean, retained, diagnostics) = match expected.entry.as_str() {
        "sheet" => {
            let report = parse_sheet(&expected.input);
            let retained = sheet_observables(report.syntax().rules(), &mut frozen);
            (
                report.is_clean(),
                retained,
                diagnostics_observable(report.diagnostics()),
            )
        }
        "style" => {
            let report = parse_style_attribute(&expected.input);
            let retained =
                declaration_observables(report.syntax().as_slice(), "public", true, &mut frozen);
            (
                report.is_clean(),
                retained,
                diagnostics_observable(report.diagnostics()),
            )
        }
        _ => unreachable!("validated fixture entry"),
    };
    frozen.finish();
    Observation {
        clean: clean.to_string(),
        retained: nonempty(retained.join("~")),
        diagnostics: nonempty(diagnostics.join("~")),
        wrapper_comparisons: frozen.wrapper_comparisons,
        prior_public_comparisons: frozen.prior_public_comparisons,
    }
}

fn nonempty(value: String) -> String {
    if value.is_empty() {
        "-".to_owned()
    } else {
        value
    }
}

fn sheet_observables(rules: &[CssRule], frozen: &mut FrozenDeclarationCursor<'_>) -> Vec<String> {
    let mut retained = Vec::new();
    for rule in rules {
        rule_observables(rule, &mut retained, frozen);
    }
    retained
}

fn rule_observables(
    rule: &CssRule,
    retained: &mut Vec<String>,
    frozen: &mut FrozenDeclarationCursor<'_>,
) {
    match rule {
        CssRule::Import(_) => retained.push("rule:baseline.rule.import".to_owned()),
        CssRule::LayerStatement(_) => {
            retained.push("rule:baseline.rule.layer-statement".to_owned())
        }
        CssRule::LayerBlock(rule) => {
            retained.push("rule:baseline.rule.layer-block".to_owned());
            for child in rule.rules() {
                rule_observables(child, retained, frozen);
            }
        }
        CssRule::FontFace(_) => retained.push("rule:baseline.rule.font-face".to_owned()),
        CssRule::Keyframes(rule) => {
            retained.push("rule:baseline.rule.keyframes".to_owned());
            for block in rule.blocks() {
                for declaration in block.declarations().iter() {
                    let name = declaration.property_name();
                    let id = property_id(name);
                    retained.push(format!("property:{id}"));
                    let (semantic, authored) = frozen.next(false);
                    assert_eq!(authored.id, id, "{} declaration identity", frozen.case_id);
                    assert_eq!(
                        authored.importance_capability, "keyframe-grammar",
                        "{} {id} importance capability",
                        frozen.case_id
                    );
                    assert_eq!(
                        authored.importance, "normal",
                        "{} {id} authored importance",
                        frozen.case_id
                    );
                    assert_declaration_value(
                        name,
                        declaration.custom(),
                        declaration.known(),
                        semantic,
                        &authored,
                        frozen,
                    );
                }
            }
        }
        CssRule::Style(rule) => {
            retained.push("rule:baseline.rule.style".to_owned());
            let ids =
                declaration_observables(rule.declarations().as_slice(), "public", true, frozen);
            retained.extend(ids);
        }
        CssRule::Media(rule) => {
            retained.push("rule:baseline.rule.media".to_owned());
            for child in rule.rules() {
                rule_observables(child, retained, frozen);
            }
        }
        CssRule::Container(rule) => {
            retained.push("rule:baseline.rule.container".to_owned());
            for child in rule.rules() {
                rule_observables(child, retained, frozen);
            }
        }
        CssRule::Scope(rule) => {
            retained.push("rule:baseline.rule.scope".to_owned());
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, frozen);
            }
        }
        _ => retained.push("rule:future".to_owned()),
    }
}

fn scoped_rule_observables(
    rule: &CssScopedRule,
    retained: &mut Vec<String>,
    frozen: &mut FrozenDeclarationCursor<'_>,
) {
    match rule {
        CssScopedRule::Style(rule) => {
            retained.push("rule:baseline.rule.style".to_owned());
            let ids =
                declaration_observables(rule.declarations().as_slice(), "public", true, frozen);
            retained.extend(ids);
        }
        CssScopedRule::Media(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, frozen);
            }
        }
        CssScopedRule::Container(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, frozen);
            }
        }
        CssScopedRule::LayerStatement(_) => {
            retained.push("rule:baseline.rule.layer-statement".to_owned())
        }
        CssScopedRule::LayerBlock(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, frozen);
            }
        }
        CssScopedRule::Scope(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, frozen);
            }
        }
        _ => retained.push("rule:future".to_owned()),
    }
}

fn declaration_observables(
    declarations: &[CssDeclaration],
    importance_capability: &str,
    includes_semantic_value: bool,
    frozen: &mut FrozenDeclarationCursor<'_>,
) -> Vec<String> {
    let mut retained = Vec::new();
    declaration_values(
        declarations
            .iter()
            .map(|declaration| (declaration.property_name(), Some(declaration))),
        &mut retained,
        importance_capability,
        includes_semantic_value,
        frozen,
    );
    retained
}

fn declaration_values<'a>(
    declarations: impl Iterator<Item = (CssPropertyNameRef<'a>, Option<&'a CssDeclaration>)>,
    retained: &mut Vec<String>,
    importance_capability: &str,
    includes_semantic_value: bool,
    frozen: &mut FrozenDeclarationCursor<'_>,
) {
    for (name, declaration) in declarations {
        let id = property_id(name);
        retained.push(format!("property:{id}"));
        if let Some(declaration) = declaration {
            let (semantic, authored) = frozen.next(includes_semantic_value);
            let importance = match declaration.importance() {
                CssImportance::Normal => "normal",
                CssImportance::Important => "important",
            };
            assert_eq!(authored.id, id, "{} declaration identity", frozen.case_id);
            assert_eq!(
                authored.importance_capability, importance_capability,
                "{} {id} importance capability",
                frozen.case_id
            );
            assert_eq!(
                authored.importance, importance,
                "{} {id} authored importance",
                frozen.case_id
            );
            if let Some(semantic) = semantic {
                assert_eq!(semantic.id, id, "{} semantic identity", frozen.case_id);
                assert_eq!(
                    semantic.importance, importance,
                    "{} {id} semantic importance",
                    frozen.case_id
                );
            }
            assert_declaration_value(
                name,
                declaration.custom(),
                declaration.known(),
                semantic,
                &authored,
                frozen,
            );
        }
    }
}

fn property_id(name: CssPropertyNameRef<'_>) -> String {
    match name {
        CssPropertyNameRef::Known(property) => property.stable_id().to_owned(),
        CssPropertyNameRef::Custom(name) => format!("custom:{}", name.as_str()),
        _ => "future-property".to_owned(),
    }
}

fn assert_declaration_value(
    name: CssPropertyNameRef<'_>,
    custom: Option<&surgeist_css::CssCustomDeclaration>,
    known: Option<&surgeist_css::CssKnownDeclaration>,
    semantic: Option<FrozenSemanticValue<'_>>,
    authored: &AuthoredDeclaration<'_>,
    frozen: &mut FrozenDeclarationCursor<'_>,
) {
    if let Some(custom) = custom {
        if let Some(value) = custom.value().value() {
            assert_eq!(
                authored.value_capability, "public",
                "{} {} authored-value capability",
                frozen.case_id, authored.id
            );
            assert_eq!(
                authored.value,
                value.as_css(),
                "{} {} publicly exposed authored slice",
                frozen.case_id,
                authored.id
            );
            frozen.prior_public_comparisons += 1;
            if let Some(semantic) = semantic {
                assert_eq!(
                    semantic.payload,
                    value.as_css(),
                    "{} {} frozen custom-property payload",
                    frozen.case_id,
                    authored.id
                );
            }
        } else {
            let keyword = custom.value().global().expect("symbolic custom global");
            assert_eq!(
                authored.value_capability, "deferred-i01",
                "{} {} authored-value capability",
                frozen.case_id, authored.id
            );
            assert_ne!(
                authored.value, "<unavailable>",
                "{} {} deferred slice must remain explicit in the TSV",
                frozen.case_id, authored.id
            );
            assert_eq!(
                authored.value,
                global_keyword_css(keyword),
                "{} {} custom-global authored slice",
                frozen.case_id,
                authored.id
            );
            frozen.wrapper_comparisons += 1;
            if let Some(semantic) = semantic {
                assert_eq!(
                    semantic.payload,
                    custom_global_semantic_payload(keyword),
                    "{} {} frozen custom-global payload",
                    frozen.case_id,
                    authored.id
                );
            }
        }
        return;
    }

    let Some(known) = known else {
        assert_eq!(
            authored.value_capability, "deferred-i01",
            "{} {} future authored-value capability",
            frozen.case_id, authored.id
        );
        assert_eq!(
            authored.value, "<unavailable>",
            "{} {} future authored slice",
            frozen.case_id, authored.id
        );
        if let Some(semantic) = semantic {
            assert_eq!(
                semantic.payload, "future",
                "{} {} future semantic payload",
                frozen.case_id, authored.id
            );
        }
        return;
    };
    assert_eq!(
        property_id(name),
        known.property().stable_id(),
        "{} known property/declaration identity",
        frozen.case_id
    );
    match known.declared_value() {
        surgeist_css::CssKnownDeclaredValueRef::Property(value) => {
            assert_known_property_value(known.property(), value, semantic, authored, frozen);
        }
        surgeist_css::CssKnownDeclaredValueRef::Global(value) => {
            assert_eq!(
                authored.value_capability, "deferred-i01",
                "{} {} authored-value capability",
                frozen.case_id, authored.id
            );
            assert_ne!(
                authored.value, "<unavailable>",
                "{} {} deferred slice must remain explicit in the TSV",
                frozen.case_id, authored.id
            );
            assert_eq!(
                authored.value,
                global_keyword_css(value),
                "{} {} global authored slice",
                frozen.case_id,
                authored.id
            );
            frozen.wrapper_comparisons += 1;
            if let Some(semantic) = semantic {
                assert_eq!(
                    semantic.payload,
                    known_global_semantic_payload(value),
                    "{} {} frozen global payload",
                    frozen.case_id,
                    authored.id
                );
            }
        }
        surgeist_css::CssKnownDeclaredValueRef::SubstitutionDependent(value) => {
            assert_eq!(
                authored.value_capability, "public",
                "{} {} authored-value capability",
                frozen.case_id, authored.id
            );
            assert_eq!(
                authored.value,
                value.as_css(),
                "{} {} publicly exposed authored slice",
                frozen.case_id,
                authored.id
            );
            frozen.prior_public_comparisons += 1;
            if let Some(semantic) = semantic {
                assert_eq!(
                    semantic.payload,
                    format!("substitution:{}", value.as_css()),
                    "{} {} frozen substitution payload",
                    frozen.case_id,
                    authored.id
                );
            }
        }
        _ => {
            assert_eq!(
                authored.value_capability, "deferred-i01",
                "{} {} future authored-value capability",
                frozen.case_id, authored.id
            );
            assert_eq!(
                authored.value, "<unavailable>",
                "{} {} future authored slice",
                frozen.case_id, authored.id
            );
            if let Some(semantic) = semantic {
                assert_eq!(
                    semantic.payload, "future",
                    "{} {} future semantic payload",
                    frozen.case_id, authored.id
                );
            }
        }
    }
}

macro_rules! assert_property_specific_value {
    (
        $property:expr,
        $value:expr,
        $semantic:expr,
        $authored:expr,
        $frozen:expr;
        $($variant:ident,)*
    ) => {
        match ($property, $value) {
            $(
                (
                    surgeist_css::CssKnownProperty::$variant,
                    surgeist_css::CssKnownPropertyValueRef::$variant(value),
                ) => {
                    let expected_id =
                        surgeist_css::CssKnownProperty::$variant.stable_id();
                    assert_eq!(
                        $authored.id, expected_id,
                        "{} {} property-specific authored identity",
                        $frozen.case_id, expected_id
                    );
                    assert_eq!(
                        $authored.value_capability, "deferred-i01",
                        "{} {} authored-value capability",
                        $frozen.case_id, expected_id
                    );
                    assert_ne!(
                        $authored.value, "<unavailable>",
                        "{} {} deferred slice must remain explicit in the TSV",
                        $frozen.case_id, expected_id
                    );
                    assert_eq!(
                        value.as_css(),
                        $authored.value,
                        "{} {} concrete wrapper authored slice",
                        $frozen.case_id, expected_id
                    );
                    let typed = value.i01_subset().unwrap_or_else(|| {
                        panic!(
                            "{}: {} concrete wrapper lacks its typed I01 payload",
                            $frozen.case_id, expected_id
                        )
                    });
                    if let Some(semantic) = $semantic {
                        assert_eq!(
                            semantic.id, expected_id,
                            "{} {} property-specific semantic identity",
                            $frozen.case_id, expected_id
                        );
                        assert_eq!(
                            semantic.payload,
                            format!("typed:{typed:?}"),
                            "{} {} frozen I01 public Debug payload",
                            $frozen.case_id, expected_id
                        );
                    }
                    $frozen.wrapper_comparisons += 1;
                }
            )*
            _ => panic!(
                "{}: known property identity and concrete value wrapper disagree",
                $frozen.case_id
            ),
        }
    };
}

fn assert_known_property_value(
    property: surgeist_css::CssKnownProperty,
    value: surgeist_css::CssKnownPropertyValueRef<'_>,
    semantic: Option<FrozenSemanticValue<'_>>,
    authored: &AuthoredDeclaration<'_>,
    frozen: &mut FrozenDeclarationCursor<'_>,
) {
    assert_property_specific_value!(
        property,
        value,
        semantic,
        authored,
        frozen;
            All,
            Display,
            BoxSizing,
            Position,
            Direction,
            Overflow,
            OverflowX,
            OverflowY,
            FlexDirection,
            FlexWrap,
            Float,
            Clear,
            AlignContent,
            JustifyContent,
            AlignItems,
            AlignSelf,
            JustifyItems,
            JustifySelf,
            PlaceContent,
            PlaceItems,
            PlaceSelf,
            Visibility,
            Content,
            ContentVisibility,
            ListStyleType,
            ListStylePosition,
            ListStyleImage,
            ListStyle,
            CounterReset,
            CounterIncrement,
            CounterSet,
            Width,
            Height,
            MinWidth,
            MinHeight,
            MaxWidth,
            MaxHeight,
            FlexBasis,
            Gap,
            RowGap,
            ColumnGap,
            GridFlowTolerance,
            GridTemplateRows,
            GridTemplateColumns,
            GridTemplateAreas,
            GridTemplate,
            GridAutoRows,
            GridAutoColumns,
            GridAutoFlow,
            GridRowStart,
            GridRowEnd,
            GridColumnStart,
            GridColumnEnd,
            GridRow,
            GridColumn,
            GridArea,
            Grid,
            FontSize,
            LineHeight,
            WritingMode,
            TextAlign,
            TextAlignLast,
            TextIndent,
            VerticalAlign,
            FontFamily,
            Font,
            FontWeight,
            FontStyle,
            FontStretch,
            FontVariant,
            FontFeatureSettings,
            LetterSpacing,
            TextWrap,
            WhiteSpace,
            WordBreak,
            OverflowWrap,
            TextOverflow,
            TextDecoration,
            TextDecorationLine,
            TextDecorationColor,
            TextDecorationStyle,
            TextDecorationThickness,
            TextTransform,
            Inset,
            Top,
            Right,
            Bottom,
            Left,
            ZIndex,
            BoxDecorationBreak,
            Margin,
            MarginTop,
            MarginRight,
            MarginBottom,
            MarginLeft,
            Padding,
            PaddingTop,
            PaddingRight,
            PaddingBottom,
            PaddingLeft,
            Border,
            BorderTop,
            BorderRight,
            BorderBottom,
            BorderLeft,
            BorderWidth,
            BorderTopWidth,
            BorderRightWidth,
            BorderBottomWidth,
            BorderLeftWidth,
            Color,
            Background,
            BackgroundColor,
            BorderColor,
            BorderTopColor,
            BorderRightColor,
            BorderBottomColor,
            BorderLeftColor,
            BackgroundImage,
            BackgroundPosition,
            BackgroundSize,
            BackgroundRepeat,
            BackgroundOrigin,
            BackgroundClip,
            BackgroundAttachment,
            BorderStyle,
            BorderTopStyle,
            BorderRightStyle,
            BorderBottomStyle,
            BorderLeftStyle,
            BorderRadius,
            BorderTopLeftRadius,
            BorderTopRightRadius,
            BorderBottomRightRadius,
            BorderBottomLeftRadius,
            BoxShadow,
            Opacity,
            FlexGrow,
            FlexShrink,
            Order,
            Flex,
            JustifyTracks,
            AlignTracks,
            AspectRatio,
            ScrollbarWidth,
            Cursor,
            PointerEvents,
            UserSelect,
            Outline,
            OutlineColor,
            OutlineStyle,
            OutlineWidth,
            Transform,
            TransformOrigin,
            Translate,
            Rotate,
            Scale,
            Filter,
            BackdropFilter,
            ClipPath,
            Mask,
            MaskImage,
            MaskSize,
            MaskPosition,
            MaskRepeat,
            TransitionProperty,
            TransitionDuration,
            TransitionDelay,
            TransitionTimingFunction,
            Transition,
            AnimationName,
            AnimationDuration,
            AnimationDelay,
            AnimationTimingFunction,
            AnimationIterationCount,
            AnimationDirection,
            AnimationFillMode,
            AnimationPlayState,
            Animation,
    );
}
fn global_keyword_css(keyword: surgeist_css::CssGlobalKeyword) -> &'static str {
    match keyword {
        surgeist_css::CssGlobalKeyword::Inherit => "inherit",
        surgeist_css::CssGlobalKeyword::Initial => "initial",
        surgeist_css::CssGlobalKeyword::Unset => "unset",
        surgeist_css::CssGlobalKeyword::Revert => "revert",
        surgeist_css::CssGlobalKeyword::RevertLayer => "revert-layer",
        _ => "<future-global>",
    }
}

fn custom_global_semantic_payload(keyword: surgeist_css::CssGlobalKeyword) -> &'static str {
    match keyword {
        surgeist_css::CssGlobalKeyword::Inherit => "global:Some(Inherit)",
        surgeist_css::CssGlobalKeyword::Initial => "global:Some(Initial)",
        surgeist_css::CssGlobalKeyword::Unset => "global:Some(Unset)",
        surgeist_css::CssGlobalKeyword::Revert => "global:Some(Revert)",
        surgeist_css::CssGlobalKeyword::RevertLayer => "global:Some(RevertLayer)",
        _ => panic!("future CSS global keyword lacks a frozen custom-global payload"),
    }
}

fn known_global_semantic_payload(keyword: surgeist_css::CssGlobalKeyword) -> &'static str {
    match keyword {
        surgeist_css::CssGlobalKeyword::Inherit => "global:Inherit",
        surgeist_css::CssGlobalKeyword::Initial => "global:Initial",
        surgeist_css::CssGlobalKeyword::Unset => "global:Unset",
        surgeist_css::CssGlobalKeyword::Revert => "global:Revert",
        surgeist_css::CssGlobalKeyword::RevertLayer => "global:RevertLayer",
        _ => panic!("future CSS global keyword lacks a frozen known-global payload"),
    }
}

fn diagnostics_observable(diagnostics: &[CssRecoveryDiagnostic]) -> Vec<String> {
    diagnostics.iter().map(diagnostic_observable).collect()
}

fn diagnostic_observable(diagnostic: &CssRecoveryDiagnostic) -> String {
    let error = diagnostic.error();
    let position = error.position();
    let span = diagnostic.span();
    format!(
        "{}/{}/{}@{}:{}:{}>{}:{}:{}-{}:{}:{}:{}",
        code_name(error.code()),
        root_and_payload(error.kind()),
        action_name(diagnostic.action()),
        position.byte_offset().value(),
        position.line().value(),
        position.column().value(),
        span.start().byte_offset().value(),
        span.start().line().value(),
        span.start().column().value(),
        span.end().byte_offset().value(),
        span.end().line().value(),
        span.end().column().value(),
        span.end().byte_offset().value(),
    )
}

fn token(token: Option<&surgeist_css::CssTokenSummary>) -> String {
    token.map_or_else(
        || "-".to_owned(),
        |token| format!("{}:{}", token_kind_name(token.kind()), token.authored()),
    )
}

fn token_kind_name(kind: surgeist_css::CssTokenKind) -> &'static str {
    match kind {
        surgeist_css::CssTokenKind::Ident => "Ident",
        surgeist_css::CssTokenKind::AtKeyword => "AtKeyword",
        surgeist_css::CssTokenKind::Hash => "Hash",
        surgeist_css::CssTokenKind::IdHash => "IdHash",
        surgeist_css::CssTokenKind::String => "String",
        surgeist_css::CssTokenKind::Url => "Url",
        surgeist_css::CssTokenKind::Delim => "Delim",
        surgeist_css::CssTokenKind::Number => "Number",
        surgeist_css::CssTokenKind::Percentage => "Percentage",
        surgeist_css::CssTokenKind::Dimension => "Dimension",
        surgeist_css::CssTokenKind::Whitespace => "Whitespace",
        surgeist_css::CssTokenKind::Comment => "Comment",
        surgeist_css::CssTokenKind::Colon => "Colon",
        surgeist_css::CssTokenKind::Semicolon => "Semicolon",
        surgeist_css::CssTokenKind::Comma => "Comma",
        surgeist_css::CssTokenKind::IncludeMatch => "IncludeMatch",
        surgeist_css::CssTokenKind::DashMatch => "DashMatch",
        surgeist_css::CssTokenKind::PrefixMatch => "PrefixMatch",
        surgeist_css::CssTokenKind::SuffixMatch => "SuffixMatch",
        surgeist_css::CssTokenKind::SubstringMatch => "SubstringMatch",
        surgeist_css::CssTokenKind::Cdo => "Cdo",
        surgeist_css::CssTokenKind::Cdc => "Cdc",
        surgeist_css::CssTokenKind::Function => "Function",
        surgeist_css::CssTokenKind::ParenthesisBlock => "ParenthesisBlock",
        surgeist_css::CssTokenKind::SquareBracketBlock => "SquareBracketBlock",
        surgeist_css::CssTokenKind::CurlyBracketBlock => "CurlyBracketBlock",
        surgeist_css::CssTokenKind::BadUrl => "BadUrl",
        surgeist_css::CssTokenKind::BadString => "BadString",
        surgeist_css::CssTokenKind::CloseParenthesis => "CloseParenthesis",
        surgeist_css::CssTokenKind::CloseSquareBracket => "CloseSquareBracket",
        surgeist_css::CssTokenKind::CloseCurlyBracket => "CloseCurlyBracket",
        _ => panic!("future CSS token kind lacks a frozen oracle name"),
    }
}

fn root_and_payload(kind: &ErrorKind) -> String {
    match kind {
        ErrorKind::UnexpectedEnd(detail) => {
            format!("UnexpectedEnd:{}", detail.expectation().as_str())
        }
        ErrorKind::UnexpectedToken(detail) => format!(
            "UnexpectedToken:{}:{}",
            detail.expectation().as_str(),
            token(Some(detail.encountered()))
        ),
        ErrorKind::InvalidEncodingDeclaration(detail) => format!(
            "InvalidEncodingDeclaration:{}:{}",
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidAtRulePlacement(detail) => format!(
            "InvalidAtRulePlacement:{}:{}",
            detail.name().as_str(),
            detail.expected_context().as_str()
        ),
        ErrorKind::InvalidAtRulePrelude(detail) => format!(
            "InvalidAtRulePrelude:{}:{}:{}:{}",
            detail.name().as_str(),
            detail.production().as_str(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidAtRuleBody(detail) => format!(
            "InvalidAtRuleBody:{}:{}:{}:{}",
            detail.name().as_str(),
            detail.production().as_str(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::UnknownAtRule(detail) => format!("UnknownAtRule:{}", detail.name().as_str()),
        ErrorKind::UnsupportedAtRule(detail) => format!(
            "UnsupportedAtRule:{}:{}",
            detail.name().as_str(),
            detail.feature().as_str()
        ),
        ErrorKind::InvalidQualifiedRule(detail) => format!(
            "InvalidQualifiedRule:{}:{}:{}",
            detail.production().as_str(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidSelector(detail) => format!(
            "InvalidSelector:{}:{}:{}",
            detail.production().map_or("-", |value| value.as_str()),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidMediaQuery(detail) => format!(
            "InvalidMediaQuery:{}:{}:{}",
            detail.feature().map_or("-", |value| value.as_str()),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::UnknownProperty(detail) => format!("UnknownProperty:{}", detail.name().as_str()),
        ErrorKind::UnsupportedProperty(detail) => format!(
            "UnsupportedProperty:{}:{}",
            detail.name().as_str(),
            detail.feature().as_str()
        ),
        ErrorKind::InvalidPropertyValue(detail) => format!(
            "InvalidPropertyValue:{}:{}:{}",
            detail.property().stable_id(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidDeclarationAnnotation(detail) => format!(
            "InvalidDeclarationAnnotation:{}:{}",
            declaration_context(detail.context()),
            token(Some(detail.encountered()))
        ),
        ErrorKind::UnknownDescriptor(detail) => format!(
            "UnknownDescriptor:{}:{}",
            detail.at_rule().as_str(),
            detail.descriptor().as_str()
        ),
        ErrorKind::UnsupportedDescriptor(detail) => format!(
            "UnsupportedDescriptor:{}:{}:{}",
            detail.at_rule().as_str(),
            detail.descriptor().as_str(),
            detail.feature().as_str()
        ),
        ErrorKind::InvalidDescriptorValue(detail) => format!(
            "InvalidDescriptorValue:{}:{}:{}:{}",
            detail.at_rule().as_str(),
            detail.descriptor().as_str(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidDescriptorCombination(detail) => format!(
            "InvalidDescriptorCombination:{}:{}:{}",
            detail.at_rule().as_str(),
            detail.responsible().as_str(),
            detail
                .conflicting()
                .iter()
                .map(|value| value.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ),
        ErrorKind::InvalidColorSyntax(detail) => format!(
            "InvalidColorSyntax:{}:{}:{}",
            detail.component().map_or("-", |value| value.as_str()),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::NestingLimit(detail) => format!(
            "NestingLimit:{}:{}",
            detail.limit(),
            detail.enclosing_production().as_str()
        ),
        _ => "Future".to_owned(),
    }
}

fn declaration_context(context: CssDeclarationContextRef<'_>) -> String {
    match context {
        CssDeclarationContextRef::KnownProperty(property) => {
            format!("known:{}", property.stable_id())
        }
        CssDeclarationContextRef::CustomProperty(name) => format!("custom:{}", name.as_str()),
        CssDeclarationContextRef::Keyframe(property) => {
            format!("keyframe:{}", property.stable_id())
        }
        CssDeclarationContextRef::KeyframeCustomProperty(name) => {
            format!("keyframe-custom:{}", name.as_str())
        }
        CssDeclarationContextRef::Descriptor {
            at_rule,
            descriptor,
        } => format!("descriptor:{}:{}", at_rule.as_str(), descriptor.as_str()),
        _ => "future".to_owned(),
    }
}

fn code_name(code: CssErrorCode) -> &'static str {
    match code {
        CssErrorCode::UnexpectedEnd => "UnexpectedEnd",
        CssErrorCode::UnexpectedToken => "UnexpectedToken",
        CssErrorCode::InvalidEncodingDeclaration => "InvalidEncodingDeclaration",
        CssErrorCode::InvalidAtRulePlacement => "InvalidAtRulePlacement",
        CssErrorCode::InvalidAtRulePrelude => "InvalidAtRulePrelude",
        CssErrorCode::InvalidAtRuleBody => "InvalidAtRuleBody",
        CssErrorCode::UnknownAtRule => "UnknownAtRule",
        CssErrorCode::UnsupportedAtRule => "UnsupportedAtRule",
        CssErrorCode::InvalidQualifiedRule => "InvalidQualifiedRule",
        CssErrorCode::InvalidSelector => "InvalidSelector",
        CssErrorCode::InvalidMediaQuery => "InvalidMediaQuery",
        CssErrorCode::UnknownProperty => "UnknownProperty",
        CssErrorCode::UnsupportedProperty => "UnsupportedProperty",
        CssErrorCode::InvalidPropertyValue => "InvalidPropertyValue",
        CssErrorCode::InvalidDeclarationAnnotation => "InvalidDeclarationAnnotation",
        CssErrorCode::UnknownDescriptor => "UnknownDescriptor",
        CssErrorCode::UnsupportedDescriptor => "UnsupportedDescriptor",
        CssErrorCode::InvalidDescriptorValue => "InvalidDescriptorValue",
        CssErrorCode::InvalidDescriptorCombination => "InvalidDescriptorCombination",
        CssErrorCode::InvalidColorSyntax => "InvalidColorSyntax",
        CssErrorCode::NestingLimit => "NestingLimit",
        _ => "Future",
    }
}

fn action_name(action: CssRecoveryAction) -> &'static str {
    match action {
        CssRecoveryAction::DropDeclaration => "DropDeclaration",
        CssRecoveryAction::DropDescriptor => "DropDescriptor",
        CssRecoveryAction::DropQualifiedRule => "DropQualifiedRule",
        CssRecoveryAction::DropAtRule => "DropAtRule",
        CssRecoveryAction::DropKeyframeBlock => "DropKeyframeBlock",
        CssRecoveryAction::DropSelectorListItem => "DropSelectorListItem",
        CssRecoveryAction::ReplaceMediaQueryWithNever => "ReplaceMediaQueryWithNever",
        CssRecoveryAction::RetainWithImplicitClosure => "RetainWithImplicitClosure",
        CssRecoveryAction::IgnoreLegacyToken => "IgnoreLegacyToken",
        CssRecoveryAction::StopAtNestingLimit => "StopAtNestingLimit",
        _ => "Future",
    }
}

#[cfg(feature = "app-strict")]
fn assert_strict_parity(row: &Row) {
    match row.entry.as_str() {
        "sheet" => {
            let ordinary = parse_sheet(&row.input);
            let strict = surgeist_css::validate_sheet(&row.input);
            if ordinary.is_clean() {
                assert_eq!(
                    strict,
                    Ok(ordinary.syntax().clone()),
                    "{} strict sheet",
                    row.case_id
                );
            } else {
                assert_eq!(
                    strict.expect_err("recovered sheet").diagnostics(),
                    ordinary.diagnostics(),
                    "{} strict sheet",
                    row.case_id
                );
            }
        }
        "style" => {
            let ordinary = parse_style_attribute(&row.input);
            let strict = surgeist_css::validate_style_attribute(&row.input);
            if ordinary.is_clean() {
                assert_eq!(
                    strict,
                    Ok(ordinary.syntax().clone()),
                    "{} strict style",
                    row.case_id
                );
            } else {
                assert_eq!(
                    strict.expect_err("recovered style").diagnostics(),
                    ordinary.diagnostics(),
                    "{} strict style",
                    row.case_id
                );
            }
        }
        _ => unreachable!(),
    }
}

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

    fn report_fields(&self) -> [&str; 9] {
        [
            &self.case_id,
            &self.owner,
            &self.entry,
            &self.feature,
            &self.input,
            &self.clean,
            &self.retained,
            &self.values,
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
            "{}: retained/authored declaration identity mismatch: {retained:?} != {authored_ids:?}",
            row.case_id
        ));
    }
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct AuthoredDeclaration<'a> {
    id: String,
    value_capability: &'a str,
    value: &'a str,
    importance_capability: &'a str,
    importance: &'a str,
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

fn assert_authored_declarations(actual: &Row, expected: &Row) {
    let actual = parse_authored_declarations(&actual.authored_declarations, &expected.case_id)
        .expect("runtime authored-declaration observation");
    let expected_declarations =
        parse_authored_declarations(&expected.authored_declarations, &expected.case_id)
            .expect("frozen authored-declaration expectation");
    assert_eq!(
        actual.len(),
        expected_declarations.len(),
        "{} authored declaration count",
        expected.case_id
    );
    for (actual, expected_declaration) in actual.iter().zip(&expected_declarations) {
        assert_eq!(
            actual.id, expected_declaration.id,
            "{} authored declaration identity",
            expected.case_id
        );
        assert_eq!(
            actual.value_capability, expected_declaration.value_capability,
            "{} {} authored-value capability",
            expected.case_id, expected_declaration.id
        );
        assert_eq!(
            actual.importance_capability, expected_declaration.importance_capability,
            "{} {} importance capability",
            expected.case_id, expected_declaration.id
        );
        assert_eq!(
            actual.importance, expected_declaration.importance,
            "{} {} importance",
            expected.case_id, expected_declaration.id
        );
        if expected_declaration.value_capability == "public" {
            assert_eq!(
                actual.value, expected_declaration.value,
                "{} {} publicly exposed authored slice",
                expected.case_id, expected_declaration.id
            );
        } else {
            assert_ne!(
                expected_declaration.value, "<unavailable>",
                "{} {} deferred slice must remain explicit in the TSV",
                expected.case_id, expected_declaration.id
            );
            assert_eq!(
                actual.value, "<unavailable>",
                "{} {} I01 runtime must not infer or fabricate a deferred authored slice",
                expected.case_id, expected_declaration.id
            );
        }
    }
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
        return Err(format!(
            "exact I01 owner/cardinality closure: expected {exact_owners:?}, got {owners:?}"
        ));
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
    for row in rows {
        if row.feature == "app-strict" && !cfg!(feature = "app-strict") {
            continue;
        }
        let actual = observe(&row);
        assert_eq!(
            actual.report_fields(),
            row.report_fields(),
            "{} public report",
            row.case_id
        );
        assert_authored_declarations(&actual, &row);
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

fn observe(expected: &Row) -> Row {
    let (clean, retained, values, authored_declarations, diagnostics) =
        match expected.entry.as_str() {
            "sheet" => {
                let report = parse_sheet(&expected.input);
                let (retained, values, authored_declarations) =
                    sheet_observables(report.syntax().rules());
                (
                    report.is_clean(),
                    retained,
                    values,
                    authored_declarations,
                    diagnostics_observable(report.diagnostics()),
                )
            }
            "style" => {
                let report = parse_style_attribute(&expected.input);
                let (retained, values, authored_declarations) =
                    declaration_observables(report.syntax().as_slice(), "public");
                (
                    report.is_clean(),
                    retained,
                    values,
                    authored_declarations,
                    diagnostics_observable(report.diagnostics()),
                )
            }
            _ => unreachable!("validated fixture entry"),
        };
    Row {
        case_id: expected.case_id.clone(),
        owner: expected.owner.clone(),
        entry: expected.entry.clone(),
        feature: expected.feature.clone(),
        input: expected.input.clone(),
        clean: clean.to_string(),
        retained: nonempty(retained.join("~")),
        values: nonempty(values.join("~")),
        authored_declarations: nonempty(authored_declarations.join("~")),
        diagnostics: nonempty(diagnostics.join("~")),
    }
}

fn nonempty(value: String) -> String {
    if value.is_empty() {
        "-".to_owned()
    } else {
        value
    }
}

fn sheet_observables(rules: &[CssRule]) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut retained = Vec::new();
    let mut values = Vec::new();
    let mut authored_declarations = Vec::new();
    for rule in rules {
        rule_observables(rule, &mut retained, &mut values, &mut authored_declarations);
    }
    (retained, values, authored_declarations)
}

fn rule_observables(
    rule: &CssRule,
    retained: &mut Vec<String>,
    values: &mut Vec<String>,
    authored_declarations: &mut Vec<String>,
) {
    match rule {
        CssRule::Import(_) => retained.push("rule:baseline.rule.import".to_owned()),
        CssRule::LayerStatement(_) => {
            retained.push("rule:baseline.rule.layer-statement".to_owned())
        }
        CssRule::LayerBlock(rule) => {
            retained.push("rule:baseline.rule.layer-block".to_owned());
            for child in rule.rules() {
                rule_observables(child, retained, values, authored_declarations);
            }
        }
        CssRule::FontFace(_) => retained.push("rule:baseline.rule.font-face".to_owned()),
        CssRule::Keyframes(rule) => {
            retained.push("rule:baseline.rule.keyframes".to_owned());
            for block in rule.blocks() {
                for declaration in block.declarations().iter() {
                    let (id, semantic, authored) = declaration_value_observables(
                        declaration.property_name(),
                        declaration.custom(),
                        declaration.known(),
                    );
                    retained.push(format!("property:{id}"));
                    let _ = semantic;
                    authored_declarations.push(format!(
                        "{id}={}:{}@keyframe-grammar:normal",
                        authored.capability,
                        authored.value.as_deref().unwrap_or("<unavailable>")
                    ));
                }
            }
        }
        CssRule::Style(rule) => {
            retained.push("rule:baseline.rule.style".to_owned());
            let (ids, semantic, authored) =
                declaration_observables(rule.declarations().as_slice(), "public");
            retained.extend(ids);
            values.extend(semantic);
            authored_declarations.extend(authored);
        }
        CssRule::Media(rule) => {
            retained.push("rule:baseline.rule.media".to_owned());
            for child in rule.rules() {
                rule_observables(child, retained, values, authored_declarations);
            }
        }
        CssRule::Container(rule) => {
            retained.push("rule:baseline.rule.container".to_owned());
            for child in rule.rules() {
                rule_observables(child, retained, values, authored_declarations);
            }
        }
        CssRule::Scope(rule) => {
            retained.push("rule:baseline.rule.scope".to_owned());
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values, authored_declarations);
            }
        }
        _ => retained.push("rule:future".to_owned()),
    }
}

fn scoped_rule_observables(
    rule: &CssScopedRule,
    retained: &mut Vec<String>,
    values: &mut Vec<String>,
    authored_declarations: &mut Vec<String>,
) {
    match rule {
        CssScopedRule::Style(rule) => {
            retained.push("rule:baseline.rule.style".to_owned());
            let (ids, semantic, authored) =
                declaration_observables(rule.declarations().as_slice(), "public");
            retained.extend(ids);
            values.extend(semantic);
            authored_declarations.extend(authored);
        }
        CssScopedRule::Media(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values, authored_declarations);
            }
        }
        CssScopedRule::Container(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values, authored_declarations);
            }
        }
        CssScopedRule::LayerStatement(_) => {
            retained.push("rule:baseline.rule.layer-statement".to_owned())
        }
        CssScopedRule::LayerBlock(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values, authored_declarations);
            }
        }
        CssScopedRule::Scope(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values, authored_declarations);
            }
        }
    }
}

fn declaration_observables(
    declarations: &[CssDeclaration],
    importance_capability: &str,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut retained = Vec::new();
    let mut values = Vec::new();
    let mut authored_declarations = Vec::new();
    declaration_values(
        declarations
            .iter()
            .map(|declaration| (declaration.property_name(), Some(declaration))),
        &mut retained,
        &mut values,
        &mut authored_declarations,
        importance_capability,
    );
    (retained, values, authored_declarations)
}

fn declaration_values<'a>(
    declarations: impl Iterator<Item = (CssPropertyNameRef<'a>, Option<&'a CssDeclaration>)>,
    retained: &mut Vec<String>,
    values: &mut Vec<String>,
    authored_declarations: &mut Vec<String>,
    importance_capability: &str,
) {
    for (name, declaration) in declarations {
        let id = property_id(name);
        retained.push(format!("property:{id}"));
        if let Some(declaration) = declaration {
            let (_, semantic, authored) =
                declaration_value_observables(name, declaration.custom(), declaration.known());
            let importance = match declaration.importance() {
                CssImportance::Normal => "normal",
                CssImportance::Important => "important",
            };
            values.push(format!("{id}={semantic}@{importance}"));
            authored_declarations.push(format!(
                "{id}={}:{}@{importance_capability}:{importance}",
                authored.capability,
                authored.value.as_deref().unwrap_or("<unavailable>")
            ));
        }
    }
}

struct RuntimeAuthoredValue {
    capability: &'static str,
    value: Option<String>,
}

fn property_id(name: CssPropertyNameRef<'_>) -> String {
    match name {
        CssPropertyNameRef::Known(property) => property.stable_id().to_owned(),
        CssPropertyNameRef::Custom(name) => format!("custom:{}", name.as_str()),
        _ => "future-property".to_owned(),
    }
}

fn declaration_value_observables(
    name: CssPropertyNameRef<'_>,
    custom: Option<&surgeist_css::CssCustomDeclaration>,
    known: Option<&surgeist_css::CssKnownDeclaration>,
) -> (String, String, RuntimeAuthoredValue) {
    let id = property_id(name);
    if let Some(custom) = custom {
        let (semantic, authored) = custom.value().value().map_or_else(
            || {
                (
                    format!("global:{:?}", custom.value().global()),
                    RuntimeAuthoredValue {
                        capability: "deferred-i01",
                        value: None,
                    },
                )
            },
            |value| {
                (
                    value.as_css().to_owned(),
                    RuntimeAuthoredValue {
                        capability: "public",
                        value: Some(value.as_css().to_owned()),
                    },
                )
            },
        );
        return (id, semantic, authored);
    }
    let (semantic, authored) = known.map_or_else(
        || {
            (
                "future".to_owned(),
                RuntimeAuthoredValue {
                    capability: "deferred-i01",
                    value: None,
                },
            )
        },
        known_value,
    );
    (id, semantic, authored)
}

fn declared_value<T: std::fmt::Debug>(
    value: &surgeist_css::CssDeclaredValue<T>,
) -> (String, RuntimeAuthoredValue) {
    match value {
        surgeist_css::CssDeclaredValue::Value(value) => (
            format!("typed:{value:?}"),
            RuntimeAuthoredValue {
                capability: "deferred-i01",
                value: None,
            },
        ),
        surgeist_css::CssDeclaredValue::Global(value) => (
            format!("global:{value:?}"),
            RuntimeAuthoredValue {
                capability: "deferred-i01",
                value: None,
            },
        ),
        surgeist_css::CssDeclaredValue::SubstitutionDependent(value) => (
            format!("substitution:{}", value.as_css()),
            RuntimeAuthoredValue {
                capability: "public",
                value: Some(value.as_css().to_owned()),
            },
        ),
        _ => (
            "future".to_owned(),
            RuntimeAuthoredValue {
                capability: "deferred-i01",
                value: None,
            },
        ),
    }
}

fn known_value(known: &surgeist_css::CssKnownDeclaration) -> (String, RuntimeAuthoredValue) {
    macro_rules! arms {
        ($($variant:ident),+ $(,)?) => { match known {
            $(surgeist_css::CssKnownDeclaration::$variant(value) => declared_value(value),)+
            _ => (
                "future".to_owned(),
                RuntimeAuthoredValue {
                    capability: "deferred-i01",
                    value: None,
                },
            ),
        } };
    }
    match known {
        surgeist_css::CssKnownDeclaration::All(value) => match value {
            surgeist_css::CssAllDeclaredValue::Global(value) => (
                format!("global:{value:?}"),
                RuntimeAuthoredValue {
                    capability: "deferred-i01",
                    value: None,
                },
            ),
            surgeist_css::CssAllDeclaredValue::SubstitutionDependent(value) => (
                format!("substitution:{}", value.as_css()),
                RuntimeAuthoredValue {
                    capability: "public",
                    value: Some(value.as_css().to_owned()),
                },
            ),
            _ => (
                "future".to_owned(),
                RuntimeAuthoredValue {
                    capability: "deferred-i01",
                    value: None,
                },
            ),
        },
        _ => arms!(
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
        ),
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
        |token| format!("{:?}:{}", token.kind(), token.authored()),
    )
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

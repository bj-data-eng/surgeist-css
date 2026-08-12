use std::collections::BTreeSet;

use surgeist_css::{
    CssErrorCode, CssFeatureKind, CssImportance, CssKnownProperty, CssRecoveryAction, CssRule,
    CssSupportStatus, ErrorKind, feature_catalog, feature_metadata, parse_sheet,
    parse_style_attribute, property_metadata,
};

const MIGRATION_RECORD: &str = include_str!("../plans/handoffs/P01-I01-css-migration.md");

const EXPECTED_CARGO_MANIFEST: &str = r#"[package]
name = "surgeist-css"
version = "0.1.0"
edition = "2024"
license = "MIT"
repository = "https://github.com/bj-data-eng/surgeist-css"
readme = "README.md"
description = "Strict CSS ingestion for Surgeist style sheets."

[lib]
name = "surgeist_css"
path = "src/lib.rs"

[features]
default = []
app-strict = []

[dependencies]
cssparser = "=0.37.0"
cssparser-color = "=0.5.0"
"#;

const EXPECTED_COMMAND_EVIDENCE: &[&str] = &[
    "P14.07.PUBLIC-SURFACE|cargo test -p surgeist-css --offline --test public_surface",
    "P14.07.DOCTEST.DEFAULT|cargo test -p surgeist-css --offline --no-default-features --doc",
    "P14.07.DOCTEST.APP-STRICT|cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc",
    "P14.07.RUSTDOC.DEFAULT|RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps",
    "P14.07.RUSTDOC.APP-STRICT|RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps",
    "P14.08.CHECK.DEFAULT|cargo check -p surgeist-css --offline --no-default-features",
    "P14.08.TEST.DEFAULT|cargo test -p surgeist-css --offline --no-default-features",
    "P14.08.CHECK.APP-STRICT|cargo check -p surgeist-css --offline --no-default-features --features app-strict",
    "P14.08.TEST.APP-STRICT|cargo test -p surgeist-css --offline --no-default-features --features app-strict",
    "P14.08.CONFORMANCE-CATALOG|cargo test -p surgeist-css --offline --test conformance_catalog",
    "P14.08.CATALOG-INVENTORY|cargo test -p surgeist-css --offline --test catalog_inventory",
    "P14.08.INITIATIVE-AUDIT|cargo test -p surgeist-css --offline initiative_i01_audit_",
    "P14.08.FORMAT|cargo fmt --check",
    "F2.24.CLIPPY.DEFAULT|cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings",
    "F2.24.CLIPPY.APP-STRICT|cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings",
    "F2.25.CRATE-ROOT-PROHIBITION|rg -n '^#!\\[forbid\\(unsafe_code\\)\\]$' src/lib.rs",
    "F2.25.OWNED-RUST-SCAN|! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\\{|fn|trait|impl|extern)|#!?\\[[[:space:]]*unsafe|#!?\\[[^]]*(allow|expect)\\(unsafe_code\\)' .",
    "P14.09.DIFF-CHECK|git diff --check",
    "P14.09.PROCESS-CHECK.BEFORE-CLEAN|! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'",
    "P14.09.CLEAN|cargo clean",
    "P14.09.TARGET-ABSENT|test ! -d target",
    "P14.09.PROCESS-CHECK.AFTER-CLEAN|! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'",
    "P14.09.STATUS|git status --short",
];

const EXPECTED_ROOT_FOLLOW_UP: &[&str] = &[
    "ROOT.01.PUBLISHED-CANDIDATE|Verify the selected candidate is reachable from the leaf authority's published main, check it against root's committed MSRV, and deliberately update the crates/surgeist-css gitlink.",
    "ROOT.02.FACADE-ADAPTERS|Migrate facade reexports and CSS-to-Surgeist adapters to reports, structured diagnostics, semantic positions, and property-coupled declarations.",
    "ROOT.03.FEATURE-FORWARDING|Decide root feature forwarding for app-strict while keeping ordinary parser access report-based.",
    "ROOT.04.AUTHORED-VALUES|Preserve custom and substitution-dependent authored values until the root-owned cascade/substitution layer resolves them, and carry declaration importance into root-owned cascade input.",
    "ROOT.05.API-ARTIFACTS|Run root's committed API generator and update only root-owned API audit artifacts.",
    "ROOT.06.DOCUMENTATION|Update root documentation and examples for retained syntax, diagnostics, clean reports, coordinates, style attributes, and the removal of whole-sheet rejection semantics.",
    "ROOT.07.INTEGRATION-TESTS|Cover clean and recovered sheets and style attributes, exact diagnostics, non-BMP coordinates, importance, authored-value preservation, property coupling, metadata lookup, and forwarded strict validation.",
    "ROOT.08.ROOT-GATES|Run root's complete workspace, feature, lint, format, API-artifact, dependency, MSRV, unsafe, and publication gates before reporting promotion.",
];

fn contract_lines<'a>(artifact: &'a str, heading: &str) -> Result<Vec<&'a str>, String> {
    let start = format!("{heading}\n```text\n");
    let (_, remainder) = artifact
        .split_once(&start)
        .ok_or_else(|| format!("missing contract start `{heading}`"))?;
    let (body, _) = remainder
        .split_once("\n```\n")
        .ok_or_else(|| format!("missing contract end `{heading}`"))?;
    Ok(body.lines().collect())
}

fn assert_exact_contract(artifact: &str, heading: &str, expected: &[&str]) {
    assert_eq!(
        contract_lines(artifact, heading).expect("declared artifact contract"),
        expected,
        "exact artifact contract `{heading}`"
    );
}

fn contains_full_git_sha(artifact: &str) -> bool {
    artifact
        .split(|character: char| !character.is_ascii_hexdigit())
        .any(|token| token.len() == 40)
}

fn actions(source: &str) -> Vec<CssRecoveryAction> {
    parse_sheet(source)
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.action())
        .collect()
}

#[test]
fn initiative_i01_audit_p14_01_recovery_front_doors_retain_valid_siblings() {
    let sheet_source = ".before { color: red; } @unknown x; .middle { mystery: 1; width: 2px; } ??? { color: black; } .after { height: 3px; }";
    let sheet = parse_sheet(sheet_source);
    assert_eq!(sheet.syntax().rules().len(), 3, "P14.01 retained rules");
    assert_eq!(sheet.diagnostics().len(), 3, "P14.01 diagnosed units");
    assert_eq!(
        sheet
            .diagnostics()
            .iter()
            .map(|diagnostic| diagnostic.action())
            .collect::<Vec<_>>(),
        [
            CssRecoveryAction::DropAtRule,
            CssRecoveryAction::DropDeclaration,
            CssRecoveryAction::DropQualifiedRule,
        ],
        "P14.01 recovery boundaries"
    );

    let attribute = parse_style_attribute("color: red; broken; width: 2px");
    assert_eq!(attribute.syntax().len(), 2, "P14.01 style retention");
    assert_eq!(attribute.diagnostics().len(), 1, "P14.01 style recovery");
    assert_eq!(
        attribute.diagnostics()[0].action(),
        CssRecoveryAction::DropDeclaration,
        "P14.01 style boundary"
    );
}

#[test]
fn initiative_i01_audit_p14_01_all_recovery_actions_are_observable() {
    let cases = [
        (
            CssRecoveryAction::DropDeclaration,
            ".x { mystery: 1; color: red; }",
        ),
        (
            CssRecoveryAction::DropDescriptor,
            "@font-face { font-family: Demo; src: url(demo); mystery: 1; }",
        ),
        (
            CssRecoveryAction::DropQualifiedRule,
            "??? { color: red; } .after { color: blue; }",
        ),
        (CssRecoveryAction::DropAtRule, "@unknown value;"),
        (
            CssRecoveryAction::DropKeyframeBlock,
            "@keyframes fade { fn(a) { opacity: .5; } to { opacity: 1; } }",
        ),
        (
            CssRecoveryAction::DropSelectorListItem,
            ":is(.kept,???) { color: red; }",
        ),
        (
            CssRecoveryAction::ReplaceMediaQueryWithNever,
            "@media screen, ??? { .x { color: red; } }",
        ),
        (
            CssRecoveryAction::RetainWithImplicitClosure,
            ".x { color: red;",
        ),
        (
            CssRecoveryAction::IgnoreLegacyToken,
            "<!-- .x { color: red; }",
        ),
    ];

    for (expected, source) in cases {
        assert!(
            actions(source).contains(&expected),
            "P14.01 missing {expected:?} for {source}"
        );
    }

    let mut over_limit = ":is(".repeat(257);
    over_limit.push_str(".leaf");
    over_limit.push_str(&")".repeat(257));
    over_limit.push_str(" { color: red; }");
    assert!(
        actions(&over_limit).contains(&CssRecoveryAction::StopAtNestingLimit),
        "P14.01 missing StopAtNestingLimit"
    );
}

#[test]
fn initiative_i01_audit_p14_02_f2_22_f2_23_diagnostics_are_exact_and_ordered() {
    let source = ".😀 { mystery: 1; width: bogus; color: red; } @unknown x;";
    let report = parse_sheet(source);
    let diagnostics = report.diagnostics();
    assert_eq!(diagnostics.len(), 3, "P14.02/F2.23 diagnostic count");
    assert_eq!(
        diagnostics
            .iter()
            .map(|diagnostic| diagnostic.error().code())
            .collect::<Vec<_>>(),
        [
            CssErrorCode::UnknownProperty,
            CssErrorCode::InvalidPropertyValue,
            CssErrorCode::UnknownAtRule,
        ],
        "P14.02/F2.23 structured error order"
    );

    let offsets = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.error().position().byte_offset().value())
        .collect::<Vec<_>>();
    assert_eq!(offsets[0], source.find("mystery").expect("mystery"));
    assert_eq!(offsets[1], source.find("bogus").expect("bogus"));
    assert_eq!(
        offsets[2],
        source.find("@unknown").expect("unknown at-rule")
    );
    assert!(offsets.windows(2).all(|pair| pair[0] < pair[1]));

    let first = &diagnostics[0];
    assert_eq!(first.action(), CssRecoveryAction::DropDeclaration);
    assert_eq!(first.span().start().byte_offset().value(), offsets[0]);
    assert_eq!(
        first.error().position().column().value(),
        u32::try_from(source[..offsets[0]].encode_utf16().count()).expect("UTF-16 column"),
        "F2.22 UTF-16 coordinate"
    );
    assert_ne!(
        offsets[0],
        first.error().position().column().value() as usize
    );
    let ErrorKind::UnknownProperty(detail) = first.error().kind() else {
        panic!("P14.02/F2.23 expected structured unknown-property detail");
    };
    assert_eq!(detail.name().as_str(), "mystery");
}

#[test]
fn initiative_i01_audit_p14_03_default_ordinary_parsing_is_deterministic() {
    for source in [
        ".x { color: red; }",
        ".x { mystery: 1; width: 2px; }",
        "@media screen, ??? { .x { color: red; } }",
    ] {
        assert_eq!(parse_sheet(source), parse_sheet(source), "P14.03 {source}");
    }
    for source in ["color: red", "mystery: 1; width: 2px"] {
        assert_eq!(
            parse_style_attribute(source),
            parse_style_attribute(source),
            "P14.03 {source}"
        );
    }
}

#[cfg(feature = "app-strict")]
#[test]
fn initiative_i01_audit_p14_03_app_strict_wraps_the_ordinary_report() {
    let clean_source = ".x { color: red; }";
    let clean = parse_sheet(clean_source);
    assert_eq!(
        surgeist_css::validate_sheet(clean_source),
        Ok(clean.syntax().clone()),
        "P14.03 clean validation"
    );

    let recovered_source = "color: red; mystery: 1; width: 2px";
    let recovered = parse_style_attribute(recovered_source);
    let failure = surgeist_css::validate_style_attribute(recovered_source)
        .expect_err("P14.03 recovered input must fail validation");
    assert!(!failure.diagnostics().is_empty());
    assert_eq!(failure.first(), &recovered.diagnostics()[0]);
    assert_eq!(failure.diagnostics(), recovered.diagnostics());
}

#[test]
fn initiative_i01_audit_p14_04_f2_05_f2_06_f2_20_declarations_are_coupled() {
    let report = parse_style_attribute(
        "--Theme: RGB(1, 2, var(--fallback)); width: var(--size, 2px) !important; color: red",
    );
    assert!(report.is_clean(), "P14.04/F2.5 style-attribute report");
    assert_eq!(report.syntax().len(), 3);

    let custom = report.syntax()[0].custom().expect("P14.04 custom body");
    assert_eq!(custom.name().as_str(), "--Theme");
    assert_eq!(
        custom
            .value()
            .value()
            .expect("P14.04 authored custom value")
            .as_css(),
        "RGB(1, 2, var(--fallback))"
    );

    let width = &report.syntax()[1];
    assert_eq!(width.importance(), CssImportance::Important, "F2.6");
    assert!(matches!(
        width.property_name(),
        surgeist_css::CssPropertyNameRef::Known(CssKnownProperty::Width)
    ));
    let value = width
        .known()
        .expect("P14.04/F2.20 known width")
        .substitution_dependent()
        .expect("P14.04/F2.20 expected property-coupled width value");
    assert_eq!(value.as_css(), "var(--size, 2px)");

    let color = &report.syntax()[2];
    assert!(matches!(
        color.property_name(),
        surgeist_css::CssPropertyNameRef::Known(CssKnownProperty::Color)
    ));
    assert_eq!(
        color.known().map(|known| known.property()),
        Some(CssKnownProperty::Color)
    );
}

#[test]
fn initiative_i01_audit_p14_05_f2_18_catalog_is_independent_and_truthful() {
    let catalog = feature_catalog();
    assert_eq!(catalog.len(), 219, "P14.05/F2.18 catalog size");
    assert_eq!(
        catalog
            .iter()
            .filter(|record| record.kind() == CssFeatureKind::Property)
            .count(),
        179,
        "P14.05/F2.18 property inventory"
    );

    let ids = catalog
        .iter()
        .map(|record| record.id().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), catalog.len(), "P14.05/F2.18 unique IDs");
    for record in catalog {
        match record.status() {
            CssSupportStatus::Complete => {
                assert_eq!(record.supported_subset(), None, "{}", record.id().as_str());
                assert_eq!(
                    record.unsupported_remainder(),
                    None,
                    "{}",
                    record.id().as_str()
                );
                assert_eq!(record.recognized_unsupported_code(), None);
            }
            CssSupportStatus::Partial => {
                assert!(
                    record
                        .supported_subset()
                        .is_some_and(|text| !text.is_empty())
                );
                assert!(
                    record
                        .unsupported_remainder()
                        .is_some_and(|text| !text.is_empty())
                );
                assert_eq!(record.recognized_unsupported_code(), None);
            }
            CssSupportStatus::RecognizedUnsupported => {
                assert_eq!(record.supported_subset(), None);
                assert_eq!(record.unsupported_remainder(), None);
                assert!(record.recognized_unsupported_code().is_some());
            }
        }
    }

    let width = property_metadata("WiDtH").expect("P14.05 width metadata");
    assert_eq!(width.property(), CssKnownProperty::Width);
    assert_eq!(width.feature().id().as_str(), "baseline.property.width");
    assert_eq!(property_metadata("--width"), None);
    assert_eq!(property_metadata("not-a-property"), None);
    assert_eq!(feature_metadata("BASELINE.PROPERTY.WIDTH"), None);
}

#[test]
fn initiative_i01_audit_p14_06_f2_15_encoding_is_retained_or_diagnosed() {
    let valid = parse_sheet("@charset \"UTF-8\"; .x { color: red; }");
    assert!(valid.is_clean(), "P14.06/F2.15 valid encoding");
    assert_eq!(
        valid
            .syntax()
            .encoding()
            .expect("P14.06/F2.15 encoding metadata")
            .label(),
        "UTF-8"
    );

    for source in [
        "@charset bogus; .x { color: red; }",
        "@charset \"UTF-8\" .x { color: red; }",
        ".x { color: red; } @charset \"UTF-8\";",
    ] {
        let report = parse_sheet(source);
        assert!(
            report
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.error().code()
                    == CssErrorCode::InvalidEncodingDeclaration),
            "P14.06/F2.15 missing encoding diagnostic for {source}"
        );
    }
}

#[test]
fn initiative_i01_audit_p14_07_f2_21_public_surface_contract_is_observable() {
    let recovered = parse_sheet(".kept { color: red; } @unknown x;");
    assert!(!recovered.is_clean());
    let (sheet, diagnostics) = recovered.into_parts();
    assert_eq!(sheet.rules().len(), 1);
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].error().code(), CssErrorCode::UnknownAtRule);
    assert_eq!(diagnostics[0].action(), CssRecoveryAction::DropAtRule);

    let attribute = parse_style_attribute("--Theme: var(--fallback); width: 2px !important");
    assert!(attribute.is_clean());
    assert_eq!(attribute.syntax().len(), 2);
    assert_eq!(attribute.syntax()[1].importance(), CssImportance::Important);
    assert_eq!(
        feature_metadata("foundation.declaration.importance")
            .expect("P14.07/F2.21 metadata lookup")
            .status(),
        CssSupportStatus::Complete
    );

    let readme_contract = "The ordinary `parse_sheet` and `parse_style_attribute` entry points use browser recovery: each returns a report containing valid retained syntax and every structured recovery diagnostic in source order. Unsupported or malformed source units are dropped, replaced, retained with an implicit closure, ignored, or stopped at the documented boundary; valid siblings remain eligible. A clean report means that no recovery diagnostic was produced. An empty retained tree alone does not establish that the source was clean, so consumers should inspect `is_clean()` or `diagnostics()` rather than infer validity from syntax length.";
    assert!(include_str!("../README.md").contains(readme_contract));

    let crate_boundary_contract = "//! This crate owns authored CSS syntax, intrinsic grammar validation, recovery\n//! boundaries, diagnostic provenance, and support metadata. It does not apply\n//! cascade or inheritance; substitute custom properties; validate computed\n//! post-substitution values; evaluate queries; match selectors; resolve URLs,\n//! resources, units, or colors; perform layout, painting, or animation; expose a\n//! mutable CSSOM; or lower CSS into sibling Surgeist types.";
    assert!(include_str!("../src/lib.rs").contains(crate_boundary_contract));

    let command_evidence = contract_lines(
        MIGRATION_RECORD,
        "### Configured Command Evidence Manifest V1",
    )
    .expect("P14.07 configured command manifest");
    for command_id in [
        "P14.07.PUBLIC-SURFACE",
        "P14.07.DOCTEST.DEFAULT",
        "P14.07.DOCTEST.APP-STRICT",
        "P14.07.RUSTDOC.DEFAULT",
        "P14.07.RUSTDOC.APP-STRICT",
    ] {
        assert!(
            command_evidence
                .iter()
                .any(|entry| entry.starts_with(&format!("{command_id}|"))),
            "missing exact configured command identity {command_id}"
        );
    }
}

#[test]
fn initiative_i01_audit_p14_08_f2_24_f2_25_exact_guards_and_commands_are_fixed() {
    assert_eq!(include_str!("../Cargo.toml"), EXPECTED_CARGO_MANIFEST);
    assert!(
        include_str!("../src/lib.rs").starts_with("#![forbid(unsafe_code)]\n"),
        "F2.25 crate-root unsafe prohibition"
    );
    assert!(!include_str!("../src/lib.rs").contains("allow(unsafe_code)"));
    assert!(!include_str!("../src/lib.rs").contains("expect(unsafe_code)"));

    assert_exact_contract(
        MIGRATION_RECORD,
        "### Configured Command Evidence Manifest V1",
        EXPECTED_COMMAND_EVIDENCE,
    );
}

#[test]
fn initiative_i01_audit_p14_09_migration_artifact_is_sha_free_and_complete() {
    assert!(!contains_full_git_sha(MIGRATION_RECORD));
    for forbidden_revision_field in [
        "candidate_sha:",
        "cycle_head_sha:",
        "remote_main_at_push_sha:",
        "remote_main_at_readback_sha:",
    ] {
        assert!(!MIGRATION_RECORD.contains(forbidden_revision_field));
    }
    assert_exact_contract(
        MIGRATION_RECORD,
        "### Root Follow-Up Manifest V1",
        EXPECTED_ROOT_FOLLOW_UP,
    );

    let omitted = MIGRATION_RECORD.replacen(EXPECTED_ROOT_FOLLOW_UP[4], "", 1);
    assert_ne!(
        contract_lines(&omitted, "### Root Follow-Up Manifest V1")
            .expect("mutated root manifest remains parseable"),
        EXPECTED_ROOT_FOLLOW_UP,
        "P14.09 omission mutation must be rejected"
    );
    let altered = MIGRATION_RECORD.replacen(
        EXPECTED_COMMAND_EVIDENCE[13],
        "F2.24.CLIPPY.DEFAULT|cargo clippy -p surgeist-css",
        1,
    );
    assert_ne!(
        contract_lines(&altered, "### Configured Command Evidence Manifest V1")
            .expect("mutated command manifest remains parseable"),
        EXPECTED_COMMAND_EVIDENCE,
        "F2.24 command alteration mutation must be rejected"
    );
}

#[test]
fn initiative_i01_audit_f2_19_parser_owned_recovery_state_is_read_only() {
    let report = parse_sheet("@media screen, ??? { .x { color: red; } }");
    let [CssRule::Media(media)] = report.syntax().rules() else {
        panic!("F2.19 expected retained media rule");
    };
    assert!(
        media
            .query()
            .queries()
            .iter()
            .any(|query| query.is_guaranteed_false()),
        "F2.19 parser-owned Never state"
    );
    assert_eq!(
        report.diagnostics()[0].action(),
        CssRecoveryAction::ReplaceMediaQueryWithNever
    );
}

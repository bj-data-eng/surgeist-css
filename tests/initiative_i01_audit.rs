use std::collections::BTreeSet;

use surgeist_css::{
    CssDeclaredValue, CssErrorCode, CssFeatureKind, CssImportance, CssKnownDeclaration,
    CssKnownProperty, CssRecoveryAction, CssRule, CssSupportStatus, ErrorKind, feature_catalog,
    feature_metadata, parse_sheet, parse_style_attribute, property_metadata,
};

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
    let Some(CssKnownDeclaration::Width(CssDeclaredValue::SubstitutionDependent(value))) =
        width.known()
    else {
        panic!("P14.04/F2.20 expected property-coupled width value");
    };
    assert_eq!(value.as_css(), "var(--size, 2px)");

    let color = &report.syntax()[2];
    assert!(matches!(
        color.property_name(),
        surgeist_css::CssPropertyNameRef::Known(CssKnownProperty::Color)
    ));
    assert!(matches!(color.known(), Some(CssKnownDeclaration::Color(_))));
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
fn initiative_i01_audit_p14_07_f2_21_public_guidance_names_the_breaking_surface() {
    let guidance = format!(
        "{}\n{}",
        include_str!("../README.md"),
        include_str!("../src/lib.rs")
    )
    .to_ascii_lowercase();
    for required in [
        "browser recovery",
        "clean report",
        "style attribute",
        "utf-16",
        "!important",
        "custom properties",
        "substitution-dependent",
        "support status",
        "app-strict",
        "does not apply cascade",
    ] {
        assert!(
            guidance.contains(required),
            "P14.07/F2.21 missing guidance `{required}`"
        );
    }
}

#[test]
fn initiative_i01_audit_p14_08_f2_24_f2_25_manifest_and_crate_guards_are_fixed() {
    let manifest = include_str!("../Cargo.toml");
    for required in [
        "edition = \"2024\"",
        "default = []",
        "app-strict = []",
        "cssparser = \"=0.37.0\"",
        "cssparser-color = \"=0.5.0\"",
    ] {
        assert!(
            manifest.contains(required),
            "P14.08/F2.24 missing manifest contract `{required}`"
        );
    }
    assert!(!manifest.contains("rust-version"), "P14.08 leaf MSRV");

    // Compiling this external integration target proves the crate-root
    // `forbid(unsafe_code)` attribute is active. The T4 command set supplies the
    // complementary whole-owned-source scan and warning-denied Clippy evidence.
    assert_eq!(
        feature_metadata("foundation.declaration.importance")
            .expect("P14.08/F2.25 public crate compiled")
            .status(),
        CssSupportStatus::Complete
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

use surgeist_css::{
    CssDeclaredValue, CssErrorCode, CssFeatureKind, CssImportance, CssKnownDeclaration,
    CssKnownProperty, CssPropertyNameRef, CssRecoveryAction, CssRule, CssSupportStatus, ErrorKind,
    feature_catalog, feature_metadata, parse_sheet, parse_style_attribute, property_metadata,
};

fn rule_kind(rule: &CssRule) -> &'static str {
    match rule {
        CssRule::Import(_) => "import",
        CssRule::LayerStatement(_) => "layer statement",
        CssRule::LayerBlock(_) => "layer block",
        CssRule::FontFace(_) => "font face",
        CssRule::Keyframes(_) => "keyframes",
        CssRule::Style(_) => "style",
        CssRule::Media(_) => "media",
        CssRule::Container(_) => "container",
        CssRule::Scope(_) => "scope",
        _ => "future rule",
    }
}

fn error_kind(kind: &ErrorKind) -> &'static str {
    match kind {
        ErrorKind::UnknownProperty(_) => "unknown property",
        ErrorKind::InvalidPropertyValue(_) => "invalid property value",
        _ => "other or future error",
    }
}

fn feature_kind(kind: CssFeatureKind) -> &'static str {
    match kind {
        CssFeatureKind::Rule => "rule",
        CssFeatureKind::Declaration => "declaration",
        CssFeatureKind::Descriptor => "descriptor",
        CssFeatureKind::Value => "value",
        CssFeatureKind::Property => "property",
        CssFeatureKind::Selector => "selector",
        CssFeatureKind::MediaQuery => "media query",
        CssFeatureKind::ContainerQuery => "container query",
        _ => "future feature kind",
    }
}

fn emitted_actions(source: &str) -> Vec<CssRecoveryAction> {
    parse_sheet(source)
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.action())
        .collect()
}

#[test]
fn public_surface_sheet_reports_expose_retained_syntax_and_structured_recovery() {
    let clean = parse_sheet(".clean { color: red; }");
    assert!(clean.is_clean());
    assert!(clean.diagnostics().is_empty());
    assert_eq!(clean.syntax().rules().len(), 1);
    assert_eq!(rule_kind(&clean.syntax().rules()[0]), "style");

    let source = ".before { color: red; } @unknown x; .after { color: blue; }";
    let recovered = parse_sheet(source);
    assert!(!recovered.is_clean());
    assert_eq!(recovered.syntax().rules().len(), 2);
    let diagnostic = &recovered.diagnostics()[0];
    assert_eq!(diagnostic.error().code(), CssErrorCode::UnknownAtRule);
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropAtRule);
    assert_eq!(
        diagnostic.error().position().byte_offset().value(),
        source.find("@unknown").expect("unknown at-rule")
    );
    assert_eq!(diagnostic.span().start(), diagnostic.error().position());
    assert_eq!(
        diagnostic.span().end().byte_offset().value(),
        source.find(" .after").expect("later retained sibling")
    );

    let (sheet, diagnostics) = recovered.into_parts();
    assert_eq!(sheet.rules().len(), 2);
    assert_eq!(diagnostics.len(), 1);
}

#[test]
fn public_surface_style_attributes_preserve_importance_custom_and_substitution_syntax() {
    let source = "color: red; --Theme: RGB(1, 2, var(--fallback)); width: var(--size, 2px) !important; mystery: 1";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().as_slice().len(), 3);
    assert_eq!(report.syntax().iter().count(), 3);
    assert_eq!(report.syntax().len(), 3);
    assert!(!report.syntax().is_empty());
    assert_eq!(report.diagnostics().len(), 1);

    let custom = &report.syntax()[1];
    assert!(matches!(
        custom.body(),
        surgeist_css::CssDeclarationBody::Custom(_)
    ));
    assert!(custom.known().is_none());
    let custom = custom.custom().expect("custom declaration");
    assert_eq!(custom.name().as_str(), "--Theme");
    assert_eq!(
        custom
            .value()
            .value()
            .expect("authored custom value")
            .as_css(),
        "RGB(1, 2, var(--fallback))"
    );
    assert!(custom.value().global().is_none());

    let width = &report.syntax()[2];
    assert_eq!(width.importance(), CssImportance::Important);
    assert_eq!(
        width.position().byte_offset().value(),
        source.find("width").expect("width declaration")
    );
    assert!(matches!(
        width.property_name(),
        CssPropertyNameRef::Known(CssKnownProperty::Width)
    ));
    let Some(CssKnownDeclaration::Width(value)) = width.known() else {
        panic!("expected coupled width declaration");
    };
    assert!(value.value().is_none());
    assert!(value.global().is_none());
    assert_eq!(
        value
            .substitution_dependent()
            .expect("substitution-dependent value")
            .as_css(),
        "var(--size, 2px)"
    );
    assert!(matches!(value, CssDeclaredValue::SubstitutionDependent(_)));
}

#[test]
fn public_surface_metadata_exposes_every_final_accessor_and_bounded_status() {
    assert_eq!(feature_catalog().len(), 219);

    let complete =
        feature_metadata("foundation.declaration.importance").expect("importance catalog record");
    assert_eq!(complete.id().as_str(), "foundation.declaration.importance");
    assert_eq!(feature_kind(complete.kind()), "declaration");
    assert_eq!(
        complete.spelling(),
        "terminal declaration !important annotation"
    );
    assert!(!complete.production().is_empty());
    assert_eq!(complete.status(), CssSupportStatus::Complete);
    assert_eq!(complete.supported_subset(), None);
    assert_eq!(complete.unsupported_remainder(), None);
    assert_eq!(complete.recognized_unsupported_code(), None);
    assert!(complete.source().url().is_some());
    assert_eq!(complete.source().repository_provenance(), None);

    let partial = feature_metadata("baseline.rule.style").expect("style-rule record");
    assert_eq!(partial.status(), CssSupportStatus::Partial);
    assert!(
        partial
            .supported_subset()
            .is_some_and(|text| !text.is_empty())
    );
    assert!(
        partial
            .unsupported_remainder()
            .is_some_and(|text| !text.is_empty())
    );
    assert_eq!(partial.source().url(), None);
    assert_eq!(
        partial.source().repository_provenance(),
        Some("4b288d6:src/parser/mod.rs")
    );

    let unsupported = feature_metadata("later.rule.namespace").expect("namespace record");
    assert_eq!(
        unsupported.status(),
        CssSupportStatus::RecognizedUnsupported
    );
    assert_eq!(
        unsupported.recognized_unsupported_code(),
        Some(CssErrorCode::UnsupportedAtRule)
    );

    let property = property_metadata("WiDtH").expect("ASCII-insensitive property lookup");
    assert_eq!(property.feature().id().as_str(), "baseline.property.width");
    assert_eq!(property.property(), CssKnownProperty::Width);
    assert_eq!(property.canonical_name(), "width");
    assert!(property.aliases().is_empty());
    assert_eq!(property_metadata("--width"), None);
    assert_eq!(property_metadata("not-a-property"), None);
    assert_eq!(feature_metadata("BASELINE.PROPERTY.WIDTH"), None);
}

#[test]
fn public_surface_non_bmp_coordinates_are_byte_line_and_utf16_based() {
    let source = ".😀 { mystery: 1; color: red; }";
    let report = parse_sheet(source);
    let diagnostic = &report.diagnostics()[0];
    let position = diagnostic.error().position();
    let byte_offset = source.find("mystery").expect("unknown property");

    assert_eq!(position.byte_offset().value(), byte_offset);
    assert_eq!(position.line().value(), 0);
    assert_eq!(
        position.column().value(),
        u32::try_from(source[..byte_offset].encode_utf16().count()).expect("UTF-16 column")
    );
    assert_ne!(
        position.byte_offset().value(),
        position.column().value() as usize
    );
    assert_eq!(error_kind(diagnostic.error().kind()), "unknown property");
    let ErrorKind::UnknownProperty(detail) = diagnostic.error().kind() else {
        panic!("expected typed unknown-property detail");
    };
    assert_eq!(detail.name().as_str(), "mystery");
}

#[test]
fn public_surface_emits_all_ten_recovery_actions() {
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
            emitted_actions(source).contains(&expected),
            "{source} did not emit {expected:?}"
        );
    }

    let mut over_limit = ":is(".repeat(257);
    over_limit.push_str(".leaf");
    over_limit.push_str(&")".repeat(257));
    over_limit.push_str(" { color: red; }");
    assert!(
        emitted_actions(&over_limit).contains(&CssRecoveryAction::StopAtNestingLimit),
        "over-limit selector did not emit StopAtNestingLimit"
    );
}

#[test]
fn readme_describes_recovery_and_rejects_the_obsolete_strict_only_contract() {
    let readme = include_str!("../README.md");
    let normalized = readme.to_ascii_lowercase();

    for required in [
        "browser recovery",
        "clean report",
        "diagnostic",
        "utf-16",
        "!important",
        "custom properties",
        "substitution-dependent",
        "support status",
        "style attribute",
        "app-strict",
    ] {
        assert!(
            normalized.contains(required),
            "README is missing `{required}`"
        );
    }

    for stale in [
        "does not recover from invalid application CSS",
        "reject the whole sheet",
        "rejects the whole sheet",
    ] {
        assert!(
            !normalized.contains(stale),
            "README retains stale claim `{stale}`"
        );
    }
}

#[cfg(feature = "app-strict")]
#[test]
fn public_surface_enabled_validators_accept_clean_reports_and_preserve_failures() {
    let sheet = surgeist_css::validate_sheet(".x { color: red; }")
        .expect("clean sheet passes application-strict validation");
    assert_eq!(sheet.rules().len(), 1);

    let ordinary = parse_style_attribute("color: red; mystery: 1");
    let failure = surgeist_css::validate_style_attribute("color: red; mystery: 1")
        .expect_err("recovered style attribute fails application-strict validation");
    assert_eq!(failure.first(), &ordinary.diagnostics()[0]);
    assert_eq!(failure.diagnostics(), ordinary.diagnostics());
    assert_eq!(failure.into_diagnostics(), ordinary.diagnostics());
}

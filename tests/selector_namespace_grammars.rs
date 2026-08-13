use surgeist_css::{CssErrorCode, CssRecoveryAction, CssRule, ErrorKind, parse_sheet};

#[test]
fn namespace_rules_obey_namespaces3_prelude_ordering() {
    let report = parse_sheet("@namespace svg url(https://example.test/svg);");

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().rules().len(), 1);
}

#[test]
fn late_namespace_is_a_placement_error_not_an_unsupported_rule() {
    let source = concat!(
        ".before { color: red; } ",
        "@namespace svg url(http://example.test/a;b); ",
        ".after { color: blue; }",
    );
    let report = parse_sheet(source);

    assert!(matches!(
        report.syntax().rules(),
        [CssRule::Style(_), CssRule::Style(_)]
    ));
    let [diagnostic] = report.diagnostics() else {
        panic!("expected one late-namespace recovery diagnostic")
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidAtRulePlacement
    );
    assert_ne!(diagnostic.error().code(), CssErrorCode::UnsupportedAtRule);
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropAtRule);
    let ErrorKind::InvalidAtRulePlacement(detail) = diagnostic.error().kind() else {
        panic!("expected namespace placement payload")
    };
    assert_eq!(detail.name().as_str(), "namespace");
    assert_eq!(
        detail.expected_context().as_str(),
        "after imports and before every layer or body rule"
    );
}

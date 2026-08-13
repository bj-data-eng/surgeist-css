use surgeist_css::parse_sheet;

#[test]
fn namespace_rules_obey_namespaces3_prelude_ordering() {
    let report = parse_sheet("@namespace svg url(https://example.test/svg);");

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().rules().len(), 1);
}

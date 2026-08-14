use surgeist_css::parse_style_attribute;

#[test]
fn valid_authored_flex_flow_is_retained_without_recovery() {
    let report = parse_style_attribute("flex-flow: column wrap");

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 1);
}

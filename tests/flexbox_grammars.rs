use surgeist_css::parse_style_attribute;

#[test]
fn c14_flex_flow_retain_typed_structure() {
    let report = parse_style_attribute("flex-flow: wrap-reverse column");

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 1);
}

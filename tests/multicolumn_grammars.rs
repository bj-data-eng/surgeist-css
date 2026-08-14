use surgeist_css::parse_style_attribute;

#[test]
fn c14_multicolumn_properties_retain_typed_structure() {
    let report = parse_style_attribute(concat!(
        "column-count: 3; ",
        "column-fill: balance-all; ",
        "column-rule: thick dashed rebeccapurple; ",
        "column-rule-color: currentcolor; ",
        "column-rule-style: double; ",
        "column-rule-width: 2px; ",
        "column-span: all; ",
        "column-width: 12em; ",
        "columns: 4 10rem",
    ));

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 9);
}

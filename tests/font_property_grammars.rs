use surgeist_css::parse_style_attribute;

#[test]
fn font_size_family_line_height_and_shorthand_follow_fonts3() {
    let report = parse_style_attribute("font-size: medium; font: menu");

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 2);
}

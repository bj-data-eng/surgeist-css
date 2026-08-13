use surgeist_css::{CssKnownProperty, parse_style_attribute};

#[test]
fn opacity_percentage_is_retained_with_its_valid_sibling() {
    let report = parse_style_attribute("opacity: 150%; color: red");

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 2);
    assert_eq!(
        report.syntax()[0].known().map(|known| known.property()),
        Some(CssKnownProperty::Opacity),
    );
    assert_eq!(
        report.syntax()[1].known().map(|known| known.property()),
        Some(CssKnownProperty::Color),
    );
}

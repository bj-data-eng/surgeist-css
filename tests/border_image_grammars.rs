use surgeist_css::parse_style_attribute;

#[test]
fn c13_border_images_retain_typed_structure() {
    let report = parse_style_attribute(
        "border-image: url(frame.png) 10% 20 30% 40 fill / 1 auto 25% 4px / 0 2px 3 4px round space",
    );

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 1);
    assert_eq!(
        report.syntax()[0]
            .known()
            .map(|known| known.property().canonical_name()),
        Some("border-image"),
    );
}

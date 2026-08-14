use surgeist_css::{CssErrorCode, CssKnownProperty, parse_style_attribute};

#[test]
fn css2_residual_properties_retain_typed_values() {
    let report = parse_style_attribute(concat!(
        "border-collapse: collapse; ",
        "border-spacing: 2px 3px; ",
        "caption-side: bottom; ",
        "clip: rect(auto, 10px, 20px, -1px); ",
        "empty-cells: hide; ",
        "orphans: 3; ",
        "page-break-after: right; ",
        "page-break-before: always; ",
        "page-break-inside: avoid; ",
        "quotes: \"«\" \"»\" \"‹\" \"›\"; ",
        "table-layout: fixed; ",
        "widows: 4; ",
        "word-spacing: -0.25em; ",
        "color: red",
    ));

    if !report.is_clean() {
        assert_eq!(report.diagnostics().len(), 13);
        assert!(
            report
                .diagnostics()
                .iter()
                .all(|diagnostic| diagnostic.error().code() == CssErrorCode::UnknownProperty)
        );
        assert_eq!(report.syntax().len(), 1);
        assert_eq!(
            report.syntax()[0].known().unwrap().property(),
            CssKnownProperty::Color,
        );
    }

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 14);
    assert_eq!(
        report
            .syntax()
            .iter()
            .map(|declaration| declaration.known().unwrap().property().canonical_name())
            .collect::<Vec<_>>(),
        [
            "border-collapse",
            "border-spacing",
            "caption-side",
            "clip",
            "empty-cells",
            "orphans",
            "page-break-after",
            "page-break-before",
            "page-break-inside",
            "quotes",
            "table-layout",
            "widows",
            "word-spacing",
            "color",
        ],
    );
}

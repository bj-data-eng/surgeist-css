use surgeist_css::{CssErrorCode, CssKnownProperty, parse_style_attribute};

#[test]
fn writing_modes_and_legacy_alias_are_typed() {
    let report = parse_style_attribute(concat!(
        "text-combine-upright: all; ",
        "text-orientation: sideways; ",
        "unicode-bidi: isolate-override; ",
        "glyph-orientation-vertical: 90; ",
        "color: red",
    ));

    if !report.is_clean() {
        assert_eq!(report.diagnostics().len(), 4);
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
    assert_eq!(report.syntax().len(), 5);
    assert_eq!(
        report
            .syntax()
            .iter()
            .map(|declaration| declaration.known().unwrap().property().canonical_name())
            .collect::<Vec<_>>(),
        [
            "text-combine-upright",
            "text-orientation",
            "unicode-bidi",
            "text-orientation",
            "color",
        ],
    );
}

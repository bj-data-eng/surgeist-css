use surgeist_css::{CssErrorCode, CssKnownProperty, parse_style_attribute};

#[test]
fn residual_ui_containment_and_compositing_properties_are_typed() {
    let report = parse_style_attribute(concat!(
        "caret-color: rebeccapurple; ",
        "outline-offset: -2px; ",
        "resize: horizontal; ",
        "contain: paint size; ",
        "transform-box: view-box; ",
        "background-blend-mode: multiply, luminosity; ",
        "isolation: isolate; ",
        "mix-blend-mode: soft-light; ",
        "color: red",
    ));

    if !report.is_clean() {
        assert_eq!(report.diagnostics().len(), 8);
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
    assert_eq!(report.syntax().len(), 9);
    assert_eq!(
        report
            .syntax()
            .iter()
            .map(|declaration| declaration.known().unwrap().property().canonical_name())
            .collect::<Vec<_>>(),
        [
            "caret-color",
            "outline-offset",
            "resize",
            "contain",
            "transform-box",
            "background-blend-mode",
            "isolation",
            "mix-blend-mode",
            "color",
        ],
    );
}

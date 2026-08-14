use surgeist_css::{CssErrorCode, parse_sheet};

#[test]
fn counter_style_rules_retain_valid_core_definitions() {
    let report = parse_sheet(concat!(
        ".before { color: red; } ",
        "@counter-style cycle { system: cyclic; symbols: ● ○; prefix: 👍; suffix: \" \"; } ",
        "@counter-style digits { system: numeric; symbols: \"0\" \"1\"; } ",
        "@counter-style letters { system: alphabetic; symbols: a b; symbols: x y; } ",
        ".after { color: blue; }",
    ));

    assert!(
        report.is_clean(),
        "valid core counter styles should not recover: {:?}",
        report.diagnostics()
    );
    assert_eq!(report.syntax().rules().len(), 5);
    assert!(
        report
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.error().code() != CssErrorCode::UnsupportedAtRule)
    );
}

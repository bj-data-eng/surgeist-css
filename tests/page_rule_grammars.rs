use surgeist_css::{CssErrorCode, parse_sheet};

#[test]
fn page_rules_and_pseudos_retain_valid_authored_structure() {
    let report = parse_sheet(concat!(
        "@import \"print.css\"; ",
        "@page { margin: 1cm; } ",
        "@page :left { margin-left: -12mm !important; } ",
        "@page :right { margin-right: 10%; } ",
        "@page :first { margin-top: auto; margin-bottom: 0; }",
    ));

    assert!(
        report.is_clean(),
        "valid page rules and pseudos should not recover: {:?}",
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

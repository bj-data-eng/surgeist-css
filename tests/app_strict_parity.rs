#![cfg(feature = "app-strict")]

use surgeist_css::{
    CssDeclarationList, CssParseReport, CssRecoveryAction, CssSheet, parse_sheet,
    parse_style_attribute, validate_sheet, validate_style_attribute,
};

fn assert_sheet_parity(source: &str) -> CssParseReport<CssSheet> {
    let ordinary = parse_sheet(source);
    let strict = validate_sheet(source);

    if ordinary.is_clean() {
        assert_eq!(strict, Ok(ordinary.syntax().clone()), "{source}");
    } else {
        let failure = strict.expect_err("recovered sheet must fail strict validation");
        assert_eq!(failure.first(), &ordinary.diagnostics()[0], "{source}");
        assert_eq!(failure.diagnostics(), ordinary.diagnostics(), "{source}");
        assert_eq!(
            failure.into_diagnostics(),
            ordinary.diagnostics(),
            "{source}"
        );
    }

    ordinary
}

fn assert_style_parity(source: &str) -> CssParseReport<CssDeclarationList> {
    let ordinary = parse_style_attribute(source);
    let strict = validate_style_attribute(source);

    if ordinary.is_clean() {
        assert_eq!(strict, Ok(ordinary.syntax().clone()), "{source}");
    } else {
        let failure = strict.expect_err("recovered style attribute must fail strict validation");
        assert_eq!(failure.first(), &ordinary.diagnostics()[0], "{source}");
        assert_eq!(failure.diagnostics(), ordinary.diagnostics(), "{source}");
        assert_eq!(
            failure.into_diagnostics(),
            ordinary.diagnostics(),
            "{source}"
        );
    }

    ordinary
}

fn nested_selector(depth: usize) -> String {
    format!("{}{}{}", ":is(".repeat(depth), ".leaf", ")".repeat(depth)) + "{color:red}"
}

#[test]
fn app_strict_parity_clean_and_recovered_sheet_and_style_results_are_exact() {
    assert!(assert_sheet_parity(".x { color: red; }").is_clean());
    assert!(assert_style_parity("color: red").is_clean());

    let sheet = assert_sheet_parity(".x { mystery: 1; }");
    assert_eq!(sheet.diagnostics().len(), 1);
    let style = assert_style_parity("mystery: 1");
    assert_eq!(style.diagnostics().len(), 1);
}

#[test]
fn app_strict_parity_preserves_multiple_diagnostics_and_every_special_action() {
    let sheet = assert_sheet_parity("<!-- .x { mystery: 1; width: nope; } -->");
    assert!(sheet.diagnostics().len() >= 4);
    assert!(
        sheet
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.action() == CssRecoveryAction::IgnoreLegacyToken)
    );

    let style = assert_style_parity("mystery: 1; width: nope; color: red");
    assert_eq!(style.diagnostics().len(), 2);

    let never = assert_sheet_parity("@media screen, ??? { .x { color: red; } }");
    assert!(never.diagnostics().iter().any(|diagnostic| {
        diagnostic.action() == CssRecoveryAction::ReplaceMediaQueryWithNever
    }));

    let implicit_sheet = assert_sheet_parity(".x { color: red;");
    assert!(
        implicit_sheet.diagnostics().iter().any(|diagnostic| {
            diagnostic.action() == CssRecoveryAction::RetainWithImplicitClosure
        })
    );
    let implicit_style = assert_style_parity("--x: f(value");
    assert!(
        implicit_style.diagnostics().iter().any(|diagnostic| {
            diagnostic.action() == CssRecoveryAction::RetainWithImplicitClosure
        })
    );
}

#[test]
fn app_strict_parity_preserves_structural_and_specialized_depth_diagnostics() {
    let mut structural = ".x{".repeat(257);
    structural.push_str("color:red;");
    structural.push_str(&"}".repeat(257));
    let structural = assert_sheet_parity(&structural);
    assert!(
        structural
            .diagnostics()
            .iter()
            .any(|diagnostic| { diagnostic.action() == CssRecoveryAction::StopAtNestingLimit })
    );

    let selector = assert_sheet_parity(&nested_selector(257));
    assert!(
        selector
            .diagnostics()
            .iter()
            .any(|diagnostic| { diagnostic.action() == CssRecoveryAction::StopAtNestingLimit })
    );

    let mut style = String::from("--x:");
    style.push_str(&"f(".repeat(257));
    style.push('x');
    style.push_str(&")".repeat(257));
    let style = assert_style_parity(&style);
    assert!(
        style
            .diagnostics()
            .iter()
            .any(|diagnostic| { diagnostic.action() == CssRecoveryAction::StopAtNestingLimit })
    );
}

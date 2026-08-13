use surgeist_css::{CssRule, parse_sheet};

#[test]
fn supports_conditions_and_group_rules_follow_conditional3() {
    let report = parse_sheet("@supports (display: grid) { .x { color: red; } }");

    assert!(report.is_clean(), "diagnostics: {:?}", report.diagnostics());
    assert_eq!(report.syntax().rules().len(), 1);
    assert!(!matches!(report.syntax().rules()[0], CssRule::Style(_)));
}

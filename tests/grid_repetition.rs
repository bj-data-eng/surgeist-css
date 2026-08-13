use surgeist_css::{
    CssErrorCode, CssKnownProperty, CssRecoveryAction, parse_style_attribute,
};

#[test]
fn grid_repeat_models_reject_invalid_cross_products() {
    for invalid in [
        "grid-template-columns: repeat(2, repeat(3, 10px))",
        "grid-template-columns: repeat(auto-fit, 1fr)",
    ] {
        let source = format!("{invalid}; color: red");
        let report = parse_style_attribute(&source);

        assert_eq!(report.syntax().len(), 1, "{source}");
        assert_eq!(
            report.syntax()[0]
                .known()
                .expect("retained sibling")
                .property(),
            CssKnownProperty::Color,
            "{source}",
        );
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: invalid repetition must recover once");
        };
        assert_eq!(diagnostic.error().code(), CssErrorCode::InvalidPropertyValue);
        assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    }
}

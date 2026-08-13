use surgeist_css::{CssMediaQuery, CssRule, parse_sheet};

#[test]
fn mq3_named_types_and_features_follow_exact_domains() {
    let report = parse_sheet(concat!(
        "@media speech { .speech { color: red; } } ",
        "@media (device-width: 1px) { .device { color: blue; } }",
    ));

    assert!(
        report.is_clean(),
        "MQ3 speech and device-width are valid authored syntax: {:?}",
        report.diagnostics()
    );

    let [CssRule::Media(speech), CssRule::Media(device)] = report.syntax().rules() else {
        panic!("expected both valid MQ3 media rules to be retained")
    };
    assert!(matches!(speech.query().queries(), [CssMediaQuery::Typed(_)]));
    assert!(matches!(
        device.query().queries(),
        [CssMediaQuery::Condition(_)]
    ));
}

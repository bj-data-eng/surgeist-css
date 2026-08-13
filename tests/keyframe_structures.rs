use surgeist_css::{CssKeyframeSelector, CssRule, parse_sheet};

#[test]
fn keyframes_preserve_empty_and_duplicate_authored_structure() {
    let source = concat!(
        "@keyframes fade { ",
        "from, 0%, from { } ",
        "from { opacity: 0; } ",
        "0% { opacity: 1; } ",
        "} ",
        "@keyframes empty {}",
    );
    let report = parse_sheet(source);

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let [CssRule::Keyframes(fade), CssRule::Keyframes(empty)] = report.syntax().rules() else {
        panic!("expected both authored keyframes rules");
    };
    assert_eq!(fade.blocks().len(), 3);
    assert!(fade.blocks()[0].declarations().is_empty());
    assert_eq!(
        fade.blocks()[0].selectors().selectors(),
        [
            CssKeyframeSelector::From,
            CssKeyframeSelector::Percent(
                surgeist_css::CssKeyframePercent::try_new(0.0).unwrap()
            ),
            CssKeyframeSelector::From,
        ]
    );
    assert!(matches!(
        fade.blocks()[1].selectors().selectors(),
        [CssKeyframeSelector::From]
    ));
    assert!(matches!(
        fade.blocks()[2].selectors().selectors(),
        [CssKeyframeSelector::Percent(percent)] if percent.value().value() == 0.0
    ));
    assert_eq!(fade.blocks()[1].declarations().len(), 1);
    assert_eq!(fade.blocks()[2].declarations().len(), 1);
    assert!(empty.blocks().is_empty());
}

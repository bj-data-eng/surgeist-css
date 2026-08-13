use surgeist_css::{CssRule, parse_sheet};

#[test]
fn import_conditions_and_prelude_phases_follow_cascade() {
    let conditional =
        parse_sheet("@import url(theme.css) layer(theme) supports(display: grid) screen;");
    assert!(conditional.is_clean(), "{:?}", conditional.diagnostics());
    assert!(matches!(conditional.syntax().rules(), [CssRule::Import(_)]));

    let initial_layer = parse_sheet("@layer reset; @import url(theme.css); .after { color: red; }");
    assert!(
        initial_layer.is_clean(),
        "{:?}",
        initial_layer.diagnostics()
    );
    assert!(matches!(
        initial_layer.syntax().rules(),
        [
            CssRule::LayerStatement(_),
            CssRule::Import(_),
            CssRule::Style(_)
        ]
    ));
}

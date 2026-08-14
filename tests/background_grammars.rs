use surgeist_css::{CssKnownProperty, parse_style_attribute};

#[test]
fn c13_background_layers_retain_typed_structure() {
    let report = parse_style_attribute(concat!(
        "background: url(hero.png) left 10px top 20px / 40px auto ",
        "no-repeat fixed padding-box content-box, ",
        "linear-gradient(red, blue) center / cover repeat-y local border-box #123456",
    ));

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 1);
    assert_eq!(
        report.syntax()[0].known().map(|known| known.property()),
        Some(CssKnownProperty::Background),
    );
}

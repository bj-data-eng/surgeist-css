use surgeist_css::{CssKnownProperty, parse_style_attribute};

fn assert_generic_position_accepted(value: &str) {
    let source = format!("mask-position: {value}");
    let report = parse_style_attribute(&source);
    assert!(report.is_clean(), "{source}: {:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 1, "{source}");
    assert_eq!(
        report.syntax()[0]
            .known()
            .expect("known mask-position declaration")
            .property(),
        CssKnownProperty::MaskPosition,
        "{source}",
    );
}

fn assert_generic_position_rejected(value: &str) {
    let source = format!("mask-position: {value}; color: red");
    let report = parse_style_attribute(&source);
    assert_eq!(report.syntax().len(), 1, "{source}");
    assert_eq!(report.diagnostics().len(), 1, "{source}");
    assert_eq!(
        report.syntax()[0]
            .known()
            .expect("retained color declaration")
            .property(),
        CssKnownProperty::Color,
        "{source}",
    );

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(&source)
            .expect_err("strict validation rejects a recovered generic position");
        assert_eq!(failure.diagnostics(), report.diagnostics(), "{source}");
    }
}

#[test]
fn generic_position_accepts_every_one_two_and_four_component_branch() {
    for value in [
        "left",
        "top",
        "25%",
        "left top",
        "top left",
        "center bottom",
        "bottom center",
        "center center",
        "left 25%",
        "center 25%",
        "25% top",
        "25% center",
        "25% 75%",
        "left 10px top 20%",
        "bottom 20% right 10px",
        "left calc(10px + 5%) bottom calc(20px + 10%)",
    ] {
        assert_generic_position_accepted(value);
    }
}

#[test]
fn generic_position_rejects_axis_order_partial_pairs_and_duplicate_axes() {
    for value in [
        "left right",
        "50% left",
        "left top 10px",
        "left 10px top",
        "center 10px top 20px",
        "left 10px right 20px",
        "top 10px",
        "bottom calc(1px * 2)",
    ] {
        assert_generic_position_rejected(value);
    }
}

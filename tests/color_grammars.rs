use surgeist_css::{
    CssAuthoredColorComponent, CssAuthoredColorSyntax, CssAuthoredHue, CssAuthoredSystemColor,
    CssKnownDeclaredValueRef, CssKnownProperty, CssKnownPropertyValueRef, CssOpacityValue,
    parse_style_attribute,
};

fn color_value(source: &str) -> surgeist_css::CssColorPropertyValue {
    let report = parse_style_attribute(source);
    assert!(report.is_clean(), "{source}: {:?}", report.diagnostics());
    let CssKnownPropertyValueRef::Color(value) = report.syntax()[0]
        .known()
        .expect("known color declaration")
        .property_value()
        .expect("ordinary color value")
    else {
        panic!("expected color wrapper");
    };
    value.clone()
}

#[test]
fn opacity_percentage_is_retained_with_its_valid_sibling() {
    let report = parse_style_attribute("opacity: 150%; color: red");

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 2);
    assert_eq!(
        report.syntax()[0].known().map(|known| known.property()),
        Some(CssKnownProperty::Opacity),
    );
    assert_eq!(
        report.syntax()[1].known().map(|known| known.property()),
        Some(CssKnownProperty::Color),
    );
}

#[test]
fn deprecated_system_color_is_retained_with_its_valid_sibling() {
    let report = parse_style_attribute("color: ActiveBorder; opacity: 0.5");

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 2);
    assert_eq!(
        report.syntax()[0].known().map(|known| known.property()),
        Some(CssKnownProperty::Color),
    );
    assert_eq!(
        report.syntax()[1].known().map(|known| known.property()),
        Some(CssKnownProperty::Opacity),
    );
}

#[test]
fn authored_keyword_and_hex_colors_preserve_their_current_branches() {
    let current = color_value("color: CurrentColor");
    assert!(current.current().is_current_color());
    assert!(matches!(
        current.i01_subset(),
        Some(surgeist_css::CssColor::CurrentColor)
    ));

    let transparent = color_value("color: transparent");
    assert!(transparent.current().is_transparent());
    assert!(transparent.i01_subset().is_some());

    for (source, digits) in [
        ("color: #0f8", "0f8"),
        ("color: #0f8c", "0f8c"),
        ("color: #00ff88", "00ff88"),
        ("color: #00ff88cc", "00ff88cc"),
    ] {
        let value = color_value(source);
        assert_eq!(value.current().hex_value().unwrap().digits(), digits);
        assert!(value.i01_subset().is_some());
    }

    let named = color_value("color: ReBeccAPurple");
    assert_eq!(named.current().named().unwrap().name(), "rebeccapurple");
    assert!(named.i01_subset().is_some());
}

#[test]
fn authored_system_colors_distinguish_current_and_deprecated_sets() {
    let current = color_value("color: CanvasText");
    assert_eq!(
        current.current().system(),
        Some(CssAuthoredSystemColor::CanvasText)
    );
    assert!(current.i01_subset().is_some());

    let deprecated = color_value("color: ThreeDLightShadow");
    assert_eq!(
        deprecated.current().system(),
        Some(CssAuthoredSystemColor::ThreeDLightShadow)
    );
    assert!(deprecated.i01_subset().is_none());
}

#[test]
fn authored_rgb_keeps_legacy_and_modern_component_domains() {
    let legacy = color_value("color: rgba(300, -20, 40, 150%)");
    let rgb = legacy.current().rgb_value().unwrap();
    assert_eq!(rgb.syntax(), CssAuthoredColorSyntax::Legacy);
    assert!(matches!(
        rgb.channels(),
        [
            CssAuthoredColorComponent::Number(red),
            CssAuthoredColorComponent::Number(green),
            CssAuthoredColorComponent::Number(blue),
        ] if red.value() == 300.0 && green.value() == -20.0 && blue.value() == 40.0
    ));
    assert!(matches!(
        rgb.alpha(),
        Some(CssAuthoredColorComponent::Percentage(value)) if value.value() == 150.0
    ));

    let modern = color_value(concat!(
        "color: rgb(calc(1 + 2) calc(10% + 20%) none / ",
        "calc(50% + 10%))",
    ));
    let rgb = modern.current().rgb_value().unwrap();
    assert_eq!(rgb.syntax(), CssAuthoredColorSyntax::Modern);
    assert!(matches!(
        rgb.channels(),
        [
            CssAuthoredColorComponent::NumberCalculation(_),
            CssAuthoredColorComponent::PercentageCalculation(_),
            CssAuthoredColorComponent::None,
        ]
    ));
    assert!(matches!(
        rgb.alpha(),
        Some(CssAuthoredColorComponent::PercentageCalculation(_))
    ));
    assert!(modern.i01_subset().is_none());
}

#[test]
fn authored_hsl_and_hwb_keep_hue_and_percentage_domains() {
    let hsl = color_value("color: hsl(calc(1turn - 90deg) 120% -20% / none)");
    let hsl = hsl.current().hsl_value().unwrap();
    assert_eq!(hsl.syntax(), CssAuthoredColorSyntax::Modern);
    assert!(matches!(hsl.hue(), CssAuthoredHue::AngleCalculation(_)));
    assert!(matches!(
        hsl.saturation(),
        CssAuthoredColorComponent::Percentage(value) if (value.value() - 120.0).abs() < 0.001
    ));
    assert!(matches!(hsl.alpha(), Some(CssAuthoredColorComponent::None)));

    let legacy = color_value("color: hsla(-30, 120%, -10%, 2)");
    assert_eq!(
        legacy.current().hsl_value().unwrap().syntax(),
        CssAuthoredColorSyntax::Legacy
    );

    let hwb = color_value("color: hwb(none calc(20% + 5%) 120% / -10%)");
    let hwb = hwb.current().hwb_value().unwrap();
    assert!(matches!(hwb.hue(), CssAuthoredHue::None));
    assert!(matches!(
        hwb.whiteness(),
        CssAuthoredColorComponent::PercentageCalculation(_)
    ));
    assert!(matches!(
        hwb.blackness(),
        CssAuthoredColorComponent::Percentage(value) if (value.value() - 120.0).abs() < 0.001
    ));
}

#[test]
fn invalid_color_separators_units_and_arities_drop_only_the_declaration() {
    for invalid in [
        "rgb(1, 20%, 3)",
        "rgb(1, 2 3)",
        "rgb(1 2)",
        "rgb(1px 2 3)",
        "hsl(20, 30%, 40% / 50%)",
        "hsl(20 30 40%)",
        "hwb(20, 30%, 40%)",
        "hwb(20deg 30% 40% 50%)",
    ] {
        let source = format!("color: {invalid}; opacity: 0.5");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "{invalid}");
        assert_eq!(report.diagnostics().len(), 1, "{invalid}");
        assert_eq!(
            report.syntax()[0].known().map(|known| known.property()),
            Some(CssKnownProperty::Opacity),
            "{invalid}",
        );
    }
}

#[test]
fn authored_color_calculations_preserve_the_exact_depth_boundary() {
    for depth in [254_usize, 255] {
        let source = format!(
            "color: rgb({}1{} 2 3); opacity: 0.5",
            "calc(".repeat(depth),
            ")".repeat(depth),
        );
        let report = parse_style_attribute(&source);
        assert!(
            report.is_clean(),
            "depth {depth}: {:?}",
            report.diagnostics()
        );
        assert_eq!(report.syntax().len(), 2);
    }

    for depth in [256_usize, 257] {
        let source = format!(
            "color: rgb({}1{} 2 3); opacity: 0.5",
            "calc(".repeat(depth),
            ")".repeat(depth),
        );
        let first_over_limit = source.match_indices("calc(").nth(255).unwrap().0;
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "depth {depth}");
        let [diagnostic] = report.diagnostics() else {
            panic!("depth {depth}: expected one diagnostic");
        };
        assert_eq!(
            diagnostic.error().code(),
            surgeist_css::CssErrorCode::NestingLimit,
            "depth {depth}",
        );
        assert_eq!(
            diagnostic.action(),
            surgeist_css::CssRecoveryAction::StopAtNestingLimit,
            "depth {depth}",
        );
        assert_eq!(
            diagnostic.error().position().byte_offset().value(),
            first_over_limit,
            "depth {depth}",
        );
    }
}

#[test]
fn opacity_preserves_finite_authored_number_and_percentage_branches() {
    let report = parse_style_attribute(concat!(
        "opacity: 0.5; ",
        "opacity: -0.5; ",
        "opacity: 1.5; ",
        "opacity: -25%; ",
        "opacity: 150%; ",
        "color: red",
    ));
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 6);

    let mut opacity = report.syntax()[..5].iter().map(|declaration| {
        let CssKnownPropertyValueRef::Opacity(value) = declaration
            .known()
            .expect("known opacity declaration")
            .property_value()
            .expect("ordinary opacity value")
        else {
            panic!("expected opacity wrapper");
        };
        value
    });

    let value = opacity.next().unwrap();
    assert!(matches!(value.value(), CssOpacityValue::Literal(value) if value.value() == 0.5));
    assert_eq!(value.i01_subset().map(|value| value.value()), Some(0.5));

    for expected in [-0.5, 1.5] {
        let value = opacity.next().unwrap();
        assert!(
            matches!(value.value(), CssOpacityValue::Number(value) if value.value() == expected)
        );
        assert!(value.i01_subset().is_none());
    }

    for expected in [-25.0, 150.0] {
        let value = opacity.next().unwrap();
        assert!(
            matches!(value.value(), CssOpacityValue::Percentage(value) if value.value() == expected)
        );
        assert!(value.i01_subset().is_none());
    }
}

#[test]
fn opacity_ordinary_global_and_substitution_values_remain_distinct() {
    let report =
        parse_style_attribute("opacity: 150%; opacity: inherit; opacity: var(--authored-opacity)");
    assert!(report.is_clean(), "{:?}", report.diagnostics());

    assert!(matches!(
        report.syntax()[0]
            .known()
            .expect("ordinary opacity")
            .declared_value(),
        CssKnownDeclaredValueRef::Property(CssKnownPropertyValueRef::Opacity(_))
    ));
    assert!(matches!(
        report.syntax()[1]
            .known()
            .expect("global opacity")
            .declared_value(),
        CssKnownDeclaredValueRef::Global(_)
    ));
    assert!(matches!(
        report.syntax()[2]
            .known()
            .expect("substitution-dependent opacity")
            .declared_value(),
        CssKnownDeclaredValueRef::SubstitutionDependent(_)
    ));
}

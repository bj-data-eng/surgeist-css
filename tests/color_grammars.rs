use surgeist_css::{
    CssKnownDeclaredValueRef, CssKnownProperty, CssKnownPropertyValueRef, CssOpacityValue,
    parse_style_attribute,
};

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

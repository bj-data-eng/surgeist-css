use surgeist_css::{
    CssFontFamilyNameKind, CssFontSize, CssFontStyle, CssFontValue, CssGenericFontFamily,
    CssKnownDeclaredValueRef, CssKnownProperty, CssKnownPropertyValueRef, CssLineHeight,
    CssNonNegativeNumberValue, CssSystemFont, parse_style_attribute,
};

#[test]
fn font_size_family_line_height_and_shorthand_follow_fonts3() {
    let report = parse_style_attribute(concat!(
        "font-size: medium; ",
        "line-height: 1.25; ",
        "font-family: \"inherit\", Avenir Next, serif; ",
        "font: condensed 700 small-caps italic medium/1.2 \"Avenir Next\", sans-serif; ",
        "font: menu",
    ));

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 5);

    let CssKnownPropertyValueRef::FontSize(size) = report.syntax()[0]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected font-size");
    };
    assert!(matches!(size.size(), CssFontSize::Medium));
    assert!(size.i01_subset().is_none());

    let CssKnownPropertyValueRef::LineHeight(line_height) = report.syntax()[1]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected line-height");
    };
    assert!(matches!(
        line_height.line_height(),
        CssLineHeight::Number(CssNonNegativeNumberValue::Literal(value))
            if value.value() == 1.25
    ));
    assert!(line_height.i01_subset().is_none());

    let CssKnownPropertyValueRef::FontFamily(family) = report.syntax()[2]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected font-family");
    };
    let families = family.families().families();
    assert_eq!(families[0].kind(), CssFontFamilyNameKind::Quoted);
    assert_eq!(families[0].as_str(), "inherit");
    assert_eq!(families[1].kind(), CssFontFamilyNameKind::IdentSequence);
    assert_eq!(families[1].as_str(), "Avenir Next");
    assert_eq!(families[2].kind(), CssFontFamilyNameKind::Generic);
    assert_eq!(
        families[2].generic_family(),
        Some(CssGenericFontFamily::Serif)
    );
    assert_eq!(
        family.i01_subset().unwrap().families()[2].kind(),
        CssFontFamilyNameKind::IdentSequence,
    );

    let CssKnownPropertyValueRef::Font(explicit) = report.syntax()[3]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected font shorthand");
    };
    let CssFontValue::Explicit(explicit) = explicit.font() else {
        panic!("expected explicit font shorthand");
    };
    assert!(matches!(explicit.size(), CssFontSize::Medium));
    assert!(matches!(
        explicit.line_height(),
        Some(CssLineHeight::Number(_))
    ));
    assert!(
        report.syntax()[3]
            .known()
            .unwrap()
            .property_value()
            .is_some()
    );
    assert!(matches!(
        report.syntax()[4]
            .known()
            .unwrap()
            .property_value()
            .unwrap(),
        CssKnownPropertyValueRef::Font(value)
            if matches!(value.font(), CssFontValue::System(CssSystemFont::Menu))
                && value.i01_subset().is_none()
    ));
}

#[test]
fn font_keywords_generics_globals_and_signed_boundaries_are_exact() {
    let source = concat!(
        "font-size: xx-small; font-size: x-small; font-size: small; ",
        "font-size: medium; font-size: large; font-size: x-large; ",
        "font-size: xx-large; font-size: larger; font-size: smaller; ",
        "font-size: +12px; line-height: +1.5; line-height: 120%; ",
        "font-family: serif, sans-serif, cursive, fantasy, monospace; ",
        "font-family: inherit; font-family: \"inherit\"",
    );
    let report = parse_style_attribute(source);
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 15);
    for (index, expected) in [
        CssFontSize::XxSmall,
        CssFontSize::XSmall,
        CssFontSize::Small,
        CssFontSize::Medium,
        CssFontSize::Large,
        CssFontSize::XLarge,
        CssFontSize::XxLarge,
        CssFontSize::Larger,
        CssFontSize::Smaller,
    ]
    .iter()
    .enumerate()
    {
        let CssKnownPropertyValueRef::FontSize(value) = report.syntax()[index]
            .known()
            .unwrap()
            .property_value()
            .unwrap()
        else {
            panic!("expected font-size keyword");
        };
        assert_eq!(value.size(), expected);
        assert!(value.i01_subset().is_none());
    }
    let CssKnownPropertyValueRef::FontFamily(generics) = report.syntax()[12]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected generic font-family list");
    };
    assert_eq!(
        generics
            .families()
            .families()
            .iter()
            .map(|family| family.generic_family().unwrap())
            .collect::<Vec<_>>(),
        vec![
            CssGenericFontFamily::Serif,
            CssGenericFontFamily::SansSerif,
            CssGenericFontFamily::Cursive,
            CssGenericFontFamily::Fantasy,
            CssGenericFontFamily::Monospace,
        ]
    );
    assert!(matches!(
        report.syntax()[13].known().unwrap().declared_value(),
        CssKnownDeclaredValueRef::Global(_)
    ));

    let globals = parse_style_attribute(concat!(
        "font-size: initial; line-height: unset; ",
        "font-family: revert; font: inherit",
    ));
    assert!(globals.is_clean(), "{:?}", globals.diagnostics());
    assert!(globals.syntax().iter().all(|declaration| matches!(
        declaration.known().unwrap().declared_value(),
        CssKnownDeclaredValueRef::Global(_)
    )));
    assert!(matches!(
        report.syntax()[14]
            .known()
            .unwrap()
            .property_value()
            .unwrap(),
        CssKnownPropertyValueRef::FontFamily(value)
            if value.families().families()[0].kind() == CssFontFamilyNameKind::Quoted
    ));

    for invalid in [
        "font-size: -1px",
        "font-size: -1%",
        "line-height: -0.1",
        "line-height: -1px",
        "font-family: inherit, serif",
        "font-family: My serif",
    ] {
        let source = format!("{invalid}; color: red");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        assert_eq!(report.diagnostics().len(), 1, "{source}");
        assert_eq!(
            report.syntax()[0].known().unwrap().property(),
            CssKnownProperty::Color,
            "{source}",
        );
    }
}

#[test]
fn system_fonts_are_whole_values_and_explicit_components_are_unique() {
    for (keyword, expected) in [
        ("caption", CssSystemFont::Caption),
        ("icon", CssSystemFont::Icon),
        ("menu", CssSystemFont::Menu),
        ("message-box", CssSystemFont::MessageBox),
        ("small-caption", CssSystemFont::SmallCaption),
        ("status-bar", CssSystemFont::StatusBar),
    ] {
        let report = parse_style_attribute(&format!("font: {keyword}"));
        assert!(report.is_clean(), "{keyword}: {:?}", report.diagnostics());
        let CssKnownPropertyValueRef::Font(value) = report.syntax()[0]
            .known()
            .unwrap()
            .property_value()
            .unwrap()
        else {
            panic!("expected font");
        };
        assert_eq!(value.font(), &CssFontValue::System(expected));
        assert!(value.i01_subset().is_none());
    }

    let legacy = parse_style_attribute("font: italic small-caps 700 condensed 16px/normal Arial");
    assert!(legacy.is_clean(), "{:?}", legacy.diagnostics());
    let CssKnownPropertyValueRef::Font(legacy) = legacy.syntax()[0]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected font");
    };
    assert!(legacy.i01_subset().is_some());

    for invalid in [
        "font: menu serif",
        "font: italic menu",
        "font: italic italic 16px serif",
        "font: small-caps small-caps 16px serif",
        "font: bold 700 16px serif",
        "font: condensed expanded 16px serif",
        "font: 16px/normal",
        "font: 16px serif/1.2",
    ] {
        let source = format!("{invalid}; color: red");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        assert_eq!(report.diagnostics().len(), 1, "{source}");
    }
}

#[test]
fn font_shorthand_retains_ambiguous_normal_before_explicit_style() {
    let report = parse_style_attribute(concat!(
        "font: normal italic 16px serif; ",
        "font: normal normal italic 16px serif; ",
        "font: normal normal normal italic 16px serif; ",
        "font: normal small-caps italic 16px serif; ",
        "font: normal 700 italic 16px serif; ",
        "font: normal condensed oblique 16px serif",
    ));

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 6);

    for (index, expected_style) in [
        CssFontStyle::Italic,
        CssFontStyle::Italic,
        CssFontStyle::Italic,
        CssFontStyle::Italic,
        CssFontStyle::Italic,
        CssFontStyle::Oblique,
    ]
    .iter()
    .enumerate()
    {
        let CssKnownPropertyValueRef::Font(value) = report.syntax()[index]
            .known()
            .expect("retained known font declaration")
            .property_value()
            .expect("retained current font value")
        else {
            panic!("expected font shorthand at declaration {index}");
        };
        let CssFontValue::Explicit(explicit) = value.font() else {
            panic!("expected explicit font shorthand at declaration {index}");
        };
        assert_eq!(explicit.style(), Some(*expected_style));
    }
}

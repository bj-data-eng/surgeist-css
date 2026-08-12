use surgeist_css::{
    CssAllDeclaredValue, CssDeclarationBody, CssDeclaredValue, CssGlobalKeyword,
    CssKnownDeclaration, CssKnownProperty, CssPropertyNameRef, CssRule, parse_sheet,
};

fn declarations(source: &str) -> Vec<surgeist_css::CssDeclaration> {
    let sheet = parse_sheet(source).expect("valid stylesheet");
    let [CssRule::Style(rule)] = sheet.rules() else {
        panic!("expected one style rule");
    };
    rule.declarations().as_slice().to_vec()
}

#[test]
fn coupled_adjacent_properties_expose_distinct_typed_values() {
    let declarations = declarations(".x { width: 12px; opacity: 0.5; }");
    let [width, opacity] = declarations.as_slice() else {
        panic!("expected adjacent declarations");
    };

    let Some(CssKnownDeclaration::Width(value)) = width.known() else {
        panic!("expected typed width declaration");
    };
    assert!(matches!(
        value.value().expect("typed width"),
        surgeist_css::CssLength::Px(length) if length.value() == 12.0
    ));

    let Some(CssKnownDeclaration::Opacity(value)) = opacity.known() else {
        panic!("expected typed opacity declaration");
    };
    assert_eq!(value.value().expect("typed opacity").value(), 0.5);
}

#[test]
fn coupled_declared_value_views_distinguish_typed_global_and_substitution() {
    let declarations =
        declarations(".x { width: 1px; height: inherit; min-width: var(--size, 2px); }");

    let CssKnownDeclaration::Width(typed) = declarations[0].known().unwrap() else {
        panic!("expected width");
    };
    assert!(typed.value().is_some());
    assert!(typed.global().is_none());
    assert!(typed.substitution_dependent().is_none());

    let CssKnownDeclaration::Height(global) = declarations[1].known().unwrap() else {
        panic!("expected height");
    };
    assert_eq!(global.global(), Some(CssGlobalKeyword::Inherit));
    assert!(global.value().is_none());

    let CssKnownDeclaration::MinWidth(substitution) = declarations[2].known().unwrap() else {
        panic!("expected min-width");
    };
    assert_eq!(
        substitution
            .substitution_dependent()
            .expect("substitution-dependent value")
            .as_css(),
        "var(--size, 2px)"
    );
}

#[test]
fn coupled_all_never_exposes_an_ordinary_typed_value() {
    let declarations = declarations(".x { all: initial; all: var(--mode); }");
    let [global, substitution] = declarations.as_slice() else {
        panic!("expected all declarations");
    };

    let Some(CssKnownDeclaration::All(CssAllDeclaredValue::Global(keyword))) = global.known()
    else {
        panic!("expected global all value");
    };
    assert_eq!(*keyword, CssGlobalKeyword::Initial);

    let Some(CssKnownDeclaration::All(CssAllDeclaredValue::SubstitutionDependent(value))) =
        substitution.known()
    else {
        panic!("expected substitution-dependent all value");
    };
    assert_eq!(value.as_css(), "var(--mode)");
}

#[test]
fn coupled_custom_body_preserves_current_authored_value() {
    let declarations = declarations(".x { --BrandColor: rgb(1, 2, 3); }");
    let declaration = &declarations[0];
    let CssDeclarationBody::Custom(custom) = declaration.body() else {
        panic!("expected custom body");
    };
    assert_eq!(custom.name().as_str(), "--BrandColor");
    assert_eq!(
        custom
            .value()
            .value()
            .expect("authored custom value")
            .as_css(),
        "rgb(1, 2, 3)"
    );
    assert!(declaration.known().is_none());
    assert!(declaration.custom().is_some());
}

#[test]
fn coupled_identity_is_derived_and_lookup_is_case_insensitive() {
    let declarations = declarations(".x { WiDtH: 1px; --WiDtH: 2px; }");
    assert_eq!(
        declarations[0].known().unwrap().property(),
        CssKnownProperty::Width
    );
    assert!(matches!(
        declarations[0].property_name(),
        CssPropertyNameRef::Known(CssKnownProperty::Width)
    ));
    match declarations[1].property_name() {
        CssPropertyNameRef::Custom(name) => assert_eq!(name.as_str(), "--WiDtH"),
        _ => panic!("expected case-sensitive custom name"),
    }
}

#[test]
fn coupled_declaration_position_is_property_name_start() {
    let declarations = declarations("\n  .x {\n    width: 1px;\n  }");
    let position = declarations[0].position();
    assert_eq!(position.byte_offset().value(), 12);
    assert_eq!(position.line().value(), 2);
    assert_eq!(position.column().value(), 4);
}

#[test]
fn coupled_declared_value_enum_is_wildcard_compatible() {
    let declarations = declarations(".x { width: initial; }");
    let CssKnownDeclaration::Width(value) = declarations[0].known().unwrap() else {
        panic!("expected width");
    };
    match value {
        CssDeclaredValue::Global(CssGlobalKeyword::Initial) => {}
        _ => panic!("expected global declared value"),
    }
}

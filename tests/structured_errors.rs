use surgeist_css::{
    CssDeclarationContextRef, CssErrorCode, CssProperty, CssTokenKind, ErrorKind, parse_sheet,
};

#[test]
fn error_unknown_and_recognized_unsupported_at_rules_have_distinct_codes() {
    let unknown = parse_sheet("@not-a-css-rule;").expect_err("unknown at-rule must fail");
    assert_eq!(unknown.code(), CssErrorCode::UnknownAtRule);
    match unknown.kind() {
        ErrorKind::UnknownAtRule(detail) => {
            assert_eq!(detail.name().as_str(), "not-a-css-rule");
        }
        _ => panic!("unexpected error root"),
    }

    let unsupported = parse_sheet("@supports (display: grid) {}")
        .expect_err("known unsupported at-rule must fail");
    assert_eq!(unsupported.code(), CssErrorCode::UnsupportedAtRule);
    match unsupported.kind() {
        ErrorKind::UnsupportedAtRule(detail) => {
            assert_eq!(detail.name().as_str(), "supports");
            assert_eq!(detail.feature().as_str(), "later.rule.supports");
        }
        _ => panic!("unexpected error root"),
    }
}

#[test]
fn error_at_rule_placement_prelude_and_body_expose_distinct_details() {
    let placement =
        parse_sheet(".x { width: 1px; } @import 'x.css';").expect_err("late import must fail");
    assert_eq!(placement.code(), CssErrorCode::InvalidAtRulePlacement);
    match placement.kind() {
        ErrorKind::InvalidAtRulePlacement(detail) => {
            assert_eq!(detail.name().as_str(), "import");
            assert_eq!(
                detail.expected_context().as_str(),
                "before every non-import top-level rule"
            );
        }
        _ => panic!("unexpected error root"),
    }

    let prelude =
        parse_sheet("@font-face nope {}").expect_err("non-empty font-face prelude must fail");
    assert_eq!(prelude.code(), CssErrorCode::InvalidAtRulePrelude);
    match prelude.kind() {
        ErrorKind::InvalidAtRulePrelude(detail) => {
            assert_eq!(detail.name().as_str(), "font-face");
            assert_eq!(detail.production().as_str(), "baseline.rule.font-face");
            assert_eq!(detail.expectation().as_str(), "an empty @font-face prelude");
            assert_eq!(detail.encountered().unwrap().authored(), "nope");
        }
        _ => panic!("unexpected error root"),
    }

    let body = parse_sheet("@media screen;").expect_err("media requires a block");
    assert_eq!(body.code(), CssErrorCode::InvalidAtRuleBody);
    match body.kind() {
        ErrorKind::InvalidAtRuleBody(detail) => {
            assert_eq!(detail.name().as_str(), "media");
            assert_eq!(detail.production().as_str(), "baseline.rule.media");
            assert!(detail.encountered().is_none());
        }
        _ => panic!("unexpected error root"),
    }
}

#[test]
fn error_selector_media_and_qualified_rule_failures_keep_production_context() {
    let selector = parse_sheet("??? { width: 1px; }").expect_err("selector must fail");
    match selector.kind() {
        ErrorKind::InvalidSelector(detail) => {
            assert_eq!(
                detail.production().unwrap().as_str(),
                "baseline.selector.complex"
            );
            assert_eq!(detail.expectation().as_str(), "a supported selector");
            assert_eq!(detail.encountered().unwrap().authored(), "?");
        }
        _ => panic!("unexpected error root"),
    }

    let media = parse_sheet("@media (unknown: yes) { .x { width: 1px; } }")
        .expect_err("unsupported media feature must fail");
    assert_eq!(media.code(), CssErrorCode::InvalidMediaQuery);
    match media.kind() {
        ErrorKind::InvalidMediaQuery(detail) => {
            assert_eq!(detail.feature().unwrap().as_str(), "unknown");
            assert_eq!(detail.expectation().as_str(), "a supported media query");
            assert_eq!(detail.encountered().unwrap().authored(), "unknown");
        }
        _ => panic!("unexpected error root"),
    }

    let qualified = parse_sheet("x;").expect_err("qualified rule without a block must fail");
    assert_eq!(qualified.code(), CssErrorCode::InvalidQualifiedRule);
    match qualified.kind() {
        ErrorKind::InvalidQualifiedRule(detail) => {
            assert_eq!(detail.production().as_str(), "css.qualified-rule");
            assert_eq!(detail.expectation().as_str(), "valid CSS syntax");
            assert!(detail.encountered().is_none());
        }
        _ => panic!("unexpected error root"),
    }
}

#[test]
fn error_property_descriptor_color_and_annotation_roots_expose_all_fields() {
    let property = parse_sheet(".x { WIDHT: 1px; }").expect_err("unknown property must fail");
    assert_eq!(property.code(), CssErrorCode::UnknownProperty);
    match property.kind() {
        ErrorKind::UnknownProperty(detail) => assert_eq!(detail.name().as_str(), "WIDHT"),
        _ => panic!("unexpected error root"),
    }

    let descriptor =
        parse_sheet("@font-face { mystery: x; font-family: Test; src: url(test.woff2); }")
            .expect_err("unknown descriptor must fail");
    assert_eq!(descriptor.code(), CssErrorCode::UnknownDescriptor);
    match descriptor.kind() {
        ErrorKind::UnknownDescriptor(detail) => {
            assert_eq!(detail.at_rule().as_str(), "font-face");
            assert_eq!(detail.descriptor().as_str(), "mystery");
        }
        _ => panic!("unexpected error root"),
    }

    let descriptor_value = parse_sheet(
        "@font-face { font-family: Test; src: url(test.woff2); font-display: mystery; }",
    )
    .expect_err("invalid descriptor value must fail");
    assert_eq!(
        descriptor_value.code(),
        CssErrorCode::InvalidDescriptorValue
    );
    match descriptor_value.kind() {
        ErrorKind::InvalidDescriptorValue(detail) => {
            assert_eq!(detail.at_rule().as_str(), "font-face");
            assert_eq!(detail.descriptor().as_str(), "font-display");
            assert_eq!(
                detail.expectation().as_str(),
                "a value accepted by the descriptor grammar"
            );
            assert_eq!(detail.encountered().unwrap().authored(), "mystery");
        }
        _ => panic!("unexpected error root"),
    }

    let combination =
        parse_sheet("@font-face { font-family: One; font-family: Two; src: url(test.woff2); }")
            .expect_err("duplicate descriptor must fail");
    assert_eq!(
        combination.code(),
        CssErrorCode::InvalidDescriptorCombination
    );
    match combination.kind() {
        ErrorKind::InvalidDescriptorCombination(detail) => {
            assert_eq!(detail.at_rule().as_str(), "font-face");
            assert_eq!(detail.responsible().as_str(), "font-family");
            assert_eq!(detail.conflicting().len(), 1);
            assert_eq!(detail.conflicting()[0].as_str(), "font-family");
        }
        _ => panic!("unexpected error root"),
    }

    let color = parse_sheet(".x { color: #ggg; }").expect_err("invalid color must fail");
    assert_eq!(color.code(), CssErrorCode::InvalidColorSyntax);
    match color.kind() {
        ErrorKind::InvalidColorSyntax(detail) => {
            assert!(detail.component().is_none());
            assert_eq!(detail.expectation().as_str(), "valid color syntax");
            assert_eq!(detail.encountered().unwrap().authored(), "#ggg");
        }
        _ => panic!("unexpected error root"),
    }

    let annotation = parse_sheet(".x { width: 1px !oops; }").expect_err("bad annotation must fail");
    assert_eq!(
        annotation.code(),
        CssErrorCode::InvalidDeclarationAnnotation
    );
    match annotation.kind() {
        ErrorKind::InvalidDeclarationAnnotation(detail) => {
            match detail.context() {
                CssDeclarationContextRef::KnownProperty(property) => {
                    assert_eq!(property, &CssProperty::Width);
                }
                _ => panic!("unexpected declaration context"),
            }
            assert_eq!(detail.encountered().authored(), "!");
        }
        _ => panic!("unexpected error root"),
    }
}

#[test]
fn error_invalid_property_value_retains_canonical_property_and_authored_token() {
    let error = parse_sheet(".panel { WIDTH: n\\6f pe; }")
        .expect_err("invalid known-property value must fail");

    assert_eq!(error.code(), CssErrorCode::InvalidPropertyValue);
    match error.kind() {
        ErrorKind::InvalidPropertyValue(detail) => {
            assert_eq!(detail.property(), &CssProperty::Width);
            assert_eq!(
                detail.expectation().as_str(),
                "a value accepted by the property's grammar"
            );
            let token = detail.encountered().expect("non-EOF token");
            assert_eq!(token.kind(), CssTokenKind::Ident);
            assert_eq!(token.authored(), "n\\6f pe");
        }
        _ => panic!("unexpected error root"),
    }
}

#[test]
fn error_property_value_at_bounded_end_uses_absent_encountered_token() {
    let error = parse_sheet(".panel { width:").expect_err("missing value must fail");

    assert_eq!(error.code(), CssErrorCode::InvalidPropertyValue);
    match error.kind() {
        ErrorKind::InvalidPropertyValue(detail) => {
            assert_eq!(detail.property(), &CssProperty::Width);
            assert!(detail.encountered().is_none());
        }
        _ => panic!("unexpected error root"),
    }
}

#[test]
fn error_malformed_declaration_exposes_unexpected_authored_token() {
    let error =
        parse_sheet(".panel { width 1px; }").expect_err("missing declaration colon must fail");

    assert_eq!(error.code(), CssErrorCode::UnexpectedToken);
    match error.kind() {
        ErrorKind::UnexpectedToken(detail) => {
            assert_eq!(detail.expectation().as_str(), "valid CSS syntax");
            assert_eq!(detail.encountered().kind(), CssTokenKind::Dimension);
            assert_eq!(detail.encountered().authored(), "1px");
        }
        _ => panic!("unexpected error root"),
    }
}

#[test]
fn error_position_uses_byte_line_and_utf16_coordinates_and_display_is_one_based() {
    let source = ".ok { width: 1px; }\n.emoji-😀 { width: nope; }";
    let error = parse_sheet(source).expect_err("invalid value must fail");
    let position = error.position();

    assert_eq!(position.byte_offset().value(), source.find("nope").unwrap());
    assert_eq!(position.line().value(), 1);
    assert_eq!(position.column().value(), 19);
    assert!(error.to_string().contains("2:20"));
}

#[test]
fn error_public_non_exhaustive_kinds_are_matched_with_wildcards() {
    let error = parse_sheet("??? { width: 1px; }").expect_err("invalid selector must fail");

    let expectation = match error.kind() {
        ErrorKind::InvalidSelector(detail) => detail.expectation().as_str(),
        _ => "different extensible root",
    };
    assert_eq!(expectation, "a supported selector");
}

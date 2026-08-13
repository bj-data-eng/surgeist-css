mod common;

use common::CssParseReportTestExt;
use surgeist_css::{
    CssDeclarationContextRef, CssErrorCode, CssKnownProperty, CssRecoveryAction, CssTokenKind,
    ErrorKind, parse_sheet, parse_style_attribute,
};

#[test]
fn transform_separator_and_domain_failures_report_exact_tokens_and_retain_siblings() {
    for (value, responsible, token_kind) in [
        ("matrix(1 0 0 1 10 20)", "0", CssTokenKind::Number),
        ("perspective(10%)", "10%", CssTokenKind::Percentage),
        ("translate3d(1px, 2px, 3%)", "3%", CssTokenKind::Percentage),
    ] {
        let source = format!("transform: {value}; color: red");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: expected one transform diagnostic");
        };
        assert_eq!(
            diagnostic.error().code(),
            CssErrorCode::InvalidPropertyValue,
            "{source}",
        );
        assert_eq!(
            diagnostic.action(),
            CssRecoveryAction::DropDeclaration,
            "{source}",
        );
        let responsible_offset = if value.starts_with("matrix") {
            source.find(" 0").expect("first missing-comma operand") + 1
        } else {
            source.rfind(responsible).expect("responsible token")
        };
        assert_eq!(
            diagnostic.error().position().byte_offset().value(),
            responsible_offset,
            "{source}",
        );
        assert_eq!(diagnostic.error().position().line().value(), 0, "{source}");
        assert_eq!(
            diagnostic.error().position().column().value() as usize,
            responsible_offset,
            "{source}",
        );
        assert_eq!(
            diagnostic.span().start().byte_offset().value(),
            0,
            "{source}"
        );
        assert_eq!(
            diagnostic.span().end().byte_offset().value(),
            source.find(';').expect("declaration semicolon") + 1,
            "{source}",
        );
        let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
            panic!("{source}: expected property-value root");
        };
        assert_eq!(detail.property(), CssKnownProperty::Transform, "{source}");
        let encountered = detail.encountered().expect("responsible transform token");
        assert_eq!(encountered.kind(), token_kind, "{source}");
        assert_eq!(encountered.authored(), responsible, "{source}");
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
                .expect_err("strict validation rejects recovered transform mutation");
            assert_eq!(failure.diagnostics(), report.diagnostics(), "{source}");
        }
    }
}

#[test]
fn repeated_transform_failures_make_progress_to_a_valid_sibling() {
    let source = concat!(
        "transform: matrix(1 0 0 1 0 0); ",
        "transform: perspective(-1px); ",
        "transform: translateZ(10%); color: red",
    );
    let report = parse_style_attribute(source);
    assert_eq!(report.diagnostics().len(), 3);
    assert_eq!(report.syntax().len(), 1);
    assert_eq!(
        report.syntax()[0]
            .known()
            .expect("retained color declaration")
            .property(),
        CssKnownProperty::Color,
    );
    assert!(
        report
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.action() == CssRecoveryAction::DropDeclaration)
    );

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects repeated recovered transforms");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

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
fn error_at_rule_placement_exposes_expected_context() {
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
}

#[test]
fn error_malformed_at_rule_prelude_reports_responsible_token_position() {
    let error =
        parse_sheet("@font-face nope {}").expect_err("non-empty font-face prelude must fail");
    assert_eq!(error.code(), CssErrorCode::InvalidAtRulePrelude);
    match error.kind() {
        ErrorKind::InvalidAtRulePrelude(detail) => {
            assert_eq!(detail.name().as_str(), "font-face");
            assert_eq!(detail.production().as_str(), "baseline.rule.font-face");
            assert_eq!(detail.expectation().as_str(), "an empty @font-face prelude");
            assert_eq!(detail.encountered().unwrap().authored(), "nope");
        }
        _ => panic!("unexpected error root"),
    }
    let position = error.position();
    assert_eq!(position.byte_offset().value(), 11);
    assert_eq!(position.line().value(), 0);
    assert_eq!(position.column().value(), 11);
}

#[test]
fn error_missing_at_rule_body_reports_missing_token_position() {
    let error = parse_sheet("@media screen;").expect_err("media requires a block");
    assert_eq!(error.code(), CssErrorCode::InvalidAtRuleBody);
    match error.kind() {
        ErrorKind::InvalidAtRuleBody(detail) => {
            assert_eq!(detail.name().as_str(), "media");
            assert_eq!(detail.production().as_str(), "baseline.rule.media");
            assert!(detail.encountered().is_none());
        }
        _ => panic!("unexpected error root"),
    }
    let position = error.position();
    assert_eq!(position.byte_offset().value(), 14);
    assert_eq!(position.line().value(), 0);
    assert_eq!(position.column().value(), 14);
}

#[test]
fn error_empty_font_face_body_reports_missing_descriptor_at_body_end() {
    let error = parse_sheet("@font-face {\n}")
        .expect_err("font-face requires font-family and src descriptors");
    assert_eq!(error.code(), CssErrorCode::InvalidAtRuleBody);
    match error.kind() {
        ErrorKind::InvalidAtRuleBody(detail) => {
            assert_eq!(detail.name().as_str(), "font-face");
            assert_eq!(detail.production().as_str(), "baseline.rule.font-face");
            assert_eq!(
                detail.expectation().as_str(),
                "font-family and src descriptors"
            );
            assert!(detail.encountered().is_none());
        }
        _ => panic!("unexpected error root"),
    }
    let position = error.position();
    assert_eq!(position.byte_offset().value(), 13);
    assert_eq!(position.line().value(), 1);
    assert_eq!(position.column().value(), 0);
}

#[test]
fn error_multi_name_layer_block_reports_offending_block_token() {
    let error = parse_sheet("@layer 😀, theme {}")
        .expect_err("a layer block accepts at most one layer name");
    assert_eq!(error.code(), CssErrorCode::InvalidAtRuleBody);
    match error.kind() {
        ErrorKind::InvalidAtRuleBody(detail) => {
            assert_eq!(detail.name().as_str(), "layer");
            assert_eq!(detail.production().as_str(), "baseline.rule.layer-block");
            assert_eq!(
                detail.expectation().as_str(),
                "at most one layer name before a block"
            );
            let token = detail.encountered().expect("offending block token");
            assert_eq!(token.kind(), CssTokenKind::CurlyBracketBlock);
            assert_eq!(token.authored(), "{");
        }
        _ => panic!("unexpected error root"),
    }
    let position = error.position();
    assert_eq!(position.byte_offset().value(), 19);
    assert_eq!(position.line().value(), 0);
    assert_eq!(position.column().value(), 17);
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
                    assert_eq!(property, CssKnownProperty::Width);
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
            assert_eq!(detail.property(), CssKnownProperty::Width);
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
            assert_eq!(detail.property(), CssKnownProperty::Width);
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

#[test]
fn typed_calculation_type_error_has_exact_non_bmp_coordinates_span_and_recovery() {
    let source = "--😀: 1; opacity: calc(1px + 2px); color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("the invalid typed calculation must recover exactly once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    assert_eq!(diagnostic.error().position().byte_offset().value(), 25);
    assert_eq!(diagnostic.error().position().line().value(), 0);
    assert_eq!(diagnostic.error().position().column().value(), 23);
    assert_eq!(diagnostic.span().start().byte_offset().value(), 11);
    assert_eq!(diagnostic.span().start().column().value(), 9);
    assert_eq!(diagnostic.span().end().byte_offset().value(), 36);
    assert_eq!(diagnostic.span().end().column().value(), 34);
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured property-value error");
    };
    assert_eq!(detail.property(), CssKnownProperty::Opacity);
    let encountered = detail.encountered().expect("responsible typed leaf");
    assert_eq!(encountered.kind(), CssTokenKind::Dimension);
    assert_eq!(encountered.authored(), "1px");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation must reject recovered typed calculation input");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn typed_calculation_operator_and_divisor_errors_retain_later_siblings() {
    for (source, property, authored, kind) in [
        (
            "width: calc(1px * 2px); color: red",
            CssKnownProperty::Width,
            "*",
            CssTokenKind::Delim,
        ),
        (
            "order: calc(1 / 0); color: red",
            CssKnownProperty::Order,
            "/",
            CssTokenKind::Delim,
        ),
        (
            "width: calc(1px / 1px); color: red",
            CssKnownProperty::Width,
            "/",
            CssTokenKind::Delim,
        ),
    ] {
        let report = parse_style_attribute(source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: expected one diagnostic");
        };
        assert_eq!(
            diagnostic.error().code(),
            CssErrorCode::InvalidPropertyValue
        );
        assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
        let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
            panic!("{source}: expected property-value detail");
        };
        assert_eq!(detail.property(), property, "{source}");
        let encountered = detail.encountered().expect("responsible operator");
        assert_eq!(encountered.authored(), authored, "{source}");
        assert_eq!(encountered.kind(), kind, "{source}");
    }
}

#[test]
fn timing_first_duration_failure_has_exact_payload_span_action_and_sibling_recovery() {
    let source = "transition: opacity -1s 2s; color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 1);
    let [diagnostic] = report.diagnostics() else {
        panic!("negative first shorthand time must recover once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    assert_eq!(diagnostic.error().position().byte_offset().value(), 20);
    assert_eq!(diagnostic.error().position().line().value(), 0);
    assert_eq!(diagnostic.error().position().column().value(), 20);
    assert_eq!(diagnostic.span().start().byte_offset().value(), 0);
    assert_eq!(diagnostic.span().start().column().value(), 0);
    assert_eq!(diagnostic.span().end().byte_offset().value(), 27);
    assert_eq!(diagnostic.span().end().column().value(), 27);
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured property-value error");
    };
    assert_eq!(detail.property(), CssKnownProperty::Transition);
    let encountered = detail.encountered().expect("responsible negative duration");
    assert_eq!(encountered.kind(), CssTokenKind::Dimension);
    assert_eq!(encountered.authored(), "-1s");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered transition input");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn generic_position_mutations_report_the_responsible_token_and_retain_the_sibling() {
    for (value, responsible, token_kind) in [
        ("left right", "right", CssTokenKind::Ident),
        ("50% left", "left", CssTokenKind::Ident),
        ("left top 10px", "10px", CssTokenKind::Dimension),
        ("left 10px top", "top", CssTokenKind::Ident),
        ("center 10px top 20px", "top", CssTokenKind::Ident),
        ("left 10px right 20px", "right", CssTokenKind::Ident),
        ("top 10px", "10px", CssTokenKind::Dimension),
    ] {
        let source = format!("mask-position: {value}; color: red");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: expected one generic-position diagnostic");
        };
        assert_eq!(
            diagnostic.error().code(),
            CssErrorCode::InvalidPropertyValue,
            "{source}",
        );
        assert_eq!(
            diagnostic.action(),
            CssRecoveryAction::DropDeclaration,
            "{source}",
        );
        assert_eq!(
            diagnostic.error().position().byte_offset().value(),
            source.find(responsible).expect("responsible token"),
            "{source}",
        );
        assert_eq!(diagnostic.error().position().line().value(), 0, "{source}");
        assert_eq!(
            diagnostic.error().position().column().value() as usize,
            source.find(responsible).expect("responsible column"),
            "{source}",
        );
        assert_eq!(
            diagnostic.span().start().byte_offset().value(),
            0,
            "{source}"
        );
        assert_eq!(
            diagnostic.span().end().byte_offset().value(),
            source.find(';').expect("declaration semicolon") + 1,
            "{source}",
        );
        let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
            panic!("{source}: expected property-value root");
        };
        assert_eq!(
            detail.property(),
            CssKnownProperty::MaskPosition,
            "{source}"
        );
        let encountered = detail.encountered().expect("responsible position token");
        assert_eq!(encountered.kind(), token_kind, "{source}");
        assert_eq!(encountered.authored(), responsible, "{source}");
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
                .expect_err("strict validation rejects the recovered mutation");
            assert_eq!(failure.diagnostics(), report.diagnostics(), "{source}");
        }
    }
}

#[test]
fn layered_position_mutations_report_exact_property_token_span_and_recovery() {
    for (property, known_property, value, responsible, token_kind) in [
        (
            "background-position",
            CssKnownProperty::BackgroundPosition,
            "left top 10px 20px",
            "20px",
            CssTokenKind::Dimension,
        ),
        (
            "background-position",
            CssKnownProperty::BackgroundPosition,
            "left, right left, top",
            "left",
            CssTokenKind::Ident,
        ),
        (
            "mask-position",
            CssKnownProperty::MaskPosition,
            "left 10px top",
            "top",
            CssTokenKind::Ident,
        ),
        (
            "mask-position",
            CssKnownProperty::MaskPosition,
            "center, top 10px",
            "10px",
            CssTokenKind::Dimension,
        ),
    ] {
        let source = format!("{property}: {value}; color: red");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: expected one diagnostic");
        };
        assert_eq!(
            diagnostic.error().code(),
            CssErrorCode::InvalidPropertyValue
        );
        assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
        let responsible_offset = if value.contains(", right left") {
            source.find("right left").expect("invalid layer") + "right ".len()
        } else {
            source.rfind(responsible).expect("responsible token")
        };
        assert_eq!(
            diagnostic.error().position().byte_offset().value(),
            responsible_offset,
            "{source}",
        );
        assert_eq!(diagnostic.error().position().line().value(), 0, "{source}");
        assert_eq!(
            diagnostic.error().position().column().value() as usize,
            responsible_offset,
            "{source}",
        );
        assert_eq!(
            diagnostic.span().start().byte_offset().value(),
            0,
            "{source}"
        );
        assert_eq!(
            diagnostic.span().end().byte_offset().value(),
            source.find(';').expect("declaration semicolon") + 1,
            "{source}",
        );
        let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
            panic!("{source}: expected property-value root");
        };
        assert_eq!(detail.property(), known_property, "{source}");
        let encountered = detail.encountered().expect("responsible position token");
        assert_eq!(encountered.kind(), token_kind, "{source}");
        assert_eq!(encountered.authored(), responsible, "{source}");

        #[cfg(feature = "app-strict")]
        {
            let failure = surgeist_css::validate_style_attribute(&source)
                .expect_err("strict validation rejects layered position mutation");
            assert_eq!(failure.diagnostics(), report.diagnostics(), "{source}");
        }
    }
}

#[test]
fn property_specific_origin_mutations_report_exact_payload_span_and_recovery() {
    for (property, known_property, value, responsible, token_kind) in [
        (
            "object-position",
            CssKnownProperty::ObjectPosition,
            "left top 10px",
            "10px",
            CssTokenKind::Dimension,
        ),
        (
            "transform-origin",
            CssKnownProperty::TransformOrigin,
            "left top 50%",
            "50%",
            CssTokenKind::Percentage,
        ),
        (
            "transform-origin",
            CssKnownProperty::TransformOrigin,
            "left top calc(10%)",
            "calc(",
            CssTokenKind::Function,
        ),
        (
            "transform-origin",
            CssKnownProperty::TransformOrigin,
            "left top calc(1px + 10%)",
            "calc(",
            CssTokenKind::Function,
        ),
        (
            "transform-origin",
            CssKnownProperty::TransformOrigin,
            "top 10px 20px",
            "20px",
            CssTokenKind::Dimension,
        ),
        (
            "transform-origin",
            CssKnownProperty::TransformOrigin,
            "top calc(10%)",
            "calc(",
            CssTokenKind::Function,
        ),
        (
            "transform-origin",
            CssKnownProperty::TransformOrigin,
            "top calc(1px + 10%)",
            "calc(",
            CssTokenKind::Function,
        ),
        (
            "transform-origin",
            CssKnownProperty::TransformOrigin,
            "left top 10px 20px",
            "20px",
            CssTokenKind::Dimension,
        ),
        (
            "transform-origin",
            CssKnownProperty::TransformOrigin,
            "left top bottom",
            "bottom",
            CssTokenKind::Ident,
        ),
    ] {
        let source = format!("{property}: {value}; color: red");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: expected one origin diagnostic");
        };
        assert_eq!(
            diagnostic.error().code(),
            CssErrorCode::InvalidPropertyValue
        );
        assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
        let responsible_offset = source.rfind(responsible).expect("responsible token");
        assert_eq!(
            diagnostic.error().position().byte_offset().value(),
            responsible_offset,
            "{source}",
        );
        assert_eq!(diagnostic.error().position().line().value(), 0, "{source}");
        assert_eq!(
            diagnostic.error().position().column().value() as usize,
            responsible_offset,
            "{source}",
        );
        assert_eq!(
            diagnostic.span().start().byte_offset().value(),
            0,
            "{source}"
        );
        assert_eq!(diagnostic.span().start().line().value(), 0, "{source}");
        assert_eq!(diagnostic.span().start().column().value(), 0, "{source}");
        let declaration_end = source.find(';').expect("declaration semicolon") + 1;
        assert_eq!(
            diagnostic.span().end().byte_offset().value(),
            declaration_end,
            "{source}",
        );
        assert_eq!(diagnostic.span().end().line().value(), 0, "{source}");
        assert_eq!(
            diagnostic.span().end().column().value() as usize,
            declaration_end,
            "{source}",
        );
        let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
            panic!("{source}: expected property-value root");
        };
        assert_eq!(detail.property(), known_property, "{source}");
        let encountered = detail.encountered().expect("responsible origin token");
        assert_eq!(encountered.kind(), token_kind, "{source}");
        assert_eq!(encountered.authored(), responsible, "{source}");
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
                .expect_err("strict validation rejects the recovered origin mutation");
            assert_eq!(failure.diagnostics(), report.diagnostics(), "{source}");
        }
    }
}

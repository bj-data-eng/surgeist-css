use std::hash::Hash;

mod common;

use common::CssParseReportTestExt;
use surgeist_css::{
    CssByteOffset, CssErrorCode, CssKnownProperty, CssLineIndex, CssRecoveryAction, CssRule,
    CssSourcePosition, CssSourceSpan, CssTokenKind, CssUtf16ColumnIndex, ErrorKind, parse_sheet,
    parse_style_attribute,
};

fn assert_copy_hash_ord<T: Copy + Eq + Ord + Hash>() {}

fn assert_position(position: CssSourcePosition, byte_offset: usize, line: u32, column: u32) {
    assert_eq!(position.byte_offset().value(), byte_offset);
    assert_eq!(position.line().value(), line);
    assert_eq!(position.column().value(), column);
}

#[test]
fn timing_type_error_after_non_bmp_text_has_exact_utf16_coordinates_and_span() {
    let source = "--😀: 1; transition-duration: calc(1px + 2px); color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("invalid time calculation must recover once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    assert_position(diagnostic.error().position(), 37, 0, 35);
    assert_position(diagnostic.span().start(), 11, 0, 9);
    assert_position(diagnostic.span().end(), 48, 0, 46);
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured timing property error");
    };
    assert_eq!(detail.property(), CssKnownProperty::TransitionDuration);
    let encountered = detail.encountered().expect("responsible length token");
    assert_eq!(encountered.kind(), CssTokenKind::Dimension);
    assert_eq!(encountered.authored(), "1px");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered timing type error");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn opacity_domain_error_after_non_bmp_text_has_exact_coordinates_and_span() {
    let source = "--😀: 1; opacity: 1px; color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("invalid opacity dimension must recover once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    assert_position(diagnostic.error().position(), 20, 0, 18);
    assert_position(diagnostic.span().start(), 11, 0, 9);
    assert_position(diagnostic.span().end(), 24, 0, 22);
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured opacity property error");
    };
    assert_eq!(detail.property(), CssKnownProperty::Opacity);
    let encountered = detail.encountered().expect("responsible dimension token");
    assert_eq!(encountered.kind(), CssTokenKind::Dimension);
    assert_eq!(encountered.authored(), "1px");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered opacity dimension");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn color_component_error_after_non_bmp_text_has_exact_coordinates_and_span() {
    let source = "--😀: 1; color: rgb(1px 2 3); opacity: 0.5";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("invalid RGB component must recover once");
    };
    assert_eq!(diagnostic.error().code(), CssErrorCode::InvalidColorSyntax);
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    let responsible = source.find("1px").expect("responsible RGB component");
    let declaration_start = source.find("color").expect("color declaration");
    let declaration_end = source.find(';').expect("custom property terminator")
        + 1
        + source[source.find(';').unwrap() + 1..]
            .find(';')
            .expect("color declaration terminator")
        + 1;
    assert_position(
        diagnostic.error().position(),
        responsible,
        0,
        u32::try_from(responsible - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().start(),
        declaration_start,
        0,
        u32::try_from(declaration_start - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().end(),
        declaration_end,
        0,
        u32::try_from(declaration_end - 2).expect("UTF-16 column"),
    );
    let ErrorKind::InvalidColorSyntax(detail) = diagnostic.error().kind() else {
        panic!("expected structured color error");
    };
    assert_eq!(
        detail.component().map(|value| value.as_str()),
        Some("component")
    );
    assert_eq!(
        detail.encountered().unwrap().kind(),
        CssTokenKind::Dimension
    );
    assert_eq!(detail.encountered().unwrap().authored(), "1px");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered color component error");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn legacy_rgb_mixed_domain_reports_the_later_component_and_retains_its_sibling() {
    let source = "--😀: 1; color: rgb(1, 20%, 3); opacity: 0.5";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    assert_eq!(
        report.syntax()[1]
            .known()
            .expect("retained opacity declaration")
            .property(),
        CssKnownProperty::Opacity,
    );
    let [diagnostic] = report.diagnostics() else {
        panic!("mixed legacy RGB component domains must recover once");
    };
    assert_eq!(diagnostic.error().code(), CssErrorCode::InvalidColorSyntax);
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    let responsible = source.find("20%").expect("responsible RGB component");
    assert_position(
        diagnostic.error().position(),
        responsible,
        0,
        u32::try_from(source[..responsible].encode_utf16().count()).expect("UTF-16 column"),
    );
    let ErrorKind::InvalidColorSyntax(detail) = diagnostic.error().kind() else {
        panic!("expected structured color error");
    };
    assert_eq!(
        detail.component().map(|value| value.as_str()),
        Some("component")
    );
    let encountered = detail.encountered().expect("responsible percentage token");
    assert_eq!(encountered.kind(), CssTokenKind::Percentage);
    assert_eq!(encountered.authored(), "20%");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects mixed legacy RGB component domains");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn easing_count_error_after_non_bmp_text_has_exact_utf16_coordinates_and_span() {
    let source = "--😀: 1; transition-timing-function: steps(1, jump-none); color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("invalid jump-none count must recover once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    let responsible = source.find("steps(1").expect("steps function") + "steps(".len();
    let declaration_start = source
        .find("transition-timing-function")
        .expect("timing declaration start");
    let declaration_end = declaration_start
        + source[declaration_start..]
            .find(';')
            .expect("timing declaration end")
        + 1;
    assert_position(
        diagnostic.error().position(),
        responsible,
        0,
        u32::try_from(responsible - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().start(),
        declaration_start,
        0,
        u32::try_from(declaration_start - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().end(),
        declaration_end,
        0,
        u32::try_from(declaration_end - 2).expect("UTF-16 column"),
    );
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured easing property error");
    };
    assert_eq!(
        detail.property(),
        CssKnownProperty::TransitionTimingFunction
    );
    let encountered = detail.encountered().expect("responsible step count");
    assert_eq!(encountered.kind(), CssTokenKind::Number);
    assert_eq!(encountered.authored(), "1");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered easing count error");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn generic_position_error_after_non_bmp_text_has_exact_utf16_coordinates_and_span() {
    let source = "--😀: 1; mask-position: 50% left; color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("invalid generic position must recover once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    assert_position(diagnostic.error().position(), 30, 0, 28);
    assert_position(diagnostic.span().start(), 11, 0, 9);
    assert_position(diagnostic.span().end(), 35, 0, 33);
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured position property error");
    };
    assert_eq!(detail.property(), CssKnownProperty::MaskPosition);
    let encountered = detail
        .encountered()
        .expect("responsible horizontal keyword");
    assert_eq!(encountered.kind(), CssTokenKind::Ident);
    assert_eq!(encountered.authored(), "left");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered generic position input");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn object_position_error_after_non_bmp_text_has_exact_utf16_coordinates_and_span() {
    let source = "--😀: 1; object-position: left top 10px; color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("invalid object position must recover once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    let responsible = source.find("10px").expect("responsible third component");
    let declaration_start = source.find("object-position").expect("property start");
    let declaration_end = declaration_start
        + source[declaration_start..]
            .find(';')
            .expect("declaration end")
        + 1;
    assert_position(
        diagnostic.error().position(),
        responsible,
        0,
        u32::try_from(responsible - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().start(),
        declaration_start,
        0,
        u32::try_from(declaration_start - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().end(),
        declaration_end,
        0,
        u32::try_from(declaration_end - 2).expect("UTF-16 column"),
    );
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured object-position property error");
    };
    assert_eq!(detail.property(), CssKnownProperty::ObjectPosition);
    let encountered = detail.encountered().expect("responsible third component");
    assert_eq!(encountered.kind(), CssTokenKind::Dimension);
    assert_eq!(encountered.authored(), "10px");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered object-position input");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn transform_domain_error_after_non_bmp_text_has_exact_utf16_coordinates_and_span() {
    let source = "--😀: 1; transform: translate3d(1px, 2px, 3%); color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("invalid transform z percentage must recover once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    let responsible = source.find("3%").expect("responsible z percentage");
    let declaration_start = source.find("transform").expect("property start");
    let declaration_end = declaration_start
        + source[declaration_start..]
            .find(';')
            .expect("declaration end")
        + 1;
    assert_position(
        diagnostic.error().position(),
        responsible,
        0,
        u32::try_from(responsible - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().start(),
        declaration_start,
        0,
        u32::try_from(declaration_start - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().end(),
        declaration_end,
        0,
        u32::try_from(declaration_end - 2).expect("UTF-16 column"),
    );
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured transform property error");
    };
    assert_eq!(detail.property(), CssKnownProperty::Transform);
    let encountered = detail.encountered().expect("responsible z percentage");
    assert_eq!(encountered.kind(), CssTokenKind::Percentage);
    assert_eq!(encountered.authored(), "3%");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered transform input");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn filter_domain_error_after_non_bmp_text_has_exact_utf16_coordinates_and_span() {
    let source = "--😀: 1; filter: brightness(-1); color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("invalid filter amount must recover once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    let responsible = source.find("-1").expect("responsible filter amount");
    let declaration_start = source.find("filter").expect("property start");
    let declaration_end = declaration_start
        + source[declaration_start..]
            .find(';')
            .expect("declaration end")
        + 1;
    assert_position(
        diagnostic.error().position(),
        responsible,
        0,
        u32::try_from(responsible - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().start(),
        declaration_start,
        0,
        u32::try_from(declaration_start - 2).expect("UTF-16 column"),
    );
    assert_position(
        diagnostic.span().end(),
        declaration_end,
        0,
        u32::try_from(declaration_end - 2).expect("UTF-16 column"),
    );
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured filter property error");
    };
    assert_eq!(detail.property(), CssKnownProperty::Filter);
    let encountered = detail.encountered().expect("responsible filter number");
    assert_eq!(encountered.kind(), CssTokenKind::Number);
    assert_eq!(encountered.authored(), "-1");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered filter input");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn basic_shape_error_after_non_bmp_text_has_exact_utf16_coordinates_and_span() {
    let source = "--😀: 1; clip-path: polygon(round -1px, 0 0); color: red";
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 2);
    let [diagnostic] = report.diagnostics() else {
        panic!("invalid polygon rounding must recover once");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    let responsible = source.find("-1px").unwrap();
    let declaration_start = source.find("clip-path").unwrap();
    let declaration_end = declaration_start + source[declaration_start..].find(';').unwrap() + 1;
    assert_position(
        diagnostic.error().position(),
        responsible,
        0,
        u32::try_from(responsible - 2).unwrap(),
    );
    assert_position(
        diagnostic.span().start(),
        declaration_start,
        0,
        u32::try_from(declaration_start - 2).unwrap(),
    );
    assert_position(
        diagnostic.span().end(),
        declaration_end,
        0,
        u32::try_from(declaration_end - 2).unwrap(),
    );
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured clip-path property error");
    };
    assert_eq!(detail.property(), CssKnownProperty::ClipPath);
    let encountered = detail.encountered().expect("responsible polygon radius");
    assert_eq!(encountered.kind(), CssTokenKind::Dimension);
    assert_eq!(encountered.authored(), "-1px");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered basic shape");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

fn first_declaration_position(source: &str) -> CssSourcePosition {
    let sheet = parse_sheet(source).expect("valid stylesheet");
    let CssRule::Style(rule) = &sheet.rules()[0] else {
        panic!("expected style rule");
    };
    rule.declarations().as_slice()[0].position()
}

#[test]
fn source_types_are_copyable_comparable_and_hashable() {
    assert_copy_hash_ord::<CssByteOffset>();
    assert_copy_hash_ord::<CssLineIndex>();
    assert_copy_hash_ord::<CssUtf16ColumnIndex>();
    assert_copy_hash_ord::<CssSourcePosition>();
    assert_copy_hash_ord::<CssSourceSpan>();
}

#[test]
fn source_public_nodes_expose_zero_based_byte_line_and_utf16_coordinates() {
    assert_position(first_declaration_position(".a { width: 1px; }"), 5, 0, 5);
    assert_position(first_declaration_position(".a {\n  width: 1px; }"), 7, 1, 2);
    assert_position(
        first_declaration_position(".a {\r\n  width: 1px; }"),
        8,
        1,
        2,
    );
    assert_position(
        first_declaration_position(".\\61 bc { width: 1px; }"),
        10,
        0,
        10,
    );
    assert_position(first_declaration_position(".😀2 { width: 1px; }"), 9, 0, 7);

    let sheet = parse_sheet("/*a\nbc*/@import \"theme.css\";").expect("valid import");
    let CssRule::Import(rule) = &sheet.rules()[0] else {
        panic!("expected import rule");
    };
    assert_position(rule.position(), 8, 1, 4);
}

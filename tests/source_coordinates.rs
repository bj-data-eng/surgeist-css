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

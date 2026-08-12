use std::hash::Hash;

use surgeist_css::{
    CssByteOffset, CssLineIndex, CssRule, CssSourcePosition, CssSourceSpan, CssUtf16ColumnIndex,
    parse_sheet,
};

fn assert_copy_hash_ord<T: Copy + Eq + Ord + Hash>() {}

fn assert_position(position: CssSourcePosition, byte_offset: usize, line: u32, column: u32) {
    assert_eq!(position.byte_offset().value(), byte_offset);
    assert_eq!(position.line().value(), line);
    assert_eq!(position.column().value(), column);
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

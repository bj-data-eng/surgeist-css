use std::fmt;

/// A UTF-8 byte offset into the original authored CSS source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CssByteOffset(usize);

impl CssByteOffset {
    const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Returns the zero-based byte offset.
    #[must_use]
    pub const fn value(self) -> usize {
        self.0
    }
}

/// A zero-based line index in authored CSS source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CssLineIndex(u32);

impl CssLineIndex {
    const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the zero-based line index.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// A zero-based authored-source column measured in UTF-16 code units.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CssUtf16ColumnIndex(u32);

impl CssUtf16ColumnIndex {
    const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the zero-based UTF-16 column index.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// A position in the original authored CSS source.
///
/// Byte offsets refer to the original UTF-8 input. Lines and UTF-16 columns are
/// zero based. Positions are parser-produced metadata and cannot be forged by a
/// public constructor.
///
/// ```compile_fail
/// use surgeist_css::{CssByteOffset, CssLineIndex, CssSourcePosition, CssUtf16ColumnIndex};
///
/// let _ = CssSourcePosition {
///     byte_offset: CssByteOffset(0),
///     line: CssLineIndex(0),
///     column: CssUtf16ColumnIndex(0),
/// };
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CssSourcePosition {
    byte_offset: CssByteOffset,
    line: CssLineIndex,
    column: CssUtf16ColumnIndex,
}

impl CssSourcePosition {
    pub(crate) fn from_cssparser(
        position: cssparser::SourcePosition,
        location: cssparser::SourceLocation,
    ) -> Self {
        Self {
            byte_offset: CssByteOffset::new(position.byte_index()),
            line: CssLineIndex::new(location.line),
            column: CssUtf16ColumnIndex::new(location.column.saturating_sub(1)),
        }
    }

    /// Returns the zero-based UTF-8 byte offset.
    #[must_use]
    pub const fn byte_offset(self) -> CssByteOffset {
        self.byte_offset
    }

    /// Returns the zero-based line index.
    #[must_use]
    pub const fn line(self) -> CssLineIndex {
        self.line
    }

    /// Returns the zero-based UTF-16 column index.
    #[must_use]
    pub const fn column(self) -> CssUtf16ColumnIndex {
        self.column
    }
}

impl fmt::Display for CssSourcePosition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.line.value().saturating_add(1),
            self.column.value().saturating_add(1)
        )
    }
}

/// An inclusive-start, exclusive-end span in authored CSS source.
///
/// Spans are parser-produced metadata. Crate-owned construction rejects an end
/// that precedes the start byte offset; zero-width spans remain available for
/// missing-token and implicit-EOF diagnostics.
///
/// ```compile_fail
/// use surgeist_css::{CssSourcePosition, CssSourceSpan};
///
/// fn forge(start: CssSourcePosition, end: CssSourcePosition) -> CssSourceSpan {
///     CssSourceSpan { start, end }
/// }
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CssSourceSpan {
    start: CssSourcePosition,
    end: CssSourcePosition,
}

impl CssSourceSpan {
    #[must_use]
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "span construction is consumed by recovery diagnostics in cycle task T3"
        )
    )]
    pub(crate) const fn new(start: CssSourcePosition, end: CssSourcePosition) -> Option<Self> {
        if start.byte_offset.value() <= end.byte_offset.value() {
            Some(Self { start, end })
        } else {
            None
        }
    }

    /// Returns the inclusive start position.
    #[must_use]
    pub const fn start(self) -> CssSourcePosition {
        self.start
    }

    /// Returns the exclusive end position.
    #[must_use]
    pub const fn end(self) -> CssSourcePosition {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use cssparser::{Parser, ParserInput};

    use super::*;

    fn position_after_tokenizing(source: &str) -> CssSourcePosition {
        let mut input = ParserInput::new(source);
        let mut parser = Parser::new(&mut input);
        while parser.next().is_ok() {}
        CssSourcePosition::from_cssparser(parser.position(), parser.current_source_location())
    }

    fn assert_position(position: CssSourcePosition, byte_offset: usize, line: u32, column: u32) {
        assert_eq!(position.byte_offset().value(), byte_offset);
        assert_eq!(position.line().value(), line);
        assert_eq!(position.column().value(), column);
    }

    #[test]
    fn source_empty_input_starts_at_zero() {
        assert_position(position_after_tokenizing(""), 0, 0, 0);
    }

    #[test]
    fn source_first_and_later_columns_are_zero_based() {
        assert_position(position_after_tokenizing("a"), 1, 0, 1);
        assert_position(position_after_tokenizing("abc"), 3, 0, 3);
    }

    #[test]
    fn source_lf_and_crlf_advance_one_zero_based_line() {
        assert_position(position_after_tokenizing("a\nb"), 3, 1, 1);
        assert_position(position_after_tokenizing("a\r\nb"), 4, 1, 1);
    }

    #[test]
    fn source_multiline_comment_preserves_authored_coordinates() {
        assert_position(position_after_tokenizing("/*a\nbc*/"), 8, 1, 4);
    }

    #[test]
    fn source_escape_counts_authored_bytes_instead_of_decoded_spelling() {
        assert_position(position_after_tokenizing("\\61 bc"), 6, 0, 6);
    }

    #[test]
    fn source_supplementary_scalar_has_distinct_utf8_and_utf16_widths() {
        assert_position(position_after_tokenizing("😀2x"), 6, 0, 4);
    }

    #[test]
    fn source_span_rejects_reverse_order_and_allows_zero_width() {
        let start = position_after_tokenizing("a");
        let end = position_after_tokenizing("ab");

        let span = CssSourceSpan::new(start, end).expect("ordered span");
        assert_eq!(span.start(), start);
        assert_eq!(span.end(), end);
        let empty = CssSourceSpan::new(start, start).expect("zero-width span");
        assert_eq!(empty.start(), start);
        assert_eq!(empty.end(), start);
        assert_eq!(CssSourceSpan::new(end, start), None);
    }

    #[test]
    fn source_dependency_zero_column_saturates_to_zero() {
        let mut input = ParserInput::new("");
        let parser = Parser::new(&mut input);
        let position = CssSourcePosition::from_cssparser(
            parser.position(),
            cssparser::SourceLocation { line: 0, column: 0 },
        );

        assert_position(position, 0, 0, 0);
    }
}

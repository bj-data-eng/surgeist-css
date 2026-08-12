#![forbid(unsafe_code)]
#![cfg_attr(
    not(feature = "app-strict"),
    doc = r#"
The application-strict validators are intentionally absent unless the
`app-strict` feature is enabled.

```compile_fail
use surgeist_css::validate_sheet;

let _ = validate_sheet(".x { color: red; }");
```

```compile_fail
use surgeist_css::validate_style_attribute;

let _ = validate_style_attribute("color: red");
```
"#
)]
//! Recovering CSS ingestion for Surgeist style sheets.
//!
//! This crate parses CSS syntax into CSS-owned authored syntax values. It is
//! strict about retained values: unsupported or malformed top-level rules are
//! discarded with structured recovery diagnostics rather than represented as
//! invalid syntax.
//!
//! Parse failures expose typed [`ErrorKind`] values, stable [`CssErrorCode`]
//! roots, and semantic source positions so callers do not parse display text.
//! [`parse_sheet`] and [`parse_style_attribute`] return retained authored syntax
//! and diagnostic provenance in one report. They do not run cascade,
//! substitution, selector matching, contextual resolution, or resource loading.

mod error;
mod parser;
mod properties;
mod report;
mod source;
mod syntax;
#[cfg(test)]
mod test_support;
mod validation;

pub use error::*;
pub use parser::{parse_sheet, parse_style_attribute};
pub use properties::{CssKnownDeclaration, CssKnownProperty, CssOverflowPropertyValue};
pub use report::*;
pub use source::*;
pub use syntax::*;
#[cfg(test)]
pub(crate) use test_support::{CssParseReportTestExt, CssProperty, CssValue};

/// Validates a stylesheet by accepting only a clean ordinary parse report.
///
/// This application-strict wrapper invokes [`parse_sheet`] once. A clean report
/// yields its retained authored syntax; a recovered report yields every
/// parser-produced diagnostic in unchanged order. Validation does not select a
/// different grammar, reparse input, or perform cascade, substitution,
/// contextual resolution, selector matching, or resource loading.
///
/// ```
/// use surgeist_css::validate_sheet;
///
/// let sheet = validate_sheet(".x { color: red; }").expect("clean stylesheet");
/// assert_eq!(sheet.rules().len(), 1);
/// ```
#[cfg(feature = "app-strict")]
pub fn validate_sheet(input: &str) -> Result<CssSheet, CssValidationFailure> {
    parser::parse_sheet(input).into_validation_result()
}

/// Validates a style attribute by accepting only a clean ordinary parse report.
///
/// This application-strict wrapper invokes [`parse_style_attribute`] once. A
/// clean report yields its retained authored declarations; a recovered report
/// yields the complete parser-produced diagnostic sequence unchanged. It does
/// not select a different declaration grammar, reparse input, or apply cascade,
/// substitution, contextual resolution, selector matching, or resource loading.
///
/// ```
/// use surgeist_css::validate_style_attribute;
///
/// let declarations = validate_style_attribute("color: red")
///     .expect("clean style attribute");
/// assert_eq!(declarations.len(), 1);
/// ```
#[cfg(feature = "app-strict")]
pub fn validate_style_attribute(input: &str) -> Result<CssDeclarationList, CssValidationFailure> {
    parser::parse_style_attribute(input).into_validation_result()
}

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "app-strict"))]
mod app_strict_tests {
    use super::{validate_sheet, validate_style_attribute};
    use crate::parser::{
        finish_ordinary_parser_invocation_count, reset_ordinary_parser_invocation_count,
    };

    #[test]
    fn app_strict_one_pass_clean_and_recovered_sheet_and_style_validation() {
        for source in [".x { color: red; }", ".x { mystery: 1; }"] {
            reset_ordinary_parser_invocation_count();
            let _ = validate_sheet(source);
            assert_eq!(finish_ordinary_parser_invocation_count(), 1, "{source}");
        }

        for source in ["color: red", "mystery: 1"] {
            reset_ordinary_parser_invocation_count();
            let _ = validate_style_attribute(source);
            assert_eq!(finish_ordinary_parser_invocation_count(), 1, "{source}");
        }
    }
}

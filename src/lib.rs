#![forbid(unsafe_code)]
//! Recovering CSS ingestion for Surgeist style sheets.
//!
//! This crate parses CSS syntax into CSS-owned authored syntax values. It is
//! strict about retained values: unsupported or malformed top-level rules are
//! discarded with structured recovery diagnostics rather than represented as
//! invalid syntax.
//!
//! Parse failures expose typed [`ErrorKind`] values, stable [`CssErrorCode`]
//! roots, and semantic source positions so callers do not parse display text.
//! [`parse_sheet`] returns retained authored syntax and diagnostic provenance in
//! one report. It does not run cascade, substitution, selector matching,
//! contextual resolution, or resource loading.

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
pub use parser::parse_sheet;
pub use properties::{CssKnownDeclaration, CssKnownProperty, CssOverflowPropertyValue};
pub use report::*;
pub use source::*;
pub use syntax::*;
#[cfg(test)]
pub(crate) use test_support::{CssParseReportTestExt, CssProperty, CssValue};

#[cfg(test)]
mod tests;

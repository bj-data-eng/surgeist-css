#![forbid(unsafe_code)]
//! Strict CSS ingestion for Surgeist style sheets.
//!
//! This crate parses CSS syntax into CSS-owned authored syntax values. It is
//! strict by design: unsupported selectors, at-rules, properties, and values are
//! errors instead of browser-style recoverable invalid declarations.
//!
//! Parse failures expose typed [`ErrorKind`] values, stable [`CssErrorCode`]
//! roots, and semantic source positions so callers do not parse display text.
//! Recovery report and diagnostic value types describe the future browser-style
//! recovery boundary without changing the current strict [`parse_sheet`] entry
//! point. They retain authored syntax and diagnostic provenance; they do not run
//! cascade, substitution, selector matching, contextual resolution, or resource
//! loading.

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
pub use properties::{CssKnownProperty, CssProperty};
pub use report::*;
pub use source::*;
pub use syntax::*;

#[cfg(test)]
mod tests;

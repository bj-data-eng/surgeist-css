//! Strict CSS ingestion for Surgeist style sheets.
//!
//! This crate parses CSS syntax into CSS-owned authored syntax values. It is
//! strict by design: unsupported selectors, at-rules, properties, and values are
//! errors instead of browser-style recoverable invalid declarations.
//!
//! Parse failures expose typed [`ErrorKind`] values, stable [`CssErrorCode`]
//! roots, and semantic source positions so callers do not parse display text.

mod error;
mod parser;
mod source;
mod syntax;
#[cfg(test)]
mod test_support;
mod validation;

pub use error::*;
pub use parser::parse_sheet;
pub use source::*;
pub use syntax::*;

#[cfg(test)]
mod tests;

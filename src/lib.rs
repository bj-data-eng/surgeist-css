//! Strict CSS ingestion for Surgeist style sheets.
//!
//! This crate parses CSS syntax into CSS-owned authored syntax values. It is
//! strict by design: unsupported selectors, at-rules, properties, and values are
//! errors instead of browser-style recoverable invalid declarations.
//!
//! Parse failures expose typed [`ErrorKind`] values plus source line and column
//! information so callers do not need to parse display strings.

mod error;
mod parser;
mod syntax;
#[cfg(test)]
mod test_support;
mod validation;

pub use error::{Error, ErrorKind, Result};
pub use parser::parse_sheet;
pub use syntax::*;

#[cfg(test)]
mod tests;

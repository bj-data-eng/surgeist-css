use std::fmt;

use cssparser::{BasicParseError, BasicParseErrorKind, ParseError, ParseErrorKind, Parser, ToCss};

use crate::validation::{PropertyNameStatus, classify_property_name};

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    InvalidSyntax {
        reason: String,
    },
    InvalidSelector {
        reason: String,
    },
    UnsupportedAtRule {
        name: String,
    },
    UnknownProperty {
        name: String,
    },
    UnsupportedProperty {
        name: String,
    },
    UnsupportedValue {
        property: Option<String>,
        reason: String,
    },
    InvalidColor {
        value: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    line: u32,
    column: u32,
}

impl Error {
    fn at(
        location: cssparser::SourceLocation,
        kind: ErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            line: location.line,
            column: location.column,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    #[must_use]
    pub const fn column(&self) -> u32 {
        self.column
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CSS parse error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for Error {}

pub(crate) fn from_parse_error(error: ParseError<'_, Error>) -> Error {
    match error.kind {
        ParseErrorKind::Custom(error) => error,
        ParseErrorKind::Basic(kind) => basic_error(error.location, kind),
    }
}

fn basic_error(location: cssparser::SourceLocation, kind: BasicParseErrorKind<'_>) -> Error {
    match kind {
        BasicParseErrorKind::EndOfInput => Error::at(
            location,
            ErrorKind::InvalidSyntax {
                reason: "unexpected end of CSS input".to_owned(),
            },
            "unexpected end of CSS input",
        ),
        BasicParseErrorKind::AtRuleInvalid(name) => {
            let name = name.to_string();
            Error::at(
                location,
                ErrorKind::UnsupportedAtRule { name: name.clone() },
                format!("unsupported CSS at-rule `@{name}`"),
            )
        }
        BasicParseErrorKind::QualifiedRuleInvalid => Error::at(
            location,
            ErrorKind::InvalidSyntax {
                reason: "invalid CSS rule".to_owned(),
            },
            "invalid CSS rule",
        ),
        BasicParseErrorKind::AtRuleBodyInvalid => Error::at(
            location,
            ErrorKind::InvalidSyntax {
                reason: "invalid CSS at-rule body".to_owned(),
            },
            "invalid CSS at-rule body",
        ),
        BasicParseErrorKind::UnexpectedToken(token) => {
            let reason = format!("unexpected CSS token `{}`", token.to_css_string());
            Error::at(
                location,
                ErrorKind::InvalidSyntax {
                    reason: reason.clone(),
                },
                reason,
            )
        }
    }
}

pub(crate) fn basic<'i>(error: BasicParseError<'i>) -> ParseError<'i, Error> {
    error.into()
}

pub(crate) fn selector_basic<'i>(error: BasicParseError<'i>) -> ParseError<'i, Error> {
    let location = error.location;
    let reason = basic_error(location, error.kind).message().to_owned();
    invalid_selector_at(location, reason)
}

pub(crate) fn invalid_syntax<'i>(
    location: cssparser::SourceLocation,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    let reason = reason.into();
    error_at(
        location,
        ErrorKind::InvalidSyntax {
            reason: reason.clone(),
        },
        reason,
    )
}

pub(crate) fn invalid_selector<'i, 't>(
    input: &Parser<'i, 't>,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    invalid_selector_at(input.current_source_location(), reason)
}

fn invalid_selector_at<'i>(
    location: cssparser::SourceLocation,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    let reason = reason.into();
    error_at(
        location,
        ErrorKind::InvalidSelector {
            reason: reason.clone(),
        },
        reason,
    )
}

pub(crate) fn unsupported_property<'i, 't>(
    input: &Parser<'i, 't>,
    name: impl Into<String>,
) -> ParseError<'i, Error> {
    let name = name.into();
    error_at(
        input.current_source_location(),
        ErrorKind::UnsupportedProperty { name: name.clone() },
        format!("unsupported CSS property `{name}`"),
    )
}

pub(crate) fn property_name_error<'i, 't>(
    input: &Parser<'i, 't>,
    name: &str,
) -> ParseError<'i, Error> {
    match classify_property_name(name) {
        PropertyNameStatus::Supported => unsupported_property(input, name),
        PropertyNameStatus::KnownUnsupported => unsupported_property(input, name),
        PropertyNameStatus::Unknown => unknown_property(input, name),
    }
}

fn unknown_property<'i, 't>(
    input: &Parser<'i, 't>,
    name: impl Into<String>,
) -> ParseError<'i, Error> {
    let name = name.into();
    error_at(
        input.current_source_location(),
        ErrorKind::UnknownProperty { name: name.clone() },
        format!("unknown CSS property `{name}`"),
    )
}

pub(crate) fn unsupported_value<'i, 't>(
    input: &Parser<'i, 't>,
    property: Option<&str>,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    let reason = reason.into();
    error_at(
        input.current_source_location(),
        ErrorKind::UnsupportedValue {
            property: property.map(str::to_owned),
            reason: reason.clone(),
        },
        reason,
    )
}

pub(crate) fn unsupported_value_at<'i>(
    location: cssparser::SourceLocation,
    property: Option<&str>,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    let reason = reason.into();
    error_at(
        location,
        ErrorKind::UnsupportedValue {
            property: property.map(str::to_owned),
            reason: reason.clone(),
        },
        reason,
    )
}

pub(crate) fn invalid_color<'i>(
    location: cssparser::SourceLocation,
    value: impl Into<String>,
    message: impl Into<String>,
) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::InvalidColor {
            value: value.into(),
        },
        message,
    )
}

pub(crate) fn with_property_context<'i>(
    mut error: ParseError<'i, Error>,
    property: &str,
) -> ParseError<'i, Error> {
    if let ParseErrorKind::Custom(Error {
        kind: ErrorKind::UnsupportedValue {
            property: context, ..
        },
        ..
    }) = &mut error.kind
        && context.is_none()
    {
        *context = Some(property.to_owned());
    }
    error
}

pub(crate) fn error_at<'i>(
    location: cssparser::SourceLocation,
    kind: ErrorKind,
    message: impl Into<String>,
) -> ParseError<'i, Error> {
    ParseError {
        kind: ParseErrorKind::Custom(Error::at(location, kind, message)),
        location,
    }
}

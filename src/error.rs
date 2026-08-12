use std::fmt;

use cssparser::{
    BasicParseError, BasicParseErrorKind, ParseError, ParseErrorKind, Parser, ParserInput, ToCss,
    Token,
};

use crate::source::CssSourcePosition;
use crate::syntax::{CssCustomPropertyName, CssProperty};
use crate::validation::{PropertyNameStatus, classify_property_name, property_for_supported_name};

pub type Result<T> = std::result::Result<T, Error>;

/// A stable machine-readable root category for a CSS parse failure.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssErrorCode {
    UnexpectedEnd,
    UnexpectedToken,
    InvalidEncodingDeclaration,
    InvalidAtRulePlacement,
    InvalidAtRulePrelude,
    InvalidAtRuleBody,
    UnknownAtRule,
    UnsupportedAtRule,
    InvalidQualifiedRule,
    InvalidSelector,
    InvalidMediaQuery,
    UnknownProperty,
    UnsupportedProperty,
    InvalidPropertyValue,
    InvalidDeclarationAnnotation,
    UnknownDescriptor,
    UnsupportedDescriptor,
    InvalidDescriptorValue,
    InvalidDescriptorCombination,
    InvalidColorSyntax,
    NestingLimit,
}

/// A static identifier for one CSS grammar production.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssProductionId(&'static str);

impl CssProductionId {
    pub(crate) const fn new(value: &'static str) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A static description of the grammar expected at a failure position.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssGrammarExpectation(&'static str);

impl CssGrammarExpectation {
    pub(crate) const fn new(value: &'static str) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// A stable support-catalog identity attached to recognized unsupported syntax.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssFeatureId(&'static str);

impl CssFeatureId {
    pub(crate) const fn new(value: &'static str) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

macro_rules! owned_name {
    ($name:ident, $docs:literal) => {
        #[doc = $docs]
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $name(String);

        impl $name {
            pub(crate) fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

owned_name!(CssAtRuleName, "A decoded authored CSS at-rule name.");
owned_name!(CssPropertyName, "A decoded authored CSS property name.");
owned_name!(CssDescriptorName, "A decoded authored CSS descriptor name.");
owned_name!(
    CssMediaFeatureName,
    "A decoded authored media-feature name."
);
owned_name!(CssColorComponentName, "A semantic color component name.");

/// The CSS token category retained by an error summary.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssTokenKind {
    Ident,
    AtKeyword,
    Hash,
    IdHash,
    String,
    Url,
    Delim,
    Number,
    Percentage,
    Dimension,
    Whitespace,
    Comment,
    Colon,
    Semicolon,
    Comma,
    IncludeMatch,
    DashMatch,
    PrefixMatch,
    SuffixMatch,
    SubstringMatch,
    Cdo,
    Cdc,
    Function,
    ParenthesisBlock,
    SquareBracketBlock,
    CurlyBracketBlock,
    BadUrl,
    BadString,
    CloseParenthesis,
    CloseSquareBracket,
    CloseCurlyBracket,
}

/// A semantic token category plus its exact authored source slice.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssTokenSummary {
    kind: CssTokenKind,
    authored: String,
}

impl CssTokenSummary {
    #[must_use]
    pub const fn kind(&self) -> CssTokenKind {
        self.kind
    }

    #[must_use]
    pub fn authored(&self) -> &str {
        &self.authored
    }

    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token_kind(token),
            authored: token.to_css_string(),
        }
    }
}

const EXPECT_CSS_SYNTAX: CssGrammarExpectation = CssGrammarExpectation::new("valid CSS syntax");
const EXPECT_DECLARATION_VALUE: CssGrammarExpectation =
    CssGrammarExpectation::new("a declaration value");
const EXPECT_PROPERTY_VALUE: CssGrammarExpectation =
    CssGrammarExpectation::new("a value accepted by the property's grammar");
const EXPECT_DESCRIPTOR_VALUE: CssGrammarExpectation =
    CssGrammarExpectation::new("a value accepted by the descriptor grammar");
const EXPECT_SELECTOR: CssGrammarExpectation = CssGrammarExpectation::new("a supported selector");
const EXPECT_MEDIA_QUERY: CssGrammarExpectation =
    CssGrammarExpectation::new("a supported media query");
const EXPECT_COLOR: CssGrammarExpectation = CssGrammarExpectation::new("valid color syntax");

const QUALIFIED_RULE: CssProductionId = CssProductionId::new("css.qualified-rule");
const SELECTOR_LIST: CssProductionId = CssProductionId::new("baseline.selector.complex");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUnexpectedEndError {
    expectation: CssGrammarExpectation,
}

impl CssUnexpectedEndError {
    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUnexpectedTokenError {
    expectation: CssGrammarExpectation,
    encountered: CssTokenSummary,
}

impl CssUnexpectedTokenError {
    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }

    #[must_use]
    pub const fn encountered(&self) -> &CssTokenSummary {
        &self.encountered
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssEncodingDeclarationError {
    expectation: CssGrammarExpectation,
    encountered: Option<CssTokenSummary>,
}

impl CssEncodingDeclarationError {
    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }

    #[must_use]
    pub const fn encountered(&self) -> Option<&CssTokenSummary> {
        self.encountered.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssAtRulePlacementError {
    name: CssAtRuleName,
    expected_context: CssGrammarExpectation,
}

impl CssAtRulePlacementError {
    #[must_use]
    pub const fn name(&self) -> &CssAtRuleName {
        &self.name
    }

    #[must_use]
    pub const fn expected_context(&self) -> CssGrammarExpectation {
        self.expected_context
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssAtRuleSyntaxError {
    name: CssAtRuleName,
    production: CssProductionId,
    expectation: CssGrammarExpectation,
    encountered: Option<CssTokenSummary>,
}

impl CssAtRuleSyntaxError {
    #[must_use]
    pub const fn name(&self) -> &CssAtRuleName {
        &self.name
    }

    #[must_use]
    pub const fn production(&self) -> CssProductionId {
        self.production
    }

    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }

    #[must_use]
    pub const fn encountered(&self) -> Option<&CssTokenSummary> {
        self.encountered.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUnknownAtRuleError {
    name: CssAtRuleName,
}

impl CssUnknownAtRuleError {
    #[must_use]
    pub const fn name(&self) -> &CssAtRuleName {
        &self.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUnsupportedAtRuleError {
    name: CssAtRuleName,
    feature: CssFeatureId,
}

impl CssUnsupportedAtRuleError {
    #[must_use]
    pub const fn name(&self) -> &CssAtRuleName {
        &self.name
    }

    #[must_use]
    pub const fn feature(&self) -> CssFeatureId {
        self.feature
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssQualifiedRuleError {
    production: CssProductionId,
    expectation: CssGrammarExpectation,
    encountered: Option<CssTokenSummary>,
}

impl CssQualifiedRuleError {
    #[must_use]
    pub const fn production(&self) -> CssProductionId {
        self.production
    }

    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }

    #[must_use]
    pub const fn encountered(&self) -> Option<&CssTokenSummary> {
        self.encountered.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssSelectorError {
    production: Option<CssProductionId>,
    expectation: CssGrammarExpectation,
    encountered: Option<CssTokenSummary>,
}

impl CssSelectorError {
    #[must_use]
    pub const fn production(&self) -> Option<CssProductionId> {
        self.production
    }

    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }

    #[must_use]
    pub const fn encountered(&self) -> Option<&CssTokenSummary> {
        self.encountered.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssMediaQueryError {
    feature: Option<CssMediaFeatureName>,
    expectation: CssGrammarExpectation,
    encountered: Option<CssTokenSummary>,
}

impl CssMediaQueryError {
    #[must_use]
    pub const fn feature(&self) -> Option<&CssMediaFeatureName> {
        self.feature.as_ref()
    }

    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }

    #[must_use]
    pub const fn encountered(&self) -> Option<&CssTokenSummary> {
        self.encountered.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUnknownPropertyError {
    name: CssPropertyName,
}

impl CssUnknownPropertyError {
    #[must_use]
    pub const fn name(&self) -> &CssPropertyName {
        &self.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUnsupportedPropertyError {
    name: CssPropertyName,
    feature: CssFeatureId,
}

impl CssUnsupportedPropertyError {
    #[must_use]
    pub const fn name(&self) -> &CssPropertyName {
        &self.name
    }

    #[must_use]
    pub const fn feature(&self) -> CssFeatureId {
        self.feature
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssPropertyValueError {
    property: CssProperty,
    expectation: CssGrammarExpectation,
    encountered: Option<CssTokenSummary>,
}

impl CssPropertyValueError {
    #[must_use]
    pub const fn property(&self) -> &CssProperty {
        &self.property
    }

    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }

    #[must_use]
    pub const fn encountered(&self) -> Option<&CssTokenSummary> {
        self.encountered.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CssDeclarationContext {
    Ordinary(CssProperty),
    Keyframe(CssProperty),
    Descriptor {
        at_rule: CssAtRuleName,
        descriptor: CssDescriptorName,
    },
}

/// A borrowed, extensible inspection boundary for a declaration's grammar context.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssDeclarationContextRef<'a> {
    KnownProperty(&'a CssProperty),
    CustomProperty(&'a CssCustomPropertyName),
    Keyframe(&'a CssProperty),
    Descriptor {
        at_rule: &'a CssAtRuleName,
        descriptor: &'a CssDescriptorName,
    },
}

impl CssDeclarationContext {
    const fn as_ref(&self) -> CssDeclarationContextRef<'_> {
        match self {
            Self::Ordinary(CssProperty::Custom(property)) => {
                CssDeclarationContextRef::CustomProperty(property)
            }
            Self::Ordinary(property) => CssDeclarationContextRef::KnownProperty(property),
            Self::Keyframe(property) => CssDeclarationContextRef::Keyframe(property),
            Self::Descriptor {
                at_rule,
                descriptor,
            } => CssDeclarationContextRef::Descriptor {
                at_rule,
                descriptor,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssDeclarationAnnotationError {
    context: CssDeclarationContext,
    encountered: CssTokenSummary,
}

impl CssDeclarationAnnotationError {
    #[must_use]
    pub const fn context(&self) -> CssDeclarationContextRef<'_> {
        self.context.as_ref()
    }

    #[must_use]
    pub const fn encountered(&self) -> &CssTokenSummary {
        &self.encountered
    }

    fn make_keyframe(&mut self) {
        if let CssDeclarationContext::Ordinary(property) = &self.context {
            self.context = CssDeclarationContext::Keyframe(property.clone());
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUnknownDescriptorError {
    at_rule: CssAtRuleName,
    descriptor: CssDescriptorName,
}

impl CssUnknownDescriptorError {
    #[must_use]
    pub const fn at_rule(&self) -> &CssAtRuleName {
        &self.at_rule
    }

    #[must_use]
    pub const fn descriptor(&self) -> &CssDescriptorName {
        &self.descriptor
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUnsupportedDescriptorError {
    at_rule: CssAtRuleName,
    descriptor: CssDescriptorName,
    feature: CssFeatureId,
}

impl CssUnsupportedDescriptorError {
    #[must_use]
    pub const fn at_rule(&self) -> &CssAtRuleName {
        &self.at_rule
    }

    #[must_use]
    pub const fn descriptor(&self) -> &CssDescriptorName {
        &self.descriptor
    }

    #[must_use]
    pub const fn feature(&self) -> CssFeatureId {
        self.feature
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssDescriptorValueError {
    at_rule: CssAtRuleName,
    descriptor: CssDescriptorName,
    expectation: CssGrammarExpectation,
    encountered: Option<CssTokenSummary>,
}

impl CssDescriptorValueError {
    #[must_use]
    pub const fn at_rule(&self) -> &CssAtRuleName {
        &self.at_rule
    }

    #[must_use]
    pub const fn descriptor(&self) -> &CssDescriptorName {
        &self.descriptor
    }

    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }

    #[must_use]
    pub const fn encountered(&self) -> Option<&CssTokenSummary> {
        self.encountered.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssDescriptorCombinationError {
    at_rule: CssAtRuleName,
    responsible: CssDescriptorName,
    conflicting: Vec<CssDescriptorName>,
}

impl CssDescriptorCombinationError {
    #[must_use]
    pub const fn at_rule(&self) -> &CssAtRuleName {
        &self.at_rule
    }

    #[must_use]
    pub const fn responsible(&self) -> &CssDescriptorName {
        &self.responsible
    }

    #[must_use]
    pub fn conflicting(&self) -> &[CssDescriptorName] {
        &self.conflicting
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssColorSyntaxError {
    component: Option<CssColorComponentName>,
    expectation: CssGrammarExpectation,
    encountered: Option<CssTokenSummary>,
}

impl CssColorSyntaxError {
    #[must_use]
    pub const fn component(&self) -> Option<&CssColorComponentName> {
        self.component.as_ref()
    }

    #[must_use]
    pub const fn expectation(&self) -> CssGrammarExpectation {
        self.expectation
    }

    #[must_use]
    pub const fn encountered(&self) -> Option<&CssTokenSummary> {
        self.encountered.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssNestingLimitError {
    limit: u32,
    enclosing_production: CssProductionId,
}

impl CssNestingLimitError {
    #[must_use]
    pub const fn limit(&self) -> u32 {
        self.limit
    }

    #[must_use]
    pub const fn enclosing_production(&self) -> CssProductionId {
        self.enclosing_production
    }
}

/// The structured semantic detail for a CSS parse failure.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    UnexpectedEnd(CssUnexpectedEndError),
    UnexpectedToken(CssUnexpectedTokenError),
    InvalidEncodingDeclaration(CssEncodingDeclarationError),
    InvalidAtRulePlacement(CssAtRulePlacementError),
    InvalidAtRulePrelude(CssAtRuleSyntaxError),
    InvalidAtRuleBody(CssAtRuleSyntaxError),
    UnknownAtRule(CssUnknownAtRuleError),
    UnsupportedAtRule(CssUnsupportedAtRuleError),
    InvalidQualifiedRule(CssQualifiedRuleError),
    InvalidSelector(CssSelectorError),
    InvalidMediaQuery(CssMediaQueryError),
    UnknownProperty(CssUnknownPropertyError),
    UnsupportedProperty(CssUnsupportedPropertyError),
    InvalidPropertyValue(CssPropertyValueError),
    InvalidDeclarationAnnotation(CssDeclarationAnnotationError),
    UnknownDescriptor(CssUnknownDescriptorError),
    UnsupportedDescriptor(CssUnsupportedDescriptorError),
    InvalidDescriptorValue(CssDescriptorValueError),
    InvalidDescriptorCombination(CssDescriptorCombinationError),
    InvalidColorSyntax(CssColorSyntaxError),
    NestingLimit(CssNestingLimitError),
}

/// A structured CSS parse failure at one semantic source position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    position: CssSourcePosition,
}

impl Error {
    fn at(location: cssparser::SourceLocation, kind: ErrorKind) -> Self {
        Self {
            kind,
            position: CssSourcePosition::from_source_location(location),
        }
    }

    fn at_exact_nonzero_byte_offset(position: CssSourcePosition, kind: ErrorKind) -> Self {
        debug_assert_ne!(position.byte_offset().value(), 0);
        Self { kind, position }
    }

    #[must_use]
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    #[must_use]
    pub const fn code(&self) -> CssErrorCode {
        match self.kind {
            ErrorKind::UnexpectedEnd(_) => CssErrorCode::UnexpectedEnd,
            ErrorKind::UnexpectedToken(_) => CssErrorCode::UnexpectedToken,
            ErrorKind::InvalidEncodingDeclaration(_) => CssErrorCode::InvalidEncodingDeclaration,
            ErrorKind::InvalidAtRulePlacement(_) => CssErrorCode::InvalidAtRulePlacement,
            ErrorKind::InvalidAtRulePrelude(_) => CssErrorCode::InvalidAtRulePrelude,
            ErrorKind::InvalidAtRuleBody(_) => CssErrorCode::InvalidAtRuleBody,
            ErrorKind::UnknownAtRule(_) => CssErrorCode::UnknownAtRule,
            ErrorKind::UnsupportedAtRule(_) => CssErrorCode::UnsupportedAtRule,
            ErrorKind::InvalidQualifiedRule(_) => CssErrorCode::InvalidQualifiedRule,
            ErrorKind::InvalidSelector(_) => CssErrorCode::InvalidSelector,
            ErrorKind::InvalidMediaQuery(_) => CssErrorCode::InvalidMediaQuery,
            ErrorKind::UnknownProperty(_) => CssErrorCode::UnknownProperty,
            ErrorKind::UnsupportedProperty(_) => CssErrorCode::UnsupportedProperty,
            ErrorKind::InvalidPropertyValue(_) => CssErrorCode::InvalidPropertyValue,
            ErrorKind::InvalidDeclarationAnnotation(_) => {
                CssErrorCode::InvalidDeclarationAnnotation
            }
            ErrorKind::UnknownDescriptor(_) => CssErrorCode::UnknownDescriptor,
            ErrorKind::UnsupportedDescriptor(_) => CssErrorCode::UnsupportedDescriptor,
            ErrorKind::InvalidDescriptorValue(_) => CssErrorCode::InvalidDescriptorValue,
            ErrorKind::InvalidDescriptorCombination(_) => {
                CssErrorCode::InvalidDescriptorCombination
            }
            ErrorKind::InvalidColorSyntax(_) => CssErrorCode::InvalidColorSyntax,
            ErrorKind::NestingLimit(_) => CssErrorCode::NestingLimit,
        }
    }

    #[must_use]
    pub const fn position(&self) -> CssSourcePosition {
        self.position
    }

    fn resolve_source(mut self, source: &str) -> Self {
        // Location-only errors carry byte zero until they can be resolved against
        // the authored source. Body-parser errors retain their exact nonzero cursor.
        self.position = if self.position.byte_offset().value() != 0 {
            CssSourcePosition::from_byte_offset_in(source, self.position.byte_offset().value())
        } else {
            let source_location = cssparser::SourceLocation {
                line: self.position.line().value(),
                column: self.position.column().value().saturating_add(1),
            };
            CssSourcePosition::from_source_location_in(source, source_location)
        };

        if let Some((start, summary)) = next_authored_token_at(source, self.position) {
            if let Some(token) = encountered_mut(&mut self.kind) {
                if token.kind == summary.kind {
                    token.authored = summary.authored;
                    self.position = CssSourcePosition::from_byte_offset_in(source, start);
                }
            } else if let Some(slot) = optional_encountered_mut(&mut self.kind)
                && !is_bounded_end_token(summary.kind)
            {
                *slot = Some(summary);
                self.position = CssSourcePosition::from_byte_offset_in(source, start);
            }
        }

        if let Some(slot) = optional_encountered_mut(&mut self.kind)
            && slot.is_none()
            && let Some((start, summary)) = previous_authored_token_before(source, self.position)
            && !is_boundary_token(summary.kind)
        {
            *slot = Some(summary);
            self.position = CssSourcePosition::from_byte_offset_in(source, start);
        }

        if let Some(name) = at_rule_name(&self.kind)
            && let Some(start) = authored_at_rule_start(source, self.position, name.as_str())
        {
            self.position = CssSourcePosition::from_byte_offset_in(source, start);
        }

        if let ErrorKind::InvalidAtRuleBody(detail) = &mut self.kind
            && detail.name.as_str() == "at-rule"
            && let Some((_, name)) = authored_at_rule_before(source, self.position)
        {
            detail.name = CssAtRuleName::new(name);
            detail.production = production_for_at_rule(detail.name.as_str());
        }
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "CSS parse error {:?} at {}",
            self.code(),
            self.position
        )
    }
}

impl std::error::Error for Error {}

pub(crate) fn from_parse_error(source: &str, error: ParseError<'_, Error>) -> Error {
    match error.kind {
        ParseErrorKind::Custom(error) => error,
        ParseErrorKind::Basic(kind) => basic_error(error.location, kind),
    }
    .resolve_source(source)
}

pub(crate) fn from_rule_parse_error(
    source: &str,
    failed_unit: &str,
    error: ParseError<'_, Error>,
) -> Error {
    let mut error = from_parse_error(source, error);
    let unit = failed_unit.trim_start();
    if let Some(after_at) = unit.strip_prefix('@') {
        let name_end = after_at
            .find(|character: char| !character.is_alphanumeric() && character != '-')
            .unwrap_or(after_at.len());
        let name = &after_at[..name_end];
        if unit.trim_end().ends_with(';') && at_rule_requires_block(name) {
            error.kind = ErrorKind::InvalidAtRuleBody(CssAtRuleSyntaxError {
                name: CssAtRuleName::new(name),
                production: production_for_at_rule(name),
                expectation: CssGrammarExpectation::new("a block body for this at-rule"),
                encountered: None,
            });
        }
    } else if !unit.contains('{')
        && matches!(
            error.kind,
            ErrorKind::UnexpectedEnd(_) | ErrorKind::UnexpectedToken(_)
        )
    {
        error.kind = ErrorKind::InvalidQualifiedRule(CssQualifiedRuleError {
            production: QUALIFIED_RULE,
            expectation: EXPECT_CSS_SYNTAX,
            encountered: None,
        });
    }
    error
}

fn basic_error(location: cssparser::SourceLocation, kind: BasicParseErrorKind<'_>) -> Error {
    match kind {
        BasicParseErrorKind::EndOfInput => Error::at(
            location,
            ErrorKind::UnexpectedEnd(CssUnexpectedEndError {
                expectation: EXPECT_CSS_SYNTAX,
            }),
        ),
        BasicParseErrorKind::UnexpectedToken(token) => Error::at(
            location,
            ErrorKind::UnexpectedToken(CssUnexpectedTokenError {
                expectation: EXPECT_CSS_SYNTAX,
                encountered: CssTokenSummary::from_token(&token),
            }),
        ),
        BasicParseErrorKind::AtRuleInvalid(name) => {
            let name = name.to_string();
            if let Some(feature) = unsupported_at_rule_feature(&name) {
                Error::at(
                    location,
                    ErrorKind::UnsupportedAtRule(CssUnsupportedAtRuleError {
                        name: CssAtRuleName::new(name),
                        feature,
                    }),
                )
            } else {
                Error::at(
                    location,
                    ErrorKind::UnknownAtRule(CssUnknownAtRuleError {
                        name: CssAtRuleName::new(name),
                    }),
                )
            }
        }
        BasicParseErrorKind::QualifiedRuleInvalid => Error::at(
            location,
            ErrorKind::InvalidQualifiedRule(CssQualifiedRuleError {
                production: QUALIFIED_RULE,
                expectation: EXPECT_CSS_SYNTAX,
                encountered: None,
            }),
        ),
        BasicParseErrorKind::AtRuleBodyInvalid => Error::at(
            location,
            ErrorKind::InvalidAtRuleBody(CssAtRuleSyntaxError {
                name: CssAtRuleName::new("at-rule"),
                production: CssProductionId::new("css.at-rule"),
                expectation: EXPECT_CSS_SYNTAX,
                encountered: None,
            }),
        ),
    }
}

pub(crate) fn basic<'i>(error: BasicParseError<'i>) -> ParseError<'i, Error> {
    error.into()
}

pub(crate) fn selector_basic<'i>(error: BasicParseError<'i>) -> ParseError<'i, Error> {
    let location = error.location;
    let encountered = match error.kind {
        BasicParseErrorKind::UnexpectedToken(token) => Some(CssTokenSummary::from_token(&token)),
        _ => None,
    };
    error_at(
        location,
        ErrorKind::InvalidSelector(CssSelectorError {
            production: Some(SELECTOR_LIST),
            expectation: EXPECT_SELECTOR,
            encountered,
        }),
    )
}

pub(crate) fn invalid_syntax<'i>(
    location: cssparser::SourceLocation,
    _reason: impl Into<String>,
) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::InvalidQualifiedRule(CssQualifiedRuleError {
            production: QUALIFIED_RULE,
            expectation: EXPECT_CSS_SYNTAX,
            encountered: None,
        }),
    )
}

pub(crate) fn invalid_qualified_rule<'i>(
    location: cssparser::SourceLocation,
    production: &'static str,
    expectation: &'static str,
) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::InvalidQualifiedRule(CssQualifiedRuleError {
            production: CssProductionId::new(production),
            expectation: CssGrammarExpectation::new(expectation),
            encountered: None,
        }),
    )
}

pub(crate) fn invalid_at_rule_placement<'i>(
    location: cssparser::SourceLocation,
    name: &str,
    expected_context: &'static str,
) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::InvalidAtRulePlacement(CssAtRulePlacementError {
            name: CssAtRuleName::new(name),
            expected_context: CssGrammarExpectation::new(expected_context),
        }),
    )
}

pub(crate) fn with_at_rule_prelude_context<'i>(
    mut error: ParseError<'i, Error>,
    name: &str,
    production: &'static str,
    expectation: &'static str,
) -> ParseError<'i, Error> {
    let encountered = take_encountered(&mut error.kind);
    if !matches!(
        error.kind,
        ParseErrorKind::Custom(Error {
            kind: ErrorKind::InvalidMediaQuery(_),
            ..
        })
    ) {
        error.kind = ParseErrorKind::Custom(Error::at(
            error.location,
            ErrorKind::InvalidAtRulePrelude(CssAtRuleSyntaxError {
                name: CssAtRuleName::new(name),
                production: CssProductionId::new(production),
                expectation: CssGrammarExpectation::new(expectation),
                encountered,
            }),
        ));
    }
    error
}

pub(crate) fn invalid_at_rule_body<'i, 't>(
    input: &Parser<'i, 't>,
    name: &str,
    production: &'static str,
    expectation: &'static str,
) -> ParseError<'i, Error> {
    invalid_at_rule_body_at(
        CssSourcePosition::from_cssparser(input.position(), input.current_source_location()),
        input.current_source_location(),
        name,
        production,
        expectation,
    )
}

pub(crate) fn invalid_at_rule_block<'i, 't>(
    input: &Parser<'i, 't>,
    name: &str,
    production: &'static str,
    expectation: &'static str,
) -> ParseError<'i, Error> {
    let position =
        CssSourcePosition::from_cssparser(input.position(), input.current_source_location())
            .previous_ascii_byte();
    let mut location = input.current_source_location();
    location.column = location.column.saturating_sub(1).max(1);
    invalid_at_rule_body_at(position, location, name, production, expectation)
}

fn invalid_at_rule_body_at<'i>(
    position: CssSourcePosition,
    location: cssparser::SourceLocation,
    name: &str,
    production: &'static str,
    expectation: &'static str,
) -> ParseError<'i, Error> {
    ParseError {
        kind: ParseErrorKind::Custom(Error::at_exact_nonzero_byte_offset(
            position,
            ErrorKind::InvalidAtRuleBody(CssAtRuleSyntaxError {
                name: CssAtRuleName::new(name),
                production: CssProductionId::new(production),
                expectation: CssGrammarExpectation::new(expectation),
                encountered: None,
            }),
        )),
        location,
    }
}

pub(crate) fn invalid_selector<'i, 't>(
    input: &Parser<'i, 't>,
    _reason: impl Into<String>,
) -> ParseError<'i, Error> {
    error_at(
        input.current_source_location(),
        ErrorKind::InvalidSelector(CssSelectorError {
            production: Some(SELECTOR_LIST),
            expectation: EXPECT_SELECTOR,
            encountered: None,
        }),
    )
}

pub(crate) fn unsupported_property<'i>(
    location: cssparser::SourceLocation,
    name: impl Into<String>,
) -> ParseError<'i, Error> {
    let name = name.into();
    error_at(
        location,
        ErrorKind::UnsupportedProperty(CssUnsupportedPropertyError {
            feature: property_feature_id(&name),
            name: CssPropertyName::new(name),
        }),
    )
}

pub(crate) fn property_name_error<'i>(
    location: cssparser::SourceLocation,
    name: &str,
) -> ParseError<'i, Error> {
    match classify_property_name(name) {
        PropertyNameStatus::Supported | PropertyNameStatus::KnownUnsupported => {
            unsupported_property(location, name)
        }
        PropertyNameStatus::Unknown => error_at(
            location,
            ErrorKind::UnknownProperty(CssUnknownPropertyError {
                name: CssPropertyName::new(name),
            }),
        ),
    }
}

pub(crate) fn descriptor_name_error<'i>(
    location: cssparser::SourceLocation,
    at_rule: &str,
    name: &str,
) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::UnknownDescriptor(CssUnknownDescriptorError {
            at_rule: CssAtRuleName::new(at_rule),
            descriptor: CssDescriptorName::new(name),
        }),
    )
}

pub(crate) fn unsupported_value<'i, 't>(
    input: &Parser<'i, 't>,
    _property: Option<&str>,
    _reason: impl Into<String>,
) -> ParseError<'i, Error> {
    unexpected_at(input.current_source_location())
}

pub(crate) fn unsupported_value_at<'i>(
    location: cssparser::SourceLocation,
    _property: Option<&str>,
    _reason: impl Into<String>,
) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::UnexpectedEnd(CssUnexpectedEndError {
            expectation: EXPECT_DECLARATION_VALUE,
        }),
    )
}

pub(crate) fn invalid_color<'i>(
    location: cssparser::SourceLocation,
    component: Option<&str>,
) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::InvalidColorSyntax(CssColorSyntaxError {
            component: component.map(CssColorComponentName::new),
            expectation: EXPECT_COLOR,
            encountered: None,
        }),
    )
}

pub(crate) fn with_color_context<'i>(
    mut error: ParseError<'i, Error>,
    component: Option<&str>,
) -> ParseError<'i, Error> {
    let encountered = take_encountered(&mut error.kind);
    error.kind = ParseErrorKind::Custom(Error::at(
        error.location,
        ErrorKind::InvalidColorSyntax(CssColorSyntaxError {
            component: component.map(CssColorComponentName::new),
            expectation: EXPECT_COLOR,
            encountered,
        }),
    ));
    error
}

pub(crate) fn with_media_query_context<'i>(
    mut error: ParseError<'i, Error>,
    feature: Option<&str>,
) -> ParseError<'i, Error> {
    if matches!(
        error.kind,
        ParseErrorKind::Custom(Error {
            kind: ErrorKind::InvalidMediaQuery(_),
            ..
        })
    ) {
        return error;
    }
    let encountered = take_encountered(&mut error.kind);
    error.kind = ParseErrorKind::Custom(Error::at(
        error.location,
        ErrorKind::InvalidMediaQuery(CssMediaQueryError {
            feature: feature.map(CssMediaFeatureName::new),
            expectation: EXPECT_MEDIA_QUERY,
            encountered,
        }),
    ));
    error
}

pub(crate) fn with_property_context<'i>(
    mut error: ParseError<'i, Error>,
    property: &str,
) -> ParseError<'i, Error> {
    let Some(property) = property_for_supported_name(property) else {
        return error;
    };
    if matches!(
        error.kind,
        ParseErrorKind::Custom(Error {
            kind: ErrorKind::InvalidColorSyntax(_),
            ..
        })
    ) {
        return error;
    }
    let encountered = take_encountered(&mut error.kind);
    error.kind = ParseErrorKind::Custom(Error::at(
        error.location,
        ErrorKind::InvalidPropertyValue(CssPropertyValueError {
            property,
            expectation: EXPECT_PROPERTY_VALUE,
            encountered,
        }),
    ));
    error
}

pub(crate) fn with_descriptor_context<'i>(
    mut error: ParseError<'i, Error>,
    at_rule: &str,
    descriptor: &str,
) -> ParseError<'i, Error> {
    if matches!(
        error.kind,
        ParseErrorKind::Custom(Error {
            kind: ErrorKind::UnknownDescriptor(_) | ErrorKind::UnsupportedDescriptor(_),
            ..
        })
    ) {
        return error;
    }
    let encountered = take_encountered(&mut error.kind);
    error.kind = ParseErrorKind::Custom(Error::at(
        error.location,
        ErrorKind::InvalidDescriptorValue(CssDescriptorValueError {
            at_rule: CssAtRuleName::new(at_rule),
            descriptor: CssDescriptorName::new(descriptor),
            expectation: EXPECT_DESCRIPTOR_VALUE,
            encountered,
        }),
    ));
    error
}

pub(crate) fn with_declaration_annotation_context<'i>(
    mut error: ParseError<'i, Error>,
    property: CssProperty,
    keyframe: bool,
) -> ParseError<'i, Error> {
    let Some(encountered) = take_encountered(&mut error.kind) else {
        return error;
    };
    if encountered.kind != CssTokenKind::Delim || encountered.authored != "!" {
        return error;
    }
    let context = if keyframe {
        CssDeclarationContext::Keyframe(property)
    } else {
        CssDeclarationContext::Ordinary(property)
    };
    error.kind = ParseErrorKind::Custom(Error::at(
        error.location,
        ErrorKind::InvalidDeclarationAnnotation(CssDeclarationAnnotationError {
            context,
            encountered,
        }),
    ));
    error
}

pub(crate) fn with_descriptor_annotation_context<'i>(
    mut error: ParseError<'i, Error>,
    at_rule: &str,
    descriptor: &str,
) -> ParseError<'i, Error> {
    let Some(encountered) = take_encountered(&mut error.kind) else {
        return error;
    };
    if encountered.kind != CssTokenKind::Delim || encountered.authored != "!" {
        return error;
    }
    error.kind = ParseErrorKind::Custom(Error::at(
        error.location,
        ErrorKind::InvalidDeclarationAnnotation(CssDeclarationAnnotationError {
            context: CssDeclarationContext::Descriptor {
                at_rule: CssAtRuleName::new(at_rule),
                descriptor: CssDescriptorName::new(descriptor),
            },
            encountered,
        }),
    ));
    error
}

pub(crate) fn make_keyframe_annotation_context(error: &mut ParseError<'_, Error>) {
    if let ParseErrorKind::Custom(Error {
        kind: ErrorKind::InvalidDeclarationAnnotation(detail),
        ..
    }) = &mut error.kind
    {
        detail.make_keyframe();
    }
}

pub(crate) fn invalid_descriptor_combination<'i>(
    location: cssparser::SourceLocation,
    at_rule: &str,
    responsible: &str,
    conflicting: &[&str],
) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::InvalidDescriptorCombination(CssDescriptorCombinationError {
            at_rule: CssAtRuleName::new(at_rule),
            responsible: CssDescriptorName::new(responsible),
            conflicting: conflicting
                .iter()
                .map(|name| CssDescriptorName::new(*name))
                .collect(),
        }),
    )
}

pub(crate) fn error_at<'i>(
    location: cssparser::SourceLocation,
    kind: ErrorKind,
) -> ParseError<'i, Error> {
    ParseError {
        kind: ParseErrorKind::Custom(Error::at(location, kind)),
        location,
    }
}

fn unexpected_at<'i>(location: cssparser::SourceLocation) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::UnexpectedEnd(CssUnexpectedEndError {
            expectation: EXPECT_DECLARATION_VALUE,
        }),
    )
}

fn take_encountered(kind: &mut ParseErrorKind<'_, Error>) -> Option<CssTokenSummary> {
    match kind {
        ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(token)) => {
            Some(CssTokenSummary::from_token(token))
        }
        ParseErrorKind::Custom(error) => encountered_mut(&mut error.kind).cloned(),
        _ => None,
    }
}

fn encountered_mut(kind: &mut ErrorKind) -> Option<&mut CssTokenSummary> {
    match kind {
        ErrorKind::UnexpectedToken(detail) => Some(&mut detail.encountered),
        ErrorKind::InvalidEncodingDeclaration(detail) => detail.encountered.as_mut(),
        ErrorKind::InvalidAtRulePrelude(detail) | ErrorKind::InvalidAtRuleBody(detail) => {
            detail.encountered.as_mut()
        }
        ErrorKind::InvalidQualifiedRule(detail) => detail.encountered.as_mut(),
        ErrorKind::InvalidSelector(detail) => detail.encountered.as_mut(),
        ErrorKind::InvalidMediaQuery(detail) => detail.encountered.as_mut(),
        ErrorKind::InvalidPropertyValue(detail) => detail.encountered.as_mut(),
        ErrorKind::InvalidDeclarationAnnotation(detail) => Some(&mut detail.encountered),
        ErrorKind::InvalidDescriptorValue(detail) => detail.encountered.as_mut(),
        ErrorKind::InvalidColorSyntax(detail) => detail.encountered.as_mut(),
        _ => None,
    }
}

fn optional_encountered_mut(kind: &mut ErrorKind) -> Option<&mut Option<CssTokenSummary>> {
    match kind {
        ErrorKind::InvalidEncodingDeclaration(detail) => Some(&mut detail.encountered),
        ErrorKind::InvalidAtRulePrelude(detail) | ErrorKind::InvalidAtRuleBody(detail) => {
            Some(&mut detail.encountered)
        }
        ErrorKind::InvalidQualifiedRule(detail) => Some(&mut detail.encountered),
        ErrorKind::InvalidSelector(detail) => Some(&mut detail.encountered),
        ErrorKind::InvalidMediaQuery(detail) => Some(&mut detail.encountered),
        ErrorKind::InvalidPropertyValue(detail) => Some(&mut detail.encountered),
        ErrorKind::InvalidDescriptorValue(detail) => Some(&mut detail.encountered),
        ErrorKind::InvalidColorSyntax(detail) => Some(&mut detail.encountered),
        _ => None,
    }
}

fn token_kind(token: &Token<'_>) -> CssTokenKind {
    match token {
        Token::Ident(_) => CssTokenKind::Ident,
        Token::AtKeyword(_) => CssTokenKind::AtKeyword,
        Token::Hash(_) => CssTokenKind::Hash,
        Token::IDHash(_) => CssTokenKind::IdHash,
        Token::QuotedString(_) => CssTokenKind::String,
        Token::UnquotedUrl(_) => CssTokenKind::Url,
        Token::Delim(_) => CssTokenKind::Delim,
        Token::Number { .. } => CssTokenKind::Number,
        Token::Percentage { .. } => CssTokenKind::Percentage,
        Token::Dimension { .. } => CssTokenKind::Dimension,
        Token::WhiteSpace(_) => CssTokenKind::Whitespace,
        Token::Comment(_) => CssTokenKind::Comment,
        Token::Colon => CssTokenKind::Colon,
        Token::Semicolon => CssTokenKind::Semicolon,
        Token::Comma => CssTokenKind::Comma,
        Token::IncludeMatch => CssTokenKind::IncludeMatch,
        Token::DashMatch => CssTokenKind::DashMatch,
        Token::PrefixMatch => CssTokenKind::PrefixMatch,
        Token::SuffixMatch => CssTokenKind::SuffixMatch,
        Token::SubstringMatch => CssTokenKind::SubstringMatch,
        Token::CDO => CssTokenKind::Cdo,
        Token::CDC => CssTokenKind::Cdc,
        Token::Function(_) => CssTokenKind::Function,
        Token::ParenthesisBlock => CssTokenKind::ParenthesisBlock,
        Token::SquareBracketBlock => CssTokenKind::SquareBracketBlock,
        Token::CurlyBracketBlock => CssTokenKind::CurlyBracketBlock,
        Token::BadUrl(_) => CssTokenKind::BadUrl,
        Token::BadString(_) => CssTokenKind::BadString,
        Token::CloseParenthesis => CssTokenKind::CloseParenthesis,
        Token::CloseSquareBracket => CssTokenKind::CloseSquareBracket,
        Token::CloseCurlyBracket => CssTokenKind::CloseCurlyBracket,
    }
}

fn next_authored_token_at(
    source: &str,
    position: CssSourcePosition,
) -> Option<(usize, CssTokenSummary)> {
    let start = position.byte_offset().value();
    let tail = source.get(start..)?;
    if tail.is_empty() {
        return None;
    }
    let mut input = ParserInput::new(tail);
    let mut parser = Parser::new(&mut input);
    loop {
        let token_start = parser.position().byte_index();
        let token = parser
            .next_including_whitespace_and_comments()
            .ok()?
            .clone();
        let token_end = parser.position().byte_index();
        if matches!(token, Token::WhiteSpace(_) | Token::Comment(_)) {
            continue;
        }
        let authored = tail.get(token_start..token_end)?.to_owned();
        return Some((
            start + token_start,
            CssTokenSummary {
                kind: token_kind(&token),
                authored,
            },
        ));
    }
}

const fn is_bounded_end_token(kind: CssTokenKind) -> bool {
    matches!(
        kind,
        CssTokenKind::Semicolon
            | CssTokenKind::CloseParenthesis
            | CssTokenKind::CloseSquareBracket
            | CssTokenKind::CloseCurlyBracket
    )
}

const fn is_boundary_token(kind: CssTokenKind) -> bool {
    is_bounded_end_token(kind)
        || matches!(
            kind,
            CssTokenKind::Colon
                | CssTokenKind::Comma
                | CssTokenKind::ParenthesisBlock
                | CssTokenKind::SquareBracketBlock
                | CssTokenKind::CurlyBracketBlock
        )
}

fn previous_authored_token_before(
    source: &str,
    position: CssSourcePosition,
) -> Option<(usize, CssTokenSummary)> {
    let end = position.byte_offset().value().min(source.len());
    let window_start = source[..end]
        .char_indices()
        .rev()
        .find_map(|(index, character)| {
            matches!(character, '{' | ';' | ':' | ',' | '(' | '[').then_some(index + 1)
        })
        .unwrap_or(0);
    let window = source.get(window_start..end)?;
    let mut input = ParserInput::new(window);
    let mut parser = Parser::new(&mut input);
    let mut previous = None;
    while !parser.is_exhausted() {
        let token_start = parser.position().byte_index();
        let token = parser
            .next_including_whitespace_and_comments()
            .ok()?
            .clone();
        let token_end = parser.position().byte_index();
        if matches!(token, Token::WhiteSpace(_) | Token::Comment(_)) {
            continue;
        }
        previous = Some((
            window_start + token_start,
            CssTokenSummary {
                kind: token_kind(&token),
                authored: window.get(token_start..token_end)?.to_owned(),
            },
        ));
    }
    previous
}

fn at_rule_name(kind: &ErrorKind) -> Option<&CssAtRuleName> {
    match kind {
        ErrorKind::InvalidAtRulePlacement(detail) => Some(&detail.name),
        ErrorKind::UnknownAtRule(detail) => Some(&detail.name),
        ErrorKind::UnsupportedAtRule(detail) => Some(&detail.name),
        _ => None,
    }
}

fn authored_at_rule_start(
    source: &str,
    position: CssSourcePosition,
    expected_name: &str,
) -> Option<usize> {
    authored_at_rule_before(source, position)
        .and_then(|(start, name)| name.eq_ignore_ascii_case(expected_name).then_some(start))
}

fn authored_at_rule_before(source: &str, position: CssSourcePosition) -> Option<(usize, String)> {
    let end = position.byte_offset().value().min(source.len());
    for (start, _) in source[..end].rmatch_indices('@') {
        let tail = source.get(start..)?;
        let mut input = ParserInput::new(tail);
        let mut parser = Parser::new(&mut input);
        if let Ok(Token::AtKeyword(name)) = parser.next_including_whitespace_and_comments() {
            return Some((start, name.to_string()));
        }
    }
    None
}

fn production_for_at_rule(name: &str) -> CssProductionId {
    if name.eq_ignore_ascii_case("import") {
        CssProductionId::new("baseline.rule.import")
    } else if name.eq_ignore_ascii_case("layer") {
        CssProductionId::new("baseline.rule.layer-block")
    } else if name.eq_ignore_ascii_case("font-face") {
        CssProductionId::new("baseline.rule.font-face")
    } else if name.eq_ignore_ascii_case("keyframes") {
        CssProductionId::new("baseline.rule.keyframes")
    } else if name.eq_ignore_ascii_case("media") {
        CssProductionId::new("baseline.rule.media")
    } else if name.eq_ignore_ascii_case("container") {
        CssProductionId::new("baseline.rule.container")
    } else if name.eq_ignore_ascii_case("scope") {
        CssProductionId::new("baseline.rule.scope")
    } else {
        CssProductionId::new("css.at-rule")
    }
}

fn unsupported_at_rule_feature(name: &str) -> Option<CssFeatureId> {
    if name.eq_ignore_ascii_case("namespace") {
        Some(CssFeatureId::new("later.rule.namespace"))
    } else if name.eq_ignore_ascii_case("supports") {
        Some(CssFeatureId::new("later.rule.supports"))
    } else if name.eq_ignore_ascii_case("counter-style") {
        Some(CssFeatureId::new("later.rule.counter-style"))
    } else if name.eq_ignore_ascii_case("page") {
        Some(CssFeatureId::new("later.rule.page"))
    } else if name.eq_ignore_ascii_case("font-feature-values") {
        Some(CssFeatureId::new("later.rule.font-feature-values"))
    } else {
        None
    }
}

fn at_rule_requires_block(name: &str) -> bool {
    name.eq_ignore_ascii_case("font-face")
        || name.eq_ignore_ascii_case("keyframes")
        || name.eq_ignore_ascii_case("media")
        || name.eq_ignore_ascii_case("container")
        || name.eq_ignore_ascii_case("scope")
}

fn property_feature_id(name: &str) -> CssFeatureId {
    let _ = name;
    CssFeatureId::new("baseline.property.recognized-unsupported")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position() -> cssparser::SourceLocation {
        cssparser::SourceLocation { line: 0, column: 1 }
    }

    #[test]
    fn error_code_has_one_to_one_mapping_for_every_root() {
        let token = CssTokenSummary {
            kind: CssTokenKind::Ident,
            authored: "x".to_owned(),
        };
        let at_rule = CssAtRuleName::new("x");
        let descriptor = CssDescriptorName::new("x");
        let roots = vec![
            (
                ErrorKind::UnexpectedEnd(CssUnexpectedEndError {
                    expectation: EXPECT_CSS_SYNTAX,
                }),
                CssErrorCode::UnexpectedEnd,
            ),
            (
                ErrorKind::UnexpectedToken(CssUnexpectedTokenError {
                    expectation: EXPECT_CSS_SYNTAX,
                    encountered: token.clone(),
                }),
                CssErrorCode::UnexpectedToken,
            ),
            (
                ErrorKind::InvalidEncodingDeclaration(CssEncodingDeclarationError {
                    expectation: EXPECT_CSS_SYNTAX,
                    encountered: None,
                }),
                CssErrorCode::InvalidEncodingDeclaration,
            ),
            (
                ErrorKind::InvalidAtRulePlacement(CssAtRulePlacementError {
                    name: at_rule.clone(),
                    expected_context: EXPECT_CSS_SYNTAX,
                }),
                CssErrorCode::InvalidAtRulePlacement,
            ),
            (
                ErrorKind::InvalidAtRulePrelude(CssAtRuleSyntaxError {
                    name: at_rule.clone(),
                    production: QUALIFIED_RULE,
                    expectation: EXPECT_CSS_SYNTAX,
                    encountered: None,
                }),
                CssErrorCode::InvalidAtRulePrelude,
            ),
            (
                ErrorKind::InvalidAtRuleBody(CssAtRuleSyntaxError {
                    name: at_rule.clone(),
                    production: QUALIFIED_RULE,
                    expectation: EXPECT_CSS_SYNTAX,
                    encountered: None,
                }),
                CssErrorCode::InvalidAtRuleBody,
            ),
            (
                ErrorKind::UnknownAtRule(CssUnknownAtRuleError {
                    name: at_rule.clone(),
                }),
                CssErrorCode::UnknownAtRule,
            ),
            (
                ErrorKind::UnsupportedAtRule(CssUnsupportedAtRuleError {
                    name: at_rule.clone(),
                    feature: CssFeatureId::new("x"),
                }),
                CssErrorCode::UnsupportedAtRule,
            ),
            (
                ErrorKind::InvalidQualifiedRule(CssQualifiedRuleError {
                    production: QUALIFIED_RULE,
                    expectation: EXPECT_CSS_SYNTAX,
                    encountered: None,
                }),
                CssErrorCode::InvalidQualifiedRule,
            ),
            (
                ErrorKind::InvalidSelector(CssSelectorError {
                    production: None,
                    expectation: EXPECT_SELECTOR,
                    encountered: None,
                }),
                CssErrorCode::InvalidSelector,
            ),
            (
                ErrorKind::InvalidMediaQuery(CssMediaQueryError {
                    feature: None,
                    expectation: EXPECT_MEDIA_QUERY,
                    encountered: None,
                }),
                CssErrorCode::InvalidMediaQuery,
            ),
            (
                ErrorKind::UnknownProperty(CssUnknownPropertyError {
                    name: CssPropertyName::new("x"),
                }),
                CssErrorCode::UnknownProperty,
            ),
            (
                ErrorKind::UnsupportedProperty(CssUnsupportedPropertyError {
                    name: CssPropertyName::new("x"),
                    feature: CssFeatureId::new("x"),
                }),
                CssErrorCode::UnsupportedProperty,
            ),
            (
                ErrorKind::InvalidPropertyValue(CssPropertyValueError {
                    property: CssProperty::Width,
                    expectation: EXPECT_PROPERTY_VALUE,
                    encountered: None,
                }),
                CssErrorCode::InvalidPropertyValue,
            ),
            (
                ErrorKind::InvalidDeclarationAnnotation(CssDeclarationAnnotationError {
                    context: CssDeclarationContext::Ordinary(CssProperty::Width),
                    encountered: token.clone(),
                }),
                CssErrorCode::InvalidDeclarationAnnotation,
            ),
            (
                ErrorKind::UnknownDescriptor(CssUnknownDescriptorError {
                    at_rule: at_rule.clone(),
                    descriptor: descriptor.clone(),
                }),
                CssErrorCode::UnknownDescriptor,
            ),
            (
                ErrorKind::UnsupportedDescriptor(CssUnsupportedDescriptorError {
                    at_rule: at_rule.clone(),
                    descriptor: descriptor.clone(),
                    feature: CssFeatureId::new("x"),
                }),
                CssErrorCode::UnsupportedDescriptor,
            ),
            (
                ErrorKind::InvalidDescriptorValue(CssDescriptorValueError {
                    at_rule: at_rule.clone(),
                    descriptor: descriptor.clone(),
                    expectation: EXPECT_DESCRIPTOR_VALUE,
                    encountered: None,
                }),
                CssErrorCode::InvalidDescriptorValue,
            ),
            (
                ErrorKind::InvalidDescriptorCombination(CssDescriptorCombinationError {
                    at_rule: at_rule.clone(),
                    responsible: descriptor.clone(),
                    conflicting: vec![descriptor],
                }),
                CssErrorCode::InvalidDescriptorCombination,
            ),
            (
                ErrorKind::InvalidColorSyntax(CssColorSyntaxError {
                    component: None,
                    expectation: EXPECT_COLOR,
                    encountered: None,
                }),
                CssErrorCode::InvalidColorSyntax,
            ),
            (
                ErrorKind::NestingLimit(CssNestingLimitError {
                    limit: 256,
                    enclosing_production: QUALIFIED_RULE,
                }),
                CssErrorCode::NestingLimit,
            ),
        ];

        for (kind, expected) in roots {
            assert_eq!(Error::at(position(), kind).code(), expected);
        }
    }

    #[test]
    fn error_unreachable_detail_accessors_expose_every_semantic_field() {
        let token = CssTokenSummary {
            kind: CssTokenKind::Delim,
            authored: "!".to_owned(),
        };
        let unexpected_end = CssUnexpectedEndError {
            expectation: EXPECT_CSS_SYNTAX,
        };
        assert_eq!(unexpected_end.expectation().as_str(), "valid CSS syntax");
        let unexpected_token = CssUnexpectedTokenError {
            expectation: EXPECT_CSS_SYNTAX,
            encountered: token.clone(),
        };
        assert_eq!(unexpected_token.expectation().as_str(), "valid CSS syntax");
        assert_eq!(unexpected_token.encountered().kind(), CssTokenKind::Delim);
        let encoding = CssEncodingDeclarationError {
            expectation: CssGrammarExpectation::new("a quoted encoding label"),
            encountered: Some(token.clone()),
        };
        assert_eq!(encoding.expectation().as_str(), "a quoted encoding label");
        assert_eq!(encoding.encountered().unwrap().authored(), "!");

        let unsupported_property = CssUnsupportedPropertyError {
            name: CssPropertyName::new("future-property"),
            feature: CssFeatureId::new("later.property.future-property"),
        };
        assert_eq!(unsupported_property.name().as_str(), "future-property");
        assert_eq!(
            unsupported_property.feature().as_str(),
            "later.property.future-property"
        );

        let unsupported_descriptor = CssUnsupportedDescriptorError {
            at_rule: CssAtRuleName::new("font-face"),
            descriptor: CssDescriptorName::new("future-descriptor"),
            feature: CssFeatureId::new("later.descriptor.future-descriptor"),
        };
        assert_eq!(unsupported_descriptor.at_rule().as_str(), "font-face");
        assert_eq!(
            unsupported_descriptor.descriptor().as_str(),
            "future-descriptor"
        );
        assert_eq!(
            unsupported_descriptor.feature().as_str(),
            "later.descriptor.future-descriptor"
        );

        let annotation = CssDeclarationAnnotationError {
            context: CssDeclarationContext::Descriptor {
                at_rule: CssAtRuleName::new("font-face"),
                descriptor: CssDescriptorName::new("src"),
            },
            encountered: token.clone(),
        };
        match annotation.context() {
            CssDeclarationContextRef::Descriptor {
                at_rule,
                descriptor,
            } => {
                assert_eq!(at_rule.as_str(), "font-face");
                assert_eq!(descriptor.as_str(), "src");
            }
            _ => panic!("wrong declaration context"),
        }
        assert_eq!(annotation.encountered().authored(), "!");

        let custom = CssDeclarationAnnotationError {
            context: CssDeclarationContext::Ordinary(CssProperty::Custom(
                CssCustomPropertyName::new("--theme"),
            )),
            encountered: token.clone(),
        };
        match custom.context() {
            CssDeclarationContextRef::CustomProperty(name) => {
                assert_eq!(name.as_str(), "--theme");
            }
            _ => panic!("wrong custom-property context"),
        }

        let keyframe = CssDeclarationAnnotationError {
            context: CssDeclarationContext::Keyframe(CssProperty::Opacity),
            encountered: token,
        };
        match keyframe.context() {
            CssDeclarationContextRef::Keyframe(property) => {
                assert_eq!(property, &CssProperty::Opacity);
            }
            _ => panic!("wrong keyframe context"),
        }

        let nesting = CssNestingLimitError {
            limit: 256,
            enclosing_production: QUALIFIED_RULE,
        };
        assert_eq!(nesting.limit(), 256);
        assert_eq!(
            nesting.enclosing_production().as_str(),
            "css.qualified-rule"
        );
    }
}

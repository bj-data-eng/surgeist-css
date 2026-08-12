//! Strict CSS ingestion for Surgeist style sheets.
//!
//! This module parses CSS syntax into CSS-owned authored syntax values. It is
//! strict by design: unsupported selectors, at-rules, properties, and values are
//! errors instead of browser-style recoverable invalid declarations.
//!
//! Parse failures expose typed [`ErrorKind`] values plus source line and column
//! information so callers do not need to parse display strings.

mod background;
mod box_model;
mod effects;
mod font_face;
mod generated_content;
mod grid;
mod keyframes;
mod layout;
mod nesting;
mod queries;
mod selectors;
mod timing;
mod typography;
mod values;
mod variables;

use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, Delimiter, ParseError, Parser, ParserInput,
    ParserState, QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, Token,
    match_ignore_ascii_case,
};

use background::*;
use box_model::*;
use effects::*;
use font_face::parse_font_face_rule;
use generated_content::*;
use grid::*;
use keyframes::{parse_keyframes_name, parse_keyframes_rule};
use layout::*;
use nesting::parse_style_rule_block;
#[cfg(test)]
pub(crate) use queries::parse_container_condition_for_test;
#[cfg(test)]
pub(crate) use queries::parse_media_query_list_for_test;
use queries::{parse_container_condition, parse_media_query_list};
use selectors::{
    parse_rule_selector_list, parse_scope_boundary_selector_list, parse_scoped_style_selector_list,
};
use timing::*;
use typography::*;
use values::*;
use variables::{
    collect_authored_declaration_value, parse_custom_property_name, parse_custom_property_value,
};

use crate::error::{
    Error, basic, from_parse_error, from_rule_parse_error, invalid_at_rule_block,
    invalid_at_rule_placement, invalid_custom_declaration_annotation,
    invalid_descriptor_annotation, invalid_encoding_declaration,
    invalid_known_declaration_annotation, invalid_root_syntax, invalid_syntax,
    normalize_encoding_error, property_name_error, unsupported_value, with_at_rule_prelude_context,
    with_encoding_declaration_context, with_media_query_context, with_property_context,
};
use crate::properties::{CssOverflowPropertyValue, property_schema};
use crate::syntax::*;
use crate::validation::parse_global_keyword;

macro_rules! define_property_dispatch {
    ($input:ident;
        All, $all_canonical:literal, [$($all_alias:literal),*], $all_stable_id:literal,
        $all_value:ty, $all_parser:ident, $all_dispatch:block;
        $(
        $variant:ident, $canonical:literal, [$($alias:literal),*], $stable_id:literal,
        $value:ty, $parser:ident, $dispatch:block;
    )*) => {
        fn parse_known_property_value<'i, 't>(
            property: crate::CssKnownProperty,
            $input: &mut Parser<'i, 't>,
        ) -> std::result::Result<CssKnownDeclaration, ParseError<'i, Error>> {
            match property {
                crate::CssKnownProperty::All => {
                    let _authored_value_type = std::marker::PhantomData::<$all_value>;
                    let keyword = $all_dispatch;
                    Ok(CssKnownDeclaration::All(CssAllDeclaredValue::Global(keyword)))
                }
                $(crate::CssKnownProperty::$variant => {
                    let _authored_value_type = std::marker::PhantomData::<$value>;
                    let value = $dispatch;
                    Ok(CssKnownDeclaration::$variant(CssDeclaredValue::Value(value)))
                },)*
            }
        }
    };
}

property_schema!(define_property_dispatch, input);

fn parse_all_property<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGlobalKeyword, ParseError<'i, Error>> {
    Err(unsupported_value(
        input,
        None,
        "`all` only accepts CSS-wide global keywords",
    ))
}

fn parse_overflow_property<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssOverflowPropertyValue, ParseError<'i, Error>> {
    match parse_overflow_value(input)? {
        CssValue::Overflow(value) => Ok(CssOverflowPropertyValue::Single(value)),
        CssValue::OverflowAxes(value) => Ok(CssOverflowPropertyValue::Pair(value)),
    }
}

/// Parses a UTF-8 stylesheet into valid authored syntax and recovery diagnostics.
///
/// The ordinary parser retains valid top-level rules in source order and reports
/// each discarded top-level rule with its complete balanced source span. A valid
/// leading legacy `@charset` declaration is metadata only and never decodes the
/// already-UTF-8 input. Recovery does not apply cascade, substitution, selector
/// matching, contextual resolution, or resource loading.
///
/// ```
/// use surgeist_css::{CssRecoveryAction, CssRule, parse_sheet};
///
/// let report = parse_sheet(
///     ".before { color: red; } @unknown fn({x;y}); .after { color: blue; }",
/// );
/// assert_eq!(report.syntax().rules().len(), 2);
/// assert!(!report.is_clean());
/// assert!(matches!(report.syntax().rules()[0], CssRule::Style(_)));
/// assert!(matches!(
///     report.diagnostics()[0].action(),
///     CssRecoveryAction::DropAtRule
/// ));
/// ```
pub fn parse_sheet(source: &str) -> crate::CssParseReport<CssSheet> {
    let mut input = ParserInput::new(source);
    let mut parser = Parser::new(&mut input);
    if source.starts_with('\u{feff}') {
        let _ = parser.next_including_whitespace_and_comments();
    }
    let mut rule_parser = StrictRuleParser::top_level(source);
    let mut sheet = CssSheet::new();
    let mut diagnostics = Vec::new();
    let mut previous_end = parser.position().byte_index();

    {
        let mut rules = RuleBodyParser::new(&mut parser, &mut rule_parser);
        loop {
            if let Some(diagnostic) = discard_malformed_top_level_token(source, rules.input) {
                rules.parser.encoding_allowed = false;
                previous_end = diagnostic.span().end().byte_offset().value();
                diagnostics.push(diagnostic);
                continue;
            }
            let Some(result) = rules.next() else {
                break;
            };
            let position_after_result = rules.input.position().byte_index();
            if result.is_err()
                && position_after_result > 0
                && source.as_bytes().get(position_after_result - 1) == Some(&b'{')
            {
                let _: std::result::Result<(), ParseError<'_, ()>> =
                    rules.input.parse_nested_block(|nested| {
                        while nested.next_including_whitespace_and_comments().is_ok() {}
                        Ok(())
                    });
            }
            let unit_end = rules.input.position().byte_index();
            diagnostics.append(&mut rules.parser.diagnostics);
            match result {
                Ok(parsed_rules) => {
                    for rule in parsed_rules {
                        sheet.push_rule(rule);
                    }
                }
                Err((error, failed_unit)) => {
                    let unit_start =
                        recovery_unit_start(source, previous_end, unit_end, failed_unit);
                    let error = from_rule_parse_error(source, failed_unit, error);
                    let error = if recovery_at_rule_name(failed_unit)
                        .is_some_and(|name| name.eq_ignore_ascii_case("charset"))
                    {
                        normalize_encoding_error(source, unit_start, unit_end, failed_unit, error)
                    } else {
                        error
                    };
                    let action = if failed_unit.trim_start().starts_with('@') {
                        crate::CssRecoveryAction::DropAtRule
                    } else {
                        crate::CssRecoveryAction::DropQualifiedRule
                    };
                    if let Some(span) = crate::CssSourceSpan::new(
                        crate::CssSourcePosition::from_byte_offset_in(source, unit_start),
                        crate::CssSourcePosition::from_byte_offset_in(source, unit_end),
                    ) && let Some(diagnostic) =
                        crate::CssRecoveryDiagnostic::new(error, span, action)
                    {
                        diagnostics.push(diagnostic);
                    }
                }
            }
            previous_end = unit_end;
        }
    }

    if let Some(encoding) = rule_parser.encoding.take() {
        sheet.set_encoding(encoding);
    }

    crate::CssParseReport::new(sheet, diagnostics)
}

fn discard_malformed_top_level_token(
    source: &str,
    input: &mut Parser<'_, '_>,
) -> Option<crate::CssRecoveryDiagnostic> {
    loop {
        let state = input.state();
        let token_start = input.position().byte_index();
        match input.next_including_whitespace_and_comments() {
            Ok(Token::WhiteSpace(_) | Token::Comment(_) | Token::CDO | Token::CDC) => {}
            Ok(token @ (Token::Semicolon | Token::CloseCurlyBracket)) => {
                let token = token.clone();
                let token_end = input.position().byte_index();
                let error = invalid_root_syntax(source, token_start, &token);
                let span = crate::CssSourceSpan::new(
                    crate::CssSourcePosition::from_byte_offset_in(source, token_start),
                    crate::CssSourcePosition::from_byte_offset_in(source, token_end),
                )
                .expect("a consumed top-level token has an ordered non-empty span");
                let diagnostic = crate::CssRecoveryDiagnostic::new(
                    error,
                    span,
                    crate::CssRecoveryAction::DropQualifiedRule,
                )
                .expect("the responsible top-level token starts its recovery span");
                return Some(diagnostic);
            }
            Ok(_) | Err(_) => {
                input.reset(&state);
                return None;
            }
        }
    }
}

fn recovery_unit_start(
    source: &str,
    previous_end: usize,
    unit_end: usize,
    failed_unit: &str,
) -> usize {
    let bounded_end = unit_end.min(source.len());
    let bounded_start = previous_end.min(bounded_end);
    source[bounded_start..bounded_end]
        .find(failed_unit)
        .map_or(bounded_start, |relative| bounded_start + relative)
}

fn recovery_at_rule_name(failed_unit: &str) -> Option<&str> {
    let after_at = failed_unit.trim_start().strip_prefix('@')?;
    let name_end = after_at
        .find(|character: char| !character.is_alphanumeric() && character != '-')
        .unwrap_or(after_at.len());
    Some(&after_at[..name_end])
}

pub(super) struct Recovered<T> {
    pub(super) syntax: T,
    pub(super) diagnostics: Vec<crate::CssRecoveryDiagnostic>,
}

pub(super) fn block_item_diagnostic(
    source: &str,
    error: ParseError<'_, Error>,
    failed_unit: &str,
    unit_end: usize,
    action: crate::CssRecoveryAction,
) -> Option<crate::CssRecoveryDiagnostic> {
    let unit_start = unit_end.saturating_sub(failed_unit.len());
    block_item_diagnostic_from_start(source, error, unit_start, unit_end, action)
}

pub(super) fn block_item_diagnostic_from_start(
    source: &str,
    error: ParseError<'_, Error>,
    unit_start: usize,
    unit_end: usize,
    action: crate::CssRecoveryAction,
) -> Option<crate::CssRecoveryDiagnostic> {
    let error = from_parse_error(source, error);
    let span = crate::CssSourceSpan::new(
        crate::CssSourcePosition::from_byte_offset_in(source, unit_start),
        crate::CssSourcePosition::from_byte_offset_in(source, unit_end),
    )?;
    if span.start() == span.end() {
        return None;
    }
    crate::CssRecoveryDiagnostic::new(error, span, action)
}

pub(super) fn is_declaration_recovery_unit(failed_unit: &str) -> bool {
    let mut input = ParserInput::new(failed_unit);
    let mut parser = Parser::new(&mut input);
    matches!(
        parser.next_including_whitespace_and_comments(),
        Ok(Token::Ident(_))
    )
}

struct StrictRuleParser<'s> {
    source: &'s str,
    is_top_level: bool,
    imports_allowed: bool,
    encoding_allowed: bool,
    source_len: usize,
    encoding: Option<CssEncodingDeclaration>,
    diagnostics: Vec<crate::CssRecoveryDiagnostic>,
}

impl<'s> StrictRuleParser<'s> {
    fn top_level(source: &'s str) -> Self {
        Self {
            source,
            is_top_level: true,
            imports_allowed: true,
            encoding_allowed: true,
            source_len: source.len(),
            encoding: None,
            diagnostics: Vec::new(),
        }
    }

    fn nested(source: &'s str) -> Self {
        Self {
            source,
            is_top_level: false,
            imports_allowed: false,
            encoding_allowed: false,
            source_len: usize::MAX,
            encoding: None,
            diagnostics: Vec::new(),
        }
    }

    fn mark_non_import_top_level_rule(&mut self) {
        if self.is_top_level {
            self.imports_allowed = false;
        }
    }
}

enum StrictAtRulePrelude {
    Encoding(String),
    Import(CssImportPrelude),
    Layer(Vec<CssLayerName>),
    FontFace,
    Keyframes(CssKeyframesName),
    Media(CssMediaQueryList),
    Container(CssContainerPrelude),
    Scope(CssScopePrelude),
}

struct CssImportPrelude {
    target: CssImportTarget,
    layer: Option<CssImportLayer>,
    media: Option<CssMediaQueryList>,
}

struct CssContainerPrelude {
    name: Option<CssContainerName>,
    condition: CssContainerCondition,
}

struct CssScopePrelude {
    root: Option<CssScopeSelectorList>,
    limit: Option<CssScopeSelectorList>,
}

impl<'i> AtRuleParser<'i> for StrictRuleParser<'i> {
    type Prelude = StrictAtRulePrelude;
    type AtRule = Vec<CssRule>;
    type Error = Error;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::Prelude, ParseError<'i, Self::Error>> {
        if name.eq_ignore_ascii_case("charset") {
            let encoding_allowed = self.encoding_allowed;
            self.encoding_allowed = false;
            if !encoding_allowed {
                return Err(invalid_encoding_declaration(
                    input.current_source_location(),
                ));
            }

            let prelude_start = input.position();
            let label = input
                .expect_string_cloned()
                .map_err(|error| with_encoding_declaration_context(error.into()))?;
            let authored = input.slice(prelude_start..input.position());
            let quote = authored.trim_start().as_bytes().first().copied();
            if !matches!(quote, Some(b'"')) || label.is_empty() {
                return Err(with_encoding_declaration_context(
                    input.new_unexpected_token_error(cssparser::Token::QuotedString(label)),
                ));
            }
            if !input.is_exhausted() {
                let token = input.next_including_whitespace_and_comments()?.clone();
                return Err(with_encoding_declaration_context(
                    input.new_error(cssparser::BasicParseErrorKind::UnexpectedToken(token)),
                ));
            }
            if input.position().byte_index() == self.source_len {
                return Err(with_encoding_declaration_context(
                    input.new_error(cssparser::BasicParseErrorKind::EndOfInput),
                ));
            }
            return Ok(StrictAtRulePrelude::Encoding(label.to_string()));
        }

        self.encoding_allowed = false;
        match_ignore_ascii_case! { &name,
            "import" => {
                if !self.is_top_level {
                    return Err(invalid_at_rule_placement(
                        input.current_source_location(),
                        "import",
                        "the stylesheet top level",
                    ));
                }
                if !self.imports_allowed {
                    return Err(invalid_at_rule_placement(
                        input.current_source_location(),
                        "import",
                        "before every non-import top-level rule",
                    ));
                }
                let prelude = parse_import_prelude(input).map_err(|error| {
                    with_at_rule_prelude_context(
                        error,
                        "import",
                        "baseline.rule.import",
                        "a supported @import prelude",
                    )
                })?;
                Ok(StrictAtRulePrelude::Import(prelude))
            },
            "font-face" => {
                if !input.is_exhausted() {
                    return Err(with_at_rule_prelude_context(
                        invalid_syntax(
                            input.current_source_location(),
                            "unexpected token after font-face at-rule name",
                        ),
                        "font-face",
                        "baseline.rule.font-face",
                        "an empty @font-face prelude",
                    ));
                }
                Ok(StrictAtRulePrelude::FontFace)
            },
            "layer" => Ok(StrictAtRulePrelude::Layer(
                parse_layer_prelude(input).map_err(|error| {
                    with_at_rule_prelude_context(
                        error,
                        "layer",
                        "baseline.rule.layer-block",
                        "a supported @layer prelude",
                    )
                })?,
            )),
            "keyframes" => {
                let name = parse_keyframes_name(input).map_err(|error| {
                    with_at_rule_prelude_context(
                        error,
                        "keyframes",
                        "baseline.rule.keyframes",
                        "a supported keyframes name",
                    )
                })?;
                if !input.is_exhausted() {
                    return Err(with_at_rule_prelude_context(
                        invalid_syntax(
                            input.current_source_location(),
                            "unexpected token after keyframes name",
                        ),
                        "keyframes",
                        "baseline.rule.keyframes",
                        "the end of the @keyframes prelude",
                    ));
                }
                Ok(StrictAtRulePrelude::Keyframes(name))
            },
            "media" => {
                let query = parse_media_query_list(input)?;
                if !input.is_exhausted() {
                    return Err(crate::error::with_media_query_context(
                        invalid_syntax(
                            input.current_source_location(),
                            "unexpected token after media query list",
                        ),
                        None,
                    ));
                }
                Ok(StrictAtRulePrelude::Media(query))
            },
            "container" => {
                let prelude = parse_container_prelude(input).map_err(|error| {
                    with_at_rule_prelude_context(
                        error,
                        "container",
                        "baseline.rule.container",
                        "a supported @container prelude",
                    )
                })?;
                if !input.is_exhausted() {
                    return Err(with_at_rule_prelude_context(
                        invalid_syntax(
                            input.current_source_location(),
                            "unexpected token after container condition",
                        ),
                        "container",
                        "baseline.rule.container",
                        "the end of the @container prelude",
                    ));
                }
                Ok(StrictAtRulePrelude::Container(prelude))
            },
            "scope" => Ok(StrictAtRulePrelude::Scope(
                parse_scope_prelude(input).map_err(|error| {
                    with_at_rule_prelude_context(
                        error,
                        "scope",
                        "baseline.rule.scope",
                        "a supported @scope prelude",
                    )
                })?,
            )),
            _ => Err(input.new_error(cssparser::BasicParseErrorKind::AtRuleInvalid(name))),
        }
    }

    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
    ) -> std::result::Result<Self::AtRule, ()> {
        match prelude {
            StrictAtRulePrelude::Encoding(label) => {
                self.encoding = Some(CssEncodingDeclaration::new(
                    label,
                    crate::source::CssSourcePosition::from_cssparser(
                        start.position(),
                        start.source_location(),
                    ),
                ));
                Ok(Vec::new())
            }
            StrictAtRulePrelude::Import(prelude) => Ok(vec![CssRule::Import(CssImportRule::new(
                prelude.target,
                prelude.layer,
                prelude.media,
                crate::source::CssSourcePosition::from_cssparser(
                    start.position(),
                    start.source_location(),
                ),
            ))]),
            StrictAtRulePrelude::Layer(names) => {
                let names = CssLayerNameList::try_new(names).ok_or(())?;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::LayerStatement(CssLayerStatementRule::new(
                    names,
                    crate::source::CssSourcePosition::from_cssparser(
                        start.position(),
                        start.source_location(),
                    ),
                ))])
            }
            StrictAtRulePrelude::FontFace => Err(()),
            StrictAtRulePrelude::Keyframes(_) => Err(()),
            StrictAtRulePrelude::Media(_) => Err(()),
            StrictAtRulePrelude::Container(_) => Err(()),
            StrictAtRulePrelude::Scope(_) => Err(()),
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::AtRule, ParseError<'i, Self::Error>> {
        match prelude {
            StrictAtRulePrelude::Encoding(_) => Err(invalid_encoding_declaration(
                input.current_source_location(),
            )),
            StrictAtRulePrelude::Import(_) => Err(invalid_at_rule_block(
                input,
                "import",
                "baseline.rule.import",
                "a semicolon-terminated @import rule",
            )),
            StrictAtRulePrelude::Layer(names) => {
                if names.len() > 1 {
                    return Err(invalid_at_rule_block(
                        input,
                        "layer",
                        "baseline.rule.layer-block",
                        "at most one layer name before a block",
                    ));
                }
                let name = names.into_iter().next();
                let recovered = parse_nested_group_rules(self.source, input)?;
                self.diagnostics.extend(recovered.diagnostics);
                let rules = recovered.syntax;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::LayerBlock(CssLayerBlockRule::new(
                    name,
                    rules,
                    crate::source::CssSourcePosition::from_cssparser(
                        start.position(),
                        start.source_location(),
                    ),
                ))])
            }
            StrictAtRulePrelude::FontFace => {
                let rule = parse_font_face_rule(self.source, input, start, &mut self.diagnostics)?;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::FontFace(rule)])
            }
            StrictAtRulePrelude::Keyframes(name) => {
                let rule = parse_keyframes_rule(name, input, start)?;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::Keyframes(rule)])
            }
            StrictAtRulePrelude::Media(query) => {
                let recovered = parse_nested_group_rules(self.source, input)?;
                self.diagnostics.extend(recovered.diagnostics);
                let rules = recovered.syntax;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::Media(CssMediaRule::new(
                    query,
                    rules,
                    crate::source::CssSourcePosition::from_cssparser(
                        start.position(),
                        start.source_location(),
                    ),
                ))])
            }
            StrictAtRulePrelude::Container(prelude) => {
                let recovered = parse_nested_group_rules(self.source, input)?;
                self.diagnostics.extend(recovered.diagnostics);
                let rules = recovered.syntax;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::Container(CssContainerRule::new(
                    prelude.name,
                    prelude.condition,
                    rules,
                    crate::source::CssSourcePosition::from_cssparser(
                        start.position(),
                        start.source_location(),
                    ),
                ))])
            }
            StrictAtRulePrelude::Scope(prelude) => {
                let recovered = parse_scoped_rule_list(self.source, input)?;
                self.diagnostics.extend(recovered.diagnostics);
                let rules = recovered.syntax;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::Scope(CssScopeRule::new(
                    prelude.root,
                    prelude.limit,
                    rules,
                    crate::source::CssSourcePosition::from_cssparser(
                        start.position(),
                        start.source_location(),
                    ),
                ))])
            }
        }
    }
}

impl<'i> QualifiedRuleParser<'i> for StrictRuleParser<'i> {
    type Prelude = Vec<CssSelector>;
    type QualifiedRule = Vec<CssRule>;
    type Error = Error;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::Prelude, ParseError<'i, Self::Error>> {
        self.encoding_allowed = false;
        parse_rule_selector_list(input)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let recovered = parse_style_rule_block(self.source, selectors, input)?;
        self.diagnostics.extend(recovered.diagnostics);
        let rules = recovered.syntax;
        self.mark_non_import_top_level_rule();
        Ok(rules)
    }
}

impl<'i> DeclarationParser<'i> for StrictRuleParser<'i> {
    type Declaration = Vec<CssRule>;
    type Error = Error;
}

impl<'i> RuleBodyItemParser<'i, Vec<CssRule>, Error> for StrictRuleParser<'i> {
    fn parse_declarations(&self) -> bool {
        false
    }

    fn parse_qualified(&self) -> bool {
        true
    }
}

fn parse_import_prelude<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssImportPrelude, ParseError<'i, Error>> {
    let target = parse_import_target(input)?;
    let layer = parse_import_layer(input)?;
    let media = if input.is_exhausted() {
        None
    } else {
        Some(parse_media_query_list(input)?)
    };

    if !input.is_exhausted() {
        return Err(invalid_syntax(
            input.current_source_location(),
            "unexpected token after import rule",
        ));
    }

    Ok(CssImportPrelude {
        target,
        layer,
        media,
    })
}

fn parse_container_prelude<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssContainerPrelude, ParseError<'i, Error>> {
    let state = input.state();
    let name = if let Ok(name) = input.try_parse(Parser::expect_ident_cloned) {
        if let Some(name) = CssContainerName::try_new(name.to_string()) {
            Some(name)
        } else {
            input.reset(&state);
            None
        }
    } else {
        None
    };
    let condition = parse_container_condition(input)?;

    Ok(CssContainerPrelude { name, condition })
}

fn parse_nested_group_rules<'i, 't>(
    source: &'i str,
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Recovered<Vec<CssRule>>, ParseError<'i, Error>> {
    let mut rule_parser = StrictRuleParser::nested(source);
    let mut rules = Vec::new();
    for rule in StyleSheetParser::new(input, &mut rule_parser) {
        rules.extend(rule.map_err(|(error, _)| error)?);
    }
    Ok(Recovered {
        syntax: rules,
        diagnostics: rule_parser.diagnostics,
    })
}

pub(super) fn parse_scoped_rule_list<'i, 't>(
    source: &'i str,
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Recovered<CssScopedRuleList>, ParseError<'i, Error>> {
    let mut rule_parser = ScopedRuleParser {
        source,
        diagnostics: Vec::new(),
    };
    let mut rules = Vec::new();
    for rule in StyleSheetParser::new(input, &mut rule_parser) {
        rules.extend(rule.map_err(|(error, _)| error)?);
    }
    Ok(Recovered {
        syntax: CssScopedRuleList::from_rules(rules),
        diagnostics: rule_parser.diagnostics,
    })
}

fn parse_import_target<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssImportTarget, ParseError<'i, Error>> {
    let location = input.current_source_location();

    if let Ok(value) = input.try_parse(Parser::expect_string_cloned) {
        return CssImportString::try_new(value.as_ref())
            .map(CssImportTarget::String)
            .ok_or_else(|| invalid_syntax(location, "import string target must not be empty"));
    }

    if let Ok(value) = input.try_parse(Parser::expect_url) {
        return CssImportUrl::try_new(value.as_ref())
            .map(CssImportTarget::Url)
            .ok_or_else(|| invalid_syntax(location, "import URL target must not be empty"));
    }

    Err(invalid_syntax(
        location,
        "expected string or URL import target",
    ))
}

fn parse_import_layer<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Option<CssImportLayer>, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("layer"))
        .is_ok()
    {
        return Ok(Some(CssImportLayer::Anonymous));
    }

    if input
        .try_parse(|input| input.expect_function_matching("layer"))
        .is_ok()
    {
        let layer_name = input.parse_nested_block(parse_import_layer_name)?;
        return Ok(Some(CssImportLayer::Named(layer_name)));
    }

    Ok(None)
}

fn parse_import_layer_name<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLayerName, ParseError<'i, Error>> {
    let name = parse_layer_name(input)?;
    if !input.is_exhausted() {
        return Err(invalid_syntax(
            input.current_source_location(),
            "unexpected token in import layer name",
        ));
    }
    Ok(name)
}

fn parse_layer_prelude<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Vec<CssLayerName>, ParseError<'i, Error>> {
    if input.is_exhausted() {
        return Ok(Vec::new());
    }

    let mut names = Vec::new();
    loop {
        names.push(parse_layer_name(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    if !input.is_exhausted() {
        return Err(invalid_syntax(
            input.current_source_location(),
            "unexpected token after layer name list",
        ));
    }
    Ok(names)
}

fn parse_layer_name<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLayerName, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let mut components = vec![input.expect_ident_cloned().map_err(basic)?.to_string()];

    while input.try_parse(|input| input.expect_delim('.')).is_ok() {
        components.push(input.expect_ident_cloned().map_err(basic)?.to_string());
    }

    CssLayerName::try_new(components).ok_or_else(|| invalid_syntax(location, "invalid layer name"))
}

fn parse_scope_prelude<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssScopePrelude, ParseError<'i, Error>> {
    let root = if input.try_parse(Parser::expect_parenthesis_block).is_ok() {
        Some(input.parse_nested_block(parse_scope_boundary_selector_list)?)
    } else {
        None
    };

    let limit = if input
        .try_parse(|input| input.expect_ident_matching("to"))
        .is_ok()
    {
        input.expect_parenthesis_block().map_err(basic)?;
        Some(input.parse_nested_block(parse_scope_boundary_selector_list)?)
    } else {
        None
    };

    if !input.is_exhausted() {
        return Err(invalid_syntax(
            input.current_source_location(),
            "unexpected token after scope prelude",
        ));
    }

    Ok(CssScopePrelude { root, limit })
}

struct ScopedRuleParser<'s> {
    source: &'s str,
    diagnostics: Vec<crate::CssRecoveryDiagnostic>,
}

enum ScopedAtRulePrelude {
    Media(CssMediaQueryList),
    Container(CssContainerPrelude),
    Layer(Vec<CssLayerName>),
    Scope(CssScopePrelude),
}

impl<'i> AtRuleParser<'i> for ScopedRuleParser<'i> {
    type Prelude = ScopedAtRulePrelude;
    type AtRule = Vec<CssScopedRule>;
    type Error = Error;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::Prelude, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! { &name,
            "media" => {
                let query = parse_media_query_list(input)?;
                if !input.is_exhausted() {
                    return Err(with_media_query_context(
                        invalid_syntax(
                            input.current_source_location(),
                            "unexpected token after media query list",
                        ),
                        None,
                    ));
                }
                Ok(ScopedAtRulePrelude::Media(query))
            },
            "container" => {
                let prelude = parse_container_prelude(input).map_err(|error| {
                    with_at_rule_prelude_context(
                        error,
                        "container",
                        "baseline.rule.container",
                        "a supported @container prelude",
                    )
                })?;
                if !input.is_exhausted() {
                    return Err(with_at_rule_prelude_context(
                        invalid_syntax(
                            input.current_source_location(),
                            "unexpected token after container condition",
                        ),
                        "container",
                        "baseline.rule.container",
                        "the end of the @container prelude",
                    ));
                }
                Ok(ScopedAtRulePrelude::Container(prelude))
            },
            "layer" => Ok(ScopedAtRulePrelude::Layer(
                parse_layer_prelude(input).map_err(|error| {
                    with_at_rule_prelude_context(
                        error,
                        "layer",
                        "baseline.rule.layer-block",
                        "a supported @layer prelude",
                    )
                })?,
            )),
            "scope" => Ok(ScopedAtRulePrelude::Scope(
                parse_scope_prelude(input).map_err(|error| {
                    with_at_rule_prelude_context(
                        error,
                        "scope",
                        "baseline.rule.scope",
                        "a supported @scope prelude",
                    )
                })?,
            )),
            "import" => Err(invalid_at_rule_placement(
                input.current_source_location(),
                "import",
                "the stylesheet top level",
            )),
            "font-face" => Err(invalid_at_rule_placement(
                input.current_source_location(),
                "font-face",
                "a stylesheet or conditional group rule list",
            )),
            "keyframes" => Err(invalid_at_rule_placement(
                input.current_source_location(),
                "keyframes",
                "a stylesheet or conditional group rule list",
            )),
            _ => Err(input.new_error(cssparser::BasicParseErrorKind::AtRuleInvalid(name))),
        }
    }

    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
    ) -> std::result::Result<Self::AtRule, ()> {
        match prelude {
            ScopedAtRulePrelude::Layer(names) => {
                let names = CssLayerNameList::try_new(names).ok_or(())?;
                Ok(vec![CssScopedRule::LayerStatement(
                    CssScopedLayerStatementRule::new(
                        names,
                        crate::source::CssSourcePosition::from_cssparser(
                            start.position(),
                            start.source_location(),
                        ),
                    ),
                )])
            }
            ScopedAtRulePrelude::Media(_)
            | ScopedAtRulePrelude::Container(_)
            | ScopedAtRulePrelude::Scope(_) => Err(()),
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::AtRule, ParseError<'i, Self::Error>> {
        let position = crate::source::CssSourcePosition::from_cssparser(
            start.position(),
            start.source_location(),
        );
        match prelude {
            ScopedAtRulePrelude::Media(query) => {
                let recovered = parse_scoped_rule_list(self.source, input)?;
                self.diagnostics.extend(recovered.diagnostics);
                let rules = recovered.syntax;
                Ok(vec![CssScopedRule::Media(CssScopedMediaRule::new(
                    query, rules, position,
                ))])
            }
            ScopedAtRulePrelude::Container(prelude) => {
                let recovered = parse_scoped_rule_list(self.source, input)?;
                self.diagnostics.extend(recovered.diagnostics);
                let rules = recovered.syntax;
                Ok(vec![CssScopedRule::Container(CssScopedContainerRule::new(
                    prelude.name,
                    prelude.condition,
                    rules,
                    position,
                ))])
            }
            ScopedAtRulePrelude::Layer(names) => {
                if names.len() > 1 {
                    return Err(invalid_at_rule_block(
                        input,
                        "layer",
                        "baseline.rule.layer-block",
                        "at most one layer name before a block",
                    ));
                }
                let name = names.into_iter().next();
                let recovered = parse_scoped_rule_list(self.source, input)?;
                self.diagnostics.extend(recovered.diagnostics);
                let rules = recovered.syntax;
                Ok(vec![CssScopedRule::LayerBlock(
                    CssScopedLayerBlockRule::new(name, rules, position),
                )])
            }
            ScopedAtRulePrelude::Scope(prelude) => {
                let recovered = parse_scoped_rule_list(self.source, input)?;
                self.diagnostics.extend(recovered.diagnostics);
                let rules = recovered.syntax;
                Ok(vec![CssScopedRule::Scope(CssScopeRule::new(
                    prelude.root,
                    prelude.limit,
                    rules,
                    position,
                ))])
            }
        }
    }
}

impl<'i> QualifiedRuleParser<'i> for ScopedRuleParser<'i> {
    type Prelude = CssScopedStyleSelectorList;
    type QualifiedRule = Vec<CssScopedRule>;
    type Error = Error;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::Prelude, ParseError<'i, Self::Error>> {
        parse_scoped_style_selector_list(input)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let recovered = parse_declaration_block(self.source, input)?;
        self.diagnostics.extend(recovered.diagnostics);
        let declarations = recovered.syntax;
        Ok(vec![CssScopedRule::Style(CssScopedStyleRule::new(
            selectors,
            declarations,
        ))])
    }
}

fn parse_declaration_block<'i, 't>(
    source: &'i str,
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Recovered<CssDeclarationList>, ParseError<'i, Error>> {
    let mut declarations = Vec::new();
    let mut diagnostics = Vec::new();
    let mut declaration_parser = StrictDeclarationParser;
    let mut items = RuleBodyParser::new(input, &mut declaration_parser);
    while let Some(item) = items.next() {
        let unit_end = items.input.position().byte_index();
        match item {
            Ok(declaration) => declarations.push(declaration),
            Err((error, failed_unit)) if is_declaration_recovery_unit(failed_unit) => {
                if let Some(diagnostic) = block_item_diagnostic(
                    source,
                    error,
                    failed_unit,
                    unit_end,
                    crate::CssRecoveryAction::DropDeclaration,
                ) {
                    diagnostics.push(diagnostic);
                }
            }
            Err((error, _)) => return Err(error),
        }
    }
    Ok(Recovered {
        syntax: CssDeclarationList::new(declarations),
        diagnostics,
    })
}

struct StrictDeclarationParser;

impl<'i> AtRuleParser<'i> for StrictDeclarationParser {
    type Prelude = ();
    type AtRule = CssDeclaration;
    type Error = Error;
}

impl<'i> QualifiedRuleParser<'i> for StrictDeclarationParser {
    type Prelude = ();
    type QualifiedRule = CssDeclaration;
    type Error = Error;
}

impl<'i> RuleBodyItemParser<'i, CssDeclaration, Error> for StrictDeclarationParser {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

impl<'i> DeclarationParser<'i> for StrictDeclarationParser {
    type Declaration = CssDeclaration;
    type Error = Error;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        declaration_start: &ParserState,
    ) -> std::result::Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let parsed =
            parse_declaration_core(DeclarationMode::Ordinary, name, input, declaration_start)?;
        Ok(CssDeclaration::new_with_importance(
            parsed.body,
            parsed.importance,
            parsed.position,
        ))
    }
}

#[derive(Clone, Copy)]
pub(super) enum DeclarationMode {
    Ordinary,
    Keyframe,
}

pub(super) struct ParsedDeclaration {
    pub(super) body: CssDeclarationBody,
    importance: CssImportance,
    pub(super) position: crate::source::CssSourcePosition,
}

enum DeclarationBoundaryContext {
    OrdinaryKnown(crate::CssKnownProperty),
    OrdinaryCustom(CssCustomPropertyName),
    KeyframeKnown(crate::CssKnownProperty),
    KeyframeCustom(CssCustomPropertyName),
    Descriptor {
        at_rule: &'static str,
        descriptor: &'static str,
    },
}

pub(super) fn parse_declaration_core<'i, 't>(
    mode: DeclarationMode,
    name: CowRcStr<'i>,
    input: &mut Parser<'i, 't>,
    declaration_start: &ParserState,
) -> std::result::Result<ParsedDeclaration, ParseError<'i, Error>> {
    let position = crate::source::CssSourcePosition::from_cssparser(
        declaration_start.position(),
        declaration_start.source_location(),
    );
    if name.starts_with("--") {
        let Some(custom_name) = parse_custom_property_name(name.as_ref()) else {
            return Err(property_name_error(
                declaration_start.source_location(),
                name.as_ref(),
            ));
        };
        let context = match mode {
            DeclarationMode::Ordinary => {
                DeclarationBoundaryContext::OrdinaryCustom(custom_name.clone())
            }
            DeclarationMode::Keyframe => {
                DeclarationBoundaryContext::KeyframeCustom(custom_name.clone())
            }
        };
        let (value, importance) = parse_declaration_boundary(input, &context, |input| {
            parse_custom_property_value(input)
                .map_err(|error| with_property_context(error, name.as_ref()))
        })?;
        return Ok(ParsedDeclaration {
            body: CssDeclarationBody::Custom(CssCustomDeclaration::new(custom_name, value)),
            importance,
            position,
        });
    }

    let known_property = crate::CssKnownProperty::from_name(name.as_ref())
        .ok_or_else(|| property_name_error(declaration_start.source_location(), name.as_ref()))?;
    let context = match mode {
        DeclarationMode::Ordinary => DeclarationBoundaryContext::OrdinaryKnown(known_property),
        DeclarationMode::Keyframe => DeclarationBoundaryContext::KeyframeKnown(known_property),
    };
    let (body, importance) = parse_declaration_boundary(input, &context, |input| {
        parse_known_declaration_body(name.as_ref(), known_property, input)
    })?;
    Ok(ParsedDeclaration {
        body,
        importance,
        position,
    })
}

fn parse_known_declaration_body<'i, 't>(
    name: &str,
    known_property: crate::CssKnownProperty,
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssDeclarationBody, ParseError<'i, Error>> {
    let state = input.state();
    let (authored, has_substitution) = collect_authored_declaration_value(input)
        .map_err(|error| with_property_context(error, name))?;
    if has_substitution {
        return Ok(CssDeclarationBody::Known(
            CssKnownDeclaration::from_substitution_dependent(
                known_property,
                CssSubstitutionDependentValue::new(authored),
            ),
        ));
    }
    input.reset(&state);

    let state = input.state();
    if let Ok(ident) = input.expect_ident_cloned() {
        if let Some(keyword) = parse_global_keyword(&ident) {
            if !input.is_exhausted() {
                return Err(with_property_context(
                    invalid_syntax(
                        input.current_source_location(),
                        "CSS global keyword must be the entire declaration value",
                    ),
                    name,
                ));
            }
            return Ok(CssDeclarationBody::Known(CssKnownDeclaration::from_global(
                known_property,
                keyword,
            )));
        }
        input.reset(&state);
    } else {
        input.reset(&state);
    }

    let declaration = parse_known_property_value(known_property, input)
        .map_err(|error| with_property_context(error, name))?;
    input
        .expect_exhausted()
        .map_err(|error| with_property_context(error.into(), name))?;
    Ok(CssDeclarationBody::Known(declaration))
}

fn parse_declaration_boundary<'i, 't, T>(
    input: &mut Parser<'i, 't>,
    context: &DeclarationBoundaryContext,
    parse_value: impl for<'tt> FnOnce(
        &mut Parser<'i, 'tt>,
    ) -> std::result::Result<T, ParseError<'i, Error>>,
) -> std::result::Result<(T, CssImportance), ParseError<'i, Error>> {
    let value = input.parse_until_before(Delimiter::Bang, parse_value)?;
    if input.is_exhausted() {
        return Ok((value, CssImportance::Normal));
    }

    let bang_location = input.current_source_location();
    let annotation_valid = input.expect_delim('!').is_ok()
        && input.expect_ident_matching("important").is_ok()
        && input.is_exhausted();
    let ordinary = matches!(
        context,
        DeclarationBoundaryContext::OrdinaryKnown(_)
            | DeclarationBoundaryContext::OrdinaryCustom(_)
    );
    if annotation_valid && ordinary {
        Ok((value, CssImportance::Important))
    } else {
        Err(invalid_annotation_for_context(bang_location, context))
    }
}

fn invalid_annotation_for_context<'i>(
    location: cssparser::SourceLocation,
    context: &DeclarationBoundaryContext,
) -> ParseError<'i, Error> {
    match context {
        DeclarationBoundaryContext::OrdinaryKnown(property) => {
            invalid_known_declaration_annotation(location, *property, false)
        }
        DeclarationBoundaryContext::OrdinaryCustom(property) => {
            invalid_custom_declaration_annotation(location, property, false)
        }
        DeclarationBoundaryContext::KeyframeKnown(property) => {
            invalid_known_declaration_annotation(location, *property, true)
        }
        DeclarationBoundaryContext::KeyframeCustom(property) => {
            invalid_custom_declaration_annotation(location, property, true)
        }
        DeclarationBoundaryContext::Descriptor {
            at_rule,
            descriptor,
        } => invalid_descriptor_annotation(location, at_rule, descriptor),
    }
}

pub(super) fn parse_descriptor_boundary<'i, 't, T>(
    input: &mut Parser<'i, 't>,
    at_rule: &'static str,
    descriptor: &'static str,
    parse_value: impl for<'tt> FnOnce(
        &mut Parser<'i, 'tt>,
    ) -> std::result::Result<T, ParseError<'i, Error>>,
) -> std::result::Result<T, ParseError<'i, Error>> {
    let context = DeclarationBoundaryContext::Descriptor {
        at_rule,
        descriptor,
    };
    parse_declaration_boundary(input, &context, parse_value).map(|(value, _)| value)
}

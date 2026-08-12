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
    ParserState, QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser,
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
    Error, Result, basic, from_rule_parse_error, invalid_at_rule_block, invalid_at_rule_placement,
    invalid_custom_declaration_annotation, invalid_descriptor_annotation,
    invalid_known_declaration_annotation, invalid_syntax, property_name_error, unsupported_value,
    with_at_rule_prelude_context, with_media_query_context, with_property_context,
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

pub fn parse_sheet(source: &str) -> Result<CssSheet> {
    let mut input = ParserInput::new(source);
    let mut parser = Parser::new(&mut input);
    let mut rule_parser = StrictRuleParser::top_level();
    let mut sheet = CssSheet::new();

    for rule in StyleSheetParser::new(&mut parser, &mut rule_parser) {
        for rule in
            rule.map_err(|(error, failed_unit)| from_rule_parse_error(source, failed_unit, error))?
        {
            sheet.push_rule(rule);
        }
    }

    Ok(sheet)
}

struct StrictRuleParser {
    is_top_level: bool,
    imports_allowed: bool,
}

impl StrictRuleParser {
    const fn top_level() -> Self {
        Self {
            is_top_level: true,
            imports_allowed: true,
        }
    }

    const fn nested() -> Self {
        Self {
            is_top_level: false,
            imports_allowed: false,
        }
    }

    fn mark_non_import_top_level_rule(&mut self) {
        if self.is_top_level {
            self.imports_allowed = false;
        }
    }
}

enum StrictAtRulePrelude {
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

impl<'i> AtRuleParser<'i> for StrictRuleParser {
    type Prelude = StrictAtRulePrelude;
    type AtRule = Vec<CssRule>;
    type Error = Error;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::Prelude, ParseError<'i, Self::Error>> {
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
                let rules = parse_nested_group_rules(input)?;
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
                let rule = parse_font_face_rule(input, start)?;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::FontFace(rule)])
            }
            StrictAtRulePrelude::Keyframes(name) => {
                let rule = parse_keyframes_rule(name, input, start)?;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::Keyframes(rule)])
            }
            StrictAtRulePrelude::Media(query) => {
                let rules = parse_nested_group_rules(input)?;
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
                let rules = parse_nested_group_rules(input)?;
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
                let rules = parse_scoped_rule_list(input)?;
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

impl<'i> QualifiedRuleParser<'i> for StrictRuleParser {
    type Prelude = Vec<CssSelector>;
    type QualifiedRule = Vec<CssRule>;
    type Error = Error;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::Prelude, ParseError<'i, Self::Error>> {
        parse_rule_selector_list(input)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let rules = parse_style_rule_block(selectors, input)?;
        self.mark_non_import_top_level_rule();
        Ok(rules)
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
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Vec<CssRule>, ParseError<'i, Error>> {
    let mut rule_parser = StrictRuleParser::nested();
    let mut rules = Vec::new();
    for rule in StyleSheetParser::new(input, &mut rule_parser) {
        rules.extend(rule.map_err(|(error, _)| error)?);
    }
    Ok(rules)
}

fn parse_scoped_rule_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssScopedRuleList, ParseError<'i, Error>> {
    let mut rule_parser = ScopedRuleParser;
    let mut rules = Vec::new();
    for rule in StyleSheetParser::new(input, &mut rule_parser) {
        rules.extend(rule.map_err(|(error, _)| error)?);
    }
    Ok(CssScopedRuleList::from_rules(rules))
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

struct ScopedRuleParser;

enum ScopedAtRulePrelude {
    Media(CssMediaQueryList),
    Container(CssContainerPrelude),
    Layer(Vec<CssLayerName>),
    Scope(CssScopePrelude),
}

impl<'i> AtRuleParser<'i> for ScopedRuleParser {
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
                let rules = parse_scoped_rule_list(input)?;
                Ok(vec![CssScopedRule::Media(CssScopedMediaRule::new(
                    query, rules, position,
                ))])
            }
            ScopedAtRulePrelude::Container(prelude) => {
                let rules = parse_scoped_rule_list(input)?;
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
                let rules = parse_scoped_rule_list(input)?;
                Ok(vec![CssScopedRule::LayerBlock(
                    CssScopedLayerBlockRule::new(name, rules, position),
                )])
            }
            ScopedAtRulePrelude::Scope(prelude) => {
                let rules = parse_scoped_rule_list(input)?;
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

impl<'i> QualifiedRuleParser<'i> for ScopedRuleParser {
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
        let declarations = parse_declaration_block(input)?;
        Ok(vec![CssScopedRule::Style(CssScopedStyleRule::new(
            selectors,
            declarations,
        ))])
    }
}

fn parse_declaration_block<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssDeclarationList, ParseError<'i, Error>> {
    let mut declarations = Vec::new();
    let mut declaration_parser = StrictDeclarationParser;
    for declaration in RuleBodyParser::new(input, &mut declaration_parser) {
        declarations.push(declaration.map_err(|(error, _)| error)?);
    }
    Ok(CssDeclarationList::new(declarations))
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

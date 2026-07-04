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
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserInput, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser,
    match_ignore_ascii_case,
};

use background::*;
use box_model::*;
use effects::*;
use font_face::parse_font_face_rule;
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
    Error, Result, basic, from_parse_error, invalid_syntax, property_name_error, unsupported_value,
    with_property_context,
};
use crate::syntax::*;
use crate::validation::{PropertyNameStatus, classify_property_name, parse_global_keyword};

pub(crate) use crate::validation::property_for_supported_name;

pub fn parse_sheet(input: &str) -> Result<CssSheet> {
    let mut input = ParserInput::new(input);
    let mut parser = Parser::new(&mut input);
    let mut rule_parser = StrictRuleParser::top_level();
    let mut sheet = CssSheet::new();

    for rule in StyleSheetParser::new(&mut parser, &mut rule_parser) {
        for rule in rule.map_err(|(error, _)| from_parse_error(error))? {
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
                    return Err(invalid_syntax(
                        input.current_source_location(),
                        "@import rules are only supported at the top level",
                    ));
                }
                if !self.imports_allowed {
                    return Err(invalid_syntax(
                        input.current_source_location(),
                        "@import rules must precede all non-import top-level rules",
                    ));
                }
                Ok(StrictAtRulePrelude::Import(parse_import_prelude(input)?))
            },
            "font-face" => {
                if !input.is_exhausted() {
                    return Err(invalid_syntax(
                        input.current_source_location(),
                        "unexpected token after font-face at-rule name",
                    ));
                }
                Ok(StrictAtRulePrelude::FontFace)
            },
            "layer" => Ok(StrictAtRulePrelude::Layer(parse_layer_prelude(input)?)),
            "keyframes" => {
                let name = parse_keyframes_name(input)?;
                if !input.is_exhausted() {
                    return Err(invalid_syntax(
                        input.current_source_location(),
                        "unexpected token after keyframes name",
                    ));
                }
                Ok(StrictAtRulePrelude::Keyframes(name))
            },
            "media" => {
                let query = parse_media_query_list(input)?;
                if !input.is_exhausted() {
                    return Err(invalid_syntax(
                        input.current_source_location(),
                        "unexpected token after media query list",
                    ));
                }
                Ok(StrictAtRulePrelude::Media(query))
            },
            "container" => {
                let prelude = parse_container_prelude(input)?;
                if !input.is_exhausted() {
                    return Err(invalid_syntax(
                        input.current_source_location(),
                        "unexpected token after container condition",
                    ));
                }
                Ok(StrictAtRulePrelude::Container(prelude))
            },
            "scope" => Ok(StrictAtRulePrelude::Scope(parse_scope_prelude(input)?)),
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
                CssSourceLocation::from_cssparser(start.source_location()),
            ))]),
            StrictAtRulePrelude::Layer(names) => {
                let names = CssLayerNameList::try_new(names).ok_or(())?;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::LayerStatement(CssLayerStatementRule::new(
                    names,
                    CssSourceLocation::from_cssparser(start.source_location()),
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
            StrictAtRulePrelude::Import(_) => Err(invalid_syntax(
                start.source_location(),
                "@import rules must not have a block",
            )),
            StrictAtRulePrelude::Layer(names) => {
                if names.len() > 1 {
                    return Err(invalid_syntax(
                        start.source_location(),
                        "@layer block rules accept at most one layer name",
                    ));
                }
                let name = names.into_iter().next();
                let rules = parse_nested_group_rules(input)?;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::LayerBlock(CssLayerBlockRule::new(
                    name,
                    rules,
                    CssSourceLocation::from_cssparser(start.source_location()),
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
                    CssSourceLocation::from_cssparser(start.source_location()),
                ))])
            }
            StrictAtRulePrelude::Container(prelude) => {
                let rules = parse_nested_group_rules(input)?;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::Container(CssContainerRule::new(
                    prelude.name,
                    prelude.condition,
                    rules,
                    CssSourceLocation::from_cssparser(start.source_location()),
                ))])
            }
            StrictAtRulePrelude::Scope(prelude) => {
                let rules = parse_scoped_rule_list(input)?;
                self.mark_non_import_top_level_rule();
                Ok(vec![CssRule::Scope(CssScopeRule::new(
                    prelude.root,
                    prelude.limit,
                    rules,
                    CssSourceLocation::from_cssparser(start.source_location()),
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
                    return Err(invalid_syntax(
                        input.current_source_location(),
                        "unexpected token after media query list",
                    ));
                }
                Ok(ScopedAtRulePrelude::Media(query))
            },
            "container" => {
                let prelude = parse_container_prelude(input)?;
                if !input.is_exhausted() {
                    return Err(invalid_syntax(
                        input.current_source_location(),
                        "unexpected token after container condition",
                    ));
                }
                Ok(ScopedAtRulePrelude::Container(prelude))
            },
            "layer" => Ok(ScopedAtRulePrelude::Layer(parse_layer_prelude(input)?)),
            "scope" => Ok(ScopedAtRulePrelude::Scope(parse_scope_prelude(input)?)),
            "import" => Err(invalid_syntax(
                input.current_source_location(),
                "@import rules are not supported inside scope blocks",
            )),
            "font-face" => Err(invalid_syntax(
                input.current_source_location(),
                "@font-face rules are not supported inside scope blocks",
            )),
            "keyframes" => Err(invalid_syntax(
                input.current_source_location(),
                "@keyframes rules are not supported inside scope blocks",
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
                        CssSourceLocation::from_cssparser(start.source_location()),
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
        let location = CssSourceLocation::from_cssparser(start.source_location());
        match prelude {
            ScopedAtRulePrelude::Media(query) => {
                let rules = parse_scoped_rule_list(input)?;
                Ok(vec![CssScopedRule::Media(CssScopedMediaRule::new(
                    query, rules, location,
                ))])
            }
            ScopedAtRulePrelude::Container(prelude) => {
                let rules = parse_scoped_rule_list(input)?;
                Ok(vec![CssScopedRule::Container(CssScopedContainerRule::new(
                    prelude.name,
                    prelude.condition,
                    rules,
                    location,
                ))])
            }
            ScopedAtRulePrelude::Layer(names) => {
                if names.len() > 1 {
                    return Err(invalid_syntax(
                        start.source_location(),
                        "@layer block rules accept at most one layer name",
                    ));
                }
                let name = names.into_iter().next();
                let rules = parse_scoped_rule_list(input)?;
                Ok(vec![CssScopedRule::LayerBlock(
                    CssScopedLayerBlockRule::new(name, rules, location),
                )])
            }
            ScopedAtRulePrelude::Scope(prelude) => {
                let rules = parse_scoped_rule_list(input)?;
                Ok(vec![CssScopedRule::Scope(CssScopeRule::new(
                    prelude.root,
                    prelude.limit,
                    rules,
                    location,
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
) -> std::result::Result<Vec<CssDeclaration>, ParseError<'i, Error>> {
    let mut declarations = Vec::new();
    let mut declaration_parser = StrictDeclarationParser;
    for declaration in RuleBodyParser::new(input, &mut declaration_parser) {
        declarations.push(declaration.map_err(|(error, _)| error)?);
    }
    Ok(declarations)
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
        let location = CssSourceLocation::from_cssparser(declaration_start.source_location());
        if name.starts_with("--") {
            let Some(custom_name) = parse_custom_property_name(name.as_ref()) else {
                return Err(property_name_error(input, name.as_ref()));
            };
            let value = parse_custom_property_value(input)
                .map_err(|error| with_property_context(error, name.as_ref()))?;
            return Ok(CssDeclaration::new(
                CssProperty::Custom(custom_name),
                value,
                location,
            ));
        }

        if let Some(supported_property) = property_for_supported_name(name.as_ref()) {
            let state = input.state();
            let (authored, references) = collect_authored_declaration_value(input)
                .map_err(|error| with_property_context(error, name.as_ref()))?;
            if !references.is_empty() {
                return Ok(CssDeclaration::new(
                    supported_property,
                    CssValue::VariableDependent(CssVariableDependentValue::new(
                        authored, references,
                    )),
                    location,
                ));
            }
            input.reset(&state);
        }

        let state = input.state();
        if let Ok(ident) = input.expect_ident_cloned() {
            if let Some(keyword) = parse_global_keyword(&ident) {
                match classify_property_name(name.as_ref()) {
                    PropertyNameStatus::Supported => {
                        if !input.is_exhausted() {
                            return Err(invalid_syntax(
                                input.current_source_location(),
                                "CSS global keyword must be the entire declaration value",
                            ));
                        }
                        return Ok(CssDeclaration::new(
                            property_for_supported_name(name.as_ref())
                                .expect("supported property has CssProperty"),
                            CssValue::GlobalKeyword(keyword),
                            location,
                        ));
                    }
                    PropertyNameStatus::KnownUnsupported | PropertyNameStatus::Unknown => {
                        input.reset(&state);
                        return Err(property_name_error(input, name.as_ref()));
                    }
                }
            }
            input.reset(&state);
        } else {
            input.reset(&state);
        }
        let result = (|| {
            Ok(match_ignore_ascii_case! { &name,
            "all" => return Err(unsupported_value(input, None, "`all` only accepts CSS-wide global keywords")),
            "display" => (CssProperty::Display, CssValue::Display(parse_display(input)?)),
            "box-sizing" => (CssProperty::BoxSizing, CssValue::BoxSizing(parse_box_sizing(input)?)),
            "position" => (CssProperty::Position, CssValue::Position(parse_position(input)?)),
            "direction" => (CssProperty::Direction, CssValue::Direction(parse_direction(input)?)),
            "overflow" => (CssProperty::Overflow, parse_overflow_value(input)?),
            "overflow-x" => (CssProperty::OverflowX, CssValue::Overflow(parse_overflow(input)?)),
            "overflow-y" => (CssProperty::OverflowY, CssValue::Overflow(parse_overflow(input)?)),
            "flex-direction" => (CssProperty::FlexDirection, CssValue::FlexDirection(parse_flex_direction(input)?)),
            "flex-wrap" => (CssProperty::FlexWrap, CssValue::FlexWrap(parse_flex_wrap(input)?)),
            "float" => (CssProperty::Float, CssValue::Float(parse_float(input)?)),
            "clear" => (CssProperty::Clear, CssValue::Clear(parse_clear(input)?)),
            "align-content" => (CssProperty::AlignContent, CssValue::Alignment(parse_content_alignment(input)?)),
            "justify-content" => (CssProperty::JustifyContent, CssValue::Alignment(parse_content_alignment(input)?)),
            "align-items" => (CssProperty::AlignItems, CssValue::AlignItems(parse_align_items(input)?)),
            "align-self" => (CssProperty::AlignSelf, CssValue::AlignItems(parse_align_items(input)?)),
            "justify-items" => (CssProperty::JustifyItems, CssValue::AlignItems(parse_align_items(input)?)),
            "justify-self" => (CssProperty::JustifySelf, CssValue::AlignItems(parse_align_items(input)?)),
            "place-content" => (CssProperty::PlaceContent, CssValue::PlaceAlignment(parse_place_alignment(input, parse_content_alignment, CssPlaceAlignment::content)?)),
            "place-items" => (CssProperty::PlaceItems, CssValue::PlaceAlignment(parse_place_alignment(input, parse_align_items, CssPlaceAlignment::items)?)),
            "place-self" => (CssProperty::PlaceSelf, CssValue::PlaceAlignment(parse_place_alignment(input, parse_align_items, CssPlaceAlignment::items)?)),
            "visibility" => (CssProperty::Visibility, CssValue::Visibility(parse_visibility(input)?)),
            "content-visibility" => (CssProperty::ContentVisibility, CssValue::ContentVisibility(parse_content_visibility(input)?)),
            "width" => (CssProperty::Width, CssValue::Length(parse_box_size_value(input)?)),
            "height" => (CssProperty::Height, CssValue::Length(parse_box_size_value(input)?)),
            "min-width" => (CssProperty::MinWidth, CssValue::Length(parse_box_size_value(input)?)),
            "min-height" => (CssProperty::MinHeight, CssValue::Length(parse_box_size_value(input)?)),
            "max-width" => (CssProperty::MaxWidth, CssValue::Length(parse_box_size_value(input)?)),
            "max-height" => (CssProperty::MaxHeight, CssValue::Length(parse_box_size_value(input)?)),
            "flex-basis" => (CssProperty::FlexBasis, CssValue::Length(parse_box_size_value(input)?)),
            "gap" => (CssProperty::Gap, CssValue::Length(parse_gap_value(input)?)),
            "row-gap" => (CssProperty::RowGap, CssValue::Length(parse_gap_value(input)?)),
            "column-gap" => (CssProperty::ColumnGap, CssValue::Length(parse_gap_value(input)?)),
            "grid-flow-tolerance" => (CssProperty::GridFlowTolerance, CssValue::GridFlowTolerance(parse_grid_flow_tolerance(input)?)),
            "grid-template-rows" => (CssProperty::GridTemplateRows, CssValue::GridTrackList(parse_grid_track_list(input)?)),
            "grid-template-columns" => (CssProperty::GridTemplateColumns, CssValue::GridTrackList(parse_grid_track_list(input)?)),
            "grid-template-areas" => (CssProperty::GridTemplateAreas, CssValue::GridTemplateAreas(parse_grid_template_areas(input)?)),
            "grid-template" => (CssProperty::GridTemplate, CssValue::GridTemplate(parse_grid_template(input)?)),
            "grid-auto-rows" => (CssProperty::GridAutoRows, CssValue::GridTrackList(parse_grid_track_list(input)?)),
            "grid-auto-columns" => (CssProperty::GridAutoColumns, CssValue::GridTrackList(parse_grid_track_list(input)?)),
            "grid-auto-flow" => (CssProperty::GridAutoFlow, CssValue::GridAutoFlow(parse_grid_auto_flow(input)?)),
            "grid-row-start" => (CssProperty::GridRowStart, CssValue::GridLine(parse_grid_line(input)?)),
            "grid-row-end" => (CssProperty::GridRowEnd, CssValue::GridLine(parse_grid_line(input)?)),
            "grid-column-start" => (CssProperty::GridColumnStart, CssValue::GridLine(parse_grid_line(input)?)),
            "grid-column-end" => (CssProperty::GridColumnEnd, CssValue::GridLine(parse_grid_line(input)?)),
            "grid-row" => (CssProperty::GridRow, CssValue::GridLineRange(parse_grid_line_range(input)?)),
            "grid-column" => (CssProperty::GridColumn, CssValue::GridLineRange(parse_grid_line_range(input)?)),
            "grid-area" => (CssProperty::GridArea, CssValue::GridArea(parse_grid_area(input)?)),
            "grid" => (CssProperty::Grid, CssValue::Grid(parse_grid(input)?)),
            "font-size" => (CssProperty::FontSize, CssValue::Length(parse_font_size(input)?)),
            "line-height" => (CssProperty::LineHeight, CssValue::Length(parse_line_height(input)?)),
            "writing-mode" => (CssProperty::WritingMode, CssValue::WritingMode(parse_writing_mode(input)?)),
            "text-align" => (CssProperty::TextAlign, CssValue::TextAlign(parse_text_align(input)?)),
            "text-align-last" => (CssProperty::TextAlignLast, CssValue::TextAlignLast(parse_text_align_last(input)?)),
            "text-indent" => (CssProperty::TextIndent, CssValue::TextIndent(parse_text_indent(input)?)),
            "vertical-align" => (CssProperty::VerticalAlign, CssValue::VerticalAlign(parse_vertical_align(input)?)),
            "font-family" => (CssProperty::FontFamily, CssValue::FontFamily(parse_font_family_list(input)?)),
            "font" => (CssProperty::Font, CssValue::Font(parse_font(input)?)),
            "font-weight" => (CssProperty::FontWeight, CssValue::FontWeight(parse_font_weight(input)?)),
            "font-style" => (CssProperty::FontStyle, CssValue::FontStyle(parse_font_style(input)?)),
            "font-stretch" => (CssProperty::FontStretch, CssValue::FontStretch(parse_font_stretch(input)?)),
            "font-variant" => (CssProperty::FontVariant, CssValue::FontVariant(parse_font_variant(input)?)),
            "font-feature-settings" => (CssProperty::FontFeatureSettings, CssValue::FontFeatureSettings(parse_font_feature_settings(input)?)),
            "letter-spacing" => (CssProperty::LetterSpacing, CssValue::LetterSpacing(parse_letter_spacing(input)?)),
            "text-wrap" => (CssProperty::TextWrap, CssValue::TextWrap(parse_text_wrap(input)?)),
            "white-space" => (CssProperty::WhiteSpace, CssValue::WhiteSpace(parse_white_space(input)?)),
            "word-break" => (CssProperty::WordBreak, CssValue::WordBreak(parse_word_break(input)?)),
            "overflow-wrap" => (CssProperty::OverflowWrap, CssValue::OverflowWrap(parse_overflow_wrap(input)?)),
            "text-overflow" => (CssProperty::TextOverflow, CssValue::TextOverflow(parse_text_overflow(input)?)),
            "text-decoration" => (CssProperty::TextDecoration, CssValue::TextDecoration(parse_text_decoration(input)?)),
            "text-decoration-line" => (CssProperty::TextDecorationLine, CssValue::TextDecorationLine(parse_text_decoration_line(input)?)),
            "text-decoration-color" => (CssProperty::TextDecorationColor, CssValue::TextDecorationColor(parse_color(input)?)),
            "text-decoration-style" => (CssProperty::TextDecorationStyle, CssValue::TextDecorationStyle(parse_text_decoration_style(input)?)),
            "text-decoration-thickness" => (CssProperty::TextDecorationThickness, CssValue::TextDecorationThickness(parse_text_decoration_thickness(input)?)),
            "text-transform" => (CssProperty::TextTransform, CssValue::TextTransform(parse_text_transform(input)?)),
            "inset" => (CssProperty::Inset, CssValue::Edges(parse_edges(input, parse_inset_component)?)),
            "top" => (CssProperty::Top, CssValue::Length(parse_inset_component(input)?)),
            "right" => (CssProperty::Right, CssValue::Length(parse_inset_component(input)?)),
            "bottom" => (CssProperty::Bottom, CssValue::Length(parse_inset_component(input)?)),
            "left" => (CssProperty::Left, CssValue::Length(parse_inset_component(input)?)),
            "z-index" => (CssProperty::ZIndex, CssValue::ZIndex(parse_z_index(input)?)),
            "box-decoration-break" => (CssProperty::BoxDecorationBreak, CssValue::BoxDecorationBreak(parse_box_decoration_break(input)?)),
            "margin" => (CssProperty::Margin, CssValue::Edges(parse_edges(input, parse_margin_component)?)),
            "margin-top" => (CssProperty::MarginTop, CssValue::Length(parse_margin_component(input)?)),
            "margin-right" => (CssProperty::MarginRight, CssValue::Length(parse_margin_component(input)?)),
            "margin-bottom" => (CssProperty::MarginBottom, CssValue::Length(parse_margin_component(input)?)),
            "margin-left" => (CssProperty::MarginLeft, CssValue::Length(parse_margin_component(input)?)),
            "padding" => (CssProperty::Padding, CssValue::Edges(parse_edges(input, parse_padding_component)?)),
            "padding-top" => (CssProperty::PaddingTop, CssValue::Length(parse_padding_component(input)?)),
            "padding-right" => (CssProperty::PaddingRight, CssValue::Length(parse_padding_component(input)?)),
            "padding-bottom" => (CssProperty::PaddingBottom, CssValue::Length(parse_padding_component(input)?)),
            "padding-left" => (CssProperty::PaddingLeft, CssValue::Length(parse_padding_component(input)?)),
            "border" => (CssProperty::Border, CssValue::Border(parse_border(input)?)),
            "border-top" => (CssProperty::BorderTop, CssValue::Border(parse_border(input)?)),
            "border-right" => (CssProperty::BorderRight, CssValue::Border(parse_border(input)?)),
            "border-bottom" => (CssProperty::BorderBottom, CssValue::Border(parse_border(input)?)),
            "border-left" => (CssProperty::BorderLeft, CssValue::Border(parse_border(input)?)),
            "border-width" => (CssProperty::BorderWidth, CssValue::Edges(parse_edges(input, parse_border_width_component)?)),
            "border-top-width" => (CssProperty::BorderTopWidth, CssValue::Length(parse_border_width_component(input)?)),
            "border-right-width" => (CssProperty::BorderRightWidth, CssValue::Length(parse_border_width_component(input)?)),
            "border-bottom-width" => (CssProperty::BorderBottomWidth, CssValue::Length(parse_border_width_component(input)?)),
            "border-left-width" => (CssProperty::BorderLeftWidth, CssValue::Length(parse_border_width_component(input)?)),
            "color" => (CssProperty::Color, CssValue::Color(parse_color(input)?)),
            "background" => (CssProperty::Background, CssValue::Color(parse_color(input)?)),
            "background-color" => (CssProperty::BackgroundColor, CssValue::Color(parse_color(input)?)),
            "border-color" => (CssProperty::BorderColor, CssValue::Color(parse_color(input)?)),
            "border-top-color" => (CssProperty::BorderTopColor, CssValue::Color(parse_color(input)?)),
            "border-right-color" => (CssProperty::BorderRightColor, CssValue::Color(parse_color(input)?)),
            "border-bottom-color" => (CssProperty::BorderBottomColor, CssValue::Color(parse_color(input)?)),
            "border-left-color" => (CssProperty::BorderLeftColor, CssValue::Color(parse_color(input)?)),
            "background-image" => (CssProperty::BackgroundImage, CssValue::BackgroundImage(parse_image_layer_list(input)?)),
            "background-position" => (CssProperty::BackgroundPosition, CssValue::BackgroundPosition(parse_position_list(input)?)),
            "background-size" => (CssProperty::BackgroundSize, CssValue::BackgroundSize(parse_background_size_list(input)?)),
            "background-repeat" => (CssProperty::BackgroundRepeat, CssValue::BackgroundRepeat(parse_background_repeat_list(input)?)),
            "background-origin" => (CssProperty::BackgroundOrigin, CssValue::BackgroundBox(parse_background_box(input)?)),
            "background-clip" => (CssProperty::BackgroundClip, CssValue::BackgroundBox(parse_background_box(input)?)),
            "background-attachment" => (CssProperty::BackgroundAttachment, CssValue::BackgroundAttachment(parse_background_attachment_list(input)?)),
            "border-style" => (CssProperty::BorderStyle, CssValue::BorderStyles(parse_border_styles(input)?)),
            "border-top-style" => (CssProperty::BorderTopStyle, CssValue::BorderStyle(parse_border_style(input)?)),
            "border-right-style" => (CssProperty::BorderRightStyle, CssValue::BorderStyle(parse_border_style(input)?)),
            "border-bottom-style" => (CssProperty::BorderBottomStyle, CssValue::BorderStyle(parse_border_style(input)?)),
            "border-left-style" => (CssProperty::BorderLeftStyle, CssValue::BorderStyle(parse_border_style(input)?)),
            "border-radius" => (CssProperty::BorderRadius, CssValue::BorderRadius(parse_border_radius(input)?)),
            "border-top-left-radius" => (CssProperty::BorderTopLeftRadius, CssValue::CornerRadius(parse_corner_radius(input)?)),
            "border-top-right-radius" => (CssProperty::BorderTopRightRadius, CssValue::CornerRadius(parse_corner_radius(input)?)),
            "border-bottom-right-radius" => (CssProperty::BorderBottomRightRadius, CssValue::CornerRadius(parse_corner_radius(input)?)),
            "border-bottom-left-radius" => (CssProperty::BorderBottomLeftRadius, CssValue::CornerRadius(parse_corner_radius(input)?)),
            "box-shadow" => (CssProperty::BoxShadow, CssValue::BoxShadow(parse_box_shadow(input)?)),
            "opacity" => (CssProperty::Opacity, CssValue::Opacity(parse_opacity(input)?)),
            "flex-grow" => (CssProperty::FlexGrow, CssValue::FlexGrow(parse_flex_factor(input, "flex-grow")?)),
            "flex-shrink" => (CssProperty::FlexShrink, CssValue::FlexShrink(parse_flex_factor(input, "flex-shrink")?)),
            "order" => (CssProperty::Order, CssValue::Order(parse_order(input)?)),
            "flex" => (CssProperty::Flex, CssValue::Flex(parse_flex(input)?)),
            "justify-tracks" => (CssProperty::JustifyTracks, CssValue::Alignment(parse_content_alignment(input)?)),
            "align-tracks" => (CssProperty::AlignTracks, CssValue::Alignment(parse_content_alignment(input)?)),
            "aspect-ratio" => (CssProperty::AspectRatio, CssValue::AspectRatio(parse_aspect_ratio(input)?)),
            "scrollbar-width" => (CssProperty::ScrollbarWidth, CssValue::ScrollbarWidth(parse_scrollbar_width(input)?)),
            "cursor" => (CssProperty::Cursor, CssValue::Cursor(parse_cursor(input)?)),
            "pointer-events" => (CssProperty::PointerEvents, CssValue::PointerEvents(parse_pointer_events(input)?)),
            "user-select" => (CssProperty::UserSelect, CssValue::UserSelect(parse_user_select(input)?)),
            "outline" => (CssProperty::Outline, CssValue::Outline(parse_outline(input)?)),
            "outline-color" => (CssProperty::OutlineColor, CssValue::OutlineColor(parse_color(input)?)),
            "outline-style" => (CssProperty::OutlineStyle, CssValue::OutlineStyle(parse_outline_style(input)?)),
            "outline-width" => (CssProperty::OutlineWidth, CssValue::OutlineWidth(parse_outline_width(input)?)),
            "transform" => (CssProperty::Transform, CssValue::Transform(parse_transform(input)?)),
            "transform-origin" => (CssProperty::TransformOrigin, CssValue::TransformOrigin(parse_css_position(input)?)),
            "translate" => (CssProperty::Translate, CssValue::Translate(parse_translate(input)?)),
            "rotate" => (CssProperty::Rotate, CssValue::Rotate(parse_rotate(input)?)),
            "scale" => (CssProperty::Scale, CssValue::Scale(parse_scale(input)?)),
            "filter" => (CssProperty::Filter, CssValue::Filter(parse_filter(input)?)),
            "backdrop-filter" => (CssProperty::BackdropFilter, CssValue::Filter(parse_filter(input)?)),
            "clip-path" => (CssProperty::ClipPath, CssValue::ClipPath(parse_clip_path(input)?)),
            "mask" => (CssProperty::Mask, CssValue::Mask(parse_mask_list(input)?)),
            "mask-image" => (CssProperty::MaskImage, CssValue::MaskImage(parse_image_layer_list(input)?)),
            "mask-size" => (CssProperty::MaskSize, CssValue::MaskSize(parse_background_size_list(input)?)),
            "mask-position" => (CssProperty::MaskPosition, CssValue::MaskPosition(parse_position_list(input)?)),
            "mask-repeat" => (CssProperty::MaskRepeat, CssValue::MaskRepeat(parse_background_repeat_list(input)?)),
            "transition-property" => (CssProperty::TransitionProperty, CssValue::TransitionProperty(parse_transition_property_list(input)?)),
            "transition-duration" => (CssProperty::TransitionDuration, CssValue::TimeList(parse_time_list(input)?)),
            "transition-delay" => (CssProperty::TransitionDelay, CssValue::TimeList(parse_time_list(input)?)),
            "transition-timing-function" => (CssProperty::TransitionTimingFunction, CssValue::EasingList(parse_easing_list(input)?)),
            "transition" => (CssProperty::Transition, CssValue::Transition(parse_transition_list(input)?)),
            "animation-name" => (CssProperty::AnimationName, CssValue::AnimationName(parse_animation_name_list(input)?)),
            "animation-duration" => (CssProperty::AnimationDuration, CssValue::TimeList(parse_time_list(input)?)),
            "animation-delay" => (CssProperty::AnimationDelay, CssValue::TimeList(parse_time_list(input)?)),
            "animation-timing-function" => (CssProperty::AnimationTimingFunction, CssValue::EasingList(parse_easing_list(input)?)),
            "animation-iteration-count" => (CssProperty::AnimationIterationCount, CssValue::AnimationIterationCount(parse_animation_iteration_count_list(input)?)),
            "animation-direction" => (CssProperty::AnimationDirection, CssValue::AnimationDirection(parse_animation_direction_list(input)?)),
            "animation-fill-mode" => (CssProperty::AnimationFillMode, CssValue::AnimationFillMode(parse_animation_fill_mode_list(input)?)),
            "animation-play-state" => (CssProperty::AnimationPlayState, CssValue::AnimationPlayState(parse_animation_play_state_list(input)?)),
            "animation" => (CssProperty::Animation, CssValue::Animation(parse_animation_list(input)?)),
            _ => return Err(property_name_error(input, name.as_ref())),
            })
        })()
        .map_err(|error| with_property_context(error, name.as_ref()))?;
        input.expect_exhausted().map_err(basic)?;
        let (property, value) = result;
        Ok(CssDeclaration::new(property, value, location))
    }
}

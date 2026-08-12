use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, StyleSheetParser, Token,
    match_ignore_ascii_case,
};

use super::values::parse_custom_ident_from_str_at;
use super::{DeclarationMode, parse_declaration_core};
use crate::error::{
    Error, basic, invalid_at_rule_body, invalid_qualified_rule, unsupported_value,
    unsupported_value_at,
};
use crate::syntax::*;

pub(super) fn parse_keyframes_name<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssKeyframesName, ParseError<'i, Error>> {
    let location = input.current_source_location();
    if let Ok(name) = input.try_parse(Parser::expect_ident_cloned) {
        if name.eq_ignore_ascii_case("none") {
            return Err(unsupported_value_at(
                location,
                None,
                "`none` is reserved and cannot be a keyframes name",
            ));
        }
        return parse_custom_ident_from_str_at("keyframes name", name.as_ref(), location)
            .map(CssKeyframesName::Ident);
    }

    let value = input.expect_string_cloned().map_err(basic)?;
    CssKeyframesString::try_new(value.to_string())
        .map(CssKeyframesName::String)
        .ok_or_else(|| unsupported_value(input, None, "keyframes string name is empty"))
}

pub(super) fn parse_keyframes_rule<'i, 't>(
    name: CssKeyframesName,
    input: &mut Parser<'i, 't>,
    start: &ParserState,
) -> std::result::Result<CssKeyframesRule, ParseError<'i, Error>> {
    let mut parser = KeyframeBlockParser;
    let mut blocks = Vec::new();
    for block in StyleSheetParser::new(input, &mut parser) {
        blocks.push(block.map_err(|(error, _)| error)?);
    }

    CssKeyframesRule::try_new(
        name,
        blocks,
        crate::source::CssSourcePosition::from_cssparser(start.position(), start.source_location()),
    )
    .ok_or_else(|| {
        invalid_at_rule_body(
            input,
            "keyframes",
            "baseline.rule.keyframes",
            "one or more valid keyframe blocks",
        )
    })
}

struct KeyframeBlockParser;

impl<'i> AtRuleParser<'i> for KeyframeBlockParser {
    type Prelude = ();
    type AtRule = CssKeyframeBlock;
    type Error = Error;
}

impl<'i> QualifiedRuleParser<'i> for KeyframeBlockParser {
    type Prelude = CssKeyframeSelectorList;
    type QualifiedRule = CssKeyframeBlock;
    type Error = Error;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::Prelude, ParseError<'i, Self::Error>> {
        parse_keyframe_selector_list(input)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let mut declarations = Vec::new();
        let mut declaration_parser = KeyframeDeclarationParser;
        for declaration in RuleBodyParser::new(input, &mut declaration_parser) {
            declarations.push(declaration.map_err(|(error, _)| error)?);
        }

        CssKeyframeBlock::try_new(
            selectors,
            CssKeyframeDeclarationList::new(declarations),
            crate::source::CssSourcePosition::from_cssparser(
                start.position(),
                start.source_location(),
            ),
        )
        .ok_or_else(|| {
            invalid_qualified_rule(
                start.source_location(),
                "baseline.keyframes.block",
                "a non-empty keyframe block",
            )
        })
    }
}

fn parse_keyframe_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssKeyframeSelectorList, ParseError<'i, Error>> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(parse_keyframe_selector(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "keyframe selector list has an empty item",
            ));
        }
    }

    CssKeyframeSelectorList::try_new(selectors)
        .ok_or_else(|| unsupported_value(input, None, "invalid keyframe selector list"))
}

fn parse_keyframe_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssKeyframeSelector, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Ident(ident) => match_ignore_ascii_case! { ident,
            "from" => Ok(CssKeyframeSelector::From),
            "to" => Ok(CssKeyframeSelector::To),
            _ => Err(unsupported_value_at(
                location,
                None,
                format!("unsupported keyframe selector `{ident}`"),
            )),
        },
        Token::Percentage { unit_value, .. } => {
            let value = *unit_value * 100.0;
            CssKeyframePercent::try_new(value)
                .map(CssKeyframeSelector::Percent)
                .ok_or_else(|| {
                    unsupported_value_at(
                        location,
                        None,
                        "keyframe selector must be 0% through 100%",
                    )
                })
        }
        Token::Number { .. } => Err(unsupported_value_at(
            location,
            None,
            "keyframe selector percentages must include `%`",
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

struct KeyframeDeclarationParser;

impl<'i> AtRuleParser<'i> for KeyframeDeclarationParser {
    type Prelude = ();
    type AtRule = CssKeyframeDeclaration;
    type Error = Error;
}

impl<'i> QualifiedRuleParser<'i> for KeyframeDeclarationParser {
    type Prelude = ();
    type QualifiedRule = CssKeyframeDeclaration;
    type Error = Error;
}

impl<'i> RuleBodyItemParser<'i, CssKeyframeDeclaration, Error> for KeyframeDeclarationParser {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

impl<'i> DeclarationParser<'i> for KeyframeDeclarationParser {
    type Declaration = CssKeyframeDeclaration;
    type Error = Error;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        declaration_start: &ParserState,
    ) -> std::result::Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let parsed =
            parse_declaration_core(DeclarationMode::Keyframe, name, input, declaration_start)?;
        Ok(CssKeyframeDeclaration::new(parsed.body, parsed.position))
    }
}

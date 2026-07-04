use cssparser::{BasicParseErrorKind, ParseError, Parser, Token};

use crate::error::{Error, basic, invalid_syntax};
use crate::syntax::{
    CssAuthoredDeclarationValue, CssCustomPropertyName, CssCustomPropertyValue, CssValue,
    CssVariableFallback, CssVariableReference,
};
use crate::validation::parse_global_keyword;

pub(crate) fn parse_custom_property_name(name: &str) -> Option<CssCustomPropertyName> {
    let suffix = name.strip_prefix("--")?;
    if suffix.is_empty() {
        None
    } else {
        Some(CssCustomPropertyName::new(name))
    }
}

pub(crate) fn parse_custom_property_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssValue, ParseError<'i, Error>> {
    let state = input.state();
    if let Ok(ident) = input.expect_ident_cloned()
        && let Some(keyword) = parse_global_keyword(&ident)
    {
        if input.is_exhausted() {
            return Ok(CssValue::GlobalKeyword(keyword));
        }
        return Err(invalid_syntax(
            input.current_source_location(),
            "CSS global keyword must be the entire custom property value",
        ));
    }
    input.reset(&state);

    let (authored, references) = collect_authored_declaration_value(input)?;
    Ok(CssValue::CustomProperty(CssCustomPropertyValue::new(
        authored, references,
    )))
}

pub(crate) fn collect_authored_declaration_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<(CssAuthoredDeclarationValue, Vec<CssVariableReference>), ParseError<'i, Error>> {
    input.skip_whitespace();
    let start = input.position();
    let mut references = Vec::new();
    consume_authored_value_tokens(input, &mut references)?;
    Ok((
        CssAuthoredDeclarationValue::new(input.slice_from(start).trim_end()),
        references,
    ))
}

fn consume_authored_value_tokens<'i, 't>(
    input: &mut Parser<'i, 't>,
    references: &mut Vec<CssVariableReference>,
) -> Result<(), ParseError<'i, Error>> {
    loop {
        let token = match input.next() {
            Ok(token) => token.clone(),
            Err(error) => {
                return match error.kind {
                    BasicParseErrorKind::EndOfInput => Ok(()),
                    _ => Err(basic(error)),
                };
            }
        };
        if token.is_parse_error() {
            return Err(input.new_unexpected_token_error(token));
        }
        if let Token::Function(name) = &token
            && name.eq_ignore_ascii_case("var")
        {
            let reference = input.parse_nested_block(parse_variable_reference)?;
            references.push(reference);
        } else if is_nested_block_start(&token) {
            input.parse_nested_block(|input| consume_authored_value_tokens(input, references))?;
        }
    }
}

fn parse_variable_reference<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssVariableReference, ParseError<'i, Error>> {
    let name_location = input.current_source_location();
    let name = input.expect_ident_cloned().map_err(basic)?;
    let Some(name) = parse_custom_property_name(&name) else {
        return Err(invalid_syntax(
            name_location,
            "`var()` must reference a custom property name",
        ));
    };

    if input.is_exhausted() {
        return Ok(CssVariableReference::new(name, None));
    }

    input.expect_comma().map_err(basic)?;
    let fallback = parse_variable_fallback(input)?;
    Ok(CssVariableReference::new(name, Some(fallback)))
}

fn parse_variable_fallback<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssVariableFallback, ParseError<'i, Error>> {
    input.skip_whitespace();
    let start = input.position();
    let mut references = Vec::new();
    consume_authored_value_tokens(input, &mut references)?;
    Ok(CssVariableFallback::new(
        CssAuthoredDeclarationValue::new(input.slice_from(start).trim_end()),
        references,
    ))
}

fn is_nested_block_start(token: &Token<'_>) -> bool {
    matches!(
        token,
        Token::Function(_)
            | Token::ParenthesisBlock
            | Token::SquareBracketBlock
            | Token::CurlyBracketBlock
    )
}

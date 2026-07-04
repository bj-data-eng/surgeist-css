use cssparser::{BasicParseErrorKind, ParseError, Parser, Token};

use crate::error::{Error, basic, invalid_syntax};
use crate::syntax::{
    CssAuthoredDeclarationValue, CssCustomPropertyName, CssCustomPropertyValue, CssValue,
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

    let authored = collect_authored_declaration_value(input)?;
    Ok(CssValue::CustomProperty(CssCustomPropertyValue::new(
        authored,
        Vec::new(),
    )))
}

pub(crate) fn collect_authored_declaration_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssAuthoredDeclarationValue, ParseError<'i, Error>> {
    input.skip_whitespace();
    let start = input.position();
    consume_authored_value_tokens(input)?;
    Ok(CssAuthoredDeclarationValue::new(
        input.slice_from(start).trim_end(),
    ))
}

fn consume_authored_value_tokens<'i, 't>(
    input: &mut Parser<'i, 't>,
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
        if is_nested_block_start(&token) {
            input.parse_nested_block(consume_authored_value_tokens)?;
        }
    }
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

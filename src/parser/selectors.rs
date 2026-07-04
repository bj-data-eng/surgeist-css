use cssparser::{BasicParseErrorKind, ParseError, Parser, ToCss, Token};

use crate::error::{Error, invalid_selector, selector_basic};
use crate::syntax::*;

pub(super) fn parse_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Vec<CssSelector>, ParseError<'i, Error>> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(parse_compound_selector(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted().map_err(selector_basic)?;
    Ok(selectors)
}

pub(super) fn parse_compound_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssSelector, ParseError<'i, Error>> {
    let mut tag_name = None;
    let mut key_name = None;
    let mut class_names = Vec::new();
    let mut pseudo_classes = Vec::new();

    if let Ok(tag) = input.try_parse(Parser::expect_ident_cloned) {
        let tag = tag.to_string();
        tag_name = Some(tag);
    }

    loop {
        if input.try_parse(|input| input.expect_delim('.')).is_ok() {
            let class = input.expect_ident_cloned().map_err(selector_basic)?;
            let class = class.to_string();
            class_names.push(class);
            continue;
        }

        if input.try_parse(Parser::expect_colon).is_ok() {
            let pseudo_class = parse_pseudo_class(input)?;
            pseudo_classes.push(pseudo_class);
            continue;
        }

        let state = input.state();
        match input.next() {
            Ok(Token::IDHash(key)) => {
                let key = key.to_string();
                key_name = Some(key);
            }
            Ok(token) => {
                let message = format!("unexpected selector token `{}`", token.to_css_string());
                input.reset(&state);
                if tag_name.is_none()
                    && key_name.is_none()
                    && class_names.is_empty()
                    && pseudo_classes.is_empty()
                {
                    return Err(invalid_selector(input, message));
                }
                break;
            }
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(selector_basic(error)),
        }
    }

    if tag_name.is_none()
        && key_name.is_none()
        && class_names.is_empty()
        && pseudo_classes.is_empty()
    {
        return Err(invalid_selector(
            input,
            "selector is missing a simple selector",
        ));
    }
    if let (None, None, [class], []) = (
        tag_name.as_ref(),
        key_name.as_ref(),
        class_names.as_slice(),
        pseudo_classes.as_slice(),
    ) {
        return Ok(CssSelector::Class(class.clone()));
    }
    if let (Some(tag), None, [], []) = (
        tag_name.as_ref(),
        key_name.as_ref(),
        class_names.as_slice(),
        pseudo_classes.as_slice(),
    ) {
        return Ok(CssSelector::Tag(tag.clone()));
    }
    if let (None, Some(key), [], []) = (
        tag_name.as_ref(),
        key_name.as_ref(),
        class_names.as_slice(),
        pseudo_classes.as_slice(),
    ) {
        return Ok(CssSelector::Key(key.clone()));
    }
    if let (None, None, [], [pseudo_class]) = (
        tag_name.as_ref(),
        key_name.as_ref(),
        class_names.as_slice(),
        pseudo_classes.as_slice(),
    ) {
        return Ok(CssSelector::PseudoClass(*pseudo_class));
    }
    Ok(CssSelector::Compound(CssCompoundSelector::new(
        tag_name,
        key_name,
        class_names,
        pseudo_classes,
    )))
}

fn parse_pseudo_class<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPseudoClass, ParseError<'i, Error>> {
    let state = input.state();
    match input.next() {
        Ok(Token::Ident(name)) if name.eq_ignore_ascii_case("root") => Ok(CssPseudoClass::Root),
        Ok(token) => {
            let message = format!("unsupported pseudo-class `:{}`", token.to_css_string());
            input.reset(&state);
            Err(invalid_selector(input, message))
        }
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => Err(
            invalid_selector(input, "selector pseudo-class is missing a name"),
        ),
        Err(error) => Err(selector_basic(error)),
    }
}

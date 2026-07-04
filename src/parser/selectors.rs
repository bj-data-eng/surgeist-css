use cssparser::{BasicParseErrorKind, ParseError, Parser, ToCss, Token, match_ignore_ascii_case};

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
    loop {
        let state = input.state();
        match input.next_including_whitespace() {
            Ok(Token::WhiteSpace(_)) => continue,
            Ok(_) => {
                input.reset(&state);
                break;
            }
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
                input.reset(&state);
                break;
            }
            Err(error) => return Err(selector_basic(error)),
        }
    }

    let mut tag_name = None;
    let mut key_name = None;
    let mut class_names = Vec::new();
    let mut pseudo_classes = Vec::new();

    if let Ok(tag) = input.try_parse(Parser::expect_ident_cloned) {
        let tag = tag.to_string();
        tag_name = Some(tag);
    }

    loop {
        let state = input.state();
        match input.next_including_whitespace() {
            Ok(Token::WhiteSpace(_)) => {
                input.reset(&state);
                break;
            }
            Ok(_) => input.reset(&state),
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(selector_basic(error)),
        }

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
        return Ok(CssSelector::PseudoClass(pseudo_class.clone()));
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
        Ok(Token::Ident(name)) => match_ignore_ascii_case! { &name,
            "root" => Ok(CssPseudoClass::Root),
            "hover" => Ok(CssPseudoClass::Hover),
            "active" => Ok(CssPseudoClass::Active),
            "focus" => Ok(CssPseudoClass::Focus),
            "focus-visible" => Ok(CssPseudoClass::FocusVisible),
            "focus-within" => Ok(CssPseudoClass::FocusWithin),
            "disabled" => Ok(CssPseudoClass::Disabled),
            "enabled" => Ok(CssPseudoClass::Enabled),
            "checked" => Ok(CssPseudoClass::Checked),
            "required" => Ok(CssPseudoClass::Required),
            "optional" => Ok(CssPseudoClass::Optional),
            "valid" => Ok(CssPseudoClass::Valid),
            "invalid" => Ok(CssPseudoClass::Invalid),
            "placeholder-shown" => Ok(CssPseudoClass::PlaceholderShown),
            "first-child" => Ok(CssPseudoClass::FirstChild),
            "last-child" => Ok(CssPseudoClass::LastChild),
            "only-child" => Ok(CssPseudoClass::OnlyChild),
            "empty" => Ok(CssPseudoClass::Empty),
            "first-of-type" => Ok(CssPseudoClass::FirstOfType),
            "last-of-type" => Ok(CssPseudoClass::LastOfType),
            "only-of-type" => Ok(CssPseudoClass::OnlyOfType),
            _ => {
                let message = format!("unsupported pseudo-class `:{name}`");
                Err(invalid_selector(input, message))
            }
        },
        Ok(Token::Function(name)) => {
            let name = name.clone();
            input.parse_nested_block(|input| {
                let pseudo_class = match_ignore_ascii_case! { &name,
                    "nth-child" => CssPseudoClass::NthChild(parse_nth_pattern(input)?),
                    "nth-last-child" => CssPseudoClass::NthLastChild(parse_nth_pattern(input)?),
                    "nth-of-type" => CssPseudoClass::NthOfType(parse_nth_pattern(input)?),
                    "nth-last-of-type" => {
                        CssPseudoClass::NthLastOfType(parse_nth_pattern(input)?)
                    },
                    "not" => CssPseudoClass::Not(parse_pseudo_selector_list(input)?),
                    "is" => CssPseudoClass::Is(parse_pseudo_selector_list(input)?),
                    "where" => CssPseudoClass::Where(parse_pseudo_selector_list(input)?),
                    "has" => CssPseudoClass::Has(parse_pseudo_selector_list(input)?),
                    _ => {
                        let message = format!("unsupported pseudo-class `:{name}(`");
                        return Err(invalid_selector(input, message));
                    }
                };
                input.expect_exhausted().map_err(selector_basic)?;
                Ok(pseudo_class)
            })
        }
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

fn parse_pseudo_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssSelectorList, ParseError<'i, Error>> {
    let selectors = parse_selector_list(input)?;
    CssSelectorList::try_new(selectors)
        .ok_or_else(|| invalid_selector(input, "pseudo-class selector list must not be empty"))
}

fn parse_nth_pattern<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssNthPattern, ParseError<'i, Error>> {
    let token = input.next().map_err(selector_basic)?.clone();
    match token {
        Token::Ident(value) => parse_nth_ident(input, &value),
        Token::Number {
            int_value: Some(value),
            ..
        } => Ok(CssNthPattern::Integer(value)),
        Token::Dimension {
            int_value: Some(a),
            unit,
            ..
        } if unit.eq_ignore_ascii_case("n") => parse_nth_dimension(input, a),
        Token::Dimension {
            int_value: Some(a),
            unit,
            ..
        } => parse_nth_dimension_unit(input, a, &unit),
        Token::Delim('+') => {
            let token = input.next_including_whitespace().map_err(selector_basic)?;
            match token {
                Token::Ident(value) if value.eq_ignore_ascii_case("n") => {
                    Ok(CssNthPattern::AnPlusB(CssNthAnPlusB::new(1, 0)))
                }
                _ => Err(invalid_selector(
                    input,
                    "unsupported nth pseudo-class pattern",
                )),
            }
        }
        _ => Err(invalid_selector(
            input,
            "unsupported nth pseudo-class pattern",
        )),
    }
}

fn parse_nth_ident<'i, 't>(
    input: &mut Parser<'i, 't>,
    value: &str,
) -> std::result::Result<CssNthPattern, ParseError<'i, Error>> {
    match_ignore_ascii_case! { value,
        "odd" => Ok(CssNthPattern::Odd),
        "even" => Ok(CssNthPattern::Even),
        "n" => Ok(CssNthPattern::AnPlusB(CssNthAnPlusB::new(1, 0))),
        "-n" => parse_optional_nth_b(input, -1),
        _ => Err(invalid_selector(input, "unsupported nth pseudo-class pattern")),
    }
}

fn parse_nth_dimension<'i, 't>(
    input: &mut Parser<'i, 't>,
    a: i32,
) -> std::result::Result<CssNthPattern, ParseError<'i, Error>> {
    parse_optional_nth_b(input, a)
}

fn parse_nth_dimension_unit<'i, 't>(
    input: &mut Parser<'i, 't>,
    a: i32,
    unit: &str,
) -> std::result::Result<CssNthPattern, ParseError<'i, Error>> {
    let Some(b_digits) = unit.strip_prefix("n-").or_else(|| unit.strip_prefix("N-")) else {
        return Err(invalid_selector(
            input,
            "unsupported nth pseudo-class pattern",
        ));
    };
    if b_digits.is_empty() || !b_digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid_selector(
            input,
            "unsupported nth pseudo-class pattern",
        ));
    }
    let b = b_digits
        .parse::<i32>()
        .map_err(|_| invalid_selector(input, "unsupported nth pseudo-class pattern"))?;
    Ok(CssNthPattern::AnPlusB(CssNthAnPlusB::new(a, -b)))
}

fn parse_optional_nth_b<'i, 't>(
    input: &mut Parser<'i, 't>,
    a: i32,
) -> std::result::Result<CssNthPattern, ParseError<'i, Error>> {
    let state = input.state();
    match input.next_including_whitespace() {
        Ok(Token::Number {
            has_sign: true,
            int_value: Some(b),
            ..
        }) => Ok(CssNthPattern::AnPlusB(CssNthAnPlusB::new(a, *b))),
        Ok(_) => {
            input.reset(&state);
            Ok(CssNthPattern::AnPlusB(CssNthAnPlusB::new(a, 0)))
        }
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
            input.reset(&state);
            Ok(CssNthPattern::AnPlusB(CssNthAnPlusB::new(a, 0)))
        }
        Err(error) => Err(selector_basic(error)),
    }
}

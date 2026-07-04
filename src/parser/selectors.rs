use cssparser::{BasicParseErrorKind, ParseError, Parser, ToCss, Token, match_ignore_ascii_case};

use crate::error::{Error, invalid_selector, selector_basic};
use crate::syntax::*;

pub(super) fn parse_rule_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Vec<CssSelector>, ParseError<'i, Error>> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(parse_rule_selector(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted().map_err(selector_basic)?;
    Ok(selectors)
}

pub(super) fn parse_rule_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssSelector, ParseError<'i, Error>> {
    parse_rule_selector_with_has_policy(input, true)
}

fn parse_rule_selector_with_has_policy<'i, 't>(
    input: &mut Parser<'i, 't>,
    allow_has: bool,
) -> std::result::Result<CssSelector, ParseError<'i, Error>> {
    let first = parse_compound_selector_model_with_has_policy(input, allow_has)?;
    parse_selector_after_first_compound(input, first, allow_has)
}

fn parse_selector_after_first_compound<'i, 't>(
    input: &mut Parser<'i, 't>,
    first: CssCompoundSelector,
    allow_has: bool,
) -> std::result::Result<CssSelector, ParseError<'i, Error>> {
    let mut rest = Vec::new();

    loop {
        let had_whitespace = consume_selector_whitespace(input)?;
        let state = input.state();
        match input.next_including_whitespace() {
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
                input.reset(&state);
                break;
            }
            Err(error) => return Err(selector_basic(error)),
            Ok(Token::Comma) => {
                input.reset(&state);
                break;
            }
            Ok(Token::Delim('>')) => {
                rest.push(parse_complex_selector_part_with_has_policy(
                    input,
                    CssSelectorCombinator::Child,
                    allow_has,
                )?);
            }
            Ok(Token::Delim('+')) => {
                rest.push(parse_complex_selector_part_with_has_policy(
                    input,
                    CssSelectorCombinator::NextSibling,
                    allow_has,
                )?);
            }
            Ok(Token::Delim('~')) => {
                rest.push(parse_complex_selector_part_with_has_policy(
                    input,
                    CssSelectorCombinator::SubsequentSibling,
                    allow_has,
                )?);
            }
            Ok(Token::Delim('|')) => {
                return Err(invalid_selector(
                    input,
                    "unsupported selector combinator `||`",
                ));
            }
            Ok(_) if had_whitespace => {
                input.reset(&state);
                let selector = parse_compound_selector_model_with_has_policy(input, allow_has)?;
                rest.push(CssComplexSelectorPart::new(
                    CssSelectorCombinator::Descendant,
                    selector,
                ));
            }
            Ok(token) => {
                let message = format!("unexpected selector token `{}`", token.to_css_string());
                input.reset(&state);
                return Err(invalid_selector(input, message));
            }
        }
    }

    if rest.is_empty() {
        Ok(compound_selector_to_selector(first))
    } else {
        Ok(CssSelector::Complex(CssComplexSelector::new(first, rest)))
    }
}

pub(super) fn parse_complex_selector_part<'i, 't>(
    input: &mut Parser<'i, 't>,
    combinator: CssSelectorCombinator,
) -> std::result::Result<CssComplexSelectorPart, ParseError<'i, Error>> {
    parse_complex_selector_part_with_has_policy(input, combinator, true)
}

fn parse_complex_selector_part_with_has_policy<'i, 't>(
    input: &mut Parser<'i, 't>,
    combinator: CssSelectorCombinator,
    allow_has: bool,
) -> std::result::Result<CssComplexSelectorPart, ParseError<'i, Error>> {
    consume_selector_whitespace(input)?;
    let selector = parse_compound_selector_model_with_has_policy(input, allow_has)?;
    Ok(CssComplexSelectorPart::new(combinator, selector))
}

pub(super) fn consume_selector_whitespace<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<bool, ParseError<'i, Error>> {
    let mut consumed = false;
    loop {
        let state = input.state();
        match input.next_including_whitespace() {
            Ok(Token::WhiteSpace(_)) => consumed = true,
            Ok(_) => {
                input.reset(&state);
                return Ok(consumed);
            }
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
                input.reset(&state);
                return Ok(consumed);
            }
            Err(error) => return Err(selector_basic(error)),
        }
    }
}

pub(super) fn parse_compound_selector_model<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssCompoundSelector, ParseError<'i, Error>> {
    parse_compound_selector_model_with_has_policy(input, true)
}

fn parse_compound_selector_model_with_has_policy<'i, 't>(
    input: &mut Parser<'i, 't>,
    allow_has: bool,
) -> std::result::Result<CssCompoundSelector, ParseError<'i, Error>> {
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
    let mut attributes = Vec::new();
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

        if input.try_parse(Parser::expect_square_bracket_block).is_ok() {
            let attribute = input.parse_nested_block(parse_attribute_selector)?;
            attributes.push(attribute);
            continue;
        }

        if input.try_parse(Parser::expect_colon).is_ok() {
            let pseudo_class = parse_pseudo_class_with_has_policy(input, allow_has)?;
            pseudo_classes.push(pseudo_class);
            continue;
        }

        let state = input.state();
        match input.next() {
            Ok(Token::IDHash(key)) => {
                let key = key.to_string();
                key_name = Some(key);
            }
            Ok(Token::Delim('|')) => {
                return Err(invalid_selector(input, "unsupported selector namespace"));
            }
            Ok(token) => {
                let message = format!("unexpected selector token `{}`", token.to_css_string());
                input.reset(&state);
                if tag_name.is_none()
                    && key_name.is_none()
                    && class_names.is_empty()
                    && attributes.is_empty()
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
        && attributes.is_empty()
        && pseudo_classes.is_empty()
    {
        return Err(invalid_selector(
            input,
            "selector is missing a simple selector",
        ));
    }
    if let (None, None, [class], [], []) = (
        tag_name.as_ref(),
        key_name.as_ref(),
        class_names.as_slice(),
        attributes.as_slice(),
        pseudo_classes.as_slice(),
    ) {
        return Ok(CssCompoundSelector::new(
            None,
            None,
            vec![class.clone()],
            Vec::new(),
            Vec::new(),
        ));
    }
    if let (Some(tag), None, [], [], []) = (
        tag_name.as_ref(),
        key_name.as_ref(),
        class_names.as_slice(),
        attributes.as_slice(),
        pseudo_classes.as_slice(),
    ) {
        return Ok(CssCompoundSelector::new(
            Some(tag.clone()),
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ));
    }
    if let (None, Some(key), [], [], []) = (
        tag_name.as_ref(),
        key_name.as_ref(),
        class_names.as_slice(),
        attributes.as_slice(),
        pseudo_classes.as_slice(),
    ) {
        return Ok(CssCompoundSelector::new(
            None,
            Some(key.clone()),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ));
    }
    if let (None, None, [], [], [pseudo_class]) = (
        tag_name.as_ref(),
        key_name.as_ref(),
        class_names.as_slice(),
        attributes.as_slice(),
        pseudo_classes.as_slice(),
    ) {
        return Ok(CssCompoundSelector::new(
            None,
            None,
            Vec::new(),
            Vec::new(),
            vec![pseudo_class.clone()],
        ));
    }
    Ok(CssCompoundSelector::new(
        tag_name,
        key_name,
        class_names,
        attributes,
        pseudo_classes,
    ))
}

fn compound_selector_to_selector(selector: CssCompoundSelector) -> CssSelector {
    if let (None, None, [class], [], []) = (
        selector.tag(),
        selector.key(),
        selector.classes(),
        selector.attributes(),
        selector.pseudo_classes(),
    ) {
        return CssSelector::Class(class.clone());
    }
    if let (Some(tag), None, [], [], []) = (
        selector.tag(),
        selector.key(),
        selector.classes(),
        selector.attributes(),
        selector.pseudo_classes(),
    ) {
        return CssSelector::Tag(tag.clone());
    }
    if let (None, Some(key), [], [], []) = (
        selector.tag(),
        selector.key(),
        selector.classes(),
        selector.attributes(),
        selector.pseudo_classes(),
    ) {
        return CssSelector::Key(key.clone());
    }
    if let (None, None, [], [], [pseudo_class]) = (
        selector.tag(),
        selector.key(),
        selector.classes(),
        selector.attributes(),
        selector.pseudo_classes(),
    ) {
        return CssSelector::PseudoClass(pseudo_class.clone());
    }
    CssSelector::Compound(selector)
}

fn parse_attribute_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAttributeSelector, ParseError<'i, Error>> {
    let name = input.expect_ident_cloned().map_err(selector_basic)?;
    let name = CssAttributeName::new(name.to_string());

    let matcher = match input.next() {
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
            return Ok(CssAttributeSelector::new(
                name,
                CssAttributeMatcher::Exists,
                CssAttributeCaseSensitivity::DocumentDefault,
            ));
        }
        Err(error) => return Err(selector_basic(error)),
        Ok(Token::Delim('=')) => {
            CssAttributeMatcher::Equals(parse_attribute_selector_value(input)?)
        }
        Ok(Token::IncludeMatch) => {
            CssAttributeMatcher::Includes(parse_attribute_selector_value(input)?)
        }
        Ok(Token::DashMatch) => {
            CssAttributeMatcher::DashMatch(parse_attribute_selector_value(input)?)
        }
        Ok(Token::PrefixMatch) => {
            CssAttributeMatcher::Prefix(parse_attribute_selector_value(input)?)
        }
        Ok(Token::SuffixMatch) => {
            CssAttributeMatcher::Suffix(parse_attribute_selector_value(input)?)
        }
        Ok(Token::SubstringMatch) => {
            CssAttributeMatcher::Substring(parse_attribute_selector_value(input)?)
        }
        Ok(token) => {
            let message = format!(
                "unsupported attribute selector token `{}`",
                token.to_css_string()
            );
            return Err(invalid_selector(input, message));
        }
    };

    let case_sensitivity = parse_attribute_case_sensitivity(input)?;
    input.expect_exhausted().map_err(selector_basic)?;
    Ok(CssAttributeSelector::new(name, matcher, case_sensitivity))
}

fn parse_attribute_selector_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<String, ParseError<'i, Error>> {
    let value = input
        .expect_ident_or_string()
        .map_err(selector_basic)?
        .to_string();
    Ok(value)
}

fn parse_attribute_case_sensitivity<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAttributeCaseSensitivity, ParseError<'i, Error>> {
    let state = input.state();
    match input.next() {
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
            input.reset(&state);
            Ok(CssAttributeCaseSensitivity::DocumentDefault)
        }
        Err(error) => Err(selector_basic(error)),
        Ok(Token::Ident(modifier)) if modifier.eq_ignore_ascii_case("i") => {
            Ok(CssAttributeCaseSensitivity::AsciiCaseInsensitive)
        }
        Ok(Token::Ident(modifier)) if modifier.eq_ignore_ascii_case("s") => {
            Ok(CssAttributeCaseSensitivity::ExplicitSensitive)
        }
        Ok(token) => {
            let message = format!(
                "unsupported attribute selector case modifier `{}`",
                token.to_css_string()
            );
            Err(invalid_selector(input, message))
        }
    }
}

fn parse_pseudo_class_with_has_policy<'i, 't>(
    input: &mut Parser<'i, 't>,
    allow_has: bool,
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
            "modal" => Ok(CssPseudoClass::Modal),
            "fullscreen" => Ok(CssPseudoClass::Fullscreen),
            "popover-open" => Ok(CssPseudoClass::PopoverOpen),
            "default" => Ok(CssPseudoClass::Default),
            "indeterminate" => Ok(CssPseudoClass::Indeterminate),
            "read-only" => Ok(CssPseudoClass::ReadOnly),
            "read-write" => Ok(CssPseudoClass::ReadWrite),
            "in-range" => Ok(CssPseudoClass::InRange),
            "out-of-range" => Ok(CssPseudoClass::OutOfRange),
            _ => {
                let message = format!("unsupported pseudo-class `:{name}`");
                Err(invalid_selector(input, message))
            }
        },
        Ok(Token::Function(name)) => {
            let name = name.clone();
            input.parse_nested_block(|input| {
                let pseudo_class = match_ignore_ascii_case! { &name,
                    "nth-child" => {
                        CssPseudoClass::NthChild(parse_nth_child_pattern(input, allow_has)?)
                    },
                    "nth-last-child" => {
                        CssPseudoClass::NthLastChild(parse_nth_child_pattern(input, allow_has)?)
                    },
                    "nth-of-type" => CssPseudoClass::NthOfType(parse_nth_pattern(input)?),
                    "nth-last-of-type" => {
                        CssPseudoClass::NthLastOfType(parse_nth_pattern(input)?)
                    },
                    "not" => CssPseudoClass::Not(parse_pseudo_selector_list_with_has_policy(input, allow_has)?),
                    "is" => CssPseudoClass::Is(parse_pseudo_selector_list_with_has_policy(input, allow_has)?),
                    "where" => CssPseudoClass::Where(parse_pseudo_selector_list_with_has_policy(input, allow_has)?),
                    "has" if allow_has => CssPseudoClass::Has(parse_has_relative_selector_list(input)?),
                    "has" => {
                        return Err(invalid_selector(input, "nested `:has()` is unsupported"));
                    },
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

fn parse_pseudo_selector_list_with_has_policy<'i, 't>(
    input: &mut Parser<'i, 't>,
    allow_has: bool,
) -> std::result::Result<CssPseudoSelectorList, ParseError<'i, Error>> {
    let selectors = parse_pseudo_selector_list_items_with_has_policy(input, allow_has)?;
    CssPseudoSelectorList::try_new(selectors)
        .ok_or_else(|| invalid_selector(input, "pseudo-class selector list must not be empty"))
}

fn parse_pseudo_selector_list_items_with_has_policy<'i, 't>(
    input: &mut Parser<'i, 't>,
    allow_has: bool,
) -> std::result::Result<Vec<CssSelector>, ParseError<'i, Error>> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(parse_rule_selector_with_has_policy(input, allow_has)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted().map_err(selector_basic)?;
    Ok(selectors)
}

fn parse_has_relative_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssRelativeSelectorList, ParseError<'i, Error>> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(parse_has_relative_selector(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted().map_err(selector_basic)?;
    CssRelativeSelectorList::try_new(selectors)
        .ok_or_else(|| invalid_selector(input, "relative selector list must not be empty"))
}

fn parse_has_relative_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssRelativeSelector, ParseError<'i, Error>> {
    consume_selector_whitespace(input)?;
    let state = input.state();
    match input.next_including_whitespace() {
        Ok(Token::Delim('>')) => {
            parse_selector_after_leading_combinator(input, CssSelectorCombinator::Child)
        }
        Ok(Token::Delim('+')) => {
            parse_selector_after_leading_combinator(input, CssSelectorCombinator::NextSibling)
        }
        Ok(Token::Delim('~')) => {
            parse_selector_after_leading_combinator(input, CssSelectorCombinator::SubsequentSibling)
        }
        Ok(Token::Delim('|')) => Err(invalid_selector(
            input,
            "unsupported selector combinator `||`",
        )),
        Ok(_) => {
            input.reset(&state);
            let selector = parse_rule_selector_with_has_policy(input, false)?;
            Ok(CssRelativeSelector::new(
                CssSelectorCombinator::Descendant,
                selector,
            ))
        }
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
            input.reset(&state);
            Err(invalid_selector(input, "relative selector is missing"))
        }
        Err(error) => Err(selector_basic(error)),
    }
}

fn parse_selector_after_leading_combinator<'i, 't>(
    input: &mut Parser<'i, 't>,
    combinator: CssSelectorCombinator,
) -> std::result::Result<CssRelativeSelector, ParseError<'i, Error>> {
    consume_selector_whitespace(input)?;
    let first = parse_compound_selector_model_with_has_policy(input, false)?;
    let selector = parse_selector_after_first_compound(input, first, false)?;
    Ok(CssRelativeSelector::new(combinator, selector))
}

fn parse_nth_child_pattern<'i, 't>(
    input: &mut Parser<'i, 't>,
    allow_has: bool,
) -> std::result::Result<CssNthChildPattern, ParseError<'i, Error>> {
    let pattern = parse_nth_pattern(input)?;
    let state = input.state();
    match input.next() {
        Ok(Token::Ident(value)) if value.eq_ignore_ascii_case("of") => {
            let selector_list = parse_pseudo_selector_list_with_has_policy(input, allow_has)?;
            Ok(CssNthChildPattern::new(pattern, Some(selector_list)))
        }
        Ok(_) => {
            input.reset(&state);
            Ok(CssNthChildPattern::new(pattern, None))
        }
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
            input.reset(&state);
            Ok(CssNthChildPattern::new(pattern, None))
        }
        Err(error) => Err(selector_basic(error)),
    }
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

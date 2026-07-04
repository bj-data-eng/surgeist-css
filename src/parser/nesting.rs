use cssparser::{
    AtRuleParser, BasicParseErrorKind, CowRcStr, DeclarationParser, ParseError, Parser,
    ParserState, QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, ToCss, Token,
    match_ignore_ascii_case,
};

use super::queries::parse_media_query_list;
use super::selectors::{
    consume_selector_whitespace, parse_complex_selector_part, parse_compound_selector_model,
    parse_rule_selector,
};
use super::{CssContainerPrelude, StrictDeclarationParser, parse_container_prelude};
use crate::error::{Error, invalid_selector, invalid_syntax, selector_basic};
use crate::syntax::*;

pub(super) fn parse_style_rule_block<'i, 't>(
    parent_selectors: Vec<CssSelector>,
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Vec<CssRule>, ParseError<'i, Error>> {
    let mut body_parser = NestedStyleRuleParser { parent_selectors };
    let parent_selectors = body_parser.parent_selectors.clone();
    let mut rules = Vec::new();
    let mut declaration_buffer = Vec::new();
    let mut saw_item = false;

    for item in RuleBodyParser::new(input, &mut body_parser) {
        saw_item = true;
        match item.map_err(|(error, _)| error)? {
            StyleBlockItem::Declaration(declaration) => declaration_buffer.push(declaration),
            StyleBlockItem::NestedRules(nested_rules) => {
                flush_declarations(&parent_selectors, &mut declaration_buffer, &mut rules);
                rules.extend(nested_rules);
            }
        }
    }

    flush_declarations(&parent_selectors, &mut declaration_buffer, &mut rules);
    if !saw_item && rules.is_empty() {
        for selector in &parent_selectors {
            rules.push(CssRule::Style(CssStyleRule::new(
                selector.clone(),
                Vec::new(),
            )));
        }
    }

    Ok(rules)
}

fn flush_declarations(
    parent_selectors: &[CssSelector],
    declaration_buffer: &mut Vec<CssDeclaration>,
    rules: &mut Vec<CssRule>,
) {
    if declaration_buffer.is_empty() {
        return;
    }

    for selector in parent_selectors {
        rules.push(CssRule::Style(CssStyleRule::new(
            selector.clone(),
            declaration_buffer.clone(),
        )));
    }
    declaration_buffer.clear();
}

struct NestedStyleRuleParser {
    parent_selectors: Vec<CssSelector>,
}

enum StyleBlockItem {
    Declaration(CssDeclaration),
    NestedRules(Vec<CssRule>),
}

enum NestedStyleAtRulePrelude {
    Media(CssMediaQueryList),
    Container(CssContainerPrelude),
}

impl<'i> AtRuleParser<'i> for NestedStyleRuleParser {
    type Prelude = NestedStyleAtRulePrelude;
    type AtRule = StyleBlockItem;
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
                Ok(NestedStyleAtRulePrelude::Media(query))
            },
            "container" => {
                let prelude = parse_container_prelude(input)?;
                if !input.is_exhausted() {
                    return Err(invalid_syntax(
                        input.current_source_location(),
                        "unexpected token after container condition",
                    ));
                }
                Ok(NestedStyleAtRulePrelude::Container(prelude))
            },
            "import" => Err(invalid_syntax(
                input.current_source_location(),
                "@import rules are not supported inside style blocks",
            )),
            "font-face" => Err(invalid_syntax(
                input.current_source_location(),
                "@font-face rules are not supported inside style blocks",
            )),
            "keyframes" => Err(invalid_syntax(
                input.current_source_location(),
                "@keyframes rules are not supported inside style blocks",
            )),
            _ => Err(input.new_error(cssparser::BasicParseErrorKind::AtRuleInvalid(name))),
        }
    }

    fn rule_without_block(
        &mut self,
        _prelude: Self::Prelude,
        _start: &ParserState,
    ) -> std::result::Result<Self::AtRule, ()> {
        Err(())
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::AtRule, ParseError<'i, Self::Error>> {
        let rules = parse_style_rule_block(self.parent_selectors.clone(), input)?;
        let location = CssSourceLocation::from_cssparser(start.source_location());
        let rule = match prelude {
            NestedStyleAtRulePrelude::Media(query) => {
                CssRule::Media(CssMediaRule::new(query, rules, location))
            }
            NestedStyleAtRulePrelude::Container(prelude) => CssRule::Container(
                CssContainerRule::new(prelude.name, prelude.condition, rules, location),
            ),
        };
        Ok(StyleBlockItem::NestedRules(vec![rule]))
    }
}

impl<'i> QualifiedRuleParser<'i> for NestedStyleRuleParser {
    type Prelude = Vec<NestedSelector>;
    type QualifiedRule = StyleBlockItem;
    type Error = Error;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::Prelude, ParseError<'i, Self::Error>> {
        parse_nested_selector_list(input)
    }

    fn parse_block<'t>(
        &mut self,
        nested_selectors: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let mut flattened_selectors = Vec::new();
        for parent_selector in &self.parent_selectors {
            for nested_selector in &nested_selectors {
                flattened_selectors.push(nested_selector.flatten(parent_selector.clone(), input)?);
            }
        }

        parse_style_rule_block(flattened_selectors, input).map(StyleBlockItem::NestedRules)
    }
}

impl<'i> RuleBodyItemParser<'i, StyleBlockItem, Error> for NestedStyleRuleParser {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        true
    }
}

impl<'i> DeclarationParser<'i> for NestedStyleRuleParser {
    type Declaration = StyleBlockItem;
    type Error = Error;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        declaration_start: &ParserState,
    ) -> std::result::Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let mut declaration_parser = StrictDeclarationParser;
        declaration_parser
            .parse_value(name, input, declaration_start)
            .map(StyleBlockItem::Declaration)
    }
}

#[derive(Clone, Debug)]
enum NestedSelector {
    Descendant(CssSelector),
    Relative(Vec<CssComplexSelectorPart>),
    Parent,
    Append(CssCompoundSelector),
}

impl NestedSelector {
    fn flatten<'i, 't>(
        &self,
        parent: CssSelector,
        input: &Parser<'i, 't>,
    ) -> std::result::Result<CssSelector, ParseError<'i, Error>> {
        match self {
            Self::Descendant(child) => CssSelector::combine_descendant(parent, child.clone())
                .ok_or_else(|| invalid_selector(input, "invalid nested descendant selector")),
            Self::Relative(parts) => {
                let mut parts = parts.iter();
                let Some(first) = parts.next() else {
                    return Ok(parent);
                };
                let mut combined = CssSelector::combine_with_combinator(
                    parent,
                    first.combinator(),
                    first.selector().clone(),
                )
                .ok_or_else(|| invalid_selector(input, "invalid nested relative selector"))?;
                for part in parts {
                    combined = CssSelector::combine_with_combinator(
                        combined,
                        part.combinator(),
                        part.selector().clone(),
                    )
                    .ok_or_else(|| invalid_selector(input, "invalid nested relative selector"))?;
                }
                Ok(combined)
            }
            Self::Parent => Ok(parent),
            Self::Append(suffix) => CssSelector::append_to_subject(parent, suffix.clone())
                .ok_or_else(|| invalid_selector(input, "invalid nested selector suffix")),
        }
    }
}

fn parse_nested_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Vec<NestedSelector>, ParseError<'i, Error>> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(parse_nested_selector(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted().map_err(selector_basic)?;
    Ok(selectors)
}

fn parse_nested_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<NestedSelector, ParseError<'i, Error>> {
    consume_selector_whitespace(input)?;
    let state = input.state();
    match input.next_including_whitespace() {
        Ok(Token::Delim('&')) => parse_ampersand_nested_selector(input),
        Ok(Token::Delim('>')) => parse_relative_selector(input, CssSelectorCombinator::Child),
        Ok(Token::Delim('+')) => parse_relative_selector(input, CssSelectorCombinator::NextSibling),
        Ok(Token::Delim('~')) => {
            parse_relative_selector(input, CssSelectorCombinator::SubsequentSibling)
        }
        Ok(Token::Delim('|')) => Err(invalid_selector(
            input,
            "unsupported selector combinator `||`",
        )),
        Ok(_) => {
            input.reset(&state);
            parse_rule_selector(input).map(NestedSelector::Descendant)
        }
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
            input.reset(&state);
            Err(invalid_selector(input, "nested selector is empty"))
        }
        Err(error) => Err(selector_basic(error)),
    }
}

fn parse_ampersand_nested_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<NestedSelector, ParseError<'i, Error>> {
    let had_whitespace = consume_selector_whitespace(input)?;
    let state = input.state();
    match input.next_including_whitespace() {
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
            input.reset(&state);
            Ok(NestedSelector::Parent)
        }
        Err(error) => Err(selector_basic(error)),
        Ok(Token::Comma) => {
            input.reset(&state);
            Ok(NestedSelector::Parent)
        }
        Ok(Token::Delim('&')) => Err(invalid_selector(
            input,
            "nesting selector `&` is only supported once at the start",
        )),
        Ok(Token::Delim('>')) => parse_relative_selector(input, CssSelectorCombinator::Child),
        Ok(Token::Delim('+')) => parse_relative_selector(input, CssSelectorCombinator::NextSibling),
        Ok(Token::Delim('~')) => {
            parse_relative_selector(input, CssSelectorCombinator::SubsequentSibling)
        }
        Ok(Token::Delim('|')) => Err(invalid_selector(
            input,
            "unsupported selector combinator `||`",
        )),
        Ok(_) if had_whitespace => {
            input.reset(&state);
            parse_relative_selector(input, CssSelectorCombinator::Descendant)
        }
        Ok(_) => {
            input.reset(&state);
            let suffix = parse_compound_selector_model(input)?;
            ensure_nested_selector_boundary(input)?;
            Ok(NestedSelector::Append(suffix))
        }
    }
}

fn parse_relative_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
    first_combinator: CssSelectorCombinator,
) -> std::result::Result<NestedSelector, ParseError<'i, Error>> {
    let mut parts = vec![parse_complex_selector_part(input, first_combinator)?];
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
            Ok(Token::Delim('>')) => parts.push(parse_complex_selector_part(
                input,
                CssSelectorCombinator::Child,
            )?),
            Ok(Token::Delim('+')) => parts.push(parse_complex_selector_part(
                input,
                CssSelectorCombinator::NextSibling,
            )?),
            Ok(Token::Delim('~')) => parts.push(parse_complex_selector_part(
                input,
                CssSelectorCombinator::SubsequentSibling,
            )?),
            Ok(Token::Delim('|')) => {
                return Err(invalid_selector(
                    input,
                    "unsupported selector combinator `||`",
                ));
            }
            Ok(Token::Delim('&')) => {
                return Err(invalid_selector(
                    input,
                    "nesting selector `&` is only supported once at the start",
                ));
            }
            Ok(_) if had_whitespace => {
                input.reset(&state);
                let selector = parse_compound_selector_model(input)?;
                parts.push(CssComplexSelectorPart::new(
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
    Ok(NestedSelector::Relative(parts))
}

fn ensure_nested_selector_boundary<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<(), ParseError<'i, Error>> {
    consume_selector_whitespace(input)?;
    let state = input.state();
    match input.next_including_whitespace() {
        Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => {
            input.reset(&state);
            Ok(())
        }
        Err(error) => Err(selector_basic(error)),
        Ok(Token::Comma) => {
            input.reset(&state);
            Ok(())
        }
        Ok(Token::Delim('&')) => Err(invalid_selector(
            input,
            "nesting selector `&` is only supported once at the start",
        )),
        Ok(token) => {
            let message = format!("unexpected selector token `{}`", token.to_css_string());
            input.reset(&state);
            Err(invalid_selector(input, message))
        }
    }
}

//! Strict CSS ingestion for Surgeist style sheets.
//!
//! This module parses CSS syntax into CSS-owned authored syntax values. It is
//! strict by design: unsupported selectors, at-rules, properties, and values are
//! errors instead of browser-style recoverable invalid declarations.
//!
//! Parse failures expose typed [`ErrorKind`] values plus source line and column
//! information so callers do not need to parse display strings.

use std::fmt;

use cssparser::{
    AtRuleParser, BasicParseError, BasicParseErrorKind, CowRcStr, DeclarationParser, ParseError,
    ParseErrorKind, Parser, ParserInput, ParserState, QualifiedRuleParser, RuleBodyItemParser,
    RuleBodyParser, StyleSheetParser, ToCss, Token, match_ignore_ascii_case,
};

mod syntax;

pub use syntax::*;

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    InvalidSyntax {
        reason: String,
    },
    InvalidSelector {
        reason: String,
    },
    UnsupportedAtRule {
        name: String,
    },
    UnsupportedProperty {
        name: String,
    },
    UnsupportedValue {
        property: Option<String>,
        reason: String,
    },
    InvalidColor {
        value: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    line: u32,
    column: u32,
}

impl Error {
    fn at(
        location: cssparser::SourceLocation,
        kind: ErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            line: location.line,
            column: location.column,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    #[must_use]
    pub const fn column(&self) -> u32 {
        self.column
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CSS parse error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for Error {}

pub fn parse_sheet(input: &str) -> Result<CssSheet> {
    let mut input = ParserInput::new(input);
    let mut parser = Parser::new(&mut input);
    let mut rule_parser = StrictRuleParser;
    let mut sheet = CssSheet::new();

    for rule in StyleSheetParser::new(&mut parser, &mut rule_parser) {
        for rule in rule.map_err(|(error, _)| from_parse_error(error))? {
            sheet.push_rule(rule);
        }
    }

    Ok(sheet)
}

struct StrictRuleParser;

impl<'i> AtRuleParser<'i> for StrictRuleParser {
    type Prelude = ();
    type AtRule = Vec<CssRule>;
    type Error = Error;
}

impl<'i> QualifiedRuleParser<'i> for StrictRuleParser {
    type Prelude = Vec<CssSelector>;
    type QualifiedRule = Vec<CssRule>;
    type Error = Error;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::Prelude, ParseError<'i, Self::Error>> {
        parse_selector_list(input)
    }

    fn parse_block<'t>(
        &mut self,
        selectors: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> std::result::Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let mut declarations = Vec::new();
        let mut declaration_parser = StrictDeclarationParser;
        for declaration in RuleBodyParser::new(input, &mut declaration_parser) {
            let declaration = declaration.map_err(|(error, _)| error)?;
            declarations.push(declaration);
        }

        Ok(selectors
            .into_iter()
            .map(|selector| CssRule::new(selector, declarations.clone()))
            .collect())
    }
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
        _declaration_start: &ParserState,
    ) -> std::result::Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let result = (|| {
            Ok(match_ignore_ascii_case! { &name,
            "display" => (CssProperty::Display, CssValue::Display(parse_display(input)?)),
            "box-sizing" => (CssProperty::BoxSizing, CssValue::BoxSizing(parse_box_sizing(input)?)),
            "position" => (CssProperty::Position, CssValue::Position(parse_position(input)?)),
            "direction" => (CssProperty::Direction, CssValue::Direction(parse_direction(input)?)),
            "overflow" => (CssProperty::Overflow, parse_overflow_value(input)?),
            "overflow-x" => (CssProperty::OverflowX, CssValue::Overflow(parse_overflow(input)?)),
            "overflow-y" => (CssProperty::OverflowY, CssValue::Overflow(parse_overflow(input)?)),
            "flex-direction" => (CssProperty::FlexDirection, CssValue::FlexDirection(parse_flex_direction(input)?)),
            "flex-wrap" => (CssProperty::FlexWrap, CssValue::FlexWrap(parse_flex_wrap(input)?)),
            "align-items" => (CssProperty::AlignItems, CssValue::AlignItems(parse_align_items(input)?)),
            "align-self" => (CssProperty::AlignSelf, CssValue::AlignItems(parse_align_items(input)?)),
            "justify-items" => (CssProperty::JustifyItems, CssValue::AlignItems(parse_align_items(input)?)),
            "justify-self" => (CssProperty::JustifySelf, CssValue::AlignItems(parse_align_items(input)?)),
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
            "font-size" => (CssProperty::FontSize, CssValue::Length(parse_font_size(input)?)),
            "line-height" => (CssProperty::LineHeight, CssValue::Length(parse_line_height(input)?)),
            "margin" => (CssProperty::Margin, CssValue::Edges(parse_edges(input, parse_margin_component)?)),
            "padding" => (CssProperty::Padding, CssValue::Edges(parse_edges(input, parse_padding_component)?)),
            "border-width" => (CssProperty::BorderWidth, CssValue::Edges(parse_edges(input, parse_border_width_component)?)),
            "color" => (CssProperty::Color, CssValue::Color(parse_color(input)?)),
            "background" | "background-color" => (CssProperty::Background, CssValue::Color(parse_color(input)?)),
            "border-color" => (CssProperty::BorderColor, CssValue::Color(parse_color(input)?)),
            "opacity" => (CssProperty::Opacity, CssValue::Number(parse_number(input)?)),
            "flex-grow" => (CssProperty::FlexGrow, CssValue::Number(parse_number(input)?)),
            "flex-shrink" => (CssProperty::FlexShrink, CssValue::Number(parse_number(input)?)),
            "aspect-ratio" => (CssProperty::AspectRatio, CssValue::Number(parse_number(input)?)),
            "scrollbar-width" => (CssProperty::ScrollbarWidth, CssValue::Number(parse_number(input)?)),
            _ => return Err(unsupported_property(input, name.as_ref())),
            })
        })()
        .map_err(|error| with_property_context(error, name.as_ref()))?;
        input.expect_exhausted().map_err(basic)?;
        let (property, value) = result;
        Ok(CssDeclaration::new(property, value))
    }
}

fn parse_selector_list<'i, 't>(
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

fn parse_compound_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssSelector, ParseError<'i, Error>> {
    let mut tag_name = None;
    let mut key_name = None;
    let mut class_names = Vec::new();

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

        let state = input.state();
        match input.next() {
            Ok(Token::IDHash(key)) => {
                let key = key.to_string();
                key_name = Some(key);
            }
            Ok(token) => {
                let message = format!("unexpected selector token `{}`", token.to_css_string());
                input.reset(&state);
                if tag_name.is_none() && key_name.is_none() && class_names.is_empty() {
                    return Err(invalid_selector(input, message));
                }
                break;
            }
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(selector_basic(error)),
        }
    }

    if tag_name.is_none() && key_name.is_none() && class_names.is_empty() {
        return Err(invalid_selector(
            input,
            "selector is missing a simple selector",
        ));
    }
    if let (None, None, [class]) = (tag_name.as_ref(), key_name.as_ref(), class_names.as_slice()) {
        return Ok(CssSelector::Class(class.clone()));
    }
    if let (Some(tag), None, []) = (tag_name.as_ref(), key_name.as_ref(), class_names.as_slice()) {
        return Ok(CssSelector::Tag(tag.clone()));
    }
    if let (None, Some(key), []) = (tag_name.as_ref(), key_name.as_ref(), class_names.as_slice()) {
        return Ok(CssSelector::Key(key.clone()));
    }
    Ok(CssSelector::Compound(CssCompoundSelector::new(
        tag_name,
        key_name,
        class_names,
    )))
}

fn parse_display<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssDisplay, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "block" => Ok(CssDisplay::Block),
        "flex" => Ok(CssDisplay::Flex),
        "grid" => Ok(CssDisplay::Grid),
        "inline-block" => Ok(CssDisplay::InlineBlock),
        "inline-grid" => Ok(CssDisplay::InlineGrid),
        "grid-lanes" => Ok(CssDisplay::GridLanes),
        "inline-grid-lanes" => Ok(CssDisplay::InlineGridLanes),
        "none" => Ok(CssDisplay::None),
        _ => Err(unsupported_value(input, None, format!("unsupported display `{ident}`"))),
    }
}

fn parse_box_sizing<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBoxSizing, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "content-box" => Ok(CssBoxSizing::ContentBox),
        "border-box" => Ok(CssBoxSizing::BorderBox),
        _ => Err(unsupported_value(input, None, format!("unsupported box-sizing `{ident}`"))),
    }
}

fn parse_position<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLayoutPosition, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "relative" => Ok(CssLayoutPosition::Relative),
        "absolute" => Ok(CssLayoutPosition::Absolute),
        _ => Err(unsupported_value(input, None, format!("unsupported position `{ident}`"))),
    }
}

fn parse_direction<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssDirection, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "ltr" => Ok(CssDirection::Ltr),
        "rtl" => Ok(CssDirection::Rtl),
        _ => Err(unsupported_value(input, None, format!("unsupported direction `{ident}`"))),
    }
}

fn parse_overflow<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssOverflow, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "visible" => Ok(CssOverflow::Visible),
        "clip" => Ok(CssOverflow::Clip),
        "hidden" => Ok(CssOverflow::Hidden),
        "scroll" => Ok(CssOverflow::Scroll),
        _ => Err(unsupported_value(input, None, format!("unsupported overflow `{ident}`"))),
    }
}

fn parse_overflow_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssValue, ParseError<'i, Error>> {
    let x = parse_overflow(input)?;
    if input.is_exhausted() {
        Ok(CssValue::Overflow(x))
    } else {
        let y = parse_overflow(input)?;
        Ok(CssValue::OverflowAxes(CssOverflowAxes::new(x, y)))
    }
}

fn parse_flex_direction<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFlexDirection, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "row" => Ok(CssFlexDirection::Row),
        "column" => Ok(CssFlexDirection::Column),
        "row-reverse" => Ok(CssFlexDirection::RowReverse),
        "column-reverse" => Ok(CssFlexDirection::ColumnReverse),
        _ => Err(unsupported_value(input, None, format!("unsupported flex-direction `{ident}`"))),
    }
}

fn parse_flex_wrap<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFlexWrap, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "nowrap" => Ok(CssFlexWrap::NoWrap),
        "wrap" => Ok(CssFlexWrap::Wrap),
        "wrap-reverse" => Ok(CssFlexWrap::WrapReverse),
        _ => Err(unsupported_value(input, None, format!("unsupported flex-wrap `{ident}`"))),
    }
}

fn parse_align_items<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAlignItems, ParseError<'i, Error>> {
    let first = input.expect_ident_cloned().map_err(basic)?;
    let first = first.to_ascii_lowercase();
    let safe = first == "safe";
    let has_overflow_prefix = safe || first == "unsafe";
    let keyword = if has_overflow_prefix {
        input
            .expect_ident_cloned()
            .map_err(basic)?
            .to_ascii_lowercase()
    } else {
        first.clone()
    };

    match keyword.as_str() {
        "start" => Ok(CssAlignItems::Start),
        "end" if safe => Ok(CssAlignItems::SafeEnd),
        "end" => Ok(CssAlignItems::End),
        "flex-start" => Ok(CssAlignItems::FlexStart),
        "flex-end" if safe => Ok(CssAlignItems::SafeFlexEnd),
        "flex-end" => Ok(CssAlignItems::FlexEnd),
        "center" if safe => Ok(CssAlignItems::SafeCenter),
        "center" => Ok(CssAlignItems::Center),
        "baseline" if !has_overflow_prefix => Ok(CssAlignItems::Baseline),
        "first" => {
            let baseline = input.expect_ident_cloned().map_err(basic)?;
            if has_overflow_prefix {
                Err(unsupported_value(
                    input,
                    None,
                    format!("unsupported alignment `{first} first {baseline}`"),
                ))
            } else if baseline.eq_ignore_ascii_case("baseline") {
                Ok(CssAlignItems::Baseline)
            } else {
                Err(unsupported_value(
                    input,
                    None,
                    format!("unsupported alignment `first {baseline}`"),
                ))
            }
        }
        "last" => {
            let baseline = input.expect_ident_cloned().map_err(basic)?;
            if has_overflow_prefix {
                Err(unsupported_value(
                    input,
                    None,
                    format!("unsupported alignment `{first} last {baseline}`"),
                ))
            } else if baseline.eq_ignore_ascii_case("baseline") {
                Ok(CssAlignItems::LastBaseline)
            } else {
                Err(unsupported_value(
                    input,
                    None,
                    format!("unsupported alignment `last {baseline}`"),
                ))
            }
        }
        "stretch" => Ok(CssAlignItems::Stretch),
        _ => Err(unsupported_value(
            input,
            None,
            format!("unsupported alignment `{keyword}`"),
        )),
    }
}

fn parse_grid_flow_tolerance<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridFlowTolerance, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "normal" => Ok(CssGridFlowTolerance::Normal),
            "infinite" => Ok(CssGridFlowTolerance::Infinite),
            _ => Err(unsupported_value(input, None, format!("unsupported grid-flow-tolerance `{ident}`"))),
        };
    }

    match parse_box_size_value(input)? {
        CssLength::Percent(value) => Ok(CssGridFlowTolerance::Percent(value)),
        length => Ok(CssGridFlowTolerance::Length(length)),
    }
}

fn parse_edges<'i, 't>(
    input: &mut Parser<'i, 't>,
    mut parse_component: impl FnMut(
        &mut Parser<'i, 't>,
    ) -> std::result::Result<CssLength, ParseError<'i, Error>>,
) -> std::result::Result<CssEdges, ParseError<'i, Error>> {
    let mut values = Vec::new();
    while !input.is_exhausted() {
        values.push(parse_component(input)?);
        if values.len() == 4 && !input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "edge shorthand has too many values",
            ));
        }
    }
    Ok(match values.as_slice() {
        [all] => CssEdges::all(all.clone()),
        [vertical, horizontal] => CssEdges::new(
            vertical.clone(),
            horizontal.clone(),
            vertical.clone(),
            horizontal.clone(),
        ),
        [top, horizontal, bottom] => CssEdges::new(
            top.clone(),
            horizontal.clone(),
            bottom.clone(),
            horizontal.clone(),
        ),
        [top, right, bottom, left] => {
            CssEdges::new(top.clone(), right.clone(), bottom.clone(), left.clone())
        }
        [] => {
            return Err(unsupported_value(
                input,
                None,
                "edge shorthand is missing a value",
            ));
        }
        _ => unreachable!("edge shorthand parser caps values at four"),
    })
}

fn parse_box_size_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::box_size(), "box size")
}

fn parse_margin_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::margin(), "margin")
}

fn parse_padding_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::padding(), "padding")
}

fn parse_border_width_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::border_width(), "border-width")
}

fn parse_gap_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("normal"))
        .is_ok()
    {
        Ok(CssLength::Normal)
    } else {
        parse_length_with(input, LengthOptions::gap(), "gap")
    }
}

fn parse_font_size<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::font_size(), "font-size")
}

fn parse_line_height<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("normal"))
        .is_ok()
    {
        Ok(CssLength::Normal)
    } else {
        parse_length_with(input, LengthOptions::line_height(), "line-height")
    }
}

#[derive(Clone, Copy)]
struct LengthOptions {
    percent: bool,
    auto: bool,
    intrinsic: bool,
    normal: bool,
    calc_percent: bool,
}

impl LengthOptions {
    const fn box_size() -> Self {
        Self {
            percent: true,
            auto: true,
            intrinsic: true,
            normal: false,
            calc_percent: true,
        }
    }

    const fn margin() -> Self {
        Self {
            percent: true,
            auto: true,
            intrinsic: false,
            normal: false,
            calc_percent: true,
        }
    }

    const fn padding() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
        }
    }

    const fn border_width() -> Self {
        Self {
            percent: false,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: false,
        }
    }

    const fn gap() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: true,
            calc_percent: true,
        }
    }

    const fn font_size() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
        }
    }

    const fn line_height() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: true,
            calc_percent: true,
        }
    }
}

fn parse_length_with<'i, 't>(
    input: &mut Parser<'i, 't>,
    options: LengthOptions,
    context: &str,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, unit, .. } if unit.eq_ignore_ascii_case("px") => {
            Ok(CssLength::px(*value))
        }
        Token::Percentage { unit_value, .. } if options.percent => {
            Ok(CssLength::percent(*unit_value * 100.0))
        }
        Token::Percentage { .. } => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported {context} percentage"),
        )),
        Token::Number { value, .. } if *value == 0.0 => Ok(CssLength::Zero),
        Token::Ident(ident) => match_ignore_ascii_case! { ident,
            "auto" if options.auto => Ok(CssLength::Auto),
            "normal" if options.normal => Ok(CssLength::Normal),
            "min-content" if options.intrinsic => Ok(CssLength::MinContent),
            "max-content" if options.intrinsic => Ok(CssLength::MaxContent),
            "fit-content" if options.intrinsic => Ok(CssLength::FitContent),
            _ => Err(unsupported_value_at(
                location,
                None,
                format!("unsupported {context} `{ident}`"),
            )),
        },
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            let calc =
                input.parse_nested_block(|input| parse_calc_length_with_options(input, options))?;
            Ok(CssLength::Calc(calc))
        }
        Token::Function(name) => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported length function `{name}` for {context}"),
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_calc_length_with_options<'i, 't>(
    input: &mut Parser<'i, 't>,
    options: LengthOptions,
) -> std::result::Result<CssCalcLength, ParseError<'i, Error>> {
    let mut terms = Vec::new();
    terms.push(CssCalcLengthTerm::add(parse_calc_component(
        input, options,
    )?));

    while !input.is_exhausted() {
        let location = input.current_source_location();
        let operator = match input.next().map_err(basic)? {
            Token::Delim('+') => CssCalcLengthTerm::add,
            Token::Delim('-') => CssCalcLengthTerm::sub,
            token => {
                return Err(unsupported_value_at(
                    location,
                    None,
                    format!("expected calc operator, got `{}`", token.to_css_string()),
                ));
            }
        };
        let component = parse_calc_component(input, options)?;
        terms.push(operator(component));
    }

    let mut terms = terms.into_iter();
    let first = terms
        .next()
        .expect("calc parser records the first term before parsing operators");
    Ok(CssCalcLength::sum(first, terms))
}

fn parse_calc_component<'i, 't>(
    input: &mut Parser<'i, 't>,
    options: LengthOptions,
) -> std::result::Result<CssCalcLength, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, unit, .. } if unit.eq_ignore_ascii_case("px") => {
            Ok(CssCalcLength::px(*value))
        }
        Token::Dimension { unit, .. } => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported calc length unit `{unit}`"),
        )),
        Token::Percentage { unit_value, .. } if options.calc_percent => {
            Ok(CssCalcLength::percent(*unit_value * 100.0))
        }
        Token::Percentage { .. } => Err(unsupported_value_at(
            location,
            None,
            "unsupported calc percentage",
        )),
        Token::Number { value, .. } if *value == 0.0 => Ok(CssCalcLength::px(0.0)),
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            input.parse_nested_block(|input| parse_calc_length_with_options(input, options))
        }
        Token::Function(name) => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported calc function `{name}`"),
        )),
        token => Err(unsupported_value_at(
            location,
            None,
            format!("unexpected calc token `{}`", token.to_css_string()),
        )),
    }
}

fn parse_number<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<f32, ParseError<'i, Error>> {
    input.expect_number().map_err(basic)
}

fn parse_color<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColor, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::IDHash(hex) | Token::Hash(hex) => color_from_hex(location, hex.as_ref()),
        Token::Ident(ident) => match_ignore_ascii_case! { ident,
            "transparent" => Ok(CssColor::TRANSPARENT),
            "black" => Ok(CssColor::BLACK),
            "white" => Ok(CssColor::WHITE),
            _ => Err(unsupported_value_at(
                location,
                None,
                format!("unsupported color `{ident}`"),
            )),
        },
        token => Err(invalid_syntax(
            location,
            format!("unexpected CSS token `{}`", token.to_css_string()),
        )),
    }
}

fn color_from_hex<'i>(
    location: cssparser::SourceLocation,
    hex: &str,
) -> std::result::Result<CssColor, ParseError<'i, Error>> {
    let expanded = match hex.len() {
        3 => hex.chars().flat_map(|ch| [ch, ch]).collect::<String>(),
        6 => hex.to_owned(),
        _ => {
            return Err(invalid_color(
                location,
                format!("#{hex}"),
                format!("unsupported hex color `#{hex}`"),
            ));
        }
    };
    let value = u32::from_str_radix(&expanded, 16).map_err(|_| {
        invalid_color(
            location,
            format!("#{hex}"),
            format!("invalid hex color `#{hex}`"),
        )
    })?;
    Ok(CssColor::rgba(
        ((value >> 16) & 0xff) as f32 / 255.0,
        ((value >> 8) & 0xff) as f32 / 255.0,
        (value & 0xff) as f32 / 255.0,
        1.0,
    ))
}

fn from_parse_error(error: ParseError<'_, Error>) -> Error {
    match error.kind {
        ParseErrorKind::Custom(error) => error,
        ParseErrorKind::Basic(kind) => basic_error(error.location, kind),
    }
}

fn basic_error(location: cssparser::SourceLocation, kind: BasicParseErrorKind<'_>) -> Error {
    match kind {
        BasicParseErrorKind::EndOfInput => Error::at(
            location,
            ErrorKind::InvalidSyntax {
                reason: "unexpected end of CSS input".to_owned(),
            },
            "unexpected end of CSS input",
        ),
        BasicParseErrorKind::AtRuleInvalid(name) => {
            let name = name.to_string();
            Error::at(
                location,
                ErrorKind::UnsupportedAtRule { name: name.clone() },
                format!("unsupported CSS at-rule `@{name}`"),
            )
        }
        BasicParseErrorKind::QualifiedRuleInvalid => Error::at(
            location,
            ErrorKind::InvalidSyntax {
                reason: "invalid CSS rule".to_owned(),
            },
            "invalid CSS rule",
        ),
        BasicParseErrorKind::AtRuleBodyInvalid => Error::at(
            location,
            ErrorKind::InvalidSyntax {
                reason: "invalid CSS at-rule body".to_owned(),
            },
            "invalid CSS at-rule body",
        ),
        BasicParseErrorKind::UnexpectedToken(token) => {
            let reason = format!("unexpected CSS token `{}`", token.to_css_string());
            Error::at(
                location,
                ErrorKind::InvalidSyntax {
                    reason: reason.clone(),
                },
                reason,
            )
        }
    }
}

fn basic<'i>(error: BasicParseError<'i>) -> ParseError<'i, Error> {
    error.into()
}

fn selector_basic<'i>(error: BasicParseError<'i>) -> ParseError<'i, Error> {
    let location = error.location;
    let reason = basic_error(location, error.kind).message().to_owned();
    invalid_selector_at(location, reason)
}

fn invalid_syntax<'i>(
    location: cssparser::SourceLocation,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    let reason = reason.into();
    error_at(
        location,
        ErrorKind::InvalidSyntax {
            reason: reason.clone(),
        },
        reason,
    )
}

fn invalid_selector<'i, 't>(
    input: &Parser<'i, 't>,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    invalid_selector_at(input.current_source_location(), reason)
}

fn invalid_selector_at<'i>(
    location: cssparser::SourceLocation,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    let reason = reason.into();
    error_at(
        location,
        ErrorKind::InvalidSelector {
            reason: reason.clone(),
        },
        reason,
    )
}

fn unsupported_property<'i, 't>(
    input: &Parser<'i, 't>,
    name: impl Into<String>,
) -> ParseError<'i, Error> {
    let name = name.into();
    error_at(
        input.current_source_location(),
        ErrorKind::UnsupportedProperty { name: name.clone() },
        format!("unsupported CSS property `{name}`"),
    )
}

fn unsupported_value<'i, 't>(
    input: &Parser<'i, 't>,
    property: Option<&str>,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    let reason = reason.into();
    error_at(
        input.current_source_location(),
        ErrorKind::UnsupportedValue {
            property: property.map(str::to_owned),
            reason: reason.clone(),
        },
        reason,
    )
}

fn unsupported_value_at<'i>(
    location: cssparser::SourceLocation,
    property: Option<&str>,
    reason: impl Into<String>,
) -> ParseError<'i, Error> {
    let reason = reason.into();
    error_at(
        location,
        ErrorKind::UnsupportedValue {
            property: property.map(str::to_owned),
            reason: reason.clone(),
        },
        reason,
    )
}

fn invalid_color<'i>(
    location: cssparser::SourceLocation,
    value: impl Into<String>,
    message: impl Into<String>,
) -> ParseError<'i, Error> {
    error_at(
        location,
        ErrorKind::InvalidColor {
            value: value.into(),
        },
        message,
    )
}

fn with_property_context<'i>(
    mut error: ParseError<'i, Error>,
    property: &str,
) -> ParseError<'i, Error> {
    if let ParseErrorKind::Custom(Error {
        kind: ErrorKind::UnsupportedValue {
            property: context, ..
        },
        ..
    }) = &mut error.kind
        && context.is_none()
    {
        *context = Some(property.to_owned());
    }
    error
}

fn error_at<'i>(
    location: cssparser::SourceLocation,
    kind: ErrorKind,
    message: impl Into<String>,
) -> ParseError<'i, Error> {
    ParseError {
        kind: ParseErrorKind::Custom(Error::at(location, kind, message)),
        location,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declaration_value(input: &str, property: CssProperty) -> CssValue {
        let sheet = parse_sheet(input).unwrap();
        sheet.rules()[0]
            .declarations()
            .iter()
            .find(|declaration| declaration.property() == property)
            .unwrap()
            .value()
            .clone()
    }

    #[test]
    fn parses_calc_width_as_css_calc_length() {
        let value = declaration_value(".panel { width: calc(20px + 10%); }", CssProperty::Width);

        match value {
            CssValue::Length(CssLength::Calc(calc)) => {
                assert!(calc.uses_percentage());
                assert_eq!(calc.to_css_string(), "calc(20px + 10%)");
            }
            other => panic!("expected calc length, got {other:?}"),
        }
    }

    #[test]
    fn parses_nested_calc_width_with_subtraction_as_css_syntax() {
        let value = declaration_value(
            ".panel { width: calc(100% - calc(12px + 3%)); }",
            CssProperty::Width,
        );

        match value {
            CssValue::Length(CssLength::Calc(calc)) => {
                assert!(calc.uses_percentage());
                assert_eq!(calc.to_css_string(), "calc(100% - calc(12px + 3%))");
            }
            other => panic!("expected nested calc length, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unsupported_calc_units() {
        let error = parse_sheet(".panel { width: calc(1em + 2px); }").unwrap_err();
        assert!(error.message().contains("unsupported calc length unit"));
    }

    #[test]
    fn unsupported_property_has_typed_error_kind() {
        let error = parse_sheet(".panel { float: left; }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedProperty {
                name: "float".to_owned(),
            }
        );
        assert!(error.message().contains("unsupported CSS property `float`"));
    }

    #[test]
    fn unsupported_calc_unit_has_typed_error_kind() {
        let error = parse_sheet(".panel { width: calc(1em + 2px); }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedValue {
                property: Some("width".to_owned()),
                reason: "unsupported calc length unit `em`".to_owned(),
            }
        );
        assert!(
            error
                .message()
                .contains("unsupported calc length unit `em`")
        );
    }

    #[test]
    fn selector_parse_failure_has_typed_error_kind() {
        let error = parse_sheet("??? { width: 10px; }").unwrap_err();

        assert!(matches!(error.kind(), ErrorKind::InvalidSelector { .. }));
        assert!(error.message().contains("unexpected selector token"));
    }

    #[test]
    fn selector_missing_class_name_has_typed_error_kind() {
        let error = parse_sheet(". { width: 10px; }").unwrap_err();

        assert!(matches!(error.kind(), ErrorKind::InvalidSelector { .. }));
    }

    #[test]
    fn grid_flow_tolerance_calc_is_preserved_as_css_syntax() {
        let value = declaration_value(
            ".panel { grid-flow-tolerance: calc(8px + 2%); }",
            CssProperty::GridFlowTolerance,
        );

        match value {
            CssValue::GridFlowTolerance(CssGridFlowTolerance::Length(CssLength::Calc(calc))) => {
                assert!(calc.uses_percentage());
                assert_eq!(calc.to_css_string(), "calc(8px + 2%)");
            }
            other => panic!("expected calc grid-flow-tolerance, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unknown_calc_functions() {
        let error = parse_sheet(".panel { width: min(10px, 20px); }").unwrap_err();
        assert!(error.message().contains("unsupported length function"));
    }

    #[test]
    fn parses_calc_in_edge_shorthands() {
        let sheet = parse_sheet(".panel { margin: calc(4px + 1%) 2px; }").unwrap();
        let edges = match declaration_value(
            ".panel { margin: calc(4px + 1%) 2px; }",
            CssProperty::Margin,
        ) {
            CssValue::Edges(edges) => edges,
            other => panic!("expected edges, got {other:?}"),
        };

        match &edges.top {
            CssLength::Calc(calc) => {
                assert!(calc.uses_percentage());
                assert_eq!(calc.to_css_string(), "calc(4px + 1%)");
            }
            other => panic!("expected calc top edge, got {other:?}"),
        }
        assert_eq!(edges.right, CssLength::px(2.0));
        match &edges.bottom {
            CssLength::Calc(calc) => {
                assert!(calc.uses_percentage());
                assert_eq!(calc.to_css_string(), "calc(4px + 1%)");
            }
            other => panic!("expected calc bottom edge, got {other:?}"),
        }
        assert_eq!(edges.left, CssLength::px(2.0));

        assert_eq!(sheet.rules()[0].declarations().len(), 1);
    }

    #[test]
    fn parses_authored_normal_gap_without_canonicalizing_it() {
        let value = declaration_value(".panel { gap: normal; }", CssProperty::Gap);
        assert_eq!(value, CssValue::Length(CssLength::Normal));
    }

    #[test]
    fn parses_authored_calc_gap_without_canonicalizing_it() {
        let value = declaration_value(".panel { gap: calc(8px + 2%); }", CssProperty::Gap);
        match value {
            CssValue::Length(CssLength::Calc(calc)) => {
                assert!(calc.uses_percentage());
                assert_eq!(calc.to_css_string(), "calc(8px + 2%)");
            }
            other => panic!("expected calc gap, got {other:?}"),
        }
    }

    #[test]
    fn rejects_line_height_auto() {
        let error = parse_sheet(".panel { line-height: auto; }").unwrap_err();
        assert!(error.message().contains("unsupported line-height"));
    }

    #[test]
    fn rejects_line_height_min_content() {
        let error = parse_sheet(".panel { line-height: min-content; }").unwrap_err();
        assert!(error.message().contains("unsupported line-height"));
    }

    #[test]
    fn rejects_font_size_auto() {
        let error = parse_sheet(".panel { font-size: auto; }").unwrap_err();
        assert!(error.message().contains("unsupported font-size"));
    }

    #[test]
    fn rejects_padding_auto() {
        let error = parse_sheet(".panel { padding: auto; }").unwrap_err();
        assert!(error.message().contains("unsupported padding"));
    }

    #[test]
    fn rejects_border_width_percent() {
        let error = parse_sheet(".panel { border-width: 10%; }").unwrap_err();
        assert!(error.message().contains("unsupported border-width"));
    }

    #[test]
    fn rejects_gap_auto() {
        let error = parse_sheet(".panel { gap: auto; }").unwrap_err();
        assert!(error.message().contains("unsupported gap"));
    }

    #[test]
    fn accepts_margin_auto() {
        assert_eq!(
            declaration_value(".panel { margin: auto; }", CssProperty::Margin),
            CssValue::Edges(CssEdges::all(CssLength::Auto))
        );
    }
}

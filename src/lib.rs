//! Strict CSS ingestion for Surgeist style sheets.
//!
//! This module parses CSS syntax into CSS-owned authored syntax values. It is
//! strict by design: unsupported selectors, at-rules, properties, and values are
//! errors instead of browser-style recoverable invalid declarations.
//!
//! Parse failures expose typed [`ErrorKind`] values plus source line and column
//! information so callers do not need to parse display strings.

use std::{collections::HashMap, fmt};

use cssparser::{
    AtRuleParser, BasicParseError, BasicParseErrorKind, CowRcStr, DeclarationParser, ParseError,
    ParseErrorKind, Parser, ParserInput, ParserState, QualifiedRuleParser, RuleBodyItemParser,
    RuleBodyParser, StyleSheetParser, ToCss, Token, match_ignore_ascii_case,
};

mod syntax;
mod validation;

pub use syntax::*;

use validation::{
    LengthUnitStatus, PropertyNameStatus, classify_length_unit, classify_property_name,
    parse_global_keyword, unsupported_keyword_reason,
};

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
    UnknownProperty {
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
        declaration_start: &ParserState,
    ) -> std::result::Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let location = CssSourceLocation::from_cssparser(declaration_start.source_location());
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
            "background" | "background-color" => (CssProperty::Background, CssValue::Color(parse_color(input)?)),
            "border-color" => (CssProperty::BorderColor, CssValue::Color(parse_color(input)?)),
            "border-top-color" => (CssProperty::BorderTopColor, CssValue::Color(parse_color(input)?)),
            "border-right-color" => (CssProperty::BorderRightColor, CssValue::Color(parse_color(input)?)),
            "border-bottom-color" => (CssProperty::BorderBottomColor, CssValue::Color(parse_color(input)?)),
            "border-left-color" => (CssProperty::BorderLeftColor, CssValue::Color(parse_color(input)?)),
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
            "opacity" => (CssProperty::Opacity, CssValue::Number(parse_number(input)?)),
            "flex-grow" => (CssProperty::FlexGrow, CssValue::Number(parse_number(input)?)),
            "flex-shrink" => (CssProperty::FlexShrink, CssValue::Number(parse_number(input)?)),
            "order" => (CssProperty::Order, CssValue::Order(parse_order(input)?)),
            "flex" => (CssProperty::Flex, CssValue::Flex(parse_flex(input)?)),
            "justify-tracks" => (CssProperty::JustifyTracks, CssValue::Alignment(parse_content_alignment(input)?)),
            "align-tracks" => (CssProperty::AlignTracks, CssValue::Alignment(parse_content_alignment(input)?)),
            "aspect-ratio" => (CssProperty::AspectRatio, CssValue::Number(parse_number(input)?)),
            "scrollbar-width" => (CssProperty::ScrollbarWidth, CssValue::Number(parse_number(input)?)),
            _ => return Err(property_name_error(input, name.as_ref())),
            })
        })()
        .map_err(|error| with_property_context(error, name.as_ref()))?;
        input.expect_exhausted().map_err(basic)?;
        let (property, value) = result;
        Ok(CssDeclaration::new(property, value, location))
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
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("display", ident.as_ref()),
        )),
    }
}

fn parse_box_sizing<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBoxSizing, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "content-box" => Ok(CssBoxSizing::ContentBox),
        "border-box" => Ok(CssBoxSizing::BorderBox),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("box-sizing", ident.as_ref()),
        )),
    }
}

fn parse_position<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLayoutPosition, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "static" => Ok(CssLayoutPosition::Static),
        "relative" => Ok(CssLayoutPosition::Relative),
        "absolute" => Ok(CssLayoutPosition::Absolute),
        "fixed" => Ok(CssLayoutPosition::Fixed),
        "sticky" => Ok(CssLayoutPosition::Sticky),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("position", ident.as_ref()),
        )),
    }
}

fn parse_direction<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssDirection, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "ltr" => Ok(CssDirection::Ltr),
        "rtl" => Ok(CssDirection::Rtl),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("direction", ident.as_ref()),
        )),
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
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("overflow", ident.as_ref()),
        )),
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
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("flex-direction", ident.as_ref()),
        )),
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
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("flex-wrap", ident.as_ref()),
        )),
    }
}

fn parse_float<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFloat, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "left" => Ok(CssFloat::Left),
        "right" => Ok(CssFloat::Right),
        "none" => Ok(CssFloat::None),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("float", ident.as_ref()),
        )),
    }
}

fn parse_clear<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssClear, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "left" => Ok(CssClear::Left),
        "right" => Ok(CssClear::Right),
        "both" => Ok(CssClear::Both),
        "none" => Ok(CssClear::None),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("clear", ident.as_ref()),
        )),
    }
}

#[derive(Clone, Copy)]
struct AlignmentOptions {
    normal: bool,
    distribution: bool,
}

impl AlignmentOptions {
    const fn content() -> Self {
        Self {
            normal: true,
            distribution: true,
        }
    }

    const fn item() -> Self {
        Self {
            normal: true,
            distribution: false,
        }
    }
}

fn parse_content_alignment<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAlignment, ParseError<'i, Error>> {
    parse_alignment(input, AlignmentOptions::content())
}

fn parse_align_items<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAlignItems, ParseError<'i, Error>> {
    let alignment = parse_alignment(input, AlignmentOptions::item())?;
    Ok(match alignment {
        CssAlignment::Normal => CssAlignItems::Normal,
        CssAlignment::Start => CssAlignItems::Start,
        CssAlignment::End => CssAlignItems::End,
        CssAlignment::SafeEnd => CssAlignItems::SafeEnd,
        CssAlignment::FlexStart => CssAlignItems::FlexStart,
        CssAlignment::FlexEnd => CssAlignItems::FlexEnd,
        CssAlignment::SafeFlexEnd => CssAlignItems::SafeFlexEnd,
        CssAlignment::Center => CssAlignItems::Center,
        CssAlignment::SafeCenter => CssAlignItems::SafeCenter,
        CssAlignment::Baseline => CssAlignItems::Baseline,
        CssAlignment::FirstBaseline => CssAlignItems::FirstBaseline,
        CssAlignment::LastBaseline => CssAlignItems::LastBaseline,
        CssAlignment::Stretch => CssAlignItems::Stretch,
        CssAlignment::SpaceBetween | CssAlignment::SpaceAround | CssAlignment::SpaceEvenly => {
            unreachable!("item alignment parser disables distribution keywords")
        }
    })
}

fn parse_alignment<'i, 't>(
    input: &mut Parser<'i, 't>,
    options: AlignmentOptions,
) -> std::result::Result<CssAlignment, ParseError<'i, Error>> {
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
    let original = if has_overflow_prefix {
        format!("{first} {keyword}")
    } else {
        keyword.clone()
    };

    if first == "unsafe" {
        return Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("alignment", original),
        ));
    }

    match keyword.as_str() {
        "normal" if options.normal && !has_overflow_prefix => Ok(CssAlignment::Normal),
        "start" if !has_overflow_prefix => Ok(CssAlignment::Start),
        "end" if safe => Ok(CssAlignment::SafeEnd),
        "end" => Ok(CssAlignment::End),
        "flex-start" if !has_overflow_prefix => Ok(CssAlignment::FlexStart),
        "flex-end" if safe => Ok(CssAlignment::SafeFlexEnd),
        "flex-end" => Ok(CssAlignment::FlexEnd),
        "center" if safe => Ok(CssAlignment::SafeCenter),
        "center" => Ok(CssAlignment::Center),
        "baseline" if !has_overflow_prefix => Ok(CssAlignment::Baseline),
        "first" => {
            let baseline = input.expect_ident_cloned().map_err(basic)?;
            if has_overflow_prefix {
                Err(unsupported_value(
                    input,
                    None,
                    unsupported_keyword_reason("alignment", format!("{first} first {baseline}")),
                ))
            } else if baseline.eq_ignore_ascii_case("baseline") {
                Ok(CssAlignment::FirstBaseline)
            } else {
                Err(unsupported_value(
                    input,
                    None,
                    unsupported_keyword_reason("alignment", format!("first {baseline}")),
                ))
            }
        }
        "last" => {
            let baseline = input.expect_ident_cloned().map_err(basic)?;
            if has_overflow_prefix {
                Err(unsupported_value(
                    input,
                    None,
                    unsupported_keyword_reason("alignment", format!("{first} last {baseline}")),
                ))
            } else if baseline.eq_ignore_ascii_case("baseline") {
                Ok(CssAlignment::LastBaseline)
            } else {
                Err(unsupported_value(
                    input,
                    None,
                    unsupported_keyword_reason("alignment", format!("last {baseline}")),
                ))
            }
        }
        "stretch" if !has_overflow_prefix => Ok(CssAlignment::Stretch),
        "space-between" if options.distribution && !has_overflow_prefix => {
            Ok(CssAlignment::SpaceBetween)
        }
        "space-around" if options.distribution && !has_overflow_prefix => {
            Ok(CssAlignment::SpaceAround)
        }
        "space-evenly" if options.distribution && !has_overflow_prefix => {
            Ok(CssAlignment::SpaceEvenly)
        }
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("alignment", original),
        )),
    }
}

fn parse_place_alignment<'i, 't, T: Copy>(
    input: &mut Parser<'i, 't>,
    mut parse_component: impl FnMut(
        &mut Parser<'i, 't>,
    ) -> std::result::Result<T, ParseError<'i, Error>>,
    make: impl Fn(T, T) -> CssPlaceAlignment,
) -> std::result::Result<CssPlaceAlignment, ParseError<'i, Error>> {
    let first = parse_component(input)?;
    let second = if input.is_exhausted() {
        first
    } else {
        parse_component(input)?
    };
    if !input.is_exhausted() {
        return Err(unsupported_value(
            input,
            None,
            "place alignment shorthand has too many values",
        ));
    }
    Ok(make(first, second))
}

fn parse_visibility<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssVisibility, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "visible" => Ok(CssVisibility::Visible),
        "hidden" => Ok(CssVisibility::Hidden),
        "collapse" => Ok(CssVisibility::Collapse),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("visibility", ident.as_ref()),
        )),
    }
}

fn parse_content_visibility<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssContentVisibility, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "visible" => Ok(CssContentVisibility::Visible),
        "hidden" => Ok(CssContentVisibility::Hidden),
        "auto" => Ok(CssContentVisibility::Auto),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("content-visibility", ident.as_ref()),
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
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("grid-flow-tolerance", ident.as_ref()),
            )),
        };
    }

    match parse_box_size_value(input)? {
        CssLength::Percent(value) => Ok(CssGridFlowTolerance::Percent(value)),
        length => Ok(CssGridFlowTolerance::Length(length)),
    }
}

fn parse_grid_track_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridTrackList, ParseError<'i, Error>> {
    parse_grid_track_list_until_slash(input, false)
}

fn parse_grid_track_list_until_slash<'i, 't>(
    input: &mut Parser<'i, 't>,
    stop_at_slash: bool,
) -> std::result::Result<CssGridTrackList, ParseError<'i, Error>> {
    let mut components = Vec::new();
    while !input.is_exhausted() {
        if stop_at_slash && next_is_delim(input, '/') {
            break;
        }
        components.push(parse_grid_track_component(input)?);
    }
    if components.is_empty() {
        Err(unsupported_value(
            input,
            None,
            "grid track list is missing a track",
        ))
    } else {
        Ok(CssGridTrackList::new(components))
    }
}

fn parse_grid_track_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridTrackComponent, ParseError<'i, Error>> {
    let state = input.state();
    match input.next().map_err(basic)? {
        Token::SquareBracketBlock => {
            return input
                .parse_nested_block(parse_grid_line_names)
                .map(CssGridTrackComponent::LineNames);
        }
        Token::Function(name) if name.eq_ignore_ascii_case("repeat") => {
            return input
                .parse_nested_block(parse_grid_repeat)
                .map(CssGridTrackComponent::Repeat);
        }
        _ => input.reset(&state),
    }

    parse_grid_track_size(input).map(CssGridTrackComponent::TrackSize)
}

fn parse_grid_line_names<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridLineNames, ParseError<'i, Error>> {
    let mut names = Vec::new();
    while !input.is_exhausted() {
        let location = input.current_source_location();
        let ident = input.expect_ident_cloned().map_err(basic)?;
        names.push(parse_custom_ident_from_str_at(
            "grid line name",
            ident.as_ref(),
            location,
        )?);
    }
    if names.is_empty() {
        Err(unsupported_value(input, None, "grid line names are empty"))
    } else {
        Ok(CssGridLineNames::new(names))
    }
}

fn parse_grid_repeat<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridRepeat, ParseError<'i, Error>> {
    let count = if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        match_ignore_ascii_case! { &ident,
            "auto-fill" => CssGridRepeatCount::AutoFill,
            "auto-fit" => CssGridRepeatCount::AutoFit,
            _ => return Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("grid repeat count", ident.as_ref()),
            )),
        }
    } else {
        let count = parse_positive_integer(input, "grid repeat count")?;
        CssGridRepeatCount::integer(count)
    };

    input.expect_comma().map_err(basic)?;
    let tracks = parse_grid_track_list(input)?;
    Ok(CssGridRepeat::new(count, tracks))
}

fn parse_grid_track_size<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridTrackSize, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let state = input.state();
    match input.next().map_err(basic)? {
        Token::Function(name) if name.eq_ignore_ascii_case("minmax") => {
            input.parse_nested_block(|input| {
                let min = parse_grid_track_breadth(input)?;
                input.expect_comma().map_err(basic)?;
                let max = parse_grid_track_breadth(input)?;
                input.expect_exhausted().map_err(basic)?;
                Ok(CssGridTrackSize::minmax(min, max))
            })
        }
        Token::Function(name) if name.eq_ignore_ascii_case("fit-content") => input
            .parse_nested_block(|input| {
                let limit =
                    parse_length_with(input, LengthOptions::grid_track(), "grid fit-content")?;
                input.expect_exhausted().map_err(basic)?;
                Ok(CssGridTrackSize::fit_content(limit))
            }),
        Token::Function(name) if name.eq_ignore_ascii_case("repeat") => Err(unsupported_value_at(
            location,
            None,
            "repeat() is a grid track list component, not a track size",
        )),
        _ => {
            input.reset(&state);
            parse_grid_track_breadth(input).map(CssGridTrackSize::breadth)
        }
    }
}

fn parse_grid_track_breadth<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridTrackBreadth, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, unit, .. } if unit.eq_ignore_ascii_case("fr") => {
            if *value < 0.0 {
                Err(unsupported_value_at(
                    location,
                    None,
                    "unsupported negative grid flex fraction",
                ))
            } else {
                Ok(CssGridTrackBreadth::Fraction(*value))
            }
        }
        Token::Dimension { value, unit, .. } => match classify_length_unit(unit) {
            LengthUnitStatus::Supported(_) if *value < 0.0 => Err(unsupported_value_at(
                location,
                None,
                "unsupported negative grid track length",
            )),
            LengthUnitStatus::Supported(unit) => Ok(CssGridTrackBreadth::length(
                CssLength::dimension(*value, unit),
            )),
            LengthUnitStatus::Unknown => Err(unsupported_value_at(
                location,
                None,
                format!("unknown grid track unit `{unit}`"),
            )),
        },
        Token::Percentage { unit_value, .. } if *unit_value < 0.0 => Err(unsupported_value_at(
            location,
            None,
            "unsupported negative grid track percentage",
        )),
        Token::Percentage { unit_value, .. } => Ok(CssGridTrackBreadth::length(
            CssLength::percent(*unit_value * 100.0),
        )),
        Token::Number { value, .. } if *value == 0.0 => {
            Ok(CssGridTrackBreadth::length(CssLength::Zero))
        }
        Token::Ident(ident) => match_ignore_ascii_case! { ident,
            "min-content" => Ok(CssGridTrackBreadth::MinContent),
            "max-content" => Ok(CssGridTrackBreadth::MaxContent),
            "auto" => Ok(CssGridTrackBreadth::Auto),
            _ => Err(unsupported_value_at(
                location,
                None,
                unsupported_keyword_reason("grid track", ident.as_ref()),
            )),
        },
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            let calc = input.parse_nested_block(|input| {
                parse_calc_length_with_options(input, LengthOptions::grid_track())
            })?;
            if syntax::calc_has_negative_component(&calc) {
                return Err(unsupported_value_at(
                    location,
                    None,
                    "unsupported negative grid track calc component",
                ));
            }
            Ok(CssGridTrackBreadth::length(CssLength::Calc(calc)))
        }
        Token::Function(name) => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported grid track function `{name}`"),
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_grid_template_areas<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridTemplateAreas, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "none" => Ok(CssGridTemplateAreas::None),
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("grid-template-areas", ident.as_ref()),
            )),
        };
    }

    let mut rows = Vec::new();
    while !input.is_exhausted() {
        let location = input.current_source_location();
        let row = input.expect_string_cloned().map_err(basic)?;
        rows.push(parse_grid_template_area_row(row.as_ref(), location)?);
    }
    validate_grid_template_area_rectangles(&rows, input)?;
    Ok(CssGridTemplateAreas::rows(rows))
}

fn parse_grid_template_area_row<'i>(
    row: &str,
    location: cssparser::SourceLocation,
) -> std::result::Result<CssGridTemplateAreaRow, ParseError<'i, Error>> {
    let cells = row
        .split_whitespace()
        .map(|token| {
            if token.chars().all(|ch| ch == '.') {
                Ok(CssGridTemplateAreaCell::Empty)
            } else if token.contains('.') {
                Err(error_at(
                    location,
                    ErrorKind::UnsupportedValue {
                        property: None,
                        reason: format!("invalid grid template area token `{token}`"),
                    },
                    format!("invalid grid template area token `{token}`"),
                ))
            } else {
                parse_custom_ident_from_str_at("grid template area", token, location)
                    .map(CssGridTemplateAreaCell::Named)
            }
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;

    if cells.is_empty() {
        Err(error_at(
            location,
            ErrorKind::UnsupportedValue {
                property: None,
                reason: "grid template area row is empty".to_owned(),
            },
            "grid template area row is empty",
        ))
    } else {
        Ok(CssGridTemplateAreaRow::new(cells))
    }
}

#[derive(Clone, Copy)]
struct GridAreaBounds {
    min_row: usize,
    max_row: usize,
    min_col: usize,
    max_col: usize,
    count: usize,
}

fn validate_grid_template_area_rectangles<'i, 't>(
    rows: &[CssGridTemplateAreaRow],
    input: &Parser<'i, 't>,
) -> std::result::Result<(), ParseError<'i, Error>> {
    if rows.is_empty() {
        return Err(unsupported_value(
            input,
            None,
            "grid-template-areas is missing rows",
        ));
    }

    let width = rows[0].cells().len();
    let mut bounds = HashMap::<String, GridAreaBounds>::new();
    for (row_index, row) in rows.iter().enumerate() {
        if row.cells().len() != width {
            return Err(unsupported_value(
                input,
                None,
                "grid-template-areas rows have inconsistent widths",
            ));
        }
        for (col_index, cell) in row.cells().iter().enumerate() {
            let CssGridTemplateAreaCell::Named(name) = cell else {
                continue;
            };
            bounds
                .entry(name.as_str().to_owned())
                .and_modify(|bounds| {
                    bounds.min_row = bounds.min_row.min(row_index);
                    bounds.max_row = bounds.max_row.max(row_index);
                    bounds.min_col = bounds.min_col.min(col_index);
                    bounds.max_col = bounds.max_col.max(col_index);
                    bounds.count += 1;
                })
                .or_insert(GridAreaBounds {
                    min_row: row_index,
                    max_row: row_index,
                    min_col: col_index,
                    max_col: col_index,
                    count: 1,
                });
        }
    }

    for (name, bounds) in bounds {
        let rectangle_area =
            (bounds.max_row - bounds.min_row + 1) * (bounds.max_col - bounds.min_col + 1);
        if rectangle_area != bounds.count {
            return Err(unsupported_value(
                input,
                None,
                format!("grid template area `{name}` is not rectangular"),
            ));
        }
    }
    Ok(())
}

fn parse_grid_template<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridTemplate, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "none" => Ok(CssGridTemplate::None),
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("grid-template", ident.as_ref()),
            )),
        };
    }

    let rows = parse_grid_track_list_until_slash(input, true)?;
    let columns = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(parse_grid_track_list(input)?)
    } else {
        None
    };
    Ok(CssGridTemplate::RowsColumns { rows, columns })
}

fn parse_grid_auto_flow<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridAutoFlow, ParseError<'i, Error>> {
    let axis = parse_grid_auto_flow_axis(input)?;
    let dense = input
        .try_parse(|input| input.expect_ident_matching("dense"))
        .is_ok();
    Ok(CssGridAutoFlow::new(axis, dense))
}

fn parse_grid_auto_flow_axis<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridAutoFlowAxis, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "row" => Ok(CssGridAutoFlowAxis::Row),
        "column" => Ok(CssGridAutoFlowAxis::Column),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("grid-auto-flow", ident.as_ref()),
        )),
    }
}

fn parse_grid_line<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridLine, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Ident(ident) if ident.eq_ignore_ascii_case("auto") => Ok(CssGridLine::Auto),
        Token::Ident(ident) if ident.eq_ignore_ascii_case("span") => parse_grid_line_span(input),
        Token::Ident(ident) => {
            parse_custom_ident_from_str_at("grid line", ident.as_ref(), location)
                .map(CssGridLine::CustomIdent)
        }
        Token::Number {
            int_value: Some(value),
            ..
        } if *value != 0 => Ok(CssGridLine::integer(*value)),
        Token::Number {
            int_value: Some(_), ..
        } => Err(unsupported_value_at(
            location,
            None,
            "grid line integer must not be zero",
        )),
        Token::Number { .. } => Err(unsupported_value_at(
            location,
            None,
            "grid line number must be an integer",
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_grid_line_span<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridLine, ParseError<'i, Error>> {
    let mut integer = None;
    let mut name = None;

    while !input.is_exhausted() && !next_is_delim(input, '/') {
        if integer.is_none() {
            let parsed = input.try_parse(|input| parse_positive_integer(input, "grid span"));
            if let Ok(value) = parsed {
                integer = Some(value);
                continue;
            }
        }

        if name.is_none() {
            let location = input.current_source_location();
            let parsed = input.try_parse(Parser::expect_ident_cloned);
            if let Ok(ident) = parsed {
                name = Some(parse_custom_ident_from_str_at(
                    "grid span",
                    ident.as_ref(),
                    location,
                )?);
                continue;
            }
        }

        return Err(unsupported_value(
            input,
            None,
            "unsupported grid span component",
        ));
    }

    if integer.is_none() && name.is_none() {
        Err(unsupported_value(
            input,
            None,
            "grid span is missing an integer or name",
        ))
    } else {
        Ok(CssGridLine::span(integer, name))
    }
}

fn parse_grid_line_range<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridLineRange, ParseError<'i, Error>> {
    let start = parse_grid_line(input)?;
    let end = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(parse_grid_line(input)?)
    } else {
        None
    };
    Ok(CssGridLineRange::new(start, end))
}

fn parse_grid_area<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridArea, ParseError<'i, Error>> {
    let row_start = parse_grid_line(input)?;
    let column_start = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(parse_grid_line(input)?)
    } else {
        None
    };
    let row_end = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(parse_grid_line(input)?)
    } else {
        None
    };
    let column_end = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(parse_grid_line(input)?)
    } else {
        None
    };
    Ok(CssGridArea::new(
        row_start,
        column_start,
        row_end,
        column_end,
    ))
}

fn parse_grid<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGrid, ParseError<'i, Error>> {
    if let Ok(grid) = input.try_parse(parse_grid_auto_flow_shorthand) {
        Ok(grid)
    } else {
        parse_grid_template(input).map(CssGrid::Template)
    }
}

fn parse_grid_auto_flow_shorthand<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGrid, ParseError<'i, Error>> {
    input.expect_ident_matching("auto-flow").map_err(basic)?;
    let dense = input
        .try_parse(|input| input.expect_ident_matching("dense"))
        .is_ok();
    let auto_tracks = if !input.is_exhausted() && !next_is_delim(input, '/') {
        Some(parse_grid_track_list_until_slash(input, true)?)
    } else {
        None
    };
    input.expect_delim('/').map_err(basic)?;
    let explicit_tracks = parse_grid_track_list(input)?;
    Ok(CssGrid::AutoFlow {
        flow: CssGridAutoFlow::new(CssGridAutoFlowAxis::Row, dense),
        auto_tracks,
        explicit_tracks,
    })
}

fn parse_order<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssOrder, ParseError<'i, Error>> {
    parse_integer(input, "order").map(CssOrder::Integer)
}

fn parse_flex<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFlex, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "none" => Ok(CssFlex::None),
            "auto" => Ok(CssFlex::Auto),
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("flex", ident.as_ref()),
            )),
        };
    }

    let grow = parse_non_negative_number(input, "flex-grow")?;
    let mut shrink = None;
    let mut basis = None;
    if !input.is_exhausted() {
        if let Ok(parsed_shrink) =
            input.try_parse(|input| parse_non_negative_number(input, "flex-shrink"))
        {
            shrink = Some(parsed_shrink);
            if !input.is_exhausted() {
                basis = Some(parse_box_size_value(input)?);
            }
        } else {
            basis = Some(parse_box_size_value(input)?);
        }
    }
    Ok(CssFlex::Components {
        grow,
        shrink,
        basis,
    })
}

fn parse_z_index<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssZIndex, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Ident(ident) if ident.eq_ignore_ascii_case("auto") => Ok(CssZIndex::Auto),
        Token::Ident(ident) => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported z-index `{ident}`"),
        )),
        Token::Number {
            int_value: Some(value),
            ..
        } => Ok(CssZIndex::Integer(*value)),
        Token::Number { .. } => Err(unsupported_value_at(
            location,
            None,
            "unsupported z-index non-integer number",
        )),
        Token::Dimension { unit, .. } => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported z-index length unit `{unit}`"),
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_box_decoration_break<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBoxDecorationBreak, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "slice" => Ok(CssBoxDecorationBreak::Slice),
        "clone" => Ok(CssBoxDecorationBreak::Clone),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("box-decoration-break", ident.as_ref()),
        )),
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

fn parse_border_styles<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBorderStyles, ParseError<'i, Error>> {
    let mut values = Vec::new();
    while !input.is_exhausted() {
        values.push(parse_border_style(input)?);
        if values.len() == 4 && !input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "border-style shorthand has too many values",
            ));
        }
    }
    Ok(match values.as_slice() {
        [all] => CssBorderStyles::all(*all),
        [vertical, horizontal] => {
            CssBorderStyles::new(*vertical, *horizontal, *vertical, *horizontal)
        }
        [top, horizontal, bottom] => CssBorderStyles::new(*top, *horizontal, *bottom, *horizontal),
        [top, right, bottom, left] => CssBorderStyles::new(*top, *right, *bottom, *left),
        [] => {
            return Err(unsupported_value(
                input,
                None,
                "border-style shorthand is missing a value",
            ));
        }
        _ => unreachable!("border-style shorthand parser caps values at four"),
    })
}

fn parse_border_style<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBorderStyle, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "none" => Ok(CssBorderStyle::None),
        "hidden" => Ok(CssBorderStyle::Hidden),
        "dotted" => Ok(CssBorderStyle::Dotted),
        "dashed" => Ok(CssBorderStyle::Dashed),
        "solid" => Ok(CssBorderStyle::Solid),
        "double" => Ok(CssBorderStyle::Double),
        "groove" => Ok(CssBorderStyle::Groove),
        "ridge" => Ok(CssBorderStyle::Ridge),
        "inset" => Ok(CssBorderStyle::Inset),
        "outset" => Ok(CssBorderStyle::Outset),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("border-style", ident.as_ref()),
        )),
    }
}

fn parse_border<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBorder, ParseError<'i, Error>> {
    let mut width = None;
    let mut style = None;
    let mut color = None;

    while !input.is_exhausted() {
        if let Ok(parsed_width) = input.try_parse(parse_border_width_component) {
            if width.replace(parsed_width).is_some() {
                return Err(unsupported_value(input, None, "duplicate border width"));
            }
            continue;
        }
        if let Ok(parsed_style) = input.try_parse(parse_border_style) {
            if style.replace(parsed_style).is_some() {
                return Err(unsupported_value(input, None, "duplicate border style"));
            }
            continue;
        }
        if let Ok(parsed_color) = input.try_parse(parse_color) {
            if color.replace(parsed_color).is_some() {
                return Err(unsupported_value(input, None, "duplicate border color"));
            }
            continue;
        }

        return Err(unsupported_value(
            input,
            None,
            "unsupported border component",
        ));
    }

    if width.is_none() && style.is_none() && color.is_none() {
        Err(unsupported_value(
            input,
            None,
            "border shorthand is missing a component",
        ))
    } else {
        Ok(CssBorder::new(width, style, color))
    }
}

fn parse_corner_radius<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssCornerRadius, ParseError<'i, Error>> {
    let horizontal = parse_radius_component(input)?;
    let vertical = if input.is_exhausted() {
        horizontal.clone()
    } else {
        parse_radius_component(input)?
    };
    Ok(CssCornerRadius::new(horizontal, vertical))
}

fn parse_border_radius<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBorderRadii, ParseError<'i, Error>> {
    let horizontal = parse_radius_component_list(input)?;
    if horizontal.is_empty() {
        return Err(unsupported_value(
            input,
            None,
            "border-radius shorthand is missing a value",
        ));
    }

    let vertical = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        let vertical = parse_radius_component_list(input)?;
        if vertical.is_empty() {
            return Err(unsupported_value(
                input,
                None,
                "border-radius slash is missing vertical radii",
            ));
        }
        vertical
    } else {
        horizontal.clone()
    };

    let (h_top_left, h_top_right, h_bottom_right, h_bottom_left) =
        expand_radius_components(horizontal);
    let (v_top_left, v_top_right, v_bottom_right, v_bottom_left) =
        expand_radius_components(vertical);

    Ok(CssBorderRadii::new(
        CssCornerRadius::new(h_top_left, v_top_left),
        CssCornerRadius::new(h_top_right, v_top_right),
        CssCornerRadius::new(h_bottom_right, v_bottom_right),
        CssCornerRadius::new(h_bottom_left, v_bottom_left),
    ))
}

fn parse_radius_component_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Vec<CssLength>, ParseError<'i, Error>> {
    let mut values = Vec::new();
    while !input.is_exhausted() {
        let state = input.state();
        if input.try_parse(|input| input.expect_delim('/')).is_ok() {
            input.reset(&state);
            break;
        }

        values.push(parse_radius_component(input)?);
        if values.len() == 4 && !input.is_exhausted() {
            let state = input.state();
            let slash_is_next = input.try_parse(|input| input.expect_delim('/')).is_ok();
            input.reset(&state);
            if !slash_is_next {
                return Err(unsupported_value(
                    input,
                    None,
                    "border-radius shorthand has too many values",
                ));
            }
        }
    }
    Ok(values)
}

fn expand_radius_components(
    values: Vec<CssLength>,
) -> (CssLength, CssLength, CssLength, CssLength) {
    match values.as_slice() {
        [all] => (all.clone(), all.clone(), all.clone(), all.clone()),
        [vertical, horizontal] => (
            vertical.clone(),
            horizontal.clone(),
            vertical.clone(),
            horizontal.clone(),
        ),
        [top_left, top_right_bottom_left, bottom_right] => (
            top_left.clone(),
            top_right_bottom_left.clone(),
            bottom_right.clone(),
            top_right_bottom_left.clone(),
        ),
        [top_left, top_right, bottom_right, bottom_left] => (
            top_left.clone(),
            top_right.clone(),
            bottom_right.clone(),
            bottom_left.clone(),
        ),
        [] => unreachable!("caller validates non-empty border-radius components"),
        _ => unreachable!("border-radius component parser caps values at four"),
    }
}

fn parse_box_shadow<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBoxShadow, ParseError<'i, Error>> {
    let state = input.state();
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned)
        && ident.eq_ignore_ascii_case("none")
        && input.is_exhausted()
    {
        return Ok(CssBoxShadow::None);
    }
    input.reset(&state);

    let mut shadows = Vec::new();
    loop {
        shadows.push(parse_shadow(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "box-shadow list has an empty item",
            ));
        }
    }

    Ok(CssBoxShadow::Shadows(
        CssBoxShadowList::new(shadows).expect("box-shadow parser records at least one shadow"),
    ))
}

fn parse_shadow<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssShadow, ParseError<'i, Error>> {
    let mut inset = false;
    let mut color = None;
    let mut lengths = Vec::new();

    while !input.is_exhausted() {
        let state = input.state();
        if input.try_parse(Parser::expect_comma).is_ok() {
            input.reset(&state);
            break;
        }

        if input
            .try_parse(|input| input.expect_ident_matching("inset"))
            .is_ok()
        {
            if inset {
                return Err(unsupported_value(input, None, "duplicate box-shadow inset"));
            }
            inset = true;
            continue;
        }

        if let Ok(parsed_color) = input.try_parse(parse_color) {
            if color.replace(parsed_color).is_some() {
                return Err(unsupported_value(input, None, "duplicate box-shadow color"));
            }
            continue;
        }

        if lengths.len() < 4
            && let Ok(length) = if lengths.len() == 2 {
                input.try_parse(parse_shadow_blur_length)
            } else {
                input.try_parse(parse_shadow_length)
            }
        {
            lengths.push(length);
            continue;
        }

        return Err(unsupported_value(
            input,
            None,
            "unsupported box-shadow component",
        ));
    }

    match lengths.as_slice() {
        [offset_x, offset_y] => Ok(CssShadow::new(
            inset,
            offset_x.clone(),
            offset_y.clone(),
            None,
            None,
            color,
        )),
        [offset_x, offset_y, blur] => Ok(CssShadow::new(
            inset,
            offset_x.clone(),
            offset_y.clone(),
            Some(blur.clone()),
            None,
            color,
        )),
        [offset_x, offset_y, blur, spread] => Ok(CssShadow::new(
            inset,
            offset_x.clone(),
            offset_y.clone(),
            Some(blur.clone()),
            Some(spread.clone()),
            color,
        )),
        _ => Err(unsupported_value(
            input,
            None,
            "box-shadow requires at least two offsets",
        )),
    }
}

fn parse_box_size_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::box_size(), "box size")
}

fn parse_inset_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::box_size(), "inset")
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

fn parse_radius_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::radius(), "border-radius")
}

fn parse_shadow_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::shadow(), "box-shadow")
}

fn parse_shadow_blur_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::shadow_blur(), "box-shadow blur")
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
    non_negative: bool,
}

impl LengthOptions {
    const fn box_size() -> Self {
        Self {
            percent: true,
            auto: true,
            intrinsic: true,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    const fn margin() -> Self {
        Self {
            percent: true,
            auto: true,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    const fn padding() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: true,
        }
    }

    const fn border_width() -> Self {
        Self {
            percent: false,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: false,
            non_negative: true,
        }
    }

    const fn radius() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: true,
        }
    }

    const fn shadow() -> Self {
        Self {
            percent: false,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: false,
            non_negative: false,
        }
    }

    const fn shadow_blur() -> Self {
        Self {
            percent: false,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: false,
            non_negative: true,
        }
    }

    const fn gap() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: true,
            calc_percent: true,
            non_negative: false,
        }
    }

    const fn font_size() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    const fn line_height() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: true,
            calc_percent: true,
            non_negative: false,
        }
    }

    const fn grid_track() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: true,
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
        Token::Dimension { value, unit, .. } => match classify_length_unit(unit) {
            LengthUnitStatus::Supported(_) if options.non_negative && *value < 0.0 => {
                Err(unsupported_value_at(
                    location,
                    None,
                    format!("unsupported negative {context} length"),
                ))
            }
            LengthUnitStatus::Supported(unit) => Ok(CssLength::dimension(*value, unit)),
            LengthUnitStatus::Unknown => Err(unsupported_value_at(
                location,
                None,
                format!("unknown {context} unit `{unit}`"),
            )),
        },
        Token::Percentage { unit_value, .. } if options.non_negative && *unit_value < 0.0 => {
            Err(unsupported_value_at(
                location,
                None,
                format!("unsupported negative {context} percentage"),
            ))
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
            if options.non_negative && syntax::calc_has_negative_component(&calc) {
                return Err(unsupported_value_at(
                    location,
                    None,
                    format!("unsupported negative {context} calc component"),
                ));
            }
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
        Token::Dimension { value, unit, .. } => match classify_length_unit(unit) {
            LengthUnitStatus::Supported(_) if options.non_negative && *value < 0.0 => Err(
                unsupported_value_at(location, None, "unsupported negative calc length"),
            ),
            LengthUnitStatus::Supported(unit) => Ok(CssCalcLength::dimension(*value, unit)),
            LengthUnitStatus::Unknown => Err(unsupported_value_at(
                location,
                None,
                format!("unknown calc length unit `{unit}`"),
            )),
        },
        Token::Percentage { unit_value, .. } if options.non_negative && *unit_value < 0.0 => Err(
            unsupported_value_at(location, None, "unsupported negative calc percentage"),
        ),
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

fn parse_non_negative_number<'i, 't>(
    input: &mut Parser<'i, 't>,
    context: &str,
) -> std::result::Result<f32, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let value = input.expect_number().map_err(basic)?;
    if value < 0.0 {
        Err(unsupported_value_at(
            location,
            None,
            format!("unsupported negative {context}"),
        ))
    } else {
        Ok(value)
    }
}

fn parse_integer<'i, 't>(
    input: &mut Parser<'i, 't>,
    context: &str,
) -> std::result::Result<i32, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Number {
            int_value: Some(value),
            ..
        } => Ok(*value),
        Token::Number { .. } => Err(unsupported_value_at(
            location,
            None,
            format!("{context} must be an integer"),
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_positive_integer<'i, 't>(
    input: &mut Parser<'i, 't>,
    context: &str,
) -> std::result::Result<i32, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let value = parse_integer(input, context)?;
    if value <= 0 {
        Err(unsupported_value_at(
            location,
            None,
            format!("{context} must be a positive integer"),
        ))
    } else {
        Ok(value)
    }
}

fn parse_custom_ident_from_str_at<'i>(
    context: &str,
    ident: &str,
    location: cssparser::SourceLocation,
) -> std::result::Result<CssCustomIdent, ParseError<'i, Error>> {
    if ident.is_empty()
        || parse_global_keyword(ident).is_some()
        || ident.eq_ignore_ascii_case("span")
        || ident.eq_ignore_ascii_case("auto")
    {
        Err(error_at(
            location,
            ErrorKind::UnsupportedValue {
                property: None,
                reason: format!("unsupported {context} `{ident}`"),
            },
            format!("unsupported {context} `{ident}`"),
        ))
    } else {
        Ok(CssCustomIdent::new(ident))
    }
}

fn next_is_delim<'i, 't>(input: &mut Parser<'i, 't>, delim: char) -> bool {
    let state = input.state();
    let is_delim = input.try_parse(|input| input.expect_delim(delim)).is_ok();
    input.reset(&state);
    is_delim
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
                unsupported_keyword_reason("color", ident.as_ref()),
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

fn property_name_error<'i, 't>(input: &Parser<'i, 't>, name: &str) -> ParseError<'i, Error> {
    match classify_property_name(name) {
        PropertyNameStatus::Supported => unsupported_property(input, name),
        PropertyNameStatus::KnownUnsupported => unsupported_property(input, name),
        PropertyNameStatus::Unknown => unknown_property(input, name),
    }
}

fn property_for_supported_name(name: &str) -> Option<CssProperty> {
    Some(match_ignore_ascii_case! { name,
        "display" => CssProperty::Display,
        "box-sizing" => CssProperty::BoxSizing,
        "position" => CssProperty::Position,
        "direction" => CssProperty::Direction,
        "overflow" => CssProperty::Overflow,
        "overflow-x" => CssProperty::OverflowX,
        "overflow-y" => CssProperty::OverflowY,
        "flex-direction" => CssProperty::FlexDirection,
        "flex-wrap" => CssProperty::FlexWrap,
        "float" => CssProperty::Float,
        "clear" => CssProperty::Clear,
        "align-content" => CssProperty::AlignContent,
        "justify-content" => CssProperty::JustifyContent,
        "align-items" => CssProperty::AlignItems,
        "align-self" => CssProperty::AlignSelf,
        "justify-items" => CssProperty::JustifyItems,
        "justify-self" => CssProperty::JustifySelf,
        "place-content" => CssProperty::PlaceContent,
        "place-items" => CssProperty::PlaceItems,
        "place-self" => CssProperty::PlaceSelf,
        "visibility" => CssProperty::Visibility,
        "content-visibility" => CssProperty::ContentVisibility,
        "width" => CssProperty::Width,
        "height" => CssProperty::Height,
        "min-width" => CssProperty::MinWidth,
        "min-height" => CssProperty::MinHeight,
        "max-width" => CssProperty::MaxWidth,
        "max-height" => CssProperty::MaxHeight,
        "flex-basis" => CssProperty::FlexBasis,
        "gap" => CssProperty::Gap,
        "row-gap" => CssProperty::RowGap,
        "column-gap" => CssProperty::ColumnGap,
        "grid-flow-tolerance" => CssProperty::GridFlowTolerance,
        "grid-template-rows" => CssProperty::GridTemplateRows,
        "grid-template-columns" => CssProperty::GridTemplateColumns,
        "grid-template-areas" => CssProperty::GridTemplateAreas,
        "grid-template" => CssProperty::GridTemplate,
        "grid-auto-rows" => CssProperty::GridAutoRows,
        "grid-auto-columns" => CssProperty::GridAutoColumns,
        "grid-auto-flow" => CssProperty::GridAutoFlow,
        "grid-row-start" => CssProperty::GridRowStart,
        "grid-row-end" => CssProperty::GridRowEnd,
        "grid-column-start" => CssProperty::GridColumnStart,
        "grid-column-end" => CssProperty::GridColumnEnd,
        "grid-row" => CssProperty::GridRow,
        "grid-column" => CssProperty::GridColumn,
        "grid-area" => CssProperty::GridArea,
        "grid" => CssProperty::Grid,
        "font-size" => CssProperty::FontSize,
        "line-height" => CssProperty::LineHeight,
        "inset" => CssProperty::Inset,
        "top" => CssProperty::Top,
        "right" => CssProperty::Right,
        "bottom" => CssProperty::Bottom,
        "left" => CssProperty::Left,
        "z-index" => CssProperty::ZIndex,
        "box-decoration-break" => CssProperty::BoxDecorationBreak,
        "margin" => CssProperty::Margin,
        "margin-top" => CssProperty::MarginTop,
        "margin-right" => CssProperty::MarginRight,
        "margin-bottom" => CssProperty::MarginBottom,
        "margin-left" => CssProperty::MarginLeft,
        "padding" => CssProperty::Padding,
        "padding-top" => CssProperty::PaddingTop,
        "padding-right" => CssProperty::PaddingRight,
        "padding-bottom" => CssProperty::PaddingBottom,
        "padding-left" => CssProperty::PaddingLeft,
        "border" => CssProperty::Border,
        "border-top" => CssProperty::BorderTop,
        "border-right" => CssProperty::BorderRight,
        "border-bottom" => CssProperty::BorderBottom,
        "border-left" => CssProperty::BorderLeft,
        "border-width" => CssProperty::BorderWidth,
        "border-top-width" => CssProperty::BorderTopWidth,
        "border-right-width" => CssProperty::BorderRightWidth,
        "border-bottom-width" => CssProperty::BorderBottomWidth,
        "border-left-width" => CssProperty::BorderLeftWidth,
        "color" => CssProperty::Color,
        "background" | "background-color" => CssProperty::Background,
        "border-color" => CssProperty::BorderColor,
        "border-top-color" => CssProperty::BorderTopColor,
        "border-right-color" => CssProperty::BorderRightColor,
        "border-bottom-color" => CssProperty::BorderBottomColor,
        "border-left-color" => CssProperty::BorderLeftColor,
        "border-style" => CssProperty::BorderStyle,
        "border-top-style" => CssProperty::BorderTopStyle,
        "border-right-style" => CssProperty::BorderRightStyle,
        "border-bottom-style" => CssProperty::BorderBottomStyle,
        "border-left-style" => CssProperty::BorderLeftStyle,
        "border-radius" => CssProperty::BorderRadius,
        "border-top-left-radius" => CssProperty::BorderTopLeftRadius,
        "border-top-right-radius" => CssProperty::BorderTopRightRadius,
        "border-bottom-right-radius" => CssProperty::BorderBottomRightRadius,
        "border-bottom-left-radius" => CssProperty::BorderBottomLeftRadius,
        "box-shadow" => CssProperty::BoxShadow,
        "opacity" => CssProperty::Opacity,
        "flex-grow" => CssProperty::FlexGrow,
        "flex-shrink" => CssProperty::FlexShrink,
        "order" => CssProperty::Order,
        "flex" => CssProperty::Flex,
        "justify-tracks" => CssProperty::JustifyTracks,
        "align-tracks" => CssProperty::AlignTracks,
        "aspect-ratio" => CssProperty::AspectRatio,
        "scrollbar-width" => CssProperty::ScrollbarWidth,
        _ => return None,
    })
}

fn unknown_property<'i, 't>(
    input: &Parser<'i, 't>,
    name: impl Into<String>,
) -> ParseError<'i, Error> {
    let name = name.into();
    error_at(
        input.current_source_location(),
        ErrorKind::UnknownProperty { name: name.clone() },
        format!("unknown CSS property `{name}`"),
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

    fn declaration(input: &str, property: CssProperty) -> CssDeclaration {
        let sheet = parse_sheet(input).unwrap();
        sheet.rules()[0]
            .declarations()
            .iter()
            .find(|declaration| declaration.property() == property)
            .unwrap()
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
    fn exposes_nested_calc_terms_structurally() {
        let value = declaration_value(
            ".panel { width: calc(100% - calc(12px + 3%)); }",
            CssProperty::Width,
        );

        let calc = match value {
            CssValue::Length(CssLength::Calc(calc)) => calc,
            other => panic!("expected nested calc length, got {other:?}"),
        };

        let terms = match calc {
            CssCalcLength::Sum(terms) => terms,
            other => panic!("expected calc sum, got {other:?}"),
        };
        assert_eq!(terms.len(), 2);
        assert_eq!(terms[0].operator(), CssCalcOperator::Add);
        assert_eq!(terms[0].value(), &CssCalcLength::Percent(100.0));
        assert_eq!(terms[1].operator(), CssCalcOperator::Subtract);

        let nested_terms = match terms[1].value() {
            CssCalcLength::Sum(terms) => terms,
            other => panic!("expected nested calc sum, got {other:?}"),
        };
        assert_eq!(nested_terms.len(), 2);
        assert_eq!(nested_terms[0].operator(), CssCalcOperator::Add);
        assert_eq!(nested_terms[0].value(), &CssCalcLength::Px(12.0));
        assert_eq!(nested_terms[1].operator(), CssCalcOperator::Add);
        assert_eq!(nested_terms[1].value(), &CssCalcLength::Percent(3.0));
    }

    #[test]
    fn successful_declarations_expose_authored_source_location() {
        let input = ".panel {\n  height: 20px;\n  width: calc(100% - 4px);\n}\n";
        let height = declaration(input, CssProperty::Height);
        let width = declaration(input, CssProperty::Width);

        assert_eq!(height.location(), CssSourceLocation::new(1, 3));
        assert_eq!(width.location(), CssSourceLocation::new(2, 3));
        assert_eq!(width.line(), 2);
        assert_eq!(width.column(), 3);
    }

    #[test]
    fn parses_supported_length_units_as_authored_dimensions() {
        let cases = [
            ("1em", 1.0, CssLengthUnit::Em),
            ("2rem", 2.0, CssLengthUnit::Rem),
            ("3vw", 3.0, CssLengthUnit::Vw),
            ("4svh", 4.0, CssLengthUnit::Svh),
            ("5lvw", 5.0, CssLengthUnit::Lvw),
            ("6dvb", 6.0, CssLengthUnit::Dvb),
            ("7cqi", 7.0, CssLengthUnit::Cqi),
            ("8cm", 8.0, CssLengthUnit::Cm),
            ("9pt", 9.0, CssLengthUnit::Pt),
        ];

        for (authored, expected_value, expected_unit) in cases {
            let value = declaration_value(
                &format!(".panel {{ width: {authored}; }}"),
                CssProperty::Width,
            );

            match value {
                CssValue::Length(CssLength::Dimension(length)) => {
                    assert_eq!(length.value(), expected_value);
                    assert_eq!(length.unit(), expected_unit);
                    assert_eq!(length.to_css_string(), authored);
                }
                other => panic!("expected authored dimension for {authored}, got {other:?}"),
            }
        }
    }

    #[test]
    fn parses_supported_calc_length_units_as_authored_dimensions() {
        let cases = [
            ("1em", 1.0, CssLengthUnit::Em),
            ("2rem", 2.0, CssLengthUnit::Rem),
            ("3vw", 3.0, CssLengthUnit::Vw),
            ("4svh", 4.0, CssLengthUnit::Svh),
            ("5lvw", 5.0, CssLengthUnit::Lvw),
            ("6dvb", 6.0, CssLengthUnit::Dvb),
            ("7cqi", 7.0, CssLengthUnit::Cqi),
            ("8cm", 8.0, CssLengthUnit::Cm),
            ("9pt", 9.0, CssLengthUnit::Pt),
        ];

        for (authored, expected_value, expected_unit) in cases {
            let value = declaration_value(
                &format!(".panel {{ width: calc({authored} + 2px); }}"),
                CssProperty::Width,
            );

            let CssValue::Length(CssLength::Calc(CssCalcLength::Sum(terms))) = value else {
                panic!("expected calc length for {authored}");
            };
            assert_eq!(terms.len(), 2);
            match terms[0].value() {
                CssCalcLength::Dimension(length) => {
                    assert_eq!(length.value(), expected_value);
                    assert_eq!(length.unit(), expected_unit);
                    assert_eq!(length.to_css_string(), authored);
                }
                other => panic!("expected authored calc dimension for {authored}, got {other:?}"),
            }
            assert_eq!(terms[1].value(), &CssCalcLength::Px(2.0));
        }
    }

    #[test]
    fn known_but_unsupported_property_has_typed_error_kind() {
        let error = parse_sheet(".panel { text-align: left; }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedProperty {
                name: "text-align".to_owned(),
            }
        );
        assert!(
            error
                .message()
                .contains("unsupported CSS property `text-align`")
        );
    }

    #[test]
    fn another_known_but_unsupported_property_is_not_treated_as_unknown() {
        let error = parse_sheet(".panel { writing-mode: horizontal-tb; }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedProperty {
                name: "writing-mode".to_owned(),
            }
        );
    }

    #[test]
    fn typo_property_has_unknown_property_error_kind() {
        let error = parse_sheet(".panel { widht: 10px; }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnknownProperty {
                name: "widht".to_owned(),
            }
        );
        assert!(error.message().contains("unknown CSS property `widht`"));
    }

    #[test]
    fn parses_global_keywords_for_different_value_domains() {
        assert_eq!(
            declaration_value(".panel { width: inherit; }", CssProperty::Width),
            CssValue::GlobalKeyword(CssGlobalKeyword::Inherit)
        );
        assert_eq!(
            declaration_value(".panel { display: initial; }", CssProperty::Display),
            CssValue::GlobalKeyword(CssGlobalKeyword::Initial)
        );
        assert_eq!(
            declaration_value(".panel { color: unset; }", CssProperty::Color),
            CssValue::GlobalKeyword(CssGlobalKeyword::Unset)
        );
    }

    #[test]
    fn parses_newer_global_keywords_as_authored_syntax() {
        assert_eq!(
            declaration_value(".panel { padding: revert; }", CssProperty::Padding),
            CssValue::GlobalKeyword(CssGlobalKeyword::Revert)
        );
        assert_eq!(
            declaration_value(".panel { margin: revert-layer; }", CssProperty::Margin),
            CssValue::GlobalKeyword(CssGlobalKeyword::RevertLayer)
        );
    }

    #[test]
    fn global_keyword_must_be_the_whole_value() {
        let error = parse_sheet(".panel { width: inherit 10px; }").unwrap_err();

        assert!(matches!(error.kind(), ErrorKind::InvalidSyntax { .. }));
    }

    #[test]
    fn unsupported_display_keyword_is_typed_with_property_context() {
        let error = parse_sheet(".panel { display: inline; }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedValue {
                property: Some("display".to_owned()),
                reason: "unsupported display keyword `inline`".to_owned(),
            }
        );
    }

    #[test]
    fn unsupported_overflow_keyword_is_typed_with_property_context() {
        let error = parse_sheet(".panel { overflow: auto; }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedValue {
                property: Some("overflow".to_owned()),
                reason: "unsupported overflow keyword `auto`".to_owned(),
            }
        );
    }

    #[test]
    fn unsupported_position_keyword_is_typed_with_property_context() {
        let error = parse_sheet(".panel { position: running; }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedValue {
                property: Some("position".to_owned()),
                reason: "unsupported position keyword `running`".to_owned(),
            }
        );
    }

    #[test]
    fn unsupported_alignment_keyword_is_typed_with_property_context() {
        let error = parse_sheet(".panel { align-items: unsafe center; }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedValue {
                property: Some("align-items".to_owned()),
                reason: "unsupported alignment keyword `unsafe center`".to_owned(),
            }
        );
    }

    #[test]
    fn parses_position_float_clear_visibility_values() {
        assert_eq!(
            declaration_value(".panel { position: static; }", CssProperty::Position),
            CssValue::Position(CssLayoutPosition::Static)
        );
        assert_eq!(
            declaration_value(".panel { position: fixed; }", CssProperty::Position),
            CssValue::Position(CssLayoutPosition::Fixed)
        );
        assert_eq!(
            declaration_value(".panel { position: sticky; }", CssProperty::Position),
            CssValue::Position(CssLayoutPosition::Sticky)
        );
        assert_eq!(
            declaration_value(".panel { float: left; }", CssProperty::Float),
            CssValue::Float(CssFloat::Left)
        );
        assert_eq!(
            declaration_value(".panel { clear: both; }", CssProperty::Clear),
            CssValue::Clear(CssClear::Both)
        );
        assert_eq!(
            declaration_value(".panel { visibility: collapse; }", CssProperty::Visibility),
            CssValue::Visibility(CssVisibility::Collapse)
        );
        assert_eq!(
            declaration_value(
                ".panel { content-visibility: auto; }",
                CssProperty::ContentVisibility
            ),
            CssValue::ContentVisibility(CssContentVisibility::Auto)
        );
    }

    #[test]
    fn parses_content_alignment_and_place_shorthands() {
        assert_eq!(
            declaration_value(
                ".panel { align-content: space-between; }",
                CssProperty::AlignContent
            ),
            CssValue::Alignment(CssAlignment::SpaceBetween)
        );
        assert_eq!(
            declaration_value(
                ".panel { justify-content: safe center; }",
                CssProperty::JustifyContent
            ),
            CssValue::Alignment(CssAlignment::SafeCenter)
        );
        assert_eq!(
            declaration_value(
                ".panel { align-items: first baseline; }",
                CssProperty::AlignItems
            ),
            CssValue::AlignItems(CssAlignItems::FirstBaseline)
        );
        assert_eq!(
            declaration_value(
                ".panel { place-content: center end; }",
                CssProperty::PlaceContent
            ),
            CssValue::PlaceAlignment(CssPlaceAlignment::content(
                CssAlignment::Center,
                CssAlignment::End
            ))
        );
        assert_eq!(
            declaration_value(".panel { place-items: stretch; }", CssProperty::PlaceItems),
            CssValue::PlaceAlignment(CssPlaceAlignment::items_all(CssAlignItems::Stretch))
        );
        assert_eq!(
            declaration_value(".panel { place-self: end center; }", CssProperty::PlaceSelf),
            CssValue::PlaceAlignment(CssPlaceAlignment::items(
                CssAlignItems::End,
                CssAlignItems::Center
            ))
        );
    }

    #[test]
    fn preserves_explicit_safe_alignment_values() {
        assert_eq!(
            declaration_value(".panel { align-items: safe end; }", CssProperty::AlignItems),
            CssValue::AlignItems(CssAlignItems::SafeEnd)
        );
        assert_eq!(
            declaration_value(
                ".panel { align-self: safe flex-end; }",
                CssProperty::AlignSelf
            ),
            CssValue::AlignItems(CssAlignItems::SafeFlexEnd)
        );
        assert_eq!(
            declaration_value(
                ".panel { justify-content: safe center; }",
                CssProperty::JustifyContent
            ),
            CssValue::Alignment(CssAlignment::SafeCenter)
        );
    }

    #[test]
    fn rejects_positioning_alignment_and_visibility_leakage_values() {
        let cases = [
            ".panel { float: center; }",
            ".panel { clear: start; }",
            ".panel { align-content: left; }",
            ".panel { justify-content: auto; }",
            ".panel { place-items: auto; }",
            ".panel { place-items: space-between; }",
            ".panel { visibility: auto; }",
            ".panel { content-visibility: collapse; }",
        ];

        for case in cases {
            assert!(parse_sheet(case).is_err(), "{case} should be rejected");
        }
    }

    #[test]
    fn rejects_unmodeled_safe_prefixed_alignment_values() {
        let cases = [
            ".panel { align-items: safe start; }",
            ".panel { align-items: safe flex-start; }",
            ".panel { align-items: safe stretch; }",
            ".panel { align-content: safe start; }",
            ".panel { align-content: safe flex-start; }",
            ".panel { align-content: safe stretch; }",
            ".panel { place-content: safe start; }",
            ".panel { place-content: safe flex-start; }",
            ".panel { place-content: safe stretch; }",
        ];

        for case in cases {
            assert!(parse_sheet(case).is_err(), "{case} should be rejected");
        }
    }

    #[test]
    fn unknown_dimension_units_are_reported_as_unknown_units() {
        let error = parse_sheet(".panel { width: 1quux; }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedValue {
                property: Some("width".to_owned()),
                reason: "unknown box size unit `quux`".to_owned(),
            }
        );
    }

    #[test]
    fn unknown_calc_dimension_units_are_reported_as_unknown_units() {
        let error = parse_sheet(".panel { width: calc(1quux + 2px); }").unwrap_err();

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedValue {
                property: Some("width".to_owned()),
                reason: "unknown calc length unit `quux`".to_owned(),
            }
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

    #[test]
    fn parses_spacing_inset_and_z_index_values() {
        assert_eq!(
            declaration_value(".panel { inset: auto 10px 5%; }", CssProperty::Inset),
            CssValue::Edges(CssEdges::new(
                CssLength::Auto,
                CssLength::px(10.0),
                CssLength::percent(5.0),
                CssLength::px(10.0),
            ))
        );
        assert_eq!(
            declaration_value(".panel { top: calc(10px + 5%); }", CssProperty::Top),
            CssValue::Length(CssLength::Calc(CssCalcLength::sum(
                CssCalcLengthTerm::add(CssCalcLength::Px(10.0)),
                [CssCalcLengthTerm::add(CssCalcLength::Percent(5.0))]
            )))
        );
        assert_eq!(
            declaration_value(".panel { z-index: -2; }", CssProperty::ZIndex),
            CssValue::ZIndex(CssZIndex::Integer(-2))
        );
        assert_eq!(
            declaration_value(
                ".panel { box-decoration-break: clone; }",
                CssProperty::BoxDecorationBreak
            ),
            CssValue::BoxDecorationBreak(CssBoxDecorationBreak::Clone)
        );
    }

    #[test]
    fn parses_spacing_longhands_with_existing_component_rules() {
        assert_eq!(
            declaration_value(".panel { margin-left: auto; }", CssProperty::MarginLeft),
            CssValue::Length(CssLength::Auto)
        );
        assert_eq!(
            declaration_value(".panel { padding-top: 12px; }", CssProperty::PaddingTop),
            CssValue::Length(CssLength::px(12.0))
        );
        assert_eq!(
            declaration_value(
                ".panel { border-right-width: 2px; }",
                CssProperty::BorderRightWidth
            ),
            CssValue::Length(CssLength::px(2.0))
        );
    }

    #[test]
    fn parses_border_style_and_border_shorthand_values() {
        assert_eq!(
            declaration_value(
                ".panel { border-style: solid dashed; }",
                CssProperty::BorderStyle
            ),
            CssValue::BorderStyles(CssBorderStyles::new(
                CssBorderStyle::Solid,
                CssBorderStyle::Dashed,
                CssBorderStyle::Solid,
                CssBorderStyle::Dashed,
            ))
        );
        assert_eq!(
            declaration_value(
                ".panel { border-left-style: groove; }",
                CssProperty::BorderLeftStyle
            ),
            CssValue::BorderStyle(CssBorderStyle::Groove)
        );
        assert_eq!(
            declaration_value(".panel { border: solid 2px #fff; }", CssProperty::Border),
            CssValue::Border(CssBorder::new(
                Some(CssLength::px(2.0)),
                Some(CssBorderStyle::Solid),
                Some(CssColor::WHITE),
            ))
        );
        assert_eq!(
            declaration_value(
                ".panel { border-top: black dotted; }",
                CssProperty::BorderTop
            ),
            CssValue::Border(CssBorder::new(
                None,
                Some(CssBorderStyle::Dotted),
                Some(CssColor::BLACK),
            ))
        );
    }

    #[test]
    fn parses_border_radius_shorthand_and_longhands() {
        assert_eq!(
            declaration_value(
                ".panel { border-top-left-radius: 4px 10%; }",
                CssProperty::BorderTopLeftRadius,
            ),
            CssValue::CornerRadius(CssCornerRadius::new(
                CssLength::px(4.0),
                CssLength::percent(10.0),
            ))
        );
        assert_eq!(
            declaration_value(
                ".panel { border-radius: 1px 2px 3px / 4px 5px; }",
                CssProperty::BorderRadius,
            ),
            CssValue::BorderRadius(CssBorderRadii::new(
                CssCornerRadius::new(CssLength::px(1.0), CssLength::px(4.0)),
                CssCornerRadius::new(CssLength::px(2.0), CssLength::px(5.0)),
                CssCornerRadius::new(CssLength::px(3.0), CssLength::px(4.0)),
                CssCornerRadius::new(CssLength::px(2.0), CssLength::px(5.0)),
            ))
        );
    }

    #[test]
    fn parses_box_shadow_none_and_shadow_lists() {
        assert_eq!(
            declaration_value(".panel { box-shadow: none; }", CssProperty::BoxShadow),
            CssValue::BoxShadow(CssBoxShadow::None)
        );

        let value = declaration_value(
            ".panel { box-shadow: inset 1px 2px 3px 4px black, 0 1px #fff; }",
            CssProperty::BoxShadow,
        );

        let CssValue::BoxShadow(CssBoxShadow::Shadows(shadows)) = value else {
            panic!("expected box-shadow list");
        };
        assert_eq!(shadows.shadows().len(), 2);
        assert_eq!(
            shadows.shadows()[0],
            CssShadow::new(
                true,
                CssLength::px(1.0),
                CssLength::px(2.0),
                Some(CssLength::px(3.0)),
                Some(CssLength::px(4.0)),
                Some(CssColor::BLACK),
            )
        );
        assert_eq!(
            shadows.shadows()[1],
            CssShadow::new(
                false,
                CssLength::Zero,
                CssLength::px(1.0),
                None,
                None,
                Some(CssColor::WHITE),
            )
        );
    }

    #[test]
    fn checked_border_constructor_rejects_empty_shorthands() {
        assert_eq!(CssBorder::try_new(None, None, None), None);
        assert_eq!(
            CssBorder::try_new(None, Some(CssBorderStyle::Solid), None),
            Some(CssBorder::new(None, Some(CssBorderStyle::Solid), None))
        );
    }

    #[test]
    fn checked_border_constructor_rejects_parser_invalid_widths() {
        for width in [
            CssLength::Auto,
            CssLength::percent(10.0),
            CssLength::px(-1.0),
            CssLength::MinContent,
            CssLength::Normal,
            CssLength::Calc(CssCalcLength::Percent(10.0)),
            CssLength::Calc(CssCalcLength::Px(-1.0)),
        ] {
            assert_eq!(
                CssBorder::try_new(Some(width), Some(CssBorderStyle::Solid), None),
                None
            );
        }

        assert_eq!(
            CssBorder::try_new(
                Some(CssLength::Calc(CssCalcLength::Px(1.0))),
                Some(CssBorderStyle::Solid),
                None,
            ),
            Some(CssBorder::new(
                Some(CssLength::Calc(CssCalcLength::Px(1.0))),
                Some(CssBorderStyle::Solid),
                None,
            ))
        );
    }

    #[test]
    fn checked_corner_radius_constructor_rejects_parser_invalid_values() {
        for value in [
            CssLength::Auto,
            CssLength::MinContent,
            CssLength::MaxContent,
            CssLength::FitContent,
            CssLength::Normal,
            CssLength::px(-1.0),
            CssLength::percent(-1.0),
            CssLength::Calc(CssCalcLength::Px(-1.0)),
            CssLength::Calc(CssCalcLength::Percent(-1.0)),
        ] {
            assert_eq!(
                CssCornerRadius::try_new(value.clone(), CssLength::px(1.0)),
                None
            );
            assert_eq!(CssCornerRadius::try_new(CssLength::px(1.0), value), None);
        }

        assert_eq!(
            CssCornerRadius::try_new(CssLength::px(1.0), CssLength::percent(25.0)),
            Some(CssCornerRadius::new(
                CssLength::px(1.0),
                CssLength::percent(25.0)
            ))
        );
    }

    #[test]
    fn checked_shadow_constructor_rejects_invalid_pairings_and_lengths() {
        assert_eq!(
            CssShadow::try_new(false, CssLength::Auto, CssLength::px(2.0), None, None, None,),
            None
        );
        assert_eq!(
            CssShadow::try_new(
                false,
                CssLength::px(1.0),
                CssLength::px(2.0),
                None,
                Some(CssLength::px(4.0)),
                None,
            ),
            None
        );
        assert_eq!(
            CssShadow::try_new(
                false,
                CssLength::px(1.0),
                CssLength::px(2.0),
                Some(CssLength::px(-3.0)),
                None,
                None,
            ),
            None
        );
        assert_eq!(
            CssShadow::try_new(
                false,
                CssLength::px(-1.0),
                CssLength::px(2.0),
                Some(CssLength::px(3.0)),
                Some(CssLength::px(-4.0)),
                None,
            ),
            Some(CssShadow::new(
                false,
                CssLength::px(-1.0),
                CssLength::px(2.0),
                Some(CssLength::px(3.0)),
                Some(CssLength::px(-4.0)),
                None,
            ))
        );
    }

    #[test]
    fn parses_every_task_2_supported_property_name() {
        let sheet = parse_sheet(
            ".panel {
                inset: auto 1px 2%;
                top: auto;
                right: 1px;
                bottom: 2%;
                left: calc(3px + 4%);
                z-index: 7;
                box-decoration-break: slice;
                margin-top: auto;
                margin-right: 1px;
                margin-bottom: 2%;
                margin-left: calc(3px + 4%);
                padding-top: 1px;
                padding-right: 2%;
                padding-bottom: calc(3px + 4%);
                padding-left: 0;
                border: 1px solid black;
                border-top: solid;
                border-right: 1px;
                border-bottom: #fff;
                border-left: dashed black;
                border-top-width: 1px;
                border-right-width: 2px;
                border-bottom-width: 3px;
                border-left-width: 4px;
                border-top-color: black;
                border-right-color: white;
                border-bottom-color: transparent;
                border-left-color: #fff;
                border-style: none hidden dotted dashed;
                border-top-style: solid;
                border-right-style: double;
                border-bottom-style: ridge;
                border-left-style: outset;
                border-radius: 1px 2px / 3px 4px;
                border-top-left-radius: 1px;
                border-top-right-radius: 1px 2px;
                border-bottom-right-radius: 10%;
                border-bottom-left-radius: calc(1px + 2%);
                box-shadow: 1px 2px;
            }",
        )
        .unwrap();
        let declarations = sheet.rules()[0].declarations();

        for property in [
            CssProperty::Inset,
            CssProperty::Top,
            CssProperty::Right,
            CssProperty::Bottom,
            CssProperty::Left,
            CssProperty::ZIndex,
            CssProperty::BoxDecorationBreak,
            CssProperty::MarginTop,
            CssProperty::MarginRight,
            CssProperty::MarginBottom,
            CssProperty::MarginLeft,
            CssProperty::PaddingTop,
            CssProperty::PaddingRight,
            CssProperty::PaddingBottom,
            CssProperty::PaddingLeft,
            CssProperty::Border,
            CssProperty::BorderTop,
            CssProperty::BorderRight,
            CssProperty::BorderBottom,
            CssProperty::BorderLeft,
            CssProperty::BorderTopWidth,
            CssProperty::BorderRightWidth,
            CssProperty::BorderBottomWidth,
            CssProperty::BorderLeftWidth,
            CssProperty::BorderTopColor,
            CssProperty::BorderRightColor,
            CssProperty::BorderBottomColor,
            CssProperty::BorderLeftColor,
            CssProperty::BorderStyle,
            CssProperty::BorderTopStyle,
            CssProperty::BorderRightStyle,
            CssProperty::BorderBottomStyle,
            CssProperty::BorderLeftStyle,
            CssProperty::BorderRadius,
            CssProperty::BorderTopLeftRadius,
            CssProperty::BorderTopRightRadius,
            CssProperty::BorderBottomRightRadius,
            CssProperty::BorderBottomLeftRadius,
            CssProperty::BoxShadow,
        ] {
            assert!(
                declarations
                    .iter()
                    .any(|declaration| declaration.property() == property),
                "missing parsed declaration for {property:?}",
            );
        }
    }

    #[test]
    fn rejects_negative_lengths_for_non_negative_task_2_properties() {
        for input in [
            ".panel { border-radius: -1px; }",
            ".panel { padding-top: -1px; }",
            ".panel { border-width: -1px; }",
            ".panel { box-shadow: 1px 2px -3px; }",
        ] {
            let error = parse_sheet(input).expect_err(input);
            assert!(matches!(error.kind(), ErrorKind::UnsupportedValue { .. }));
        }
    }

    #[test]
    fn rejects_task_2_cross_family_leakage_values() {
        for input in [
            ".panel { padding-top: auto; }",
            ".panel { border-width: 10%; }",
            ".panel { border-style: 10px; }",
            ".panel { border-color: solid; }",
            ".panel { border-radius: auto; }",
            ".panel { box-shadow: auto; }",
            ".panel { z-index: 1.5; }",
        ] {
            let error = parse_sheet(input).expect_err(input);
            assert!(matches!(
                error.kind(),
                ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
            ));
        }
    }

    #[test]
    fn parses_grid_track_lists_and_template_areas() {
        assert_eq!(
            declaration_value(
                ".panel { grid-template-columns: [main] repeat(2, minmax(10px, 1fr)) fit-content(20%); }",
                CssProperty::GridTemplateColumns,
            ),
            CssValue::GridTrackList(CssGridTrackList::new(vec![
                CssGridTrackComponent::LineNames(CssGridLineNames::new(vec![CssCustomIdent::new(
                    "main"
                )])),
                CssGridTrackComponent::Repeat(CssGridRepeat::new(
                    CssGridRepeatCount::integer(2),
                    CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                        CssGridTrackSize::minmax(
                            CssGridTrackBreadth::length(CssLength::px(10.0)),
                            CssGridTrackBreadth::Fraction(1.0),
                        )
                    )]),
                )),
                CssGridTrackComponent::TrackSize(CssGridTrackSize::fit_content(
                    CssLength::percent(20.0)
                )),
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { grid-template-areas: \"header header\" \"nav main\"; }",
                CssProperty::GridTemplateAreas,
            ),
            CssValue::GridTemplateAreas(CssGridTemplateAreas::rows(vec![
                CssGridTemplateAreaRow::new(vec![
                    CssGridTemplateAreaCell::Named(CssCustomIdent::new("header")),
                    CssGridTemplateAreaCell::Named(CssCustomIdent::new("header")),
                ]),
                CssGridTemplateAreaRow::new(vec![
                    CssGridTemplateAreaCell::Named(CssCustomIdent::new("nav")),
                    CssGridTemplateAreaCell::Named(CssCustomIdent::new("main")),
                ]),
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { grid-template-areas: none; }",
                CssProperty::GridTemplateAreas,
            ),
            CssValue::GridTemplateAreas(CssGridTemplateAreas::None)
        );
    }

    #[test]
    fn parses_grid_flow_lines_and_shorthands() {
        assert_eq!(
            declaration_value(
                ".panel { grid-auto-flow: column dense; }",
                CssProperty::GridAutoFlow,
            ),
            CssValue::GridAutoFlow(CssGridAutoFlow::new(CssGridAutoFlowAxis::Column, true))
        );
        assert_eq!(
            declaration_value(
                ".panel { grid-row-start: span 2 main; }",
                CssProperty::GridRowStart
            ),
            CssValue::GridLine(CssGridLine::span(
                Some(2),
                Some(CssCustomIdent::new("main"))
            ))
        );
        assert_eq!(
            declaration_value(
                ".panel { grid-column: nav / span 3; }",
                CssProperty::GridColumn
            ),
            CssValue::GridLineRange(CssGridLineRange::new(
                CssGridLine::CustomIdent(CssCustomIdent::new("nav")),
                Some(CssGridLine::span(Some(3), None)),
            ))
        );
        assert_eq!(
            declaration_value(
                ".panel { grid-area: header / 1 / span 2 / main; }",
                CssProperty::GridArea
            ),
            CssValue::GridArea(CssGridArea::new(
                CssGridLine::CustomIdent(CssCustomIdent::new("header")),
                Some(CssGridLine::integer(1)),
                Some(CssGridLine::span(Some(2), None)),
                Some(CssGridLine::CustomIdent(CssCustomIdent::new("main"))),
            ))
        );
    }

    #[test]
    fn parses_grid_template_and_grid_shorthands() {
        assert_eq!(
            declaration_value(
                ".panel { grid-template: 100px 1fr / repeat(2, minmax(10px, 1fr)); }",
                CssProperty::GridTemplate,
            ),
            CssValue::GridTemplate(CssGridTemplate::RowsColumns {
                rows: CssGridTrackList::new(vec![
                    CssGridTrackComponent::TrackSize(CssGridTrackSize::breadth(
                        CssGridTrackBreadth::length(CssLength::px(100.0))
                    )),
                    CssGridTrackComponent::TrackSize(CssGridTrackSize::breadth(
                        CssGridTrackBreadth::Fraction(1.0)
                    )),
                ]),
                columns: Some(CssGridTrackList::new(vec![CssGridTrackComponent::Repeat(
                    CssGridRepeat::new(
                        CssGridRepeatCount::integer(2),
                        CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                            CssGridTrackSize::minmax(
                                CssGridTrackBreadth::length(CssLength::px(10.0)),
                                CssGridTrackBreadth::Fraction(1.0),
                            )
                        )]),
                    )
                )])),
            })
        );
        assert_eq!(
            declaration_value(
                ".panel { grid: auto-flow dense 12px / repeat(auto-fit, 1fr); }",
                CssProperty::Grid,
            ),
            CssValue::Grid(CssGrid::AutoFlow {
                flow: CssGridAutoFlow::new(CssGridAutoFlowAxis::Row, true),
                auto_tracks: Some(CssGridTrackList::new(vec![
                    CssGridTrackComponent::TrackSize(CssGridTrackSize::breadth(
                        CssGridTrackBreadth::length(CssLength::px(12.0))
                    ),)
                ])),
                explicit_tracks: CssGridTrackList::new(vec![CssGridTrackComponent::Repeat(
                    CssGridRepeat::new(
                        CssGridRepeatCount::AutoFit,
                        CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                            CssGridTrackSize::breadth(CssGridTrackBreadth::Fraction(1.0))
                        )]),
                    )
                )]),
            })
        );
    }

    #[test]
    fn parses_order_flex_and_track_alignment() {
        assert_eq!(
            declaration_value(".panel { order: -2; }", CssProperty::Order),
            CssValue::Order(CssOrder::Integer(-2))
        );
        assert_eq!(
            declaration_value(".panel { flex: 2 0 10rem; }", CssProperty::Flex),
            CssValue::Flex(CssFlex::Components {
                grow: 2.0,
                shrink: Some(0.0),
                basis: Some(CssLength::dimension(10.0, CssLengthUnit::Rem)),
            })
        );
        assert_eq!(
            declaration_value(".panel { flex: none; }", CssProperty::Flex),
            CssValue::Flex(CssFlex::None)
        );
        assert_eq!(
            declaration_value(".panel { flex: auto; }", CssProperty::Flex),
            CssValue::Flex(CssFlex::Auto)
        );
        assert_eq!(
            declaration_value(
                ".panel { justify-tracks: space-evenly; }",
                CssProperty::JustifyTracks,
            ),
            CssValue::Alignment(CssAlignment::SpaceEvenly)
        );
        assert_eq!(
            declaration_value(".panel { align-tracks: center; }", CssProperty::AlignTracks),
            CssValue::Alignment(CssAlignment::Center)
        );
    }

    #[test]
    fn parses_every_task_4_supported_property_name() {
        let sheet = parse_sheet(
            ".panel {
                grid-template-rows: [top] 100px 1fr;
                grid-template-columns: repeat(2, minmax(10px, 1fr));
                grid-template-areas: \"header header\" \"nav main\";
                grid-template: 100px / 1fr 2fr;
                grid-auto-rows: minmax(10px, auto);
                grid-auto-columns: fit-content(20%);
                grid-auto-flow: row dense;
                grid-row-start: 1;
                grid-row-end: span 2;
                grid-column-start: nav;
                grid-column-end: auto;
                grid-row: 1 / span 2;
                grid-column: nav / main;
                grid-area: header / nav / main / 4;
                grid: auto-flow 12px / repeat(auto-fill, 1fr);
                order: 2;
                flex: 1 1 auto;
                justify-tracks: space-between;
                align-tracks: stretch;
            }",
        )
        .unwrap();
        let declarations = sheet.rules()[0].declarations();

        for property in [
            CssProperty::GridTemplateRows,
            CssProperty::GridTemplateColumns,
            CssProperty::GridTemplateAreas,
            CssProperty::GridTemplate,
            CssProperty::GridAutoRows,
            CssProperty::GridAutoColumns,
            CssProperty::GridAutoFlow,
            CssProperty::GridRowStart,
            CssProperty::GridRowEnd,
            CssProperty::GridColumnStart,
            CssProperty::GridColumnEnd,
            CssProperty::GridRow,
            CssProperty::GridColumn,
            CssProperty::GridArea,
            CssProperty::Grid,
            CssProperty::Order,
            CssProperty::Flex,
            CssProperty::JustifyTracks,
            CssProperty::AlignTracks,
        ] {
            assert!(
                declarations
                    .iter()
                    .any(|declaration| declaration.property() == property),
                "missing parsed declaration for {property:?}",
            );
        }
    }

    #[test]
    fn rejects_task_4_cross_family_leakage_values() {
        for input in [
            ".panel { order: 1.2; }",
            ".panel { grid-auto-flow: left; }",
            ".panel { grid-template-areas: \"a a\" \"a .\"; }",
            ".panel { grid-row: 1 / / 2; }",
            ".panel { flex: solid; }",
            ".panel { justify-tracks: auto; }",
        ] {
            let error = parse_sheet(input).expect_err(input);
            assert!(matches!(
                error.kind(),
                ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
            ));
        }
    }

    #[test]
    fn checked_grid_constructors_reject_parser_invalid_states() {
        assert_eq!(CssCustomIdent::try_new(""), None);
        assert_eq!(CssCustomIdent::try_new("auto"), None);
        assert_eq!(
            CssCustomIdent::try_new("main"),
            Some(CssCustomIdent::new("main"))
        );
        assert_eq!(CssGridLineNames::try_new(Vec::new()), None);
        assert_eq!(CssGridTrackList::try_new(Vec::new()), None);
        assert_eq!(CssGridRepeatCount::try_integer(0), None);
        assert_eq!(
            CssGridRepeat::try_new(
                CssGridRepeatCount::integer(1),
                CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                    CssGridTrackSize::breadth(CssGridTrackBreadth::Fraction(1.0))
                )])
            ),
            Some(CssGridRepeat::new(
                CssGridRepeatCount::integer(1),
                CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                    CssGridTrackSize::breadth(CssGridTrackBreadth::Fraction(1.0))
                )])
            ))
        );
        assert_eq!(
            CssGridRepeat::try_new(
                CssGridRepeatCount::integer(1),
                CssGridTrackList::new(vec![])
            ),
            None
        );
        assert_eq!(CssGridTemplateAreaRow::try_new(Vec::new()), None);
        assert_eq!(CssGridTemplateAreas::try_rows(Vec::new()), None);
        assert_eq!(
            CssGridTemplateAreas::try_rows(vec![
                CssGridTemplateAreaRow::new(vec![
                    CssGridTemplateAreaCell::Named(CssCustomIdent::new("a")),
                    CssGridTemplateAreaCell::Named(CssCustomIdent::new("a")),
                ]),
                CssGridTemplateAreaRow::new(vec![
                    CssGridTemplateAreaCell::Named(CssCustomIdent::new("a")),
                    CssGridTemplateAreaCell::Empty,
                ]),
            ]),
            None
        );
        assert_eq!(CssGridLine::try_integer(0), None);
        assert_eq!(CssGridLineSpan::try_new(None, None), None);
        assert_eq!(CssGridLineSpan::try_new(Some(0), None), None);
    }

    #[test]
    fn rejects_grid_auto_flow_shorthand_without_explicit_tracks() {
        for input in [
            ".panel { grid: auto-flow; }",
            ".panel { grid: auto-flow dense; }",
            ".panel { grid: auto-flow 12px; }",
        ] {
            let error = parse_sheet(input).expect_err(input);
            assert!(matches!(
                error.kind(),
                ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
            ));
        }
    }

    #[test]
    fn invalid_parser_custom_ident_errors_keep_source_location() {
        let error = parse_sheet(".panel {\n  grid-template-columns: [auto] 1fr;\n}").unwrap_err();

        assert!(matches!(error.kind(), ErrorKind::UnsupportedValue { .. }));
        assert_ne!(error.line(), 0);
        assert_ne!(error.column(), 0);
        assert_eq!(error.line(), 1);
    }

    #[test]
    fn rejects_inconsistent_grid_template_area_row_widths() {
        let error = parse_sheet(".panel { grid-template-areas: \"a a\" \"b\"; }").unwrap_err();

        assert!(matches!(error.kind(), ErrorKind::UnsupportedValue { .. }));
        assert!(error.message().contains("inconsistent widths"));
    }
}

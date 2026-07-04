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
#[cfg(test)]
mod test_support;
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
            "background" | "background-color" => (CssProperty::Background, CssValue::Color(parse_color(input)?)),
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
            "opacity" => (CssProperty::Opacity, CssValue::Number(parse_number(input)?)),
            "flex-grow" => (CssProperty::FlexGrow, CssValue::Number(parse_number(input)?)),
            "flex-shrink" => (CssProperty::FlexShrink, CssValue::Number(parse_number(input)?)),
            "order" => (CssProperty::Order, CssValue::Order(parse_order(input)?)),
            "flex" => (CssProperty::Flex, CssValue::Flex(parse_flex(input)?)),
            "justify-tracks" => (CssProperty::JustifyTracks, CssValue::Alignment(parse_content_alignment(input)?)),
            "align-tracks" => (CssProperty::AlignTracks, CssValue::Alignment(parse_content_alignment(input)?)),
            "aspect-ratio" => (CssProperty::AspectRatio, CssValue::Number(parse_number(input)?)),
            "scrollbar-width" => (CssProperty::ScrollbarWidth, CssValue::Number(parse_number(input)?)),
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

fn parse_writing_mode<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssWritingMode, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "horizontal-tb" => Ok(CssWritingMode::HorizontalTb),
        "vertical-rl" => Ok(CssWritingMode::VerticalRl),
        "vertical-lr" => Ok(CssWritingMode::VerticalLr),
        "sideways-rl" => Ok(CssWritingMode::SidewaysRl),
        "sideways-lr" => Ok(CssWritingMode::SidewaysLr),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("writing-mode", ident.as_ref()),
        )),
    }
}

fn parse_text_align<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextAlign, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "start" => Ok(CssTextAlign::Start),
        "end" => Ok(CssTextAlign::End),
        "left" => Ok(CssTextAlign::Left),
        "right" => Ok(CssTextAlign::Right),
        "center" => Ok(CssTextAlign::Center),
        "justify" => Ok(CssTextAlign::Justify),
        "match-parent" => Ok(CssTextAlign::MatchParent),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("text-align", ident.as_ref()),
        )),
    }
}

fn parse_text_align_last<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextAlignLast, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "auto" => Ok(CssTextAlignLast::Auto),
        "start" => Ok(CssTextAlignLast::Start),
        "end" => Ok(CssTextAlignLast::End),
        "left" => Ok(CssTextAlignLast::Left),
        "right" => Ok(CssTextAlignLast::Right),
        "center" => Ok(CssTextAlignLast::Center),
        "justify" => Ok(CssTextAlignLast::Justify),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("text-align-last", ident.as_ref()),
        )),
    }
}

fn parse_text_indent<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextIndent, ParseError<'i, Error>> {
    let length = parse_length_with(input, LengthOptions::text_indent(), "text-indent")?;
    let mut hanging = false;
    let mut each_line = false;

    while !input.is_exhausted() {
        let ident = input.expect_ident_cloned().map_err(basic)?;
        match_ignore_ascii_case! { &ident,
            "hanging" if !hanging => hanging = true,
            "each-line" if !each_line => each_line = true,
            _ => return Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("text-indent", ident.as_ref()),
            )),
        }
    }

    Ok(CssTextIndent::new(length, hanging, each_line))
}

fn parse_vertical_align<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssVerticalAlign, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "baseline" => Ok(CssVerticalAlign::Baseline),
            "sub" => Ok(CssVerticalAlign::Sub),
            "super" => Ok(CssVerticalAlign::Super),
            "text-top" => Ok(CssVerticalAlign::TextTop),
            "text-bottom" => Ok(CssVerticalAlign::TextBottom),
            "middle" => Ok(CssVerticalAlign::Middle),
            "top" => Ok(CssVerticalAlign::Top),
            "bottom" => Ok(CssVerticalAlign::Bottom),
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("vertical-align", ident.as_ref()),
            )),
        };
    }

    parse_length_with(input, LengthOptions::vertical_align(), "vertical-align")
        .map(CssVerticalAlignLength::new)
        .map(CssVerticalAlign::Length)
}

fn parse_font_family_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFontFamilyList, ParseError<'i, Error>> {
    let mut families = Vec::new();
    loop {
        families.push(parse_font_family_name(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "font-family list has an empty item",
            ));
        }
    }

    CssFontFamilyList::try_new(families)
        .ok_or_else(|| unsupported_value(input, None, "font-family list is empty"))
}

fn parse_font_family_name<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFontFamilyName, ParseError<'i, Error>> {
    if let Ok(name) = input.try_parse(Parser::expect_string_cloned) {
        if name.is_empty() {
            return Err(unsupported_value(
                input,
                None,
                "font family string is empty",
            ));
        }
        return Ok(CssFontFamilyName::quoted(name.to_string()));
    }

    let mut parts = Vec::new();
    while !input.is_exhausted() && !next_is_comma(input) {
        let location = input.current_source_location();
        match input.next().map_err(basic)? {
            Token::Ident(ident) => parts.push(ident.to_string()),
            token => return Err(location.new_unexpected_token_error::<Error>(token.clone())),
        }
    }

    if parts.is_empty() {
        Err(unsupported_value(input, None, "font family name is empty"))
    } else {
        Ok(CssFontFamilyName::ident_sequence(parts.join(" ")))
    }
}

fn parse_font<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFont, ParseError<'i, Error>> {
    let mut style = None;
    let mut variant = None;
    let mut weight = None;
    let mut stretch = None;
    let size;

    loop {
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "font shorthand is missing a size",
            ));
        }

        if let Ok(parsed_size) = input.try_parse(parse_font_size) {
            size = parsed_size;
            break;
        }

        if let Ok(()) = input.try_parse(|input| {
            input.expect_ident_matching("normal").map_err(basic)?;
            if style.is_none() {
                style = Some(CssFontStyle::Normal);
            } else if variant.is_none() {
                variant = Some(CssFontVariant::Normal);
            } else if weight.is_none() {
                weight = Some(CssFontWeight::Normal);
            } else if stretch.is_none() {
                stretch = Some(CssFontStretch::Normal);
            } else {
                return Err(unsupported_value(
                    input,
                    None,
                    "duplicate font normal component",
                ));
            }
            Ok(())
        }) {
            continue;
        }

        if style.is_none()
            && let Ok(parsed_style) = input.try_parse(parse_font_style)
        {
            style = Some(parsed_style);
            continue;
        }
        if variant.is_none()
            && let Ok(parsed_variant) = input.try_parse(parse_font_variant)
        {
            variant = Some(parsed_variant);
            continue;
        }
        if weight.is_none()
            && let Ok(parsed_weight) = input.try_parse(parse_font_weight)
        {
            weight = Some(parsed_weight);
            continue;
        }
        if stretch.is_none()
            && let Ok(parsed_stretch) = input.try_parse(parse_font_stretch)
        {
            stretch = Some(parsed_stretch);
            continue;
        }

        return Err(unsupported_value(
            input,
            None,
            "unsupported font shorthand component before size",
        ));
    }

    let line_height = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(parse_line_height(input)?)
    } else {
        None
    };
    let families = parse_font_family_list(input)?;

    CssFont::try_new(style, variant, weight, stretch, size, line_height, families)
        .ok_or_else(|| unsupported_value(input, None, "invalid font shorthand"))
}

fn parse_font_weight<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFontWeight, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Ident(ident) => match_ignore_ascii_case! { ident,
            "normal" => Ok(CssFontWeight::Normal),
            "bold" => Ok(CssFontWeight::Bold),
            "bolder" => Ok(CssFontWeight::Bolder),
            "lighter" => Ok(CssFontWeight::Lighter),
            _ => Err(unsupported_value_at(
                location,
                None,
                unsupported_keyword_reason("font-weight", ident.as_ref()),
            )),
        },
        Token::Number {
            int_value: Some(value),
            ..
        } if CssFontWeightNumber::try_new(*value).is_some() => {
            Ok(CssFontWeight::Number(CssFontWeightNumber::new(*value)))
        }
        Token::Number {
            int_value: Some(_), ..
        } => Err(unsupported_value_at(
            location,
            None,
            "font-weight must be 1 through 1000",
        )),
        Token::Number { .. } => Err(unsupported_value_at(
            location,
            None,
            "font-weight number must be an integer",
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_font_style<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFontStyle, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "normal" => Ok(CssFontStyle::Normal),
        "italic" => Ok(CssFontStyle::Italic),
        "oblique" => Ok(CssFontStyle::Oblique),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("font-style", ident.as_ref()),
        )),
    }
}

fn parse_font_stretch<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFontStretch, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "normal" => Ok(CssFontStretch::Normal),
        "ultra-condensed" => Ok(CssFontStretch::UltraCondensed),
        "extra-condensed" => Ok(CssFontStretch::ExtraCondensed),
        "condensed" => Ok(CssFontStretch::Condensed),
        "semi-condensed" => Ok(CssFontStretch::SemiCondensed),
        "semi-expanded" => Ok(CssFontStretch::SemiExpanded),
        "expanded" => Ok(CssFontStretch::Expanded),
        "extra-expanded" => Ok(CssFontStretch::ExtraExpanded),
        "ultra-expanded" => Ok(CssFontStretch::UltraExpanded),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("font-stretch", ident.as_ref()),
        )),
    }
}

fn parse_font_variant<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFontVariant, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "normal" => Ok(CssFontVariant::Normal),
        "small-caps" => Ok(CssFontVariant::SmallCaps),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("font-variant", ident.as_ref()),
        )),
    }
}

fn parse_font_feature_settings<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFontFeatureSettings, ParseError<'i, Error>> {
    let state = input.state();
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        if ident.eq_ignore_ascii_case("normal") && input.is_exhausted() {
            return Ok(CssFontFeatureSettings::Normal);
        }
        input.reset(&state);
    }

    let mut features = Vec::new();
    loop {
        features.push(parse_font_feature(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "font-feature-settings list has an empty item",
            ));
        }
    }

    CssFontFeatureList::try_new(features)
        .map(CssFontFeatureSettings::Features)
        .ok_or_else(|| unsupported_value(input, None, "font-feature-settings list is empty"))
}

fn parse_font_feature<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFontFeature, ParseError<'i, Error>> {
    let tag = input.expect_string_cloned().map_err(basic)?.to_string();
    if tag.is_empty() {
        return Err(unsupported_value(input, None, "font feature tag is empty"));
    }

    let value = if input.is_exhausted() || next_is_comma(input) {
        None
    } else if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        Some(match_ignore_ascii_case! { &ident,
            "on" => CssFontFeatureValue::On,
            "off" => CssFontFeatureValue::Off,
            _ => return Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("font feature value", ident.as_ref()),
            )),
        })
    } else {
        let value = parse_integer(input, "font feature value")?;
        Some(CssFontFeatureValue::Integer(value))
    };

    CssFontFeature::try_new(tag, value)
        .ok_or_else(|| unsupported_value(input, None, "font feature tag must be four characters"))
}

fn parse_letter_spacing<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLetterSpacing, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("normal"))
        .is_ok()
    {
        Ok(CssLetterSpacing::Normal)
    } else {
        parse_length_with(input, LengthOptions::letter_spacing(), "letter-spacing")
            .map(CssLetterSpacingLength::new)
            .map(CssLetterSpacing::Length)
    }
}

fn parse_text_wrap<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextWrap, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "wrap" => Ok(CssTextWrap::Wrap),
        "nowrap" => Ok(CssTextWrap::NoWrap),
        "balance" => Ok(CssTextWrap::Balance),
        "pretty" => Ok(CssTextWrap::Pretty),
        "stable" => Ok(CssTextWrap::Stable),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("text-wrap", ident.as_ref()),
        )),
    }
}

fn parse_white_space<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssWhiteSpace, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "normal" => Ok(CssWhiteSpace::Normal),
        "nowrap" => Ok(CssWhiteSpace::NoWrap),
        "pre" => Ok(CssWhiteSpace::Pre),
        "pre-wrap" => Ok(CssWhiteSpace::PreWrap),
        "pre-line" => Ok(CssWhiteSpace::PreLine),
        "break-spaces" => Ok(CssWhiteSpace::BreakSpaces),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("white-space", ident.as_ref()),
        )),
    }
}

fn parse_word_break<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssWordBreak, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "normal" => Ok(CssWordBreak::Normal),
        "break-all" => Ok(CssWordBreak::BreakAll),
        "keep-all" => Ok(CssWordBreak::KeepAll),
        "break-word" => Ok(CssWordBreak::BreakWord),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("word-break", ident.as_ref()),
        )),
    }
}

fn parse_overflow_wrap<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssOverflowWrap, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "normal" => Ok(CssOverflowWrap::Normal),
        "break-word" => Ok(CssOverflowWrap::BreakWord),
        "anywhere" => Ok(CssOverflowWrap::Anywhere),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("overflow-wrap", ident.as_ref()),
        )),
    }
}

fn parse_text_overflow<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextOverflow, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "clip" => Ok(CssTextOverflow::Clip),
        "ellipsis" => Ok(CssTextOverflow::Ellipsis),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("text-overflow", ident.as_ref()),
        )),
    }
}

fn parse_text_decoration<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextDecoration, ParseError<'i, Error>> {
    let mut line_components = Vec::new();
    let mut line_none = false;
    let mut color = None;
    let mut style = None;
    let mut thickness = None;

    while !input.is_exhausted() {
        if let Ok(component) = input.try_parse(parse_text_decoration_line_component) {
            if line_none {
                return Err(unsupported_value(
                    input,
                    None,
                    "text-decoration line mixes none with line components",
                ));
            }
            if line_components.contains(&component) {
                return Err(unsupported_value(
                    input,
                    None,
                    "duplicate text-decoration-line component",
                ));
            }
            line_components.push(component);
            continue;
        }
        if input
            .try_parse(|input| input.expect_ident_matching("none"))
            .is_ok()
        {
            if line_none || !line_components.is_empty() {
                return Err(unsupported_value(
                    input,
                    None,
                    "duplicate text-decoration-line none",
                ));
            }
            line_none = true;
            continue;
        }
        if style.is_none()
            && let Ok(parsed_style) = input.try_parse(parse_text_decoration_style)
        {
            style = Some(parsed_style);
            continue;
        }
        if thickness.is_none()
            && let Ok(parsed_thickness) = input.try_parse(parse_text_decoration_thickness)
        {
            thickness = Some(parsed_thickness);
            continue;
        }
        if color.is_none()
            && let Ok(parsed_color) = input.try_parse(parse_color)
        {
            color = Some(parsed_color);
            continue;
        }

        return Err(unsupported_value(
            input,
            None,
            "unsupported text-decoration component",
        ));
    }

    let line = if line_none {
        Some(CssTextDecorationLine::none())
    } else if line_components.is_empty() {
        None
    } else {
        Some(CssTextDecorationLine::new(line_components))
    };

    CssTextDecoration::try_new(line, color, style, thickness)
        .ok_or_else(|| unsupported_value(input, None, "text-decoration shorthand is empty"))
}

fn parse_text_decoration_line<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextDecorationLine, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(CssTextDecorationLine::none());
    }

    let mut components = Vec::new();
    while !input.is_exhausted() {
        let component = parse_text_decoration_line_component(input)?;
        if components.contains(&component) {
            return Err(unsupported_value(
                input,
                None,
                "duplicate text-decoration-line component",
            ));
        }
        components.push(component);
    }

    CssTextDecorationLine::try_new(components)
        .ok_or_else(|| unsupported_value(input, None, "text-decoration-line is empty"))
}

fn parse_text_decoration_line_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextDecorationLineComponent, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "underline" => Ok(CssTextDecorationLineComponent::Underline),
        "overline" => Ok(CssTextDecorationLineComponent::Overline),
        "line-through" => Ok(CssTextDecorationLineComponent::LineThrough),
        "blink" => Ok(CssTextDecorationLineComponent::Blink),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("text-decoration-line", ident.as_ref()),
        )),
    }
}

fn parse_text_decoration_style<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextDecorationStyle, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "solid" => Ok(CssTextDecorationStyle::Solid),
        "double" => Ok(CssTextDecorationStyle::Double),
        "dotted" => Ok(CssTextDecorationStyle::Dotted),
        "dashed" => Ok(CssTextDecorationStyle::Dashed),
        "wavy" => Ok(CssTextDecorationStyle::Wavy),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("text-decoration-style", ident.as_ref()),
        )),
    }
}

fn parse_text_decoration_thickness<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextDecorationThickness, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "auto" => Ok(CssTextDecorationThickness::Auto),
            "from-font" => Ok(CssTextDecorationThickness::FromFont),
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("text-decoration-thickness", ident.as_ref()),
            )),
        };
    }

    parse_length_with(
        input,
        LengthOptions::text_decoration_thickness(),
        "text-decoration-thickness",
    )
    .map(CssTextDecorationThicknessLength::new)
    .map(CssTextDecorationThickness::Length)
}

fn parse_text_transform<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextTransform, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "none" => Ok(CssTextTransform::None),
        "capitalize" => Ok(CssTextTransform::Capitalize),
        "uppercase" => Ok(CssTextTransform::Uppercase),
        "lowercase" => Ok(CssTextTransform::Lowercase),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("text-transform", ident.as_ref()),
        )),
    }
}

fn parse_image_layer_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssImageLayerList, ParseError<'i, Error>> {
    let mut layers = Vec::new();
    loop {
        layers.push(parse_image_layer(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "image layer list has an empty item",
            ));
        }
    }
    CssImageLayerList::try_new(layers)
        .ok_or_else(|| unsupported_value(input, None, "image layer list is empty"))
}

fn parse_image_layer<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssImageLayer, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(CssImageLayer::None);
    }
    parse_url(input).map(CssImageLayer::Url)
}

fn parse_url<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssUrl, ParseError<'i, Error>> {
    let value = input.expect_url().map_err(basic)?.to_string();
    CssUrl::try_new(value).ok_or_else(|| unsupported_value(input, None, "URL is empty"))
}

fn parse_position_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPositionList, ParseError<'i, Error>> {
    let mut positions = Vec::new();
    loop {
        positions.push(parse_css_position(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "position list has an empty item",
            ));
        }
    }
    CssPositionList::try_new(positions)
        .ok_or_else(|| unsupported_value(input, None, "position list is empty"))
}

fn parse_css_position<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPosition, ParseError<'i, Error>> {
    let mut components = Vec::new();
    while !input.is_exhausted() && !next_is_comma(input) && !next_is_delim(input, '/') {
        components.push(parse_position_component(input, &components)?);
        if components.len() > 4 {
            return Err(unsupported_value(
                input,
                None,
                "position has too many components",
            ));
        }
    }
    CssPosition::try_new(components)
        .ok_or_else(|| unsupported_value(input, None, "position is empty"))
}

fn parse_position_component<'i, 't>(
    input: &mut Parser<'i, 't>,
    previous: &[CssPositionComponent],
) -> std::result::Result<CssPositionComponent, ParseError<'i, Error>> {
    let state = input.state();
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "left" => Ok(CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Left)),
            "right" => Ok(CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Right)),
            "top" => Ok(CssPositionComponent::Vertical(CssVerticalPositionKeyword::Top)),
            "bottom" => Ok(CssPositionComponent::Vertical(CssVerticalPositionKeyword::Bottom)),
            "center" => {
                let has_horizontal = previous.iter().any(|component| matches!(component, CssPositionComponent::Horizontal(_)));
                if has_horizontal {
                    Ok(CssPositionComponent::Vertical(CssVerticalPositionKeyword::Center))
                } else {
                    Ok(CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center))
                }
            },
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("position", ident.as_ref()),
            )),
        };
    }
    input.reset(&state);
    parse_length_with(input, LengthOptions::position(), "position")
        .map(CssPositionComponent::Length)
}

fn parse_background_size_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundSizeList, ParseError<'i, Error>> {
    let mut sizes = Vec::new();
    loop {
        sizes.push(parse_background_size(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "background-size list has an empty item",
            ));
        }
    }
    CssBackgroundSizeList::try_new(sizes)
        .ok_or_else(|| unsupported_value(input, None, "background-size list is empty"))
}

fn parse_background_size<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundSize, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "cover" => Ok(CssBackgroundSize::Cover),
            "contain" => Ok(CssBackgroundSize::Contain),
            "auto" => {
                let height = if !input.is_exhausted() && !next_is_comma(input) {
                    Some(parse_background_size_component(input)?)
                } else {
                    None
                };
                Ok(CssBackgroundSize::Explicit {
                    width: CssBackgroundSizeComponent::Auto,
                    height,
                })
            },
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("background-size", ident.as_ref()),
            )),
        };
    }

    let width = parse_background_size_component(input)?;
    let height = if !input.is_exhausted() && !next_is_comma(input) {
        Some(parse_background_size_component(input)?)
    } else {
        None
    };
    Ok(CssBackgroundSize::Explicit { width, height })
}

fn parse_background_size_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundSizeComponent, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("auto"))
        .is_ok()
    {
        Ok(CssBackgroundSizeComponent::Auto)
    } else {
        parse_length_with(input, LengthOptions::background_size(), "background-size")
            .map(CssBackgroundSizeComponent::Length)
    }
}

fn parse_background_repeat_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundRepeatList, ParseError<'i, Error>> {
    let mut repeats = Vec::new();
    loop {
        repeats.push(parse_background_repeat(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "background-repeat list has an empty item",
            ));
        }
    }
    CssBackgroundRepeatList::try_new(repeats)
        .ok_or_else(|| unsupported_value(input, None, "background-repeat list is empty"))
}

fn parse_background_repeat<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundRepeat, ParseError<'i, Error>> {
    let first = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &first,
        "repeat-x" => Ok(CssBackgroundRepeat::RepeatX),
        "repeat-y" => Ok(CssBackgroundRepeat::RepeatY),
        _ => {
            let x = parse_background_repeat_style_from_ident(input, first.as_ref())?;
            let y = if input.is_exhausted() || next_is_comma(input) {
                x
            } else {
                let second = input.expect_ident_cloned().map_err(basic)?;
                parse_background_repeat_style_from_ident(input, second.as_ref())?
            };
            Ok(CssBackgroundRepeat::Axes { x, y })
        }
    }
}

fn parse_background_repeat_style_from_ident<'i, 't>(
    input: &Parser<'i, 't>,
    ident: &str,
) -> std::result::Result<CssBackgroundRepeatStyle, ParseError<'i, Error>> {
    match ident.to_ascii_lowercase().as_str() {
        "repeat" => Ok(CssBackgroundRepeatStyle::Repeat),
        "space" => Ok(CssBackgroundRepeatStyle::Space),
        "round" => Ok(CssBackgroundRepeatStyle::Round),
        "no-repeat" => Ok(CssBackgroundRepeatStyle::NoRepeat),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("background-repeat", ident),
        )),
    }
}

fn parse_background_box<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundBox, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "border-box" => Ok(CssBackgroundBox::BorderBox),
        "padding-box" => Ok(CssBackgroundBox::PaddingBox),
        "content-box" => Ok(CssBackgroundBox::ContentBox),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("background box", ident.as_ref()),
        )),
    }
}

fn parse_background_attachment_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundAttachmentList, ParseError<'i, Error>> {
    let mut attachments = Vec::new();
    loop {
        attachments.push(parse_background_attachment(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "background-attachment list has an empty item",
            ));
        }
    }
    CssBackgroundAttachmentList::try_new(attachments)
        .ok_or_else(|| unsupported_value(input, None, "background-attachment list is empty"))
}

fn parse_background_attachment<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundAttachment, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "scroll" => Ok(CssBackgroundAttachment::Scroll),
        "fixed" => Ok(CssBackgroundAttachment::Fixed),
        "local" => Ok(CssBackgroundAttachment::Local),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("background-attachment", ident.as_ref()),
        )),
    }
}

fn parse_cursor<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssCursor, ParseError<'i, Error>> {
    let mut urls = Vec::new();
    while let Ok(url) = input.try_parse(parse_url) {
        urls.push(url);
        input.expect_comma().map_err(basic)?;
    }
    let fallback = parse_cursor_keyword(input)?;
    if urls.is_empty() {
        Ok(CssCursor::Keyword(fallback))
    } else {
        Ok(CssCursor::urls(urls, fallback))
    }
}

fn parse_cursor_keyword<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssCursorKeyword, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "auto" => Ok(CssCursorKeyword::Auto),
        "default" => Ok(CssCursorKeyword::Default),
        "none" => Ok(CssCursorKeyword::None),
        "context-menu" => Ok(CssCursorKeyword::ContextMenu),
        "help" => Ok(CssCursorKeyword::Help),
        "pointer" => Ok(CssCursorKeyword::Pointer),
        "progress" => Ok(CssCursorKeyword::Progress),
        "wait" => Ok(CssCursorKeyword::Wait),
        "cell" => Ok(CssCursorKeyword::Cell),
        "crosshair" => Ok(CssCursorKeyword::Crosshair),
        "text" => Ok(CssCursorKeyword::Text),
        "vertical-text" => Ok(CssCursorKeyword::VerticalText),
        "alias" => Ok(CssCursorKeyword::Alias),
        "copy" => Ok(CssCursorKeyword::Copy),
        "move" => Ok(CssCursorKeyword::Move),
        "no-drop" => Ok(CssCursorKeyword::NoDrop),
        "not-allowed" => Ok(CssCursorKeyword::NotAllowed),
        "grab" => Ok(CssCursorKeyword::Grab),
        "grabbing" => Ok(CssCursorKeyword::Grabbing),
        "all-scroll" => Ok(CssCursorKeyword::AllScroll),
        "col-resize" => Ok(CssCursorKeyword::ColResize),
        "row-resize" => Ok(CssCursorKeyword::RowResize),
        "n-resize" => Ok(CssCursorKeyword::NResize),
        "e-resize" => Ok(CssCursorKeyword::EResize),
        "s-resize" => Ok(CssCursorKeyword::SResize),
        "w-resize" => Ok(CssCursorKeyword::WResize),
        "ne-resize" => Ok(CssCursorKeyword::NeResize),
        "nw-resize" => Ok(CssCursorKeyword::NwResize),
        "se-resize" => Ok(CssCursorKeyword::SeResize),
        "sw-resize" => Ok(CssCursorKeyword::SwResize),
        "ew-resize" => Ok(CssCursorKeyword::EwResize),
        "ns-resize" => Ok(CssCursorKeyword::NsResize),
        "nesw-resize" => Ok(CssCursorKeyword::NeswResize),
        "nwse-resize" => Ok(CssCursorKeyword::NwseResize),
        "zoom-in" => Ok(CssCursorKeyword::ZoomIn),
        "zoom-out" => Ok(CssCursorKeyword::ZoomOut),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("cursor", ident.as_ref()),
        )),
    }
}

fn parse_pointer_events<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPointerEvents, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "auto" => Ok(CssPointerEvents::Auto),
        "none" => Ok(CssPointerEvents::None),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("pointer-events", ident.as_ref()),
        )),
    }
}

fn parse_user_select<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssUserSelect, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "auto" => Ok(CssUserSelect::Auto),
        "text" => Ok(CssUserSelect::Text),
        "none" => Ok(CssUserSelect::None),
        "all" => Ok(CssUserSelect::All),
        "contain" => Ok(CssUserSelect::Contain),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("user-select", ident.as_ref()),
        )),
    }
}

fn parse_outline<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssOutline, ParseError<'i, Error>> {
    let mut width = None;
    let mut style = None;
    let mut color = None;
    while !input.is_exhausted() {
        if width.is_none()
            && let Ok(parsed_width) = input.try_parse(parse_outline_width)
        {
            width = Some(parsed_width);
            continue;
        }
        if style.is_none()
            && let Ok(parsed_style) = input.try_parse(parse_outline_style)
        {
            style = Some(parsed_style);
            continue;
        }
        if color.is_none()
            && let Ok(parsed_color) = input.try_parse(parse_color)
        {
            color = Some(parsed_color);
            continue;
        }
        return Err(unsupported_value(
            input,
            None,
            "unsupported outline component",
        ));
    }
    CssOutline::try_new(width, style, color)
        .ok_or_else(|| unsupported_value(input, None, "outline shorthand is empty"))
}

fn parse_outline_style<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssOutlineStyle, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("auto"))
        .is_ok()
    {
        Ok(CssOutlineStyle::Auto)
    } else {
        parse_border_style(input).map(CssOutlineStyle::Border)
    }
}

fn parse_outline_width<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssOutlineWidth, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "thin" => Ok(CssOutlineWidth::Thin),
            "medium" => Ok(CssOutlineWidth::Medium),
            "thick" => Ok(CssOutlineWidth::Thick),
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("outline-width", ident.as_ref()),
            )),
        };
    }
    parse_length_with(input, LengthOptions::border_width(), "outline-width")
        .map(CssOutlineWidth::Length)
}

fn parse_transform<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransform, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(CssTransform::None);
    }
    let mut functions = Vec::new();
    while !input.is_exhausted() {
        functions.push(parse_transform_function(input)?);
    }
    CssTransformFunctionList::try_new(functions)
        .map(CssTransform::Functions)
        .ok_or_else(|| unsupported_value(input, None, "transform function list is empty"))
}

fn parse_transform_function<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransformFunction, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let name = match input.next().map_err(basic)? {
        Token::Function(name) => name.clone(),
        token => return Err(location.new_unexpected_token_error::<Error>(token.clone())),
    };
    let kind = parse_transform_function_kind(input, name.as_ref())?;
    let arguments =
        input.parse_nested_block(|input| parse_transform_function_arguments(input, kind))?;
    Ok(CssTransformFunction::new(kind, arguments))
}

fn parse_transform_function_kind<'i, 't>(
    input: &Parser<'i, 't>,
    name: &str,
) -> std::result::Result<CssTransformFunctionKind, ParseError<'i, Error>> {
    match name.to_ascii_lowercase().as_str() {
        "matrix" => Ok(CssTransformFunctionKind::Matrix),
        "matrix3d" => Ok(CssTransformFunctionKind::Matrix3d),
        "perspective" => Ok(CssTransformFunctionKind::Perspective),
        "rotate" => Ok(CssTransformFunctionKind::Rotate),
        "rotate3d" => Ok(CssTransformFunctionKind::Rotate3d),
        "rotatex" => Ok(CssTransformFunctionKind::RotateX),
        "rotatey" => Ok(CssTransformFunctionKind::RotateY),
        "rotatez" => Ok(CssTransformFunctionKind::RotateZ),
        "scale" => Ok(CssTransformFunctionKind::Scale),
        "scale3d" => Ok(CssTransformFunctionKind::Scale3d),
        "scalex" => Ok(CssTransformFunctionKind::ScaleX),
        "scaley" => Ok(CssTransformFunctionKind::ScaleY),
        "scalez" => Ok(CssTransformFunctionKind::ScaleZ),
        "skew" => Ok(CssTransformFunctionKind::Skew),
        "skewx" => Ok(CssTransformFunctionKind::SkewX),
        "skewy" => Ok(CssTransformFunctionKind::SkewY),
        "translate" => Ok(CssTransformFunctionKind::Translate),
        "translate3d" => Ok(CssTransformFunctionKind::Translate3d),
        "translatex" => Ok(CssTransformFunctionKind::TranslateX),
        "translatey" => Ok(CssTransformFunctionKind::TranslateY),
        "translatez" => Ok(CssTransformFunctionKind::TranslateZ),
        _ => Err(unsupported_value(
            input,
            None,
            format!("unsupported transform function `{name}`"),
        )),
    }
}

fn parse_transform_function_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    kind: CssTransformFunctionKind,
) -> std::result::Result<CssFunctionArguments, ParseError<'i, Error>> {
    parse_validated_function_arguments(input, "transform function", |input| match kind {
        CssTransformFunctionKind::Translate => validate_length_sequence(input, 1, 2),
        CssTransformFunctionKind::TranslateX
        | CssTransformFunctionKind::TranslateY
        | CssTransformFunctionKind::TranslateZ
        | CssTransformFunctionKind::Perspective => validate_length_sequence(input, 1, 1),
        CssTransformFunctionKind::Translate3d => validate_length_sequence(input, 3, 3),
        CssTransformFunctionKind::Scale => validate_number_sequence(input, 1, 2),
        CssTransformFunctionKind::ScaleX
        | CssTransformFunctionKind::ScaleY
        | CssTransformFunctionKind::ScaleZ => validate_number_sequence(input, 1, 1),
        CssTransformFunctionKind::Scale3d => validate_number_sequence(input, 3, 3),
        CssTransformFunctionKind::Rotate
        | CssTransformFunctionKind::RotateX
        | CssTransformFunctionKind::RotateY
        | CssTransformFunctionKind::RotateZ
        | CssTransformFunctionKind::SkewX
        | CssTransformFunctionKind::SkewY => validate_angle_sequence(input, 1, 1),
        CssTransformFunctionKind::Skew => validate_angle_sequence(input, 1, 2),
        CssTransformFunctionKind::Rotate3d => {
            validate_number_sequence_prefix(input, 3)
                && consume_optional_comma(input)
                && validate_angle(input)
                && input.is_exhausted()
        }
        CssTransformFunctionKind::Matrix => validate_number_sequence(input, 6, 6),
        CssTransformFunctionKind::Matrix3d => validate_number_sequence(input, 16, 16),
    })
}

fn parse_filter_function_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    name: &str,
) -> std::result::Result<CssFunctionArguments, ParseError<'i, Error>> {
    parse_validated_function_arguments(input, "filter function", |input| {
        match name.to_ascii_lowercase().as_str() {
            "blur" => validate_non_negative_length(input),
            "brightness" | "contrast" | "grayscale" | "invert" | "opacity" | "saturate"
            | "sepia" => validate_number_or_percent(input),
            "hue-rotate" => validate_angle(input) && input.is_exhausted(),
            "drop-shadow" => input.try_parse(parse_shadow).is_ok() && input.is_exhausted(),
            _ => false,
        }
    })
}

fn parse_basic_shape_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    name: &str,
) -> std::result::Result<CssFunctionArguments, ParseError<'i, Error>> {
    parse_validated_function_arguments(input, "basic shape", |input| {
        match name.to_ascii_lowercase().as_str() {
            "circle" => validate_circle_shape(input),
            "ellipse" => validate_ellipse_shape(input),
            "inset" => validate_inset_shape(input),
            "polygon" => validate_polygon_shape(input),
            _ => false,
        }
    })
}

fn parse_easing_function_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    name: &str,
) -> std::result::Result<CssFunctionArguments, ParseError<'i, Error>> {
    parse_validated_function_arguments(input, "easing function", |input| {
        match name.to_ascii_lowercase().as_str() {
            "cubic-bezier" => validate_cubic_bezier(input),
            "steps" => validate_steps(input),
            _ => false,
        }
    })
}

fn parse_validated_function_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    context: &str,
    validate: impl for<'a, 'b> FnMut(&mut Parser<'a, 'b>) -> bool,
) -> std::result::Result<CssFunctionArguments, ParseError<'i, Error>> {
    let value = collect_authored_tokens(input)?;
    if value.is_empty() {
        return Err(unsupported_value(
            input,
            None,
            "function arguments are empty",
        ));
    }
    if !validate_authored_function_arguments(&value, validate) {
        return Err(unsupported_value(
            input,
            None,
            format!("invalid {context} arguments"),
        ));
    }
    Ok(CssFunctionArguments::new(value))
}

fn validate_authored_function_arguments(
    value: &str,
    mut validate: impl for<'i, 't> FnMut(&mut Parser<'i, 't>) -> bool,
) -> bool {
    let mut input = ParserInput::new(value);
    let mut parser = Parser::new(&mut input);
    validate(&mut parser) && parser.is_exhausted()
}

fn collect_authored_tokens<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<String, ParseError<'i, Error>> {
    let mut value = String::new();
    while !input.is_exhausted() {
        let token = input.next().map_err(basic)?;
        let token_css = token.to_css_string();
        if matches!(token, Token::Comma) {
            if value.ends_with(' ') {
                value.pop();
            }
            value.push_str(", ");
        } else {
            if !value.is_empty() && !value.ends_with(' ') {
                value.push(' ');
            }
            value.push_str(&token_css);
        }
    }
    Ok(value.trim().to_owned())
}

fn consume_optional_comma<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    input.try_parse(Parser::expect_comma).is_ok()
}

fn validate_length_sequence<'i, 't>(input: &mut Parser<'i, 't>, min: usize, max: usize) -> bool {
    let mut count = 0;
    while !input.is_exhausted() {
        if count == max
            || parse_length_with(input, LengthOptions::position(), "function length").is_err()
        {
            return false;
        }
        count += 1;
        if !input.is_exhausted() {
            consume_optional_comma(input);
        }
    }
    count >= min
}

fn validate_non_negative_length<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    parse_length_with(input, LengthOptions::border_width(), "function length").is_ok()
        && input.is_exhausted()
}

fn validate_number_sequence<'i, 't>(input: &mut Parser<'i, 't>, min: usize, max: usize) -> bool {
    let mut count = 0;
    while !input.is_exhausted() {
        if count == max || input.expect_number().is_err() {
            return false;
        }
        count += 1;
        if !input.is_exhausted() {
            consume_optional_comma(input);
        }
    }
    count >= min
}

fn validate_number_sequence_prefix<'i, 't>(input: &mut Parser<'i, 't>, count: usize) -> bool {
    for index in 0..count {
        if input.expect_number().is_err() {
            return false;
        }
        if index + 1 < count {
            consume_optional_comma(input);
        }
    }
    true
}

fn validate_number_or_percent<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    let parsed = match input.next() {
        Ok(Token::Number { .. } | Token::Percentage { .. }) => true,
        Ok(_) => false,
        Err(_) => false,
    };
    parsed && input.is_exhausted()
}

fn validate_angle_sequence<'i, 't>(input: &mut Parser<'i, 't>, min: usize, max: usize) -> bool {
    let mut count = 0;
    while !input.is_exhausted() {
        if count == max || !validate_angle(input) {
            return false;
        }
        count += 1;
        if !input.is_exhausted() {
            consume_optional_comma(input);
        }
    }
    count >= min
}

fn validate_angle<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    match input.next() {
        Ok(Token::Dimension { unit, .. }) => {
            unit.eq_ignore_ascii_case("deg")
                || unit.eq_ignore_ascii_case("rad")
                || unit.eq_ignore_ascii_case("grad")
                || unit.eq_ignore_ascii_case("turn")
        }
        Ok(Token::Number { value, .. }) => *value == 0.0,
        _ => false,
    }
}

fn validate_shape_radius<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    if input
        .try_parse(|input| {
            let ident = input.expect_ident_cloned().map_err(basic)?;
            match_ignore_ascii_case! { &ident,
                "closest-side" | "farthest-side" | "closest-corner" | "farthest-corner" => Ok(()),
                _ => Err(unsupported_value(
                    input,
                    None,
                    unsupported_keyword_reason("shape radius", ident.as_ref()),
                )),
            }
        })
        .is_ok()
    {
        true
    } else {
        parse_length_with(input, LengthOptions::background_size(), "shape radius").is_ok()
    }
}

fn validate_circle_shape<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    if input
        .try_parse(|input| input.expect_ident_matching("at"))
        .is_ok()
    {
        return parse_css_position(input).is_ok() && input.is_exhausted();
    }
    if !validate_shape_radius(input) {
        return false;
    }
    if input.is_exhausted() {
        return true;
    }
    input
        .try_parse(|input| input.expect_ident_matching("at"))
        .is_ok()
        && parse_css_position(input).is_ok()
        && input.is_exhausted()
}

fn validate_ellipse_shape<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    if input
        .try_parse(|input| input.expect_ident_matching("at"))
        .is_ok()
    {
        return parse_css_position(input).is_ok() && input.is_exhausted();
    }
    if !validate_shape_radius(input) {
        return false;
    }
    if !input.is_exhausted() && !next_is_ident(input, "at") && !validate_shape_radius(input) {
        return false;
    }
    if input.is_exhausted() {
        return true;
    }
    input
        .try_parse(|input| input.expect_ident_matching("at"))
        .is_ok()
        && parse_css_position(input).is_ok()
        && input.is_exhausted()
}

fn validate_inset_shape<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    let mut count = 0;
    while !input.is_exhausted() && !next_is_ident(input, "round") {
        if count == 4
            || parse_length_with(input, LengthOptions::background_size(), "inset shape").is_err()
        {
            return false;
        }
        count += 1;
    }
    if count == 0 {
        return false;
    }
    if input.is_exhausted() {
        return true;
    }
    input
        .try_parse(|input| input.expect_ident_matching("round"))
        .is_ok()
        && validate_length_sequence(input, 1, 4)
}

fn validate_polygon_shape<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    let mut points = 0;
    loop {
        if parse_length_with(input, LengthOptions::position(), "polygon x").is_err()
            || parse_length_with(input, LengthOptions::position(), "polygon y").is_err()
        {
            return false;
        }
        points += 1;
        if input.is_exhausted() {
            return points >= 1;
        }
        if input.try_parse(Parser::expect_comma).is_err() {
            return false;
        }
        if input.is_exhausted() {
            return false;
        }
    }
}

fn validate_cubic_bezier<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    for index in 0..4 {
        if input.expect_number().is_err() {
            return false;
        }
        if index < 3 && input.expect_comma().is_err() {
            return false;
        }
    }
    input.is_exhausted()
}

fn validate_steps<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    let Ok(Token::Number {
        int_value: Some(value),
        ..
    }) = input.next()
    else {
        return false;
    };
    if *value <= 0 {
        return false;
    }
    if input.is_exhausted() {
        return true;
    }
    if input.expect_comma().is_err() {
        return false;
    }
    let Ok(ident) = input.expect_ident_cloned() else {
        return false;
    };
    let valid = matches!(
        ident.to_ascii_lowercase().as_str(),
        "jump-start" | "jump-end" | "jump-none" | "jump-both" | "start" | "end"
    );
    valid && input.is_exhausted()
}

fn parse_translate<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTranslate, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(CssTranslate::None);
    }
    let mut values = Vec::new();
    while !input.is_exhausted() {
        values.push(parse_length_with(
            input,
            LengthOptions::position(),
            "translate",
        )?);
        if values.len() > 3 {
            return Err(unsupported_value(
                input,
                None,
                "translate has too many values",
            ));
        }
    }
    CssTranslateValues::try_new(values)
        .map(CssTranslate::Values)
        .ok_or_else(|| unsupported_value(input, None, "translate is empty"))
}

fn parse_rotate<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssRotate, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(CssRotate::None);
    }
    let location = input.current_source_location();
    let token = input.next().map_err(basic)?;
    let value = match token {
        Token::Dimension { unit, .. }
            if unit.eq_ignore_ascii_case("deg")
                || unit.eq_ignore_ascii_case("rad")
                || unit.eq_ignore_ascii_case("grad")
                || unit.eq_ignore_ascii_case("turn") =>
        {
            token.to_css_string()
        }
        Token::Number { value, .. } if *value == 0.0 => token.to_css_string(),
        _ => return Err(location.new_unexpected_token_error::<Error>(token.clone())),
    };
    Ok(CssRotate::Value(value))
}

fn parse_scale<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssScale, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(CssScale::None);
    }
    let mut values = Vec::new();
    while !input.is_exhausted() {
        values.push(parse_number(input)?);
        if values.len() > 3 {
            return Err(unsupported_value(input, None, "scale has too many values"));
        }
    }
    CssScaleValues::try_new(values)
        .map(CssScale::Values)
        .ok_or_else(|| unsupported_value(input, None, "scale is empty"))
}

fn parse_filter<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFilter, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(CssFilter::None);
    }
    let mut functions = Vec::new();
    while !input.is_exhausted() {
        functions.push(parse_filter_function(input)?);
    }
    CssFilterFunctionList::try_new(functions)
        .map(CssFilter::Functions)
        .ok_or_else(|| unsupported_value(input, None, "filter function list is empty"))
}

fn parse_filter_function<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssFilterFunction, ParseError<'i, Error>> {
    if let Ok(url) = input.try_parse(parse_url) {
        return Ok(CssFilterFunction::Url(url));
    }
    let location = input.current_source_location();
    let name = match input.next().map_err(basic)? {
        Token::Function(name) => name.clone(),
        token => return Err(location.new_unexpected_token_error::<Error>(token.clone())),
    };
    let arguments =
        input.parse_nested_block(|input| parse_filter_function_arguments(input, name.as_ref()))?;
    match name.to_ascii_lowercase().as_str() {
        "blur" => Ok(CssFilterFunction::Blur(arguments)),
        "brightness" => Ok(CssFilterFunction::Brightness(arguments)),
        "contrast" => Ok(CssFilterFunction::Contrast(arguments)),
        "drop-shadow" => Ok(CssFilterFunction::DropShadow(arguments)),
        "grayscale" => Ok(CssFilterFunction::Grayscale(arguments)),
        "hue-rotate" => Ok(CssFilterFunction::HueRotate(arguments)),
        "invert" => Ok(CssFilterFunction::Invert(arguments)),
        "opacity" => Ok(CssFilterFunction::Opacity(arguments)),
        "saturate" => Ok(CssFilterFunction::Saturate(arguments)),
        "sepia" => Ok(CssFilterFunction::Sepia(arguments)),
        _ => Err(unsupported_value(
            input,
            None,
            format!("unsupported filter function `{name}`"),
        )),
    }
}

fn parse_clip_path<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssClipPath, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(CssClipPath::None);
    }
    if let Ok(url) = input.try_parse(parse_url) {
        return Ok(CssClipPath::Url(url));
    }
    let location = input.current_source_location();
    let name = match input.next().map_err(basic)? {
        Token::Function(name) => name.clone(),
        token => return Err(location.new_unexpected_token_error::<Error>(token.clone())),
    };
    let arguments =
        input.parse_nested_block(|input| parse_basic_shape_arguments(input, name.as_ref()))?;
    match name.to_ascii_lowercase().as_str() {
        "inset" => Ok(CssClipPath::BasicShape(CssBasicShape::Inset(arguments))),
        "circle" => Ok(CssClipPath::BasicShape(CssBasicShape::Circle(arguments))),
        "ellipse" => Ok(CssClipPath::BasicShape(CssBasicShape::Ellipse(arguments))),
        "polygon" => Ok(CssClipPath::BasicShape(CssBasicShape::Polygon(arguments))),
        _ => Err(unsupported_value(
            input,
            None,
            format!("unsupported clip-path function `{name}`"),
        )),
    }
}

fn parse_mask_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMaskList, ParseError<'i, Error>> {
    let mut layers = Vec::new();
    loop {
        layers.push(parse_mask_layer(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "mask list has an empty item",
            ));
        }
    }
    if layers.is_empty() {
        Err(unsupported_value(input, None, "mask list is empty"))
    } else {
        Ok(CssMaskList::new(layers))
    }
}

fn parse_mask_layer<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMaskLayer, ParseError<'i, Error>> {
    let mut image = None;
    let mut position = None;
    let mut size = None;
    let mut repeat = None;

    while !input.is_exhausted() && !next_is_comma(input) {
        if image.is_none()
            && let Ok(parsed_image) = input.try_parse(parse_image_layer)
        {
            image = Some(parsed_image);
            continue;
        }
        if repeat.is_none()
            && let Ok(parsed_repeat) = input.try_parse(parse_background_repeat)
        {
            repeat = Some(parsed_repeat);
            continue;
        }
        if position.is_none()
            && let Ok(parsed_position) = input.try_parse(parse_css_position)
        {
            position = Some(parsed_position);
            if input.try_parse(|input| input.expect_delim('/')).is_ok() {
                size = Some(parse_background_size(input)?);
            }
            continue;
        }
        return Err(unsupported_value(input, None, "unsupported mask component"));
    }
    CssMaskLayer::try_new(image, position, size, repeat)
        .ok_or_else(|| unsupported_value(input, None, "mask layer is empty"))
}

fn parse_time_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTimeList, ParseError<'i, Error>> {
    let mut times = Vec::new();
    loop {
        times.push(parse_time(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "time list has an empty item",
            ));
        }
    }
    CssTimeList::try_new(times).ok_or_else(|| unsupported_value(input, None, "time list is empty"))
}

fn parse_time<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTime, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, unit, .. } if unit.eq_ignore_ascii_case("s") => {
            if *value < 0.0 {
                Err(unsupported_value_at(
                    location,
                    None,
                    "CSS time must be non-negative",
                ))
            } else {
                Ok(CssTime::new(*value, CssTimeUnit::Seconds))
            }
        }
        Token::Dimension { value, unit, .. } if unit.eq_ignore_ascii_case("ms") => {
            if *value < 0.0 {
                Err(unsupported_value_at(
                    location,
                    None,
                    "CSS time must be non-negative",
                ))
            } else {
                Ok(CssTime::new(*value, CssTimeUnit::Milliseconds))
            }
        }
        Token::Dimension { unit, .. } => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported time unit `{unit}`"),
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_easing_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssEasingList, ParseError<'i, Error>> {
    let mut easings = Vec::new();
    loop {
        easings.push(parse_easing(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "easing list has an empty item",
            ));
        }
    }
    CssEasingList::try_new(easings)
        .ok_or_else(|| unsupported_value(input, None, "easing list is empty"))
}

fn parse_easing<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssEasing, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "ease" => Ok(CssEasing::Ease),
            "linear" => Ok(CssEasing::Linear),
            "ease-in" => Ok(CssEasing::EaseIn),
            "ease-out" => Ok(CssEasing::EaseOut),
            "ease-in-out" => Ok(CssEasing::EaseInOut),
            "step-start" => Ok(CssEasing::StepStart),
            "step-end" => Ok(CssEasing::StepEnd),
            _ => Err(unsupported_value(
                input,
                None,
                unsupported_keyword_reason("easing", ident.as_ref()),
            )),
        };
    }
    let location = input.current_source_location();
    let name = match input.next().map_err(basic)? {
        Token::Function(name) => name.clone(),
        token => return Err(location.new_unexpected_token_error::<Error>(token.clone())),
    };
    let arguments =
        input.parse_nested_block(|input| parse_easing_function_arguments(input, name.as_ref()))?;
    match name.to_ascii_lowercase().as_str() {
        "cubic-bezier" => Ok(CssEasing::CubicBezier(arguments)),
        "steps" => Ok(CssEasing::Steps(arguments)),
        _ => Err(unsupported_value(
            input,
            None,
            format!("unsupported easing function `{name}`"),
        )),
    }
}

fn parse_transition_property_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransitionPropertyList, ParseError<'i, Error>> {
    let mut properties = Vec::new();
    loop {
        properties.push(parse_transition_property(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "transition-property list has an empty item",
            ));
        }
    }
    CssTransitionPropertyList::try_new(properties)
        .ok_or_else(|| unsupported_value(input, None, "transition-property list is empty"))
}

fn parse_transition_property<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransitionProperty, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "all" => Ok(CssTransitionProperty::All),
        "none" => Ok(CssTransitionProperty::None),
        _ => parse_custom_ident_from_str_at("transition property", ident.as_ref(), location)
            .map(CssTransitionProperty::Custom),
    }
}

fn parse_transition_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransitionList, ParseError<'i, Error>> {
    let mut items = Vec::new();
    loop {
        items.push(parse_transition_item(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "transition list has an empty item",
            ));
        }
    }
    CssTransitionList::try_new(items)
        .ok_or_else(|| unsupported_value(input, None, "transition list is empty"))
}

fn parse_transition_item<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransition, ParseError<'i, Error>> {
    let mut property = None;
    let mut duration = None;
    let mut delay = None;
    let mut timing_function = None;
    while !input.is_exhausted() && !next_is_comma(input) {
        if let Ok(time) = input.try_parse(parse_time) {
            if duration.is_none() {
                duration = Some(time);
            } else if delay.is_none() {
                delay = Some(time);
            } else {
                return Err(unsupported_value(input, None, "duplicate transition time"));
            }
            continue;
        }
        if timing_function.is_none()
            && let Ok(easing) = input.try_parse(parse_easing)
        {
            timing_function = Some(easing);
            continue;
        }
        if property.is_none()
            && let Ok(parsed_property) = input.try_parse(parse_transition_property)
        {
            property = Some(parsed_property);
            continue;
        }
        return Err(unsupported_value(
            input,
            None,
            "unsupported transition component",
        ));
    }
    CssTransition::try_new(property, duration, delay, timing_function)
        .ok_or_else(|| unsupported_value(input, None, "transition item is empty"))
}

fn parse_animation_name_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationNameList, ParseError<'i, Error>> {
    let mut names = Vec::new();
    loop {
        names.push(parse_animation_name(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "animation-name list has an empty item",
            ));
        }
    }
    CssAnimationNameList::try_new(names)
        .ok_or_else(|| unsupported_value(input, None, "animation-name list is empty"))
}

fn parse_animation_name<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationName, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    if ident.eq_ignore_ascii_case("none") {
        Ok(CssAnimationName::None)
    } else {
        parse_custom_ident_from_str_at("animation name", ident.as_ref(), location)
            .map(CssAnimationName::Custom)
    }
}

fn parse_animation_iteration_count_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationIterationCountList, ParseError<'i, Error>> {
    let mut counts = Vec::new();
    loop {
        counts.push(parse_animation_iteration_count(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "animation-iteration-count list has an empty item",
            ));
        }
    }
    CssAnimationIterationCountList::try_new(counts)
        .ok_or_else(|| unsupported_value(input, None, "animation-iteration-count list is empty"))
}

fn parse_animation_iteration_count<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationIterationCount, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("infinite"))
        .is_ok()
    {
        return Ok(CssAnimationIterationCount::Infinite);
    }
    let location = input.current_source_location();
    let value = input.expect_number().map_err(basic)?;
    if value < 0.0 {
        Err(unsupported_value_at(
            location,
            None,
            "animation iteration count must be non-negative",
        ))
    } else {
        Ok(CssAnimationIterationCount::number(value))
    }
}

fn parse_animation_direction_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationDirectionList, ParseError<'i, Error>> {
    let mut directions = Vec::new();
    loop {
        directions.push(parse_animation_direction(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "animation-direction list has an empty item",
            ));
        }
    }
    CssAnimationDirectionList::try_new(directions)
        .ok_or_else(|| unsupported_value(input, None, "animation-direction list is empty"))
}

fn parse_animation_direction<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationDirection, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "normal" => Ok(CssAnimationDirection::Normal),
        "reverse" => Ok(CssAnimationDirection::Reverse),
        "alternate" => Ok(CssAnimationDirection::Alternate),
        "alternate-reverse" => Ok(CssAnimationDirection::AlternateReverse),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("animation-direction", ident.as_ref()),
        )),
    }
}

fn parse_animation_fill_mode_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationFillModeList, ParseError<'i, Error>> {
    let mut modes = Vec::new();
    loop {
        modes.push(parse_animation_fill_mode(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "animation-fill-mode list has an empty item",
            ));
        }
    }
    CssAnimationFillModeList::try_new(modes)
        .ok_or_else(|| unsupported_value(input, None, "animation-fill-mode list is empty"))
}

fn parse_animation_fill_mode<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationFillMode, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "none" => Ok(CssAnimationFillMode::None),
        "forwards" => Ok(CssAnimationFillMode::Forwards),
        "backwards" => Ok(CssAnimationFillMode::Backwards),
        "both" => Ok(CssAnimationFillMode::Both),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("animation-fill-mode", ident.as_ref()),
        )),
    }
}

fn parse_animation_play_state_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationPlayStateList, ParseError<'i, Error>> {
    let mut states = Vec::new();
    loop {
        states.push(parse_animation_play_state(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "animation-play-state list has an empty item",
            ));
        }
    }
    CssAnimationPlayStateList::try_new(states)
        .ok_or_else(|| unsupported_value(input, None, "animation-play-state list is empty"))
}

fn parse_animation_play_state<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationPlayState, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "running" => Ok(CssAnimationPlayState::Running),
        "paused" => Ok(CssAnimationPlayState::Paused),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("animation-play-state", ident.as_ref()),
        )),
    }
}

fn parse_animation_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationList, ParseError<'i, Error>> {
    let mut items = Vec::new();
    loop {
        items.push(parse_animation_item(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "animation list has an empty item",
            ));
        }
    }
    CssAnimationList::try_new(items)
        .ok_or_else(|| unsupported_value(input, None, "animation list is empty"))
}

fn parse_animation_item<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimation, ParseError<'i, Error>> {
    let mut name = None;
    let mut duration = None;
    let mut delay = None;
    let mut timing_function = None;
    let mut iteration_count = None;
    let mut direction = None;
    let mut fill_mode = None;
    let mut play_state = None;

    while !input.is_exhausted() && !next_is_comma(input) {
        if let Ok(time) = input.try_parse(parse_time) {
            if duration.is_none() {
                duration = Some(time);
            } else if delay.is_none() {
                delay = Some(time);
            } else {
                return Err(unsupported_value(input, None, "duplicate animation time"));
            }
            continue;
        }
        if timing_function.is_none()
            && let Ok(easing) = input.try_parse(parse_easing)
        {
            timing_function = Some(easing);
            continue;
        }
        if iteration_count.is_none()
            && let Ok(count) = input.try_parse(parse_animation_iteration_count)
        {
            iteration_count = Some(count);
            continue;
        }
        if direction.is_none()
            && let Ok(parsed_direction) = input.try_parse(parse_animation_direction)
        {
            direction = Some(parsed_direction);
            continue;
        }
        if fill_mode.is_none()
            && let Ok(parsed_fill_mode) = input.try_parse(parse_animation_fill_mode)
        {
            fill_mode = Some(parsed_fill_mode);
            continue;
        }
        if play_state.is_none()
            && let Ok(parsed_play_state) = input.try_parse(parse_animation_play_state)
        {
            play_state = Some(parsed_play_state);
            continue;
        }
        if name.is_none()
            && let Ok(parsed_name) = input.try_parse(parse_animation_name)
        {
            name = Some(parsed_name);
            continue;
        }
        return Err(unsupported_value(
            input,
            None,
            "unsupported animation component",
        ));
    }

    CssAnimation::try_new(CssAnimationComponents {
        name,
        duration,
        delay,
        timing_function,
        iteration_count,
        direction,
        fill_mode,
        play_state,
    })
    .ok_or_else(|| unsupported_value(input, None, "animation item is empty"))
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

    const fn text_indent() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    const fn vertical_align() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    const fn letter_spacing() -> Self {
        Self {
            percent: false,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: false,
            non_negative: false,
        }
    }

    const fn text_decoration_thickness() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: true,
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

    const fn position() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    const fn background_size() -> Self {
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

fn next_is_comma<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    let state = input.state();
    let is_comma = input.try_parse(Parser::expect_comma).is_ok();
    input.reset(&state);
    is_comma
}

fn next_is_ident<'i, 't>(input: &mut Parser<'i, 't>, expected: &str) -> bool {
    let state = input.state();
    let is_ident = input
        .try_parse(|input| input.expect_ident_matching(expected))
        .is_ok();
    input.reset(&state);
    is_ident
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
        "all" => CssProperty::All,
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
        "writing-mode" => CssProperty::WritingMode,
        "text-align" => CssProperty::TextAlign,
        "text-align-last" => CssProperty::TextAlignLast,
        "text-indent" => CssProperty::TextIndent,
        "vertical-align" => CssProperty::VerticalAlign,
        "font-family" => CssProperty::FontFamily,
        "font" => CssProperty::Font,
        "font-weight" => CssProperty::FontWeight,
        "font-style" => CssProperty::FontStyle,
        "font-stretch" => CssProperty::FontStretch,
        "font-variant" => CssProperty::FontVariant,
        "font-feature-settings" => CssProperty::FontFeatureSettings,
        "letter-spacing" => CssProperty::LetterSpacing,
        "text-wrap" => CssProperty::TextWrap,
        "white-space" => CssProperty::WhiteSpace,
        "word-break" => CssProperty::WordBreak,
        "overflow-wrap" => CssProperty::OverflowWrap,
        "text-overflow" => CssProperty::TextOverflow,
        "text-decoration" => CssProperty::TextDecoration,
        "text-decoration-line" => CssProperty::TextDecorationLine,
        "text-decoration-color" => CssProperty::TextDecorationColor,
        "text-decoration-style" => CssProperty::TextDecorationStyle,
        "text-decoration-thickness" => CssProperty::TextDecorationThickness,
        "text-transform" => CssProperty::TextTransform,
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
        "background-image" => CssProperty::BackgroundImage,
        "background-position" => CssProperty::BackgroundPosition,
        "background-size" => CssProperty::BackgroundSize,
        "background-repeat" => CssProperty::BackgroundRepeat,
        "background-origin" => CssProperty::BackgroundOrigin,
        "background-clip" => CssProperty::BackgroundClip,
        "background-attachment" => CssProperty::BackgroundAttachment,
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
        "cursor" => CssProperty::Cursor,
        "pointer-events" => CssProperty::PointerEvents,
        "user-select" => CssProperty::UserSelect,
        "outline" => CssProperty::Outline,
        "outline-color" => CssProperty::OutlineColor,
        "outline-style" => CssProperty::OutlineStyle,
        "outline-width" => CssProperty::OutlineWidth,
        "transform" => CssProperty::Transform,
        "transform-origin" => CssProperty::TransformOrigin,
        "translate" => CssProperty::Translate,
        "rotate" => CssProperty::Rotate,
        "scale" => CssProperty::Scale,
        "filter" => CssProperty::Filter,
        "backdrop-filter" => CssProperty::BackdropFilter,
        "clip-path" => CssProperty::ClipPath,
        "mask" => CssProperty::Mask,
        "mask-image" => CssProperty::MaskImage,
        "mask-size" => CssProperty::MaskSize,
        "mask-position" => CssProperty::MaskPosition,
        "mask-repeat" => CssProperty::MaskRepeat,
        "transition-property" => CssProperty::TransitionProperty,
        "transition-duration" => CssProperty::TransitionDuration,
        "transition-delay" => CssProperty::TransitionDelay,
        "transition-timing-function" => CssProperty::TransitionTimingFunction,
        "transition" => CssProperty::Transition,
        "animation-name" => CssProperty::AnimationName,
        "animation-duration" => CssProperty::AnimationDuration,
        "animation-delay" => CssProperty::AnimationDelay,
        "animation-timing-function" => CssProperty::AnimationTimingFunction,
        "animation-iteration-count" => CssProperty::AnimationIterationCount,
        "animation-direction" => CssProperty::AnimationDirection,
        "animation-fill-mode" => CssProperty::AnimationFillMode,
        "animation-play-state" => CssProperty::AnimationPlayState,
        "animation" => CssProperty::Animation,
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
    use crate::test_support::{
        AcceptedDeclarationCase, ExpectedErrorKind, RejectedDeclarationCase,
        accepted_declaration_cases, assert_accepts_declarations, assert_rejects_declarations,
        assert_sheet_rejected, parse_single_declaration_value,
    };

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
    fn strict_declaration_case_helpers_accept_and_reject_cases() {
        assert_accepts_declarations(&accepted_declaration_cases()[..3]);
        assert_eq!(
            parse_single_declaration_value("display", "inherit"),
            CssValue::GlobalKeyword(CssGlobalKeyword::Inherit)
        );

        assert_rejects_declarations(&[
            RejectedDeclarationCase {
                label: "unsupported display keyword",
                property_name: "display",
                authored_value: "inline",
                expected_error: ExpectedErrorKind::UnsupportedValue {
                    property: Some("display"),
                    reason: "unsupported display keyword `inline`",
                },
                property_name_should_be_recognized: true,
            },
            RejectedDeclarationCase {
                label: "unknown property name",
                property_name: "widht",
                authored_value: "10px",
                expected_error: ExpectedErrorKind::UnknownProperty { name: "widht" },
                property_name_should_be_recognized: false,
            },
        ]);

        let accepted = AcceptedDeclarationCase::global_inherit("width", CssProperty::Width);
        accepted.assert_accepts();
    }

    #[test]
    fn strict_whole_sheet_rejection_helper_rejects_mixed_declarations() {
        assert_sheet_rejected(
            ".panel { width: 10px; display: inline; }",
            &ExpectedErrorKind::UnsupportedValue {
                property: Some("display"),
                reason: "unsupported display keyword `inline`",
            },
        );
        assert_sheet_rejected(
            ".panel { width: inherit 10px; height: 20px; }",
            &ExpectedErrorKind::InvalidSyntax,
        );
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
    fn parses_all_property_global_keywords_as_authored_syntax() {
        let cases = [
            ("inherit", CssGlobalKeyword::Inherit),
            ("initial", CssGlobalKeyword::Initial),
            ("unset", CssGlobalKeyword::Unset),
            ("revert", CssGlobalKeyword::Revert),
            ("revert-layer", CssGlobalKeyword::RevertLayer),
        ];

        for (authored, expected) in cases {
            assert_eq!(
                declaration_value(&format!(".panel {{ all: {authored}; }}"), CssProperty::All,),
                CssValue::GlobalKeyword(expected)
            );
        }
    }

    #[test]
    fn rejects_non_global_all_values_with_typed_unsupported_value() {
        for input in [".panel { all: block; }", ".panel { all: 1px; }"] {
            let error = parse_sheet(input).expect_err(input);

            assert_eq!(
                error.kind(),
                &ErrorKind::UnsupportedValue {
                    property: Some("all".to_owned()),
                    reason: "`all` only accepts CSS-wide global keywords".to_owned(),
                }
            );
        }
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
    fn parses_typography_and_text_keyword_families() {
        assert_eq!(
            declaration_value(
                ".panel { writing-mode: vertical-rl; }",
                CssProperty::WritingMode,
            ),
            CssValue::WritingMode(CssWritingMode::VerticalRl)
        );
        assert_eq!(
            declaration_value(".panel { text-align: start; }", CssProperty::TextAlign),
            CssValue::TextAlign(CssTextAlign::Start)
        );
        assert_eq!(
            declaration_value(
                ".panel { text-align-last: justify; }",
                CssProperty::TextAlignLast,
            ),
            CssValue::TextAlignLast(CssTextAlignLast::Justify)
        );
        assert_eq!(
            declaration_value(".panel { text-wrap: balance; }", CssProperty::TextWrap),
            CssValue::TextWrap(CssTextWrap::Balance)
        );
        assert_eq!(
            declaration_value(".panel { white-space: pre-wrap; }", CssProperty::WhiteSpace),
            CssValue::WhiteSpace(CssWhiteSpace::PreWrap)
        );
        assert_eq!(
            declaration_value(".panel { word-break: keep-all; }", CssProperty::WordBreak),
            CssValue::WordBreak(CssWordBreak::KeepAll)
        );
        assert_eq!(
            declaration_value(
                ".panel { overflow-wrap: anywhere; }",
                CssProperty::OverflowWrap,
            ),
            CssValue::OverflowWrap(CssOverflowWrap::Anywhere)
        );
        assert_eq!(
            declaration_value(
                ".panel { text-overflow: ellipsis; }",
                CssProperty::TextOverflow
            ),
            CssValue::TextOverflow(CssTextOverflow::Ellipsis)
        );
        assert_eq!(
            declaration_value(
                ".panel { text-transform: uppercase; }",
                CssProperty::TextTransform
            ),
            CssValue::TextTransform(CssTextTransform::Uppercase)
        );
    }

    #[test]
    fn parses_typography_and_text_length_families() {
        assert_eq!(
            declaration_value(".panel { text-indent: 2em; }", CssProperty::TextIndent),
            CssValue::TextIndent(CssTextIndent::new(
                CssLength::dimension(2.0, CssLengthUnit::Em),
                false,
                false,
            ))
        );
        assert_eq!(
            declaration_value(
                ".panel { vertical-align: 4px; }",
                CssProperty::VerticalAlign
            ),
            CssValue::VerticalAlign(CssVerticalAlign::Length(CssVerticalAlignLength::new(
                CssLength::px(4.0)
            )))
        );
        assert_eq!(
            declaration_value(
                ".panel { letter-spacing: normal; }",
                CssProperty::LetterSpacing
            ),
            CssValue::LetterSpacing(CssLetterSpacing::Normal)
        );
        assert_eq!(
            declaration_value(
                ".panel { letter-spacing: 0.1em; }",
                CssProperty::LetterSpacing
            ),
            CssValue::LetterSpacing(CssLetterSpacing::Length(CssLetterSpacingLength::new(
                CssLength::dimension(0.1, CssLengthUnit::Em)
            )))
        );
        assert_eq!(
            declaration_value(
                ".panel { text-decoration-thickness: from-font; }",
                CssProperty::TextDecorationThickness,
            ),
            CssValue::TextDecorationThickness(CssTextDecorationThickness::FromFont)
        );
        assert_eq!(
            declaration_value(
                ".panel { text-decoration-thickness: 2px; }",
                CssProperty::TextDecorationThickness,
            ),
            CssValue::TextDecorationThickness(CssTextDecorationThickness::Length(
                CssTextDecorationThicknessLength::new(CssLength::px(2.0))
            ))
        );
    }

    #[test]
    fn parses_font_families_and_font_shorthand_as_authored_syntax() {
        let family = declaration_value(
            ".panel { font-family: \"Avenir Next\", Gill Sans, sans-serif; }",
            CssProperty::FontFamily,
        );
        let CssValue::FontFamily(family) = family else {
            panic!("expected font family list");
        };
        assert_eq!(
            family.families(),
            [
                CssFontFamilyName::try_quoted("Avenir Next").unwrap(),
                CssFontFamilyName::try_ident_sequence("Gill Sans").unwrap(),
                CssFontFamilyName::try_ident_sequence("sans-serif").unwrap(),
            ]
        );

        assert_eq!(
            declaration_value(".panel { font-weight: 725; }", CssProperty::FontWeight),
            CssValue::FontWeight(CssFontWeight::Number(CssFontWeightNumber::new(725)))
        );
        assert_eq!(
            declaration_value(".panel { font-style: italic; }", CssProperty::FontStyle),
            CssValue::FontStyle(CssFontStyle::Italic)
        );
        assert_eq!(
            declaration_value(
                ".panel { font-stretch: semi-condensed; }",
                CssProperty::FontStretch,
            ),
            CssValue::FontStretch(CssFontStretch::SemiCondensed)
        );
        assert_eq!(
            declaration_value(
                ".panel { font-variant: small-caps; }",
                CssProperty::FontVariant
            ),
            CssValue::FontVariant(CssFontVariant::SmallCaps)
        );
        assert_eq!(
            declaration_value(
                ".panel { font-feature-settings: \"kern\" on, \"liga\" 0; }",
                CssProperty::FontFeatureSettings,
            ),
            CssValue::FontFeatureSettings(CssFontFeatureSettings::Features(
                CssFontFeatureList::new(vec![
                    CssFontFeature::new("kern", Some(CssFontFeatureValue::On)),
                    CssFontFeature::new("liga", Some(CssFontFeatureValue::Integer(0))),
                ])
            ))
        );

        let shorthand = declaration_value(
            ".panel { font: italic small-caps 700 condensed 16px/normal \"Avenir Next\", sans-serif; }",
            CssProperty::Font,
        );
        let CssValue::Font(font) = shorthand else {
            panic!("expected font shorthand");
        };
        assert_eq!(font.style(), Some(CssFontStyle::Italic));
        assert_eq!(font.variant(), Some(CssFontVariant::SmallCaps));
        assert_eq!(
            font.weight(),
            Some(CssFontWeight::Number(CssFontWeightNumber::new(700)))
        );
        assert_eq!(font.stretch(), Some(CssFontStretch::Condensed));
        assert_eq!(font.size(), &CssLength::px(16.0));
        assert_eq!(font.line_height(), Some(&CssLength::Normal));
        assert_eq!(
            font.families().families(),
            [
                CssFontFamilyName::try_quoted("Avenir Next").unwrap(),
                CssFontFamilyName::try_ident_sequence("sans-serif").unwrap(),
            ]
        );
    }

    #[test]
    fn parses_text_decoration_family() {
        assert_eq!(
            declaration_value(
                ".panel { text-decoration-line: underline overline; }",
                CssProperty::TextDecorationLine,
            ),
            CssValue::TextDecorationLine(CssTextDecorationLine::new(vec![
                CssTextDecorationLineComponent::Underline,
                CssTextDecorationLineComponent::Overline,
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { text-decoration-color: black; }",
                CssProperty::TextDecorationColor,
            ),
            CssValue::TextDecorationColor(CssColor::BLACK)
        );
        assert_eq!(
            declaration_value(
                ".panel { text-decoration-style: wavy; }",
                CssProperty::TextDecorationStyle,
            ),
            CssValue::TextDecorationStyle(CssTextDecorationStyle::Wavy)
        );

        let value = declaration_value(
            ".panel { text-decoration: underline dotted white 3px; }",
            CssProperty::TextDecoration,
        );
        assert_eq!(
            value,
            CssValue::TextDecoration(CssTextDecoration::new(
                Some(CssTextDecorationLine::new(vec![
                    CssTextDecorationLineComponent::Underline
                ])),
                Some(CssColor::WHITE),
                Some(CssTextDecorationStyle::Dotted),
                Some(CssTextDecorationThickness::Length(
                    CssTextDecorationThicknessLength::new(CssLength::px(3.0))
                )),
            ))
        );
    }

    #[test]
    fn checked_typography_constructors_reject_invalid_states() {
        assert_eq!(CssFontFamilyList::try_new(Vec::new()), None);
        assert_eq!(CssFontWeightNumber::try_new(0), None);
        assert_eq!(CssFontWeightNumber::try_new(1001), None);
        assert_eq!(
            CssFontWeightNumber::try_new(500),
            Some(CssFontWeightNumber::new(500))
        );
        assert_eq!(CssFontFeatureList::try_new(Vec::new()), None);
        assert_eq!(CssTextDecorationLine::try_new(Vec::new()), None);
        assert!(
            CssFont::try_new(
                None,
                None,
                None,
                None,
                CssLength::px(12.0),
                None,
                CssFontFamilyList::new(vec![CssFontFamilyName::ident_sequence("sans-serif")]),
            )
            .is_some(),
        );
        assert_eq!(
            CssFont::try_new(
                None,
                None,
                None,
                None,
                CssLength::Auto,
                None,
                CssFontFamilyList::new(vec![CssFontFamilyName::ident_sequence("sans-serif")]),
            ),
            None
        );
        assert_eq!(CssFontFamilyName::try_quoted(""), None);
        assert_eq!(CssFontFamilyName::try_ident_sequence(""), None);
        assert_eq!(
            CssFontFamilyList::try_new(vec![CssFontFamilyName::ident_sequence("")]),
            None
        );
        assert_eq!(CssFontFeature::try_new("abc", None), None);
        assert_eq!(CssFontFeature::try_new("abcde", None), None);
        assert_eq!(
            CssFontFeature::try_new("kern", Some(CssFontFeatureValue::On)),
            Some(CssFontFeature::new("kern", Some(CssFontFeatureValue::On)))
        );
        assert_eq!(CssVerticalAlignLength::try_new(CssLength::Auto), None);
        assert_eq!(
            CssLetterSpacingLength::try_new(CssLength::percent(10.0)),
            None
        );
        assert_eq!(
            CssTextDecorationThicknessLength::try_new(CssLength::px(-1.0)),
            None
        );
        assert_eq!(
            CssTextDecorationLine::try_new(vec![
                CssTextDecorationLineComponent::Underline,
                CssTextDecorationLineComponent::Underline,
            ]),
            None
        );
    }

    #[test]
    fn parses_every_task_5_supported_property_name() {
        let sheet = parse_sheet(
            ".panel {
                writing-mode: horizontal-tb;
                text-align: center;
                text-align-last: auto;
                text-indent: 1rem hanging each-line;
                vertical-align: super;
                font-family: \"Avenir Next\", sans-serif;
                font: italic 700 16px/normal \"Avenir Next\", sans-serif;
                font-weight: bold;
                font-style: oblique;
                font-stretch: expanded;
                font-variant: normal;
                font-feature-settings: normal;
                letter-spacing: 1px;
                text-wrap: wrap;
                white-space: nowrap;
                word-break: break-word;
                overflow-wrap: break-word;
                text-overflow: clip;
                text-decoration: underline solid black 1px;
                text-decoration-line: none;
                text-decoration-color: transparent;
                text-decoration-style: solid;
                text-decoration-thickness: auto;
                text-transform: capitalize;
            }",
        )
        .unwrap();
        let declarations = sheet.rules()[0].declarations();

        for property in [
            CssProperty::WritingMode,
            CssProperty::TextAlign,
            CssProperty::TextAlignLast,
            CssProperty::TextIndent,
            CssProperty::VerticalAlign,
            CssProperty::FontFamily,
            CssProperty::Font,
            CssProperty::FontWeight,
            CssProperty::FontStyle,
            CssProperty::FontStretch,
            CssProperty::FontVariant,
            CssProperty::FontFeatureSettings,
            CssProperty::LetterSpacing,
            CssProperty::TextWrap,
            CssProperty::WhiteSpace,
            CssProperty::WordBreak,
            CssProperty::OverflowWrap,
            CssProperty::TextOverflow,
            CssProperty::TextDecoration,
            CssProperty::TextDecorationLine,
            CssProperty::TextDecorationColor,
            CssProperty::TextDecorationStyle,
            CssProperty::TextDecorationThickness,
            CssProperty::TextTransform,
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
    fn rejects_task_5_cross_family_leakage_values() {
        for input in [
            ".panel { font-size: auto; }",
            ".panel { font-weight: 1001; }",
            ".panel { font-style: bold; }",
            ".panel { font-family:; }",
            ".panel { letter-spacing: auto; }",
            ".panel { text-decoration-style: 2px; }",
            ".panel { text-transform: wrap; }",
            ".panel { font-feature-settings: \"abc\" on; }",
            ".panel { font-feature-settings: \"abcde\" on; }",
        ] {
            let error = parse_sheet(input).expect_err(input);
            assert!(matches!(
                error.kind(),
                ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
            ));
        }
    }

    #[test]
    fn parses_background_properties_as_authored_syntax() {
        assert_eq!(
            declaration_value(
                ".panel { background-image: url(\"hero.png\"), none; }",
                CssProperty::BackgroundImage,
            ),
            CssValue::BackgroundImage(CssImageLayerList::new(vec![
                CssImageLayer::Url(CssUrl::new("hero.png")),
                CssImageLayer::None,
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { background-position: left 10px top 20%; }",
                CssProperty::BackgroundPosition,
            ),
            CssValue::BackgroundPosition(CssPositionList::new(vec![CssPosition::new(vec![
                CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Left),
                CssPositionComponent::Length(CssLength::px(10.0)),
                CssPositionComponent::Vertical(CssVerticalPositionKeyword::Top),
                CssPositionComponent::Length(CssLength::percent(20.0)),
            ])]))
        );
        assert_eq!(
            declaration_value(
                ".panel { background-size: cover, 10px auto; }",
                CssProperty::BackgroundSize,
            ),
            CssValue::BackgroundSize(CssBackgroundSizeList::new(vec![
                CssBackgroundSize::Cover,
                CssBackgroundSize::Explicit {
                    width: CssBackgroundSizeComponent::Length(CssLength::px(10.0)),
                    height: Some(CssBackgroundSizeComponent::Auto),
                },
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { background-repeat: repeat-x, no-repeat round; }",
                CssProperty::BackgroundRepeat,
            ),
            CssValue::BackgroundRepeat(CssBackgroundRepeatList::new(vec![
                CssBackgroundRepeat::RepeatX,
                CssBackgroundRepeat::Axes {
                    x: CssBackgroundRepeatStyle::NoRepeat,
                    y: CssBackgroundRepeatStyle::Round,
                },
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { background-origin: content-box; }",
                CssProperty::BackgroundOrigin,
            ),
            CssValue::BackgroundBox(CssBackgroundBox::ContentBox)
        );
        assert_eq!(
            declaration_value(
                ".panel { background-clip: padding-box; }",
                CssProperty::BackgroundClip,
            ),
            CssValue::BackgroundBox(CssBackgroundBox::PaddingBox)
        );
        assert_eq!(
            declaration_value(
                ".panel { background-attachment: fixed, local; }",
                CssProperty::BackgroundAttachment,
            ),
            CssValue::BackgroundAttachment(CssBackgroundAttachmentList::new(vec![
                CssBackgroundAttachment::Fixed,
                CssBackgroundAttachment::Local,
            ]))
        );
    }

    #[test]
    fn parses_interaction_and_outline_properties_as_authored_syntax() {
        assert_eq!(
            declaration_value(".panel { cursor: grab; }", CssProperty::Cursor),
            CssValue::Cursor(CssCursor::Keyword(CssCursorKeyword::Grab))
        );
        assert_eq!(
            declaration_value(
                ".panel { pointer-events: none; }",
                CssProperty::PointerEvents
            ),
            CssValue::PointerEvents(CssPointerEvents::None)
        );
        assert_eq!(
            declaration_value(".panel { user-select: text; }", CssProperty::UserSelect),
            CssValue::UserSelect(CssUserSelect::Text)
        );
        assert_eq!(
            declaration_value(
                ".panel { outline: thick dotted white; }",
                CssProperty::Outline,
            ),
            CssValue::Outline(CssOutline::new(
                Some(CssOutlineWidth::Thick),
                Some(CssOutlineStyle::Border(CssBorderStyle::Dotted)),
                Some(CssColor::WHITE),
            ))
        );
        assert_eq!(
            declaration_value(".panel { outline-width: 2px; }", CssProperty::OutlineWidth),
            CssValue::OutlineWidth(CssOutlineWidth::Length(CssLength::px(2.0)))
        );
    }

    #[test]
    fn parses_transform_effect_and_mask_properties_as_authored_syntax() {
        let transform = declaration_value(
            ".panel { transform: translate(10px, 20px) rotate(45deg) scale(1.5); }",
            CssProperty::Transform,
        );
        let CssValue::Transform(CssTransform::Functions(functions)) = transform else {
            panic!("expected transform functions");
        };
        assert_eq!(functions.functions().len(), 3);
        assert_eq!(
            functions.functions()[0].kind(),
            CssTransformFunctionKind::Translate
        );

        assert_eq!(
            declaration_value(
                ".panel { transform-origin: center top; }",
                CssProperty::TransformOrigin
            ),
            CssValue::TransformOrigin(CssPosition::new(vec![
                CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center),
                CssPositionComponent::Vertical(CssVerticalPositionKeyword::Top),
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { filter: blur(4px) opacity(50%); }",
                CssProperty::Filter
            ),
            CssValue::Filter(CssFilter::Functions(CssFilterFunctionList::new(vec![
                CssFilterFunction::Blur(CssFunctionArguments::new("4px")),
                CssFilterFunction::Opacity(CssFunctionArguments::new("50%")),
            ])))
        );
        assert_eq!(
            declaration_value(
                ".panel { backdrop-filter: none; }",
                CssProperty::BackdropFilter
            ),
            CssValue::Filter(CssFilter::None)
        );
        assert_eq!(
            declaration_value(
                ".panel { clip-path: circle(50% at center); }",
                CssProperty::ClipPath
            ),
            CssValue::ClipPath(CssClipPath::BasicShape(CssBasicShape::Circle(
                CssFunctionArguments::new("50% at center"),
            )))
        );
        assert_eq!(
            declaration_value(
                ".panel { mask-image: url(mask.png), none; }",
                CssProperty::MaskImage,
            ),
            CssValue::MaskImage(CssImageLayerList::new(vec![
                CssImageLayer::Url(CssUrl::new("mask.png")),
                CssImageLayer::None,
            ]))
        );
        let CssValue::Mask(mask_layers) = declaration_value(
            ".panel { mask: url(mask.png) center / contain no-repeat; }",
            CssProperty::Mask,
        ) else {
            panic!("expected mask shorthand");
        };
        assert_eq!(mask_layers.layers().len(), 1);
    }

    #[test]
    fn parses_transition_properties_and_preserves_comma_lists() {
        assert_eq!(
            declaration_value(
                ".panel { transition-property: opacity, transform; }",
                CssProperty::TransitionProperty,
            ),
            CssValue::TransitionProperty(CssTransitionPropertyList::new(vec![
                CssTransitionProperty::Custom(CssCustomIdent::new("opacity")),
                CssTransitionProperty::Custom(CssCustomIdent::new("transform")),
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { transition-duration: 150ms, 2s; }",
                CssProperty::TransitionDuration,
            ),
            CssValue::TimeList(CssTimeList::new(vec![
                CssTime::try_milliseconds(150.0).unwrap(),
                CssTime::try_seconds(2.0).unwrap(),
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { transition-timing-function: ease-in, cubic-bezier(0.1, 0.2, 0.3, 1); }",
                CssProperty::TransitionTimingFunction,
            ),
            CssValue::EasingList(CssEasingList::new(vec![
                CssEasing::EaseIn,
                CssEasing::CubicBezier(CssFunctionArguments::new("0.1, 0.2, 0.3, 1")),
            ]))
        );

        let CssValue::Transition(transitions) = declaration_value(
            ".panel { transition: opacity 150ms ease-in 20ms, transform 2s linear; }",
            CssProperty::Transition,
        ) else {
            panic!("expected transition list");
        };
        assert_eq!(transitions.items().len(), 2);
    }

    #[test]
    fn parses_animation_properties_and_preserves_comma_lists() {
        assert_eq!(
            declaration_value(
                ".panel { animation-name: fade, none; }",
                CssProperty::AnimationName,
            ),
            CssValue::AnimationName(CssAnimationNameList::new(vec![
                CssAnimationName::Custom(CssCustomIdent::new("fade")),
                CssAnimationName::None,
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { animation-iteration-count: 2, infinite; }",
                CssProperty::AnimationIterationCount,
            ),
            CssValue::AnimationIterationCount(CssAnimationIterationCountList::new(vec![
                CssAnimationIterationCount::Number(CssAnimationIterationNumber::new(2.0)),
                CssAnimationIterationCount::Infinite,
            ]))
        );
        assert_eq!(
            declaration_value(
                ".panel { animation-play-state: running, paused; }",
                CssProperty::AnimationPlayState,
            ),
            CssValue::AnimationPlayState(CssAnimationPlayStateList::new(vec![
                CssAnimationPlayState::Running,
                CssAnimationPlayState::Paused,
            ]))
        );

        let CssValue::Animation(animations) = declaration_value(
            ".panel { animation: fade 1s ease-in 200ms 3 alternate both running, slide 2s linear; }",
            CssProperty::Animation,
        ) else {
            panic!("expected animation list");
        };
        assert_eq!(animations.items().len(), 2);
    }

    #[test]
    fn checked_task_6_constructors_reject_invalid_invariants() {
        assert_eq!(CssImageLayerList::try_new(Vec::new()), None);
        assert_eq!(CssCursorUrlList::try_new(Vec::new()), None);
        assert!(CssCursor::try_urls(Vec::new(), CssCursorKeyword::Pointer).is_none());
        assert_eq!(
            CssPosition::try_new(vec![
                CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Left),
                CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Right),
            ]),
            None
        );
        assert_eq!(
            CssPosition::try_new(vec![
                CssPositionComponent::Vertical(CssVerticalPositionKeyword::Top),
                CssPositionComponent::Vertical(CssVerticalPositionKeyword::Bottom),
            ]),
            None
        );
        assert_eq!(CssTranslateValues::try_new(Vec::new()), None);
        assert_eq!(
            CssTranslateValues::try_new(vec![
                CssLength::px(1.0),
                CssLength::px(2.0),
                CssLength::px(3.0),
                CssLength::px(4.0),
            ]),
            None
        );
        assert_eq!(CssScaleValues::try_new(Vec::new()), None);
        assert_eq!(CssScaleValues::try_new(vec![1.0, 2.0, 3.0, 4.0]), None);
        assert_eq!(CssMaskList::try_new(Vec::new()), None);
        assert_eq!(CssTransitionList::try_new(Vec::new()), None);
        assert_eq!(CssTransition::try_new(None, None, None, None), None);
        assert_eq!(CssAnimationList::try_new(Vec::new()), None);
        assert_eq!(
            CssAnimation::try_new(CssAnimationComponents::default()),
            None
        );
        assert_eq!(CssTime::try_seconds(-1.0), None);
        assert_eq!(CssAnimationIterationCount::try_number(-1.0), None);
        assert_eq!(CssOutline::try_new(None, None, None), None);
    }

    #[test]
    fn parses_every_task_6_supported_property_name() {
        let sheet = parse_sheet(
            ".panel {
                background-image: none;
                background-position: center;
                background-size: contain;
                background-repeat: no-repeat;
                background-origin: border-box;
                background-clip: content-box;
                background-attachment: scroll;
                cursor: pointer;
                pointer-events: auto;
                user-select: all;
                outline: 1px solid black;
                outline-color: white;
                outline-style: dashed;
                outline-width: thin;
                transform: none;
                transform-origin: left top;
                translate: 10px 20px;
                rotate: 45deg;
                scale: 1.5 2;
                filter: none;
                backdrop-filter: blur(4px);
                clip-path: none;
                mask: none;
                mask-image: none;
                mask-size: auto;
                mask-position: center;
                mask-repeat: repeat;
                transition-property: opacity;
                transition-duration: 1s;
                transition-delay: 20ms;
                transition-timing-function: ease;
                transition: opacity 1s ease;
                animation-name: fade;
                animation-duration: 1s;
                animation-delay: 20ms;
                animation-timing-function: ease-out;
                animation-iteration-count: infinite;
                animation-direction: alternate;
                animation-fill-mode: both;
                animation-play-state: paused;
                animation: fade 1s ease-in-out infinite alternate both running;
            }",
        )
        .unwrap();
        let declarations = sheet.rules()[0].declarations();

        for property in [
            CssProperty::BackgroundImage,
            CssProperty::BackgroundPosition,
            CssProperty::BackgroundSize,
            CssProperty::BackgroundRepeat,
            CssProperty::BackgroundOrigin,
            CssProperty::BackgroundClip,
            CssProperty::BackgroundAttachment,
            CssProperty::Cursor,
            CssProperty::PointerEvents,
            CssProperty::UserSelect,
            CssProperty::Outline,
            CssProperty::OutlineColor,
            CssProperty::OutlineStyle,
            CssProperty::OutlineWidth,
            CssProperty::Transform,
            CssProperty::TransformOrigin,
            CssProperty::Translate,
            CssProperty::Rotate,
            CssProperty::Scale,
            CssProperty::Filter,
            CssProperty::BackdropFilter,
            CssProperty::ClipPath,
            CssProperty::Mask,
            CssProperty::MaskImage,
            CssProperty::MaskSize,
            CssProperty::MaskPosition,
            CssProperty::MaskRepeat,
            CssProperty::TransitionProperty,
            CssProperty::TransitionDuration,
            CssProperty::TransitionDelay,
            CssProperty::TransitionTimingFunction,
            CssProperty::Transition,
            CssProperty::AnimationName,
            CssProperty::AnimationDuration,
            CssProperty::AnimationDelay,
            CssProperty::AnimationTimingFunction,
            CssProperty::AnimationIterationCount,
            CssProperty::AnimationDirection,
            CssProperty::AnimationFillMode,
            CssProperty::AnimationPlayState,
            CssProperty::Animation,
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
    fn rejects_task_6_cross_family_leakage_values_and_empty_lists() {
        for input in [
            ".panel { background-size: solid; }",
            ".panel { cursor: 10px; }",
            ".panel { pointer-events: grab; }",
            ".panel { outline-width: 10%; }",
            ".panel { transform: red; }",
            ".panel { filter: 10px; }",
            ".panel { transition-duration: 10px; }",
            ".panel { animation-iteration-count: -1; }",
            ".panel { animation-play-state: visible; }",
            ".panel { transition: opacity 1s, ; }",
            ".panel { animation: fade 1s, ; }",
        ] {
            let error = parse_sheet(input).expect_err(input);
            assert!(matches!(
                error.kind(),
                ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
            ));
        }
    }

    #[test]
    fn rejects_duplicate_axis_position_keywords_across_shared_position_properties() {
        for input in [
            ".panel { background-position: left right; }",
            ".panel { background-position: right left; }",
            ".panel { background-position: top bottom; }",
            ".panel { background-position: bottom top; }",
            ".panel { mask-position: left right; }",
            ".panel { mask-position: top bottom; }",
            ".panel { transform-origin: left right; }",
            ".panel { transform-origin: top bottom; }",
            ".panel { mask: url(mask.png) left right / contain no-repeat; }",
            ".panel { mask: url(mask.png) top bottom / contain no-repeat; }",
        ] {
            let error = parse_sheet(input).expect_err(input);
            assert!(matches!(
                error.kind(),
                ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
            ));
        }
    }

    #[test]
    fn preserves_valid_position_keyword_forms_after_duplicate_axis_rejection() {
        for input in [
            ".panel { background-position: left top; }",
            ".panel { background-position: right bottom; }",
            ".panel { background-position: center center; }",
            ".panel { background-position: left 10px top 20%; }",
            ".panel { mask-position: center center; }",
            ".panel { transform-origin: right bottom; }",
            ".panel { mask: url(mask.png) left top / contain no-repeat; }",
        ] {
            parse_sheet(input).unwrap_or_else(|error| panic!("{input} should parse: {error}"));
        }
    }

    #[test]
    fn rejects_task_6_invalid_function_arguments() {
        for input in [
            ".panel { transform: translate(red); }",
            ".panel { filter: opacity(red); }",
            ".panel { clip-path: circle(red); }",
            ".panel { transition-timing-function: cubic-bezier(red); }",
        ] {
            let error = parse_sheet(input).expect_err(input);
            assert!(matches!(
                error.kind(),
                ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
            ));
        }
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

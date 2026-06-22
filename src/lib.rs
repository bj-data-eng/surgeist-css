//! Strict CSS ingestion for Surgeist style sheets.
//!
//! This module parses CSS syntax into [`surgeist_style`] values. It is strict by
//! design: unsupported selectors, at-rules, properties, and values are errors
//! instead of browser-style recoverable invalid declarations.

use std::fmt;

use cssparser::{
    AtRuleParser, BasicParseError, BasicParseErrorKind, CowRcStr, DeclarationParser, ParseError,
    ParseErrorKind, Parser, ParserInput, ParserState, QualifiedRuleParser, RuleBodyItemParser,
    RuleBodyParser, StyleSheetParser, ToCss, Token, match_ignore_ascii_case,
};

use surgeist_style::{
    self as style, AlignItems, BoxSizing, CalcLength, CalcLengthTerm, Color, Declarations,
    Direction, Display, Edges, FlexDirection, FlexWrap, GridFlowTolerance, LayoutPosition, Length,
    Overflow, OverflowAxes, Property, Selector, Sheet, Value,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    message: String,
    line: u32,
    column: u32,
}

impl Error {
    fn at(location: cssparser::SourceLocation, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            line: location.line,
            column: location.column,
        }
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

pub fn parse_sheet(input: &str) -> Result<Sheet> {
    let mut input = ParserInput::new(input);
    let mut parser = Parser::new(&mut input);
    let mut rule_parser = StrictRuleParser;
    let mut sheet = Sheet::new();

    for rule in StyleSheetParser::new(&mut parser, &mut rule_parser) {
        for (selector, declarations) in rule.map_err(|(error, _)| from_parse_error(error))? {
            sheet.push_rule(selector, declarations);
        }
    }

    Ok(sheet)
}

struct StrictRuleParser;

impl<'i> AtRuleParser<'i> for StrictRuleParser {
    type Prelude = ();
    type AtRule = Vec<(Selector, Declarations)>;
    type Error = Error;
}

impl<'i> QualifiedRuleParser<'i> for StrictRuleParser {
    type Prelude = Vec<Selector>;
    type QualifiedRule = Vec<(Selector, Declarations)>;
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
        let mut declarations = Declarations::new();
        let mut declaration_parser = StrictDeclarationParser;
        let location = input.current_source_location();
        for declaration in RuleBodyParser::new(input, &mut declaration_parser) {
            let (property, value) = declaration.map_err(|(error, _)| error)?;
            declarations
                .try_insert(property, value)
                .map_err(|error| error_at(location, error.to_string()))?;
        }

        Ok(selectors
            .into_iter()
            .map(|selector| (selector, declarations.clone()))
            .collect())
    }
}

struct StrictDeclarationParser;

impl<'i> AtRuleParser<'i> for StrictDeclarationParser {
    type Prelude = ();
    type AtRule = (Property, Value);
    type Error = Error;
}

impl<'i> QualifiedRuleParser<'i> for StrictDeclarationParser {
    type Prelude = ();
    type QualifiedRule = (Property, Value);
    type Error = Error;
}

impl<'i> RuleBodyItemParser<'i, (Property, Value), Error> for StrictDeclarationParser {
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

impl<'i> DeclarationParser<'i> for StrictDeclarationParser {
    type Declaration = (Property, Value);
    type Error = Error;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        _declaration_start: &ParserState,
    ) -> std::result::Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let result = match_ignore_ascii_case! { &name,
            "display" => (Property::Display, Value::Display(parse_display(input)?)),
            "box-sizing" => (Property::BoxSizing, Value::BoxSizing(parse_box_sizing(input)?)),
            "position" => (Property::Position, Value::Position(parse_position(input)?)),
            "direction" => (Property::Direction, Value::Direction(parse_direction(input)?)),
            "overflow" => (Property::Overflow, parse_overflow_value(input)?),
            "overflow-x" => (Property::OverflowX, Value::Overflow(parse_overflow(input)?)),
            "overflow-y" => (Property::OverflowY, Value::Overflow(parse_overflow(input)?)),
            "flex-direction" => (Property::FlexDirection, Value::FlexDirection(parse_flex_direction(input)?)),
            "flex-wrap" => (Property::FlexWrap, Value::FlexWrap(parse_flex_wrap(input)?)),
            "align-items" => (Property::AlignItems, Value::AlignItems(parse_align_items(input)?)),
            "align-self" => (Property::AlignSelf, Value::AlignItems(parse_align_items(input)?)),
            "justify-items" => (Property::JustifyItems, Value::AlignItems(parse_align_items(input)?)),
            "justify-self" => (Property::JustifySelf, Value::AlignItems(parse_align_items(input)?)),
            "width" => (Property::Width, Value::Length(parse_length(input)?)),
            "height" => (Property::Height, Value::Length(parse_length(input)?)),
            "min-width" => (Property::MinWidth, Value::Length(parse_length(input)?)),
            "min-height" => (Property::MinHeight, Value::Length(parse_length(input)?)),
            "max-width" => (Property::MaxWidth, Value::Length(parse_length(input)?)),
            "max-height" => (Property::MaxHeight, Value::Length(parse_length(input)?)),
            "flex-basis" => (Property::FlexBasis, Value::Length(parse_length(input)?)),
            "gap" => (Property::Gap, Value::Length(parse_gap_length(input)?)),
            "row-gap" => (Property::RowGap, Value::Length(parse_gap_length(input)?)),
            "column-gap" => (Property::ColumnGap, Value::Length(parse_gap_length(input)?)),
            "grid-flow-tolerance" => (Property::GridFlowTolerance, Value::GridFlowTolerance(parse_grid_flow_tolerance(input)?)),
            "font-size" => (Property::FontSize, Value::Length(parse_length(input)?)),
            "line-height" => (Property::LineHeight, Value::Length(parse_length(input)?)),
            "margin" => (Property::Margin, Value::Edges(parse_edges(input)?)),
            "padding" => (Property::Padding, Value::Edges(parse_edges(input)?)),
            "border-width" => (Property::BorderWidth, Value::Edges(parse_edges(input)?)),
            "color" => (Property::Color, Value::Color(parse_color(input)?)),
            "background" | "background-color" => (Property::Background, Value::Color(parse_color(input)?)),
            "border-color" => (Property::BorderColor, Value::Color(parse_color(input)?)),
            "opacity" => (Property::Opacity, Value::Number(parse_number(input)?)),
            "flex-grow" => (Property::FlexGrow, Value::Number(parse_number(input)?)),
            "flex-shrink" => (Property::FlexShrink, Value::Number(parse_number(input)?)),
            "aspect-ratio" => (Property::AspectRatio, Value::Number(parse_number(input)?)),
            "scrollbar-width" => (Property::ScrollbarWidth, Value::Number(parse_number(input)?)),
            _ => return Err(custom_error(input, format!("unsupported CSS property `{name}`"))),
        };
        input.expect_exhausted().map_err(basic)?;
        Ok(result)
    }
}

fn parse_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Vec<Selector>, ParseError<'i, Error>> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(parse_compound_selector(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted().map_err(basic)?;
    Ok(selectors)
}

fn parse_compound_selector<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Selector, ParseError<'i, Error>> {
    let mut compound = style::Compound::new();
    let mut tag_name = None;
    let mut key_name = None;
    let mut class_names = Vec::new();

    if let Ok(tag) = input.try_parse(Parser::expect_ident_cloned) {
        let tag = tag.to_string();
        compound = compound
            .tag(&tag)
            .map_err(|error| custom_error(input, error.to_string()))?;
        tag_name = Some(tag);
    }

    loop {
        if input.try_parse(|input| input.expect_delim('.')).is_ok() {
            let class = input.expect_ident_cloned().map_err(basic)?;
            let class = class.to_string();
            compound = compound
                .class(&class)
                .map_err(|error| custom_error(input, error.to_string()))?;
            class_names.push(class);
            continue;
        }

        let state = input.state();
        match input.next() {
            Ok(Token::IDHash(key)) => {
                let key = key.to_string();
                compound = compound
                    .key(&key)
                    .map_err(|error| custom_error(input, error.to_string()))?;
                key_name = Some(key);
            }
            Ok(token) => {
                let message = format!("unexpected selector token `{}`", token.to_css_string());
                input.reset(&state);
                if tag_name.is_none() && key_name.is_none() && class_names.is_empty() {
                    return Err(custom_error(input, message));
                }
                break;
            }
            Err(error) if matches!(error.kind, BasicParseErrorKind::EndOfInput) => break,
            Err(error) => return Err(basic(error)),
        }
    }

    if tag_name.is_none() && key_name.is_none() && class_names.is_empty() {
        return Err(custom_error(input, "selector is missing a simple selector"));
    }
    if let (None, None, [class]) = (tag_name.as_ref(), key_name.as_ref(), class_names.as_slice()) {
        return Selector::class(class).map_err(|error| custom_error(input, error.to_string()));
    }
    if let (Some(tag), None, []) = (tag_name.as_ref(), key_name.as_ref(), class_names.as_slice()) {
        return Selector::tag(tag).map_err(|error| custom_error(input, error.to_string()));
    }
    if let (None, Some(key), []) = (tag_name.as_ref(), key_name.as_ref(), class_names.as_slice()) {
        return Selector::key(key).map_err(|error| custom_error(input, error.to_string()));
    }
    Ok(compound.selector())
}

fn parse_display<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Display, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "block" => Ok(Display::Block),
        "flex" => Ok(Display::Flex),
        "grid" => Ok(Display::Grid),
        "inline-block" => Ok(Display::InlineBlock),
        "inline-grid" => Ok(Display::InlineGrid),
        "grid-lanes" => Ok(Display::GridLanes),
        "inline-grid-lanes" => Ok(Display::InlineGridLanes),
        "none" => Ok(Display::None),
        _ => Err(custom_error(input, format!("unsupported display `{ident}`"))),
    }
}

fn parse_box_sizing<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<BoxSizing, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "content-box" => Ok(BoxSizing::ContentBox),
        "border-box" => Ok(BoxSizing::BorderBox),
        _ => Err(custom_error(input, format!("unsupported box-sizing `{ident}`"))),
    }
}

fn parse_position<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<LayoutPosition, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "relative" => Ok(LayoutPosition::Relative),
        "absolute" => Ok(LayoutPosition::Absolute),
        _ => Err(custom_error(input, format!("unsupported position `{ident}`"))),
    }
}

fn parse_direction<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Direction, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "ltr" => Ok(Direction::Ltr),
        "rtl" => Ok(Direction::Rtl),
        _ => Err(custom_error(input, format!("unsupported direction `{ident}`"))),
    }
}

fn parse_overflow<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Overflow, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "visible" => Ok(Overflow::Visible),
        "clip" => Ok(Overflow::Clip),
        "hidden" => Ok(Overflow::Hidden),
        "scroll" => Ok(Overflow::Scroll),
        _ => Err(custom_error(input, format!("unsupported overflow `{ident}`"))),
    }
}

fn parse_overflow_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Value, ParseError<'i, Error>> {
    let x = parse_overflow(input)?;
    if input.is_exhausted() {
        Ok(Value::Overflow(x))
    } else {
        let y = parse_overflow(input)?;
        Ok(Value::OverflowAxes(OverflowAxes::new(x, y)))
    }
}

fn parse_flex_direction<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<FlexDirection, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "row" => Ok(FlexDirection::Row),
        "column" => Ok(FlexDirection::Column),
        "row-reverse" => Ok(FlexDirection::RowReverse),
        "column-reverse" => Ok(FlexDirection::ColumnReverse),
        _ => Err(custom_error(input, format!("unsupported flex-direction `{ident}`"))),
    }
}

fn parse_flex_wrap<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<FlexWrap, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "nowrap" => Ok(FlexWrap::NoWrap),
        "wrap" => Ok(FlexWrap::Wrap),
        "wrap-reverse" => Ok(FlexWrap::WrapReverse),
        _ => Err(custom_error(input, format!("unsupported flex-wrap `{ident}`"))),
    }
}

fn parse_align_items<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<AlignItems, ParseError<'i, Error>> {
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
        "start" => Ok(AlignItems::Start),
        "end" if safe => Ok(AlignItems::SafeEnd),
        "end" => Ok(AlignItems::End),
        "flex-start" => Ok(AlignItems::FlexStart),
        "flex-end" if safe => Ok(AlignItems::SafeFlexEnd),
        "flex-end" => Ok(AlignItems::FlexEnd),
        "center" if safe => Ok(AlignItems::SafeCenter),
        "center" => Ok(AlignItems::Center),
        "baseline" if !has_overflow_prefix => Ok(AlignItems::Baseline),
        "first" => {
            let baseline = input.expect_ident_cloned().map_err(basic)?;
            if has_overflow_prefix {
                Err(custom_error(
                    input,
                    format!("unsupported alignment `{first} first {baseline}`"),
                ))
            } else if baseline.eq_ignore_ascii_case("baseline") {
                Ok(AlignItems::Baseline)
            } else {
                Err(custom_error(
                    input,
                    format!("unsupported alignment `first {baseline}`"),
                ))
            }
        }
        "last" => {
            let baseline = input.expect_ident_cloned().map_err(basic)?;
            if has_overflow_prefix {
                Err(custom_error(
                    input,
                    format!("unsupported alignment `{first} last {baseline}`"),
                ))
            } else if baseline.eq_ignore_ascii_case("baseline") {
                Ok(AlignItems::LastBaseline)
            } else {
                Err(custom_error(
                    input,
                    format!("unsupported alignment `last {baseline}`"),
                ))
            }
        }
        "stretch" => Ok(AlignItems::Stretch),
        _ => Err(custom_error(
            input,
            format!("unsupported alignment `{keyword}`"),
        )),
    }
}

fn parse_grid_flow_tolerance<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<GridFlowTolerance, ParseError<'i, Error>> {
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "normal" => Ok(GridFlowTolerance::Normal),
            "infinite" => Ok(GridFlowTolerance::Infinite),
            _ => Err(custom_error(input, format!("unsupported grid-flow-tolerance `{ident}`"))),
        };
    }

    match parse_length(input)? {
        Length::Percent(value) => Ok(GridFlowTolerance::Percent(value)),
        length @ Length::Px(_) => Ok(GridFlowTolerance::Length(length)),
        length => Err(custom_error(
            input,
            format!("unsupported grid-flow-tolerance length `{length:?}`"),
        )),
    }
}

fn parse_edges<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Edges, ParseError<'i, Error>> {
    let mut values = Vec::new();
    while !input.is_exhausted() {
        values.push(parse_length(input)?);
        if values.len() == 4 && !input.is_exhausted() {
            return Err(custom_error(input, "edge shorthand has too many values"));
        }
    }
    Ok(match values.as_slice() {
        [all] => Edges::all(all.clone()),
        [vertical, horizontal] => Edges::new(
            vertical.clone(),
            horizontal.clone(),
            vertical.clone(),
            horizontal.clone(),
        ),
        [top, horizontal, bottom] => Edges::new(
            top.clone(),
            horizontal.clone(),
            bottom.clone(),
            horizontal.clone(),
        ),
        [top, right, bottom, left] => {
            Edges::new(top.clone(), right.clone(), bottom.clone(), left.clone())
        }
        [] => return Err(custom_error(input, "edge shorthand is missing a value")),
        _ => unreachable!("edge shorthand parser caps values at four"),
    })
}

fn parse_gap_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Length, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("normal"))
        .is_ok()
    {
        Ok(Length::NORMAL)
    } else {
        parse_length(input)
    }
}

fn parse_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Length, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, unit, .. } if unit.eq_ignore_ascii_case("px") => {
            Ok(Length::px(*value))
        }
        Token::Percentage { unit_value, .. } => Ok(Length::percent(*unit_value * 100.0)),
        Token::Number { value, .. } if *value == 0.0 => Ok(Length::ZERO),
        Token::Ident(ident) => match_ignore_ascii_case! { ident,
            "auto" => Ok(Length::Auto),
            "min-content" => Ok(Length::MinContent),
            "max-content" => Ok(Length::MaxContent),
            "fit-content" => Ok(Length::Fit),
            _ => Err(error_at(location, format!("unsupported length `{ident}`"))),
        },
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            let calc = input.parse_nested_block(parse_calc_length)?;
            Ok(Length::Calc(calc))
        }
        Token::Function(name) => Err(error_at(
            location,
            format!("unsupported length function `{name}`"),
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_calc_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CalcLength, ParseError<'i, Error>> {
    let mut terms = Vec::new();
    terms.push(CalcLengthTerm::add(parse_calc_component(input)?));

    while !input.is_exhausted() {
        let location = input.current_source_location();
        let operator = match input.next().map_err(basic)? {
            Token::Delim('+') => CalcLengthTerm::add,
            Token::Delim('-') => CalcLengthTerm::sub,
            token => {
                return Err(error_at(
                    location,
                    format!("expected calc operator, got `{}`", token.to_css_string()),
                ));
            }
        };
        let component = parse_calc_component(input)?;
        terms.push(operator(component));
    }

    Ok(CalcLength::sum(terms))
}

fn parse_calc_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CalcLength, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, unit, .. } if unit.eq_ignore_ascii_case("px") => {
            Ok(CalcLength::px(*value))
        }
        Token::Dimension { unit, .. } => Err(error_at(
            location,
            format!("unsupported calc length unit `{unit}`"),
        )),
        Token::Percentage { unit_value, .. } => Ok(CalcLength::percent(*unit_value * 100.0)),
        Token::Number { value, .. } if *value == 0.0 => Ok(CalcLength::px(0.0)),
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            input.parse_nested_block(parse_calc_length)
        }
        Token::Function(name) => Err(error_at(
            location,
            format!("unsupported calc function `{name}`"),
        )),
        token => Err(error_at(
            location,
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
) -> std::result::Result<Color, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::IDHash(hex) | Token::Hash(hex) => color_from_hex(location, hex.as_ref()),
        Token::Ident(ident) => match_ignore_ascii_case! { ident,
            "transparent" => Ok(Color::TRANSPARENT),
            "black" => Ok(Color::BLACK),
            "white" => Ok(Color::rgba(1.0, 1.0, 1.0, 1.0)),
            _ => Err(error_at(location, format!("unsupported color `{ident}`"))),
        },
        token => Err(error_at(
            location,
            format!("unexpected CSS token `{}`", token.to_css_string()),
        )),
    }
}

fn color_from_hex<'i>(
    location: cssparser::SourceLocation,
    hex: &str,
) -> std::result::Result<Color, ParseError<'i, Error>> {
    let expanded = match hex.len() {
        3 => hex.chars().flat_map(|ch| [ch, ch]).collect::<String>(),
        6 => hex.to_owned(),
        _ => {
            return Err(error_at(
                location,
                format!("unsupported hex color `#{hex}`"),
            ));
        }
    };
    let value = u32::from_str_radix(&expanded, 16)
        .map_err(|_| error_at(location, format!("invalid hex color `#{hex}`")))?;
    Ok(Color::rgba(
        ((value >> 16) & 0xff) as f32 / 255.0,
        ((value >> 8) & 0xff) as f32 / 255.0,
        (value & 0xff) as f32 / 255.0,
        1.0,
    ))
}

fn from_parse_error(error: ParseError<'_, Error>) -> Error {
    match error.kind {
        ParseErrorKind::Custom(error) => error,
        ParseErrorKind::Basic(kind) => Error::at(error.location, basic_error_message(kind)),
    }
}

fn basic_error_message(kind: BasicParseErrorKind<'_>) -> String {
    match kind {
        BasicParseErrorKind::EndOfInput => "unexpected end of CSS input".to_owned(),
        BasicParseErrorKind::AtRuleInvalid(name) => {
            format!("unsupported CSS at-rule `@{name}`")
        }
        BasicParseErrorKind::QualifiedRuleInvalid => "invalid CSS rule".to_owned(),
        BasicParseErrorKind::AtRuleBodyInvalid => "invalid CSS at-rule body".to_owned(),
        BasicParseErrorKind::UnexpectedToken(token) => {
            format!("unexpected CSS token `{}`", token.to_css_string())
        }
    }
}

fn basic<'i>(error: BasicParseError<'i>) -> ParseError<'i, Error> {
    error.into()
}

fn custom_error<'i, 't>(
    input: &Parser<'i, 't>,
    message: impl Into<String>,
) -> ParseError<'i, Error> {
    input.new_custom_error(Error::at(input.current_source_location(), message))
}

fn error_at<'i>(
    location: cssparser::SourceLocation,
    message: impl Into<String>,
) -> ParseError<'i, Error> {
    ParseError {
        kind: ParseErrorKind::Custom(Error::at(location, message)),
        location,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declaration_value(input: &str, property: style::Property) -> style::Value {
        let sheet = parse_sheet(input).unwrap();
        sheet.rules()[0]
            .declarations()
            .get(property)
            .unwrap()
            .clone()
    }

    #[test]
    fn parses_calc_width_as_style_calc_length() {
        let value = declaration_value(
            ".panel { width: calc(20px + 10%); }",
            style::Property::Width,
        );

        match value {
            style::Value::Length(style::Length::Calc(calc)) => {
                assert!(calc.uses_percentage());
                assert_eq!(calc.to_css_string(), "calc(20px + 10%)");
            }
            other => panic!("expected calc length, got {other:?}"),
        }
    }

    #[test]
    fn parses_nested_calc_width_with_subtraction() {
        let value = declaration_value(
            ".panel { width: calc(100% - calc(12px + 3%)); }",
            style::Property::Width,
        );

        match value {
            style::Value::Length(style::Length::Calc(calc)) => {
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
    fn rejects_unknown_calc_functions() {
        let error = parse_sheet(".panel { width: min(10px, 20px); }").unwrap_err();
        assert!(error.message().contains("unsupported length function"));
    }
}

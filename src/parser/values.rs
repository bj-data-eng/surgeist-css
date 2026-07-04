use cssparser::{ParseError, Parser, ToCss, Token, match_ignore_ascii_case};

use crate::error::{
    Error, ErrorKind, basic, error_at, invalid_color, invalid_syntax, unsupported_value_at,
};
use crate::syntax::{self, *};
use crate::validation::{
    LengthUnitStatus, classify_length_unit, parse_global_keyword, unsupported_keyword_reason,
};

pub(super) fn parse_box_size_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthGrammar::BoxSize)
}

pub(super) fn parse_inset_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthGrammar::Inset)
}

pub(super) fn parse_margin_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthGrammar::Margin)
}

pub(super) fn parse_padding_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthGrammar::Padding)
}

pub(super) fn parse_border_width_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthGrammar::BorderWidth)
}

pub(super) fn parse_radius_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthGrammar::Radius)
}

pub(super) fn parse_shadow_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthGrammar::ShadowOffset)
}

pub(super) fn parse_shadow_blur_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthGrammar::ShadowBlur)
}

pub(super) fn parse_gap_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("normal"))
        .is_ok()
    {
        Ok(CssLength::Normal)
    } else {
        parse_length_with(input, LengthGrammar::Gap)
    }
}

#[derive(Clone, Copy)]
pub(super) enum LengthGrammar {
    BoxSize,
    Inset,
    Margin,
    Padding,
    BorderWidth,
    Radius,
    ShadowOffset,
    ShadowBlur,
    Gap,
    FontSize,
    LineHeight,
    TextIndent,
    VerticalAlign,
    LetterSpacing,
    TextDecorationThickness,
    GridTrack,
    BackgroundSize,
    Position,
}

impl LengthGrammar {
    const fn allows_percent(self) -> bool {
        matches!(
            self,
            Self::BoxSize
                | Self::Inset
                | Self::Margin
                | Self::Padding
                | Self::Radius
                | Self::Gap
                | Self::FontSize
                | Self::LineHeight
                | Self::TextIndent
                | Self::VerticalAlign
                | Self::TextDecorationThickness
                | Self::GridTrack
                | Self::BackgroundSize
                | Self::Position
        )
    }

    const fn allows_auto(self) -> bool {
        matches!(self, Self::BoxSize | Self::Inset | Self::Margin)
    }

    const fn allows_intrinsic(self) -> bool {
        matches!(self, Self::BoxSize | Self::Inset)
    }

    const fn allows_normal(self) -> bool {
        matches!(self, Self::Gap | Self::LineHeight)
    }

    const fn allows_calc_percent(self) -> bool {
        matches!(
            self,
            Self::BoxSize
                | Self::Inset
                | Self::Margin
                | Self::Padding
                | Self::Radius
                | Self::Gap
                | Self::FontSize
                | Self::LineHeight
                | Self::TextIndent
                | Self::VerticalAlign
                | Self::TextDecorationThickness
                | Self::GridTrack
                | Self::BackgroundSize
                | Self::Position
        )
    }

    const fn requires_non_negative(self) -> bool {
        matches!(
            self,
            Self::Padding
                | Self::BorderWidth
                | Self::Radius
                | Self::ShadowBlur
                | Self::TextDecorationThickness
                | Self::GridTrack
                | Self::BackgroundSize
        )
    }

    const fn context(self) -> &'static str {
        match self {
            Self::BoxSize => "box size",
            Self::Inset => "inset",
            Self::Margin => "margin",
            Self::Padding => "padding",
            Self::BorderWidth => "border-width",
            Self::Radius => "border-radius",
            Self::ShadowOffset => "box-shadow",
            Self::ShadowBlur => "box-shadow blur",
            Self::Gap => "gap",
            Self::FontSize => "font-size",
            Self::LineHeight => "line-height",
            Self::TextIndent => "text-indent",
            Self::VerticalAlign => "vertical-align",
            Self::LetterSpacing => "letter-spacing",
            Self::TextDecorationThickness => "text-decoration-thickness",
            Self::GridTrack => "grid track",
            Self::BackgroundSize => "background-size",
            Self::Position => "position",
        }
    }
}

pub(super) fn parse_length_with<'i, 't>(
    input: &mut Parser<'i, 't>,
    grammar: LengthGrammar,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with_context(input, grammar, grammar.context())
}

pub(super) fn parse_length_with_context<'i, 't>(
    input: &mut Parser<'i, 't>,
    grammar: LengthGrammar,
    context: &str,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, .. } if !value.is_finite() => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported non-finite {context} length"),
        )),
        Token::Dimension { value, unit, .. } => match classify_length_unit(unit) {
            LengthUnitStatus::Supported(_) if grammar.requires_non_negative() && *value < 0.0 => {
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
        Token::Percentage { unit_value, .. } if !unit_value.is_finite() => {
            Err(unsupported_value_at(
                location,
                None,
                format!("unsupported non-finite {context} percentage"),
            ))
        }
        Token::Percentage { unit_value, .. }
            if grammar.requires_non_negative() && *unit_value < 0.0 =>
        {
            Err(unsupported_value_at(
                location,
                None,
                format!("unsupported negative {context} percentage"),
            ))
        }
        Token::Percentage { unit_value, .. } if grammar.allows_percent() => {
            Ok(CssLength::percent(*unit_value * 100.0))
        }
        Token::Percentage { .. } => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported {context} percentage"),
        )),
        Token::Number { value, .. } if *value == 0.0 => Ok(CssLength::Zero),
        Token::Ident(ident) => match_ignore_ascii_case! { ident,
            "auto" if grammar.allows_auto() => Ok(CssLength::Auto),
            "normal" if grammar.allows_normal() => Ok(CssLength::Normal),
            "min-content" if grammar.allows_intrinsic() => Ok(CssLength::MinContent),
            "max-content" if grammar.allows_intrinsic() => Ok(CssLength::MaxContent),
            "fit-content" if grammar.allows_intrinsic() => Ok(CssLength::FitContent),
            _ => Err(unsupported_value_at(
                location,
                None,
                format!("unsupported {context} `{ident}`"),
            )),
        },
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            let calc =
                input.parse_nested_block(|input| parse_calc_length_with_grammar(input, grammar))?;
            if grammar.requires_non_negative() && syntax::calc_has_negative_component(&calc) {
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

pub(super) fn parse_calc_length_with_grammar<'i, 't>(
    input: &mut Parser<'i, 't>,
    grammar: LengthGrammar,
) -> std::result::Result<CssCalcLength, ParseError<'i, Error>> {
    let mut terms = Vec::new();
    terms.push(CssCalcLengthTerm::add(parse_calc_component(
        input, grammar,
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
        let component = parse_calc_component(input, grammar)?;
        terms.push(operator(component));
    }

    let mut terms = terms.into_iter();
    let first = terms
        .next()
        .expect("calc parser records the first term before parsing operators");
    Ok(CssCalcLength::sum(first, terms))
}

pub(super) fn parse_calc_component<'i, 't>(
    input: &mut Parser<'i, 't>,
    grammar: LengthGrammar,
) -> std::result::Result<CssCalcLength, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, .. } if !value.is_finite() => Err(unsupported_value_at(
            location,
            None,
            "unsupported non-finite calc length",
        )),
        Token::Dimension { value, unit, .. } => match classify_length_unit(unit) {
            LengthUnitStatus::Supported(_) if grammar.requires_non_negative() && *value < 0.0 => {
                Err(unsupported_value_at(
                    location,
                    None,
                    "unsupported negative calc length",
                ))
            }
            LengthUnitStatus::Supported(unit) => Ok(CssCalcLength::dimension(*value, unit)),
            LengthUnitStatus::Unknown => Err(unsupported_value_at(
                location,
                None,
                format!("unknown calc length unit `{unit}`"),
            )),
        },
        Token::Percentage { unit_value, .. } if !unit_value.is_finite() => Err(
            unsupported_value_at(location, None, "unsupported non-finite calc percentage"),
        ),
        Token::Percentage { unit_value, .. }
            if grammar.requires_non_negative() && *unit_value < 0.0 =>
        {
            Err(unsupported_value_at(
                location,
                None,
                "unsupported negative calc percentage",
            ))
        }
        Token::Percentage { unit_value, .. } if grammar.allows_calc_percent() => {
            Ok(CssCalcLength::percent(*unit_value * 100.0))
        }
        Token::Percentage { .. } => Err(unsupported_value_at(
            location,
            None,
            "unsupported calc percentage",
        )),
        Token::Number { value, .. } if *value == 0.0 => Ok(CssCalcLength::px(0.0)),
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            input.parse_nested_block(|input| parse_calc_length_with_grammar(input, grammar))
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

pub(super) fn parse_number<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<f32, ParseError<'i, Error>> {
    input.expect_number().map_err(basic)
}

pub(super) fn parse_integer<'i, 't>(
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

pub(super) fn parse_positive_integer<'i, 't>(
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

pub(super) fn parse_custom_ident_from_str_at<'i>(
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

pub(super) fn next_is_delim<'i, 't>(input: &mut Parser<'i, 't>, delim: char) -> bool {
    let state = input.state();
    let is_delim = input.try_parse(|input| input.expect_delim(delim)).is_ok();
    input.reset(&state);
    is_delim
}

pub(super) fn next_is_comma<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    let state = input.state();
    let is_comma = input.try_parse(Parser::expect_comma).is_ok();
    input.reset(&state);
    is_comma
}

pub(super) fn next_is_ident<'i, 't>(input: &mut Parser<'i, 't>, expected: &str) -> bool {
    let state = input.state();
    let is_ident = input
        .try_parse(|input| input.expect_ident_matching(expected))
        .is_ok();
    input.reset(&state);
    is_ident
}

pub(super) fn parse_color<'i, 't>(
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

pub(super) fn color_from_hex<'i>(
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
    Ok(CssColor::rgba_unchecked(
        ((value >> 16) & 0xff) as f32 / 255.0,
        ((value >> 8) & 0xff) as f32 / 255.0,
        (value & 0xff) as f32 / 255.0,
        1.0,
    ))
}

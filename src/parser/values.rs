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
    parse_length_with(input, LengthOptions::box_size(), "box size")
}

pub(super) fn parse_inset_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::box_size(), "inset")
}

pub(super) fn parse_margin_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::margin(), "margin")
}

pub(super) fn parse_padding_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::padding(), "padding")
}

pub(super) fn parse_border_width_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::border_width(), "border-width")
}

pub(super) fn parse_radius_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::radius(), "border-radius")
}

pub(super) fn parse_shadow_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::shadow(), "box-shadow")
}

pub(super) fn parse_shadow_blur_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, LengthOptions::shadow_blur(), "box-shadow blur")
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
        parse_length_with(input, LengthOptions::gap(), "gap")
    }
}

#[derive(Clone, Copy)]
pub(super) struct LengthOptions {
    percent: bool,
    auto: bool,
    intrinsic: bool,
    normal: bool,
    calc_percent: bool,
    non_negative: bool,
}

impl LengthOptions {
    pub(super) const fn box_size() -> Self {
        Self {
            percent: true,
            auto: true,
            intrinsic: true,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    pub(super) const fn margin() -> Self {
        Self {
            percent: true,
            auto: true,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    pub(super) const fn padding() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: true,
        }
    }

    pub(super) const fn border_width() -> Self {
        Self {
            percent: false,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: false,
            non_negative: true,
        }
    }

    pub(super) const fn radius() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: true,
        }
    }

    pub(super) const fn shadow() -> Self {
        Self {
            percent: false,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: false,
            non_negative: false,
        }
    }

    pub(super) const fn shadow_blur() -> Self {
        Self {
            percent: false,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: false,
            non_negative: true,
        }
    }

    pub(super) const fn gap() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: true,
            calc_percent: true,
            non_negative: false,
        }
    }

    pub(super) const fn font_size() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    pub(super) const fn line_height() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: true,
            calc_percent: true,
            non_negative: false,
        }
    }

    pub(super) const fn text_indent() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    pub(super) const fn vertical_align() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    pub(super) const fn letter_spacing() -> Self {
        Self {
            percent: false,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: false,
            non_negative: false,
        }
    }

    pub(super) const fn text_decoration_thickness() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: true,
        }
    }

    pub(super) const fn grid_track() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: true,
        }
    }

    pub(super) const fn position() -> Self {
        Self {
            percent: true,
            auto: false,
            intrinsic: false,
            normal: false,
            calc_percent: true,
            non_negative: false,
        }
    }

    pub(super) const fn background_size() -> Self {
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

pub(super) fn parse_length_with<'i, 't>(
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

pub(super) fn parse_calc_length_with_options<'i, 't>(
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

pub(super) fn parse_calc_component<'i, 't>(
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

pub(super) fn parse_number<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<f32, ParseError<'i, Error>> {
    input.expect_number().map_err(basic)
}

pub(super) fn parse_non_negative_number<'i, 't>(
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
    Ok(CssColor::rgba(
        ((value >> 16) & 0xff) as f32 / 255.0,
        ((value >> 8) & 0xff) as f32 / 255.0,
        (value & 0xff) as f32 / 255.0,
        1.0,
    ))
}

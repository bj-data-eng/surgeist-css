use cssparser::{ParseError, Parser, Token, match_ignore_ascii_case};

use super::values::{
    AllowedLengthSyntax, next_is_comma, parse_color, parse_integer, parse_length_with,
};
use crate::error::{Error, basic, unsupported_value, unsupported_value_at};
use crate::syntax::*;
use crate::validation::unsupported_keyword_reason;

pub(super) fn parse_font_size<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with(input, AllowedLengthSyntax::font_size(), "font-size")
}

pub(super) fn parse_line_height<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("normal"))
        .is_ok()
    {
        Ok(CssLength::Normal)
    } else {
        parse_length_with(input, AllowedLengthSyntax::line_height(), "line-height")
    }
}

pub(super) fn parse_writing_mode<'i, 't>(
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

pub(super) fn parse_text_align<'i, 't>(
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

pub(super) fn parse_text_align_last<'i, 't>(
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

pub(super) fn parse_text_indent<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTextIndent, ParseError<'i, Error>> {
    let length = parse_length_with(input, AllowedLengthSyntax::text_indent(), "text-indent")?;
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

pub(super) fn parse_vertical_align<'i, 't>(
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

    parse_length_with(
        input,
        AllowedLengthSyntax::vertical_align(),
        "vertical-align",
    )
    .map(CssVerticalAlignLength::new)
    .map(CssVerticalAlign::Length)
}

pub(super) fn parse_font_family_list<'i, 't>(
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

pub(super) fn parse_font_family_name<'i, 't>(
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

pub(super) fn parse_font<'i, 't>(
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

pub(super) fn parse_font_weight<'i, 't>(
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

pub(super) fn parse_font_style<'i, 't>(
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

pub(super) fn parse_font_stretch<'i, 't>(
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

pub(super) fn parse_font_variant<'i, 't>(
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

pub(super) fn parse_font_feature_settings<'i, 't>(
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

pub(super) fn parse_font_feature<'i, 't>(
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

pub(super) fn parse_letter_spacing<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssLetterSpacing, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("normal"))
        .is_ok()
    {
        Ok(CssLetterSpacing::Normal)
    } else {
        parse_length_with(
            input,
            AllowedLengthSyntax::letter_spacing(),
            "letter-spacing",
        )
        .map(CssLetterSpacingLength::new)
        .map(CssLetterSpacing::Length)
    }
}

pub(super) fn parse_text_wrap<'i, 't>(
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

pub(super) fn parse_white_space<'i, 't>(
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

pub(super) fn parse_word_break<'i, 't>(
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

pub(super) fn parse_overflow_wrap<'i, 't>(
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

pub(super) fn parse_text_overflow<'i, 't>(
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

pub(super) fn parse_text_decoration<'i, 't>(
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

pub(super) fn parse_text_decoration_line<'i, 't>(
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

pub(super) fn parse_text_decoration_line_component<'i, 't>(
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

pub(super) fn parse_text_decoration_style<'i, 't>(
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

pub(super) fn parse_text_decoration_thickness<'i, 't>(
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
        AllowedLengthSyntax::text_decoration_thickness(),
        "text-decoration-thickness",
    )
    .map(CssTextDecorationThicknessLength::new)
    .map(CssTextDecorationThickness::Length)
}

pub(super) fn parse_text_transform<'i, 't>(
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

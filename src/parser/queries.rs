#[cfg(test)]
use cssparser::ParserInput;
use cssparser::{ParseError, Parser, ToCss, Token, match_ignore_ascii_case};

use super::variables::collect_authored_declaration_value;
use crate::error::{Error, basic, invalid_syntax, unsupported_value_at};
#[cfg(test)]
use crate::error::{Result as CrateResult, from_parse_error};
use crate::syntax::*;

pub(crate) fn parse_media_query_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMediaQueryList, ParseError<'i, Error>> {
    let mut queries = vec![parse_media_query(input)?];

    while input.try_parse(Parser::expect_comma).is_ok() {
        queries.push(parse_media_query(input)?);
    }

    Ok(CssMediaQueryList::new(queries))
}

#[cfg(test)]
pub(crate) fn parse_media_query_list_for_test(input: &str) -> CrateResult<CssMediaQueryList> {
    let mut input = ParserInput::new(input);
    let mut parser = Parser::new(&mut input);
    let list = parse_media_query_list(&mut parser).map_err(from_parse_error)?;
    if !parser.is_exhausted() {
        return Err(from_parse_error(invalid_syntax(
            parser.current_source_location(),
            "unexpected token after media query list",
        )));
    }
    Ok(list)
}

pub(crate) fn parse_container_condition<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssContainerCondition, ParseError<'i, Error>> {
    let first = parse_container_condition_atom(input)?;

    if input
        .try_parse(|input| input.expect_ident_matching("and"))
        .is_ok()
    {
        let mut conditions = vec![first, parse_container_condition_atom(input)?];
        while input
            .try_parse(|input| input.expect_ident_matching("and"))
            .is_ok()
        {
            conditions.push(parse_container_condition_atom(input)?);
        }
        return Ok(CssContainerCondition::And(CssContainerConditionList::new(
            conditions,
        )));
    }

    if input
        .try_parse(|input| input.expect_ident_matching("or"))
        .is_ok()
    {
        let mut conditions = vec![first, parse_container_condition_atom(input)?];
        while input
            .try_parse(|input| input.expect_ident_matching("or"))
            .is_ok()
        {
            conditions.push(parse_container_condition_atom(input)?);
        }
        return Ok(CssContainerCondition::Or(CssContainerConditionList::new(
            conditions,
        )));
    }

    Ok(first)
}

#[cfg(test)]
pub(crate) fn parse_container_condition_for_test(
    input: &str,
) -> CrateResult<CssContainerCondition> {
    let mut input = ParserInput::new(input);
    let mut parser = Parser::new(&mut input);
    let condition = parse_container_condition(&mut parser).map_err(from_parse_error)?;
    if !parser.is_exhausted() {
        return Err(from_parse_error(invalid_syntax(
            parser.current_source_location(),
            "unexpected token after container condition",
        )));
    }
    Ok(condition)
}

fn parse_container_condition_atom<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssContainerCondition, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("not"))
        .is_ok()
    {
        return Ok(CssContainerCondition::Not(Box::new(
            parse_container_condition_atom(input)?,
        )));
    }

    if let Ok(style) = input.try_parse(parse_container_style_query) {
        return Ok(CssContainerCondition::Style(style));
    }

    input.expect_parenthesis_block().map_err(basic)?;
    let feature = input.parse_nested_block(|input| {
        let feature = parse_container_feature_query(input)?;
        if !input.is_exhausted() {
            return Err(invalid_syntax(
                input.current_source_location(),
                "unexpected token in container feature query",
            ));
        }
        Ok(feature)
    })?;
    Ok(CssContainerCondition::Feature(feature))
}

fn parse_container_feature_query<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssContainerFeatureQuery, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    let Some(feature_name) = ContainerFeatureName::parse(&ident) else {
        return Err(unsupported_value_at(
            location,
            None,
            format!("unsupported container feature `{ident}`"),
        ));
    };

    match feature_name {
        ContainerFeatureName::Width(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_query_length(input)?;
            Ok(CssContainerFeatureQuery::Width(CssRangeFeature::new(
                comparison, value,
            )))
        }
        ContainerFeatureName::Height(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_query_length(input)?;
            Ok(CssContainerFeatureQuery::Height(CssRangeFeature::new(
                comparison, value,
            )))
        }
        ContainerFeatureName::InlineSize(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_query_length(input)?;
            Ok(CssContainerFeatureQuery::InlineSize(CssRangeFeature::new(
                comparison, value,
            )))
        }
        ContainerFeatureName::BlockSize(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_query_length(input)?;
            Ok(CssContainerFeatureQuery::BlockSize(CssRangeFeature::new(
                comparison, value,
            )))
        }
        ContainerFeatureName::AspectRatio(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_ratio(input)?;
            Ok(CssContainerFeatureQuery::AspectRatio(CssRangeFeature::new(
                comparison, value,
            )))
        }
        ContainerFeatureName::Orientation => {
            input.expect_colon().map_err(basic)?;
            parse_orientation(input).map(CssContainerFeatureQuery::Orientation)
        }
    }
}

fn parse_container_style_query<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssContainerStyleQuery, ParseError<'i, Error>> {
    input.expect_function_matching("style").map_err(basic)?;
    input.parse_nested_block(|input| {
        let location = input.current_source_location();
        let name = input.expect_ident_cloned().map_err(basic)?;
        let Some(name) = CssCustomPropertyName::try_new(name.to_string()) else {
            return Err(invalid_syntax(
                location,
                "container style queries only support custom properties",
            ));
        };

        if input.is_exhausted() {
            return Ok(CssContainerStyleQuery::CustomPropertyPresence(name));
        }

        input.expect_colon().map_err(basic)?;
        let (value, _) = collect_authored_declaration_value(input)?;
        if value.as_css().trim().is_empty() {
            return Err(invalid_syntax(
                input.current_source_location(),
                "container style query custom property value must not be empty",
            ));
        }

        Ok(CssContainerStyleQuery::CustomPropertyValue { name, value })
    })
}

fn parse_media_query<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMediaQuery, ParseError<'i, Error>> {
    if let Ok(query) = input.try_parse(parse_typed_media_query) {
        return Ok(CssMediaQuery::Typed(query));
    }

    parse_media_condition(input).map(CssMediaQuery::Condition)
}

fn parse_typed_media_query<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTypedMediaQuery, ParseError<'i, Error>> {
    let modifier = input.try_parse(parse_media_query_modifier).ok();
    let media_type = parse_media_type(input)?;
    let condition = if input
        .try_parse(|input| input.expect_ident_matching("and"))
        .is_ok()
    {
        Some(parse_media_condition(input)?)
    } else {
        None
    };

    Ok(CssTypedMediaQuery::new(modifier, media_type, condition))
}

fn parse_media_query_modifier<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMediaQueryModifier, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "not" => Ok(CssMediaQueryModifier::Not),
        "only" => Ok(CssMediaQueryModifier::Only),
        _ => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported media query modifier `{ident}`"),
        )),
    }
}

fn parse_media_type<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMediaType, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "all" => Ok(CssMediaType::All),
        "screen" => Ok(CssMediaType::Screen),
        "print" => Ok(CssMediaType::Print),
        _ => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported media type `{ident}`"),
        )),
    }
}

fn parse_media_condition<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMediaCondition, ParseError<'i, Error>> {
    let first = parse_media_condition_atom(input)?;

    if input
        .try_parse(|input| input.expect_ident_matching("and"))
        .is_ok()
    {
        let mut conditions = vec![first, parse_media_condition_atom(input)?];
        while input
            .try_parse(|input| input.expect_ident_matching("and"))
            .is_ok()
        {
            conditions.push(parse_media_condition_atom(input)?);
        }
        return Ok(CssMediaCondition::And(CssMediaConditionList::new(
            conditions,
        )));
    }

    if input
        .try_parse(|input| input.expect_ident_matching("or"))
        .is_ok()
    {
        let mut conditions = vec![first, parse_media_condition_atom(input)?];
        while input
            .try_parse(|input| input.expect_ident_matching("or"))
            .is_ok()
        {
            conditions.push(parse_media_condition_atom(input)?);
        }
        return Ok(CssMediaCondition::Or(CssMediaConditionList::new(
            conditions,
        )));
    }

    Ok(first)
}

fn parse_media_condition_atom<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMediaCondition, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("not"))
        .is_ok()
    {
        return Ok(CssMediaCondition::Not(Box::new(
            parse_media_condition_atom(input)?,
        )));
    }

    input.expect_parenthesis_block().map_err(basic)?;
    let feature = input.parse_nested_block(|input| {
        let feature = parse_media_feature_query(input)?;
        if !input.is_exhausted() {
            return Err(invalid_syntax(
                input.current_source_location(),
                "unexpected token in media feature query",
            ));
        }
        Ok(feature)
    })?;
    Ok(CssMediaCondition::Feature(feature))
}

fn parse_media_feature_query<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMediaFeatureQuery, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    let Some(feature_name) = MediaFeatureName::parse(&ident) else {
        return Err(unsupported_value_at(
            location,
            None,
            format!("unsupported media feature `{ident}`"),
        ));
    };

    match feature_name {
        MediaFeatureName::Width(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_query_length(input)?;
            Ok(CssMediaFeatureQuery::Width(CssRangeFeature::new(
                comparison, value,
            )))
        }
        MediaFeatureName::Height(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_query_length(input)?;
            Ok(CssMediaFeatureQuery::Height(CssRangeFeature::new(
                comparison, value,
            )))
        }
        MediaFeatureName::Resolution(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_resolution(input)?;
            Ok(CssMediaFeatureQuery::Resolution(CssRangeFeature::new(
                comparison, value,
            )))
        }
        MediaFeatureName::Color(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_non_negative_integer(input)?;
            Ok(CssMediaFeatureQuery::Color(CssRangeFeature::new(
                comparison, value,
            )))
        }
        MediaFeatureName::Monochrome(prefix) => {
            let comparison = parse_range_feature_comparison(input, prefix)?;
            let value = parse_non_negative_integer(input)?;
            Ok(CssMediaFeatureQuery::Monochrome(CssRangeFeature::new(
                comparison, value,
            )))
        }
        MediaFeatureName::Orientation => {
            input.expect_colon().map_err(basic)?;
            parse_orientation(input).map(CssMediaFeatureQuery::Orientation)
        }
        MediaFeatureName::PrefersColorScheme => {
            input.expect_colon().map_err(basic)?;
            parse_color_scheme_preference(input).map(CssMediaFeatureQuery::PrefersColorScheme)
        }
        MediaFeatureName::PrefersReducedMotion => {
            input.expect_colon().map_err(basic)?;
            parse_reduced_motion_preference(input).map(CssMediaFeatureQuery::PrefersReducedMotion)
        }
        MediaFeatureName::PrefersReducedTransparency => {
            input.expect_colon().map_err(basic)?;
            parse_reduced_transparency_preference(input)
                .map(CssMediaFeatureQuery::PrefersReducedTransparency)
        }
        MediaFeatureName::PrefersContrast => {
            input.expect_colon().map_err(basic)?;
            parse_contrast_preference(input).map(CssMediaFeatureQuery::PrefersContrast)
        }
        MediaFeatureName::ForcedColors => {
            input.expect_colon().map_err(basic)?;
            parse_forced_colors_mode(input).map(CssMediaFeatureQuery::ForcedColors)
        }
        MediaFeatureName::Hover => {
            input.expect_colon().map_err(basic)?;
            parse_hover_capability(input).map(CssMediaFeatureQuery::Hover)
        }
        MediaFeatureName::AnyHover => {
            input.expect_colon().map_err(basic)?;
            parse_hover_capability(input).map(CssMediaFeatureQuery::AnyHover)
        }
        MediaFeatureName::Pointer => {
            input.expect_colon().map_err(basic)?;
            parse_pointer_capability(input).map(CssMediaFeatureQuery::Pointer)
        }
        MediaFeatureName::AnyPointer => {
            input.expect_colon().map_err(basic)?;
            parse_pointer_capability(input).map(CssMediaFeatureQuery::AnyPointer)
        }
        MediaFeatureName::DisplayMode => {
            input.expect_colon().map_err(basic)?;
            parse_display_mode(input).map(CssMediaFeatureQuery::DisplayMode)
        }
    }
}

#[derive(Clone, Copy)]
enum RangePrefix {
    Min,
    Max,
}

#[derive(Clone, Copy)]
enum MediaFeatureName {
    Width(Option<RangePrefix>),
    Height(Option<RangePrefix>),
    Resolution(Option<RangePrefix>),
    Color(Option<RangePrefix>),
    Monochrome(Option<RangePrefix>),
    Orientation,
    PrefersColorScheme,
    PrefersReducedMotion,
    PrefersReducedTransparency,
    PrefersContrast,
    ForcedColors,
    Hover,
    AnyHover,
    Pointer,
    AnyPointer,
    DisplayMode,
}

#[derive(Clone, Copy)]
enum ContainerFeatureName {
    Width(Option<RangePrefix>),
    Height(Option<RangePrefix>),
    InlineSize(Option<RangePrefix>),
    BlockSize(Option<RangePrefix>),
    AspectRatio(Option<RangePrefix>),
    Orientation,
}

impl ContainerFeatureName {
    fn parse(name: &str) -> Option<Self> {
        Some(match name.to_ascii_lowercase().as_str() {
            "width" => Self::Width(None),
            "min-width" => Self::Width(Some(RangePrefix::Min)),
            "max-width" => Self::Width(Some(RangePrefix::Max)),
            "height" => Self::Height(None),
            "min-height" => Self::Height(Some(RangePrefix::Min)),
            "max-height" => Self::Height(Some(RangePrefix::Max)),
            "inline-size" => Self::InlineSize(None),
            "min-inline-size" => Self::InlineSize(Some(RangePrefix::Min)),
            "max-inline-size" => Self::InlineSize(Some(RangePrefix::Max)),
            "block-size" => Self::BlockSize(None),
            "min-block-size" => Self::BlockSize(Some(RangePrefix::Min)),
            "max-block-size" => Self::BlockSize(Some(RangePrefix::Max)),
            "aspect-ratio" => Self::AspectRatio(None),
            "min-aspect-ratio" => Self::AspectRatio(Some(RangePrefix::Min)),
            "max-aspect-ratio" => Self::AspectRatio(Some(RangePrefix::Max)),
            "orientation" => Self::Orientation,
            _ => return None,
        })
    }
}

impl MediaFeatureName {
    fn parse(name: &str) -> Option<Self> {
        Some(match name.to_ascii_lowercase().as_str() {
            "width" => Self::Width(None),
            "min-width" => Self::Width(Some(RangePrefix::Min)),
            "max-width" => Self::Width(Some(RangePrefix::Max)),
            "height" => Self::Height(None),
            "min-height" => Self::Height(Some(RangePrefix::Min)),
            "max-height" => Self::Height(Some(RangePrefix::Max)),
            "resolution" => Self::Resolution(None),
            "min-resolution" => Self::Resolution(Some(RangePrefix::Min)),
            "max-resolution" => Self::Resolution(Some(RangePrefix::Max)),
            "color" => Self::Color(None),
            "min-color" => Self::Color(Some(RangePrefix::Min)),
            "max-color" => Self::Color(Some(RangePrefix::Max)),
            "monochrome" => Self::Monochrome(None),
            "min-monochrome" => Self::Monochrome(Some(RangePrefix::Min)),
            "max-monochrome" => Self::Monochrome(Some(RangePrefix::Max)),
            "orientation" => Self::Orientation,
            "prefers-color-scheme" => Self::PrefersColorScheme,
            "prefers-reduced-motion" => Self::PrefersReducedMotion,
            "prefers-reduced-transparency" => Self::PrefersReducedTransparency,
            "prefers-contrast" => Self::PrefersContrast,
            "forced-colors" => Self::ForcedColors,
            "hover" => Self::Hover,
            "any-hover" => Self::AnyHover,
            "pointer" => Self::Pointer,
            "any-pointer" => Self::AnyPointer,
            "display-mode" => Self::DisplayMode,
            _ => return None,
        })
    }
}

fn parse_range_feature_comparison<'i, 't>(
    input: &mut Parser<'i, 't>,
    prefix: Option<RangePrefix>,
) -> std::result::Result<Option<CssQueryComparison>, ParseError<'i, Error>> {
    if input.try_parse(Parser::expect_colon).is_ok() {
        return Ok(Some(match prefix {
            Some(RangePrefix::Min) => CssQueryComparison::GreaterThanOrEqual,
            Some(RangePrefix::Max) => CssQueryComparison::LessThanOrEqual,
            None => CssQueryComparison::Equal,
        }));
    }

    if prefix.is_some() {
        return Err(invalid_syntax(
            input.current_source_location(),
            "prefixed media range features require colon syntax",
        ));
    }

    parse_query_comparison(input).map(Some)
}

fn parse_query_comparison<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssQueryComparison, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let Token::Delim(delim) = input.next().map_err(basic)? else {
        return Err(invalid_syntax(
            location,
            "expected media feature comparison",
        ));
    };
    let delim = *delim;

    match delim {
        '<' if input.try_parse(|input| input.expect_delim('=')).is_ok() => {
            Ok(CssQueryComparison::LessThanOrEqual)
        }
        '<' => Ok(CssQueryComparison::LessThan),
        '>' if input.try_parse(|input| input.expect_delim('=')).is_ok() => {
            Ok(CssQueryComparison::GreaterThanOrEqual)
        }
        '>' => Ok(CssQueryComparison::GreaterThan),
        '=' => Ok(CssQueryComparison::Equal),
        _ => Err(invalid_syntax(
            location,
            "expected media feature comparison",
        )),
    }
}

fn parse_query_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssQueryLength, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, unit, .. } => {
            let Some(unit) = CssLengthUnit::from_css_unit(unit) else {
                return Err(unsupported_value_at(
                    location,
                    None,
                    format!("unknown media query length unit `{unit}`"),
                ));
            };
            CssQueryLength::try_new(*value, unit).ok_or_else(|| {
                unsupported_value_at(location, None, "unsupported media query length")
            })
        }
        token => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported media query length `{}`", token.to_css_string()),
        )),
    }
}

fn parse_ratio<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssRatio, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let numerator = match input.next().map_err(basic)? {
        Token::Number { value, .. } => *value,
        token => {
            return Err(unsupported_value_at(
                location,
                None,
                format!("unsupported query ratio `{}`", token.to_css_string()),
            ));
        }
    };

    input.expect_delim('/').map_err(basic)?;

    let denominator_location = input.current_source_location();
    let denominator = match input.next().map_err(basic)? {
        Token::Number { value, .. } => *value,
        token => {
            return Err(unsupported_value_at(
                denominator_location,
                None,
                format!("unsupported query ratio `{}`", token.to_css_string()),
            ));
        }
    };

    CssRatio::try_new(numerator, denominator)
        .ok_or_else(|| unsupported_value_at(location, None, "unsupported query ratio"))
}

fn parse_resolution<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssResolution, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, unit, .. } => {
            let unit = match_ignore_ascii_case! { unit,
                "dpi" => CssResolutionUnit::Dpi,
                "dpcm" => CssResolutionUnit::Dpcm,
                "dppx" => CssResolutionUnit::Dppx,
                _ => return Err(unsupported_value_at(
                    location,
                    None,
                    format!("unknown media query resolution unit `{unit}`"),
                )),
            };
            CssResolution::try_new(*value, unit).ok_or_else(|| {
                unsupported_value_at(location, None, "unsupported media query resolution")
            })
        }
        token => Err(unsupported_value_at(
            location,
            None,
            format!(
                "unsupported media query resolution `{}`",
                token.to_css_string()
            ),
        )),
    }
}

fn parse_non_negative_integer<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssNonNegativeInteger, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Number {
            int_value: Some(value),
            ..
        } => u32::try_from(*value)
            .map(CssNonNegativeInteger::new)
            .map_err(|_| unsupported_value_at(location, None, "unsupported negative integer")),
        token => Err(unsupported_value_at(
            location,
            None,
            format!(
                "unsupported media query integer `{}`",
                token.to_css_string()
            ),
        )),
    }
}

fn parse_orientation<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssOrientation, ParseError<'i, Error>> {
    parse_discrete_ident(input, "orientation", |ident| {
        match_ignore_ascii_case! { ident,
            "portrait" => Some(CssOrientation::Portrait),
            "landscape" => Some(CssOrientation::Landscape),
            _ => None,
        }
    })
}

fn parse_color_scheme_preference<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColorSchemePreference, ParseError<'i, Error>> {
    parse_discrete_ident(input, "prefers-color-scheme", |ident| {
        match_ignore_ascii_case! { ident,
            "light" => Some(CssColorSchemePreference::Light),
            "dark" => Some(CssColorSchemePreference::Dark),
            _ => None,
        }
    })
}

fn parse_reduced_motion_preference<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssReducedMotionPreference, ParseError<'i, Error>> {
    parse_discrete_ident(input, "prefers-reduced-motion", |ident| {
        match_ignore_ascii_case! { ident,
            "reduce" => Some(CssReducedMotionPreference::Reduce),
            "no-preference" => Some(CssReducedMotionPreference::NoPreference),
            _ => None,
        }
    })
}

fn parse_reduced_transparency_preference<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssReducedTransparencyPreference, ParseError<'i, Error>> {
    parse_discrete_ident(input, "prefers-reduced-transparency", |ident| {
        match_ignore_ascii_case! { ident,
            "reduce" => Some(CssReducedTransparencyPreference::Reduce),
            "no-preference" => Some(CssReducedTransparencyPreference::NoPreference),
            _ => None,
        }
    })
}

fn parse_contrast_preference<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssContrastPreference, ParseError<'i, Error>> {
    parse_discrete_ident(input, "prefers-contrast", |ident| {
        match_ignore_ascii_case! { ident,
            "no-preference" => Some(CssContrastPreference::NoPreference),
            "more" => Some(CssContrastPreference::More),
            "less" => Some(CssContrastPreference::Less),
            "custom" => Some(CssContrastPreference::Custom),
            _ => None,
        }
    })
}

fn parse_forced_colors_mode<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssForcedColorsMode, ParseError<'i, Error>> {
    parse_discrete_ident(input, "forced-colors", |ident| {
        match_ignore_ascii_case! { ident,
            "none" => Some(CssForcedColorsMode::None),
            "active" => Some(CssForcedColorsMode::Active),
            _ => None,
        }
    })
}

fn parse_hover_capability<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssHoverCapability, ParseError<'i, Error>> {
    parse_discrete_ident(input, "hover", |ident| {
        match_ignore_ascii_case! { ident,
            "none" => Some(CssHoverCapability::None),
            "hover" => Some(CssHoverCapability::Hover),
            _ => None,
        }
    })
}

fn parse_pointer_capability<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPointerCapability, ParseError<'i, Error>> {
    parse_discrete_ident(input, "pointer", |ident| {
        match_ignore_ascii_case! { ident,
            "none" => Some(CssPointerCapability::None),
            "coarse" => Some(CssPointerCapability::Coarse),
            "fine" => Some(CssPointerCapability::Fine),
            _ => None,
        }
    })
}

fn parse_display_mode<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssDisplayMode, ParseError<'i, Error>> {
    parse_discrete_ident(input, "display-mode", |ident| {
        match_ignore_ascii_case! { ident,
            "fullscreen" => Some(CssDisplayMode::Fullscreen),
            "standalone" => Some(CssDisplayMode::Standalone),
            "minimal-ui" => Some(CssDisplayMode::MinimalUi),
            "browser" => Some(CssDisplayMode::Browser),
            "picture-in-picture" => Some(CssDisplayMode::PictureInPicture),
            _ => None,
        }
    })
}

fn parse_discrete_ident<'i, 't, T>(
    input: &mut Parser<'i, 't>,
    feature: &str,
    parse: impl FnOnce(&str) -> Option<T>,
) -> std::result::Result<T, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    parse(&ident).ok_or_else(|| {
        unsupported_value_at(
            location,
            None,
            format!("unsupported {feature} value `{ident}`"),
        )
    })
}

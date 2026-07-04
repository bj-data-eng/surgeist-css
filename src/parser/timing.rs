use cssparser::{ParseError, Parser, Token, match_ignore_ascii_case};

use super::effects::parse_easing_function_arguments;
use super::values::{next_is_comma, parse_custom_ident_from_str_at};
use crate::error::{Error, basic, unsupported_value, unsupported_value_at};
use crate::syntax::*;
use crate::validation::unsupported_keyword_reason;

pub(super) fn parse_time_list<'i, 't>(
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

pub(super) fn parse_time<'i, 't>(
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

pub(super) fn parse_easing_list<'i, 't>(
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

pub(super) fn parse_easing<'i, 't>(
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

pub(super) fn parse_transition_property_list<'i, 't>(
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

pub(super) fn parse_transition_property<'i, 't>(
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

pub(super) fn parse_transition_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransitionList, ParseError<'i, Error>> {
    let mut items = Vec::new();
    loop {
        items.push(parse_single_transition(input)?);
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

pub(super) fn parse_single_transition<'i, 't>(
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

pub(super) fn parse_animation_name_list<'i, 't>(
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

pub(super) fn parse_animation_name<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationName, ParseError<'i, Error>> {
    let location = input.current_source_location();
    if let Ok(value) = input.try_parse(Parser::expect_string_cloned) {
        return CssKeyframesString::try_new(value.to_string())
            .map(CssAnimationName::String)
            .ok_or_else(|| unsupported_value_at(location, None, "animation string name is empty"));
    }

    let ident = input.expect_ident_cloned().map_err(basic)?;
    if ident.eq_ignore_ascii_case("none") {
        Ok(CssAnimationName::None)
    } else {
        parse_custom_ident_from_str_at("animation name", ident.as_ref(), location)
            .map(CssAnimationName::Custom)
    }
}

pub(super) fn parse_animation_iteration_count_list<'i, 't>(
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

pub(super) fn parse_animation_iteration_count<'i, 't>(
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

pub(super) fn parse_animation_direction_list<'i, 't>(
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

pub(super) fn parse_animation_direction<'i, 't>(
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

pub(super) fn parse_animation_fill_mode_list<'i, 't>(
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

pub(super) fn parse_animation_fill_mode<'i, 't>(
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

pub(super) fn parse_animation_play_state_list<'i, 't>(
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

pub(super) fn parse_animation_play_state<'i, 't>(
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

pub(super) fn parse_animation_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssAnimationList, ParseError<'i, Error>> {
    let mut items = Vec::new();
    loop {
        items.push(parse_single_animation(input)?);
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

pub(super) fn parse_single_animation<'i, 't>(
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

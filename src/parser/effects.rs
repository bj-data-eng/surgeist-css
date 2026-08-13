use cssparser::{ParseError, Parser, ParserInput, ToCss, Token, match_ignore_ascii_case};

use super::background::{
    parse_background_repeat, parse_background_size, parse_css_position, parse_css_position_legacy,
    parse_image_layer, parse_url,
};
use super::box_model::parse_shadow;
use super::values::{
    CalculationRoot, LengthGrammar, checked_percentage_value, next_is_comma, next_is_ident,
    parse_length_with_context, parse_length_with_context_legacy, parse_number,
    parse_typed_calculation,
};
use crate::error::{Error, basic, unsupported_value, unsupported_value_at};
use crate::syntax::*;
use crate::validation::unsupported_keyword_reason;

pub(super) fn parse_transform<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssParsedTransform, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("none"))
        .is_ok()
    {
        return Ok(CssParsedTransform::new(
            CssTransformValue::None,
            CssTransform::None,
        ));
    }
    let mut current_functions = Vec::new();
    let mut legacy_functions = Vec::new();
    while !input.is_exhausted() {
        let (current, legacy) = parse_transform_function(input)?;
        current_functions.push(current);
        legacy_functions.push(legacy);
    }
    let current = CssTransformFunctionValueList::try_new(current_functions)
        .map(CssTransformValue::Functions)
        .ok_or_else(|| unsupported_value(input, None, "transform function list is empty"))?;
    let legacy = CssTransformFunctionList::try_new(legacy_functions)
        .map(CssTransform::Functions)
        .ok_or_else(|| unsupported_value(input, None, "transform function list is empty"))?;
    Ok(CssParsedTransform::new(current, legacy))
}

fn parse_transform_function<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<(CssTransformFunctionValue, CssTransformFunction), ParseError<'i, Error>> {
    let location = input.current_source_location();
    let name = match input.next().map_err(basic)? {
        Token::Function(name) => name.clone(),
        token => return Err(location.new_unexpected_token_error::<Error>(token.clone())),
    };
    let kind = parse_transform_function_kind(input, name.as_ref())?;
    let (current, arguments) = input.parse_nested_block(|input| {
        let state = input.state();
        let authored = collect_transform_authored_tokens(input)?;
        input.reset(&state);
        let current = parse_transform_function_value(input, kind)?;
        Ok((
            current,
            CssTransformArguments::new(CssAuthoredFunctionArguments::new(authored)),
        ))
    })?;
    Ok((current, CssTransformFunction::new(kind, arguments)))
}

fn collect_transform_authored_tokens<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<String, ParseError<'i, Error>> {
    let mut value = String::new();
    while !input.is_exhausted() {
        let token = input.next().map_err(basic)?.clone();
        let token_css = match token {
            Token::Function(_) => {
                let mut css = token.to_css_string();
                css.push_str(&input.parse_nested_block(collect_transform_authored_tokens)?);
                css.push(')');
                css
            }
            Token::ParenthesisBlock => {
                let nested = input.parse_nested_block(collect_transform_authored_tokens)?;
                format!("({nested})")
            }
            _ => token.to_css_string(),
        };
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

pub(super) fn parse_transform_function_kind<'i, 't>(
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

fn parse_transform_function_value<'i, 't>(
    input: &mut Parser<'i, 't>,
    kind: CssTransformFunctionKind,
) -> std::result::Result<CssTransformFunctionValue, ParseError<'i, Error>> {
    let value = match kind {
        CssTransformFunctionKind::Matrix => {
            let components = parse_exact_comma_list(input, 6, parse_transform_number)?;
            let components = components.try_into().map_err(|_| {
                unsupported_value(input, None, "matrix() requires exactly six numbers")
            })?;
            CssTransformFunctionValue::Matrix(CssTransformMatrix::new(components))
        }
        CssTransformFunctionKind::Matrix3d => {
            let components = parse_exact_comma_list(input, 16, parse_transform_number)?;
            let components = components.try_into().map_err(|_| {
                unsupported_value(input, None, "matrix3d() requires exactly sixteen numbers")
            })?;
            CssTransformFunctionValue::Matrix3d(Box::new(CssTransformMatrix3d::new(components)))
        }
        CssTransformFunctionKind::Perspective => {
            let perspective = if input
                .try_parse(|input| input.expect_ident_matching("none"))
                .is_ok()
            {
                CssTransformPerspective::None
            } else {
                let location = input.current_source_location();
                let length = parse_transform_length(input)?;
                let length = CssTransformNonNegativeLength::try_new(length.value().clone())
                    .ok_or_else(|| {
                        unsupported_value_at(
                            location,
                            None,
                            "perspective() requires a non-negative length or none",
                        )
                    })?;
                CssTransformPerspective::Length(length)
            };
            input.expect_exhausted().map_err(basic)?;
            CssTransformFunctionValue::Perspective(perspective)
        }
        CssTransformFunctionKind::Rotate => {
            CssTransformFunctionValue::Rotate(parse_one(input, parse_transform_angle)?)
        }
        CssTransformFunctionKind::Rotate3d => {
            let x = parse_transform_number(input)?;
            input.expect_comma().map_err(basic)?;
            let y = parse_transform_number(input)?;
            input.expect_comma().map_err(basic)?;
            let z = parse_transform_number(input)?;
            input.expect_comma().map_err(basic)?;
            let angle = parse_transform_angle(input)?;
            input.expect_exhausted().map_err(basic)?;
            CssTransformFunctionValue::Rotate3d(CssTransformRotate3d::new(x, y, z, angle))
        }
        CssTransformFunctionKind::RotateX => {
            CssTransformFunctionValue::RotateX(parse_one(input, parse_transform_angle)?)
        }
        CssTransformFunctionKind::RotateY => {
            CssTransformFunctionValue::RotateY(parse_one(input, parse_transform_angle)?)
        }
        CssTransformFunctionKind::RotateZ => {
            CssTransformFunctionValue::RotateZ(parse_one(input, parse_transform_angle)?)
        }
        CssTransformFunctionKind::Scale => {
            let (x, y) = parse_one_or_two(input, parse_transform_number)?;
            CssTransformFunctionValue::Scale(CssTransformScale::new(x, y))
        }
        CssTransformFunctionKind::Scale3d => {
            let mut components = parse_exact_comma_list(input, 3, parse_transform_scale_component)?;
            let z = components.pop().ok_or_else(|| {
                unsupported_value(input, None, "scale3d() requires exactly three operands")
            })?;
            let y = components.pop().ok_or_else(|| {
                unsupported_value(input, None, "scale3d() requires exactly three operands")
            })?;
            let x = components.pop().ok_or_else(|| {
                unsupported_value(input, None, "scale3d() requires exactly three operands")
            })?;
            CssTransformFunctionValue::Scale3d(CssTransformScale3d::new(x, y, z))
        }
        CssTransformFunctionKind::ScaleX => {
            CssTransformFunctionValue::ScaleX(parse_one(input, parse_transform_number)?)
        }
        CssTransformFunctionKind::ScaleY => {
            CssTransformFunctionValue::ScaleY(parse_one(input, parse_transform_number)?)
        }
        CssTransformFunctionKind::ScaleZ => {
            CssTransformFunctionValue::ScaleZ(parse_one(input, parse_transform_scale_component)?)
        }
        CssTransformFunctionKind::Skew => {
            let (x, y) = parse_one_or_two(input, parse_transform_angle)?;
            CssTransformFunctionValue::Skew(CssTransformSkew::new(x, y))
        }
        CssTransformFunctionKind::SkewX => {
            CssTransformFunctionValue::SkewX(parse_one(input, parse_transform_angle)?)
        }
        CssTransformFunctionKind::SkewY => {
            CssTransformFunctionValue::SkewY(parse_one(input, parse_transform_angle)?)
        }
        CssTransformFunctionKind::Translate => {
            let (x, y) = parse_one_or_two(input, parse_transform_length_percentage)?;
            CssTransformFunctionValue::Translate(CssTransformTranslate::new(x, y))
        }
        CssTransformFunctionKind::Translate3d => {
            let x = parse_transform_length_percentage(input)?;
            input.expect_comma().map_err(basic)?;
            let y = parse_transform_length_percentage(input)?;
            input.expect_comma().map_err(basic)?;
            let z = parse_transform_length(input)?;
            input.expect_exhausted().map_err(basic)?;
            CssTransformFunctionValue::Translate3d(CssTransformTranslate3d::new(x, y, z))
        }
        CssTransformFunctionKind::TranslateX => CssTransformFunctionValue::TranslateX(parse_one(
            input,
            parse_transform_length_percentage,
        )?),
        CssTransformFunctionKind::TranslateY => CssTransformFunctionValue::TranslateY(parse_one(
            input,
            parse_transform_length_percentage,
        )?),
        CssTransformFunctionKind::TranslateZ => {
            CssTransformFunctionValue::TranslateZ(parse_one(input, parse_transform_length)?)
        }
    };
    Ok(value)
}

fn parse_exact_comma_list<'i, 't, T>(
    input: &mut Parser<'i, 't>,
    count: usize,
    mut parse: impl FnMut(&mut Parser<'i, 't>) -> std::result::Result<T, ParseError<'i, Error>>,
) -> std::result::Result<Vec<T>, ParseError<'i, Error>> {
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        if index != 0 {
            input.expect_comma().map_err(basic)?;
        }
        values.push(parse(input)?);
    }
    input.expect_exhausted().map_err(basic)?;
    Ok(values)
}

fn parse_one<'i, 't, T>(
    input: &mut Parser<'i, 't>,
    mut parse: impl FnMut(&mut Parser<'i, 't>) -> std::result::Result<T, ParseError<'i, Error>>,
) -> std::result::Result<T, ParseError<'i, Error>> {
    let value = parse(input)?;
    input.expect_exhausted().map_err(basic)?;
    Ok(value)
}

fn parse_one_or_two<'i, 't, T>(
    input: &mut Parser<'i, 't>,
    mut parse: impl FnMut(&mut Parser<'i, 't>) -> std::result::Result<T, ParseError<'i, Error>>,
) -> std::result::Result<(T, Option<T>), ParseError<'i, Error>> {
    let first = parse(input)?;
    let second = if input.is_exhausted() {
        None
    } else {
        input.expect_comma().map_err(basic)?;
        Some(parse(input)?)
    };
    input.expect_exhausted().map_err(basic)?;
    Ok((first, second))
}

fn parse_transform_number<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransformNumber, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Number { value, .. } => CssFiniteNumber::try_new(*value)
            .map(CssTransformNumber::Literal)
            .ok_or_else(|| unsupported_value_at(location, None, "transform number must be finite")),
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => input
            .parse_nested_block(|input| parse_typed_calculation(input, CalculationRoot::Number))
            .map(CssNumberCalculation::from_expression)
            .map(CssTransformNumber::Calculation),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_transform_percentage<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransformPercentage, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Percentage { unit_value, .. } => {
            let value = checked_percentage_value(
                location,
                *unit_value,
                "transform percentage must be finite",
            )?;
            CssFiniteNumber::try_new(value)
                .map(CssTransformPercentage::Literal)
                .ok_or_else(|| {
                    unsupported_value_at(location, None, "transform percentage must be finite")
                })
        }
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => input
            .parse_nested_block(|input| parse_typed_calculation(input, CalculationRoot::Percentage))
            .map(CssPercentageCalculation::from_expression)
            .map(CssTransformPercentage::Calculation),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_transform_scale_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransformScaleComponent, ParseError<'i, Error>> {
    let state = input.state();
    if let Ok(number) = input.try_parse(parse_transform_number) {
        return Ok(CssTransformScaleComponent::Number(number));
    }
    input.reset(&state);
    parse_transform_percentage(input).map(CssTransformScaleComponent::Percentage)
}

fn parse_transform_angle<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransformAngle, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Number { value, .. } if *value == 0.0 => Ok(CssTransformAngle::Zero),
        Token::Dimension { value, unit, .. } => {
            let unit = match unit.to_ascii_lowercase().as_str() {
                "deg" => CssAngleUnit::Degrees,
                "grad" => CssAngleUnit::Gradians,
                "rad" => CssAngleUnit::Radians,
                "turn" => CssAngleUnit::Turns,
                _ => {
                    return Err(unsupported_value_at(
                        location,
                        None,
                        format!("unsupported transform angle unit `{unit}`"),
                    ));
                }
            };
            CssAngleLiteral::try_new(*value, unit)
                .map(CssTransformAngle::Literal)
                .ok_or_else(|| {
                    unsupported_value_at(location, None, "transform angle must be finite")
                })
        }
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => input
            .parse_nested_block(|input| {
                let location = input.current_source_location();
                let expression = parse_typed_calculation(input, CalculationRoot::Angle)?;
                if expression.result_type() != CssCalculationType::Angle {
                    return Err(unsupported_value_at(
                        location,
                        None,
                        "transform angle calculation must have an angle result",
                    ));
                }
                Ok(expression)
            })
            .map(CssAngleCalculation::from_expression)
            .map(CssTransformAngle::Calculation),
        token => Err(location.new_unexpected_token_error::<Error>(token.clone())),
    }
}

fn parse_transform_length_percentage<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransformLengthPercentage, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let value = parse_length_with_context(input, LengthGrammar::Position, "translate")?;
    CssTransformLengthPercentage::try_new(value).ok_or_else(|| {
        unsupported_value_at(
            location,
            None,
            "transform translation requires a length-percentage",
        )
    })
}

fn parse_transform_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransformLength, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let value = parse_length_with_context(input, LengthGrammar::Position, "translate")?;
    CssTransformLength::try_new(value).ok_or_else(|| {
        unsupported_value_at(location, None, "transform translation requires a length")
    })
}

pub(super) fn parse_filter_function_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    name: &str,
) -> std::result::Result<CssFilterArguments, ParseError<'i, Error>> {
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
    .map(CssFilterArguments::new)
}

pub(super) fn parse_basic_shape_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    name: &str,
) -> std::result::Result<CssBasicShapeArguments, ParseError<'i, Error>> {
    parse_validated_function_arguments(input, "basic shape", |input| {
        match name.to_ascii_lowercase().as_str() {
            "circle" => validate_circle_shape(input),
            "ellipse" => validate_ellipse_shape(input),
            "inset" => validate_inset_shape(input),
            "polygon" => validate_polygon_shape(input),
            _ => false,
        }
    })
    .map(CssBasicShapeArguments::new)
}

pub(super) fn parse_easing_function_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    name: &str,
) -> std::result::Result<CssEasingArguments, ParseError<'i, Error>> {
    parse_validated_function_arguments(input, "easing function", |input| {
        match name.to_ascii_lowercase().as_str() {
            "cubic-bezier" => validate_cubic_bezier(input),
            "steps" => validate_steps(input),
            _ => false,
        }
    })
    .map(CssEasingArguments::new)
}

pub(super) fn parse_validated_function_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    context: &str,
    validate: impl for<'a, 'b> FnMut(&mut Parser<'a, 'b>) -> bool,
) -> std::result::Result<CssAuthoredFunctionArguments, ParseError<'i, Error>> {
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
    Ok(CssAuthoredFunctionArguments::new(value))
}

pub(super) fn validate_authored_function_arguments(
    value: &str,
    mut validate: impl for<'i, 't> FnMut(&mut Parser<'i, 't>) -> bool,
) -> bool {
    let mut input = ParserInput::new(value);
    let mut parser = Parser::new(&mut input);
    validate(&mut parser) && parser.is_exhausted()
}

pub(super) fn collect_authored_tokens<'i, 't>(
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

pub(super) fn consume_optional_comma<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    input.try_parse(Parser::expect_comma).is_ok()
}

pub(super) fn validate_length_sequence<'i, 't>(
    input: &mut Parser<'i, 't>,
    min: usize,
    max: usize,
) -> bool {
    let mut count = 0;
    while !input.is_exhausted() {
        if count == max
            || parse_length_with_context_legacy(input, LengthGrammar::Position, "function length")
                .is_err()
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

pub(super) fn validate_non_negative_length<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    parse_length_with_context_legacy(input, LengthGrammar::BorderWidth, "function length").is_ok()
        && input.is_exhausted()
}

pub(super) fn validate_number_or_percent<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    let parsed = match input.next() {
        Ok(Token::Number { .. } | Token::Percentage { .. }) => true,
        Ok(_) => false,
        Err(_) => false,
    };
    parsed && input.is_exhausted()
}

pub(super) fn validate_angle<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
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

pub(super) fn validate_shape_radius<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
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
        parse_length_with_context_legacy(input, LengthGrammar::BackgroundSize, "shape radius")
            .is_ok()
    }
}

pub(super) fn validate_circle_shape<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    if input
        .try_parse(|input| input.expect_ident_matching("at"))
        .is_ok()
    {
        return parse_css_position_legacy(input).is_ok() && input.is_exhausted();
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
        && parse_css_position_legacy(input).is_ok()
        && input.is_exhausted()
}

pub(super) fn validate_ellipse_shape<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    if input
        .try_parse(|input| input.expect_ident_matching("at"))
        .is_ok()
    {
        return parse_css_position_legacy(input).is_ok() && input.is_exhausted();
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
        && parse_css_position_legacy(input).is_ok()
        && input.is_exhausted()
}

pub(super) fn validate_inset_shape<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    let mut count = 0;
    while !input.is_exhausted() && !next_is_ident(input, "round") {
        if count == 4
            || parse_length_with_context_legacy(input, LengthGrammar::BackgroundSize, "inset shape")
                .is_err()
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

pub(super) fn validate_polygon_shape<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
    let mut points = 0;
    loop {
        if parse_length_with_context_legacy(input, LengthGrammar::Position, "polygon x").is_err()
            || parse_length_with_context_legacy(input, LengthGrammar::Position, "polygon y")
                .is_err()
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

pub(super) fn validate_cubic_bezier<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
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

pub(super) fn validate_steps<'i, 't>(input: &mut Parser<'i, 't>) -> bool {
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

pub(super) fn parse_translate<'i, 't>(
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
        values.push(parse_length_with_context(
            input,
            LengthGrammar::Position,
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

pub(super) fn parse_rotate<'i, 't>(
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

pub(super) fn parse_scale<'i, 't>(
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

pub(super) fn parse_filter<'i, 't>(
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

pub(super) fn parse_filter_function<'i, 't>(
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

pub(super) fn parse_clip_path<'i, 't>(
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

pub(super) fn parse_mask_list<'i, 't>(
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

pub(super) fn parse_mask_layer<'i, 't>(
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

#[cfg(test)]
mod transform_tests {
    use cssparser::{Parser, ParserInput};

    use super::*;

    fn parse(value: &str) -> Result<CssParsedTransform, ParseError<'_, Error>> {
        let mut input = ParserInput::new(value);
        Parser::new(&mut input).parse_entirely(parse_transform)
    }

    #[test]
    fn transform_parser_requires_commas_at_the_function_boundary() {
        assert!(parse("matrix(1 0 0 1 0 0)").is_err());
        assert!(parse("rotate3d(1 0 0 45deg)").is_err());
        assert!(parse("translate3d(1px 2px 3px)").is_err());
    }

    #[test]
    fn transform_parser_builds_checked_percentage_scale_and_length_only_z_values() {
        let parsed =
            parse("scale3d(1, 50%, 2) translate3d(10%, 20%, 3px)").expect("valid typed transforms");
        let (current, legacy) = parsed.into_parts();
        let CssTransformValue::Functions(functions) = current else {
            panic!("expected current transform functions");
        };
        assert!(matches!(
            &functions.functions()[0],
            CssTransformFunctionValue::Scale3d(scale)
                if matches!(scale.y(), CssTransformScaleComponent::Percentage(_))
        ));
        assert!(matches!(
            &functions.functions()[1],
            CssTransformFunctionValue::Translate3d(translation)
                if matches!(translation.z().value(), CssLength::Px(value) if value.value() == 3.0)
        ));
        let CssTransform::Functions(legacy) = legacy else {
            panic!("expected legacy transform projection");
        };
        assert_eq!(legacy.functions()[0].arguments().as_css(), "1, 50%, 2");
    }
}

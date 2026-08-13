use cssparser::{
    ParseError, Parser, ToCss, Token, color::PredefinedColorSpace as ParsedPredefinedColorSpace,
    match_ignore_ascii_case,
};
use cssparser_color::{Color as ParsedColor, DefaultColorParser, parse_color_with};

use crate::error::{
    CssFeatureId, Error, basic, invalid_color, unsupported_value_at, with_color_context,
};
use crate::syntax::*;
use crate::validation::{LengthUnitStatus, classify_length_unit, parse_global_keyword};

pub(crate) static IMPLEMENTED_SHARED_VALUES: &[CssFeatureId] = &[
    CssFeatureId::new("official.value.integer"),
    CssFeatureId::new("official.value.number"),
    CssFeatureId::new("official.value.dimension"),
    CssFeatureId::new("official.value.percentage"),
    CssFeatureId::new("official.value.length"),
    CssFeatureId::new("official.value.length-percentage"),
    CssFeatureId::new("official.value.angle"),
    CssFeatureId::new("official.value.angle-percentage"),
    CssFeatureId::new("official.value.time"),
    CssFeatureId::new("official.value.time-percentage"),
    CssFeatureId::new("official.value.frequency"),
    CssFeatureId::new("official.value.frequency-percentage"),
    CssFeatureId::new("official.value.resolution"),
    CssFeatureId::new("official.value.calc"),
];

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

pub(super) fn checked_percentage_value<'i>(
    location: cssparser::SourceLocation,
    unit_value: f32,
    non_finite_reason: impl Into<String>,
) -> std::result::Result<f32, ParseError<'i, Error>> {
    let value = unit_value * 100.0;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(unsupported_value_at(location, None, non_finite_reason))
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
    parse_length_with_context_mode(
        input,
        grammar,
        context,
        typed_length_calculation_is_current_consumer(context),
    )
}

pub(super) fn parse_length_with_context_legacy<'i, 't>(
    input: &mut Parser<'i, 't>,
    grammar: LengthGrammar,
    context: &str,
) -> std::result::Result<CssLength, ParseError<'i, Error>> {
    parse_length_with_context_mode(input, grammar, context, false)
}

fn parse_length_with_context_mode<'i, 't>(
    input: &mut Parser<'i, 't>,
    grammar: LengthGrammar,
    context: &str,
    allow_typed_calculation: bool,
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
        Token::Percentage { unit_value, .. } => {
            let value = checked_percentage_value(
                location,
                *unit_value,
                format!("unsupported non-finite {context} percentage"),
            )?;
            if grammar.requires_non_negative() && value < 0.0 {
                Err(unsupported_value_at(
                    location,
                    None,
                    format!("unsupported negative {context} percentage"),
                ))
            } else if grammar.allows_percent() {
                Ok(CssLength::percent(value))
            } else {
                Err(unsupported_value_at(
                    location,
                    None,
                    format!("unsupported {context} percentage"),
                ))
            }
        }
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
            let calc = input.parse_nested_block(|input| {
                if allow_typed_calculation {
                    parse_calc_length_with_grammar(input, grammar)
                } else {
                    parse_legacy_calc_length_with_grammar(input, grammar)
                }
            })?;
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

fn typed_length_calculation_is_current_consumer(context: &str) -> bool {
    matches!(
        context,
        "box size"
            | "inset"
            | "margin"
            | "padding"
            | "border-width"
            | "border-radius"
            | "box-shadow"
            | "box-shadow blur"
            | "gap"
            | "font-size"
            | "line-height"
            | "text-indent"
            | "vertical-align"
            | "letter-spacing"
            | "text-decoration-thickness"
            | "grid track"
            | "grid fit-content"
            | "background-size"
            | "position"
            | "outline-width"
            | "translate"
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "T2 root kinds are consumed by the staged T3 property integration"
    )
)]
pub(super) enum CalculationRoot {
    Number,
    Integer,
    Percentage,
    Length,
    Angle,
    Time,
    Frequency,
}

const CALCULATION_NESTING_LIMIT: u16 = 256;

pub(super) fn parse_typed_calculation<'i, 't>(
    input: &mut Parser<'i, 't>,
    root: CalculationRoot,
) -> std::result::Result<CssCalculationExpression, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let expression = parse_calculation_sum(input, 0)?;
    input.expect_exhausted().map_err(basic)?;
    if calculation_root_accepts(root, expression.result_type()) {
        Ok(expression)
    } else {
        Err(calculation_error(location))
    }
}

fn parse_calculation_sum<'i, 't>(
    input: &mut Parser<'i, 't>,
    depth: u16,
) -> std::result::Result<CssCalculationExpression, ParseError<'i, Error>> {
    let first = parse_calculation_product(input, depth)?;
    let mut result_type = first.result_type();
    let mut terms = vec![CssCalculationSumTerm {
        operator: None,
        expression: first,
    }];

    loop {
        let state = input.state();
        let location = input.current_source_location();
        let operator = match input.next() {
            Ok(Token::Delim('+')) => Some(CssCalculationSumOperator::Add),
            Ok(Token::Delim('-')) => Some(CssCalculationSumOperator::Subtract),
            Ok(_) | Err(_) => None,
        };
        let Some(operator) = operator else {
            input.reset(&state);
            break;
        };
        let expression = parse_calculation_product(input, depth)?;
        result_type = calculation_sum_type(result_type, expression.result_type())
            .ok_or_else(|| calculation_error(location))?;
        terms.push(CssCalculationSumTerm {
            operator: Some(operator),
            expression,
        });
    }

    if terms.len() == 1 {
        return match terms.pop() {
            Some(term) => Ok(term.expression),
            None => Err(calculation_error(input.current_source_location())),
        };
    }
    let expression = CssCalculationExpression::Sum { terms, result_type };
    validate_calculation_arithmetic(&expression, input.current_source_location())?;
    Ok(expression)
}

fn parse_calculation_product<'i, 't>(
    input: &mut Parser<'i, 't>,
    depth: u16,
) -> std::result::Result<CssCalculationExpression, ParseError<'i, Error>> {
    let first = parse_calculation_unary(input, depth)?;
    let mut result_type = first.result_type();
    let mut factors = vec![CssCalculationProductFactor {
        operator: None,
        expression: first,
    }];

    loop {
        let state = input.state();
        let location = input.current_source_location();
        let operator = match input.next() {
            Ok(Token::Delim('*')) => Some(CssCalculationProductOperator::Multiply),
            Ok(Token::Delim('/')) => Some(CssCalculationProductOperator::Divide),
            Ok(_) | Err(_) => None,
        };
        let Some(operator) = operator else {
            input.reset(&state);
            break;
        };
        let expression = parse_calculation_unary(input, depth)?;
        result_type = match operator {
            CssCalculationProductOperator::Multiply => {
                calculation_product_type(result_type, expression.result_type())
            }
            CssCalculationProductOperator::Divide => {
                if !calculation_type_is_number(expression.result_type())
                    || matches!(calculation_numeric_value(&expression), Ok(Some(value)) if value == 0.0)
                {
                    None
                } else {
                    calculation_quotient_type(result_type, expression.result_type())
                }
            }
        }
        .ok_or_else(|| calculation_error(location))?;
        factors.push(CssCalculationProductFactor {
            operator: Some(operator),
            expression,
        });
    }

    if factors.len() == 1 {
        return match factors.pop() {
            Some(factor) => Ok(factor.expression),
            None => Err(calculation_error(input.current_source_location())),
        };
    }
    let expression = CssCalculationExpression::Product {
        factors,
        result_type,
    };
    validate_calculation_arithmetic(&expression, input.current_source_location())?;
    Ok(expression)
}

fn parse_calculation_unary<'i, 't>(
    input: &mut Parser<'i, 't>,
    depth: u16,
) -> std::result::Result<CssCalculationExpression, ParseError<'i, Error>> {
    let state = input.state();
    let location = input.current_source_location();
    if matches!(input.next(), Ok(Token::Delim('-'))) {
        let operand = parse_calculation_unary(input, depth)?;
        let expression = CssCalculationExpression::Negate(Box::new(operand));
        validate_calculation_arithmetic(&expression, location)?;
        return Ok(expression);
    }
    input.reset(&state);
    parse_calculation_value(input, depth)
}

fn parse_calculation_value<'i, 't>(
    input: &mut Parser<'i, 't>,
    depth: u16,
) -> std::result::Result<CssCalculationExpression, ParseError<'i, Error>> {
    input.skip_whitespace();
    let location = input.current_source_location();
    let token_start = input.position();
    let token = input.next().map_err(basic)?.clone();
    let authored_token = input.slice_from(token_start);
    match token {
        Token::Number { value, .. } if !value.is_finite() => Err(calculation_error(location)),
        Token::Number {
            value: _,
            int_value: Some(integer),
            ..
        } if authored_token.parse::<i32>() == Ok(integer) => Ok(CssCalculationExpression::Value(
            CssCalculationValue::Integer(integer),
        )),
        Token::Number {
            int_value: Some(_), ..
        } => Err(calculation_error(location)),
        Token::Number { value, .. } => CssFiniteNumber::try_new(value)
            .map(CssCalculationValue::Number)
            .map(CssCalculationExpression::Value)
            .ok_or_else(|| calculation_error(location)),
        Token::Percentage { unit_value, .. } => {
            let value = checked_percentage_value(
                location,
                unit_value,
                "unsupported non-finite calculation percentage",
            )?;
            CssFiniteNumber::try_new(value)
                .map(CssCalculationValue::Percentage)
                .map(CssCalculationExpression::Value)
                .ok_or_else(|| calculation_error(location))
        }
        Token::Dimension { value, .. } if !value.is_finite() => Err(calculation_error(location)),
        Token::Dimension { value, unit, .. } => parse_calculation_dimension(value, &unit, location),
        Token::ParenthesisBlock => {
            let nested_depth = checked_calculation_depth(depth, location)?;
            let operand =
                input.parse_nested_block(|input| parse_calculation_sum(input, nested_depth))?;
            Ok(CssCalculationExpression::Group(Box::new(operand)))
        }
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            let nested_depth = checked_calculation_depth(depth, location)?;
            let operand =
                input.parse_nested_block(|input| parse_calculation_sum(input, nested_depth))?;
            Ok(CssCalculationExpression::NestedCalc(Box::new(operand)))
        }
        _ => Err(calculation_error(location)),
    }
}

fn parse_calculation_dimension<'i>(
    value: f32,
    unit: &str,
    location: cssparser::SourceLocation,
) -> std::result::Result<CssCalculationExpression, ParseError<'i, Error>> {
    let parsed = match classify_length_unit(unit) {
        LengthUnitStatus::Supported(unit) => {
            CssLengthDimension::try_new(value, unit).map(CssCalculationValue::Length)
        }
        LengthUnitStatus::Unknown if unit.eq_ignore_ascii_case("deg") => {
            CssAngleLiteral::try_new(value, CssAngleUnit::Degrees).map(CssCalculationValue::Angle)
        }
        LengthUnitStatus::Unknown if unit.eq_ignore_ascii_case("grad") => {
            CssAngleLiteral::try_new(value, CssAngleUnit::Gradians).map(CssCalculationValue::Angle)
        }
        LengthUnitStatus::Unknown if unit.eq_ignore_ascii_case("rad") => {
            CssAngleLiteral::try_new(value, CssAngleUnit::Radians).map(CssCalculationValue::Angle)
        }
        LengthUnitStatus::Unknown if unit.eq_ignore_ascii_case("turn") => {
            CssAngleLiteral::try_new(value, CssAngleUnit::Turns).map(CssCalculationValue::Angle)
        }
        LengthUnitStatus::Unknown if unit.eq_ignore_ascii_case("s") => {
            CssDelayLiteral::try_new(value, CssTimeUnit::Seconds).map(CssCalculationValue::Time)
        }
        LengthUnitStatus::Unknown if unit.eq_ignore_ascii_case("ms") => {
            CssDelayLiteral::try_new(value, CssTimeUnit::Milliseconds)
                .map(CssCalculationValue::Time)
        }
        LengthUnitStatus::Unknown if unit.eq_ignore_ascii_case("hz") => {
            CssFrequencyLiteral::try_new(value, CssFrequencyUnit::Hertz)
                .map(CssCalculationValue::Frequency)
        }
        LengthUnitStatus::Unknown if unit.eq_ignore_ascii_case("khz") => {
            CssFrequencyLiteral::try_new(value, CssFrequencyUnit::Kilohertz)
                .map(CssCalculationValue::Frequency)
        }
        LengthUnitStatus::Unknown => None,
    };
    parsed
        .map(CssCalculationExpression::Value)
        .ok_or_else(|| calculation_error(location))
}

fn checked_calculation_depth<'i>(
    depth: u16,
    location: cssparser::SourceLocation,
) -> std::result::Result<u16, ParseError<'i, Error>> {
    if depth >= CALCULATION_NESTING_LIMIT {
        Err(calculation_error(location))
    } else {
        Ok(depth + 1)
    }
}

const fn calculation_root_accepts(root: CalculationRoot, result_type: CssCalculationType) -> bool {
    match root {
        CalculationRoot::Number => matches!(
            result_type,
            CssCalculationType::Integer | CssCalculationType::Number
        ),
        CalculationRoot::Integer => matches!(result_type, CssCalculationType::Integer),
        CalculationRoot::Percentage => matches!(result_type, CssCalculationType::Percentage),
        CalculationRoot::Length => matches!(
            result_type,
            CssCalculationType::Length
                | CssCalculationType::Percentage
                | CssCalculationType::LengthPercentage
        ),
        CalculationRoot::Angle => matches!(
            result_type,
            CssCalculationType::Angle
                | CssCalculationType::Percentage
                | CssCalculationType::AnglePercentage
        ),
        CalculationRoot::Time => matches!(
            result_type,
            CssCalculationType::Time
                | CssCalculationType::Percentage
                | CssCalculationType::TimePercentage
        ),
        CalculationRoot::Frequency => matches!(
            result_type,
            CssCalculationType::Frequency
                | CssCalculationType::Percentage
                | CssCalculationType::FrequencyPercentage
        ),
    }
}

const fn calculation_type_is_number(result_type: CssCalculationType) -> bool {
    matches!(
        result_type,
        CssCalculationType::Integer | CssCalculationType::Number
    )
}

fn calculation_sum_type(
    left: CssCalculationType,
    right: CssCalculationType,
) -> Option<CssCalculationType> {
    if left == right {
        return Some(left);
    }
    match (left, right) {
        (CssCalculationType::Integer, CssCalculationType::Number)
        | (CssCalculationType::Number, CssCalculationType::Integer) => {
            Some(CssCalculationType::Number)
        }
        (CssCalculationType::Length, CssCalculationType::Percentage)
        | (CssCalculationType::Percentage, CssCalculationType::Length)
        | (CssCalculationType::LengthPercentage, CssCalculationType::Length)
        | (CssCalculationType::Length, CssCalculationType::LengthPercentage)
        | (CssCalculationType::LengthPercentage, CssCalculationType::Percentage)
        | (CssCalculationType::Percentage, CssCalculationType::LengthPercentage) => {
            Some(CssCalculationType::LengthPercentage)
        }
        (CssCalculationType::Angle, CssCalculationType::Percentage)
        | (CssCalculationType::Percentage, CssCalculationType::Angle)
        | (CssCalculationType::AnglePercentage, CssCalculationType::Angle)
        | (CssCalculationType::Angle, CssCalculationType::AnglePercentage)
        | (CssCalculationType::AnglePercentage, CssCalculationType::Percentage)
        | (CssCalculationType::Percentage, CssCalculationType::AnglePercentage) => {
            Some(CssCalculationType::AnglePercentage)
        }
        (CssCalculationType::Time, CssCalculationType::Percentage)
        | (CssCalculationType::Percentage, CssCalculationType::Time)
        | (CssCalculationType::TimePercentage, CssCalculationType::Time)
        | (CssCalculationType::Time, CssCalculationType::TimePercentage)
        | (CssCalculationType::TimePercentage, CssCalculationType::Percentage)
        | (CssCalculationType::Percentage, CssCalculationType::TimePercentage) => {
            Some(CssCalculationType::TimePercentage)
        }
        (CssCalculationType::Frequency, CssCalculationType::Percentage)
        | (CssCalculationType::Percentage, CssCalculationType::Frequency)
        | (CssCalculationType::FrequencyPercentage, CssCalculationType::Frequency)
        | (CssCalculationType::Frequency, CssCalculationType::FrequencyPercentage)
        | (CssCalculationType::FrequencyPercentage, CssCalculationType::Percentage)
        | (CssCalculationType::Percentage, CssCalculationType::FrequencyPercentage) => {
            Some(CssCalculationType::FrequencyPercentage)
        }
        _ => None,
    }
}

const fn calculation_product_type(
    left: CssCalculationType,
    right: CssCalculationType,
) -> Option<CssCalculationType> {
    match (
        calculation_type_is_number(left),
        calculation_type_is_number(right),
    ) {
        (true, true)
            if matches!(left, CssCalculationType::Number)
                || matches!(right, CssCalculationType::Number) =>
        {
            Some(CssCalculationType::Number)
        }
        (true, true) => Some(CssCalculationType::Integer),
        (true, false) => Some(right),
        (false, true) => Some(left),
        (false, false) => None,
    }
}

const fn calculation_quotient_type(
    numerator: CssCalculationType,
    denominator: CssCalculationType,
) -> Option<CssCalculationType> {
    if !calculation_type_is_number(denominator) {
        None
    } else if calculation_type_is_number(numerator) {
        Some(CssCalculationType::Number)
    } else {
        Some(numerator)
    }
}

fn validate_calculation_arithmetic<'i>(
    expression: &CssCalculationExpression,
    location: cssparser::SourceLocation,
) -> std::result::Result<(), ParseError<'i, Error>> {
    match calculation_numeric_value(expression) {
        Ok(_) => Ok(()),
        Err(()) => Err(calculation_error(location)),
    }
}

fn calculation_numeric_value(expression: &CssCalculationExpression) -> Result<Option<f32>, ()> {
    match expression {
        CssCalculationExpression::Value(CssCalculationValue::Integer(value)) => {
            Ok(Some(*value as f32))
        }
        CssCalculationExpression::Value(CssCalculationValue::Number(value)) => {
            Ok(Some(value.value()))
        }
        CssCalculationExpression::Value(
            CssCalculationValue::Percentage(_)
            | CssCalculationValue::Length(_)
            | CssCalculationValue::Angle(_)
            | CssCalculationValue::Time(_)
            | CssCalculationValue::Frequency(_),
        ) => Ok(None),
        CssCalculationExpression::Sum { terms, .. } => {
            let mut value = None;
            for term in terms {
                let term_value = calculation_numeric_value(&term.expression)?;
                value = match (value, term_value, term.operator) {
                    (None, next, None) => next,
                    (Some(current), Some(next), Some(CssCalculationSumOperator::Add)) => {
                        Some(current + next)
                    }
                    (Some(current), Some(next), Some(CssCalculationSumOperator::Subtract)) => {
                        Some(current - next)
                    }
                    _ => None,
                };
                if matches!(value, Some(value) if !value.is_finite()) {
                    return Err(());
                }
            }
            Ok(value)
        }
        CssCalculationExpression::Product { factors, .. } => {
            let mut value = None;
            for factor in factors {
                let factor_value = calculation_numeric_value(&factor.expression)?;
                value = match (value, factor_value, factor.operator) {
                    (None, next, None) => next,
                    (Some(current), Some(next), Some(CssCalculationProductOperator::Multiply)) => {
                        Some(current * next)
                    }
                    (Some(_), Some(0.0), Some(CssCalculationProductOperator::Divide)) => {
                        return Err(());
                    }
                    (Some(current), Some(next), Some(CssCalculationProductOperator::Divide)) => {
                        Some(current / next)
                    }
                    _ => None,
                };
                if matches!(value, Some(value) if !value.is_finite()) {
                    return Err(());
                }
            }
            Ok(value)
        }
        CssCalculationExpression::Negate(operand) => {
            let value = calculation_numeric_value(operand)?.map(|value| -value);
            if matches!(value, Some(value) if !value.is_finite()) {
                Err(())
            } else {
                Ok(value)
            }
        }
        CssCalculationExpression::Group(operand)
        | CssCalculationExpression::NestedCalc(operand) => calculation_numeric_value(operand),
    }
}

fn calculation_error<'i>(location: cssparser::SourceLocation) -> ParseError<'i, Error> {
    unsupported_value_at(location, None, "invalid typed calculation")
}

pub(super) fn parse_calc_length_with_grammar<'i, 't>(
    input: &mut Parser<'i, 't>,
    grammar: LengthGrammar,
) -> std::result::Result<CssCalcLength, ParseError<'i, Error>> {
    let location = input.current_source_location();
    if let Ok(legacy) =
        input.try_parse(|input| parse_legacy_calc_length_with_grammar(input, grammar))
    {
        return Ok(legacy);
    }

    let expression = parse_typed_calculation(input, CalculationRoot::Length)?;
    if !grammar.allows_calc_percent()
        && matches!(
            expression.result_type(),
            CssCalculationType::Percentage | CssCalculationType::LengthPercentage
        )
    {
        return Err(calculation_error(location));
    }
    Ok(CssCalcLength::Typed(CssLengthCalculation::from_expression(
        expression,
    )))
}

pub(super) fn parse_legacy_calc_length_with_grammar<'i, 't>(
    input: &mut Parser<'i, 't>,
    grammar: LengthGrammar,
) -> std::result::Result<CssCalcLength, ParseError<'i, Error>> {
    let first = CssCalcLengthTerm::add(parse_calc_component(input, grammar)?);
    let mut terms = Vec::new();

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
        Token::Percentage { unit_value, .. } => {
            let value = checked_percentage_value(
                location,
                *unit_value,
                "unsupported non-finite calc percentage",
            )?;
            if grammar.requires_non_negative() && value < 0.0 {
                Err(unsupported_value_at(
                    location,
                    None,
                    "unsupported negative calc percentage",
                ))
            } else if grammar.allows_calc_percent() {
                Ok(CssCalcLength::percent(value))
            } else {
                Err(unsupported_value_at(
                    location,
                    None,
                    "unsupported calc percentage",
                ))
            }
        }
        Token::Number { value, .. } if *value == 0.0 => Ok(CssCalcLength::px(0.0)),
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            input.parse_nested_block(|input| parse_legacy_calc_length_with_grammar(input, grammar))
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
        Err(unsupported_value_at(
            location,
            None,
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
    parse_color_inner(input).map_err(|error| with_color_context(error, None))
}

fn parse_color_inner<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColor, ParseError<'i, Error>> {
    if let Ok(color) = input.try_parse(parse_relative_color) {
        return Ok(color);
    }
    if let Ok(color) = input.try_parse(parse_color_mix) {
        return Ok(color);
    }
    if let Ok(color) = input.try_parse(parse_absolute_color_with_cssparser_color) {
        return Ok(color);
    }
    if let Ok(color) = input.try_parse(parse_system_color) {
        return Ok(color);
    }
    Err(invalid_color(input.current_source_location(), None))
}

fn parse_relative_color<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColor, ParseError<'i, Error>> {
    let state = input.state();
    let location = input.current_source_location();
    let token = input.next().map_err(basic)?.clone();
    match token {
        Token::Function(name) => {
            let Some(function) = relative_color_function_from_name(&name) else {
                input.reset(&state);
                return Err(location.new_unexpected_token_error::<Error>(Token::Function(name)));
            };
            input
                .parse_nested_block(|input| parse_relative_color_arguments(input, function))
                .map(CssColor::Relative)
        }
        token => {
            input.reset(&state);
            Err(location.new_unexpected_token_error::<Error>(token))
        }
    }
}

#[derive(Clone, Copy)]
enum RelativeColorFunction {
    Rgb,
    Hsl,
    Hwb,
    Lab,
    Lch,
    Oklab,
    Oklch,
    Color,
}

fn relative_color_function_from_name(name: &str) -> Option<RelativeColorFunction> {
    let function = match_ignore_ascii_case! { name,
        "rgb" | "rgba" => RelativeColorFunction::Rgb,
        "hsl" | "hsla" => RelativeColorFunction::Hsl,
        "hwb" => RelativeColorFunction::Hwb,
        "lab" => RelativeColorFunction::Lab,
        "lch" => RelativeColorFunction::Lch,
        "oklab" => RelativeColorFunction::Oklab,
        "oklch" => RelativeColorFunction::Oklch,
        "color" => RelativeColorFunction::Color,
        _ => return None,
    };
    Some(function)
}

fn parse_relative_color_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
    function: RelativeColorFunction,
) -> std::result::Result<CssRelativeColor, ParseError<'i, Error>> {
    let location = input.current_source_location();
    input.expect_ident_matching("from").map_err(basic)?;
    let source = parse_color_inner(input)?;
    let function = match function {
        RelativeColorFunction::Rgb => CssRelativeColorFunction::Rgb,
        RelativeColorFunction::Hsl => CssRelativeColorFunction::Hsl,
        RelativeColorFunction::Hwb => CssRelativeColorFunction::Hwb,
        RelativeColorFunction::Lab => CssRelativeColorFunction::Lab,
        RelativeColorFunction::Lch => CssRelativeColorFunction::Lch,
        RelativeColorFunction::Oklab => CssRelativeColorFunction::Oklab,
        RelativeColorFunction::Oklch => CssRelativeColorFunction::Oklch,
        RelativeColorFunction::Color => {
            CssRelativeColorFunction::Color(parse_relative_predefined_color_space(input)?)
        }
    };

    let mut components = Vec::with_capacity(function.component_count());
    for _ in 0..function.component_count() {
        components.push(parse_color_component_expression(input)?);
    }

    let alpha = if input.try_parse(|input| input.expect_delim('/')).is_ok() {
        Some(parse_color_component_expression(input)?)
    } else {
        None
    };

    input.expect_exhausted().map_err(basic)?;
    CssRelativeColor::try_new(function, source, components, alpha).ok_or_else(|| {
        unsupported_value_at(location, None, "unsupported relative color component count")
    })
}

fn parse_relative_predefined_color_space<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPredefinedColorSpace, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    let color_space = match_ignore_ascii_case! { &ident,
        "srgb" => CssPredefinedColorSpace::Srgb,
        "srgb-linear" => CssPredefinedColorSpace::SrgbLinear,
        "display-p3" => CssPredefinedColorSpace::DisplayP3,
        "display-p3-linear" => CssPredefinedColorSpace::DisplayP3Linear,
        "a98-rgb" => CssPredefinedColorSpace::A98Rgb,
        "prophoto-rgb" => CssPredefinedColorSpace::ProphotoRgb,
        "rec2020" => CssPredefinedColorSpace::Rec2020,
        "xyz" => CssPredefinedColorSpace::XyzD65,
        "xyz-d50" => CssPredefinedColorSpace::XyzD50,
        "xyz-d65" => CssPredefinedColorSpace::XyzD65,
        _ => return Err(unsupported_value_at(
            location,
            None,
            format!("unsupported relative color space `{ident}`"),
        )),
    };
    Ok(color_space)
}

fn parse_color_component_expression<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColorComponentExpression, ParseError<'i, Error>> {
    input.skip_whitespace();
    let start = input.position();
    consume_color_component_expression(input)?;
    let authored = CssAuthoredDeclarationValue::new(input.slice_from(start).trim_end());
    Ok(CssColorComponentExpression::new(authored, Vec::new()))
}

fn consume_color_component_expression<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<(), ParseError<'i, Error>> {
    let location = input.current_source_location();
    let token = input.next().map_err(basic)?.clone();
    if token.is_parse_error() {
        return Err(input.new_unexpected_token_error(token));
    }
    match token {
        Token::Ident(_)
        | Token::Number { .. }
        | Token::Percentage { .. }
        | Token::Dimension { .. } => Ok(()),
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            input.parse_nested_block(parse_color_component_calc_expression)
        }
        Token::Function(name) => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported relative color component function `{name}`"),
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token)),
    }
}

fn parse_color_component_calc_expression<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<(), ParseError<'i, Error>> {
    parse_color_component_calc_operand(input)?;
    while !input.is_exhausted() {
        parse_color_component_calc_operator(input)?;
        parse_color_component_calc_operand(input)?;
    }
    Ok(())
}

fn parse_color_component_calc_operator<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<(), ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Delim('+') | Token::Delim('-') | Token::Delim('*') | Token::Delim('/') => Ok(()),
        token => Err(unsupported_value_at(
            location,
            None,
            format!(
                "expected relative color calc operator, got `{}`",
                token.to_css_string()
            ),
        )),
    }
}

fn parse_color_component_calc_operand<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<(), ParseError<'i, Error>> {
    let location = input.current_source_location();
    let token = input.next().map_err(basic)?.clone();
    if token.is_parse_error() {
        return Err(input.new_unexpected_token_error(token));
    }
    match token {
        Token::Ident(_)
        | Token::Number { .. }
        | Token::Percentage { .. }
        | Token::Dimension { .. } => Ok(()),
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            input.parse_nested_block(parse_color_component_calc_expression)
        }
        Token::Function(name) => Err(unsupported_value_at(
            location,
            None,
            format!("unsupported relative color calc function `{name}`"),
        )),
        token => Err(location.new_unexpected_token_error::<Error>(token)),
    }
}

fn parse_color_mix<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColor, ParseError<'i, Error>> {
    let state = input.state();
    let location = input.current_source_location();
    let token = input.next().map_err(basic)?.clone();
    match token {
        Token::Function(name) if name.eq_ignore_ascii_case("color-mix") => input
            .parse_nested_block(parse_color_mix_arguments)
            .map(CssColor::ColorMix),
        token => {
            input.reset(&state);
            Err(location.new_unexpected_token_error::<Error>(token))
        }
    }
}

fn parse_color_mix_arguments<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColorMix, ParseError<'i, Error>> {
    input.expect_ident_matching("in").map_err(basic)?;
    let interpolation = parse_color_mix_interpolation_method(input)?;
    input.expect_comma().map_err(basic)?;
    let left = parse_color_mix_component(input)?;
    input.expect_comma().map_err(basic)?;
    let right = parse_color_mix_component(input)?;

    Ok(CssColorMix::new(interpolation, left, right))
}

fn parse_color_mix_interpolation_method<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColorInterpolationMethod, ParseError<'i, Error>> {
    let space = parse_color_mix_interpolation_space(input)?;
    let hue = input.try_parse(parse_color_mix_hue_interpolation).ok();
    Ok(CssColorInterpolationMethod::new(space, hue))
}

fn parse_color_mix_interpolation_space<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColorInterpolationSpace, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    let space = match_ignore_ascii_case! { &ident,
        "srgb" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::Srgb),
        "srgb-linear" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::SrgbLinear),
        "display-p3" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::DisplayP3),
        "display-p3-linear" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::DisplayP3Linear),
        "a98-rgb" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::A98Rgb),
        "prophoto-rgb" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::ProphotoRgb),
        "rec2020" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::Rec2020),
        "xyz" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::XyzD65),
        "xyz-d50" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::XyzD50),
        "xyz-d65" => CssColorInterpolationSpace::Predefined(CssPredefinedColorSpace::XyzD65),
        "hsl" => CssColorInterpolationSpace::Hsl,
        "hwb" => CssColorInterpolationSpace::Hwb,
        "lab" => CssColorInterpolationSpace::Lab,
        "lch" => CssColorInterpolationSpace::Lch,
        "oklab" => CssColorInterpolationSpace::Oklab,
        "oklch" => CssColorInterpolationSpace::Oklch,
        _ => return Err(unsupported_value_at(
            location,
            None,
            format!("unsupported color-mix interpolation space `{ident}`"),
        )),
    };
    Ok(space)
}

fn parse_color_mix_hue_interpolation<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssHueInterpolationMethod, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    let hue = match_ignore_ascii_case! { &ident,
        "shorter" => CssHueInterpolationMethod::Shorter,
        "longer" => CssHueInterpolationMethod::Longer,
        "increasing" => CssHueInterpolationMethod::Increasing,
        "decreasing" => CssHueInterpolationMethod::Decreasing,
        _ => return Err(unsupported_value_at(
            location,
            None,
            format!("unsupported color-mix hue interpolation method `{ident}`"),
        )),
    };
    input.expect_ident_matching("hue").map_err(basic)?;
    Ok(hue)
}

fn parse_color_mix_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColorMixComponent, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let color = parse_color_inner(input)?;
    let percentage = input.try_parse(parse_color_mix_percentage).ok();
    CssColorMixComponent::try_new(color, percentage).ok_or_else(|| {
        unsupported_value_at(location, None, "unsupported color-mix component percentage")
    })
}

fn parse_color_mix_percentage<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<f32, ParseError<'i, Error>> {
    input
        .expect_percentage()
        .map(|percentage| percentage * 100.0)
        .map_err(basic)
}

fn parse_absolute_color_with_cssparser_color<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColor, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match parse_color_with(&DefaultColorParser, input) {
        Ok(parsed) => map_parsed_color(parsed, location),
        Err(_) => Err(invalid_color(location, None)),
    }
}

fn parse_system_color<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssColor, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    let color = match_ignore_ascii_case! { &ident,
        "canvas" => CssSystemColor::Canvas,
        "canvastext" => CssSystemColor::CanvasText,
        "linktext" => CssSystemColor::LinkText,
        "visitedtext" => CssSystemColor::VisitedText,
        "activetext" => CssSystemColor::ActiveText,
        "buttonface" => CssSystemColor::ButtonFace,
        "buttontext" => CssSystemColor::ButtonText,
        "buttonborder" => CssSystemColor::ButtonBorder,
        "field" => CssSystemColor::Field,
        "fieldtext" => CssSystemColor::FieldText,
        "highlight" => CssSystemColor::Highlight,
        "highlighttext" => CssSystemColor::HighlightText,
        "mark" => CssSystemColor::Mark,
        "marktext" => CssSystemColor::MarkText,
        "graytext" => CssSystemColor::GrayText,
        "selecteditem" => CssSystemColor::SelectedItem,
        "selecteditemtext" => CssSystemColor::SelectedItemText,
        "accentcolor" => CssSystemColor::AccentColor,
        "accentcolortext" => CssSystemColor::AccentColorText,
        _ => return Err(unsupported_value_at(
            location,
            None,
            format!("unsupported system color `{ident}`"),
        )),
    };
    Ok(CssColor::System(color))
}

fn map_parsed_color<'i>(
    parsed: ParsedColor,
    location: cssparser::SourceLocation,
) -> std::result::Result<CssColor, ParseError<'i, Error>> {
    let color = match parsed {
        ParsedColor::CurrentColor => CssColor::CurrentColor,
        ParsedColor::Rgba(color) => CssColor::Rgba(
            CssRgbaColor::try_new(color.red, color.green, color.blue, color.alpha)
                .ok_or_else(|| invalid_color_component(location))?,
        ),
        ParsedColor::Hsl(color) => CssColor::Hsl(
            CssHslColor::try_new(color.hue, color.saturation, color.lightness, color.alpha)
                .ok_or_else(|| invalid_color_component(location))?,
        ),
        ParsedColor::Hwb(color) => CssColor::Hwb(
            CssHwbColor::try_new(color.hue, color.whiteness, color.blackness, color.alpha)
                .ok_or_else(|| invalid_color_component(location))?,
        ),
        ParsedColor::Lab(color) => CssColor::Lab(
            CssLabColor::try_new(color.lightness, color.a, color.b, color.alpha)
                .ok_or_else(|| invalid_color_component(location))?,
        ),
        ParsedColor::Lch(color) => CssColor::Lch(
            CssLchColor::try_new(color.lightness, color.chroma, color.hue, color.alpha)
                .ok_or_else(|| invalid_color_component(location))?,
        ),
        ParsedColor::Oklab(color) => CssColor::Oklab(
            CssLabColor::try_new(color.lightness, color.a, color.b, color.alpha)
                .ok_or_else(|| invalid_color_component(location))?,
        ),
        ParsedColor::Oklch(color) => CssColor::Oklch(
            CssLchColor::try_new(color.lightness, color.chroma, color.hue, color.alpha)
                .ok_or_else(|| invalid_color_component(location))?,
        ),
        ParsedColor::ColorFunction(color) => CssColor::ColorFunction(
            CssColorFunction::try_new(
                map_predefined_color_space(color.color_space),
                [color.c1, color.c2, color.c3],
                color.alpha,
            )
            .ok_or_else(|| invalid_color_component(location))?,
        ),
    };
    Ok(color)
}

fn invalid_color_component<'i>(location: cssparser::SourceLocation) -> ParseError<'i, Error> {
    invalid_color(location, Some("component"))
}

fn map_predefined_color_space(color_space: ParsedPredefinedColorSpace) -> CssPredefinedColorSpace {
    match color_space {
        ParsedPredefinedColorSpace::Srgb => CssPredefinedColorSpace::Srgb,
        ParsedPredefinedColorSpace::SrgbLinear => CssPredefinedColorSpace::SrgbLinear,
        ParsedPredefinedColorSpace::DisplayP3 => CssPredefinedColorSpace::DisplayP3,
        ParsedPredefinedColorSpace::DisplayP3Linear => CssPredefinedColorSpace::DisplayP3Linear,
        ParsedPredefinedColorSpace::A98Rgb => CssPredefinedColorSpace::A98Rgb,
        ParsedPredefinedColorSpace::ProphotoRgb => CssPredefinedColorSpace::ProphotoRgb,
        ParsedPredefinedColorSpace::Rec2020 => CssPredefinedColorSpace::Rec2020,
        ParsedPredefinedColorSpace::XyzD50 => CssPredefinedColorSpace::XyzD50,
        ParsedPredefinedColorSpace::XyzD65 => CssPredefinedColorSpace::XyzD65,
    }
}

#[cfg(test)]
mod typed_calculation_tests {
    use cssparser::{Parser, ParserInput};

    use super::*;
    use crate::error::{CssErrorCode, from_parse_error};

    fn parse(
        source: &str,
        root: CalculationRoot,
    ) -> Result<CssCalculationExpression, crate::Error> {
        let mut input = ParserInput::new(source);
        let mut parser = Parser::new(&mut input);
        parser
            .parse_entirely(|input| parse_typed_calculation(input, root))
            .map_err(|error| from_parse_error(source, error))
    }

    #[test]
    fn typed_root_parser_preserves_all_compound_node_kinds_and_precedence() {
        let number = parse("1 + 6 / 2", CalculationRoot::Number).unwrap();
        assert_eq!(number.result_type(), CssCalculationType::Number);
        let CssCalculationExpressionRef::Sum(sum) = number.as_ref() else {
            panic!("expected number sum");
        };
        assert_eq!(sum.len(), 2);
        assert_eq!(sum.term(0).unwrap().operator(), None);
        assert_eq!(
            sum.term(1).unwrap().operator(),
            Some(CssCalculationSumOperator::Add)
        );
        let CssCalculationExpressionRef::Product(quotient) = sum.term(1).unwrap().expression()
        else {
            panic!("division must bind inside the sum");
        };
        assert_eq!(quotient.len(), 2);
        assert_eq!(quotient.factor(0).unwrap().operator(), None);
        assert_eq!(
            quotient.factor(1).unwrap().operator(),
            Some(CssCalculationProductOperator::Divide)
        );

        let integer = parse("1 + 2 * 3", CalculationRoot::Integer).unwrap();
        assert_eq!(integer.result_type(), CssCalculationType::Integer);
        let CssCalculationExpressionRef::Sum(integer_sum) = integer.as_ref() else {
            panic!("expected integer sum");
        };
        let CssCalculationExpressionRef::Product(integer_product) =
            integer_sum.term(1).unwrap().expression()
        else {
            panic!("expected integer product");
        };
        assert_eq!(
            integer_product.factor(1).unwrap().operator(),
            Some(CssCalculationProductOperator::Multiply)
        );
        assert!(integer_product.factor(integer_product.len()).is_none());

        let percentage = parse("10% - 20%", CalculationRoot::Percentage).unwrap();
        assert_eq!(percentage.result_type(), CssCalculationType::Percentage);
        let CssCalculationExpressionRef::Sum(percentage_sum) = percentage.as_ref() else {
            panic!("expected percentage sum");
        };
        assert_eq!(
            percentage_sum.term(1).unwrap().operator(),
            Some(CssCalculationSumOperator::Subtract)
        );
        assert!(percentage_sum.term(percentage_sum.len()).is_none());

        let length = parse("1px + (2em * 3)", CalculationRoot::Length).unwrap();
        let CssCalculationExpressionRef::Sum(sum) = length.as_ref() else {
            panic!("expected length sum");
        };
        let CssCalculationExpressionRef::Group(group) = sum.term(1).unwrap().expression() else {
            panic!("expected retained authored group");
        };
        assert!(matches!(
            group.operand(),
            CssCalculationExpressionRef::Product(_)
        ));

        let angle = parse("1deg + calc(2turn)", CalculationRoot::Angle).unwrap();
        assert_eq!(angle.result_type(), CssCalculationType::Angle);
        let CssCalculationExpressionRef::Sum(sum) = angle.as_ref() else {
            panic!("expected angle sum");
        };
        let CssCalculationExpressionRef::NestedCalc(nested) = sum.term(1).unwrap().expression()
        else {
            panic!("expected retained nested calc");
        };
        assert!(matches!(
            nested.operand(),
            CssCalculationExpressionRef::Value(CssCalculationValueRef::Angle(value))
                if value.value() == 2.0 && value.unit() == CssAngleUnit::Turns
        ));

        let time = parse("1s + -(2ms)", CalculationRoot::Time).unwrap();
        assert_eq!(time.result_type(), CssCalculationType::Time);
        let CssCalculationExpressionRef::Sum(sum) = time.as_ref() else {
            panic!("expected time sum");
        };
        let CssCalculationExpressionRef::Negate(negate) = sum.term(1).unwrap().expression() else {
            panic!("expected retained authored negation");
        };
        assert!(matches!(
            negate.operand(),
            CssCalculationExpressionRef::Group(_)
        ));

        let frequency = parse("1khz / 2", CalculationRoot::Frequency).unwrap();
        assert_eq!(frequency.result_type(), CssCalculationType::Frequency);
        assert!(matches!(
            frequency.as_ref(),
            CssCalculationExpressionRef::Product(_)
        ));
    }

    #[test]
    fn typed_root_parser_promotes_only_compatible_percentage_dimensions() {
        for (root, expected) in [
            (
                CalculationRoot::Length,
                CssCalculationType::LengthPercentage,
            ),
            (CalculationRoot::Angle, CssCalculationType::AnglePercentage),
            (CalculationRoot::Time, CssCalculationType::TimePercentage),
            (
                CalculationRoot::Frequency,
                CssCalculationType::FrequencyPercentage,
            ),
        ] {
            let unit = match root {
                CalculationRoot::Length => "px",
                CalculationRoot::Angle => "deg",
                CalculationRoot::Time => "s",
                CalculationRoot::Frequency => "hz",
                CalculationRoot::Number
                | CalculationRoot::Integer
                | CalculationRoot::Percentage => unreachable!("test-owned root table"),
            };
            let expression = parse(&format!("1{unit} + 2%"), root).unwrap();
            assert_eq!(expression.result_type(), expected);
        }
    }

    #[test]
    fn typed_root_parser_rejects_invalid_dimensions_divisors_arithmetic_and_consumption() {
        let cases = [
            (
                "1px + 2deg",
                CalculationRoot::Length,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1px * 2em",
                CalculationRoot::Length,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1px / 2em",
                CalculationRoot::Length,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1px / 0",
                CalculationRoot::Length,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1px / -0",
                CalculationRoot::Length,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1px / calc(1 - 1)",
                CalculationRoot::Length,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "3.4e38 * 2",
                CalculationRoot::Number,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1e999",
                CalculationRoot::Number,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "2147483648",
                CalculationRoot::Integer,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1e999%",
                CalculationRoot::Percentage,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1e999px",
                CalculationRoot::Length,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1e999deg",
                CalculationRoot::Angle,
                CssErrorCode::UnexpectedEnd,
            ),
            ("1e999s", CalculationRoot::Time, CssErrorCode::UnexpectedEnd),
            (
                "1e999hz",
                CalculationRoot::Frequency,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1px +",
                CalculationRoot::Length,
                CssErrorCode::UnexpectedEnd,
            ),
            (
                "1px red",
                CalculationRoot::Length,
                CssErrorCode::UnexpectedToken,
            ),
            ("1px", CalculationRoot::Number, CssErrorCode::UnexpectedEnd),
        ];

        for (source, root, code) in cases {
            let error = parse(source, root).expect_err(source);
            assert_eq!(error.code(), code, "{source}: {error:?}");
        }
    }

    #[test]
    fn typed_root_parser_accepts_256_nested_calculations_and_rejects_257() {
        for depth in [255_usize, 256] {
            let source = format!("{}1px{}", "calc(".repeat(depth), ")".repeat(depth));
            let result_type = std::thread::scope(|scope| {
                std::thread::Builder::new()
                    .stack_size(16 * 1024 * 1024)
                    .spawn_scoped(scope, || {
                        parse(&source, CalculationRoot::Length)
                            .map(|expression| expression.result_type())
                    })
                    .unwrap()
                    .join()
                    .unwrap()
            })
            .unwrap();
            assert_eq!(result_type, CssCalculationType::Length);
        }

        let depth = 257_usize;
        let source = format!("{}1px{}", "calc(".repeat(depth), ")".repeat(depth));
        let error = std::thread::scope(|scope| {
            std::thread::Builder::new()
                .stack_size(16 * 1024 * 1024)
                .spawn_scoped(scope, || parse(&source, CalculationRoot::Length))
                .unwrap()
                .join()
                .unwrap()
        })
        .expect_err("depth 257 must fail");
        assert_eq!(error.code(), CssErrorCode::UnexpectedEnd);
    }
}

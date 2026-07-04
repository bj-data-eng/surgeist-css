use std::collections::HashMap;

use cssparser::{ParseError, Parser, Token, match_ignore_ascii_case};

use super::values::{
    LengthOptions, next_is_delim, parse_box_size_value, parse_calc_length_with_options,
    parse_custom_ident_from_str_at, parse_length_with, parse_positive_integer,
};
use crate::error::{Error, ErrorKind, basic, error_at, unsupported_value, unsupported_value_at};
use crate::syntax::{self, *};
use crate::validation::{LengthUnitStatus, classify_length_unit, unsupported_keyword_reason};

pub(super) fn parse_grid_flow_tolerance<'i, 't>(
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

pub(super) fn parse_grid_track_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridTrackList, ParseError<'i, Error>> {
    parse_grid_track_list_until_slash(input, false)
}

pub(super) fn parse_grid_track_list_until_slash<'i, 't>(
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

pub(super) fn parse_grid_track_component<'i, 't>(
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

pub(super) fn parse_grid_line_names<'i, 't>(
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

pub(super) fn parse_grid_repeat<'i, 't>(
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

pub(super) fn parse_grid_track_size<'i, 't>(
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

pub(super) fn parse_grid_track_breadth<'i, 't>(
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

pub(super) fn parse_grid_template_areas<'i, 't>(
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

pub(super) fn parse_grid_template_area_row<'i>(
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
pub(super) struct GridAreaBounds {
    min_row: usize,
    max_row: usize,
    min_col: usize,
    max_col: usize,
    count: usize,
}

pub(super) fn validate_grid_template_area_rectangles<'i, 't>(
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

pub(super) fn parse_grid_template<'i, 't>(
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

pub(super) fn parse_grid_auto_flow<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGridAutoFlow, ParseError<'i, Error>> {
    let axis = parse_grid_auto_flow_axis(input)?;
    let dense = input
        .try_parse(|input| input.expect_ident_matching("dense"))
        .is_ok();
    Ok(CssGridAutoFlow::new(axis, dense))
}

pub(super) fn parse_grid_auto_flow_axis<'i, 't>(
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

pub(super) fn parse_grid_line<'i, 't>(
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

pub(super) fn parse_grid_line_span<'i, 't>(
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

pub(super) fn parse_grid_line_range<'i, 't>(
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

pub(super) fn parse_grid_area<'i, 't>(
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

pub(super) fn parse_grid<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssGrid, ParseError<'i, Error>> {
    if let Ok(grid) = input.try_parse(parse_grid_auto_flow_shorthand) {
        Ok(grid)
    } else {
        parse_grid_template(input).map(CssGrid::Template)
    }
}

pub(super) fn parse_grid_auto_flow_shorthand<'i, 't>(
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

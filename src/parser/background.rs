use cssparser::{ParseError, Parser, ParserState, match_ignore_ascii_case};

use super::box_model::parse_border_style;
use super::values::{
    LengthGrammar, next_is_comma, next_is_delim, parse_color, parse_length_with,
    parse_length_with_context, parse_length_with_context_legacy,
};
use crate::error::{Error, basic, unsupported_value};
use crate::syntax::*;
use crate::validation::unsupported_keyword_reason;

pub(super) fn parse_image_layer_list<'i, 't>(
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

pub(super) fn parse_image_layer<'i, 't>(
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

pub(super) fn parse_url<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssUrl, ParseError<'i, Error>> {
    let value = input.expect_url().map_err(basic)?.to_string();
    CssUrl::try_new(value).ok_or_else(|| unsupported_value(input, None, "URL is empty"))
}

pub(super) fn parse_mask_position_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssMaskPositionList, ParseError<'i, Error>> {
    let mut positions = Vec::new();
    loop {
        let (current, legacy) = parse_generic_position(input)?;
        let legacy = (!position_has_typed_calculation(&legacy)).then_some(legacy);
        positions.push(CssMaskPosition::new(current, legacy));
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "mask-position list has an empty item",
            ));
        }
    }
    CssMaskPositionList::try_new(positions)
        .ok_or_else(|| unsupported_value(input, None, "mask-position list is empty"))
}

pub(super) fn parse_background_position_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundPositionList, ParseError<'i, Error>> {
    let mut positions = Vec::new();
    loop {
        positions.push(parse_background_position(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "background-position list has an empty item",
            ));
        }
    }
    CssBackgroundPositionList::try_new(positions)
        .ok_or_else(|| unsupported_value(input, None, "background-position list is empty"))
}

pub(super) fn parse_object_position<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssObjectPosition, ParseError<'i, Error>> {
    parse_generic_position(input).map(|(position, _)| CssObjectPosition::new(position))
}

pub(super) fn parse_transform_origin<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssTransformOrigin, ParseError<'i, Error>> {
    let (atoms, states) = parse_position_atoms(input)?;

    if atoms.len() <= 2
        && let Some((position, legacy)) = build_generic_position(&atoms)
    {
        let legacy = (!position_has_typed_calculation(&legacy)).then_some(legacy);
        return Ok(CssTransformOrigin::new(position, None, legacy));
    }

    if (2..=3).contains(&atoms.len()) {
        let z_index = atoms.len() - 1;
        if let Some((position, _)) = build_generic_position(&atoms[..z_index]) {
            let z = transform_origin_z(&atoms[z_index])
                .ok_or_else(|| invalid_generic_position_atom(input, &states[z_index]))?;
            let legacy = CssPosition::new(contextual_legacy_components(&atoms));
            let legacy = (!position_has_typed_calculation(&legacy)).then_some(legacy);
            return Ok(CssTransformOrigin::new(position, Some(z), legacy));
        }
    }

    let invalid_index = if atoms.len() > 3 {
        3
    } else {
        invalid_atom_index(&atoms)
    };
    Err(invalid_generic_position_atom(input, &states[invalid_index]))
}

pub(super) fn parse_css_position<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPosition, ParseError<'i, Error>> {
    parse_generic_position(input).map(|(_, legacy)| legacy)
}

pub(super) fn parse_css_position_legacy<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPosition, ParseError<'i, Error>> {
    parse_css_position_legacy_components(input, false)
}

fn parse_css_position_legacy_components<'i, 't>(
    input: &mut Parser<'i, 't>,
    allow_typed_calculation: bool,
) -> std::result::Result<CssPosition, ParseError<'i, Error>> {
    let mut components = Vec::new();
    while !input.is_exhausted() && !next_is_comma(input) && !next_is_delim(input, '/') {
        components.push(parse_legacy_position_component(
            input,
            &components,
            allow_typed_calculation,
        )?);
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

fn parse_legacy_position_component<'i, 't>(
    input: &mut Parser<'i, 't>,
    previous: &[CssPositionComponent],
    allow_typed_calculation: bool,
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
    if allow_typed_calculation {
        parse_length_with(input, LengthGrammar::Position)
    } else {
        parse_length_with_context_legacy(input, LengthGrammar::Position, "position")
    }
    .map(CssPositionComponent::Length)
}

#[derive(Clone, Debug)]
enum GenericPositionAtom {
    Horizontal(CssHorizontalPositionKeyword),
    Vertical(CssVerticalPositionKeyword),
    Center,
    Offset(CssPositionOffset),
}

fn parse_generic_position<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<(CssPositionValue, CssPosition), ParseError<'i, Error>> {
    let (atoms, states) = parse_position_atoms(input)?;
    build_generic_position(&atoms)
        .ok_or_else(|| invalid_generic_position_atom(input, &states[invalid_atom_index(&atoms)]))
}

fn parse_position_atoms<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<(Vec<GenericPositionAtom>, Vec<ParserState>), ParseError<'i, Error>> {
    let mut atoms = Vec::new();
    let mut states = Vec::new();
    while !input.is_exhausted() && !next_is_comma(input) && !next_is_delim(input, '/') {
        states.push(input.state());
        atoms.push(parse_generic_position_atom(input)?);
        if atoms.len() > 4 {
            return Err(invalid_generic_position_atom(input, &states[4]));
        }
    }
    if atoms.is_empty() {
        return Err(unsupported_value(input, None, "position is empty"));
    }
    Ok((atoms, states))
}

fn parse_background_position<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundPosition, ParseError<'i, Error>> {
    let (atoms, states) = parse_position_atoms(input)?;
    build_background_position(&atoms).ok_or_else(|| {
        invalid_generic_position_atom(input, &states[invalid_background_atom_index(&atoms)])
    })
}

#[cfg(test)]
fn parse_css_position_value<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPositionValue, ParseError<'i, Error>> {
    parse_generic_position(input).map(|(current, _)| current)
}

fn parse_generic_position_atom<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<GenericPositionAtom, ParseError<'i, Error>> {
    let state = input.state();
    if let Ok(ident) = input.try_parse(Parser::expect_ident_cloned) {
        return match_ignore_ascii_case! { &ident,
            "left" => Ok(GenericPositionAtom::Horizontal(CssHorizontalPositionKeyword::Left)),
            "right" => Ok(GenericPositionAtom::Horizontal(CssHorizontalPositionKeyword::Right)),
            "top" => Ok(GenericPositionAtom::Vertical(CssVerticalPositionKeyword::Top)),
            "bottom" => Ok(GenericPositionAtom::Vertical(CssVerticalPositionKeyword::Bottom)),
            "center" => Ok(GenericPositionAtom::Center),
            _ => {
                input.reset(&state);
                Err(invalid_generic_position_atom(input, &state))
            },
        };
    }
    input.reset(&state);
    let value = parse_length_with(input, LengthGrammar::Position)?;
    let Some(offset) = CssPositionOffset::try_new(value) else {
        return Err(invalid_generic_position_atom(input, &state));
    };
    Ok(GenericPositionAtom::Offset(offset))
}

fn invalid_generic_position_atom<'i, 't>(
    input: &mut Parser<'i, 't>,
    state: &ParserState,
) -> ParseError<'i, Error> {
    input.reset(state);
    let location = input.current_source_location();
    match input.next() {
        Ok(token) => location.new_unexpected_token_error::<Error>(token.clone()),
        Err(error) => error.into(),
    }
}

fn transform_origin_z(atom: &GenericPositionAtom) -> Option<CssTransformOriginZ> {
    let GenericPositionAtom::Offset(offset) = atom else {
        return None;
    };
    CssTransformOriginZ::try_new(offset.value().clone())
}

fn invalid_atom_index(atoms: &[GenericPositionAtom]) -> usize {
    match atoms.len() {
        1 => 0,
        2 => 1,
        3 => 2,
        4 => {
            if build_generic_position(&atoms[..2]).is_some() {
                2
            } else if !matches!(
                atoms[0],
                GenericPositionAtom::Horizontal(
                    CssHorizontalPositionKeyword::Left | CssHorizontalPositionKeyword::Right
                ) | GenericPositionAtom::Vertical(
                    CssVerticalPositionKeyword::Top | CssVerticalPositionKeyword::Bottom
                )
            ) {
                0
            } else if !matches!(atoms[1], GenericPositionAtom::Offset(_)) {
                1
            } else if !matches!(
                atoms[2],
                GenericPositionAtom::Horizontal(
                    CssHorizontalPositionKeyword::Left | CssHorizontalPositionKeyword::Right
                ) | GenericPositionAtom::Vertical(
                    CssVerticalPositionKeyword::Top | CssVerticalPositionKeyword::Bottom
                )
            ) {
                2
            } else if !matches!(atoms[3], GenericPositionAtom::Offset(_)) {
                3
            } else {
                2
            }
        }
        _ => 0,
    }
}

fn build_generic_position(
    atoms: &[GenericPositionAtom],
) -> Option<(CssPositionValue, CssPosition)> {
    use GenericPositionAtom::{Center, Horizontal, Offset, Vertical};

    let (horizontal, vertical, components) = match atoms {
        [Horizontal(keyword)] => (
            horizontal_keyword(*keyword),
            CssVerticalPosition::Center,
            vec![CssPositionComponent::Horizontal(*keyword)],
        ),
        [Vertical(keyword)] => (
            CssHorizontalPosition::Center,
            vertical_keyword(*keyword),
            vec![CssPositionComponent::Vertical(*keyword)],
        ),
        [Center] => (
            CssHorizontalPosition::Center,
            CssVerticalPosition::Center,
            vec![CssPositionComponent::Horizontal(
                CssHorizontalPositionKeyword::Center,
            )],
        ),
        [Offset(offset)] => (
            CssHorizontalPosition::Offset(offset.clone()),
            CssVerticalPosition::Center,
            vec![CssPositionComponent::Length(offset.value().clone())],
        ),
        [Horizontal(horizontal), Vertical(vertical)] => (
            horizontal_keyword(*horizontal),
            vertical_keyword(*vertical),
            vec![
                CssPositionComponent::Horizontal(*horizontal),
                CssPositionComponent::Vertical(*vertical),
            ],
        ),
        [Vertical(vertical), Horizontal(horizontal)] => (
            horizontal_keyword(*horizontal),
            vertical_keyword(*vertical),
            vec![
                CssPositionComponent::Vertical(*vertical),
                CssPositionComponent::Horizontal(*horizontal),
            ],
        ),
        [Horizontal(horizontal), Center] => (
            horizontal_keyword(*horizontal),
            CssVerticalPosition::Center,
            vec![
                CssPositionComponent::Horizontal(*horizontal),
                CssPositionComponent::Vertical(CssVerticalPositionKeyword::Center),
            ],
        ),
        [Center, Horizontal(horizontal)] => (
            horizontal_keyword(*horizontal),
            CssVerticalPosition::Center,
            vec![
                CssPositionComponent::Vertical(CssVerticalPositionKeyword::Center),
                CssPositionComponent::Horizontal(*horizontal),
            ],
        ),
        [Vertical(vertical), Center] => (
            CssHorizontalPosition::Center,
            vertical_keyword(*vertical),
            vec![
                CssPositionComponent::Vertical(*vertical),
                CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center),
            ],
        ),
        [Center, Vertical(vertical)] => (
            CssHorizontalPosition::Center,
            vertical_keyword(*vertical),
            vec![
                CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center),
                CssPositionComponent::Vertical(*vertical),
            ],
        ),
        [Center, Center] => (
            CssHorizontalPosition::Center,
            CssVerticalPosition::Center,
            vec![
                CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center),
                CssPositionComponent::Vertical(CssVerticalPositionKeyword::Center),
            ],
        ),
        [Horizontal(horizontal), Offset(offset)] => (
            horizontal_keyword(*horizontal),
            CssVerticalPosition::Offset(offset.clone()),
            vec![
                CssPositionComponent::Horizontal(*horizontal),
                CssPositionComponent::Length(offset.value().clone()),
            ],
        ),
        [Center, Offset(offset)] => (
            CssHorizontalPosition::Center,
            CssVerticalPosition::Offset(offset.clone()),
            vec![
                CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center),
                CssPositionComponent::Length(offset.value().clone()),
            ],
        ),
        [Offset(offset), Vertical(vertical)] => (
            CssHorizontalPosition::Offset(offset.clone()),
            vertical_keyword(*vertical),
            vec![
                CssPositionComponent::Length(offset.value().clone()),
                CssPositionComponent::Vertical(*vertical),
            ],
        ),
        [Offset(offset), Center] => (
            CssHorizontalPosition::Offset(offset.clone()),
            CssVerticalPosition::Center,
            vec![
                CssPositionComponent::Length(offset.value().clone()),
                CssPositionComponent::Vertical(CssVerticalPositionKeyword::Center),
            ],
        ),
        [Offset(horizontal), Offset(vertical)] => (
            CssHorizontalPosition::Offset(horizontal.clone()),
            CssVerticalPosition::Offset(vertical.clone()),
            vec![
                CssPositionComponent::Length(horizontal.value().clone()),
                CssPositionComponent::Length(vertical.value().clone()),
            ],
        ),
        [
            Horizontal(horizontal),
            Offset(horizontal_offset),
            Vertical(vertical),
            Offset(vertical_offset),
        ] if is_horizontal_edge(*horizontal) && is_vertical_edge(*vertical) => (
            horizontal_edge_offset(*horizontal, horizontal_offset.clone()),
            vertical_edge_offset(*vertical, vertical_offset.clone()),
            legacy_components(atoms),
        ),
        [
            Vertical(vertical),
            Offset(vertical_offset),
            Horizontal(horizontal),
            Offset(horizontal_offset),
        ] if is_vertical_edge(*vertical) && is_horizontal_edge(*horizontal) => (
            horizontal_edge_offset(*horizontal, horizontal_offset.clone()),
            vertical_edge_offset(*vertical, vertical_offset.clone()),
            legacy_components(atoms),
        ),
        _ => return None,
    };

    Some((
        CssPositionValue::new(horizontal, vertical),
        CssPosition::new(components),
    ))
}

fn invalid_background_atom_index(atoms: &[GenericPositionAtom]) -> usize {
    if atoms.len() == 4 && build_background_position(&atoms[..3]).is_some() {
        3
    } else {
        invalid_atom_index(atoms)
    }
}

fn build_background_position(atoms: &[GenericPositionAtom]) -> Option<CssBackgroundPosition> {
    use GenericPositionAtom::{Center, Horizontal, Offset, Vertical};

    let (horizontal, vertical) = match atoms {
        [Horizontal(horizontal), Offset(offset), Vertical(vertical)]
            if is_horizontal_edge(*horizontal) && is_vertical_edge(*vertical) =>
        {
            (
                horizontal_edge_offset(*horizontal, offset.clone()),
                vertical_keyword(*vertical),
            )
        }
        [Horizontal(horizontal), Offset(offset), Center] if is_horizontal_edge(*horizontal) => (
            horizontal_edge_offset(*horizontal, offset.clone()),
            CssVerticalPosition::Center,
        ),
        [Vertical(vertical), Offset(offset), Horizontal(horizontal)]
            if is_vertical_edge(*vertical) && is_horizontal_edge(*horizontal) =>
        {
            (
                horizontal_keyword(*horizontal),
                vertical_edge_offset(*vertical, offset.clone()),
            )
        }
        [Vertical(vertical), Offset(offset), Center] if is_vertical_edge(*vertical) => (
            CssHorizontalPosition::Center,
            vertical_edge_offset(*vertical, offset.clone()),
        ),
        [Horizontal(horizontal), Vertical(vertical), Offset(offset)]
            if is_horizontal_edge(*horizontal) && is_vertical_edge(*vertical) =>
        {
            (
                horizontal_keyword(*horizontal),
                vertical_edge_offset(*vertical, offset.clone()),
            )
        }
        [Center, Vertical(vertical), Offset(offset)] if is_vertical_edge(*vertical) => (
            CssHorizontalPosition::Center,
            vertical_edge_offset(*vertical, offset.clone()),
        ),
        [Vertical(vertical), Horizontal(horizontal), Offset(offset)]
            if is_vertical_edge(*vertical) && is_horizontal_edge(*horizontal) =>
        {
            (
                horizontal_edge_offset(*horizontal, offset.clone()),
                vertical_keyword(*vertical),
            )
        }
        [Center, Horizontal(horizontal), Offset(offset)] if is_horizontal_edge(*horizontal) => (
            horizontal_edge_offset(*horizontal, offset.clone()),
            CssVerticalPosition::Center,
        ),
        _ => {
            let (position, legacy) = build_generic_position(atoms)?;
            let legacy = (!position_has_typed_calculation(&legacy)).then_some(legacy);
            return Some(CssBackgroundPosition::new(
                position.horizontal().clone(),
                position.vertical().clone(),
                legacy,
            ));
        }
    };

    let legacy = CssPosition::new(contextual_legacy_components(atoms));
    let legacy = (!position_has_typed_calculation(&legacy)).then_some(legacy);
    Some(CssBackgroundPosition::new(horizontal, vertical, legacy))
}

fn contextual_legacy_components(atoms: &[GenericPositionAtom]) -> Vec<CssPositionComponent> {
    let mut components = Vec::with_capacity(atoms.len());
    for atom in atoms {
        let component = match atom {
            GenericPositionAtom::Horizontal(keyword) => CssPositionComponent::Horizontal(*keyword),
            GenericPositionAtom::Vertical(keyword) => CssPositionComponent::Vertical(*keyword),
            GenericPositionAtom::Center => {
                if components
                    .iter()
                    .any(|component| matches!(component, CssPositionComponent::Horizontal(_)))
                {
                    CssPositionComponent::Vertical(CssVerticalPositionKeyword::Center)
                } else {
                    CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center)
                }
            }
            GenericPositionAtom::Offset(offset) => {
                CssPositionComponent::Length(offset.value().clone())
            }
        };
        components.push(component);
    }
    components
}

fn position_has_typed_calculation(position: &CssPosition) -> bool {
    position.components().iter().any(|component| {
        matches!(
            component,
            CssPositionComponent::Length(CssLength::Calc(CssCalcLength::Typed(_)))
        )
    })
}

fn legacy_components(atoms: &[GenericPositionAtom]) -> Vec<CssPositionComponent> {
    atoms
        .iter()
        .map(|atom| match atom {
            GenericPositionAtom::Horizontal(keyword) => CssPositionComponent::Horizontal(*keyword),
            GenericPositionAtom::Vertical(keyword) => CssPositionComponent::Vertical(*keyword),
            GenericPositionAtom::Center => {
                CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center)
            }
            GenericPositionAtom::Offset(offset) => {
                CssPositionComponent::Length(offset.value().clone())
            }
        })
        .collect()
}

const fn horizontal_keyword(keyword: CssHorizontalPositionKeyword) -> CssHorizontalPosition {
    match keyword {
        CssHorizontalPositionKeyword::Left => CssHorizontalPosition::Left,
        CssHorizontalPositionKeyword::Center => CssHorizontalPosition::Center,
        CssHorizontalPositionKeyword::Right => CssHorizontalPosition::Right,
    }
}

const fn vertical_keyword(keyword: CssVerticalPositionKeyword) -> CssVerticalPosition {
    match keyword {
        CssVerticalPositionKeyword::Top => CssVerticalPosition::Top,
        CssVerticalPositionKeyword::Center => CssVerticalPosition::Center,
        CssVerticalPositionKeyword::Bottom => CssVerticalPosition::Bottom,
    }
}

const fn is_horizontal_edge(keyword: CssHorizontalPositionKeyword) -> bool {
    matches!(
        keyword,
        CssHorizontalPositionKeyword::Left | CssHorizontalPositionKeyword::Right
    )
}

const fn is_vertical_edge(keyword: CssVerticalPositionKeyword) -> bool {
    matches!(
        keyword,
        CssVerticalPositionKeyword::Top | CssVerticalPositionKeyword::Bottom
    )
}

fn horizontal_edge_offset(
    keyword: CssHorizontalPositionKeyword,
    offset: CssPositionOffset,
) -> CssHorizontalPosition {
    match keyword {
        CssHorizontalPositionKeyword::Left => CssHorizontalPosition::LeftOffset(offset),
        CssHorizontalPositionKeyword::Right => CssHorizontalPosition::RightOffset(offset),
        CssHorizontalPositionKeyword::Center => CssHorizontalPosition::Center,
    }
}

fn vertical_edge_offset(
    keyword: CssVerticalPositionKeyword,
    offset: CssPositionOffset,
) -> CssVerticalPosition {
    match keyword {
        CssVerticalPositionKeyword::Top => CssVerticalPosition::TopOffset(offset),
        CssVerticalPositionKeyword::Bottom => CssVerticalPosition::BottomOffset(offset),
        CssVerticalPositionKeyword::Center => CssVerticalPosition::Center,
    }
}

pub(super) fn parse_background_size_list<'i, 't>(
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

pub(super) fn parse_background_size<'i, 't>(
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

pub(super) fn parse_background_size_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssBackgroundSizeComponent, ParseError<'i, Error>> {
    if input
        .try_parse(|input| input.expect_ident_matching("auto"))
        .is_ok()
    {
        Ok(CssBackgroundSizeComponent::Auto)
    } else {
        parse_length_with(input, LengthGrammar::BackgroundSize)
            .map(CssBackgroundSizeComponent::Length)
    }
}

pub(super) fn parse_background_repeat_list<'i, 't>(
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

pub(super) fn parse_background_repeat<'i, 't>(
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

pub(super) fn parse_background_repeat_style_from_ident<'i, 't>(
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

pub(super) fn parse_background_box<'i, 't>(
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

pub(super) fn parse_background_attachment_list<'i, 't>(
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

pub(super) fn parse_background_attachment<'i, 't>(
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

pub(super) fn parse_cursor<'i, 't>(
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

pub(super) fn parse_cursor_keyword<'i, 't>(
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

pub(super) fn parse_pointer_events<'i, 't>(
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

pub(super) fn parse_user_select<'i, 't>(
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

pub(super) fn parse_outline<'i, 't>(
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

pub(super) fn parse_outline_style<'i, 't>(
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

pub(super) fn parse_outline_width<'i, 't>(
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
    parse_length_with_context(input, LengthGrammar::BorderWidth, "outline-width")
        .map(CssOutlineWidth::Length)
}

#[cfg(test)]
mod tests {
    use cssparser::{Parser, ParserInput};

    use super::*;

    fn parse_current(source: &str) -> CssPositionValue {
        let mut input = ParserInput::new(source);
        let mut parser = Parser::new(&mut input);
        parser
            .parse_entirely(parse_css_position_value)
            .expect("valid generic position")
    }

    #[test]
    fn generic_position_model_distinguishes_omitted_and_free_offset_axes() {
        let top = parse_current("top");
        assert!(matches!(top.horizontal(), CssHorizontalPosition::Center));
        assert!(matches!(top.vertical(), CssVerticalPosition::Top));

        let free = parse_current("25% 10px");
        assert!(matches!(
            free.horizontal(),
            CssHorizontalPosition::Offset(offset)
                if matches!(offset.value(), CssLength::Percent(value) if value.value() == 25.0)
        ));
        assert!(matches!(
            free.vertical(),
            CssVerticalPosition::Offset(offset)
                if matches!(offset.value(), CssLength::Px(value) if value.value() == 10.0)
        ));
    }

    #[test]
    fn generic_position_model_retains_each_edge_offset_origin_and_pair_order() {
        for source in ["left 10px bottom 20%", "bottom 20% left 10px"] {
            let position = parse_current(source);
            assert!(matches!(
                position.horizontal(),
                CssHorizontalPosition::LeftOffset(offset)
                    if matches!(offset.value(), CssLength::Px(value) if value.value() == 10.0)
            ));
            assert!(matches!(
                position.vertical(),
                CssVerticalPosition::BottomOffset(offset)
                    if matches!(offset.value(), CssLength::Percent(value) if value.value() == 20.0)
            ));
        }

        let opposite = parse_current("right calc(1px * 2) top calc(10% + 2px)");
        assert!(matches!(
            opposite.horizontal(),
            CssHorizontalPosition::RightOffset(offset)
                if matches!(offset.value(), CssLength::Calc(CssCalcLength::Typed(_)))
        ));
        assert!(matches!(
            opposite.vertical(),
            CssVerticalPosition::TopOffset(offset)
                if matches!(offset.value(), CssLength::Calc(_))
        ));
    }
}

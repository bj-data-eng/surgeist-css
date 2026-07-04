use cssparser::{ParseError, Parser, match_ignore_ascii_case};

use super::box_model::parse_border_style;
use super::values::{LengthOptions, next_is_comma, next_is_delim, parse_color, parse_length_with};
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

pub(super) fn parse_position_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPositionList, ParseError<'i, Error>> {
    let mut positions = Vec::new();
    loop {
        positions.push(parse_css_position(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
        if input.is_exhausted() {
            return Err(unsupported_value(
                input,
                None,
                "position list has an empty item",
            ));
        }
    }
    CssPositionList::try_new(positions)
        .ok_or_else(|| unsupported_value(input, None, "position list is empty"))
}

pub(super) fn parse_css_position<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssPosition, ParseError<'i, Error>> {
    let mut components = Vec::new();
    while !input.is_exhausted() && !next_is_comma(input) && !next_is_delim(input, '/') {
        components.push(parse_position_component(input, &components)?);
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

pub(super) fn parse_position_component<'i, 't>(
    input: &mut Parser<'i, 't>,
    previous: &[CssPositionComponent],
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
    parse_length_with(input, LengthOptions::position(), "position")
        .map(CssPositionComponent::Length)
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
        parse_length_with(input, LengthOptions::background_size(), "background-size")
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
    parse_length_with(input, LengthOptions::border_width(), "outline-width")
        .map(CssOutlineWidth::Length)
}

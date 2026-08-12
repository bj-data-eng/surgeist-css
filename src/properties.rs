//! Canonical identity and parser wiring for recognized CSS properties.
//!
//! The crate-private schema in this module is the single authority for the
//! frozen property set. Public identity values describe authored property names;
//! they do not apply cascade, substitute variables, or resolve authored values.

use crate::syntax::*;

macro_rules! property_schema {
    ($callback:ident, $input:ident) => {
        $callback! {
            $input;
            All, "all", [], "baseline.property.all", CssAllDeclaredValue, parse_all_property, { parse_all_property($input)? };
            Display, "display", [], "baseline.property.display", CssDisplay, parse_display, { parse_display($input)? };
            BoxSizing, "box-sizing", [], "baseline.property.box-sizing", CssBoxSizing, parse_box_sizing, { parse_box_sizing($input)? };
            Position, "position", [], "baseline.property.position", CssLayoutPosition, parse_position, { parse_position($input)? };
            Direction, "direction", [], "baseline.property.direction", CssDirection, parse_direction, { parse_direction($input)? };
            Overflow, "overflow", [], "baseline.property.overflow", CssOverflowPropertyValue, parse_overflow_property, { parse_overflow_property($input)? };
            OverflowX, "overflow-x", [], "baseline.property.overflow-x", CssOverflow, parse_overflow, { parse_overflow($input)? };
            OverflowY, "overflow-y", [], "baseline.property.overflow-y", CssOverflow, parse_overflow, { parse_overflow($input)? };
            FlexDirection, "flex-direction", [], "baseline.property.flex-direction", CssFlexDirection, parse_flex_direction, { parse_flex_direction($input)? };
            FlexWrap, "flex-wrap", [], "baseline.property.flex-wrap", CssFlexWrap, parse_flex_wrap, { parse_flex_wrap($input)? };
            Float, "float", [], "baseline.property.float", CssFloat, parse_float, { parse_float($input)? };
            Clear, "clear", [], "baseline.property.clear", CssClear, parse_clear, { parse_clear($input)? };
            AlignContent, "align-content", [], "baseline.property.align-content", CssAlignment, parse_content_alignment, { parse_content_alignment($input)? };
            JustifyContent, "justify-content", [], "baseline.property.justify-content", CssAlignment, parse_content_alignment, { parse_content_alignment($input)? };
            AlignItems, "align-items", [], "baseline.property.align-items", CssAlignItems, parse_align_items, { parse_align_items($input)? };
            AlignSelf, "align-self", [], "baseline.property.align-self", CssAlignItems, parse_align_items, { parse_align_items($input)? };
            JustifyItems, "justify-items", [], "baseline.property.justify-items", CssAlignItems, parse_align_items, { parse_align_items($input)? };
            JustifySelf, "justify-self", [], "baseline.property.justify-self", CssAlignItems, parse_align_items, { parse_align_items($input)? };
            PlaceContent, "place-content", [], "baseline.property.place-content", CssPlaceAlignment, parse_place_alignment, { parse_place_alignment($input, parse_content_alignment, CssPlaceAlignment::content)? };
            PlaceItems, "place-items", [], "baseline.property.place-items", CssPlaceAlignment, parse_place_alignment, { parse_place_alignment($input, parse_align_items, CssPlaceAlignment::items)? };
            PlaceSelf, "place-self", [], "baseline.property.place-self", CssPlaceAlignment, parse_place_alignment, { parse_place_alignment($input, parse_align_items, CssPlaceAlignment::items)? };
            Visibility, "visibility", [], "baseline.property.visibility", CssVisibility, parse_visibility, { parse_visibility($input)? };
            Content, "content", [], "baseline.property.content", CssContent, parse_content, { parse_content($input)? };
            ContentVisibility, "content-visibility", [], "baseline.property.content-visibility", CssContentVisibility, parse_content_visibility, { parse_content_visibility($input)? };
            ListStyleType, "list-style-type", [], "baseline.property.list-style-type", CssListStyleType, parse_list_style_type, { parse_list_style_type($input)? };
            ListStylePosition, "list-style-position", [], "baseline.property.list-style-position", CssListStylePosition, parse_list_style_position, { parse_list_style_position($input)? };
            ListStyleImage, "list-style-image", [], "baseline.property.list-style-image", CssListStyleImage, parse_list_style_image, { parse_list_style_image($input)? };
            ListStyle, "list-style", [], "baseline.property.list-style", CssListStyle, parse_list_style, { parse_list_style($input)? };
            CounterReset, "counter-reset", [], "baseline.property.counter-reset", CssCounterChanges, parse_counter_changes, { parse_counter_changes($input)? };
            CounterIncrement, "counter-increment", [], "baseline.property.counter-increment", CssCounterChanges, parse_counter_changes, { parse_counter_changes($input)? };
            CounterSet, "counter-set", [], "baseline.property.counter-set", CssCounterChanges, parse_counter_changes, { parse_counter_changes($input)? };
            Width, "width", [], "baseline.property.width", CssLength, parse_box_size_value, { parse_box_size_value($input)? };
            Height, "height", [], "baseline.property.height", CssLength, parse_box_size_value, { parse_box_size_value($input)? };
            MinWidth, "min-width", [], "baseline.property.min-width", CssLength, parse_box_size_value, { parse_box_size_value($input)? };
            MinHeight, "min-height", [], "baseline.property.min-height", CssLength, parse_box_size_value, { parse_box_size_value($input)? };
            MaxWidth, "max-width", [], "baseline.property.max-width", CssLength, parse_box_size_value, { parse_box_size_value($input)? };
            MaxHeight, "max-height", [], "baseline.property.max-height", CssLength, parse_box_size_value, { parse_box_size_value($input)? };
            FlexBasis, "flex-basis", [], "baseline.property.flex-basis", CssLength, parse_box_size_value, { parse_box_size_value($input)? };
            Gap, "gap", [], "baseline.property.gap", CssLength, parse_gap_value, { parse_gap_value($input)? };
            RowGap, "row-gap", [], "baseline.property.row-gap", CssLength, parse_gap_value, { parse_gap_value($input)? };
            ColumnGap, "column-gap", [], "baseline.property.column-gap", CssLength, parse_gap_value, { parse_gap_value($input)? };
            GridFlowTolerance, "grid-flow-tolerance", [], "baseline.property.grid-flow-tolerance", CssGridFlowTolerance, parse_grid_flow_tolerance, { parse_grid_flow_tolerance($input)? };
            GridTemplateRows, "grid-template-rows", [], "baseline.property.grid-template-rows", CssGridTrackList, parse_grid_track_list, { parse_grid_track_list($input)? };
            GridTemplateColumns, "grid-template-columns", [], "baseline.property.grid-template-columns", CssGridTrackList, parse_grid_track_list, { parse_grid_track_list($input)? };
            GridTemplateAreas, "grid-template-areas", [], "baseline.property.grid-template-areas", CssGridTemplateAreas, parse_grid_template_areas, { parse_grid_template_areas($input)? };
            GridTemplate, "grid-template", [], "baseline.property.grid-template", CssGridTemplate, parse_grid_template, { parse_grid_template($input)? };
            GridAutoRows, "grid-auto-rows", [], "baseline.property.grid-auto-rows", CssGridTrackList, parse_grid_track_list, { parse_grid_track_list($input)? };
            GridAutoColumns, "grid-auto-columns", [], "baseline.property.grid-auto-columns", CssGridTrackList, parse_grid_track_list, { parse_grid_track_list($input)? };
            GridAutoFlow, "grid-auto-flow", [], "baseline.property.grid-auto-flow", CssGridAutoFlow, parse_grid_auto_flow, { parse_grid_auto_flow($input)? };
            GridRowStart, "grid-row-start", [], "baseline.property.grid-row-start", CssGridLine, parse_grid_line, { parse_grid_line($input)? };
            GridRowEnd, "grid-row-end", [], "baseline.property.grid-row-end", CssGridLine, parse_grid_line, { parse_grid_line($input)? };
            GridColumnStart, "grid-column-start", [], "baseline.property.grid-column-start", CssGridLine, parse_grid_line, { parse_grid_line($input)? };
            GridColumnEnd, "grid-column-end", [], "baseline.property.grid-column-end", CssGridLine, parse_grid_line, { parse_grid_line($input)? };
            GridRow, "grid-row", [], "baseline.property.grid-row", CssGridLineRange, parse_grid_line_range, { parse_grid_line_range($input)? };
            GridColumn, "grid-column", [], "baseline.property.grid-column", CssGridLineRange, parse_grid_line_range, { parse_grid_line_range($input)? };
            GridArea, "grid-area", [], "baseline.property.grid-area", CssGridArea, parse_grid_area, { parse_grid_area($input)? };
            Grid, "grid", [], "baseline.property.grid", CssGrid, parse_grid, { parse_grid($input)? };
            FontSize, "font-size", [], "baseline.property.font-size", CssLength, parse_font_size, { parse_font_size($input)? };
            LineHeight, "line-height", [], "baseline.property.line-height", CssLength, parse_line_height, { parse_line_height($input)? };
            WritingMode, "writing-mode", [], "baseline.property.writing-mode", CssWritingMode, parse_writing_mode, { parse_writing_mode($input)? };
            TextAlign, "text-align", [], "baseline.property.text-align", CssTextAlign, parse_text_align, { parse_text_align($input)? };
            TextAlignLast, "text-align-last", [], "baseline.property.text-align-last", CssTextAlignLast, parse_text_align_last, { parse_text_align_last($input)? };
            TextIndent, "text-indent", [], "baseline.property.text-indent", CssTextIndent, parse_text_indent, { parse_text_indent($input)? };
            VerticalAlign, "vertical-align", [], "baseline.property.vertical-align", CssVerticalAlign, parse_vertical_align, { parse_vertical_align($input)? };
            FontFamily, "font-family", [], "baseline.property.font-family", CssFontFamilyList, parse_font_family_list, { parse_font_family_list($input)? };
            Font, "font", [], "baseline.property.font", CssFont, parse_font, { parse_font($input)? };
            FontWeight, "font-weight", [], "baseline.property.font-weight", CssFontWeight, parse_font_weight, { parse_font_weight($input)? };
            FontStyle, "font-style", [], "baseline.property.font-style", CssFontStyle, parse_font_style, { parse_font_style($input)? };
            FontStretch, "font-stretch", [], "baseline.property.font-stretch", CssFontStretch, parse_font_stretch, { parse_font_stretch($input)? };
            FontVariant, "font-variant", [], "baseline.property.font-variant", CssFontVariant, parse_font_variant, { parse_font_variant($input)? };
            FontFeatureSettings, "font-feature-settings", [], "baseline.property.font-feature-settings", CssFontFeatureSettings, parse_font_feature_settings, { parse_font_feature_settings($input)? };
            LetterSpacing, "letter-spacing", [], "baseline.property.letter-spacing", CssLetterSpacing, parse_letter_spacing, { parse_letter_spacing($input)? };
            TextWrap, "text-wrap", [], "baseline.property.text-wrap", CssTextWrap, parse_text_wrap, { parse_text_wrap($input)? };
            WhiteSpace, "white-space", [], "baseline.property.white-space", CssWhiteSpace, parse_white_space, { parse_white_space($input)? };
            WordBreak, "word-break", [], "baseline.property.word-break", CssWordBreak, parse_word_break, { parse_word_break($input)? };
            OverflowWrap, "overflow-wrap", [], "baseline.property.overflow-wrap", CssOverflowWrap, parse_overflow_wrap, { parse_overflow_wrap($input)? };
            TextOverflow, "text-overflow", [], "baseline.property.text-overflow", CssTextOverflow, parse_text_overflow, { parse_text_overflow($input)? };
            TextDecoration, "text-decoration", [], "baseline.property.text-decoration", CssTextDecoration, parse_text_decoration, { parse_text_decoration($input)? };
            TextDecorationLine, "text-decoration-line", [], "baseline.property.text-decoration-line", CssTextDecorationLine, parse_text_decoration_line, { parse_text_decoration_line($input)? };
            TextDecorationColor, "text-decoration-color", [], "baseline.property.text-decoration-color", CssColor, parse_color, { parse_color($input)? };
            TextDecorationStyle, "text-decoration-style", [], "baseline.property.text-decoration-style", CssTextDecorationStyle, parse_text_decoration_style, { parse_text_decoration_style($input)? };
            TextDecorationThickness, "text-decoration-thickness", [], "baseline.property.text-decoration-thickness", CssTextDecorationThickness, parse_text_decoration_thickness, { parse_text_decoration_thickness($input)? };
            TextTransform, "text-transform", [], "baseline.property.text-transform", CssTextTransform, parse_text_transform, { parse_text_transform($input)? };
            Inset, "inset", [], "baseline.property.inset", CssEdges, parse_edges, { parse_edges($input, parse_inset_component)? };
            Top, "top", [], "baseline.property.top", CssLength, parse_inset_component, { parse_inset_component($input)? };
            Right, "right", [], "baseline.property.right", CssLength, parse_inset_component, { parse_inset_component($input)? };
            Bottom, "bottom", [], "baseline.property.bottom", CssLength, parse_inset_component, { parse_inset_component($input)? };
            Left, "left", [], "baseline.property.left", CssLength, parse_inset_component, { parse_inset_component($input)? };
            ZIndex, "z-index", [], "baseline.property.z-index", CssZIndex, parse_z_index, { parse_z_index($input)? };
            BoxDecorationBreak, "box-decoration-break", [], "baseline.property.box-decoration-break", CssBoxDecorationBreak, parse_box_decoration_break, { parse_box_decoration_break($input)? };
            Margin, "margin", [], "baseline.property.margin", CssEdges, parse_edges, { parse_edges($input, parse_margin_component)? };
            MarginTop, "margin-top", [], "baseline.property.margin-top", CssLength, parse_margin_component, { parse_margin_component($input)? };
            MarginRight, "margin-right", [], "baseline.property.margin-right", CssLength, parse_margin_component, { parse_margin_component($input)? };
            MarginBottom, "margin-bottom", [], "baseline.property.margin-bottom", CssLength, parse_margin_component, { parse_margin_component($input)? };
            MarginLeft, "margin-left", [], "baseline.property.margin-left", CssLength, parse_margin_component, { parse_margin_component($input)? };
            Padding, "padding", [], "baseline.property.padding", CssEdges, parse_edges, { parse_edges($input, parse_padding_component)? };
            PaddingTop, "padding-top", [], "baseline.property.padding-top", CssLength, parse_padding_component, { parse_padding_component($input)? };
            PaddingRight, "padding-right", [], "baseline.property.padding-right", CssLength, parse_padding_component, { parse_padding_component($input)? };
            PaddingBottom, "padding-bottom", [], "baseline.property.padding-bottom", CssLength, parse_padding_component, { parse_padding_component($input)? };
            PaddingLeft, "padding-left", [], "baseline.property.padding-left", CssLength, parse_padding_component, { parse_padding_component($input)? };
            Border, "border", [], "baseline.property.border", CssBorder, parse_border, { parse_border($input)? };
            BorderTop, "border-top", [], "baseline.property.border-top", CssBorder, parse_border, { parse_border($input)? };
            BorderRight, "border-right", [], "baseline.property.border-right", CssBorder, parse_border, { parse_border($input)? };
            BorderBottom, "border-bottom", [], "baseline.property.border-bottom", CssBorder, parse_border, { parse_border($input)? };
            BorderLeft, "border-left", [], "baseline.property.border-left", CssBorder, parse_border, { parse_border($input)? };
            BorderWidth, "border-width", [], "baseline.property.border-width", CssEdges, parse_edges, { parse_edges($input, parse_border_width_component)? };
            BorderTopWidth, "border-top-width", [], "baseline.property.border-top-width", CssLength, parse_border_width_component, { parse_border_width_component($input)? };
            BorderRightWidth, "border-right-width", [], "baseline.property.border-right-width", CssLength, parse_border_width_component, { parse_border_width_component($input)? };
            BorderBottomWidth, "border-bottom-width", [], "baseline.property.border-bottom-width", CssLength, parse_border_width_component, { parse_border_width_component($input)? };
            BorderLeftWidth, "border-left-width", [], "baseline.property.border-left-width", CssLength, parse_border_width_component, { parse_border_width_component($input)? };
            Color, "color", [], "baseline.property.color", CssColor, parse_color, { parse_color($input)? };
            Background, "background", [], "baseline.property.background", CssColor, parse_color, { parse_color($input)? };
            BackgroundColor, "background-color", [], "baseline.property.background-color", CssColor, parse_color, { parse_color($input)? };
            BorderColor, "border-color", [], "baseline.property.border-color", CssColor, parse_color, { parse_color($input)? };
            BorderTopColor, "border-top-color", [], "baseline.property.border-top-color", CssColor, parse_color, { parse_color($input)? };
            BorderRightColor, "border-right-color", [], "baseline.property.border-right-color", CssColor, parse_color, { parse_color($input)? };
            BorderBottomColor, "border-bottom-color", [], "baseline.property.border-bottom-color", CssColor, parse_color, { parse_color($input)? };
            BorderLeftColor, "border-left-color", [], "baseline.property.border-left-color", CssColor, parse_color, { parse_color($input)? };
            BackgroundImage, "background-image", [], "baseline.property.background-image", CssImageLayerList, parse_image_layer_list, { parse_image_layer_list($input)? };
            BackgroundPosition, "background-position", [], "baseline.property.background-position", CssPositionList, parse_position_list, { parse_position_list($input)? };
            BackgroundSize, "background-size", [], "baseline.property.background-size", CssBackgroundSizeList, parse_background_size_list, { parse_background_size_list($input)? };
            BackgroundRepeat, "background-repeat", [], "baseline.property.background-repeat", CssBackgroundRepeatList, parse_background_repeat_list, { parse_background_repeat_list($input)? };
            BackgroundOrigin, "background-origin", [], "baseline.property.background-origin", CssBackgroundBox, parse_background_box, { parse_background_box($input)? };
            BackgroundClip, "background-clip", [], "baseline.property.background-clip", CssBackgroundBox, parse_background_box, { parse_background_box($input)? };
            BackgroundAttachment, "background-attachment", [], "baseline.property.background-attachment", CssBackgroundAttachmentList, parse_background_attachment_list, { parse_background_attachment_list($input)? };
            BorderStyle, "border-style", [], "baseline.property.border-style", CssBorderStyles, parse_border_styles, { parse_border_styles($input)? };
            BorderTopStyle, "border-top-style", [], "baseline.property.border-top-style", CssBorderStyle, parse_border_style, { parse_border_style($input)? };
            BorderRightStyle, "border-right-style", [], "baseline.property.border-right-style", CssBorderStyle, parse_border_style, { parse_border_style($input)? };
            BorderBottomStyle, "border-bottom-style", [], "baseline.property.border-bottom-style", CssBorderStyle, parse_border_style, { parse_border_style($input)? };
            BorderLeftStyle, "border-left-style", [], "baseline.property.border-left-style", CssBorderStyle, parse_border_style, { parse_border_style($input)? };
            BorderRadius, "border-radius", [], "baseline.property.border-radius", CssBorderRadii, parse_border_radius, { parse_border_radius($input)? };
            BorderTopLeftRadius, "border-top-left-radius", [], "baseline.property.border-top-left-radius", CssCornerRadius, parse_corner_radius, { parse_corner_radius($input)? };
            BorderTopRightRadius, "border-top-right-radius", [], "baseline.property.border-top-right-radius", CssCornerRadius, parse_corner_radius, { parse_corner_radius($input)? };
            BorderBottomRightRadius, "border-bottom-right-radius", [], "baseline.property.border-bottom-right-radius", CssCornerRadius, parse_corner_radius, { parse_corner_radius($input)? };
            BorderBottomLeftRadius, "border-bottom-left-radius", [], "baseline.property.border-bottom-left-radius", CssCornerRadius, parse_corner_radius, { parse_corner_radius($input)? };
            BoxShadow, "box-shadow", [], "baseline.property.box-shadow", CssBoxShadow, parse_box_shadow, { parse_box_shadow($input)? };
            Opacity, "opacity", [], "baseline.property.opacity", CssOpacity, parse_opacity, { parse_opacity($input)? };
            FlexGrow, "flex-grow", [], "baseline.property.flex-grow", CssFlexFactor, parse_flex_factor, { parse_flex_factor($input, "flex-grow")? };
            FlexShrink, "flex-shrink", [], "baseline.property.flex-shrink", CssFlexFactor, parse_flex_factor, { parse_flex_factor($input, "flex-shrink")? };
            Order, "order", [], "baseline.property.order", CssOrder, parse_order, { parse_order($input)? };
            Flex, "flex", [], "baseline.property.flex", CssFlex, parse_flex, { parse_flex($input)? };
            JustifyTracks, "justify-tracks", [], "baseline.property.justify-tracks", CssAlignment, parse_content_alignment, { parse_content_alignment($input)? };
            AlignTracks, "align-tracks", [], "baseline.property.align-tracks", CssAlignment, parse_content_alignment, { parse_content_alignment($input)? };
            AspectRatio, "aspect-ratio", [], "baseline.property.aspect-ratio", CssAspectRatio, parse_aspect_ratio, { parse_aspect_ratio($input)? };
            ScrollbarWidth, "scrollbar-width", [], "baseline.property.scrollbar-width", CssScrollbarWidth, parse_scrollbar_width, { parse_scrollbar_width($input)? };
            Cursor, "cursor", [], "baseline.property.cursor", CssCursor, parse_cursor, { parse_cursor($input)? };
            PointerEvents, "pointer-events", [], "baseline.property.pointer-events", CssPointerEvents, parse_pointer_events, { parse_pointer_events($input)? };
            UserSelect, "user-select", [], "baseline.property.user-select", CssUserSelect, parse_user_select, { parse_user_select($input)? };
            Outline, "outline", [], "baseline.property.outline", CssOutline, parse_outline, { parse_outline($input)? };
            OutlineColor, "outline-color", [], "baseline.property.outline-color", CssColor, parse_color, { parse_color($input)? };
            OutlineStyle, "outline-style", [], "baseline.property.outline-style", CssOutlineStyle, parse_outline_style, { parse_outline_style($input)? };
            OutlineWidth, "outline-width", [], "baseline.property.outline-width", CssOutlineWidth, parse_outline_width, { parse_outline_width($input)? };
            Transform, "transform", [], "baseline.property.transform", CssTransform, parse_transform, { parse_transform($input)? };
            TransformOrigin, "transform-origin", [], "baseline.property.transform-origin", CssPosition, parse_css_position, { parse_css_position($input)? };
            Translate, "translate", [], "baseline.property.translate", CssTranslate, parse_translate, { parse_translate($input)? };
            Rotate, "rotate", [], "baseline.property.rotate", CssRotate, parse_rotate, { parse_rotate($input)? };
            Scale, "scale", [], "baseline.property.scale", CssScale, parse_scale, { parse_scale($input)? };
            Filter, "filter", [], "baseline.property.filter", CssFilter, parse_filter, { parse_filter($input)? };
            BackdropFilter, "backdrop-filter", [], "baseline.property.backdrop-filter", CssFilter, parse_filter, { parse_filter($input)? };
            ClipPath, "clip-path", [], "baseline.property.clip-path", CssClipPath, parse_clip_path, { parse_clip_path($input)? };
            Mask, "mask", [], "baseline.property.mask", CssMaskList, parse_mask_list, { parse_mask_list($input)? };
            MaskImage, "mask-image", [], "baseline.property.mask-image", CssImageLayerList, parse_image_layer_list, { parse_image_layer_list($input)? };
            MaskSize, "mask-size", [], "baseline.property.mask-size", CssBackgroundSizeList, parse_background_size_list, { parse_background_size_list($input)? };
            MaskPosition, "mask-position", [], "baseline.property.mask-position", CssPositionList, parse_position_list, { parse_position_list($input)? };
            MaskRepeat, "mask-repeat", [], "baseline.property.mask-repeat", CssBackgroundRepeatList, parse_background_repeat_list, { parse_background_repeat_list($input)? };
            TransitionProperty, "transition-property", [], "baseline.property.transition-property", CssTransitionPropertyList, parse_transition_property_list, { parse_transition_property_list($input)? };
            TransitionDuration, "transition-duration", [], "baseline.property.transition-duration", CssTimeList, parse_time_list, { parse_time_list($input)? };
            TransitionDelay, "transition-delay", [], "baseline.property.transition-delay", CssTimeList, parse_time_list, { parse_time_list($input)? };
            TransitionTimingFunction, "transition-timing-function", [], "baseline.property.transition-timing-function", CssEasingList, parse_easing_list, { parse_easing_list($input)? };
            Transition, "transition", [], "baseline.property.transition", CssTransitionList, parse_transition_list, { parse_transition_list($input)? };
            AnimationName, "animation-name", [], "baseline.property.animation-name", CssAnimationNameList, parse_animation_name_list, { parse_animation_name_list($input)? };
            AnimationDuration, "animation-duration", [], "baseline.property.animation-duration", CssTimeList, parse_time_list, { parse_time_list($input)? };
            AnimationDelay, "animation-delay", [], "baseline.property.animation-delay", CssTimeList, parse_time_list, { parse_time_list($input)? };
            AnimationTimingFunction, "animation-timing-function", [], "baseline.property.animation-timing-function", CssEasingList, parse_easing_list, { parse_easing_list($input)? };
            AnimationIterationCount, "animation-iteration-count", [], "baseline.property.animation-iteration-count", CssAnimationIterationCountList, parse_animation_iteration_count_list, { parse_animation_iteration_count_list($input)? };
            AnimationDirection, "animation-direction", [], "baseline.property.animation-direction", CssAnimationDirectionList, parse_animation_direction_list, { parse_animation_direction_list($input)? };
            AnimationFillMode, "animation-fill-mode", [], "baseline.property.animation-fill-mode", CssAnimationFillModeList, parse_animation_fill_mode_list, { parse_animation_fill_mode_list($input)? };
            AnimationPlayState, "animation-play-state", [], "baseline.property.animation-play-state", CssAnimationPlayStateList, parse_animation_play_state_list, { parse_animation_play_state_list($input)? };
            Animation, "animation", [], "baseline.property.animation", CssAnimationList, parse_animation_list, { parse_animation_list($input)? };
        }
    };
}

pub(crate) use property_schema;

macro_rules! define_property_identity {
    ($input:ident;
        All, $all_canonical:literal, [$($all_alias:literal),*], $all_stable_id:literal,
        $all_value:ty, $all_parser:ident, $all_dispatch:block;
        $(
        $variant:ident, $canonical:literal, [$($alias:literal),*], $stable_id:literal,
        $value:ty, $parser:ident, $dispatch:block;
    )*) => {
        /// Canonical identity for a recognized non-custom authored CSS property.
        ///
        /// Identity lookup is ASCII-case-insensitive and normalizes aliases to their canonical
        /// property. This type classifies authored syntax only; it does not apply cascade,
        /// substitute variables, or resolve property values.
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub enum CssKnownProperty {
            /// The authored `all` property.
            All,
            $(
                #[doc = concat!("The authored `", $canonical, "` property.")]
                $variant,
            )*
        }

        impl CssKnownProperty {
            /// Looks up a canonical property name or reviewed alias without changing CSS meaning.
            #[must_use]
            pub fn from_name(name: &str) -> Option<Self> {
                property_implementation_inventory()
                    .iter()
                    .find(|row| {
                        row.name.eq_ignore_ascii_case(name)
                            || row.aliases.iter().any(|alias| alias.eq_ignore_ascii_case(name))
                    })
                    .map(|row| row.known_property)
            }

            /// Returns every frozen canonical property identity in schema order.
            #[must_use]
            pub const fn all() -> &'static [Self] {
                KNOWN_PROPERTIES
            }

            /// Returns the canonical lowercase authored spelling.
            #[must_use]
            pub const fn canonical_name(self) -> &'static str {
                match self {
                    Self::All => $all_canonical,
                    $(Self::$variant => $canonical,)*
                }
            }

            /// Returns the stable baseline identity for this authored property.
            #[must_use]
            pub const fn stable_id(self) -> &'static str {
                match self {
                    Self::All => $all_stable_id,
                    $(Self::$variant => $stable_id,)*
                }
            }

            /// Returns reviewed authored aliases that normalize to this property.
            #[must_use]
            pub const fn aliases(self) -> &'static [&'static str] {
                match self {
                    Self::All => &[$($all_alias),*],
                    $(Self::$variant => &[$($alias),*],)*
                }
            }
        }

        /// A property-coupled known declaration in the authored syntax phase.
        ///
        /// Each variant carries only the declared-value type selected by the canonical property
        /// schema, so a value for one property cannot be paired with another property. The `all`
        /// branch has its dedicated no-ordinary-value representation. This syntax does not apply
        /// cascade, perform substitution, or resolve context-dependent values.
        #[non_exhaustive]
        #[derive(Clone, Debug, PartialEq)]
        pub enum CssKnownDeclaration {
            /// An authored `all` value, limited to global or substitution-dependent syntax.
            All(CssAllDeclaredValue),
            $(
                #[doc = concat!(
                    "An authored `", $canonical,
                    "` value coupled to its schema-selected value type."
                )]
                $variant(CssDeclaredValue<$value>),
            )*
        }

        impl CssKnownDeclaration {
            /// Returns the canonical property identity derived from the active value variant.
            #[must_use]
            pub const fn property(&self) -> CssKnownProperty {
                match self {
                    Self::All(_) => CssKnownProperty::All,
                    $(Self::$variant(_) => CssKnownProperty::$variant,)*
                }
            }

            pub(crate) fn from_global(
                property: CssKnownProperty,
                keyword: CssGlobalKeyword,
            ) -> Self {
                match property {
                    CssKnownProperty::All => Self::All(CssAllDeclaredValue::Global(keyword)),
                    $(CssKnownProperty::$variant => {
                        Self::$variant(CssDeclaredValue::Global(keyword))
                    },)*
                }
            }

            pub(crate) fn from_substitution_dependent(
                property: CssKnownProperty,
                value: CssSubstitutionDependentValue,
            ) -> Self {
                match property {
                    CssKnownProperty::All => {
                        Self::All(CssAllDeclaredValue::SubstitutionDependent(value))
                    }
                    $(CssKnownProperty::$variant => {
                        Self::$variant(CssDeclaredValue::SubstitutionDependent(value))
                    },)*
                }
            }
        }

        const KNOWN_PROPERTIES: &[CssKnownProperty] = &[
            CssKnownProperty::All,
            $(CssKnownProperty::$variant,)*
        ];

        const IMPLEMENTED_PROPERTIES: &[PropertyImplementation] = &[
            PropertyImplementation {
                known_property: CssKnownProperty::All,
                name: $all_canonical,
                aliases: &[$($all_alias),*],
                stable_id: $all_stable_id,
                authored_value_type: stringify!($all_value),
                parser: stringify!($all_parser),
            },
            $(
                PropertyImplementation {
                    known_property: CssKnownProperty::$variant,
                    name: $canonical,
                    aliases: &[$($alias),*],
                    stable_id: $stable_id,
                    authored_value_type: stringify!($value),
                    parser: stringify!($parser),
                },
            )*
        ];
    };
}

property_schema!(define_property_identity, schema_input);

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PropertyImplementation {
    pub(crate) known_property: CssKnownProperty,
    pub(crate) name: &'static str,
    pub(crate) aliases: &'static [&'static str],
    pub(crate) stable_id: &'static str,
    pub(crate) authored_value_type: &'static str,
    pub(crate) parser: &'static str,
}

pub(crate) const fn property_implementation_inventory() -> &'static [PropertyImplementation] {
    IMPLEMENTED_PROPERTIES
}

/// The authored value domain of the `overflow` shorthand.
///
/// One or two parsed axis values remain property-specific syntax. This value does not compute
/// scrolling behavior, layout, or used overflow values.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum CssOverflowPropertyValue {
    /// One authored overflow value applying to both axes.
    Single(CssOverflow),
    /// Two authored overflow values preserving distinct axes.
    Pair(CssOverflowAxes),
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn property_schema_inventory_rows_are_bidirectional_and_unique() {
        let inventory = property_implementation_inventory();
        assert_eq!(inventory.len(), 179);

        let mut names = HashSet::new();
        let mut ids = HashSet::new();
        let mut properties = HashSet::new();
        for row in inventory {
            assert!(names.insert(row.name));
            assert!(ids.insert(row.stable_id));
            assert!(properties.insert(row.known_property));
            assert_eq!(row.known_property.canonical_name(), row.name);
            assert_eq!(row.known_property.stable_id(), row.stable_id);
            assert_eq!(row.known_property.aliases(), row.aliases);
            assert_eq!(
                CssKnownProperty::from_name(row.name),
                Some(row.known_property)
            );
            assert!(!row.authored_value_type.is_empty());
            assert!(!row.parser.is_empty());
        }

        assert_eq!(names.len(), 179);
        assert_eq!(ids.len(), 179);
        assert_eq!(properties.len(), 179);
    }

    #[test]
    fn property_schema_alias_inventory_matches_reviewed_empty_set() {
        assert!(
            property_implementation_inventory()
                .iter()
                .all(|row| row.aliases.is_empty())
        );
    }
}

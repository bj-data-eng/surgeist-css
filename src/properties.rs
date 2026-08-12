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
            All, "all", [], "baseline.property.all", CssAllDeclaredValue, CssAllPropertyValue, CssAllPropertyValueRepresentation, parse_all_property, { parse_all_property($input)? };
            Display, "display", [], "baseline.property.display", CssDisplay, CssDisplayPropertyValue, CssDisplayPropertyValueRepresentation, parse_display, { parse_display($input)? };
            BoxSizing, "box-sizing", [], "baseline.property.box-sizing", CssBoxSizing, CssBoxSizingPropertyValue, CssBoxSizingPropertyValueRepresentation, parse_box_sizing, { parse_box_sizing($input)? };
            Position, "position", [], "baseline.property.position", CssLayoutPosition, CssPositionPropertyValue, CssPositionPropertyValueRepresentation, parse_position, { parse_position($input)? };
            Direction, "direction", [], "baseline.property.direction", CssDirection, CssDirectionPropertyValue, CssDirectionPropertyValueRepresentation, parse_direction, { parse_direction($input)? };
            Overflow, "overflow", [], "baseline.property.overflow", CssOverflowI01PropertyValue, CssOverflowPropertyValue, CssOverflowPropertyValueRepresentation, parse_overflow_property, { parse_overflow_property($input)? };
            OverflowX, "overflow-x", [], "baseline.property.overflow-x", CssOverflow, CssOverflowXPropertyValue, CssOverflowXPropertyValueRepresentation, parse_overflow, { parse_overflow($input)? };
            OverflowY, "overflow-y", [], "baseline.property.overflow-y", CssOverflow, CssOverflowYPropertyValue, CssOverflowYPropertyValueRepresentation, parse_overflow, { parse_overflow($input)? };
            FlexDirection, "flex-direction", [], "baseline.property.flex-direction", CssFlexDirection, CssFlexDirectionPropertyValue, CssFlexDirectionPropertyValueRepresentation, parse_flex_direction, { parse_flex_direction($input)? };
            FlexWrap, "flex-wrap", [], "baseline.property.flex-wrap", CssFlexWrap, CssFlexWrapPropertyValue, CssFlexWrapPropertyValueRepresentation, parse_flex_wrap, { parse_flex_wrap($input)? };
            Float, "float", [], "baseline.property.float", CssFloat, CssFloatPropertyValue, CssFloatPropertyValueRepresentation, parse_float, { parse_float($input)? };
            Clear, "clear", [], "baseline.property.clear", CssClear, CssClearPropertyValue, CssClearPropertyValueRepresentation, parse_clear, { parse_clear($input)? };
            AlignContent, "align-content", [], "baseline.property.align-content", CssAlignment, CssAlignContentPropertyValue, CssAlignContentPropertyValueRepresentation, parse_content_alignment, { parse_content_alignment($input)? };
            JustifyContent, "justify-content", [], "baseline.property.justify-content", CssAlignment, CssJustifyContentPropertyValue, CssJustifyContentPropertyValueRepresentation, parse_content_alignment, { parse_content_alignment($input)? };
            AlignItems, "align-items", [], "baseline.property.align-items", CssAlignItems, CssAlignItemsPropertyValue, CssAlignItemsPropertyValueRepresentation, parse_align_items, { parse_align_items($input)? };
            AlignSelf, "align-self", [], "baseline.property.align-self", CssAlignItems, CssAlignSelfPropertyValue, CssAlignSelfPropertyValueRepresentation, parse_align_items, { parse_align_items($input)? };
            JustifyItems, "justify-items", [], "baseline.property.justify-items", CssAlignItems, CssJustifyItemsPropertyValue, CssJustifyItemsPropertyValueRepresentation, parse_align_items, { parse_align_items($input)? };
            JustifySelf, "justify-self", [], "baseline.property.justify-self", CssAlignItems, CssJustifySelfPropertyValue, CssJustifySelfPropertyValueRepresentation, parse_align_items, { parse_align_items($input)? };
            PlaceContent, "place-content", [], "baseline.property.place-content", CssPlaceAlignment, CssPlaceContentPropertyValue, CssPlaceContentPropertyValueRepresentation, parse_place_alignment, { parse_place_alignment($input, parse_content_alignment, CssPlaceAlignment::content)? };
            PlaceItems, "place-items", [], "baseline.property.place-items", CssPlaceAlignment, CssPlaceItemsPropertyValue, CssPlaceItemsPropertyValueRepresentation, parse_place_alignment, { parse_place_alignment($input, parse_align_items, CssPlaceAlignment::items)? };
            PlaceSelf, "place-self", [], "baseline.property.place-self", CssPlaceAlignment, CssPlaceSelfPropertyValue, CssPlaceSelfPropertyValueRepresentation, parse_place_alignment, { parse_place_alignment($input, parse_align_items, CssPlaceAlignment::items)? };
            Visibility, "visibility", [], "baseline.property.visibility", CssVisibility, CssVisibilityPropertyValue, CssVisibilityPropertyValueRepresentation, parse_visibility, { parse_visibility($input)? };
            Content, "content", [], "baseline.property.content", CssContent, CssContentPropertyValue, CssContentPropertyValueRepresentation, parse_content, { parse_content($input)? };
            ContentVisibility, "content-visibility", [], "baseline.property.content-visibility", CssContentVisibility, CssContentVisibilityPropertyValue, CssContentVisibilityPropertyValueRepresentation, parse_content_visibility, { parse_content_visibility($input)? };
            ListStyleType, "list-style-type", [], "baseline.property.list-style-type", CssListStyleType, CssListStyleTypePropertyValue, CssListStyleTypePropertyValueRepresentation, parse_list_style_type, { parse_list_style_type($input)? };
            ListStylePosition, "list-style-position", [], "baseline.property.list-style-position", CssListStylePosition, CssListStylePositionPropertyValue, CssListStylePositionPropertyValueRepresentation, parse_list_style_position, { parse_list_style_position($input)? };
            ListStyleImage, "list-style-image", [], "baseline.property.list-style-image", CssListStyleImage, CssListStyleImagePropertyValue, CssListStyleImagePropertyValueRepresentation, parse_list_style_image, { parse_list_style_image($input)? };
            ListStyle, "list-style", [], "baseline.property.list-style", CssListStyle, CssListStylePropertyValue, CssListStylePropertyValueRepresentation, parse_list_style, { parse_list_style($input)? };
            CounterReset, "counter-reset", [], "baseline.property.counter-reset", CssCounterChanges, CssCounterResetPropertyValue, CssCounterResetPropertyValueRepresentation, parse_counter_changes, { parse_counter_changes($input)? };
            CounterIncrement, "counter-increment", [], "baseline.property.counter-increment", CssCounterChanges, CssCounterIncrementPropertyValue, CssCounterIncrementPropertyValueRepresentation, parse_counter_changes, { parse_counter_changes($input)? };
            CounterSet, "counter-set", [], "baseline.property.counter-set", CssCounterChanges, CssCounterSetPropertyValue, CssCounterSetPropertyValueRepresentation, parse_counter_changes, { parse_counter_changes($input)? };
            Width, "width", [], "baseline.property.width", CssLength, CssWidthPropertyValue, CssWidthPropertyValueRepresentation, parse_box_size_value, { parse_box_size_value($input)? };
            Height, "height", [], "baseline.property.height", CssLength, CssHeightPropertyValue, CssHeightPropertyValueRepresentation, parse_box_size_value, { parse_box_size_value($input)? };
            MinWidth, "min-width", [], "baseline.property.min-width", CssLength, CssMinWidthPropertyValue, CssMinWidthPropertyValueRepresentation, parse_box_size_value, { parse_box_size_value($input)? };
            MinHeight, "min-height", [], "baseline.property.min-height", CssLength, CssMinHeightPropertyValue, CssMinHeightPropertyValueRepresentation, parse_box_size_value, { parse_box_size_value($input)? };
            MaxWidth, "max-width", [], "baseline.property.max-width", CssLength, CssMaxWidthPropertyValue, CssMaxWidthPropertyValueRepresentation, parse_box_size_value, { parse_box_size_value($input)? };
            MaxHeight, "max-height", [], "baseline.property.max-height", CssLength, CssMaxHeightPropertyValue, CssMaxHeightPropertyValueRepresentation, parse_box_size_value, { parse_box_size_value($input)? };
            FlexBasis, "flex-basis", [], "baseline.property.flex-basis", CssLength, CssFlexBasisPropertyValue, CssFlexBasisPropertyValueRepresentation, parse_box_size_value, { parse_box_size_value($input)? };
            Gap, "gap", [], "baseline.property.gap", CssLength, CssGapPropertyValue, CssGapPropertyValueRepresentation, parse_gap_value, { parse_gap_value($input)? };
            RowGap, "row-gap", [], "baseline.property.row-gap", CssLength, CssRowGapPropertyValue, CssRowGapPropertyValueRepresentation, parse_gap_value, { parse_gap_value($input)? };
            ColumnGap, "column-gap", [], "baseline.property.column-gap", CssLength, CssColumnGapPropertyValue, CssColumnGapPropertyValueRepresentation, parse_gap_value, { parse_gap_value($input)? };
            GridFlowTolerance, "grid-flow-tolerance", [], "baseline.property.grid-flow-tolerance", CssGridFlowTolerance, CssGridFlowTolerancePropertyValue, CssGridFlowTolerancePropertyValueRepresentation, parse_grid_flow_tolerance, { parse_grid_flow_tolerance($input)? };
            GridTemplateRows, "grid-template-rows", [], "baseline.property.grid-template-rows", CssGridTrackList, CssGridTemplateRowsPropertyValue, CssGridTemplateRowsPropertyValueRepresentation, parse_grid_track_list, { parse_grid_track_list($input)? };
            GridTemplateColumns, "grid-template-columns", [], "baseline.property.grid-template-columns", CssGridTrackList, CssGridTemplateColumnsPropertyValue, CssGridTemplateColumnsPropertyValueRepresentation, parse_grid_track_list, { parse_grid_track_list($input)? };
            GridTemplateAreas, "grid-template-areas", [], "baseline.property.grid-template-areas", CssGridTemplateAreas, CssGridTemplateAreasPropertyValue, CssGridTemplateAreasPropertyValueRepresentation, parse_grid_template_areas, { parse_grid_template_areas($input)? };
            GridTemplate, "grid-template", [], "baseline.property.grid-template", CssGridTemplate, CssGridTemplatePropertyValue, CssGridTemplatePropertyValueRepresentation, parse_grid_template, { parse_grid_template($input)? };
            GridAutoRows, "grid-auto-rows", [], "baseline.property.grid-auto-rows", CssGridTrackList, CssGridAutoRowsPropertyValue, CssGridAutoRowsPropertyValueRepresentation, parse_grid_track_list, { parse_grid_track_list($input)? };
            GridAutoColumns, "grid-auto-columns", [], "baseline.property.grid-auto-columns", CssGridTrackList, CssGridAutoColumnsPropertyValue, CssGridAutoColumnsPropertyValueRepresentation, parse_grid_track_list, { parse_grid_track_list($input)? };
            GridAutoFlow, "grid-auto-flow", [], "baseline.property.grid-auto-flow", CssGridAutoFlow, CssGridAutoFlowPropertyValue, CssGridAutoFlowPropertyValueRepresentation, parse_grid_auto_flow, { parse_grid_auto_flow($input)? };
            GridRowStart, "grid-row-start", [], "baseline.property.grid-row-start", CssGridLine, CssGridRowStartPropertyValue, CssGridRowStartPropertyValueRepresentation, parse_grid_line, { parse_grid_line($input)? };
            GridRowEnd, "grid-row-end", [], "baseline.property.grid-row-end", CssGridLine, CssGridRowEndPropertyValue, CssGridRowEndPropertyValueRepresentation, parse_grid_line, { parse_grid_line($input)? };
            GridColumnStart, "grid-column-start", [], "baseline.property.grid-column-start", CssGridLine, CssGridColumnStartPropertyValue, CssGridColumnStartPropertyValueRepresentation, parse_grid_line, { parse_grid_line($input)? };
            GridColumnEnd, "grid-column-end", [], "baseline.property.grid-column-end", CssGridLine, CssGridColumnEndPropertyValue, CssGridColumnEndPropertyValueRepresentation, parse_grid_line, { parse_grid_line($input)? };
            GridRow, "grid-row", [], "baseline.property.grid-row", CssGridLineRange, CssGridRowPropertyValue, CssGridRowPropertyValueRepresentation, parse_grid_line_range, { parse_grid_line_range($input)? };
            GridColumn, "grid-column", [], "baseline.property.grid-column", CssGridLineRange, CssGridColumnPropertyValue, CssGridColumnPropertyValueRepresentation, parse_grid_line_range, { parse_grid_line_range($input)? };
            GridArea, "grid-area", [], "baseline.property.grid-area", CssGridArea, CssGridAreaPropertyValue, CssGridAreaPropertyValueRepresentation, parse_grid_area, { parse_grid_area($input)? };
            Grid, "grid", [], "baseline.property.grid", CssGrid, CssGridPropertyValue, CssGridPropertyValueRepresentation, parse_grid, { parse_grid($input)? };
            FontSize, "font-size", [], "baseline.property.font-size", CssLength, CssFontSizePropertyValue, CssFontSizePropertyValueRepresentation, parse_font_size, { parse_font_size($input)? };
            LineHeight, "line-height", [], "baseline.property.line-height", CssLength, CssLineHeightPropertyValue, CssLineHeightPropertyValueRepresentation, parse_line_height, { parse_line_height($input)? };
            WritingMode, "writing-mode", [], "baseline.property.writing-mode", CssWritingMode, CssWritingModePropertyValue, CssWritingModePropertyValueRepresentation, parse_writing_mode, { parse_writing_mode($input)? };
            TextAlign, "text-align", [], "baseline.property.text-align", CssTextAlign, CssTextAlignPropertyValue, CssTextAlignPropertyValueRepresentation, parse_text_align, { parse_text_align($input)? };
            TextAlignLast, "text-align-last", [], "baseline.property.text-align-last", CssTextAlignLast, CssTextAlignLastPropertyValue, CssTextAlignLastPropertyValueRepresentation, parse_text_align_last, { parse_text_align_last($input)? };
            TextIndent, "text-indent", [], "baseline.property.text-indent", CssTextIndent, CssTextIndentPropertyValue, CssTextIndentPropertyValueRepresentation, parse_text_indent, { parse_text_indent($input)? };
            VerticalAlign, "vertical-align", [], "baseline.property.vertical-align", CssVerticalAlign, CssVerticalAlignPropertyValue, CssVerticalAlignPropertyValueRepresentation, parse_vertical_align, { parse_vertical_align($input)? };
            FontFamily, "font-family", [], "baseline.property.font-family", CssFontFamilyList, CssFontFamilyPropertyValue, CssFontFamilyPropertyValueRepresentation, parse_font_family_list, { parse_font_family_list($input)? };
            Font, "font", [], "baseline.property.font", CssFont, CssFontPropertyValue, CssFontPropertyValueRepresentation, parse_font, { parse_font($input)? };
            FontWeight, "font-weight", [], "baseline.property.font-weight", CssFontWeight, CssFontWeightPropertyValue, CssFontWeightPropertyValueRepresentation, parse_font_weight, { parse_font_weight($input)? };
            FontStyle, "font-style", [], "baseline.property.font-style", CssFontStyle, CssFontStylePropertyValue, CssFontStylePropertyValueRepresentation, parse_font_style, { parse_font_style($input)? };
            FontStretch, "font-stretch", [], "baseline.property.font-stretch", CssFontStretch, CssFontStretchPropertyValue, CssFontStretchPropertyValueRepresentation, parse_font_stretch, { parse_font_stretch($input)? };
            FontVariant, "font-variant", [], "baseline.property.font-variant", CssFontVariant, CssFontVariantPropertyValue, CssFontVariantPropertyValueRepresentation, parse_font_variant, { parse_font_variant($input)? };
            FontFeatureSettings, "font-feature-settings", [], "baseline.property.font-feature-settings", CssFontFeatureSettings, CssFontFeatureSettingsPropertyValue, CssFontFeatureSettingsPropertyValueRepresentation, parse_font_feature_settings, { parse_font_feature_settings($input)? };
            LetterSpacing, "letter-spacing", [], "baseline.property.letter-spacing", CssLetterSpacing, CssLetterSpacingPropertyValue, CssLetterSpacingPropertyValueRepresentation, parse_letter_spacing, { parse_letter_spacing($input)? };
            TextWrap, "text-wrap", [], "baseline.property.text-wrap", CssTextWrap, CssTextWrapPropertyValue, CssTextWrapPropertyValueRepresentation, parse_text_wrap, { parse_text_wrap($input)? };
            WhiteSpace, "white-space", [], "baseline.property.white-space", CssWhiteSpace, CssWhiteSpacePropertyValue, CssWhiteSpacePropertyValueRepresentation, parse_white_space, { parse_white_space($input)? };
            WordBreak, "word-break", [], "baseline.property.word-break", CssWordBreak, CssWordBreakPropertyValue, CssWordBreakPropertyValueRepresentation, parse_word_break, { parse_word_break($input)? };
            OverflowWrap, "overflow-wrap", [], "baseline.property.overflow-wrap", CssOverflowWrap, CssOverflowWrapPropertyValue, CssOverflowWrapPropertyValueRepresentation, parse_overflow_wrap, { parse_overflow_wrap($input)? };
            TextOverflow, "text-overflow", [], "baseline.property.text-overflow", CssTextOverflow, CssTextOverflowPropertyValue, CssTextOverflowPropertyValueRepresentation, parse_text_overflow, { parse_text_overflow($input)? };
            TextDecoration, "text-decoration", [], "baseline.property.text-decoration", CssTextDecoration, CssTextDecorationPropertyValue, CssTextDecorationPropertyValueRepresentation, parse_text_decoration, { parse_text_decoration($input)? };
            TextDecorationLine, "text-decoration-line", [], "baseline.property.text-decoration-line", CssTextDecorationLine, CssTextDecorationLinePropertyValue, CssTextDecorationLinePropertyValueRepresentation, parse_text_decoration_line, { parse_text_decoration_line($input)? };
            TextDecorationColor, "text-decoration-color", [], "baseline.property.text-decoration-color", CssColor, CssTextDecorationColorPropertyValue, CssTextDecorationColorPropertyValueRepresentation, parse_color, { parse_color($input)? };
            TextDecorationStyle, "text-decoration-style", [], "baseline.property.text-decoration-style", CssTextDecorationStyle, CssTextDecorationStylePropertyValue, CssTextDecorationStylePropertyValueRepresentation, parse_text_decoration_style, { parse_text_decoration_style($input)? };
            TextDecorationThickness, "text-decoration-thickness", [], "baseline.property.text-decoration-thickness", CssTextDecorationThickness, CssTextDecorationThicknessPropertyValue, CssTextDecorationThicknessPropertyValueRepresentation, parse_text_decoration_thickness, { parse_text_decoration_thickness($input)? };
            TextTransform, "text-transform", [], "baseline.property.text-transform", CssTextTransform, CssTextTransformPropertyValue, CssTextTransformPropertyValueRepresentation, parse_text_transform, { parse_text_transform($input)? };
            Inset, "inset", [], "baseline.property.inset", CssEdges, CssInsetPropertyValue, CssInsetPropertyValueRepresentation, parse_edges, { parse_edges($input, parse_inset_component)? };
            Top, "top", [], "baseline.property.top", CssLength, CssTopPropertyValue, CssTopPropertyValueRepresentation, parse_inset_component, { parse_inset_component($input)? };
            Right, "right", [], "baseline.property.right", CssLength, CssRightPropertyValue, CssRightPropertyValueRepresentation, parse_inset_component, { parse_inset_component($input)? };
            Bottom, "bottom", [], "baseline.property.bottom", CssLength, CssBottomPropertyValue, CssBottomPropertyValueRepresentation, parse_inset_component, { parse_inset_component($input)? };
            Left, "left", [], "baseline.property.left", CssLength, CssLeftPropertyValue, CssLeftPropertyValueRepresentation, parse_inset_component, { parse_inset_component($input)? };
            ZIndex, "z-index", [], "baseline.property.z-index", CssZIndex, CssZIndexPropertyValue, CssZIndexPropertyValueRepresentation, parse_z_index, { parse_z_index($input)? };
            BoxDecorationBreak, "box-decoration-break", [], "baseline.property.box-decoration-break", CssBoxDecorationBreak, CssBoxDecorationBreakPropertyValue, CssBoxDecorationBreakPropertyValueRepresentation, parse_box_decoration_break, { parse_box_decoration_break($input)? };
            Margin, "margin", [], "baseline.property.margin", CssEdges, CssMarginPropertyValue, CssMarginPropertyValueRepresentation, parse_edges, { parse_edges($input, parse_margin_component)? };
            MarginTop, "margin-top", [], "baseline.property.margin-top", CssLength, CssMarginTopPropertyValue, CssMarginTopPropertyValueRepresentation, parse_margin_component, { parse_margin_component($input)? };
            MarginRight, "margin-right", [], "baseline.property.margin-right", CssLength, CssMarginRightPropertyValue, CssMarginRightPropertyValueRepresentation, parse_margin_component, { parse_margin_component($input)? };
            MarginBottom, "margin-bottom", [], "baseline.property.margin-bottom", CssLength, CssMarginBottomPropertyValue, CssMarginBottomPropertyValueRepresentation, parse_margin_component, { parse_margin_component($input)? };
            MarginLeft, "margin-left", [], "baseline.property.margin-left", CssLength, CssMarginLeftPropertyValue, CssMarginLeftPropertyValueRepresentation, parse_margin_component, { parse_margin_component($input)? };
            Padding, "padding", [], "baseline.property.padding", CssEdges, CssPaddingPropertyValue, CssPaddingPropertyValueRepresentation, parse_edges, { parse_edges($input, parse_padding_component)? };
            PaddingTop, "padding-top", [], "baseline.property.padding-top", CssLength, CssPaddingTopPropertyValue, CssPaddingTopPropertyValueRepresentation, parse_padding_component, { parse_padding_component($input)? };
            PaddingRight, "padding-right", [], "baseline.property.padding-right", CssLength, CssPaddingRightPropertyValue, CssPaddingRightPropertyValueRepresentation, parse_padding_component, { parse_padding_component($input)? };
            PaddingBottom, "padding-bottom", [], "baseline.property.padding-bottom", CssLength, CssPaddingBottomPropertyValue, CssPaddingBottomPropertyValueRepresentation, parse_padding_component, { parse_padding_component($input)? };
            PaddingLeft, "padding-left", [], "baseline.property.padding-left", CssLength, CssPaddingLeftPropertyValue, CssPaddingLeftPropertyValueRepresentation, parse_padding_component, { parse_padding_component($input)? };
            Border, "border", [], "baseline.property.border", CssBorder, CssBorderPropertyValue, CssBorderPropertyValueRepresentation, parse_border, { parse_border($input)? };
            BorderTop, "border-top", [], "baseline.property.border-top", CssBorder, CssBorderTopPropertyValue, CssBorderTopPropertyValueRepresentation, parse_border, { parse_border($input)? };
            BorderRight, "border-right", [], "baseline.property.border-right", CssBorder, CssBorderRightPropertyValue, CssBorderRightPropertyValueRepresentation, parse_border, { parse_border($input)? };
            BorderBottom, "border-bottom", [], "baseline.property.border-bottom", CssBorder, CssBorderBottomPropertyValue, CssBorderBottomPropertyValueRepresentation, parse_border, { parse_border($input)? };
            BorderLeft, "border-left", [], "baseline.property.border-left", CssBorder, CssBorderLeftPropertyValue, CssBorderLeftPropertyValueRepresentation, parse_border, { parse_border($input)? };
            BorderWidth, "border-width", [], "baseline.property.border-width", CssEdges, CssBorderWidthPropertyValue, CssBorderWidthPropertyValueRepresentation, parse_edges, { parse_edges($input, parse_border_width_component)? };
            BorderTopWidth, "border-top-width", [], "baseline.property.border-top-width", CssLength, CssBorderTopWidthPropertyValue, CssBorderTopWidthPropertyValueRepresentation, parse_border_width_component, { parse_border_width_component($input)? };
            BorderRightWidth, "border-right-width", [], "baseline.property.border-right-width", CssLength, CssBorderRightWidthPropertyValue, CssBorderRightWidthPropertyValueRepresentation, parse_border_width_component, { parse_border_width_component($input)? };
            BorderBottomWidth, "border-bottom-width", [], "baseline.property.border-bottom-width", CssLength, CssBorderBottomWidthPropertyValue, CssBorderBottomWidthPropertyValueRepresentation, parse_border_width_component, { parse_border_width_component($input)? };
            BorderLeftWidth, "border-left-width", [], "baseline.property.border-left-width", CssLength, CssBorderLeftWidthPropertyValue, CssBorderLeftWidthPropertyValueRepresentation, parse_border_width_component, { parse_border_width_component($input)? };
            Color, "color", [], "baseline.property.color", CssColor, CssColorPropertyValue, CssColorPropertyValueRepresentation, parse_color, { parse_color($input)? };
            Background, "background", [], "baseline.property.background", CssColor, CssBackgroundPropertyValue, CssBackgroundPropertyValueRepresentation, parse_color, { parse_color($input)? };
            BackgroundColor, "background-color", [], "baseline.property.background-color", CssColor, CssBackgroundColorPropertyValue, CssBackgroundColorPropertyValueRepresentation, parse_color, { parse_color($input)? };
            BorderColor, "border-color", [], "baseline.property.border-color", CssColor, CssBorderColorPropertyValue, CssBorderColorPropertyValueRepresentation, parse_color, { parse_color($input)? };
            BorderTopColor, "border-top-color", [], "baseline.property.border-top-color", CssColor, CssBorderTopColorPropertyValue, CssBorderTopColorPropertyValueRepresentation, parse_color, { parse_color($input)? };
            BorderRightColor, "border-right-color", [], "baseline.property.border-right-color", CssColor, CssBorderRightColorPropertyValue, CssBorderRightColorPropertyValueRepresentation, parse_color, { parse_color($input)? };
            BorderBottomColor, "border-bottom-color", [], "baseline.property.border-bottom-color", CssColor, CssBorderBottomColorPropertyValue, CssBorderBottomColorPropertyValueRepresentation, parse_color, { parse_color($input)? };
            BorderLeftColor, "border-left-color", [], "baseline.property.border-left-color", CssColor, CssBorderLeftColorPropertyValue, CssBorderLeftColorPropertyValueRepresentation, parse_color, { parse_color($input)? };
            BackgroundImage, "background-image", [], "baseline.property.background-image", CssImageLayerList, CssBackgroundImagePropertyValue, CssBackgroundImagePropertyValueRepresentation, parse_image_layer_list, { parse_image_layer_list($input)? };
            BackgroundPosition, "background-position", [], "baseline.property.background-position", CssPositionList, CssBackgroundPositionPropertyValue, CssBackgroundPositionPropertyValueRepresentation, parse_position_list, { parse_position_list($input)? };
            BackgroundSize, "background-size", [], "baseline.property.background-size", CssBackgroundSizeList, CssBackgroundSizePropertyValue, CssBackgroundSizePropertyValueRepresentation, parse_background_size_list, { parse_background_size_list($input)? };
            BackgroundRepeat, "background-repeat", [], "baseline.property.background-repeat", CssBackgroundRepeatList, CssBackgroundRepeatPropertyValue, CssBackgroundRepeatPropertyValueRepresentation, parse_background_repeat_list, { parse_background_repeat_list($input)? };
            BackgroundOrigin, "background-origin", [], "baseline.property.background-origin", CssBackgroundBox, CssBackgroundOriginPropertyValue, CssBackgroundOriginPropertyValueRepresentation, parse_background_box, { parse_background_box($input)? };
            BackgroundClip, "background-clip", [], "baseline.property.background-clip", CssBackgroundBox, CssBackgroundClipPropertyValue, CssBackgroundClipPropertyValueRepresentation, parse_background_box, { parse_background_box($input)? };
            BackgroundAttachment, "background-attachment", [], "baseline.property.background-attachment", CssBackgroundAttachmentList, CssBackgroundAttachmentPropertyValue, CssBackgroundAttachmentPropertyValueRepresentation, parse_background_attachment_list, { parse_background_attachment_list($input)? };
            BorderStyle, "border-style", [], "baseline.property.border-style", CssBorderStyles, CssBorderStylePropertyValue, CssBorderStylePropertyValueRepresentation, parse_border_styles, { parse_border_styles($input)? };
            BorderTopStyle, "border-top-style", [], "baseline.property.border-top-style", CssBorderStyle, CssBorderTopStylePropertyValue, CssBorderTopStylePropertyValueRepresentation, parse_border_style, { parse_border_style($input)? };
            BorderRightStyle, "border-right-style", [], "baseline.property.border-right-style", CssBorderStyle, CssBorderRightStylePropertyValue, CssBorderRightStylePropertyValueRepresentation, parse_border_style, { parse_border_style($input)? };
            BorderBottomStyle, "border-bottom-style", [], "baseline.property.border-bottom-style", CssBorderStyle, CssBorderBottomStylePropertyValue, CssBorderBottomStylePropertyValueRepresentation, parse_border_style, { parse_border_style($input)? };
            BorderLeftStyle, "border-left-style", [], "baseline.property.border-left-style", CssBorderStyle, CssBorderLeftStylePropertyValue, CssBorderLeftStylePropertyValueRepresentation, parse_border_style, { parse_border_style($input)? };
            BorderRadius, "border-radius", [], "baseline.property.border-radius", CssBorderRadii, CssBorderRadiusPropertyValue, CssBorderRadiusPropertyValueRepresentation, parse_border_radius, { parse_border_radius($input)? };
            BorderTopLeftRadius, "border-top-left-radius", [], "baseline.property.border-top-left-radius", CssCornerRadius, CssBorderTopLeftRadiusPropertyValue, CssBorderTopLeftRadiusPropertyValueRepresentation, parse_corner_radius, { parse_corner_radius($input)? };
            BorderTopRightRadius, "border-top-right-radius", [], "baseline.property.border-top-right-radius", CssCornerRadius, CssBorderTopRightRadiusPropertyValue, CssBorderTopRightRadiusPropertyValueRepresentation, parse_corner_radius, { parse_corner_radius($input)? };
            BorderBottomRightRadius, "border-bottom-right-radius", [], "baseline.property.border-bottom-right-radius", CssCornerRadius, CssBorderBottomRightRadiusPropertyValue, CssBorderBottomRightRadiusPropertyValueRepresentation, parse_corner_radius, { parse_corner_radius($input)? };
            BorderBottomLeftRadius, "border-bottom-left-radius", [], "baseline.property.border-bottom-left-radius", CssCornerRadius, CssBorderBottomLeftRadiusPropertyValue, CssBorderBottomLeftRadiusPropertyValueRepresentation, parse_corner_radius, { parse_corner_radius($input)? };
            BoxShadow, "box-shadow", [], "baseline.property.box-shadow", CssBoxShadow, CssBoxShadowPropertyValue, CssBoxShadowPropertyValueRepresentation, parse_box_shadow, { parse_box_shadow($input)? };
            Opacity, "opacity", [], "baseline.property.opacity", CssOpacity, CssOpacityPropertyValue, CssOpacityPropertyValueRepresentation, parse_opacity, { parse_opacity($input)? };
            FlexGrow, "flex-grow", [], "baseline.property.flex-grow", CssFlexFactor, CssFlexGrowPropertyValue, CssFlexGrowPropertyValueRepresentation, parse_flex_factor, { parse_flex_factor($input, "flex-grow")? };
            FlexShrink, "flex-shrink", [], "baseline.property.flex-shrink", CssFlexFactor, CssFlexShrinkPropertyValue, CssFlexShrinkPropertyValueRepresentation, parse_flex_factor, { parse_flex_factor($input, "flex-shrink")? };
            Order, "order", [], "baseline.property.order", CssOrder, CssOrderPropertyValue, CssOrderPropertyValueRepresentation, parse_order, { parse_order($input)? };
            Flex, "flex", [], "baseline.property.flex", CssFlex, CssFlexPropertyValue, CssFlexPropertyValueRepresentation, parse_flex, { parse_flex($input)? };
            JustifyTracks, "justify-tracks", [], "baseline.property.justify-tracks", CssAlignment, CssJustifyTracksPropertyValue, CssJustifyTracksPropertyValueRepresentation, parse_content_alignment, { parse_content_alignment($input)? };
            AlignTracks, "align-tracks", [], "baseline.property.align-tracks", CssAlignment, CssAlignTracksPropertyValue, CssAlignTracksPropertyValueRepresentation, parse_content_alignment, { parse_content_alignment($input)? };
            AspectRatio, "aspect-ratio", [], "baseline.property.aspect-ratio", CssAspectRatio, CssAspectRatioPropertyValue, CssAspectRatioPropertyValueRepresentation, parse_aspect_ratio, { parse_aspect_ratio($input)? };
            ScrollbarWidth, "scrollbar-width", [], "baseline.property.scrollbar-width", CssScrollbarWidth, CssScrollbarWidthPropertyValue, CssScrollbarWidthPropertyValueRepresentation, parse_scrollbar_width, { parse_scrollbar_width($input)? };
            Cursor, "cursor", [], "baseline.property.cursor", CssCursor, CssCursorPropertyValue, CssCursorPropertyValueRepresentation, parse_cursor, { parse_cursor($input)? };
            PointerEvents, "pointer-events", [], "baseline.property.pointer-events", CssPointerEvents, CssPointerEventsPropertyValue, CssPointerEventsPropertyValueRepresentation, parse_pointer_events, { parse_pointer_events($input)? };
            UserSelect, "user-select", [], "baseline.property.user-select", CssUserSelect, CssUserSelectPropertyValue, CssUserSelectPropertyValueRepresentation, parse_user_select, { parse_user_select($input)? };
            Outline, "outline", [], "baseline.property.outline", CssOutline, CssOutlinePropertyValue, CssOutlinePropertyValueRepresentation, parse_outline, { parse_outline($input)? };
            OutlineColor, "outline-color", [], "baseline.property.outline-color", CssColor, CssOutlineColorPropertyValue, CssOutlineColorPropertyValueRepresentation, parse_color, { parse_color($input)? };
            OutlineStyle, "outline-style", [], "baseline.property.outline-style", CssOutlineStyle, CssOutlineStylePropertyValue, CssOutlineStylePropertyValueRepresentation, parse_outline_style, { parse_outline_style($input)? };
            OutlineWidth, "outline-width", [], "baseline.property.outline-width", CssOutlineWidth, CssOutlineWidthPropertyValue, CssOutlineWidthPropertyValueRepresentation, parse_outline_width, { parse_outline_width($input)? };
            Transform, "transform", [], "baseline.property.transform", CssTransform, CssTransformPropertyValue, CssTransformPropertyValueRepresentation, parse_transform, { parse_transform($input)? };
            TransformOrigin, "transform-origin", [], "baseline.property.transform-origin", CssPosition, CssTransformOriginPropertyValue, CssTransformOriginPropertyValueRepresentation, parse_css_position, { parse_css_position($input)? };
            Translate, "translate", [], "baseline.property.translate", CssTranslate, CssTranslatePropertyValue, CssTranslatePropertyValueRepresentation, parse_translate, { parse_translate($input)? };
            Rotate, "rotate", [], "baseline.property.rotate", CssRotate, CssRotatePropertyValue, CssRotatePropertyValueRepresentation, parse_rotate, { parse_rotate($input)? };
            Scale, "scale", [], "baseline.property.scale", CssScale, CssScalePropertyValue, CssScalePropertyValueRepresentation, parse_scale, { parse_scale($input)? };
            Filter, "filter", [], "baseline.property.filter", CssFilter, CssFilterPropertyValue, CssFilterPropertyValueRepresentation, parse_filter, { parse_filter($input)? };
            BackdropFilter, "backdrop-filter", [], "baseline.property.backdrop-filter", CssFilter, CssBackdropFilterPropertyValue, CssBackdropFilterPropertyValueRepresentation, parse_filter, { parse_filter($input)? };
            ClipPath, "clip-path", [], "baseline.property.clip-path", CssClipPath, CssClipPathPropertyValue, CssClipPathPropertyValueRepresentation, parse_clip_path, { parse_clip_path($input)? };
            Mask, "mask", [], "baseline.property.mask", CssMaskList, CssMaskPropertyValue, CssMaskPropertyValueRepresentation, parse_mask_list, { parse_mask_list($input)? };
            MaskImage, "mask-image", [], "baseline.property.mask-image", CssImageLayerList, CssMaskImagePropertyValue, CssMaskImagePropertyValueRepresentation, parse_image_layer_list, { parse_image_layer_list($input)? };
            MaskSize, "mask-size", [], "baseline.property.mask-size", CssBackgroundSizeList, CssMaskSizePropertyValue, CssMaskSizePropertyValueRepresentation, parse_background_size_list, { parse_background_size_list($input)? };
            MaskPosition, "mask-position", [], "baseline.property.mask-position", CssPositionList, CssMaskPositionPropertyValue, CssMaskPositionPropertyValueRepresentation, parse_position_list, { parse_position_list($input)? };
            MaskRepeat, "mask-repeat", [], "baseline.property.mask-repeat", CssBackgroundRepeatList, CssMaskRepeatPropertyValue, CssMaskRepeatPropertyValueRepresentation, parse_background_repeat_list, { parse_background_repeat_list($input)? };
            TransitionProperty, "transition-property", [], "baseline.property.transition-property", CssTransitionPropertyList, CssTransitionPropertyPropertyValue, CssTransitionPropertyPropertyValueRepresentation, parse_transition_property_list, { parse_transition_property_list($input)? };
            TransitionDuration, "transition-duration", [], "baseline.property.transition-duration", CssTimeList, CssTransitionDurationPropertyValue, CssTransitionDurationPropertyValueRepresentation, parse_time_list, { parse_time_list($input)? };
            TransitionDelay, "transition-delay", [], "baseline.property.transition-delay", CssTimeList, CssTransitionDelayPropertyValue, CssTransitionDelayPropertyValueRepresentation, parse_time_list, { parse_time_list($input)? };
            TransitionTimingFunction, "transition-timing-function", [], "baseline.property.transition-timing-function", CssEasingList, CssTransitionTimingFunctionPropertyValue, CssTransitionTimingFunctionPropertyValueRepresentation, parse_easing_list, { parse_easing_list($input)? };
            Transition, "transition", [], "baseline.property.transition", CssTransitionList, CssTransitionPropertyValue, CssTransitionPropertyValueRepresentation, parse_transition_list, { parse_transition_list($input)? };
            AnimationName, "animation-name", [], "baseline.property.animation-name", CssAnimationNameList, CssAnimationNamePropertyValue, CssAnimationNamePropertyValueRepresentation, parse_animation_name_list, { parse_animation_name_list($input)? };
            AnimationDuration, "animation-duration", [], "baseline.property.animation-duration", CssTimeList, CssAnimationDurationPropertyValue, CssAnimationDurationPropertyValueRepresentation, parse_time_list, { parse_time_list($input)? };
            AnimationDelay, "animation-delay", [], "baseline.property.animation-delay", CssTimeList, CssAnimationDelayPropertyValue, CssAnimationDelayPropertyValueRepresentation, parse_time_list, { parse_time_list($input)? };
            AnimationTimingFunction, "animation-timing-function", [], "baseline.property.animation-timing-function", CssEasingList, CssAnimationTimingFunctionPropertyValue, CssAnimationTimingFunctionPropertyValueRepresentation, parse_easing_list, { parse_easing_list($input)? };
            AnimationIterationCount, "animation-iteration-count", [], "baseline.property.animation-iteration-count", CssAnimationIterationCountList, CssAnimationIterationCountPropertyValue, CssAnimationIterationCountPropertyValueRepresentation, parse_animation_iteration_count_list, { parse_animation_iteration_count_list($input)? };
            AnimationDirection, "animation-direction", [], "baseline.property.animation-direction", CssAnimationDirectionList, CssAnimationDirectionPropertyValue, CssAnimationDirectionPropertyValueRepresentation, parse_animation_direction_list, { parse_animation_direction_list($input)? };
            AnimationFillMode, "animation-fill-mode", [], "baseline.property.animation-fill-mode", CssAnimationFillModeList, CssAnimationFillModePropertyValue, CssAnimationFillModePropertyValueRepresentation, parse_animation_fill_mode_list, { parse_animation_fill_mode_list($input)? };
            AnimationPlayState, "animation-play-state", [], "baseline.property.animation-play-state", CssAnimationPlayStateList, CssAnimationPlayStatePropertyValue, CssAnimationPlayStatePropertyValueRepresentation, parse_animation_play_state_list, { parse_animation_play_state_list($input)? };
            Animation, "animation", [], "baseline.property.animation", CssAnimationList, CssAnimationPropertyValue, CssAnimationPropertyValueRepresentation, parse_animation_list, { parse_animation_list($input)? };
        }
    };
}

pub(crate) use property_schema;

macro_rules! define_property_identity {
    ($input:ident;
        All, $all_canonical:literal, [$($all_alias:literal),*], $all_stable_id:literal,
        $all_value:ty, $all_wrapper:ident, $all_representation:ident,
        $all_parser:ident, $all_dispatch:block;
        $(
        $variant:ident, $canonical:literal, [$($alias:literal),*], $stable_id:literal,
        $value:ty, $wrapper:ident, $representation:ident, $parser:ident, $dispatch:block;
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

        #[derive(Clone, Debug, PartialEq)]
        struct $all_representation($all_value);

        /// The authored ordinary value of the `all` schema row.
        ///
        /// The current grammar never constructs this wrapper because `all` accepts only global or
        /// substitution-dependent values. Its presence keeps the generated schema/view inventory
        /// exact without making either symbolic branch masquerade as an ordinary property value.
        #[derive(Clone, Debug, PartialEq)]
        pub struct $all_wrapper {
            authored: CssAuthoredDeclarationValue,
            representation: $all_representation,
        }

        impl $all_wrapper {
            /// Returns the exact authored ordinary value slice.
            #[must_use]
            pub fn as_css(&self) -> &str {
                self.authored.as_css()
            }

            /// Returns the I01 payload when this value belongs to the frozen I01 subset.
            #[must_use]
            pub const fn i01_subset(&self) -> Option<&$all_value> {
                Some(&self.representation.0)
            }
        }

        $(
            #[derive(Clone, Debug, PartialEq)]
            enum $representation {
                I01($value),
            }

            #[doc = concat!(
                "A parser-produced authored ordinary value for `", $canonical, "`."
            )]
            ///
            /// The private representation preserves property coupling while `as_css()` retains
            /// the exact authored slice and `i01_subset()` exposes only the frozen I01 payload.
            #[derive(Clone, Debug, PartialEq)]
            pub struct $wrapper {
                authored: CssAuthoredDeclarationValue,
                representation: $representation,
            }

            impl $wrapper {
                #[must_use]
                pub(crate) const fn new(
                    authored: CssAuthoredDeclarationValue,
                    value: $value,
                ) -> Self {
                    Self {
                        authored,
                        representation: $representation::I01(value),
                    }
                }

                /// Returns the exact authored ordinary value slice, excluding boundary trivia and
                /// a terminal importance annotation.
                #[must_use]
                pub fn as_css(&self) -> &str {
                    self.authored.as_css()
                }

                /// Returns the property parser's frozen I01 payload when this value belongs to
                /// that subset.
                #[must_use]
                pub const fn i01_subset(&self) -> Option<&$value> {
                    match &self.representation {
                        $representation::I01(value) => Some(value),
                    }
                }
            }
        )*

        /// A borrowed property-specific ordinary-value view.
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CssKnownPropertyValueRef<'a> {
            /// The generated `all` schema-row wrapper. Current parsing never produces this branch.
            All(&'a $all_wrapper),
            $(
                #[doc = concat!("An authored `", $canonical, "` ordinary value.")]
                $variant(&'a $wrapper),
            )*
        }

        /// A borrowed known declaration value in the authored syntax phase.
        #[non_exhaustive]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum CssKnownDeclaredValueRef<'a> {
            /// A property-specific ordinary authored value.
            Property(CssKnownPropertyValueRef<'a>),
            /// A whole-value CSS-wide keyword.
            Global(CssGlobalKeyword),
            /// Authored syntax whose grammar depends on later substitution.
            SubstitutionDependent(&'a CssSubstitutionDependentValue),
        }

        #[derive(Clone, Debug, PartialEq)]
        pub(crate) enum CssKnownDeclarationValue {
            All(CssAllDeclaredValue),
            $($variant(CssDeclaredValue<$wrapper>),)*
        }

        /// A parser-owned property-coupled known declaration in the authored syntax phase.
        ///
        /// Its private field is the sole property identity and value discriminator. Callers can
        /// inspect, but cannot construct or mutate, a property/value mismatch.
        ///
        /// ```compile_fail
        /// use surgeist_css::CssKnownDeclaration;
        /// let _ = CssKnownDeclaration { value: todo!() };
        /// ```
        ///
        /// ```compile_fail
        /// use surgeist_css::{CssAuthoredDeclarationValue, CssWidthPropertyValue};
        /// let _ = CssWidthPropertyValue {
        ///     authored: CssAuthoredDeclarationValue::try_new("1px").unwrap(),
        ///     representation: todo!(),
        /// };
        /// ```
        #[derive(Clone, Debug, PartialEq)]
        pub struct CssKnownDeclaration {
            value: CssKnownDeclarationValue,
        }

        impl CssKnownDeclaration {
            /// Returns the canonical property identity derived from the active value variant.
            #[must_use]
            pub const fn property(&self) -> CssKnownProperty {
                match &self.value {
                    CssKnownDeclarationValue::All(_) => CssKnownProperty::All,
                    $(CssKnownDeclarationValue::$variant(_) => CssKnownProperty::$variant,)*
                }
            }

            /// Returns exactly one borrowed declared-value branch.
            #[must_use]
            pub const fn declared_value(&self) -> CssKnownDeclaredValueRef<'_> {
                match &self.value {
                    CssKnownDeclarationValue::All(CssAllDeclaredValue::Global(keyword)) => {
                        CssKnownDeclaredValueRef::Global(*keyword)
                    }
                    CssKnownDeclarationValue::All(
                        CssAllDeclaredValue::SubstitutionDependent(value),
                    ) => CssKnownDeclaredValueRef::SubstitutionDependent(value),
                    $(
                        CssKnownDeclarationValue::$variant(CssDeclaredValue::Value(value)) => {
                            CssKnownDeclaredValueRef::Property(
                                CssKnownPropertyValueRef::$variant(value),
                            )
                        }
                        CssKnownDeclarationValue::$variant(CssDeclaredValue::Global(keyword)) => {
                            CssKnownDeclaredValueRef::Global(*keyword)
                        }
                        CssKnownDeclarationValue::$variant(
                            CssDeclaredValue::SubstitutionDependent(value),
                        ) => CssKnownDeclaredValueRef::SubstitutionDependent(value),
                    )*
                }
            }

            /// Returns the property-specific ordinary-value view when present.
            #[must_use]
            pub const fn property_value(&self) -> Option<CssKnownPropertyValueRef<'_>> {
                match self.declared_value() {
                    CssKnownDeclaredValueRef::Property(value) => Some(value),
                    CssKnownDeclaredValueRef::Global(_)
                    | CssKnownDeclaredValueRef::SubstitutionDependent(_) => None,
                }
            }

            /// Returns the CSS-wide keyword when present.
            #[must_use]
            pub const fn global(&self) -> Option<CssGlobalKeyword> {
                match self.declared_value() {
                    CssKnownDeclaredValueRef::Global(keyword) => Some(keyword),
                    CssKnownDeclaredValueRef::Property(_)
                    | CssKnownDeclaredValueRef::SubstitutionDependent(_) => None,
                }
            }

            /// Returns the substitution-dependent authored value when present.
            #[must_use]
            pub const fn substitution_dependent(
                &self,
            ) -> Option<&CssSubstitutionDependentValue> {
                match self.declared_value() {
                    CssKnownDeclaredValueRef::SubstitutionDependent(value) => Some(value),
                    CssKnownDeclaredValueRef::Property(_)
                    | CssKnownDeclaredValueRef::Global(_) => None,
                }
            }

            pub(crate) const fn from_value(value: CssKnownDeclarationValue) -> Self {
                Self { value }
            }

            pub(crate) fn from_global(
                property: CssKnownProperty,
                keyword: CssGlobalKeyword,
            ) -> Self {
                let value = match property {
                    CssKnownProperty::All => {
                        CssKnownDeclarationValue::All(CssAllDeclaredValue::Global(keyword))
                    }
                    $(CssKnownProperty::$variant => {
                        CssKnownDeclarationValue::$variant(CssDeclaredValue::Global(keyword))
                    },)*
                };
                Self { value }
            }

            pub(crate) fn from_substitution_dependent(
                property: CssKnownProperty,
                value: CssSubstitutionDependentValue,
            ) -> Self {
                let value = match property {
                    CssKnownProperty::All => {
                        CssKnownDeclarationValue::All(
                            CssAllDeclaredValue::SubstitutionDependent(value),
                        )
                    }
                    $(CssKnownProperty::$variant => {
                        CssKnownDeclarationValue::$variant(
                            CssDeclaredValue::SubstitutionDependent(value),
                        )
                    },)*
                };
                Self { value }
            }
        }

        const KNOWN_PROPERTIES: &[CssKnownProperty] = &[
            CssKnownProperty::All,
            $(CssKnownProperty::$variant,)*
        ];

        const IMPLEMENTED_PROPERTIES: &[PropertyImplementation] = &[
            PropertyImplementation {
                known_property: CssKnownProperty::All,
                schema_variant: stringify!(All),
                name: $all_canonical,
                aliases: &[$($all_alias),*],
                stable_id: $all_stable_id,
                authored_value_type: stringify!($all_value),
                wrapper: stringify!($all_wrapper),
                representation: stringify!($all_representation),
                parser: stringify!($all_parser),
            },
            $(
                PropertyImplementation {
                    known_property: CssKnownProperty::$variant,
                    schema_variant: stringify!($variant),
                    name: $canonical,
                    aliases: &[$($alias),*],
                    stable_id: $stable_id,
                    authored_value_type: stringify!($value),
                    wrapper: stringify!($wrapper),
                    representation: stringify!($representation),
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
    pub(crate) schema_variant: &'static str,
    pub(crate) name: &'static str,
    pub(crate) aliases: &'static [&'static str],
    pub(crate) stable_id: &'static str,
    pub(crate) authored_value_type: &'static str,
    pub(crate) wrapper: &'static str,
    pub(crate) representation: &'static str,
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
pub enum CssOverflowI01PropertyValue {
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
        let mut wrappers = HashSet::new();
        let mut representations = HashSet::new();
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
            assert!(wrappers.insert(row.wrapper));
            assert!(representations.insert(row.representation));
            assert_eq!(
                row.wrapper,
                format!("Css{}PropertyValue", row.schema_variant)
            );
            assert_eq!(
                row.representation,
                format!("Css{}PropertyValueRepresentation", row.schema_variant)
            );
        }

        assert_eq!(names.len(), 179);
        assert_eq!(ids.len(), 179);
        assert_eq!(properties.len(), 179);
        assert_eq!(wrappers.len(), 179);
        assert_eq!(representations.len(), 179);
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

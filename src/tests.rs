use super::*;
use crate::test_support::{
    AcceptedDeclarationCase, AcceptedValueCase, ExpectedErrorKind, RejectedDeclarationCase,
    RejectedSheetCase, accepted_declaration_cases, assert_accepts_declarations,
    assert_accepts_value_cases, assert_rejects_declarations, assert_rejects_sheets,
    assert_sheet_rejected, parse_single_declaration, parse_single_declaration_value,
};

fn declaration_value(input: &str, property: CssProperty) -> CssValue {
    let sheet = parse_sheet(input).unwrap();
    sheet.rules()[0]
        .declarations()
        .iter()
        .find(|declaration| declaration.property() == property)
        .unwrap()
        .value()
        .clone()
}

fn declaration(input: &str, property: CssProperty) -> CssDeclaration {
    let sheet = parse_sheet(input).unwrap();
    sheet.rules()[0]
        .declarations()
        .iter()
        .find(|declaration| declaration.property() == property)
        .unwrap()
        .clone()
}

fn single_declaration(input: &str) -> CssDeclaration {
    let sheet = parse_sheet(input).unwrap();
    let [rule] = sheet.rules() else {
        panic!("{input} should parse exactly one rule");
    };
    let [declaration] = rule.declarations() else {
        panic!("{input} should parse exactly one declaration");
    };
    declaration.clone()
}

#[test]
fn background_color_preserves_authored_property_identity() {
    let declaration = single_declaration(".panel { background-color: black; }");
    assert_eq!(declaration.property(), CssProperty::BackgroundColor);
    assert_eq!(declaration.value(), &CssValue::Color(CssColor::BLACK));
}

fn assert_global_value(value: &CssValue) {
    assert!(matches!(value, CssValue::GlobalKeyword(_)));
}

fn assert_display_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Display(_)));
}

fn assert_box_sizing_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BoxSizing(_)));
}

fn assert_position_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Position(_)));
}

fn assert_direction_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Direction(_)));
}

fn assert_overflow_value(value: &CssValue) {
    assert!(matches!(
        value,
        CssValue::Overflow(_) | CssValue::OverflowAxes(_)
    ));
}

fn assert_flex_direction_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FlexDirection(_)));
}

fn assert_flex_wrap_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FlexWrap(_)));
}

fn assert_float_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Float(_)));
}

fn assert_clear_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Clear(_)));
}

fn assert_alignment_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Alignment(_)));
}

fn assert_align_items_value(value: &CssValue) {
    assert!(matches!(value, CssValue::AlignItems(_)));
}

fn assert_place_alignment_value(value: &CssValue) {
    assert!(matches!(value, CssValue::PlaceAlignment(_)));
}

fn assert_visibility_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Visibility(_)));
}

fn assert_content_visibility_value(value: &CssValue) {
    assert!(matches!(value, CssValue::ContentVisibility(_)));
}

fn assert_length_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Length(_)));
}

fn assert_edges_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Edges(_)));
}

fn assert_color_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Color(_)));
}

fn assert_border_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Border(_)));
}

fn assert_border_style_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BorderStyle(_)));
}

fn assert_border_styles_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BorderStyles(_)));
}

fn assert_border_radius_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BorderRadius(_)));
}

fn assert_corner_radius_value(value: &CssValue) {
    assert!(matches!(value, CssValue::CornerRadius(_)));
}

fn assert_box_shadow_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BoxShadow(_)));
}

fn assert_opacity_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Opacity(_)));
}

fn assert_flex_grow_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FlexGrow(_)));
}

fn assert_flex_shrink_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FlexShrink(_)));
}

fn assert_aspect_ratio_value(value: &CssValue) {
    assert!(matches!(value, CssValue::AspectRatio(_)));
}

fn assert_scrollbar_width_value(value: &CssValue) {
    assert!(matches!(value, CssValue::ScrollbarWidth(_)));
}

fn assert_order_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Order(_)));
}

fn assert_flex_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Flex(_)));
}

fn assert_z_index_value(value: &CssValue) {
    assert!(matches!(value, CssValue::ZIndex(_)));
}

fn assert_box_decoration_break_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BoxDecorationBreak(_)));
}

fn assert_grid_flow_tolerance_value(value: &CssValue) {
    assert!(matches!(value, CssValue::GridFlowTolerance(_)));
}

fn assert_grid_track_list_value(value: &CssValue) {
    assert!(matches!(value, CssValue::GridTrackList(_)));
}

fn assert_grid_template_areas_value(value: &CssValue) {
    assert!(matches!(value, CssValue::GridTemplateAreas(_)));
}

fn assert_grid_template_value(value: &CssValue) {
    assert!(matches!(value, CssValue::GridTemplate(_)));
}

fn assert_grid_auto_flow_value(value: &CssValue) {
    assert!(matches!(value, CssValue::GridAutoFlow(_)));
}

fn assert_grid_line_value(value: &CssValue) {
    assert!(matches!(value, CssValue::GridLine(_)));
}

fn assert_grid_line_range_value(value: &CssValue) {
    assert!(matches!(value, CssValue::GridLineRange(_)));
}

fn assert_grid_area_value(value: &CssValue) {
    assert!(matches!(value, CssValue::GridArea(_)));
}

fn assert_grid_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Grid(_)));
}

fn assert_writing_mode_value(value: &CssValue) {
    assert!(matches!(value, CssValue::WritingMode(_)));
}

fn assert_text_align_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextAlign(_)));
}

fn assert_text_align_last_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextAlignLast(_)));
}

fn assert_text_indent_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextIndent(_)));
}

fn assert_vertical_align_value(value: &CssValue) {
    assert!(matches!(value, CssValue::VerticalAlign(_)));
}

fn assert_font_family_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FontFamily(_)));
}

fn assert_font_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Font(_)));
}

fn assert_font_weight_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FontWeight(_)));
}

fn assert_font_style_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FontStyle(_)));
}

fn assert_font_stretch_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FontStretch(_)));
}

fn assert_font_variant_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FontVariant(_)));
}

fn assert_font_feature_settings_value(value: &CssValue) {
    assert!(matches!(value, CssValue::FontFeatureSettings(_)));
}

fn assert_letter_spacing_value(value: &CssValue) {
    assert!(matches!(value, CssValue::LetterSpacing(_)));
}

fn assert_text_wrap_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextWrap(_)));
}

fn assert_white_space_value(value: &CssValue) {
    assert!(matches!(value, CssValue::WhiteSpace(_)));
}

fn assert_word_break_value(value: &CssValue) {
    assert!(matches!(value, CssValue::WordBreak(_)));
}

fn assert_overflow_wrap_value(value: &CssValue) {
    assert!(matches!(value, CssValue::OverflowWrap(_)));
}

fn assert_text_overflow_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextOverflow(_)));
}

fn assert_text_decoration_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextDecoration(_)));
}

fn assert_text_decoration_line_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextDecorationLine(_)));
}

fn assert_text_decoration_color_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextDecorationColor(_)));
}

fn assert_text_decoration_style_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextDecorationStyle(_)));
}

fn assert_text_decoration_thickness_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextDecorationThickness(_)));
}

fn assert_text_transform_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TextTransform(_)));
}

fn assert_background_image_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BackgroundImage(_)));
}

fn assert_background_position_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BackgroundPosition(_)));
}

fn assert_background_size_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BackgroundSize(_)));
}

fn assert_background_repeat_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BackgroundRepeat(_)));
}

fn assert_background_box_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BackgroundBox(_)));
}

fn assert_background_attachment_value(value: &CssValue) {
    assert!(matches!(value, CssValue::BackgroundAttachment(_)));
}

fn assert_cursor_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Cursor(_)));
}

fn assert_pointer_events_value(value: &CssValue) {
    assert!(matches!(value, CssValue::PointerEvents(_)));
}

fn assert_user_select_value(value: &CssValue) {
    assert!(matches!(value, CssValue::UserSelect(_)));
}

fn assert_outline_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Outline(_)));
}

fn assert_outline_color_value(value: &CssValue) {
    assert!(matches!(value, CssValue::OutlineColor(_)));
}

fn assert_outline_style_value(value: &CssValue) {
    assert!(matches!(value, CssValue::OutlineStyle(_)));
}

fn assert_outline_width_value(value: &CssValue) {
    assert!(matches!(value, CssValue::OutlineWidth(_)));
}

fn assert_transform_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Transform(_)));
}

fn assert_transform_origin_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TransformOrigin(_)));
}

fn assert_translate_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Translate(_)));
}

fn assert_rotate_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Rotate(_)));
}

fn assert_scale_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Scale(_)));
}

fn assert_filter_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Filter(_)));
}

fn assert_clip_path_value(value: &CssValue) {
    assert!(matches!(value, CssValue::ClipPath(_)));
}

fn assert_mask_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Mask(_)));
}

fn assert_mask_image_value(value: &CssValue) {
    assert!(matches!(value, CssValue::MaskImage(_)));
}

fn assert_mask_size_value(value: &CssValue) {
    assert!(matches!(value, CssValue::MaskSize(_)));
}

fn assert_mask_position_value(value: &CssValue) {
    assert!(matches!(value, CssValue::MaskPosition(_)));
}

fn assert_mask_repeat_value(value: &CssValue) {
    assert!(matches!(value, CssValue::MaskRepeat(_)));
}

fn assert_transition_property_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TransitionProperty(_)));
}

fn assert_time_list_value(value: &CssValue) {
    assert!(matches!(value, CssValue::TimeList(_)));
}

fn assert_easing_list_value(value: &CssValue) {
    assert!(matches!(value, CssValue::EasingList(_)));
}

fn assert_transition_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Transition(_)));
}

fn assert_animation_name_value(value: &CssValue) {
    assert!(matches!(value, CssValue::AnimationName(_)));
}

fn assert_animation_iteration_count_value(value: &CssValue) {
    assert!(matches!(value, CssValue::AnimationIterationCount(_)));
}

fn assert_animation_direction_value(value: &CssValue) {
    assert!(matches!(value, CssValue::AnimationDirection(_)));
}

fn assert_animation_fill_mode_value(value: &CssValue) {
    assert!(matches!(value, CssValue::AnimationFillMode(_)));
}

fn assert_animation_play_state_value(value: &CssValue) {
    assert!(matches!(value, CssValue::AnimationPlayState(_)));
}

fn assert_animation_value(value: &CssValue) {
    assert!(matches!(value, CssValue::Animation(_)));
}

macro_rules! value_case {
    ($label:literal, $property_name:literal, $authored_value:literal, $property:expr, $assertion:path) => {
        AcceptedValueCase {
            label: $label,
            property_name: $property_name,
            authored_value: $authored_value,
            expected_property: $property,
            assert_value: $assertion,
        }
    };
}

fn property_specific_rejection_probe(property_name: &str) -> &'static str {
    match property_name {
        "all" => "block",
        "display" => "inline",
        "box-sizing" => "padding-box",
        "position" => "running",
        "direction" => "block",
        "overflow" | "overflow-x" | "overflow-y" => "auto",
        "flex-direction" => "wrap",
        "flex-wrap" => "column",
        "float" => "center",
        "clear" => "start",
        "align-content" | "justify-content" | "place-content" | "justify-tracks"
        | "align-tracks" => "auto",
        "align-items" | "align-self" | "justify-items" | "justify-self" | "place-items"
        | "place-self" => "space-between",
        "visibility" => "auto",
        "content-visibility" => "collapse",
        "width" | "height" | "min-width" | "min-height" | "max-width" | "max-height"
        | "flex-basis" | "inset" | "top" | "right" | "bottom" | "left" | "margin"
        | "margin-top" | "margin-right" | "margin-bottom" | "margin-left" => "solid",
        "gap" | "row-gap" | "column-gap" => "auto",
        "grid-flow-tolerance" => "solid",
        "grid-template-rows"
        | "grid-template-columns"
        | "grid-template"
        | "grid-auto-rows"
        | "grid-auto-columns" => "solid",
        "grid-template-areas" => "\"a a\" \"a .\"",
        "grid-auto-flow" => "left",
        "grid-row-start" | "grid-row-end" | "grid-column-start" | "grid-column-end"
        | "grid-row" | "grid-column" | "grid-area" => "0",
        "grid" => "auto-flow",
        "font-size" | "line-height" | "text-indent" | "vertical-align" => "auto",
        "writing-mode" => "lr",
        "text-align" => "auto",
        "text-align-last" => "match-parent",
        "font-family" => "sans-serif,",
        "font" => "bold sans-serif",
        "font-weight" => "1001",
        "font-style" => "bold",
        "font-stretch" => "wide",
        "font-variant" => "italic",
        "font-feature-settings" => "\"abc\" on",
        "letter-spacing" => "auto",
        "text-wrap" => "auto",
        "white-space" => "balance",
        "word-break" => "nowrap",
        "overflow-wrap" => "ellipsis",
        "text-overflow" => "wrap",
        "text-decoration" | "text-decoration-line" => "underline underline",
        "text-decoration-color" => "solid",
        "text-decoration-style" => "auto",
        "text-decoration-thickness" => "-1px",
        "text-transform" => "wrap",
        "z-index" => "1.5",
        "box-decoration-break" => "auto",
        "padding" | "padding-top" | "padding-right" | "padding-bottom" | "padding-left" => "auto",
        "border" | "border-top" | "border-right" | "border-bottom" | "border-left" => {
            "solid dotted"
        }
        "border-width"
        | "border-top-width"
        | "border-right-width"
        | "border-bottom-width"
        | "border-left-width"
        | "outline-width" => "10%",
        "color"
        | "background"
        | "background-color"
        | "border-color"
        | "border-top-color"
        | "border-right-color"
        | "border-bottom-color"
        | "border-left-color"
        | "outline-color" => "solid",
        "background-image" | "mask-image" => "url(\"\")",
        "background-position" | "mask-position" | "transform-origin" => "left right",
        "background-size" | "mask-size" => "solid",
        "background-repeat" | "mask-repeat" => "solid",
        "background-origin" | "background-clip" => "margin-box",
        "background-attachment" => "sticky",
        "border-style"
        | "border-top-style"
        | "border-right-style"
        | "border-bottom-style"
        | "border-left-style" => "auto",
        "outline-style" => "10px",
        "border-radius"
        | "border-top-left-radius"
        | "border-top-right-radius"
        | "border-bottom-right-radius"
        | "border-bottom-left-radius" => "-1px",
        "box-shadow" => "1px 2px -3px",
        "opacity" | "flex-grow" | "flex-shrink" | "aspect-ratio" | "scrollbar-width" => "solid",
        "flex" => "-1",
        "order" => "1.5",
        "cursor" => "10px",
        "pointer-events" => "grab",
        "user-select" => "grab",
        "outline" => "solid dotted",
        "transform" => "translate(red)",
        "translate" => "red",
        "rotate" => "45px",
        "scale" => "solid",
        "filter" | "backdrop-filter" => "opacity(red)",
        "clip-path" => "circle(red)",
        "mask" => "solid",
        "transition-property" | "animation-name" => "auto",
        "transition-duration" | "transition-delay" | "animation-duration" | "animation-delay" => {
            "10px"
        }
        "transition-timing-function" | "animation-timing-function" => "bounce",
        "transition" => "opacity 1s 2s 3s",
        "animation-iteration-count" => "-1",
        "animation-direction" => "running",
        "animation-fill-mode" => "running",
        "animation-play-state" => "alternate",
        "animation" => "fade 1s 2s 3s",
        other => panic!("missing rejection probe for supported property `{other}`"),
    }
}

#[test]
fn strict_declaration_case_helpers_accept_and_reject_cases() {
    assert_accepts_declarations(&accepted_declaration_cases()[..3]);
    assert_eq!(
        parse_single_declaration_value("display", "inherit"),
        CssValue::GlobalKeyword(CssGlobalKeyword::Inherit)
    );

    assert_rejects_declarations(&[
        RejectedDeclarationCase {
            label: "unsupported display keyword",
            property_name: "display",
            authored_value: "inline",
            expected_error: ExpectedErrorKind::UnsupportedValue {
                property: Some("display"),
                reason: "unsupported display keyword `inline`",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "unknown property name",
            property_name: "widht",
            authored_value: "10px",
            expected_error: ExpectedErrorKind::UnknownProperty { name: "widht" },
            property_name_should_be_recognized: false,
        },
    ]);

    let accepted = AcceptedDeclarationCase::global_inherit("width", CssProperty::Width);
    accepted.assert_accepts();
}

#[test]
fn strict_whole_sheet_rejection_helper_rejects_mixed_declarations() {
    assert_sheet_rejected(
        ".panel { width: 10px; display: inline; }",
        &ExpectedErrorKind::UnsupportedValue {
            property: Some("display"),
            reason: "unsupported display keyword `inline`",
        },
    );
    assert_sheet_rejected(
        ".panel { width: inherit 10px; height: 20px; }",
        &ExpectedErrorKind::InvalidSyntax,
    );
}

#[test]
fn strict_no_recovery_whole_sheet_rejects_every_invalid_surface() {
    assert_rejects_sheets(&[
        RejectedSheetCase {
            label: "valid declaration before invalid declaration fails the whole sheet",
            input: ".panel { width: 10px; display: inline; height: 20px; }",
            expected_error: ExpectedErrorKind::UnsupportedValue {
                property: Some("display"),
                reason: "unsupported display keyword `inline`",
            },
        },
        RejectedSheetCase {
            label: "invalid declaration before valid declaration fails the whole sheet",
            input: ".panel { display: inline; width: 10px; }",
            expected_error: ExpectedErrorKind::UnsupportedValue {
                property: Some("display"),
                reason: "unsupported display keyword `inline`",
            },
        },
        RejectedSheetCase {
            label: "unknown property fails the whole sheet",
            input: ".panel { widht: 10px; width: 20px; }",
            expected_error: ExpectedErrorKind::UnknownProperty { name: "widht" },
        },
        RejectedSheetCase {
            label: "unsupported at-rule fails the whole sheet",
            input: "@media screen { .panel { width: 10px; } }",
            expected_error: ExpectedErrorKind::UnsupportedAtRule { name: "media" },
        },
        RejectedSheetCase {
            label: "invalid selector fails the whole sheet",
            input: "??? { width: 10px; }",
            expected_error: ExpectedErrorKind::InvalidSelector,
        },
        RejectedSheetCase {
            label: "malformed declaration block fails the whole sheet",
            input: ".panel { width 10px; height: 20px; }",
            expected_error: ExpectedErrorKind::InvalidSyntax,
        },
        RejectedSheetCase {
            label: "trailing junk after a value fails the whole sheet",
            input: ".panel { width: 10px solid; }",
            expected_error: ExpectedErrorKind::InvalidSyntax,
        },
        RejectedSheetCase {
            label: "invalid comma-list item fails the whole sheet",
            input: ".panel { transition-duration: 150ms, solid; }",
            expected_error: ExpectedErrorKind::InvalidSyntaxOrUnsupportedValueForProperty {
                property: "transition-duration",
            },
        },
        RejectedSheetCase {
            label: "invalid shorthand component fails the whole sheet",
            input: ".panel { border: 1px solid dotted; }",
            expected_error: ExpectedErrorKind::InvalidSyntaxOrUnsupportedValueForProperty {
                property: "border",
            },
        },
    ]);
}

#[test]
fn rejection_property_specific_matrix_rejects_every_supported_property() {
    for case in accepted_declaration_cases() {
        let authored_value = property_specific_rejection_probe(case.property_name);
        RejectedDeclarationCase {
            label: case.property_name,
            property_name: case.property_name,
            authored_value,
            expected_error: ExpectedErrorKind::InvalidSyntaxOrUnsupportedValueForProperty {
                property: case.property_name,
            },
            property_name_should_be_recognized: true,
        }
        .assert_rejects();
    }
}

#[test]
fn leakage_wrong_keyword_and_unit_matrix_rejects_property_family_crossovers() {
    assert_rejects_declarations(&[
        RejectedDeclarationCase {
            label: "display rejects unsupported inline keyword",
            property_name: "display",
            authored_value: "inline",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "display",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "overflow rejects auto keyword",
            property_name: "overflow",
            authored_value: "auto",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "overflow",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "align-items rejects content distribution keyword",
            property_name: "align-items",
            authored_value: "space-between",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "align-items",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "padding rejects auto keyword",
            property_name: "padding",
            authored_value: "auto",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "padding",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "border-width rejects percentage",
            property_name: "border-width",
            authored_value: "10%",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "border-width",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "border-color rejects border style keyword",
            property_name: "border-color",
            authored_value: "solid",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "border-color",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "font-size rejects auto keyword",
            property_name: "font-size",
            authored_value: "auto",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "font-size",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "grid-auto-flow rejects position keyword",
            property_name: "grid-auto-flow",
            authored_value: "left",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "grid-auto-flow",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "cursor rejects length",
            property_name: "cursor",
            authored_value: "10px",
            expected_error: ExpectedErrorKind::InvalidSyntaxOrUnsupportedValueForProperty {
                property: "cursor",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "transition-duration rejects length unit",
            property_name: "transition-duration",
            authored_value: "10px",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "transition-duration",
            },
            property_name_should_be_recognized: true,
        },
    ]);
}

#[test]
fn rejection_malformed_functions_lists_and_shorthands_matrix() {
    for input in [
        ".panel { width: calc(10px + ); }",
        ".panel { width: calc(10px * 2); }",
        ".panel { transform: translate(red); }",
        ".panel { filter: opacity(red); }",
        ".panel { clip-path: polygon(0 0, ); }",
        ".panel { transition-timing-function: cubic-bezier(0.1, red, 0.3, 1); }",
        ".panel { font-family: sans-serif,; }",
        ".panel { background-image: none,; }",
        ".panel { transition-property: opacity,; }",
        ".panel { animation-name: fade,; }",
        ".panel { border: 1px 2px solid; }",
        ".panel { box-shadow: inset inset 1px 2px; }",
        ".panel { text-decoration: underline underline; }",
        ".panel { transition: opacity 1s 2s 3s; }",
        ".panel { animation: fade 1s 2s 3s; }",
    ] {
        let error = parse_sheet(input).expect_err(input);
        assert!(
            matches!(
                error.kind(),
                ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
            ),
            "{input} rejected with unexpected error kind: {:?}",
            error.kind(),
        );
    }
}

#[test]
fn rejection_negative_numbers_and_public_constructor_invariants_matrix() {
    assert_rejects_declarations(&[
        RejectedDeclarationCase {
            label: "flex-grow rejects negative numbers",
            property_name: "flex-grow",
            authored_value: "-1",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "flex-grow",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "flex-shrink rejects negative numbers",
            property_name: "flex-shrink",
            authored_value: "-1",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "flex-shrink",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "padding rejects negative lengths",
            property_name: "padding",
            authored_value: "-1px",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "padding",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "border-radius rejects negative lengths",
            property_name: "border-radius",
            authored_value: "-1px",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "border-radius",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "animation-iteration-count rejects negative numbers",
            property_name: "animation-iteration-count",
            authored_value: "-1",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "animation-iteration-count",
            },
            property_name_should_be_recognized: true,
        },
    ]);

    assert_eq!(CssFontFamilyList::try_new(Vec::new()), None);
    assert_eq!(CssGridTrackList::try_new(Vec::new()), None);
    assert_eq!(
        CssPosition::try_new(vec![
            CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Left),
            CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Right),
        ]),
        None
    );
    assert_eq!(CssTransitionList::try_new(Vec::new()), None);
    assert_eq!(
        CssAnimation::try_new(CssAnimationComponents::default()),
        None
    );
}

#[test]
fn numeric_properties_use_property_specific_authored_models() {
    assert_eq!(
        single_declaration(".panel { opacity: 0.5; }").value(),
        &CssValue::Opacity(CssOpacity::try_new(0.5).unwrap())
    );
    assert_eq!(
        single_declaration(".panel { flex-grow: 2; }").value(),
        &CssValue::FlexGrow(CssFlexFactor::try_new(2.0).unwrap())
    );
    assert_eq!(
        single_declaration(".panel { flex-shrink: 0; }").value(),
        &CssValue::FlexShrink(CssFlexFactor::try_new(0.0).unwrap())
    );
    assert_eq!(
        single_declaration(".panel { aspect-ratio: 1.5; }").value(),
        &CssValue::AspectRatio(CssAspectRatio::try_new(1.5).unwrap())
    );
    assert_eq!(
        single_declaration(".panel { scrollbar-width: thin; }").value(),
        &CssValue::ScrollbarWidth(CssScrollbarWidth::Thin)
    );
    assert_eq!(CssOpacity::try_new(0.5).unwrap().value(), 0.5);
    assert_eq!(CssFlexFactor::try_new(2.0).unwrap().value(), 2.0);
    assert_eq!(CssAspectRatio::try_new(1.5).unwrap().value(), 1.5);
}

#[test]
fn numeric_property_models_reject_invalid_authored_values() {
    assert_rejects_declarations(&[
        RejectedDeclarationCase {
            label: "opacity rejects negative values",
            property_name: "opacity",
            authored_value: "-0.1",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "opacity",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "opacity rejects values above one",
            property_name: "opacity",
            authored_value: "2",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "opacity",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "flex-grow rejects negative values",
            property_name: "flex-grow",
            authored_value: "-1",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "flex-grow",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "flex-shrink rejects negative values",
            property_name: "flex-shrink",
            authored_value: "-1",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "flex-shrink",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "aspect-ratio rejects zero",
            property_name: "aspect-ratio",
            authored_value: "0",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "aspect-ratio",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "aspect-ratio rejects negative values",
            property_name: "aspect-ratio",
            authored_value: "-1",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "aspect-ratio",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "scrollbar-width rejects numbers",
            property_name: "scrollbar-width",
            authored_value: "8",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "scrollbar-width",
            },
            property_name_should_be_recognized: true,
        },
    ]);

    assert_eq!(CssOpacity::try_new(-0.1), None);
    assert_eq!(CssOpacity::try_new(1.1), None);
    assert_eq!(CssOpacity::try_new(f32::NAN), None);
    assert_eq!(CssFlexFactor::try_new(-1.0), None);
    assert_eq!(CssFlexFactor::try_new(f32::INFINITY), None);
    assert_eq!(CssAspectRatio::try_new(0.0), None);
    assert_eq!(CssAspectRatio::try_new(f32::NEG_INFINITY), None);
}

#[test]
fn constructor_invariants_reject_invalid_public_numeric_values() {
    assert_eq!(CssFiniteNumber::try_new(1.25).unwrap().value(), 1.25);
    assert_eq!(CssFiniteNumber::try_new(f32::NAN), None);
    assert_eq!(CssFiniteNumber::try_new(f32::INFINITY), None);

    assert_eq!(CssNonNegativeNumber::try_new(1.25).unwrap().value(), 1.25);
    assert_eq!(CssNonNegativeNumber::try_new(-0.1), None);
    assert_eq!(CssNonNegativeNumber::try_new(f32::NEG_INFINITY), None);

    assert_eq!(
        CssLengthDimension::try_new(2.0, CssLengthUnit::Rem)
            .unwrap()
            .value(),
        2.0
    );
    assert_eq!(
        CssLengthDimension::try_new(f32::NAN, CssLengthUnit::Rem),
        None
    );

    assert_eq!(CssLength::try_px(f32::NAN), None);
    assert_eq!(CssLength::try_percent(f32::INFINITY), None);
    assert_eq!(
        CssLength::try_dimension(f32::NEG_INFINITY, CssLengthUnit::Rem),
        None
    );
    assert_eq!(CssLength::try_px(3.0).unwrap(), CssLength::px(3.0));
    assert_eq!(
        CssLength::try_dimension(4.0, CssLengthUnit::Px).unwrap(),
        CssLength::px(4.0)
    );

    assert_eq!(CssGridTrackBreadth::try_fraction(f32::NAN), None);
    assert_eq!(CssGridTrackBreadth::try_fraction(-0.1), None);
    assert_eq!(
        CssGridTrackBreadth::try_fraction(1.0).unwrap(),
        CssGridTrackBreadth::fraction(1.0)
    );

    assert_eq!(CssScaleValues::try_new(vec![1.0, f32::NAN]), None);
    assert_eq!(CssScaleValues::try_new(vec![f32::INFINITY]), None);

    assert_eq!(CssCalcLength::try_px(f32::NAN), None);
    assert_eq!(CssCalcLength::try_percent(f32::INFINITY), None);
    assert_eq!(
        CssCalcLength::try_dimension(f32::NEG_INFINITY, CssLengthUnit::Rem),
        None
    );
    assert_eq!(
        CssCalcLengthTerm::add(CssCalcLength::try_px(1.0).unwrap()),
        CssCalcLengthTerm::add(CssCalcLength::px(1.0))
    );

    assert_eq!(CssFlexFactor::try_new(f32::NAN), None);
    assert_eq!(CssFlexFactor::try_new(-1.0), None);
    assert_eq!(
        CssFlex::components(
            CssFlexFactor::try_new(1.0).unwrap(),
            Some(CssFlexFactor::try_new(0.0).unwrap()),
            Some(CssLength::px(2.0)),
        ),
        CssFlex::Components {
            grow: CssFlexFactor::try_new(1.0).unwrap(),
            shrink: Some(CssFlexFactor::try_new(0.0).unwrap()),
            basis: Some(CssLength::px(2.0)),
        }
    );
}

#[test]
fn checked_color_construction_rejects_invalid_channels() {
    let color = CssColor::try_rgba(0.25, 0.5, 0.75, 1.0).unwrap();
    assert_eq!(color.r(), 0.25);
    assert_eq!(color.g(), 0.5);
    assert_eq!(color.b(), 0.75);
    assert_eq!(color.a(), 1.0);

    assert_eq!(CssColor::try_rgba(f32::NAN, 0.0, 0.0, 1.0), None);
    assert_eq!(CssColor::try_rgba(0.0, f32::INFINITY, 0.0, 1.0), None);
    assert_eq!(CssColor::try_rgba(-0.1, 0.0, 0.0, 1.0), None);
    assert_eq!(CssColor::try_rgba(0.0, 0.0, 0.0, 1.1), None);
    assert_eq!(CssColor::BLACK.a(), 1.0);
    assert_eq!(CssColor::WHITE.r(), 1.0);
    assert_eq!(CssColor::TRANSPARENT.a(), 0.0);
}

#[test]
fn rejection_unsupported_but_syntactically_valid_css_keywords_stay_rejected() {
    assert_rejects_declarations(&[
        RejectedDeclarationCase {
            label: "display inline remains unsupported",
            property_name: "display",
            authored_value: "inline",
            expected_error: ExpectedErrorKind::UnsupportedValue {
                property: Some("display"),
                reason: "unsupported display keyword `inline`",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "overflow auto remains unsupported",
            property_name: "overflow",
            authored_value: "auto",
            expected_error: ExpectedErrorKind::UnsupportedValue {
                property: Some("overflow"),
                reason: "unsupported overflow keyword `auto`",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "text-align-last match-parent remains unsupported",
            property_name: "text-align-last",
            authored_value: "match-parent",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "text-align-last",
            },
            property_name_should_be_recognized: true,
        },
        RejectedDeclarationCase {
            label: "background-origin margin-box remains unsupported",
            property_name: "background-origin",
            authored_value: "margin-box",
            expected_error: ExpectedErrorKind::UnsupportedValueForProperty {
                property: "background-origin",
            },
            property_name_should_be_recognized: true,
        },
    ]);
}

#[test]
fn acceptance_css_wide_global_keyword_matrix_accepts_supported_globals() {
    let cases = [
        AcceptedDeclarationCase {
            label: "all inherit",
            property_name: "all",
            authored_value: "inherit",
            expected_property: CssProperty::All,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::Inherit),
        },
        AcceptedDeclarationCase {
            label: "all initial",
            property_name: "all",
            authored_value: "initial",
            expected_property: CssProperty::All,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::Initial),
        },
        AcceptedDeclarationCase {
            label: "all unset",
            property_name: "all",
            authored_value: "unset",
            expected_property: CssProperty::All,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::Unset),
        },
        AcceptedDeclarationCase {
            label: "all revert",
            property_name: "all",
            authored_value: "revert",
            expected_property: CssProperty::All,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::Revert),
        },
        AcceptedDeclarationCase {
            label: "all revert-layer",
            property_name: "all",
            authored_value: "revert-layer",
            expected_property: CssProperty::All,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::RevertLayer),
        },
        AcceptedDeclarationCase {
            label: "display global initial",
            property_name: "display",
            authored_value: "initial",
            expected_property: CssProperty::Display,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::Initial),
        },
        AcceptedDeclarationCase {
            label: "width global unset",
            property_name: "width",
            authored_value: "unset",
            expected_property: CssProperty::Width,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::Unset),
        },
        AcceptedDeclarationCase {
            label: "color global revert",
            property_name: "color",
            authored_value: "revert",
            expected_property: CssProperty::Color,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::Revert),
        },
        AcceptedDeclarationCase {
            label: "animation global revert-layer",
            property_name: "animation",
            authored_value: "revert-layer",
            expected_property: CssProperty::Animation,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::RevertLayer),
        },
    ];

    assert_accepts_declarations(&cases);

    assert_accepts_value_cases(&[
        value_case!(
            "background-color global inherit preserves authored property",
            "background-color",
            "inherit",
            CssProperty::BackgroundColor,
            assert_global_value
        ),
        value_case!(
            "mask global unset",
            "mask",
            "unset",
            CssProperty::Mask,
            assert_global_value
        ),
    ]);
}

#[test]
fn acceptance_box_layout_and_spacing_family_matrix_accepts_supported_values() {
    let cases = [
        value_case!(
            "display block",
            "display",
            "block",
            CssProperty::Display,
            assert_display_value
        ),
        value_case!(
            "display inline-grid-lanes",
            "display",
            "inline-grid-lanes",
            CssProperty::Display,
            assert_display_value
        ),
        value_case!(
            "box-sizing border-box",
            "box-sizing",
            "border-box",
            CssProperty::BoxSizing,
            assert_box_sizing_value
        ),
        value_case!(
            "position sticky",
            "position",
            "sticky",
            CssProperty::Position,
            assert_position_value
        ),
        value_case!(
            "direction rtl",
            "direction",
            "rtl",
            CssProperty::Direction,
            assert_direction_value
        ),
        value_case!(
            "overflow axes",
            "overflow",
            "hidden scroll",
            CssProperty::Overflow,
            assert_overflow_value
        ),
        value_case!(
            "overflow-x clip",
            "overflow-x",
            "clip",
            CssProperty::OverflowX,
            assert_overflow_value
        ),
        value_case!(
            "overflow-y visible",
            "overflow-y",
            "visible",
            CssProperty::OverflowY,
            assert_overflow_value
        ),
        value_case!(
            "flex-direction column-reverse",
            "flex-direction",
            "column-reverse",
            CssProperty::FlexDirection,
            assert_flex_direction_value
        ),
        value_case!(
            "flex-wrap wrap-reverse",
            "flex-wrap",
            "wrap-reverse",
            CssProperty::FlexWrap,
            assert_flex_wrap_value
        ),
        value_case!(
            "width calc",
            "width",
            "calc(100% - 12px)",
            CssProperty::Width,
            assert_length_value
        ),
        value_case!(
            "height auto",
            "height",
            "auto",
            CssProperty::Height,
            assert_length_value
        ),
        value_case!(
            "min-width zero",
            "min-width",
            "0",
            CssProperty::MinWidth,
            assert_length_value
        ),
        value_case!(
            "min-height min-content",
            "min-height",
            "min-content",
            CssProperty::MinHeight,
            assert_length_value
        ),
        value_case!(
            "max-width max-content",
            "max-width",
            "max-content",
            CssProperty::MaxWidth,
            assert_length_value
        ),
        value_case!(
            "max-height fit-content",
            "max-height",
            "fit-content",
            CssProperty::MaxHeight,
            assert_length_value
        ),
        value_case!(
            "flex-basis rem",
            "flex-basis",
            "10rem",
            CssProperty::FlexBasis,
            assert_length_value
        ),
        value_case!(
            "gap two lengths",
            "gap",
            "12px",
            CssProperty::Gap,
            assert_length_value
        ),
        value_case!(
            "row-gap normal",
            "row-gap",
            "normal",
            CssProperty::RowGap,
            assert_length_value
        ),
        value_case!(
            "column-gap percent",
            "column-gap",
            "5%",
            CssProperty::ColumnGap,
            assert_length_value
        ),
        value_case!(
            "inset shorthand",
            "inset",
            "auto 10px 5%",
            CssProperty::Inset,
            assert_edges_value
        ),
        value_case!(
            "top auto",
            "top",
            "auto",
            CssProperty::Top,
            assert_length_value
        ),
        value_case!(
            "right length",
            "right",
            "10px",
            CssProperty::Right,
            assert_length_value
        ),
        value_case!(
            "bottom percent",
            "bottom",
            "5%",
            CssProperty::Bottom,
            assert_length_value
        ),
        value_case!(
            "left calc",
            "left",
            "calc(3px + 4%)",
            CssProperty::Left,
            assert_length_value
        ),
        value_case!(
            "z-index integer",
            "z-index",
            "-2",
            CssProperty::ZIndex,
            assert_z_index_value
        ),
        value_case!(
            "box-decoration-break clone",
            "box-decoration-break",
            "clone",
            CssProperty::BoxDecorationBreak,
            assert_box_decoration_break_value
        ),
        value_case!(
            "margin shorthand",
            "margin",
            "auto 10px 5%",
            CssProperty::Margin,
            assert_edges_value
        ),
        value_case!(
            "margin-top auto",
            "margin-top",
            "auto",
            CssProperty::MarginTop,
            assert_length_value
        ),
        value_case!(
            "margin-right length",
            "margin-right",
            "10px",
            CssProperty::MarginRight,
            assert_length_value
        ),
        value_case!(
            "margin-bottom percent",
            "margin-bottom",
            "5%",
            CssProperty::MarginBottom,
            assert_length_value
        ),
        value_case!(
            "margin-left calc",
            "margin-left",
            "calc(3px + 4%)",
            CssProperty::MarginLeft,
            assert_length_value
        ),
        value_case!(
            "padding shorthand",
            "padding",
            "1px 2% calc(3px + 4%) 0",
            CssProperty::Padding,
            assert_edges_value
        ),
        value_case!(
            "padding-top length",
            "padding-top",
            "12px",
            CssProperty::PaddingTop,
            assert_length_value
        ),
        value_case!(
            "padding-right percent",
            "padding-right",
            "2%",
            CssProperty::PaddingRight,
            assert_length_value
        ),
        value_case!(
            "padding-bottom calc",
            "padding-bottom",
            "calc(3px + 4%)",
            CssProperty::PaddingBottom,
            assert_length_value
        ),
        value_case!(
            "padding-left zero",
            "padding-left",
            "0",
            CssProperty::PaddingLeft,
            assert_length_value
        ),
        value_case!(
            "border-width shorthand",
            "border-width",
            "1px 2px 3px 4px",
            CssProperty::BorderWidth,
            assert_edges_value
        ),
        value_case!(
            "border-top-width length",
            "border-top-width",
            "1px",
            CssProperty::BorderTopWidth,
            assert_length_value
        ),
        value_case!(
            "border-right-width length",
            "border-right-width",
            "2px",
            CssProperty::BorderRightWidth,
            assert_length_value
        ),
        value_case!(
            "border-bottom-width length",
            "border-bottom-width",
            "3px",
            CssProperty::BorderBottomWidth,
            assert_length_value
        ),
        value_case!(
            "border-left-width length",
            "border-left-width",
            "4px",
            CssProperty::BorderLeftWidth,
            assert_length_value
        ),
        value_case!(
            "border-radius shorthand",
            "border-radius",
            "1px 2px 3px / 4px 5px",
            CssProperty::BorderRadius,
            assert_border_radius_value
        ),
        value_case!(
            "border-top-left-radius pair",
            "border-top-left-radius",
            "4px 10%",
            CssProperty::BorderTopLeftRadius,
            assert_corner_radius_value
        ),
        value_case!(
            "border-top-right-radius length",
            "border-top-right-radius",
            "1px",
            CssProperty::BorderTopRightRadius,
            assert_corner_radius_value
        ),
        value_case!(
            "border-bottom-right-radius percent",
            "border-bottom-right-radius",
            "10%",
            CssProperty::BorderBottomRightRadius,
            assert_corner_radius_value
        ),
        value_case!(
            "border-bottom-left-radius calc",
            "border-bottom-left-radius",
            "calc(1px + 2%)",
            CssProperty::BorderBottomLeftRadius,
            assert_corner_radius_value
        ),
    ];

    assert_accepts_value_cases(&cases);
}

#[test]
fn acceptance_color_background_border_outline_and_shadow_matrix_accepts_supported_values() {
    let cases = [
        value_case!(
            "color named",
            "color",
            "black",
            CssProperty::Color,
            assert_color_value
        ),
        value_case!(
            "background color",
            "background",
            "#fff",
            CssProperty::Background,
            assert_color_value
        ),
        value_case!(
            "background-color authored property",
            "background-color",
            "transparent",
            CssProperty::BackgroundColor,
            assert_color_value
        ),
        value_case!(
            "border shorthand",
            "border",
            "solid 2px #fff",
            CssProperty::Border,
            assert_border_value
        ),
        value_case!(
            "border-top shorthand",
            "border-top",
            "black dotted",
            CssProperty::BorderTop,
            assert_border_value
        ),
        value_case!(
            "border-right width-only",
            "border-right",
            "1px",
            CssProperty::BorderRight,
            assert_border_value
        ),
        value_case!(
            "border-bottom color-only",
            "border-bottom",
            "#fff",
            CssProperty::BorderBottom,
            assert_border_value
        ),
        value_case!(
            "border-left style-color",
            "border-left",
            "dashed black",
            CssProperty::BorderLeft,
            assert_border_value
        ),
        value_case!(
            "border-color named",
            "border-color",
            "black",
            CssProperty::BorderColor,
            assert_color_value
        ),
        value_case!(
            "border-top-color named",
            "border-top-color",
            "black",
            CssProperty::BorderTopColor,
            assert_color_value
        ),
        value_case!(
            "border-right-color named",
            "border-right-color",
            "white",
            CssProperty::BorderRightColor,
            assert_color_value
        ),
        value_case!(
            "border-bottom-color transparent",
            "border-bottom-color",
            "transparent",
            CssProperty::BorderBottomColor,
            assert_color_value
        ),
        value_case!(
            "border-left-color hex",
            "border-left-color",
            "#fff",
            CssProperty::BorderLeftColor,
            assert_color_value
        ),
        value_case!(
            "border-style shorthand",
            "border-style",
            "none hidden dotted dashed",
            CssProperty::BorderStyle,
            assert_border_styles_value
        ),
        value_case!(
            "border-top-style solid",
            "border-top-style",
            "solid",
            CssProperty::BorderTopStyle,
            assert_border_style_value
        ),
        value_case!(
            "border-right-style double",
            "border-right-style",
            "double",
            CssProperty::BorderRightStyle,
            assert_border_style_value
        ),
        value_case!(
            "border-bottom-style ridge",
            "border-bottom-style",
            "ridge",
            CssProperty::BorderBottomStyle,
            assert_border_style_value
        ),
        value_case!(
            "border-left-style outset",
            "border-left-style",
            "outset",
            CssProperty::BorderLeftStyle,
            assert_border_style_value
        ),
        value_case!(
            "background-image list",
            "background-image",
            "url(\"hero.png\"), none",
            CssProperty::BackgroundImage,
            assert_background_image_value
        ),
        value_case!(
            "background-position offset",
            "background-position",
            "left 10px top 20%",
            CssProperty::BackgroundPosition,
            assert_background_position_value
        ),
        value_case!(
            "background-size list",
            "background-size",
            "cover, 10px auto",
            CssProperty::BackgroundSize,
            assert_background_size_value
        ),
        value_case!(
            "background-repeat list",
            "background-repeat",
            "repeat-x, no-repeat round",
            CssProperty::BackgroundRepeat,
            assert_background_repeat_value
        ),
        value_case!(
            "background-origin box",
            "background-origin",
            "content-box",
            CssProperty::BackgroundOrigin,
            assert_background_box_value
        ),
        value_case!(
            "background-clip box",
            "background-clip",
            "padding-box",
            CssProperty::BackgroundClip,
            assert_background_box_value
        ),
        value_case!(
            "background-attachment list",
            "background-attachment",
            "fixed, local",
            CssProperty::BackgroundAttachment,
            assert_background_attachment_value
        ),
        value_case!(
            "outline shorthand",
            "outline",
            "thick dotted white",
            CssProperty::Outline,
            assert_outline_value
        ),
        value_case!(
            "outline-color",
            "outline-color",
            "black",
            CssProperty::OutlineColor,
            assert_outline_color_value
        ),
        value_case!(
            "outline-style auto",
            "outline-style",
            "auto",
            CssProperty::OutlineStyle,
            assert_outline_style_value
        ),
        value_case!(
            "outline-width length",
            "outline-width",
            "2px",
            CssProperty::OutlineWidth,
            assert_outline_width_value
        ),
        value_case!(
            "box-shadow none",
            "box-shadow",
            "none",
            CssProperty::BoxShadow,
            assert_box_shadow_value
        ),
        value_case!(
            "box-shadow list",
            "box-shadow",
            "inset 1px 2px 3px 4px black, 0 1px #fff",
            CssProperty::BoxShadow,
            assert_box_shadow_value
        ),
        value_case!(
            "opacity number",
            "opacity",
            "0.5",
            CssProperty::Opacity,
            assert_opacity_value
        ),
    ];

    assert_accepts_value_cases(&cases);
}

#[test]
fn acceptance_position_alignment_flex_and_grid_matrix_accepts_supported_values() {
    let cases = [
        value_case!(
            "float left",
            "float",
            "left",
            CssProperty::Float,
            assert_float_value
        ),
        value_case!(
            "clear both",
            "clear",
            "both",
            CssProperty::Clear,
            assert_clear_value
        ),
        value_case!(
            "align-content distribution",
            "align-content",
            "space-between",
            CssProperty::AlignContent,
            assert_alignment_value
        ),
        value_case!(
            "justify-content safe center",
            "justify-content",
            "safe center",
            CssProperty::JustifyContent,
            assert_alignment_value
        ),
        value_case!(
            "align-items baseline",
            "align-items",
            "first baseline",
            CssProperty::AlignItems,
            assert_align_items_value
        ),
        value_case!(
            "align-self safe flex-end",
            "align-self",
            "safe flex-end",
            CssProperty::AlignSelf,
            assert_align_items_value
        ),
        value_case!(
            "justify-items stretch",
            "justify-items",
            "stretch",
            CssProperty::JustifyItems,
            assert_align_items_value
        ),
        value_case!(
            "justify-self center",
            "justify-self",
            "center",
            CssProperty::JustifySelf,
            assert_align_items_value
        ),
        value_case!(
            "place-content pair",
            "place-content",
            "center end",
            CssProperty::PlaceContent,
            assert_place_alignment_value
        ),
        value_case!(
            "place-items single",
            "place-items",
            "stretch",
            CssProperty::PlaceItems,
            assert_place_alignment_value
        ),
        value_case!(
            "place-self pair",
            "place-self",
            "end center",
            CssProperty::PlaceSelf,
            assert_place_alignment_value
        ),
        value_case!(
            "visibility collapse",
            "visibility",
            "collapse",
            CssProperty::Visibility,
            assert_visibility_value
        ),
        value_case!(
            "content-visibility auto",
            "content-visibility",
            "auto",
            CssProperty::ContentVisibility,
            assert_content_visibility_value
        ),
        value_case!(
            "flex-grow number",
            "flex-grow",
            "2",
            CssProperty::FlexGrow,
            assert_flex_grow_value
        ),
        value_case!(
            "flex-shrink number",
            "flex-shrink",
            "0",
            CssProperty::FlexShrink,
            assert_flex_shrink_value
        ),
        value_case!(
            "order negative integer",
            "order",
            "-2",
            CssProperty::Order,
            assert_order_value
        ),
        value_case!(
            "flex components",
            "flex",
            "2 0 10rem",
            CssProperty::Flex,
            assert_flex_value
        ),
        value_case!(
            "flex keyword none",
            "flex",
            "none",
            CssProperty::Flex,
            assert_flex_value
        ),
        value_case!(
            "justify-tracks distribution",
            "justify-tracks",
            "space-evenly",
            CssProperty::JustifyTracks,
            assert_alignment_value
        ),
        value_case!(
            "align-tracks center",
            "align-tracks",
            "center",
            CssProperty::AlignTracks,
            assert_alignment_value
        ),
        value_case!(
            "aspect-ratio number",
            "aspect-ratio",
            "1.5",
            CssProperty::AspectRatio,
            assert_aspect_ratio_value
        ),
        value_case!(
            "scrollbar-width keyword",
            "scrollbar-width",
            "thin",
            CssProperty::ScrollbarWidth,
            assert_scrollbar_width_value
        ),
        value_case!(
            "grid-flow-tolerance infinite",
            "grid-flow-tolerance",
            "infinite",
            CssProperty::GridFlowTolerance,
            assert_grid_flow_tolerance_value
        ),
        value_case!(
            "grid-template-rows tracks",
            "grid-template-rows",
            "[top] 100px 1fr",
            CssProperty::GridTemplateRows,
            assert_grid_track_list_value
        ),
        value_case!(
            "grid-template-columns repeat",
            "grid-template-columns",
            "repeat(2, minmax(10px, 1fr))",
            CssProperty::GridTemplateColumns,
            assert_grid_track_list_value
        ),
        value_case!(
            "grid-template-areas rows",
            "grid-template-areas",
            "\"header header\" \"nav main\"",
            CssProperty::GridTemplateAreas,
            assert_grid_template_areas_value
        ),
        value_case!(
            "grid-template shorthand",
            "grid-template",
            "100px 1fr / repeat(2, minmax(10px, 1fr))",
            CssProperty::GridTemplate,
            assert_grid_template_value
        ),
        value_case!(
            "grid-auto-rows minmax",
            "grid-auto-rows",
            "minmax(10px, auto)",
            CssProperty::GridAutoRows,
            assert_grid_track_list_value
        ),
        value_case!(
            "grid-auto-columns fit-content",
            "grid-auto-columns",
            "fit-content(20%)",
            CssProperty::GridAutoColumns,
            assert_grid_track_list_value
        ),
        value_case!(
            "grid-auto-flow dense",
            "grid-auto-flow",
            "column dense",
            CssProperty::GridAutoFlow,
            assert_grid_auto_flow_value
        ),
        value_case!(
            "grid-row-start span",
            "grid-row-start",
            "span 2 main",
            CssProperty::GridRowStart,
            assert_grid_line_value
        ),
        value_case!(
            "grid-row-end auto",
            "grid-row-end",
            "auto",
            CssProperty::GridRowEnd,
            assert_grid_line_value
        ),
        value_case!(
            "grid-column-start ident",
            "grid-column-start",
            "nav",
            CssProperty::GridColumnStart,
            assert_grid_line_value
        ),
        value_case!(
            "grid-column-end integer",
            "grid-column-end",
            "4",
            CssProperty::GridColumnEnd,
            assert_grid_line_value
        ),
        value_case!(
            "grid-row range",
            "grid-row",
            "1 / span 2",
            CssProperty::GridRow,
            assert_grid_line_range_value
        ),
        value_case!(
            "grid-column range",
            "grid-column",
            "nav / main",
            CssProperty::GridColumn,
            assert_grid_line_range_value
        ),
        value_case!(
            "grid-area shorthand",
            "grid-area",
            "header / 1 / span 2 / main",
            CssProperty::GridArea,
            assert_grid_area_value
        ),
        value_case!(
            "grid auto-flow shorthand",
            "grid",
            "auto-flow dense 12px / repeat(auto-fit, 1fr)",
            CssProperty::Grid,
            assert_grid_value
        ),
    ];

    assert_accepts_value_cases(&cases);
}

#[test]
fn acceptance_typography_and_text_family_matrix_accepts_supported_values() {
    let cases = [
        value_case!(
            "font-size length",
            "font-size",
            "16px",
            CssProperty::FontSize,
            assert_length_value
        ),
        value_case!(
            "line-height normal",
            "line-height",
            "normal",
            CssProperty::LineHeight,
            assert_length_value
        ),
        value_case!(
            "writing-mode vertical",
            "writing-mode",
            "vertical-rl",
            CssProperty::WritingMode,
            assert_writing_mode_value
        ),
        value_case!(
            "text-align start",
            "text-align",
            "start",
            CssProperty::TextAlign,
            assert_text_align_value
        ),
        value_case!(
            "text-align-last justify",
            "text-align-last",
            "justify",
            CssProperty::TextAlignLast,
            assert_text_align_last_value
        ),
        value_case!(
            "text-indent flags",
            "text-indent",
            "1rem hanging each-line",
            CssProperty::TextIndent,
            assert_text_indent_value
        ),
        value_case!(
            "vertical-align keyword",
            "vertical-align",
            "super",
            CssProperty::VerticalAlign,
            assert_vertical_align_value
        ),
        value_case!(
            "vertical-align length",
            "vertical-align",
            "4px",
            CssProperty::VerticalAlign,
            assert_vertical_align_value
        ),
        value_case!(
            "font-family list",
            "font-family",
            "\"Avenir Next\", Gill Sans, sans-serif",
            CssProperty::FontFamily,
            assert_font_family_value
        ),
        value_case!(
            "font shorthand",
            "font",
            "italic small-caps 700 condensed 16px/normal \"Avenir Next\", sans-serif",
            CssProperty::Font,
            assert_font_value
        ),
        value_case!(
            "font-weight number",
            "font-weight",
            "725",
            CssProperty::FontWeight,
            assert_font_weight_value
        ),
        value_case!(
            "font-style italic",
            "font-style",
            "italic",
            CssProperty::FontStyle,
            assert_font_style_value
        ),
        value_case!(
            "font-stretch semi-condensed",
            "font-stretch",
            "semi-condensed",
            CssProperty::FontStretch,
            assert_font_stretch_value
        ),
        value_case!(
            "font-variant small-caps",
            "font-variant",
            "small-caps",
            CssProperty::FontVariant,
            assert_font_variant_value
        ),
        value_case!(
            "font-feature-settings list",
            "font-feature-settings",
            "\"kern\" on, \"liga\" 0",
            CssProperty::FontFeatureSettings,
            assert_font_feature_settings_value
        ),
        value_case!(
            "letter-spacing normal",
            "letter-spacing",
            "normal",
            CssProperty::LetterSpacing,
            assert_letter_spacing_value
        ),
        value_case!(
            "letter-spacing length",
            "letter-spacing",
            "0.1em",
            CssProperty::LetterSpacing,
            assert_letter_spacing_value
        ),
        value_case!(
            "text-wrap balance",
            "text-wrap",
            "balance",
            CssProperty::TextWrap,
            assert_text_wrap_value
        ),
        value_case!(
            "white-space pre-wrap",
            "white-space",
            "pre-wrap",
            CssProperty::WhiteSpace,
            assert_white_space_value
        ),
        value_case!(
            "word-break keep-all",
            "word-break",
            "keep-all",
            CssProperty::WordBreak,
            assert_word_break_value
        ),
        value_case!(
            "overflow-wrap anywhere",
            "overflow-wrap",
            "anywhere",
            CssProperty::OverflowWrap,
            assert_overflow_wrap_value
        ),
        value_case!(
            "text-overflow ellipsis",
            "text-overflow",
            "ellipsis",
            CssProperty::TextOverflow,
            assert_text_overflow_value
        ),
        value_case!(
            "text-decoration shorthand",
            "text-decoration",
            "underline dotted white 3px",
            CssProperty::TextDecoration,
            assert_text_decoration_value
        ),
        value_case!(
            "text-decoration-line list",
            "text-decoration-line",
            "underline overline",
            CssProperty::TextDecorationLine,
            assert_text_decoration_line_value
        ),
        value_case!(
            "text-decoration-color",
            "text-decoration-color",
            "black",
            CssProperty::TextDecorationColor,
            assert_text_decoration_color_value
        ),
        value_case!(
            "text-decoration-style",
            "text-decoration-style",
            "wavy",
            CssProperty::TextDecorationStyle,
            assert_text_decoration_style_value
        ),
        value_case!(
            "text-decoration-thickness length",
            "text-decoration-thickness",
            "2px",
            CssProperty::TextDecorationThickness,
            assert_text_decoration_thickness_value
        ),
        value_case!(
            "text-transform uppercase",
            "text-transform",
            "uppercase",
            CssProperty::TextTransform,
            assert_text_transform_value
        ),
    ];

    assert_accepts_value_cases(&cases);
}

#[test]
fn acceptance_interaction_effect_mask_transition_animation_matrix_accepts_supported_values() {
    let cases = [
        value_case!(
            "cursor keyword",
            "cursor",
            "grab",
            CssProperty::Cursor,
            assert_cursor_value
        ),
        value_case!(
            "cursor url fallback",
            "cursor",
            "url(cursor.png), pointer",
            CssProperty::Cursor,
            assert_cursor_value
        ),
        value_case!(
            "pointer-events none",
            "pointer-events",
            "none",
            CssProperty::PointerEvents,
            assert_pointer_events_value
        ),
        value_case!(
            "user-select text",
            "user-select",
            "text",
            CssProperty::UserSelect,
            assert_user_select_value
        ),
        value_case!(
            "transform functions",
            "transform",
            "translate(10px, 20px) rotate(45deg) scale(1.5)",
            CssProperty::Transform,
            assert_transform_value
        ),
        value_case!(
            "transform none",
            "transform",
            "none",
            CssProperty::Transform,
            assert_transform_value
        ),
        value_case!(
            "transform-origin position",
            "transform-origin",
            "center top",
            CssProperty::TransformOrigin,
            assert_transform_origin_value
        ),
        value_case!(
            "translate values",
            "translate",
            "10px 20px",
            CssProperty::Translate,
            assert_translate_value
        ),
        value_case!(
            "rotate angle",
            "rotate",
            "45deg",
            CssProperty::Rotate,
            assert_rotate_value
        ),
        value_case!(
            "scale values",
            "scale",
            "1.5 2",
            CssProperty::Scale,
            assert_scale_value
        ),
        value_case!(
            "filter functions",
            "filter",
            "blur(4px) opacity(50%)",
            CssProperty::Filter,
            assert_filter_value
        ),
        value_case!(
            "backdrop-filter none",
            "backdrop-filter",
            "none",
            CssProperty::BackdropFilter,
            assert_filter_value
        ),
        value_case!(
            "clip-path shape",
            "clip-path",
            "circle(50% at center)",
            CssProperty::ClipPath,
            assert_clip_path_value
        ),
        value_case!(
            "mask shorthand",
            "mask",
            "url(mask.png) center / contain no-repeat",
            CssProperty::Mask,
            assert_mask_value
        ),
        value_case!(
            "mask-image list",
            "mask-image",
            "url(mask.png), none",
            CssProperty::MaskImage,
            assert_mask_image_value
        ),
        value_case!(
            "mask-size contain",
            "mask-size",
            "contain",
            CssProperty::MaskSize,
            assert_mask_size_value
        ),
        value_case!(
            "mask-position center",
            "mask-position",
            "center",
            CssProperty::MaskPosition,
            assert_mask_position_value
        ),
        value_case!(
            "mask-repeat repeat",
            "mask-repeat",
            "repeat",
            CssProperty::MaskRepeat,
            assert_mask_repeat_value
        ),
        value_case!(
            "transition-property list",
            "transition-property",
            "opacity, transform",
            CssProperty::TransitionProperty,
            assert_transition_property_value
        ),
        value_case!(
            "transition-duration list",
            "transition-duration",
            "150ms, 2s",
            CssProperty::TransitionDuration,
            assert_time_list_value
        ),
        value_case!(
            "transition-delay time",
            "transition-delay",
            "20ms",
            CssProperty::TransitionDelay,
            assert_time_list_value
        ),
        value_case!(
            "transition-timing-function list",
            "transition-timing-function",
            "ease-in, cubic-bezier(0.1, 0.2, 0.3, 1)",
            CssProperty::TransitionTimingFunction,
            assert_easing_list_value
        ),
        value_case!(
            "transition shorthand list",
            "transition",
            "opacity 150ms ease-in 20ms, transform 2s linear",
            CssProperty::Transition,
            assert_transition_value
        ),
        value_case!(
            "animation-name list",
            "animation-name",
            "fade, none",
            CssProperty::AnimationName,
            assert_animation_name_value
        ),
        value_case!(
            "animation-duration time",
            "animation-duration",
            "1s",
            CssProperty::AnimationDuration,
            assert_time_list_value
        ),
        value_case!(
            "animation-delay time",
            "animation-delay",
            "200ms",
            CssProperty::AnimationDelay,
            assert_time_list_value
        ),
        value_case!(
            "animation-timing-function easing",
            "animation-timing-function",
            "ease-out",
            CssProperty::AnimationTimingFunction,
            assert_easing_list_value
        ),
        value_case!(
            "animation-iteration-count list",
            "animation-iteration-count",
            "2, infinite",
            CssProperty::AnimationIterationCount,
            assert_animation_iteration_count_value
        ),
        value_case!(
            "animation-direction",
            "animation-direction",
            "alternate",
            CssProperty::AnimationDirection,
            assert_animation_direction_value
        ),
        value_case!(
            "animation-fill-mode",
            "animation-fill-mode",
            "both",
            CssProperty::AnimationFillMode,
            assert_animation_fill_mode_value
        ),
        value_case!(
            "animation-play-state list",
            "animation-play-state",
            "running, paused",
            CssProperty::AnimationPlayState,
            assert_animation_play_state_value
        ),
        value_case!(
            "animation shorthand list",
            "animation",
            "fade 1s ease-in 200ms 3 alternate both running, slide 2s linear",
            CssProperty::Animation,
            assert_animation_value
        ),
    ];

    assert_accepts_value_cases(&cases);
}

#[test]
fn parses_calc_width_as_css_calc_length() {
    let value = declaration_value(".panel { width: calc(20px + 10%); }", CssProperty::Width);

    match value {
        CssValue::Length(CssLength::Calc(calc)) => {
            assert!(calc.uses_percentage());
            assert_eq!(calc.to_css_string(), "calc(20px + 10%)");
        }
        other => panic!("expected calc length, got {other:?}"),
    }
}

#[test]
fn parses_nested_calc_width_with_subtraction_as_css_syntax() {
    let value = declaration_value(
        ".panel { width: calc(100% - calc(12px + 3%)); }",
        CssProperty::Width,
    );

    match value {
        CssValue::Length(CssLength::Calc(calc)) => {
            assert!(calc.uses_percentage());
            assert_eq!(calc.to_css_string(), "calc(100% - calc(12px + 3%))");
        }
        other => panic!("expected nested calc length, got {other:?}"),
    }
}

#[test]
fn exposes_nested_calc_terms_structurally() {
    let value = declaration_value(
        ".panel { width: calc(100% - calc(12px + 3%)); }",
        CssProperty::Width,
    );

    let calc = match value {
        CssValue::Length(CssLength::Calc(calc)) => calc,
        other => panic!("expected nested calc length, got {other:?}"),
    };

    let terms = match calc {
        CssCalcLength::Sum(terms) => terms,
        other => panic!("expected calc sum, got {other:?}"),
    };
    assert_eq!(terms.len(), 2);
    assert_eq!(terms[0].operator(), CssCalcOperator::Add);
    assert_eq!(terms[0].value(), &CssCalcLength::percent(100.0));
    assert_eq!(terms[1].operator(), CssCalcOperator::Subtract);

    let nested_terms = match terms[1].value() {
        CssCalcLength::Sum(terms) => terms,
        other => panic!("expected nested calc sum, got {other:?}"),
    };
    assert_eq!(nested_terms.len(), 2);
    assert_eq!(nested_terms[0].operator(), CssCalcOperator::Add);
    assert_eq!(nested_terms[0].value(), &CssCalcLength::px(12.0));
    assert_eq!(nested_terms[1].operator(), CssCalcOperator::Add);
    assert_eq!(nested_terms[1].value(), &CssCalcLength::percent(3.0));
}

#[test]
fn successful_declarations_expose_authored_source_location() {
    let input = ".panel {\n  height: 20px;\n  width: calc(100% - 4px);\n}\n";
    let height = declaration(input, CssProperty::Height);
    let width = declaration(input, CssProperty::Width);

    assert_eq!(height.location(), CssSourceLocation::new(1, 3));
    assert_eq!(width.location(), CssSourceLocation::new(2, 3));
    assert_eq!(width.line(), 2);
    assert_eq!(width.column(), 3);
}

#[test]
fn parses_supported_length_units_as_authored_dimensions() {
    let cases = [
        ("1em", 1.0, CssLengthUnit::Em),
        ("2rem", 2.0, CssLengthUnit::Rem),
        ("3vw", 3.0, CssLengthUnit::Vw),
        ("4svh", 4.0, CssLengthUnit::Svh),
        ("5lvw", 5.0, CssLengthUnit::Lvw),
        ("6dvb", 6.0, CssLengthUnit::Dvb),
        ("7cqi", 7.0, CssLengthUnit::Cqi),
        ("8cm", 8.0, CssLengthUnit::Cm),
        ("9pt", 9.0, CssLengthUnit::Pt),
    ];

    for (authored, expected_value, expected_unit) in cases {
        let value = declaration_value(
            &format!(".panel {{ width: {authored}; }}"),
            CssProperty::Width,
        );

        match value {
            CssValue::Length(CssLength::Dimension(length)) => {
                assert_eq!(length.value(), expected_value);
                assert_eq!(length.unit(), expected_unit);
                assert_eq!(length.to_css_string(), authored);
            }
            other => panic!("expected authored dimension for {authored}, got {other:?}"),
        }
    }
}

#[test]
fn parses_supported_calc_length_units_as_authored_dimensions() {
    let cases = [
        ("1em", 1.0, CssLengthUnit::Em),
        ("2rem", 2.0, CssLengthUnit::Rem),
        ("3vw", 3.0, CssLengthUnit::Vw),
        ("4svh", 4.0, CssLengthUnit::Svh),
        ("5lvw", 5.0, CssLengthUnit::Lvw),
        ("6dvb", 6.0, CssLengthUnit::Dvb),
        ("7cqi", 7.0, CssLengthUnit::Cqi),
        ("8cm", 8.0, CssLengthUnit::Cm),
        ("9pt", 9.0, CssLengthUnit::Pt),
    ];

    for (authored, expected_value, expected_unit) in cases {
        let value = declaration_value(
            &format!(".panel {{ width: calc({authored} + 2px); }}"),
            CssProperty::Width,
        );

        let CssValue::Length(CssLength::Calc(CssCalcLength::Sum(terms))) = value else {
            panic!("expected calc length for {authored}");
        };
        assert_eq!(terms.len(), 2);
        match terms[0].value() {
            CssCalcLength::Dimension(length) => {
                assert_eq!(length.value(), expected_value);
                assert_eq!(length.unit(), expected_unit);
                assert_eq!(length.to_css_string(), authored);
            }
            other => panic!("expected authored calc dimension for {authored}, got {other:?}"),
        }
        assert_eq!(terms[1].value(), &CssCalcLength::px(2.0));
    }
}

#[test]
fn unit_matrix_accepts_every_supported_length_unit_in_ordinary_length_contexts() {
    for unit in supported_length_units() {
        let authored = format!("1{}", unit.as_css_str());
        let declaration = parse_single_declaration("width", &authored);

        assert_eq!(declaration.property(), CssProperty::Width);
        assert_eq!(
            declaration.value(),
            &CssValue::Length(CssLength::dimension(1.0, unit)),
            "{authored} should preserve its supported length unit",
        );
    }
}

#[test]
fn unit_matrix_accepts_every_supported_length_unit_in_calc_contexts() {
    for unit in supported_length_units() {
        let authored = format!("calc(1{} + 2px)", unit.as_css_str());
        let declaration = parse_single_declaration("width", &authored);

        assert_eq!(declaration.property(), CssProperty::Width);
        let CssValue::Length(CssLength::Calc(CssCalcLength::Sum(terms))) = declaration.value()
        else {
            panic!("{authored} should parse as a calc length");
        };
        assert_eq!(terms.len(), 2);
        assert_eq!(
            terms[0].value(),
            &CssCalcLength::dimension(1.0, unit),
            "{authored} should preserve its supported calc length unit",
        );
        assert_eq!(terms[1].value(), &CssCalcLength::px(2.0));
    }
}

#[test]
fn unit_matrix_rejects_unknown_length_units_in_ordinary_and_calc_contexts() {
    assert_sheet_rejected(
        ".panel { width: 1quux; }",
        &ExpectedErrorKind::UnsupportedValue {
            property: Some("width"),
            reason: "unknown box size unit `quux`",
        },
    );
    assert_sheet_rejected(
        ".panel { width: calc(1quux + 2px); }",
        &ExpectedErrorKind::UnsupportedValue {
            property: Some("width"),
            reason: "unknown calc length unit `quux`",
        },
    );
}

fn supported_length_units() -> [CssLengthUnit; 49] {
    [
        CssLengthUnit::Px,
        CssLengthUnit::Em,
        CssLengthUnit::Rem,
        CssLengthUnit::Ex,
        CssLengthUnit::Rex,
        CssLengthUnit::Cap,
        CssLengthUnit::Rcap,
        CssLengthUnit::Ch,
        CssLengthUnit::Rch,
        CssLengthUnit::Ic,
        CssLengthUnit::Ric,
        CssLengthUnit::Lh,
        CssLengthUnit::Rlh,
        CssLengthUnit::Vw,
        CssLengthUnit::Vh,
        CssLengthUnit::Vi,
        CssLengthUnit::Vb,
        CssLengthUnit::Vmin,
        CssLengthUnit::Vmax,
        CssLengthUnit::Svw,
        CssLengthUnit::Svh,
        CssLengthUnit::Svi,
        CssLengthUnit::Svb,
        CssLengthUnit::Svmin,
        CssLengthUnit::Svmax,
        CssLengthUnit::Lvw,
        CssLengthUnit::Lvh,
        CssLengthUnit::Lvi,
        CssLengthUnit::Lvb,
        CssLengthUnit::Lvmin,
        CssLengthUnit::Lvmax,
        CssLengthUnit::Dvw,
        CssLengthUnit::Dvh,
        CssLengthUnit::Dvi,
        CssLengthUnit::Dvb,
        CssLengthUnit::Dvmin,
        CssLengthUnit::Dvmax,
        CssLengthUnit::Cqw,
        CssLengthUnit::Cqh,
        CssLengthUnit::Cqi,
        CssLengthUnit::Cqb,
        CssLengthUnit::Cqmin,
        CssLengthUnit::Cqmax,
        CssLengthUnit::Cm,
        CssLengthUnit::Mm,
        CssLengthUnit::Q,
        CssLengthUnit::In,
        CssLengthUnit::Pc,
        CssLengthUnit::Pt,
    ]
}

#[test]
fn typo_property_has_unknown_property_error_kind() {
    let error = parse_sheet(".panel { widht: 10px; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnknownProperty {
            name: "widht".to_owned(),
        }
    );
    assert!(error.message().contains("unknown CSS property `widht`"));
}

#[test]
fn parses_global_keywords_for_different_value_domains() {
    assert_eq!(
        declaration_value(".panel { width: inherit; }", CssProperty::Width),
        CssValue::GlobalKeyword(CssGlobalKeyword::Inherit)
    );
    assert_eq!(
        declaration_value(".panel { display: initial; }", CssProperty::Display),
        CssValue::GlobalKeyword(CssGlobalKeyword::Initial)
    );
    assert_eq!(
        declaration_value(".panel { color: unset; }", CssProperty::Color),
        CssValue::GlobalKeyword(CssGlobalKeyword::Unset)
    );
}

#[test]
fn parses_newer_global_keywords_as_authored_syntax() {
    assert_eq!(
        declaration_value(".panel { padding: revert; }", CssProperty::Padding),
        CssValue::GlobalKeyword(CssGlobalKeyword::Revert)
    );
    assert_eq!(
        declaration_value(".panel { margin: revert-layer; }", CssProperty::Margin),
        CssValue::GlobalKeyword(CssGlobalKeyword::RevertLayer)
    );
}

#[test]
fn parses_all_property_global_keywords_as_authored_syntax() {
    let cases = [
        ("inherit", CssGlobalKeyword::Inherit),
        ("initial", CssGlobalKeyword::Initial),
        ("unset", CssGlobalKeyword::Unset),
        ("revert", CssGlobalKeyword::Revert),
        ("revert-layer", CssGlobalKeyword::RevertLayer),
    ];

    for (authored, expected) in cases {
        assert_eq!(
            declaration_value(&format!(".panel {{ all: {authored}; }}"), CssProperty::All,),
            CssValue::GlobalKeyword(expected)
        );
    }
}

#[test]
fn rejects_non_global_all_values_with_typed_unsupported_value() {
    for input in [".panel { all: block; }", ".panel { all: 1px; }"] {
        let error = parse_sheet(input).expect_err(input);

        assert_eq!(
            error.kind(),
            &ErrorKind::UnsupportedValue {
                property: Some("all".to_owned()),
                reason: "`all` only accepts CSS-wide global keywords".to_owned(),
            }
        );
    }
}

#[test]
fn global_keyword_must_be_the_whole_value() {
    let error = parse_sheet(".panel { width: inherit 10px; }").unwrap_err();

    assert!(matches!(error.kind(), ErrorKind::InvalidSyntax { .. }));
}

#[test]
fn unsupported_display_keyword_is_typed_with_property_context() {
    let error = parse_sheet(".panel { display: inline; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("display".to_owned()),
            reason: "unsupported display keyword `inline`".to_owned(),
        }
    );
}

#[test]
fn unsupported_overflow_keyword_is_typed_with_property_context() {
    let error = parse_sheet(".panel { overflow: auto; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("overflow".to_owned()),
            reason: "unsupported overflow keyword `auto`".to_owned(),
        }
    );
}

#[test]
fn unsupported_position_keyword_is_typed_with_property_context() {
    let error = parse_sheet(".panel { position: running; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("position".to_owned()),
            reason: "unsupported position keyword `running`".to_owned(),
        }
    );
}

#[test]
fn unsupported_alignment_keyword_is_typed_with_property_context() {
    let error = parse_sheet(".panel { align-items: unsafe center; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("align-items".to_owned()),
            reason: "unsupported alignment keyword `unsafe center`".to_owned(),
        }
    );
}

#[test]
fn parses_position_float_clear_visibility_values() {
    assert_eq!(
        declaration_value(".panel { position: static; }", CssProperty::Position),
        CssValue::Position(CssLayoutPosition::Static)
    );
    assert_eq!(
        declaration_value(".panel { position: fixed; }", CssProperty::Position),
        CssValue::Position(CssLayoutPosition::Fixed)
    );
    assert_eq!(
        declaration_value(".panel { position: sticky; }", CssProperty::Position),
        CssValue::Position(CssLayoutPosition::Sticky)
    );
    assert_eq!(
        declaration_value(".panel { float: left; }", CssProperty::Float),
        CssValue::Float(CssFloat::Left)
    );
    assert_eq!(
        declaration_value(".panel { clear: both; }", CssProperty::Clear),
        CssValue::Clear(CssClear::Both)
    );
    assert_eq!(
        declaration_value(".panel { visibility: collapse; }", CssProperty::Visibility),
        CssValue::Visibility(CssVisibility::Collapse)
    );
    assert_eq!(
        declaration_value(
            ".panel { content-visibility: auto; }",
            CssProperty::ContentVisibility
        ),
        CssValue::ContentVisibility(CssContentVisibility::Auto)
    );
}

#[test]
fn parses_content_alignment_and_place_shorthands() {
    assert_eq!(
        declaration_value(
            ".panel { align-content: space-between; }",
            CssProperty::AlignContent
        ),
        CssValue::Alignment(CssAlignment::SpaceBetween)
    );
    assert_eq!(
        declaration_value(
            ".panel { justify-content: safe center; }",
            CssProperty::JustifyContent
        ),
        CssValue::Alignment(CssAlignment::SafeCenter)
    );
    assert_eq!(
        declaration_value(
            ".panel { align-items: first baseline; }",
            CssProperty::AlignItems
        ),
        CssValue::AlignItems(CssAlignItems::FirstBaseline)
    );
    assert_eq!(
        declaration_value(
            ".panel { place-content: center end; }",
            CssProperty::PlaceContent
        ),
        CssValue::PlaceAlignment(CssPlaceAlignment::content(
            CssAlignment::Center,
            CssAlignment::End
        ))
    );
    assert_eq!(
        declaration_value(".panel { place-items: stretch; }", CssProperty::PlaceItems),
        CssValue::PlaceAlignment(CssPlaceAlignment::items_all(CssAlignItems::Stretch))
    );
    assert_eq!(
        declaration_value(".panel { place-self: end center; }", CssProperty::PlaceSelf),
        CssValue::PlaceAlignment(CssPlaceAlignment::items(
            CssAlignItems::End,
            CssAlignItems::Center
        ))
    );
}

#[test]
fn preserves_explicit_safe_alignment_values() {
    assert_eq!(
        declaration_value(".panel { align-items: safe end; }", CssProperty::AlignItems),
        CssValue::AlignItems(CssAlignItems::SafeEnd)
    );
    assert_eq!(
        declaration_value(
            ".panel { align-self: safe flex-end; }",
            CssProperty::AlignSelf
        ),
        CssValue::AlignItems(CssAlignItems::SafeFlexEnd)
    );
    assert_eq!(
        declaration_value(
            ".panel { justify-content: safe center; }",
            CssProperty::JustifyContent
        ),
        CssValue::Alignment(CssAlignment::SafeCenter)
    );
}

#[test]
fn rejects_positioning_alignment_and_visibility_leakage_values() {
    let cases = [
        ".panel { float: center; }",
        ".panel { clear: start; }",
        ".panel { align-content: left; }",
        ".panel { justify-content: auto; }",
        ".panel { place-items: auto; }",
        ".panel { place-items: space-between; }",
        ".panel { visibility: auto; }",
        ".panel { content-visibility: collapse; }",
    ];

    for case in cases {
        assert!(parse_sheet(case).is_err(), "{case} should be rejected");
    }
}

#[test]
fn rejects_unmodeled_safe_prefixed_alignment_values() {
    let cases = [
        ".panel { align-items: safe start; }",
        ".panel { align-items: safe flex-start; }",
        ".panel { align-items: safe stretch; }",
        ".panel { align-content: safe start; }",
        ".panel { align-content: safe flex-start; }",
        ".panel { align-content: safe stretch; }",
        ".panel { place-content: safe start; }",
        ".panel { place-content: safe flex-start; }",
        ".panel { place-content: safe stretch; }",
    ];

    for case in cases {
        assert!(parse_sheet(case).is_err(), "{case} should be rejected");
    }
}

#[test]
fn unknown_dimension_units_are_reported_as_unknown_units() {
    let error = parse_sheet(".panel { width: 1quux; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("width".to_owned()),
            reason: "unknown box size unit `quux`".to_owned(),
        }
    );
}

#[test]
fn unknown_calc_dimension_units_are_reported_as_unknown_units() {
    let error = parse_sheet(".panel { width: calc(1quux + 2px); }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("width".to_owned()),
            reason: "unknown calc length unit `quux`".to_owned(),
        }
    );
}

#[test]
fn selector_parse_failure_has_typed_error_kind() {
    let error = parse_sheet("??? { width: 10px; }").unwrap_err();

    assert!(matches!(error.kind(), ErrorKind::InvalidSelector { .. }));
    assert!(error.message().contains("unexpected selector token"));
}

#[test]
fn selector_missing_class_name_has_typed_error_kind() {
    let error = parse_sheet(". { width: 10px; }").unwrap_err();

    assert!(matches!(error.kind(), ErrorKind::InvalidSelector { .. }));
}

#[test]
fn grid_flow_tolerance_calc_is_preserved_as_css_syntax() {
    let value = declaration_value(
        ".panel { grid-flow-tolerance: calc(8px + 2%); }",
        CssProperty::GridFlowTolerance,
    );

    match value {
        CssValue::GridFlowTolerance(CssGridFlowTolerance::Length(CssLength::Calc(calc))) => {
            assert!(calc.uses_percentage());
            assert_eq!(calc.to_css_string(), "calc(8px + 2%)");
        }
        other => panic!("expected calc grid-flow-tolerance, got {other:?}"),
    }
}

#[test]
fn rejects_unknown_calc_functions() {
    let error = parse_sheet(".panel { width: min(10px, 20px); }").unwrap_err();
    assert!(error.message().contains("unsupported length function"));
}

#[test]
fn parses_calc_in_edge_shorthands() {
    let sheet = parse_sheet(".panel { margin: calc(4px + 1%) 2px; }").unwrap();
    let edges = match declaration_value(
        ".panel { margin: calc(4px + 1%) 2px; }",
        CssProperty::Margin,
    ) {
        CssValue::Edges(edges) => edges,
        other => panic!("expected edges, got {other:?}"),
    };

    match &edges.top {
        CssLength::Calc(calc) => {
            assert!(calc.uses_percentage());
            assert_eq!(calc.to_css_string(), "calc(4px + 1%)");
        }
        other => panic!("expected calc top edge, got {other:?}"),
    }
    assert_eq!(edges.right, CssLength::px(2.0));
    match &edges.bottom {
        CssLength::Calc(calc) => {
            assert!(calc.uses_percentage());
            assert_eq!(calc.to_css_string(), "calc(4px + 1%)");
        }
        other => panic!("expected calc bottom edge, got {other:?}"),
    }
    assert_eq!(edges.left, CssLength::px(2.0));

    assert_eq!(sheet.rules()[0].declarations().len(), 1);
}

#[test]
fn parses_authored_normal_gap_without_canonicalizing_it() {
    let value = declaration_value(".panel { gap: normal; }", CssProperty::Gap);
    assert_eq!(value, CssValue::Length(CssLength::Normal));
}

#[test]
fn parses_authored_calc_gap_without_canonicalizing_it() {
    let value = declaration_value(".panel { gap: calc(8px + 2%); }", CssProperty::Gap);
    match value {
        CssValue::Length(CssLength::Calc(calc)) => {
            assert!(calc.uses_percentage());
            assert_eq!(calc.to_css_string(), "calc(8px + 2%)");
        }
        other => panic!("expected calc gap, got {other:?}"),
    }
}

#[test]
fn rejects_line_height_auto() {
    let error = parse_sheet(".panel { line-height: auto; }").unwrap_err();
    assert!(error.message().contains("unsupported line-height"));
}

#[test]
fn rejects_line_height_min_content() {
    let error = parse_sheet(".panel { line-height: min-content; }").unwrap_err();
    assert!(error.message().contains("unsupported line-height"));
}

#[test]
fn rejects_font_size_auto() {
    let error = parse_sheet(".panel { font-size: auto; }").unwrap_err();
    assert!(error.message().contains("unsupported font-size"));
}

#[test]
fn parses_typography_and_text_keyword_families() {
    assert_eq!(
        declaration_value(
            ".panel { writing-mode: vertical-rl; }",
            CssProperty::WritingMode,
        ),
        CssValue::WritingMode(CssWritingMode::VerticalRl)
    );
    assert_eq!(
        declaration_value(".panel { text-align: start; }", CssProperty::TextAlign),
        CssValue::TextAlign(CssTextAlign::Start)
    );
    assert_eq!(
        declaration_value(
            ".panel { text-align-last: justify; }",
            CssProperty::TextAlignLast,
        ),
        CssValue::TextAlignLast(CssTextAlignLast::Justify)
    );
    assert_eq!(
        declaration_value(".panel { text-wrap: balance; }", CssProperty::TextWrap),
        CssValue::TextWrap(CssTextWrap::Balance)
    );
    assert_eq!(
        declaration_value(".panel { white-space: pre-wrap; }", CssProperty::WhiteSpace),
        CssValue::WhiteSpace(CssWhiteSpace::PreWrap)
    );
    assert_eq!(
        declaration_value(".panel { word-break: keep-all; }", CssProperty::WordBreak),
        CssValue::WordBreak(CssWordBreak::KeepAll)
    );
    assert_eq!(
        declaration_value(
            ".panel { overflow-wrap: anywhere; }",
            CssProperty::OverflowWrap,
        ),
        CssValue::OverflowWrap(CssOverflowWrap::Anywhere)
    );
    assert_eq!(
        declaration_value(
            ".panel { text-overflow: ellipsis; }",
            CssProperty::TextOverflow
        ),
        CssValue::TextOverflow(CssTextOverflow::Ellipsis)
    );
    assert_eq!(
        declaration_value(
            ".panel { text-transform: uppercase; }",
            CssProperty::TextTransform
        ),
        CssValue::TextTransform(CssTextTransform::Uppercase)
    );
}

#[test]
fn parses_typography_and_text_length_families() {
    assert_eq!(
        declaration_value(".panel { text-indent: 2em; }", CssProperty::TextIndent),
        CssValue::TextIndent(CssTextIndent::new(
            CssLength::dimension(2.0, CssLengthUnit::Em),
            false,
            false,
        ))
    );
    assert_eq!(
        declaration_value(
            ".panel { vertical-align: 4px; }",
            CssProperty::VerticalAlign
        ),
        CssValue::VerticalAlign(CssVerticalAlign::Length(CssVerticalAlignLength::new(
            CssLength::px(4.0)
        )))
    );
    assert_eq!(
        declaration_value(
            ".panel { letter-spacing: normal; }",
            CssProperty::LetterSpacing
        ),
        CssValue::LetterSpacing(CssLetterSpacing::Normal)
    );
    assert_eq!(
        declaration_value(
            ".panel { letter-spacing: 0.1em; }",
            CssProperty::LetterSpacing
        ),
        CssValue::LetterSpacing(CssLetterSpacing::Length(CssLetterSpacingLength::new(
            CssLength::dimension(0.1, CssLengthUnit::Em)
        )))
    );
    assert_eq!(
        declaration_value(
            ".panel { text-decoration-thickness: from-font; }",
            CssProperty::TextDecorationThickness,
        ),
        CssValue::TextDecorationThickness(CssTextDecorationThickness::FromFont)
    );
    assert_eq!(
        declaration_value(
            ".panel { text-decoration-thickness: 2px; }",
            CssProperty::TextDecorationThickness,
        ),
        CssValue::TextDecorationThickness(CssTextDecorationThickness::Length(
            CssTextDecorationThicknessLength::new(CssLength::px(2.0))
        ))
    );
}

#[test]
fn parses_font_families_and_font_shorthand_as_authored_syntax() {
    let family = declaration_value(
        ".panel { font-family: \"Avenir Next\", Gill Sans, sans-serif; }",
        CssProperty::FontFamily,
    );
    let CssValue::FontFamily(family) = family else {
        panic!("expected font family list");
    };
    assert_eq!(
        family.families(),
        [
            CssFontFamilyName::try_quoted("Avenir Next").unwrap(),
            CssFontFamilyName::try_ident_sequence("Gill Sans").unwrap(),
            CssFontFamilyName::try_ident_sequence("sans-serif").unwrap(),
        ]
    );

    assert_eq!(
        declaration_value(".panel { font-weight: 725; }", CssProperty::FontWeight),
        CssValue::FontWeight(CssFontWeight::Number(CssFontWeightNumber::new(725)))
    );
    assert_eq!(
        declaration_value(".panel { font-style: italic; }", CssProperty::FontStyle),
        CssValue::FontStyle(CssFontStyle::Italic)
    );
    assert_eq!(
        declaration_value(
            ".panel { font-stretch: semi-condensed; }",
            CssProperty::FontStretch,
        ),
        CssValue::FontStretch(CssFontStretch::SemiCondensed)
    );
    assert_eq!(
        declaration_value(
            ".panel { font-variant: small-caps; }",
            CssProperty::FontVariant
        ),
        CssValue::FontVariant(CssFontVariant::SmallCaps)
    );
    assert_eq!(
        declaration_value(
            ".panel { font-feature-settings: \"kern\" on, \"liga\" 0; }",
            CssProperty::FontFeatureSettings,
        ),
        CssValue::FontFeatureSettings(CssFontFeatureSettings::Features(CssFontFeatureList::new(
            vec![
                CssFontFeature::new("kern", Some(CssFontFeatureValue::On)),
                CssFontFeature::new("liga", Some(CssFontFeatureValue::Integer(0))),
            ]
        )))
    );

    let shorthand = declaration_value(
        ".panel { font: italic small-caps 700 condensed 16px/normal \"Avenir Next\", sans-serif; }",
        CssProperty::Font,
    );
    let CssValue::Font(font) = shorthand else {
        panic!("expected font shorthand");
    };
    assert_eq!(font.style(), Some(CssFontStyle::Italic));
    assert_eq!(font.variant(), Some(CssFontVariant::SmallCaps));
    assert_eq!(
        font.weight(),
        Some(CssFontWeight::Number(CssFontWeightNumber::new(700)))
    );
    assert_eq!(font.stretch(), Some(CssFontStretch::Condensed));
    assert_eq!(font.size(), &CssLength::px(16.0));
    assert_eq!(font.line_height(), Some(&CssLength::Normal));
    assert_eq!(
        font.families().families(),
        [
            CssFontFamilyName::try_quoted("Avenir Next").unwrap(),
            CssFontFamilyName::try_ident_sequence("sans-serif").unwrap(),
        ]
    );
}

#[test]
fn parses_text_decoration_family() {
    assert_eq!(
        declaration_value(
            ".panel { text-decoration-line: underline overline; }",
            CssProperty::TextDecorationLine,
        ),
        CssValue::TextDecorationLine(CssTextDecorationLine::new(vec![
            CssTextDecorationLineComponent::Underline,
            CssTextDecorationLineComponent::Overline,
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { text-decoration-color: black; }",
            CssProperty::TextDecorationColor,
        ),
        CssValue::TextDecorationColor(CssColor::BLACK)
    );
    assert_eq!(
        declaration_value(
            ".panel { text-decoration-style: wavy; }",
            CssProperty::TextDecorationStyle,
        ),
        CssValue::TextDecorationStyle(CssTextDecorationStyle::Wavy)
    );

    let value = declaration_value(
        ".panel { text-decoration: underline dotted white 3px; }",
        CssProperty::TextDecoration,
    );
    assert_eq!(
        value,
        CssValue::TextDecoration(CssTextDecoration::new(
            Some(CssTextDecorationLine::new(vec![
                CssTextDecorationLineComponent::Underline
            ])),
            Some(CssColor::WHITE),
            Some(CssTextDecorationStyle::Dotted),
            Some(CssTextDecorationThickness::Length(
                CssTextDecorationThicknessLength::new(CssLength::px(3.0))
            )),
        ))
    );
}

#[test]
fn checked_typography_constructors_reject_invalid_states() {
    assert_eq!(CssFontFamilyList::try_new(Vec::new()), None);
    assert_eq!(CssFontWeightNumber::try_new(0), None);
    assert_eq!(CssFontWeightNumber::try_new(1001), None);
    assert_eq!(
        CssFontWeightNumber::try_new(500),
        Some(CssFontWeightNumber::new(500))
    );
    assert_eq!(CssFontFeatureList::try_new(Vec::new()), None);
    assert_eq!(CssTextDecorationLine::try_new(Vec::new()), None);
    assert!(
        CssFont::try_new(
            None,
            None,
            None,
            None,
            CssLength::px(12.0),
            None,
            CssFontFamilyList::new(vec![CssFontFamilyName::ident_sequence("sans-serif")]),
        )
        .is_some(),
    );
    assert_eq!(
        CssFont::try_new(
            None,
            None,
            None,
            None,
            CssLength::Auto,
            None,
            CssFontFamilyList::new(vec![CssFontFamilyName::ident_sequence("sans-serif")]),
        ),
        None
    );
    assert_eq!(CssFontFamilyName::try_quoted(""), None);
    assert_eq!(CssFontFamilyName::try_ident_sequence(""), None);
    assert_eq!(
        CssFontFamilyList::try_new(vec![CssFontFamilyName::ident_sequence("")]),
        None
    );
    assert_eq!(CssFontFeature::try_new("abc", None), None);
    assert_eq!(CssFontFeature::try_new("abcde", None), None);
    assert_eq!(
        CssFontFeature::try_new("kern", Some(CssFontFeatureValue::On)),
        Some(CssFontFeature::new("kern", Some(CssFontFeatureValue::On)))
    );
    assert_eq!(CssVerticalAlignLength::try_new(CssLength::Auto), None);
    assert_eq!(
        CssLetterSpacingLength::try_new(CssLength::percent(10.0)),
        None
    );
    assert_eq!(
        CssTextDecorationThicknessLength::try_new(CssLength::px(-1.0)),
        None
    );
    assert_eq!(
        CssTextDecorationLine::try_new(vec![
            CssTextDecorationLineComponent::Underline,
            CssTextDecorationLineComponent::Underline,
        ]),
        None
    );
}

#[test]
fn parses_every_task_5_supported_property_name() {
    let sheet = parse_sheet(
        ".panel {
            writing-mode: horizontal-tb;
            text-align: center;
            text-align-last: auto;
            text-indent: 1rem hanging each-line;
            vertical-align: super;
            font-family: \"Avenir Next\", sans-serif;
            font: italic 700 16px/normal \"Avenir Next\", sans-serif;
            font-weight: bold;
            font-style: oblique;
            font-stretch: expanded;
            font-variant: normal;
            font-feature-settings: normal;
            letter-spacing: 1px;
            text-wrap: wrap;
            white-space: nowrap;
            word-break: break-word;
            overflow-wrap: break-word;
            text-overflow: clip;
            text-decoration: underline solid black 1px;
            text-decoration-line: none;
            text-decoration-color: transparent;
            text-decoration-style: solid;
            text-decoration-thickness: auto;
            text-transform: capitalize;
        }",
    )
    .unwrap();
    let declarations = sheet.rules()[0].declarations();

    for property in [
        CssProperty::WritingMode,
        CssProperty::TextAlign,
        CssProperty::TextAlignLast,
        CssProperty::TextIndent,
        CssProperty::VerticalAlign,
        CssProperty::FontFamily,
        CssProperty::Font,
        CssProperty::FontWeight,
        CssProperty::FontStyle,
        CssProperty::FontStretch,
        CssProperty::FontVariant,
        CssProperty::FontFeatureSettings,
        CssProperty::LetterSpacing,
        CssProperty::TextWrap,
        CssProperty::WhiteSpace,
        CssProperty::WordBreak,
        CssProperty::OverflowWrap,
        CssProperty::TextOverflow,
        CssProperty::TextDecoration,
        CssProperty::TextDecorationLine,
        CssProperty::TextDecorationColor,
        CssProperty::TextDecorationStyle,
        CssProperty::TextDecorationThickness,
        CssProperty::TextTransform,
    ] {
        assert!(
            declarations
                .iter()
                .any(|declaration| declaration.property() == property),
            "missing parsed declaration for {property:?}",
        );
    }
}

#[test]
fn rejects_task_5_cross_family_leakage_values() {
    for input in [
        ".panel { font-size: auto; }",
        ".panel { font-weight: 1001; }",
        ".panel { font-style: bold; }",
        ".panel { font-family:; }",
        ".panel { letter-spacing: auto; }",
        ".panel { text-decoration-style: 2px; }",
        ".panel { text-transform: wrap; }",
        ".panel { font-feature-settings: \"abc\" on; }",
        ".panel { font-feature-settings: \"abcde\" on; }",
    ] {
        let error = parse_sheet(input).expect_err(input);
        assert!(matches!(
            error.kind(),
            ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
        ));
    }
}

#[test]
fn parses_background_properties_as_authored_syntax() {
    assert_eq!(
        declaration_value(
            ".panel { background-image: url(\"hero.png\"), none; }",
            CssProperty::BackgroundImage,
        ),
        CssValue::BackgroundImage(CssImageLayerList::new(vec![
            CssImageLayer::Url(CssUrl::new("hero.png")),
            CssImageLayer::None,
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { background-position: left 10px top 20%; }",
            CssProperty::BackgroundPosition,
        ),
        CssValue::BackgroundPosition(CssPositionList::new(vec![CssPosition::new(vec![
            CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Left),
            CssPositionComponent::Length(CssLength::px(10.0)),
            CssPositionComponent::Vertical(CssVerticalPositionKeyword::Top),
            CssPositionComponent::Length(CssLength::percent(20.0)),
        ])]))
    );
    assert_eq!(
        declaration_value(
            ".panel { background-size: cover, 10px auto; }",
            CssProperty::BackgroundSize,
        ),
        CssValue::BackgroundSize(CssBackgroundSizeList::new(vec![
            CssBackgroundSize::Cover,
            CssBackgroundSize::Explicit {
                width: CssBackgroundSizeComponent::Length(CssLength::px(10.0)),
                height: Some(CssBackgroundSizeComponent::Auto),
            },
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { background-repeat: repeat-x, no-repeat round; }",
            CssProperty::BackgroundRepeat,
        ),
        CssValue::BackgroundRepeat(CssBackgroundRepeatList::new(vec![
            CssBackgroundRepeat::RepeatX,
            CssBackgroundRepeat::Axes {
                x: CssBackgroundRepeatStyle::NoRepeat,
                y: CssBackgroundRepeatStyle::Round,
            },
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { background-origin: content-box; }",
            CssProperty::BackgroundOrigin,
        ),
        CssValue::BackgroundBox(CssBackgroundBox::ContentBox)
    );
    assert_eq!(
        declaration_value(
            ".panel { background-clip: padding-box; }",
            CssProperty::BackgroundClip,
        ),
        CssValue::BackgroundBox(CssBackgroundBox::PaddingBox)
    );
    assert_eq!(
        declaration_value(
            ".panel { background-attachment: fixed, local; }",
            CssProperty::BackgroundAttachment,
        ),
        CssValue::BackgroundAttachment(CssBackgroundAttachmentList::new(vec![
            CssBackgroundAttachment::Fixed,
            CssBackgroundAttachment::Local,
        ]))
    );
}

#[test]
fn parses_interaction_and_outline_properties_as_authored_syntax() {
    assert_eq!(
        declaration_value(".panel { cursor: grab; }", CssProperty::Cursor),
        CssValue::Cursor(CssCursor::Keyword(CssCursorKeyword::Grab))
    );
    assert_eq!(
        declaration_value(
            ".panel { pointer-events: none; }",
            CssProperty::PointerEvents
        ),
        CssValue::PointerEvents(CssPointerEvents::None)
    );
    assert_eq!(
        declaration_value(".panel { user-select: text; }", CssProperty::UserSelect),
        CssValue::UserSelect(CssUserSelect::Text)
    );
    assert_eq!(
        declaration_value(
            ".panel { outline: thick dotted white; }",
            CssProperty::Outline,
        ),
        CssValue::Outline(CssOutline::new(
            Some(CssOutlineWidth::Thick),
            Some(CssOutlineStyle::Border(CssBorderStyle::Dotted)),
            Some(CssColor::WHITE),
        ))
    );
    assert_eq!(
        declaration_value(".panel { outline-width: 2px; }", CssProperty::OutlineWidth),
        CssValue::OutlineWidth(CssOutlineWidth::Length(CssLength::px(2.0)))
    );
}

#[test]
fn parses_transform_effect_and_mask_properties_as_authored_syntax() {
    let transform = declaration_value(
        ".panel { transform: translate(10px, 20px) rotate(45deg) scale(1.5); }",
        CssProperty::Transform,
    );
    let CssValue::Transform(CssTransform::Functions(functions)) = transform else {
        panic!("expected transform functions");
    };
    assert_eq!(functions.functions().len(), 3);
    assert_eq!(
        functions.functions()[0].kind(),
        CssTransformFunctionKind::Translate
    );

    assert_eq!(
        declaration_value(
            ".panel { transform-origin: center top; }",
            CssProperty::TransformOrigin
        ),
        CssValue::TransformOrigin(CssPosition::new(vec![
            CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center),
            CssPositionComponent::Vertical(CssVerticalPositionKeyword::Top),
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { filter: blur(4px) opacity(50%); }",
            CssProperty::Filter
        ),
        CssValue::Filter(CssFilter::Functions(CssFilterFunctionList::new(vec![
            CssFilterFunction::Blur(CssFunctionArguments::new("4px")),
            CssFilterFunction::Opacity(CssFunctionArguments::new("50%")),
        ])))
    );
    assert_eq!(
        declaration_value(
            ".panel { backdrop-filter: none; }",
            CssProperty::BackdropFilter
        ),
        CssValue::Filter(CssFilter::None)
    );
    assert_eq!(
        declaration_value(
            ".panel { clip-path: circle(50% at center); }",
            CssProperty::ClipPath
        ),
        CssValue::ClipPath(CssClipPath::BasicShape(CssBasicShape::Circle(
            CssFunctionArguments::new("50% at center"),
        )))
    );
    assert_eq!(
        declaration_value(
            ".panel { mask-image: url(mask.png), none; }",
            CssProperty::MaskImage,
        ),
        CssValue::MaskImage(CssImageLayerList::new(vec![
            CssImageLayer::Url(CssUrl::new("mask.png")),
            CssImageLayer::None,
        ]))
    );
    let CssValue::Mask(mask_layers) = declaration_value(
        ".panel { mask: url(mask.png) center / contain no-repeat; }",
        CssProperty::Mask,
    ) else {
        panic!("expected mask shorthand");
    };
    assert_eq!(mask_layers.layers().len(), 1);
}

#[test]
fn parses_transition_properties_and_preserves_comma_lists() {
    assert_eq!(
        declaration_value(
            ".panel { transition-property: opacity, transform; }",
            CssProperty::TransitionProperty,
        ),
        CssValue::TransitionProperty(CssTransitionPropertyList::new(vec![
            CssTransitionProperty::Custom(CssCustomIdent::new("opacity")),
            CssTransitionProperty::Custom(CssCustomIdent::new("transform")),
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { transition-duration: 150ms, 2s; }",
            CssProperty::TransitionDuration,
        ),
        CssValue::TimeList(CssTimeList::new(vec![
            CssTime::try_milliseconds(150.0).unwrap(),
            CssTime::try_seconds(2.0).unwrap(),
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { transition-timing-function: ease-in, cubic-bezier(0.1, 0.2, 0.3, 1); }",
            CssProperty::TransitionTimingFunction,
        ),
        CssValue::EasingList(CssEasingList::new(vec![
            CssEasing::EaseIn,
            CssEasing::CubicBezier(CssFunctionArguments::new("0.1, 0.2, 0.3, 1")),
        ]))
    );

    let CssValue::Transition(transitions) = declaration_value(
        ".panel { transition: opacity 150ms ease-in 20ms, transform 2s linear; }",
        CssProperty::Transition,
    ) else {
        panic!("expected transition list");
    };
    assert_eq!(transitions.items().len(), 2);
}

#[test]
fn parses_animation_properties_and_preserves_comma_lists() {
    assert_eq!(
        declaration_value(
            ".panel { animation-name: fade, none; }",
            CssProperty::AnimationName,
        ),
        CssValue::AnimationName(CssAnimationNameList::new(vec![
            CssAnimationName::Custom(CssCustomIdent::new("fade")),
            CssAnimationName::None,
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { animation-iteration-count: 2, infinite; }",
            CssProperty::AnimationIterationCount,
        ),
        CssValue::AnimationIterationCount(CssAnimationIterationCountList::new(vec![
            CssAnimationIterationCount::Number(CssAnimationIterationNumber::new(2.0)),
            CssAnimationIterationCount::Infinite,
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { animation-play-state: running, paused; }",
            CssProperty::AnimationPlayState,
        ),
        CssValue::AnimationPlayState(CssAnimationPlayStateList::new(vec![
            CssAnimationPlayState::Running,
            CssAnimationPlayState::Paused,
        ]))
    );

    let CssValue::Animation(animations) = declaration_value(
        ".panel { animation: fade 1s ease-in 200ms 3 alternate both running, slide 2s linear; }",
        CssProperty::Animation,
    ) else {
        panic!("expected animation list");
    };
    assert_eq!(animations.items().len(), 2);
}

#[test]
fn checked_task_6_constructors_reject_invalid_invariants() {
    assert_eq!(CssImageLayerList::try_new(Vec::new()), None);
    assert_eq!(CssCursorUrlList::try_new(Vec::new()), None);
    assert!(CssCursor::try_urls(Vec::new(), CssCursorKeyword::Pointer).is_none());
    assert_eq!(
        CssPosition::try_new(vec![
            CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Left),
            CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Right),
        ]),
        None
    );
    assert_eq!(
        CssPosition::try_new(vec![
            CssPositionComponent::Vertical(CssVerticalPositionKeyword::Top),
            CssPositionComponent::Vertical(CssVerticalPositionKeyword::Bottom),
        ]),
        None
    );
    assert_eq!(CssTranslateValues::try_new(Vec::new()), None);
    assert_eq!(
        CssTranslateValues::try_new(vec![
            CssLength::px(1.0),
            CssLength::px(2.0),
            CssLength::px(3.0),
            CssLength::px(4.0),
        ]),
        None
    );
    assert_eq!(CssScaleValues::try_new(Vec::new()), None);
    assert_eq!(CssScaleValues::try_new(vec![1.0, 2.0, 3.0, 4.0]), None);
    assert_eq!(CssMaskList::try_new(Vec::new()), None);
    assert_eq!(CssTransitionList::try_new(Vec::new()), None);
    assert_eq!(CssTransition::try_new(None, None, None, None), None);
    assert_eq!(CssAnimationList::try_new(Vec::new()), None);
    assert_eq!(
        CssAnimation::try_new(CssAnimationComponents::default()),
        None
    );
    assert_eq!(CssTime::try_seconds(-1.0), None);
    assert_eq!(CssAnimationIterationCount::try_number(-1.0), None);
    assert_eq!(CssOutline::try_new(None, None, None), None);
}

#[test]
fn parses_every_task_6_supported_property_name() {
    let sheet = parse_sheet(
        ".panel {
            background-image: none;
            background-position: center;
            background-size: contain;
            background-repeat: no-repeat;
            background-origin: border-box;
            background-clip: content-box;
            background-attachment: scroll;
            cursor: pointer;
            pointer-events: auto;
            user-select: all;
            outline: 1px solid black;
            outline-color: white;
            outline-style: dashed;
            outline-width: thin;
            transform: none;
            transform-origin: left top;
            translate: 10px 20px;
            rotate: 45deg;
            scale: 1.5 2;
            filter: none;
            backdrop-filter: blur(4px);
            clip-path: none;
            mask: none;
            mask-image: none;
            mask-size: auto;
            mask-position: center;
            mask-repeat: repeat;
            transition-property: opacity;
            transition-duration: 1s;
            transition-delay: 20ms;
            transition-timing-function: ease;
            transition: opacity 1s ease;
            animation-name: fade;
            animation-duration: 1s;
            animation-delay: 20ms;
            animation-timing-function: ease-out;
            animation-iteration-count: infinite;
            animation-direction: alternate;
            animation-fill-mode: both;
            animation-play-state: paused;
            animation: fade 1s ease-in-out infinite alternate both running;
        }",
    )
    .unwrap();
    let declarations = sheet.rules()[0].declarations();

    for property in [
        CssProperty::BackgroundImage,
        CssProperty::BackgroundPosition,
        CssProperty::BackgroundSize,
        CssProperty::BackgroundRepeat,
        CssProperty::BackgroundOrigin,
        CssProperty::BackgroundClip,
        CssProperty::BackgroundAttachment,
        CssProperty::Cursor,
        CssProperty::PointerEvents,
        CssProperty::UserSelect,
        CssProperty::Outline,
        CssProperty::OutlineColor,
        CssProperty::OutlineStyle,
        CssProperty::OutlineWidth,
        CssProperty::Transform,
        CssProperty::TransformOrigin,
        CssProperty::Translate,
        CssProperty::Rotate,
        CssProperty::Scale,
        CssProperty::Filter,
        CssProperty::BackdropFilter,
        CssProperty::ClipPath,
        CssProperty::Mask,
        CssProperty::MaskImage,
        CssProperty::MaskSize,
        CssProperty::MaskPosition,
        CssProperty::MaskRepeat,
        CssProperty::TransitionProperty,
        CssProperty::TransitionDuration,
        CssProperty::TransitionDelay,
        CssProperty::TransitionTimingFunction,
        CssProperty::Transition,
        CssProperty::AnimationName,
        CssProperty::AnimationDuration,
        CssProperty::AnimationDelay,
        CssProperty::AnimationTimingFunction,
        CssProperty::AnimationIterationCount,
        CssProperty::AnimationDirection,
        CssProperty::AnimationFillMode,
        CssProperty::AnimationPlayState,
        CssProperty::Animation,
    ] {
        assert!(
            declarations
                .iter()
                .any(|declaration| declaration.property() == property),
            "missing parsed declaration for {property:?}",
        );
    }
}

#[test]
fn rejects_task_6_cross_family_leakage_values_and_empty_lists() {
    for input in [
        ".panel { background-size: solid; }",
        ".panel { cursor: 10px; }",
        ".panel { pointer-events: grab; }",
        ".panel { outline-width: 10%; }",
        ".panel { transform: red; }",
        ".panel { filter: 10px; }",
        ".panel { transition-duration: 10px; }",
        ".panel { animation-iteration-count: -1; }",
        ".panel { animation-play-state: visible; }",
        ".panel { transition: opacity 1s, ; }",
        ".panel { animation: fade 1s, ; }",
    ] {
        let error = parse_sheet(input).expect_err(input);
        assert!(matches!(
            error.kind(),
            ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
        ));
    }
}

#[test]
fn rejects_duplicate_axis_position_keywords_across_shared_position_properties() {
    for input in [
        ".panel { background-position: left right; }",
        ".panel { background-position: right left; }",
        ".panel { background-position: top bottom; }",
        ".panel { background-position: bottom top; }",
        ".panel { mask-position: left right; }",
        ".panel { mask-position: top bottom; }",
        ".panel { transform-origin: left right; }",
        ".panel { transform-origin: top bottom; }",
        ".panel { mask: url(mask.png) left right / contain no-repeat; }",
        ".panel { mask: url(mask.png) top bottom / contain no-repeat; }",
    ] {
        let error = parse_sheet(input).expect_err(input);
        assert!(matches!(
            error.kind(),
            ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
        ));
    }
}

#[test]
fn preserves_valid_position_keyword_forms_after_duplicate_axis_rejection() {
    for input in [
        ".panel { background-position: left top; }",
        ".panel { background-position: right bottom; }",
        ".panel { background-position: center center; }",
        ".panel { background-position: left 10px top 20%; }",
        ".panel { mask-position: center center; }",
        ".panel { transform-origin: right bottom; }",
        ".panel { mask: url(mask.png) left top / contain no-repeat; }",
    ] {
        parse_sheet(input).unwrap_or_else(|error| panic!("{input} should parse: {error}"));
    }
}

#[test]
fn rejects_task_6_invalid_function_arguments() {
    for input in [
        ".panel { transform: translate(red); }",
        ".panel { filter: opacity(red); }",
        ".panel { clip-path: circle(red); }",
        ".panel { transition-timing-function: cubic-bezier(red); }",
    ] {
        let error = parse_sheet(input).expect_err(input);
        assert!(matches!(
            error.kind(),
            ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
        ));
    }
}

#[test]
fn rejects_padding_auto() {
    let error = parse_sheet(".panel { padding: auto; }").unwrap_err();
    assert!(error.message().contains("unsupported padding"));
}

#[test]
fn rejects_border_width_percent() {
    let error = parse_sheet(".panel { border-width: 10%; }").unwrap_err();
    assert!(error.message().contains("unsupported border-width"));
}

#[test]
fn rejects_gap_auto() {
    let error = parse_sheet(".panel { gap: auto; }").unwrap_err();
    assert!(error.message().contains("unsupported gap"));
}

#[test]
fn accepts_margin_auto() {
    assert_eq!(
        declaration_value(".panel { margin: auto; }", CssProperty::Margin),
        CssValue::Edges(CssEdges::all(CssLength::Auto))
    );
}

#[test]
fn parses_spacing_inset_and_z_index_values() {
    assert_eq!(
        declaration_value(".panel { inset: auto 10px 5%; }", CssProperty::Inset),
        CssValue::Edges(CssEdges::new(
            CssLength::Auto,
            CssLength::px(10.0),
            CssLength::percent(5.0),
            CssLength::px(10.0),
        ))
    );
    assert_eq!(
        declaration_value(".panel { top: calc(10px + 5%); }", CssProperty::Top),
        CssValue::Length(CssLength::Calc(CssCalcLength::sum(
            CssCalcLengthTerm::add(CssCalcLength::px(10.0)),
            [CssCalcLengthTerm::add(CssCalcLength::percent(5.0))]
        )))
    );
    assert_eq!(
        declaration_value(".panel { z-index: -2; }", CssProperty::ZIndex),
        CssValue::ZIndex(CssZIndex::Integer(-2))
    );
    assert_eq!(
        declaration_value(
            ".panel { box-decoration-break: clone; }",
            CssProperty::BoxDecorationBreak
        ),
        CssValue::BoxDecorationBreak(CssBoxDecorationBreak::Clone)
    );
}

#[test]
fn parses_spacing_longhands_with_existing_component_rules() {
    assert_eq!(
        declaration_value(".panel { margin-left: auto; }", CssProperty::MarginLeft),
        CssValue::Length(CssLength::Auto)
    );
    assert_eq!(
        declaration_value(".panel { padding-top: 12px; }", CssProperty::PaddingTop),
        CssValue::Length(CssLength::px(12.0))
    );
    assert_eq!(
        declaration_value(
            ".panel { border-right-width: 2px; }",
            CssProperty::BorderRightWidth
        ),
        CssValue::Length(CssLength::px(2.0))
    );
}

#[test]
fn parses_border_style_and_border_shorthand_values() {
    assert_eq!(
        declaration_value(
            ".panel { border-style: solid dashed; }",
            CssProperty::BorderStyle
        ),
        CssValue::BorderStyles(CssBorderStyles::new(
            CssBorderStyle::Solid,
            CssBorderStyle::Dashed,
            CssBorderStyle::Solid,
            CssBorderStyle::Dashed,
        ))
    );
    assert_eq!(
        declaration_value(
            ".panel { border-left-style: groove; }",
            CssProperty::BorderLeftStyle
        ),
        CssValue::BorderStyle(CssBorderStyle::Groove)
    );
    assert_eq!(
        declaration_value(".panel { border: solid 2px #fff; }", CssProperty::Border),
        CssValue::Border(CssBorder::new(
            Some(CssLength::px(2.0)),
            Some(CssBorderStyle::Solid),
            Some(CssColor::WHITE),
        ))
    );
    assert_eq!(
        declaration_value(
            ".panel { border-top: black dotted; }",
            CssProperty::BorderTop
        ),
        CssValue::Border(CssBorder::new(
            None,
            Some(CssBorderStyle::Dotted),
            Some(CssColor::BLACK),
        ))
    );
}

#[test]
fn parses_border_radius_shorthand_and_longhands() {
    assert_eq!(
        declaration_value(
            ".panel { border-top-left-radius: 4px 10%; }",
            CssProperty::BorderTopLeftRadius,
        ),
        CssValue::CornerRadius(CssCornerRadius::new(
            CssLength::px(4.0),
            CssLength::percent(10.0),
        ))
    );
    assert_eq!(
        declaration_value(
            ".panel { border-radius: 1px 2px 3px / 4px 5px; }",
            CssProperty::BorderRadius,
        ),
        CssValue::BorderRadius(CssBorderRadii::new(
            CssCornerRadius::new(CssLength::px(1.0), CssLength::px(4.0)),
            CssCornerRadius::new(CssLength::px(2.0), CssLength::px(5.0)),
            CssCornerRadius::new(CssLength::px(3.0), CssLength::px(4.0)),
            CssCornerRadius::new(CssLength::px(2.0), CssLength::px(5.0)),
        ))
    );
}

#[test]
fn parses_box_shadow_none_and_shadow_lists() {
    assert_eq!(
        declaration_value(".panel { box-shadow: none; }", CssProperty::BoxShadow),
        CssValue::BoxShadow(CssBoxShadow::None)
    );

    let value = declaration_value(
        ".panel { box-shadow: inset 1px 2px 3px 4px black, 0 1px #fff; }",
        CssProperty::BoxShadow,
    );

    let CssValue::BoxShadow(CssBoxShadow::Shadows(shadows)) = value else {
        panic!("expected box-shadow list");
    };
    assert_eq!(shadows.shadows().len(), 2);
    assert_eq!(
        shadows.shadows()[0],
        CssShadow::new(
            true,
            CssLength::px(1.0),
            CssLength::px(2.0),
            Some(CssLength::px(3.0)),
            Some(CssLength::px(4.0)),
            Some(CssColor::BLACK),
        )
    );
    assert_eq!(
        shadows.shadows()[1],
        CssShadow::new(
            false,
            CssLength::Zero,
            CssLength::px(1.0),
            None,
            None,
            Some(CssColor::WHITE),
        )
    );
}

#[test]
fn checked_border_constructor_rejects_empty_shorthands() {
    assert_eq!(CssBorder::try_new(None, None, None), None);
    assert_eq!(
        CssBorder::try_new(None, Some(CssBorderStyle::Solid), None),
        Some(CssBorder::new(None, Some(CssBorderStyle::Solid), None))
    );
}

#[test]
fn checked_border_constructor_rejects_parser_invalid_widths() {
    for width in [
        CssLength::Auto,
        CssLength::percent(10.0),
        CssLength::px(-1.0),
        CssLength::MinContent,
        CssLength::Normal,
        CssLength::Calc(CssCalcLength::percent(10.0)),
        CssLength::Calc(CssCalcLength::px(-1.0)),
    ] {
        assert_eq!(
            CssBorder::try_new(Some(width), Some(CssBorderStyle::Solid), None),
            None
        );
    }

    assert_eq!(
        CssBorder::try_new(
            Some(CssLength::Calc(CssCalcLength::px(1.0))),
            Some(CssBorderStyle::Solid),
            None,
        ),
        Some(CssBorder::new(
            Some(CssLength::Calc(CssCalcLength::px(1.0))),
            Some(CssBorderStyle::Solid),
            None,
        ))
    );
}

#[test]
fn checked_corner_radius_constructor_rejects_parser_invalid_values() {
    for value in [
        CssLength::Auto,
        CssLength::MinContent,
        CssLength::MaxContent,
        CssLength::FitContent,
        CssLength::Normal,
        CssLength::px(-1.0),
        CssLength::percent(-1.0),
        CssLength::Calc(CssCalcLength::px(-1.0)),
        CssLength::Calc(CssCalcLength::percent(-1.0)),
    ] {
        assert_eq!(
            CssCornerRadius::try_new(value.clone(), CssLength::px(1.0)),
            None
        );
        assert_eq!(CssCornerRadius::try_new(CssLength::px(1.0), value), None);
    }

    assert_eq!(
        CssCornerRadius::try_new(CssLength::px(1.0), CssLength::percent(25.0)),
        Some(CssCornerRadius::new(
            CssLength::px(1.0),
            CssLength::percent(25.0)
        ))
    );
}

#[test]
fn checked_shadow_constructor_rejects_invalid_pairings_and_lengths() {
    assert_eq!(
        CssShadow::try_new(false, CssLength::Auto, CssLength::px(2.0), None, None, None,),
        None
    );
    assert_eq!(
        CssShadow::try_new(
            false,
            CssLength::px(1.0),
            CssLength::px(2.0),
            None,
            Some(CssLength::px(4.0)),
            None,
        ),
        None
    );
    assert_eq!(
        CssShadow::try_new(
            false,
            CssLength::px(1.0),
            CssLength::px(2.0),
            Some(CssLength::px(-3.0)),
            None,
            None,
        ),
        None
    );
    assert_eq!(
        CssShadow::try_new(
            false,
            CssLength::px(-1.0),
            CssLength::px(2.0),
            Some(CssLength::px(3.0)),
            Some(CssLength::px(-4.0)),
            None,
        ),
        Some(CssShadow::new(
            false,
            CssLength::px(-1.0),
            CssLength::px(2.0),
            Some(CssLength::px(3.0)),
            Some(CssLength::px(-4.0)),
            None,
        ))
    );
}

#[test]
fn parses_every_task_2_supported_property_name() {
    let sheet = parse_sheet(
        ".panel {
            inset: auto 1px 2%;
            top: auto;
            right: 1px;
            bottom: 2%;
            left: calc(3px + 4%);
            z-index: 7;
            box-decoration-break: slice;
            margin-top: auto;
            margin-right: 1px;
            margin-bottom: 2%;
            margin-left: calc(3px + 4%);
            padding-top: 1px;
            padding-right: 2%;
            padding-bottom: calc(3px + 4%);
            padding-left: 0;
            border: 1px solid black;
            border-top: solid;
            border-right: 1px;
            border-bottom: #fff;
            border-left: dashed black;
            border-top-width: 1px;
            border-right-width: 2px;
            border-bottom-width: 3px;
            border-left-width: 4px;
            border-top-color: black;
            border-right-color: white;
            border-bottom-color: transparent;
            border-left-color: #fff;
            border-style: none hidden dotted dashed;
            border-top-style: solid;
            border-right-style: double;
            border-bottom-style: ridge;
            border-left-style: outset;
            border-radius: 1px 2px / 3px 4px;
            border-top-left-radius: 1px;
            border-top-right-radius: 1px 2px;
            border-bottom-right-radius: 10%;
            border-bottom-left-radius: calc(1px + 2%);
            box-shadow: 1px 2px;
        }",
    )
    .unwrap();
    let declarations = sheet.rules()[0].declarations();

    for property in [
        CssProperty::Inset,
        CssProperty::Top,
        CssProperty::Right,
        CssProperty::Bottom,
        CssProperty::Left,
        CssProperty::ZIndex,
        CssProperty::BoxDecorationBreak,
        CssProperty::MarginTop,
        CssProperty::MarginRight,
        CssProperty::MarginBottom,
        CssProperty::MarginLeft,
        CssProperty::PaddingTop,
        CssProperty::PaddingRight,
        CssProperty::PaddingBottom,
        CssProperty::PaddingLeft,
        CssProperty::Border,
        CssProperty::BorderTop,
        CssProperty::BorderRight,
        CssProperty::BorderBottom,
        CssProperty::BorderLeft,
        CssProperty::BorderTopWidth,
        CssProperty::BorderRightWidth,
        CssProperty::BorderBottomWidth,
        CssProperty::BorderLeftWidth,
        CssProperty::BorderTopColor,
        CssProperty::BorderRightColor,
        CssProperty::BorderBottomColor,
        CssProperty::BorderLeftColor,
        CssProperty::BorderStyle,
        CssProperty::BorderTopStyle,
        CssProperty::BorderRightStyle,
        CssProperty::BorderBottomStyle,
        CssProperty::BorderLeftStyle,
        CssProperty::BorderRadius,
        CssProperty::BorderTopLeftRadius,
        CssProperty::BorderTopRightRadius,
        CssProperty::BorderBottomRightRadius,
        CssProperty::BorderBottomLeftRadius,
        CssProperty::BoxShadow,
    ] {
        assert!(
            declarations
                .iter()
                .any(|declaration| declaration.property() == property),
            "missing parsed declaration for {property:?}",
        );
    }
}

#[test]
fn rejects_negative_lengths_for_non_negative_task_2_properties() {
    for input in [
        ".panel { border-radius: -1px; }",
        ".panel { padding-top: -1px; }",
        ".panel { border-width: -1px; }",
        ".panel { box-shadow: 1px 2px -3px; }",
    ] {
        let error = parse_sheet(input).expect_err(input);
        assert!(matches!(error.kind(), ErrorKind::UnsupportedValue { .. }));
    }
}

#[test]
fn rejects_task_2_cross_family_leakage_values() {
    for input in [
        ".panel { padding-top: auto; }",
        ".panel { border-width: 10%; }",
        ".panel { border-style: 10px; }",
        ".panel { border-color: solid; }",
        ".panel { border-radius: auto; }",
        ".panel { box-shadow: auto; }",
        ".panel { z-index: 1.5; }",
    ] {
        let error = parse_sheet(input).expect_err(input);
        assert!(matches!(
            error.kind(),
            ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
        ));
    }
}

#[test]
fn parses_grid_track_lists_and_template_areas() {
    assert_eq!(
        declaration_value(
            ".panel { grid-template-columns: [main] repeat(2, minmax(10px, 1fr)) fit-content(20%); }",
            CssProperty::GridTemplateColumns,
        ),
        CssValue::GridTrackList(CssGridTrackList::new(vec![
            CssGridTrackComponent::LineNames(CssGridLineNames::new(vec![CssCustomIdent::new(
                "main"
            )])),
            CssGridTrackComponent::Repeat(CssGridRepeat::new(
                CssGridRepeatCount::integer(2),
                CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                    CssGridTrackSize::minmax(
                        CssGridTrackBreadth::length(CssLength::px(10.0)),
                        CssGridTrackBreadth::fraction(1.0),
                    )
                )]),
            )),
            CssGridTrackComponent::TrackSize(CssGridTrackSize::fit_content(CssLength::percent(
                20.0
            ))),
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { grid-template-areas: \"header header\" \"nav main\"; }",
            CssProperty::GridTemplateAreas,
        ),
        CssValue::GridTemplateAreas(CssGridTemplateAreas::rows(vec![
            CssGridTemplateAreaRow::new(vec![
                CssGridTemplateAreaCell::Named(CssCustomIdent::new("header")),
                CssGridTemplateAreaCell::Named(CssCustomIdent::new("header")),
            ]),
            CssGridTemplateAreaRow::new(vec![
                CssGridTemplateAreaCell::Named(CssCustomIdent::new("nav")),
                CssGridTemplateAreaCell::Named(CssCustomIdent::new("main")),
            ]),
        ]))
    );
    assert_eq!(
        declaration_value(
            ".panel { grid-template-areas: none; }",
            CssProperty::GridTemplateAreas,
        ),
        CssValue::GridTemplateAreas(CssGridTemplateAreas::None)
    );
}

#[test]
fn parses_grid_flow_lines_and_shorthands() {
    assert_eq!(
        declaration_value(
            ".panel { grid-auto-flow: column dense; }",
            CssProperty::GridAutoFlow,
        ),
        CssValue::GridAutoFlow(CssGridAutoFlow::new(CssGridAutoFlowAxis::Column, true))
    );
    assert_eq!(
        declaration_value(
            ".panel { grid-row-start: span 2 main; }",
            CssProperty::GridRowStart
        ),
        CssValue::GridLine(CssGridLine::span(
            Some(2),
            Some(CssCustomIdent::new("main"))
        ))
    );
    assert_eq!(
        declaration_value(
            ".panel { grid-column: nav / span 3; }",
            CssProperty::GridColumn
        ),
        CssValue::GridLineRange(CssGridLineRange::new(
            CssGridLine::CustomIdent(CssCustomIdent::new("nav")),
            Some(CssGridLine::span(Some(3), None)),
        ))
    );
    assert_eq!(
        declaration_value(
            ".panel { grid-area: header / 1 / span 2 / main; }",
            CssProperty::GridArea
        ),
        CssValue::GridArea(CssGridArea::new(
            CssGridLine::CustomIdent(CssCustomIdent::new("header")),
            Some(CssGridLine::integer(1)),
            Some(CssGridLine::span(Some(2), None)),
            Some(CssGridLine::CustomIdent(CssCustomIdent::new("main"))),
        ))
    );
}

#[test]
fn parses_grid_template_and_grid_shorthands() {
    assert_eq!(
        declaration_value(
            ".panel { grid-template: 100px 1fr / repeat(2, minmax(10px, 1fr)); }",
            CssProperty::GridTemplate,
        ),
        CssValue::GridTemplate(CssGridTemplate::RowsColumns {
            rows: CssGridTrackList::new(vec![
                CssGridTrackComponent::TrackSize(CssGridTrackSize::breadth(
                    CssGridTrackBreadth::length(CssLength::px(100.0))
                )),
                CssGridTrackComponent::TrackSize(CssGridTrackSize::breadth(
                    CssGridTrackBreadth::fraction(1.0)
                )),
            ]),
            columns: Some(CssGridTrackList::new(vec![CssGridTrackComponent::Repeat(
                CssGridRepeat::new(
                    CssGridRepeatCount::integer(2),
                    CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                        CssGridTrackSize::minmax(
                            CssGridTrackBreadth::length(CssLength::px(10.0)),
                            CssGridTrackBreadth::fraction(1.0),
                        )
                    )]),
                )
            )])),
        })
    );
    assert_eq!(
        declaration_value(
            ".panel { grid: auto-flow dense 12px / repeat(auto-fit, 1fr); }",
            CssProperty::Grid,
        ),
        CssValue::Grid(CssGrid::AutoFlow {
            flow: CssGridAutoFlow::new(CssGridAutoFlowAxis::Row, true),
            auto_tracks: Some(CssGridTrackList::new(vec![
                CssGridTrackComponent::TrackSize(CssGridTrackSize::breadth(
                    CssGridTrackBreadth::length(CssLength::px(12.0))
                ),)
            ])),
            explicit_tracks: CssGridTrackList::new(vec![CssGridTrackComponent::Repeat(
                CssGridRepeat::new(
                    CssGridRepeatCount::AutoFit,
                    CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                        CssGridTrackSize::breadth(CssGridTrackBreadth::fraction(1.0))
                    )]),
                )
            )]),
        })
    );
}

#[test]
fn parses_order_flex_and_track_alignment() {
    assert_eq!(
        declaration_value(".panel { order: -2; }", CssProperty::Order),
        CssValue::Order(CssOrder::Integer(-2))
    );
    assert_eq!(
        declaration_value(".panel { flex: 2 0 10rem; }", CssProperty::Flex),
        CssValue::Flex(CssFlex::Components {
            grow: CssFlexFactor::try_new(2.0).unwrap(),
            shrink: Some(CssFlexFactor::try_new(0.0).unwrap()),
            basis: Some(CssLength::dimension(10.0, CssLengthUnit::Rem)),
        })
    );
    assert_eq!(
        declaration_value(".panel { flex: none; }", CssProperty::Flex),
        CssValue::Flex(CssFlex::None)
    );
    assert_eq!(
        declaration_value(".panel { flex: auto; }", CssProperty::Flex),
        CssValue::Flex(CssFlex::Auto)
    );
    assert_eq!(
        declaration_value(
            ".panel { justify-tracks: space-evenly; }",
            CssProperty::JustifyTracks,
        ),
        CssValue::Alignment(CssAlignment::SpaceEvenly)
    );
    assert_eq!(
        declaration_value(".panel { align-tracks: center; }", CssProperty::AlignTracks),
        CssValue::Alignment(CssAlignment::Center)
    );
}

#[test]
fn parses_every_task_4_supported_property_name() {
    let sheet = parse_sheet(
        ".panel {
            grid-template-rows: [top] 100px 1fr;
            grid-template-columns: repeat(2, minmax(10px, 1fr));
            grid-template-areas: \"header header\" \"nav main\";
            grid-template: 100px / 1fr 2fr;
            grid-auto-rows: minmax(10px, auto);
            grid-auto-columns: fit-content(20%);
            grid-auto-flow: row dense;
            grid-row-start: 1;
            grid-row-end: span 2;
            grid-column-start: nav;
            grid-column-end: auto;
            grid-row: 1 / span 2;
            grid-column: nav / main;
            grid-area: header / nav / main / 4;
            grid: auto-flow 12px / repeat(auto-fill, 1fr);
            order: 2;
            flex: 1 1 auto;
            justify-tracks: space-between;
            align-tracks: stretch;
        }",
    )
    .unwrap();
    let declarations = sheet.rules()[0].declarations();

    for property in [
        CssProperty::GridTemplateRows,
        CssProperty::GridTemplateColumns,
        CssProperty::GridTemplateAreas,
        CssProperty::GridTemplate,
        CssProperty::GridAutoRows,
        CssProperty::GridAutoColumns,
        CssProperty::GridAutoFlow,
        CssProperty::GridRowStart,
        CssProperty::GridRowEnd,
        CssProperty::GridColumnStart,
        CssProperty::GridColumnEnd,
        CssProperty::GridRow,
        CssProperty::GridColumn,
        CssProperty::GridArea,
        CssProperty::Grid,
        CssProperty::Order,
        CssProperty::Flex,
        CssProperty::JustifyTracks,
        CssProperty::AlignTracks,
    ] {
        assert!(
            declarations
                .iter()
                .any(|declaration| declaration.property() == property),
            "missing parsed declaration for {property:?}",
        );
    }
}

#[test]
fn rejects_task_4_cross_family_leakage_values() {
    for input in [
        ".panel { order: 1.2; }",
        ".panel { grid-auto-flow: left; }",
        ".panel { grid-template-areas: \"a a\" \"a .\"; }",
        ".panel { grid-row: 1 / / 2; }",
        ".panel { flex: solid; }",
        ".panel { justify-tracks: auto; }",
    ] {
        let error = parse_sheet(input).expect_err(input);
        assert!(matches!(
            error.kind(),
            ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
        ));
    }
}

#[test]
fn checked_grid_constructors_reject_parser_invalid_states() {
    assert_eq!(CssCustomIdent::try_new(""), None);
    assert_eq!(CssCustomIdent::try_new("auto"), None);
    assert_eq!(
        CssCustomIdent::try_new("main"),
        Some(CssCustomIdent::new("main"))
    );
    assert_eq!(CssGridLineNames::try_new(Vec::new()), None);
    assert_eq!(CssGridTrackList::try_new(Vec::new()), None);
    assert_eq!(CssGridRepeatCount::try_integer(0), None);
    assert_eq!(
        CssGridRepeat::try_new(
            CssGridRepeatCount::integer(1),
            CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                CssGridTrackSize::breadth(CssGridTrackBreadth::fraction(1.0))
            )])
        ),
        Some(CssGridRepeat::new(
            CssGridRepeatCount::integer(1),
            CssGridTrackList::new(vec![CssGridTrackComponent::TrackSize(
                CssGridTrackSize::breadth(CssGridTrackBreadth::fraction(1.0))
            )])
        ))
    );
    assert_eq!(
        CssGridRepeat::try_new(
            CssGridRepeatCount::integer(1),
            CssGridTrackList::new(vec![])
        ),
        None
    );
    assert_eq!(CssGridTemplateAreaRow::try_new(Vec::new()), None);
    assert_eq!(CssGridTemplateAreas::try_rows(Vec::new()), None);
    assert_eq!(
        CssGridTemplateAreas::try_rows(vec![
            CssGridTemplateAreaRow::new(vec![
                CssGridTemplateAreaCell::Named(CssCustomIdent::new("a")),
                CssGridTemplateAreaCell::Named(CssCustomIdent::new("a")),
            ]),
            CssGridTemplateAreaRow::new(vec![
                CssGridTemplateAreaCell::Named(CssCustomIdent::new("a")),
                CssGridTemplateAreaCell::Empty,
            ]),
        ]),
        None
    );
    assert_eq!(CssGridLine::try_integer(0), None);
    assert_eq!(CssGridLineSpan::try_new(None, None), None);
    assert_eq!(CssGridLineSpan::try_new(Some(0), None), None);
}

#[test]
fn rejects_grid_auto_flow_shorthand_without_explicit_tracks() {
    for input in [
        ".panel { grid: auto-flow; }",
        ".panel { grid: auto-flow dense; }",
        ".panel { grid: auto-flow 12px; }",
    ] {
        let error = parse_sheet(input).expect_err(input);
        assert!(matches!(
            error.kind(),
            ErrorKind::UnsupportedValue { .. } | ErrorKind::InvalidSyntax { .. }
        ));
    }
}

#[test]
fn invalid_parser_custom_ident_errors_keep_source_location() {
    let error = parse_sheet(".panel {\n  grid-template-columns: [auto] 1fr;\n}").unwrap_err();

    assert!(matches!(error.kind(), ErrorKind::UnsupportedValue { .. }));
    assert_ne!(error.line(), 0);
    assert_ne!(error.column(), 0);
    assert_eq!(error.line(), 1);
}

#[test]
fn rejects_inconsistent_grid_template_area_row_widths() {
    let error = parse_sheet(".panel { grid-template-areas: \"a a\" \"b\"; }").unwrap_err();

    assert!(matches!(error.kind(), ErrorKind::UnsupportedValue { .. }));
    assert!(error.message().contains("inconsistent widths"));
}

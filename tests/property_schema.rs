use std::collections::HashSet;

mod common;

use common::CssParseReportTestExt;
use surgeist_css::{
    CssErrorCode, CssImportance, CssKnownProperty, CssKnownPropertyValueRef, CssRule, ErrorKind,
    parse_sheet, parse_style_attribute,
};

const FROZEN_PROPERTIES: &[&str] = &[
    "all",
    "display",
    "box-sizing",
    "position",
    "direction",
    "overflow",
    "overflow-x",
    "overflow-y",
    "float",
    "clear",
    "visibility",
    "content-visibility",
    "flex-direction",
    "flex-wrap",
    "align-content",
    "justify-content",
    "align-items",
    "align-self",
    "justify-items",
    "justify-self",
    "place-content",
    "place-items",
    "place-self",
    "gap",
    "row-gap",
    "column-gap",
    "flex-basis",
    "flex-grow",
    "flex-shrink",
    "order",
    "flex",
    "justify-tracks",
    "align-tracks",
    "content",
    "list-style-type",
    "list-style-position",
    "list-style-image",
    "list-style",
    "counter-reset",
    "counter-increment",
    "counter-set",
    "width",
    "height",
    "min-width",
    "min-height",
    "max-width",
    "max-height",
    "grid-flow-tolerance",
    "grid-template-rows",
    "grid-template-columns",
    "grid-template-areas",
    "grid-template",
    "grid-auto-rows",
    "grid-auto-columns",
    "grid-auto-flow",
    "grid-row-start",
    "grid-row-end",
    "grid-column-start",
    "grid-column-end",
    "grid-row",
    "grid-column",
    "grid-area",
    "grid",
    "aspect-ratio",
    "font-size",
    "line-height",
    "writing-mode",
    "text-align",
    "text-align-last",
    "text-indent",
    "vertical-align",
    "font-family",
    "font",
    "font-weight",
    "font-style",
    "font-stretch",
    "font-variant",
    "font-feature-settings",
    "letter-spacing",
    "text-wrap",
    "white-space",
    "word-break",
    "overflow-wrap",
    "text-overflow",
    "text-decoration",
    "text-decoration-line",
    "text-decoration-color",
    "text-decoration-style",
    "text-decoration-thickness",
    "text-transform",
    "inset",
    "top",
    "right",
    "bottom",
    "left",
    "z-index",
    "box-decoration-break",
    "margin",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "padding",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "border",
    "border-top",
    "border-right",
    "border-bottom",
    "border-left",
    "border-width",
    "border-top-width",
    "border-right-width",
    "border-bottom-width",
    "border-left-width",
    "color",
    "background",
    "background-color",
    "border-color",
    "border-top-color",
    "border-right-color",
    "border-bottom-color",
    "border-left-color",
    "background-image",
    "background-position",
    "background-size",
    "background-repeat",
    "background-origin",
    "background-clip",
    "background-attachment",
    "border-style",
    "border-top-style",
    "border-right-style",
    "border-bottom-style",
    "border-left-style",
    "border-radius",
    "border-top-left-radius",
    "border-top-right-radius",
    "border-bottom-right-radius",
    "border-bottom-left-radius",
    "box-shadow",
    "opacity",
    "scrollbar-width",
    "cursor",
    "pointer-events",
    "user-select",
    "outline",
    "outline-color",
    "outline-style",
    "outline-width",
    "transform",
    "transform-origin",
    "translate",
    "rotate",
    "scale",
    "filter",
    "backdrop-filter",
    "clip-path",
    "mask",
    "mask-image",
    "mask-size",
    "mask-position",
    "mask-repeat",
    "transition-property",
    "transition-duration",
    "transition-delay",
    "transition-timing-function",
    "transition",
    "animation-name",
    "animation-duration",
    "animation-delay",
    "animation-timing-function",
    "animation-iteration-count",
    "animation-direction",
    "animation-fill-mode",
    "animation-play-state",
    "animation",
];

#[test]
fn property_schema_dispatch_exposes_property_specific_authored_wrappers() {
    for vector in PROPERTY_DISPATCH_VECTORS.iter().skip(1) {
        let source = format!("{}: {}", vector.property_name, vector.authored_value);
        let report = parse_style_attribute(&source);
        assert!(report.is_clean(), "{source}: {:?}", report.diagnostics());
        let declaration = report.syntax()[0].known().expect("known declaration");
        let value = declaration
            .property_value()
            .expect("ordinary value has a property wrapper");
        assert_eq!(
            declaration.property().canonical_name(),
            vector.property_name
        );
        assert_eq!(property_value_css(value), vector.authored_value);
    }
}

fn property_value_css(value: CssKnownPropertyValueRef<'_>) -> &str {
    match value {
        CssKnownPropertyValueRef::All(value) => value.as_css(),
        CssKnownPropertyValueRef::Display(value) => value.as_css(),
        CssKnownPropertyValueRef::BoxSizing(value) => value.as_css(),
        CssKnownPropertyValueRef::Position(value) => value.as_css(),
        CssKnownPropertyValueRef::Direction(value) => value.as_css(),
        CssKnownPropertyValueRef::Overflow(value) => value.as_css(),
        CssKnownPropertyValueRef::OverflowX(value) => value.as_css(),
        CssKnownPropertyValueRef::OverflowY(value) => value.as_css(),
        CssKnownPropertyValueRef::FlexDirection(value) => value.as_css(),
        CssKnownPropertyValueRef::FlexWrap(value) => value.as_css(),
        CssKnownPropertyValueRef::Float(value) => value.as_css(),
        CssKnownPropertyValueRef::Clear(value) => value.as_css(),
        CssKnownPropertyValueRef::AlignContent(value) => value.as_css(),
        CssKnownPropertyValueRef::JustifyContent(value) => value.as_css(),
        CssKnownPropertyValueRef::AlignItems(value) => value.as_css(),
        CssKnownPropertyValueRef::AlignSelf(value) => value.as_css(),
        CssKnownPropertyValueRef::JustifyItems(value) => value.as_css(),
        CssKnownPropertyValueRef::JustifySelf(value) => value.as_css(),
        CssKnownPropertyValueRef::PlaceContent(value) => value.as_css(),
        CssKnownPropertyValueRef::PlaceItems(value) => value.as_css(),
        CssKnownPropertyValueRef::PlaceSelf(value) => value.as_css(),
        CssKnownPropertyValueRef::Visibility(value) => value.as_css(),
        CssKnownPropertyValueRef::Content(value) => value.as_css(),
        CssKnownPropertyValueRef::ContentVisibility(value) => value.as_css(),
        CssKnownPropertyValueRef::ListStyleType(value) => value.as_css(),
        CssKnownPropertyValueRef::ListStylePosition(value) => value.as_css(),
        CssKnownPropertyValueRef::ListStyleImage(value) => value.as_css(),
        CssKnownPropertyValueRef::ListStyle(value) => value.as_css(),
        CssKnownPropertyValueRef::CounterReset(value) => value.as_css(),
        CssKnownPropertyValueRef::CounterIncrement(value) => value.as_css(),
        CssKnownPropertyValueRef::CounterSet(value) => value.as_css(),
        CssKnownPropertyValueRef::Width(value) => value.as_css(),
        CssKnownPropertyValueRef::Height(value) => value.as_css(),
        CssKnownPropertyValueRef::MinWidth(value) => value.as_css(),
        CssKnownPropertyValueRef::MinHeight(value) => value.as_css(),
        CssKnownPropertyValueRef::MaxWidth(value) => value.as_css(),
        CssKnownPropertyValueRef::MaxHeight(value) => value.as_css(),
        CssKnownPropertyValueRef::FlexBasis(value) => value.as_css(),
        CssKnownPropertyValueRef::Gap(value) => value.as_css(),
        CssKnownPropertyValueRef::RowGap(value) => value.as_css(),
        CssKnownPropertyValueRef::ColumnGap(value) => value.as_css(),
        CssKnownPropertyValueRef::GridFlowTolerance(value) => value.as_css(),
        CssKnownPropertyValueRef::GridTemplateRows(value) => value.as_css(),
        CssKnownPropertyValueRef::GridTemplateColumns(value) => value.as_css(),
        CssKnownPropertyValueRef::GridTemplateAreas(value) => value.as_css(),
        CssKnownPropertyValueRef::GridTemplate(value) => value.as_css(),
        CssKnownPropertyValueRef::GridAutoRows(value) => value.as_css(),
        CssKnownPropertyValueRef::GridAutoColumns(value) => value.as_css(),
        CssKnownPropertyValueRef::GridAutoFlow(value) => value.as_css(),
        CssKnownPropertyValueRef::GridRowStart(value) => value.as_css(),
        CssKnownPropertyValueRef::GridRowEnd(value) => value.as_css(),
        CssKnownPropertyValueRef::GridColumnStart(value) => value.as_css(),
        CssKnownPropertyValueRef::GridColumnEnd(value) => value.as_css(),
        CssKnownPropertyValueRef::GridRow(value) => value.as_css(),
        CssKnownPropertyValueRef::GridColumn(value) => value.as_css(),
        CssKnownPropertyValueRef::GridArea(value) => value.as_css(),
        CssKnownPropertyValueRef::Grid(value) => value.as_css(),
        CssKnownPropertyValueRef::FontSize(value) => value.as_css(),
        CssKnownPropertyValueRef::LineHeight(value) => value.as_css(),
        CssKnownPropertyValueRef::WritingMode(value) => value.as_css(),
        CssKnownPropertyValueRef::TextAlign(value) => value.as_css(),
        CssKnownPropertyValueRef::TextAlignLast(value) => value.as_css(),
        CssKnownPropertyValueRef::TextIndent(value) => value.as_css(),
        CssKnownPropertyValueRef::VerticalAlign(value) => value.as_css(),
        CssKnownPropertyValueRef::FontFamily(value) => value.as_css(),
        CssKnownPropertyValueRef::Font(value) => value.as_css(),
        CssKnownPropertyValueRef::FontWeight(value) => value.as_css(),
        CssKnownPropertyValueRef::FontStyle(value) => value.as_css(),
        CssKnownPropertyValueRef::FontStretch(value) => value.as_css(),
        CssKnownPropertyValueRef::FontVariant(value) => value.as_css(),
        CssKnownPropertyValueRef::FontFeatureSettings(value) => value.as_css(),
        CssKnownPropertyValueRef::LetterSpacing(value) => value.as_css(),
        CssKnownPropertyValueRef::TextWrap(value) => value.as_css(),
        CssKnownPropertyValueRef::WhiteSpace(value) => value.as_css(),
        CssKnownPropertyValueRef::WordBreak(value) => value.as_css(),
        CssKnownPropertyValueRef::OverflowWrap(value) => value.as_css(),
        CssKnownPropertyValueRef::TextOverflow(value) => value.as_css(),
        CssKnownPropertyValueRef::TextDecoration(value) => value.as_css(),
        CssKnownPropertyValueRef::TextDecorationLine(value) => value.as_css(),
        CssKnownPropertyValueRef::TextDecorationColor(value) => value.as_css(),
        CssKnownPropertyValueRef::TextDecorationStyle(value) => value.as_css(),
        CssKnownPropertyValueRef::TextDecorationThickness(value) => value.as_css(),
        CssKnownPropertyValueRef::TextTransform(value) => value.as_css(),
        CssKnownPropertyValueRef::Inset(value) => value.as_css(),
        CssKnownPropertyValueRef::Top(value) => value.as_css(),
        CssKnownPropertyValueRef::Right(value) => value.as_css(),
        CssKnownPropertyValueRef::Bottom(value) => value.as_css(),
        CssKnownPropertyValueRef::Left(value) => value.as_css(),
        CssKnownPropertyValueRef::ZIndex(value) => value.as_css(),
        CssKnownPropertyValueRef::BoxDecorationBreak(value) => value.as_css(),
        CssKnownPropertyValueRef::Margin(value) => value.as_css(),
        CssKnownPropertyValueRef::MarginTop(value) => value.as_css(),
        CssKnownPropertyValueRef::MarginRight(value) => value.as_css(),
        CssKnownPropertyValueRef::MarginBottom(value) => value.as_css(),
        CssKnownPropertyValueRef::MarginLeft(value) => value.as_css(),
        CssKnownPropertyValueRef::Padding(value) => value.as_css(),
        CssKnownPropertyValueRef::PaddingTop(value) => value.as_css(),
        CssKnownPropertyValueRef::PaddingRight(value) => value.as_css(),
        CssKnownPropertyValueRef::PaddingBottom(value) => value.as_css(),
        CssKnownPropertyValueRef::PaddingLeft(value) => value.as_css(),
        CssKnownPropertyValueRef::Border(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderTop(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderRight(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderBottom(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderLeft(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderWidth(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderTopWidth(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderRightWidth(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderBottomWidth(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderLeftWidth(value) => value.as_css(),
        CssKnownPropertyValueRef::Color(value) => value.as_css(),
        CssKnownPropertyValueRef::Background(value) => value.as_css(),
        CssKnownPropertyValueRef::BackgroundColor(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderColor(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderTopColor(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderRightColor(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderBottomColor(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderLeftColor(value) => value.as_css(),
        CssKnownPropertyValueRef::BackgroundImage(value) => value.as_css(),
        CssKnownPropertyValueRef::BackgroundPosition(value) => value.as_css(),
        CssKnownPropertyValueRef::BackgroundSize(value) => value.as_css(),
        CssKnownPropertyValueRef::BackgroundRepeat(value) => value.as_css(),
        CssKnownPropertyValueRef::BackgroundOrigin(value) => value.as_css(),
        CssKnownPropertyValueRef::BackgroundClip(value) => value.as_css(),
        CssKnownPropertyValueRef::BackgroundAttachment(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderStyle(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderTopStyle(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderRightStyle(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderBottomStyle(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderLeftStyle(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderRadius(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderTopLeftRadius(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderTopRightRadius(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderBottomRightRadius(value) => value.as_css(),
        CssKnownPropertyValueRef::BorderBottomLeftRadius(value) => value.as_css(),
        CssKnownPropertyValueRef::BoxShadow(value) => value.as_css(),
        CssKnownPropertyValueRef::Opacity(value) => value.as_css(),
        CssKnownPropertyValueRef::FlexGrow(value) => value.as_css(),
        CssKnownPropertyValueRef::FlexShrink(value) => value.as_css(),
        CssKnownPropertyValueRef::Order(value) => value.as_css(),
        CssKnownPropertyValueRef::Flex(value) => value.as_css(),
        CssKnownPropertyValueRef::JustifyTracks(value) => value.as_css(),
        CssKnownPropertyValueRef::AlignTracks(value) => value.as_css(),
        CssKnownPropertyValueRef::AspectRatio(value) => value.as_css(),
        CssKnownPropertyValueRef::ScrollbarWidth(value) => value.as_css(),
        CssKnownPropertyValueRef::Cursor(value) => value.as_css(),
        CssKnownPropertyValueRef::PointerEvents(value) => value.as_css(),
        CssKnownPropertyValueRef::UserSelect(value) => value.as_css(),
        CssKnownPropertyValueRef::Outline(value) => value.as_css(),
        CssKnownPropertyValueRef::OutlineColor(value) => value.as_css(),
        CssKnownPropertyValueRef::OutlineStyle(value) => value.as_css(),
        CssKnownPropertyValueRef::OutlineWidth(value) => value.as_css(),
        CssKnownPropertyValueRef::Transform(value) => value.as_css(),
        CssKnownPropertyValueRef::TransformOrigin(value) => value.as_css(),
        CssKnownPropertyValueRef::Translate(value) => value.as_css(),
        CssKnownPropertyValueRef::Rotate(value) => value.as_css(),
        CssKnownPropertyValueRef::Scale(value) => value.as_css(),
        CssKnownPropertyValueRef::Filter(value) => value.as_css(),
        CssKnownPropertyValueRef::BackdropFilter(value) => value.as_css(),
        CssKnownPropertyValueRef::ClipPath(value) => value.as_css(),
        CssKnownPropertyValueRef::Mask(value) => value.as_css(),
        CssKnownPropertyValueRef::MaskImage(value) => value.as_css(),
        CssKnownPropertyValueRef::MaskSize(value) => value.as_css(),
        CssKnownPropertyValueRef::MaskPosition(value) => value.as_css(),
        CssKnownPropertyValueRef::MaskRepeat(value) => value.as_css(),
        CssKnownPropertyValueRef::TransitionProperty(value) => value.as_css(),
        CssKnownPropertyValueRef::TransitionDuration(value) => value.as_css(),
        CssKnownPropertyValueRef::TransitionDelay(value) => value.as_css(),
        CssKnownPropertyValueRef::TransitionTimingFunction(value) => value.as_css(),
        CssKnownPropertyValueRef::Transition(value) => value.as_css(),
        CssKnownPropertyValueRef::AnimationName(value) => value.as_css(),
        CssKnownPropertyValueRef::AnimationDuration(value) => value.as_css(),
        CssKnownPropertyValueRef::AnimationDelay(value) => value.as_css(),
        CssKnownPropertyValueRef::AnimationTimingFunction(value) => value.as_css(),
        CssKnownPropertyValueRef::AnimationIterationCount(value) => value.as_css(),
        CssKnownPropertyValueRef::AnimationDirection(value) => value.as_css(),
        CssKnownPropertyValueRef::AnimationFillMode(value) => value.as_css(),
        CssKnownPropertyValueRef::AnimationPlayState(value) => value.as_css(),
        CssKnownPropertyValueRef::Animation(value) => value.as_css(),
        _ => "<future property value>",
    }
}

#[derive(Clone, Copy)]
struct DispatchVector {
    property_name: &'static str,
    authored_value: &'static str,
}

macro_rules! dispatch_vector {
    ($property_name:literal, $authored_value:literal) => {
        DispatchVector {
            property_name: $property_name,
            authored_value: $authored_value,
        }
    };
}

// This table is intentionally hand-authored separately from the property schema and its
// implementation inventory. Each value is an ordinary property-specific probe: successful
// vectors must pass generated dispatch, while `all` deliberately exercises its typed error path.
const PROPERTY_DISPATCH_VECTORS: &[DispatchVector] = &[
    dispatch_vector!("all", "block"),
    dispatch_vector!("display", "block"),
    dispatch_vector!("box-sizing", "border-box"),
    dispatch_vector!("position", "sticky"),
    dispatch_vector!("direction", "rtl"),
    dispatch_vector!("overflow", "hidden scroll"),
    dispatch_vector!("overflow-x", "clip"),
    dispatch_vector!("overflow-y", "visible"),
    dispatch_vector!("float", "left"),
    dispatch_vector!("clear", "both"),
    dispatch_vector!("visibility", "collapse"),
    dispatch_vector!("content-visibility", "auto"),
    dispatch_vector!("flex-direction", "column-reverse"),
    dispatch_vector!("flex-wrap", "wrap-reverse"),
    dispatch_vector!("align-content", "space-between"),
    dispatch_vector!("justify-content", "safe center"),
    dispatch_vector!("align-items", "first baseline"),
    dispatch_vector!("align-self", "safe flex-end"),
    dispatch_vector!("justify-items", "stretch"),
    dispatch_vector!("justify-self", "center"),
    dispatch_vector!("place-content", "center end"),
    dispatch_vector!("place-items", "stretch"),
    dispatch_vector!("place-self", "end center"),
    dispatch_vector!("gap", "12px"),
    dispatch_vector!("row-gap", "normal"),
    dispatch_vector!("column-gap", "5%"),
    dispatch_vector!("flex-basis", "10rem"),
    dispatch_vector!("flex-grow", "2"),
    dispatch_vector!("flex-shrink", "0"),
    dispatch_vector!("order", "-2"),
    dispatch_vector!("flex", "2 0 10rem"),
    dispatch_vector!("justify-tracks", "space-evenly"),
    dispatch_vector!("align-tracks", "center"),
    dispatch_vector!("content", r#""Chapter ""#),
    dispatch_vector!("list-style-type", "square"),
    dispatch_vector!("list-style-position", "inside"),
    dispatch_vector!("list-style-image", "url(marker.svg)"),
    dispatch_vector!("list-style", "url(marker.svg) inside square"),
    dispatch_vector!("counter-reset", "section 2"),
    dispatch_vector!("counter-increment", "section 1"),
    dispatch_vector!("counter-set", "section 3"),
    dispatch_vector!("width", "calc(100% - 12px)"),
    dispatch_vector!("height", "auto"),
    dispatch_vector!("min-width", "0"),
    dispatch_vector!("min-height", "min-content"),
    dispatch_vector!("max-width", "max-content"),
    dispatch_vector!("max-height", "fit-content"),
    dispatch_vector!("grid-flow-tolerance", "infinite"),
    dispatch_vector!("grid-template-rows", "[top] 100px 1fr"),
    dispatch_vector!("grid-template-columns", "repeat(2, minmax(10px, 1fr))"),
    dispatch_vector!("grid-template-areas", r#""header header" "nav main""#),
    dispatch_vector!("grid-template", "100px 1fr / repeat(2, minmax(10px, 1fr))"),
    dispatch_vector!("grid-auto-rows", "minmax(10px, auto)"),
    dispatch_vector!("grid-auto-columns", "fit-content(20%)"),
    dispatch_vector!("grid-auto-flow", "column dense"),
    dispatch_vector!("grid-row-start", "span 2 main"),
    dispatch_vector!("grid-row-end", "auto"),
    dispatch_vector!("grid-column-start", "nav"),
    dispatch_vector!("grid-column-end", "4"),
    dispatch_vector!("grid-row", "1 / span 2"),
    dispatch_vector!("grid-column", "nav / main"),
    dispatch_vector!("grid-area", "header / 1 / span 2 / main"),
    dispatch_vector!("grid", "auto-flow dense 12px / repeat(auto-fit, 1fr)"),
    dispatch_vector!("aspect-ratio", "1.5"),
    dispatch_vector!("font-size", "16px"),
    dispatch_vector!("line-height", "normal"),
    dispatch_vector!("writing-mode", "vertical-rl"),
    dispatch_vector!("text-align", "start"),
    dispatch_vector!("text-align-last", "justify"),
    dispatch_vector!("text-indent", "1rem hanging each-line"),
    dispatch_vector!("vertical-align", "super"),
    dispatch_vector!("font-family", r#""Avenir Next", sans-serif"#),
    dispatch_vector!(
        "font",
        r#"italic small-caps 700 condensed 16px/normal "Avenir Next", sans-serif"#
    ),
    dispatch_vector!("font-weight", "725"),
    dispatch_vector!("font-style", "italic"),
    dispatch_vector!("font-stretch", "semi-condensed"),
    dispatch_vector!("font-variant", "small-caps"),
    dispatch_vector!("font-feature-settings", r#""kern" on, "liga" 0"#),
    dispatch_vector!("letter-spacing", "0.1em"),
    dispatch_vector!("text-wrap", "balance"),
    dispatch_vector!("white-space", "pre-wrap"),
    dispatch_vector!("word-break", "keep-all"),
    dispatch_vector!("overflow-wrap", "anywhere"),
    dispatch_vector!("text-overflow", "ellipsis"),
    dispatch_vector!("text-decoration", "underline dotted white 3px"),
    dispatch_vector!("text-decoration-line", "underline overline"),
    dispatch_vector!("text-decoration-color", "black"),
    dispatch_vector!("text-decoration-style", "wavy"),
    dispatch_vector!("text-decoration-thickness", "2px"),
    dispatch_vector!("text-transform", "uppercase"),
    dispatch_vector!("inset", "auto 10px 5%"),
    dispatch_vector!("top", "auto"),
    dispatch_vector!("right", "10px"),
    dispatch_vector!("bottom", "5%"),
    dispatch_vector!("left", "calc(3px + 4%)"),
    dispatch_vector!("z-index", "-2"),
    dispatch_vector!("box-decoration-break", "clone"),
    dispatch_vector!("margin", "auto 10px 5%"),
    dispatch_vector!("margin-top", "auto"),
    dispatch_vector!("margin-right", "10px"),
    dispatch_vector!("margin-bottom", "5%"),
    dispatch_vector!("margin-left", "calc(3px + 4%)"),
    dispatch_vector!("padding", "1px 2% calc(3px + 4%) 0"),
    dispatch_vector!("padding-top", "12px"),
    dispatch_vector!("padding-right", "2%"),
    dispatch_vector!("padding-bottom", "calc(3px + 4%)"),
    dispatch_vector!("padding-left", "0"),
    dispatch_vector!("border", "solid 2px #fff"),
    dispatch_vector!("border-top", "black dotted"),
    dispatch_vector!("border-right", "1px"),
    dispatch_vector!("border-bottom", "#fff"),
    dispatch_vector!("border-left", "dashed black"),
    dispatch_vector!("border-width", "1px 2px 3px 4px"),
    dispatch_vector!("border-top-width", "1px"),
    dispatch_vector!("border-right-width", "2px"),
    dispatch_vector!("border-bottom-width", "3px"),
    dispatch_vector!("border-left-width", "4px"),
    dispatch_vector!("color", "black"),
    dispatch_vector!("background", "#fff"),
    dispatch_vector!("background-color", "transparent"),
    dispatch_vector!("border-color", "black"),
    dispatch_vector!("border-top-color", "black"),
    dispatch_vector!("border-right-color", "white"),
    dispatch_vector!("border-bottom-color", "transparent"),
    dispatch_vector!("border-left-color", "#fff"),
    dispatch_vector!("background-image", "url(\"hero.png\"), none"),
    dispatch_vector!("background-position", "left 10px top 20%"),
    dispatch_vector!("background-size", "cover, 10px auto"),
    dispatch_vector!("background-repeat", "repeat-x, no-repeat round"),
    dispatch_vector!("background-origin", "content-box"),
    dispatch_vector!("background-clip", "padding-box"),
    dispatch_vector!("background-attachment", "fixed, local"),
    dispatch_vector!("border-style", "none hidden dotted dashed"),
    dispatch_vector!("border-top-style", "solid"),
    dispatch_vector!("border-right-style", "double"),
    dispatch_vector!("border-bottom-style", "ridge"),
    dispatch_vector!("border-left-style", "outset"),
    dispatch_vector!("border-radius", "1px 2px 3px / 4px 5px"),
    dispatch_vector!("border-top-left-radius", "4px 10%"),
    dispatch_vector!("border-top-right-radius", "1px"),
    dispatch_vector!("border-bottom-right-radius", "10%"),
    dispatch_vector!("border-bottom-left-radius", "calc(1px + 2%)"),
    dispatch_vector!("box-shadow", "inset 1px 2px 3px 4px black"),
    dispatch_vector!("opacity", "0.5"),
    dispatch_vector!("scrollbar-width", "thin"),
    dispatch_vector!("cursor", "grab"),
    dispatch_vector!("pointer-events", "none"),
    dispatch_vector!("user-select", "text"),
    dispatch_vector!("outline", "thick dotted white"),
    dispatch_vector!("outline-color", "black"),
    dispatch_vector!("outline-style", "auto"),
    dispatch_vector!("outline-width", "2px"),
    dispatch_vector!(
        "transform",
        "translate(10px, 20px) rotate(45deg) scale(1.5)"
    ),
    dispatch_vector!("transform-origin", "center top"),
    dispatch_vector!("translate", "10px 20px"),
    dispatch_vector!("rotate", "45deg"),
    dispatch_vector!("scale", "1.5 2"),
    dispatch_vector!("filter", "blur(4px) opacity(50%)"),
    dispatch_vector!("backdrop-filter", "none"),
    dispatch_vector!("clip-path", "circle(50% at center)"),
    dispatch_vector!("mask", "url(mask.png) center / contain no-repeat"),
    dispatch_vector!("mask-image", "url(mask.png), none"),
    dispatch_vector!("mask-size", "contain"),
    dispatch_vector!("mask-position", "center"),
    dispatch_vector!("mask-repeat", "repeat"),
    dispatch_vector!("transition-property", "opacity, transform"),
    dispatch_vector!("transition-duration", "150ms, 2s"),
    dispatch_vector!("transition-delay", "20ms"),
    dispatch_vector!(
        "transition-timing-function",
        "ease-in, cubic-bezier(0.1, 0.2, 0.3, 1)"
    ),
    dispatch_vector!(
        "transition",
        "opacity 150ms ease-in 20ms, transform 2s linear"
    ),
    dispatch_vector!("animation-name", "fade, none"),
    dispatch_vector!("animation-duration", "1s"),
    dispatch_vector!("animation-delay", "200ms"),
    dispatch_vector!("animation-timing-function", "ease-out"),
    dispatch_vector!("animation-iteration-count", "2, infinite"),
    dispatch_vector!("animation-direction", "alternate"),
    dispatch_vector!("animation-fill-mode", "both"),
    dispatch_vector!("animation-play-state", "running, paused"),
    dispatch_vector!(
        "animation",
        "fade 1s ease-in 200ms 3 alternate both running"
    ),
];

#[test]
fn property_schema_frozen_names_have_generated_identity() {
    assert_eq!(FROZEN_PROPERTIES.len(), 179);

    let mut names = HashSet::new();
    let mut ids = HashSet::new();
    for &name in FROZEN_PROPERTIES {
        assert!(
            names.insert(name),
            "duplicate frozen property name `{name}`"
        );

        let property = CssKnownProperty::from_name(name)
            .unwrap_or_else(|| panic!("missing generated identity for `{name}`"));
        assert_eq!(property.canonical_name(), name);
        assert_eq!(property.stable_id(), format!("baseline.property.{name}"));
        assert!(property.aliases().is_empty());
        assert!(ids.insert(property.stable_id()));

        let mixed_case = name.to_ascii_uppercase();
        assert_eq!(CssKnownProperty::from_name(&mixed_case), Some(property));
    }

    assert_eq!(names.len(), 179);
    assert_eq!(ids.len(), 179);
    assert_eq!(CssKnownProperty::all().len(), 179);
    assert!(CssKnownProperty::from_name("--custom").is_none());
    assert!(CssKnownProperty::from_name("definitely-unknown").is_none());
}

#[test]
fn property_schema_parser_identity_matches_every_frozen_name() {
    assert_eq!(PROPERTY_DISPATCH_VECTORS.len(), 179);

    let frozen_names: HashSet<_> = FROZEN_PROPERTIES.iter().copied().collect();
    let mut vector_names = HashSet::new();

    for vector in PROPERTY_DISPATCH_VECTORS {
        let name = vector.property_name;
        assert!(
            vector_names.insert(name),
            "duplicate dispatch vector for `{name}`"
        );
        assert!(
            !["inherit", "initial", "unset", "revert", "revert-layer"]
                .contains(&vector.authored_value.trim().to_ascii_lowercase().as_str()),
            "`{name}` dispatch vector must not use a CSS-wide global keyword"
        );
        assert!(
            !vector.authored_value.to_ascii_lowercase().contains("var("),
            "`{name}` dispatch vector must not use substitution-dependent parsing"
        );

        let source = format!(
            ".test {{ {}: {}; }}",
            name.to_ascii_uppercase(),
            vector.authored_value
        );

        let important_value = if name == "all" {
            "inherit"
        } else {
            vector.authored_value
        };
        let important_source = format!(
            ".test {{ {}: {} !important; }}",
            name.to_ascii_uppercase(),
            important_value
        );
        let important_sheet = parse_sheet(&important_source).unwrap_or_else(|error| {
            panic!("`{name}: {important_value} !important` must parse: {error}")
        });
        let [CssRule::Style(important_rule)] = important_sheet.rules() else {
            panic!("`{name}` importance path should produce one style rule");
        };
        let [important_declaration] = important_rule.declarations().as_slice() else {
            panic!("`{name}` importance path should produce one declaration");
        };
        assert_eq!(important_declaration.importance(), CssImportance::Important);
        assert_eq!(
            important_declaration.known().map(|known| known.property()),
            CssKnownProperty::from_name(name),
            "`{name}` importance path diverged from schema identity",
        );

        if name == "all" {
            let error = parse_sheet(&source).expect_err("`all` ordinary value must be rejected");
            assert_eq!(error.code(), CssErrorCode::InvalidPropertyValue);
            let ErrorKind::InvalidPropertyValue(detail) = error.kind() else {
                panic!(
                    "`all` returned the wrong structured error: {:?}",
                    error.kind()
                );
            };
            assert_eq!(detail.property(), CssKnownProperty::All);
            continue;
        }

        let sheet = parse_sheet(&source).unwrap_or_else(|error| {
            panic!(
                "`{name}: {}` should parse through generated dispatch: {error}",
                vector.authored_value
            )
        });
        let [CssRule::Style(rule)] = sheet.rules() else {
            panic!("`{name}` should produce one style rule");
        };
        let [declaration] = rule.declarations().as_slice() else {
            panic!("`{name}` should produce one declaration");
        };
        assert_eq!(
            declaration.known().map(|known| known.property()),
            CssKnownProperty::from_name(name),
            "`{name}` parser identity diverged from schema lookup",
        );
    }

    assert_eq!(vector_names, frozen_names);
}

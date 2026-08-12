mod common;

use common::CssParseReportTestExt;
use surgeist_css::{
    CssErrorCode, CssImportance, CssKnownProperty, CssKnownPropertyValueRef, CssRule, ErrorKind,
    parse_sheet, parse_style_attribute,
};

macro_rules! assert_property_specific_css {
    ($declaration:expr, $value:expr, $expected:expr; $($variant:ident),+ $(,)?) => {
        match ($declaration.property(), $value) {
            $(
                (CssKnownProperty::$variant, CssKnownPropertyValueRef::$variant(value)) => {
                    assert_eq!(value.as_css(), $expected)
                }
            )+
            _ => panic!("property/value wrapper mismatch for `{}`", $expected),
        }
    };
}

macro_rules! with_property_value_variants {
    ($callback:ident, $declaration:expr, $value:expr, $expected:expr) => {
        $callback! {
            $declaration, $value, $expected;
            All,
            Display,
            BoxSizing,
            Position,
            Direction,
            Overflow,
            OverflowX,
            OverflowY,
            FlexDirection,
            FlexWrap,
            Float,
            Clear,
            AlignContent,
            JustifyContent,
            AlignItems,
            AlignSelf,
            JustifyItems,
            JustifySelf,
            PlaceContent,
            PlaceItems,
            PlaceSelf,
            Visibility,
            Content,
            ContentVisibility,
            ListStyleType,
            ListStylePosition,
            ListStyleImage,
            ListStyle,
            CounterReset,
            CounterIncrement,
            CounterSet,
            Width,
            Height,
            MinWidth,
            MinHeight,
            MaxWidth,
            MaxHeight,
            FlexBasis,
            Gap,
            RowGap,
            ColumnGap,
            GridFlowTolerance,
            GridTemplateRows,
            GridTemplateColumns,
            GridTemplateAreas,
            GridTemplate,
            GridAutoRows,
            GridAutoColumns,
            GridAutoFlow,
            GridRowStart,
            GridRowEnd,
            GridColumnStart,
            GridColumnEnd,
            GridRow,
            GridColumn,
            GridArea,
            Grid,
            FontSize,
            LineHeight,
            WritingMode,
            TextAlign,
            TextAlignLast,
            TextIndent,
            VerticalAlign,
            FontFamily,
            Font,
            FontWeight,
            FontStyle,
            FontStretch,
            FontVariant,
            FontFeatureSettings,
            LetterSpacing,
            TextWrap,
            WhiteSpace,
            WordBreak,
            OverflowWrap,
            TextOverflow,
            TextDecoration,
            TextDecorationLine,
            TextDecorationColor,
            TextDecorationStyle,
            TextDecorationThickness,
            TextTransform,
            Inset,
            Top,
            Right,
            Bottom,
            Left,
            ZIndex,
            BoxDecorationBreak,
            Margin,
            MarginTop,
            MarginRight,
            MarginBottom,
            MarginLeft,
            Padding,
            PaddingTop,
            PaddingRight,
            PaddingBottom,
            PaddingLeft,
            Border,
            BorderTop,
            BorderRight,
            BorderBottom,
            BorderLeft,
            BorderWidth,
            BorderTopWidth,
            BorderRightWidth,
            BorderBottomWidth,
            BorderLeftWidth,
            Color,
            Background,
            BackgroundColor,
            BorderColor,
            BorderTopColor,
            BorderRightColor,
            BorderBottomColor,
            BorderLeftColor,
            BackgroundImage,
            BackgroundPosition,
            BackgroundSize,
            BackgroundRepeat,
            BackgroundOrigin,
            BackgroundClip,
            BackgroundAttachment,
            BorderStyle,
            BorderTopStyle,
            BorderRightStyle,
            BorderBottomStyle,
            BorderLeftStyle,
            BorderRadius,
            BorderTopLeftRadius,
            BorderTopRightRadius,
            BorderBottomRightRadius,
            BorderBottomLeftRadius,
            BoxShadow,
            Opacity,
            FlexGrow,
            FlexShrink,
            Order,
            Flex,
            JustifyTracks,
            AlignTracks,
            AspectRatio,
            ScrollbarWidth,
            Cursor,
            PointerEvents,
            UserSelect,
            Outline,
            OutlineColor,
            OutlineStyle,
            OutlineWidth,
            Transform,
            TransformOrigin,
            Translate,
            Rotate,
            Scale,
            Filter,
            BackdropFilter,
            ClipPath,
            Mask,
            MaskImage,
            MaskSize,
            MaskPosition,
            MaskRepeat,
            TransitionProperty,
            TransitionDuration,
            TransitionDelay,
            TransitionTimingFunction,
            Transition,
            AnimationName,
            AnimationDuration,
            AnimationDelay,
            AnimationTimingFunction,
            AnimationIterationCount,
            AnimationDirection,
            AnimationFillMode,
            AnimationPlayState,
            Animation,
        }
    };
}

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
        with_property_value_variants!(
            assert_property_specific_css,
            declaration,
            value,
            vector.authored_value
        );
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

// Each authored case is an explicit public parser stimulus. Ordinary values must
// expose their concrete property wrapper, while `all` exercises its typed error
// and global-keyword paths.
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
fn explicit_property_dispatch_cases_preserve_ordinary_and_important_behavior() {
    for vector in PROPERTY_DISPATCH_VECTORS {
        let name = vector.property_name;
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
            assert_eq!(
                important_declaration
                    .known()
                    .and_then(|known| known.global()),
                Some(surgeist_css::CssGlobalKeyword::Inherit),
            );
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

        let declaration = declaration.known().expect("known ordinary declaration");
        let value = declaration
            .property_value()
            .expect("ordinary value has a property wrapper");
        with_property_value_variants!(
            assert_property_specific_css,
            declaration,
            value,
            vector.authored_value
        );

        let important_declaration = important_declaration
            .known()
            .expect("known important declaration");
        let important_value = important_declaration
            .property_value()
            .expect("important ordinary value has a property wrapper");
        with_property_value_variants!(
            assert_property_specific_css,
            important_declaration,
            important_value,
            vector.authored_value
        );
    }
}

mod common;

use common::CssParseReportTestExt;
use surgeist_css::{
    CssErrorCode, CssImportance, CssKnownProperty, CssKnownPropertyValueRef, CssRecoveryAction,
    CssRule, ErrorKind, parse_sheet, parse_style_attribute,
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

#[test]
fn layered_position_wrappers_keep_current_global_and_substitution_branches_distinct() {
    let report = parse_style_attribute(concat!(
        "background-position: left 10px top, center; ",
        "mask-position: right bottom; ",
        "background-position: inherit; ",
        "mask-position: var(--position)",
    ));
    assert!(report.is_clean(), "{:?}", report.diagnostics());

    let background = report.syntax()[0].known().expect("background declaration");
    let CssKnownPropertyValueRef::BackgroundPosition(value) = background
        .property_value()
        .expect("ordinary background position")
    else {
        panic!("expected background-position value");
    };
    assert_eq!(value.positions().positions().len(), 2);
    assert!(background.global().is_none());
    assert!(background.substitution_dependent().is_none());

    let mask = report.syntax()[1].known().expect("mask declaration");
    let CssKnownPropertyValueRef::MaskPosition(value) =
        mask.property_value().expect("ordinary mask position")
    else {
        panic!("expected mask-position value");
    };
    assert_eq!(value.positions().positions().len(), 1);
    assert!(mask.global().is_none());
    assert!(mask.substitution_dependent().is_none());

    let global = report.syntax()[2].known().expect("global declaration");
    assert!(global.property_value().is_none());
    assert!(global.global().is_some());
    assert!(global.substitution_dependent().is_none());

    let substitution = report.syntax()[3]
        .known()
        .expect("substitution-dependent declaration");
    assert!(substitution.property_value().is_none());
    assert!(substitution.global().is_none());
    assert!(substitution.substitution_dependent().is_some());
}

#[test]
fn timing_wrappers_expose_exact_property_specific_current_accessors() {
    let report = parse_style_attribute(concat!(
        "transition-duration: calc(1s + 2s); transition-delay: -1s; ",
        "animation-duration: calc(3ms * 2); animation-delay: -4ms; ",
        "animation-iteration-count: calc(1 + 2); ",
        "transition: opacity 1s -2s; animation: fade 3s -4s 2"
    ));
    assert!(report.is_clean(), "{:?}", report.diagnostics());

    for declaration in report.syntax().as_slice() {
        match declaration.known().unwrap().property_value().unwrap() {
            CssKnownPropertyValueRef::TransitionDuration(value) => {
                assert_eq!(value.durations().values().len(), 1);
                assert!(value.i01_subset().is_none());
            }
            CssKnownPropertyValueRef::TransitionDelay(value) => {
                assert_eq!(value.delays().values().len(), 1);
                assert!(value.i01_subset().is_none());
            }
            CssKnownPropertyValueRef::AnimationDuration(value) => {
                assert_eq!(value.durations().values().len(), 1);
                assert!(value.i01_subset().is_none());
            }
            CssKnownPropertyValueRef::AnimationDelay(value) => {
                assert_eq!(value.delays().values().len(), 1);
                assert!(value.i01_subset().is_none());
            }
            CssKnownPropertyValueRef::AnimationIterationCount(value) => {
                assert_eq!(value.iteration_counts().values().len(), 1);
                assert!(value.i01_subset().is_none());
            }
            CssKnownPropertyValueRef::Transition(value) => {
                assert_eq!(value.transitions().values().len(), 1);
                assert!(value.i01_subset().is_none());
            }
            CssKnownPropertyValueRef::Animation(value) => {
                assert_eq!(value.animations().values().len(), 1);
                assert!(value.i01_subset().is_none());
            }
            _ => panic!("unexpected property wrapper"),
        }
    }
}

#[test]
fn typed_length_calculations_are_accepted_by_the_exact_current_consumer_set() {
    for (property, value) in [
        ("width", "calc((1px + 2%) * 3)"),
        ("height", "calc((1px + 2%) * 3)"),
        ("min-width", "calc((1px + 2%) * 3)"),
        ("min-height", "calc((1px + 2%) * 3)"),
        ("max-width", "calc((1px + 2%) * 3)"),
        ("max-height", "calc((1px + 2%) * 3)"),
        ("flex-basis", "calc((1px + 2%) * 3)"),
        ("gap", "calc((1px + 2%) * 3)"),
        ("row-gap", "calc((1px + 2%) * 3)"),
        ("column-gap", "calc((1px + 2%) * 3)"),
        ("grid-template-rows", "calc((1px + 2%) * 3)"),
        ("grid-template-columns", "calc((1px + 2%) * 3)"),
        ("grid-auto-rows", "calc((1px + 2%) * 3)"),
        ("grid-auto-columns", "calc((1px + 2%) * 3)"),
        ("font-size", "calc((1px + 2%) * 3)"),
        ("line-height", "calc((1px + 2%) * 3)"),
        (
            "font",
            "italic calc((10px + 2%) * 2)/calc((1px + 2%) * 3) serif",
        ),
        ("text-indent", "calc((1px + 2%) * 3) hanging"),
        ("vertical-align", "calc((1px + 2%) * 3)"),
        ("letter-spacing", "calc((1px + 2em) * 3)"),
        ("text-decoration", "underline calc((1px + 2%) * 3)"),
        ("text-decoration-thickness", "calc((1px + 2%) * 3)"),
        ("inset", "calc((1px + 2%) * 3)"),
        ("top", "calc((1px + 2%) * 3)"),
        ("right", "calc((1px + 2%) * 3)"),
        ("bottom", "calc((1px + 2%) * 3)"),
        ("left", "calc((1px + 2%) * 3)"),
        ("margin", "calc((1px + 2%) * 3)"),
        ("margin-top", "calc((1px + 2%) * 3)"),
        ("margin-right", "calc((1px + 2%) * 3)"),
        ("margin-bottom", "calc((1px + 2%) * 3)"),
        ("margin-left", "calc((1px + 2%) * 3)"),
        ("padding", "calc((-1px + 2%) * 3)"),
        ("padding-top", "calc((-1px + 2%) * 3)"),
        ("padding-right", "calc((-1px + 2%) * 3)"),
        ("padding-bottom", "calc((-1px + 2%) * 3)"),
        ("padding-left", "calc((-1px + 2%) * 3)"),
        ("border", "solid calc((-1px + 2px) * 3)"),
        ("border-top", "solid calc((-1px + 2px) * 3)"),
        ("border-right", "solid calc((-1px + 2px) * 3)"),
        ("border-bottom", "solid calc((-1px + 2px) * 3)"),
        ("border-left", "solid calc((-1px + 2px) * 3)"),
        ("border-width", "calc((-1px + 2px) * 3)"),
        ("border-top-width", "calc((-1px + 2px) * 3)"),
        ("border-right-width", "calc((-1px + 2px) * 3)"),
        ("border-bottom-width", "calc((-1px + 2px) * 3)"),
        ("border-left-width", "calc((-1px + 2px) * 3)"),
        ("border-radius", "calc((-1px + 2%) * 3)"),
        ("border-top-left-radius", "calc((-1px + 2%) * 3)"),
        ("border-top-right-radius", "calc((-1px + 2%) * 3)"),
        ("border-bottom-right-radius", "calc((-1px + 2%) * 3)"),
        ("border-bottom-left-radius", "calc((-1px + 2%) * 3)"),
        ("box-shadow", "calc((1px + 2px) * 3) 2px"),
        ("outline", "solid calc((-1px + 2px) * 3)"),
        ("outline-width", "calc((-1px + 2px) * 3)"),
        ("background-position", "calc((1px + 2%) * 3) top"),
        ("background-size", "calc((-1px + 2%) * 3) auto"),
        ("mask-position", "calc((1px + 2%) * 3) top"),
        ("mask-size", "calc((-1px + 2%) * 3) auto"),
        ("transform-origin", "calc((1px + 2%) * 3) top"),
        ("translate", "calc((1px + 2%) * 3)"),
    ] {
        let source = format!("{property}: {value}");
        let report = parse_style_attribute(&source);
        assert!(report.is_clean(), "{source}: {:?}", report.diagnostics());
        assert_eq!(report.syntax().len(), 1, "{source}");
        assert_eq!(
            report.syntax()[0].known().unwrap().property(),
            CssKnownProperty::from_name(property).unwrap(),
            "{source}",
        );
    }

    for source in [
        "transform: translate(calc((1px + 2%) * 3))",
        "clip-path: polygon(calc((1px + 2%) * 3) 0px, 1px 1px)",
        "grid-template: calc((1px + 2%) * 3) / 1fr",
        "grid: calc((1px + 2%) * 3) / 1fr",
    ] {
        let report = parse_style_attribute(source);
        assert!(
            !report.is_clean(),
            "later function grammar changed: {source}"
        );
        assert!(report.syntax().is_empty(), "{source}");
    }
}

#[test]
fn deferred_basic_shape_math_drops_only_the_invalid_declaration() {
    for shape in [
        "circle(calc((1px + 2%)) 1px * 2)",
        "ellipse(calc((1px + 2%)) 1px * 2)",
        "inset(calc((1px + 2%)) 1px * 2)",
    ] {
        let invalid = format!("clip-path: {shape};");
        let source = format!("{invalid} color: red");
        let report = parse_style_attribute(&source);

        assert_eq!(report.syntax().len(), 1, "{source}");
        assert_eq!(
            report.syntax()[0].known().unwrap().property(),
            CssKnownProperty::Color,
            "{source}",
        );
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: expected one recovered declaration");
        };
        assert_eq!(
            diagnostic.error().code(),
            CssErrorCode::InvalidPropertyValue,
            "{source}",
        );
        assert_eq!(
            diagnostic.action(),
            CssRecoveryAction::DropDeclaration,
            "{source}",
        );
        assert_eq!(
            diagnostic.span().start().byte_offset().value(),
            0,
            "{source}"
        );
        assert_eq!(
            diagnostic.span().end().byte_offset().value(),
            invalid.len(),
            "{source}",
        );
        let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
            panic!("{source}: expected property-value detail");
        };
        assert_eq!(detail.property(), CssKnownProperty::ClipPath, "{source}");

        #[cfg(feature = "app-strict")]
        {
            let failure = surgeist_css::validate_style_attribute(&source)
                .expect_err("recovered basic-shape math must fail strict validation");
            assert_eq!(failure.diagnostics(), report.diagnostics(), "{source}");
        }
    }
}

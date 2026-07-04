use crate::{CssGlobalKeyword, CssLengthUnit, CssProperty};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LengthUnitStatus {
    Supported(CssLengthUnit),
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PropertyNameStatus {
    Supported,
    KnownUnsupported,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SupportedProperty {
    pub(crate) name: &'static str,
    pub(crate) property: CssProperty,
}

macro_rules! supported_property {
    ($name:literal, $property:ident) => {
        SupportedProperty {
            name: $name,
            property: CssProperty::$property,
        }
    };
}

const SUPPORTED_PROPERTIES: &[SupportedProperty] = &[
    supported_property!("all", All),
    supported_property!("display", Display),
    supported_property!("box-sizing", BoxSizing),
    supported_property!("position", Position),
    supported_property!("direction", Direction),
    supported_property!("overflow", Overflow),
    supported_property!("overflow-x", OverflowX),
    supported_property!("overflow-y", OverflowY),
    supported_property!("flex-direction", FlexDirection),
    supported_property!("flex-wrap", FlexWrap),
    supported_property!("float", Float),
    supported_property!("clear", Clear),
    supported_property!("align-content", AlignContent),
    supported_property!("justify-content", JustifyContent),
    supported_property!("align-items", AlignItems),
    supported_property!("align-self", AlignSelf),
    supported_property!("justify-items", JustifyItems),
    supported_property!("justify-self", JustifySelf),
    supported_property!("place-content", PlaceContent),
    supported_property!("place-items", PlaceItems),
    supported_property!("place-self", PlaceSelf),
    supported_property!("visibility", Visibility),
    supported_property!("content-visibility", ContentVisibility),
    supported_property!("width", Width),
    supported_property!("height", Height),
    supported_property!("min-width", MinWidth),
    supported_property!("min-height", MinHeight),
    supported_property!("max-width", MaxWidth),
    supported_property!("max-height", MaxHeight),
    supported_property!("flex-basis", FlexBasis),
    supported_property!("gap", Gap),
    supported_property!("row-gap", RowGap),
    supported_property!("column-gap", ColumnGap),
    supported_property!("grid-flow-tolerance", GridFlowTolerance),
    supported_property!("grid-template-rows", GridTemplateRows),
    supported_property!("grid-template-columns", GridTemplateColumns),
    supported_property!("grid-template-areas", GridTemplateAreas),
    supported_property!("grid-template", GridTemplate),
    supported_property!("grid-auto-rows", GridAutoRows),
    supported_property!("grid-auto-columns", GridAutoColumns),
    supported_property!("grid-auto-flow", GridAutoFlow),
    supported_property!("grid-row-start", GridRowStart),
    supported_property!("grid-row-end", GridRowEnd),
    supported_property!("grid-column-start", GridColumnStart),
    supported_property!("grid-column-end", GridColumnEnd),
    supported_property!("grid-row", GridRow),
    supported_property!("grid-column", GridColumn),
    supported_property!("grid-area", GridArea),
    supported_property!("grid", Grid),
    supported_property!("font-size", FontSize),
    supported_property!("line-height", LineHeight),
    supported_property!("writing-mode", WritingMode),
    supported_property!("text-align", TextAlign),
    supported_property!("text-align-last", TextAlignLast),
    supported_property!("text-indent", TextIndent),
    supported_property!("vertical-align", VerticalAlign),
    supported_property!("font-family", FontFamily),
    supported_property!("font", Font),
    supported_property!("font-weight", FontWeight),
    supported_property!("font-style", FontStyle),
    supported_property!("font-stretch", FontStretch),
    supported_property!("font-variant", FontVariant),
    supported_property!("font-feature-settings", FontFeatureSettings),
    supported_property!("letter-spacing", LetterSpacing),
    supported_property!("text-wrap", TextWrap),
    supported_property!("white-space", WhiteSpace),
    supported_property!("word-break", WordBreak),
    supported_property!("overflow-wrap", OverflowWrap),
    supported_property!("text-overflow", TextOverflow),
    supported_property!("text-decoration", TextDecoration),
    supported_property!("text-decoration-line", TextDecorationLine),
    supported_property!("text-decoration-color", TextDecorationColor),
    supported_property!("text-decoration-style", TextDecorationStyle),
    supported_property!("text-decoration-thickness", TextDecorationThickness),
    supported_property!("text-transform", TextTransform),
    supported_property!("inset", Inset),
    supported_property!("top", Top),
    supported_property!("right", Right),
    supported_property!("bottom", Bottom),
    supported_property!("left", Left),
    supported_property!("z-index", ZIndex),
    supported_property!("box-decoration-break", BoxDecorationBreak),
    supported_property!("margin", Margin),
    supported_property!("margin-top", MarginTop),
    supported_property!("margin-right", MarginRight),
    supported_property!("margin-bottom", MarginBottom),
    supported_property!("margin-left", MarginLeft),
    supported_property!("padding", Padding),
    supported_property!("padding-top", PaddingTop),
    supported_property!("padding-right", PaddingRight),
    supported_property!("padding-bottom", PaddingBottom),
    supported_property!("padding-left", PaddingLeft),
    supported_property!("border", Border),
    supported_property!("border-top", BorderTop),
    supported_property!("border-right", BorderRight),
    supported_property!("border-bottom", BorderBottom),
    supported_property!("border-left", BorderLeft),
    supported_property!("border-width", BorderWidth),
    supported_property!("border-top-width", BorderTopWidth),
    supported_property!("border-right-width", BorderRightWidth),
    supported_property!("border-bottom-width", BorderBottomWidth),
    supported_property!("border-left-width", BorderLeftWidth),
    supported_property!("color", Color),
    supported_property!("background", Background),
    supported_property!("background-color", BackgroundColor),
    supported_property!("border-color", BorderColor),
    supported_property!("border-top-color", BorderTopColor),
    supported_property!("border-right-color", BorderRightColor),
    supported_property!("border-bottom-color", BorderBottomColor),
    supported_property!("border-left-color", BorderLeftColor),
    supported_property!("background-image", BackgroundImage),
    supported_property!("background-position", BackgroundPosition),
    supported_property!("background-size", BackgroundSize),
    supported_property!("background-repeat", BackgroundRepeat),
    supported_property!("background-origin", BackgroundOrigin),
    supported_property!("background-clip", BackgroundClip),
    supported_property!("background-attachment", BackgroundAttachment),
    supported_property!("border-style", BorderStyle),
    supported_property!("border-top-style", BorderTopStyle),
    supported_property!("border-right-style", BorderRightStyle),
    supported_property!("border-bottom-style", BorderBottomStyle),
    supported_property!("border-left-style", BorderLeftStyle),
    supported_property!("border-radius", BorderRadius),
    supported_property!("border-top-left-radius", BorderTopLeftRadius),
    supported_property!("border-top-right-radius", BorderTopRightRadius),
    supported_property!("border-bottom-right-radius", BorderBottomRightRadius),
    supported_property!("border-bottom-left-radius", BorderBottomLeftRadius),
    supported_property!("box-shadow", BoxShadow),
    supported_property!("opacity", Opacity),
    supported_property!("flex-grow", FlexGrow),
    supported_property!("flex-shrink", FlexShrink),
    supported_property!("order", Order),
    supported_property!("flex", Flex),
    supported_property!("justify-tracks", JustifyTracks),
    supported_property!("align-tracks", AlignTracks),
    supported_property!("aspect-ratio", AspectRatio),
    supported_property!("scrollbar-width", ScrollbarWidth),
    supported_property!("cursor", Cursor),
    supported_property!("pointer-events", PointerEvents),
    supported_property!("user-select", UserSelect),
    supported_property!("outline", Outline),
    supported_property!("outline-color", OutlineColor),
    supported_property!("outline-style", OutlineStyle),
    supported_property!("outline-width", OutlineWidth),
    supported_property!("transform", Transform),
    supported_property!("transform-origin", TransformOrigin),
    supported_property!("translate", Translate),
    supported_property!("rotate", Rotate),
    supported_property!("scale", Scale),
    supported_property!("filter", Filter),
    supported_property!("backdrop-filter", BackdropFilter),
    supported_property!("clip-path", ClipPath),
    supported_property!("mask", Mask),
    supported_property!("mask-image", MaskImage),
    supported_property!("mask-size", MaskSize),
    supported_property!("mask-position", MaskPosition),
    supported_property!("mask-repeat", MaskRepeat),
    supported_property!("transition-property", TransitionProperty),
    supported_property!("transition-duration", TransitionDuration),
    supported_property!("transition-delay", TransitionDelay),
    supported_property!("transition-timing-function", TransitionTimingFunction),
    supported_property!("transition", Transition),
    supported_property!("animation-name", AnimationName),
    supported_property!("animation-duration", AnimationDuration),
    supported_property!("animation-delay", AnimationDelay),
    supported_property!("animation-timing-function", AnimationTimingFunction),
    supported_property!("animation-iteration-count", AnimationIterationCount),
    supported_property!("animation-direction", AnimationDirection),
    supported_property!("animation-fill-mode", AnimationFillMode),
    supported_property!("animation-play-state", AnimationPlayState),
    supported_property!("animation", Animation),
];

const KNOWN_UNSUPPORTED_PROPERTY_NAMES: &[&str] = &[];

pub(crate) const fn supported_properties() -> &'static [SupportedProperty] {
    SUPPORTED_PROPERTIES
}

pub(crate) fn property_for_supported_name(name: &str) -> Option<CssProperty> {
    supported_properties()
        .iter()
        .find(|property| property.name.eq_ignore_ascii_case(name))
        .map(|property| property.property)
}

pub(crate) fn classify_property_name(name: &str) -> PropertyNameStatus {
    if property_for_supported_name(name).is_some() {
        PropertyNameStatus::Supported
    } else if contains_ascii_case(KNOWN_UNSUPPORTED_PROPERTY_NAMES, name) {
        PropertyNameStatus::KnownUnsupported
    } else {
        PropertyNameStatus::Unknown
    }
}

pub(crate) fn classify_length_unit(unit: &str) -> LengthUnitStatus {
    CssLengthUnit::from_css_unit(unit)
        .map_or(LengthUnitStatus::Unknown, LengthUnitStatus::Supported)
}

pub(crate) fn parse_global_keyword(name: &str) -> Option<CssGlobalKeyword> {
    if name.eq_ignore_ascii_case("inherit") {
        Some(CssGlobalKeyword::Inherit)
    } else if name.eq_ignore_ascii_case("initial") {
        Some(CssGlobalKeyword::Initial)
    } else if name.eq_ignore_ascii_case("unset") {
        Some(CssGlobalKeyword::Unset)
    } else if name.eq_ignore_ascii_case("revert") {
        Some(CssGlobalKeyword::Revert)
    } else if name.eq_ignore_ascii_case("revert-layer") {
        Some(CssGlobalKeyword::RevertLayer)
    } else {
        None
    }
}

pub(crate) fn unsupported_keyword_reason(domain: &str, keyword: impl AsRef<str>) -> String {
    format!("unsupported {domain} keyword `{}`", keyword.as_ref())
}

fn contains_ascii_case(haystack: &[&str], needle: &str) -> bool {
    haystack
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(needle))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::accepted_declaration_cases;
    use std::collections::HashSet;

    #[test]
    fn known_unsupported_property_registry_is_empty() {
        assert!(KNOWN_UNSUPPORTED_PROPERTY_NAMES.is_empty());
    }

    #[test]
    fn coverage_supported_property_registry_has_accepted_cases() {
        assert!(KNOWN_UNSUPPORTED_PROPERTY_NAMES.is_empty());

        let mut covered = HashSet::new();
        let mut covered_properties = HashSet::new();
        for case in accepted_declaration_cases() {
            assert!(
                covered.insert(case.property_name),
                "duplicate accepted declaration case for `{}`",
                case.property_name,
            );
            covered_properties.insert(case.expected_property);
            case.assert_accepts();
        }

        let accepted_cases = accepted_declaration_cases();
        let mut supported_names = HashSet::new();
        let mut supported_property_set = HashSet::new();
        for supported_property in supported_properties() {
            assert!(
                supported_names.insert(supported_property.name.to_ascii_lowercase()),
                "duplicate supported property name `{}`",
                supported_property.name,
            );
            assert!(
                covered.contains(supported_property.name),
                "missing accepted declaration case for `{}`",
                supported_property.name,
            );

            let covered_case = accepted_cases
                .iter()
                .find(|case| case.property_name == supported_property.name)
                .expect("covered property has an accepted declaration case");
            assert_eq!(
                covered_case.expected_property, supported_property.property,
                "accepted declaration case for `{}` expects the wrong CssProperty",
                supported_property.name,
            );
            supported_property_set.insert(supported_property.property);
        }
        assert_eq!(
            covered.len(),
            supported_names.len(),
            "accepted declaration case names must exactly match supported properties",
        );
        assert_eq!(
            covered_properties, supported_property_set,
            "accepted declaration case properties must match supported parser mappings",
        );
    }
}

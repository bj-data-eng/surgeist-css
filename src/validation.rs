use crate::{CssGlobalKeyword, CssLengthUnit};

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

const SUPPORTED_PROPERTY_NAMES: &[&str] = &[
    "display",
    "box-sizing",
    "position",
    "direction",
    "overflow",
    "overflow-x",
    "overflow-y",
    "flex-direction",
    "flex-wrap",
    "float",
    "clear",
    "align-content",
    "justify-content",
    "align-items",
    "align-self",
    "justify-items",
    "justify-self",
    "place-content",
    "place-items",
    "place-self",
    "visibility",
    "content-visibility",
    "width",
    "height",
    "min-width",
    "min-height",
    "max-width",
    "max-height",
    "flex-basis",
    "gap",
    "row-gap",
    "column-gap",
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
    "flex-grow",
    "flex-shrink",
    "order",
    "flex",
    "justify-tracks",
    "align-tracks",
    "aspect-ratio",
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

const KNOWN_UNSUPPORTED_PROPERTY_NAMES: &[&str] = &["all"];

pub(crate) fn classify_property_name(name: &str) -> PropertyNameStatus {
    if contains_ascii_case(SUPPORTED_PROPERTY_NAMES, name) {
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

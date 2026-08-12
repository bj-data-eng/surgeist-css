// These manifests are intentionally hand-authored independently from both the
// conformance catalog and the crate-private property schema. Stable IDs are
// repeated explicitly so omission, extra, and duplicate mutations are visible.

#[derive(Clone, Copy, Debug)]
pub struct PropertyVector {
    pub id: &'static str,
    pub canonical_name: &'static str,
    pub authored_value: &'static str,
}

macro_rules! vector {
    ($id:literal, $canonical_name:literal, $authored_value:literal) => {
        PropertyVector {
            id: $id,
            canonical_name: $canonical_name,
            authored_value: $authored_value,
        }
    };
}

pub const PROPERTY_POSITIVE_VECTORS: &[PropertyVector] = &[
    vector!("baseline.property.all", "all", "inherit"),
    vector!("baseline.property.display", "display", "inherit"),
    vector!("baseline.property.box-sizing", "box-sizing", "inherit"),
    vector!("baseline.property.position", "position", "inherit"),
    vector!("baseline.property.direction", "direction", "inherit"),
    vector!("baseline.property.overflow", "overflow", "inherit"),
    vector!("baseline.property.overflow-x", "overflow-x", "inherit"),
    vector!("baseline.property.overflow-y", "overflow-y", "inherit"),
    vector!(
        "baseline.property.flex-direction",
        "flex-direction",
        "inherit"
    ),
    vector!("baseline.property.flex-wrap", "flex-wrap", "inherit"),
    vector!("baseline.property.float", "float", "inherit"),
    vector!("baseline.property.clear", "clear", "inherit"),
    vector!(
        "baseline.property.align-content",
        "align-content",
        "inherit"
    ),
    vector!(
        "baseline.property.justify-content",
        "justify-content",
        "inherit"
    ),
    vector!("baseline.property.align-items", "align-items", "inherit"),
    vector!("baseline.property.align-self", "align-self", "inherit"),
    vector!(
        "baseline.property.justify-items",
        "justify-items",
        "inherit"
    ),
    vector!("baseline.property.justify-self", "justify-self", "inherit"),
    vector!(
        "baseline.property.place-content",
        "place-content",
        "inherit"
    ),
    vector!("baseline.property.place-items", "place-items", "inherit"),
    vector!("baseline.property.place-self", "place-self", "inherit"),
    vector!("baseline.property.visibility", "visibility", "inherit"),
    vector!("baseline.property.content", "content", "inherit"),
    vector!(
        "baseline.property.content-visibility",
        "content-visibility",
        "inherit"
    ),
    vector!(
        "baseline.property.list-style-type",
        "list-style-type",
        "inherit"
    ),
    vector!(
        "baseline.property.list-style-position",
        "list-style-position",
        "inherit"
    ),
    vector!(
        "baseline.property.list-style-image",
        "list-style-image",
        "inherit"
    ),
    vector!("baseline.property.list-style", "list-style", "inherit"),
    vector!(
        "baseline.property.counter-reset",
        "counter-reset",
        "inherit"
    ),
    vector!(
        "baseline.property.counter-increment",
        "counter-increment",
        "inherit"
    ),
    vector!("baseline.property.counter-set", "counter-set", "inherit"),
    vector!("baseline.property.width", "width", "inherit"),
    vector!("baseline.property.height", "height", "inherit"),
    vector!("baseline.property.min-width", "min-width", "inherit"),
    vector!("baseline.property.min-height", "min-height", "inherit"),
    vector!("baseline.property.max-width", "max-width", "inherit"),
    vector!("baseline.property.max-height", "max-height", "inherit"),
    vector!("baseline.property.flex-basis", "flex-basis", "inherit"),
    vector!("baseline.property.gap", "gap", "inherit"),
    vector!("baseline.property.row-gap", "row-gap", "inherit"),
    vector!("baseline.property.column-gap", "column-gap", "inherit"),
    vector!(
        "baseline.property.grid-flow-tolerance",
        "grid-flow-tolerance",
        "inherit"
    ),
    vector!(
        "baseline.property.grid-template-rows",
        "grid-template-rows",
        "inherit"
    ),
    vector!(
        "baseline.property.grid-template-columns",
        "grid-template-columns",
        "inherit"
    ),
    vector!(
        "baseline.property.grid-template-areas",
        "grid-template-areas",
        "inherit"
    ),
    vector!(
        "baseline.property.grid-template",
        "grid-template",
        "inherit"
    ),
    vector!(
        "baseline.property.grid-auto-rows",
        "grid-auto-rows",
        "inherit"
    ),
    vector!(
        "baseline.property.grid-auto-columns",
        "grid-auto-columns",
        "inherit"
    ),
    vector!(
        "baseline.property.grid-auto-flow",
        "grid-auto-flow",
        "inherit"
    ),
    vector!(
        "baseline.property.grid-row-start",
        "grid-row-start",
        "inherit"
    ),
    vector!("baseline.property.grid-row-end", "grid-row-end", "inherit"),
    vector!(
        "baseline.property.grid-column-start",
        "grid-column-start",
        "inherit"
    ),
    vector!(
        "baseline.property.grid-column-end",
        "grid-column-end",
        "inherit"
    ),
    vector!("baseline.property.grid-row", "grid-row", "inherit"),
    vector!("baseline.property.grid-column", "grid-column", "inherit"),
    vector!("baseline.property.grid-area", "grid-area", "inherit"),
    vector!("baseline.property.grid", "grid", "inherit"),
    vector!("baseline.property.font-size", "font-size", "inherit"),
    vector!("baseline.property.line-height", "line-height", "inherit"),
    vector!("baseline.property.writing-mode", "writing-mode", "inherit"),
    vector!("baseline.property.text-align", "text-align", "inherit"),
    vector!(
        "baseline.property.text-align-last",
        "text-align-last",
        "inherit"
    ),
    vector!("baseline.property.text-indent", "text-indent", "inherit"),
    vector!(
        "baseline.property.vertical-align",
        "vertical-align",
        "inherit"
    ),
    vector!("baseline.property.font-family", "font-family", "inherit"),
    vector!("baseline.property.font", "font", "inherit"),
    vector!("baseline.property.font-weight", "font-weight", "inherit"),
    vector!("baseline.property.font-style", "font-style", "inherit"),
    vector!("baseline.property.font-stretch", "font-stretch", "inherit"),
    vector!("baseline.property.font-variant", "font-variant", "inherit"),
    vector!(
        "baseline.property.font-feature-settings",
        "font-feature-settings",
        "inherit"
    ),
    vector!(
        "baseline.property.letter-spacing",
        "letter-spacing",
        "inherit"
    ),
    vector!("baseline.property.text-wrap", "text-wrap", "inherit"),
    vector!("baseline.property.white-space", "white-space", "inherit"),
    vector!("baseline.property.word-break", "word-break", "inherit"),
    vector!(
        "baseline.property.overflow-wrap",
        "overflow-wrap",
        "inherit"
    ),
    vector!(
        "baseline.property.text-overflow",
        "text-overflow",
        "inherit"
    ),
    vector!(
        "baseline.property.text-decoration",
        "text-decoration",
        "inherit"
    ),
    vector!(
        "baseline.property.text-decoration-line",
        "text-decoration-line",
        "inherit"
    ),
    vector!(
        "baseline.property.text-decoration-color",
        "text-decoration-color",
        "inherit"
    ),
    vector!(
        "baseline.property.text-decoration-style",
        "text-decoration-style",
        "inherit"
    ),
    vector!(
        "baseline.property.text-decoration-thickness",
        "text-decoration-thickness",
        "inherit"
    ),
    vector!(
        "baseline.property.text-transform",
        "text-transform",
        "inherit"
    ),
    vector!("baseline.property.inset", "inset", "inherit"),
    vector!("baseline.property.top", "top", "inherit"),
    vector!("baseline.property.right", "right", "inherit"),
    vector!("baseline.property.bottom", "bottom", "inherit"),
    vector!("baseline.property.left", "left", "inherit"),
    vector!("baseline.property.z-index", "z-index", "inherit"),
    vector!(
        "baseline.property.box-decoration-break",
        "box-decoration-break",
        "inherit"
    ),
    vector!("baseline.property.margin", "margin", "inherit"),
    vector!("baseline.property.margin-top", "margin-top", "inherit"),
    vector!("baseline.property.margin-right", "margin-right", "inherit"),
    vector!(
        "baseline.property.margin-bottom",
        "margin-bottom",
        "inherit"
    ),
    vector!("baseline.property.margin-left", "margin-left", "inherit"),
    vector!("baseline.property.padding", "padding", "inherit"),
    vector!("baseline.property.padding-top", "padding-top", "inherit"),
    vector!(
        "baseline.property.padding-right",
        "padding-right",
        "inherit"
    ),
    vector!(
        "baseline.property.padding-bottom",
        "padding-bottom",
        "inherit"
    ),
    vector!("baseline.property.padding-left", "padding-left", "inherit"),
    vector!("baseline.property.border", "border", "inherit"),
    vector!("baseline.property.border-top", "border-top", "inherit"),
    vector!("baseline.property.border-right", "border-right", "inherit"),
    vector!(
        "baseline.property.border-bottom",
        "border-bottom",
        "inherit"
    ),
    vector!("baseline.property.border-left", "border-left", "inherit"),
    vector!("baseline.property.border-width", "border-width", "inherit"),
    vector!(
        "baseline.property.border-top-width",
        "border-top-width",
        "inherit"
    ),
    vector!(
        "baseline.property.border-right-width",
        "border-right-width",
        "inherit"
    ),
    vector!(
        "baseline.property.border-bottom-width",
        "border-bottom-width",
        "inherit"
    ),
    vector!(
        "baseline.property.border-left-width",
        "border-left-width",
        "inherit"
    ),
    vector!("baseline.property.color", "color", "inherit"),
    vector!("baseline.property.background", "background", "inherit"),
    vector!(
        "baseline.property.background-color",
        "background-color",
        "inherit"
    ),
    vector!("baseline.property.border-color", "border-color", "inherit"),
    vector!(
        "baseline.property.border-top-color",
        "border-top-color",
        "inherit"
    ),
    vector!(
        "baseline.property.border-right-color",
        "border-right-color",
        "inherit"
    ),
    vector!(
        "baseline.property.border-bottom-color",
        "border-bottom-color",
        "inherit"
    ),
    vector!(
        "baseline.property.border-left-color",
        "border-left-color",
        "inherit"
    ),
    vector!(
        "baseline.property.background-image",
        "background-image",
        "inherit"
    ),
    vector!(
        "baseline.property.background-position",
        "background-position",
        "inherit"
    ),
    vector!(
        "baseline.property.background-size",
        "background-size",
        "inherit"
    ),
    vector!(
        "baseline.property.background-repeat",
        "background-repeat",
        "inherit"
    ),
    vector!(
        "baseline.property.background-origin",
        "background-origin",
        "inherit"
    ),
    vector!(
        "baseline.property.background-clip",
        "background-clip",
        "inherit"
    ),
    vector!(
        "baseline.property.background-attachment",
        "background-attachment",
        "inherit"
    ),
    vector!("baseline.property.border-style", "border-style", "inherit"),
    vector!(
        "baseline.property.border-top-style",
        "border-top-style",
        "inherit"
    ),
    vector!(
        "baseline.property.border-right-style",
        "border-right-style",
        "inherit"
    ),
    vector!(
        "baseline.property.border-bottom-style",
        "border-bottom-style",
        "inherit"
    ),
    vector!(
        "baseline.property.border-left-style",
        "border-left-style",
        "inherit"
    ),
    vector!(
        "baseline.property.border-radius",
        "border-radius",
        "inherit"
    ),
    vector!(
        "baseline.property.border-top-left-radius",
        "border-top-left-radius",
        "inherit"
    ),
    vector!(
        "baseline.property.border-top-right-radius",
        "border-top-right-radius",
        "inherit"
    ),
    vector!(
        "baseline.property.border-bottom-right-radius",
        "border-bottom-right-radius",
        "inherit"
    ),
    vector!(
        "baseline.property.border-bottom-left-radius",
        "border-bottom-left-radius",
        "inherit"
    ),
    vector!("baseline.property.box-shadow", "box-shadow", "inherit"),
    vector!("baseline.property.opacity", "opacity", "inherit"),
    vector!("baseline.property.flex-grow", "flex-grow", "inherit"),
    vector!("baseline.property.flex-shrink", "flex-shrink", "inherit"),
    vector!("baseline.property.order", "order", "inherit"),
    vector!("baseline.property.flex", "flex", "inherit"),
    vector!(
        "baseline.property.justify-tracks",
        "justify-tracks",
        "inherit"
    ),
    vector!("baseline.property.align-tracks", "align-tracks", "inherit"),
    vector!("baseline.property.aspect-ratio", "aspect-ratio", "inherit"),
    vector!(
        "baseline.property.scrollbar-width",
        "scrollbar-width",
        "inherit"
    ),
    vector!("baseline.property.cursor", "cursor", "inherit"),
    vector!(
        "baseline.property.pointer-events",
        "pointer-events",
        "inherit"
    ),
    vector!("baseline.property.user-select", "user-select", "inherit"),
    vector!("baseline.property.outline", "outline", "inherit"),
    vector!(
        "baseline.property.outline-color",
        "outline-color",
        "inherit"
    ),
    vector!(
        "baseline.property.outline-style",
        "outline-style",
        "inherit"
    ),
    vector!(
        "baseline.property.outline-width",
        "outline-width",
        "inherit"
    ),
    vector!("baseline.property.transform", "transform", "inherit"),
    vector!(
        "baseline.property.transform-origin",
        "transform-origin",
        "inherit"
    ),
    vector!("baseline.property.translate", "translate", "inherit"),
    vector!("baseline.property.rotate", "rotate", "inherit"),
    vector!("baseline.property.scale", "scale", "inherit"),
    vector!("baseline.property.filter", "filter", "inherit"),
    vector!(
        "baseline.property.backdrop-filter",
        "backdrop-filter",
        "inherit"
    ),
    vector!("baseline.property.clip-path", "clip-path", "inherit"),
    vector!("baseline.property.mask", "mask", "inherit"),
    vector!("baseline.property.mask-image", "mask-image", "inherit"),
    vector!("baseline.property.mask-size", "mask-size", "inherit"),
    vector!(
        "baseline.property.mask-position",
        "mask-position",
        "inherit"
    ),
    vector!("baseline.property.mask-repeat", "mask-repeat", "inherit"),
    vector!(
        "baseline.property.transition-property",
        "transition-property",
        "inherit"
    ),
    vector!(
        "baseline.property.transition-duration",
        "transition-duration",
        "inherit"
    ),
    vector!(
        "baseline.property.transition-delay",
        "transition-delay",
        "inherit"
    ),
    vector!(
        "baseline.property.transition-timing-function",
        "transition-timing-function",
        "inherit"
    ),
    vector!("baseline.property.transition", "transition", "inherit"),
    vector!(
        "baseline.property.animation-name",
        "animation-name",
        "inherit"
    ),
    vector!(
        "baseline.property.animation-duration",
        "animation-duration",
        "inherit"
    ),
    vector!(
        "baseline.property.animation-delay",
        "animation-delay",
        "inherit"
    ),
    vector!(
        "baseline.property.animation-timing-function",
        "animation-timing-function",
        "inherit"
    ),
    vector!(
        "baseline.property.animation-iteration-count",
        "animation-iteration-count",
        "inherit"
    ),
    vector!(
        "baseline.property.animation-direction",
        "animation-direction",
        "inherit"
    ),
    vector!(
        "baseline.property.animation-fill-mode",
        "animation-fill-mode",
        "inherit"
    ),
    vector!(
        "baseline.property.animation-play-state",
        "animation-play-state",
        "inherit"
    ),
    vector!("baseline.property.animation", "animation", "inherit"),
];

pub const PROPERTY_NEGATIVE_VECTORS: &[PropertyVector] = &[
    vector!("baseline.property.all", "all", "initial 1px"),
    vector!("baseline.property.display", "display", "initial 1px"),
    vector!("baseline.property.box-sizing", "box-sizing", "initial 1px"),
    vector!("baseline.property.position", "position", "initial 1px"),
    vector!("baseline.property.direction", "direction", "initial 1px"),
    vector!("baseline.property.overflow", "overflow", "initial 1px"),
    vector!("baseline.property.overflow-x", "overflow-x", "initial 1px"),
    vector!("baseline.property.overflow-y", "overflow-y", "initial 1px"),
    vector!(
        "baseline.property.flex-direction",
        "flex-direction",
        "initial 1px"
    ),
    vector!("baseline.property.flex-wrap", "flex-wrap", "initial 1px"),
    vector!("baseline.property.float", "float", "initial 1px"),
    vector!("baseline.property.clear", "clear", "initial 1px"),
    vector!(
        "baseline.property.align-content",
        "align-content",
        "initial 1px"
    ),
    vector!(
        "baseline.property.justify-content",
        "justify-content",
        "initial 1px"
    ),
    vector!(
        "baseline.property.align-items",
        "align-items",
        "initial 1px"
    ),
    vector!("baseline.property.align-self", "align-self", "initial 1px"),
    vector!(
        "baseline.property.justify-items",
        "justify-items",
        "initial 1px"
    ),
    vector!(
        "baseline.property.justify-self",
        "justify-self",
        "initial 1px"
    ),
    vector!(
        "baseline.property.place-content",
        "place-content",
        "initial 1px"
    ),
    vector!(
        "baseline.property.place-items",
        "place-items",
        "initial 1px"
    ),
    vector!("baseline.property.place-self", "place-self", "initial 1px"),
    vector!("baseline.property.visibility", "visibility", "initial 1px"),
    vector!("baseline.property.content", "content", "initial 1px"),
    vector!(
        "baseline.property.content-visibility",
        "content-visibility",
        "initial 1px"
    ),
    vector!(
        "baseline.property.list-style-type",
        "list-style-type",
        "initial 1px"
    ),
    vector!(
        "baseline.property.list-style-position",
        "list-style-position",
        "initial 1px"
    ),
    vector!(
        "baseline.property.list-style-image",
        "list-style-image",
        "initial 1px"
    ),
    vector!("baseline.property.list-style", "list-style", "initial 1px"),
    vector!(
        "baseline.property.counter-reset",
        "counter-reset",
        "initial 1px"
    ),
    vector!(
        "baseline.property.counter-increment",
        "counter-increment",
        "initial 1px"
    ),
    vector!(
        "baseline.property.counter-set",
        "counter-set",
        "initial 1px"
    ),
    vector!("baseline.property.width", "width", "initial 1px"),
    vector!("baseline.property.height", "height", "initial 1px"),
    vector!("baseline.property.min-width", "min-width", "initial 1px"),
    vector!("baseline.property.min-height", "min-height", "initial 1px"),
    vector!("baseline.property.max-width", "max-width", "initial 1px"),
    vector!("baseline.property.max-height", "max-height", "initial 1px"),
    vector!("baseline.property.flex-basis", "flex-basis", "initial 1px"),
    vector!("baseline.property.gap", "gap", "initial 1px"),
    vector!("baseline.property.row-gap", "row-gap", "initial 1px"),
    vector!("baseline.property.column-gap", "column-gap", "initial 1px"),
    vector!(
        "baseline.property.grid-flow-tolerance",
        "grid-flow-tolerance",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-template-rows",
        "grid-template-rows",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-template-columns",
        "grid-template-columns",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-template-areas",
        "grid-template-areas",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-template",
        "grid-template",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-auto-rows",
        "grid-auto-rows",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-auto-columns",
        "grid-auto-columns",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-auto-flow",
        "grid-auto-flow",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-row-start",
        "grid-row-start",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-row-end",
        "grid-row-end",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-column-start",
        "grid-column-start",
        "initial 1px"
    ),
    vector!(
        "baseline.property.grid-column-end",
        "grid-column-end",
        "initial 1px"
    ),
    vector!("baseline.property.grid-row", "grid-row", "initial 1px"),
    vector!(
        "baseline.property.grid-column",
        "grid-column",
        "initial 1px"
    ),
    vector!("baseline.property.grid-area", "grid-area", "initial 1px"),
    vector!("baseline.property.grid", "grid", "initial 1px"),
    vector!("baseline.property.font-size", "font-size", "initial 1px"),
    vector!(
        "baseline.property.line-height",
        "line-height",
        "initial 1px"
    ),
    vector!(
        "baseline.property.writing-mode",
        "writing-mode",
        "initial 1px"
    ),
    vector!("baseline.property.text-align", "text-align", "initial 1px"),
    vector!(
        "baseline.property.text-align-last",
        "text-align-last",
        "initial 1px"
    ),
    vector!(
        "baseline.property.text-indent",
        "text-indent",
        "initial 1px"
    ),
    vector!(
        "baseline.property.vertical-align",
        "vertical-align",
        "initial 1px"
    ),
    vector!(
        "baseline.property.font-family",
        "font-family",
        "initial 1px"
    ),
    vector!("baseline.property.font", "font", "initial 1px"),
    vector!(
        "baseline.property.font-weight",
        "font-weight",
        "initial 1px"
    ),
    vector!("baseline.property.font-style", "font-style", "initial 1px"),
    vector!(
        "baseline.property.font-stretch",
        "font-stretch",
        "initial 1px"
    ),
    vector!(
        "baseline.property.font-variant",
        "font-variant",
        "initial 1px"
    ),
    vector!(
        "baseline.property.font-feature-settings",
        "font-feature-settings",
        "initial 1px"
    ),
    vector!(
        "baseline.property.letter-spacing",
        "letter-spacing",
        "initial 1px"
    ),
    vector!("baseline.property.text-wrap", "text-wrap", "initial 1px"),
    vector!(
        "baseline.property.white-space",
        "white-space",
        "initial 1px"
    ),
    vector!("baseline.property.word-break", "word-break", "initial 1px"),
    vector!(
        "baseline.property.overflow-wrap",
        "overflow-wrap",
        "initial 1px"
    ),
    vector!(
        "baseline.property.text-overflow",
        "text-overflow",
        "initial 1px"
    ),
    vector!(
        "baseline.property.text-decoration",
        "text-decoration",
        "initial 1px"
    ),
    vector!(
        "baseline.property.text-decoration-line",
        "text-decoration-line",
        "initial 1px"
    ),
    vector!(
        "baseline.property.text-decoration-color",
        "text-decoration-color",
        "initial 1px"
    ),
    vector!(
        "baseline.property.text-decoration-style",
        "text-decoration-style",
        "initial 1px"
    ),
    vector!(
        "baseline.property.text-decoration-thickness",
        "text-decoration-thickness",
        "initial 1px"
    ),
    vector!(
        "baseline.property.text-transform",
        "text-transform",
        "initial 1px"
    ),
    vector!("baseline.property.inset", "inset", "initial 1px"),
    vector!("baseline.property.top", "top", "initial 1px"),
    vector!("baseline.property.right", "right", "initial 1px"),
    vector!("baseline.property.bottom", "bottom", "initial 1px"),
    vector!("baseline.property.left", "left", "initial 1px"),
    vector!("baseline.property.z-index", "z-index", "initial 1px"),
    vector!(
        "baseline.property.box-decoration-break",
        "box-decoration-break",
        "initial 1px"
    ),
    vector!("baseline.property.margin", "margin", "initial 1px"),
    vector!("baseline.property.margin-top", "margin-top", "initial 1px"),
    vector!(
        "baseline.property.margin-right",
        "margin-right",
        "initial 1px"
    ),
    vector!(
        "baseline.property.margin-bottom",
        "margin-bottom",
        "initial 1px"
    ),
    vector!(
        "baseline.property.margin-left",
        "margin-left",
        "initial 1px"
    ),
    vector!("baseline.property.padding", "padding", "initial 1px"),
    vector!(
        "baseline.property.padding-top",
        "padding-top",
        "initial 1px"
    ),
    vector!(
        "baseline.property.padding-right",
        "padding-right",
        "initial 1px"
    ),
    vector!(
        "baseline.property.padding-bottom",
        "padding-bottom",
        "initial 1px"
    ),
    vector!(
        "baseline.property.padding-left",
        "padding-left",
        "initial 1px"
    ),
    vector!("baseline.property.border", "border", "initial 1px"),
    vector!("baseline.property.border-top", "border-top", "initial 1px"),
    vector!(
        "baseline.property.border-right",
        "border-right",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-bottom",
        "border-bottom",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-left",
        "border-left",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-width",
        "border-width",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-top-width",
        "border-top-width",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-right-width",
        "border-right-width",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-bottom-width",
        "border-bottom-width",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-left-width",
        "border-left-width",
        "initial 1px"
    ),
    vector!("baseline.property.color", "color", "initial 1px"),
    vector!("baseline.property.background", "background", "initial 1px"),
    vector!(
        "baseline.property.background-color",
        "background-color",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-color",
        "border-color",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-top-color",
        "border-top-color",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-right-color",
        "border-right-color",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-bottom-color",
        "border-bottom-color",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-left-color",
        "border-left-color",
        "initial 1px"
    ),
    vector!(
        "baseline.property.background-image",
        "background-image",
        "initial 1px"
    ),
    vector!(
        "baseline.property.background-position",
        "background-position",
        "initial 1px"
    ),
    vector!(
        "baseline.property.background-size",
        "background-size",
        "initial 1px"
    ),
    vector!(
        "baseline.property.background-repeat",
        "background-repeat",
        "initial 1px"
    ),
    vector!(
        "baseline.property.background-origin",
        "background-origin",
        "initial 1px"
    ),
    vector!(
        "baseline.property.background-clip",
        "background-clip",
        "initial 1px"
    ),
    vector!(
        "baseline.property.background-attachment",
        "background-attachment",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-style",
        "border-style",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-top-style",
        "border-top-style",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-right-style",
        "border-right-style",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-bottom-style",
        "border-bottom-style",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-left-style",
        "border-left-style",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-radius",
        "border-radius",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-top-left-radius",
        "border-top-left-radius",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-top-right-radius",
        "border-top-right-radius",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-bottom-right-radius",
        "border-bottom-right-radius",
        "initial 1px"
    ),
    vector!(
        "baseline.property.border-bottom-left-radius",
        "border-bottom-left-radius",
        "initial 1px"
    ),
    vector!("baseline.property.box-shadow", "box-shadow", "initial 1px"),
    vector!("baseline.property.opacity", "opacity", "initial 1px"),
    vector!("baseline.property.flex-grow", "flex-grow", "initial 1px"),
    vector!(
        "baseline.property.flex-shrink",
        "flex-shrink",
        "initial 1px"
    ),
    vector!("baseline.property.order", "order", "initial 1px"),
    vector!("baseline.property.flex", "flex", "initial 1px"),
    vector!(
        "baseline.property.justify-tracks",
        "justify-tracks",
        "initial 1px"
    ),
    vector!(
        "baseline.property.align-tracks",
        "align-tracks",
        "initial 1px"
    ),
    vector!(
        "baseline.property.aspect-ratio",
        "aspect-ratio",
        "initial 1px"
    ),
    vector!(
        "baseline.property.scrollbar-width",
        "scrollbar-width",
        "initial 1px"
    ),
    vector!("baseline.property.cursor", "cursor", "initial 1px"),
    vector!(
        "baseline.property.pointer-events",
        "pointer-events",
        "initial 1px"
    ),
    vector!(
        "baseline.property.user-select",
        "user-select",
        "initial 1px"
    ),
    vector!("baseline.property.outline", "outline", "initial 1px"),
    vector!(
        "baseline.property.outline-color",
        "outline-color",
        "initial 1px"
    ),
    vector!(
        "baseline.property.outline-style",
        "outline-style",
        "initial 1px"
    ),
    vector!(
        "baseline.property.outline-width",
        "outline-width",
        "initial 1px"
    ),
    vector!("baseline.property.transform", "transform", "initial 1px"),
    vector!(
        "baseline.property.transform-origin",
        "transform-origin",
        "initial 1px"
    ),
    vector!("baseline.property.translate", "translate", "initial 1px"),
    vector!("baseline.property.rotate", "rotate", "initial 1px"),
    vector!("baseline.property.scale", "scale", "initial 1px"),
    vector!("baseline.property.filter", "filter", "initial 1px"),
    vector!(
        "baseline.property.backdrop-filter",
        "backdrop-filter",
        "initial 1px"
    ),
    vector!("baseline.property.clip-path", "clip-path", "initial 1px"),
    vector!("baseline.property.mask", "mask", "initial 1px"),
    vector!("baseline.property.mask-image", "mask-image", "initial 1px"),
    vector!("baseline.property.mask-size", "mask-size", "initial 1px"),
    vector!(
        "baseline.property.mask-position",
        "mask-position",
        "initial 1px"
    ),
    vector!(
        "baseline.property.mask-repeat",
        "mask-repeat",
        "initial 1px"
    ),
    vector!(
        "baseline.property.transition-property",
        "transition-property",
        "initial 1px"
    ),
    vector!(
        "baseline.property.transition-duration",
        "transition-duration",
        "initial 1px"
    ),
    vector!(
        "baseline.property.transition-delay",
        "transition-delay",
        "initial 1px"
    ),
    vector!(
        "baseline.property.transition-timing-function",
        "transition-timing-function",
        "initial 1px"
    ),
    vector!("baseline.property.transition", "transition", "initial 1px"),
    vector!(
        "baseline.property.animation-name",
        "animation-name",
        "initial 1px"
    ),
    vector!(
        "baseline.property.animation-duration",
        "animation-duration",
        "initial 1px"
    ),
    vector!(
        "baseline.property.animation-delay",
        "animation-delay",
        "initial 1px"
    ),
    vector!(
        "baseline.property.animation-timing-function",
        "animation-timing-function",
        "initial 1px"
    ),
    vector!(
        "baseline.property.animation-iteration-count",
        "animation-iteration-count",
        "initial 1px"
    ),
    vector!(
        "baseline.property.animation-direction",
        "animation-direction",
        "initial 1px"
    ),
    vector!(
        "baseline.property.animation-fill-mode",
        "animation-fill-mode",
        "initial 1px"
    ),
    vector!(
        "baseline.property.animation-play-state",
        "animation-play-state",
        "initial 1px"
    ),
    vector!("baseline.property.animation", "animation", "initial 1px"),
];

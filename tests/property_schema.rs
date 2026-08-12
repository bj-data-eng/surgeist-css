use std::collections::HashSet;

use surgeist_css::{CssKnownProperty, CssRule, parse_sheet};

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
    for &name in FROZEN_PROPERTIES {
        let source = format!(".test {{ {}: inherit; }}", name.to_ascii_uppercase());
        let sheet = parse_sheet(&source)
            .unwrap_or_else(|error| panic!("`{name}` should parse through the schema: {error}"));
        let [CssRule::Style(rule)] = sheet.rules() else {
            panic!("`{name}` should produce one style rule");
        };
        let [declaration] = rule.declarations() else {
            panic!("`{name}` should produce one declaration");
        };
        assert_eq!(
            declaration.property().known(),
            CssKnownProperty::from_name(name),
            "`{name}` parser identity diverged from schema lookup",
        );
    }
}

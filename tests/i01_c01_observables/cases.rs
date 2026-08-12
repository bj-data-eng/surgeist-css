#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryPoint {
    Sheet,
    Style,
}

impl EntryPoint {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sheet => "sheet",
            Self::Style => "style",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeatureMode {
    Both,
    AppStrict,
}

impl FeatureMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Both => "both",
            Self::AppStrict => "app-strict",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Case {
    pub id: String,
    pub owner: String,
    pub entry: EntryPoint,
    pub feature: FeatureMode,
    pub input: String,
}

impl Case {
    pub fn new(
        id: impl Into<String>,
        owner: impl Into<String>,
        entry: EntryPoint,
        feature: FeatureMode,
        input: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            owner: owner.into(),
            entry,
            feature,
            input: input.into(),
        }
    }
}

fn case(
    id: impl Into<String>,
    owner: impl Into<String>,
    entry: EntryPoint,
    input: impl Into<String>,
) -> Case {
    Case::new(id, owner, entry, FeatureMode::Both, input)
}

pub fn non_property_cases() -> Vec<Case> {
    use EntryPoint::{Sheet, Style};
    let mut cases = Vec::new();
    macro_rules! pair {
        ($id:literal, $entry:ident, $positive:literal, $boundary:literal) => {{
            cases.push(case(
                concat!("catalog.non-property.", $id, ".positive"),
                concat!("conformance_catalog::", $id, "/positive"),
                $entry,
                $positive,
            ));
            cases.push(case(
                concat!("catalog.non-property.", $id, ".boundary"),
                concat!("conformance_catalog::", $id, "/boundary"),
                $entry,
                $boundary,
            ));
        }};
    }
    macro_rules! positive {
        ($id:literal, $entry:ident, $input:literal) => {
            cases.push(case(
                concat!("catalog.non-property.", $id, ".positive"),
                concat!("conformance_catalog::", $id, "/positive"),
                $entry,
                $input,
            ));
        };
    }
    macro_rules! boundary {
        ($id:literal, $entry:ident, $input:literal) => {
            cases.push(case(
                concat!("catalog.non-property.", $id, ".boundary"),
                concat!("conformance_catalog::", $id, "/boundary"),
                $entry,
                $input,
            ));
        };
    }
    pair!(
        "baseline.rule.import",
        Sheet,
        "@import \"theme.css\";",
        "@import url(theme.css) supports(display: grid);"
    );
    pair!(
        "baseline.rule.layer-statement",
        Sheet,
        "@layer reset, theme;",
        "@layer initial;"
    );
    pair!(
        "baseline.rule.layer-block",
        Sheet,
        "@layer theme { .x { color: red; } }",
        "@layer first, second { .x { color: red; } }"
    );
    pair!(
        "baseline.rule.font-face",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); }",
        "@font-face named { font-family: Inter; src: url(inter.woff2); }"
    );
    pair!(
        "baseline.rule.keyframes",
        Sheet,
        "@keyframes fade { from { opacity: 0; } to { opacity: 1; } }",
        "@keyframes none { from { opacity: 0; } }"
    );
    pair!(
        "baseline.rule.style",
        Sheet,
        ".x { color: red; }",
        "??? { color: red; }"
    );
    pair!(
        "baseline.rule.media",
        Sheet,
        "@media screen { .x { color: red; } }",
        "@media (width: calc(1px)) { .x { color: red; } }"
    );
    pair!(
        "baseline.rule.container",
        Sheet,
        "@container (width > 1px) { .x { color: red; } }",
        "@container scroll-state(stuck: top) { .x { color: red; } }"
    );
    pair!(
        "baseline.rule.scope",
        Sheet,
        "@scope (.card) { .title { color: red; } }",
        "@scope .card { .title { color: red; } }"
    );
    positive!(
        "foundation.encoding.charset",
        Sheet,
        "@charset \"UTF-8\"; .x { color: red; }"
    );
    positive!(
        "foundation.declaration-list.style-attribute",
        Style,
        "color: red; width: 1px"
    );
    positive!(
        "foundation.declaration.importance",
        Style,
        "color: red !important"
    );
    pair!(
        "baseline.declaration.custom-property",
        Style,
        "--theme: dark",
        "--x: inherit 1px"
    );
    pair!(
        "baseline.value.substitution-dependent",
        Style,
        "width: var(--width, 1px)",
        "width: var(color)"
    );
    boundary!(
        "later.rule.namespace",
        Sheet,
        "@namespace svg url(https://example.test/svg);"
    );
    boundary!(
        "later.rule.supports",
        Sheet,
        "@supports (display: grid) { .x { color: red; } }"
    );
    boundary!(
        "later.rule.counter-style",
        Sheet,
        "@counter-style thumbs { system: cyclic; symbols: 👍; suffix: \" \"; }"
    );
    boundary!("later.rule.page", Sheet, "@page { margin: 1cm; }");
    boundary!(
        "later.rule.font-feature-values",
        Sheet,
        "@font-feature-values Font One { @styleset { nice: 1; } }"
    );
    pair!(
        "baseline.descriptor.font-family",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); }",
        "@font-face { font-family: serif, sans-serif; src: url(inter.woff2); }"
    );
    pair!(
        "baseline.descriptor.src",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2) format(woff2); }",
        "@font-face { font-family: Inter; src: url(inter.woff2) format(woff3); }"
    );
    pair!(
        "baseline.descriptor.font-weight",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); font-weight: 400 700; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); font-weight: bolder; }"
    );
    pair!(
        "baseline.descriptor.font-style",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); font-style: italic; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); font-style: made-up; }"
    );
    pair!(
        "baseline.descriptor.font-stretch",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); font-stretch: 75% 125%; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); font-stretch: wide; }"
    );
    pair!(
        "baseline.descriptor.font-display",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); font-display: swap; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); font-display: made-up; }"
    );
    pair!(
        "baseline.descriptor.unicode-range",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); unicode-range: U+0000-00FF; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); unicode-range: U+110000-110001; }"
    );
    pair!(
        "baseline.selector.complex",
        Sheet,
        "article#main.card[data-ready][lang|=\"en\"] > span + a ~ b { color: red; }",
        "svg|a { color: red; }"
    );
    pair!(
        "baseline.selector.pseudo-class",
        Sheet,
        ".button:hover { color: red; }",
        ".link:visited { color: red; }"
    );
    pair!(
        "baseline.selector.functional",
        Sheet,
        ".item:nth-child(2n+1) { color: red; }",
        ".item:lang(en) { color: red; }"
    );
    pair!(
        "baseline.selector.extension-state",
        Sheet,
        ".button:focus-visible { color: red; }",
        ".target:target { color: red; }"
    );
    pair!(
        "baseline.selector.extension-functional",
        Sheet,
        ".item:is(.primary, .secondary) { color: red; }",
        ".item:has(:has(.nested)) { color: red; }"
    );
    pair!(
        "baseline.selector.attribute-case",
        Sheet,
        "[data-kind=\"primary\" i] { color: red; }",
        "[data-kind=\"primary\" q] { color: red; }"
    );
    pair!(
        "baseline.selector.pseudo-element",
        Sheet,
        ".item::before { content: \"x\"; }",
        ".item::first-line { color: red; }"
    );
    pair!(
        "baseline.selector.nesting",
        Sheet,
        ".card { & > .title { color: red; } }",
        ".card { & || .title { color: red; } }"
    );
    pair!(
        "baseline.media.query-list",
        Sheet,
        "@media screen and (min-width: 1px), print { .x { color: red; } }",
        "@media screen, ??? { .x { color: red; } }"
    );
    pair!(
        "baseline.media.type",
        Sheet,
        "@media print { .x { color: red; } }",
        "@media speech { .x { color: red; } }"
    );
    pair!(
        "baseline.media.range-feature",
        Sheet,
        "@media (width >= 1px) { .x { color: red; } }",
        "@media (device-width: 1px) { .x { color: red; } }"
    );
    pair!(
        "baseline.media.discrete-feature",
        Sheet,
        "@media (orientation: landscape) { .x { color: red; } }",
        "@media (scripting: enabled) { .x { color: red; } }"
    );
    pair!(
        "baseline.container.condition",
        Sheet,
        "@container (width > 1px) and style(--theme) { .x { color: red; } }",
        "@container style(color: red) { .x { color: red; } }"
    );
    pair!(
        "baseline.container.size-feature",
        Sheet,
        "@container (inline-size > 1px) { .x { color: red; } }",
        "@container (unknown-size > 1px) { .x { color: red; } }"
    );
    cases
}

/// Literal I01 owner/case identities, independent of the executable vector imports.
///
/// The fixture reader checks this complete mapping, the literal total, and exact per-owner
/// cardinalities. Removing or renaming a loop element therefore changes executable inputs but
/// cannot silently redefine the stable closure.
pub const STABLE_CASE_OWNERS: &str = r#"
catalog.non-property.baseline.container.condition.boundary	conformance_catalog::baseline.container.condition/boundary
catalog.non-property.baseline.container.condition.positive	conformance_catalog::baseline.container.condition/positive
catalog.non-property.baseline.container.size-feature.boundary	conformance_catalog::baseline.container.size-feature/boundary
catalog.non-property.baseline.container.size-feature.positive	conformance_catalog::baseline.container.size-feature/positive
catalog.non-property.baseline.declaration.custom-property.boundary	conformance_catalog::baseline.declaration.custom-property/boundary
catalog.non-property.baseline.declaration.custom-property.positive	conformance_catalog::baseline.declaration.custom-property/positive
catalog.non-property.baseline.descriptor.font-display.boundary	conformance_catalog::baseline.descriptor.font-display/boundary
catalog.non-property.baseline.descriptor.font-display.positive	conformance_catalog::baseline.descriptor.font-display/positive
catalog.non-property.baseline.descriptor.font-family.boundary	conformance_catalog::baseline.descriptor.font-family/boundary
catalog.non-property.baseline.descriptor.font-family.positive	conformance_catalog::baseline.descriptor.font-family/positive
catalog.non-property.baseline.descriptor.font-stretch.boundary	conformance_catalog::baseline.descriptor.font-stretch/boundary
catalog.non-property.baseline.descriptor.font-stretch.positive	conformance_catalog::baseline.descriptor.font-stretch/positive
catalog.non-property.baseline.descriptor.font-style.boundary	conformance_catalog::baseline.descriptor.font-style/boundary
catalog.non-property.baseline.descriptor.font-style.positive	conformance_catalog::baseline.descriptor.font-style/positive
catalog.non-property.baseline.descriptor.font-weight.boundary	conformance_catalog::baseline.descriptor.font-weight/boundary
catalog.non-property.baseline.descriptor.font-weight.positive	conformance_catalog::baseline.descriptor.font-weight/positive
catalog.non-property.baseline.descriptor.src.boundary	conformance_catalog::baseline.descriptor.src/boundary
catalog.non-property.baseline.descriptor.src.positive	conformance_catalog::baseline.descriptor.src/positive
catalog.non-property.baseline.descriptor.unicode-range.boundary	conformance_catalog::baseline.descriptor.unicode-range/boundary
catalog.non-property.baseline.descriptor.unicode-range.positive	conformance_catalog::baseline.descriptor.unicode-range/positive
catalog.non-property.baseline.media.discrete-feature.boundary	conformance_catalog::baseline.media.discrete-feature/boundary
catalog.non-property.baseline.media.discrete-feature.positive	conformance_catalog::baseline.media.discrete-feature/positive
catalog.non-property.baseline.media.query-list.boundary	conformance_catalog::baseline.media.query-list/boundary
catalog.non-property.baseline.media.query-list.positive	conformance_catalog::baseline.media.query-list/positive
catalog.non-property.baseline.media.range-feature.boundary	conformance_catalog::baseline.media.range-feature/boundary
catalog.non-property.baseline.media.range-feature.positive	conformance_catalog::baseline.media.range-feature/positive
catalog.non-property.baseline.media.type.boundary	conformance_catalog::baseline.media.type/boundary
catalog.non-property.baseline.media.type.positive	conformance_catalog::baseline.media.type/positive
catalog.non-property.baseline.rule.container.boundary	conformance_catalog::baseline.rule.container/boundary
catalog.non-property.baseline.rule.container.positive	conformance_catalog::baseline.rule.container/positive
catalog.non-property.baseline.rule.font-face.boundary	conformance_catalog::baseline.rule.font-face/boundary
catalog.non-property.baseline.rule.font-face.positive	conformance_catalog::baseline.rule.font-face/positive
catalog.non-property.baseline.rule.import.boundary	conformance_catalog::baseline.rule.import/boundary
catalog.non-property.baseline.rule.import.positive	conformance_catalog::baseline.rule.import/positive
catalog.non-property.baseline.rule.keyframes.boundary	conformance_catalog::baseline.rule.keyframes/boundary
catalog.non-property.baseline.rule.keyframes.positive	conformance_catalog::baseline.rule.keyframes/positive
catalog.non-property.baseline.rule.layer-block.boundary	conformance_catalog::baseline.rule.layer-block/boundary
catalog.non-property.baseline.rule.layer-block.positive	conformance_catalog::baseline.rule.layer-block/positive
catalog.non-property.baseline.rule.layer-statement.boundary	conformance_catalog::baseline.rule.layer-statement/boundary
catalog.non-property.baseline.rule.layer-statement.positive	conformance_catalog::baseline.rule.layer-statement/positive
catalog.non-property.baseline.rule.media.boundary	conformance_catalog::baseline.rule.media/boundary
catalog.non-property.baseline.rule.media.positive	conformance_catalog::baseline.rule.media/positive
catalog.non-property.baseline.rule.scope.boundary	conformance_catalog::baseline.rule.scope/boundary
catalog.non-property.baseline.rule.scope.positive	conformance_catalog::baseline.rule.scope/positive
catalog.non-property.baseline.rule.style.boundary	conformance_catalog::baseline.rule.style/boundary
catalog.non-property.baseline.rule.style.positive	conformance_catalog::baseline.rule.style/positive
catalog.non-property.baseline.selector.attribute-case.boundary	conformance_catalog::baseline.selector.attribute-case/boundary
catalog.non-property.baseline.selector.attribute-case.positive	conformance_catalog::baseline.selector.attribute-case/positive
catalog.non-property.baseline.selector.complex.boundary	conformance_catalog::baseline.selector.complex/boundary
catalog.non-property.baseline.selector.complex.positive	conformance_catalog::baseline.selector.complex/positive
catalog.non-property.baseline.selector.extension-functional.boundary	conformance_catalog::baseline.selector.extension-functional/boundary
catalog.non-property.baseline.selector.extension-functional.positive	conformance_catalog::baseline.selector.extension-functional/positive
catalog.non-property.baseline.selector.extension-state.boundary	conformance_catalog::baseline.selector.extension-state/boundary
catalog.non-property.baseline.selector.extension-state.positive	conformance_catalog::baseline.selector.extension-state/positive
catalog.non-property.baseline.selector.functional.boundary	conformance_catalog::baseline.selector.functional/boundary
catalog.non-property.baseline.selector.functional.positive	conformance_catalog::baseline.selector.functional/positive
catalog.non-property.baseline.selector.nesting.boundary	conformance_catalog::baseline.selector.nesting/boundary
catalog.non-property.baseline.selector.nesting.positive	conformance_catalog::baseline.selector.nesting/positive
catalog.non-property.baseline.selector.pseudo-class.boundary	conformance_catalog::baseline.selector.pseudo-class/boundary
catalog.non-property.baseline.selector.pseudo-class.positive	conformance_catalog::baseline.selector.pseudo-class/positive
catalog.non-property.baseline.selector.pseudo-element.boundary	conformance_catalog::baseline.selector.pseudo-element/boundary
catalog.non-property.baseline.selector.pseudo-element.positive	conformance_catalog::baseline.selector.pseudo-element/positive
catalog.non-property.baseline.value.substitution-dependent.boundary	conformance_catalog::baseline.value.substitution-dependent/boundary
catalog.non-property.baseline.value.substitution-dependent.positive	conformance_catalog::baseline.value.substitution-dependent/positive
catalog.non-property.foundation.declaration-list.style-attribute.positive	conformance_catalog::foundation.declaration-list.style-attribute/positive
catalog.non-property.foundation.declaration.importance.positive	conformance_catalog::foundation.declaration.importance/positive
catalog.non-property.foundation.encoding.charset.positive	conformance_catalog::foundation.encoding.charset/positive
catalog.non-property.later.rule.counter-style.boundary	conformance_catalog::later.rule.counter-style/boundary
catalog.non-property.later.rule.font-feature-values.boundary	conformance_catalog::later.rule.font-feature-values/boundary
catalog.non-property.later.rule.namespace.boundary	conformance_catalog::later.rule.namespace/boundary
catalog.non-property.later.rule.page.boundary	conformance_catalog::later.rule.page/boundary
catalog.non-property.later.rule.supports.boundary	conformance_catalog::later.rule.supports/boundary
catalog.property.baseline.property.align-content.boundary	catalog_inventory::baseline.property.align-content/negative
catalog.property.baseline.property.align-content.positive	catalog_inventory::baseline.property.align-content/positive
catalog.property.baseline.property.align-items.boundary	catalog_inventory::baseline.property.align-items/negative
catalog.property.baseline.property.align-items.positive	catalog_inventory::baseline.property.align-items/positive
catalog.property.baseline.property.align-self.boundary	catalog_inventory::baseline.property.align-self/negative
catalog.property.baseline.property.align-self.positive	catalog_inventory::baseline.property.align-self/positive
catalog.property.baseline.property.align-tracks.boundary	catalog_inventory::baseline.property.align-tracks/negative
catalog.property.baseline.property.align-tracks.positive	catalog_inventory::baseline.property.align-tracks/positive
catalog.property.baseline.property.all.boundary	catalog_inventory::baseline.property.all/negative
catalog.property.baseline.property.all.positive	catalog_inventory::baseline.property.all/positive
catalog.property.baseline.property.animation-delay.boundary	catalog_inventory::baseline.property.animation-delay/negative
catalog.property.baseline.property.animation-delay.positive	catalog_inventory::baseline.property.animation-delay/positive
catalog.property.baseline.property.animation-direction.boundary	catalog_inventory::baseline.property.animation-direction/negative
catalog.property.baseline.property.animation-direction.positive	catalog_inventory::baseline.property.animation-direction/positive
catalog.property.baseline.property.animation-duration.boundary	catalog_inventory::baseline.property.animation-duration/negative
catalog.property.baseline.property.animation-duration.positive	catalog_inventory::baseline.property.animation-duration/positive
catalog.property.baseline.property.animation-fill-mode.boundary	catalog_inventory::baseline.property.animation-fill-mode/negative
catalog.property.baseline.property.animation-fill-mode.positive	catalog_inventory::baseline.property.animation-fill-mode/positive
catalog.property.baseline.property.animation-iteration-count.boundary	catalog_inventory::baseline.property.animation-iteration-count/negative
catalog.property.baseline.property.animation-iteration-count.positive	catalog_inventory::baseline.property.animation-iteration-count/positive
catalog.property.baseline.property.animation-name.boundary	catalog_inventory::baseline.property.animation-name/negative
catalog.property.baseline.property.animation-name.positive	catalog_inventory::baseline.property.animation-name/positive
catalog.property.baseline.property.animation-play-state.boundary	catalog_inventory::baseline.property.animation-play-state/negative
catalog.property.baseline.property.animation-play-state.positive	catalog_inventory::baseline.property.animation-play-state/positive
catalog.property.baseline.property.animation-timing-function.boundary	catalog_inventory::baseline.property.animation-timing-function/negative
catalog.property.baseline.property.animation-timing-function.positive	catalog_inventory::baseline.property.animation-timing-function/positive
catalog.property.baseline.property.animation.boundary	catalog_inventory::baseline.property.animation/negative
catalog.property.baseline.property.animation.positive	catalog_inventory::baseline.property.animation/positive
catalog.property.baseline.property.aspect-ratio.boundary	catalog_inventory::baseline.property.aspect-ratio/negative
catalog.property.baseline.property.aspect-ratio.positive	catalog_inventory::baseline.property.aspect-ratio/positive
catalog.property.baseline.property.backdrop-filter.boundary	catalog_inventory::baseline.property.backdrop-filter/negative
catalog.property.baseline.property.backdrop-filter.positive	catalog_inventory::baseline.property.backdrop-filter/positive
catalog.property.baseline.property.background-attachment.boundary	catalog_inventory::baseline.property.background-attachment/negative
catalog.property.baseline.property.background-attachment.positive	catalog_inventory::baseline.property.background-attachment/positive
catalog.property.baseline.property.background-clip.boundary	catalog_inventory::baseline.property.background-clip/negative
catalog.property.baseline.property.background-clip.positive	catalog_inventory::baseline.property.background-clip/positive
catalog.property.baseline.property.background-color.boundary	catalog_inventory::baseline.property.background-color/negative
catalog.property.baseline.property.background-color.positive	catalog_inventory::baseline.property.background-color/positive
catalog.property.baseline.property.background-image.boundary	catalog_inventory::baseline.property.background-image/negative
catalog.property.baseline.property.background-image.positive	catalog_inventory::baseline.property.background-image/positive
catalog.property.baseline.property.background-origin.boundary	catalog_inventory::baseline.property.background-origin/negative
catalog.property.baseline.property.background-origin.positive	catalog_inventory::baseline.property.background-origin/positive
catalog.property.baseline.property.background-position.boundary	catalog_inventory::baseline.property.background-position/negative
catalog.property.baseline.property.background-position.positive	catalog_inventory::baseline.property.background-position/positive
catalog.property.baseline.property.background-repeat.boundary	catalog_inventory::baseline.property.background-repeat/negative
catalog.property.baseline.property.background-repeat.positive	catalog_inventory::baseline.property.background-repeat/positive
catalog.property.baseline.property.background-size.boundary	catalog_inventory::baseline.property.background-size/negative
catalog.property.baseline.property.background-size.positive	catalog_inventory::baseline.property.background-size/positive
catalog.property.baseline.property.background.boundary	catalog_inventory::baseline.property.background/negative
catalog.property.baseline.property.background.positive	catalog_inventory::baseline.property.background/positive
catalog.property.baseline.property.border-bottom-color.boundary	catalog_inventory::baseline.property.border-bottom-color/negative
catalog.property.baseline.property.border-bottom-color.positive	catalog_inventory::baseline.property.border-bottom-color/positive
catalog.property.baseline.property.border-bottom-left-radius.boundary	catalog_inventory::baseline.property.border-bottom-left-radius/negative
catalog.property.baseline.property.border-bottom-left-radius.positive	catalog_inventory::baseline.property.border-bottom-left-radius/positive
catalog.property.baseline.property.border-bottom-right-radius.boundary	catalog_inventory::baseline.property.border-bottom-right-radius/negative
catalog.property.baseline.property.border-bottom-right-radius.positive	catalog_inventory::baseline.property.border-bottom-right-radius/positive
catalog.property.baseline.property.border-bottom-style.boundary	catalog_inventory::baseline.property.border-bottom-style/negative
catalog.property.baseline.property.border-bottom-style.positive	catalog_inventory::baseline.property.border-bottom-style/positive
catalog.property.baseline.property.border-bottom-width.boundary	catalog_inventory::baseline.property.border-bottom-width/negative
catalog.property.baseline.property.border-bottom-width.positive	catalog_inventory::baseline.property.border-bottom-width/positive
catalog.property.baseline.property.border-bottom.boundary	catalog_inventory::baseline.property.border-bottom/negative
catalog.property.baseline.property.border-bottom.positive	catalog_inventory::baseline.property.border-bottom/positive
catalog.property.baseline.property.border-color.boundary	catalog_inventory::baseline.property.border-color/negative
catalog.property.baseline.property.border-color.positive	catalog_inventory::baseline.property.border-color/positive
catalog.property.baseline.property.border-left-color.boundary	catalog_inventory::baseline.property.border-left-color/negative
catalog.property.baseline.property.border-left-color.positive	catalog_inventory::baseline.property.border-left-color/positive
catalog.property.baseline.property.border-left-style.boundary	catalog_inventory::baseline.property.border-left-style/negative
catalog.property.baseline.property.border-left-style.positive	catalog_inventory::baseline.property.border-left-style/positive
catalog.property.baseline.property.border-left-width.boundary	catalog_inventory::baseline.property.border-left-width/negative
catalog.property.baseline.property.border-left-width.positive	catalog_inventory::baseline.property.border-left-width/positive
catalog.property.baseline.property.border-left.boundary	catalog_inventory::baseline.property.border-left/negative
catalog.property.baseline.property.border-left.positive	catalog_inventory::baseline.property.border-left/positive
catalog.property.baseline.property.border-radius.boundary	catalog_inventory::baseline.property.border-radius/negative
catalog.property.baseline.property.border-radius.positive	catalog_inventory::baseline.property.border-radius/positive
catalog.property.baseline.property.border-right-color.boundary	catalog_inventory::baseline.property.border-right-color/negative
catalog.property.baseline.property.border-right-color.positive	catalog_inventory::baseline.property.border-right-color/positive
catalog.property.baseline.property.border-right-style.boundary	catalog_inventory::baseline.property.border-right-style/negative
catalog.property.baseline.property.border-right-style.positive	catalog_inventory::baseline.property.border-right-style/positive
catalog.property.baseline.property.border-right-width.boundary	catalog_inventory::baseline.property.border-right-width/negative
catalog.property.baseline.property.border-right-width.positive	catalog_inventory::baseline.property.border-right-width/positive
catalog.property.baseline.property.border-right.boundary	catalog_inventory::baseline.property.border-right/negative
catalog.property.baseline.property.border-right.positive	catalog_inventory::baseline.property.border-right/positive
catalog.property.baseline.property.border-style.boundary	catalog_inventory::baseline.property.border-style/negative
catalog.property.baseline.property.border-style.positive	catalog_inventory::baseline.property.border-style/positive
catalog.property.baseline.property.border-top-color.boundary	catalog_inventory::baseline.property.border-top-color/negative
catalog.property.baseline.property.border-top-color.positive	catalog_inventory::baseline.property.border-top-color/positive
catalog.property.baseline.property.border-top-left-radius.boundary	catalog_inventory::baseline.property.border-top-left-radius/negative
catalog.property.baseline.property.border-top-left-radius.positive	catalog_inventory::baseline.property.border-top-left-radius/positive
catalog.property.baseline.property.border-top-right-radius.boundary	catalog_inventory::baseline.property.border-top-right-radius/negative
catalog.property.baseline.property.border-top-right-radius.positive	catalog_inventory::baseline.property.border-top-right-radius/positive
catalog.property.baseline.property.border-top-style.boundary	catalog_inventory::baseline.property.border-top-style/negative
catalog.property.baseline.property.border-top-style.positive	catalog_inventory::baseline.property.border-top-style/positive
catalog.property.baseline.property.border-top-width.boundary	catalog_inventory::baseline.property.border-top-width/negative
catalog.property.baseline.property.border-top-width.positive	catalog_inventory::baseline.property.border-top-width/positive
catalog.property.baseline.property.border-top.boundary	catalog_inventory::baseline.property.border-top/negative
catalog.property.baseline.property.border-top.positive	catalog_inventory::baseline.property.border-top/positive
catalog.property.baseline.property.border-width.boundary	catalog_inventory::baseline.property.border-width/negative
catalog.property.baseline.property.border-width.positive	catalog_inventory::baseline.property.border-width/positive
catalog.property.baseline.property.border.boundary	catalog_inventory::baseline.property.border/negative
catalog.property.baseline.property.border.positive	catalog_inventory::baseline.property.border/positive
catalog.property.baseline.property.bottom.boundary	catalog_inventory::baseline.property.bottom/negative
catalog.property.baseline.property.bottom.positive	catalog_inventory::baseline.property.bottom/positive
catalog.property.baseline.property.box-decoration-break.boundary	catalog_inventory::baseline.property.box-decoration-break/negative
catalog.property.baseline.property.box-decoration-break.positive	catalog_inventory::baseline.property.box-decoration-break/positive
catalog.property.baseline.property.box-shadow.boundary	catalog_inventory::baseline.property.box-shadow/negative
catalog.property.baseline.property.box-shadow.positive	catalog_inventory::baseline.property.box-shadow/positive
catalog.property.baseline.property.box-sizing.boundary	catalog_inventory::baseline.property.box-sizing/negative
catalog.property.baseline.property.box-sizing.positive	catalog_inventory::baseline.property.box-sizing/positive
catalog.property.baseline.property.clear.boundary	catalog_inventory::baseline.property.clear/negative
catalog.property.baseline.property.clear.positive	catalog_inventory::baseline.property.clear/positive
catalog.property.baseline.property.clip-path.boundary	catalog_inventory::baseline.property.clip-path/negative
catalog.property.baseline.property.clip-path.positive	catalog_inventory::baseline.property.clip-path/positive
catalog.property.baseline.property.color.boundary	catalog_inventory::baseline.property.color/negative
catalog.property.baseline.property.color.positive	catalog_inventory::baseline.property.color/positive
catalog.property.baseline.property.column-gap.boundary	catalog_inventory::baseline.property.column-gap/negative
catalog.property.baseline.property.column-gap.positive	catalog_inventory::baseline.property.column-gap/positive
catalog.property.baseline.property.content-visibility.boundary	catalog_inventory::baseline.property.content-visibility/negative
catalog.property.baseline.property.content-visibility.positive	catalog_inventory::baseline.property.content-visibility/positive
catalog.property.baseline.property.content.boundary	catalog_inventory::baseline.property.content/negative
catalog.property.baseline.property.content.positive	catalog_inventory::baseline.property.content/positive
catalog.property.baseline.property.counter-increment.boundary	catalog_inventory::baseline.property.counter-increment/negative
catalog.property.baseline.property.counter-increment.positive	catalog_inventory::baseline.property.counter-increment/positive
catalog.property.baseline.property.counter-reset.boundary	catalog_inventory::baseline.property.counter-reset/negative
catalog.property.baseline.property.counter-reset.positive	catalog_inventory::baseline.property.counter-reset/positive
catalog.property.baseline.property.counter-set.boundary	catalog_inventory::baseline.property.counter-set/negative
catalog.property.baseline.property.counter-set.positive	catalog_inventory::baseline.property.counter-set/positive
catalog.property.baseline.property.cursor.boundary	catalog_inventory::baseline.property.cursor/negative
catalog.property.baseline.property.cursor.positive	catalog_inventory::baseline.property.cursor/positive
catalog.property.baseline.property.direction.boundary	catalog_inventory::baseline.property.direction/negative
catalog.property.baseline.property.direction.positive	catalog_inventory::baseline.property.direction/positive
catalog.property.baseline.property.display.boundary	catalog_inventory::baseline.property.display/negative
catalog.property.baseline.property.display.positive	catalog_inventory::baseline.property.display/positive
catalog.property.baseline.property.filter.boundary	catalog_inventory::baseline.property.filter/negative
catalog.property.baseline.property.filter.positive	catalog_inventory::baseline.property.filter/positive
catalog.property.baseline.property.flex-basis.boundary	catalog_inventory::baseline.property.flex-basis/negative
catalog.property.baseline.property.flex-basis.positive	catalog_inventory::baseline.property.flex-basis/positive
catalog.property.baseline.property.flex-direction.boundary	catalog_inventory::baseline.property.flex-direction/negative
catalog.property.baseline.property.flex-direction.positive	catalog_inventory::baseline.property.flex-direction/positive
catalog.property.baseline.property.flex-grow.boundary	catalog_inventory::baseline.property.flex-grow/negative
catalog.property.baseline.property.flex-grow.positive	catalog_inventory::baseline.property.flex-grow/positive
catalog.property.baseline.property.flex-shrink.boundary	catalog_inventory::baseline.property.flex-shrink/negative
catalog.property.baseline.property.flex-shrink.positive	catalog_inventory::baseline.property.flex-shrink/positive
catalog.property.baseline.property.flex-wrap.boundary	catalog_inventory::baseline.property.flex-wrap/negative
catalog.property.baseline.property.flex-wrap.positive	catalog_inventory::baseline.property.flex-wrap/positive
catalog.property.baseline.property.flex.boundary	catalog_inventory::baseline.property.flex/negative
catalog.property.baseline.property.flex.positive	catalog_inventory::baseline.property.flex/positive
catalog.property.baseline.property.float.boundary	catalog_inventory::baseline.property.float/negative
catalog.property.baseline.property.float.positive	catalog_inventory::baseline.property.float/positive
catalog.property.baseline.property.font-family.boundary	catalog_inventory::baseline.property.font-family/negative
catalog.property.baseline.property.font-family.positive	catalog_inventory::baseline.property.font-family/positive
catalog.property.baseline.property.font-feature-settings.boundary	catalog_inventory::baseline.property.font-feature-settings/negative
catalog.property.baseline.property.font-feature-settings.positive	catalog_inventory::baseline.property.font-feature-settings/positive
catalog.property.baseline.property.font-size.boundary	catalog_inventory::baseline.property.font-size/negative
catalog.property.baseline.property.font-size.positive	catalog_inventory::baseline.property.font-size/positive
catalog.property.baseline.property.font-stretch.boundary	catalog_inventory::baseline.property.font-stretch/negative
catalog.property.baseline.property.font-stretch.positive	catalog_inventory::baseline.property.font-stretch/positive
catalog.property.baseline.property.font-style.boundary	catalog_inventory::baseline.property.font-style/negative
catalog.property.baseline.property.font-style.positive	catalog_inventory::baseline.property.font-style/positive
catalog.property.baseline.property.font-variant.boundary	catalog_inventory::baseline.property.font-variant/negative
catalog.property.baseline.property.font-variant.positive	catalog_inventory::baseline.property.font-variant/positive
catalog.property.baseline.property.font-weight.boundary	catalog_inventory::baseline.property.font-weight/negative
catalog.property.baseline.property.font-weight.positive	catalog_inventory::baseline.property.font-weight/positive
catalog.property.baseline.property.font.boundary	catalog_inventory::baseline.property.font/negative
catalog.property.baseline.property.font.positive	catalog_inventory::baseline.property.font/positive
catalog.property.baseline.property.gap.boundary	catalog_inventory::baseline.property.gap/negative
catalog.property.baseline.property.gap.positive	catalog_inventory::baseline.property.gap/positive
catalog.property.baseline.property.grid-area.boundary	catalog_inventory::baseline.property.grid-area/negative
catalog.property.baseline.property.grid-area.positive	catalog_inventory::baseline.property.grid-area/positive
catalog.property.baseline.property.grid-auto-columns.boundary	catalog_inventory::baseline.property.grid-auto-columns/negative
catalog.property.baseline.property.grid-auto-columns.positive	catalog_inventory::baseline.property.grid-auto-columns/positive
catalog.property.baseline.property.grid-auto-flow.boundary	catalog_inventory::baseline.property.grid-auto-flow/negative
catalog.property.baseline.property.grid-auto-flow.positive	catalog_inventory::baseline.property.grid-auto-flow/positive
catalog.property.baseline.property.grid-auto-rows.boundary	catalog_inventory::baseline.property.grid-auto-rows/negative
catalog.property.baseline.property.grid-auto-rows.positive	catalog_inventory::baseline.property.grid-auto-rows/positive
catalog.property.baseline.property.grid-column-end.boundary	catalog_inventory::baseline.property.grid-column-end/negative
catalog.property.baseline.property.grid-column-end.positive	catalog_inventory::baseline.property.grid-column-end/positive
catalog.property.baseline.property.grid-column-start.boundary	catalog_inventory::baseline.property.grid-column-start/negative
catalog.property.baseline.property.grid-column-start.positive	catalog_inventory::baseline.property.grid-column-start/positive
catalog.property.baseline.property.grid-column.boundary	catalog_inventory::baseline.property.grid-column/negative
catalog.property.baseline.property.grid-column.positive	catalog_inventory::baseline.property.grid-column/positive
catalog.property.baseline.property.grid-flow-tolerance.boundary	catalog_inventory::baseline.property.grid-flow-tolerance/negative
catalog.property.baseline.property.grid-flow-tolerance.positive	catalog_inventory::baseline.property.grid-flow-tolerance/positive
catalog.property.baseline.property.grid-row-end.boundary	catalog_inventory::baseline.property.grid-row-end/negative
catalog.property.baseline.property.grid-row-end.positive	catalog_inventory::baseline.property.grid-row-end/positive
catalog.property.baseline.property.grid-row-start.boundary	catalog_inventory::baseline.property.grid-row-start/negative
catalog.property.baseline.property.grid-row-start.positive	catalog_inventory::baseline.property.grid-row-start/positive
catalog.property.baseline.property.grid-row.boundary	catalog_inventory::baseline.property.grid-row/negative
catalog.property.baseline.property.grid-row.positive	catalog_inventory::baseline.property.grid-row/positive
catalog.property.baseline.property.grid-template-areas.boundary	catalog_inventory::baseline.property.grid-template-areas/negative
catalog.property.baseline.property.grid-template-areas.positive	catalog_inventory::baseline.property.grid-template-areas/positive
catalog.property.baseline.property.grid-template-columns.boundary	catalog_inventory::baseline.property.grid-template-columns/negative
catalog.property.baseline.property.grid-template-columns.positive	catalog_inventory::baseline.property.grid-template-columns/positive
catalog.property.baseline.property.grid-template-rows.boundary	catalog_inventory::baseline.property.grid-template-rows/negative
catalog.property.baseline.property.grid-template-rows.positive	catalog_inventory::baseline.property.grid-template-rows/positive
catalog.property.baseline.property.grid-template.boundary	catalog_inventory::baseline.property.grid-template/negative
catalog.property.baseline.property.grid-template.positive	catalog_inventory::baseline.property.grid-template/positive
catalog.property.baseline.property.grid.boundary	catalog_inventory::baseline.property.grid/negative
catalog.property.baseline.property.grid.positive	catalog_inventory::baseline.property.grid/positive
catalog.property.baseline.property.height.boundary	catalog_inventory::baseline.property.height/negative
catalog.property.baseline.property.height.positive	catalog_inventory::baseline.property.height/positive
catalog.property.baseline.property.inset.boundary	catalog_inventory::baseline.property.inset/negative
catalog.property.baseline.property.inset.positive	catalog_inventory::baseline.property.inset/positive
catalog.property.baseline.property.justify-content.boundary	catalog_inventory::baseline.property.justify-content/negative
catalog.property.baseline.property.justify-content.positive	catalog_inventory::baseline.property.justify-content/positive
catalog.property.baseline.property.justify-items.boundary	catalog_inventory::baseline.property.justify-items/negative
catalog.property.baseline.property.justify-items.positive	catalog_inventory::baseline.property.justify-items/positive
catalog.property.baseline.property.justify-self.boundary	catalog_inventory::baseline.property.justify-self/negative
catalog.property.baseline.property.justify-self.positive	catalog_inventory::baseline.property.justify-self/positive
catalog.property.baseline.property.justify-tracks.boundary	catalog_inventory::baseline.property.justify-tracks/negative
catalog.property.baseline.property.justify-tracks.positive	catalog_inventory::baseline.property.justify-tracks/positive
catalog.property.baseline.property.left.boundary	catalog_inventory::baseline.property.left/negative
catalog.property.baseline.property.left.positive	catalog_inventory::baseline.property.left/positive
catalog.property.baseline.property.letter-spacing.boundary	catalog_inventory::baseline.property.letter-spacing/negative
catalog.property.baseline.property.letter-spacing.positive	catalog_inventory::baseline.property.letter-spacing/positive
catalog.property.baseline.property.line-height.boundary	catalog_inventory::baseline.property.line-height/negative
catalog.property.baseline.property.line-height.positive	catalog_inventory::baseline.property.line-height/positive
catalog.property.baseline.property.list-style-image.boundary	catalog_inventory::baseline.property.list-style-image/negative
catalog.property.baseline.property.list-style-image.positive	catalog_inventory::baseline.property.list-style-image/positive
catalog.property.baseline.property.list-style-position.boundary	catalog_inventory::baseline.property.list-style-position/negative
catalog.property.baseline.property.list-style-position.positive	catalog_inventory::baseline.property.list-style-position/positive
catalog.property.baseline.property.list-style-type.boundary	catalog_inventory::baseline.property.list-style-type/negative
catalog.property.baseline.property.list-style-type.positive	catalog_inventory::baseline.property.list-style-type/positive
catalog.property.baseline.property.list-style.boundary	catalog_inventory::baseline.property.list-style/negative
catalog.property.baseline.property.list-style.positive	catalog_inventory::baseline.property.list-style/positive
catalog.property.baseline.property.margin-bottom.boundary	catalog_inventory::baseline.property.margin-bottom/negative
catalog.property.baseline.property.margin-bottom.positive	catalog_inventory::baseline.property.margin-bottom/positive
catalog.property.baseline.property.margin-left.boundary	catalog_inventory::baseline.property.margin-left/negative
catalog.property.baseline.property.margin-left.positive	catalog_inventory::baseline.property.margin-left/positive
catalog.property.baseline.property.margin-right.boundary	catalog_inventory::baseline.property.margin-right/negative
catalog.property.baseline.property.margin-right.positive	catalog_inventory::baseline.property.margin-right/positive
catalog.property.baseline.property.margin-top.boundary	catalog_inventory::baseline.property.margin-top/negative
catalog.property.baseline.property.margin-top.positive	catalog_inventory::baseline.property.margin-top/positive
catalog.property.baseline.property.margin.boundary	catalog_inventory::baseline.property.margin/negative
catalog.property.baseline.property.margin.positive	catalog_inventory::baseline.property.margin/positive
catalog.property.baseline.property.mask-image.boundary	catalog_inventory::baseline.property.mask-image/negative
catalog.property.baseline.property.mask-image.positive	catalog_inventory::baseline.property.mask-image/positive
catalog.property.baseline.property.mask-position.boundary	catalog_inventory::baseline.property.mask-position/negative
catalog.property.baseline.property.mask-position.positive	catalog_inventory::baseline.property.mask-position/positive
catalog.property.baseline.property.mask-repeat.boundary	catalog_inventory::baseline.property.mask-repeat/negative
catalog.property.baseline.property.mask-repeat.positive	catalog_inventory::baseline.property.mask-repeat/positive
catalog.property.baseline.property.mask-size.boundary	catalog_inventory::baseline.property.mask-size/negative
catalog.property.baseline.property.mask-size.positive	catalog_inventory::baseline.property.mask-size/positive
catalog.property.baseline.property.mask.boundary	catalog_inventory::baseline.property.mask/negative
catalog.property.baseline.property.mask.positive	catalog_inventory::baseline.property.mask/positive
catalog.property.baseline.property.max-height.boundary	catalog_inventory::baseline.property.max-height/negative
catalog.property.baseline.property.max-height.positive	catalog_inventory::baseline.property.max-height/positive
catalog.property.baseline.property.max-width.boundary	catalog_inventory::baseline.property.max-width/negative
catalog.property.baseline.property.max-width.positive	catalog_inventory::baseline.property.max-width/positive
catalog.property.baseline.property.min-height.boundary	catalog_inventory::baseline.property.min-height/negative
catalog.property.baseline.property.min-height.positive	catalog_inventory::baseline.property.min-height/positive
catalog.property.baseline.property.min-width.boundary	catalog_inventory::baseline.property.min-width/negative
catalog.property.baseline.property.min-width.positive	catalog_inventory::baseline.property.min-width/positive
catalog.property.baseline.property.opacity.boundary	catalog_inventory::baseline.property.opacity/negative
catalog.property.baseline.property.opacity.positive	catalog_inventory::baseline.property.opacity/positive
catalog.property.baseline.property.order.boundary	catalog_inventory::baseline.property.order/negative
catalog.property.baseline.property.order.positive	catalog_inventory::baseline.property.order/positive
catalog.property.baseline.property.outline-color.boundary	catalog_inventory::baseline.property.outline-color/negative
catalog.property.baseline.property.outline-color.positive	catalog_inventory::baseline.property.outline-color/positive
catalog.property.baseline.property.outline-style.boundary	catalog_inventory::baseline.property.outline-style/negative
catalog.property.baseline.property.outline-style.positive	catalog_inventory::baseline.property.outline-style/positive
catalog.property.baseline.property.outline-width.boundary	catalog_inventory::baseline.property.outline-width/negative
catalog.property.baseline.property.outline-width.positive	catalog_inventory::baseline.property.outline-width/positive
catalog.property.baseline.property.outline.boundary	catalog_inventory::baseline.property.outline/negative
catalog.property.baseline.property.outline.positive	catalog_inventory::baseline.property.outline/positive
catalog.property.baseline.property.overflow-wrap.boundary	catalog_inventory::baseline.property.overflow-wrap/negative
catalog.property.baseline.property.overflow-wrap.positive	catalog_inventory::baseline.property.overflow-wrap/positive
catalog.property.baseline.property.overflow-x.boundary	catalog_inventory::baseline.property.overflow-x/negative
catalog.property.baseline.property.overflow-x.positive	catalog_inventory::baseline.property.overflow-x/positive
catalog.property.baseline.property.overflow-y.boundary	catalog_inventory::baseline.property.overflow-y/negative
catalog.property.baseline.property.overflow-y.positive	catalog_inventory::baseline.property.overflow-y/positive
catalog.property.baseline.property.overflow.boundary	catalog_inventory::baseline.property.overflow/negative
catalog.property.baseline.property.overflow.positive	catalog_inventory::baseline.property.overflow/positive
catalog.property.baseline.property.padding-bottom.boundary	catalog_inventory::baseline.property.padding-bottom/negative
catalog.property.baseline.property.padding-bottom.positive	catalog_inventory::baseline.property.padding-bottom/positive
catalog.property.baseline.property.padding-left.boundary	catalog_inventory::baseline.property.padding-left/negative
catalog.property.baseline.property.padding-left.positive	catalog_inventory::baseline.property.padding-left/positive
catalog.property.baseline.property.padding-right.boundary	catalog_inventory::baseline.property.padding-right/negative
catalog.property.baseline.property.padding-right.positive	catalog_inventory::baseline.property.padding-right/positive
catalog.property.baseline.property.padding-top.boundary	catalog_inventory::baseline.property.padding-top/negative
catalog.property.baseline.property.padding-top.positive	catalog_inventory::baseline.property.padding-top/positive
catalog.property.baseline.property.padding.boundary	catalog_inventory::baseline.property.padding/negative
catalog.property.baseline.property.padding.positive	catalog_inventory::baseline.property.padding/positive
catalog.property.baseline.property.place-content.boundary	catalog_inventory::baseline.property.place-content/negative
catalog.property.baseline.property.place-content.positive	catalog_inventory::baseline.property.place-content/positive
catalog.property.baseline.property.place-items.boundary	catalog_inventory::baseline.property.place-items/negative
catalog.property.baseline.property.place-items.positive	catalog_inventory::baseline.property.place-items/positive
catalog.property.baseline.property.place-self.boundary	catalog_inventory::baseline.property.place-self/negative
catalog.property.baseline.property.place-self.positive	catalog_inventory::baseline.property.place-self/positive
catalog.property.baseline.property.pointer-events.boundary	catalog_inventory::baseline.property.pointer-events/negative
catalog.property.baseline.property.pointer-events.positive	catalog_inventory::baseline.property.pointer-events/positive
catalog.property.baseline.property.position.boundary	catalog_inventory::baseline.property.position/negative
catalog.property.baseline.property.position.positive	catalog_inventory::baseline.property.position/positive
catalog.property.baseline.property.right.boundary	catalog_inventory::baseline.property.right/negative
catalog.property.baseline.property.right.positive	catalog_inventory::baseline.property.right/positive
catalog.property.baseline.property.rotate.boundary	catalog_inventory::baseline.property.rotate/negative
catalog.property.baseline.property.rotate.positive	catalog_inventory::baseline.property.rotate/positive
catalog.property.baseline.property.row-gap.boundary	catalog_inventory::baseline.property.row-gap/negative
catalog.property.baseline.property.row-gap.positive	catalog_inventory::baseline.property.row-gap/positive
catalog.property.baseline.property.scale.boundary	catalog_inventory::baseline.property.scale/negative
catalog.property.baseline.property.scale.positive	catalog_inventory::baseline.property.scale/positive
catalog.property.baseline.property.scrollbar-width.boundary	catalog_inventory::baseline.property.scrollbar-width/negative
catalog.property.baseline.property.scrollbar-width.positive	catalog_inventory::baseline.property.scrollbar-width/positive
catalog.property.baseline.property.text-align-last.boundary	catalog_inventory::baseline.property.text-align-last/negative
catalog.property.baseline.property.text-align-last.positive	catalog_inventory::baseline.property.text-align-last/positive
catalog.property.baseline.property.text-align.boundary	catalog_inventory::baseline.property.text-align/negative
catalog.property.baseline.property.text-align.positive	catalog_inventory::baseline.property.text-align/positive
catalog.property.baseline.property.text-decoration-color.boundary	catalog_inventory::baseline.property.text-decoration-color/negative
catalog.property.baseline.property.text-decoration-color.positive	catalog_inventory::baseline.property.text-decoration-color/positive
catalog.property.baseline.property.text-decoration-line.boundary	catalog_inventory::baseline.property.text-decoration-line/negative
catalog.property.baseline.property.text-decoration-line.positive	catalog_inventory::baseline.property.text-decoration-line/positive
catalog.property.baseline.property.text-decoration-style.boundary	catalog_inventory::baseline.property.text-decoration-style/negative
catalog.property.baseline.property.text-decoration-style.positive	catalog_inventory::baseline.property.text-decoration-style/positive
catalog.property.baseline.property.text-decoration-thickness.boundary	catalog_inventory::baseline.property.text-decoration-thickness/negative
catalog.property.baseline.property.text-decoration-thickness.positive	catalog_inventory::baseline.property.text-decoration-thickness/positive
catalog.property.baseline.property.text-decoration.boundary	catalog_inventory::baseline.property.text-decoration/negative
catalog.property.baseline.property.text-decoration.positive	catalog_inventory::baseline.property.text-decoration/positive
catalog.property.baseline.property.text-indent.boundary	catalog_inventory::baseline.property.text-indent/negative
catalog.property.baseline.property.text-indent.positive	catalog_inventory::baseline.property.text-indent/positive
catalog.property.baseline.property.text-overflow.boundary	catalog_inventory::baseline.property.text-overflow/negative
catalog.property.baseline.property.text-overflow.positive	catalog_inventory::baseline.property.text-overflow/positive
catalog.property.baseline.property.text-transform.boundary	catalog_inventory::baseline.property.text-transform/negative
catalog.property.baseline.property.text-transform.positive	catalog_inventory::baseline.property.text-transform/positive
catalog.property.baseline.property.text-wrap.boundary	catalog_inventory::baseline.property.text-wrap/negative
catalog.property.baseline.property.text-wrap.positive	catalog_inventory::baseline.property.text-wrap/positive
catalog.property.baseline.property.top.boundary	catalog_inventory::baseline.property.top/negative
catalog.property.baseline.property.top.positive	catalog_inventory::baseline.property.top/positive
catalog.property.baseline.property.transform-origin.boundary	catalog_inventory::baseline.property.transform-origin/negative
catalog.property.baseline.property.transform-origin.positive	catalog_inventory::baseline.property.transform-origin/positive
catalog.property.baseline.property.transform.boundary	catalog_inventory::baseline.property.transform/negative
catalog.property.baseline.property.transform.positive	catalog_inventory::baseline.property.transform/positive
catalog.property.baseline.property.transition-delay.boundary	catalog_inventory::baseline.property.transition-delay/negative
catalog.property.baseline.property.transition-delay.positive	catalog_inventory::baseline.property.transition-delay/positive
catalog.property.baseline.property.transition-duration.boundary	catalog_inventory::baseline.property.transition-duration/negative
catalog.property.baseline.property.transition-duration.positive	catalog_inventory::baseline.property.transition-duration/positive
catalog.property.baseline.property.transition-property.boundary	catalog_inventory::baseline.property.transition-property/negative
catalog.property.baseline.property.transition-property.positive	catalog_inventory::baseline.property.transition-property/positive
catalog.property.baseline.property.transition-timing-function.boundary	catalog_inventory::baseline.property.transition-timing-function/negative
catalog.property.baseline.property.transition-timing-function.positive	catalog_inventory::baseline.property.transition-timing-function/positive
catalog.property.baseline.property.transition.boundary	catalog_inventory::baseline.property.transition/negative
catalog.property.baseline.property.transition.positive	catalog_inventory::baseline.property.transition/positive
catalog.property.baseline.property.translate.boundary	catalog_inventory::baseline.property.translate/negative
catalog.property.baseline.property.translate.positive	catalog_inventory::baseline.property.translate/positive
catalog.property.baseline.property.user-select.boundary	catalog_inventory::baseline.property.user-select/negative
catalog.property.baseline.property.user-select.positive	catalog_inventory::baseline.property.user-select/positive
catalog.property.baseline.property.vertical-align.boundary	catalog_inventory::baseline.property.vertical-align/negative
catalog.property.baseline.property.vertical-align.positive	catalog_inventory::baseline.property.vertical-align/positive
catalog.property.baseline.property.visibility.boundary	catalog_inventory::baseline.property.visibility/negative
catalog.property.baseline.property.visibility.positive	catalog_inventory::baseline.property.visibility/positive
catalog.property.baseline.property.white-space.boundary	catalog_inventory::baseline.property.white-space/negative
catalog.property.baseline.property.white-space.positive	catalog_inventory::baseline.property.white-space/positive
catalog.property.baseline.property.width.boundary	catalog_inventory::baseline.property.width/negative
catalog.property.baseline.property.width.positive	catalog_inventory::baseline.property.width/positive
catalog.property.baseline.property.word-break.boundary	catalog_inventory::baseline.property.word-break/negative
catalog.property.baseline.property.word-break.positive	catalog_inventory::baseline.property.word-break/positive
catalog.property.baseline.property.writing-mode.boundary	catalog_inventory::baseline.property.writing-mode/negative
catalog.property.baseline.property.writing-mode.positive	catalog_inventory::baseline.property.writing-mode/positive
catalog.property.baseline.property.z-index.boundary	catalog_inventory::baseline.property.z-index/negative
catalog.property.baseline.property.z-index.positive	catalog_inventory::baseline.property.z-index/positive
focused.app-strict.clean-sheet	app_strict_parity::clean-sheet
focused.app-strict.clean-style	app_strict_parity::clean-style
focused.app-strict.implicit-sheet	app_strict_parity::implicit-sheet
focused.app-strict.implicit-style	app_strict_parity::implicit-style
focused.app-strict.multi-sheet	app_strict_parity::multi-sheet
focused.app-strict.multi-style	app_strict_parity::multi-style
focused.app-strict.never	app_strict_parity::never
focused.app-strict.recovered-sheet	app_strict_parity::recovered-sheet
focused.app-strict.recovered-style	app_strict_parity::recovered-style
focused.app-strict.selector-depth	app_strict_parity::selector-depth
focused.app-strict.structural-depth	app_strict_parity::structural-depth
focused.app-strict.style-depth	app_strict_parity::style-depth
focused.authored-values.00	authored_declaration_values::case/0
focused.authored-values.01	authored_declaration_values::case/1
focused.authored-values.02	authored_declaration_values::case/2
focused.authored-values.03	authored_declaration_values::case/3
focused.authored-values.04	authored_declaration_values::case/4
focused.authored-values.05	authored_declaration_values::case/5
focused.authored-values.06	authored_declaration_values::case/6
focused.authored-values.07	authored_declaration_values::case/7
focused.authored-values.08	authored_declaration_values::case/8
focused.authored-values.09	authored_declaration_values::case/9
focused.coupled.00	coupled_declarations::case/0
focused.coupled.01	coupled_declarations::case/1
focused.coupled.02	coupled_declarations::case/2
focused.coupled.03	coupled_declarations::case/3
focused.coupled.04	coupled_declarations::case/4
focused.coupled.05	coupled_declarations::case/5
focused.importance.00	declaration_importance::case/0
focused.importance.01	declaration_importance::case/1
focused.importance.02	declaration_importance::case/2
focused.importance.03	declaration_importance::case/3
focused.importance.04	declaration_importance::case/4
focused.importance.05	declaration_importance::case/5
focused.importance.06	declaration_importance::case/6
focused.importance.07	declaration_importance::case/7
focused.initiative-audit.00	initiative_i01_audit::case/0
focused.initiative-audit.01	initiative_i01_audit::case/1
focused.initiative-audit.02	initiative_i01_audit::case/2
focused.initiative-audit.03	initiative_i01_audit::case/3
focused.initiative-audit.04	initiative_i01_audit::case/4
focused.initiative-audit.05	initiative_i01_audit::case/5
focused.initiative-audit.06	initiative_i01_audit::case/6
focused.initiative-audit.07	initiative_i01_audit::case/7
focused.nested-structural.group.container	nested_structural_recovery::nested_structural_group_contexts_retain_siblings_around_balanced_at_rule_failure/container
focused.nested-structural.group.layer	nested_structural_recovery::nested_structural_group_contexts_retain_siblings_around_balanced_at_rule_failure/layer
focused.nested-structural.group.media	nested_structural_recovery::nested_structural_group_contexts_retain_siblings_around_balanced_at_rule_failure/media
focused.nested-structural.keyframes-balanced	nested_structural_recovery::keyframes-balanced
focused.nested-structural.keyframes-child-loss	nested_structural_recovery::keyframes-child-loss
focused.nested-structural.keyframes-recover	nested_structural_recovery::keyframes-recover
focused.nested-structural.qualified.group	nested_structural_recovery::nested_structural_qualified_failures_recover_in_group_scope_and_style_contexts/group
focused.nested-structural.qualified.scope	nested_structural_recovery::nested_structural_qualified_failures_recover_in_group_scope_and_style_contexts/scope
focused.nested-structural.qualified.style	nested_structural_recovery::nested_structural_qualified_failures_recover_in_group_scope_and_style_contexts/style
focused.nested-structural.repeated	nested_structural_recovery::repeated
focused.nested-structural.scope-at-rule	nested_structural_recovery::scope-at-rule
focused.nested-structural.style-at-rule	nested_structural_recovery::style-at-rule
focused.property-schema.baseline.property.align-content.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.align-content/important
focused.property-schema.baseline.property.align-content.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.align-content/ordinary
focused.property-schema.baseline.property.align-items.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.align-items/important
focused.property-schema.baseline.property.align-items.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.align-items/ordinary
focused.property-schema.baseline.property.align-self.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.align-self/important
focused.property-schema.baseline.property.align-self.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.align-self/ordinary
focused.property-schema.baseline.property.align-tracks.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.align-tracks/important
focused.property-schema.baseline.property.align-tracks.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.align-tracks/ordinary
focused.property-schema.baseline.property.all.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.all/important
focused.property-schema.baseline.property.all.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.all/ordinary
focused.property-schema.baseline.property.animation-delay.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-delay/important
focused.property-schema.baseline.property.animation-delay.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-delay/ordinary
focused.property-schema.baseline.property.animation-direction.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-direction/important
focused.property-schema.baseline.property.animation-direction.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-direction/ordinary
focused.property-schema.baseline.property.animation-duration.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-duration/important
focused.property-schema.baseline.property.animation-duration.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-duration/ordinary
focused.property-schema.baseline.property.animation-fill-mode.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-fill-mode/important
focused.property-schema.baseline.property.animation-fill-mode.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-fill-mode/ordinary
focused.property-schema.baseline.property.animation-iteration-count.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-iteration-count/important
focused.property-schema.baseline.property.animation-iteration-count.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-iteration-count/ordinary
focused.property-schema.baseline.property.animation-name.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-name/important
focused.property-schema.baseline.property.animation-name.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-name/ordinary
focused.property-schema.baseline.property.animation-play-state.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-play-state/important
focused.property-schema.baseline.property.animation-play-state.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-play-state/ordinary
focused.property-schema.baseline.property.animation-timing-function.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-timing-function/important
focused.property-schema.baseline.property.animation-timing-function.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation-timing-function/ordinary
focused.property-schema.baseline.property.animation.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation/important
focused.property-schema.baseline.property.animation.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.animation/ordinary
focused.property-schema.baseline.property.aspect-ratio.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.aspect-ratio/important
focused.property-schema.baseline.property.aspect-ratio.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.aspect-ratio/ordinary
focused.property-schema.baseline.property.backdrop-filter.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.backdrop-filter/important
focused.property-schema.baseline.property.backdrop-filter.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.backdrop-filter/ordinary
focused.property-schema.baseline.property.background-attachment.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-attachment/important
focused.property-schema.baseline.property.background-attachment.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-attachment/ordinary
focused.property-schema.baseline.property.background-clip.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-clip/important
focused.property-schema.baseline.property.background-clip.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-clip/ordinary
focused.property-schema.baseline.property.background-color.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-color/important
focused.property-schema.baseline.property.background-color.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-color/ordinary
focused.property-schema.baseline.property.background-image.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-image/important
focused.property-schema.baseline.property.background-image.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-image/ordinary
focused.property-schema.baseline.property.background-origin.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-origin/important
focused.property-schema.baseline.property.background-origin.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-origin/ordinary
focused.property-schema.baseline.property.background-position.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-position/important
focused.property-schema.baseline.property.background-position.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-position/ordinary
focused.property-schema.baseline.property.background-repeat.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-repeat/important
focused.property-schema.baseline.property.background-repeat.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-repeat/ordinary
focused.property-schema.baseline.property.background-size.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-size/important
focused.property-schema.baseline.property.background-size.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background-size/ordinary
focused.property-schema.baseline.property.background.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background/important
focused.property-schema.baseline.property.background.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.background/ordinary
focused.property-schema.baseline.property.border-bottom-color.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-color/important
focused.property-schema.baseline.property.border-bottom-color.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-color/ordinary
focused.property-schema.baseline.property.border-bottom-left-radius.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-left-radius/important
focused.property-schema.baseline.property.border-bottom-left-radius.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-left-radius/ordinary
focused.property-schema.baseline.property.border-bottom-right-radius.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-right-radius/important
focused.property-schema.baseline.property.border-bottom-right-radius.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-right-radius/ordinary
focused.property-schema.baseline.property.border-bottom-style.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-style/important
focused.property-schema.baseline.property.border-bottom-style.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-style/ordinary
focused.property-schema.baseline.property.border-bottom-width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-width/important
focused.property-schema.baseline.property.border-bottom-width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom-width/ordinary
focused.property-schema.baseline.property.border-bottom.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom/important
focused.property-schema.baseline.property.border-bottom.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-bottom/ordinary
focused.property-schema.baseline.property.border-color.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-color/important
focused.property-schema.baseline.property.border-color.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-color/ordinary
focused.property-schema.baseline.property.border-left-color.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-left-color/important
focused.property-schema.baseline.property.border-left-color.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-left-color/ordinary
focused.property-schema.baseline.property.border-left-style.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-left-style/important
focused.property-schema.baseline.property.border-left-style.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-left-style/ordinary
focused.property-schema.baseline.property.border-left-width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-left-width/important
focused.property-schema.baseline.property.border-left-width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-left-width/ordinary
focused.property-schema.baseline.property.border-left.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-left/important
focused.property-schema.baseline.property.border-left.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-left/ordinary
focused.property-schema.baseline.property.border-radius.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-radius/important
focused.property-schema.baseline.property.border-radius.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-radius/ordinary
focused.property-schema.baseline.property.border-right-color.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-right-color/important
focused.property-schema.baseline.property.border-right-color.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-right-color/ordinary
focused.property-schema.baseline.property.border-right-style.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-right-style/important
focused.property-schema.baseline.property.border-right-style.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-right-style/ordinary
focused.property-schema.baseline.property.border-right-width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-right-width/important
focused.property-schema.baseline.property.border-right-width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-right-width/ordinary
focused.property-schema.baseline.property.border-right.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-right/important
focused.property-schema.baseline.property.border-right.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-right/ordinary
focused.property-schema.baseline.property.border-style.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-style/important
focused.property-schema.baseline.property.border-style.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-style/ordinary
focused.property-schema.baseline.property.border-top-color.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-color/important
focused.property-schema.baseline.property.border-top-color.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-color/ordinary
focused.property-schema.baseline.property.border-top-left-radius.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-left-radius/important
focused.property-schema.baseline.property.border-top-left-radius.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-left-radius/ordinary
focused.property-schema.baseline.property.border-top-right-radius.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-right-radius/important
focused.property-schema.baseline.property.border-top-right-radius.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-right-radius/ordinary
focused.property-schema.baseline.property.border-top-style.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-style/important
focused.property-schema.baseline.property.border-top-style.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-style/ordinary
focused.property-schema.baseline.property.border-top-width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-width/important
focused.property-schema.baseline.property.border-top-width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top-width/ordinary
focused.property-schema.baseline.property.border-top.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top/important
focused.property-schema.baseline.property.border-top.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-top/ordinary
focused.property-schema.baseline.property.border-width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-width/important
focused.property-schema.baseline.property.border-width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border-width/ordinary
focused.property-schema.baseline.property.border.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border/important
focused.property-schema.baseline.property.border.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.border/ordinary
focused.property-schema.baseline.property.bottom.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.bottom/important
focused.property-schema.baseline.property.bottom.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.bottom/ordinary
focused.property-schema.baseline.property.box-decoration-break.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.box-decoration-break/important
focused.property-schema.baseline.property.box-decoration-break.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.box-decoration-break/ordinary
focused.property-schema.baseline.property.box-shadow.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.box-shadow/important
focused.property-schema.baseline.property.box-shadow.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.box-shadow/ordinary
focused.property-schema.baseline.property.box-sizing.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.box-sizing/important
focused.property-schema.baseline.property.box-sizing.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.box-sizing/ordinary
focused.property-schema.baseline.property.clear.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.clear/important
focused.property-schema.baseline.property.clear.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.clear/ordinary
focused.property-schema.baseline.property.clip-path.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.clip-path/important
focused.property-schema.baseline.property.clip-path.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.clip-path/ordinary
focused.property-schema.baseline.property.color.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.color/important
focused.property-schema.baseline.property.color.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.color/ordinary
focused.property-schema.baseline.property.column-gap.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.column-gap/important
focused.property-schema.baseline.property.column-gap.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.column-gap/ordinary
focused.property-schema.baseline.property.content-visibility.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.content-visibility/important
focused.property-schema.baseline.property.content-visibility.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.content-visibility/ordinary
focused.property-schema.baseline.property.content.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.content/important
focused.property-schema.baseline.property.content.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.content/ordinary
focused.property-schema.baseline.property.counter-increment.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.counter-increment/important
focused.property-schema.baseline.property.counter-increment.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.counter-increment/ordinary
focused.property-schema.baseline.property.counter-reset.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.counter-reset/important
focused.property-schema.baseline.property.counter-reset.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.counter-reset/ordinary
focused.property-schema.baseline.property.counter-set.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.counter-set/important
focused.property-schema.baseline.property.counter-set.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.counter-set/ordinary
focused.property-schema.baseline.property.cursor.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.cursor/important
focused.property-schema.baseline.property.cursor.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.cursor/ordinary
focused.property-schema.baseline.property.direction.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.direction/important
focused.property-schema.baseline.property.direction.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.direction/ordinary
focused.property-schema.baseline.property.display.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.display/important
focused.property-schema.baseline.property.display.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.display/ordinary
focused.property-schema.baseline.property.filter.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.filter/important
focused.property-schema.baseline.property.filter.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.filter/ordinary
focused.property-schema.baseline.property.flex-basis.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-basis/important
focused.property-schema.baseline.property.flex-basis.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-basis/ordinary
focused.property-schema.baseline.property.flex-direction.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-direction/important
focused.property-schema.baseline.property.flex-direction.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-direction/ordinary
focused.property-schema.baseline.property.flex-grow.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-grow/important
focused.property-schema.baseline.property.flex-grow.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-grow/ordinary
focused.property-schema.baseline.property.flex-shrink.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-shrink/important
focused.property-schema.baseline.property.flex-shrink.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-shrink/ordinary
focused.property-schema.baseline.property.flex-wrap.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-wrap/important
focused.property-schema.baseline.property.flex-wrap.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex-wrap/ordinary
focused.property-schema.baseline.property.flex.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex/important
focused.property-schema.baseline.property.flex.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.flex/ordinary
focused.property-schema.baseline.property.float.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.float/important
focused.property-schema.baseline.property.float.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.float/ordinary
focused.property-schema.baseline.property.font-family.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-family/important
focused.property-schema.baseline.property.font-family.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-family/ordinary
focused.property-schema.baseline.property.font-feature-settings.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-feature-settings/important
focused.property-schema.baseline.property.font-feature-settings.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-feature-settings/ordinary
focused.property-schema.baseline.property.font-size.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-size/important
focused.property-schema.baseline.property.font-size.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-size/ordinary
focused.property-schema.baseline.property.font-stretch.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-stretch/important
focused.property-schema.baseline.property.font-stretch.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-stretch/ordinary
focused.property-schema.baseline.property.font-style.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-style/important
focused.property-schema.baseline.property.font-style.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-style/ordinary
focused.property-schema.baseline.property.font-variant.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-variant/important
focused.property-schema.baseline.property.font-variant.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-variant/ordinary
focused.property-schema.baseline.property.font-weight.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-weight/important
focused.property-schema.baseline.property.font-weight.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font-weight/ordinary
focused.property-schema.baseline.property.font.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font/important
focused.property-schema.baseline.property.font.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.font/ordinary
focused.property-schema.baseline.property.gap.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.gap/important
focused.property-schema.baseline.property.gap.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.gap/ordinary
focused.property-schema.baseline.property.grid-area.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-area/important
focused.property-schema.baseline.property.grid-area.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-area/ordinary
focused.property-schema.baseline.property.grid-auto-columns.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-auto-columns/important
focused.property-schema.baseline.property.grid-auto-columns.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-auto-columns/ordinary
focused.property-schema.baseline.property.grid-auto-flow.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-auto-flow/important
focused.property-schema.baseline.property.grid-auto-flow.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-auto-flow/ordinary
focused.property-schema.baseline.property.grid-auto-rows.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-auto-rows/important
focused.property-schema.baseline.property.grid-auto-rows.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-auto-rows/ordinary
focused.property-schema.baseline.property.grid-column-end.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-column-end/important
focused.property-schema.baseline.property.grid-column-end.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-column-end/ordinary
focused.property-schema.baseline.property.grid-column-start.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-column-start/important
focused.property-schema.baseline.property.grid-column-start.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-column-start/ordinary
focused.property-schema.baseline.property.grid-column.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-column/important
focused.property-schema.baseline.property.grid-column.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-column/ordinary
focused.property-schema.baseline.property.grid-flow-tolerance.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-flow-tolerance/important
focused.property-schema.baseline.property.grid-flow-tolerance.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-flow-tolerance/ordinary
focused.property-schema.baseline.property.grid-row-end.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-row-end/important
focused.property-schema.baseline.property.grid-row-end.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-row-end/ordinary
focused.property-schema.baseline.property.grid-row-start.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-row-start/important
focused.property-schema.baseline.property.grid-row-start.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-row-start/ordinary
focused.property-schema.baseline.property.grid-row.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-row/important
focused.property-schema.baseline.property.grid-row.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-row/ordinary
focused.property-schema.baseline.property.grid-template-areas.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-template-areas/important
focused.property-schema.baseline.property.grid-template-areas.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-template-areas/ordinary
focused.property-schema.baseline.property.grid-template-columns.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-template-columns/important
focused.property-schema.baseline.property.grid-template-columns.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-template-columns/ordinary
focused.property-schema.baseline.property.grid-template-rows.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-template-rows/important
focused.property-schema.baseline.property.grid-template-rows.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-template-rows/ordinary
focused.property-schema.baseline.property.grid-template.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-template/important
focused.property-schema.baseline.property.grid-template.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid-template/ordinary
focused.property-schema.baseline.property.grid.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid/important
focused.property-schema.baseline.property.grid.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.grid/ordinary
focused.property-schema.baseline.property.height.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.height/important
focused.property-schema.baseline.property.height.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.height/ordinary
focused.property-schema.baseline.property.inset.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.inset/important
focused.property-schema.baseline.property.inset.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.inset/ordinary
focused.property-schema.baseline.property.justify-content.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.justify-content/important
focused.property-schema.baseline.property.justify-content.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.justify-content/ordinary
focused.property-schema.baseline.property.justify-items.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.justify-items/important
focused.property-schema.baseline.property.justify-items.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.justify-items/ordinary
focused.property-schema.baseline.property.justify-self.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.justify-self/important
focused.property-schema.baseline.property.justify-self.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.justify-self/ordinary
focused.property-schema.baseline.property.justify-tracks.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.justify-tracks/important
focused.property-schema.baseline.property.justify-tracks.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.justify-tracks/ordinary
focused.property-schema.baseline.property.left.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.left/important
focused.property-schema.baseline.property.left.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.left/ordinary
focused.property-schema.baseline.property.letter-spacing.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.letter-spacing/important
focused.property-schema.baseline.property.letter-spacing.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.letter-spacing/ordinary
focused.property-schema.baseline.property.line-height.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.line-height/important
focused.property-schema.baseline.property.line-height.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.line-height/ordinary
focused.property-schema.baseline.property.list-style-image.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.list-style-image/important
focused.property-schema.baseline.property.list-style-image.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.list-style-image/ordinary
focused.property-schema.baseline.property.list-style-position.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.list-style-position/important
focused.property-schema.baseline.property.list-style-position.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.list-style-position/ordinary
focused.property-schema.baseline.property.list-style-type.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.list-style-type/important
focused.property-schema.baseline.property.list-style-type.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.list-style-type/ordinary
focused.property-schema.baseline.property.list-style.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.list-style/important
focused.property-schema.baseline.property.list-style.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.list-style/ordinary
focused.property-schema.baseline.property.margin-bottom.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin-bottom/important
focused.property-schema.baseline.property.margin-bottom.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin-bottom/ordinary
focused.property-schema.baseline.property.margin-left.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin-left/important
focused.property-schema.baseline.property.margin-left.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin-left/ordinary
focused.property-schema.baseline.property.margin-right.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin-right/important
focused.property-schema.baseline.property.margin-right.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin-right/ordinary
focused.property-schema.baseline.property.margin-top.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin-top/important
focused.property-schema.baseline.property.margin-top.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin-top/ordinary
focused.property-schema.baseline.property.margin.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin/important
focused.property-schema.baseline.property.margin.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.margin/ordinary
focused.property-schema.baseline.property.mask-image.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask-image/important
focused.property-schema.baseline.property.mask-image.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask-image/ordinary
focused.property-schema.baseline.property.mask-position.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask-position/important
focused.property-schema.baseline.property.mask-position.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask-position/ordinary
focused.property-schema.baseline.property.mask-repeat.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask-repeat/important
focused.property-schema.baseline.property.mask-repeat.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask-repeat/ordinary
focused.property-schema.baseline.property.mask-size.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask-size/important
focused.property-schema.baseline.property.mask-size.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask-size/ordinary
focused.property-schema.baseline.property.mask.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask/important
focused.property-schema.baseline.property.mask.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.mask/ordinary
focused.property-schema.baseline.property.max-height.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.max-height/important
focused.property-schema.baseline.property.max-height.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.max-height/ordinary
focused.property-schema.baseline.property.max-width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.max-width/important
focused.property-schema.baseline.property.max-width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.max-width/ordinary
focused.property-schema.baseline.property.min-height.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.min-height/important
focused.property-schema.baseline.property.min-height.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.min-height/ordinary
focused.property-schema.baseline.property.min-width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.min-width/important
focused.property-schema.baseline.property.min-width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.min-width/ordinary
focused.property-schema.baseline.property.opacity.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.opacity/important
focused.property-schema.baseline.property.opacity.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.opacity/ordinary
focused.property-schema.baseline.property.order.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.order/important
focused.property-schema.baseline.property.order.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.order/ordinary
focused.property-schema.baseline.property.outline-color.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.outline-color/important
focused.property-schema.baseline.property.outline-color.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.outline-color/ordinary
focused.property-schema.baseline.property.outline-style.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.outline-style/important
focused.property-schema.baseline.property.outline-style.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.outline-style/ordinary
focused.property-schema.baseline.property.outline-width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.outline-width/important
focused.property-schema.baseline.property.outline-width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.outline-width/ordinary
focused.property-schema.baseline.property.outline.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.outline/important
focused.property-schema.baseline.property.outline.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.outline/ordinary
focused.property-schema.baseline.property.overflow-wrap.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.overflow-wrap/important
focused.property-schema.baseline.property.overflow-wrap.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.overflow-wrap/ordinary
focused.property-schema.baseline.property.overflow-x.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.overflow-x/important
focused.property-schema.baseline.property.overflow-x.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.overflow-x/ordinary
focused.property-schema.baseline.property.overflow-y.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.overflow-y/important
focused.property-schema.baseline.property.overflow-y.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.overflow-y/ordinary
focused.property-schema.baseline.property.overflow.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.overflow/important
focused.property-schema.baseline.property.overflow.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.overflow/ordinary
focused.property-schema.baseline.property.padding-bottom.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding-bottom/important
focused.property-schema.baseline.property.padding-bottom.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding-bottom/ordinary
focused.property-schema.baseline.property.padding-left.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding-left/important
focused.property-schema.baseline.property.padding-left.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding-left/ordinary
focused.property-schema.baseline.property.padding-right.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding-right/important
focused.property-schema.baseline.property.padding-right.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding-right/ordinary
focused.property-schema.baseline.property.padding-top.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding-top/important
focused.property-schema.baseline.property.padding-top.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding-top/ordinary
focused.property-schema.baseline.property.padding.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding/important
focused.property-schema.baseline.property.padding.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.padding/ordinary
focused.property-schema.baseline.property.place-content.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.place-content/important
focused.property-schema.baseline.property.place-content.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.place-content/ordinary
focused.property-schema.baseline.property.place-items.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.place-items/important
focused.property-schema.baseline.property.place-items.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.place-items/ordinary
focused.property-schema.baseline.property.place-self.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.place-self/important
focused.property-schema.baseline.property.place-self.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.place-self/ordinary
focused.property-schema.baseline.property.pointer-events.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.pointer-events/important
focused.property-schema.baseline.property.pointer-events.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.pointer-events/ordinary
focused.property-schema.baseline.property.position.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.position/important
focused.property-schema.baseline.property.position.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.position/ordinary
focused.property-schema.baseline.property.right.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.right/important
focused.property-schema.baseline.property.right.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.right/ordinary
focused.property-schema.baseline.property.rotate.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.rotate/important
focused.property-schema.baseline.property.rotate.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.rotate/ordinary
focused.property-schema.baseline.property.row-gap.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.row-gap/important
focused.property-schema.baseline.property.row-gap.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.row-gap/ordinary
focused.property-schema.baseline.property.scale.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.scale/important
focused.property-schema.baseline.property.scale.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.scale/ordinary
focused.property-schema.baseline.property.scrollbar-width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.scrollbar-width/important
focused.property-schema.baseline.property.scrollbar-width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.scrollbar-width/ordinary
focused.property-schema.baseline.property.text-align-last.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-align-last/important
focused.property-schema.baseline.property.text-align-last.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-align-last/ordinary
focused.property-schema.baseline.property.text-align.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-align/important
focused.property-schema.baseline.property.text-align.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-align/ordinary
focused.property-schema.baseline.property.text-decoration-color.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration-color/important
focused.property-schema.baseline.property.text-decoration-color.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration-color/ordinary
focused.property-schema.baseline.property.text-decoration-line.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration-line/important
focused.property-schema.baseline.property.text-decoration-line.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration-line/ordinary
focused.property-schema.baseline.property.text-decoration-style.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration-style/important
focused.property-schema.baseline.property.text-decoration-style.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration-style/ordinary
focused.property-schema.baseline.property.text-decoration-thickness.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration-thickness/important
focused.property-schema.baseline.property.text-decoration-thickness.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration-thickness/ordinary
focused.property-schema.baseline.property.text-decoration.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration/important
focused.property-schema.baseline.property.text-decoration.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-decoration/ordinary
focused.property-schema.baseline.property.text-indent.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-indent/important
focused.property-schema.baseline.property.text-indent.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-indent/ordinary
focused.property-schema.baseline.property.text-overflow.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-overflow/important
focused.property-schema.baseline.property.text-overflow.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-overflow/ordinary
focused.property-schema.baseline.property.text-transform.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-transform/important
focused.property-schema.baseline.property.text-transform.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-transform/ordinary
focused.property-schema.baseline.property.text-wrap.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-wrap/important
focused.property-schema.baseline.property.text-wrap.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.text-wrap/ordinary
focused.property-schema.baseline.property.top.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.top/important
focused.property-schema.baseline.property.top.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.top/ordinary
focused.property-schema.baseline.property.transform-origin.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transform-origin/important
focused.property-schema.baseline.property.transform-origin.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transform-origin/ordinary
focused.property-schema.baseline.property.transform.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transform/important
focused.property-schema.baseline.property.transform.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transform/ordinary
focused.property-schema.baseline.property.transition-delay.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition-delay/important
focused.property-schema.baseline.property.transition-delay.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition-delay/ordinary
focused.property-schema.baseline.property.transition-duration.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition-duration/important
focused.property-schema.baseline.property.transition-duration.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition-duration/ordinary
focused.property-schema.baseline.property.transition-property.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition-property/important
focused.property-schema.baseline.property.transition-property.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition-property/ordinary
focused.property-schema.baseline.property.transition-timing-function.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition-timing-function/important
focused.property-schema.baseline.property.transition-timing-function.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition-timing-function/ordinary
focused.property-schema.baseline.property.transition.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition/important
focused.property-schema.baseline.property.transition.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.transition/ordinary
focused.property-schema.baseline.property.translate.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.translate/important
focused.property-schema.baseline.property.translate.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.translate/ordinary
focused.property-schema.baseline.property.user-select.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.user-select/important
focused.property-schema.baseline.property.user-select.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.user-select/ordinary
focused.property-schema.baseline.property.vertical-align.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.vertical-align/important
focused.property-schema.baseline.property.vertical-align.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.vertical-align/ordinary
focused.property-schema.baseline.property.visibility.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.visibility/important
focused.property-schema.baseline.property.visibility.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.visibility/ordinary
focused.property-schema.baseline.property.white-space.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.white-space/important
focused.property-schema.baseline.property.white-space.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.white-space/ordinary
focused.property-schema.baseline.property.width.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.width/important
focused.property-schema.baseline.property.width.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.width/ordinary
focused.property-schema.baseline.property.word-break.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.word-break/important
focused.property-schema.baseline.property.word-break.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.word-break/ordinary
focused.property-schema.baseline.property.writing-mode.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.writing-mode/important
focused.property-schema.baseline.property.writing-mode.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.writing-mode/ordinary
focused.property-schema.baseline.property.z-index.important	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.z-index/important
focused.property-schema.baseline.property.z-index.ordinary	property_schema::property_schema_parser_identity_matches_every_frozen_name/baseline.property.z-index/ordinary
focused.public.actions.drop-at-rule	public_surface::public_surface_emits_all_ten_recovery_actions/drop-at-rule
focused.public.actions.drop-declaration	public_surface::public_surface_emits_all_ten_recovery_actions/drop-declaration
focused.public.actions.drop-descriptor	public_surface::public_surface_emits_all_ten_recovery_actions/drop-descriptor
focused.public.actions.drop-keyframe-block	public_surface::public_surface_emits_all_ten_recovery_actions/drop-keyframe-block
focused.public.actions.drop-qualified-rule	public_surface::public_surface_emits_all_ten_recovery_actions/drop-qualified-rule
focused.public.actions.drop-selector-list-item	public_surface::public_surface_emits_all_ten_recovery_actions/drop-selector-list-item
focused.public.actions.ignore-legacy	public_surface::public_surface_emits_all_ten_recovery_actions/ignore-legacy
focused.public.actions.implicit-closure	public_surface::public_surface_emits_all_ten_recovery_actions/implicit-closure
focused.public.actions.replace-media	public_surface::public_surface_emits_all_ten_recovery_actions/replace-media
focused.public.actions.stop-at-limit	public_surface::public_surface_emits_all_ten_recovery_actions/stop-at-limit
focused.public.non-bmp	public_surface::public_surface_non_bmp_coordinates_are_byte_line_and_utf16_based
focused.public.sheet.clean	public_surface::public_surface_sheet_reports_expose_retained_syntax_and_structured_recovery/clean
focused.public.sheet.recovered	public_surface::public_surface_sheet_reports_expose_retained_syntax_and_structured_recovery/recovered
focused.public.strict.style	public_surface::public_surface_enabled_validators_accept_clean_reports_and_preserve_failures
focused.public.style.authored	public_surface::public_surface_style_attributes_preserve_importance_custom_and_substitution_syntax
focused.source-coordinates.00	source_coordinates::source_public_nodes_expose_zero_based_byte_line_and_utf16_coordinates/0
focused.source-coordinates.01	source_coordinates::source_public_nodes_expose_zero_based_byte_line_and_utf16_coordinates/1
focused.source-coordinates.02	source_coordinates::source_public_nodes_expose_zero_based_byte_line_and_utf16_coordinates/2
focused.source-coordinates.03	source_coordinates::source_public_nodes_expose_zero_based_byte_line_and_utf16_coordinates/3
focused.source-coordinates.04	source_coordinates::source_public_nodes_expose_zero_based_byte_line_and_utf16_coordinates/4
focused.source-coordinates.05	source_coordinates::source_public_nodes_expose_zero_based_byte_line_and_utf16_coordinates/5
focused.specialized.balanced-media	specialized_list_recovery::balanced-media
focused.specialized.clean-media	specialized_list_recovery::clean-media
focused.specialized.delegation	specialized_list_recovery::delegation
focused.specialized.empty-media	specialized_list_recovery::empty-media
focused.specialized.forgiving-balanced	specialized_list_recovery::specialized_list_forgiving_recovery_stops_at_balanced_nested_commas
focused.specialized.forgiving-empty	specialized_list_recovery::specialized_list_empty_forgiving_member_uses_its_delimiting_comma_span
focused.specialized.forgiving.00	specialized_list_recovery::specialized_list_forgiving_selector_members_drop_independently_in_authored_order/0
focused.specialized.forgiving.01	specialized_list_recovery::specialized_list_forgiving_selector_members_drop_independently_in_authored_order/1
focused.specialized.forgiving.02	specialized_list_recovery::specialized_list_forgiving_selector_members_drop_independently_in_authored_order/2
focused.specialized.forgiving.03	specialized_list_recovery::specialized_list_forgiving_selector_members_drop_independently_in_authored_order/3
focused.specialized.import	specialized_list_recovery::import
focused.specialized.media-position	specialized_list_recovery::media-position
focused.specialized.media.00	specialized_list_recovery::specialized_list_media_members_become_never_in_authored_order/0
focused.specialized.media.01	specialized_list_recovery::specialized_list_media_members_become_never_in_authored_order/1
focused.specialized.media.02	specialized_list_recovery::specialized_list_media_members_become_never_in_authored_order/2
focused.specialized.nested	specialized_list_recovery::nested
focused.specialized.non-bmp	specialized_list_recovery::non-bmp
focused.specialized.repeated-media	specialized_list_recovery::repeated-media
focused.specialized.repeated-selector	specialized_list_recovery::repeated-selector
focused.specialized.scoped	specialized_list_recovery::scoped
focused.specialized.unforgiving.00	specialized_list_recovery::specialized_list_not_has_nth_and_ordinary_selector_lists_remain_unforgiving/0
focused.specialized.unforgiving.01	specialized_list_recovery::specialized_list_not_has_nth_and_ordinary_selector_lists_remain_unforgiving/1
focused.specialized.unforgiving.02	specialized_list_recovery::specialized_list_not_has_nth_and_ordinary_selector_lists_remain_unforgiving/2
focused.specialized.unforgiving.03	specialized_list_recovery::specialized_list_not_has_nth_and_ordinary_selector_lists_remain_unforgiving/3
focused.structural.component.curly.255	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/curly/255
focused.structural.component.curly.256	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/curly/256
focused.structural.component.curly.257	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/curly/257
focused.structural.component.function.255	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/function/255
focused.structural.component.function.256	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/function/256
focused.structural.component.function.257	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/function/257
focused.structural.component.paren.255	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/paren/255
focused.structural.component.paren.256	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/paren/256
focused.structural.component.paren.257	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/paren/257
focused.structural.component.square.255	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/square/255
focused.structural.component.square.256	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/square/256
focused.structural.component.square.257	structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/square/257
focused.structural.layers.255	structural_recovery_adversarial::structural_recovery_accepts_256_rule_blocks_and_drops_only_level_257/255
focused.structural.layers.256	structural_recovery_adversarial::structural_recovery_accepts_256_rule_blocks_and_drops_only_level_257/256
focused.structural.layers.257	structural_recovery_adversarial::structural_recovery_accepts_256_rule_blocks_and_drops_only_level_257/257
focused.structural.layers.eof-257	structural_recovery_adversarial::structural_recovery_nesting_limit_at_eof_spans_remaining_bounded_unit
focused.structural.misc.00	structural_recovery_adversarial::misc/0
focused.structural.misc.01	structural_recovery_adversarial::misc/1
focused.structural.misc.02	structural_recovery_adversarial::misc/2
focused.structural.misc.03	structural_recovery_adversarial::misc/3
focused.structural.misc.04	structural_recovery_adversarial::misc/4
focused.structural.misc.05	structural_recovery_adversarial::misc/5
focused.structural.misc.06	structural_recovery_adversarial::misc/6
focused.structural.misc.07	structural_recovery_adversarial::misc/7
focused.structured-errors.00	structured_errors::case/0
focused.structured-errors.01	structured_errors::case/1
focused.structured-errors.02	structured_errors::case/2
focused.structured-errors.03	structured_errors::case/3
focused.structured-errors.04	structured_errors::case/4
focused.structured-errors.05	structured_errors::case/5
focused.structured-errors.06	structured_errors::case/6
focused.structured-errors.07	structured_errors::case/7
focused.structured-errors.08	structured_errors::case/8
focused.structured-errors.09	structured_errors::case/9
focused.structured-errors.10	structured_errors::case/10
focused.structured-errors.11	structured_errors::case/11
focused.structured-errors.12	structured_errors::case/12
focused.structured-errors.13	structured_errors::case/13
focused.structured-errors.14	structured_errors::case/14
focused.structured-errors.15	structured_errors::case/15
focused.structured-errors.16	structured_errors::case/16
focused.structured-errors.17	structured_errors::case/17
focused.structured-errors.18	structured_errors::case/18
focused.style-attribute.bad-url	style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/bad-url
focused.style-attribute.clean.00	style_attribute_recovery::style_attribute_empty_trivia_and_optional_final_semicolon_are_clean/0
focused.style-attribute.clean.01	style_attribute_recovery::style_attribute_empty_trivia_and_optional_final_semicolon_are_clean/1
focused.style-attribute.clean.02	style_attribute_recovery::style_attribute_empty_trivia_and_optional_final_semicolon_are_clean/2
focused.style-attribute.clean.03	style_attribute_recovery::style_attribute_empty_trivia_and_optional_final_semicolon_are_clean/3
focused.style-attribute.closers	style_attribute_recovery::style_attribute_block_at_rules_and_malformed_closers_drop_without_hiding_later_values
focused.style-attribute.depth.255	style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/255
focused.style-attribute.depth.256	style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/256
focused.style-attribute.depth.257	style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/257
focused.style-attribute.error-class.00	style_attribute_recovery::style_attribute_declaration_error_classes_match_style_blocks_modulo_wrapper_offset/0
focused.style-attribute.error-class.01	style_attribute_recovery::style_attribute_declaration_error_classes_match_style_blocks_modulo_wrapper_offset/1
focused.style-attribute.error-class.02	style_attribute_recovery::style_attribute_declaration_error_classes_match_style_blocks_modulo_wrapper_offset/2
focused.style-attribute.error-class.03	style_attribute_recovery::style_attribute_declaration_error_classes_match_style_blocks_modulo_wrapper_offset/3
focused.style-attribute.error-class.04	style_attribute_recovery::style_attribute_declaration_error_classes_match_style_blocks_modulo_wrapper_offset/4
focused.style-attribute.error-class.05	style_attribute_recovery::style_attribute_declaration_error_classes_match_style_blocks_modulo_wrapper_offset/5
focused.style-attribute.error-class.06	style_attribute_recovery::style_attribute_declaration_error_classes_match_style_blocks_modulo_wrapper_offset/6
focused.style-attribute.implicit	style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/implicit
focused.style-attribute.non-declarations	style_attribute_recovery::style_attribute_non_declaration_units_drop_independently_in_source_order
focused.style-attribute.parity	style_attribute_recovery::style_attribute_ordinary_custom_global_substitution_and_importance_match_style_blocks
focused.stylesheet-recovery.00	stylesheet_recovery::case/0
focused.stylesheet-recovery.01	stylesheet_recovery::case/1
focused.stylesheet-recovery.02	stylesheet_recovery::case/2
focused.stylesheet-recovery.03	stylesheet_recovery::case/3
focused.stylesheet-recovery.04	stylesheet_recovery::case/4
focused.stylesheet-recovery.05	stylesheet_recovery::case/5
focused.stylesheet-recovery.06	stylesheet_recovery::case/6
focused.stylesheet-recovery.07	stylesheet_recovery::case/7
focused.stylesheet-recovery.08	stylesheet_recovery::case/8
focused.stylesheet-recovery.09	stylesheet_recovery::case/9
focused.stylesheet-recovery.10	stylesheet_recovery::case/10
focused.stylesheet-recovery.11	stylesheet_recovery::case/11
focused.stylesheet-recovery.12	stylesheet_recovery::case/12
focused.stylesheet-recovery.13	stylesheet_recovery::case/13
focused.stylesheet-recovery.14	stylesheet_recovery::case/14
focused.stylesheet-recovery.15	stylesheet_recovery::case/15
focused.stylesheet-recovery.16	stylesheet_recovery::case/16
focused.stylesheet-recovery.17	stylesheet_recovery::case/17
focused.stylesheet-recovery.18	stylesheet_recovery::case/18
focused.stylesheet-recovery.19	stylesheet_recovery::case/19
focused.stylesheet-recovery.20	stylesheet_recovery::case/20
focused.stylesheet-recovery.21	stylesheet_recovery::case/21
focused.stylesheet-recovery.22	stylesheet_recovery::case/22
"#;

pub fn focused_cases() -> Vec<Case> {
    use EntryPoint::{Sheet, Style};
    let mut cases = Vec::new();
    macro_rules! c {
        ($id:literal, $owner:literal, $entry:ident, $input:expr) => {
            cases.push(case(concat!("focused.", $id), $owner, $entry, $input));
        };
    }
    // Public front doors and the app-strict-specific consumer cases.
    c!(
        "public.sheet.clean",
        "public_surface::public_surface_sheet_reports_expose_retained_syntax_and_structured_recovery/clean",
        Sheet,
        ".clean { color: red; }"
    );
    c!(
        "public.sheet.recovered",
        "public_surface::public_surface_sheet_reports_expose_retained_syntax_and_structured_recovery/recovered",
        Sheet,
        ".before { color: red; } @unknown x; .after { color: blue; }"
    );
    c!(
        "public.style.authored",
        "public_surface::public_surface_style_attributes_preserve_importance_custom_and_substitution_syntax",
        Style,
        "color: red; --Theme: RGB(1, 2, var(--fallback)); width: var(--size, 2px) !important; mystery: 1"
    );
    c!(
        "public.non-bmp",
        "public_surface::public_surface_non_bmp_coordinates_are_byte_line_and_utf16_based",
        Sheet,
        ".😀 { mystery: 1; color: red; }"
    );
    for (suffix, input) in [
        ("drop-declaration", ".x { mystery: 1; color: red; }"),
        (
            "drop-descriptor",
            "@font-face { font-family: Demo; src: url(demo); mystery: 1; }",
        ),
        (
            "drop-qualified-rule",
            "??? { color: red; } .after { color: blue; }",
        ),
        ("drop-at-rule", "@unknown value;"),
        (
            "drop-keyframe-block",
            "@keyframes fade { fn(a) { opacity: .5; } to { opacity: 1; } }",
        ),
        ("drop-selector-list-item", ":is(.kept,???) { color: red; }"),
        ("replace-media", "@media screen, ??? { .x { color: red; } }"),
        ("implicit-closure", ".x { color: red;"),
        ("ignore-legacy", "<!-- .x { color: red; }"),
    ] {
        cases.push(case(
            format!("focused.public.actions.{suffix}"),
            format!("public_surface::public_surface_emits_all_ten_recovery_actions/{suffix}"),
            Sheet,
            input,
        ));
    }
    let over_limit = format!(
        "{}{}{} {{ color: red; }}",
        ":is(".repeat(257),
        ".leaf",
        ")".repeat(257)
    );
    cases.push(case(
        "focused.public.actions.stop-at-limit",
        "public_surface::public_surface_emits_all_ten_recovery_actions/stop-at-limit",
        Sheet,
        over_limit,
    ));
    c!(
        "public.strict.style",
        "public_surface::public_surface_enabled_validators_accept_clean_reports_and_preserve_failures",
        Style,
        "color: red; mystery: 1"
    );
    // Focused style-attribute recovery table and boundary families.
    for (index, input) in ["", " \t/**/\n", "color: red", "color: red;"]
        .into_iter()
        .enumerate()
    {
        cases.push(case(format!("focused.style-attribute.clean.{index:02}"), format!("style_attribute_recovery::style_attribute_empty_trivia_and_optional_final_semicolon_are_clean/{index}"), Style, input));
    }
    c!(
        "style-attribute.parity",
        "style_attribute_recovery::style_attribute_ordinary_custom_global_substitution_and_importance_match_style_blocks",
        Style,
        "width: 2px; --Theme: ready; color: inherit; height: var(--h, 8px) !important"
    );
    for (index, unit) in [
        "mystery: 1;",
        "width: nope;",
        "width: 2px !oops;",
        "color: #ggg;",
        "--bad name: 1px;",
        "--x: inherit 1px;",
        "broken;",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.style-attribute.error-class.{index:02}"), format!("style_attribute_recovery::style_attribute_declaration_error_classes_match_style_blocks_modulo_wrapper_offset/{index}"), Style, format!("color: red; {unit} height: 3px;")));
    }
    c!(
        "style-attribute.non-declarations",
        "style_attribute_recovery::style_attribute_non_declaration_units_drop_independently_in_source_order",
        Style,
        "@unknown x; color: red; .nested { width: 1px; } opacity: 1; broken; height: 2px; ,; --kept: yes;"
    );
    c!(
        "style-attribute.closers",
        "style_attribute_recovery::style_attribute_block_at_rules_and_malformed_closers_drop_without_hiding_later_values",
        Style,
        "@unknown screen { color: red; } } color: red; ) width: 2px; ] height: 3px;"
    );
    c!(
        "style-attribute.implicit",
        "style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/implicit",
        Style,
        "--value: fn([x"
    );
    c!(
        "style-attribute.bad-url",
        "style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/bad-url",
        Style,
        "background-image: url(bad url"
    );
    for depth in [255_usize, 256, 257] {
        cases.push(case(format!("focused.style-attribute.depth.{depth}"), format!("style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/{depth}"), Style, format!("--deep: {}x{}", "f(".repeat(depth), ")".repeat(depth))));
    }
    // Specialized list-recovery concrete tables.
    for (index, input) in [
        ":is(???,.a,.b) { color: red; }",
        ":is(.a,???,.b) { color: red; }",
        ":is(.a,.b,???) { color: red; }",
        ":where(???) { color: red; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.specialized.forgiving.{index:02}"), format!("specialized_list_recovery::specialized_list_forgiving_selector_members_drop_independently_in_authored_order/{index}"), Sheet, input));
    }
    c!(
        "specialized.forgiving-empty",
        "specialized_list_recovery::specialized_list_empty_forgiving_member_uses_its_delimiting_comma_span",
        Sheet,
        ":is(.a,,.b) { color: red; }"
    );
    c!(
        "specialized.forgiving-balanced",
        "specialized_list_recovery::specialized_list_forgiving_recovery_stops_at_balanced_nested_commas",
        Sheet,
        ":is(???(a,b),.ok) { color: red; }"
    );
    for (index, input) in [
        ":not(.a,???,.b) { color: red; }",
        ":has(.a,???,.b) { color: red; }",
        ":nth-child(2n of .a,???,.b) { color: red; }",
        ".a,???,.b { color: red; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.specialized.unforgiving.{index:02}"), format!("specialized_list_recovery::specialized_list_not_has_nth_and_ordinary_selector_lists_remain_unforgiving/{index}"), Sheet, input));
    }
    for (index, input) in [
        "@media ???,screen,print { .x { color: red; } }",
        "@media screen,???,print { .x { color: red; } }",
        "@media screen,print,??? { .x { color: red; } }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.specialized.media.{index:02}"), format!("specialized_list_recovery::specialized_list_media_members_become_never_in_authored_order/{index}"), Sheet, input));
    }
    for (id, input) in [
        ("repeated-selector", ":is(???,.ok,???) { color: red; }"),
        (
            "repeated-media",
            "@media ???,screen,,print,??? { .x { color: red; } }",
        ),
        ("empty-media", "@media screen,,print { .x { color: red; } }"),
        (
            "balanced-media",
            "@media ???(a,b),screen { .x { color: red; } }",
        ),
        (
            "media-position",
            "@media screen,(unknown: yes),print { .x { color: red; } }",
        ),
        ("import", "@import \"theme.css\" ???,screen;"),
        ("nested", ".parent { &:is(.ok,???) { color: red; } }"),
        (
            "scoped",
            "@scope (:is(.root,???)) { :where(.kept,???) { color: red; } @media ???,screen {} }",
        ),
        (
            "non-bmp",
            "@media screen, /*😀*/ ???, print { .x { color: red; } }",
        ),
        (
            "clean-media",
            "@media screen,(width: 1px) { .x { color: red; } }",
        ),
        (
            "delegation",
            "@media /*😀*/ not (width: 1px), only screen, ??? { .x { color: red; } }",
        ),
    ] {
        cases.push(case(
            format!("focused.specialized.{id}"),
            format!("specialized_list_recovery::{id}"),
            Sheet,
            input,
        ));
    }
    // App-strict public cases have their own feature identity while all ordinary cases also run under the feature.
    for (id, entry, input) in [
        ("clean-sheet", Sheet, ".x { color: red; }"),
        ("recovered-sheet", Sheet, ".x { mystery: 1; }"),
        ("clean-style", Style, "color: red"),
        ("recovered-style", Style, "mystery: 1"),
        (
            "multi-sheet",
            Sheet,
            "<!-- .x { mystery: 1; width: nope; } -->",
        ),
        ("multi-style", Style, "mystery: 1; width: nope; color: red"),
        ("never", Sheet, "@media screen, ??? { .x { color: red; } }"),
        ("implicit-sheet", Sheet, ".x { color: red;"),
        ("implicit-style", Style, "--x: f(value"),
    ] {
        cases.push(Case::new(
            format!("focused.app-strict.{id}"),
            format!("app_strict_parity::{id}"),
            entry,
            FeatureMode::AppStrict,
            input,
        ));
    }
    // Structured diagnostic public cases.
    for (index, input) in [
        "@not-a-css-rule;",
        "@supports (display: grid) {}",
        ".x { width: 1px; } @import 'x.css';",
        "@font-face nope {}",
        "@media screen;",
        "@font-face {\n}",
        "@layer 😀, theme {}",
        "??? { width: 1px; }",
        "@media (unknown: yes) { .x { width: 1px; } }",
        "x;",
        ".x { WIDHT: 1px; }",
        "@font-face { mystery: x; font-family: Test; src: url(test.woff2); }",
        "@font-face { font-family: One; font-family: Two; src: url(test.woff2); }",
        ".x { color: #ggg; }",
        ".x { width: 1px !oops; }",
        ".panel { WIDTH: n\\6f pe; }",
        ".panel { width:",
        ".panel { width 1px; }",
        "??? { width: 1px; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.structured-errors.{index:02}"),
            format!("structured_errors::case/{index}"),
            Sheet,
            input,
        ));
    }
    // Source-coordinate and coupled declaration cases.
    for (index, input) in [
        ".a { width: 1px; }",
        ".a {\n  width: 1px; }",
        ".a {\r\n  width: 1px; }",
        ".\\61 bc { width: 1px; }",
        ".😀2 { width: 1px; }",
        "/*a\nbc*/@import \"theme.css\";",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.source-coordinates.{index:02}"), format!("source_coordinates::source_public_nodes_expose_zero_based_byte_line_and_utf16_coordinates/{index}"), Sheet, input));
    }
    for (index, input) in [
        ".x { width: 1px; color: red; opacity: .5; }",
        ".x { width: 1px; width: inherit; width: var(--x); }",
        ".x { all: inherit; all: var(--x); }",
        ".x { --Theme: RGB(1, 2, var(--fallback)); }",
        ".x { WIDTH: 1px; }",
        ".x { color: red; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.coupled.{index:02}"),
            format!("coupled_declarations::case/{index}"),
            Sheet,
            input,
        ));
    }
    // Declaration importance and authored-value concrete boundaries.
    for (index, input) in [
        ".x { color: red; width: 1px !important; }",
        ".x { --x: bang!important; --y: f(!important) !important; }",
        ".x { width: 1px !oops; }",
        ".x { color: red; width: 1px !oops; height: 2px; }",
        "@keyframes fade { from { opacity: 0 } to { opacity: 1; } }",
        "@keyframes fade { from { opacity: 0 !important; } }",
        "@keyframes fade { from { --x: value !important; } }",
        "@font-face { font-family: Demo; src: url(demo.woff2); font-display: swap; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.importance.{index:02}"),
            format!("declaration_importance::case/{index}"),
            Sheet,
            input,
        ));
    }
    for (index, input) in [
        ".x { --empty:; --space:   ; }",
        ".x { --Theme: RGB(1, 2, var(--fallback)); }",
        ".x { --a: inherit; --b: initial; --c: unset; --d: revert; --e: revert-layer; }",
        ".x { width: 1px; height: var(--h, 8px); color: red; }",
        ".x { width: var(--x, red); }",
        ".x { width: var(--x, red; blue); }",
        ".x { width: var(--x, red ! blue); }",
        ".x { --x: fn([a;b!important]); }",
        ".x { --bad name: 1px; }",
        ".x { width: var(color); }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.authored-values.{index:02}"),
            format!("authored_declaration_values::case/{index}"),
            Sheet,
            input,
        ));
    }
    // Core stylesheet recovery concrete rows and finite encoding tables.
    for (index, input) in [
        "", "<!-- .before { color: red; } <!-- .after { color: blue; }", "--> .before { color: red; } --> .after { color: blue; }",
        "; .after { color: blue; }", ".before { color: red; } ; .after { color: blue; }", "} .after { color: blue; }",
        ".before { color: red; } } .after { color: blue; }", "; @charset \"UTF-8\"; .after { color: blue; }",
        "} @charset \"UTF-8\"; .after { color: blue; }", " \n/**/ <!-- --> \t",
        ".before { color: red; } @mystery one(foo; bar) { nested: {x; y}; } .after { color: blue; }",
        ".before { color: red; } @namespace svg url(http://example.test/a;b); .after { color: blue; }",
        ".before { color: red; } ??? { width: 1px; nested: fn({x;y}); } .after { color: blue; }",
        "\u{feff} /* leading */ @charset \"UTF-8\"; .after { color: blue; }",
        "@charset UTF-8; .after { color: blue; }", "@charset \"\"; .after { color: blue; }",
        "@charset \"UTF-8\" { ignored; } .after { color: blue; }", "@charset UTF-8;", "@charset \"\";",
        "@charset 'UTF-8';", "@charset /*comment*/ 'UTF-8';", "@charset \"UTF-8\"", "@charset \"UTF-8\" {}",
    ].into_iter().enumerate() { cases.push(case(format!("focused.stylesheet-recovery.{index:02}"), format!("stylesheet_recovery::case/{index}"), Sheet, input)); }
    // Initiative audit public-report stimuli (duplicates remain distinct owner identities).
    for (index, (entry, input)) in [
        (Sheet, ".before { color: red; } @unknown x; .middle { mystery: 1; width: 2px; } ??? { color: black; } .after { height: 3px; }"),
        (Style, "color: red; broken; width: 2px"), (Sheet, ".😀 { mystery: 1; width: bogus; color: red; } @unknown x;"),
        (Style, "--Theme: RGB(1, 2, var(--fallback)); width: var(--size, 2px) !important; color: red"),
        (Sheet, "@charset \"UTF-8\"; .x { color: red; }"), (Sheet, ".kept { color: red; } @unknown x;"),
        (Style, "--Theme: var(--fallback); width: 2px !important"), (Sheet, "@media screen, ??? { .x { color: red; } }"),
    ].into_iter().enumerate() { cases.push(case(format!("focused.initiative-audit.{index:02}"), format!("initiative_i01_audit::case/{index}"), entry, input)); }
    // Structural-limit loop families use explicit deterministic inputs at every tested boundary.
    for depth in [255_usize, 256, 257] {
        cases.push(case(format!("focused.structural.layers.{depth}"), format!("structural_recovery_adversarial::structural_recovery_accepts_256_rule_blocks_and_drops_only_level_257/{depth}"), Sheet, format!("{}{}.after{{color:red}}", "@layer{".repeat(depth), "}".repeat(depth))));
        for (kind, opener, closer) in [
            ("function", "f(", ")"),
            ("paren", "(", ")"),
            ("square", "[", "]"),
            ("curly", "{", "}"),
        ] {
            let component_depth = depth.saturating_sub(1);
            cases.push(case(format!("focused.structural.component.{kind}.{depth}"), format!("structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/{kind}/{depth}"), Sheet, format!(".target{{--x:{}x{};color:blue}}.after{{color:red}}", opener.repeat(component_depth), closer.repeat(component_depth))));
        }
    }
    cases.push(case("focused.structural.layers.eof-257", "structural_recovery_adversarial::structural_recovery_nesting_limit_at_eof_spans_remaining_bounded_unit", Sheet, "@layer{".repeat(257)));
    for (index, input) in [
        "/*({[({[*/.target{--x:\"})]})]\\\"tail\";--y:ident\\(\\[\\{;color:blue}.after{color:red}"
            .to_owned(),
        format!(
            "{} .x{{color:red}} {}",
            "@scope{".repeat(255),
            "}".repeat(255)
        ),
        format!(
            "{}color:red{}}}.after{{color:blue}}",
            ".x{".repeat(256),
            "}".repeat(255)
        ),
        "@keyframes fade { from { mystery: 1; } } .after { color: red; }".to_owned(),
        "".to_owned(),
        ";;;;;}}}}\0\u{fffd}".to_owned(),
        "🦊💥\n@unknown fn({a;b}); .after{color:red}".to_owned(),
        format!("{}{}", "@bad{};".repeat(256), ".after{color:red}"),
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.structural.misc.{index:02}"),
            format!("structural_recovery_adversarial::misc/{index}"),
            Sheet,
            input,
        ));
    }
    // App-strict generated depth rows.
    let mut structural = ".x{".repeat(257);
    structural.push_str("color:red;");
    structural.push_str(&"}".repeat(257));
    cases.push(Case::new(
        "focused.app-strict.structural-depth",
        "app_strict_parity::structural-depth",
        Sheet,
        FeatureMode::AppStrict,
        structural,
    ));
    let selector = format!(
        "{}{}{}{{color:red}}",
        ":is(".repeat(257),
        ".leaf",
        ")".repeat(257)
    );
    cases.push(Case::new(
        "focused.app-strict.selector-depth",
        "app_strict_parity::selector-depth",
        Sheet,
        FeatureMode::AppStrict,
        selector,
    ));
    let style = format!("--x:{}x{}", "f(".repeat(257), ")".repeat(257));
    cases.push(Case::new(
        "focused.app-strict.style-depth",
        "app_strict_parity::style-depth",
        Style,
        FeatureMode::AppStrict,
        style,
    ));
    // Nested structural recovery cases, including each generated group-context input.
    let failed_at = "@mystery fn({x; y}) { .lost { color: black; } }";
    for (kind, source) in [
        (
            "layer",
            format!(
                "@layer theme {{ .before {{ color: red; }} {failed_at} .after {{ color: blue; }} }}"
            ),
        ),
        (
            "media",
            format!(
                "@media screen {{ .before {{ color: red; }} {failed_at} .after {{ color: blue; }} }}"
            ),
        ),
        (
            "container",
            format!(
                "@container (width > 1px) {{ .before {{ color: red; }} {failed_at} .after {{ color: blue; }} }}"
            ),
        ),
    ] {
        cases.push(case(format!("focused.nested-structural.group.{kind}"), format!("nested_structural_recovery::nested_structural_group_contexts_retain_siblings_around_balanced_at_rule_failure/{kind}"), Sheet, source));
    }
    let failed_rule = ".bad:is(.one, .two), { color: black; }";
    for (kind, source) in [
        (
            "group",
            format!(
                "@media screen {{ .before {{ color: red; }} {failed_rule} .after {{ color: blue; }} }}"
            ),
        ),
        (
            "scope",
            format!(
                "@scope {{ .before {{ color: red; }} {failed_rule} .after {{ color: blue; }} }}"
            ),
        ),
        (
            "style",
            format!(
                ".host {{ color: red; & .before {{ width: 1px; }} {failed_rule} & .after {{ height: 2px; }} opacity: 1; }}"
            ),
        ),
    ] {
        cases.push(case(format!("focused.nested-structural.qualified.{kind}"), format!("nested_structural_recovery::nested_structural_qualified_failures_recover_in_group_scope_and_style_contexts/{kind}"), Sheet, source));
    }
    for (id, input) in [
        (
            "repeated",
            "@layer empty { @one fn({a; b}); .bad, { color: red; } } .after { color: blue; }",
        ),
        (
            "style-at-rule",
            ".host { color: red; & .before { width: 1px; } @mystery fn({x; y}); & .after { height: 2px; } opacity: 1; }",
        ),
        (
            "scope-at-rule",
            "@scope { .before { color: red; } @mystery fn({x; y}); .after { color: blue; } }",
        ),
        (
            "keyframes-recover",
            "@keyframes fade { from { opacity: 0; mystery: fn({a; b}); width: 1px; } 55 { opacity: .5; } to { opacity: 1; } } .after { color: blue; }",
        ),
        (
            "keyframes-child-loss",
            "@keyframes fade { from { mystery: 1; } } .after { color: blue; }",
        ),
        (
            "keyframes-balanced",
            "@keyframes fade { from { opacity: 0; } 25% { opacity: .25; @media fn(a, b) { width: 1px; } height: 2px; } fn(a, b) { opacity: .5; } to { opacity: 1; } }",
        ),
    ] {
        cases.push(case(
            format!("focused.nested-structural.{id}"),
            format!("nested_structural_recovery::{id}"),
            Sheet,
            input,
        ));
    }
    cases
}

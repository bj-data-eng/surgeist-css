use surgeist_css::{
    CssErrorCode, CssFeatureKind, CssSupportStatus, ErrorKind, feature_metadata, parse_sheet,
    parse_style_attribute,
};

#[derive(Clone, Copy)]
enum ExpectedSource {
    Url(&'static str),
    Repository(&'static str),
}

#[derive(Clone, Copy)]
enum Input {
    Sheet(&'static str),
    Style(&'static str),
}

struct ExpectedFeature {
    id: &'static str,
    kind: CssFeatureKind,
    spelling: &'static str,
    source: ExpectedSource,
    production: &'static str,
    status: CssSupportStatus,
    supported_subset: Option<&'static str>,
    unsupported_remainder: Option<&'static str>,
    recognized_code: Option<CssErrorCode>,
    positive: Option<Input>,
    negative: Option<(Input, CssErrorCode)>,
}

const BASELINE_RULE_SUBSET: &str =
    "The baseline parser spelling and the I01 recovery extensions are supported.";
const BASELINE_RULE_REMAINDER: &str =
    "Other valid forms of the cited rule production are outside the I01 subset.";
const DESCRIPTOR_SUBSET: &str =
    "The baseline descriptor grammar and the I01 recovery extensions are supported.";
const DESCRIPTOR_REMAINDER: &str =
    "Other valid forms of the cited descriptor production are outside the I01 subset.";
const SELECTOR_REMAINDER: &str =
    "Other valid forms of the cited Selectors production are outside the I01 subset.";
const QUERY_REMAINDER: &str =
    "Other valid forms of the cited query production are outside the I01 subset.";

const EXPECTED: &[ExpectedFeature] = &[
    ExpectedFeature {
        id: "baseline.rule.import",
        kind: CssFeatureKind::Rule,
        spelling: "@import",
        source: ExpectedSource::Repository("4b288d6:src/parser/mod.rs"),
        production: "@import rule",
        status: CssSupportStatus::Partial,
        supported_subset: Some(BASELINE_RULE_SUBSET),
        unsupported_remainder: Some(BASELINE_RULE_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet("@import \"theme.css\";")),
        negative: Some((
            Input::Sheet("@import url(theme.css) supports(display: grid);"),
            CssErrorCode::InvalidMediaQuery,
        )),
    },
    ExpectedFeature {
        id: "baseline.rule.layer-statement",
        kind: CssFeatureKind::Rule,
        spelling: "@layer ...;",
        source: ExpectedSource::Repository("4b288d6:src/parser/mod.rs"),
        production: "@layer statement rule",
        status: CssSupportStatus::Partial,
        supported_subset: Some(BASELINE_RULE_SUBSET),
        unsupported_remainder: Some(BASELINE_RULE_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet("@layer reset, theme;")),
        negative: Some((
            Input::Sheet("@layer initial;"),
            CssErrorCode::InvalidAtRulePrelude,
        )),
    },
    ExpectedFeature {
        id: "baseline.rule.layer-block",
        kind: CssFeatureKind::Rule,
        spelling: "@layer {...}",
        source: ExpectedSource::Repository("4b288d6:src/parser/mod.rs"),
        production: "@layer block rule",
        status: CssSupportStatus::Partial,
        supported_subset: Some(BASELINE_RULE_SUBSET),
        unsupported_remainder: Some(BASELINE_RULE_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet("@layer theme { .x { color: red; } }")),
        negative: Some((
            Input::Sheet("@layer first, second { .x { color: red; } }"),
            CssErrorCode::InvalidAtRuleBody,
        )),
    },
    ExpectedFeature {
        id: "baseline.rule.font-face",
        kind: CssFeatureKind::Rule,
        spelling: "@font-face",
        source: ExpectedSource::Repository("4b288d6:src/parser/font_face.rs"),
        production: "@font-face rule",
        status: CssSupportStatus::Partial,
        supported_subset: Some(BASELINE_RULE_SUBSET),
        unsupported_remainder: Some(BASELINE_RULE_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@font-face { font-family: Inter; src: url(inter.woff2); }",
        )),
        negative: Some((
            Input::Sheet("@font-face named { font-family: Inter; src: url(inter.woff2); }"),
            CssErrorCode::InvalidAtRulePrelude,
        )),
    },
    ExpectedFeature {
        id: "baseline.rule.keyframes",
        kind: CssFeatureKind::Rule,
        spelling: "@keyframes",
        source: ExpectedSource::Repository("4b288d6:src/parser/keyframes.rs"),
        production: "@keyframes rule",
        status: CssSupportStatus::Partial,
        supported_subset: Some(BASELINE_RULE_SUBSET),
        unsupported_remainder: Some(BASELINE_RULE_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@keyframes fade { from { opacity: 0; } to { opacity: 1; } }",
        )),
        negative: Some((
            Input::Sheet("@keyframes none { from { opacity: 0; } }"),
            CssErrorCode::InvalidAtRulePrelude,
        )),
    },
    ExpectedFeature {
        id: "baseline.rule.style",
        kind: CssFeatureKind::Rule,
        spelling: "style and nested qualified rules",
        source: ExpectedSource::Repository("4b288d6:src/parser/mod.rs"),
        production: "style rule",
        status: CssSupportStatus::Partial,
        supported_subset: Some(BASELINE_RULE_SUBSET),
        unsupported_remainder: Some(BASELINE_RULE_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(".x { color: red; }")),
        negative: Some((
            Input::Sheet("??? { color: red; }"),
            CssErrorCode::InvalidSelector,
        )),
    },
    ExpectedFeature {
        id: "baseline.rule.media",
        kind: CssFeatureKind::Rule,
        spelling: "@media",
        source: ExpectedSource::Repository("4b288d6:src/parser/mod.rs"),
        production: "@media rule",
        status: CssSupportStatus::Partial,
        supported_subset: Some(BASELINE_RULE_SUBSET),
        unsupported_remainder: Some(BASELINE_RULE_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet("@media screen { .x { color: red; } }")),
        negative: Some((
            Input::Sheet("@media (width: calc(1px)) { .x { color: red; } }"),
            CssErrorCode::InvalidMediaQuery,
        )),
    },
    ExpectedFeature {
        id: "baseline.rule.container",
        kind: CssFeatureKind::Rule,
        spelling: "@container",
        source: ExpectedSource::Repository("4b288d6:src/parser/mod.rs"),
        production: "@container rule",
        status: CssSupportStatus::Partial,
        supported_subset: Some(BASELINE_RULE_SUBSET),
        unsupported_remainder: Some(BASELINE_RULE_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@container (width > 1px) { .x { color: red; } }",
        )),
        negative: Some((
            Input::Sheet("@container scroll-state(stuck: top) { .x { color: red; } }"),
            CssErrorCode::InvalidAtRulePrelude,
        )),
    },
    ExpectedFeature {
        id: "baseline.rule.scope",
        kind: CssFeatureKind::Rule,
        spelling: "@scope",
        source: ExpectedSource::Repository("4b288d6:src/parser/mod.rs"),
        production: "@scope rule",
        status: CssSupportStatus::Partial,
        supported_subset: Some(BASELINE_RULE_SUBSET),
        unsupported_remainder: Some(BASELINE_RULE_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet("@scope (.card) { .title { color: red; } }")),
        negative: Some((
            Input::Sheet("@scope .card { .title { color: red; } }"),
            CssErrorCode::InvalidAtRulePrelude,
        )),
    },
    ExpectedFeature {
        id: "foundation.encoding.charset",
        kind: CssFeatureKind::Rule,
        spelling: "optional leading legacy @charset metadata",
        source: ExpectedSource::Url("https://www.w3.org/TR/2021/CRD-css-syntax-3-20211224/"),
        production: "CSS Syntax 3 section 3 input byte stream",
        status: CssSupportStatus::Complete,
        supported_subset: None,
        unsupported_remainder: None,
        recognized_code: None,
        positive: Some(Input::Sheet("@charset \"UTF-8\"; .x { color: red; }")),
        negative: None,
    },
    ExpectedFeature {
        id: "foundation.declaration-list.style-attribute",
        kind: CssFeatureKind::Declaration,
        spelling: "style-attribute declaration-list structure",
        source: ExpectedSource::Url("https://www.w3.org/TR/2013/REC-css-style-attr-20131107/"),
        production: "style attribute",
        status: CssSupportStatus::Complete,
        supported_subset: None,
        unsupported_remainder: None,
        recognized_code: None,
        positive: Some(Input::Style("color: red; width: 1px")),
        negative: None,
    },
    ExpectedFeature {
        id: "foundation.declaration.importance",
        kind: CssFeatureKind::Declaration,
        spelling: "terminal declaration !important annotation",
        source: ExpectedSource::Url("https://www.w3.org/TR/2022/CR-css-cascade-4-20220113/"),
        production: "important declaration",
        status: CssSupportStatus::Complete,
        supported_subset: None,
        unsupported_remainder: None,
        recognized_code: None,
        positive: Some(Input::Style("color: red !important")),
        negative: None,
    },
    ExpectedFeature {
        id: "baseline.declaration.custom-property",
        kind: CssFeatureKind::Declaration,
        spelling: "custom-property names and authored token streams",
        source: ExpectedSource::Repository("4b288d6:src/parser/variables.rs"),
        production: "custom-property declaration",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "Baseline custom-property names and authored token streams, including I01 recovery behavior, are supported.",
        ),
        unsupported_remainder: Some(
            "Other valid CSS Variables custom-property declaration forms are outside the I01 subset.",
        ),
        recognized_code: None,
        positive: Some(Input::Style("--theme: dark")),
        negative: Some((
            Input::Style("--x: inherit 1px"),
            CssErrorCode::InvalidQualifiedRule,
        )),
    },
    ExpectedFeature {
        id: "baseline.value.substitution-dependent",
        kind: CssFeatureKind::Value,
        spelling: "preserved known-property values containing substitution functions",
        source: ExpectedSource::Repository("4b288d6:src/parser/variables.rs"),
        production: "substitution-dependent declaration value",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "Known-property values with syntactically admissible var() references remain authored and symbolic.",
        ),
        unsupported_remainder: Some(
            "Other valid CSS Variables substitution functions and post-substitution forms are outside the I01 subset.",
        ),
        recognized_code: None,
        positive: Some(Input::Style("width: var(--width, 1px)")),
        negative: Some((
            Input::Style("width: var(color)"),
            CssErrorCode::InvalidPropertyValue,
        )),
    },
    ExpectedFeature {
        id: "later.rule.namespace",
        kind: CssFeatureKind::Rule,
        spelling: "@namespace",
        source: ExpectedSource::Url("https://www.w3.org/TR/css3-namespace/"),
        production: "namespace declaration",
        status: CssSupportStatus::RecognizedUnsupported,
        supported_subset: None,
        unsupported_remainder: None,
        recognized_code: Some(CssErrorCode::UnsupportedAtRule),
        positive: None,
        negative: Some((
            Input::Sheet("@namespace svg url(https://example.test/svg);"),
            CssErrorCode::UnsupportedAtRule,
        )),
    },
    ExpectedFeature {
        id: "later.rule.supports",
        kind: CssFeatureKind::Rule,
        spelling: "@supports",
        source: ExpectedSource::Url("https://www.w3.org/TR/css-conditional-3/"),
        production: "@supports rule",
        status: CssSupportStatus::RecognizedUnsupported,
        supported_subset: None,
        unsupported_remainder: None,
        recognized_code: Some(CssErrorCode::UnsupportedAtRule),
        positive: None,
        negative: Some((
            Input::Sheet("@supports (display: grid) { .x { color: red; } }"),
            CssErrorCode::UnsupportedAtRule,
        )),
    },
    ExpectedFeature {
        id: "later.rule.counter-style",
        kind: CssFeatureKind::Rule,
        spelling: "@counter-style",
        source: ExpectedSource::Url("https://www.w3.org/TR/css-counter-styles-3/"),
        production: "@counter-style rule",
        status: CssSupportStatus::RecognizedUnsupported,
        supported_subset: None,
        unsupported_remainder: None,
        recognized_code: Some(CssErrorCode::UnsupportedAtRule),
        positive: None,
        negative: Some((
            Input::Sheet("@counter-style thumbs { system: cyclic; symbols: 👍; suffix: \" \"; }"),
            CssErrorCode::UnsupportedAtRule,
        )),
    },
    ExpectedFeature {
        id: "later.rule.page",
        kind: CssFeatureKind::Rule,
        spelling: "@page",
        source: ExpectedSource::Url("https://www.w3.org/TR/CSS2/page.html"),
        production: "page rule",
        status: CssSupportStatus::RecognizedUnsupported,
        supported_subset: None,
        unsupported_remainder: None,
        recognized_code: Some(CssErrorCode::UnsupportedAtRule),
        positive: None,
        negative: Some((
            Input::Sheet("@page { margin: 1cm; }"),
            CssErrorCode::UnsupportedAtRule,
        )),
    },
    ExpectedFeature {
        id: "later.rule.font-feature-values",
        kind: CssFeatureKind::Rule,
        spelling: "@font-feature-values",
        source: ExpectedSource::Url("https://www.w3.org/TR/css-fonts-4/"),
        production: "@font-feature-values rule",
        status: CssSupportStatus::RecognizedUnsupported,
        supported_subset: None,
        unsupported_remainder: None,
        recognized_code: Some(CssErrorCode::UnsupportedAtRule),
        positive: None,
        negative: Some((
            Input::Sheet("@font-feature-values Font One { @styleset { nice: 1; } }"),
            CssErrorCode::UnsupportedAtRule,
        )),
    },
    ExpectedFeature {
        id: "baseline.descriptor.font-family",
        kind: CssFeatureKind::Descriptor,
        spelling: "font-family in @font-face",
        source: ExpectedSource::Repository("4b288d6:src/parser/font_face.rs"),
        production: "font-family descriptor",
        status: CssSupportStatus::Partial,
        supported_subset: Some(DESCRIPTOR_SUBSET),
        unsupported_remainder: Some(DESCRIPTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@font-face { font-family: Inter; src: url(inter.woff2); }",
        )),
        negative: Some((
            Input::Sheet("@font-face { font-family: serif, sans-serif; src: url(inter.woff2); }"),
            CssErrorCode::InvalidDescriptorValue,
        )),
    },
    ExpectedFeature {
        id: "baseline.descriptor.src",
        kind: CssFeatureKind::Descriptor,
        spelling: "src in @font-face",
        source: ExpectedSource::Repository("4b288d6:src/parser/font_face.rs"),
        production: "src descriptor",
        status: CssSupportStatus::Partial,
        supported_subset: Some(DESCRIPTOR_SUBSET),
        unsupported_remainder: Some(DESCRIPTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@font-face { font-family: Inter; src: url(inter.woff2) format(woff2); }",
        )),
        negative: Some((
            Input::Sheet("@font-face { font-family: Inter; src: url(inter.woff2) format(woff3); }"),
            CssErrorCode::InvalidDescriptorValue,
        )),
    },
    ExpectedFeature {
        id: "baseline.descriptor.font-weight",
        kind: CssFeatureKind::Descriptor,
        spelling: "font-weight in @font-face",
        source: ExpectedSource::Repository("4b288d6:src/parser/font_face.rs"),
        production: "font-weight descriptor",
        status: CssSupportStatus::Partial,
        supported_subset: Some(DESCRIPTOR_SUBSET),
        unsupported_remainder: Some(DESCRIPTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@font-face { font-family: Inter; src: url(inter.woff2); font-weight: 400 700; }",
        )),
        negative: Some((
            Input::Sheet(
                "@font-face { font-family: Inter; src: url(inter.woff2); font-weight: bolder; }",
            ),
            CssErrorCode::InvalidDescriptorValue,
        )),
    },
    ExpectedFeature {
        id: "baseline.descriptor.font-style",
        kind: CssFeatureKind::Descriptor,
        spelling: "font-style in @font-face",
        source: ExpectedSource::Repository("4b288d6:src/parser/font_face.rs"),
        production: "font-style descriptor",
        status: CssSupportStatus::Partial,
        supported_subset: Some(DESCRIPTOR_SUBSET),
        unsupported_remainder: Some(DESCRIPTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@font-face { font-family: Inter; src: url(inter.woff2); font-style: italic; }",
        )),
        negative: Some((
            Input::Sheet(
                "@font-face { font-family: Inter; src: url(inter.woff2); font-style: made-up; }",
            ),
            CssErrorCode::InvalidDescriptorValue,
        )),
    },
    ExpectedFeature {
        id: "baseline.descriptor.font-stretch",
        kind: CssFeatureKind::Descriptor,
        spelling: "font-stretch in @font-face",
        source: ExpectedSource::Repository("4b288d6:src/parser/font_face.rs"),
        production: "font-stretch descriptor",
        status: CssSupportStatus::Partial,
        supported_subset: Some(DESCRIPTOR_SUBSET),
        unsupported_remainder: Some(DESCRIPTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@font-face { font-family: Inter; src: url(inter.woff2); font-stretch: 75% 125%; }",
        )),
        negative: Some((
            Input::Sheet(
                "@font-face { font-family: Inter; src: url(inter.woff2); font-stretch: wide; }",
            ),
            CssErrorCode::InvalidDescriptorValue,
        )),
    },
    ExpectedFeature {
        id: "baseline.descriptor.font-display",
        kind: CssFeatureKind::Descriptor,
        spelling: "font-display in @font-face",
        source: ExpectedSource::Repository("4b288d6:src/parser/font_face.rs"),
        production: "font-display descriptor",
        status: CssSupportStatus::Partial,
        supported_subset: Some(DESCRIPTOR_SUBSET),
        unsupported_remainder: Some(DESCRIPTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@font-face { font-family: Inter; src: url(inter.woff2); font-display: swap; }",
        )),
        negative: Some((
            Input::Sheet(
                "@font-face { font-family: Inter; src: url(inter.woff2); font-display: made-up; }",
            ),
            CssErrorCode::InvalidDescriptorValue,
        )),
    },
    ExpectedFeature {
        id: "baseline.descriptor.unicode-range",
        kind: CssFeatureKind::Descriptor,
        spelling: "unicode-range in @font-face",
        source: ExpectedSource::Repository("4b288d6:src/parser/font_face.rs"),
        production: "unicode-range descriptor",
        status: CssSupportStatus::Partial,
        supported_subset: Some(DESCRIPTOR_SUBSET),
        unsupported_remainder: Some(DESCRIPTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@font-face { font-family: Inter; src: url(inter.woff2); unicode-range: U+0000-00FF; }",
        )),
        negative: Some((
            Input::Sheet(
                "@font-face { font-family: Inter; src: url(inter.woff2); unicode-range: U+110000-110001; }",
            ),
            CssErrorCode::InvalidDescriptorValue,
        )),
    },
    ExpectedFeature {
        id: "baseline.selector.complex",
        kind: CssFeatureKind::Selector,
        spelling: "type, universal, ID, class; presence and six valued attribute matchers; descendant, child, next-sibling, subsequent-sibling combinators",
        source: ExpectedSource::Repository("4b288d6:src/parser/selectors.rs"),
        production: "complex selector",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact baseline-recognized complex-selector spelling group is supported.",
        ),
        unsupported_remainder: Some(SELECTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "article#main.card[data-ready][lang|=\"en\"] > span + a ~ b { color: red; }",
        )),
        negative: Some((
            Input::Sheet("svg|a { color: red; }"),
            CssErrorCode::InvalidSelector,
        )),
    },
    ExpectedFeature {
        id: "baseline.selector.pseudo-class",
        kind: CssFeatureKind::Selector,
        spelling: ":root, :hover, :active, :focus, :disabled, :enabled, :checked, :first-child, :last-child, :only-child, :empty, :first-of-type, :last-of-type, :only-of-type",
        source: ExpectedSource::Repository("4b288d6:src/parser/selectors.rs"),
        production: "baseline pseudo-class selector",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact baseline-recognized pseudo-class spelling group is supported.",
        ),
        unsupported_remainder: Some(SELECTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(".button:hover { color: red; }")),
        negative: Some((
            Input::Sheet(".link:visited { color: red; }"),
            CssErrorCode::InvalidSelector,
        )),
    },
    ExpectedFeature {
        id: "baseline.selector.functional",
        kind: CssFeatureKind::Selector,
        spelling: ":nth-child(), :nth-last-child(), :nth-of-type(), :nth-last-of-type(), :not()",
        source: ExpectedSource::Repository("4b288d6:src/parser/selectors.rs"),
        production: "baseline functional pseudo-class selector",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact baseline-recognized functional pseudo-class spelling group is supported.",
        ),
        unsupported_remainder: Some(SELECTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(".item:nth-child(2n+1) { color: red; }")),
        negative: Some((
            Input::Sheet(".item:lang(en) { color: red; }"),
            CssErrorCode::InvalidSelector,
        )),
    },
    ExpectedFeature {
        id: "baseline.selector.extension-state",
        kind: CssFeatureKind::Selector,
        spelling: ":scope, :focus-visible, :focus-within, :required, :optional, :valid, :invalid, :placeholder-shown, :default, :indeterminate, :read-only, :read-write, :in-range, :out-of-range, :modal, :fullscreen, :popover-open",
        source: ExpectedSource::Repository("4b288d6:src/parser/selectors.rs"),
        production: "extension state pseudo-class selector",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact I01 extension-state pseudo-class spelling group is supported.",
        ),
        unsupported_remainder: Some(SELECTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(".button:focus-visible { color: red; }")),
        negative: Some((
            Input::Sheet(".target:target { color: red; }"),
            CssErrorCode::InvalidSelector,
        )),
    },
    ExpectedFeature {
        id: "baseline.selector.extension-functional",
        kind: CssFeatureKind::Selector,
        spelling: ":is(), :where(), complex :not(), :has(), and nth-child of lists",
        source: ExpectedSource::Repository("4b288d6:src/parser/selectors.rs"),
        production: "extension functional pseudo-class selector",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact I01 extension-functional pseudo-class spelling group is supported.",
        ),
        unsupported_remainder: Some(SELECTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            ".item:is(.primary, .secondary) { color: red; }",
        )),
        negative: Some((
            Input::Sheet(".item:has(:has(.nested)) { color: red; }"),
            CssErrorCode::InvalidSelector,
        )),
    },
    ExpectedFeature {
        id: "baseline.selector.attribute-case",
        kind: CssFeatureKind::Selector,
        spelling: "i and s attribute-selector modifiers",
        source: ExpectedSource::Repository("4b288d6:src/parser/selectors.rs"),
        production: "attribute-selector case-sensitivity modifier",
        status: CssSupportStatus::Partial,
        supported_subset: Some("The i and s attribute-selector case modifiers are supported."),
        unsupported_remainder: Some(SELECTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet("[data-kind=\"primary\" i] { color: red; }")),
        negative: Some((
            Input::Sheet("[data-kind=\"primary\" q] { color: red; }"),
            CssErrorCode::InvalidSelector,
        )),
    },
    ExpectedFeature {
        id: "baseline.selector.pseudo-element",
        kind: CssFeatureKind::Selector,
        spelling: "::before, ::after, ::marker, ::selection, ::backdrop, and generated ::marker sequences",
        source: ExpectedSource::Repository("4b288d6:src/parser/selectors.rs"),
        production: "pseudo-element selector",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact baseline-recognized pseudo-element spelling group is supported.",
        ),
        unsupported_remainder: Some(SELECTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(".item::before { content: \"x\"; }")),
        negative: Some((
            Input::Sheet(".item::first-line { color: red; }"),
            CssErrorCode::InvalidSelector,
        )),
    },
    ExpectedFeature {
        id: "baseline.selector.nesting",
        kind: CssFeatureKind::Selector,
        spelling: "nesting &, scoped selector anchors, and scoped relative selectors",
        source: ExpectedSource::Repository("4b288d6:src/parser/nesting.rs"),
        production: "nesting selector",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "Nesting &, scoped selector anchors, and scoped relative selectors are supported.",
        ),
        unsupported_remainder: Some(SELECTOR_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(".card { & > .title { color: red; } }")),
        negative: Some((
            Input::Sheet(".card { & || .title { color: red; } }"),
            CssErrorCode::InvalidSelector,
        )),
    },
    ExpectedFeature {
        id: "baseline.media.query-list",
        kind: CssFeatureKind::MediaQuery,
        spelling: "typed/condition query lists, not/only, and/or/not, range and colon forms, and malformed-member Never recovery",
        source: ExpectedSource::Repository("4b288d6:src/parser/queries.rs"),
        production: "media query list",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact baseline-recognized media query-list spelling group and malformed-member Never recovery are supported.",
        ),
        unsupported_remainder: Some(QUERY_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@media screen and (min-width: 1px), print { .x { color: red; } }",
        )),
        negative: Some((
            Input::Sheet("@media screen, ??? { .x { color: red; } }"),
            CssErrorCode::InvalidMediaQuery,
        )),
    },
    ExpectedFeature {
        id: "baseline.media.type",
        kind: CssFeatureKind::MediaQuery,
        spelling: "all, screen, print",
        source: ExpectedSource::Repository("4b288d6:src/parser/queries.rs"),
        production: "media type",
        status: CssSupportStatus::Partial,
        supported_subset: Some("The all, screen, and print media types are supported."),
        unsupported_remainder: Some(QUERY_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet("@media print { .x { color: red; } }")),
        negative: Some((
            Input::Sheet("@media speech { .x { color: red; } }"),
            CssErrorCode::InvalidMediaQuery,
        )),
    },
    ExpectedFeature {
        id: "baseline.media.range-feature",
        kind: CssFeatureKind::MediaQuery,
        spelling: "width, height, resolution, color, monochrome and their min-/max- names",
        source: ExpectedSource::Repository("4b288d6:src/parser/queries.rs"),
        production: "media range feature",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact baseline-recognized media range-feature spelling group is supported.",
        ),
        unsupported_remainder: Some(QUERY_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet("@media (width >= 1px) { .x { color: red; } }")),
        negative: Some((
            Input::Sheet("@media (device-width: 1px) { .x { color: red; } }"),
            CssErrorCode::InvalidMediaQuery,
        )),
    },
    ExpectedFeature {
        id: "baseline.media.discrete-feature",
        kind: CssFeatureKind::MediaQuery,
        spelling: "orientation, prefers-color-scheme, prefers-reduced-motion, prefers-reduced-transparency, prefers-contrast, forced-colors, hover, any-hover, pointer, any-pointer, display-mode",
        source: ExpectedSource::Repository("4b288d6:src/parser/queries.rs"),
        production: "media discrete feature",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact baseline-recognized media discrete-feature spelling group is supported.",
        ),
        unsupported_remainder: Some(QUERY_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@media (orientation: landscape) { .x { color: red; } }",
        )),
        negative: Some((
            Input::Sheet("@media (scripting: enabled) { .x { color: red; } }"),
            CssErrorCode::InvalidMediaQuery,
        )),
    },
    ExpectedFeature {
        id: "baseline.container.condition",
        kind: CssFeatureKind::ContainerQuery,
        spelling: "and/or/not, size features, and custom-property style existence/equality",
        source: ExpectedSource::Repository("4b288d6:src/parser/queries.rs"),
        production: "container condition",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact baseline-recognized container-condition spelling group is supported.",
        ),
        unsupported_remainder: Some(QUERY_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@container (width > 1px) and style(--theme) { .x { color: red; } }",
        )),
        negative: Some((
            Input::Sheet("@container style(color: red) { .x { color: red; } }"),
            CssErrorCode::InvalidAtRulePrelude,
        )),
    },
    ExpectedFeature {
        id: "baseline.container.size-feature",
        kind: CssFeatureKind::ContainerQuery,
        spelling: "width, height, inline-size, block-size, aspect-ratio, orientation and applicable min-/max- names",
        source: ExpectedSource::Repository("4b288d6:src/parser/queries.rs"),
        production: "container size feature",
        status: CssSupportStatus::Partial,
        supported_subset: Some(
            "The exact baseline-recognized container size-feature spelling group is supported.",
        ),
        unsupported_remainder: Some(QUERY_REMAINDER),
        recognized_code: None,
        positive: Some(Input::Sheet(
            "@container (inline-size > 1px) { .x { color: red; } }",
        )),
        negative: Some((
            Input::Sheet("@container (unknown-size > 1px) { .x { color: red; } }"),
            CssErrorCode::InvalidAtRulePrelude,
        )),
    },
];

#[test]
fn named_conformance_records_expose_declared_metadata() {
    for expected in EXPECTED {
        let actual = feature_metadata(expected.id).expect("expected catalog record");
        assert_eq!(actual.id().as_str(), expected.id);
        assert_eq!(actual.kind(), expected.kind, "{} kind", expected.id);
        assert_eq!(
            actual.spelling(),
            expected.spelling,
            "{} spelling",
            expected.id
        );
        assert_eq!(
            actual.production(),
            expected.production,
            "{} production",
            expected.id
        );
        assert_eq!(actual.status(), expected.status, "{} status", expected.id);
        assert_eq!(
            actual.supported_subset(),
            expected.supported_subset,
            "{} supported subset",
            expected.id
        );
        assert_eq!(
            actual.unsupported_remainder(),
            expected.unsupported_remainder,
            "{} unsupported remainder",
            expected.id
        );
        assert_eq!(
            actual.recognized_unsupported_code(),
            expected.recognized_code,
            "{} recognized code",
            expected.id
        );
        match expected.source {
            ExpectedSource::Url(url) => {
                assert_eq!(
                    actual.source().url(),
                    Some(url),
                    "{} source URL",
                    expected.id
                );
                assert_eq!(
                    actual.source().repository_provenance(),
                    None,
                    "{} repository source",
                    expected.id
                );
            }
            ExpectedSource::Repository(provenance) => {
                assert_eq!(actual.source().url(), None, "{} source URL", expected.id);
                assert_eq!(
                    actual.source().repository_provenance(),
                    Some(provenance),
                    "{} repository source",
                    expected.id
                );
            }
        }

        let partial = expected.status == CssSupportStatus::Partial;
        assert_eq!(
            actual.supported_subset().is_some(),
            partial,
            "{} subset status invariant",
            expected.id
        );
        assert_eq!(
            actual.unsupported_remainder().is_some(),
            partial,
            "{} remainder status invariant",
            expected.id
        );
        assert_eq!(
            actual.recognized_unsupported_code().is_some(),
            expected.status == CssSupportStatus::RecognizedUnsupported,
            "{} recognized status invariant",
            expected.id
        );
    }

    assert!(
        feature_metadata("BASELINE.RULE.IMPORT").is_none(),
        "lookup is exact"
    );
    assert!(
        feature_metadata("baseline.rule.import ").is_none(),
        "lookup does not trim"
    );
    assert!(feature_metadata("baseline.property.display").is_some());
    assert!(feature_metadata("").is_none());
}

#[test]
fn conformance_catalog_vectors_cover_each_supported_and_unsupported_boundary() {
    for expected in EXPECTED {
        match expected.status {
            CssSupportStatus::Complete | CssSupportStatus::Partial => {
                let input = expected
                    .positive
                    .expect("supported record needs a positive vector");
                let diagnostics = diagnostics(input);
                assert!(
                    diagnostics.is_empty(),
                    "{} positive vector produced {diagnostics:?}",
                    expected.id
                );
            }
            CssSupportStatus::RecognizedUnsupported => {
                assert!(
                    expected.positive.is_none(),
                    "recognized unsupported has no positive vector"
                );
            }
        }

        match expected.status {
            CssSupportStatus::Complete => assert!(expected.negative.is_none()),
            CssSupportStatus::Partial | CssSupportStatus::RecognizedUnsupported => {
                let (input, code) = expected
                    .negative
                    .expect("unsupported boundary needs a vector");
                let diagnostics = diagnostics(input);
                assert!(
                    !diagnostics.is_empty(),
                    "{} negative vector was accepted",
                    expected.id
                );
                assert_eq!(
                    diagnostics[0].0, code,
                    "{} negative vector root",
                    expected.id
                );
                if expected.status == CssSupportStatus::RecognizedUnsupported {
                    assert_eq!(
                        diagnostics[0].1,
                        Some(expected.id),
                        "{} diagnostic feature identity",
                        expected.id
                    );
                }
            }
        }
    }
}

fn diagnostics(input: Input) -> Vec<(CssErrorCode, Option<&'static str>)> {
    let diagnostics = match input {
        Input::Sheet(source) => parse_sheet(source).diagnostics().to_vec(),
        Input::Style(source) => parse_style_attribute(source).diagnostics().to_vec(),
    };
    diagnostics
        .iter()
        .map(|diagnostic| {
            let feature = match diagnostic.error().kind() {
                ErrorKind::UnsupportedAtRule(detail) => Some(detail.feature().as_str()),
                _ => None,
            };
            (diagnostic.error().code(), feature)
        })
        .collect()
}

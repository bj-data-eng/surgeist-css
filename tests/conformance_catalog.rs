use surgeist_css::{
    CssAngleCalculation, CssAngleUnit, CssCalculationType, CssDelayLiteral, CssErrorCode,
    CssExclusionReason, CssFeatureKind, CssFrequencyCalculation, CssFrequencyUnit,
    CssIntegerCalculation, CssKnownProperty, CssLengthCalculation, CssLengthDimension,
    CssLengthUnit, CssNumberCalculation, CssPercentageCalculation, CssRecoveryAction,
    CssResolution, CssResolutionUnit, CssSpecificationTier, CssSupportStatus, CssTimeCalculation,
    CssTimeUnit, ErrorKind, conformance_exclusion, conformance_exclusions, feature_metadata,
    parse_sheet, parse_style_attribute, specification_source, specification_sources,
};

#[derive(Clone, Copy)]
enum ExpectedSource {
    Id(&'static str),
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
const DIMENSION_SUBSET: &str =
    "Selected typed length, angle, time, frequency, and resolution dimensions are supported.";
const DIMENSION_REMAINDER: &str =
    "Other valid CSS dimension families remain for their owning later grammar cycles.";
const ANGLE_SUBSET: &str = "The public typed angle model and calculation root are supported.";
const ANGLE_REMAINDER: &str =
    "Angle property consumers remain for their owning later grammar cycles.";
const ANGLE_PERCENTAGE_SUBSET: &str =
    "The public typed angle and percentage calculation models are supported.";
const ANGLE_PERCENTAGE_REMAINDER: &str =
    "Angle-percentage property consumers remain for their owning later grammar cycles.";
const TIME_PERCENTAGE_SUBSET: &str =
    "The public typed time and percentage calculation models are supported.";
const TIME_PERCENTAGE_REMAINDER: &str =
    "Time-percentage property consumers remain for their owning later grammar cycles.";
const FREQUENCY_SUBSET: &str =
    "The public typed frequency model and calculation root are supported.";
const FREQUENCY_REMAINDER: &str =
    "Frequency property consumers remain for their owning later grammar cycles.";
const FREQUENCY_PERCENTAGE_SUBSET: &str =
    "The public typed frequency and percentage calculation models are supported.";
const FREQUENCY_PERCENTAGE_REMAINDER: &str =
    "Frequency-percentage property consumers remain for their owning later grammar cycles.";
const CALC_SUBSET: &str = "Typed sum, product, division, negation, grouping, and nested calc() trees are supported for the C03 roots and integrated property consumers.";
const CALC_REMAINDER: &str = "Angle, frequency, Media resolution, keyframe percentage, font-feature numeric, and C05 function-owned consumer integrations remain for their owning later cycles.";
const TIMING_SUBSET: &str = "The I01 shorthand components plus C03 duration, signed delay, iteration, and typed calculation syntax are supported.";
const TIMING_REMAINDER: &str = "C05 easing and function grammar closure remains unsupported.";

const EXPECTED: &[ExpectedFeature] = &[
    ExpectedFeature {
        id: "baseline.rule.import",
        kind: CssFeatureKind::Rule,
        spelling: "@import",
        source: ExpectedSource::Id("O-CASCADE4"),
        production: "#at-import",
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
        source: ExpectedSource::Id("R-CASCADE5"),
        production: "#layering",
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
        source: ExpectedSource::Id("R-CASCADE5"),
        production: "#layering",
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
        source: ExpectedSource::Id("O-FONTS3"),
        production: "#font-face-rule",
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
        source: ExpectedSource::Id("I-ANIMATIONS1"),
        production: "#keyframes",
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
        source: ExpectedSource::Id("O-SYNTAX3"),
        production: "#style-rules",
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
        source: ExpectedSource::Id("O-CONDITIONAL3"),
        production: "#at-media",
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
        source: ExpectedSource::Id("X-CONTAIN3"),
        production: "#container-rule",
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
        source: ExpectedSource::Id("X-CASCADE6"),
        production: "#scope-atrule",
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
        source: ExpectedSource::Id("O-SYNTAX3"),
        production: "#charset-rule",
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
        source: ExpectedSource::Id("O-STYLE-ATTR"),
        production: "#syntax",
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
        source: ExpectedSource::Id("O-CASCADE4"),
        production: "#importance",
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
        source: ExpectedSource::Id("O-VARIABLES1"),
        production: "#defining-variables,#syntax",
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
        source: ExpectedSource::Id("O-VARIABLES1"),
        production: "#using-variables",
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
        source: ExpectedSource::Id("O-NAMESPACES3"),
        production: "#declaration,#syntax",
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
        source: ExpectedSource::Id("O-CONDITIONAL3"),
        production: "#at-supports",
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
        source: ExpectedSource::Id("O-COUNTERSTYLES3"),
        production: "#the-counter-style-rule",
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
        source: ExpectedSource::Id("O-CSS2"),
        production: "page.html#page-box",
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
        source: ExpectedSource::Id("I-FONTS4"),
        production: "#font-feature-values-rule",
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
        source: ExpectedSource::Id("O-FONTS3"),
        production: "#font-family-desc",
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
        source: ExpectedSource::Id("O-FONTS3"),
        production: "#src-desc",
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
        source: ExpectedSource::Id("O-FONTS3"),
        production: "#font-prop-desc",
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
        source: ExpectedSource::Id("O-FONTS3"),
        production: "#font-prop-desc",
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
        source: ExpectedSource::Id("O-FONTS3"),
        production: "#font-prop-desc",
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
        source: ExpectedSource::Id("I-FONTS4"),
        production: "#font-display-desc",
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
        source: ExpectedSource::Id("O-FONTS3"),
        production: "#unicode-range-desc",
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
        source: ExpectedSource::Id("O-SELECTORS3"),
        production: "#type-selectors,#universal-selector,#attribute-representation,#attribute-substrings,#class-html,#id-selectors,#descendant-combinators,#child-combinators,#adjacent-sibling-combinators,#general-sibling-combinators",
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
        source: ExpectedSource::Id("O-SELECTORS3"),
        production: "#dynamic-pseudos,#UIstates,#structural-pseudos",
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
        source: ExpectedSource::Id("O-SELECTORS3"),
        production: "#structural-pseudos,#negation",
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
        source: ExpectedSource::Id("I-SELECTORS4"),
        production: "#useraction-pseudos,#input-pseudos,#resource-pseudos,#display-state-pseudos",
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
        source: ExpectedSource::Id("I-SELECTORS4"),
        production: "#matches,#zero-matches,#relational,#negation,#the-nth-child-pseudo",
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
        source: ExpectedSource::Id("I-SELECTORS4"),
        production: "#attribute-case",
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
        source: ExpectedSource::Id("I01-BASE-SELECTORS"),
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
        source: ExpectedSource::Id("I-NESTING1"),
        production: "#nest-selector",
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
        source: ExpectedSource::Id("I01-BASE-QUERIES"),
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
        source: ExpectedSource::Id("O-MEDIA3"),
        production: "#media1",
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
        source: ExpectedSource::Id("I01-BASE-QUERIES"),
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
        source: ExpectedSource::Id("I01-BASE-QUERIES"),
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
        source: ExpectedSource::Id("X-CONTAIN3"),
        production: "#container-rule",
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
        source: ExpectedSource::Id("X-CONTAIN3"),
        production: "#size-container",
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

fn assert_c03_value_metadata(
    id: &str,
    spelling: &str,
    production: &str,
    status: CssSupportStatus,
    subset: Option<&str>,
    remainder: Option<&str>,
) {
    let metadata = feature_metadata(id).unwrap_or_else(|| panic!("missing `{id}` metadata"));
    assert_eq!(metadata.id().as_str(), id);
    assert_eq!(metadata.kind(), CssFeatureKind::Value, "{id} kind");
    assert_eq!(metadata.spelling(), spelling, "{id} spelling");
    assert_eq!(metadata.source().id().as_str(), "O-VALUES3", "{id} source");
    assert_eq!(metadata.production(), production, "{id} production");
    assert_eq!(metadata.status(), status, "{id} status");
    assert_eq!(metadata.supported_subset(), subset, "{id} subset");
    assert_eq!(
        metadata.unsupported_remainder(),
        remainder,
        "{id} remainder"
    );
    assert_eq!(metadata.recognized_unsupported_code(), None, "{id} code");
}

fn assert_c03_timing_metadata(
    id: &str,
    property: &str,
    source: &str,
    production: &str,
    status: CssSupportStatus,
    subset: Option<&str>,
    remainder: Option<&str>,
) {
    let report = parse_style_attribute(property);
    assert!(report.is_clean(), "{property}: {:?}", report.diagnostics());
    let [declaration] = report.syntax().as_slice() else {
        panic!("{property}: expected one retained declaration");
    };
    let known = declaration.known().expect("known timing declaration");
    assert_eq!(known.property().stable_id(), id, "{property} identity");

    let metadata = feature_metadata(id).unwrap_or_else(|| panic!("missing `{id}` metadata"));
    assert_eq!(metadata.id().as_str(), id);
    assert_eq!(metadata.kind(), CssFeatureKind::Property, "{id} kind");
    assert_eq!(
        metadata.spelling(),
        known.property().canonical_name(),
        "{id} spelling"
    );
    assert_eq!(metadata.source().id().as_str(), source, "{id} source");
    assert_eq!(metadata.production(), production, "{id} production");
    assert_eq!(metadata.status(), status, "{id} status");
    assert_eq!(metadata.supported_subset(), subset, "{id} subset");
    assert_eq!(
        metadata.unsupported_remainder(),
        remainder,
        "{id} remainder"
    );
    assert_eq!(metadata.recognized_unsupported_code(), None, "{id} code");
}

#[test]
fn official_integer_metadata_matches_checked_integer_behavior() {
    let value = CssIntegerCalculation::literal(-3);
    assert_eq!(value.result_type(), CssCalculationType::Integer);
    assert_c03_value_metadata(
        "official.value.integer",
        "<integer>",
        "#integers",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn official_number_metadata_matches_checked_number_behavior() {
    let value = CssNumberCalculation::try_literal(-3.5).expect("finite number");
    assert_eq!(value.result_type(), CssCalculationType::Number);
    assert_c03_value_metadata(
        "official.value.number",
        "<number>",
        "#numbers",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn official_dimension_metadata_matches_checked_dimension_behavior() {
    let value = CssLengthDimension::try_new(1.5, CssLengthUnit::Rem).expect("finite dimension");
    assert_eq!(value.value(), 1.5);
    assert_eq!(value.unit(), CssLengthUnit::Rem);
    assert_c03_value_metadata(
        "official.value.dimension",
        "<dimension>",
        "#dimensions",
        CssSupportStatus::Partial,
        Some(DIMENSION_SUBSET),
        Some(DIMENSION_REMAINDER),
    );
}

#[test]
fn official_percentage_metadata_matches_checked_percentage_behavior() {
    let value = CssPercentageCalculation::try_literal(-12.5).expect("finite percentage");
    assert_eq!(value.result_type(), CssCalculationType::Percentage);
    assert_c03_value_metadata(
        "official.value.percentage",
        "<percentage>",
        "#percentages",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn official_length_metadata_matches_checked_length_behavior() {
    let value = CssLengthCalculation::try_dimension(2.0, CssLengthUnit::Cqw)
        .expect("finite length dimension");
    assert_eq!(value.result_type(), CssCalculationType::Length);
    assert_c03_value_metadata(
        "official.value.length",
        "<length>",
        "#lengths",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn official_length_percentage_metadata_matches_mixed_length_parser_behavior() {
    let report = parse_style_attribute("width: calc((1px + 2%) * 3)");
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(
        report.syntax()[0].known().unwrap().property(),
        CssKnownProperty::Width
    );
    assert_c03_value_metadata(
        "official.value.length-percentage",
        "<length-percentage>",
        "#mixed-percentages",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn official_angle_metadata_matches_checked_angle_behavior() {
    let value = CssAngleCalculation::try_literal(-0.5, CssAngleUnit::Turns).expect("finite angle");
    assert_eq!(value.result_type(), CssCalculationType::Angle);
    assert_c03_value_metadata(
        "official.value.angle",
        "<angle>",
        "#angles",
        CssSupportStatus::Partial,
        Some(ANGLE_SUBSET),
        Some(ANGLE_REMAINDER),
    );
}

#[test]
fn official_angle_percentage_metadata_matches_checked_mixed_models() {
    let angle =
        CssAngleCalculation::try_literal(45.0, CssAngleUnit::Degrees).expect("finite angle");
    let percentage = CssPercentageCalculation::try_literal(25.0).expect("finite percentage");
    assert_eq!(angle.result_type(), CssCalculationType::Angle);
    assert_eq!(percentage.result_type(), CssCalculationType::Percentage);
    assert_c03_value_metadata(
        "official.value.angle-percentage",
        "<angle-percentage>",
        "#mixed-percentages",
        CssSupportStatus::Partial,
        Some(ANGLE_PERCENTAGE_SUBSET),
        Some(ANGLE_PERCENTAGE_REMAINDER),
    );
}

#[test]
fn official_time_metadata_matches_checked_time_behavior() {
    let value =
        CssDelayLiteral::try_new(-250.0, CssTimeUnit::Milliseconds).expect("finite signed time");
    assert_eq!(value.value(), -250.0);
    assert_eq!(value.unit(), CssTimeUnit::Milliseconds);
    assert_c03_value_metadata(
        "official.value.time",
        "<time>",
        "#time",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn official_time_percentage_metadata_matches_checked_mixed_models() {
    let time = CssTimeCalculation::try_literal(-1.0, CssTimeUnit::Seconds).expect("finite time");
    let percentage = CssPercentageCalculation::try_literal(50.0).expect("finite percentage");
    assert_eq!(time.result_type(), CssCalculationType::Time);
    assert_eq!(percentage.result_type(), CssCalculationType::Percentage);
    assert_c03_value_metadata(
        "official.value.time-percentage",
        "<time-percentage>",
        "#mixed-percentages",
        CssSupportStatus::Partial,
        Some(TIME_PERCENTAGE_SUBSET),
        Some(TIME_PERCENTAGE_REMAINDER),
    );
}

#[test]
fn official_frequency_metadata_matches_checked_frequency_behavior() {
    let value = CssFrequencyCalculation::try_literal(440.0, CssFrequencyUnit::Hertz)
        .expect("finite frequency");
    assert_eq!(value.result_type(), CssCalculationType::Frequency);
    assert_c03_value_metadata(
        "official.value.frequency",
        "<frequency>",
        "#frequency",
        CssSupportStatus::Partial,
        Some(FREQUENCY_SUBSET),
        Some(FREQUENCY_REMAINDER),
    );
}

#[test]
fn official_frequency_percentage_metadata_matches_checked_mixed_models() {
    let frequency = CssFrequencyCalculation::try_literal(1.5, CssFrequencyUnit::Kilohertz)
        .expect("finite frequency");
    let percentage = CssPercentageCalculation::try_literal(75.0).expect("finite percentage");
    assert_eq!(frequency.result_type(), CssCalculationType::Frequency);
    assert_eq!(percentage.result_type(), CssCalculationType::Percentage);
    assert_c03_value_metadata(
        "official.value.frequency-percentage",
        "<frequency-percentage>",
        "#mixed-percentages",
        CssSupportStatus::Partial,
        Some(FREQUENCY_PERCENTAGE_SUBSET),
        Some(FREQUENCY_PERCENTAGE_REMAINDER),
    );
}

#[test]
fn official_resolution_metadata_matches_checked_resolution_behavior() {
    let value = CssResolution::try_new(2.0, CssResolutionUnit::Dppx).expect("finite resolution");
    assert_eq!(value.value().value(), 2.0);
    assert_eq!(value.unit(), CssResolutionUnit::Dppx);
    assert_c03_value_metadata(
        "official.value.resolution",
        "<resolution>",
        "#resolution",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn official_calc_metadata_matches_typed_calculation_parser_behavior() {
    let report = parse_style_attribute("width: calc((1px + 2%) * 3)");
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(
        report.syntax()[0].known().unwrap().property(),
        CssKnownProperty::Width
    );
    assert_c03_value_metadata(
        "official.value.calc",
        "calc()",
        "#calc-notation,#calc-syntax,#calc-type-checking",
        CssSupportStatus::Partial,
        Some(CALC_SUBSET),
        Some(CALC_REMAINDER),
    );
}

#[test]
fn transition_duration_metadata_matches_typed_duration_behavior() {
    assert_c03_timing_metadata(
        "baseline.property.transition-duration",
        "transition-duration: calc((1s + 250ms) * 2)",
        "I-TRANSITIONS1",
        "#propdef-transition-duration",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn transition_delay_metadata_matches_signed_delay_behavior() {
    assert_c03_timing_metadata(
        "baseline.property.transition-delay",
        "transition-delay: -250ms",
        "I-TRANSITIONS1",
        "#propdef-transition-delay",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn animation_duration_metadata_matches_typed_duration_behavior() {
    assert_c03_timing_metadata(
        "baseline.property.animation-duration",
        "animation-duration: calc(1s + 250ms)",
        "I-ANIMATIONS1",
        "#propdef-animation-duration",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn animation_delay_metadata_matches_signed_delay_behavior() {
    assert_c03_timing_metadata(
        "baseline.property.animation-delay",
        "animation-delay: -1s",
        "I-ANIMATIONS1",
        "#propdef-animation-delay",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn animation_iteration_count_metadata_matches_typed_iteration_behavior() {
    assert_c03_timing_metadata(
        "baseline.property.animation-iteration-count",
        "animation-iteration-count: calc((1 + 2) * 3)",
        "I-ANIMATIONS1",
        "#propdef-animation-iteration-count",
        CssSupportStatus::Complete,
        None,
        None,
    );
}

#[test]
fn transition_metadata_matches_c03_shorthand_behavior_and_c05_remainder() {
    assert_c03_timing_metadata(
        "baseline.property.transition",
        "transition: opacity calc(1s + 250ms) linear -200ms",
        "I-TRANSITIONS1",
        "#propdef-transition",
        CssSupportStatus::Partial,
        Some(TIMING_SUBSET),
        Some(TIMING_REMAINDER),
    );
}

#[test]
fn animation_metadata_matches_c03_shorthand_behavior_and_c05_remainder() {
    assert_c03_timing_metadata(
        "baseline.property.animation",
        "animation: fade calc(1s + 250ms) linear calc((1 + 2) * 3) -200ms both running",
        "I-ANIMATIONS1",
        "#propdef-animation",
        CssSupportStatus::Partial,
        Some(TIMING_SUBSET),
        Some(TIMING_REMAINDER),
    );
}

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
        let ExpectedSource::Id(source_id) = expected.source;
        assert_eq!(
            actual.source().id().as_str(),
            source_id,
            "{} source",
            expected.id
        );

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

#[test]
fn source_registry_lookups_are_exact_and_preserve_provenance_xor() {
    let filter = specification_source("X-FILTER2-BASE").expect("filter baseline source");
    assert_eq!(filter.module(), "Filter Effects");
    assert_eq!(filter.level(), "2 baseline subset");
    assert_eq!(filter.tier(), CssSpecificationTier::SurgeistExtension);
    assert_eq!(filter.url(), None);
    assert_eq!(
        filter.repository_provenance(),
        Some("bc5394f:src/parser/effects.rs")
    );

    for source in specification_sources() {
        assert_ne!(
            source.url().is_some(),
            source.repository_provenance().is_some()
        );
    }
    assert!(specification_source("o-color4").is_none());
    assert!(specification_source(" O-COLOR4").is_none());
    assert!(specification_source("O-COLOR4 ").is_none());
    assert!(specification_source("").is_none());
}

#[test]
fn source_registry_exposes_scrollbars_level_one_reliable_provenance() {
    let source = specification_source("R-SCROLLBARS1").expect("Scrollbars 1 source");
    assert_eq!(source.id().as_str(), "R-SCROLLBARS1");
    assert_eq!(source.module(), "CSS Scrollbars Styling");
    assert_eq!(source.level(), "1");
    assert_eq!(source.tier(), CssSpecificationTier::Snapshot2026Reliable);
    assert_eq!(
        source.url(),
        Some("https://www.w3.org/TR/2021/CR-css-scrollbars-1-20211209/")
    );
    assert_eq!(source.repository_provenance(), None);

    assert!(specification_source("r-scrollbars1").is_none());
    assert!(specification_source("R-SCROLLBARS1 ").is_none());
}

#[test]
fn source_registry_exposes_containment_level_three_extension_provenance() {
    let source = specification_source("X-CONTAIN3").expect("Containment 3 source");
    assert_eq!(source.id().as_str(), "X-CONTAIN3");
    assert_eq!(source.module(), "CSS Containment");
    assert_eq!(source.level(), "3");
    assert_eq!(source.tier(), CssSpecificationTier::SurgeistExtension);
    assert_eq!(
        source.url(),
        Some("https://www.w3.org/TR/2022/WD-css-contain-3-20220818/")
    );
    assert_eq!(source.repository_provenance(), None);

    assert!(specification_source("x-contain3").is_none());
    assert!(specification_source(" X-CONTAIN3").is_none());
}

#[test]
fn preserved_i01_catalog_exposes_dated_atomic_provenance_and_alias_targets() {
    let scrollbar =
        feature_metadata("baseline.property.scrollbar-width").expect("scrollbar-width record");
    assert_eq!(scrollbar.source().id().as_str(), "R-SCROLLBARS1");
    assert_eq!(
        scrollbar.source().tier(),
        CssSpecificationTier::Snapshot2026Reliable
    );

    for id in [
        "baseline.rule.container",
        "baseline.container.condition",
        "baseline.container.size-feature",
    ] {
        let feature = feature_metadata(id).unwrap_or_else(|| panic!("missing `{id}`"));
        assert_eq!(feature.source().id().as_str(), "X-CONTAIN3", "{id}");
        assert_eq!(
            feature.source().tier(),
            CssSpecificationTier::SurgeistExtension
        );
    }

    let alias = feature_metadata("baseline.selector.pseudo-element")
        .expect("preserved pseudo-element alias");
    let targets: Vec<_> = alias
        .baseline_alias_targets()
        .iter()
        .map(|id| id.as_str())
        .collect();
    assert_eq!(
        targets,
        [
            "official.selector.generated",
            "ext.pseudo-element.marker",
            "ext.pseudo-element.selection",
            "ext.pseudo-element.backdrop",
            "ext.pseudo-element.generated-marker",
        ]
    );

    let generated =
        feature_metadata("official.selector.generated").expect("atomic generated pseudo-element");
    assert_eq!(generated.source().id().as_str(), "O-SELECTORS3");
    assert!(generated.baseline_alias_targets().is_empty());

    let report = parse_sheet(".note::before { color: red; }");
    assert!(
        report.is_clean(),
        "atomic parser case: {:?}",
        report.diagnostics()
    );
    assert_eq!(report.syntax().rules().len(), 1);
}

#[test]
fn every_alias_atomic_target_has_declared_metadata_and_public_parser_evidence() {
    let aliases: &[(&str, &[&str])] = &[
        (
            "baseline.selector.pseudo-element",
            &[
                "official.selector.generated",
                "ext.pseudo-element.marker",
                "ext.pseudo-element.selection",
                "ext.pseudo-element.backdrop",
                "ext.pseudo-element.generated-marker",
            ],
        ),
        (
            "baseline.media.query-list",
            &[
                "official.media.query-list-core",
                "ext.media.condition-syntax",
                "ext.media.malformed-member-never",
            ],
        ),
        (
            "baseline.media.range-feature",
            &[
                "official.media.feature.width",
                "official.media.feature.height",
                "official.media.feature.resolution",
                "official.media.feature.color",
                "official.media.feature.monochrome",
                "ext.media.range.width",
                "ext.media.range.height",
                "ext.media.range.resolution",
                "ext.media.range.color",
                "ext.media.range.monochrome",
            ],
        ),
        (
            "baseline.media.discrete-feature",
            &[
                "official.media.feature.orientation",
                "ext.media.hover",
                "ext.media.any-hover",
                "ext.media.pointer",
                "ext.media.any-pointer",
                "ext.media.prefers-color-scheme",
                "ext.media.prefers-reduced-motion",
                "ext.media.prefers-reduced-transparency",
                "ext.media.prefers-contrast",
                "ext.media.forced-colors",
                "ext.media.display-mode",
            ],
        ),
    ];
    for (alias_id, expected_targets) in aliases {
        let alias = feature_metadata(alias_id).unwrap_or_else(|| panic!("missing `{alias_id}`"));
        let actual_targets: Vec<_> = alias
            .baseline_alias_targets()
            .iter()
            .map(|target| target.as_str())
            .collect();
        assert_eq!(actual_targets, *expected_targets, "{alias_id}");
    }

    let clean_cases = [
        (
            "official.selector.generated",
            "O-SELECTORS3",
            ".x::after { color: red; }",
        ),
        (
            "ext.pseudo-element.marker",
            "X-PSEUDO4",
            "li::marker { color: red; }",
        ),
        (
            "ext.pseudo-element.selection",
            "X-PSEUDO4",
            "::selection { color: red; }",
        ),
        (
            "ext.pseudo-element.backdrop",
            "X-PSEUDO4",
            ".dialog::backdrop { color: red; }",
        ),
        (
            "ext.pseudo-element.generated-marker",
            "X-PSEUDO4",
            ".item::before::marker { color: red; }",
        ),
        (
            "official.media.query-list-core",
            "O-MEDIA3",
            "@media screen, print { .x { color: red; } }",
        ),
        (
            "ext.media.condition-syntax",
            "R-MEDIA4",
            "@media not screen and (width: 1px) { .x { color: red; } }",
        ),
        (
            "official.media.feature.width",
            "O-MEDIA3",
            "@media (min-width: 1px) { .x { color: red; } }",
        ),
        (
            "official.media.feature.height",
            "O-MEDIA3",
            "@media (max-height: 2px) { .x { color: red; } }",
        ),
        (
            "official.media.feature.resolution",
            "O-MEDIA3",
            "@media (resolution: 2dppx) { .x { color: red; } }",
        ),
        (
            "official.media.feature.color",
            "O-MEDIA3",
            "@media (color: 8) { .x { color: red; } }",
        ),
        (
            "official.media.feature.monochrome",
            "O-MEDIA3",
            "@media (monochrome: 1) { .x { color: red; } }",
        ),
        (
            "ext.media.range.width",
            "R-MEDIA4",
            "@media (width >= 1px) { .x { color: red; } }",
        ),
        (
            "ext.media.range.height",
            "R-MEDIA4",
            "@media (height < 2px) { .x { color: red; } }",
        ),
        (
            "ext.media.range.resolution",
            "R-MEDIA4",
            "@media (resolution > 1dppx) { .x { color: red; } }",
        ),
        (
            "ext.media.range.color",
            "R-MEDIA4",
            "@media (color >= 8) { .x { color: red; } }",
        ),
        (
            "ext.media.range.monochrome",
            "R-MEDIA4",
            "@media (monochrome = 1) { .x { color: red; } }",
        ),
        (
            "official.media.feature.orientation",
            "O-MEDIA3",
            "@media (orientation: portrait) { .x { color: red; } }",
        ),
        (
            "ext.media.hover",
            "R-MEDIA4",
            "@media (hover: hover) { .x { color: red; } }",
        ),
        (
            "ext.media.any-hover",
            "R-MEDIA4",
            "@media (any-hover: none) { .x { color: red; } }",
        ),
        (
            "ext.media.pointer",
            "R-MEDIA4",
            "@media (pointer: fine) { .x { color: red; } }",
        ),
        (
            "ext.media.any-pointer",
            "R-MEDIA4",
            "@media (any-pointer: coarse) { .x { color: red; } }",
        ),
        (
            "ext.media.prefers-color-scheme",
            "X-MEDIA5",
            "@media (prefers-color-scheme: dark) { .x { color: red; } }",
        ),
        (
            "ext.media.prefers-reduced-motion",
            "X-MEDIA5",
            "@media (prefers-reduced-motion: reduce) { .x { color: red; } }",
        ),
        (
            "ext.media.prefers-reduced-transparency",
            "X-MEDIA5",
            "@media (prefers-reduced-transparency: reduce) { .x { color: red; } }",
        ),
        (
            "ext.media.prefers-contrast",
            "X-MEDIA5",
            "@media (prefers-contrast: more) { .x { color: red; } }",
        ),
        (
            "ext.media.forced-colors",
            "X-MEDIA5",
            "@media (forced-colors: active) { .x { color: red; } }",
        ),
        (
            "ext.media.display-mode",
            "X-DISPLAY-MODE-BASE",
            "@media (display-mode: picture-in-picture) { .x { color: red; } }",
        ),
    ];
    for (id, source_id, css) in clean_cases {
        let metadata = feature_metadata(id).unwrap_or_else(|| panic!("missing `{id}`"));
        assert_eq!(metadata.source().id().as_str(), source_id, "{id}");
        assert!(metadata.baseline_alias_targets().is_empty(), "{id}");
        let report = parse_sheet(css);
        assert!(report.is_clean(), "{id}: {:?}", report.diagnostics());
        assert_eq!(report.syntax().rules().len(), 1, "{id}");
    }

    let malformed = feature_metadata("ext.media.malformed-member-never")
        .expect("malformed-member recovery target");
    assert_eq!(malformed.source().id().as_str(), "R-MEDIA4");
    assert!(malformed.baseline_alias_targets().is_empty());
    let source = "@media screen, ??? { .x { color: red; } }";
    let report = parse_sheet(source);
    assert_eq!(report.syntax().rules().len(), 1);
    let [diagnostic] = report.diagnostics() else {
        panic!("expected one malformed-member diagnostic");
    };
    assert_eq!(diagnostic.error().code(), CssErrorCode::InvalidMediaQuery);
    assert_eq!(
        diagnostic.action(),
        CssRecoveryAction::ReplaceMediaQueryWithNever
    );
    assert_eq!(
        diagnostic.error().position().byte_offset().value(),
        source.find("???").expect("responsible malformed member")
    );
    assert_eq!(
        diagnostic.span().start().byte_offset().value(),
        source.find(" ???").expect("malformed recovery unit")
    );
    assert_eq!(
        diagnostic.span().end().byte_offset().value(),
        source.find('{').expect("end of media query list")
    );
}

#[test]
fn exclusion_registry_exposes_named_official_audit_facts() {
    let predecessor = conformance_exclusion("excluded.O-CSS2.property.margin")
        .expect("superseded CSS2 margin definition");
    assert_eq!(predecessor.source().id().as_str(), "O-CSS2");
    assert_eq!(predecessor.production(), "box.html#propdef-margin");
    assert_eq!(
        predecessor.reason(),
        CssExclusionReason::SupersededWithoutCurrentProduction
    );
    assert_eq!(
        predecessor.superseding_ids().map(|ids| ids[0].as_str()),
        Some("baseline.property.margin")
    );

    let informative = conformance_exclusion("excluded.O-CSS2.informative-property.azimuth")
        .expect("informative CSS2 Appendix A property row");
    assert_eq!(informative.production(), "aural.html#propdef-azimuth");
    assert_eq!(informative.reason(), CssExclusionReason::InformativeOnly);
    assert_eq!(informative.superseding_ids(), None);

    let processing = conformance_exclusion("excluded.O-IMAGES3.processing")
        .expect("out-of-boundary Images processing row");
    assert_eq!(
        processing.reason(),
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    );

    let audit = conformance_exclusion("excluded.O-COLOR4.informative-audit")
        .expect("Color 4 informative source audit row");
    assert_eq!(audit.source().id().as_str(), "O-COLOR4");
    assert_eq!(audit.reason(), CssExclusionReason::InformativeOnly);
    assert!(audit.superseding_ids().is_none());

    assert!(
        conformance_exclusions()
            .iter()
            .any(|row| row.id() == audit.id())
    );
    assert!(conformance_exclusion("EXCLUDED.O-COLOR4.INFORMATIVE-AUDIT").is_none());
    assert!(conformance_exclusion(" excluded.O-COLOR4.informative-audit").is_none());
    assert!(conformance_exclusion("excluded.O-COLOR4.informative-audit ").is_none());
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

#[test]
fn object_position_recognition_does_not_recognize_other_images_three_properties() {
    let object = parse_style_attribute("object-position: left top");
    assert!(object.is_clean(), "{:?}", object.diagnostics());
    assert_eq!(
        object.syntax()[0]
            .known()
            .expect("known object position")
            .property(),
        CssKnownProperty::ObjectPosition,
    );

    for property in ["object-fit", "image-rendering", "image-orientation"] {
        let source = format!("{property}: auto; color: red");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: expected one unknown-property diagnostic");
        };
        assert_eq!(
            diagnostic.error().code(),
            CssErrorCode::UnknownProperty,
            "{source}"
        );
        assert_eq!(
            diagnostic.action(),
            CssRecoveryAction::DropDeclaration,
            "{source}"
        );
        let ErrorKind::UnknownProperty(detail) = diagnostic.error().kind() else {
            panic!("{source}: expected unknown-property payload");
        };
        assert_eq!(detail.name().as_str(), property, "{source}");
        assert_eq!(
            report.syntax()[0]
                .known()
                .expect("retained color declaration")
                .property(),
            CssKnownProperty::Color,
            "{source}",
        );
    }
}

# CSS Queries Fonts Imports And Selectors Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add strict authored CSS syntax support for media/container query at-rules, font-face/import contracts, combinator selectors, and attribute selectors without adding resource loading, query evaluation, or browser recovery.

**Architecture:** `surgeist-css` remains a parser and authored-syntax owner. It should produce typed CSS-owned contracts for nested rules, imports, font faces, media queries, container queries, complex selectors, and attribute selectors, while downstream root/style layers decide loading, matching, query evaluation, cascade application, and font activation. All new models must preserve authored structure with private fields and checked constructors where invariants matter.

**Tech Stack:** Rust, `cssparser`, crate-local parser modules, crate-local parser tests, `cargo fmt`, `cargo test -p surgeist-css`, `cargo clippy -p surgeist-css --all-targets -- -D warnings`.

---

## Source Requirements

- Follow `AGENTS.md`.
- Use `guidance/surgeist-rust-modeling-guide.md`.
- Coordinator does not write implementation code.
- Workers do not commit.
- Commit each clean worker/reviewer cycle as a logical point.
- No branches.
- Work only in `/Users/codex/Development/surgeist-css`.
- Preserve strict invalid-CSS rejection. Do not add browser-style recovery.
- Keep all new syntax CSS-owned and authored/parser-facing.
- Do not load files, fetch URLs, resolve imports, match selectors, evaluate queries, activate fonts, compute cascade, or apply conditional rules in this crate.
- Breaking public API changes are allowed when they improve authored CSS modeling.
- External/root tests must exercise public integration behavior only; do not preserve accidental APIs solely for compatibility.

## Specification References

Use these as grammar references, but implement only the strict subset explicitly listed in this plan:

- CSS Cascading and Inheritance Level 5, especially `@import` grammar and cascade layer import syntax: <https://www.w3.org/TR/css-cascade-5/>
- CSS Fonts Module Level 4, especially `@font-face` and descriptors: <https://www.w3.org/TR/css-fonts-4/>
- Selectors Level 4, especially combinators and attribute selectors: <https://www.w3.org/TR/selectors-4/>
- Media Queries Level 5, especially boolean conditions and range/discrete media features: <https://www.w3.org/TR/mediaqueries-5/>
- CSS Containment Module Level 3, especially `@container` syntax and size/style queries: <https://www.w3.org/TR/css-contain-3/>
- CSS Conditional Rules Level 5, especially modern container query notes: <https://www.w3.org/TR/css-conditional-5/>

## Current Baseline

- `CssSheet` currently stores only `Vec<CssRule>`.
- `CssRule` currently represents only a style rule: one selector plus declarations.
- `parse_sheet` currently rejects all at-rules.
- Selectors currently support tag, id/key, class, compound simple selectors, selector lists, and the practical pseudo-classes from `plans/2026-07-04-css-practical-pseudo-classes.md`.
- Selectors intentionally reject combinators, descendant selectors, relative selectors, attribute selectors, pseudo-elements, namespaces, and many pseudo-classes.
- CSS variables are parsed symbolically; no cascade substitution happens in this crate.

## Scope

Implement authored parsing and typed syntax for:

- `@media` conditional group rules.
- `@container` conditional group rules.
- `@font-face` descriptor rules.
- `@import` rules as parse-only contracts.
- Selector combinators:
  - descendant whitespace combinator
  - child `>`
  - next-sibling `+`
  - subsequent-sibling `~`
- Attribute selectors:
  - existence `[name]`
  - exact `[name=value]`
  - includes `[name~=value]`
  - dash-match `[name|=value]`
  - prefix `[name^=value]`
  - suffix `[name$=value]`
  - substring `[name*=value]`
  - optional ASCII case modifier `i`
  - optional explicitly-sensitive modifier `s`

Do not implement in this pass:

- `@supports` as a standalone at-rule.
- `supports(...)` conditions in `@import`.
- `@layer`, except the optional `layer` / `layer(name)` clause carried by `@import`.
- `@keyframes`.
- `@scope`.
- `@property`.
- `@charset`.
- `@namespace`.
- `@page`, print margin at-rules, counters, view transitions, or other at-rules.
- Font loading, URL fetching, path resolution, local font lookup, or font activation.
- Media or container query evaluation.
- Container scroll-state queries.
- General property validation inside container `style(...)` queries.
- Selector namespaces, pseudo-elements, column combinator `||`, relative selector lists, or shadow DOM selector grammar.
- Browser-compatible recovery for invalid at-rules, invalid descriptors, duplicate font descriptors, or unsupported selector/query syntax.

## Modeling Rules

- At-rules are authored syntax. They may carry nested rules, descriptors, or conditions, but they do not apply behavior in `surgeist-css`.
- `@import` is parse-only. This crate records the import target and authored clauses; root/style decides loading and dependency resolution.
- `@font-face` is parse-only. This crate records typed descriptors; root/style/text decides font resource loading, validation against platform support, and activation.
- `@media` and `@container` conditions are symbolic typed query syntax. This crate does not decide whether conditions are true.
- Complex selectors are authored selector syntax. This crate does not match selectors to nodes or calculate specificity.
- Attribute selector names and values must be structured, not stored as a broad raw selector string.
- Top-level rule selectors may become complex selectors in this pass. Selector lists inside functional pseudo-classes must remain on the current restricted compound-selector grammar in this pass, so `:has(> .icon)`, `:has(.field > .icon)`, and `:not(.field .icon)` keep rejecting until relative/complex pseudo-class arguments are deliberately modeled.
- Functional pseudo-classes must not keep using a public selector-list type that can contain `CssSelector::Complex`. Add a dedicated restricted pseudo selector-list model or make the constructor reject complex selectors before `CssSelector::Complex` becomes public.
- Unsupported but valid CSS remains rejected until deliberately modeled.
- Malformed syntax rejects the whole sheet.
- Duplicate `@font-face` descriptors should reject rather than silently last-one-wins. That keeps Surgeist strict and avoids browser recovery semantics.
- `CssValue` must not become a cross-property validation bag. New query/font/import models should be separate authored syntax types, not shoehorned into declaration values.

## Target Rule Model

Replace the style-rule-only `CssRule` struct with a closed rule enum and a style-rule struct:

```rust
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CssSheet {
    rules: Vec<CssRule>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssRule {
    Style(CssStyleRule),
    Import(CssImportRule),
    FontFace(CssFontFaceRule),
    Media(CssMediaRule),
    Container(CssContainerRule),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssStyleRule {
    selector: CssSelector,
    declarations: Vec<CssDeclaration>,
}

impl CssStyleRule {
    pub const fn selector(&self) -> &CssSelector;
    pub fn declarations(&self) -> &[CssDeclaration];
}
```

Keep `CssSheet::rules() -> &[CssRule]`. Add test helpers that unwrap style rules deliberately instead of depending on the old `CssRule` struct shape.

## Target Selector Model

Keep simple selectors inspectable, and add an explicit complex-selector layer:

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum CssSelector {
    Tag(String),
    Key(String),
    Class(String),
    PseudoClass(CssPseudoClass),
    Compound(CssCompoundSelector),
    Complex(CssComplexSelector),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssComplexSelector {
    first: CssCompoundSelector,
    rest: Vec<CssComplexSelectorPart>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssComplexSelectorPart {
    combinator: CssSelectorCombinator,
    selector: CssCompoundSelector,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssSelectorCombinator {
    Descendant,
    Child,
    NextSibling,
    SubsequentSibling,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssAttributeSelector {
    name: CssAttributeName,
    matcher: CssAttributeMatcher,
    case_sensitivity: CssAttributeCaseSensitivity,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CssAttributeName {
    name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssAttributeMatcher {
    Exists,
    Equals(String),
    Includes(String),
    DashMatch(String),
    Prefix(String),
    Suffix(String),
    Substring(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssAttributeCaseSensitivity {
    DocumentDefault,
    AsciiCaseInsensitive,
    ExplicitSensitive,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssPseudoSelectorList {
    selectors: Vec<CssSelector>,
}
```

Add `attributes: Vec<CssAttributeSelector>` to `CssCompoundSelector`, plus `attributes()`.

When `CssSelector::Complex` is added, update `CssPseudoClass::{Not, Is, Where, Has}` to carry `CssPseudoSelectorList` instead of unrestricted `CssSelectorList`.

Required public accessors:

```rust
impl CssComplexSelector {
    pub fn try_new(first: CssCompoundSelector, rest: Vec<CssComplexSelectorPart>) -> Option<Self>;
    pub(crate) fn new(first: CssCompoundSelector, rest: Vec<CssComplexSelectorPart>) -> Self;
    pub const fn first(&self) -> &CssCompoundSelector;
    pub fn rest(&self) -> &[CssComplexSelectorPart];
}

impl CssComplexSelectorPart {
    pub const fn combinator(&self) -> CssSelectorCombinator;
    pub const fn selector(&self) -> &CssCompoundSelector;
}

impl CssCompoundSelector {
    pub fn attributes(&self) -> &[CssAttributeSelector];
}

impl CssAttributeSelector {
    pub const fn name(&self) -> &CssAttributeName;
    pub const fn matcher(&self) -> &CssAttributeMatcher;
    pub const fn case_sensitivity(&self) -> CssAttributeCaseSensitivity;
}

impl CssAttributeName {
    pub fn try_new(name: impl Into<String>) -> Option<Self>;
    pub(crate) fn new(name: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}

impl CssPseudoSelectorList {
    pub fn try_new(selectors: Vec<CssSelector>) -> Option<Self>;
    pub(crate) fn new(selectors: Vec<CssSelector>) -> Self;
    pub fn selectors(&self) -> &[CssSelector];
}
```

`CssAttributeName::try_new` must enforce the same shape the parser accepts:
reject empty and whitespace-only names, reject names that are not exactly one CSS
identifier, reject namespace separators such as `svg|href`, and reject trailing
tokens. Do not let public construction accept attribute names the parser would
reject.

`CssPseudoSelectorList::try_new` must reject empty lists and any selector containing `CssSelector::Complex`.
`CssComplexSelector::try_new` must reject an empty `rest` list so plain compound selectors cannot be represented as complex selectors.

## Target Import Model

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct CssImportRule {
    target: CssImportTarget,
    layer: Option<CssImportLayer>,
    media: Option<CssMediaQueryList>,
    location: CssSourceLocation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssImportTarget {
    Url(CssImportUrl),
    String(CssImportString),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssImportUrl {
    value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssImportString {
    value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssImportLayer {
    Anonymous,
    Named(CssLayerName),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CssLayerName {
    components: Vec<String>,
}
```

`CssImportRule` must not resolve or load the target. Reject `@import` after a non-import top-level rule. Reject `supports(...)` import clauses until standalone support conditions are deliberately modeled.

Required public accessors:

```rust
impl CssImportRule {
    pub const fn target(&self) -> &CssImportTarget;
    pub const fn layer(&self) -> Option<&CssImportLayer>;
    pub const fn media(&self) -> Option<&CssMediaQueryList>;
    pub const fn location(&self) -> CssSourceLocation;
}

impl CssLayerName {
    pub fn try_new(components: impl IntoIterator<Item = impl Into<String>>) -> Option<Self>;
    pub(crate) fn new(components: Vec<String>) -> Self;
    pub fn components(&self) -> &[String];
}

impl CssImportUrl {
    pub fn try_new(value: impl Into<String>) -> Option<Self>;
    pub(crate) fn new(value: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}

impl CssImportString {
    pub fn try_new(value: impl Into<String>) -> Option<Self>;
    pub(crate) fn new(value: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}
```

`CssLayerName::try_new` must reject empty layer paths, empty or
whitespace-only components, components that are not exactly one CSS identifier,
and any parser-reserved identifiers for layer-name grammar. Public construction
must not permit a layer name that `@import ... layer(...)` would reject.

## Target Media Query Model

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct CssMediaRule {
    query: CssMediaQueryList,
    rules: Vec<CssRule>,
    location: CssSourceLocation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssMediaQueryList {
    queries: Vec<CssMediaQuery>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssMediaQuery {
    Condition(CssMediaCondition),
    Typed(CssTypedMediaQuery),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssTypedMediaQuery {
    modifier: Option<CssMediaQueryModifier>,
    media_type: CssMediaType,
    condition: Option<CssMediaCondition>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssMediaQueryModifier {
    Not,
    Only,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssMediaType {
    All,
    Screen,
    Print,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssMediaCondition {
    Feature(CssMediaFeatureQuery),
    Not(Box<CssMediaCondition>),
    And(CssMediaConditionList),
    Or(CssMediaConditionList),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssMediaConditionList {
    conditions: Vec<CssMediaCondition>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssMediaFeatureQuery {
    Width(CssRangeFeature<CssQueryLength>),
    Height(CssRangeFeature<CssQueryLength>),
    Resolution(CssRangeFeature<CssResolution>),
    Color(CssRangeFeature<CssNonNegativeInteger>),
    Monochrome(CssRangeFeature<CssNonNegativeInteger>),
    Orientation(CssOrientation),
    PrefersColorScheme(CssColorSchemePreference),
    PrefersReducedMotion(CssReducedMotionPreference),
    PrefersReducedTransparency(CssReducedTransparencyPreference),
    PrefersContrast(CssContrastPreference),
    ForcedColors(CssForcedColorsMode),
    Hover(CssHoverCapability),
    AnyHover(CssHoverCapability),
    Pointer(CssPointerCapability),
    AnyPointer(CssPointerCapability),
    DisplayMode(CssDisplayMode),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssQueryComparison {
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssRangeFeature<T> {
    comparison: Option<CssQueryComparison>,
    value: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssNonNegativeInteger {
    value: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssOrientation {
    Portrait,
    Landscape,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssColorSchemePreference {
    Light,
    Dark,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssReducedMotionPreference {
    Reduce,
    NoPreference,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssReducedTransparencyPreference {
    Reduce,
    NoPreference,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssContrastPreference {
    NoPreference,
    More,
    Less,
    Custom,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssForcedColorsMode {
    None,
    Active,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssHoverCapability {
    None,
    Hover,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssPointerCapability {
    None,
    Coarse,
    Fine,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssDisplayMode {
    Fullscreen,
    Standalone,
    MinimalUi,
    Browser,
    PictureInPicture,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssResolution {
    value: CssFiniteNumber,
    unit: CssResolutionUnit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssResolutionUnit {
    Dpi,
    Dpcm,
    Dppx,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssRatio {
    numerator: CssFiniteNumber,
    denominator: CssFiniteNumber,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssQueryLength {
    value: CssFiniteNumber,
    unit: CssLengthUnit,
}
```

Required public accessors:

```rust
impl CssMediaRule {
    pub const fn query(&self) -> &CssMediaQueryList;
    pub fn rules(&self) -> &[CssRule];
    pub const fn location(&self) -> CssSourceLocation;
}

impl CssMediaQueryList {
    pub fn try_new(queries: Vec<CssMediaQuery>) -> Option<Self>;
    pub(crate) fn new(queries: Vec<CssMediaQuery>) -> Self;
    pub fn queries(&self) -> &[CssMediaQuery];
}

impl CssTypedMediaQuery {
    pub fn new(
        modifier: Option<CssMediaQueryModifier>,
        media_type: CssMediaType,
        condition: Option<CssMediaCondition>,
    ) -> Self;
    pub const fn modifier(&self) -> Option<CssMediaQueryModifier>;
    pub const fn media_type(&self) -> CssMediaType;
    pub const fn condition(&self) -> Option<&CssMediaCondition>;
}

impl CssMediaFeatureQuery {
    pub const fn name(&self) -> &'static str;
}

impl<T> CssRangeFeature<T> {
    pub(crate) fn new(comparison: Option<CssQueryComparison>, value: T) -> Self;
    pub const fn comparison(&self) -> Option<CssQueryComparison>;
    pub const fn value(&self) -> &T;
}

impl CssNonNegativeInteger {
    pub const fn new(value: u32) -> Self;
    pub const fn value(self) -> u32;
}

impl CssMediaConditionList {
    pub fn try_new(conditions: Vec<CssMediaCondition>) -> Option<Self>;
    pub(crate) fn new(conditions: Vec<CssMediaCondition>) -> Self;
    pub fn conditions(&self) -> &[CssMediaCondition];
}

impl CssResolution {
    pub fn try_new(value: f32, unit: CssResolutionUnit) -> Option<Self>;
    pub const fn value(self) -> CssFiniteNumber;
    pub const fn unit(self) -> CssResolutionUnit;
}

impl CssRatio {
    pub fn try_new(numerator: f32, denominator: f32) -> Option<Self>;
    pub const fn numerator(self) -> CssFiniteNumber;
    pub const fn denominator(self) -> CssFiniteNumber;
}

impl CssQueryLength {
    pub fn try_new(value: f32, unit: CssLengthUnit) -> Option<Self>;
    pub const fn value(self) -> CssFiniteNumber;
    pub const fn unit(self) -> CssLengthUnit;
}
```

`CssMediaQuery` must not be modeled as independent optional fields. A media query is either a condition-only query such as `(width >= 600px)` or a typed query such as `screen`, `not screen`, or `screen and (width >= 600px)`. `only` and `not` modifiers are valid only on `CssMediaQuery::Typed`.

Query numeric invariants:

- `CssQueryLength::try_new` accepts only finite concrete lengths greater than or equal to `0.0` with a supported length unit. Query length features must not reuse broad declaration `CssLength`, because declaration values such as `auto`, `normal`, `min-content`, `max-content`, `fit-content`, and percentages are not valid query lengths for this plan.
- `CssRatio::try_new` requires finite numerator and denominator values, `numerator >= 0.0`, and `denominator > 0.0`.

Supported media features in this pass:

- Range length features: `width`, `height`, `min-width`, `max-width`, `min-height`, `max-height`.
- Range numeric features: `resolution`, `min-resolution`, `max-resolution`, `color`, `min-color`, `max-color`, `monochrome`, `min-monochrome`, `max-monochrome`.
- Discrete features:
  - `orientation: portrait | landscape`
  - `prefers-color-scheme: light | dark`
  - `prefers-reduced-motion: reduce | no-preference`
  - `prefers-reduced-transparency: reduce | no-preference`
  - `prefers-contrast: no-preference | more | less | custom`
  - `forced-colors: none | active`
  - `hover: none | hover`
  - `any-hover: none | hover`
  - `pointer: none | coarse | fine`
  - `any-pointer: none | coarse | fine`
  - `display-mode: fullscreen | standalone | minimal-ui | browser | picture-in-picture`

Reject unknown media types and unknown media features. Reject old unknown browser media types rather than treating them as false.

## Target Container Query Model

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct CssContainerRule {
    name: Option<CssContainerName>,
    condition: CssContainerCondition,
    rules: Vec<CssRule>,
    location: CssSourceLocation,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CssContainerName {
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssContainerCondition {
    Feature(CssContainerFeatureQuery),
    Style(CssContainerStyleQuery),
    Not(Box<CssContainerCondition>),
    And(CssContainerConditionList),
    Or(CssContainerConditionList),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssContainerConditionList {
    conditions: Vec<CssContainerCondition>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssContainerFeatureQuery {
    Width(CssRangeFeature<CssQueryLength>),
    Height(CssRangeFeature<CssQueryLength>),
    InlineSize(CssRangeFeature<CssQueryLength>),
    BlockSize(CssRangeFeature<CssQueryLength>),
    AspectRatio(CssRangeFeature<CssRatio>),
    Orientation(CssOrientation),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssContainerStyleQuery {
    CustomPropertyPresence(CssCustomPropertyName),
    CustomPropertyValue {
        name: CssCustomPropertyName,
        value: CssAuthoredDeclarationValue,
    },
}
```

Required public accessors:

```rust
impl CssContainerRule {
    pub const fn name(&self) -> Option<&CssContainerName>;
    pub const fn condition(&self) -> &CssContainerCondition;
    pub fn rules(&self) -> &[CssRule];
    pub const fn location(&self) -> CssSourceLocation;
}

impl CssContainerName {
    pub fn try_new(name: impl Into<String>) -> Option<Self>;
    pub(crate) fn new(name: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}

impl CssContainerConditionList {
    pub fn try_new(conditions: Vec<CssContainerCondition>) -> Option<Self>;
    pub(crate) fn new(conditions: Vec<CssContainerCondition>) -> Self;
    pub fn conditions(&self) -> &[CssContainerCondition];
}
```

`CssMediaConditionList::try_new` and `CssContainerConditionList::try_new` must require at least two conditions. A single condition should stay represented as the condition itself, not as a one-item `And` or `Or` group.

`CssContainerName::try_new` must reject empty and whitespace-only names, names
that are not exactly one CSS custom-ident, and identifiers reserved by container
query grammar such as `none`, `and`, `or`, `not`, and `style`. Use the same
validation for public construction and parser lowering so invalid container
names cannot be expressed through the public model.

Supported container features in this pass:

- Size features: `width`, `height`, `inline-size`, `block-size`, `min-width`, `max-width`, `min-height`, `max-height`, `min-inline-size`, `max-inline-size`, `min-block-size`, `max-block-size`.
- Ratio/discrete features: `aspect-ratio`, `min-aspect-ratio`, `max-aspect-ratio`, `orientation: portrait | landscape`.
- Style queries only for custom properties:
  - `style(--theme)`
  - `style(--theme: dark)`
  - `style(--density: compact)`

Reject regular declaration style queries such as `style(color: red)` until a later plan decides how to model container style-query property validation.

## Target Font Face Model

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct CssFontFaceRule {
    descriptors: CssFontFaceDescriptors,
    location: CssSourceLocation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssFontFaceDescriptors {
    font_family: CssFontFaceFamily,
    src: CssFontFaceSourceList,
    font_weight: Option<CssFontFaceWeight>,
    font_style: Option<CssFontFaceStyle>,
    font_stretch: Option<CssFontFaceStretch>,
    font_display: Option<CssFontDisplay>,
    unicode_range: Option<CssUnicodeRangeList>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssFontFaceSource {
    Url(CssFontFaceUrlSource),
    Local(CssFontLocalName),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssFontFaceUrlSource {
    url: String,
    format: Option<CssFontFormatHint>,
    tech: Vec<CssFontTechHint>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssFontFaceFamily {
    name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssFontLocalName {
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssFontFaceSourceList {
    sources: Vec<CssFontFaceSource>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssFontFaceWeight {
    start: CssFontFaceWeightValue,
    end: Option<CssFontFaceWeightValue>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssFontFaceWeightValue {
    value: CssFiniteNumber,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssFontFaceStyle {
    Normal,
    Italic,
    Oblique(Option<CssFontFaceObliqueRange>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssFontFaceObliqueRange {
    start_degrees: CssFiniteNumber,
    end_degrees: Option<CssFiniteNumber>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssFontFaceStretch {
    start: CssFontFaceStretchValue,
    end: Option<CssFontFaceStretchValue>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssFontFaceStretchValue {
    percent: CssFiniteNumber,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontDisplay {
    Auto,
    Block,
    Swap,
    Fallback,
    Optional,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssUnicodeRangeList {
    ranges: Vec<CssUnicodeRange>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssUnicodeRange {
    start: u32,
    end: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontFormatHint {
    Woff,
    Woff2,
    TrueType,
    OpenType,
    Collection,
    EmbeddedOpenType,
    Svg,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontTechHint {
    Variations,
    ColorCOLRv0,
    ColorCOLRv1,
    ColorSVG,
    ColorSbix,
    ColorCBDT,
    FeaturesOpenType,
    FeaturesAAT,
    FeaturesGraphite,
    Incremental,
}
```

Required public accessors:

```rust
impl CssFontFaceRule {
    pub const fn descriptors(&self) -> &CssFontFaceDescriptors;
    pub const fn location(&self) -> CssSourceLocation;
}

impl CssFontFaceDescriptors {
    pub fn try_new(
        font_family: Option<CssFontFaceFamily>,
        src: Option<CssFontFaceSourceList>,
        font_weight: Option<CssFontFaceWeight>,
        font_style: Option<CssFontFaceStyle>,
        font_stretch: Option<CssFontFaceStretch>,
        font_display: Option<CssFontDisplay>,
        unicode_range: Option<CssUnicodeRangeList>,
    ) -> Option<Self>;
    pub const fn font_family(&self) -> &CssFontFaceFamily;
    pub const fn src(&self) -> &CssFontFaceSourceList;
    pub const fn font_weight(&self) -> Option<&CssFontFaceWeight>;
    pub const fn font_style(&self) -> Option<&CssFontFaceStyle>;
    pub const fn font_stretch(&self) -> Option<&CssFontFaceStretch>;
    pub const fn font_display(&self) -> Option<CssFontDisplay>;
    pub const fn unicode_range(&self) -> Option<&CssUnicodeRangeList>;
}

impl CssFontFaceSourceList {
    pub fn try_new(sources: Vec<CssFontFaceSource>) -> Option<Self>;
    pub(crate) fn new(sources: Vec<CssFontFaceSource>) -> Self;
    pub fn sources(&self) -> &[CssFontFaceSource];
}

impl CssFontFaceUrlSource {
    pub fn try_new(
        url: impl Into<String>,
        format: Option<CssFontFormatHint>,
        tech: Vec<CssFontTechHint>,
    ) -> Option<Self>;
    pub fn url(&self) -> &str;
    pub const fn format(&self) -> Option<&CssFontFormatHint>;
    pub fn tech(&self) -> &[CssFontTechHint];
}

impl CssFontFaceFamily {
    pub fn try_new(name: impl Into<String>) -> Option<Self>;
    pub(crate) fn new(name: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}

impl CssFontLocalName {
    pub fn try_new(name: impl Into<String>) -> Option<Self>;
    pub(crate) fn new(name: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}

impl CssFontFaceWeight {
    pub fn try_single(value: f32) -> Option<Self>;
    pub fn try_range(start: f32, end: f32) -> Option<Self>;
    pub const fn start(self) -> CssFontFaceWeightValue;
    pub const fn end(self) -> Option<CssFontFaceWeightValue>;
}

impl CssFontFaceWeightValue {
    pub fn try_new(value: f32) -> Option<Self>;
    pub const fn value(self) -> CssFiniteNumber;
}

impl CssFontFaceStretch {
    pub fn try_single_percent(percent: f32) -> Option<Self>;
    pub fn try_range_percent(start: f32, end: f32) -> Option<Self>;
    pub const fn start(self) -> CssFontFaceStretchValue;
    pub const fn end(self) -> Option<CssFontFaceStretchValue>;
}

impl CssFontFaceStretchValue {
    pub fn try_new_percent(percent: f32) -> Option<Self>;
    pub const fn percent(self) -> CssFiniteNumber;
}

impl CssUnicodeRange {
    pub fn try_new(start: u32, end: u32) -> Option<Self>;
    pub const fn start(self) -> u32;
    pub const fn end(self) -> u32;
}

impl CssUnicodeRangeList {
    pub fn try_new(ranges: Vec<CssUnicodeRange>) -> Option<Self>;
    pub(crate) fn new(ranges: Vec<CssUnicodeRange>) -> Self;
    pub fn ranges(&self) -> &[CssUnicodeRange];
}

impl CssFontFaceObliqueRange {
    pub fn try_new(start_degrees: f32, end_degrees: Option<f32>) -> Option<Self>;
    pub const fn start_degrees(self) -> CssFiniteNumber;
    pub const fn end_degrees(self) -> Option<CssFiniteNumber>;
}
```

Require `font-family` and `src`. Reject unknown descriptors and duplicate descriptors. Do not load or validate the referenced resources.

Font-face descriptor invariants:

- `CssFontFaceFamily::try_new`, `CssFontLocalName::try_new`, `CssFontFaceUrlSource::try_new`, `CssImportUrl::try_new`, and `CssImportString::try_new` reject empty or whitespace-only strings.
- `format(...)` hints must map to `CssFontFormatHint`; unknown format names reject the whole sheet.
- `tech(...)` hints must map to `CssFontTechHint`; unknown technology names reject the whole sheet.
- `CssFontFaceWeightValue::try_new` accepts only finite numeric weights in `1.0..=1000.0`; it must not reuse declaration-only relative values such as `bolder` or `lighter`.
- `CssFontFaceWeight::try_range(start, end)` requires both values in `1.0..=1000.0` and `start <= end`.
- `CssFontFaceStretchValue::try_new_percent` accepts only finite percentages greater than or equal to `0.0`.
- `CssFontFaceStretch::try_range_percent(start, end)` requires both percentages to be valid and `start <= end`.
- `CssFontFaceObliqueRange::try_new` accepts only finite degrees in `-90.0..=90.0`; if an end value is present, `start <= end`.
- `CssUnicodeRange::try_new` requires `start <= end` and `end <= 0x10FFFF`.
- `CssFontFaceSourceList::try_new` and `CssUnicodeRangeList::try_new` reject empty lists.

## Task 1: Reshape Rule Syntax For At-Rules

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/test_support.rs`
- Modify: `src/tests.rs`

- [ ] Add `CssStyleRule` and change `CssRule` into a buildable intermediate enum with only the style-rule variant:

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum CssRule {
    Style(CssStyleRule),
}
```

Later tasks add `Import`, `FontFace`, `Media`, and `Container` variants at the same time they add the corresponding payload types. Do not add enum variants whose payload types do not exist yet.

- [ ] Keep `CssSheet::rules() -> &[CssRule]`.

- [ ] Add accessors on rule variants through explicit pattern matching in tests. Do not add convenience methods that hide at-rule variants unless the reviewer agrees they are a real public front door.

- [ ] Update parser style-rule construction:

```rust
Ok(selectors
    .into_iter()
    .map(|selector| CssRule::Style(CssStyleRule::new(selector, declarations.clone())))
    .collect())
```

- [ ] Update test helpers to unwrap style rules:

```rust
fn style_rule(rule: &CssRule) -> &CssStyleRule {
    let CssRule::Style(rule) = rule;
    rule
}
```

- [ ] Add model tests:

```rust
#[test]
fn parsed_style_rule_is_explicit_rule_variant() {
    let sheet = parse_sheet(".panel { color: black; }").unwrap();
    let rule = style_rule(&sheet.rules()[0]);
    assert_eq!(rule.declarations().len(), 1);
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css parsed_style_rule_is_explicit_rule_variant
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 2: Model And Parse Attribute Selectors

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/parser/selectors.rs`
- Modify: `src/tests.rs`

- [ ] Add `CssAttributeSelector`, `CssAttributeName`, `CssAttributeMatcher`, and `CssAttributeCaseSensitivity`.

- [ ] Add `attributes: Vec<CssAttributeSelector>` to `CssCompoundSelector` and update constructor/accessors.

- [ ] Parse attributes inside compound selectors using `input.parse_square_block`.

- [ ] Accept:

```css
[disabled] { color: black; }
[data-state=open] { color: black; }
[data-role~="button"] { color: black; }
[lang|=en] { color: black; }
[href^="https"] { color: black; }
[src$=".svg"] { color: black; }
[data-id*="card"] { color: black; }
[data-state="OPEN" i] { color: black; }
[data-state="open" s] { color: black; }
button.primary[aria-expanded=true]:hover { color: black; }
```

- [ ] Reject namespaces, missing values, invalid modifiers, empty names, and trailing tokens:

```rust
assert!(parse_sheet("[svg|href] { color: black; }").is_err());
assert!(parse_sheet("[data-state=] { color: black; }").is_err());
assert!(parse_sheet("[data-state=open q] { color: black; }").is_err());
assert!(parse_sheet("[] { color: black; }").is_err());
assert!(parse_sheet("[data-state=open extra] { color: black; }").is_err());
```

- [ ] Add public API inspection tests:

```rust
#[test]
fn attribute_selectors_are_structurally_inspectable() {
    let sheet = parse_sheet(r#"[data-state="open" i] { color: black; }"#).unwrap();
    let rule = style_rule(&sheet.rules()[0]);
    let CssSelector::Compound(selector) = rule.selector() else {
        panic!("expected compound selector");
    };
    let [attribute] = selector.attributes() else {
        panic!("expected one attribute selector");
    };
    assert_eq!(attribute.name().as_str(), "data-state");
    assert_eq!(attribute.matcher(), &CssAttributeMatcher::Equals("open".to_owned()));
    assert_eq!(
        attribute.case_sensitivity(),
        CssAttributeCaseSensitivity::AsciiCaseInsensitive
    );
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css attribute_selectors
cargo test -p surgeist-css attribute
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 3: Model And Parse Combinator Selectors

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/parser/selectors.rs`
- Modify: `src/tests.rs`

- [ ] Add `CssComplexSelector`, `CssComplexSelectorPart`, and `CssSelectorCombinator`.

- [ ] Refactor selector parsing so top-level rule selector lists parse complex selectors. Keep compound parsing as a reusable inner parser.

- [ ] Split selector-list parsers deliberately:
  - `parse_rule_selector_list` or equivalent parses complex selectors for rule preludes.
  - `parse_pseudo_selector_list` or equivalent keeps the current restricted compound-selector-list grammar for `:not`, `:is`, `:where`, and `:has`.
  - Do not let Task 3 accidentally broaden functional pseudo-class selector arguments.

- [ ] Migrate `CssPseudoClass::{Not, Is, Where, Has}` to carry `CssPseudoSelectorList`, and add a constructor test proving `CssPseudoSelectorList::try_new(vec![CssSelector::Complex(...)])` rejects.

- [ ] Accept:

```css
.stack .item { color: black; }
.toolbar > button { color: black; }
label + input { color: black; }
h2 ~ p { color: black; }
.card[data-state=open] > .title:hover { color: black; }
```

- [ ] Reject leading combinators, doubled combinators, trailing combinators, column combinator, and relative selectors in `:has()`:

```rust
assert!(parse_sheet("> .item { color: black; }").is_err());
assert!(parse_sheet(".a > > .b { color: black; }").is_err());
assert!(parse_sheet(".a > { color: black; }").is_err());
assert!(parse_sheet(".col || .cell { color: black; }").is_err());
assert!(parse_sheet(".field:has(> .icon) { color: black; }").is_err());
assert!(parse_sheet(".field:has(.field > .icon) { color: black; }").is_err());
assert!(parse_sheet(".field:not(.field .icon) { color: black; }").is_err());
```

- [ ] Add public API inspection tests:

```rust
#[test]
fn combinator_selectors_are_structurally_inspectable() {
    let sheet = parse_sheet(".toolbar > button { color: black; }").unwrap();
    let rule = style_rule(&sheet.rules()[0]);
    let CssSelector::Complex(selector) = rule.selector() else {
        panic!("expected complex selector");
    };
    assert_eq!(selector.rest()[0].combinator(), CssSelectorCombinator::Child);
    assert_eq!(selector.rest()[0].selector().tag().map(String::as_str), Some("button"));
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css combinator_selectors
cargo test -p surgeist-css combinator
cargo test -p surgeist-css relative
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 4: Model Media Query Conditions

**Files:**
- Create: `src/parser/queries.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/syntax.rs`
- Modify: `src/tests.rs`

- [ ] Add the Target Media Query Model types and checked non-empty `CssMediaQueryList::try_new`.

- [ ] Parse media query lists independent of `@media` first, through a crate-private `parse_media_query_list`.

- [ ] Support comma-separated media queries:

```css
screen
print
screen and (min-width: 600px)
(width >= 600px)
(orientation: landscape)
(prefers-color-scheme: dark)
(hover: hover) and (pointer: fine)
not screen and (max-width: 400px)
screen, print
```

- [ ] Reject unsupported or malformed conditions:

```rust
assert!(parse_media_query_list_for_test("tv").is_err());
assert!(parse_media_query_list_for_test("(unknown-feature: yes)").is_err());
assert!(parse_media_query_list_for_test("(width: auto)").is_err());
assert!(parse_media_query_list_for_test("(width: min-content)").is_err());
assert!(parse_media_query_list_for_test("(width >= )").is_err());
assert!(parse_media_query_list_for_test("screen and").is_err());
assert!(parse_media_query_list_for_test("screen or print").is_err());
```

- [ ] Add tests proving range/discrete values are structural and inspectable.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css media_query
cargo test -p surgeist-css media_feature
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 5: Parse `@media` Group Rules

**Files:**
- Modify: `src/parser/mod.rs`
- Modify: `src/parser/queries.rs`
- Modify: `src/syntax.rs`
- Modify: `src/tests.rs`
- Modify: `README.md`

- [ ] Add `CssMediaRule`.

- [ ] Add `CssRule::Media(CssMediaRule)` in the same commit as `CssMediaRule`.

- [ ] In the same commit that adds `CssRule::Media`, update style-rule test
  helpers to remain exhaustive now that `CssRule` has more than one variant:

```rust
fn style_rule(rule: &CssRule) -> &CssStyleRule {
    match rule {
        CssRule::Style(rule) => rule,
        unexpected => panic!("expected style rule, got {unexpected:?}"),
    }
}
```

- [ ] Implement `AtRuleParser` support for `@media` prelude and block parsing.

- [ ] Parse nested stylesheets inside `@media` into `Vec<CssRule>`.

- [ ] Accept:

```css
@media screen and (min-width: 600px) {
  .panel { color: black; }
}
```

- [ ] Accept nested `@media` immediately. Accept nested `@container` only after Task 9 adds `@container` rule parsing; until then, reject nested `@container`.

- [ ] Reject unknown media features and invalid nested bodies:

```rust
assert!(parse_sheet("@media (unknown: yes) { .panel { color: black; } }").is_err());
assert!(parse_sheet("@media screen { .panel { made-up: value; } }").is_err());
```

- [ ] Update README to state that media queries are parsed as authored conditions and not evaluated in `surgeist-css`.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css media_rule
cargo test -p surgeist-css media_query
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 6: Model Import Rules

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/tests.rs`

- [ ] Add `CssImportRule`, `CssImportTarget`, `CssImportLayer`, and `CssLayerName`.

- [ ] Add `CssRule::Import(CssImportRule)` in the same commit as `CssImportRule`.

- [ ] Add checked constructors/accessors:

```rust
impl CssImportRule {
    pub const fn target(&self) -> &CssImportTarget;
    pub const fn layer(&self) -> Option<&CssImportLayer>;
    pub const fn media(&self) -> Option<&CssMediaQueryList>;
    pub const fn location(&self) -> CssSourceLocation;
}
```

- [ ] Add model tests:

```rust
#[test]
fn import_layer_name_rejects_empty_components() {
    assert!(CssLayerName::try_new(["theme"]).is_some());
    assert!(CssLayerName::try_new(["theme", "components"]).is_some());
    assert!(CssLayerName::try_new([""]).is_none());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css import_layer_name
cargo test -p surgeist-css import
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 7: Parse `@import` Rules As Contracts

**Files:**
- Modify: `src/parser/mod.rs`
- Modify: `src/parser/queries.rs`
- Modify: `src/syntax.rs` only if parser-facing constructors are needed.
- Modify: `src/tests.rs`
- Modify: `README.md`

- [ ] Parse top-level `@import` with string or URL target:

```css
@import "theme.css";
@import url("layout.css");
@import url(tokens.css) layer;
@import url("components.css") layer(components.buttons);
@import url("print.css") print;
@import url("wide.css") screen and (min-width: 900px);
@import url("components.css") layer(components) screen and (min-width: 900px);
```

- [ ] Preserve target strings exactly as tokenized by `cssparser`; do not resolve paths.

- [ ] Enforce top-level placement: `@import` must appear before non-import top-level rules.

- [ ] Reject:

```rust
assert!(parse_sheet(".panel { color: black; } @import \"late.css\";").is_err());
assert!(parse_sheet("@media screen { @import \"nested.css\"; }").is_err());
assert!(parse_sheet("@import url(\"theme.css\") supports(display: grid);").is_err());
assert!(parse_sheet("@import url(\"theme.css\") screen layer(components);").is_err());
assert!(parse_sheet("@import;").is_err());
```

- [ ] Update README to state that imports are parse-only and loading is owned by root/style.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css import_rule
cargo test -p surgeist-css import
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 8: Model Container Query Conditions

**Files:**
- Modify: `src/parser/queries.rs`
- Modify: `src/syntax.rs`
- Modify: `src/tests.rs`

- [ ] Add the Target Container Query Model types and checked `CssContainerName::try_new`.

- [ ] Parse container conditions independent of `@container` first, through a crate-private `parse_container_condition`.

- [ ] Support:

```css
(width > 600px)
(inline-size >= 30rem)
(aspect-ratio > 1 / 1)
(orientation: landscape)
not (width < 300px)
(width > 600px) and (orientation: landscape)
(width > 600px) or (orientation: portrait)
style(--theme)
style(--theme: dark)
```

- [ ] Reject:

```rust
assert!(parse_container_condition_for_test("(unknown > 1px)").is_err());
assert!(parse_container_condition_for_test("(width: auto)").is_err());
assert!(parse_container_condition_for_test("(width: min-content)").is_err());
assert!(parse_container_condition_for_test("(aspect-ratio: -1 / 1)").is_err());
assert!(parse_container_condition_for_test("(aspect-ratio: 1 / 0)").is_err());
assert!(parse_container_condition_for_test("style(color: red)").is_err());
assert!(parse_container_condition_for_test("scroll-state(stuck: top)").is_err());
assert!(parse_container_condition_for_test("(width > )").is_err());
```

- [ ] Add tests proving style queries preserve custom property names and authored values structurally.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css container_condition
cargo test -p surgeist-css container_style
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 9: Parse `@container` Group Rules

**Files:**
- Modify: `src/parser/mod.rs`
- Modify: `src/parser/queries.rs`
- Modify: `src/syntax.rs` only if parser-facing constructors are needed.
- Modify: `src/tests.rs`
- Modify: `README.md`

- [ ] Add `CssContainerRule`.

- [ ] Add `CssRule::Container(CssContainerRule)` in the same commit as `CssContainerRule`.

- [ ] Parse unnamed and named container rules:

```css
@container (inline-size > 30rem) {
  .card { color: black; }
}

@container sidebar (width >= 300px) {
  .title { color: black; }
}

@container style(--theme: dark) {
  .title { color: black; }
}
```

- [ ] Allow nested `@media` and `@container` group rules in either direction after both at-rules exist.

- [ ] Reject invalid conditions, empty blocks if existing parser treats empty rule bodies as invalid, nested `@import`, and invalid nested declarations:

```rust
assert!(parse_sheet("@container (unknown > 1px) { .card { color: black; } }").is_err());
assert!(parse_sheet("@container (width > 300px) { @import \"x.css\"; }").is_err());
assert!(parse_sheet("@container (width > 300px) { .card { made-up: 1; } }").is_err());
```

- [ ] Update README to state that container queries are parsed as authored conditions and not evaluated in `surgeist-css`.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css container_rule
cargo test -p surgeist-css nested_conditional_rules
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 10: Model Font Face Descriptors

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/tests.rs`

- [ ] Add the Target Font Face Model types.

- [ ] Add `CssRule::FontFace(CssFontFaceRule)` in the same commit as `CssFontFaceRule`.

- [ ] Add checked constructors for non-empty source lists, Unicode ranges, and descriptor collections that require `font-family` and `src`.

- [ ] Model font-face descriptors separately from `CssProperty` and `CssValue`. Descriptors are not normal style declarations.

- [ ] Add model invariant tests:

```rust
#[test]
fn font_face_descriptor_collection_requires_family_and_src() {
    assert!(
        CssFontFaceDescriptors::try_new(None, None, None, None, None, None, None).is_none()
    );
    assert!(CssFontFaceUrlSource::try_new("", None, Vec::new()).is_none());
    assert!(CssFontFaceUrlSource::try_new("   ", None, Vec::new()).is_none());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css font_face_descriptor_collection
cargo test -p surgeist-css font_face
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 11: Parse `@font-face` Rules

**Files:**
- Create: `src/parser/font_face.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/syntax.rs` only if parser-facing constructors are needed.
- Modify: `src/tests.rs`
- Modify: `README.md`

- [ ] Parse:

```css
@font-face {
  font-family: "Inter";
  src: url("inter.woff2") format("woff2");
  font-weight: 400 700;
  font-style: normal;
  font-display: swap;
  unicode-range: U+0000-00FF;
}
```

- [ ] Accept source list forms:

```css
src: local("Inter"), url("inter.woff2") format("woff2"), url("inter-var.woff2") tech(variations);
```

- [ ] Supported descriptors:
  - `font-family`
  - `src`
  - `font-weight`
  - `font-style`
  - `font-stretch`
  - `font-display`
  - `unicode-range`

- [ ] Reject missing required descriptors, duplicate descriptors, unknown descriptors, descriptor declarations outside `@font-face`, and nested rules:

```rust
assert!(parse_sheet("@font-face { font-family: Inter; }").is_err());
assert!(parse_sheet("@font-face { src: url(a.woff2); }").is_err());
assert!(parse_sheet("@font-face { font-family: Inter; src: url(a.woff2); unknown: x; }").is_err());
assert!(parse_sheet("@font-face { font-family: Inter; font-family: Other; src: url(a.woff2); }").is_err());
assert!(parse_sheet(".panel { src: url(a.woff2); }").is_err());
```

- [ ] Update README to state that font faces are parsed as descriptors only; font lookup/loading is downstream.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css font_face_rule
cargo test -p surgeist-css font_face
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 12: Final Strict Matrix And Public API Review Tests

**Files:**
- Modify: `src/tests.rs`
- Modify: `README.md` only if wording needs a small correction.

- [ ] Add a cross-feature acceptance matrix:

```rust
#[test]
fn advanced_css_surface_matrix_accepts_supported_forms() {
    let accepted = [
        ".toolbar > button[aria-expanded=true] { color: black; }",
        ".stack .item:hover { color: black; }",
        "@import url(\"theme.css\") screen and (min-width: 600px);",
        "@media (prefers-color-scheme: dark) { .panel { color: black; } }",
        "@container sidebar (inline-size > 30rem) { .panel { color: black; } }",
        "@font-face { font-family: Inter; src: url(\"inter.woff2\") format(\"woff2\"); }",
    ];

    for css in accepted {
        assert!(parse_sheet(css).is_ok(), "{css} should parse");
    }
}
```

- [ ] Add a rejection matrix:

```rust
#[test]
fn advanced_css_surface_matrix_rejects_unsupported_forms() {
    let rejected = [
        "@supports (display: grid) { .panel { color: black; } }",
        "@keyframes fade { from { opacity: 0; } to { opacity: 1; } }",
        "@import url(\"late.css\"); .panel { color: black; } @import url(\"later.css\");",
        "@import url(\"theme.css\") supports(display: grid);",
        "@font-face { font-family: Inter; }",
        ".field:has(> .icon) { color: black; }",
        "[svg|href] { color: black; }",
        ".col || .cell { color: black; }",
        "@container scroll-state(stuck: top) { .panel { color: black; } }",
    ];

    for css in rejected {
        assert!(parse_sheet(css).is_err(), "{css} should reject");
    }
}
```

- [ ] Add public API inspection tests proving `CssRule::Import`, `CssRule::FontFace`, `CssRule::Media`, `CssRule::Container`, `CssSelector::Complex`, and attribute selectors are structurally accessible without string parsing.

- [ ] Run:

```sh
git status --short --branch
git diff --stat
cargo fmt --check
cargo test -p surgeist-css advanced_css_surface_matrix
cargo test -p surgeist-css structurally_accessible
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 13: Final Checks And Holistic Review

**Files:**
- Inspect all changed files.

- [ ] Run:

```sh
git status --short --branch
git diff --stat
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Assign a clean-context holistic reviewer with no conversation history. Reviewer must inspect:
  - `AGENTS.md`
  - `guidance/surgeist-rust-modeling-guide.md`
  - this plan
  - `src/syntax.rs`
  - `src/parser/mod.rs`
  - `src/parser/selectors.rs`
  - `src/parser/queries.rs`
  - `src/parser/font_face.rs`
  - `src/tests.rs`
  - `README.md`

- [ ] Reviewer must verify:
  - all scoped features are implemented
  - `surgeist-css` remains parse-only for imports, fonts, selectors, and queries
  - strict no-recovery behavior remains intact
  - public APIs are typed and inspectable
  - no loading, fetching, matching, cascade evaluation, font activation, or query evaluation was added
  - unsupported at-rules and selector/query forms still reject

- [ ] Completion requires the holistic reviewer to return `APPROVED` with no unresolved findings.

## Completion Signal

Report:

- plan commit SHA
- implementation commit SHAs if execution follows
- final test count from `cargo test -p surgeist-css`
- checks run
- final holistic reviewer result
- whether the repo is pushed

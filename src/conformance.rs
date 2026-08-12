use crate::{CssErrorCode, CssFeatureId, CssKnownProperty};

/// The grammar-family role of one support-catalog feature.
///
/// This classification identifies authored syntax; it does not enable parsing,
/// perform contextual resolution, or imply support for sibling productions.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssFeatureKind {
    /// An at-rule, qualified rule, or syntax-owned rule metadata production.
    Rule,
    /// A declaration or declaration-list production.
    Declaration,
    /// A descriptor within an owning at-rule.
    Descriptor,
    /// A shared authored-value production.
    Value,
    /// A recognized non-custom authored property production.
    Property,
    /// A selector production or finite selector spelling group.
    Selector,
    /// A media-query production or finite media-query spelling group.
    MediaQuery,
    /// A container-query production or finite container-query spelling group.
    ContainerQuery,
}

/// The bounded implementation state of one exact catalog production.
///
/// The three states are closed for I01: callers can distinguish a complete
/// production, a documented supported subset, and a recognized spelling that
/// is never retained as that production.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssSupportStatus {
    /// The complete identified production is implemented.
    Complete,
    /// Only the record's documented supported subset is implemented.
    Partial,
    /// The spelling is recognized but the production is not implemented.
    RecognizedUnsupported,
}

/// Immutable provenance for one support-catalog record.
///
/// Exactly one of [`Self::url`] and [`Self::repository_provenance`] is present.
/// Values are catalog-owned and cannot be constructed by downstream callers.
///
/// ```compile_fail
/// use surgeist_css::feature_catalog;
///
/// let source = feature_catalog()[0].source();
/// let _ = source.url;
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssSpecificationSource {
    url: Option<&'static str>,
    repository_provenance: Option<&'static str>,
}

impl CssSpecificationSource {
    const fn from_url(value: &'static str) -> Self {
        Self {
            url: Some(value),
            repository_provenance: None,
        }
    }

    const fn from_repository(value: &'static str) -> Self {
        Self {
            url: None,
            repository_provenance: Some(value),
        }
    }

    /// Returns the immutable specification URL, when this record cites a specification.
    #[must_use]
    pub const fn url(self) -> Option<&'static str> {
        self.url
    }

    /// Returns the exact baseline revision and path, when this record cites repository source.
    #[must_use]
    pub const fn repository_provenance(self) -> Option<&'static str> {
        self.repository_provenance
    }
}

/// Immutable support metadata for one atomic parser-facing CSS production.
///
/// Records describe authored syntax only. They do not dispatch the parser,
/// validate source, perform selector/query matching, or resolve authored values.
/// Fields and construction are catalog-owned so stable identity and status
/// invariants cannot be forged by downstream callers.
///
/// ```compile_fail
/// use surgeist_css::feature_catalog;
///
/// let metadata = &feature_catalog()[0];
/// let _ = metadata.id;
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssFeatureMetadata {
    id: CssFeatureId,
    kind: CssFeatureKind,
    spelling: &'static str,
    source: CssSpecificationSource,
    production: &'static str,
    status: CssSupportStatus,
    supported_subset: Option<&'static str>,
    unsupported_remainder: Option<&'static str>,
    recognized_unsupported_code: Option<CssErrorCode>,
    property: Option<CssKnownProperty>,
    property_aliases: &'static [&'static str],
}

impl CssFeatureMetadata {
    const fn complete(
        id: &'static str,
        kind: CssFeatureKind,
        spelling: &'static str,
        source: CssSpecificationSource,
        production: &'static str,
    ) -> Self {
        Self {
            id: CssFeatureId::new(id),
            kind,
            spelling,
            source,
            production,
            status: CssSupportStatus::Complete,
            supported_subset: None,
            unsupported_remainder: None,
            recognized_unsupported_code: None,
            property: None,
            property_aliases: &[],
        }
    }

    const fn partial(
        id: &'static str,
        kind: CssFeatureKind,
        spelling: &'static str,
        source: CssSpecificationSource,
        production: &'static str,
        supported_subset: &'static str,
        unsupported_remainder: &'static str,
    ) -> Self {
        Self {
            id: CssFeatureId::new(id),
            kind,
            spelling,
            source,
            production,
            status: CssSupportStatus::Partial,
            supported_subset: Some(supported_subset),
            unsupported_remainder: Some(unsupported_remainder),
            recognized_unsupported_code: None,
            property: None,
            property_aliases: &[],
        }
    }

    const fn recognized_unsupported(
        id: &'static str,
        kind: CssFeatureKind,
        spelling: &'static str,
        source: CssSpecificationSource,
        production: &'static str,
        code: CssErrorCode,
    ) -> Self {
        Self {
            id: CssFeatureId::new(id),
            kind,
            spelling,
            source,
            production,
            status: CssSupportStatus::RecognizedUnsupported,
            supported_subset: None,
            unsupported_remainder: None,
            recognized_unsupported_code: Some(code),
            property: None,
            property_aliases: &[],
        }
    }

    const fn partial_property(
        id: &'static str,
        property: CssKnownProperty,
        canonical_name: &'static str,
        aliases: &'static [&'static str],
    ) -> Self {
        Self {
            id: CssFeatureId::new(id),
            kind: CssFeatureKind::Property,
            spelling: canonical_name,
            source: BASELINE_PARSER,
            production: canonical_name,
            status: CssSupportStatus::Partial,
            supported_subset: Some(PROPERTY_SUBSET),
            unsupported_remainder: Some(PROPERTY_REMAINDER),
            recognized_unsupported_code: None,
            property: Some(property),
            property_aliases: aliases,
        }
    }

    /// Returns the globally unique stable feature identity.
    #[must_use]
    pub const fn id(&self) -> CssFeatureId {
        self.id
    }

    /// Returns the authored grammar-family role.
    #[must_use]
    pub const fn kind(&self) -> CssFeatureKind {
        self.kind
    }

    /// Returns the exact authored spelling or named grammar production.
    #[must_use]
    pub const fn spelling(&self) -> &'static str {
        self.spelling
    }

    /// Returns the immutable specification or repository provenance.
    #[must_use]
    pub const fn source(&self) -> CssSpecificationSource {
        self.source
    }

    /// Returns the exact cited production or section name.
    #[must_use]
    pub const fn production(&self) -> &'static str {
        self.production
    }

    /// Returns the bounded implementation state.
    #[must_use]
    pub const fn status(&self) -> CssSupportStatus {
        self.status
    }

    /// Returns the non-empty implemented-subset description exactly for partial records.
    #[must_use]
    pub const fn supported_subset(&self) -> Option<&'static str> {
        self.supported_subset
    }

    /// Returns the non-empty valid-but-unsupported description exactly for partial records.
    #[must_use]
    pub const fn unsupported_remainder(&self) -> Option<&'static str> {
        self.unsupported_remainder
    }

    /// Returns the emitted root diagnostic code exactly for recognized-unsupported records.
    ///
    /// The complete diagnostic identity is this code together with [`Self::id`].
    #[must_use]
    pub const fn recognized_unsupported_code(&self) -> Option<CssErrorCode> {
        self.recognized_unsupported_code
    }
}

/// Immutable catalog metadata for one recognized non-custom authored CSS property.
///
/// The metadata reports parser support; it does not apply cascade, substitute
/// variables, resolve values, or dispatch property parsing. Construction is
/// catalog-owned so callers cannot forge a property-to-feature association.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssPropertyMetadata {
    feature: &'static CssFeatureMetadata,
}

impl CssPropertyMetadata {
    /// Returns the underlying support-catalog record.
    #[must_use]
    pub const fn feature(&self) -> &'static CssFeatureMetadata {
        self.feature
    }

    /// Returns the canonical authored property identity.
    #[must_use]
    pub fn property(&self) -> CssKnownProperty {
        self.feature
            .property
            .expect("property metadata always wraps a property catalog record")
    }

    /// Returns the canonical lowercase authored property spelling.
    #[must_use]
    pub const fn canonical_name(&self) -> &'static str {
        self.feature.spelling
    }

    /// Returns reviewed authored aliases that normalize to this property.
    #[must_use]
    pub const fn aliases(&self) -> &'static [&'static str] {
        self.feature.property_aliases
    }
}

const CSS_SYNTAX_3: CssSpecificationSource =
    CssSpecificationSource::from_url("https://www.w3.org/TR/2021/CRD-css-syntax-3-20211224/");
const CSS_STYLE_ATTRIBUTES: CssSpecificationSource =
    CssSpecificationSource::from_url("https://www.w3.org/TR/2013/REC-css-style-attr-20131107/");
const CSS_CASCADE_4: CssSpecificationSource =
    CssSpecificationSource::from_url("https://www.w3.org/TR/2022/CR-css-cascade-4-20220113/");
const CSS_NAMESPACES_3: CssSpecificationSource =
    CssSpecificationSource::from_url("https://www.w3.org/TR/css3-namespace/");
const CSS_CONDITIONAL_3: CssSpecificationSource =
    CssSpecificationSource::from_url("https://www.w3.org/TR/css-conditional-3/");
const CSS_COUNTER_STYLES_3: CssSpecificationSource =
    CssSpecificationSource::from_url("https://www.w3.org/TR/css-counter-styles-3/");
const CSS_2_PAGE: CssSpecificationSource =
    CssSpecificationSource::from_url("https://www.w3.org/TR/CSS2/page.html");
const CSS_FONTS_4: CssSpecificationSource =
    CssSpecificationSource::from_url("https://www.w3.org/TR/css-fonts-4/");

const BASELINE_PARSER: CssSpecificationSource =
    CssSpecificationSource::from_repository("4b288d6:src/parser/mod.rs");
const BASELINE_FONT_FACE: CssSpecificationSource =
    CssSpecificationSource::from_repository("4b288d6:src/parser/font_face.rs");
const BASELINE_KEYFRAMES: CssSpecificationSource =
    CssSpecificationSource::from_repository("4b288d6:src/parser/keyframes.rs");
const BASELINE_VARIABLES: CssSpecificationSource =
    CssSpecificationSource::from_repository("4b288d6:src/parser/variables.rs");
const BASELINE_SELECTORS: CssSpecificationSource =
    CssSpecificationSource::from_repository("4b288d6:src/parser/selectors.rs");
const BASELINE_NESTING: CssSpecificationSource =
    CssSpecificationSource::from_repository("4b288d6:src/parser/nesting.rs");
const BASELINE_QUERIES: CssSpecificationSource =
    CssSpecificationSource::from_repository("4b288d6:src/parser/queries.rs");

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
const PROPERTY_SUBSET: &str = "The property-specific parser behavior at 4b288d6:src/parser/mod.rs, plus whole-value CSS-wide keywords and syntactically admissible substitution-dependent authored values, is supported.";
const PROPERTY_REMAINDER: &str =
    "Other valid forms of the cited property production are outside the I01 subset.";

macro_rules! property_feature {
    ($property:path, $canonical_name:literal, $stable_id:literal) => {
        CssFeatureMetadata::partial_property($stable_id, $property, $canonical_name, &[])
    };
}

static FEATURE_CATALOG: [CssFeatureMetadata; 219] = [
    CssFeatureMetadata::partial(
        "baseline.rule.import",
        CssFeatureKind::Rule,
        "@import",
        BASELINE_PARSER,
        "@import rule",
        BASELINE_RULE_SUBSET,
        BASELINE_RULE_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.rule.layer-statement",
        CssFeatureKind::Rule,
        "@layer ...;",
        BASELINE_PARSER,
        "@layer statement rule",
        BASELINE_RULE_SUBSET,
        BASELINE_RULE_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.rule.layer-block",
        CssFeatureKind::Rule,
        "@layer {...}",
        BASELINE_PARSER,
        "@layer block rule",
        BASELINE_RULE_SUBSET,
        BASELINE_RULE_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.rule.font-face",
        CssFeatureKind::Rule,
        "@font-face",
        BASELINE_FONT_FACE,
        "@font-face rule",
        BASELINE_RULE_SUBSET,
        BASELINE_RULE_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.rule.keyframes",
        CssFeatureKind::Rule,
        "@keyframes",
        BASELINE_KEYFRAMES,
        "@keyframes rule",
        BASELINE_RULE_SUBSET,
        BASELINE_RULE_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.rule.style",
        CssFeatureKind::Rule,
        "style and nested qualified rules",
        BASELINE_PARSER,
        "style rule",
        BASELINE_RULE_SUBSET,
        BASELINE_RULE_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.rule.media",
        CssFeatureKind::Rule,
        "@media",
        BASELINE_PARSER,
        "@media rule",
        BASELINE_RULE_SUBSET,
        BASELINE_RULE_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.rule.container",
        CssFeatureKind::Rule,
        "@container",
        BASELINE_PARSER,
        "@container rule",
        BASELINE_RULE_SUBSET,
        BASELINE_RULE_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.rule.scope",
        CssFeatureKind::Rule,
        "@scope",
        BASELINE_PARSER,
        "@scope rule",
        BASELINE_RULE_SUBSET,
        BASELINE_RULE_REMAINDER,
    ),
    CssFeatureMetadata::complete(
        "foundation.encoding.charset",
        CssFeatureKind::Rule,
        "optional leading legacy @charset metadata",
        CSS_SYNTAX_3,
        "CSS Syntax 3 section 3 input byte stream",
    ),
    CssFeatureMetadata::complete(
        "foundation.declaration-list.style-attribute",
        CssFeatureKind::Declaration,
        "style-attribute declaration-list structure",
        CSS_STYLE_ATTRIBUTES,
        "style attribute",
    ),
    CssFeatureMetadata::complete(
        "foundation.declaration.importance",
        CssFeatureKind::Declaration,
        "terminal declaration !important annotation",
        CSS_CASCADE_4,
        "important declaration",
    ),
    CssFeatureMetadata::partial(
        "baseline.declaration.custom-property",
        CssFeatureKind::Declaration,
        "custom-property names and authored token streams",
        BASELINE_VARIABLES,
        "custom-property declaration",
        "Baseline custom-property names and authored token streams, including I01 recovery behavior, are supported.",
        "Other valid CSS Variables custom-property declaration forms are outside the I01 subset.",
    ),
    CssFeatureMetadata::partial(
        "baseline.value.substitution-dependent",
        CssFeatureKind::Value,
        "preserved known-property values containing substitution functions",
        BASELINE_VARIABLES,
        "substitution-dependent declaration value",
        "Known-property values with syntactically admissible var() references remain authored and symbolic.",
        "Other valid CSS Variables substitution functions and post-substitution forms are outside the I01 subset.",
    ),
    CssFeatureMetadata::recognized_unsupported(
        "later.rule.namespace",
        CssFeatureKind::Rule,
        "@namespace",
        CSS_NAMESPACES_3,
        "namespace declaration",
        CssErrorCode::UnsupportedAtRule,
    ),
    CssFeatureMetadata::recognized_unsupported(
        "later.rule.supports",
        CssFeatureKind::Rule,
        "@supports",
        CSS_CONDITIONAL_3,
        "@supports rule",
        CssErrorCode::UnsupportedAtRule,
    ),
    CssFeatureMetadata::recognized_unsupported(
        "later.rule.counter-style",
        CssFeatureKind::Rule,
        "@counter-style",
        CSS_COUNTER_STYLES_3,
        "@counter-style rule",
        CssErrorCode::UnsupportedAtRule,
    ),
    CssFeatureMetadata::recognized_unsupported(
        "later.rule.page",
        CssFeatureKind::Rule,
        "@page",
        CSS_2_PAGE,
        "page rule",
        CssErrorCode::UnsupportedAtRule,
    ),
    CssFeatureMetadata::recognized_unsupported(
        "later.rule.font-feature-values",
        CssFeatureKind::Rule,
        "@font-feature-values",
        CSS_FONTS_4,
        "@font-feature-values rule",
        CssErrorCode::UnsupportedAtRule,
    ),
    CssFeatureMetadata::partial(
        "baseline.descriptor.font-family",
        CssFeatureKind::Descriptor,
        "font-family in @font-face",
        BASELINE_FONT_FACE,
        "font-family descriptor",
        DESCRIPTOR_SUBSET,
        DESCRIPTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.descriptor.src",
        CssFeatureKind::Descriptor,
        "src in @font-face",
        BASELINE_FONT_FACE,
        "src descriptor",
        DESCRIPTOR_SUBSET,
        DESCRIPTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.descriptor.font-weight",
        CssFeatureKind::Descriptor,
        "font-weight in @font-face",
        BASELINE_FONT_FACE,
        "font-weight descriptor",
        DESCRIPTOR_SUBSET,
        DESCRIPTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.descriptor.font-style",
        CssFeatureKind::Descriptor,
        "font-style in @font-face",
        BASELINE_FONT_FACE,
        "font-style descriptor",
        DESCRIPTOR_SUBSET,
        DESCRIPTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.descriptor.font-stretch",
        CssFeatureKind::Descriptor,
        "font-stretch in @font-face",
        BASELINE_FONT_FACE,
        "font-stretch descriptor",
        DESCRIPTOR_SUBSET,
        DESCRIPTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.descriptor.font-display",
        CssFeatureKind::Descriptor,
        "font-display in @font-face",
        BASELINE_FONT_FACE,
        "font-display descriptor",
        DESCRIPTOR_SUBSET,
        DESCRIPTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.descriptor.unicode-range",
        CssFeatureKind::Descriptor,
        "unicode-range in @font-face",
        BASELINE_FONT_FACE,
        "unicode-range descriptor",
        DESCRIPTOR_SUBSET,
        DESCRIPTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.selector.complex",
        CssFeatureKind::Selector,
        "type, universal, ID, class; presence and six valued attribute matchers; descendant, child, next-sibling, subsequent-sibling combinators",
        BASELINE_SELECTORS,
        "complex selector",
        "The exact baseline-recognized complex-selector spelling group is supported.",
        SELECTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.selector.pseudo-class",
        CssFeatureKind::Selector,
        ":root, :hover, :active, :focus, :disabled, :enabled, :checked, :first-child, :last-child, :only-child, :empty, :first-of-type, :last-of-type, :only-of-type",
        BASELINE_SELECTORS,
        "baseline pseudo-class selector",
        "The exact baseline-recognized pseudo-class spelling group is supported.",
        SELECTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.selector.functional",
        CssFeatureKind::Selector,
        ":nth-child(), :nth-last-child(), :nth-of-type(), :nth-last-of-type(), :not()",
        BASELINE_SELECTORS,
        "baseline functional pseudo-class selector",
        "The exact baseline-recognized functional pseudo-class spelling group is supported.",
        SELECTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.selector.extension-state",
        CssFeatureKind::Selector,
        ":scope, :focus-visible, :focus-within, :required, :optional, :valid, :invalid, :placeholder-shown, :default, :indeterminate, :read-only, :read-write, :in-range, :out-of-range, :modal, :fullscreen, :popover-open",
        BASELINE_SELECTORS,
        "extension state pseudo-class selector",
        "The exact I01 extension-state pseudo-class spelling group is supported.",
        SELECTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.selector.extension-functional",
        CssFeatureKind::Selector,
        ":is(), :where(), complex :not(), :has(), and nth-child of lists",
        BASELINE_SELECTORS,
        "extension functional pseudo-class selector",
        "The exact I01 extension-functional pseudo-class spelling group is supported.",
        SELECTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.selector.attribute-case",
        CssFeatureKind::Selector,
        "i and s attribute-selector modifiers",
        BASELINE_SELECTORS,
        "attribute-selector case-sensitivity modifier",
        "The i and s attribute-selector case modifiers are supported.",
        SELECTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.selector.pseudo-element",
        CssFeatureKind::Selector,
        "::before, ::after, ::marker, ::selection, ::backdrop, and generated ::marker sequences",
        BASELINE_SELECTORS,
        "pseudo-element selector",
        "The exact baseline-recognized pseudo-element spelling group is supported.",
        SELECTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.selector.nesting",
        CssFeatureKind::Selector,
        "nesting &, scoped selector anchors, and scoped relative selectors",
        BASELINE_NESTING,
        "nesting selector",
        "Nesting &, scoped selector anchors, and scoped relative selectors are supported.",
        SELECTOR_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.media.query-list",
        CssFeatureKind::MediaQuery,
        "typed/condition query lists, not/only, and/or/not, range and colon forms, and malformed-member Never recovery",
        BASELINE_QUERIES,
        "media query list",
        "The exact baseline-recognized media query-list spelling group and malformed-member Never recovery are supported.",
        QUERY_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.media.type",
        CssFeatureKind::MediaQuery,
        "all, screen, print",
        BASELINE_QUERIES,
        "media type",
        "The all, screen, and print media types are supported.",
        QUERY_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.media.range-feature",
        CssFeatureKind::MediaQuery,
        "width, height, resolution, color, monochrome and their min-/max- names",
        BASELINE_QUERIES,
        "media range feature",
        "The exact baseline-recognized media range-feature spelling group is supported.",
        QUERY_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.media.discrete-feature",
        CssFeatureKind::MediaQuery,
        "orientation, prefers-color-scheme, prefers-reduced-motion, prefers-reduced-transparency, prefers-contrast, forced-colors, hover, any-hover, pointer, any-pointer, display-mode",
        BASELINE_QUERIES,
        "media discrete feature",
        "The exact baseline-recognized media discrete-feature spelling group is supported.",
        QUERY_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.container.condition",
        CssFeatureKind::ContainerQuery,
        "and/or/not, size features, and custom-property style existence/equality",
        BASELINE_QUERIES,
        "container condition",
        "The exact baseline-recognized container-condition spelling group is supported.",
        QUERY_REMAINDER,
    ),
    CssFeatureMetadata::partial(
        "baseline.container.size-feature",
        CssFeatureKind::ContainerQuery,
        "width, height, inline-size, block-size, aspect-ratio, orientation and applicable min-/max- names",
        BASELINE_QUERIES,
        "container size feature",
        "The exact baseline-recognized container size-feature spelling group is supported.",
        QUERY_REMAINDER,
    ),
    property_feature!(CssKnownProperty::All, "all", "baseline.property.all"),
    property_feature!(
        CssKnownProperty::Display,
        "display",
        "baseline.property.display"
    ),
    property_feature!(
        CssKnownProperty::BoxSizing,
        "box-sizing",
        "baseline.property.box-sizing"
    ),
    property_feature!(
        CssKnownProperty::Position,
        "position",
        "baseline.property.position"
    ),
    property_feature!(
        CssKnownProperty::Direction,
        "direction",
        "baseline.property.direction"
    ),
    property_feature!(
        CssKnownProperty::Overflow,
        "overflow",
        "baseline.property.overflow"
    ),
    property_feature!(
        CssKnownProperty::OverflowX,
        "overflow-x",
        "baseline.property.overflow-x"
    ),
    property_feature!(
        CssKnownProperty::OverflowY,
        "overflow-y",
        "baseline.property.overflow-y"
    ),
    property_feature!(
        CssKnownProperty::FlexDirection,
        "flex-direction",
        "baseline.property.flex-direction"
    ),
    property_feature!(
        CssKnownProperty::FlexWrap,
        "flex-wrap",
        "baseline.property.flex-wrap"
    ),
    property_feature!(CssKnownProperty::Float, "float", "baseline.property.float"),
    property_feature!(CssKnownProperty::Clear, "clear", "baseline.property.clear"),
    property_feature!(
        CssKnownProperty::AlignContent,
        "align-content",
        "baseline.property.align-content"
    ),
    property_feature!(
        CssKnownProperty::JustifyContent,
        "justify-content",
        "baseline.property.justify-content"
    ),
    property_feature!(
        CssKnownProperty::AlignItems,
        "align-items",
        "baseline.property.align-items"
    ),
    property_feature!(
        CssKnownProperty::AlignSelf,
        "align-self",
        "baseline.property.align-self"
    ),
    property_feature!(
        CssKnownProperty::JustifyItems,
        "justify-items",
        "baseline.property.justify-items"
    ),
    property_feature!(
        CssKnownProperty::JustifySelf,
        "justify-self",
        "baseline.property.justify-self"
    ),
    property_feature!(
        CssKnownProperty::PlaceContent,
        "place-content",
        "baseline.property.place-content"
    ),
    property_feature!(
        CssKnownProperty::PlaceItems,
        "place-items",
        "baseline.property.place-items"
    ),
    property_feature!(
        CssKnownProperty::PlaceSelf,
        "place-self",
        "baseline.property.place-self"
    ),
    property_feature!(
        CssKnownProperty::Visibility,
        "visibility",
        "baseline.property.visibility"
    ),
    property_feature!(
        CssKnownProperty::Content,
        "content",
        "baseline.property.content"
    ),
    property_feature!(
        CssKnownProperty::ContentVisibility,
        "content-visibility",
        "baseline.property.content-visibility"
    ),
    property_feature!(
        CssKnownProperty::ListStyleType,
        "list-style-type",
        "baseline.property.list-style-type"
    ),
    property_feature!(
        CssKnownProperty::ListStylePosition,
        "list-style-position",
        "baseline.property.list-style-position"
    ),
    property_feature!(
        CssKnownProperty::ListStyleImage,
        "list-style-image",
        "baseline.property.list-style-image"
    ),
    property_feature!(
        CssKnownProperty::ListStyle,
        "list-style",
        "baseline.property.list-style"
    ),
    property_feature!(
        CssKnownProperty::CounterReset,
        "counter-reset",
        "baseline.property.counter-reset"
    ),
    property_feature!(
        CssKnownProperty::CounterIncrement,
        "counter-increment",
        "baseline.property.counter-increment"
    ),
    property_feature!(
        CssKnownProperty::CounterSet,
        "counter-set",
        "baseline.property.counter-set"
    ),
    property_feature!(CssKnownProperty::Width, "width", "baseline.property.width"),
    property_feature!(
        CssKnownProperty::Height,
        "height",
        "baseline.property.height"
    ),
    property_feature!(
        CssKnownProperty::MinWidth,
        "min-width",
        "baseline.property.min-width"
    ),
    property_feature!(
        CssKnownProperty::MinHeight,
        "min-height",
        "baseline.property.min-height"
    ),
    property_feature!(
        CssKnownProperty::MaxWidth,
        "max-width",
        "baseline.property.max-width"
    ),
    property_feature!(
        CssKnownProperty::MaxHeight,
        "max-height",
        "baseline.property.max-height"
    ),
    property_feature!(
        CssKnownProperty::FlexBasis,
        "flex-basis",
        "baseline.property.flex-basis"
    ),
    property_feature!(CssKnownProperty::Gap, "gap", "baseline.property.gap"),
    property_feature!(
        CssKnownProperty::RowGap,
        "row-gap",
        "baseline.property.row-gap"
    ),
    property_feature!(
        CssKnownProperty::ColumnGap,
        "column-gap",
        "baseline.property.column-gap"
    ),
    property_feature!(
        CssKnownProperty::GridFlowTolerance,
        "grid-flow-tolerance",
        "baseline.property.grid-flow-tolerance"
    ),
    property_feature!(
        CssKnownProperty::GridTemplateRows,
        "grid-template-rows",
        "baseline.property.grid-template-rows"
    ),
    property_feature!(
        CssKnownProperty::GridTemplateColumns,
        "grid-template-columns",
        "baseline.property.grid-template-columns"
    ),
    property_feature!(
        CssKnownProperty::GridTemplateAreas,
        "grid-template-areas",
        "baseline.property.grid-template-areas"
    ),
    property_feature!(
        CssKnownProperty::GridTemplate,
        "grid-template",
        "baseline.property.grid-template"
    ),
    property_feature!(
        CssKnownProperty::GridAutoRows,
        "grid-auto-rows",
        "baseline.property.grid-auto-rows"
    ),
    property_feature!(
        CssKnownProperty::GridAutoColumns,
        "grid-auto-columns",
        "baseline.property.grid-auto-columns"
    ),
    property_feature!(
        CssKnownProperty::GridAutoFlow,
        "grid-auto-flow",
        "baseline.property.grid-auto-flow"
    ),
    property_feature!(
        CssKnownProperty::GridRowStart,
        "grid-row-start",
        "baseline.property.grid-row-start"
    ),
    property_feature!(
        CssKnownProperty::GridRowEnd,
        "grid-row-end",
        "baseline.property.grid-row-end"
    ),
    property_feature!(
        CssKnownProperty::GridColumnStart,
        "grid-column-start",
        "baseline.property.grid-column-start"
    ),
    property_feature!(
        CssKnownProperty::GridColumnEnd,
        "grid-column-end",
        "baseline.property.grid-column-end"
    ),
    property_feature!(
        CssKnownProperty::GridRow,
        "grid-row",
        "baseline.property.grid-row"
    ),
    property_feature!(
        CssKnownProperty::GridColumn,
        "grid-column",
        "baseline.property.grid-column"
    ),
    property_feature!(
        CssKnownProperty::GridArea,
        "grid-area",
        "baseline.property.grid-area"
    ),
    property_feature!(CssKnownProperty::Grid, "grid", "baseline.property.grid"),
    property_feature!(
        CssKnownProperty::FontSize,
        "font-size",
        "baseline.property.font-size"
    ),
    property_feature!(
        CssKnownProperty::LineHeight,
        "line-height",
        "baseline.property.line-height"
    ),
    property_feature!(
        CssKnownProperty::WritingMode,
        "writing-mode",
        "baseline.property.writing-mode"
    ),
    property_feature!(
        CssKnownProperty::TextAlign,
        "text-align",
        "baseline.property.text-align"
    ),
    property_feature!(
        CssKnownProperty::TextAlignLast,
        "text-align-last",
        "baseline.property.text-align-last"
    ),
    property_feature!(
        CssKnownProperty::TextIndent,
        "text-indent",
        "baseline.property.text-indent"
    ),
    property_feature!(
        CssKnownProperty::VerticalAlign,
        "vertical-align",
        "baseline.property.vertical-align"
    ),
    property_feature!(
        CssKnownProperty::FontFamily,
        "font-family",
        "baseline.property.font-family"
    ),
    property_feature!(CssKnownProperty::Font, "font", "baseline.property.font"),
    property_feature!(
        CssKnownProperty::FontWeight,
        "font-weight",
        "baseline.property.font-weight"
    ),
    property_feature!(
        CssKnownProperty::FontStyle,
        "font-style",
        "baseline.property.font-style"
    ),
    property_feature!(
        CssKnownProperty::FontStretch,
        "font-stretch",
        "baseline.property.font-stretch"
    ),
    property_feature!(
        CssKnownProperty::FontVariant,
        "font-variant",
        "baseline.property.font-variant"
    ),
    property_feature!(
        CssKnownProperty::FontFeatureSettings,
        "font-feature-settings",
        "baseline.property.font-feature-settings"
    ),
    property_feature!(
        CssKnownProperty::LetterSpacing,
        "letter-spacing",
        "baseline.property.letter-spacing"
    ),
    property_feature!(
        CssKnownProperty::TextWrap,
        "text-wrap",
        "baseline.property.text-wrap"
    ),
    property_feature!(
        CssKnownProperty::WhiteSpace,
        "white-space",
        "baseline.property.white-space"
    ),
    property_feature!(
        CssKnownProperty::WordBreak,
        "word-break",
        "baseline.property.word-break"
    ),
    property_feature!(
        CssKnownProperty::OverflowWrap,
        "overflow-wrap",
        "baseline.property.overflow-wrap"
    ),
    property_feature!(
        CssKnownProperty::TextOverflow,
        "text-overflow",
        "baseline.property.text-overflow"
    ),
    property_feature!(
        CssKnownProperty::TextDecoration,
        "text-decoration",
        "baseline.property.text-decoration"
    ),
    property_feature!(
        CssKnownProperty::TextDecorationLine,
        "text-decoration-line",
        "baseline.property.text-decoration-line"
    ),
    property_feature!(
        CssKnownProperty::TextDecorationColor,
        "text-decoration-color",
        "baseline.property.text-decoration-color"
    ),
    property_feature!(
        CssKnownProperty::TextDecorationStyle,
        "text-decoration-style",
        "baseline.property.text-decoration-style"
    ),
    property_feature!(
        CssKnownProperty::TextDecorationThickness,
        "text-decoration-thickness",
        "baseline.property.text-decoration-thickness"
    ),
    property_feature!(
        CssKnownProperty::TextTransform,
        "text-transform",
        "baseline.property.text-transform"
    ),
    property_feature!(CssKnownProperty::Inset, "inset", "baseline.property.inset"),
    property_feature!(CssKnownProperty::Top, "top", "baseline.property.top"),
    property_feature!(CssKnownProperty::Right, "right", "baseline.property.right"),
    property_feature!(
        CssKnownProperty::Bottom,
        "bottom",
        "baseline.property.bottom"
    ),
    property_feature!(CssKnownProperty::Left, "left", "baseline.property.left"),
    property_feature!(
        CssKnownProperty::ZIndex,
        "z-index",
        "baseline.property.z-index"
    ),
    property_feature!(
        CssKnownProperty::BoxDecorationBreak,
        "box-decoration-break",
        "baseline.property.box-decoration-break"
    ),
    property_feature!(
        CssKnownProperty::Margin,
        "margin",
        "baseline.property.margin"
    ),
    property_feature!(
        CssKnownProperty::MarginTop,
        "margin-top",
        "baseline.property.margin-top"
    ),
    property_feature!(
        CssKnownProperty::MarginRight,
        "margin-right",
        "baseline.property.margin-right"
    ),
    property_feature!(
        CssKnownProperty::MarginBottom,
        "margin-bottom",
        "baseline.property.margin-bottom"
    ),
    property_feature!(
        CssKnownProperty::MarginLeft,
        "margin-left",
        "baseline.property.margin-left"
    ),
    property_feature!(
        CssKnownProperty::Padding,
        "padding",
        "baseline.property.padding"
    ),
    property_feature!(
        CssKnownProperty::PaddingTop,
        "padding-top",
        "baseline.property.padding-top"
    ),
    property_feature!(
        CssKnownProperty::PaddingRight,
        "padding-right",
        "baseline.property.padding-right"
    ),
    property_feature!(
        CssKnownProperty::PaddingBottom,
        "padding-bottom",
        "baseline.property.padding-bottom"
    ),
    property_feature!(
        CssKnownProperty::PaddingLeft,
        "padding-left",
        "baseline.property.padding-left"
    ),
    property_feature!(
        CssKnownProperty::Border,
        "border",
        "baseline.property.border"
    ),
    property_feature!(
        CssKnownProperty::BorderTop,
        "border-top",
        "baseline.property.border-top"
    ),
    property_feature!(
        CssKnownProperty::BorderRight,
        "border-right",
        "baseline.property.border-right"
    ),
    property_feature!(
        CssKnownProperty::BorderBottom,
        "border-bottom",
        "baseline.property.border-bottom"
    ),
    property_feature!(
        CssKnownProperty::BorderLeft,
        "border-left",
        "baseline.property.border-left"
    ),
    property_feature!(
        CssKnownProperty::BorderWidth,
        "border-width",
        "baseline.property.border-width"
    ),
    property_feature!(
        CssKnownProperty::BorderTopWidth,
        "border-top-width",
        "baseline.property.border-top-width"
    ),
    property_feature!(
        CssKnownProperty::BorderRightWidth,
        "border-right-width",
        "baseline.property.border-right-width"
    ),
    property_feature!(
        CssKnownProperty::BorderBottomWidth,
        "border-bottom-width",
        "baseline.property.border-bottom-width"
    ),
    property_feature!(
        CssKnownProperty::BorderLeftWidth,
        "border-left-width",
        "baseline.property.border-left-width"
    ),
    property_feature!(CssKnownProperty::Color, "color", "baseline.property.color"),
    property_feature!(
        CssKnownProperty::Background,
        "background",
        "baseline.property.background"
    ),
    property_feature!(
        CssKnownProperty::BackgroundColor,
        "background-color",
        "baseline.property.background-color"
    ),
    property_feature!(
        CssKnownProperty::BorderColor,
        "border-color",
        "baseline.property.border-color"
    ),
    property_feature!(
        CssKnownProperty::BorderTopColor,
        "border-top-color",
        "baseline.property.border-top-color"
    ),
    property_feature!(
        CssKnownProperty::BorderRightColor,
        "border-right-color",
        "baseline.property.border-right-color"
    ),
    property_feature!(
        CssKnownProperty::BorderBottomColor,
        "border-bottom-color",
        "baseline.property.border-bottom-color"
    ),
    property_feature!(
        CssKnownProperty::BorderLeftColor,
        "border-left-color",
        "baseline.property.border-left-color"
    ),
    property_feature!(
        CssKnownProperty::BackgroundImage,
        "background-image",
        "baseline.property.background-image"
    ),
    property_feature!(
        CssKnownProperty::BackgroundPosition,
        "background-position",
        "baseline.property.background-position"
    ),
    property_feature!(
        CssKnownProperty::BackgroundSize,
        "background-size",
        "baseline.property.background-size"
    ),
    property_feature!(
        CssKnownProperty::BackgroundRepeat,
        "background-repeat",
        "baseline.property.background-repeat"
    ),
    property_feature!(
        CssKnownProperty::BackgroundOrigin,
        "background-origin",
        "baseline.property.background-origin"
    ),
    property_feature!(
        CssKnownProperty::BackgroundClip,
        "background-clip",
        "baseline.property.background-clip"
    ),
    property_feature!(
        CssKnownProperty::BackgroundAttachment,
        "background-attachment",
        "baseline.property.background-attachment"
    ),
    property_feature!(
        CssKnownProperty::BorderStyle,
        "border-style",
        "baseline.property.border-style"
    ),
    property_feature!(
        CssKnownProperty::BorderTopStyle,
        "border-top-style",
        "baseline.property.border-top-style"
    ),
    property_feature!(
        CssKnownProperty::BorderRightStyle,
        "border-right-style",
        "baseline.property.border-right-style"
    ),
    property_feature!(
        CssKnownProperty::BorderBottomStyle,
        "border-bottom-style",
        "baseline.property.border-bottom-style"
    ),
    property_feature!(
        CssKnownProperty::BorderLeftStyle,
        "border-left-style",
        "baseline.property.border-left-style"
    ),
    property_feature!(
        CssKnownProperty::BorderRadius,
        "border-radius",
        "baseline.property.border-radius"
    ),
    property_feature!(
        CssKnownProperty::BorderTopLeftRadius,
        "border-top-left-radius",
        "baseline.property.border-top-left-radius"
    ),
    property_feature!(
        CssKnownProperty::BorderTopRightRadius,
        "border-top-right-radius",
        "baseline.property.border-top-right-radius"
    ),
    property_feature!(
        CssKnownProperty::BorderBottomRightRadius,
        "border-bottom-right-radius",
        "baseline.property.border-bottom-right-radius"
    ),
    property_feature!(
        CssKnownProperty::BorderBottomLeftRadius,
        "border-bottom-left-radius",
        "baseline.property.border-bottom-left-radius"
    ),
    property_feature!(
        CssKnownProperty::BoxShadow,
        "box-shadow",
        "baseline.property.box-shadow"
    ),
    property_feature!(
        CssKnownProperty::Opacity,
        "opacity",
        "baseline.property.opacity"
    ),
    property_feature!(
        CssKnownProperty::FlexGrow,
        "flex-grow",
        "baseline.property.flex-grow"
    ),
    property_feature!(
        CssKnownProperty::FlexShrink,
        "flex-shrink",
        "baseline.property.flex-shrink"
    ),
    property_feature!(CssKnownProperty::Order, "order", "baseline.property.order"),
    property_feature!(CssKnownProperty::Flex, "flex", "baseline.property.flex"),
    property_feature!(
        CssKnownProperty::JustifyTracks,
        "justify-tracks",
        "baseline.property.justify-tracks"
    ),
    property_feature!(
        CssKnownProperty::AlignTracks,
        "align-tracks",
        "baseline.property.align-tracks"
    ),
    property_feature!(
        CssKnownProperty::AspectRatio,
        "aspect-ratio",
        "baseline.property.aspect-ratio"
    ),
    property_feature!(
        CssKnownProperty::ScrollbarWidth,
        "scrollbar-width",
        "baseline.property.scrollbar-width"
    ),
    property_feature!(
        CssKnownProperty::Cursor,
        "cursor",
        "baseline.property.cursor"
    ),
    property_feature!(
        CssKnownProperty::PointerEvents,
        "pointer-events",
        "baseline.property.pointer-events"
    ),
    property_feature!(
        CssKnownProperty::UserSelect,
        "user-select",
        "baseline.property.user-select"
    ),
    property_feature!(
        CssKnownProperty::Outline,
        "outline",
        "baseline.property.outline"
    ),
    property_feature!(
        CssKnownProperty::OutlineColor,
        "outline-color",
        "baseline.property.outline-color"
    ),
    property_feature!(
        CssKnownProperty::OutlineStyle,
        "outline-style",
        "baseline.property.outline-style"
    ),
    property_feature!(
        CssKnownProperty::OutlineWidth,
        "outline-width",
        "baseline.property.outline-width"
    ),
    property_feature!(
        CssKnownProperty::Transform,
        "transform",
        "baseline.property.transform"
    ),
    property_feature!(
        CssKnownProperty::TransformOrigin,
        "transform-origin",
        "baseline.property.transform-origin"
    ),
    property_feature!(
        CssKnownProperty::Translate,
        "translate",
        "baseline.property.translate"
    ),
    property_feature!(
        CssKnownProperty::Rotate,
        "rotate",
        "baseline.property.rotate"
    ),
    property_feature!(CssKnownProperty::Scale, "scale", "baseline.property.scale"),
    property_feature!(
        CssKnownProperty::Filter,
        "filter",
        "baseline.property.filter"
    ),
    property_feature!(
        CssKnownProperty::BackdropFilter,
        "backdrop-filter",
        "baseline.property.backdrop-filter"
    ),
    property_feature!(
        CssKnownProperty::ClipPath,
        "clip-path",
        "baseline.property.clip-path"
    ),
    property_feature!(CssKnownProperty::Mask, "mask", "baseline.property.mask"),
    property_feature!(
        CssKnownProperty::MaskImage,
        "mask-image",
        "baseline.property.mask-image"
    ),
    property_feature!(
        CssKnownProperty::MaskSize,
        "mask-size",
        "baseline.property.mask-size"
    ),
    property_feature!(
        CssKnownProperty::MaskPosition,
        "mask-position",
        "baseline.property.mask-position"
    ),
    property_feature!(
        CssKnownProperty::MaskRepeat,
        "mask-repeat",
        "baseline.property.mask-repeat"
    ),
    property_feature!(
        CssKnownProperty::TransitionProperty,
        "transition-property",
        "baseline.property.transition-property"
    ),
    property_feature!(
        CssKnownProperty::TransitionDuration,
        "transition-duration",
        "baseline.property.transition-duration"
    ),
    property_feature!(
        CssKnownProperty::TransitionDelay,
        "transition-delay",
        "baseline.property.transition-delay"
    ),
    property_feature!(
        CssKnownProperty::TransitionTimingFunction,
        "transition-timing-function",
        "baseline.property.transition-timing-function"
    ),
    property_feature!(
        CssKnownProperty::Transition,
        "transition",
        "baseline.property.transition"
    ),
    property_feature!(
        CssKnownProperty::AnimationName,
        "animation-name",
        "baseline.property.animation-name"
    ),
    property_feature!(
        CssKnownProperty::AnimationDuration,
        "animation-duration",
        "baseline.property.animation-duration"
    ),
    property_feature!(
        CssKnownProperty::AnimationDelay,
        "animation-delay",
        "baseline.property.animation-delay"
    ),
    property_feature!(
        CssKnownProperty::AnimationTimingFunction,
        "animation-timing-function",
        "baseline.property.animation-timing-function"
    ),
    property_feature!(
        CssKnownProperty::AnimationIterationCount,
        "animation-iteration-count",
        "baseline.property.animation-iteration-count"
    ),
    property_feature!(
        CssKnownProperty::AnimationDirection,
        "animation-direction",
        "baseline.property.animation-direction"
    ),
    property_feature!(
        CssKnownProperty::AnimationFillMode,
        "animation-fill-mode",
        "baseline.property.animation-fill-mode"
    ),
    property_feature!(
        CssKnownProperty::AnimationPlayState,
        "animation-play-state",
        "baseline.property.animation-play-state"
    ),
    property_feature!(
        CssKnownProperty::Animation,
        "animation",
        "baseline.property.animation"
    ),
];

/// Returns the immutable I01 support-catalog records in stable inventory order.
#[must_use]
pub fn feature_catalog() -> &'static [CssFeatureMetadata] {
    &FEATURE_CATALOG
}

/// Returns metadata for an exact stable feature ID.
///
/// Lookup is case-sensitive and performs no trimming, aliasing, parser dispatch,
/// or property-name classification.
#[must_use]
pub fn feature_metadata(id: &str) -> Option<&'static CssFeatureMetadata> {
    FEATURE_CATALOG
        .iter()
        .find(|feature| feature.id.as_str() == id)
}

/// Returns metadata for a recognized non-custom authored property name.
///
/// Canonical names and reviewed aliases use ASCII-case-insensitive matching.
/// Custom-property names and unknown spellings return `None`; this lookup does
/// not parse a declaration or classify its diagnostics.
#[must_use]
pub fn property_metadata(name: &str) -> Option<CssPropertyMetadata> {
    FEATURE_CATALOG
        .iter()
        .filter(|feature| feature.kind == CssFeatureKind::Property)
        .find(|feature| {
            feature.spelling.eq_ignore_ascii_case(name)
                || feature
                    .property_aliases
                    .iter()
                    .any(|alias| alias.eq_ignore_ascii_case(name))
        })
        .map(|feature| CssPropertyMetadata { feature })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::properties::property_implementation_inventory;

    mod catalog_inventory_vectors {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/catalog_inventory/vectors.rs"
        ));
    }

    use catalog_inventory_vectors::{PROPERTY_NEGATIVE_VECTORS, PROPERTY_POSITIVE_VECTORS};

    fn unique_owner_ids<'a>(
        ids: impl IntoIterator<Item = &'a str>,
    ) -> Result<HashSet<&'a str>, &'static str> {
        let mut unique = HashSet::new();
        for id in ids {
            if !unique.insert(id) {
                return Err("duplicate");
            }
        }
        Ok(unique)
    }

    fn compare_owner_ids<'a>(
        catalog: &HashSet<&'a str>,
        ids: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), &'static str> {
        let owner = unique_owner_ids(ids)?;
        if !owner.is_subset(catalog) {
            return Err("extra");
        }
        if !catalog.is_subset(&owner) {
            return Err("omission");
        }
        Ok(())
    }

    #[test]
    fn conformance_catalog_internal_status_and_source_invariants_hold() {
        let mut ids = HashSet::new();
        for feature in feature_catalog() {
            assert!(ids.insert(feature.id().as_str()));
            assert_ne!(feature.spelling(), "");
            assert_ne!(feature.production(), "");
            assert_ne!(
                feature.source().url().is_some(),
                feature.source().repository_provenance().is_some()
            );
            assert_eq!(
                feature.supported_subset().is_some(),
                feature.status() == CssSupportStatus::Partial
            );
            assert_eq!(
                feature.unsupported_remainder().is_some(),
                feature.status() == CssSupportStatus::Partial
            );
            assert_eq!(
                feature.recognized_unsupported_code().is_some(),
                feature.status() == CssSupportStatus::RecognizedUnsupported
            );
        }
        assert_eq!(ids.len(), 219);
    }

    #[test]
    fn catalog_inventory_three_independent_property_owners_close_bidirectionally() {
        let catalog: HashSet<_> = feature_catalog()
            .iter()
            .filter(|feature| feature.kind() == CssFeatureKind::Property)
            .map(|feature| {
                assert_eq!(feature.status(), CssSupportStatus::Partial);
                feature.id().as_str()
            })
            .collect();
        assert_eq!(catalog.len(), 179);

        compare_owner_ids(
            &catalog,
            property_implementation_inventory()
                .iter()
                .map(|row| row.stable_id),
        )
        .expect("property implementation inventory must match the catalog");
        compare_owner_ids(&catalog, PROPERTY_POSITIVE_VECTORS.iter().map(|row| row.id))
            .expect("positive vector manifest must match the catalog");
        compare_owner_ids(&catalog, PROPERTY_NEGATIVE_VECTORS.iter().map(|row| row.id))
            .expect("negative vector manifest must match the catalog");

        for vector in PROPERTY_POSITIVE_VECTORS {
            let metadata = property_metadata(vector.canonical_name)
                .unwrap_or_else(|| panic!("missing positive metadata for `{}`", vector.id));
            assert_eq!(metadata.feature().id().as_str(), vector.id);
            assert!(!vector.authored_value.is_empty());
        }
        for vector in PROPERTY_NEGATIVE_VECTORS {
            let metadata = property_metadata(vector.canonical_name)
                .unwrap_or_else(|| panic!("missing negative metadata for `{}`", vector.id));
            assert_eq!(metadata.feature().id().as_str(), vector.id);
            assert!(!vector.authored_value.is_empty());
        }

        for implementation in property_implementation_inventory() {
            let metadata = property_metadata(implementation.name)
                .unwrap_or_else(|| panic!("missing `{}` metadata", implementation.name));
            assert_eq!(metadata.property(), implementation.known_property);
            assert_eq!(metadata.canonical_name(), implementation.name);
            assert_eq!(metadata.aliases(), implementation.aliases);
            assert_eq!(metadata.feature().id().as_str(), implementation.stable_id);
        }
    }

    #[test]
    fn catalog_inventory_mutation_guards_reject_omission_extra_and_duplicate() {
        let catalog: HashSet<_> = feature_catalog()
            .iter()
            .filter(|feature| feature.kind() == CssFeatureKind::Property)
            .map(|feature| feature.id().as_str())
            .collect();
        let ids: Vec<_> = PROPERTY_POSITIVE_VECTORS.iter().map(|row| row.id).collect();

        assert_eq!(
            compare_owner_ids(&catalog, ids[1..].iter().copied()),
            Err("omission")
        );

        let mut extra = ids.clone();
        extra.push("baseline.property.not-in-the-catalog");
        assert_eq!(compare_owner_ids(&catalog, extra), Err("extra"));

        let mut duplicate = ids;
        duplicate.push(duplicate[0]);
        assert_eq!(compare_owner_ids(&catalog, duplicate), Err("duplicate"));
    }
}

use crate::{CssErrorCode, CssFeatureId};

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

static FEATURE_CATALOG: [CssFeatureMetadata; 40] = [
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
];

/// Returns the immutable I01 support-catalog records in stable inventory order.
///
/// This T1 surface contains the 40 non-property records. Property records are a
/// separate catalog-extension task.
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

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
        assert_eq!(ids.len(), 40);
    }
}

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

/// A stable identity for one immutable conformance source.
///
/// Source identities are registry-owned and cannot be forged by downstream
/// callers.
///
/// ```compile_fail
/// use surgeist_css::CssSpecificationSourceId;
///
/// let _ = CssSpecificationSourceId("O-CSS2");
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssSpecificationSourceId(&'static str);

impl CssSpecificationSourceId {
    const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Returns the exact stable source identity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// The selected conformance-profile tier of a specification source.
///
/// A tier classifies provenance only. It never implies parser support.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssSpecificationTier {
    /// An official dated source in the Snapshot 2026 profile.
    Snapshot2026Official,
    /// A reliable dated source preserved by the Snapshot 2026 profile.
    Snapshot2026Reliable,
    /// A stable dated source preserved by the Snapshot 2026 profile.
    Snapshot2026Stable,
    /// An interoperability source preserved by the Snapshot 2026 profile.
    Snapshot2026Interop,
    /// A deliberately selected Surgeist extension or repository baseline.
    SurgeistExtension,
    /// A standards-track source outside the selected profile.
    LaterStandard,
}

/// Immutable provenance for one support-catalog record.
///
/// Exactly one of [`Self::url`] and [`Self::repository_provenance`] is present.
/// Values are catalog-owned and cannot be constructed by downstream callers.
///
/// ```compile_fail
/// use surgeist_css::{CssSpecificationSource, CssSpecificationTier};
///
/// let _ = CssSpecificationSource {
///     id: todo!(),
///     module: "CSS",
///     level: "3",
///     tier: CssSpecificationTier::LaterStandard,
///     url: None,
///     repository_provenance: None,
/// };
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssSpecificationSource {
    id: CssSpecificationSourceId,
    module: &'static str,
    level: &'static str,
    tier: CssSpecificationTier,
    url: Option<&'static str>,
    repository_provenance: Option<&'static str>,
}

impl CssSpecificationSource {
    const fn from_url(
        id: &'static str,
        module: &'static str,
        level: &'static str,
        tier: CssSpecificationTier,
        value: &'static str,
    ) -> Self {
        Self {
            id: CssSpecificationSourceId::new(id),
            module,
            level,
            tier,
            url: Some(value),
            repository_provenance: None,
        }
    }

    const fn from_repository(
        id: &'static str,
        module: &'static str,
        level: &'static str,
        value: &'static str,
    ) -> Self {
        Self {
            id: CssSpecificationSourceId::new(id),
            module,
            level,
            tier: CssSpecificationTier::SurgeistExtension,
            url: None,
            repository_provenance: Some(value),
        }
    }

    /// Returns the exact stable source identity.
    #[must_use]
    pub const fn id(self) -> CssSpecificationSourceId {
        self.id
    }

    /// Returns the specification module name.
    #[must_use]
    pub const fn module(self) -> &'static str {
        self.module
    }

    /// Returns the module level or exact selected baseline slice.
    #[must_use]
    pub const fn level(self) -> &'static str {
        self.level
    }

    /// Returns the source's conformance-profile tier.
    #[must_use]
    pub const fn tier(self) -> CssSpecificationTier {
        self.tier
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

/// A stable identity for one official conformance exclusion.
///
/// ```compile_fail
/// use surgeist_css::CssConformanceExclusionId;
///
/// let _ = CssConformanceExclusionId("excluded.O-CSS2.informative-audit");
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssConformanceExclusionId(&'static str);

impl CssConformanceExclusionId {
    const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Returns the exact stable exclusion identity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// Why an official source item is not an authored parser-facing production.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssExclusionReason {
    /// The source item is informative rather than a normative production.
    InformativeOnly,
    /// A selected source replaced the item without leaving a current production.
    SupersededWithoutCurrentProduction,
    /// The item specifies semantics outside strict authored CSS syntax.
    OutsideAuthoredSyntaxBoundary,
}

/// A stable identity for a source or production that supersedes an exclusion.
///
/// This semantic identity can name an active feature, a planned official
/// feature, or a selected source when the exclusion covers a broader source
/// area. Values are registry-owned and cannot be forged downstream.
///
/// ```compile_fail
/// use surgeist_css::CssConformanceSupersedingId;
///
/// let _ = CssConformanceSupersedingId("O-SYNTAX3");
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssConformanceSupersedingId(&'static str);

impl CssConformanceSupersedingId {
    const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Returns the exact stable superseding identity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

/// Immutable metadata for one official conformance exclusion.
///
/// Exclusions are audit facts. They do not dispatch the parser, carry a support
/// status, or change how authored input is diagnosed.
///
/// ```compile_fail
/// use surgeist_css::CssExclusionReason;
///
/// let _ = surgeist_css::CssExclusionMetadata {
///     id: todo!(),
///     source: todo!(),
///     production: "example",
///     reason: CssExclusionReason::InformativeOnly,
///     superseding_ids: None,
/// };
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssExclusionMetadata {
    id: CssConformanceExclusionId,
    source: CssSpecificationSource,
    production: &'static str,
    reason: CssExclusionReason,
    superseding_ids: Option<&'static [CssConformanceSupersedingId]>,
}

impl CssExclusionMetadata {
    const fn new(
        id: &'static str,
        source: CssSpecificationSource,
        production: &'static str,
        reason: CssExclusionReason,
        superseding_ids: Option<&'static [CssConformanceSupersedingId]>,
    ) -> Self {
        Self {
            id: CssConformanceExclusionId::new(id),
            source,
            production,
            reason,
            superseding_ids,
        }
    }

    /// Returns the globally unique stable exclusion identity.
    #[must_use]
    pub const fn id(&self) -> CssConformanceExclusionId {
        self.id
    }

    /// Returns the immutable official source containing the excluded item.
    #[must_use]
    pub const fn source(&self) -> CssSpecificationSource {
        self.source
    }

    /// Returns the exact source area, production, or fragment.
    #[must_use]
    pub const fn production(&self) -> &'static str {
        self.production
    }

    /// Returns why the item is excluded from authored syntax coverage.
    #[must_use]
    pub const fn reason(&self) -> CssExclusionReason {
        self.reason
    }

    /// Returns stable IDs that supersede the item, when the contract names them.
    #[must_use]
    pub const fn superseding_ids(&self) -> Option<&'static [CssConformanceSupersedingId]> {
        self.superseding_ids
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

macro_rules! dated_source {
    ($id:literal, $module:literal, $level:literal, $tier:path, $url:literal) => {
        CssSpecificationSource::from_url($id, $module, $level, $tier, $url)
    };
}

const O_CSS2: CssSpecificationSource = dated_source!(
    "O-CSS2",
    "CSS",
    "2.1",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2011/REC-CSS2-20110607/"
);
const O_SYNTAX3: CssSpecificationSource = dated_source!(
    "O-SYNTAX3",
    "CSS Syntax",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2021/CRD-css-syntax-3-20211224/"
);
const O_STYLE_ATTR: CssSpecificationSource = dated_source!(
    "O-STYLE-ATTR",
    "CSS Style Attributes",
    "1",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2013/REC-css-style-attr-20131107/"
);
const O_MEDIA3: CssSpecificationSource = dated_source!(
    "O-MEDIA3",
    "Media Queries",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2024/REC-mediaqueries-3-20240521/"
);
const O_CONDITIONAL3: CssSpecificationSource = dated_source!(
    "O-CONDITIONAL3",
    "CSS Conditional Rules",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2024/CRD-css-conditional-3-20240815/"
);
const O_SELECTORS3: CssSpecificationSource = dated_source!(
    "O-SELECTORS3",
    "Selectors",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2018/REC-selectors-3-20181106/"
);
const O_NAMESPACES3: CssSpecificationSource = dated_source!(
    "O-NAMESPACES3",
    "CSS Namespaces",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2014/REC-css-namespaces-3-20140320/"
);
const O_CASCADE4: CssSpecificationSource = dated_source!(
    "O-CASCADE4",
    "CSS Cascading and Inheritance",
    "4",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2022/CR-css-cascade-4-20220113/"
);
const O_VALUES3: CssSpecificationSource = dated_source!(
    "O-VALUES3",
    "CSS Values and Units",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2024/CRD-css-values-3-20240322/"
);
const O_VARIABLES1: CssSpecificationSource = dated_source!(
    "O-VARIABLES1",
    "CSS Custom Properties for Cascading Variables",
    "1",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2022/CR-css-variables-1-20220616/"
);
const O_BOX3: CssSpecificationSource = dated_source!(
    "O-BOX3",
    "CSS Box Model",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2024/REC-css-box-3-20240411/"
);
const O_COLOR4: CssSpecificationSource = dated_source!(
    "O-COLOR4",
    "CSS Color",
    "4",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2026/CRD-css-color-4-20260326/"
);
const O_BACKGROUNDS3: CssSpecificationSource = dated_source!(
    "O-BACKGROUNDS3",
    "CSS Backgrounds and Borders",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2024/CRD-css-backgrounds-3-20240311/"
);
const O_IMAGES3: CssSpecificationSource = dated_source!(
    "O-IMAGES3",
    "CSS Images",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2023/CRD-css-images-3-20231218/"
);
const O_FONTS3: CssSpecificationSource = dated_source!(
    "O-FONTS3",
    "CSS Fonts",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/"
);
const O_WRITING3: CssSpecificationSource = dated_source!(
    "O-WRITING3",
    "CSS Writing Modes",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2019/REC-css-writing-modes-3-20191210/"
);
const O_MULTICOL1: CssSpecificationSource = dated_source!(
    "O-MULTICOL1",
    "CSS Multi-column Layout",
    "1",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2024/CR-css-multicol-1-20240516/"
);
const O_FLEXBOX1: CssSpecificationSource = dated_source!(
    "O-FLEXBOX1",
    "CSS Flexible Box Layout",
    "1",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/"
);
const O_UI3: CssSpecificationSource = dated_source!(
    "O-UI3",
    "CSS Basic User Interface",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2018/REC-css-ui-3-20180621/"
);
const O_CONTAIN1: CssSpecificationSource = dated_source!(
    "O-CONTAIN1",
    "CSS Containment",
    "1",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2024/REC-css-contain-1-20240625/"
);
const O_TRANSFORMS1: CssSpecificationSource = dated_source!(
    "O-TRANSFORMS1",
    "CSS Transforms",
    "1",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2019/CR-css-transforms-1-20190214/"
);
const O_COMPOSITING1: CssSpecificationSource = dated_source!(
    "O-COMPOSITING1",
    "Compositing and Blending",
    "1",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2024/CRD-compositing-1-20240321/"
);
const O_EASING1: CssSpecificationSource = dated_source!(
    "O-EASING1",
    "CSS Easing Functions",
    "1",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2023/CRD-css-easing-1-20230213/"
);
const O_COUNTERSTYLES3: CssSpecificationSource = dated_source!(
    "O-COUNTERSTYLES3",
    "CSS Counter Styles",
    "3",
    CssSpecificationTier::Snapshot2026Official,
    "https://www.w3.org/TR/2021/CR-css-counter-styles-3-20210727/"
);

macro_rules! profile_source {
    ($name:ident, $id:literal, $module:literal, $level:literal, $tier:path, $url:literal) => {
        const $name: CssSpecificationSource = dated_source!($id, $module, $level, $tier, $url);
    };
}

profile_source!(
    R_MEDIA4,
    "R-MEDIA4",
    "Media Queries",
    "4",
    CssSpecificationTier::Snapshot2026Reliable,
    "https://www.w3.org/TR/2026/CRD-mediaqueries-4-20260219/"
);
profile_source!(
    R_GRID1,
    "R-GRID1",
    "CSS Grid Layout",
    "1",
    CssSpecificationTier::Snapshot2026Reliable,
    "https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/"
);
profile_source!(
    R_GRID2,
    "R-GRID2",
    "CSS Grid Layout",
    "2",
    CssSpecificationTier::Snapshot2026Reliable,
    "https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/"
);
profile_source!(
    R_CASCADE5,
    "R-CASCADE5",
    "CSS Cascading and Inheritance",
    "5",
    CssSpecificationTier::Snapshot2026Reliable,
    "https://www.w3.org/TR/2022/CR-css-cascade-5-20220113/"
);
profile_source!(
    R_CONDITIONAL4,
    "R-CONDITIONAL4",
    "CSS Conditional Rules",
    "4",
    CssSpecificationTier::Snapshot2026Reliable,
    "https://www.w3.org/TR/2025/CRD-css-conditional-4-20250904/"
);
profile_source!(
    S_DISPLAY3,
    "S-DISPLAY3",
    "CSS Display",
    "3",
    CssSpecificationTier::Snapshot2026Stable,
    "https://www.w3.org/TR/2026/CRD-css-display-3-20260605/"
);
profile_source!(
    S_WRITING4,
    "S-WRITING4",
    "CSS Writing Modes",
    "4",
    CssSpecificationTier::Snapshot2026Stable,
    "https://www.w3.org/TR/2019/CR-css-writing-modes-4-20190730/"
);
profile_source!(
    S_BREAK3,
    "S-BREAK3",
    "CSS Fragmentation",
    "3",
    CssSpecificationTier::Snapshot2026Stable,
    "https://www.w3.org/TR/2018/CR-css-break-3-20181204/"
);
profile_source!(
    S_ALIGN3,
    "S-ALIGN3",
    "CSS Box Alignment",
    "3",
    CssSpecificationTier::Snapshot2026Stable,
    "https://www.w3.org/TR/2026/WD-css-align-3-20260130/"
);
profile_source!(
    S_SHAPES1,
    "S-SHAPES1",
    "CSS Shapes",
    "1",
    CssSpecificationTier::Snapshot2026Stable,
    "https://www.w3.org/TR/2025/CRD-css-shapes-1-20250612/"
);
profile_source!(
    S_TEXT3,
    "S-TEXT3",
    "CSS Text",
    "3",
    CssSpecificationTier::Snapshot2026Stable,
    "https://www.w3.org/TR/2026/CRD-css-text-3-20260608/"
);
profile_source!(
    S_TEXTDECOR3,
    "S-TEXTDECOR3",
    "CSS Text Decoration",
    "3",
    CssSpecificationTier::Snapshot2026Stable,
    "https://www.w3.org/TR/2022/CRD-css-text-decor-3-20220505/"
);
profile_source!(
    S_MASKING1,
    "S-MASKING1",
    "CSS Masking",
    "1",
    CssSpecificationTier::Snapshot2026Stable,
    "https://www.w3.org/TR/2021/CRD-css-masking-1-20210805/"
);
profile_source!(
    I_TRANSITIONS1,
    "I-TRANSITIONS1",
    "CSS Transitions",
    "1",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2026/WD-css-transitions-1-20260108/"
);
profile_source!(
    I_ANIMATIONS1,
    "I-ANIMATIONS1",
    "CSS Animations",
    "1",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2023/WD-css-animations-1-20230302/"
);
profile_source!(
    I_FILTER1,
    "I-FILTER1",
    "Filter Effects",
    "1",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2018/WD-filter-effects-1-20181218/"
);
profile_source!(
    I_SIZING3,
    "I-SIZING3",
    "CSS Box Sizing",
    "3",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2021/WD-css-sizing-3-20211217/"
);
profile_source!(
    I_TRANSFORMS2,
    "I-TRANSFORMS2",
    "CSS Transforms",
    "2",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2021/WD-css-transforms-2-20211109/"
);
profile_source!(
    I_LISTS3,
    "I-LISTS3",
    "CSS Lists and Counters",
    "3",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2020/WD-css-lists-3-20201117/"
);
profile_source!(
    I_POSITION3,
    "I-POSITION3",
    "CSS Positioned Layout",
    "3",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2025/WD-css-position-3-20251007/"
);
profile_source!(
    I_FONTS4,
    "I-FONTS4",
    "CSS Fonts",
    "4",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2026/WD-css-fonts-4-20260422/"
);
profile_source!(
    I_COLOR5,
    "I-COLOR5",
    "CSS Color",
    "5",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2026/WD-css-color-5-20260618/"
);
profile_source!(
    I_SELECTORS4,
    "I-SELECTORS4",
    "Selectors",
    "4",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2026/WD-selectors-4-20260122/"
);
profile_source!(
    I_CONTAIN2,
    "I-CONTAIN2",
    "CSS Containment",
    "2",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2022/WD-css-contain-2-20220917/"
);
profile_source!(
    I_NESTING1,
    "I-NESTING1",
    "CSS Nesting",
    "1",
    CssSpecificationTier::Snapshot2026Interop,
    "https://www.w3.org/TR/2026/WD-css-nesting-1-20260122/"
);
profile_source!(
    X_CONDITIONAL5,
    "X-CONDITIONAL5",
    "CSS Conditional Rules",
    "5",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2025/WD-css-conditional-5-20251030/"
);
profile_source!(
    X_CASCADE6,
    "X-CASCADE6",
    "CSS Cascading and Inheritance",
    "6",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2024/WD-css-cascade-6-20240906/"
);
profile_source!(
    X_PSEUDO4,
    "X-PSEUDO4",
    "CSS Pseudo-Elements",
    "4",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2025/WD-css-pseudo-4-20250627/"
);
profile_source!(
    X_VALUES4,
    "X-VALUES4",
    "CSS Values and Units",
    "4",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2024/WD-css-values-4-20240312/"
);
profile_source!(
    X_MEDIA5,
    "X-MEDIA5",
    "Media Queries",
    "5",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2026/WD-mediaqueries-5-20260219/"
);
profile_source!(
    X_OVERFLOW3,
    "X-OVERFLOW3",
    "CSS Overflow",
    "3",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/"
);
profile_source!(
    X_SIZING4,
    "X-SIZING4",
    "CSS Box Sizing",
    "4",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2021/WD-css-sizing-4-20210520/"
);
profile_source!(
    X_TEXT4,
    "X-TEXT4",
    "CSS Text",
    "4",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2026/WD-css-text-4-20260608/"
);
profile_source!(
    X_TEXTDECOR4,
    "X-TEXTDECOR4",
    "CSS Text Decoration",
    "4",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2022/WD-css-text-decor-4-20220504/"
);
profile_source!(
    X_UI4,
    "X-UI4",
    "CSS Basic User Interface",
    "4",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2026/WD-css-ui-4-20260120/"
);
profile_source!(
    X_CONTENT3,
    "X-CONTENT3",
    "CSS Generated Content",
    "3",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2025/WD-css-content-3-20251204/"
);
profile_source!(
    X_FULLSCREEN,
    "X-FULLSCREEN",
    "Fullscreen",
    "unleveled",
    CssSpecificationTier::SurgeistExtension,
    "https://www.w3.org/TR/2012/WD-fullscreen-20120703/"
);

const X_FILTER2_BASE: CssSpecificationSource = CssSpecificationSource::from_repository(
    "X-FILTER2-BASE",
    "Filter Effects",
    "2 baseline subset",
    "bc5394f:src/parser/effects.rs",
);
const X_DISPLAY_MODE_BASE: CssSpecificationSource = CssSpecificationSource::from_repository(
    "X-DISPLAY-MODE-BASE",
    "Media Queries",
    "display-mode baseline subset",
    "bc5394f:src/parser/queries.rs",
);
const X_GRID_TOLERANCE_BASE: CssSpecificationSource = CssSpecificationSource::from_repository(
    "X-GRID-TOLERANCE-BASE",
    "CSS Grid Layout",
    "grid-flow-tolerance baseline subset",
    "bc5394f:src/parser/grid.rs",
);

// These exact I01 sources remain active until T3 atomizes their feature records.
const I01_NAMESPACES3: CssSpecificationSource = dated_source!(
    "I01-NAMESPACES3-MOVING",
    "CSS Namespaces",
    "3 moving reference",
    CssSpecificationTier::LaterStandard,
    "https://www.w3.org/TR/css3-namespace/"
);
const I01_CONDITIONAL3: CssSpecificationSource = dated_source!(
    "I01-CONDITIONAL3-MOVING",
    "CSS Conditional Rules",
    "3 moving reference",
    CssSpecificationTier::LaterStandard,
    "https://www.w3.org/TR/css-conditional-3/"
);
const I01_COUNTERSTYLES3: CssSpecificationSource = dated_source!(
    "I01-COUNTERSTYLES3-MOVING",
    "CSS Counter Styles",
    "3 moving reference",
    CssSpecificationTier::LaterStandard,
    "https://www.w3.org/TR/css-counter-styles-3/"
);
const I01_CSS2_PAGE: CssSpecificationSource = dated_source!(
    "I01-CSS2-PAGE",
    "CSS",
    "2.1 page chapter",
    CssSpecificationTier::LaterStandard,
    "https://www.w3.org/TR/CSS2/page.html"
);
const I01_FONTS4: CssSpecificationSource = dated_source!(
    "I01-FONTS4-MOVING",
    "CSS Fonts",
    "4 moving reference",
    CssSpecificationTier::LaterStandard,
    "https://www.w3.org/TR/css-fonts-4/"
);

const CSS_SYNTAX_3: CssSpecificationSource = O_SYNTAX3;
const CSS_STYLE_ATTRIBUTES: CssSpecificationSource = O_STYLE_ATTR;
const CSS_CASCADE_4: CssSpecificationSource = O_CASCADE4;
const CSS_NAMESPACES_3: CssSpecificationSource = I01_NAMESPACES3;
const CSS_CONDITIONAL_3: CssSpecificationSource = I01_CONDITIONAL3;
const CSS_COUNTER_STYLES_3: CssSpecificationSource = I01_COUNTERSTYLES3;
const CSS_2_PAGE: CssSpecificationSource = I01_CSS2_PAGE;
const CSS_FONTS_4: CssSpecificationSource = I01_FONTS4;

const BASELINE_PARSER: CssSpecificationSource = CssSpecificationSource::from_repository(
    "I01-BASE-PARSER",
    "Surgeist CSS parser",
    "I01 baseline",
    "4b288d6:src/parser/mod.rs",
);
const BASELINE_FONT_FACE: CssSpecificationSource = CssSpecificationSource::from_repository(
    "I01-BASE-FONT-FACE",
    "Surgeist CSS font-face parser",
    "I01 baseline",
    "4b288d6:src/parser/font_face.rs",
);
const BASELINE_KEYFRAMES: CssSpecificationSource = CssSpecificationSource::from_repository(
    "I01-BASE-KEYFRAMES",
    "Surgeist CSS keyframes parser",
    "I01 baseline",
    "4b288d6:src/parser/keyframes.rs",
);
const BASELINE_VARIABLES: CssSpecificationSource = CssSpecificationSource::from_repository(
    "I01-BASE-VARIABLES",
    "Surgeist CSS variables parser",
    "I01 baseline",
    "4b288d6:src/parser/variables.rs",
);
const BASELINE_SELECTORS: CssSpecificationSource = CssSpecificationSource::from_repository(
    "I01-BASE-SELECTORS",
    "Surgeist CSS selectors parser",
    "I01 baseline",
    "4b288d6:src/parser/selectors.rs",
);
const BASELINE_NESTING: CssSpecificationSource = CssSpecificationSource::from_repository(
    "I01-BASE-NESTING",
    "Surgeist CSS nesting parser",
    "I01 baseline",
    "4b288d6:src/parser/nesting.rs",
);
const BASELINE_QUERIES: CssSpecificationSource = CssSpecificationSource::from_repository(
    "I01-BASE-QUERIES",
    "Surgeist CSS query parser",
    "I01 baseline",
    "4b288d6:src/parser/queries.rs",
);

static SPECIFICATION_SOURCES: &[CssSpecificationSource] = &[
    O_CSS2,
    O_SYNTAX3,
    O_STYLE_ATTR,
    O_MEDIA3,
    O_CONDITIONAL3,
    O_SELECTORS3,
    O_NAMESPACES3,
    O_CASCADE4,
    O_VALUES3,
    O_VARIABLES1,
    O_BOX3,
    O_COLOR4,
    O_BACKGROUNDS3,
    O_IMAGES3,
    O_FONTS3,
    O_WRITING3,
    O_MULTICOL1,
    O_FLEXBOX1,
    O_UI3,
    O_CONTAIN1,
    O_TRANSFORMS1,
    O_COMPOSITING1,
    O_EASING1,
    O_COUNTERSTYLES3,
    R_MEDIA4,
    R_GRID1,
    R_GRID2,
    R_CASCADE5,
    R_CONDITIONAL4,
    S_DISPLAY3,
    S_WRITING4,
    S_BREAK3,
    S_ALIGN3,
    S_SHAPES1,
    S_TEXT3,
    S_TEXTDECOR3,
    S_MASKING1,
    I_TRANSITIONS1,
    I_ANIMATIONS1,
    I_FILTER1,
    I_SIZING3,
    I_TRANSFORMS2,
    I_LISTS3,
    I_POSITION3,
    I_FONTS4,
    I_COLOR5,
    I_SELECTORS4,
    I_CONTAIN2,
    I_NESTING1,
    X_CONDITIONAL5,
    X_CASCADE6,
    X_PSEUDO4,
    X_VALUES4,
    X_MEDIA5,
    X_OVERFLOW3,
    X_SIZING4,
    X_TEXT4,
    X_TEXTDECOR4,
    X_UI4,
    X_CONTENT3,
    X_FULLSCREEN,
    X_FILTER2_BASE,
    X_DISPLAY_MODE_BASE,
    X_GRID_TOLERANCE_BASE,
    I01_NAMESPACES3,
    I01_CONDITIONAL3,
    I01_COUNTERSTYLES3,
    I01_CSS2_PAGE,
    I01_FONTS4,
    BASELINE_PARSER,
    BASELINE_FONT_FACE,
    BASELINE_KEYFRAMES,
    BASELINE_VARIABLES,
    BASELINE_SELECTORS,
    BASELINE_NESTING,
    BASELINE_QUERIES,
];

macro_rules! exclusion {
    ($id:expr, $source:ident, $production:expr, $reason:path) => {
        CssExclusionMetadata::new($id, $source, $production, $reason, None)
    };
    ($id:expr, $source:ident, $production:expr, $reason:path, [$($owner:literal),+ $(,)?]) => {
        CssExclusionMetadata::new(
            $id,
            $source,
            $production,
            $reason,
            Some(&[$(CssConformanceSupersedingId::new($owner)),+]),
        )
    };
}

macro_rules! superseded_css2_property {
    ($name:literal, $chapter:literal, $owner:literal) => {
        exclusion!(
            concat!("excluded.O-CSS2.property.", $name),
            O_CSS2,
            concat!($chapter, "#propdef-", $name),
            CssExclusionReason::SupersededWithoutCurrentProduction,
            [$owner]
        )
    };
}

macro_rules! informative_css2_property {
    ($name:literal) => {
        exclusion!(
            concat!("excluded.O-CSS2.informative-property.", $name),
            O_CSS2,
            concat!("aural.html#propdef-", $name),
            CssExclusionReason::InformativeOnly
        )
    };
}

const INFORMATIVE_SOURCE_AUDIT: &str = "examples, explicitly non-normative notes, status/TOC, changelogs, acknowledgments, indexes, bibliography, test inventories, and conformance boilerplate";

macro_rules! informative_source_audit {
    ($source_id:literal, $source:ident) => {
        exclusion!(
            concat!("excluded.", $source_id, ".informative-audit"),
            $source,
            INFORMATIVE_SOURCE_AUDIT,
            CssExclusionReason::InformativeOnly
        )
    };
}

static CONFORMANCE_EXCLUSIONS: &[CssExclusionMetadata] = &[
    exclusion!(
        "excluded.O-WRITING3.property.glyph-orientation-horizontal",
        O_WRITING3,
        "#propdef-glyph-orientation-horizontal",
        CssExclusionReason::SupersededWithoutCurrentProduction,
        ["official.property.text-orientation"]
    ),
    exclusion!(
        "excluded.O-UI3.property.ime-mode",
        O_UI3,
        "#propdef-ime-mode",
        CssExclusionReason::SupersededWithoutCurrentProduction
    ),
    superseded_css2_property!("margin", "box.html", "baseline.property.margin"),
    superseded_css2_property!(
        "margin-bottom",
        "box.html",
        "baseline.property.margin-bottom"
    ),
    superseded_css2_property!("margin-left", "box.html", "baseline.property.margin-left"),
    superseded_css2_property!("margin-right", "box.html", "baseline.property.margin-right"),
    superseded_css2_property!("margin-top", "box.html", "baseline.property.margin-top"),
    superseded_css2_property!("padding", "box.html", "baseline.property.padding"),
    superseded_css2_property!(
        "padding-bottom",
        "box.html",
        "baseline.property.padding-bottom"
    ),
    superseded_css2_property!("padding-left", "box.html", "baseline.property.padding-left"),
    superseded_css2_property!(
        "padding-right",
        "box.html",
        "baseline.property.padding-right"
    ),
    superseded_css2_property!("padding-top", "box.html", "baseline.property.padding-top"),
    superseded_css2_property!("color", "colors.html", "baseline.property.color"),
    superseded_css2_property!("background", "colors.html", "baseline.property.background"),
    superseded_css2_property!(
        "background-attachment",
        "colors.html",
        "baseline.property.background-attachment"
    ),
    superseded_css2_property!(
        "background-color",
        "colors.html",
        "baseline.property.background-color"
    ),
    superseded_css2_property!(
        "background-image",
        "colors.html",
        "baseline.property.background-image"
    ),
    superseded_css2_property!(
        "background-position",
        "colors.html",
        "baseline.property.background-position"
    ),
    superseded_css2_property!(
        "background-repeat",
        "colors.html",
        "baseline.property.background-repeat"
    ),
    superseded_css2_property!("border", "box.html", "baseline.property.border"),
    superseded_css2_property!(
        "border-bottom",
        "box.html",
        "baseline.property.border-bottom"
    ),
    superseded_css2_property!(
        "border-bottom-color",
        "box.html",
        "baseline.property.border-bottom-color"
    ),
    superseded_css2_property!(
        "border-bottom-style",
        "box.html",
        "baseline.property.border-bottom-style"
    ),
    superseded_css2_property!(
        "border-bottom-width",
        "box.html",
        "baseline.property.border-bottom-width"
    ),
    superseded_css2_property!("border-color", "box.html", "baseline.property.border-color"),
    superseded_css2_property!("border-left", "box.html", "baseline.property.border-left"),
    superseded_css2_property!(
        "border-left-color",
        "box.html",
        "baseline.property.border-left-color"
    ),
    superseded_css2_property!(
        "border-left-style",
        "box.html",
        "baseline.property.border-left-style"
    ),
    superseded_css2_property!(
        "border-left-width",
        "box.html",
        "baseline.property.border-left-width"
    ),
    superseded_css2_property!("border-right", "box.html", "baseline.property.border-right"),
    superseded_css2_property!(
        "border-right-color",
        "box.html",
        "baseline.property.border-right-color"
    ),
    superseded_css2_property!(
        "border-right-style",
        "box.html",
        "baseline.property.border-right-style"
    ),
    superseded_css2_property!(
        "border-right-width",
        "box.html",
        "baseline.property.border-right-width"
    ),
    superseded_css2_property!("border-style", "box.html", "baseline.property.border-style"),
    superseded_css2_property!("border-top", "box.html", "baseline.property.border-top"),
    superseded_css2_property!(
        "border-top-color",
        "box.html",
        "baseline.property.border-top-color"
    ),
    superseded_css2_property!(
        "border-top-style",
        "box.html",
        "baseline.property.border-top-style"
    ),
    superseded_css2_property!(
        "border-top-width",
        "box.html",
        "baseline.property.border-top-width"
    ),
    superseded_css2_property!("border-width", "box.html", "baseline.property.border-width"),
    superseded_css2_property!("font", "fonts.html", "baseline.property.font"),
    superseded_css2_property!("font-family", "fonts.html", "baseline.property.font-family"),
    superseded_css2_property!("font-size", "fonts.html", "baseline.property.font-size"),
    superseded_css2_property!("font-style", "fonts.html", "baseline.property.font-style"),
    superseded_css2_property!(
        "font-variant",
        "fonts.html",
        "baseline.property.font-variant"
    ),
    superseded_css2_property!("font-weight", "fonts.html", "baseline.property.font-weight"),
    superseded_css2_property!("direction", "visuren.html", "baseline.property.direction"),
    superseded_css2_property!(
        "unicode-bidi",
        "visuren.html",
        "official.property.unicode-bidi"
    ),
    superseded_css2_property!("cursor", "ui.html", "baseline.property.cursor"),
    superseded_css2_property!("outline", "ui.html", "baseline.property.outline"),
    superseded_css2_property!(
        "outline-color",
        "ui.html",
        "baseline.property.outline-color"
    ),
    superseded_css2_property!(
        "outline-style",
        "ui.html",
        "baseline.property.outline-style"
    ),
    superseded_css2_property!(
        "outline-width",
        "ui.html",
        "baseline.property.outline-width"
    ),
    informative_css2_property!("azimuth"),
    informative_css2_property!("cue"),
    informative_css2_property!("cue-after"),
    informative_css2_property!("cue-before"),
    informative_css2_property!("elevation"),
    informative_css2_property!("pause"),
    informative_css2_property!("pause-after"),
    informative_css2_property!("pause-before"),
    informative_css2_property!("pitch"),
    informative_css2_property!("pitch-range"),
    informative_css2_property!("play-during"),
    informative_css2_property!("richness"),
    informative_css2_property!("speak"),
    informative_css2_property!("speak-header"),
    informative_css2_property!("speak-numeral"),
    informative_css2_property!("speak-punctuation"),
    informative_css2_property!("speech-rate"),
    informative_css2_property!("stress"),
    informative_css2_property!("voice-family"),
    informative_css2_property!("volume"),
    exclusion!(
        "excluded.O-CSS2.superseded-syntax",
        O_CSS2,
        "syndata.html;grammar.html",
        CssExclusionReason::SupersededWithoutCurrentProduction,
        ["O-SYNTAX3"]
    ),
    exclusion!(
        "excluded.O-CSS2.superseded-media",
        O_CSS2,
        "media.html",
        CssExclusionReason::SupersededWithoutCurrentProduction,
        ["O-MEDIA3", "O-CONDITIONAL3"]
    ),
    exclusion!(
        "excluded.O-CSS2.superseded-selectors",
        O_CSS2,
        "selector.html",
        CssExclusionReason::SupersededWithoutCurrentProduction,
        ["O-SELECTORS3", "O-NAMESPACES3"]
    ),
    exclusion!(
        "excluded.O-CSS2.superseded-cascade-values",
        O_CSS2,
        "cascade.html;syndata.html",
        CssExclusionReason::SupersededWithoutCurrentProduction,
        ["O-CASCADE4", "O-VALUES3", "O-VARIABLES1"]
    ),
    exclusion!(
        "excluded.O-CSS2.non-authored-semantics",
        O_CSS2,
        "visuren.html;visufx.html;tables.html;page.html#outside-page-box,#page-breaks,#page-cascade",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-SYNTAX3.fragment-api",
        O_SYNTAX3,
        "#parse-rule,#parse-declaration,#parse-component-value,#parse-list-of-component-values,#parse-comma-separated-list-of-component-values",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-SYNTAX3.serialization",
        O_SYNTAX3,
        "#serialization",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-STYLE-ATTR.interpretation",
        O_STYLE_ATTR,
        "#interpret",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-MEDIA3.evaluation",
        O_MEDIA3,
        "#media0,#media1 evaluation portions",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-CONDITIONAL3.evaluation-api",
        O_CONDITIONAL3,
        "#processing,#the-cssmediarule-interface,#the-csssupportsrule-interface,#apis",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-SELECTORS3.matching-specificity",
        O_SELECTORS3,
        "#selectors,#specificity,#first-formatted-line,#application-in-css",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-NAMESPACES3.uri-matching",
        O_NAMESPACES3,
        "semantic portions of #scope,#css-qnames",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-CASCADE4.processing",
        O_CASCADE4,
        "#import-processing,#value-stages,#filtering,#cascading,#initial-values,#inheriting",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-VALUES3.metasyntax",
        O_VALUES3,
        "#value-defs",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-VALUES3.computation",
        O_VALUES3,
        "#calc-computed-value,#calc-range,#calc-serialize,#relative-urls",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-VARIABLES1.substitution",
        O_VARIABLES1,
        "#cycles,#invalid-variables,#variables-in-shorthands,#apis",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-BOX3.layout",
        O_BOX3,
        "#box-model,#fragmentation",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-COLOR4.color5-syntax",
        O_COLOR4,
        "Color 5 references",
        CssExclusionReason::SupersededWithoutCurrentProduction,
        ["I-COLOR5"]
    ),
    exclusion!(
        "excluded.O-COLOR4.quirky-color",
        O_COLOR4,
        "#quirky-color",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-COLOR4.processing",
        O_COLOR4,
        "conversion/interpolation/gamut/resolution/serialization/sample-code sections",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-BACKGROUNDS3.painting",
        O_BACKGROUNDS3,
        "serialization/painting/corner/border-image/shadow algorithms",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-IMAGES3.processing",
        O_IMAGES3,
        "object negotiation/sizing/interpolation/serialization algorithms",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-FONTS3.processing",
        O_FONTS3,
        "loading/fetching/matching/feature-resolution/object-model sections",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-FONTS3.font-display",
        O_FONTS3,
        "no Fonts 3 production",
        CssExclusionReason::SupersededWithoutCurrentProduction,
        ["baseline.descriptor.font-display"]
    ),
    exclusion!(
        "excluded.O-FONTS3.font-feature-values",
        O_FONTS3,
        "no Fonts 3 production",
        CssExclusionReason::SupersededWithoutCurrentProduction,
        ["later.rule.font-feature-values"]
    ),
    exclusion!(
        "excluded.O-WRITING3.layout",
        O_WRITING3,
        "bidi/inline/abstract/principal-flow/text-combine algorithms",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-MULTICOL1.layout",
        O_MULTICOL1,
        "model/pseudo-algorithm/stacking/overflow sections",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-FLEXBOX1.layout",
        O_FLEXBOX1,
        "box/items/lines/layout/pagination/axis algorithms",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-FLEXBOX1.webkit-legacy",
        O_FLEXBOX1,
        "#webkit-aliases",
        CssExclusionReason::SupersededWithoutCurrentProduction
    ),
    exclusion!(
        "excluded.O-UI3.behavior",
        O_UI3,
        "ellipsis/input/default-style behavior sections",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-CONTAIN1.semantics",
        O_CONTAIN1,
        "containment-type/optimization sections",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-TRANSFORMS1.processing",
        O_TRANSFORMS1,
        "rendering/SVG/animation/interpolation/matrix algorithms",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-COMPOSITING1.processing",
        O_COMPOSITING1,
        "Canvas/formula/backdrop/group/advanced-compositing sections",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-EASING1.evaluation",
        O_EASING1,
        "easing output/serialization sections",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    exclusion!(
        "excluded.O-COUNTERSTYLES3.processing",
        O_COUNTERSTYLES3,
        "counter algorithms/predefined rendering/APIs/sample sheet",
        CssExclusionReason::OutsideAuthoredSyntaxBoundary
    ),
    informative_source_audit!("O-CSS2", O_CSS2),
    informative_source_audit!("O-SYNTAX3", O_SYNTAX3),
    informative_source_audit!("O-STYLE-ATTR", O_STYLE_ATTR),
    informative_source_audit!("O-MEDIA3", O_MEDIA3),
    informative_source_audit!("O-CONDITIONAL3", O_CONDITIONAL3),
    informative_source_audit!("O-SELECTORS3", O_SELECTORS3),
    informative_source_audit!("O-NAMESPACES3", O_NAMESPACES3),
    informative_source_audit!("O-CASCADE4", O_CASCADE4),
    informative_source_audit!("O-VALUES3", O_VALUES3),
    informative_source_audit!("O-VARIABLES1", O_VARIABLES1),
    informative_source_audit!("O-BOX3", O_BOX3),
    informative_source_audit!("O-COLOR4", O_COLOR4),
    informative_source_audit!("O-BACKGROUNDS3", O_BACKGROUNDS3),
    informative_source_audit!("O-IMAGES3", O_IMAGES3),
    informative_source_audit!("O-FONTS3", O_FONTS3),
    informative_source_audit!("O-WRITING3", O_WRITING3),
    informative_source_audit!("O-MULTICOL1", O_MULTICOL1),
    informative_source_audit!("O-FLEXBOX1", O_FLEXBOX1),
    informative_source_audit!("O-UI3", O_UI3),
    informative_source_audit!("O-CONTAIN1", O_CONTAIN1),
    informative_source_audit!("O-TRANSFORMS1", O_TRANSFORMS1),
    informative_source_audit!("O-COMPOSITING1", O_COMPOSITING1),
    informative_source_audit!("O-EASING1", O_EASING1),
    informative_source_audit!("O-COUNTERSTYLES3", O_COUNTERSTYLES3),
];

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

/// Returns the immutable specification-source registry in stable declaration order.
#[must_use]
pub const fn specification_sources() -> &'static [CssSpecificationSource] {
    SPECIFICATION_SOURCES
}

/// Returns the specification source for an exact stable source ID.
///
/// Lookup is case-sensitive and performs no trimming or aliasing.
#[must_use]
pub fn specification_source(id: &str) -> Option<&'static CssSpecificationSource> {
    SPECIFICATION_SOURCES
        .iter()
        .find(|source| source.id.as_str() == id)
}

/// Returns the immutable official conformance-exclusion registry.
#[must_use]
pub const fn conformance_exclusions() -> &'static [CssExclusionMetadata] {
    CONFORMANCE_EXCLUSIONS
}

/// Returns exclusion metadata for an exact stable exclusion ID.
///
/// Lookup is case-sensitive and performs no trimming or aliasing.
#[must_use]
pub fn conformance_exclusion(id: &str) -> Option<&'static CssExclusionMetadata> {
    CONFORMANCE_EXCLUSIONS
        .iter()
        .find(|exclusion| exclusion.id.as_str() == id)
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
    use super::*;

    #[test]
    fn exact_lookup_exposes_named_status_metadata() {
        let importance =
            feature_metadata("foundation.declaration.importance").expect("importance metadata");
        assert_eq!(importance.status(), CssSupportStatus::Complete);
        assert_eq!(importance.supported_subset(), None);
        assert_eq!(importance.unsupported_remainder(), None);
        assert_eq!(importance.recognized_unsupported_code(), None);

        let import = feature_metadata("baseline.rule.import").expect("import metadata");
        assert_eq!(import.status(), CssSupportStatus::Partial);
        assert_eq!(
            import.supported_subset(),
            Some("The baseline parser spelling and the I01 recovery extensions are supported.")
        );
        assert_eq!(
            import.unsupported_remainder(),
            Some("Other valid forms of the cited rule production are outside the I01 subset.")
        );
        assert_eq!(import.recognized_unsupported_code(), None);

        let namespace = feature_metadata("later.rule.namespace").expect("namespace metadata");
        assert_eq!(namespace.status(), CssSupportStatus::RecognizedUnsupported);
        assert_eq!(namespace.supported_subset(), None);
        assert_eq!(namespace.unsupported_remainder(), None);
        assert_eq!(
            namespace.recognized_unsupported_code(),
            Some(CssErrorCode::UnsupportedAtRule)
        );
    }
}

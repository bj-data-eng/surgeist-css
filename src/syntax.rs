//! Authored CSS syntax values produced by this crate's parser.
//!
//! [`CssValue`] represents CSS-owned authored syntax for declarations this
//! parser currently supports. It must not grow into a broad cross-property
//! validation bag: property-specific parsers in this crate decide which value
//! forms are accepted for each declaration, and downstream crates own their own
//! normalization and validation phases.
//!
//! Successful declarations carry their authored source location so downstream
//! adapters can report validation failures at the declaration site without
//! depending on parser implementation types.

use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CssSheet {
    rules: Vec<CssRule>,
}

impl CssSheet {
    #[must_use]
    pub const fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub(crate) fn push_rule(&mut self, rule: CssRule) {
        self.rules.push(rule);
    }

    #[must_use]
    pub fn rules(&self) -> &[CssRule] {
        &self.rules
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssRule {
    selector: CssSelector,
    declarations: Vec<CssDeclaration>,
}

impl CssRule {
    #[must_use]
    pub(crate) fn new(selector: CssSelector, declarations: Vec<CssDeclaration>) -> Self {
        Self {
            selector,
            declarations,
        }
    }

    #[must_use]
    pub const fn selector(&self) -> &CssSelector {
        &self.selector
    }

    #[must_use]
    pub fn declarations(&self) -> &[CssDeclaration] {
        &self.declarations
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssDeclaration {
    property: CssProperty,
    value: CssValue,
    location: CssSourceLocation,
}

impl CssDeclaration {
    #[must_use]
    pub(crate) const fn new(
        property: CssProperty,
        value: CssValue,
        location: CssSourceLocation,
    ) -> Self {
        Self {
            property,
            value,
            location,
        }
    }

    #[must_use]
    pub const fn property(&self) -> &CssProperty {
        &self.property
    }

    #[must_use]
    pub const fn value(&self) -> &CssValue {
        &self.value
    }

    #[must_use]
    pub const fn location(&self) -> CssSourceLocation {
        self.location
    }

    #[must_use]
    pub const fn line(&self) -> u32 {
        self.location.line()
    }

    #[must_use]
    pub const fn column(&self) -> u32 {
        self.location.column()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CssSourceLocation {
    line: u32,
    column: u32,
}

impl CssSourceLocation {
    #[must_use]
    pub const fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    pub(crate) const fn from_cssparser(location: cssparser::SourceLocation) -> Self {
        Self::new(location.line, location.column)
    }

    #[must_use]
    pub const fn line(self) -> u32 {
        self.line
    }

    #[must_use]
    pub const fn column(self) -> u32 {
        self.column
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CssProperty {
    All,
    Display,
    BoxSizing,
    Position,
    Direction,
    Overflow,
    OverflowX,
    OverflowY,
    FlexDirection,
    FlexWrap,
    Float,
    Clear,
    AlignContent,
    JustifyContent,
    AlignItems,
    AlignSelf,
    JustifyItems,
    JustifySelf,
    PlaceContent,
    PlaceItems,
    PlaceSelf,
    Visibility,
    ContentVisibility,
    Width,
    Height,
    MinWidth,
    MinHeight,
    MaxWidth,
    MaxHeight,
    FlexBasis,
    Gap,
    RowGap,
    ColumnGap,
    GridFlowTolerance,
    GridTemplateRows,
    GridTemplateColumns,
    GridTemplateAreas,
    GridTemplate,
    GridAutoRows,
    GridAutoColumns,
    GridAutoFlow,
    GridRowStart,
    GridRowEnd,
    GridColumnStart,
    GridColumnEnd,
    GridRow,
    GridColumn,
    GridArea,
    Grid,
    FontSize,
    LineHeight,
    WritingMode,
    TextAlign,
    TextAlignLast,
    TextIndent,
    VerticalAlign,
    FontFamily,
    Font,
    FontWeight,
    FontStyle,
    FontStretch,
    FontVariant,
    FontFeatureSettings,
    LetterSpacing,
    TextWrap,
    WhiteSpace,
    WordBreak,
    OverflowWrap,
    TextOverflow,
    TextDecoration,
    TextDecorationLine,
    TextDecorationColor,
    TextDecorationStyle,
    TextDecorationThickness,
    TextTransform,
    Inset,
    Top,
    Right,
    Bottom,
    Left,
    ZIndex,
    BoxDecorationBreak,
    Margin,
    MarginTop,
    MarginRight,
    MarginBottom,
    MarginLeft,
    Padding,
    PaddingTop,
    PaddingRight,
    PaddingBottom,
    PaddingLeft,
    Border,
    BorderTop,
    BorderRight,
    BorderBottom,
    BorderLeft,
    BorderWidth,
    BorderTopWidth,
    BorderRightWidth,
    BorderBottomWidth,
    BorderLeftWidth,
    Color,
    Background,
    BackgroundColor,
    BorderColor,
    BorderTopColor,
    BorderRightColor,
    BorderBottomColor,
    BorderLeftColor,
    BackgroundImage,
    BackgroundPosition,
    BackgroundSize,
    BackgroundRepeat,
    BackgroundOrigin,
    BackgroundClip,
    BackgroundAttachment,
    BorderStyle,
    BorderTopStyle,
    BorderRightStyle,
    BorderBottomStyle,
    BorderLeftStyle,
    BorderRadius,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderBottomRightRadius,
    BorderBottomLeftRadius,
    BoxShadow,
    Opacity,
    FlexGrow,
    FlexShrink,
    Order,
    Flex,
    JustifyTracks,
    AlignTracks,
    AspectRatio,
    ScrollbarWidth,
    Cursor,
    PointerEvents,
    UserSelect,
    Outline,
    OutlineColor,
    OutlineStyle,
    OutlineWidth,
    Transform,
    TransformOrigin,
    Translate,
    Rotate,
    Scale,
    Filter,
    BackdropFilter,
    ClipPath,
    Mask,
    MaskImage,
    MaskSize,
    MaskPosition,
    MaskRepeat,
    TransitionProperty,
    TransitionDuration,
    TransitionDelay,
    TransitionTimingFunction,
    Transition,
    AnimationName,
    AnimationDuration,
    AnimationDelay,
    AnimationTimingFunction,
    AnimationIterationCount,
    AnimationDirection,
    AnimationFillMode,
    AnimationPlayState,
    Animation,
    Custom(CssCustomPropertyName),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CssCustomPropertyName {
    name: String,
}

impl CssCustomPropertyName {
    #[must_use]
    pub fn try_new(name: impl Into<String>) -> Option<Self> {
        let name = name.into();
        if is_valid_custom_property_name(&name) {
            Some(Self::new(name))
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

fn is_valid_custom_property_name(name: &str) -> bool {
    name.strip_prefix("--").is_some_and(|suffix| {
        !suffix.is_empty()
            && suffix.chars().all(|character| {
                character == '-' || character == '_' || character.is_alphanumeric()
            })
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssAuthoredDeclarationValue {
    css: String,
}

impl CssAuthoredDeclarationValue {
    #[must_use]
    pub fn try_new(css: impl Into<String>) -> Option<Self> {
        let css = css.into();
        if css.trim().is_empty() {
            None
        } else {
            Some(Self::new(css))
        }
    }

    #[must_use]
    pub(crate) fn new(css: impl Into<String>) -> Self {
        Self { css: css.into() }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        &self.css
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssVariableReference {
    name: CssCustomPropertyName,
    fallback: Option<CssVariableFallback>,
}

impl CssVariableReference {
    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn new(name: CssCustomPropertyName, fallback: Option<CssVariableFallback>) -> Self {
        Self { name, fallback }
    }

    #[must_use]
    pub const fn name(&self) -> &CssCustomPropertyName {
        &self.name
    }

    #[must_use]
    pub const fn fallback(&self) -> Option<&CssVariableFallback> {
        self.fallback.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssVariableFallback {
    authored: CssAuthoredDeclarationValue,
    references: Vec<CssVariableReference>,
}

impl CssVariableFallback {
    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn new(
        authored: CssAuthoredDeclarationValue,
        references: Vec<CssVariableReference>,
    ) -> Self {
        Self {
            authored,
            references,
        }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        self.authored.as_css()
    }

    #[must_use]
    pub fn references(&self) -> &[CssVariableReference] {
        &self.references
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssCustomPropertyValue {
    authored: CssAuthoredDeclarationValue,
    references: Vec<CssVariableReference>,
}

impl CssCustomPropertyValue {
    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn new(
        authored: CssAuthoredDeclarationValue,
        references: Vec<CssVariableReference>,
    ) -> Self {
        Self {
            authored,
            references,
        }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        self.authored.as_css()
    }

    #[must_use]
    pub fn references(&self) -> &[CssVariableReference] {
        &self.references
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssVariableDependentValue {
    authored: CssAuthoredDeclarationValue,
    references: Vec<CssVariableReference>,
}

impl CssVariableDependentValue {
    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn try_new(
        authored: CssAuthoredDeclarationValue,
        references: Vec<CssVariableReference>,
    ) -> Option<Self> {
        if references.is_empty() {
            None
        } else {
            Some(Self::new(authored, references))
        }
    }

    #[allow(dead_code)]
    #[must_use]
    pub(crate) fn new(
        authored: CssAuthoredDeclarationValue,
        references: Vec<CssVariableReference>,
    ) -> Self {
        debug_assert!(!references.is_empty());
        Self {
            authored,
            references,
        }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        self.authored.as_css()
    }

    #[must_use]
    pub fn references(&self) -> &[CssVariableReference] {
        &self.references
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssGlobalKeyword {
    Inherit,
    Initial,
    Unset,
    Revert,
    RevertLayer,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssValue {
    GlobalKeyword(CssGlobalKeyword),
    CustomProperty(CssCustomPropertyValue),
    VariableDependent(CssVariableDependentValue),
    Display(CssDisplay),
    BoxSizing(CssBoxSizing),
    Position(CssLayoutPosition),
    Direction(CssDirection),
    Overflow(CssOverflow),
    OverflowAxes(CssOverflowAxes),
    FlexDirection(CssFlexDirection),
    FlexWrap(CssFlexWrap),
    Float(CssFloat),
    Clear(CssClear),
    Alignment(CssAlignment),
    AlignItems(CssAlignItems),
    PlaceAlignment(CssPlaceAlignment),
    Visibility(CssVisibility),
    ContentVisibility(CssContentVisibility),
    Length(CssLength),
    GridFlowTolerance(CssGridFlowTolerance),
    GridTrackList(CssGridTrackList),
    GridTemplateAreas(CssGridTemplateAreas),
    GridTemplate(CssGridTemplate),
    GridAutoFlow(CssGridAutoFlow),
    GridLine(CssGridLine),
    GridLineRange(CssGridLineRange),
    GridArea(CssGridArea),
    Grid(CssGrid),
    WritingMode(CssWritingMode),
    TextAlign(CssTextAlign),
    TextAlignLast(CssTextAlignLast),
    TextIndent(CssTextIndent),
    VerticalAlign(CssVerticalAlign),
    FontFamily(CssFontFamilyList),
    Font(CssFont),
    FontWeight(CssFontWeight),
    FontStyle(CssFontStyle),
    FontStretch(CssFontStretch),
    FontVariant(CssFontVariant),
    FontFeatureSettings(CssFontFeatureSettings),
    LetterSpacing(CssLetterSpacing),
    TextWrap(CssTextWrap),
    WhiteSpace(CssWhiteSpace),
    WordBreak(CssWordBreak),
    OverflowWrap(CssOverflowWrap),
    TextOverflow(CssTextOverflow),
    TextDecoration(CssTextDecoration),
    TextDecorationLine(CssTextDecorationLine),
    TextDecorationColor(CssColor),
    TextDecorationStyle(CssTextDecorationStyle),
    TextDecorationThickness(CssTextDecorationThickness),
    TextTransform(CssTextTransform),
    Edges(CssEdges),
    Color(CssColor),
    ZIndex(CssZIndex),
    BoxDecorationBreak(CssBoxDecorationBreak),
    Border(CssBorder),
    BorderStyle(CssBorderStyle),
    BorderStyles(CssBorderStyles),
    BackgroundImage(CssImageLayerList),
    BackgroundPosition(CssPositionList),
    BackgroundSize(CssBackgroundSizeList),
    BackgroundRepeat(CssBackgroundRepeatList),
    BackgroundBox(CssBackgroundBox),
    BackgroundAttachment(CssBackgroundAttachmentList),
    BorderRadius(CssBorderRadii),
    CornerRadius(CssCornerRadius),
    BoxShadow(CssBoxShadow),
    Opacity(CssOpacity),
    FlexGrow(CssFlexFactor),
    FlexShrink(CssFlexFactor),
    Order(CssOrder),
    Flex(CssFlex),
    AspectRatio(CssAspectRatio),
    ScrollbarWidth(CssScrollbarWidth),
    Cursor(CssCursor),
    PointerEvents(CssPointerEvents),
    UserSelect(CssUserSelect),
    Outline(CssOutline),
    OutlineColor(CssColor),
    OutlineStyle(CssOutlineStyle),
    OutlineWidth(CssOutlineWidth),
    Transform(CssTransform),
    TransformOrigin(CssPosition),
    Translate(CssTranslate),
    Rotate(CssRotate),
    Scale(CssScale),
    Filter(CssFilter),
    ClipPath(CssClipPath),
    Mask(CssMaskList),
    MaskImage(CssImageLayerList),
    MaskSize(CssBackgroundSizeList),
    MaskPosition(CssPositionList),
    MaskRepeat(CssBackgroundRepeatList),
    TransitionProperty(CssTransitionPropertyList),
    TimeList(CssTimeList),
    EasingList(CssEasingList),
    Transition(CssTransitionList),
    AnimationName(CssAnimationNameList),
    AnimationIterationCount(CssAnimationIterationCountList),
    AnimationDirection(CssAnimationDirectionList),
    AnimationFillMode(CssAnimationFillModeList),
    AnimationPlayState(CssAnimationPlayStateList),
    Animation(CssAnimationList),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssFiniteNumber {
    value: f32,
}

impl CssFiniteNumber {
    #[must_use]
    pub fn try_new(value: f32) -> Option<Self> {
        if value.is_finite() {
            Some(Self { value })
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) const fn new_unchecked(value: f32) -> Self {
        Self { value }
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssNonNegativeNumber {
    value: CssFiniteNumber,
}

impl CssNonNegativeNumber {
    #[must_use]
    pub fn try_new(value: f32) -> Option<Self> {
        if value >= 0.0 {
            CssFiniteNumber::try_new(value).map(|value| Self { value })
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) const fn new_unchecked(value: f32) -> Self {
        Self {
            value: CssFiniteNumber::new_unchecked(value),
        }
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value.value()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssOpacity {
    value: CssFiniteNumber,
}

impl CssOpacity {
    #[must_use]
    pub fn try_new(value: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&value) {
            let value = CssFiniteNumber::try_new(value)?;
            Some(Self { value })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value.value()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssFlexFactor {
    value: CssNonNegativeNumber,
}

impl CssFlexFactor {
    #[must_use]
    pub fn try_new(value: f32) -> Option<Self> {
        CssNonNegativeNumber::try_new(value).map(|value| Self { value })
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value.value()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssAspectRatio {
    value: CssFiniteNumber,
}

impl CssAspectRatio {
    #[must_use]
    pub fn try_new(value: f32) -> Option<Self> {
        if value > 0.0 {
            let value = CssFiniteNumber::try_new(value)?;
            Some(Self { value })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value.value()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssScrollbarWidth {
    Auto,
    Thin,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssDisplay {
    Block,
    Flex,
    Grid,
    InlineBlock,
    InlineGrid,
    GridLanes,
    InlineGridLanes,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssBoxSizing {
    ContentBox,
    BorderBox,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssLayoutPosition {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssDirection {
    Ltr,
    Rtl,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssOverflow {
    Visible,
    Clip,
    Hidden,
    Scroll,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssOverflowAxes {
    pub x: CssOverflow,
    pub y: CssOverflow,
}

impl CssOverflowAxes {
    #[must_use]
    pub const fn new(x: CssOverflow, y: CssOverflow) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFloat {
    Left,
    Right,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssClear {
    Left,
    Right,
    Both,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssAlignment {
    Normal,
    Start,
    End,
    SafeEnd,
    FlexStart,
    FlexEnd,
    SafeFlexEnd,
    Center,
    SafeCenter,
    Baseline,
    FirstBaseline,
    LastBaseline,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssAlignItems {
    Normal,
    Start,
    End,
    SafeEnd,
    FlexStart,
    FlexEnd,
    SafeFlexEnd,
    Center,
    SafeCenter,
    Baseline,
    FirstBaseline,
    LastBaseline,
    Stretch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssPlaceAlignment {
    Content(CssPlaceContentAlignment),
    Items(CssPlaceItemsAlignment),
}

impl CssPlaceAlignment {
    #[must_use]
    pub const fn content(first: CssAlignment, second: CssAlignment) -> Self {
        Self::Content(CssPlaceContentAlignment::new(first, second))
    }

    #[must_use]
    pub const fn content_all(value: CssAlignment) -> Self {
        Self::content(value, value)
    }

    #[must_use]
    pub const fn items(first: CssAlignItems, second: CssAlignItems) -> Self {
        Self::Items(CssPlaceItemsAlignment::new(first, second))
    }

    #[must_use]
    pub const fn items_all(value: CssAlignItems) -> Self {
        Self::items(value, value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssPlaceContentAlignment {
    first: CssAlignment,
    second: CssAlignment,
}

impl CssPlaceContentAlignment {
    #[must_use]
    pub const fn new(first: CssAlignment, second: CssAlignment) -> Self {
        Self { first, second }
    }

    #[must_use]
    pub const fn all(value: CssAlignment) -> Self {
        Self::new(value, value)
    }

    #[must_use]
    pub const fn first(self) -> CssAlignment {
        self.first
    }

    #[must_use]
    pub const fn second(self) -> CssAlignment {
        self.second
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssPlaceItemsAlignment {
    first: CssAlignItems,
    second: CssAlignItems,
}

impl CssPlaceItemsAlignment {
    #[must_use]
    pub const fn new(first: CssAlignItems, second: CssAlignItems) -> Self {
        Self { first, second }
    }

    #[must_use]
    pub const fn all(value: CssAlignItems) -> Self {
        Self::new(value, value)
    }

    #[must_use]
    pub const fn first(self) -> CssAlignItems {
        self.first
    }

    #[must_use]
    pub const fn second(self) -> CssAlignItems {
        self.second
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssVisibility {
    Visible,
    Hidden,
    Collapse,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssContentVisibility {
    Visible,
    Hidden,
    Auto,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssGridFlowTolerance {
    Normal,
    Infinite,
    Length(CssLength),
    Percent(f32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssCustomIdent {
    value: String,
}

impl CssCustomIdent {
    #[must_use]
    pub fn try_new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if is_valid_custom_ident(&value) {
            Some(Self::new(value))
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

fn is_valid_custom_ident(value: &str) -> bool {
    !value.is_empty()
        && !matches!(
            value.to_ascii_lowercase().as_str(),
            "inherit" | "initial" | "unset" | "revert" | "revert-layer" | "span" | "auto"
        )
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssGridTrackBreadth {
    Length(CssLength),
    Fraction(CssNonNegativeNumber),
    MinContent,
    MaxContent,
    Auto,
}

impl CssGridTrackBreadth {
    #[must_use]
    pub const fn length(length: CssLength) -> Self {
        Self::Length(length)
    }

    #[must_use]
    pub fn try_fraction(value: f32) -> Option<Self> {
        CssNonNegativeNumber::try_new(value).map(Self::Fraction)
    }

    #[must_use]
    pub(crate) const fn fraction(value: f32) -> Self {
        Self::Fraction(CssNonNegativeNumber::new_unchecked(value))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssGridTrackSize {
    Breadth(CssGridTrackBreadth),
    MinMax {
        min: CssGridTrackBreadth,
        max: CssGridTrackBreadth,
    },
    FitContent(CssLength),
}

impl CssGridTrackSize {
    #[must_use]
    pub const fn breadth(breadth: CssGridTrackBreadth) -> Self {
        Self::Breadth(breadth)
    }

    #[must_use]
    pub const fn minmax(min: CssGridTrackBreadth, max: CssGridTrackBreadth) -> Self {
        Self::MinMax { min, max }
    }

    #[must_use]
    pub const fn fit_content(limit: CssLength) -> Self {
        Self::FitContent(limit)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssGridLineNames {
    names: Vec<CssCustomIdent>,
}

impl CssGridLineNames {
    #[must_use]
    pub fn try_new(names: Vec<CssCustomIdent>) -> Option<Self> {
        if names.is_empty() {
            None
        } else {
            Some(Self::new(names))
        }
    }

    #[must_use]
    pub(crate) fn new(names: Vec<CssCustomIdent>) -> Self {
        Self { names }
    }

    #[must_use]
    pub fn names(&self) -> &[CssCustomIdent] {
        &self.names
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssGridTrackComponent {
    LineNames(CssGridLineNames),
    TrackSize(CssGridTrackSize),
    Repeat(CssGridRepeat),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssGridTrackList {
    components: Vec<CssGridTrackComponent>,
}

impl CssGridTrackList {
    #[must_use]
    pub fn try_new(components: Vec<CssGridTrackComponent>) -> Option<Self> {
        if components.is_empty() {
            None
        } else {
            Some(Self::new(components))
        }
    }

    #[must_use]
    pub(crate) fn new(components: Vec<CssGridTrackComponent>) -> Self {
        Self { components }
    }

    #[must_use]
    pub fn components(&self) -> &[CssGridTrackComponent] {
        &self.components
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssGridRepeatCount {
    Integer(CssGridRepeatInteger),
    AutoFill,
    AutoFit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssGridRepeatInteger {
    value: i32,
}

impl CssGridRepeatInteger {
    #[must_use]
    pub const fn try_new(value: i32) -> Option<Self> {
        if value > 0 {
            Some(Self { value })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn value(self) -> i32 {
        self.value
    }
}

impl CssGridRepeatCount {
    #[must_use]
    pub const fn try_integer(value: i32) -> Option<Self> {
        match CssGridRepeatInteger::try_new(value) {
            Some(value) => Some(Self::Integer(value)),
            None => None,
        }
    }

    #[must_use]
    pub(crate) const fn integer(value: i32) -> Self {
        match Self::try_integer(value) {
            Some(value) => value,
            None => panic!("grid repeat integer must be positive"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssGridRepeat {
    count: CssGridRepeatCount,
    tracks: CssGridTrackList,
}

impl CssGridRepeat {
    #[must_use]
    pub fn try_new(count: CssGridRepeatCount, tracks: CssGridTrackList) -> Option<Self> {
        if tracks.components().is_empty() {
            None
        } else {
            Some(Self::new(count, tracks))
        }
    }

    #[must_use]
    pub(crate) const fn new(count: CssGridRepeatCount, tracks: CssGridTrackList) -> Self {
        Self { count, tracks }
    }

    #[must_use]
    pub const fn count(&self) -> CssGridRepeatCount {
        self.count
    }

    #[must_use]
    pub const fn tracks(&self) -> &CssGridTrackList {
        &self.tracks
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssGridTemplateAreaCell {
    Empty,
    Named(CssCustomIdent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssGridTemplateAreaRow {
    cells: Vec<CssGridTemplateAreaCell>,
}

impl CssGridTemplateAreaRow {
    #[must_use]
    pub fn try_new(cells: Vec<CssGridTemplateAreaCell>) -> Option<Self> {
        if cells.is_empty() {
            None
        } else {
            Some(Self::new(cells))
        }
    }

    #[must_use]
    pub(crate) fn new(cells: Vec<CssGridTemplateAreaCell>) -> Self {
        Self { cells }
    }

    #[must_use]
    pub fn cells(&self) -> &[CssGridTemplateAreaCell] {
        &self.cells
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssGridTemplateAreas {
    None,
    Rows(CssGridTemplateAreaRows),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssGridTemplateAreaRows {
    rows: Vec<CssGridTemplateAreaRow>,
}

impl CssGridTemplateAreaRows {
    #[must_use]
    pub fn try_new(rows: Vec<CssGridTemplateAreaRow>) -> Option<Self> {
        if grid_template_area_rows_are_valid(&rows) {
            Some(Self { rows })
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn new_unchecked(rows: Vec<CssGridTemplateAreaRow>) -> Self {
        Self { rows }
    }

    #[must_use]
    pub fn rows(&self) -> &[CssGridTemplateAreaRow] {
        &self.rows
    }
}

impl CssGridTemplateAreas {
    #[must_use]
    pub fn try_rows(rows: Vec<CssGridTemplateAreaRow>) -> Option<Self> {
        CssGridTemplateAreaRows::try_new(rows).map(Self::Rows)
    }

    #[must_use]
    pub(crate) fn rows(rows: Vec<CssGridTemplateAreaRow>) -> Self {
        Self::Rows(CssGridTemplateAreaRows::new_unchecked(rows))
    }
}

fn grid_template_area_rows_are_valid(rows: &[CssGridTemplateAreaRow]) -> bool {
    if rows.is_empty() {
        return false;
    }
    let width = rows[0].cells().len();
    if width == 0 || rows.iter().any(|row| row.cells().len() != width) {
        return false;
    }

    let mut bounds = HashMap::<String, GridAreaBounds>::new();
    for (row_index, row) in rows.iter().enumerate() {
        for (col_index, cell) in row.cells().iter().enumerate() {
            let CssGridTemplateAreaCell::Named(name) = cell else {
                continue;
            };
            bounds
                .entry(name.as_str().to_owned())
                .and_modify(|bounds| {
                    bounds.min_row = bounds.min_row.min(row_index);
                    bounds.max_row = bounds.max_row.max(row_index);
                    bounds.min_col = bounds.min_col.min(col_index);
                    bounds.max_col = bounds.max_col.max(col_index);
                    bounds.count += 1;
                })
                .or_insert(GridAreaBounds {
                    min_row: row_index,
                    max_row: row_index,
                    min_col: col_index,
                    max_col: col_index,
                    count: 1,
                });
        }
    }

    bounds.into_values().all(|bounds| {
        let rectangle_area =
            (bounds.max_row - bounds.min_row + 1) * (bounds.max_col - bounds.min_col + 1);
        rectangle_area == bounds.count
    })
}

#[derive(Clone, Copy)]
struct GridAreaBounds {
    min_row: usize,
    max_row: usize,
    min_col: usize,
    max_col: usize,
    count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssGridTemplate {
    None,
    RowsColumns {
        rows: CssGridTrackList,
        columns: Option<CssGridTrackList>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssGridAutoFlowAxis {
    Row,
    Column,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssGridAutoFlow {
    axis: CssGridAutoFlowAxis,
    dense: bool,
}

impl CssGridAutoFlow {
    #[must_use]
    pub const fn new(axis: CssGridAutoFlowAxis, dense: bool) -> Self {
        Self { axis, dense }
    }

    #[must_use]
    pub const fn axis(self) -> CssGridAutoFlowAxis {
        self.axis
    }

    #[must_use]
    pub const fn dense(self) -> bool {
        self.dense
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssGridLine {
    Auto,
    Integer(CssGridLineInteger),
    CustomIdent(CssCustomIdent),
    Span(CssGridLineSpan),
}

impl CssGridLine {
    #[must_use]
    pub fn try_integer(value: i32) -> Option<Self> {
        CssGridLineInteger::try_new(value).map(Self::Integer)
    }

    #[must_use]
    pub(crate) fn integer(value: i32) -> Self {
        match Self::try_integer(value) {
            Some(value) => value,
            None => panic!("grid line integer must be non-zero"),
        }
    }

    #[must_use]
    pub fn try_span(integer: Option<i32>, name: Option<CssCustomIdent>) -> Option<Self> {
        CssGridLineSpan::try_new(integer, name).map(Self::Span)
    }

    #[must_use]
    pub(crate) fn span(integer: Option<i32>, name: Option<CssCustomIdent>) -> Self {
        Self::Span(CssGridLineSpan::new(integer, name))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssGridLineInteger {
    value: i32,
}

impl CssGridLineInteger {
    #[must_use]
    pub const fn try_new(value: i32) -> Option<Self> {
        if value != 0 {
            Some(Self { value })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn value(self) -> i32 {
        self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssGridLineSpan {
    integer: Option<CssGridSpanInteger>,
    name: Option<CssCustomIdent>,
}

impl CssGridLineSpan {
    #[must_use]
    pub fn try_new(integer: Option<i32>, name: Option<CssCustomIdent>) -> Option<Self> {
        let integer = match integer {
            Some(value) => match CssGridSpanInteger::try_new(value) {
                Some(value) => Some(value),
                None => return None,
            },
            None => None,
        };
        if integer.is_none() && name.is_none() {
            None
        } else {
            Some(Self { integer, name })
        }
    }

    #[must_use]
    pub(crate) fn new(integer: Option<i32>, name: Option<CssCustomIdent>) -> Self {
        match Self::try_new(integer, name) {
            Some(value) => value,
            None => panic!("grid span must include a positive integer or name"),
        }
    }

    #[must_use]
    pub const fn integer(&self) -> Option<i32> {
        match self.integer {
            Some(value) => Some(value.value()),
            None => None,
        }
    }

    #[must_use]
    pub const fn name(&self) -> Option<&CssCustomIdent> {
        self.name.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssGridSpanInteger {
    value: i32,
}

impl CssGridSpanInteger {
    #[must_use]
    pub const fn try_new(value: i32) -> Option<Self> {
        if value > 0 {
            Some(Self { value })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn value(self) -> i32 {
        self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssGridLineRange {
    start: CssGridLine,
    end: Option<CssGridLine>,
}

impl CssGridLineRange {
    #[must_use]
    pub const fn new(start: CssGridLine, end: Option<CssGridLine>) -> Self {
        Self { start, end }
    }

    #[must_use]
    pub const fn start(&self) -> &CssGridLine {
        &self.start
    }

    #[must_use]
    pub const fn end(&self) -> Option<&CssGridLine> {
        self.end.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssGridArea {
    row_start: CssGridLine,
    column_start: Option<CssGridLine>,
    row_end: Option<CssGridLine>,
    column_end: Option<CssGridLine>,
}

impl CssGridArea {
    #[must_use]
    pub const fn new(
        row_start: CssGridLine,
        column_start: Option<CssGridLine>,
        row_end: Option<CssGridLine>,
        column_end: Option<CssGridLine>,
    ) -> Self {
        Self {
            row_start,
            column_start,
            row_end,
            column_end,
        }
    }

    #[must_use]
    pub const fn row_start(&self) -> &CssGridLine {
        &self.row_start
    }

    #[must_use]
    pub const fn column_start(&self) -> Option<&CssGridLine> {
        self.column_start.as_ref()
    }

    #[must_use]
    pub const fn row_end(&self) -> Option<&CssGridLine> {
        self.row_end.as_ref()
    }

    #[must_use]
    pub const fn column_end(&self) -> Option<&CssGridLine> {
        self.column_end.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssGrid {
    Template(CssGridTemplate),
    AutoFlow {
        flow: CssGridAutoFlow,
        auto_tracks: Option<CssGridTrackList>,
        explicit_tracks: CssGridTrackList,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssOrder {
    Integer(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssFlex {
    None,
    Auto,
    Components {
        grow: CssFlexFactor,
        shrink: Option<CssFlexFactor>,
        basis: Option<CssLength>,
    },
}

impl CssFlex {
    #[must_use]
    pub const fn components(
        grow: CssFlexFactor,
        shrink: Option<CssFlexFactor>,
        basis: Option<CssLength>,
    ) -> Self {
        Self::Components {
            grow,
            shrink,
            basis,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssZIndex {
    Auto,
    Integer(i32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssBoxDecorationBreak {
    Slice,
    Clone,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssWritingMode {
    HorizontalTb,
    VerticalRl,
    VerticalLr,
    SidewaysRl,
    SidewaysLr,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
    MatchParent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTextAlignLast {
    Auto,
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssTextIndent {
    length: CssLength,
    hanging: bool,
    each_line: bool,
}

impl CssTextIndent {
    #[must_use]
    pub fn try_new(length: CssLength, hanging: bool, each_line: bool) -> Option<Self> {
        if is_text_length(&length) {
            Some(Self::new(length, hanging, each_line))
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) const fn new(length: CssLength, hanging: bool, each_line: bool) -> Self {
        Self {
            length,
            hanging,
            each_line,
        }
    }

    #[must_use]
    pub const fn length(&self) -> &CssLength {
        &self.length
    }

    #[must_use]
    pub const fn hanging(&self) -> bool {
        self.hanging
    }

    #[must_use]
    pub const fn each_line(&self) -> bool {
        self.each_line
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssVerticalAlign {
    Baseline,
    Sub,
    Super,
    TextTop,
    TextBottom,
    Middle,
    Top,
    Bottom,
    Length(CssVerticalAlignLength),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssVerticalAlignLength {
    length: CssLength,
}

impl CssVerticalAlignLength {
    #[must_use]
    pub fn try_new(length: CssLength) -> Option<Self> {
        if is_vertical_align_length(&length) {
            Some(Self::new(length))
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn new(length: CssLength) -> Self {
        Self { length }
    }

    #[must_use]
    pub const fn length(&self) -> &CssLength {
        &self.length
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontFamilyNameKind {
    Quoted,
    IdentSequence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssFontFamilyName {
    kind: CssFontFamilyNameKind,
    value: String,
}

impl CssFontFamilyName {
    #[must_use]
    pub fn try_quoted(value: impl Into<String>) -> Option<Self> {
        Self::try_new(CssFontFamilyNameKind::Quoted, value)
    }

    #[must_use]
    pub fn try_ident_sequence(value: impl Into<String>) -> Option<Self> {
        Self::try_new(CssFontFamilyNameKind::IdentSequence, value)
    }

    #[must_use]
    pub(crate) fn quoted(value: impl Into<String>) -> Self {
        Self::new(CssFontFamilyNameKind::Quoted, value)
    }

    #[must_use]
    pub(crate) fn ident_sequence(value: impl Into<String>) -> Self {
        Self::new(CssFontFamilyNameKind::IdentSequence, value)
    }

    fn try_new(kind: CssFontFamilyNameKind, value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if value.is_empty() {
            None
        } else {
            Some(Self::new(kind, value))
        }
    }

    fn new(kind: CssFontFamilyNameKind, value: impl Into<String>) -> Self {
        Self {
            kind,
            value: value.into(),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> CssFontFamilyNameKind {
        self.kind
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssFontFamilyList {
    families: Vec<CssFontFamilyName>,
}

impl CssFontFamilyList {
    #[must_use]
    pub fn try_new(families: Vec<CssFontFamilyName>) -> Option<Self> {
        if families.is_empty() || families.iter().any(|family| family.as_str().is_empty()) {
            None
        } else {
            Some(Self::new(families))
        }
    }

    #[must_use]
    pub(crate) fn new(families: Vec<CssFontFamilyName>) -> Self {
        Self { families }
    }

    #[must_use]
    pub fn families(&self) -> &[CssFontFamilyName] {
        &self.families
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    Number(CssFontWeightNumber),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssFontWeightNumber {
    value: i32,
}

impl CssFontWeightNumber {
    #[must_use]
    pub const fn try_new(value: i32) -> Option<Self> {
        if value >= 1 && value <= 1000 {
            Some(Self { value })
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) const fn new(value: i32) -> Self {
        match Self::try_new(value) {
            Some(value) => value,
            None => panic!("font weight number must be between 1 and 1000"),
        }
    }

    #[must_use]
    pub const fn value(self) -> i32 {
        self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontStretch {
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontVariant {
    Normal,
    SmallCaps,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssFontFeatureSettings {
    Normal,
    Features(CssFontFeatureList),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssFontFeatureList {
    features: Vec<CssFontFeature>,
}

impl CssFontFeatureList {
    #[must_use]
    pub fn try_new(features: Vec<CssFontFeature>) -> Option<Self> {
        if features.is_empty() {
            None
        } else {
            Some(Self::new(features))
        }
    }

    #[must_use]
    pub(crate) fn new(features: Vec<CssFontFeature>) -> Self {
        Self { features }
    }

    #[must_use]
    pub fn features(&self) -> &[CssFontFeature] {
        &self.features
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssFontFeature {
    tag: String,
    value: Option<CssFontFeatureValue>,
}

impl CssFontFeature {
    #[must_use]
    pub fn try_new(tag: impl Into<String>, value: Option<CssFontFeatureValue>) -> Option<Self> {
        let tag = tag.into();
        if !is_valid_font_feature_tag(&tag) {
            None
        } else {
            Some(Self::new(tag, value))
        }
    }

    #[must_use]
    pub(crate) fn new(tag: impl Into<String>, value: Option<CssFontFeatureValue>) -> Self {
        Self {
            tag: tag.into(),
            value,
        }
    }

    #[must_use]
    pub fn tag(&self) -> &str {
        &self.tag
    }

    #[must_use]
    pub const fn value(&self) -> Option<CssFontFeatureValue> {
        self.value
    }
}

fn is_valid_font_feature_tag(tag: &str) -> bool {
    tag.chars().count() == 4
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontFeatureValue {
    On,
    Off,
    Integer(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssFont {
    style: Option<CssFontStyle>,
    variant: Option<CssFontVariant>,
    weight: Option<CssFontWeight>,
    stretch: Option<CssFontStretch>,
    size: CssLength,
    line_height: Option<CssLength>,
    families: CssFontFamilyList,
}

impl CssFont {
    #[must_use]
    pub fn try_new(
        style: Option<CssFontStyle>,
        variant: Option<CssFontVariant>,
        weight: Option<CssFontWeight>,
        stretch: Option<CssFontStretch>,
        size: CssLength,
        line_height: Option<CssLength>,
        families: CssFontFamilyList,
    ) -> Option<Self> {
        if !is_font_size_length(&size)
            || line_height.as_ref().is_some_and(|line_height| {
                !matches!(
                    line_height,
                    CssLength::Px(_)
                        | CssLength::Dimension(_)
                        | CssLength::Percent(_)
                        | CssLength::Zero
                        | CssLength::Normal
                        | CssLength::Calc(_)
                )
            })
            || families.families().is_empty()
        {
            None
        } else {
            Some(Self::new(
                style,
                variant,
                weight,
                stretch,
                size,
                line_height,
                families,
            ))
        }
    }

    #[must_use]
    pub(crate) const fn new(
        style: Option<CssFontStyle>,
        variant: Option<CssFontVariant>,
        weight: Option<CssFontWeight>,
        stretch: Option<CssFontStretch>,
        size: CssLength,
        line_height: Option<CssLength>,
        families: CssFontFamilyList,
    ) -> Self {
        Self {
            style,
            variant,
            weight,
            stretch,
            size,
            line_height,
            families,
        }
    }

    #[must_use]
    pub const fn style(&self) -> Option<CssFontStyle> {
        self.style
    }

    #[must_use]
    pub const fn variant(&self) -> Option<CssFontVariant> {
        self.variant
    }

    #[must_use]
    pub const fn weight(&self) -> Option<CssFontWeight> {
        self.weight
    }

    #[must_use]
    pub const fn stretch(&self) -> Option<CssFontStretch> {
        self.stretch
    }

    #[must_use]
    pub const fn size(&self) -> &CssLength {
        &self.size
    }

    #[must_use]
    pub const fn line_height(&self) -> Option<&CssLength> {
        self.line_height.as_ref()
    }

    #[must_use]
    pub const fn families(&self) -> &CssFontFamilyList {
        &self.families
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssLetterSpacing {
    Normal,
    Length(CssLetterSpacingLength),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssLetterSpacingLength {
    length: CssLength,
}

impl CssLetterSpacingLength {
    #[must_use]
    pub fn try_new(length: CssLength) -> Option<Self> {
        if is_letter_spacing_length(&length) {
            Some(Self::new(length))
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn new(length: CssLength) -> Self {
        Self { length }
    }

    #[must_use]
    pub const fn length(&self) -> &CssLength {
        &self.length
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTextWrap {
    Wrap,
    NoWrap,
    Balance,
    Pretty,
    Stable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssWhiteSpace {
    Normal,
    NoWrap,
    Pre,
    PreWrap,
    PreLine,
    BreakSpaces,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssWordBreak {
    Normal,
    BreakAll,
    KeepAll,
    BreakWord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssOverflowWrap {
    Normal,
    BreakWord,
    Anywhere,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTextOverflow {
    Clip,
    Ellipsis,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssTextDecoration {
    line: Option<CssTextDecorationLine>,
    color: Option<CssColor>,
    style: Option<CssTextDecorationStyle>,
    thickness: Option<CssTextDecorationThickness>,
}

impl CssTextDecoration {
    #[must_use]
    pub fn try_new(
        line: Option<CssTextDecorationLine>,
        color: Option<CssColor>,
        style: Option<CssTextDecorationStyle>,
        thickness: Option<CssTextDecorationThickness>,
    ) -> Option<Self> {
        if line.is_none() && color.is_none() && style.is_none() && thickness.is_none() {
            None
        } else {
            Some(Self::new(line, color, style, thickness))
        }
    }

    #[must_use]
    pub(crate) const fn new(
        line: Option<CssTextDecorationLine>,
        color: Option<CssColor>,
        style: Option<CssTextDecorationStyle>,
        thickness: Option<CssTextDecorationThickness>,
    ) -> Self {
        Self {
            line,
            color,
            style,
            thickness,
        }
    }

    #[must_use]
    pub const fn line(&self) -> Option<&CssTextDecorationLine> {
        self.line.as_ref()
    }

    #[must_use]
    pub const fn color(&self) -> Option<CssColor> {
        self.color
    }

    #[must_use]
    pub const fn style(&self) -> Option<CssTextDecorationStyle> {
        self.style
    }

    #[must_use]
    pub const fn thickness(&self) -> Option<&CssTextDecorationThickness> {
        self.thickness.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssTextDecorationLine {
    components: Vec<CssTextDecorationLineComponent>,
    none: bool,
}

impl CssTextDecorationLine {
    #[must_use]
    pub fn try_new(components: Vec<CssTextDecorationLineComponent>) -> Option<Self> {
        if components.is_empty() || has_duplicate_decoration_line_components(&components) {
            None
        } else {
            Some(Self::new(components))
        }
    }

    #[must_use]
    pub(crate) fn new(components: Vec<CssTextDecorationLineComponent>) -> Self {
        Self {
            components,
            none: false,
        }
    }

    #[must_use]
    pub(crate) fn none() -> Self {
        Self {
            components: Vec::new(),
            none: true,
        }
    }

    #[must_use]
    pub const fn is_none(&self) -> bool {
        self.none
    }

    #[must_use]
    pub fn components(&self) -> &[CssTextDecorationLineComponent] {
        &self.components
    }
}

fn has_duplicate_decoration_line_components(components: &[CssTextDecorationLineComponent]) -> bool {
    components.iter().enumerate().any(|(index, component)| {
        components
            .iter()
            .skip(index + 1)
            .any(|candidate| candidate == component)
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTextDecorationLineComponent {
    Underline,
    Overline,
    LineThrough,
    Blink,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssTextDecorationThickness {
    Auto,
    FromFont,
    Length(CssTextDecorationThicknessLength),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssTextDecorationThicknessLength {
    length: CssLength,
}

impl CssTextDecorationThicknessLength {
    #[must_use]
    pub fn try_new(length: CssLength) -> Option<Self> {
        if is_text_decoration_thickness_length(&length) {
            Some(Self::new(length))
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) fn new(length: CssLength) -> Self {
        Self { length }
    }

    #[must_use]
    pub const fn length(&self) -> &CssLength {
        &self.length
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTextTransform {
    None,
    Capitalize,
    Uppercase,
    Lowercase,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssLengthUnit {
    Px,
    Em,
    Rem,
    Ex,
    Rex,
    Cap,
    Rcap,
    Ch,
    Rch,
    Ic,
    Ric,
    Lh,
    Rlh,
    Vw,
    Vh,
    Vi,
    Vb,
    Vmin,
    Vmax,
    Svw,
    Svh,
    Svi,
    Svb,
    Svmin,
    Svmax,
    Lvw,
    Lvh,
    Lvi,
    Lvb,
    Lvmin,
    Lvmax,
    Dvw,
    Dvh,
    Dvi,
    Dvb,
    Dvmin,
    Dvmax,
    Cqw,
    Cqh,
    Cqi,
    Cqb,
    Cqmin,
    Cqmax,
    Cm,
    Mm,
    Q,
    In,
    Pc,
    Pt,
}

impl CssLengthUnit {
    pub(crate) fn from_css_unit(unit: &str) -> Option<Self> {
        Some(match unit.to_ascii_lowercase().as_str() {
            "px" => Self::Px,
            "em" => Self::Em,
            "rem" => Self::Rem,
            "ex" => Self::Ex,
            "rex" => Self::Rex,
            "cap" => Self::Cap,
            "rcap" => Self::Rcap,
            "ch" => Self::Ch,
            "rch" => Self::Rch,
            "ic" => Self::Ic,
            "ric" => Self::Ric,
            "lh" => Self::Lh,
            "rlh" => Self::Rlh,
            "vw" => Self::Vw,
            "vh" => Self::Vh,
            "vi" => Self::Vi,
            "vb" => Self::Vb,
            "vmin" => Self::Vmin,
            "vmax" => Self::Vmax,
            "svw" => Self::Svw,
            "svh" => Self::Svh,
            "svi" => Self::Svi,
            "svb" => Self::Svb,
            "svmin" => Self::Svmin,
            "svmax" => Self::Svmax,
            "lvw" => Self::Lvw,
            "lvh" => Self::Lvh,
            "lvi" => Self::Lvi,
            "lvb" => Self::Lvb,
            "lvmin" => Self::Lvmin,
            "lvmax" => Self::Lvmax,
            "dvw" => Self::Dvw,
            "dvh" => Self::Dvh,
            "dvi" => Self::Dvi,
            "dvb" => Self::Dvb,
            "dvmin" => Self::Dvmin,
            "dvmax" => Self::Dvmax,
            "cqw" => Self::Cqw,
            "cqh" => Self::Cqh,
            "cqi" => Self::Cqi,
            "cqb" => Self::Cqb,
            "cqmin" => Self::Cqmin,
            "cqmax" => Self::Cqmax,
            "cm" => Self::Cm,
            "mm" => Self::Mm,
            "q" => Self::Q,
            "in" => Self::In,
            "pc" => Self::Pc,
            "pt" => Self::Pt,
            _ => return None,
        })
    }

    #[must_use]
    pub const fn as_css_str(self) -> &'static str {
        match self {
            Self::Px => "px",
            Self::Em => "em",
            Self::Rem => "rem",
            Self::Ex => "ex",
            Self::Rex => "rex",
            Self::Cap => "cap",
            Self::Rcap => "rcap",
            Self::Ch => "ch",
            Self::Rch => "rch",
            Self::Ic => "ic",
            Self::Ric => "ric",
            Self::Lh => "lh",
            Self::Rlh => "rlh",
            Self::Vw => "vw",
            Self::Vh => "vh",
            Self::Vi => "vi",
            Self::Vb => "vb",
            Self::Vmin => "vmin",
            Self::Vmax => "vmax",
            Self::Svw => "svw",
            Self::Svh => "svh",
            Self::Svi => "svi",
            Self::Svb => "svb",
            Self::Svmin => "svmin",
            Self::Svmax => "svmax",
            Self::Lvw => "lvw",
            Self::Lvh => "lvh",
            Self::Lvi => "lvi",
            Self::Lvb => "lvb",
            Self::Lvmin => "lvmin",
            Self::Lvmax => "lvmax",
            Self::Dvw => "dvw",
            Self::Dvh => "dvh",
            Self::Dvi => "dvi",
            Self::Dvb => "dvb",
            Self::Dvmin => "dvmin",
            Self::Dvmax => "dvmax",
            Self::Cqw => "cqw",
            Self::Cqh => "cqh",
            Self::Cqi => "cqi",
            Self::Cqb => "cqb",
            Self::Cqmin => "cqmin",
            Self::Cqmax => "cqmax",
            Self::Cm => "cm",
            Self::Mm => "mm",
            Self::Q => "q",
            Self::In => "in",
            Self::Pc => "pc",
            Self::Pt => "pt",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssLengthDimension {
    value: CssFiniteNumber,
    unit: CssLengthUnit,
}

impl CssLengthDimension {
    #[must_use]
    pub fn try_new(value: f32, unit: CssLengthUnit) -> Option<Self> {
        CssFiniteNumber::try_new(value).map(|value| Self { value, unit })
    }

    #[must_use]
    pub(crate) const fn new(value: f32, unit: CssLengthUnit) -> Self {
        Self {
            value: CssFiniteNumber::new_unchecked(value),
            unit,
        }
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value.value()
    }

    #[must_use]
    pub const fn unit(self) -> CssLengthUnit {
        self.unit
    }

    #[must_use]
    pub fn to_css_string(self) -> String {
        format!(
            "{}{}",
            format_css_number(self.value.value()),
            self.unit.as_css_str()
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssLength {
    Px(CssFiniteNumber),
    Dimension(CssLengthDimension),
    Percent(CssFiniteNumber),
    Zero,
    Auto,
    MinContent,
    MaxContent,
    FitContent,
    Normal,
    Calc(CssCalcLength),
}

impl CssLength {
    #[must_use]
    pub fn try_px(value: f32) -> Option<Self> {
        CssFiniteNumber::try_new(value).map(Self::Px)
    }

    #[must_use]
    pub fn try_percent(value: f32) -> Option<Self> {
        CssFiniteNumber::try_new(value).map(Self::Percent)
    }

    #[must_use]
    pub fn try_dimension(value: f32, unit: CssLengthUnit) -> Option<Self> {
        match unit {
            CssLengthUnit::Px => Self::try_px(value),
            _ => CssLengthDimension::try_new(value, unit).map(Self::Dimension),
        }
    }

    #[must_use]
    pub(crate) const fn px(value: f32) -> Self {
        Self::Px(CssFiniteNumber::new_unchecked(value))
    }

    #[must_use]
    pub(crate) const fn percent(value: f32) -> Self {
        Self::Percent(CssFiniteNumber::new_unchecked(value))
    }

    #[must_use]
    pub(crate) const fn dimension(value: f32, unit: CssLengthUnit) -> Self {
        match unit {
            CssLengthUnit::Px => Self::px(value),
            _ => Self::Dimension(CssLengthDimension::new(value, unit)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssEdges {
    pub top: CssLength,
    pub right: CssLength,
    pub bottom: CssLength,
    pub left: CssLength,
}

impl CssEdges {
    #[must_use]
    pub fn all(value: CssLength) -> Self {
        Self {
            top: value.clone(),
            right: value.clone(),
            bottom: value.clone(),
            left: value,
        }
    }

    #[must_use]
    pub const fn new(top: CssLength, right: CssLength, bottom: CssLength, left: CssLength) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssBorderStyle {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssBorder {
    width: Option<CssLength>,
    style: Option<CssBorderStyle>,
    color: Option<CssColor>,
}

impl CssBorder {
    #[must_use]
    pub fn try_new(
        width: Option<CssLength>,
        style: Option<CssBorderStyle>,
        color: Option<CssColor>,
    ) -> Option<Self> {
        if width.is_none() && style.is_none() && color.is_none()
            || width.as_ref().is_some_and(|width| !is_border_width(width))
        {
            None
        } else {
            Some(Self::new(width, style, color))
        }
    }

    #[must_use]
    pub(crate) const fn new(
        width: Option<CssLength>,
        style: Option<CssBorderStyle>,
        color: Option<CssColor>,
    ) -> Self {
        Self {
            width,
            style,
            color,
        }
    }

    #[must_use]
    pub const fn width(&self) -> Option<&CssLength> {
        self.width.as_ref()
    }

    #[must_use]
    pub const fn style(&self) -> Option<CssBorderStyle> {
        self.style
    }

    #[must_use]
    pub const fn color(&self) -> Option<CssColor> {
        self.color
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssBorderStyles {
    pub top: CssBorderStyle,
    pub right: CssBorderStyle,
    pub bottom: CssBorderStyle,
    pub left: CssBorderStyle,
}

impl CssBorderStyles {
    #[must_use]
    pub const fn new(
        top: CssBorderStyle,
        right: CssBorderStyle,
        bottom: CssBorderStyle,
        left: CssBorderStyle,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    #[must_use]
    pub const fn all(value: CssBorderStyle) -> Self {
        Self {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssCornerRadius {
    horizontal: CssLength,
    vertical: CssLength,
}

impl CssCornerRadius {
    #[must_use]
    pub fn try_new(horizontal: CssLength, vertical: CssLength) -> Option<Self> {
        if is_radius_length(&horizontal) && is_radius_length(&vertical) {
            Some(Self::new(horizontal, vertical))
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) const fn new(horizontal: CssLength, vertical: CssLength) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    #[must_use]
    pub const fn horizontal(&self) -> &CssLength {
        &self.horizontal
    }

    #[must_use]
    pub const fn vertical(&self) -> &CssLength {
        &self.vertical
    }
}

fn is_border_width(length: &CssLength) -> bool {
    match length {
        CssLength::Px(value) => value.value() >= 0.0,
        CssLength::Dimension(length) => length.value() >= 0.0,
        CssLength::Zero => true,
        CssLength::Calc(calc) => !calc.uses_percentage() && !calc_has_negative_component(calc),
        CssLength::Percent(_)
        | CssLength::Auto
        | CssLength::MinContent
        | CssLength::MaxContent
        | CssLength::FitContent
        | CssLength::Normal => false,
    }
}

fn is_radius_length(length: &CssLength) -> bool {
    match length {
        CssLength::Px(value) | CssLength::Percent(value) => value.value() >= 0.0,
        CssLength::Dimension(length) => length.value() >= 0.0,
        CssLength::Zero => true,
        CssLength::Calc(calc) => !calc_has_negative_component(calc),
        CssLength::Auto
        | CssLength::MinContent
        | CssLength::MaxContent
        | CssLength::FitContent
        | CssLength::Normal => false,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssBorderRadii {
    pub top_left: CssCornerRadius,
    pub top_right: CssCornerRadius,
    pub bottom_right: CssCornerRadius,
    pub bottom_left: CssCornerRadius,
}

impl CssBorderRadii {
    #[must_use]
    pub const fn new(
        top_left: CssCornerRadius,
        top_right: CssCornerRadius,
        bottom_right: CssCornerRadius,
        bottom_left: CssCornerRadius,
    ) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssBoxShadow {
    None,
    Shadows(CssBoxShadowList),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssBoxShadowList {
    shadows: Vec<CssShadow>,
}

impl CssBoxShadowList {
    pub(crate) fn new(shadows: Vec<CssShadow>) -> Option<Self> {
        if shadows.is_empty() {
            None
        } else {
            Some(Self { shadows })
        }
    }

    #[must_use]
    pub fn shadows(&self) -> &[CssShadow] {
        &self.shadows
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssShadow {
    inset: bool,
    offset_x: CssLength,
    offset_y: CssLength,
    blur_radius: Option<CssLength>,
    spread_radius: Option<CssLength>,
    color: Option<CssColor>,
}

impl CssShadow {
    #[must_use]
    pub fn try_new(
        inset: bool,
        offset_x: CssLength,
        offset_y: CssLength,
        blur_radius: Option<CssLength>,
        spread_radius: Option<CssLength>,
        color: Option<CssColor>,
    ) -> Option<Self> {
        if !is_shadow_length(&offset_x)
            || !is_shadow_length(&offset_y)
            || blur_radius
                .as_ref()
                .is_some_and(|blur| !is_shadow_length(blur) || length_has_negative_component(blur))
            || spread_radius
                .as_ref()
                .is_some_and(|spread| !is_shadow_length(spread))
            || blur_radius.is_none() && spread_radius.is_some()
        {
            None
        } else {
            Some(Self::new(
                inset,
                offset_x,
                offset_y,
                blur_radius,
                spread_radius,
                color,
            ))
        }
    }

    #[must_use]
    pub(crate) const fn new(
        inset: bool,
        offset_x: CssLength,
        offset_y: CssLength,
        blur_radius: Option<CssLength>,
        spread_radius: Option<CssLength>,
        color: Option<CssColor>,
    ) -> Self {
        Self {
            inset,
            offset_x,
            offset_y,
            blur_radius,
            spread_radius,
            color,
        }
    }

    #[must_use]
    pub const fn inset(&self) -> bool {
        self.inset
    }

    #[must_use]
    pub const fn offset_x(&self) -> &CssLength {
        &self.offset_x
    }

    #[must_use]
    pub const fn offset_y(&self) -> &CssLength {
        &self.offset_y
    }

    #[must_use]
    pub const fn blur_radius(&self) -> Option<&CssLength> {
        self.blur_radius.as_ref()
    }

    #[must_use]
    pub const fn spread_radius(&self) -> Option<&CssLength> {
        self.spread_radius.as_ref()
    }

    #[must_use]
    pub const fn color(&self) -> Option<CssColor> {
        self.color
    }
}

fn is_shadow_length(length: &CssLength) -> bool {
    match length {
        CssLength::Px(_) | CssLength::Dimension(_) | CssLength::Zero => true,
        CssLength::Calc(calc) => !calc.uses_percentage(),
        CssLength::Percent(_)
        | CssLength::Auto
        | CssLength::MinContent
        | CssLength::MaxContent
        | CssLength::FitContent
        | CssLength::Normal => false,
    }
}

fn is_text_length(length: &CssLength) -> bool {
    matches!(
        length,
        CssLength::Px(_)
            | CssLength::Dimension(_)
            | CssLength::Percent(_)
            | CssLength::Zero
            | CssLength::Calc(_)
    )
}

fn is_vertical_align_length(length: &CssLength) -> bool {
    is_text_length(length)
}

fn is_letter_spacing_length(length: &CssLength) -> bool {
    match length {
        CssLength::Px(_) | CssLength::Dimension(_) | CssLength::Zero => true,
        CssLength::Calc(calc) => !calc.uses_percentage(),
        CssLength::Percent(_)
        | CssLength::Auto
        | CssLength::MinContent
        | CssLength::MaxContent
        | CssLength::FitContent
        | CssLength::Normal => false,
    }
}

fn is_text_decoration_thickness_length(length: &CssLength) -> bool {
    match length {
        CssLength::Px(value) | CssLength::Percent(value) => value.value() >= 0.0,
        CssLength::Dimension(length) => length.value() >= 0.0,
        CssLength::Zero => true,
        CssLength::Calc(calc) => !calc_has_negative_component(calc),
        CssLength::Auto
        | CssLength::MinContent
        | CssLength::MaxContent
        | CssLength::FitContent
        | CssLength::Normal => false,
    }
}

fn is_font_size_length(length: &CssLength) -> bool {
    matches!(
        length,
        CssLength::Px(_)
            | CssLength::Dimension(_)
            | CssLength::Percent(_)
            | CssLength::Zero
            | CssLength::Calc(_)
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssUrl {
    value: String,
}

impl CssUrl {
    #[must_use]
    pub fn try_new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if value.is_empty() {
            None
        } else {
            Some(Self::new(value))
        }
    }

    #[must_use]
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssAuthoredFunctionArguments {
    css: String,
}

impl CssAuthoredFunctionArguments {
    #[must_use]
    pub(crate) fn new(css: impl Into<String>) -> Self {
        Self { css: css.into() }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        &self.css
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssTransformArguments {
    authored: CssAuthoredFunctionArguments,
}

impl CssTransformArguments {
    #[must_use]
    pub(crate) const fn new(authored: CssAuthoredFunctionArguments) -> Self {
        Self { authored }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        self.authored.as_css()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssFilterArguments {
    authored: CssAuthoredFunctionArguments,
}

impl CssFilterArguments {
    #[must_use]
    pub(crate) const fn new(authored: CssAuthoredFunctionArguments) -> Self {
        Self { authored }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        self.authored.as_css()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssBasicShapeArguments {
    authored: CssAuthoredFunctionArguments,
}

impl CssBasicShapeArguments {
    #[must_use]
    pub(crate) const fn new(authored: CssAuthoredFunctionArguments) -> Self {
        Self { authored }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        self.authored.as_css()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssEasingArguments {
    authored: CssAuthoredFunctionArguments,
}

impl CssEasingArguments {
    #[must_use]
    pub(crate) const fn new(authored: CssAuthoredFunctionArguments) -> Self {
        Self { authored }
    }

    #[must_use]
    pub fn as_css(&self) -> &str {
        self.authored.as_css()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssImageLayer {
    None,
    Url(CssUrl),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssImageLayerList {
    layers: Vec<CssImageLayer>,
}

impl CssImageLayerList {
    #[must_use]
    pub fn try_new(layers: Vec<CssImageLayer>) -> Option<Self> {
        if layers.is_empty() {
            None
        } else {
            Some(Self::new(layers))
        }
    }

    #[must_use]
    pub(crate) fn new(layers: Vec<CssImageLayer>) -> Self {
        Self { layers }
    }

    #[must_use]
    pub fn layers(&self) -> &[CssImageLayer] {
        &self.layers
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssHorizontalPositionKeyword {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssVerticalPositionKeyword {
    Top,
    Center,
    Bottom,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssPositionComponent {
    Horizontal(CssHorizontalPositionKeyword),
    Vertical(CssVerticalPositionKeyword),
    Length(CssLength),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssPosition {
    components: Vec<CssPositionComponent>,
}

impl CssPosition {
    #[must_use]
    pub fn try_new(components: Vec<CssPositionComponent>) -> Option<Self> {
        if components.is_empty()
            || components.len() > 4
            || has_duplicate_axis_side_keywords(&components)
        {
            None
        } else {
            Some(Self::new(components))
        }
    }

    #[must_use]
    pub(crate) fn new(components: Vec<CssPositionComponent>) -> Self {
        Self { components }
    }

    #[must_use]
    pub fn components(&self) -> &[CssPositionComponent] {
        &self.components
    }
}

fn has_duplicate_axis_side_keywords(components: &[CssPositionComponent]) -> bool {
    let mut has_horizontal_side = false;
    let mut has_vertical_side = false;

    for component in components {
        match component {
            CssPositionComponent::Horizontal(
                CssHorizontalPositionKeyword::Left | CssHorizontalPositionKeyword::Right,
            ) => {
                if has_horizontal_side {
                    return true;
                }
                has_horizontal_side = true;
            }
            CssPositionComponent::Vertical(
                CssVerticalPositionKeyword::Top | CssVerticalPositionKeyword::Bottom,
            ) => {
                if has_vertical_side {
                    return true;
                }
                has_vertical_side = true;
            }
            CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Center)
            | CssPositionComponent::Vertical(CssVerticalPositionKeyword::Center)
            | CssPositionComponent::Length(_) => {}
        }
    }

    false
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssPositionList {
    positions: Vec<CssPosition>,
}

impl CssPositionList {
    #[must_use]
    pub fn try_new(positions: Vec<CssPosition>) -> Option<Self> {
        if positions.is_empty() {
            None
        } else {
            Some(Self::new(positions))
        }
    }

    #[must_use]
    pub(crate) fn new(positions: Vec<CssPosition>) -> Self {
        Self { positions }
    }

    #[must_use]
    pub fn positions(&self) -> &[CssPosition] {
        &self.positions
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssBackgroundSizeComponent {
    Auto,
    Length(CssLength),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssBackgroundSize {
    Cover,
    Contain,
    Explicit {
        width: CssBackgroundSizeComponent,
        height: Option<CssBackgroundSizeComponent>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssBackgroundSizeList {
    sizes: Vec<CssBackgroundSize>,
}

impl CssBackgroundSizeList {
    #[must_use]
    pub fn try_new(sizes: Vec<CssBackgroundSize>) -> Option<Self> {
        if sizes.is_empty() {
            None
        } else {
            Some(Self::new(sizes))
        }
    }

    #[must_use]
    pub(crate) fn new(sizes: Vec<CssBackgroundSize>) -> Self {
        Self { sizes }
    }

    #[must_use]
    pub fn sizes(&self) -> &[CssBackgroundSize] {
        &self.sizes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssBackgroundRepeatStyle {
    Repeat,
    Space,
    Round,
    NoRepeat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssBackgroundRepeat {
    RepeatX,
    RepeatY,
    Axes {
        x: CssBackgroundRepeatStyle,
        y: CssBackgroundRepeatStyle,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssBackgroundRepeatList {
    repeats: Vec<CssBackgroundRepeat>,
}

impl CssBackgroundRepeatList {
    #[must_use]
    pub fn try_new(repeats: Vec<CssBackgroundRepeat>) -> Option<Self> {
        if repeats.is_empty() {
            None
        } else {
            Some(Self::new(repeats))
        }
    }

    #[must_use]
    pub(crate) fn new(repeats: Vec<CssBackgroundRepeat>) -> Self {
        Self { repeats }
    }

    #[must_use]
    pub fn repeats(&self) -> &[CssBackgroundRepeat] {
        &self.repeats
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssBackgroundBox {
    BorderBox,
    PaddingBox,
    ContentBox,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssBackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssBackgroundAttachmentList {
    attachments: Vec<CssBackgroundAttachment>,
}

impl CssBackgroundAttachmentList {
    #[must_use]
    pub fn try_new(attachments: Vec<CssBackgroundAttachment>) -> Option<Self> {
        if attachments.is_empty() {
            None
        } else {
            Some(Self::new(attachments))
        }
    }

    #[must_use]
    pub(crate) fn new(attachments: Vec<CssBackgroundAttachment>) -> Self {
        Self { attachments }
    }

    #[must_use]
    pub fn attachments(&self) -> &[CssBackgroundAttachment] {
        &self.attachments
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssCursorKeyword {
    Auto,
    Default,
    None,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    AllScroll,
    ColResize,
    RowResize,
    NResize,
    EResize,
    SResize,
    WResize,
    NeResize,
    NwResize,
    SeResize,
    SwResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ZoomIn,
    ZoomOut,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssCursor {
    Keyword(CssCursorKeyword),
    Urls(CssCursorUrls),
}

impl CssCursor {
    #[must_use]
    pub fn try_urls(urls: Vec<CssUrl>, fallback: CssCursorKeyword) -> Option<Self> {
        CssCursorUrlList::try_new(urls).map(|urls| Self::Urls(CssCursorUrls::new(urls, fallback)))
    }

    #[must_use]
    pub(crate) fn urls(urls: Vec<CssUrl>, fallback: CssCursorKeyword) -> Self {
        match Self::try_urls(urls, fallback) {
            Some(value) => value,
            None => panic!("cursor URL fallback must include at least one URL"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssCursorUrls {
    urls: CssCursorUrlList,
    fallback: CssCursorKeyword,
}

impl CssCursorUrls {
    #[must_use]
    pub const fn new(urls: CssCursorUrlList, fallback: CssCursorKeyword) -> Self {
        Self { urls, fallback }
    }

    #[must_use]
    pub const fn urls(&self) -> &CssCursorUrlList {
        &self.urls
    }

    #[must_use]
    pub const fn fallback(&self) -> CssCursorKeyword {
        self.fallback
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssCursorUrlList {
    urls: Vec<CssUrl>,
}

impl CssCursorUrlList {
    #[must_use]
    pub fn try_new(urls: Vec<CssUrl>) -> Option<Self> {
        if urls.is_empty() {
            None
        } else {
            Some(Self::new(urls))
        }
    }

    #[must_use]
    pub(crate) fn new(urls: Vec<CssUrl>) -> Self {
        Self { urls }
    }

    #[must_use]
    pub fn urls(&self) -> &[CssUrl] {
        &self.urls
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssPointerEvents {
    Auto,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssUserSelect {
    Auto,
    Text,
    None,
    All,
    Contain,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssOutlineStyle {
    Auto,
    Border(CssBorderStyle),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssOutlineWidth {
    Thin,
    Medium,
    Thick,
    Length(CssLength),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssOutline {
    width: Option<CssOutlineWidth>,
    style: Option<CssOutlineStyle>,
    color: Option<CssColor>,
}

impl CssOutline {
    #[must_use]
    pub fn try_new(
        width: Option<CssOutlineWidth>,
        style: Option<CssOutlineStyle>,
        color: Option<CssColor>,
    ) -> Option<Self> {
        if width.is_none() && style.is_none() && color.is_none() {
            None
        } else {
            Some(Self::new(width, style, color))
        }
    }

    #[must_use]
    pub(crate) const fn new(
        width: Option<CssOutlineWidth>,
        style: Option<CssOutlineStyle>,
        color: Option<CssColor>,
    ) -> Self {
        Self {
            width,
            style,
            color,
        }
    }

    #[must_use]
    pub const fn width(&self) -> Option<&CssOutlineWidth> {
        self.width.as_ref()
    }

    #[must_use]
    pub const fn style(&self) -> Option<CssOutlineStyle> {
        self.style
    }

    #[must_use]
    pub const fn color(&self) -> Option<CssColor> {
        self.color
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTransformFunctionKind {
    Matrix,
    Matrix3d,
    Perspective,
    Rotate,
    Rotate3d,
    RotateX,
    RotateY,
    RotateZ,
    Scale,
    Scale3d,
    ScaleX,
    ScaleY,
    ScaleZ,
    Skew,
    SkewX,
    SkewY,
    Translate,
    Translate3d,
    TranslateX,
    TranslateY,
    TranslateZ,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssTransformFunction {
    kind: CssTransformFunctionKind,
    arguments: CssTransformArguments,
}

impl CssTransformFunction {
    #[must_use]
    pub const fn new(kind: CssTransformFunctionKind, arguments: CssTransformArguments) -> Self {
        Self { kind, arguments }
    }

    #[must_use]
    pub const fn kind(&self) -> CssTransformFunctionKind {
        self.kind
    }

    #[must_use]
    pub const fn arguments(&self) -> &CssTransformArguments {
        &self.arguments
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssTransformFunctionList {
    functions: Vec<CssTransformFunction>,
}

impl CssTransformFunctionList {
    #[must_use]
    pub fn try_new(functions: Vec<CssTransformFunction>) -> Option<Self> {
        if functions.is_empty() {
            None
        } else {
            Some(Self::new(functions))
        }
    }

    #[must_use]
    pub(crate) fn new(functions: Vec<CssTransformFunction>) -> Self {
        Self { functions }
    }

    #[must_use]
    pub fn functions(&self) -> &[CssTransformFunction] {
        &self.functions
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssTransform {
    None,
    Functions(CssTransformFunctionList),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssTranslate {
    None,
    Values(CssTranslateValues),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssTranslateValues {
    values: Vec<CssLength>,
}

impl CssTranslateValues {
    #[must_use]
    pub fn try_new(values: Vec<CssLength>) -> Option<Self> {
        if values.is_empty() || values.len() > 3 {
            None
        } else {
            Some(Self::new(values))
        }
    }

    #[must_use]
    pub(crate) fn new(values: Vec<CssLength>) -> Self {
        Self { values }
    }

    #[must_use]
    pub fn values(&self) -> &[CssLength] {
        &self.values
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssRotate {
    None,
    Value(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssScale {
    None,
    Values(CssScaleValues),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssScaleValues {
    values: Vec<f32>,
}

impl CssScaleValues {
    #[must_use]
    pub fn try_new(values: Vec<f32>) -> Option<Self> {
        if values.is_empty() || values.len() > 3 || values.iter().any(|value| !value.is_finite()) {
            None
        } else {
            Some(Self::new(values))
        }
    }

    #[must_use]
    pub(crate) fn new(values: Vec<f32>) -> Self {
        Self { values }
    }

    #[must_use]
    pub fn values(&self) -> &[f32] {
        &self.values
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssFilterFunction {
    Blur(CssFilterArguments),
    Brightness(CssFilterArguments),
    Contrast(CssFilterArguments),
    DropShadow(CssFilterArguments),
    Grayscale(CssFilterArguments),
    HueRotate(CssFilterArguments),
    Invert(CssFilterArguments),
    Opacity(CssFilterArguments),
    Saturate(CssFilterArguments),
    Sepia(CssFilterArguments),
    Url(CssUrl),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssFilterFunctionList {
    functions: Vec<CssFilterFunction>,
}

impl CssFilterFunctionList {
    #[must_use]
    pub fn try_new(functions: Vec<CssFilterFunction>) -> Option<Self> {
        if functions.is_empty() {
            None
        } else {
            Some(Self::new(functions))
        }
    }

    #[must_use]
    pub(crate) fn new(functions: Vec<CssFilterFunction>) -> Self {
        Self { functions }
    }

    #[must_use]
    pub fn functions(&self) -> &[CssFilterFunction] {
        &self.functions
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssFilter {
    None,
    Functions(CssFilterFunctionList),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssBasicShape {
    Inset(CssBasicShapeArguments),
    Circle(CssBasicShapeArguments),
    Ellipse(CssBasicShapeArguments),
    Polygon(CssBasicShapeArguments),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssClipPath {
    None,
    Url(CssUrl),
    BasicShape(CssBasicShape),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssMaskLayer {
    image: Option<CssImageLayer>,
    position: Option<CssPosition>,
    size: Option<CssBackgroundSize>,
    repeat: Option<CssBackgroundRepeat>,
}

impl CssMaskLayer {
    #[must_use]
    pub fn try_new(
        image: Option<CssImageLayer>,
        position: Option<CssPosition>,
        size: Option<CssBackgroundSize>,
        repeat: Option<CssBackgroundRepeat>,
    ) -> Option<Self> {
        if image.is_none() && position.is_none() && size.is_none() && repeat.is_none() {
            None
        } else {
            Some(Self::new(image, position, size, repeat))
        }
    }

    #[must_use]
    pub(crate) const fn new(
        image: Option<CssImageLayer>,
        position: Option<CssPosition>,
        size: Option<CssBackgroundSize>,
        repeat: Option<CssBackgroundRepeat>,
    ) -> Self {
        Self {
            image,
            position,
            size,
            repeat,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssMaskList {
    layers: Vec<CssMaskLayer>,
}

impl CssMaskList {
    #[must_use]
    pub fn try_new(layers: Vec<CssMaskLayer>) -> Option<Self> {
        if layers.is_empty() {
            None
        } else {
            Some(Self::new(layers))
        }
    }

    #[must_use]
    pub(crate) fn new(layers: Vec<CssMaskLayer>) -> Self {
        Self { layers }
    }

    #[must_use]
    pub fn layers(&self) -> &[CssMaskLayer] {
        &self.layers
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CssTimeUnit {
    Seconds,
    Milliseconds,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssTime {
    value: f32,
    unit: CssTimeUnit,
}

impl CssTime {
    #[must_use]
    pub const fn try_new(value: f32, unit: CssTimeUnit) -> Option<Self> {
        if value >= 0.0 {
            Some(Self { value, unit })
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) const fn new(value: f32, unit: CssTimeUnit) -> Self {
        match Self::try_new(value, unit) {
            Some(value) => value,
            None => panic!("CSS time must be non-negative"),
        }
    }

    #[must_use]
    pub const fn try_seconds(value: f32) -> Option<Self> {
        Self::try_new(value, CssTimeUnit::Seconds)
    }

    #[must_use]
    pub const fn try_milliseconds(value: f32) -> Option<Self> {
        Self::try_new(value, CssTimeUnit::Milliseconds)
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value
    }

    #[must_use]
    pub const fn unit(self) -> CssTimeUnit {
        self.unit
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssTimeList {
    times: Vec<CssTime>,
}

impl CssTimeList {
    #[must_use]
    pub fn try_new(times: Vec<CssTime>) -> Option<Self> {
        if times.is_empty() {
            None
        } else {
            Some(Self::new(times))
        }
    }

    #[must_use]
    pub(crate) fn new(times: Vec<CssTime>) -> Self {
        Self { times }
    }

    #[must_use]
    pub fn times(&self) -> &[CssTime] {
        &self.times
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssEasing {
    Ease,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
    CubicBezier(CssEasingArguments),
    Steps(CssEasingArguments),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssEasingList {
    easings: Vec<CssEasing>,
}

impl CssEasingList {
    #[must_use]
    pub fn try_new(easings: Vec<CssEasing>) -> Option<Self> {
        if easings.is_empty() {
            None
        } else {
            Some(Self::new(easings))
        }
    }

    #[must_use]
    pub(crate) fn new(easings: Vec<CssEasing>) -> Self {
        Self { easings }
    }

    #[must_use]
    pub fn easings(&self) -> &[CssEasing] {
        &self.easings
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssTransitionProperty {
    All,
    None,
    Custom(CssCustomIdent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssTransitionPropertyList {
    properties: Vec<CssTransitionProperty>,
}

impl CssTransitionPropertyList {
    #[must_use]
    pub fn try_new(properties: Vec<CssTransitionProperty>) -> Option<Self> {
        if properties.is_empty() {
            None
        } else {
            Some(Self::new(properties))
        }
    }

    #[must_use]
    pub(crate) fn new(properties: Vec<CssTransitionProperty>) -> Self {
        Self { properties }
    }

    #[must_use]
    pub fn properties(&self) -> &[CssTransitionProperty] {
        &self.properties
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssAnimationName {
    None,
    Custom(CssCustomIdent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssAnimationNameList {
    names: Vec<CssAnimationName>,
}

impl CssAnimationNameList {
    #[must_use]
    pub fn try_new(names: Vec<CssAnimationName>) -> Option<Self> {
        if names.is_empty() {
            None
        } else {
            Some(Self::new(names))
        }
    }

    #[must_use]
    pub(crate) fn new(names: Vec<CssAnimationName>) -> Self {
        Self { names }
    }

    #[must_use]
    pub fn names(&self) -> &[CssAnimationName] {
        &self.names
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CssAnimationIterationCount {
    Infinite,
    Number(CssAnimationIterationNumber),
}

impl CssAnimationIterationCount {
    #[must_use]
    pub const fn try_number(value: f32) -> Option<Self> {
        match CssAnimationIterationNumber::try_new(value) {
            Some(value) => Some(Self::Number(value)),
            None => None,
        }
    }

    #[must_use]
    pub(crate) const fn number(value: f32) -> Self {
        Self::Number(CssAnimationIterationNumber::new(value))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssAnimationIterationNumber {
    value: f32,
}

impl CssAnimationIterationNumber {
    #[must_use]
    pub const fn try_new(value: f32) -> Option<Self> {
        if value >= 0.0 {
            Some(Self { value })
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) const fn new(value: f32) -> Self {
        match Self::try_new(value) {
            Some(value) => value,
            None => panic!("animation iteration count must be non-negative"),
        }
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssAnimationIterationCountList {
    counts: Vec<CssAnimationIterationCount>,
}

impl CssAnimationIterationCountList {
    #[must_use]
    pub fn try_new(counts: Vec<CssAnimationIterationCount>) -> Option<Self> {
        if counts.is_empty() {
            None
        } else {
            Some(Self::new(counts))
        }
    }

    #[must_use]
    pub(crate) fn new(counts: Vec<CssAnimationIterationCount>) -> Self {
        Self { counts }
    }

    #[must_use]
    pub fn counts(&self) -> &[CssAnimationIterationCount] {
        &self.counts
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssAnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssAnimationDirectionList {
    directions: Vec<CssAnimationDirection>,
}

impl CssAnimationDirectionList {
    #[must_use]
    pub fn try_new(directions: Vec<CssAnimationDirection>) -> Option<Self> {
        if directions.is_empty() {
            None
        } else {
            Some(Self::new(directions))
        }
    }

    #[must_use]
    pub(crate) fn new(directions: Vec<CssAnimationDirection>) -> Self {
        Self { directions }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssAnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssAnimationFillModeList {
    modes: Vec<CssAnimationFillMode>,
}

impl CssAnimationFillModeList {
    #[must_use]
    pub fn try_new(modes: Vec<CssAnimationFillMode>) -> Option<Self> {
        if modes.is_empty() {
            None
        } else {
            Some(Self::new(modes))
        }
    }

    #[must_use]
    pub(crate) fn new(modes: Vec<CssAnimationFillMode>) -> Self {
        Self { modes }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssAnimationPlayState {
    Running,
    Paused,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssAnimationPlayStateList {
    states: Vec<CssAnimationPlayState>,
}

impl CssAnimationPlayStateList {
    #[must_use]
    pub fn try_new(states: Vec<CssAnimationPlayState>) -> Option<Self> {
        if states.is_empty() {
            None
        } else {
            Some(Self::new(states))
        }
    }

    #[must_use]
    pub(crate) fn new(states: Vec<CssAnimationPlayState>) -> Self {
        Self { states }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssTransition {
    property: Option<CssTransitionProperty>,
    duration: Option<CssTime>,
    delay: Option<CssTime>,
    timing_function: Option<CssEasing>,
}

impl CssTransition {
    #[must_use]
    pub fn try_new(
        property: Option<CssTransitionProperty>,
        duration: Option<CssTime>,
        delay: Option<CssTime>,
        timing_function: Option<CssEasing>,
    ) -> Option<Self> {
        if property.is_none() && duration.is_none() && delay.is_none() && timing_function.is_none()
        {
            None
        } else {
            Some(Self {
                property,
                duration,
                delay,
                timing_function,
            })
        }
    }

    #[must_use]
    pub const fn property(&self) -> Option<&CssTransitionProperty> {
        self.property.as_ref()
    }

    #[must_use]
    pub const fn duration(&self) -> Option<CssTime> {
        self.duration
    }

    #[must_use]
    pub const fn delay(&self) -> Option<CssTime> {
        self.delay
    }

    #[must_use]
    pub const fn timing_function(&self) -> Option<&CssEasing> {
        self.timing_function.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssTransitionList {
    items: Vec<CssTransition>,
}

impl CssTransitionList {
    #[must_use]
    pub fn try_new(items: Vec<CssTransition>) -> Option<Self> {
        if items.is_empty() {
            None
        } else {
            Some(Self::new(items))
        }
    }

    #[must_use]
    pub(crate) fn new(items: Vec<CssTransition>) -> Self {
        Self { items }
    }

    #[must_use]
    pub fn items(&self) -> &[CssTransition] {
        &self.items
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssAnimation {
    name: Option<CssAnimationName>,
    duration: Option<CssTime>,
    delay: Option<CssTime>,
    timing_function: Option<CssEasing>,
    iteration_count: Option<CssAnimationIterationCount>,
    direction: Option<CssAnimationDirection>,
    fill_mode: Option<CssAnimationFillMode>,
    play_state: Option<CssAnimationPlayState>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CssAnimationComponents {
    pub name: Option<CssAnimationName>,
    pub duration: Option<CssTime>,
    pub delay: Option<CssTime>,
    pub timing_function: Option<CssEasing>,
    pub iteration_count: Option<CssAnimationIterationCount>,
    pub direction: Option<CssAnimationDirection>,
    pub fill_mode: Option<CssAnimationFillMode>,
    pub play_state: Option<CssAnimationPlayState>,
}

impl CssAnimation {
    #[must_use]
    pub fn try_new(components: CssAnimationComponents) -> Option<Self> {
        if components.name.is_none()
            && components.duration.is_none()
            && components.delay.is_none()
            && components.timing_function.is_none()
            && components.iteration_count.is_none()
            && components.direction.is_none()
            && components.fill_mode.is_none()
            && components.play_state.is_none()
        {
            None
        } else {
            Some(Self {
                name: components.name,
                duration: components.duration,
                delay: components.delay,
                timing_function: components.timing_function,
                iteration_count: components.iteration_count,
                direction: components.direction,
                fill_mode: components.fill_mode,
                play_state: components.play_state,
            })
        }
    }

    #[must_use]
    pub const fn name(&self) -> Option<&CssAnimationName> {
        self.name.as_ref()
    }

    #[must_use]
    pub const fn duration(&self) -> Option<CssTime> {
        self.duration
    }

    #[must_use]
    pub const fn delay(&self) -> Option<CssTime> {
        self.delay
    }

    #[must_use]
    pub const fn timing_function(&self) -> Option<&CssEasing> {
        self.timing_function.as_ref()
    }

    #[must_use]
    pub const fn iteration_count(&self) -> Option<CssAnimationIterationCount> {
        self.iteration_count
    }

    #[must_use]
    pub const fn direction(&self) -> Option<CssAnimationDirection> {
        self.direction
    }

    #[must_use]
    pub const fn fill_mode(&self) -> Option<CssAnimationFillMode> {
        self.fill_mode
    }

    #[must_use]
    pub const fn play_state(&self) -> Option<CssAnimationPlayState> {
        self.play_state
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssAnimationList {
    items: Vec<CssAnimation>,
}

impl CssAnimationList {
    #[must_use]
    pub fn try_new(items: Vec<CssAnimation>) -> Option<Self> {
        if items.is_empty() {
            None
        } else {
            Some(Self::new(items))
        }
    }

    #[must_use]
    pub(crate) fn new(items: Vec<CssAnimation>) -> Self {
        Self { items }
    }

    #[must_use]
    pub fn items(&self) -> &[CssAnimation] {
        &self.items
    }
}

pub(crate) fn length_has_negative_component(length: &CssLength) -> bool {
    match length {
        CssLength::Px(value) | CssLength::Percent(value) => value.value() < 0.0,
        CssLength::Dimension(length) => length.value() < 0.0,
        CssLength::Calc(calc) => calc_has_negative_component(calc),
        CssLength::Zero
        | CssLength::Auto
        | CssLength::MinContent
        | CssLength::MaxContent
        | CssLength::FitContent
        | CssLength::Normal => false,
    }
}

pub(crate) fn calc_has_negative_component(calc: &CssCalcLength) -> bool {
    match calc {
        CssCalcLength::Px(value) | CssCalcLength::Percent(value) => value.value() < 0.0,
        CssCalcLength::Dimension(length) => length.value() < 0.0,
        CssCalcLength::Sum(terms) => terms
            .iter()
            .any(|term| calc_has_negative_component(term.value())),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl CssColor {
    pub const TRANSPARENT: Self = Self::rgba_unchecked(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = Self::rgba_unchecked(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::rgba_unchecked(1.0, 1.0, 1.0, 1.0);

    #[must_use]
    pub const fn r(self) -> f32 {
        self.r
    }

    #[must_use]
    pub const fn g(self) -> f32 {
        self.g
    }

    #[must_use]
    pub const fn b(self) -> f32 {
        self.b
    }

    #[must_use]
    pub const fn a(self) -> f32 {
        self.a
    }

    #[must_use]
    pub fn try_rgba(r: f32, g: f32, b: f32, a: f32) -> Option<Self> {
        if [r, g, b, a]
            .into_iter()
            .all(|channel| channel.is_finite() && (0.0..=1.0).contains(&channel))
        {
            Some(Self::rgba_unchecked(r, g, b, a))
        } else {
            None
        }
    }

    #[must_use]
    pub(crate) const fn rgba_unchecked(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssSelector {
    Tag(String),
    Key(String),
    Class(String),
    PseudoClass(CssPseudoClass),
    Compound(CssCompoundSelector),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssSelectorList {
    selectors: Vec<CssSelector>,
}

impl CssSelectorList {
    #[must_use]
    pub fn try_new(selectors: Vec<CssSelector>) -> Option<Self> {
        if selectors.is_empty() {
            None
        } else {
            Some(Self::new(selectors))
        }
    }

    #[must_use]
    pub(crate) fn new(selectors: Vec<CssSelector>) -> Self {
        debug_assert!(!selectors.is_empty());
        Self { selectors }
    }

    #[must_use]
    pub fn selectors(&self) -> &[CssSelector] {
        &self.selectors
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssPseudoClass {
    Root,
    Hover,
    Active,
    Focus,
    FocusVisible,
    FocusWithin,
    Disabled,
    Enabled,
    Checked,
    Required,
    Optional,
    Valid,
    Invalid,
    PlaceholderShown,
    FirstChild,
    LastChild,
    OnlyChild,
    Empty,
    NthChild(CssNthPattern),
    NthLastChild(CssNthPattern),
    FirstOfType,
    LastOfType,
    OnlyOfType,
    NthOfType(CssNthPattern),
    NthLastOfType(CssNthPattern),
    Not(CssSelectorList),
    Is(CssSelectorList),
    Where(CssSelectorList),
    Has(CssSelectorList),
    Modal,
    Fullscreen,
    PopoverOpen,
    Default,
    Indeterminate,
    ReadOnly,
    ReadWrite,
    InRange,
    OutOfRange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssNthPattern {
    Odd,
    Even,
    Integer(i32),
    AnPlusB(CssNthAnPlusB),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CssNthAnPlusB {
    a: i32,
    b: i32,
}

impl CssNthAnPlusB {
    #[must_use]
    pub const fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    #[must_use]
    pub const fn a(self) -> i32 {
        self.a
    }

    #[must_use]
    pub const fn b(self) -> i32 {
        self.b
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssCompoundSelector {
    tag: Option<String>,
    key: Option<String>,
    classes: Vec<String>,
    pseudo_classes: Vec<CssPseudoClass>,
}

impl CssCompoundSelector {
    #[must_use]
    pub(crate) fn new(
        tag: Option<String>,
        key: Option<String>,
        classes: Vec<String>,
        pseudo_classes: Vec<CssPseudoClass>,
    ) -> Self {
        Self {
            tag,
            key,
            classes,
            pseudo_classes,
        }
    }

    #[must_use]
    pub const fn tag(&self) -> Option<&String> {
        self.tag.as_ref()
    }

    #[must_use]
    pub const fn key(&self) -> Option<&String> {
        self.key.as_ref()
    }

    #[must_use]
    pub fn classes(&self) -> &[String] {
        &self.classes
    }

    #[must_use]
    pub fn pseudo_classes(&self) -> &[CssPseudoClass] {
        &self.pseudo_classes
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssCalcLength {
    Px(CssFiniteNumber),
    Dimension(CssLengthDimension),
    Percent(CssFiniteNumber),
    Sum(Vec<CssCalcLengthTerm>),
}

impl CssCalcLength {
    #[must_use]
    pub fn try_px(value: f32) -> Option<Self> {
        CssFiniteNumber::try_new(value).map(Self::Px)
    }

    #[must_use]
    pub fn try_percent(value: f32) -> Option<Self> {
        CssFiniteNumber::try_new(value).map(Self::Percent)
    }

    #[must_use]
    pub fn try_dimension(value: f32, unit: CssLengthUnit) -> Option<Self> {
        match unit {
            CssLengthUnit::Px => Self::try_px(value),
            _ => CssLengthDimension::try_new(value, unit).map(Self::Dimension),
        }
    }

    #[must_use]
    pub(crate) const fn px(value: f32) -> Self {
        Self::Px(CssFiniteNumber::new_unchecked(value))
    }

    #[must_use]
    pub(crate) const fn percent(value: f32) -> Self {
        Self::Percent(CssFiniteNumber::new_unchecked(value))
    }

    #[must_use]
    pub(crate) const fn dimension(value: f32, unit: CssLengthUnit) -> Self {
        match unit {
            CssLengthUnit::Px => Self::px(value),
            _ => Self::Dimension(CssLengthDimension::new(value, unit)),
        }
    }

    #[must_use]
    pub fn sum(
        first: CssCalcLengthTerm,
        rest: impl IntoIterator<Item = CssCalcLengthTerm>,
    ) -> Self {
        let mut terms = vec![first];
        terms.extend(rest);
        Self::Sum(terms)
    }

    #[must_use]
    pub fn uses_percentage(&self) -> bool {
        match self {
            Self::Px(_) => false,
            Self::Dimension(_) => false,
            Self::Percent(_) => true,
            Self::Sum(terms) => terms.iter().any(|term| term.value.uses_percentage()),
        }
    }

    #[must_use]
    pub fn to_css_string(&self) -> String {
        self.to_css_fragment()
    }

    fn to_css_fragment(&self) -> String {
        match self {
            Self::Px(value) => format!("{}px", format_css_number(value.value())),
            Self::Dimension(length) => length.to_css_string(),
            Self::Percent(value) => format!("{}%", format_css_number(value.value())),
            Self::Sum(terms) => {
                let mut css = String::from("calc(");
                for (index, term) in terms.iter().enumerate() {
                    if index == 0 {
                        css.push_str(&term.value.to_css_fragment());
                    } else {
                        css.push(' ');
                        css.push_str(match term.operator {
                            CssCalcOperator::Add => "+",
                            CssCalcOperator::Subtract => "-",
                        });
                        css.push(' ');
                        css.push_str(&term.value.to_css_fragment());
                    }
                }
                css.push(')');
                css
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssCalcLengthTerm {
    operator: CssCalcOperator,
    value: CssCalcLength,
}

impl CssCalcLengthTerm {
    #[must_use]
    pub const fn add(value: CssCalcLength) -> Self {
        Self {
            operator: CssCalcOperator::Add,
            value,
        }
    }

    #[must_use]
    pub const fn sub(value: CssCalcLength) -> Self {
        Self {
            operator: CssCalcOperator::Subtract,
            value,
        }
    }

    #[must_use]
    pub const fn operator(&self) -> CssCalcOperator {
        self.operator
    }

    #[must_use]
    pub const fn value(&self) -> &CssCalcLength {
        &self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssCalcOperator {
    Add,
    Subtract,
}

fn format_css_number(value: f32) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

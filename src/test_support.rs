use crate::properties::{
    PropertyImplementation as SupportedProperty,
    property_implementation_inventory as supported_properties,
};
use crate::syntax::*;
use crate::{
    CssDeclaration, CssGlobalKeyword, CssKnownProperty, CssOverflowPropertyValue, CssParseReport,
    CssRule, CssSheet, CssStyleRule, Error, ErrorKind, parse_sheet,
};

pub(crate) trait CssParseReportTestExt<T> {
    fn is_ok(&self) -> bool;
    fn is_err(&self) -> bool;
    fn unwrap(self) -> T;
    fn expect(self, message: &str) -> T;
    fn unwrap_err(self) -> Error;
    fn expect_err(self, message: &str) -> Error;
    fn unwrap_or_else<F>(self, operation: F) -> T
    where
        F: FnOnce(Error) -> T;
}

impl<T> CssParseReportTestExt<T> for CssParseReport<T> {
    fn is_ok(&self) -> bool {
        self.is_clean()
    }

    fn is_err(&self) -> bool {
        !self.is_clean()
    }

    fn unwrap(self) -> T {
        self.expect("stylesheet report contained recovery diagnostics")
    }

    fn expect(self, message: &str) -> T {
        let (syntax, diagnostics) = self.into_parts();
        assert!(diagnostics.is_empty(), "{message}: {diagnostics:?}");
        syntax
    }

    fn unwrap_err(self) -> Error {
        self.expect_err("stylesheet report was clean")
    }

    fn expect_err(self, message: &str) -> Error {
        let (_, diagnostics) = self.into_parts();
        diagnostics
            .into_iter()
            .next()
            .unwrap_or_else(|| panic!("{message}"))
            .error()
            .clone()
    }

    fn unwrap_or_else<F>(self, operation: F) -> T
    where
        F: FnOnce(Error) -> T,
    {
        let (syntax, diagnostics) = self.into_parts();
        diagnostics
            .into_iter()
            .next()
            .map_or(syntax, |diagnostic| operation(diagnostic.error().clone()))
    }
}
macro_rules! define_test_property {
    ($input:ident; $(
        $variant:ident, $canonical:literal, [$($alias:literal),*], $stable_id:literal,
        $value:ty, $parser:ident, $dispatch:block;
    )*) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub(crate) enum CssProperty {
            $($variant,)*
            Custom(CssCustomPropertyName),
        }

        impl CssProperty {
            pub(crate) const fn known(&self) -> Option<CssKnownProperty> {
                match self {
                    $(Self::$variant => Some(CssKnownProperty::$variant),)*
                    Self::Custom(_) => None,
                }
            }
        }

        impl From<CssKnownProperty> for CssProperty {
            fn from(value: CssKnownProperty) -> Self {
                match value {
                    $(CssKnownProperty::$variant => Self::$variant,)*
                }
            }
        }
    };
}

crate::properties::property_schema!(define_test_property, test_input);

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CssValue {
    GlobalKeyword(CssGlobalKeyword),
    CustomProperty(CssCustomPropertyValue),
    VariableDependent(CssSubstitutionDependentValue),
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
    Content(CssContent),
    ContentVisibility(CssContentVisibility),
    ListStyleType(CssListStyleType),
    ListStylePosition(CssListStylePosition),
    ListStyleImage(CssListStyleImage),
    ListStyle(CssListStyle),
    CounterChanges(CssCounterChanges),
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

impl PartialEq<&CssProperty> for CssProperty {
    fn eq(&self, other: &&CssProperty) -> bool {
        self == *other
    }
}

impl PartialEq<&CssValue> for CssValue {
    fn eq(&self, other: &&CssValue) -> bool {
        self == *other
    }
}

pub(crate) fn declaration_property(declaration: &CssDeclaration) -> CssProperty {
    declaration_body_property(declaration.body())
}

pub(crate) fn declaration_body_property(body: &CssDeclarationBody) -> CssProperty {
    #[expect(
        unreachable_patterns,
        reason = "crate tests intentionally demonstrate wildcard-compatible public enum matching"
    )]
    match body {
        CssDeclarationBody::Known(known) => known.property().into(),
        CssDeclarationBody::Custom(custom) => CssProperty::Custom(custom.name().clone()),
        _ => unreachable!("test adapter saw a future declaration-body branch"),
    }
}

pub(crate) fn declaration_value(declaration: &CssDeclaration) -> CssValue {
    declaration_body_value(declaration.body())
}

pub(crate) fn declaration_body_value(body: &CssDeclarationBody) -> CssValue {
    #[expect(
        unreachable_patterns,
        reason = "crate tests intentionally demonstrate wildcard-compatible public enum matching"
    )]
    match body {
        CssDeclarationBody::Known(known) => known_test_value(known),
        CssDeclarationBody::Custom(custom) =>
        {
            #[expect(
                unreachable_patterns,
                reason = "crate tests intentionally demonstrate wildcard-compatible public enum matching"
            )]
            match custom.value() {
                CssCustomPropertyDeclaredValue::Value(value) => {
                    CssValue::CustomProperty(value.clone())
                }
                CssCustomPropertyDeclaredValue::Global(keyword) => {
                    CssValue::GlobalKeyword(*keyword)
                }
                _ => unreachable!("test adapter saw a future custom declared-value branch"),
            }
        }
        _ => unreachable!("test adapter saw a future declaration-body branch"),
    }
}

#[expect(
    unreachable_patterns,
    reason = "crate tests intentionally demonstrate wildcard-compatible public enum matching"
)]
#[expect(
    clippy::clone_on_copy,
    reason = "the test-only schema adapter clones heterogeneous authored value types uniformly"
)]
fn known_test_value(declaration: &CssKnownDeclaration) -> CssValue {
    match declaration {
        CssKnownDeclaration::All(CssAllDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::All(CssAllDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Display(CssDeclaredValue::Value(value)) => {
            CssValue::Display(value.clone())
        }
        CssKnownDeclaration::Display(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Display(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BoxSizing(CssDeclaredValue::Value(value)) => {
            CssValue::BoxSizing(value.clone())
        }
        CssKnownDeclaration::BoxSizing(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BoxSizing(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Position(CssDeclaredValue::Value(value)) => {
            CssValue::Position(value.clone())
        }
        CssKnownDeclaration::Position(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Position(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Direction(CssDeclaredValue::Value(value)) => {
            CssValue::Direction(value.clone())
        }
        CssKnownDeclaration::Direction(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Direction(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Overflow(CssDeclaredValue::Value(value)) => match value {
            CssOverflowPropertyValue::Single(value) => CssValue::Overflow(*value),
            CssOverflowPropertyValue::Pair(value) => CssValue::OverflowAxes(*value),
            _ => unreachable!("test adapter saw a future overflow value"),
        },
        CssKnownDeclaration::Overflow(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Overflow(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::OverflowX(CssDeclaredValue::Value(value)) => {
            CssValue::Overflow(value.clone())
        }
        CssKnownDeclaration::OverflowX(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::OverflowX(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::OverflowY(CssDeclaredValue::Value(value)) => {
            CssValue::Overflow(value.clone())
        }
        CssKnownDeclaration::OverflowY(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::OverflowY(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FlexDirection(CssDeclaredValue::Value(value)) => {
            CssValue::FlexDirection(value.clone())
        }
        CssKnownDeclaration::FlexDirection(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FlexDirection(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FlexWrap(CssDeclaredValue::Value(value)) => {
            CssValue::FlexWrap(value.clone())
        }
        CssKnownDeclaration::FlexWrap(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FlexWrap(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Float(CssDeclaredValue::Value(value)) => {
            CssValue::Float(value.clone())
        }
        CssKnownDeclaration::Float(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Float(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Clear(CssDeclaredValue::Value(value)) => {
            CssValue::Clear(value.clone())
        }
        CssKnownDeclaration::Clear(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Clear(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AlignContent(CssDeclaredValue::Value(value)) => {
            CssValue::Alignment(value.clone())
        }
        CssKnownDeclaration::AlignContent(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AlignContent(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::JustifyContent(CssDeclaredValue::Value(value)) => {
            CssValue::Alignment(value.clone())
        }
        CssKnownDeclaration::JustifyContent(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::JustifyContent(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AlignItems(CssDeclaredValue::Value(value)) => {
            CssValue::AlignItems(value.clone())
        }
        CssKnownDeclaration::AlignItems(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AlignItems(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AlignSelf(CssDeclaredValue::Value(value)) => {
            CssValue::AlignItems(value.clone())
        }
        CssKnownDeclaration::AlignSelf(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AlignSelf(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::JustifyItems(CssDeclaredValue::Value(value)) => {
            CssValue::AlignItems(value.clone())
        }
        CssKnownDeclaration::JustifyItems(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::JustifyItems(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::JustifySelf(CssDeclaredValue::Value(value)) => {
            CssValue::AlignItems(value.clone())
        }
        CssKnownDeclaration::JustifySelf(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::JustifySelf(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::PlaceContent(CssDeclaredValue::Value(value)) => {
            CssValue::PlaceAlignment(value.clone())
        }
        CssKnownDeclaration::PlaceContent(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::PlaceContent(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::PlaceItems(CssDeclaredValue::Value(value)) => {
            CssValue::PlaceAlignment(value.clone())
        }
        CssKnownDeclaration::PlaceItems(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::PlaceItems(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::PlaceSelf(CssDeclaredValue::Value(value)) => {
            CssValue::PlaceAlignment(value.clone())
        }
        CssKnownDeclaration::PlaceSelf(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::PlaceSelf(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Visibility(CssDeclaredValue::Value(value)) => {
            CssValue::Visibility(value.clone())
        }
        CssKnownDeclaration::Visibility(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Visibility(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Content(CssDeclaredValue::Value(value)) => {
            CssValue::Content(value.clone())
        }
        CssKnownDeclaration::Content(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Content(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::ContentVisibility(CssDeclaredValue::Value(value)) => {
            CssValue::ContentVisibility(value.clone())
        }
        CssKnownDeclaration::ContentVisibility(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::ContentVisibility(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::ListStyleType(CssDeclaredValue::Value(value)) => {
            CssValue::ListStyleType(value.clone())
        }
        CssKnownDeclaration::ListStyleType(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::ListStyleType(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::ListStylePosition(CssDeclaredValue::Value(value)) => {
            CssValue::ListStylePosition(value.clone())
        }
        CssKnownDeclaration::ListStylePosition(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::ListStylePosition(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::ListStyleImage(CssDeclaredValue::Value(value)) => {
            CssValue::ListStyleImage(value.clone())
        }
        CssKnownDeclaration::ListStyleImage(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::ListStyleImage(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::ListStyle(CssDeclaredValue::Value(value)) => {
            CssValue::ListStyle(value.clone())
        }
        CssKnownDeclaration::ListStyle(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::ListStyle(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::CounterReset(CssDeclaredValue::Value(value)) => {
            CssValue::CounterChanges(value.clone())
        }
        CssKnownDeclaration::CounterReset(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::CounterReset(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::CounterIncrement(CssDeclaredValue::Value(value)) => {
            CssValue::CounterChanges(value.clone())
        }
        CssKnownDeclaration::CounterIncrement(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::CounterIncrement(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::CounterSet(CssDeclaredValue::Value(value)) => {
            CssValue::CounterChanges(value.clone())
        }
        CssKnownDeclaration::CounterSet(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::CounterSet(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Width(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::Width(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Width(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Height(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::Height(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Height(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MinWidth(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::MinWidth(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MinWidth(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MinHeight(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::MinHeight(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MinHeight(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MaxWidth(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::MaxWidth(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MaxWidth(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MaxHeight(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::MaxHeight(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MaxHeight(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FlexBasis(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::FlexBasis(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FlexBasis(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Gap(CssDeclaredValue::Value(value)) => CssValue::Length(value.clone()),
        CssKnownDeclaration::Gap(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Gap(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::RowGap(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::RowGap(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::RowGap(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::ColumnGap(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::ColumnGap(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::ColumnGap(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridFlowTolerance(CssDeclaredValue::Value(value)) => {
            CssValue::GridFlowTolerance(value.clone())
        }
        CssKnownDeclaration::GridFlowTolerance(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridFlowTolerance(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridTemplateRows(CssDeclaredValue::Value(value)) => {
            CssValue::GridTrackList(value.clone())
        }
        CssKnownDeclaration::GridTemplateRows(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridTemplateRows(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridTemplateColumns(CssDeclaredValue::Value(value)) => {
            CssValue::GridTrackList(value.clone())
        }
        CssKnownDeclaration::GridTemplateColumns(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridTemplateColumns(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::GridTemplateAreas(CssDeclaredValue::Value(value)) => {
            CssValue::GridTemplateAreas(value.clone())
        }
        CssKnownDeclaration::GridTemplateAreas(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridTemplateAreas(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridTemplate(CssDeclaredValue::Value(value)) => {
            CssValue::GridTemplate(value.clone())
        }
        CssKnownDeclaration::GridTemplate(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridTemplate(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridAutoRows(CssDeclaredValue::Value(value)) => {
            CssValue::GridTrackList(value.clone())
        }
        CssKnownDeclaration::GridAutoRows(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridAutoRows(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridAutoColumns(CssDeclaredValue::Value(value)) => {
            CssValue::GridTrackList(value.clone())
        }
        CssKnownDeclaration::GridAutoColumns(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridAutoColumns(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridAutoFlow(CssDeclaredValue::Value(value)) => {
            CssValue::GridAutoFlow(value.clone())
        }
        CssKnownDeclaration::GridAutoFlow(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridAutoFlow(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridRowStart(CssDeclaredValue::Value(value)) => {
            CssValue::GridLine(value.clone())
        }
        CssKnownDeclaration::GridRowStart(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridRowStart(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridRowEnd(CssDeclaredValue::Value(value)) => {
            CssValue::GridLine(value.clone())
        }
        CssKnownDeclaration::GridRowEnd(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridRowEnd(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridColumnStart(CssDeclaredValue::Value(value)) => {
            CssValue::GridLine(value.clone())
        }
        CssKnownDeclaration::GridColumnStart(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridColumnStart(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridColumnEnd(CssDeclaredValue::Value(value)) => {
            CssValue::GridLine(value.clone())
        }
        CssKnownDeclaration::GridColumnEnd(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridColumnEnd(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridRow(CssDeclaredValue::Value(value)) => {
            CssValue::GridLineRange(value.clone())
        }
        CssKnownDeclaration::GridRow(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridRow(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridColumn(CssDeclaredValue::Value(value)) => {
            CssValue::GridLineRange(value.clone())
        }
        CssKnownDeclaration::GridColumn(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridColumn(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::GridArea(CssDeclaredValue::Value(value)) => {
            CssValue::GridArea(value.clone())
        }
        CssKnownDeclaration::GridArea(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::GridArea(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Grid(CssDeclaredValue::Value(value)) => CssValue::Grid(value.clone()),
        CssKnownDeclaration::Grid(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Grid(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FontSize(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::FontSize(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FontSize(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::LineHeight(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::LineHeight(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::LineHeight(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::WritingMode(CssDeclaredValue::Value(value)) => {
            CssValue::WritingMode(value.clone())
        }
        CssKnownDeclaration::WritingMode(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::WritingMode(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TextAlign(CssDeclaredValue::Value(value)) => {
            CssValue::TextAlign(value.clone())
        }
        CssKnownDeclaration::TextAlign(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextAlign(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TextAlignLast(CssDeclaredValue::Value(value)) => {
            CssValue::TextAlignLast(value.clone())
        }
        CssKnownDeclaration::TextAlignLast(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextAlignLast(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TextIndent(CssDeclaredValue::Value(value)) => {
            CssValue::TextIndent(value.clone())
        }
        CssKnownDeclaration::TextIndent(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextIndent(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::VerticalAlign(CssDeclaredValue::Value(value)) => {
            CssValue::VerticalAlign(value.clone())
        }
        CssKnownDeclaration::VerticalAlign(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::VerticalAlign(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FontFamily(CssDeclaredValue::Value(value)) => {
            CssValue::FontFamily(value.clone())
        }
        CssKnownDeclaration::FontFamily(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FontFamily(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Font(CssDeclaredValue::Value(value)) => CssValue::Font(value.clone()),
        CssKnownDeclaration::Font(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Font(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FontWeight(CssDeclaredValue::Value(value)) => {
            CssValue::FontWeight(value.clone())
        }
        CssKnownDeclaration::FontWeight(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FontWeight(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FontStyle(CssDeclaredValue::Value(value)) => {
            CssValue::FontStyle(value.clone())
        }
        CssKnownDeclaration::FontStyle(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FontStyle(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FontStretch(CssDeclaredValue::Value(value)) => {
            CssValue::FontStretch(value.clone())
        }
        CssKnownDeclaration::FontStretch(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FontStretch(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FontVariant(CssDeclaredValue::Value(value)) => {
            CssValue::FontVariant(value.clone())
        }
        CssKnownDeclaration::FontVariant(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FontVariant(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FontFeatureSettings(CssDeclaredValue::Value(value)) => {
            CssValue::FontFeatureSettings(value.clone())
        }
        CssKnownDeclaration::FontFeatureSettings(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FontFeatureSettings(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::LetterSpacing(CssDeclaredValue::Value(value)) => {
            CssValue::LetterSpacing(value.clone())
        }
        CssKnownDeclaration::LetterSpacing(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::LetterSpacing(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TextWrap(CssDeclaredValue::Value(value)) => {
            CssValue::TextWrap(value.clone())
        }
        CssKnownDeclaration::TextWrap(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextWrap(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::WhiteSpace(CssDeclaredValue::Value(value)) => {
            CssValue::WhiteSpace(value.clone())
        }
        CssKnownDeclaration::WhiteSpace(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::WhiteSpace(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::WordBreak(CssDeclaredValue::Value(value)) => {
            CssValue::WordBreak(value.clone())
        }
        CssKnownDeclaration::WordBreak(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::WordBreak(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::OverflowWrap(CssDeclaredValue::Value(value)) => {
            CssValue::OverflowWrap(value.clone())
        }
        CssKnownDeclaration::OverflowWrap(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::OverflowWrap(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TextOverflow(CssDeclaredValue::Value(value)) => {
            CssValue::TextOverflow(value.clone())
        }
        CssKnownDeclaration::TextOverflow(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextOverflow(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TextDecoration(CssDeclaredValue::Value(value)) => {
            CssValue::TextDecoration(value.clone())
        }
        CssKnownDeclaration::TextDecoration(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextDecoration(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TextDecorationLine(CssDeclaredValue::Value(value)) => {
            CssValue::TextDecorationLine(value.clone())
        }
        CssKnownDeclaration::TextDecorationLine(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextDecorationLine(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TextDecorationColor(CssDeclaredValue::Value(value)) => {
            CssValue::TextDecorationColor(value.clone())
        }
        CssKnownDeclaration::TextDecorationColor(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextDecorationColor(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::TextDecorationStyle(CssDeclaredValue::Value(value)) => {
            CssValue::TextDecorationStyle(value.clone())
        }
        CssKnownDeclaration::TextDecorationStyle(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextDecorationStyle(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::TextDecorationThickness(CssDeclaredValue::Value(value)) => {
            CssValue::TextDecorationThickness(value.clone())
        }
        CssKnownDeclaration::TextDecorationThickness(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextDecorationThickness(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::TextTransform(CssDeclaredValue::Value(value)) => {
            CssValue::TextTransform(value.clone())
        }
        CssKnownDeclaration::TextTransform(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TextTransform(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Inset(CssDeclaredValue::Value(value)) => {
            CssValue::Edges(value.clone())
        }
        CssKnownDeclaration::Inset(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Inset(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Top(CssDeclaredValue::Value(value)) => CssValue::Length(value.clone()),
        CssKnownDeclaration::Top(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Top(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Right(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::Right(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Right(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Bottom(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::Bottom(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Bottom(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Left(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::Left(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Left(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::ZIndex(CssDeclaredValue::Value(value)) => {
            CssValue::ZIndex(value.clone())
        }
        CssKnownDeclaration::ZIndex(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::ZIndex(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BoxDecorationBreak(CssDeclaredValue::Value(value)) => {
            CssValue::BoxDecorationBreak(value.clone())
        }
        CssKnownDeclaration::BoxDecorationBreak(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BoxDecorationBreak(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Margin(CssDeclaredValue::Value(value)) => {
            CssValue::Edges(value.clone())
        }
        CssKnownDeclaration::Margin(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Margin(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MarginTop(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::MarginTop(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MarginTop(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MarginRight(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::MarginRight(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MarginRight(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MarginBottom(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::MarginBottom(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MarginBottom(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MarginLeft(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::MarginLeft(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MarginLeft(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Padding(CssDeclaredValue::Value(value)) => {
            CssValue::Edges(value.clone())
        }
        CssKnownDeclaration::Padding(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Padding(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::PaddingTop(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::PaddingTop(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::PaddingTop(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::PaddingRight(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::PaddingRight(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::PaddingRight(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::PaddingBottom(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::PaddingBottom(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::PaddingBottom(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::PaddingLeft(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::PaddingLeft(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::PaddingLeft(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Border(CssDeclaredValue::Value(value)) => {
            CssValue::Border(value.clone())
        }
        CssKnownDeclaration::Border(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Border(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderTop(CssDeclaredValue::Value(value)) => {
            CssValue::Border(value.clone())
        }
        CssKnownDeclaration::BorderTop(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderTop(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderRight(CssDeclaredValue::Value(value)) => {
            CssValue::Border(value.clone())
        }
        CssKnownDeclaration::BorderRight(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderRight(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderBottom(CssDeclaredValue::Value(value)) => {
            CssValue::Border(value.clone())
        }
        CssKnownDeclaration::BorderBottom(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderBottom(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderLeft(CssDeclaredValue::Value(value)) => {
            CssValue::Border(value.clone())
        }
        CssKnownDeclaration::BorderLeft(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderLeft(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderWidth(CssDeclaredValue::Value(value)) => {
            CssValue::Edges(value.clone())
        }
        CssKnownDeclaration::BorderWidth(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderWidth(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderTopWidth(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::BorderTopWidth(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderTopWidth(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderRightWidth(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::BorderRightWidth(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderRightWidth(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderBottomWidth(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::BorderBottomWidth(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderBottomWidth(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderLeftWidth(CssDeclaredValue::Value(value)) => {
            CssValue::Length(value.clone())
        }
        CssKnownDeclaration::BorderLeftWidth(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderLeftWidth(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Color(CssDeclaredValue::Value(value)) => {
            CssValue::Color(value.clone())
        }
        CssKnownDeclaration::Color(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Color(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Background(CssDeclaredValue::Value(value)) => {
            CssValue::Color(value.clone())
        }
        CssKnownDeclaration::Background(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Background(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BackgroundColor(CssDeclaredValue::Value(value)) => {
            CssValue::Color(value.clone())
        }
        CssKnownDeclaration::BackgroundColor(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BackgroundColor(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderColor(CssDeclaredValue::Value(value)) => {
            CssValue::Color(value.clone())
        }
        CssKnownDeclaration::BorderColor(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderColor(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderTopColor(CssDeclaredValue::Value(value)) => {
            CssValue::Color(value.clone())
        }
        CssKnownDeclaration::BorderTopColor(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderTopColor(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderRightColor(CssDeclaredValue::Value(value)) => {
            CssValue::Color(value.clone())
        }
        CssKnownDeclaration::BorderRightColor(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderRightColor(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderBottomColor(CssDeclaredValue::Value(value)) => {
            CssValue::Color(value.clone())
        }
        CssKnownDeclaration::BorderBottomColor(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderBottomColor(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderLeftColor(CssDeclaredValue::Value(value)) => {
            CssValue::Color(value.clone())
        }
        CssKnownDeclaration::BorderLeftColor(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderLeftColor(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BackgroundImage(CssDeclaredValue::Value(value)) => {
            CssValue::BackgroundImage(value.clone())
        }
        CssKnownDeclaration::BackgroundImage(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BackgroundImage(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BackgroundPosition(CssDeclaredValue::Value(value)) => {
            CssValue::BackgroundPosition(value.clone())
        }
        CssKnownDeclaration::BackgroundPosition(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BackgroundPosition(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BackgroundSize(CssDeclaredValue::Value(value)) => {
            CssValue::BackgroundSize(value.clone())
        }
        CssKnownDeclaration::BackgroundSize(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BackgroundSize(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BackgroundRepeat(CssDeclaredValue::Value(value)) => {
            CssValue::BackgroundRepeat(value.clone())
        }
        CssKnownDeclaration::BackgroundRepeat(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BackgroundRepeat(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BackgroundOrigin(CssDeclaredValue::Value(value)) => {
            CssValue::BackgroundBox(value.clone())
        }
        CssKnownDeclaration::BackgroundOrigin(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BackgroundOrigin(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BackgroundClip(CssDeclaredValue::Value(value)) => {
            CssValue::BackgroundBox(value.clone())
        }
        CssKnownDeclaration::BackgroundClip(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BackgroundClip(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BackgroundAttachment(CssDeclaredValue::Value(value)) => {
            CssValue::BackgroundAttachment(value.clone())
        }
        CssKnownDeclaration::BackgroundAttachment(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BackgroundAttachment(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::BorderStyle(CssDeclaredValue::Value(value)) => {
            CssValue::BorderStyles(value.clone())
        }
        CssKnownDeclaration::BorderStyle(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderStyle(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderTopStyle(CssDeclaredValue::Value(value)) => {
            CssValue::BorderStyle(value.clone())
        }
        CssKnownDeclaration::BorderTopStyle(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderTopStyle(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderRightStyle(CssDeclaredValue::Value(value)) => {
            CssValue::BorderStyle(value.clone())
        }
        CssKnownDeclaration::BorderRightStyle(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderRightStyle(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderBottomStyle(CssDeclaredValue::Value(value)) => {
            CssValue::BorderStyle(value.clone())
        }
        CssKnownDeclaration::BorderBottomStyle(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderBottomStyle(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderLeftStyle(CssDeclaredValue::Value(value)) => {
            CssValue::BorderStyle(value.clone())
        }
        CssKnownDeclaration::BorderLeftStyle(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderLeftStyle(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderRadius(CssDeclaredValue::Value(value)) => {
            CssValue::BorderRadius(value.clone())
        }
        CssKnownDeclaration::BorderRadius(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderRadius(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BorderTopLeftRadius(CssDeclaredValue::Value(value)) => {
            CssValue::CornerRadius(value.clone())
        }
        CssKnownDeclaration::BorderTopLeftRadius(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderTopLeftRadius(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::BorderTopRightRadius(CssDeclaredValue::Value(value)) => {
            CssValue::CornerRadius(value.clone())
        }
        CssKnownDeclaration::BorderTopRightRadius(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderTopRightRadius(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::BorderBottomRightRadius(CssDeclaredValue::Value(value)) => {
            CssValue::CornerRadius(value.clone())
        }
        CssKnownDeclaration::BorderBottomRightRadius(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderBottomRightRadius(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::BorderBottomLeftRadius(CssDeclaredValue::Value(value)) => {
            CssValue::CornerRadius(value.clone())
        }
        CssKnownDeclaration::BorderBottomLeftRadius(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BorderBottomLeftRadius(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::BoxShadow(CssDeclaredValue::Value(value)) => {
            CssValue::BoxShadow(value.clone())
        }
        CssKnownDeclaration::BoxShadow(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BoxShadow(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Opacity(CssDeclaredValue::Value(value)) => {
            CssValue::Opacity(value.clone())
        }
        CssKnownDeclaration::Opacity(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Opacity(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FlexGrow(CssDeclaredValue::Value(value)) => {
            CssValue::FlexGrow(value.clone())
        }
        CssKnownDeclaration::FlexGrow(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FlexGrow(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::FlexShrink(CssDeclaredValue::Value(value)) => {
            CssValue::FlexShrink(value.clone())
        }
        CssKnownDeclaration::FlexShrink(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::FlexShrink(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Order(CssDeclaredValue::Value(value)) => {
            CssValue::Order(value.clone())
        }
        CssKnownDeclaration::Order(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Order(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Flex(CssDeclaredValue::Value(value)) => CssValue::Flex(value.clone()),
        CssKnownDeclaration::Flex(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Flex(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::JustifyTracks(CssDeclaredValue::Value(value)) => {
            CssValue::Alignment(value.clone())
        }
        CssKnownDeclaration::JustifyTracks(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::JustifyTracks(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AlignTracks(CssDeclaredValue::Value(value)) => {
            CssValue::Alignment(value.clone())
        }
        CssKnownDeclaration::AlignTracks(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AlignTracks(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AspectRatio(CssDeclaredValue::Value(value)) => {
            CssValue::AspectRatio(value.clone())
        }
        CssKnownDeclaration::AspectRatio(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AspectRatio(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::ScrollbarWidth(CssDeclaredValue::Value(value)) => {
            CssValue::ScrollbarWidth(value.clone())
        }
        CssKnownDeclaration::ScrollbarWidth(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::ScrollbarWidth(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Cursor(CssDeclaredValue::Value(value)) => {
            CssValue::Cursor(value.clone())
        }
        CssKnownDeclaration::Cursor(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Cursor(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::PointerEvents(CssDeclaredValue::Value(value)) => {
            CssValue::PointerEvents(value.clone())
        }
        CssKnownDeclaration::PointerEvents(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::PointerEvents(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::UserSelect(CssDeclaredValue::Value(value)) => {
            CssValue::UserSelect(value.clone())
        }
        CssKnownDeclaration::UserSelect(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::UserSelect(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Outline(CssDeclaredValue::Value(value)) => {
            CssValue::Outline(value.clone())
        }
        CssKnownDeclaration::Outline(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Outline(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::OutlineColor(CssDeclaredValue::Value(value)) => {
            CssValue::OutlineColor(value.clone())
        }
        CssKnownDeclaration::OutlineColor(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::OutlineColor(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::OutlineStyle(CssDeclaredValue::Value(value)) => {
            CssValue::OutlineStyle(value.clone())
        }
        CssKnownDeclaration::OutlineStyle(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::OutlineStyle(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::OutlineWidth(CssDeclaredValue::Value(value)) => {
            CssValue::OutlineWidth(value.clone())
        }
        CssKnownDeclaration::OutlineWidth(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::OutlineWidth(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Transform(CssDeclaredValue::Value(value)) => {
            CssValue::Transform(value.clone())
        }
        CssKnownDeclaration::Transform(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Transform(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TransformOrigin(CssDeclaredValue::Value(value)) => {
            CssValue::TransformOrigin(value.clone())
        }
        CssKnownDeclaration::TransformOrigin(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TransformOrigin(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Translate(CssDeclaredValue::Value(value)) => {
            CssValue::Translate(value.clone())
        }
        CssKnownDeclaration::Translate(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Translate(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Rotate(CssDeclaredValue::Value(value)) => {
            CssValue::Rotate(value.clone())
        }
        CssKnownDeclaration::Rotate(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Rotate(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Scale(CssDeclaredValue::Value(value)) => {
            CssValue::Scale(value.clone())
        }
        CssKnownDeclaration::Scale(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Scale(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Filter(CssDeclaredValue::Value(value)) => {
            CssValue::Filter(value.clone())
        }
        CssKnownDeclaration::Filter(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Filter(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::BackdropFilter(CssDeclaredValue::Value(value)) => {
            CssValue::Filter(value.clone())
        }
        CssKnownDeclaration::BackdropFilter(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::BackdropFilter(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::ClipPath(CssDeclaredValue::Value(value)) => {
            CssValue::ClipPath(value.clone())
        }
        CssKnownDeclaration::ClipPath(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::ClipPath(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Mask(CssDeclaredValue::Value(value)) => CssValue::Mask(value.clone()),
        CssKnownDeclaration::Mask(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Mask(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MaskImage(CssDeclaredValue::Value(value)) => {
            CssValue::MaskImage(value.clone())
        }
        CssKnownDeclaration::MaskImage(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MaskImage(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MaskSize(CssDeclaredValue::Value(value)) => {
            CssValue::MaskSize(value.clone())
        }
        CssKnownDeclaration::MaskSize(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MaskSize(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MaskPosition(CssDeclaredValue::Value(value)) => {
            CssValue::MaskPosition(value.clone())
        }
        CssKnownDeclaration::MaskPosition(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MaskPosition(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::MaskRepeat(CssDeclaredValue::Value(value)) => {
            CssValue::MaskRepeat(value.clone())
        }
        CssKnownDeclaration::MaskRepeat(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::MaskRepeat(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TransitionProperty(CssDeclaredValue::Value(value)) => {
            CssValue::TransitionProperty(value.clone())
        }
        CssKnownDeclaration::TransitionProperty(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TransitionProperty(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TransitionDuration(CssDeclaredValue::Value(value)) => {
            CssValue::TimeList(value.clone())
        }
        CssKnownDeclaration::TransitionDuration(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TransitionDuration(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TransitionDelay(CssDeclaredValue::Value(value)) => {
            CssValue::TimeList(value.clone())
        }
        CssKnownDeclaration::TransitionDelay(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TransitionDelay(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::TransitionTimingFunction(CssDeclaredValue::Value(value)) => {
            CssValue::EasingList(value.clone())
        }
        CssKnownDeclaration::TransitionTimingFunction(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::TransitionTimingFunction(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::Transition(CssDeclaredValue::Value(value)) => {
            CssValue::Transition(value.clone())
        }
        CssKnownDeclaration::Transition(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Transition(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AnimationName(CssDeclaredValue::Value(value)) => {
            CssValue::AnimationName(value.clone())
        }
        CssKnownDeclaration::AnimationName(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AnimationName(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AnimationDuration(CssDeclaredValue::Value(value)) => {
            CssValue::TimeList(value.clone())
        }
        CssKnownDeclaration::AnimationDuration(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AnimationDuration(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AnimationDelay(CssDeclaredValue::Value(value)) => {
            CssValue::TimeList(value.clone())
        }
        CssKnownDeclaration::AnimationDelay(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AnimationDelay(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AnimationTimingFunction(CssDeclaredValue::Value(value)) => {
            CssValue::EasingList(value.clone())
        }
        CssKnownDeclaration::AnimationTimingFunction(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AnimationTimingFunction(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::AnimationIterationCount(CssDeclaredValue::Value(value)) => {
            CssValue::AnimationIterationCount(value.clone())
        }
        CssKnownDeclaration::AnimationIterationCount(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AnimationIterationCount(CssDeclaredValue::SubstitutionDependent(
            value,
        )) => CssValue::VariableDependent(value.clone()),
        CssKnownDeclaration::AnimationDirection(CssDeclaredValue::Value(value)) => {
            CssValue::AnimationDirection(value.clone())
        }
        CssKnownDeclaration::AnimationDirection(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AnimationDirection(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AnimationFillMode(CssDeclaredValue::Value(value)) => {
            CssValue::AnimationFillMode(value.clone())
        }
        CssKnownDeclaration::AnimationFillMode(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AnimationFillMode(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::AnimationPlayState(CssDeclaredValue::Value(value)) => {
            CssValue::AnimationPlayState(value.clone())
        }
        CssKnownDeclaration::AnimationPlayState(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::AnimationPlayState(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        CssKnownDeclaration::Animation(CssDeclaredValue::Value(value)) => {
            CssValue::Animation(value.clone())
        }
        CssKnownDeclaration::Animation(CssDeclaredValue::Global(keyword)) => {
            CssValue::GlobalKeyword(*keyword)
        }
        CssKnownDeclaration::Animation(CssDeclaredValue::SubstitutionDependent(value)) => {
            CssValue::VariableDependent(value.clone())
        }
        _ => unreachable!("test adapter saw a future known-declaration branch"),
    }
}

pub(crate) struct AcceptedDeclarationCase {
    pub(crate) label: &'static str,
    pub(crate) property_name: &'static str,
    pub(crate) authored_value: &'static str,
    pub(crate) expected_property: CssProperty,
    pub(crate) expected_value: CssValue,
}

impl AcceptedDeclarationCase {
    pub(crate) fn supported_global_inherit(supported_property: &SupportedProperty) -> Self {
        Self::global_inherit(
            supported_property.name,
            supported_property.known_property.into(),
        )
    }

    pub(crate) fn global_inherit(
        property_name: &'static str,
        expected_property: CssProperty,
    ) -> Self {
        Self {
            label: property_name,
            property_name,
            authored_value: "inherit",
            expected_property,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::Inherit),
        }
    }

    pub(crate) fn assert_accepts(&self) -> CssDeclaration {
        let declaration = parse_single_declaration(self.property_name, self.authored_value);
        assert_eq!(
            declaration.property(),
            &self.expected_property,
            "{} parsed to the wrong property",
            self.label,
        );
        assert_eq!(
            declaration.value(),
            &self.expected_value,
            "{} parsed to the wrong value",
            self.label,
        );
        declaration
    }
}

pub(crate) struct AcceptedValueCase {
    pub(crate) label: &'static str,
    pub(crate) property_name: &'static str,
    pub(crate) authored_value: &'static str,
    pub(crate) expected_property: CssProperty,
    pub(crate) assert_value: fn(&CssValue),
}

impl AcceptedValueCase {
    pub(crate) fn assert_accepts(&self) -> CssDeclaration {
        let declaration = parse_single_declaration(self.property_name, self.authored_value);
        assert_eq!(
            declaration.property(),
            &self.expected_property,
            "{} parsed to the wrong property",
            self.label,
        );
        (self.assert_value)(&declaration.value());
        declaration
    }
}

pub(crate) enum ExpectedErrorKind {
    InvalidSyntax,
    InvalidSelector,
    InvalidSyntaxOrUnsupportedValueForProperty { property: &'static str },
    UnsupportedAtRule { name: &'static str },
    UnknownProperty { name: &'static str },
    UnsupportedValueForProperty { property: &'static str },
    UnsupportedValue { property: Option<&'static str> },
}

impl ExpectedErrorKind {
    fn assert_matches(&self, actual: &ErrorKind, label: &str) {
        match (self, actual) {
            (
                Self::InvalidSyntax,
                ErrorKind::UnexpectedEnd(_)
                | ErrorKind::UnexpectedToken(_)
                | ErrorKind::InvalidQualifiedRule(_)
                | ErrorKind::InvalidAtRulePrelude(_)
                | ErrorKind::InvalidAtRuleBody(_)
                | ErrorKind::InvalidPropertyValue(_),
            ) => {}
            (Self::InvalidSelector, ErrorKind::InvalidSelector(_)) => {}
            (
                Self::InvalidSyntaxOrUnsupportedValueForProperty { property },
                ErrorKind::InvalidPropertyValue(detail),
            ) if crate::validation::property_for_supported_name(property)
                == Some(detail.property()) => {}
            (
                Self::InvalidSyntaxOrUnsupportedValueForProperty { .. },
                ErrorKind::UnexpectedEnd(_)
                | ErrorKind::UnexpectedToken(_)
                | ErrorKind::InvalidQualifiedRule(_)
                | ErrorKind::InvalidColorSyntax(_),
            ) => {}
            (Self::UnsupportedAtRule { name }, ErrorKind::UnsupportedAtRule(detail))
                if *name == detail.name().as_str() => {}
            (Self::UnsupportedAtRule { name }, ErrorKind::UnknownAtRule(detail))
                if *name == detail.name().as_str() => {}
            (Self::UnknownProperty { name }, ErrorKind::UnknownProperty(detail))
                if *name == detail.name().as_str() => {}
            (
                Self::UnsupportedValueForProperty { property },
                ErrorKind::InvalidPropertyValue(detail),
            ) if crate::validation::property_for_supported_name(property)
                == Some(detail.property()) => {}
            (Self::UnsupportedValueForProperty { .. }, ErrorKind::InvalidColorSyntax(_)) => {}
            (
                Self::UnsupportedValue {
                    property: Some(property),
                },
                ErrorKind::InvalidPropertyValue(detail),
            ) if crate::validation::property_for_supported_name(property)
                == Some(detail.property()) => {}
            (
                Self::UnsupportedValue { property: None },
                ErrorKind::UnexpectedEnd(_)
                | ErrorKind::UnexpectedToken(_)
                | ErrorKind::InvalidQualifiedRule(_)
                | ErrorKind::InvalidColorSyntax(_)
                | ErrorKind::InvalidMediaQuery(_),
            ) => {}
            _ => panic!("{label} rejected with unexpected error kind: {actual:?}"),
        }
    }
}

pub(crate) struct RejectedSheetCase {
    pub(crate) label: &'static str,
    pub(crate) input: &'static str,
    pub(crate) expected_error: ExpectedErrorKind,
}

impl RejectedSheetCase {
    pub(crate) fn assert_rejects(&self) -> Error {
        let error = parse_sheet(self.input).expect_err("invalid CSS must reject the whole sheet");
        self.expected_error.assert_matches(error.kind(), self.label);
        error
    }
}

pub(crate) struct RejectedDeclarationCase {
    pub(crate) label: &'static str,
    pub(crate) property_name: &'static str,
    pub(crate) authored_value: &'static str,
    pub(crate) expected_error: ExpectedErrorKind,
    pub(crate) property_name_should_be_recognized: bool,
}

impl RejectedDeclarationCase {
    pub(crate) fn assert_rejects(&self) -> Error {
        let css = declaration_sheet(self.property_name, self.authored_value);
        let error = assert_sheet_rejected(&css, &self.expected_error);
        assert_eq!(
            matches!(error.kind(), ErrorKind::UnknownProperty(_)),
            !self.property_name_should_be_recognized,
            "{} property-name recognition mismatch",
            self.label,
        );
        error
    }
}

pub(crate) fn assert_accepts_declarations(cases: &[AcceptedDeclarationCase]) {
    for case in cases {
        case.assert_accepts();
    }
}

pub(crate) fn assert_accepts_value_cases(cases: &[AcceptedValueCase]) {
    for case in cases {
        case.assert_accepts();
    }
}

pub(crate) fn assert_rejects_declarations(cases: &[RejectedDeclarationCase]) {
    for case in cases {
        case.assert_rejects();
    }
}

pub(crate) fn assert_rejects_sheets(cases: &[RejectedSheetCase]) {
    for case in cases {
        case.assert_rejects();
    }
}

pub(crate) fn parse_single_declaration(
    property_name: &str,
    authored_value: &str,
) -> CssDeclaration {
    parse_single_declaration_from_sheet(&declaration_sheet(property_name, authored_value))
}

pub(crate) fn parse_single_declaration_from_sheet(input: &str) -> CssDeclaration {
    let sheet = parse_sheet(input).unwrap_or_else(|error| panic!("{input} should parse: {error}"));
    only_declaration(&sheet, input)
}

pub(crate) fn parse_single_declaration_value(
    property_name: &str,
    authored_value: &str,
) -> CssValue {
    parse_single_declaration(property_name, authored_value).value()
}

pub(crate) fn assert_sheet_rejected(input: &str, expected_error: &ExpectedErrorKind) -> Error {
    let error = parse_sheet(input).expect_err("invalid CSS must reject the whole sheet");
    expected_error.assert_matches(error.kind(), input);
    error
}

pub(crate) fn accepted_declaration_cases() -> Vec<AcceptedDeclarationCase> {
    supported_properties()
        .iter()
        .map(AcceptedDeclarationCase::supported_global_inherit)
        .collect()
}

fn declaration_sheet(property_name: &str, authored_value: &str) -> String {
    format!(".test {{ {property_name}: {authored_value}; }}")
}

fn only_declaration(sheet: &CssSheet, input: &str) -> CssDeclaration {
    let [rule] = sheet.rules() else {
        panic!("{input} should parse exactly one rule");
    };
    let rule = style_rule(rule);
    let [declaration] = rule.declarations().as_slice() else {
        panic!("{input} should parse exactly one declaration");
    };
    declaration.clone()
}

fn style_rule(rule: &CssRule) -> &CssStyleRule {
    match rule {
        CssRule::Style(rule) => rule,
        unexpected => panic!("expected style rule, got {unexpected:?}"),
    }
}

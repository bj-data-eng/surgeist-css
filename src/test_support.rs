use crate::{
    CssDeclaration, CssGlobalKeyword, CssProperty, CssSheet, CssValue, Error, ErrorKind,
    parse_sheet,
};

pub(crate) struct AcceptedDeclarationCase {
    pub(crate) label: &'static str,
    pub(crate) property_name: &'static str,
    pub(crate) authored_value: &'static str,
    pub(crate) expected_property: CssProperty,
    pub(crate) expected_value: CssValue,
}

impl AcceptedDeclarationCase {
    pub(crate) const fn global_inherit(
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
            self.expected_property,
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
            self.expected_property,
            "{} parsed to the wrong property",
            self.label,
        );
        (self.assert_value)(declaration.value());
        declaration
    }
}

pub(crate) enum ExpectedErrorKind {
    InvalidSyntax,
    InvalidSelector,
    InvalidSyntaxOrUnsupportedValueForProperty {
        property: &'static str,
    },
    UnsupportedAtRule {
        name: &'static str,
    },
    UnknownProperty {
        name: &'static str,
    },
    UnsupportedValueForProperty {
        property: &'static str,
    },
    UnsupportedValue {
        property: Option<&'static str>,
        reason: &'static str,
    },
}

impl ExpectedErrorKind {
    fn assert_matches(&self, actual: &ErrorKind, label: &str) {
        match (self, actual) {
            (Self::InvalidSyntax, ErrorKind::InvalidSyntax { .. }) => {}
            (Self::InvalidSelector, ErrorKind::InvalidSelector { .. }) => {}
            (
                Self::InvalidSyntaxOrUnsupportedValueForProperty { property },
                ErrorKind::UnsupportedValue {
                    property: actual_property,
                    ..
                },
            ) if Some(*property) == actual_property.as_deref() => {}
            (
                Self::InvalidSyntaxOrUnsupportedValueForProperty { .. },
                ErrorKind::InvalidSyntax { .. },
            ) => {}
            (Self::UnsupportedAtRule { name }, ErrorKind::UnsupportedAtRule { name: actual })
                if name == actual => {}
            (Self::UnknownProperty { name }, ErrorKind::UnknownProperty { name: actual })
                if name == actual => {}
            (
                Self::UnsupportedValueForProperty { property },
                ErrorKind::UnsupportedValue {
                    property: actual_property,
                    ..
                },
            ) if Some(*property) == actual_property.as_deref() => {}
            (
                Self::UnsupportedValue { property, reason },
                ErrorKind::UnsupportedValue {
                    property: actual_property,
                    reason: actual_reason,
                },
            ) if *property == actual_property.as_deref() && *reason == actual_reason => {}
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
            matches!(error.kind(), ErrorKind::UnknownProperty { .. }),
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
    parse_single_declaration(property_name, authored_value)
        .value()
        .clone()
}

pub(crate) fn assert_sheet_rejected(input: &str, expected_error: &ExpectedErrorKind) -> Error {
    let error = parse_sheet(input).expect_err("invalid CSS must reject the whole sheet");
    expected_error.assert_matches(error.kind(), input);
    error
}

pub(crate) fn accepted_declaration_cases() -> &'static [AcceptedDeclarationCase] {
    ACCEPTED_DECLARATION_CASES
}

fn declaration_sheet(property_name: &str, authored_value: &str) -> String {
    format!(".test {{ {property_name}: {authored_value}; }}")
}

fn only_declaration(sheet: &CssSheet, input: &str) -> CssDeclaration {
    let [rule] = sheet.rules() else {
        panic!("{input} should parse exactly one rule");
    };
    let [declaration] = rule.declarations() else {
        panic!("{input} should parse exactly one declaration");
    };
    declaration.clone()
}

macro_rules! global_inherit_case {
    ($property_name:literal, $expected_property:expr) => {
        AcceptedDeclarationCase::global_inherit($property_name, $expected_property)
    };
}

const ACCEPTED_DECLARATION_CASES: &[AcceptedDeclarationCase] = &[
    global_inherit_case!("all", CssProperty::All),
    global_inherit_case!("display", CssProperty::Display),
    global_inherit_case!("box-sizing", CssProperty::BoxSizing),
    global_inherit_case!("position", CssProperty::Position),
    global_inherit_case!("direction", CssProperty::Direction),
    global_inherit_case!("overflow", CssProperty::Overflow),
    global_inherit_case!("overflow-x", CssProperty::OverflowX),
    global_inherit_case!("overflow-y", CssProperty::OverflowY),
    global_inherit_case!("flex-direction", CssProperty::FlexDirection),
    global_inherit_case!("flex-wrap", CssProperty::FlexWrap),
    global_inherit_case!("float", CssProperty::Float),
    global_inherit_case!("clear", CssProperty::Clear),
    global_inherit_case!("align-content", CssProperty::AlignContent),
    global_inherit_case!("justify-content", CssProperty::JustifyContent),
    global_inherit_case!("align-items", CssProperty::AlignItems),
    global_inherit_case!("align-self", CssProperty::AlignSelf),
    global_inherit_case!("justify-items", CssProperty::JustifyItems),
    global_inherit_case!("justify-self", CssProperty::JustifySelf),
    global_inherit_case!("place-content", CssProperty::PlaceContent),
    global_inherit_case!("place-items", CssProperty::PlaceItems),
    global_inherit_case!("place-self", CssProperty::PlaceSelf),
    global_inherit_case!("visibility", CssProperty::Visibility),
    global_inherit_case!("content-visibility", CssProperty::ContentVisibility),
    global_inherit_case!("width", CssProperty::Width),
    global_inherit_case!("height", CssProperty::Height),
    global_inherit_case!("min-width", CssProperty::MinWidth),
    global_inherit_case!("min-height", CssProperty::MinHeight),
    global_inherit_case!("max-width", CssProperty::MaxWidth),
    global_inherit_case!("max-height", CssProperty::MaxHeight),
    global_inherit_case!("flex-basis", CssProperty::FlexBasis),
    global_inherit_case!("gap", CssProperty::Gap),
    global_inherit_case!("row-gap", CssProperty::RowGap),
    global_inherit_case!("column-gap", CssProperty::ColumnGap),
    global_inherit_case!("grid-flow-tolerance", CssProperty::GridFlowTolerance),
    global_inherit_case!("grid-template-rows", CssProperty::GridTemplateRows),
    global_inherit_case!("grid-template-columns", CssProperty::GridTemplateColumns),
    global_inherit_case!("grid-template-areas", CssProperty::GridTemplateAreas),
    global_inherit_case!("grid-template", CssProperty::GridTemplate),
    global_inherit_case!("grid-auto-rows", CssProperty::GridAutoRows),
    global_inherit_case!("grid-auto-columns", CssProperty::GridAutoColumns),
    global_inherit_case!("grid-auto-flow", CssProperty::GridAutoFlow),
    global_inherit_case!("grid-row-start", CssProperty::GridRowStart),
    global_inherit_case!("grid-row-end", CssProperty::GridRowEnd),
    global_inherit_case!("grid-column-start", CssProperty::GridColumnStart),
    global_inherit_case!("grid-column-end", CssProperty::GridColumnEnd),
    global_inherit_case!("grid-row", CssProperty::GridRow),
    global_inherit_case!("grid-column", CssProperty::GridColumn),
    global_inherit_case!("grid-area", CssProperty::GridArea),
    global_inherit_case!("grid", CssProperty::Grid),
    global_inherit_case!("font-size", CssProperty::FontSize),
    global_inherit_case!("line-height", CssProperty::LineHeight),
    global_inherit_case!("writing-mode", CssProperty::WritingMode),
    global_inherit_case!("text-align", CssProperty::TextAlign),
    global_inherit_case!("text-align-last", CssProperty::TextAlignLast),
    global_inherit_case!("text-indent", CssProperty::TextIndent),
    global_inherit_case!("vertical-align", CssProperty::VerticalAlign),
    global_inherit_case!("font-family", CssProperty::FontFamily),
    global_inherit_case!("font", CssProperty::Font),
    global_inherit_case!("font-weight", CssProperty::FontWeight),
    global_inherit_case!("font-style", CssProperty::FontStyle),
    global_inherit_case!("font-stretch", CssProperty::FontStretch),
    global_inherit_case!("font-variant", CssProperty::FontVariant),
    global_inherit_case!("font-feature-settings", CssProperty::FontFeatureSettings),
    global_inherit_case!("letter-spacing", CssProperty::LetterSpacing),
    global_inherit_case!("text-wrap", CssProperty::TextWrap),
    global_inherit_case!("white-space", CssProperty::WhiteSpace),
    global_inherit_case!("word-break", CssProperty::WordBreak),
    global_inherit_case!("overflow-wrap", CssProperty::OverflowWrap),
    global_inherit_case!("text-overflow", CssProperty::TextOverflow),
    global_inherit_case!("text-decoration", CssProperty::TextDecoration),
    global_inherit_case!("text-decoration-line", CssProperty::TextDecorationLine),
    global_inherit_case!("text-decoration-color", CssProperty::TextDecorationColor),
    global_inherit_case!("text-decoration-style", CssProperty::TextDecorationStyle),
    global_inherit_case!(
        "text-decoration-thickness",
        CssProperty::TextDecorationThickness
    ),
    global_inherit_case!("text-transform", CssProperty::TextTransform),
    global_inherit_case!("inset", CssProperty::Inset),
    global_inherit_case!("top", CssProperty::Top),
    global_inherit_case!("right", CssProperty::Right),
    global_inherit_case!("bottom", CssProperty::Bottom),
    global_inherit_case!("left", CssProperty::Left),
    global_inherit_case!("z-index", CssProperty::ZIndex),
    global_inherit_case!("box-decoration-break", CssProperty::BoxDecorationBreak),
    global_inherit_case!("margin", CssProperty::Margin),
    global_inherit_case!("margin-top", CssProperty::MarginTop),
    global_inherit_case!("margin-right", CssProperty::MarginRight),
    global_inherit_case!("margin-bottom", CssProperty::MarginBottom),
    global_inherit_case!("margin-left", CssProperty::MarginLeft),
    global_inherit_case!("padding", CssProperty::Padding),
    global_inherit_case!("padding-top", CssProperty::PaddingTop),
    global_inherit_case!("padding-right", CssProperty::PaddingRight),
    global_inherit_case!("padding-bottom", CssProperty::PaddingBottom),
    global_inherit_case!("padding-left", CssProperty::PaddingLeft),
    global_inherit_case!("border", CssProperty::Border),
    global_inherit_case!("border-top", CssProperty::BorderTop),
    global_inherit_case!("border-right", CssProperty::BorderRight),
    global_inherit_case!("border-bottom", CssProperty::BorderBottom),
    global_inherit_case!("border-left", CssProperty::BorderLeft),
    global_inherit_case!("border-width", CssProperty::BorderWidth),
    global_inherit_case!("border-top-width", CssProperty::BorderTopWidth),
    global_inherit_case!("border-right-width", CssProperty::BorderRightWidth),
    global_inherit_case!("border-bottom-width", CssProperty::BorderBottomWidth),
    global_inherit_case!("border-left-width", CssProperty::BorderLeftWidth),
    global_inherit_case!("color", CssProperty::Color),
    global_inherit_case!("background", CssProperty::Background),
    global_inherit_case!("background-color", CssProperty::BackgroundColor),
    global_inherit_case!("border-color", CssProperty::BorderColor),
    global_inherit_case!("border-top-color", CssProperty::BorderTopColor),
    global_inherit_case!("border-right-color", CssProperty::BorderRightColor),
    global_inherit_case!("border-bottom-color", CssProperty::BorderBottomColor),
    global_inherit_case!("border-left-color", CssProperty::BorderLeftColor),
    global_inherit_case!("background-image", CssProperty::BackgroundImage),
    global_inherit_case!("background-position", CssProperty::BackgroundPosition),
    global_inherit_case!("background-size", CssProperty::BackgroundSize),
    global_inherit_case!("background-repeat", CssProperty::BackgroundRepeat),
    global_inherit_case!("background-origin", CssProperty::BackgroundOrigin),
    global_inherit_case!("background-clip", CssProperty::BackgroundClip),
    global_inherit_case!("background-attachment", CssProperty::BackgroundAttachment),
    global_inherit_case!("border-style", CssProperty::BorderStyle),
    global_inherit_case!("border-top-style", CssProperty::BorderTopStyle),
    global_inherit_case!("border-right-style", CssProperty::BorderRightStyle),
    global_inherit_case!("border-bottom-style", CssProperty::BorderBottomStyle),
    global_inherit_case!("border-left-style", CssProperty::BorderLeftStyle),
    global_inherit_case!("border-radius", CssProperty::BorderRadius),
    global_inherit_case!("border-top-left-radius", CssProperty::BorderTopLeftRadius),
    global_inherit_case!("border-top-right-radius", CssProperty::BorderTopRightRadius),
    global_inherit_case!(
        "border-bottom-right-radius",
        CssProperty::BorderBottomRightRadius
    ),
    global_inherit_case!(
        "border-bottom-left-radius",
        CssProperty::BorderBottomLeftRadius
    ),
    global_inherit_case!("box-shadow", CssProperty::BoxShadow),
    global_inherit_case!("opacity", CssProperty::Opacity),
    global_inherit_case!("flex-grow", CssProperty::FlexGrow),
    global_inherit_case!("flex-shrink", CssProperty::FlexShrink),
    global_inherit_case!("order", CssProperty::Order),
    global_inherit_case!("flex", CssProperty::Flex),
    global_inherit_case!("justify-tracks", CssProperty::JustifyTracks),
    global_inherit_case!("align-tracks", CssProperty::AlignTracks),
    global_inherit_case!("aspect-ratio", CssProperty::AspectRatio),
    global_inherit_case!("scrollbar-width", CssProperty::ScrollbarWidth),
    global_inherit_case!("cursor", CssProperty::Cursor),
    global_inherit_case!("pointer-events", CssProperty::PointerEvents),
    global_inherit_case!("user-select", CssProperty::UserSelect),
    global_inherit_case!("outline", CssProperty::Outline),
    global_inherit_case!("outline-color", CssProperty::OutlineColor),
    global_inherit_case!("outline-style", CssProperty::OutlineStyle),
    global_inherit_case!("outline-width", CssProperty::OutlineWidth),
    global_inherit_case!("transform", CssProperty::Transform),
    global_inherit_case!("transform-origin", CssProperty::TransformOrigin),
    global_inherit_case!("translate", CssProperty::Translate),
    global_inherit_case!("rotate", CssProperty::Rotate),
    global_inherit_case!("scale", CssProperty::Scale),
    global_inherit_case!("filter", CssProperty::Filter),
    global_inherit_case!("backdrop-filter", CssProperty::BackdropFilter),
    global_inherit_case!("clip-path", CssProperty::ClipPath),
    global_inherit_case!("mask", CssProperty::Mask),
    global_inherit_case!("mask-image", CssProperty::MaskImage),
    global_inherit_case!("mask-size", CssProperty::MaskSize),
    global_inherit_case!("mask-position", CssProperty::MaskPosition),
    global_inherit_case!("mask-repeat", CssProperty::MaskRepeat),
    global_inherit_case!("transition-property", CssProperty::TransitionProperty),
    global_inherit_case!("transition-duration", CssProperty::TransitionDuration),
    global_inherit_case!("transition-delay", CssProperty::TransitionDelay),
    global_inherit_case!(
        "transition-timing-function",
        CssProperty::TransitionTimingFunction
    ),
    global_inherit_case!("transition", CssProperty::Transition),
    global_inherit_case!("animation-name", CssProperty::AnimationName),
    global_inherit_case!("animation-duration", CssProperty::AnimationDuration),
    global_inherit_case!("animation-delay", CssProperty::AnimationDelay),
    global_inherit_case!(
        "animation-timing-function",
        CssProperty::AnimationTimingFunction
    ),
    global_inherit_case!(
        "animation-iteration-count",
        CssProperty::AnimationIterationCount
    ),
    global_inherit_case!("animation-direction", CssProperty::AnimationDirection),
    global_inherit_case!("animation-fill-mode", CssProperty::AnimationFillMode),
    global_inherit_case!("animation-play-state", CssProperty::AnimationPlayState),
    global_inherit_case!("animation", CssProperty::Animation),
];

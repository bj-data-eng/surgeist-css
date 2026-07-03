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
    pub const fn property(&self) -> CssProperty {
        self.property
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssProperty {
    Display,
    BoxSizing,
    Position,
    Direction,
    Overflow,
    OverflowX,
    OverflowY,
    FlexDirection,
    FlexWrap,
    AlignItems,
    AlignSelf,
    JustifyItems,
    JustifySelf,
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
    FontSize,
    LineHeight,
    Margin,
    Padding,
    BorderWidth,
    Color,
    Background,
    BorderColor,
    Opacity,
    FlexGrow,
    FlexShrink,
    AspectRatio,
    ScrollbarWidth,
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
    Display(CssDisplay),
    BoxSizing(CssBoxSizing),
    Position(CssLayoutPosition),
    Direction(CssDirection),
    Overflow(CssOverflow),
    OverflowAxes(CssOverflowAxes),
    FlexDirection(CssFlexDirection),
    FlexWrap(CssFlexWrap),
    AlignItems(CssAlignItems),
    Length(CssLength),
    GridFlowTolerance(CssGridFlowTolerance),
    Edges(CssEdges),
    Color(CssColor),
    Number(f32),
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
    Relative,
    Absolute,
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
pub enum CssAlignItems {
    Start,
    End,
    SafeEnd,
    FlexStart,
    FlexEnd,
    SafeFlexEnd,
    Center,
    SafeCenter,
    Baseline,
    LastBaseline,
    Stretch,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssGridFlowTolerance {
    Normal,
    Infinite,
    Length(CssLength),
    Percent(f32),
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
    value: f32,
    unit: CssLengthUnit,
}

impl CssLengthDimension {
    #[must_use]
    pub const fn new(value: f32, unit: CssLengthUnit) -> Self {
        Self { value, unit }
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.value
    }

    #[must_use]
    pub const fn unit(self) -> CssLengthUnit {
        self.unit
    }

    #[must_use]
    pub fn to_css_string(self) -> String {
        format!(
            "{}{}",
            format_css_number(self.value),
            self.unit.as_css_str()
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssLength {
    Px(f32),
    Dimension(CssLengthDimension),
    Percent(f32),
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
    pub const fn px(value: f32) -> Self {
        Self::Px(value)
    }

    #[must_use]
    pub const fn percent(value: f32) -> Self {
        Self::Percent(value)
    }

    #[must_use]
    pub const fn dimension(value: f32, unit: CssLengthUnit) -> Self {
        match unit {
            CssLengthUnit::Px => Self::Px(value),
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl CssColor {
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::rgba(1.0, 1.0, 1.0, 1.0);

    #[must_use]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssSelector {
    Tag(String),
    Key(String),
    Class(String),
    Compound(CssCompoundSelector),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssCompoundSelector {
    tag: Option<String>,
    key: Option<String>,
    classes: Vec<String>,
}

impl CssCompoundSelector {
    #[must_use]
    pub(crate) fn new(tag: Option<String>, key: Option<String>, classes: Vec<String>) -> Self {
        Self { tag, key, classes }
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum CssCalcLength {
    Px(f32),
    Dimension(CssLengthDimension),
    Percent(f32),
    Sum(Vec<CssCalcLengthTerm>),
}

impl CssCalcLength {
    #[must_use]
    pub const fn px(value: f32) -> Self {
        Self::Px(value)
    }

    #[must_use]
    pub const fn percent(value: f32) -> Self {
        Self::Percent(value)
    }

    #[must_use]
    pub const fn dimension(value: f32, unit: CssLengthUnit) -> Self {
        match unit {
            CssLengthUnit::Px => Self::Px(value),
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
            Self::Px(value) => format!("{}px", format_css_number(*value)),
            Self::Dimension(length) => length.to_css_string(),
            Self::Percent(value) => format!("{}%", format_css_number(*value)),
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

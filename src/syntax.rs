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
    BorderColor,
    BorderTopColor,
    BorderRightColor,
    BorderBottomColor,
    BorderLeftColor,
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
    Edges(CssEdges),
    Color(CssColor),
    ZIndex(CssZIndex),
    BoxDecorationBreak(CssBoxDecorationBreak),
    Border(CssBorder),
    BorderStyle(CssBorderStyle),
    BorderStyles(CssBorderStyles),
    BorderRadius(CssBorderRadii),
    CornerRadius(CssCornerRadius),
    BoxShadow(CssBoxShadow),
    Order(CssOrder),
    Flex(CssFlex),
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
    Fraction(f32),
    MinContent,
    MaxContent,
    Auto,
}

impl CssGridTrackBreadth {
    #[must_use]
    pub const fn length(length: CssLength) -> Self {
        Self::Length(length)
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
        grow: f32,
        shrink: Option<f32>,
        basis: Option<CssLength>,
    },
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
        CssLength::Px(value) => *value >= 0.0,
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
        CssLength::Px(value) | CssLength::Percent(value) => *value >= 0.0,
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

pub(crate) fn length_has_negative_component(length: &CssLength) -> bool {
    match length {
        CssLength::Px(value) | CssLength::Percent(value) => *value < 0.0,
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
        CssCalcLength::Px(value) | CssCalcLength::Percent(value) => *value < 0.0,
        CssCalcLength::Dimension(length) => length.value() < 0.0,
        CssCalcLength::Sum(terms) => terms
            .iter()
            .any(|term| calc_has_negative_component(term.value())),
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

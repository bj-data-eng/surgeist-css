# Surgeist CSS-Wide Validation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add CSS-wide property, unit, and keyword validation to `surgeist-css` while preserving its role as the CSS-owned authored syntax front door.

**Architecture:** Keep parsing and validation in `surgeist-css` and keep lowering to style/layout out of this crate. Add a private validation module that classifies property names, global keywords, property-domain keywords, and CSS units before constructing the public `Css*` syntax types. Public syntax only grows where authored CSS needs representation, namely CSS-wide global keywords.

**Tech Stack:** Rust 2024, `cssparser = 0.37.0`, crate-local unit tests in `src/lib.rs`, private validation helpers in `src/validation.rs`, public authored syntax in `src/syntax.rs`.

---

## Scope And References

This plan is crate-local to `/Users/codex/Development/surgeist-css`.

Use these local instructions while executing:

- `AGENTS.md`
- `guidance/surgeist-rust-modeling-guide.md`

CSS reference points used for scope:

- CSS global keywords are treated as CSS-wide authored syntax: `inherit`, `initial`, `unset`, `revert`, and `revert-layer`.
- CSS length units are classified by parser support. `px` and `%` remain the currently supported numeric authored forms in this crate; other CSS length units are recognized as known-but-unsupported instead of falling through to vague syntax errors.

Reference links:

- [CSS Cascading and Inheritance Level 5](https://www.w3.org/TR/css-cascade-5/)
- [CSS Values and Units Module Level 3](https://www.w3.org/TR/css-values-3/)
- [MDN `<length>` CSS type](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/length)

Non-goals:

- Do not edit sibling crates.
- Do not add a `surgeist-style` dependency back to this crate.
- Do not implement root CSS-to-style lowering here.
- Do not implement every unsupported CSS property parser.
- Do not add a generated API artifact; this crate snapshot has no `api/` directory.
- Do not turn `CssValue` into a broad cross-property validation bag. Property-specific parser functions remain the source of what each property accepts.

## File Structure

- Create: `src/validation.rs`
  - Own private classification helpers for property names, global keywords, CSS length units, and keyword domains.
  - No public API from this module.
- Modify: `src/syntax.rs`
  - Add `CssGlobalKeyword`.
  - Add `CssValue::GlobalKeyword(CssGlobalKeyword)`.
  - Keep all fields/constructors consistent with authored syntax only.
- Modify: `src/lib.rs`
  - Use validation helpers from `src/validation.rs`.
  - Parse CSS-wide global keywords for every supported property.
  - Distinguish unknown properties from known-but-unsupported properties.
  - Route length unit and keyword failures through validation helpers.
  - Add focused tests.
- No changes expected in `Cargo.toml`, `README.md`, `AGENTS.md`, or sibling crates.

## Public API Target

`src/syntax.rs` should gain only this public authored-syntax shape:

```rust
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
```

`ErrorKind` should distinguish typos from recognized-but-unsupported CSS properties:

```rust
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    InvalidSyntax { reason: String },
    InvalidSelector { reason: String },
    UnsupportedAtRule { name: String },
    UnknownProperty { name: String },
    UnsupportedProperty { name: String },
    UnsupportedValue {
        property: Option<String>,
        reason: String,
    },
    InvalidColor { value: String },
}
```

Keep `UnsupportedProperty` for real CSS property names this crate deliberately does not parse. Use `UnknownProperty` for names that are not in the crate-scoped property-name registry.

The property-name registry is a crate-owned validation seed, not a complete web-platform database. It must include every currently supported property spelling plus a broad set of real, common CSS property names in domains Surgeist already models or is likely to model next. It must not include style-only or Surgeist-internal vocabulary that is not a CSS property name.

---

## Task 1: Add Property Name Classification

**Files:**

- Create: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Add failing tests for known and unknown property names**

Add these tests inside `#[cfg(test)] mod tests` in `src/lib.rs`:

```rust
#[test]
fn known_but_unsupported_property_has_typed_error_kind() {
    let error = parse_sheet(".panel { float: left; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedProperty {
            name: "float".to_owned(),
        }
    );
    assert!(error.message().contains("unsupported CSS property `float`"));
}

#[test]
fn another_known_but_unsupported_property_is_not_treated_as_unknown() {
    let error = parse_sheet(".panel { z-index: 10; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedProperty {
            name: "z-index".to_owned(),
        }
    );
}

#[test]
fn typo_property_has_unknown_property_error_kind() {
    let error = parse_sheet(".panel { widht: 10px; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnknownProperty {
            name: "widht".to_owned(),
        }
    );
    assert!(error.message().contains("unknown CSS property `widht`"));
}
```

- [ ] **Step 2: Run property tests to verify failure**

Run:

```sh
cargo test -p surgeist-css property
```

Expected: fail because `UnknownProperty` and property classification do not exist yet.

- [ ] **Step 3: Add private property classification helpers**

Create `src/validation.rs`:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PropertyNameStatus {
    Supported,
    KnownUnsupported,
    Unknown,
}

const SUPPORTED_PROPERTY_NAMES: &[&str] = &[
    "display",
    "box-sizing",
    "position",
    "direction",
    "overflow",
    "overflow-x",
    "overflow-y",
    "flex-direction",
    "flex-wrap",
    "align-items",
    "align-self",
    "justify-items",
    "justify-self",
    "width",
    "height",
    "min-width",
    "min-height",
    "max-width",
    "max-height",
    "flex-basis",
    "gap",
    "row-gap",
    "column-gap",
    "grid-flow-tolerance",
    "font-size",
    "line-height",
    "margin",
    "padding",
    "border-width",
    "color",
    "background",
    "background-color",
    "border-color",
    "opacity",
    "flex-grow",
    "flex-shrink",
    "aspect-ratio",
    "scrollbar-width",
];

const KNOWN_UNSUPPORTED_PROPERTY_NAMES: &[&str] = &[
    "all",
    "inset",
    "top",
    "right",
    "bottom",
    "left",
    "z-index",
    "box-decoration-break",
    "writing-mode",
    "text-align",
    "text-align-last",
    "text-indent",
    "vertical-align",
    "float",
    "clear",
    "align-content",
    "justify-content",
    "place-content",
    "place-items",
    "place-self",
    "grid-template-rows",
    "grid-template-columns",
    "grid-template-areas",
    "grid-template",
    "grid-auto-rows",
    "grid-auto-columns",
    "grid-auto-flow",
    "grid-row-start",
    "grid-row-end",
    "grid-column-start",
    "grid-column-end",
    "grid-row",
    "grid-column",
    "grid-area",
    "grid",
    "order",
    "flex",
    "justify-tracks",
    "align-tracks",
    "margin-top",
    "margin-right",
    "margin-bottom",
    "margin-left",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "border",
    "border-top",
    "border-right",
    "border-bottom",
    "border-left",
    "border-top-width",
    "border-right-width",
    "border-bottom-width",
    "border-left-width",
    "border-top-color",
    "border-right-color",
    "border-bottom-color",
    "border-left-color",
    "border-style",
    "border-top-style",
    "border-right-style",
    "border-bottom-style",
    "border-left-style",
    "border-radius",
    "border-top-left-radius",
    "border-top-right-radius",
    "border-bottom-right-radius",
    "border-bottom-left-radius",
    "box-shadow",
    "visibility",
    "content-visibility",
    "font-family",
    "font",
    "font-weight",
    "font-style",
    "font-stretch",
    "font-variant",
    "font-feature-settings",
    "letter-spacing",
    "text-wrap",
    "white-space",
    "word-break",
    "overflow-wrap",
    "text-overflow",
    "text-decoration",
    "text-decoration-line",
    "text-decoration-color",
    "text-decoration-style",
    "text-decoration-thickness",
    "text-transform",
    "background-image",
    "background-position",
    "background-size",
    "background-repeat",
    "background-origin",
    "background-clip",
    "background-attachment",
    "cursor",
    "pointer-events",
    "user-select",
    "outline",
    "outline-color",
    "outline-style",
    "outline-width",
    "transform",
    "transform-origin",
    "translate",
    "rotate",
    "scale",
    "filter",
    "backdrop-filter",
    "clip-path",
    "mask",
    "mask-image",
    "mask-size",
    "mask-position",
    "mask-repeat",
    "transition-property",
    "transition-duration",
    "transition-delay",
    "transition-timing-function",
    "transition",
    "animation-name",
    "animation-duration",
    "animation-delay",
    "animation-timing-function",
    "animation-iteration-count",
    "animation-direction",
    "animation-fill-mode",
    "animation-play-state",
    "animation",
];

pub(crate) fn classify_property_name(name: &str) -> PropertyNameStatus {
    if contains_ascii_case(SUPPORTED_PROPERTY_NAMES, name) {
        PropertyNameStatus::Supported
    } else if contains_ascii_case(KNOWN_UNSUPPORTED_PROPERTY_NAMES, name) {
        PropertyNameStatus::KnownUnsupported
    } else {
        PropertyNameStatus::Unknown
    }
}

fn contains_ascii_case(haystack: &[&str], needle: &str) -> bool {
    haystack
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(needle))
}
```

- [ ] **Step 4: Wire the module and unknown property errors**

At the top of `src/lib.rs`, add the module:

```rust
mod syntax;
mod validation;

pub use syntax::*;

use validation::{PropertyNameStatus, classify_property_name};
```

Add `UnknownProperty` to `ErrorKind`:

```rust
UnknownProperty {
    name: String,
},
```

Replace the current unsupported-property fallback in `parse_value`:

```rust
_ => return Err(property_name_error(input, name.as_ref())),
```

Add this helper near `unsupported_property`:

```rust
fn property_name_error<'i, 't>(input: &Parser<'i, 't>, name: &str) -> ParseError<'i, Error> {
    match classify_property_name(name) {
        PropertyNameStatus::Supported => unsupported_property(input, name),
        PropertyNameStatus::KnownUnsupported => unsupported_property(input, name),
        PropertyNameStatus::Unknown => unknown_property(input, name),
    }
}

fn unknown_property<'i, 't>(
    input: &Parser<'i, 't>,
    name: impl Into<String>,
) -> ParseError<'i, Error> {
    let name = name.into();
    error_at(
        input.current_source_location(),
        ErrorKind::UnknownProperty { name: name.clone() },
        format!("unknown CSS property `{name}`"),
    )
}
```

- [ ] **Step 5: Run property tests**

Run:

```sh
cargo test -p surgeist-css property
```

Expected: pass.

- [ ] **Step 6: Commit**

```sh
git add src/lib.rs src/validation.rs
git commit -m "Classify CSS property names"
```

---

## Task 2: Add CSS-Wide Global Keywords

**Files:**

- Modify: `src/syntax.rs`
- Modify: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Add failing tests for CSS-wide global keywords**

Add these tests in `src/lib.rs`:

```rust
#[test]
fn parses_global_keywords_for_different_value_domains() {
    assert_eq!(
        declaration_value(".panel { width: inherit; }", CssProperty::Width),
        CssValue::GlobalKeyword(CssGlobalKeyword::Inherit)
    );
    assert_eq!(
        declaration_value(".panel { display: initial; }", CssProperty::Display),
        CssValue::GlobalKeyword(CssGlobalKeyword::Initial)
    );
    assert_eq!(
        declaration_value(".panel { color: unset; }", CssProperty::Color),
        CssValue::GlobalKeyword(CssGlobalKeyword::Unset)
    );
}

#[test]
fn parses_newer_global_keywords_as_authored_syntax() {
    assert_eq!(
        declaration_value(".panel { padding: revert; }", CssProperty::Padding),
        CssValue::GlobalKeyword(CssGlobalKeyword::Revert)
    );
    assert_eq!(
        declaration_value(".panel { margin: revert-layer; }", CssProperty::Margin),
        CssValue::GlobalKeyword(CssGlobalKeyword::RevertLayer)
    );
}

#[test]
fn global_keyword_must_be_the_whole_value() {
    let error = parse_sheet(".panel { width: inherit 10px; }").unwrap_err();

    assert!(matches!(error.kind(), ErrorKind::InvalidSyntax { .. }));
}
```

- [ ] **Step 2: Run global keyword tests to verify failure**

Run:

```sh
cargo test -p surgeist-css global_keyword
```

Expected: fail because `CssGlobalKeyword` and global keyword parsing do not exist.

- [ ] **Step 3: Add public authored global keyword syntax**

In `src/syntax.rs`, add this enum before `CssValue`:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssGlobalKeyword {
    Inherit,
    Initial,
    Unset,
    Revert,
    RevertLayer,
}
```

Add this variant as the first `CssValue` variant:

```rust
GlobalKeyword(CssGlobalKeyword),
```

- [ ] **Step 4: Add global keyword classification**

In `src/validation.rs`, import the public syntax enum and add:

```rust
use crate::CssGlobalKeyword;

pub(crate) fn parse_global_keyword(name: &str) -> Option<CssGlobalKeyword> {
    if name.eq_ignore_ascii_case("inherit") {
        Some(CssGlobalKeyword::Inherit)
    } else if name.eq_ignore_ascii_case("initial") {
        Some(CssGlobalKeyword::Initial)
    } else if name.eq_ignore_ascii_case("unset") {
        Some(CssGlobalKeyword::Unset)
    } else if name.eq_ignore_ascii_case("revert") {
        Some(CssGlobalKeyword::Revert)
    } else if name.eq_ignore_ascii_case("revert-layer") {
        Some(CssGlobalKeyword::RevertLayer)
    } else {
        None
    }
}
```

- [ ] **Step 5: Parse global keywords before property-specific values**

Update the validation import in `src/lib.rs`:

```rust
use validation::{PropertyNameStatus, classify_property_name, parse_global_keyword};
```

In `StrictDeclarationParser::parse_value`, after `let location = ...`, add:

```rust
let state = input.state();
if let Ok(ident) = input.expect_ident_cloned() {
    if let Some(keyword) = parse_global_keyword(&ident) {
        if !input.is_exhausted() {
            return Err(invalid_syntax(
                input.current_source_location(),
                "CSS global keyword must be the entire declaration value",
            ));
        }
        match classify_property_name(name.as_ref()) {
            PropertyNameStatus::Supported => {
                return Ok(CssDeclaration::new(
                    property_for_supported_name(name.as_ref())
                        .expect("supported property has CssProperty"),
                    CssValue::GlobalKeyword(keyword),
                    location,
                ));
            }
            PropertyNameStatus::KnownUnsupported | PropertyNameStatus::Unknown => {
                return Err(property_name_error(input, name.as_ref()));
            }
        }
    }
    input.reset(&state);
} else {
    input.reset(&state);
}
```

Add this helper near `property_name_error`:

```rust
fn property_for_supported_name(name: &str) -> Option<CssProperty> {
    Some(match_ignore_ascii_case! { name,
        "display" => CssProperty::Display,
        "box-sizing" => CssProperty::BoxSizing,
        "position" => CssProperty::Position,
        "direction" => CssProperty::Direction,
        "overflow" => CssProperty::Overflow,
        "overflow-x" => CssProperty::OverflowX,
        "overflow-y" => CssProperty::OverflowY,
        "flex-direction" => CssProperty::FlexDirection,
        "flex-wrap" => CssProperty::FlexWrap,
        "align-items" => CssProperty::AlignItems,
        "align-self" => CssProperty::AlignSelf,
        "justify-items" => CssProperty::JustifyItems,
        "justify-self" => CssProperty::JustifySelf,
        "width" => CssProperty::Width,
        "height" => CssProperty::Height,
        "min-width" => CssProperty::MinWidth,
        "min-height" => CssProperty::MinHeight,
        "max-width" => CssProperty::MaxWidth,
        "max-height" => CssProperty::MaxHeight,
        "flex-basis" => CssProperty::FlexBasis,
        "gap" => CssProperty::Gap,
        "row-gap" => CssProperty::RowGap,
        "column-gap" => CssProperty::ColumnGap,
        "grid-flow-tolerance" => CssProperty::GridFlowTolerance,
        "font-size" => CssProperty::FontSize,
        "line-height" => CssProperty::LineHeight,
        "margin" => CssProperty::Margin,
        "padding" => CssProperty::Padding,
        "border-width" => CssProperty::BorderWidth,
        "color" => CssProperty::Color,
        "background" | "background-color" => CssProperty::Background,
        "border-color" => CssProperty::BorderColor,
        "opacity" => CssProperty::Opacity,
        "flex-grow" => CssProperty::FlexGrow,
        "flex-shrink" => CssProperty::FlexShrink,
        "aspect-ratio" => CssProperty::AspectRatio,
        "scrollbar-width" => CssProperty::ScrollbarWidth,
        _ => return None,
    })
}
```

- [ ] **Step 6: Run global keyword tests**

Run:

```sh
cargo test -p surgeist-css global_keyword
```

Expected: pass.

- [ ] **Step 7: Commit**

```sh
git add src/lib.rs src/syntax.rs src/validation.rs
git commit -m "Parse CSS global keywords"
```

---

## Task 3: Centralize CSS Length Unit Validation

**Files:**

- Modify: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Add failing tests for recognized unsupported units**

Add these tests:

```rust
#[test]
fn unsupported_length_units_report_the_unit_and_property() {
    let error = parse_sheet(".panel { width: 1rem; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("width".to_owned()),
            reason: "unsupported box size unit `rem`".to_owned(),
        }
    );
}

#[test]
fn unsupported_viewport_units_report_the_unit_and_property() {
    let error = parse_sheet(".panel { font-size: 2vh; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("font-size".to_owned()),
            reason: "unsupported font-size unit `vh`".to_owned(),
        }
    );
}

#[test]
fn unsupported_calc_units_still_report_the_unit_and_property() {
    let error = parse_sheet(".panel { width: calc(1rem + 2px); }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("width".to_owned()),
            reason: "unsupported calc length unit `rem`".to_owned(),
        }
    );
}

#[test]
fn unknown_dimension_units_are_reported_as_unknown_units() {
    let error = parse_sheet(".panel { width: 1quux; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("width".to_owned()),
            reason: "unknown box size unit `quux`".to_owned(),
        }
    );
}
```

- [ ] **Step 2: Run unit tests to verify failure**

Run:

```sh
cargo test -p surgeist-css unit
```

Expected: fail because unsupported units are not classified with property-domain reasons.

- [ ] **Step 3: Add unit classification helpers**

In `src/validation.rs`, add:

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LengthUnitStatus {
    SupportedPx,
    KnownUnsupported,
    Unknown,
}

const KNOWN_UNSUPPORTED_LENGTH_UNITS: &[&str] = &[
    "em", "rem", "ex", "rex", "cap", "rcap", "ch", "rch", "ic", "ric", "lh", "rlh",
    "vw", "vh", "vi", "vb", "vmin", "vmax",
    "svw", "svh", "svi", "svb", "svmin", "svmax",
    "lvw", "lvh", "lvi", "lvb", "lvmin", "lvmax",
    "dvw", "dvh", "dvi", "dvb", "dvmin", "dvmax",
    "cqw", "cqh", "cqi", "cqb", "cqmin", "cqmax",
    "cm", "mm", "q", "in", "pc", "pt",
];

pub(crate) fn classify_length_unit(unit: &str) -> LengthUnitStatus {
    if unit.eq_ignore_ascii_case("px") {
        LengthUnitStatus::SupportedPx
    } else if contains_ascii_case(KNOWN_UNSUPPORTED_LENGTH_UNITS, unit) {
        LengthUnitStatus::KnownUnsupported
    } else {
        LengthUnitStatus::Unknown
    }
}
```

- [ ] **Step 4: Use unit classification in length parsing**

Update the validation import:

```rust
use validation::{
    LengthUnitStatus, PropertyNameStatus, classify_length_unit, classify_property_name,
    parse_global_keyword,
};
```

Replace the `Token::Dimension` arm in `parse_length_with` with:

```rust
Token::Dimension { value, unit, .. } => match classify_length_unit(unit) {
    LengthUnitStatus::SupportedPx => Ok(CssLength::px(*value)),
    LengthUnitStatus::KnownUnsupported => Err(unsupported_value_at(
        location,
        None,
        format!("unsupported {context} unit `{unit}`"),
    )),
    LengthUnitStatus::Unknown => Err(unsupported_value_at(
        location,
        None,
        format!("unknown {context} unit `{unit}`"),
    )),
},
```

Replace the dimension arms in `parse_calc_component` with:

```rust
Token::Dimension { value, unit, .. } => match classify_length_unit(unit) {
    LengthUnitStatus::SupportedPx => Ok(CssCalcLength::px(*value)),
    LengthUnitStatus::KnownUnsupported => Err(unsupported_value_at(
        location,
        None,
        format!("unsupported calc length unit `{unit}`"),
    )),
    LengthUnitStatus::Unknown => Err(unsupported_value_at(
        location,
        None,
        format!("unknown calc length unit `{unit}`"),
    )),
},
```

- [ ] **Step 5: Run unit tests**

Run:

```sh
cargo test -p surgeist-css unit
```

Expected: pass.

- [ ] **Step 6: Commit**

```sh
git add src/lib.rs src/validation.rs
git commit -m "Classify CSS length units"
```

---

## Task 4: Centralize Property Keyword Validation

**Files:**

- Modify: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] **Step 1: Add failing keyword matrix tests**

Add these tests:

```rust
#[test]
fn unsupported_display_keyword_is_typed_with_property_context() {
    let error = parse_sheet(".panel { display: inline; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("display".to_owned()),
            reason: "unsupported display keyword `inline`".to_owned(),
        }
    );
}

#[test]
fn unsupported_overflow_keyword_is_typed_with_property_context() {
    let error = parse_sheet(".panel { overflow: auto; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("overflow".to_owned()),
            reason: "unsupported overflow keyword `auto`".to_owned(),
        }
    );
}

#[test]
fn unsupported_position_keyword_is_typed_with_property_context() {
    let error = parse_sheet(".panel { position: fixed; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("position".to_owned()),
            reason: "unsupported position keyword `fixed`".to_owned(),
        }
    );
}

#[test]
fn unsupported_alignment_keyword_is_typed_with_property_context() {
    let error = parse_sheet(".panel { align-items: unsafe center; }").unwrap_err();

    assert_eq!(
        error.kind(),
        &ErrorKind::UnsupportedValue {
            property: Some("align-items".to_owned()),
            reason: "unsupported alignment keyword `unsafe center`".to_owned(),
        }
    );
}
```

- [ ] **Step 2: Run keyword matrix tests to verify failure**

Run:

```sh
cargo test -p surgeist-css keyword
```

Expected: fail because current messages use domain prose inconsistently.

- [ ] **Step 3: Add keyword message helpers**

In `src/validation.rs`, add:

```rust
pub(crate) fn unsupported_keyword_reason(domain: &str, keyword: impl AsRef<str>) -> String {
    format!("unsupported {domain} keyword `{}`", keyword.as_ref())
}
```

- [ ] **Step 4: Update parser keyword errors**

Update these parser functions in `src/lib.rs` to use `unsupported_keyword_reason`:

```rust
parse_display        -> "display"
parse_box_sizing     -> "box-sizing"
parse_position       -> "position"
parse_direction      -> "direction"
parse_overflow       -> "overflow"
parse_flex_direction -> "flex-direction"
parse_flex_wrap      -> "flex-wrap"
parse_align_items    -> "alignment"
parse_grid_flow_tolerance -> "grid-flow-tolerance"
parse_color          -> "color"
```

For example, `parse_display` should become:

```rust
fn parse_display<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssDisplay, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    match_ignore_ascii_case! { &ident,
        "block" => Ok(CssDisplay::Block),
        "flex" => Ok(CssDisplay::Flex),
        "grid" => Ok(CssDisplay::Grid),
        "inline-block" => Ok(CssDisplay::InlineBlock),
        "inline-grid" => Ok(CssDisplay::InlineGrid),
        "grid-lanes" => Ok(CssDisplay::GridLanes),
        "inline-grid-lanes" => Ok(CssDisplay::InlineGridLanes),
        "none" => Ok(CssDisplay::None),
        _ => Err(unsupported_value(
            input,
            None,
            unsupported_keyword_reason("display", ident.as_ref()),
        )),
    }
}
```

For `parse_align_items`, build full multi-token reasons where needed:

```rust
let original = if has_overflow_prefix {
    format!("{first} {keyword}")
} else {
    keyword.clone()
};
```

Reject `unsafe ...` explicitly before the `match keyword.as_str()` block, because this crate currently supports only plain alignment keywords and the `safe` overflow-position subset:

```rust
if first == "unsafe" {
    return Err(unsupported_value(
        input,
        None,
        unsupported_keyword_reason("alignment", original),
    ));
}
```

Then use `unsupported_keyword_reason` for the remaining unsupported keyword paths:

```rust
unsupported_keyword_reason("alignment", original)
```

- [ ] **Step 5: Run keyword matrix tests**

Run:

```sh
cargo test -p surgeist-css keyword
```

Expected: pass.

- [ ] **Step 6: Commit**

```sh
git add src/lib.rs src/validation.rs
git commit -m "Normalize CSS keyword validation"
```

---

## Task 5: Final Integration Review And Checks

**Files:**

- Review: `src/lib.rs`
- Review: `src/syntax.rs`
- Review: `src/validation.rs`
- Review: `Cargo.toml`

- [ ] **Step 1: Run full crate checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Expected: all commands pass.

- [ ] **Step 2: Verify no style dependency returned**

Run:

```sh
rg -n "surgeist_style|surgeist-style|style::" src Cargo.toml
```

Expected: no matches.

- [ ] **Step 3: Review public API surface**

Run:

```sh
rg -n "pub enum CssGlobalKeyword|GlobalKeyword|UnknownProperty|CssValue" src/syntax.rs src/lib.rs
```

Expected:

- `CssGlobalKeyword` exists only in `src/syntax.rs`.
- `CssValue::GlobalKeyword` exists in `src/syntax.rs`.
- `ErrorKind::UnknownProperty` exists in `src/lib.rs`.
- No public validation registry leaks out of `src/validation.rs`.

- [ ] **Step 4: Final holistic review**

Assign a separate reviewer with this prompt:

```text
Review the complete surgeist-css validation implementation against
plans/2026-07-03-surgeist-css-wide-validation.md.

Check:
- The crate stays within /Users/codex/Development/surgeist-css.
- CSS-owned syntax remains authored/parser-facing.
- CssValue only gains a global keyword authored syntax variant, not a broad validation bag.
- Known unsupported properties and unknown properties are distinguished.
- CSS-wide global keywords parse for every supported property.
- Unsupported and unknown length units are classified with property context, including calc().
- Property keyword error reasons are normalized and covered by tests.
- No surgeist-style dependency/import was reintroduced.
- Required checks pass.

Return CLEAN or CHANGES_REQUESTED with file/line findings.
```

Expected: reviewer returns CLEAN. If the reviewer requests changes, send a scoped worker prompt for only those findings, then repeat this review step.

- [ ] **Step 5: Commit any final review fixes**

If review fixes were needed:

```sh
git add src/lib.rs src/syntax.rs src/validation.rs
git commit -m "Address CSS validation review"
```

If no fixes were needed, no additional commit is required.

---

## Completion Criteria

This plan is complete only when:

- All task-scoped worker/reviewer cycles required by `AGENTS.md` are clean.
- The final holistic reviewer returns CLEAN.
- These commands pass from `/Users/codex/Development/surgeist-css`:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- The final implementation is committed on `main` at logical task points.
- The coordinator reports any pushed status only if publication is requested or root needs to fetch the commit.

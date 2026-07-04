# CSS Naming And Rust Modeling Pass Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the naming issues and Rust modeling issues identified by the final surgeist-css audits, including intentional public breaking changes where those changes make authored CSS syntax harder to misuse.

**Architecture:** Keep `surgeist-css` as the owner of strict authored CSS syntax. Public syntax types should name CSS-authored concepts, preserve authored structure for downstream adapters, and prevent ordinary callers from constructing states the parser itself would reject. Parser modules should route through narrow property-specific parsers and a single property metadata registry instead of duplicated name tables.

**Tech Stack:** Rust, `cssparser`, crate-local unit tests, `cargo fmt`, `cargo test -p surgeist-css`, `cargo clippy -p surgeist-css --all-targets -- -D warnings`.

---

## Source Requirements

- Follow `AGENTS.md`.
- Use `guidance/surgeist-rust-modeling-guide.md` as the review standard.
- Coordinator does not write implementation code.
- Workers do not commit.
- Commit each clean worker/reviewer cycle as a traceable logical point.
- No branches.
- Work only in `/Users/codex/Development/surgeist-css`.
- Preserve strict invalid-CSS rejection. Do not add browser-style recovery.
- Breaking public API changes are allowed when they improve authored syntax modeling.

## Audit Items Covered

- Naming audit:
  - Internal `LengthOptions` and `AlignmentOptions` are broad names for grammar allowance sets.
  - `parse_transition_item` and `parse_animation_item` use generic “item” terminology.
  - `background-color` currently parses as `CssProperty::Background`.
  - `CssFunctionArguments` is an authored token string, not structured arguments.
  - `CssValue::Number(f32)` is a cross-property bag.
- Ramanujan modeling audit:
  - Replace `CssValue::Number(f32)` with property-specific models.
  - Tighten public numeric constructors/variants that allow parser-invalid syntax.
  - Make `CssColor` impossible to construct with invalid channels.
  - Replace validated function argument strings with typed authored argument models where needed.
  - Replace private `LengthOptions` flag bag with named grammar allowance types.
  - Unify duplicated property identity sources of truth.

## Target File Responsibilities

- `src/lib.rs`: public front door only.
- `src/syntax.rs`: public authored CSS syntax models with private fields and invariant-preserving constructors/accessors.
- `src/error.rs`: typed parser errors.
- `src/validation.rs`: property-name classification and shared property metadata hooks.
- `src/parser/mod.rs`: stylesheet/declaration front door and property dispatch using shared metadata.
- `src/parser/values.rs`: primitive parser helpers and typed length grammar allowance models.
- `src/parser/layout.rs`: layout/alignment/flex/z-index parsing.
- `src/parser/background.rs`: background/cursor/outline parsing.
- `src/parser/effects.rs`: transform/filter/clip/mask parsing with typed authored function arguments.
- `src/parser/timing.rs`: transition/animation parsing.
- `src/tests.rs`: parser and public model tests.
- `src/test_support.rs`: reusable test matrices and property coverage helpers.

## Task 1: Internal Naming And Background Property Identity

**Files:**
- Modify: `src/parser/values.rs`
- Modify: `src/parser/layout.rs`
- Modify: `src/parser/timing.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/syntax.rs`
- Modify: `src/tests.rs`
- Modify: `src/test_support.rs`

- [ ] Add a focused failing test proving `background-color` preserves authored property identity:

```rust
#[test]
fn background_color_preserves_authored_property_identity() {
    let declaration = single_declaration(".panel { background-color: black; }");
    assert_eq!(declaration.property(), CssProperty::BackgroundColor);
    assert_eq!(declaration.value(), &CssValue::Color(CssColor::BLACK));
}
```

- [ ] Add `CssProperty::BackgroundColor` next to `CssProperty::Background`.

- [ ] Update `parse_declaration` so `"background"` maps to `CssProperty::Background` and `"background-color"` maps to `CssProperty::BackgroundColor`.

- [ ] Update `property_for_supported_name` and test support matrices so `background-color` expects `CssProperty::BackgroundColor`.

- [ ] Rename internal parser helpers only:
  - `LengthOptions` to `AllowedLengthSyntax`.
  - `AlignmentOptions` to `AllowedAlignmentKeywords`.
  - `parse_transition_item` to `parse_single_transition`.
  - `parse_animation_item` to `parse_single_animation`.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css background_color_preserves_authored_property_identity
cargo test -p surgeist-css parses_all_property_global_keywords_as_authored_syntax
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 2: Replace `CssValue::Number` With Property-Specific Authored Models

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/parser/layout.rs`
- Modify: `src/tests.rs`
- Modify: `src/test_support.rs`

- [ ] Add public modeled value types:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssOpacity {
    value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssFlexFactor {
    value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssAspectRatio {
    value: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssScrollbarWidth {
    Auto,
    Thin,
    None,
}
```

- [ ] Give each numeric wrapper `try_new(value: f32) -> Option<Self>`, a parser-private unchecked constructor only if the parser has already validated the value, and `value(self) -> f32`.

- [ ] Enforce:
  - `CssOpacity`: finite `0.0..=1.0`.
  - `CssFlexFactor`: finite `>= 0.0`.
  - `CssAspectRatio`: finite `> 0.0`.
  - `CssScrollbarWidth`: authored keywords only: `auto`, `thin`, `none`.

- [ ] Replace `CssValue::Number(f32)` with:

```rust
Opacity(CssOpacity),
FlexGrow(CssFlexFactor),
FlexShrink(CssFlexFactor),
AspectRatio(CssAspectRatio),
ScrollbarWidth(CssScrollbarWidth),
```

- [ ] Update parsers:
  - `opacity` rejects non-finite, negative, and values above `1`.
  - `flex-grow` and `flex-shrink` keep rejecting negatives and reject non-finite values.
  - `aspect-ratio` rejects zero, negatives, and non-finite values.
  - `scrollbar-width` rejects numbers and accepts only `auto`, `thin`, `none`.
  - `CssFlex::Components` should use `CssFlexFactor` for `grow` and `shrink`.

- [ ] Add tests for accepted and rejected cases:

```rust
#[test]
fn numeric_properties_use_property_specific_authored_models() {
    assert_eq!(
        single_declaration(".panel { opacity: 0.5; }").value(),
        &CssValue::Opacity(CssOpacity::try_new(0.5).unwrap())
    );
    assert_eq!(
        single_declaration(".panel { flex-grow: 2; }").value(),
        &CssValue::FlexGrow(CssFlexFactor::try_new(2.0).unwrap())
    );
    assert_eq!(
        single_declaration(".panel { flex-shrink: 0; }").value(),
        &CssValue::FlexShrink(CssFlexFactor::try_new(0.0).unwrap())
    );
    assert_eq!(
        single_declaration(".panel { aspect-ratio: 1.5; }").value(),
        &CssValue::AspectRatio(CssAspectRatio::try_new(1.5).unwrap())
    );
    assert_eq!(
        single_declaration(".panel { scrollbar-width: thin; }").value(),
        &CssValue::ScrollbarWidth(CssScrollbarWidth::Thin)
    );
}
```

```rust
#[test]
fn numeric_property_models_reject_invalid_authored_values() {
    assert_parse_error(".panel { opacity: -0.1; }", ErrorKind::UnsupportedValue { .. });
    assert_parse_error(".panel { opacity: 2; }", ErrorKind::UnsupportedValue { .. });
    assert_parse_error(".panel { flex-grow: -1; }", ErrorKind::UnsupportedValue { .. });
    assert_parse_error(".panel { aspect-ratio: 0; }", ErrorKind::UnsupportedValue { .. });
    assert_parse_error(".panel { scrollbar-width: 8; }", ErrorKind::UnsupportedValue { .. });
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css numeric_properties
cargo test -p surgeist-css rejection_property_specific_matrix_rejects_every_supported_property
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 3: Tighten Public Numeric And Color Construction

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/parser/values.rs`
- Modify: `src/parser/grid.rs`
- Modify: `src/parser/effects.rs`
- Modify: `src/tests.rs`

- [ ] Introduce a small public finite-number wrapper if repeated validation would otherwise stay scattered:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssFiniteNumber {
    value: f32,
}
```

- [ ] Use `try_new` constructors for public numeric wrappers that must reject NaN/infinite values and domain-invalid values.

- [ ] Review and tighten the public construction surface for:
  - `CssLengthDimension`.
  - `CssLength::px`, `CssLength::percent`, and `CssLength::dimension`.
  - `CssGridTrackBreadth::Fraction`.
  - `CssScaleValues`.
  - `CssFlex`.
  - `CssCalcLength` and `CssCalcLengthTerm` numeric constructors.

- [ ] Preserve parser behavior by using parser-private unchecked constructors only after parser validation.

- [ ] Make `CssColor` fields private and add:

```rust
pub const fn r(self) -> f32;
pub const fn g(self) -> f32;
pub const fn b(self) -> f32;
pub const fn a(self) -> f32;
pub fn try_rgba(r: f32, g: f32, b: f32, a: f32) -> Option<Self>;
pub(crate) const fn rgba_unchecked(r: f32, g: f32, b: f32, a: f32) -> Self;
```

- [ ] Keep `CssColor::BLACK`, `CssColor::WHITE`, and `CssColor::TRANSPARENT`.

- [ ] Add constructor invariant tests for each tightened public type.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css constructor
cargo test -p surgeist-css checked
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 4: Model Authored Function Arguments Beyond Raw Strings

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/parser/effects.rs`
- Modify: `src/parser/timing.rs`
- Modify: `src/tests.rs`

- [ ] Rename `CssFunctionArguments` to a name that states it is authored tokens:

```rust
pub struct CssAuthoredFunctionArguments {
    css: String,
}
```

- [ ] Keep a compatibility-free breaking change: update all transform/filter/basic-shape/easing function structs/enums to use `CssAuthoredFunctionArguments`.

- [ ] Add narrow wrappers where function family context matters:
  - `CssTransformArguments`.
  - `CssFilterArguments`.
  - `CssBasicShapeArguments`.
  - `CssEasingArguments`.

- [ ] Each wrapper should contain `CssAuthoredFunctionArguments` and expose `as_css(&self) -> &str`.

- [ ] Parser validation stays in parser modules, but syntax types should no longer imply that arguments are semantically structured when they are authored token text.

- [ ] Add tests proving transform/filter/easing/basic-shape values preserve authored strings through typed wrappers without requiring downstream string parsing to distinguish the function family.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css transform
cargo test -p surgeist-css filter
cargo test -p surgeist-css transition
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 5: Replace Length Grammar Flag Bag With Named Allowance Models

**Files:**
- Modify: `src/parser/values.rs`
- Modify: parser modules that import length grammar helpers.
- Modify: `src/tests.rs` only if tests need new coverage for grammar families.

- [ ] Replace boolean `AllowedLengthSyntax` fields with named grammar allowance types or enum variants that cannot express accidental combinations.

- [ ] Preferred shape:

```rust
#[derive(Clone, Copy)]
pub(super) enum LengthGrammar {
    BoxSize,
    Inset,
    Margin,
    Padding,
    BorderWidth,
    Radius,
    ShadowOffset,
    ShadowBlur,
    Gap,
    FontSize,
    LineHeight,
    TextIndent,
    VerticalAlign,
    LetterSpacing,
    TextDecorationThickness,
    GridTrack,
    BackgroundSize,
}
```

- [ ] Move grammar capability decisions behind methods such as:

```rust
impl LengthGrammar {
    const fn allows_percent(self) -> bool;
    const fn allows_auto(self) -> bool;
    const fn allows_intrinsic(self) -> bool;
    const fn allows_normal(self) -> bool;
    const fn allows_calc_percent(self) -> bool;
    const fn requires_non_negative(self) -> bool;
    const fn context(self) -> &'static str;
}
```

- [ ] Update property-specific parser helpers to pass `LengthGrammar` variants, not constructed flag structs.

- [ ] Keep the existing strict property-specific behavior exactly.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css unit_matrix
cargo test -p surgeist-css rejection_property_specific_matrix_rejects_every_supported_property
cargo test -p surgeist-css no_recovery
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 6: Unify Property Metadata And Dispatch Registry

**Files:**
- Modify: `src/validation.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/test_support.rs`
- Modify: `src/tests.rs`

- [ ] Create one shared property metadata table that owns, at minimum:

```rust
pub(crate) struct SupportedProperty {
    pub(crate) name: &'static str,
    pub(crate) property: CssProperty,
}
```

- [ ] Use the shared table for:
  - property-name classification in `validation.rs`.
  - `property_for_supported_name`.
  - coverage/test-support supported property expectations.

- [ ] Keep parser routing explicit if function pointers make lifetimes or parser errors noisy; do not force dispatch into the table if it damages readability.

- [ ] Ensure the table includes `BackgroundColor` and every supported property name exactly once.

- [ ] Preserve typed unknown-property and unsupported-value error behavior.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css coverage
cargo test -p surgeist-css property
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 7: Final Modeling Review, Documentation Sweep, And Full Checks

**Files:**
- Inspect all files changed by Tasks 1-6.
- Modify `README.md` only if public breaking model changes need a short user-facing note.

- [ ] Run:

```sh
git status --short --branch
git diff --stat
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Assign a clean-context holistic reviewer with no conversation history. Reviewer must inspect the final diff against:
  - `AGENTS.md`.
  - `guidance/surgeist-rust-modeling-guide.md`.
  - This plan.
  - Public API/modeling changes.
  - Strict no-recovery behavior.

- [ ] Completion requires the holistic reviewer to return `APPROVED` with no unresolved findings.

## Deferred Explicitly Nowhere

This pass intentionally does not defer any Ramanujan audit item. If a worker finds an item is too broad or technically blocked, they must report the exact blocker and reproduction instead of silently narrowing scope.

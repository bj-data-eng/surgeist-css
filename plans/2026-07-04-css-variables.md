# CSS Variables Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add strict authored CSS support for custom properties and `var(...)` references without performing cascade-time substitution inside `surgeist-css`.

**Architecture:** `surgeist-css` owns parsed authored syntax only. Custom property declarations and `var(...)` references stay symbolic, preserving authored token text and structured variable references for a later resolver at the style/root adapter boundary. Typed models should make malformed custom property names and malformed `var(...)` calls impossible to construct through public APIs.

**Tech Stack:** Rust, `cssparser`, crate-local parser tests, `cargo fmt`, `cargo test -p surgeist-css`, `cargo clippy -p surgeist-css --all-targets -- -D warnings`.

---

## Source Requirements

- Follow `AGENTS.md`.
- Use `guidance/surgeist-rust-modeling-guide.md`.
- Coordinator does not write implementation code.
- Workers do not commit.
- Commit each clean worker/reviewer cycle as a logical point.
- No branches.
- Work only in `/Users/codex/Development/surgeist-css`.
- Preserve strict invalid-CSS rejection. Do not add browser-style recovery.
- Breaking public API changes are allowed when they improve authored CSS modeling.
- External/root tests must exercise public integration behavior only; do not keep accidental APIs solely for compatibility.

## Scope

Implement:

- Custom property declarations such as `--gap: 8px;`.
- Case-sensitive custom property names.
- Empty custom property values if `cssparser` accepts the declaration.
- CSS-wide global keywords on custom properties as whole values.
- Structurally parsed `var(...)` references:
  - `var(--name)`
  - `var(--name, fallback tokens)`
  - nested `var(...)` inside fallback tokens
- Supported standard properties whose value contains at least one valid `var(...)`, represented as unresolved authored syntax rather than eagerly property-validated syntax.
- `:root` selector support for common CSS variable declarations.

Do not implement:

- Cascade, inheritance, custom property substitution, fallback resolution, or cycle detection.
- Post-substitution property validation.
- Browser recovery for malformed custom properties or malformed `var(...)`.
- General pseudo-class support beyond `:root`.
- `env(...)` or other custom functions.

## Target Modeling

Preferred public syntax additions:

```rust
pub enum CssProperty {
    // existing standard variants...
    Custom(CssCustomPropertyName),
}

pub struct CssCustomPropertyName {
    name: String,
}

pub struct CssAuthoredDeclarationValue {
    css: String,
}

pub struct CssVariableReference {
    name: CssCustomPropertyName,
    fallback: Option<CssVariableFallback>,
}

pub struct CssVariableFallback {
    authored: CssAuthoredDeclarationValue,
    references: Vec<CssVariableReference>,
}

pub struct CssCustomPropertyValue {
    authored: CssAuthoredDeclarationValue,
    references: Vec<CssVariableReference>,
}

pub struct CssVariableDependentValue {
    authored: CssAuthoredDeclarationValue,
    references: Vec<CssVariableReference>,
}
```

Preferred `CssValue` additions:

```rust
CustomProperty(CssCustomPropertyValue),
VariableDependent(CssVariableDependentValue),
```

Important modeling rules:

- `CssCustomPropertyName` is case-sensitive and preserves authored spelling.
- `CssCustomPropertyName::try_new` rejects names that are not CSS custom property names.
- `CssVariableReference` exposes `name()` and `fallback()`.
- `CssVariableFallback`, `CssCustomPropertyValue`, and `CssVariableDependentValue` expose `as_css()` and `references()`.
- `CssVariableDependentValue` is allowed only for supported standard properties whose authored value contains `var(...)`.
- A declaration with no `var(...)` must still go through existing strict property-specific parsers.
- A declaration with malformed `var(...)` must reject the whole sheet.

## Task 1: Model Custom Properties And Variable References

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/tests.rs`

- [ ] Add typed public models listed in Target Modeling.

- [ ] Change `CssProperty` so it can carry `Custom(CssCustomPropertyName)`. Remove `Copy` from `CssProperty` if needed and update accessors/tests to borrow or clone deliberately.

- [ ] Add checked constructors and accessors:

```rust
impl CssCustomPropertyName {
    pub fn try_new(name: impl Into<String>) -> Option<Self>;
    pub(crate) fn new(name: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}

impl CssAuthoredDeclarationValue {
    pub fn try_new(css: impl Into<String>) -> Option<Self>;
    pub(crate) fn new(css: impl Into<String>) -> Self;
    pub fn as_css(&self) -> &str;
}
```

- [ ] Add constructor invariant tests:

```rust
#[test]
fn custom_property_name_constructor_preserves_case_and_rejects_non_custom_names() {
    let name = CssCustomPropertyName::try_new("--BrandColor").unwrap();
    assert_eq!(name.as_str(), "--BrandColor");
    assert_eq!(CssCustomPropertyName::try_new("color"), None);
    assert_eq!(CssCustomPropertyName::try_new("-gap"), None);
    assert_eq!(CssCustomPropertyName::try_new("--"), None);
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css custom_property_name_constructor
cargo test -p surgeist-css constructor
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 2: Parse Custom Property Declarations

**Files:**
- Create: `src/parser/variables.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/syntax.rs` only if Task 1 models need small parser-facing accessors.
- Modify: `src/tests.rs`

- [ ] Add parser helpers in `src/parser/variables.rs` for custom property names and authored declaration token collection.

- [ ] Parse declarations whose property name is a custom property name before ordinary supported-property dispatch.

- [ ] `--name: inherit`, `--name: initial`, `--name: unset`, `--name: revert`, and `--name: revert-layer` should parse as CSS-wide global keywords for `CssProperty::Custom(name)` when the keyword is the whole value.

- [ ] Other custom property values should parse as `CssValue::CustomProperty(CssCustomPropertyValue)` and preserve authored CSS text.

- [ ] Add tests:

```rust
#[test]
fn parses_custom_property_declarations_as_authored_syntax() {
    let declaration = single_declaration(".theme { --BrandColor: #fff; }");
    assert_eq!(
        declaration.property(),
        &CssProperty::Custom(CssCustomPropertyName::try_new("--BrandColor").unwrap())
    );
    let CssValue::CustomProperty(value) = declaration.value() else {
        panic!("expected custom property value");
    };
    assert_eq!(value.as_css(), "#fff");
    assert!(value.references().is_empty());
}

#[test]
fn custom_property_global_keyword_must_be_whole_value() {
    assert_eq!(
        single_declaration(".theme { --gap: inherit; }").value(),
        &CssValue::GlobalKeyword(CssGlobalKeyword::Inherit)
    );
    assert!(parse_sheet(".theme { --gap: inherit 1px; }").is_err());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css custom_property
cargo test -p surgeist-css global_keyword
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 3: Parse `var(...)` Structurally

**Files:**
- Modify: `src/parser/variables.rs`
- Modify: `src/syntax.rs` only if small accessors are needed.
- Modify: `src/tests.rs`

- [ ] Add a structural parser for `var(...)`.

- [ ] Accept:
  - `var(--gap)`
  - `var(--gap, 8px)`
  - `var(--gap, calc(1px + var(--fallback)))`
  - fallback with empty authored token text if `cssparser` accepts `var(--x,)`

- [ ] Reject:
  - `var()`
  - `var(color)`
  - `var(--gap --other)`
  - `var(--gap,`
  - `var(--gap) trailing` only in contexts that require a single variable reference

- [ ] Expose parsed fallback references through `CssVariableFallback::references()`.

- [ ] Add tests:

```rust
#[test]
fn parses_variable_references_with_nested_fallbacks() {
    let declaration = single_declaration(".theme { --gap: var(--space, calc(1px + var(--fallback))); }");
    let CssValue::CustomProperty(value) = declaration.value() else {
        panic!("expected custom property value");
    };
    assert_eq!(value.references()[0].name().as_str(), "--space");
    let fallback = value.references()[0].fallback().unwrap();
    assert_eq!(fallback.as_css(), "calc(1px + var(--fallback))");
    assert_eq!(fallback.references()[0].name().as_str(), "--fallback");
}

#[test]
fn rejects_malformed_variable_references() {
    assert!(parse_sheet(".theme { --gap: var(); }").is_err());
    assert!(parse_sheet(".theme { --gap: var(color); }").is_err());
    assert!(parse_sheet(".theme { --gap: var(--gap --other); }").is_err());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css variable_references
cargo test -p surgeist-css malformed_variable
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 4: Accept `var(...)` In Supported Standard Properties As Symbolic Values

**Files:**
- Modify: `src/parser/mod.rs`
- Modify: `src/parser/variables.rs`
- Modify: `src/syntax.rs` only if accessors are needed.
- Modify: `src/tests.rs`
- Modify: `src/test_support.rs` only if helper signatures change after `CssProperty` becomes non-`Copy`.

- [ ] Before ordinary property-specific parsing, detect whether the authored declaration value for a supported standard property contains one or more valid `var(...)` references.

- [ ] If it does, return:

```rust
CssDeclaration::new(
    supported_property,
    CssValue::VariableDependent(CssVariableDependentValue::new(authored, references)),
    location,
)
```

- [ ] Do not property-validate variable-dependent values in `surgeist-css`; substitution and post-substitution validation belong to a later resolver.

- [ ] If the value has no `var(...)`, keep existing strict property-specific parsing exactly.

- [ ] If any `var(...)` is malformed, reject the declaration/sheet.

- [ ] Add tests:

```rust
#[test]
fn supported_properties_accept_variable_dependent_values_symbolically() {
    let declaration = single_declaration(".panel { gap: var(--space, 8px); }");
    assert_eq!(declaration.property(), &CssProperty::Gap);
    let CssValue::VariableDependent(value) = declaration.value() else {
        panic!("expected variable dependent value");
    };
    assert_eq!(value.as_css(), "var(--space, 8px)");
    assert_eq!(value.references()[0].name().as_str(), "--space");
}

#[test]
fn malformed_var_in_supported_property_rejects_whole_sheet() {
    assert!(parse_sheet(".panel { gap: var(color); }").is_err());
    assert!(parse_sheet(".panel { color: var(--brand); bogus: 1; }").is_err());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css variable_dependent
cargo test -p surgeist-css malformed_var
cargo test -p surgeist-css no_recovery
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 5: Add `:root` Selector Support For Variable Declarations

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/parser/selectors.rs`
- Modify: `src/tests.rs`

- [ ] Add a typed selector model for `:root`, such as:

```rust
pub enum CssPseudoClass {
    Root,
}
```

- [ ] Allow `:root` as a simple selector and inside compound selectors only if that fits the existing selector model cleanly.

- [ ] Do not add broad pseudo-class support.

- [ ] Add tests:

```rust
#[test]
fn parses_root_selector_for_custom_property_declarations() {
    let sheet = parse_sheet(":root { --space: 8px; }").unwrap();
    assert_eq!(sheet.rules().len(), 1);
    assert_eq!(sheet.rules()[0].declarations().len(), 1);
}

#[test]
fn rejects_unsupported_pseudo_classes() {
    assert!(parse_sheet(":hover { --space: 8px; }").is_err());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css root_selector
cargo test -p surgeist-css selector
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 6: Final Variable Matrix, Documentation, And Holistic Review

**Files:**
- Modify: `src/tests.rs`
- Modify: `README.md` only if a short public behavior note is useful.
- Inspect all changed files.

- [ ] Add an acceptance/rejection matrix covering:
  - custom property name case sensitivity
  - custom property value preservation
  - global keyword whole-value behavior
  - `var(...)` with fallback
  - nested `var(...)`
  - supported property with embedded `var(...)`
  - malformed `var(...)`
  - unsupported non-root pseudo-class rejection

- [ ] Run:

```sh
git status --short --branch
git diff --stat
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Assign a clean-context holistic reviewer with no conversation history. Reviewer must inspect the final implementation against:
  - `AGENTS.md`
  - `guidance/surgeist-rust-modeling-guide.md`
  - this plan
  - public authored syntax API
  - strict no-recovery behavior

- [ ] Completion requires the holistic reviewer to return `APPROVED` with no unresolved findings.

## Completion Signal

Report:

- commit SHAs
- final test count
- checks run
- final holistic reviewer result
- whether the repo is pushed

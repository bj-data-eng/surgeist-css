# Practical CSS Pseudo-Classes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add strict authored syntax support for practical CSS pseudo-classes that remove manual Rust UI-state styling logic from application code.

**Architecture:** `surgeist-css` parses and owns authored selector syntax only; it does not evaluate pseudo-classes against runtime state. Pseudo-classes are modeled as typed authored syntax so root/style/retained layers can later match hover, focus, form state, structure, and overlay state through explicit runtime selector matching. Functional pseudo-classes carry typed selector-list or nth-pattern arguments instead of opaque strings.

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
- Keep pseudo-class parsing crate-owned and authored-syntax-facing.
- Do not add pseudo-class runtime matching or state evaluation in this crate.
- Do not keep accidental public APIs for external tests. External/root tests must use public integration behavior.

## Scope

Implement authored syntax support for these practical pseudo-classes:

Tier 1 interaction and control-state pseudo-classes:

- `:root` already exists and must remain supported.
- `:hover`
- `:active`
- `:focus`
- `:focus-visible`
- `:focus-within`
- `:disabled`
- `:enabled`
- `:checked`
- `:required`
- `:optional`
- `:valid`
- `:invalid`
- `:placeholder-shown`

Tier 2 structural pseudo-classes:

- `:first-child`
- `:last-child`
- `:only-child`
- `:empty`
- `:nth-child(...)`
- `:nth-last-child(...)`
- `:first-of-type`
- `:last-of-type`
- `:only-of-type`
- `:nth-of-type(...)`
- `:nth-last-of-type(...)`

Tier 3 selector-list functional pseudo-classes:

- `:not(...)`
- `:is(...)`
- `:where(...)`
- `:has(...)`

Tier 4 runtime-state pseudo-classes:

- `:modal`
- `:fullscreen`
- `:popover-open`
- `:default`
- `:indeterminate`
- `:read-only`
- `:read-write`
- `:in-range`
- `:out-of-range`

Do not implement in this pass:

- Link/navigation/document pseudo-classes: `:link`, `:visited`, `:any-link`, `:target`, `:target-within`.
- Language/direction pseudo-classes: `:lang(...)`, `:dir(...)`.
- Browser/custom-element pseudo-classes: `:defined`, `:state(...)`.
- Time-dimensional pseudo-classes: `:current`, `:past`, `:future`, `:playing`, `:paused`.
- Shadow DOM pseudo-classes: `:host`, `:host(...)`, `:host-context(...)`.
- Selector combinators, descendant selectors, sibling selectors, or relative selector syntax outside what current selector parsing already supports.
- Runtime pseudo-class matching or style invalidation.

## Modeling Rules

- Pseudo-classes are authored selector syntax, not resolved state.
- Simple pseudo-classes must be represented as explicit enum variants, not strings.
- Functional pseudo-classes must carry typed arguments:
  - `:not(...)`, `:is(...)`, and `:where(...)` carry a `CssSelectorList`.
  - `:has(...)` carries a `CssSelectorList` using the currently supported selector grammar. Relative selectors with combinators stay unsupported and must reject.
  - `:nth-*` carries a checked nth pattern model.
- Unsupported valid CSS remains rejected until deliberately modeled.
- Malformed pseudo-class syntax rejects the whole sheet.
- Duplicate pseudo-classes such as `:hover:hover` may be preserved as authored syntax. Do not canonicalize or deduplicate in `surgeist-css`.

## Target Model

Preferred public syntax additions in `src/syntax.rs`:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct CssSelectorList {
    selectors: Vec<CssSelector>,
}

impl CssSelectorList {
    pub fn try_new(selectors: Vec<CssSelector>) -> Option<Self>;
    pub(crate) fn new(selectors: Vec<CssSelector>) -> Self;
    pub fn selectors(&self) -> &[CssSelector];
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
    pub const fn new(a: i32, b: i32) -> Self;
    pub const fn a(self) -> i32;
    pub const fn b(self) -> i32;
}
```

## Task 1: Model Selector Lists, Simple Pseudo-Classes, And Nth Patterns

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/parser/selectors.rs`
- Modify: `src/tests.rs`

- [ ] Add `CssSelectorList` with private `selectors: Vec<CssSelector>`.

```rust
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
```

- [ ] Replace the current `CssPseudoClass` enum with the full target enum from this plan.

- [ ] Update `src/parser/selectors.rs` so the existing single-pseudo-class fast path does not require `CssPseudoClass: Copy` after functional variants are added.

```rust
    if let (None, None, [], [pseudo_class]) = (
        tag_name.as_ref(),
        key_name.as_ref(),
        class_names.as_slice(),
        pseudo_classes.as_slice(),
    ) {
        return Ok(CssSelector::PseudoClass(pseudo_class.clone()));
    }
```

- [ ] Add checked nth pattern models.

```rust
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
```

- [ ] Add model tests.

```rust
#[test]
fn selector_list_constructor_rejects_empty_lists() {
    assert_eq!(CssSelectorList::try_new(Vec::new()), None);
    let list = CssSelectorList::try_new(vec![CssSelector::Class("button".to_owned())]).unwrap();
    assert_eq!(list.selectors(), &[CssSelector::Class("button".to_owned())]);
}

#[test]
fn nth_pattern_model_exposes_an_plus_b_coefficients() {
    let pattern = CssNthPattern::AnPlusB(CssNthAnPlusB::new(2, 1));
    let CssNthPattern::AnPlusB(value) = pattern else {
        panic!("expected an+b pattern");
    };
    assert_eq!(value.a(), 2);
    assert_eq!(value.b(), 1);
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css selector_list_constructor
cargo test -p surgeist-css nth_pattern_model
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 2: Parse Tier 1 Interaction And Control-State Pseudo-Classes

**Files:**
- Modify: `src/parser/selectors.rs`
- Modify: `src/tests.rs`

- [ ] Extend `parse_pseudo_class` to map these identifier pseudo-classes to explicit `CssPseudoClass` variants:

```rust
match_ignore_ascii_case! { &name,
    "root" => Ok(CssPseudoClass::Root),
    "hover" => Ok(CssPseudoClass::Hover),
    "active" => Ok(CssPseudoClass::Active),
    "focus" => Ok(CssPseudoClass::Focus),
    "focus-visible" => Ok(CssPseudoClass::FocusVisible),
    "focus-within" => Ok(CssPseudoClass::FocusWithin),
    "disabled" => Ok(CssPseudoClass::Disabled),
    "enabled" => Ok(CssPseudoClass::Enabled),
    "checked" => Ok(CssPseudoClass::Checked),
    "required" => Ok(CssPseudoClass::Required),
    "optional" => Ok(CssPseudoClass::Optional),
    "valid" => Ok(CssPseudoClass::Valid),
    "invalid" => Ok(CssPseudoClass::Invalid),
    "placeholder-shown" => Ok(CssPseudoClass::PlaceholderShown),
    _ => Err(invalid_selector(input, format!("unsupported pseudo-class `:{name}`"))),
}
```

- [ ] Keep function syntax for these names rejected. `:hover()` and `:focus()` must reject.

- [ ] Add tests for simple and compound Tier 1 selectors.

```rust
#[test]
fn parses_tier_1_state_pseudo_classes_as_authored_selectors() {
    let cases = [
        (":hover { color: red; }", CssPseudoClass::Hover),
        (":active { color: red; }", CssPseudoClass::Active),
        (":focus { color: red; }", CssPseudoClass::Focus),
        (":focus-visible { color: red; }", CssPseudoClass::FocusVisible),
        (":focus-within { color: red; }", CssPseudoClass::FocusWithin),
        (":disabled { color: red; }", CssPseudoClass::Disabled),
        (":enabled { color: red; }", CssPseudoClass::Enabled),
        (":checked { color: red; }", CssPseudoClass::Checked),
        (":required { color: red; }", CssPseudoClass::Required),
        (":optional { color: red; }", CssPseudoClass::Optional),
        (":valid { color: red; }", CssPseudoClass::Valid),
        (":invalid { color: red; }", CssPseudoClass::Invalid),
        (":placeholder-shown { color: red; }", CssPseudoClass::PlaceholderShown),
    ];

    for (css, expected) in cases {
        let sheet = parse_sheet(css).unwrap();
        assert_eq!(sheet.rules()[0].selector(), &CssSelector::PseudoClass(expected));
    }
}

#[test]
fn parses_compound_tier_1_state_pseudo_classes() {
    let sheet = parse_sheet(".button:hover { color: red; }").unwrap();
    let CssSelector::Compound(selector) = sheet.rules()[0].selector() else {
        panic!("expected compound selector");
    };
    assert_eq!(selector.classes(), &["button".to_owned()]);
    assert_eq!(selector.pseudo_classes(), &[CssPseudoClass::Hover]);
}

#[test]
fn rejects_function_syntax_for_simple_state_pseudo_classes() {
    assert!(parse_sheet(":hover() { color: red; }").is_err());
    assert!(parse_sheet(":focus() { color: red; }").is_err());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css tier_1_state_pseudo
cargo test -p surgeist-css compound_tier_1
cargo test -p surgeist-css simple_state_pseudo
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 3: Parse Tier 2 Non-Functional Structural Pseudo-Classes

**Files:**
- Modify: `src/parser/selectors.rs`
- Modify: `src/tests.rs`

- [ ] Extend identifier pseudo-class parsing for:

```rust
"first-child" => Ok(CssPseudoClass::FirstChild),
"last-child" => Ok(CssPseudoClass::LastChild),
"only-child" => Ok(CssPseudoClass::OnlyChild),
"empty" => Ok(CssPseudoClass::Empty),
"first-of-type" => Ok(CssPseudoClass::FirstOfType),
"last-of-type" => Ok(CssPseudoClass::LastOfType),
"only-of-type" => Ok(CssPseudoClass::OnlyOfType),
```

- [ ] Keep function syntax rejected for these non-functional names. `:first-child()` must reject.

- [ ] Add tests.

```rust
#[test]
fn parses_tier_2_structural_simple_pseudo_classes() {
    let cases = [
        (":first-child { color: red; }", CssPseudoClass::FirstChild),
        (":last-child { color: red; }", CssPseudoClass::LastChild),
        (":only-child { color: red; }", CssPseudoClass::OnlyChild),
        (":empty { color: red; }", CssPseudoClass::Empty),
        (":first-of-type { color: red; }", CssPseudoClass::FirstOfType),
        (":last-of-type { color: red; }", CssPseudoClass::LastOfType),
        (":only-of-type { color: red; }", CssPseudoClass::OnlyOfType),
    ];

    for (css, expected) in cases {
        let sheet = parse_sheet(css).unwrap();
        assert_eq!(sheet.rules()[0].selector(), &CssSelector::PseudoClass(expected));
    }
}

#[test]
fn parses_compound_structural_simple_pseudo_classes() {
    let sheet = parse_sheet("button:first-child { color: red; }").unwrap();
    let CssSelector::Compound(selector) = sheet.rules()[0].selector() else {
        panic!("expected compound selector");
    };
    assert_eq!(selector.tag().map(String::as_str), Some("button"));
    assert_eq!(selector.pseudo_classes(), &[CssPseudoClass::FirstChild]);
}

#[test]
fn rejects_function_syntax_for_non_functional_structural_pseudo_classes() {
    assert!(parse_sheet(":first-child() { color: red; }").is_err());
    assert!(parse_sheet(":empty() { color: red; }").is_err());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css structural_simple_pseudo
cargo test -p surgeist-css compound_structural
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 4: Parse Tier 2 Nth Pseudo-Class Arguments

**Files:**
- Modify: `src/parser/selectors.rs`
- Modify: `src/tests.rs`

- [ ] Add `parse_nth_pattern` in `src/parser/selectors.rs` with this exact supported subset:
  - `odd` -> `CssNthPattern::Odd`
  - `even` -> `CssNthPattern::Even`
  - integer token such as `3` or `-2` -> `CssNthPattern::Integer(value)`
  - compact `n`, `-n`, and `+n` -> `CssNthPattern::AnPlusB(CssNthAnPlusB::new(a, 0))`
  - compact `an+b`, `an-b`, `-an+b`, and `+an-b` such as `2n+1`, `2n-1`, `-n+3`, `+3n-2` -> `CssNthPattern::AnPlusB(CssNthAnPlusB::new(a, b))`

- [ ] Implement `parse_nth_pattern` by walking `cssparser` tokens from the nested pseudo-class parser and requiring the nested parser to be exhausted after one supported pattern. Reject empty input, arbitrary identifiers, trailing tokens, and whitespace-separated arithmetic such as `2n + 1` in this pass.

- [ ] Reject `of <selector-list>` in this task. Add an explicit test so valid-but-unsupported `:nth-child(2n of .item)` rejects until a later plan models it.

- [ ] Parse functional pseudo-classes:

```rust
"nth-child" => CssPseudoClass::NthChild(parse_nth_pattern(input)?),
"nth-last-child" => CssPseudoClass::NthLastChild(parse_nth_pattern(input)?),
"nth-of-type" => CssPseudoClass::NthOfType(parse_nth_pattern(input)?),
"nth-last-of-type" => CssPseudoClass::NthLastOfType(parse_nth_pattern(input)?),
```

- [ ] Add tests.

```rust
#[test]
fn parses_nth_child_patterns() {
    let cases = [
        (":nth-child(odd) { color: red; }", CssPseudoClass::NthChild(CssNthPattern::Odd)),
        (":nth-child(even) { color: red; }", CssPseudoClass::NthChild(CssNthPattern::Even)),
        (":nth-child(3) { color: red; }", CssPseudoClass::NthChild(CssNthPattern::Integer(3))),
        (
            ":nth-child(2n+1) { color: red; }",
            CssPseudoClass::NthChild(CssNthPattern::AnPlusB(CssNthAnPlusB::new(2, 1))),
        ),
        (
            ":nth-child(-n+3) { color: red; }",
            CssPseudoClass::NthChild(CssNthPattern::AnPlusB(CssNthAnPlusB::new(-1, 3))),
        ),
    ];

    for (css, expected) in cases {
        let sheet = parse_sheet(css).unwrap();
        assert_eq!(sheet.rules()[0].selector(), &CssSelector::PseudoClass(expected));
    }
}

#[test]
fn parses_all_nth_pseudo_class_families() {
    let cases = [
        (":nth-child(2n) { color: red; }", "nth-child"),
        (":nth-last-child(2n) { color: red; }", "nth-last-child"),
        (":nth-of-type(2n) { color: red; }", "nth-of-type"),
        (":nth-last-of-type(2n) { color: red; }", "nth-last-of-type"),
    ];

    for (css, label) in cases {
        assert!(parse_sheet(css).is_ok(), "{label} should parse");
    }
}

#[test]
fn rejects_malformed_and_unsupported_nth_patterns() {
    assert!(parse_sheet(":nth-child() { color: red; }").is_err());
    assert!(parse_sheet(":nth-child(foo) { color: red; }").is_err());
    assert!(parse_sheet(":nth-child(2n +) { color: red; }").is_err());
    assert!(parse_sheet(":nth-child(2n + 1) { color: red; }").is_err());
    assert!(parse_sheet(":nth-child(2n of .item) { color: red; }").is_err());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css nth_child
cargo test -p surgeist-css nth_pseudo
cargo test -p surgeist-css malformed_and_unsupported_nth
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 5: Parse Tier 3 Selector-List Functional Pseudo-Classes

**Files:**
- Modify: `src/parser/selectors.rs`
- Modify: `src/tests.rs`

- [ ] Change selector-list parsing internals to return `CssSelectorList` at the helper boundary while preserving `QualifiedRuleParser::Prelude = Vec<CssSelector>` if that keeps the public parser code smaller. Use `CssSelectorList::new(selectors)` for pseudo-class arguments.

- [ ] Add a helper:

```rust
fn parse_pseudo_selector_list<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CssSelectorList, ParseError<'i, Error>> {
    let selectors = parse_selector_list(input)?;
    CssSelectorList::try_new(selectors).ok_or_else(|| {
        invalid_selector(input, "pseudo-class selector list must not be empty")
    })
}
```

- [ ] Parse functional pseudo-classes:

```rust
"not" => CssPseudoClass::Not(parse_pseudo_selector_list(input)?),
"is" => CssPseudoClass::Is(parse_pseudo_selector_list(input)?),
"where" => CssPseudoClass::Where(parse_pseudo_selector_list(input)?),
"has" => CssPseudoClass::Has(parse_pseudo_selector_list(input)?),
```

- [ ] Use the existing supported selector grammar inside these functions:
  - `.danger`
  - `button`
  - `#primary`
  - `.button:hover`
  - comma-separated lists of supported selectors

- [ ] Reject unsupported relative/combinator syntax inside `:has(...)`, such as `:has(> .icon)`, until selector combinators are modeled.

- [ ] Add tests.

```rust
#[test]
fn parses_selector_list_functional_pseudo_classes() {
    let sheet = parse_sheet(".button:not(.disabled, .loading) { color: red; }").unwrap();
    let CssSelector::Compound(selector) = sheet.rules()[0].selector() else {
        panic!("expected compound selector");
    };
    let [CssPseudoClass::Not(list)] = selector.pseudo_classes() else {
        panic!("expected :not selector list");
    };
    assert_eq!(list.selectors().len(), 2);

    assert!(parse_sheet(":is(.primary, .secondary) { color: red; }").is_ok());
    assert!(parse_sheet(":where(button, .link) { color: red; }").is_ok());
    assert!(parse_sheet(".field:has(.error) { color: red; }").is_ok());
}

#[test]
fn functional_pseudo_classes_can_contain_supported_pseudo_class_selectors() {
    let sheet = parse_sheet(".field:not(:disabled, :focus) { color: red; }").unwrap();
    let CssSelector::Compound(selector) = sheet.rules()[0].selector() else {
        panic!("expected compound selector");
    };
    let [CssPseudoClass::Not(list)] = selector.pseudo_classes() else {
        panic!("expected :not selector list");
    };
    assert_eq!(list.selectors().len(), 2);
}

#[test]
fn rejects_empty_or_unsupported_functional_pseudo_class_arguments() {
    assert!(parse_sheet(":not() { color: red; }").is_err());
    assert!(parse_sheet(":is() { color: red; }").is_err());
    assert!(parse_sheet(":where() { color: red; }").is_err());
    assert!(parse_sheet(":has() { color: red; }").is_err());
    assert!(parse_sheet(".field:has(> .icon) { color: red; }").is_err());
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css functional_pseudo
cargo test -p surgeist-css unsupported_functional_pseudo
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 6: Parse Tier 4 Runtime-State Pseudo-Classes

**Files:**
- Modify: `src/parser/selectors.rs`
- Modify: `src/tests.rs`
- Modify: `README.md`

- [ ] Extend identifier pseudo-class parsing for:

```rust
"modal" => Ok(CssPseudoClass::Modal),
"fullscreen" => Ok(CssPseudoClass::Fullscreen),
"popover-open" => Ok(CssPseudoClass::PopoverOpen),
"default" => Ok(CssPseudoClass::Default),
"indeterminate" => Ok(CssPseudoClass::Indeterminate),
"read-only" => Ok(CssPseudoClass::ReadOnly),
"read-write" => Ok(CssPseudoClass::ReadWrite),
"in-range" => Ok(CssPseudoClass::InRange),
"out-of-range" => Ok(CssPseudoClass::OutOfRange),
```

- [ ] Keep function syntax rejected for these names.

- [ ] Add tests.

```rust
#[test]
fn parses_tier_4_runtime_state_pseudo_classes() {
    let cases = [
        (":modal { color: red; }", CssPseudoClass::Modal),
        (":fullscreen { color: red; }", CssPseudoClass::Fullscreen),
        (":popover-open { color: red; }", CssPseudoClass::PopoverOpen),
        (":default { color: red; }", CssPseudoClass::Default),
        (":indeterminate { color: red; }", CssPseudoClass::Indeterminate),
        (":read-only { color: red; }", CssPseudoClass::ReadOnly),
        (":read-write { color: red; }", CssPseudoClass::ReadWrite),
        (":in-range { color: red; }", CssPseudoClass::InRange),
        (":out-of-range { color: red; }", CssPseudoClass::OutOfRange),
    ];

    for (css, expected) in cases {
        let sheet = parse_sheet(css).unwrap();
        assert_eq!(sheet.rules()[0].selector(), &CssSelector::PseudoClass(expected));
    }
}

#[test]
fn rejects_function_syntax_for_runtime_state_pseudo_classes() {
    assert!(parse_sheet(":modal() { color: red; }").is_err());
    assert!(parse_sheet(":fullscreen() { color: red; }").is_err());
    assert!(parse_sheet(":read-only() { color: red; }").is_err());
}
```

- [ ] Update `README.md` with a short note that these pseudo-classes are parsed as authored selector syntax and are not evaluated in `surgeist-css`.

```md
Pseudo-classes for UI interaction, form state, structure, selector-list filtering, and overlay state are parsed as authored selector syntax. This crate does not evaluate pseudo-class matches; runtime matching belongs to downstream Surgeist layers with node and interaction state.
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css runtime_state_pseudo
cargo test -p surgeist-css function_syntax_for_runtime_state
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 7: Strict Rejection Matrix And Public API Review

**Files:**
- Modify: `src/tests.rs`
- Modify: `README.md` only if Task 6 note needs a small wording adjustment.

- [ ] Add a final strict matrix test for pseudo-class accept/reject behavior.

```rust
#[test]
fn practical_pseudo_class_matrix_accepts_supported_and_rejects_unsupported_forms() {
    let accepted = [
        ":hover { color: red; }",
        ":focus-visible { color: red; }",
        ":disabled { color: red; }",
        ":first-child { color: red; }",
        ":nth-child(2n+1) { color: red; }",
        ":not(.disabled) { color: red; }",
        ":is(.primary, .secondary) { color: red; }",
        ":where(button, .link) { color: red; }",
        ".field:has(.error) { color: red; }",
        ":modal { color: red; }",
        ":read-only { color: red; }",
    ];

    for css in accepted {
        assert!(parse_sheet(css).is_ok(), "{css} should parse");
    }

    let rejected = [
        ":visited { color: red; }",
        ":target { color: red; }",
        ":lang(en) { color: red; }",
        ":host { color: red; }",
        ":state(open) { color: red; }",
        ":hover() { color: red; }",
        ":not() { color: red; }",
        ":nth-child(2n of .item) { color: red; }",
        ".field:has(> .icon) { color: red; }",
    ];

    for css in rejected {
        assert!(parse_sheet(css).is_err(), "{css} should reject");
    }
}
```

- [ ] Add public API review tests for structural access to selector-list and nth arguments.

```rust
#[test]
fn functional_pseudo_class_arguments_are_publicly_inspectable() {
    let sheet = parse_sheet(".button:not(.disabled) { color: red; }").unwrap();
    let CssSelector::Compound(selector) = sheet.rules()[0].selector() else {
        panic!("expected compound selector");
    };
    let [CssPseudoClass::Not(list)] = selector.pseudo_classes() else {
        panic!("expected :not");
    };
    assert_eq!(list.selectors(), &[CssSelector::Class("disabled".to_owned())]);
}

#[test]
fn nth_pseudo_class_arguments_are_publicly_inspectable() {
    let sheet = parse_sheet(":nth-child(2n+1) { color: red; }").unwrap();
    let CssSelector::PseudoClass(CssPseudoClass::NthChild(CssNthPattern::AnPlusB(value))) =
        sheet.rules()[0].selector()
    else {
        panic!("expected nth-child an+b selector");
    };
    assert_eq!(value.a(), 2);
    assert_eq!(value.b(), 1);
}
```

- [ ] Run:

```sh
git status --short --branch
git diff --stat
cargo fmt --check
cargo test -p surgeist-css practical_pseudo_class_matrix
cargo test -p surgeist-css publicly_inspectable
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 8: Final Checks And Holistic Review

**Files:**
- Inspect all changed files.

- [ ] Run:

```sh
git status --short --branch
git diff --stat
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Assign a clean-context holistic reviewer with no conversation history. Reviewer must inspect:
  - `AGENTS.md`
  - `guidance/surgeist-rust-modeling-guide.md`
  - this plan
  - public selector and pseudo-class syntax API
  - strict no-recovery behavior
  - tier coverage and explicit unsupported pseudo-class rejection

- [ ] Completion requires the holistic reviewer to return `APPROVED` with no unresolved findings.

## Completion Signal

Report:

- plan commit SHA
- implementation commit SHAs if execution follows
- final test count from `cargo test -p surgeist-css`
- checks run
- final holistic reviewer result
- whether the repo is pushed

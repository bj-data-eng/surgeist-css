# Strict Selector Argument Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement full-but-strict Selectors Level 4 selector-argument support for the selector-taking pseudo-classes currently owned by `surgeist-css`.

**Architecture:** Keep selector parsing authored and structural. Ordinary selector-list pseudos (`:is`, `:where`, `:not`) should store non-empty lists of the same supported complex selectors that top-level style rules already parse. Relational `:has` should use a distinct relative-selector-list model so leading combinators and implied descendant anchoring are represented explicitly. `:nth-child` and `:nth-last-child` should use a dedicated typed pattern that carries the existing `An+B` model plus an optional strict selector list for the `of S` clause.

**Tech Stack:** Rust 2024, `cssparser`, existing `surgeist-css` parser modules, crate-local tests in `src/tests.rs`.

---

## Source References

- W3C Selectors Level 4: https://www.w3.org/TR/selectors-4/
- Relevant spec concepts:
  - Compound selectors are sequences of simple selectors without combinators.
  - Complex selectors connect compound selectors with combinators.
  - Relative selectors begin with an explicit combinator or use an implied descendant combinator.
  - `:is()` and `:where()` use selector lists.
  - `:not()` accepts a complex-real-selector-list.
  - `:has()` accepts a relative-selector-list and disallows nested `:has()`.
  - `:nth-child()` and `:nth-last-child()` accept `An+B` with optional `of <complex-real-selector-list>`.

## Strict Surgeist Interpretation

Browsers treat some selector-list pseudo arguments as forgiving. `surgeist-css` must not. If any selector item inside `:is`, `:where`, `:not`, `:has`, or an `of` clause is malformed or unsupported, reject the whole sheet.

This plan expands accepted selector shapes but does not add browser recovery, selector matching, cascade, specificity calculation, namespace support, pseudo-element support, or style/root integration behavior.

## Current State

- `src/parser/selectors.rs` parses top-level style rule selectors with:
  - compound selectors: tags, IDs, classes, attributes, supported pseudo-classes
  - complex selectors with descendant, child, next-sibling, and subsequent-sibling combinators
- Functional pseudos currently route through `parse_pseudo_selector_list`, which uses a compound-only parser and `CssPseudoSelectorList::try_new` rejects complex selectors.
- `:has(> .icon)`, `:has(.field > .icon)`, and `:not(.field .icon)` are currently pinned as rejected in tests.
- `CssNthPattern` models only the `An+B` portion of nth selectors. There is no typed `of S` support.

## Target Behavior

Accept and structurally preserve:

```css
:not(.field .icon, button.primary:hover)
:is(.card > .title, button.primary:hover, [data-state="open"].active)
:where(.toolbar + .panel, .stack ~ .item)
.card:has(.field > .icon)
.card:has(> .icon)
.card:has(+ .error, ~ .warning)
li:nth-child(2n+1 of li.important, .row[hidden])
li:nth-last-child(even of .item.selected)
```

Reject strictly:

```css
:is(.valid, .bad..selector)
:where(.valid, .col || .cell)
:not(.valid, ::before)
:has()
:has(:has(.nested))
:has(::before)
:has(.valid, .bad..selector)
:nth-child(odd of)
:nth-child(odd of .valid, .bad..selector)
:nth-of-type(odd of .item)
```

## File Structure

- Modify `src/syntax.rs`
  - Relax `CssPseudoSelectorList` to represent non-empty strict complex selector lists instead of compound-only lists.
  - Add `CssRelativeSelector` and `CssRelativeSelectorList`.
  - Change `CssPseudoClass::Has` to store `CssRelativeSelectorList`.
  - Add `CssNthChildPattern` or equivalent dedicated type for `An+B` plus optional `CssPseudoSelectorList`.
  - Change `CssPseudoClass::NthChild` and `CssPseudoClass::NthLastChild` to use the new nth-child pattern type while keeping `NthOfType` and `NthLastOfType` on `CssNthPattern`.
- Modify `src/parser/selectors.rs`
  - Parse `:is`, `:where`, and `:not` with the existing complex selector parser, not the compound-only path.
  - Parse `:has` with a strict relative selector-list parser that supports explicit leading `>`, `+`, `~` and implied descendant.
  - Reject nested `:has` while allowing other supported pseudo-classes inside relative selector arguments.
  - Parse optional `of <selector-list>` for `:nth-child` and `:nth-last-child`.
  - Keep `:nth-of-type` and `:nth-last-of-type` accepting only existing `An+B`.
- Modify `src/tests.rs`
  - Update old rejection assertions that intentionally rejected complex arguments.
  - Add structural tests for complex arguments in `:is`, `:where`, and `:not`.
  - Add structural tests for relative selectors in `:has`.
  - Add structural tests for `nth-child(... of S)` and `nth-last-child(... of S)`.
  - Add strict rejection tests for malformed/unsupported entries in every selector-argument family.
- Modify `README.md`
  - Document that selector-list pseudo arguments are strict and structural, not forgiving.

---

## Task 1: Model Strict Selector Argument Types

**Files:**
- Modify: `/Users/codex/Development/surgeist-css/src/syntax.rs`
- Modify: `/Users/codex/Development/surgeist-css/src/tests.rs`

- [ ] **Step 1: Check starting status**

Run:

```sh
git status --short --branch
```

Expected: clean worktree on `main`; do not create a branch.

- [ ] **Step 2: Add tests for constructor/model invariants**

Add tests that exercise public constructors without parsing:

```rust
#[test]
fn pseudo_selector_list_constructor_accepts_complex_selectors() {
    let first = CssCompoundSelector::new(
        None,
        None,
        vec!["field".to_owned()],
        Vec::new(),
        Vec::new(),
    );
    let part = CssComplexSelectorPart::new(
        CssSelectorCombinator::Descendant,
        CssCompoundSelector::new(
            None,
            None,
            vec!["icon".to_owned()],
            Vec::new(),
            Vec::new(),
        ),
    );
    let complex = CssSelector::Complex(CssComplexSelector::new(first, vec![part]));

    let list = CssPseudoSelectorList::try_new(vec![complex.clone()]).unwrap();
    assert_eq!(list.selectors(), &[complex]);
}

#[test]
fn relative_selector_list_constructor_requires_selectors() {
    assert_eq!(CssRelativeSelectorList::try_new(Vec::new()), None);
}

#[test]
fn relative_selector_preserves_combinator_and_selector() {
    let selector = CssRelativeSelector::new(
        CssSelectorCombinator::Child,
        CssSelector::Class("icon".to_owned()),
    );

    assert_eq!(selector.combinator(), CssSelectorCombinator::Child);
    assert_eq!(selector.selector(), &CssSelector::Class("icon".to_owned()));
}

#[test]
fn nth_child_pattern_preserves_optional_selector_list() {
    let list = CssPseudoSelectorList::try_new(vec![CssSelector::Class("important".to_owned())])
        .unwrap();
    let pattern =
        CssNthChildPattern::new(CssNthPattern::AnPlusB(CssNthAnPlusB::new(2, 1)), Some(list));

    assert!(matches!(pattern.pattern(), CssNthPattern::AnPlusB(value) if value.a() == 2 && value.b() == 1));
    assert_eq!(pattern.selector_list().unwrap().selectors(), &[CssSelector::Class("important".to_owned())]);
}
```

Run:

```sh
cargo test -p surgeist-css pseudo_selector_list_constructor_accepts_complex_selectors
cargo test -p surgeist-css relative_selector_list_constructor_requires_selectors
cargo test -p surgeist-css relative_selector_preserves_combinator_and_selector
cargo test -p surgeist-css nth_child_pattern_preserves_optional_selector_list
```

Expected before implementation: compile failure for missing `CssRelativeSelectorList` and `CssNthChildPattern`, and/or assertion failure for complex selector list construction.

- [ ] **Step 3: Implement the syntax model**

Model requirements:

- `CssPseudoSelectorList::try_new` rejects only empty lists.
- `CssRelativeSelector` has private fields:
  - `combinator: CssSelectorCombinator`
  - `selector: CssSelector`
- `CssRelativeSelector::new(combinator, selector)` preserves the explicit or implied combinator.
- `CssRelativeSelector::combinator(&self) -> CssSelectorCombinator`
- `CssRelativeSelector::selector(&self) -> &CssSelector`
- `CssRelativeSelectorList::try_new(Vec<CssRelativeSelector>) -> Option<Self>` rejects empty lists.
- `CssRelativeSelectorList::selectors(&self) -> &[CssRelativeSelector]`
- `CssPseudoClass::Has(CssRelativeSelectorList)`
- `CssNthChildPattern` has private fields:
  - `pattern: CssNthPattern`
  - `selector_list: Option<CssPseudoSelectorList>`
- `CssNthChildPattern::new(pattern, selector_list)`
- `CssNthChildPattern::pattern(&self) -> CssNthPattern`
- `CssNthChildPattern::selector_list(&self) -> Option<&CssPseudoSelectorList>`
- `CssPseudoClass::NthChild(CssNthChildPattern)`
- `CssPseudoClass::NthLastChild(CssNthChildPattern)`
- `CssPseudoClass::NthOfType(CssNthPattern)` remains unchanged.
- `CssPseudoClass::NthLastOfType(CssNthPattern)` remains unchanged.

Keep fields private. Do not expose mutable vectors.

- [ ] **Step 4: Update existing tests for the breaking nth API**

Existing tests matching `CssPseudoClass::NthChild(CssNthPattern::...)` should match the new wrapper and assert `selector_list().is_none()` for old forms.

Example pattern:

```rust
let CssSelector::PseudoClass(CssPseudoClass::NthChild(pattern)) =
    style_rule(&sheet.rules()[0]).selector()
else {
    panic!("expected nth-child selector");
};
assert_eq!(pattern.pattern(), CssNthPattern::Odd);
assert!(pattern.selector_list().is_none());
```

- [ ] **Step 5: Run focused model tests**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-css pseudo_selector_list_constructor_accepts_complex_selectors
cargo test -p surgeist-css relative_selector_list_constructor_requires_selectors
cargo test -p surgeist-css relative_selector_preserves_combinator_and_selector
cargo test -p surgeist-css nth_child_pattern_preserves_optional_selector_list
```

Expected: pass.

- [ ] **Step 6: Report**

Report files changed, tests run, and `git status --short --branch`. Workers do not commit.

Coordinator gate after clean reviewer:

```sh
cargo fmt --check
cargo test -p surgeist-css pseudo_selector_list_constructor_accepts_complex_selectors
cargo test -p surgeist-css relative_selector_list_constructor_requires_selectors
cargo test -p surgeist-css relative_selector_preserves_combinator_and_selector
cargo test -p surgeist-css nth_child_pattern_preserves_optional_selector_list
git diff --check
git add src/syntax.rs src/tests.rs
git commit -m "Model strict selector argument syntax"
```

## Task 2: Parse Complex Selector Lists For `:is`, `:where`, And `:not`

**Files:**
- Modify: `/Users/codex/Development/surgeist-css/src/parser/selectors.rs`
- Modify: `/Users/codex/Development/surgeist-css/src/tests.rs`

- [ ] **Step 1: Add failing structural tests**

Add parser tests proving the ordinary selector-list pseudos accept full supported complex selectors and preserve structure:

```rust
#[test]
fn functional_selector_lists_accept_supported_complex_selectors() {
    let sheet = parse_sheet(
        ".scope:is(.card > .title, button.primary:hover, [data-state=\"open\"].active) { color: black; }",
    )
    .unwrap();
    let CssSelector::Compound(selector) = style_rule(&sheet.rules()[0]).selector() else {
        panic!("expected compound selector");
    };
    let [CssPseudoClass::Is(list)] = selector.pseudo_classes() else {
        panic!("expected :is selector list");
    };
    assert_eq!(list.selectors().len(), 3);
    assert!(matches!(list.selectors()[0], CssSelector::Complex(_)));
    assert!(matches!(list.selectors()[1], CssSelector::Compound(_)));
    assert!(matches!(list.selectors()[2], CssSelector::Compound(_)));

    let sheet =
        parse_sheet(":not(.field .icon, button.primary:hover) { color: black; }").unwrap();
    let CssSelector::PseudoClass(CssPseudoClass::Not(list)) =
        style_rule(&sheet.rules()[0]).selector()
    else {
        panic!("expected :not selector list");
    };
    assert_eq!(list.selectors().len(), 2);
    assert!(matches!(list.selectors()[0], CssSelector::Complex(_)));
    assert!(matches!(list.selectors()[1], CssSelector::Compound(_)));

    let sheet = parse_sheet(":where(.toolbar + .panel, .stack ~ .item) { color: black; }")
        .unwrap();
    let CssSelector::PseudoClass(CssPseudoClass::Where(list)) =
        style_rule(&sheet.rules()[0]).selector()
    else {
        panic!("expected :where selector list");
    };
    assert_eq!(list.selectors().len(), 2);
    assert!(list
        .selectors()
        .iter()
        .all(|selector| matches!(selector, CssSelector::Complex(_))));
}
```

Add strict rejection tests:

```rust
#[test]
fn functional_selector_lists_reject_invalid_entries_strictly() {
    assert!(parse_sheet(":is(.valid, .bad..selector) { color: black; }").is_err());
    assert!(parse_sheet(":where(.valid, .col || .cell) { color: black; }").is_err());
    assert!(parse_sheet(":not(.valid, ::before) { color: black; }").is_err());
    assert!(parse_sheet(":is(.valid,) { color: black; }").is_err());
}
```

Run:

```sh
cargo test -p surgeist-css functional_selector_lists_accept_supported_complex_selectors
cargo test -p surgeist-css functional_selector_lists_reject_invalid_entries_strictly
```

Expected before parser change: complex accepts fail; strict invalid entries should still fail.

- [ ] **Step 2: Replace compound-only pseudo selector-list parsing**

In `src/parser/selectors.rs`:

- Replace `parse_pseudo_compound_selector_list` with a parser that calls `parse_rule_selector(input)` for each comma-separated item.
- Keep `input.expect_exhausted()` after the list to reject trailing tokens.
- Preserve strict list behavior: do not drop invalid items.
- Keep `CssPseudoSelectorList::try_new` as the non-empty guard.

Expected shape:

```rust
fn parse_pseudo_selector_list_items<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<Vec<CssSelector>, ParseError<'i, Error>> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(parse_rule_selector(input)?);
        if input.try_parse(Parser::expect_comma).is_err() {
            break;
        }
    }
    input.expect_exhausted().map_err(selector_basic)?;
    Ok(selectors)
}
```

Use this helper from `parse_pseudo_selector_list`.

- [ ] **Step 3: Keep unsupported forms unsupported**

Do not add namespaces, pseudo-elements, column combinator, nesting selector `&`, or universal selectors in this task.

- [ ] **Step 4: Run focused tests**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-css functional_selector_lists_accept_supported_complex_selectors
cargo test -p surgeist-css functional_selector_lists_reject_invalid_entries_strictly
cargo test -p surgeist-css parses_selector_list_functional_pseudo_classes
cargo test -p surgeist-css parses_compound_selector_list_functional_pseudo_classes
```

Expected: pass.

- [ ] **Step 5: Report**

Report changed files, tests run, and `git status --short --branch`. Workers do not commit.

Coordinator gate after clean reviewer:

```sh
cargo fmt --check
cargo test -p surgeist-css functional_selector_lists
git diff --check
git add src/parser/selectors.rs src/tests.rs
git commit -m "Parse strict complex selector-list pseudos"
```

## Task 3: Parse Relative Selector Lists For `:has`

**Files:**
- Modify: `/Users/codex/Development/surgeist-css/src/parser/selectors.rs`
- Modify: `/Users/codex/Development/surgeist-css/src/tests.rs`

- [ ] **Step 1: Add failing structural tests for relative selectors**

Add tests:

```rust
#[test]
fn has_accepts_strict_relative_selector_lists() {
    let sheet = parse_sheet(".card:has(.field > .icon) { color: black; }").unwrap();
    let CssSelector::Compound(selector) = style_rule(&sheet.rules()[0]).selector() else {
        panic!("expected compound selector");
    };
    let [CssPseudoClass::Has(list)] = selector.pseudo_classes() else {
        panic!("expected :has selector list");
    };
    assert_eq!(list.selectors().len(), 1);
    assert_eq!(
        list.selectors()[0].combinator(),
        CssSelectorCombinator::Descendant
    );
    assert!(matches!(list.selectors()[0].selector(), CssSelector::Complex(_)));

    let sheet = parse_sheet(".card:has(> .icon, + .error, ~ .warning) { color: black; }")
        .unwrap();
    let CssSelector::Compound(selector) = style_rule(&sheet.rules()[0]).selector() else {
        panic!("expected compound selector");
    };
    let [CssPseudoClass::Has(list)] = selector.pseudo_classes() else {
        panic!("expected :has selector list");
    };
    assert_eq!(list.selectors().len(), 3);
    assert_eq!(list.selectors()[0].combinator(), CssSelectorCombinator::Child);
    assert_eq!(
        list.selectors()[1].combinator(),
        CssSelectorCombinator::NextSibling
    );
    assert_eq!(
        list.selectors()[2].combinator(),
        CssSelectorCombinator::SubsequentSibling
    );
}
```

Add strict rejection tests:

```rust
#[test]
fn has_rejects_invalid_relative_selector_entries_strictly() {
    assert!(parse_sheet(".card:has() { color: black; }").is_err());
    assert!(parse_sheet(".card:has(.valid, .bad..selector) { color: black; }").is_err());
    assert!(parse_sheet(".card:has(:has(.nested)) { color: black; }").is_err());
    assert!(parse_sheet(".card:has(:is(:has(.nested))) { color: black; }").is_err());
    assert!(parse_sheet(".card:has(:nth-child(odd of :has(.nested))) { color: black; }").is_err());
    assert!(parse_sheet(".card:has(::before) { color: black; }").is_err());
    assert!(parse_sheet(".card:has(| .bad) { color: black; }").is_err());
}
```

Run:

```sh
cargo test -p surgeist-css has_accepts_strict_relative_selector_lists
cargo test -p surgeist-css has_rejects_invalid_relative_selector_entries_strictly
```

Expected before parser change: leading combinator and complex relative forms fail.

- [ ] **Step 2: Implement `:has`-specific parser path**

Add parser helpers in `src/parser/selectors.rs`:

- `parse_has_relative_selector_list`
- `parse_has_relative_selector`
- `parse_selector_after_leading_combinator`

Required behavior:

- `:has(.field > .icon)` stores `CssRelativeSelector::new(CssSelectorCombinator::Descendant, CssSelector::Complex(...))`.
- `:has(> .icon)` stores `CssRelativeSelector::new(CssSelectorCombinator::Child, CssSelector::Class("icon"))` or an equivalent compound selector representation.
- `:has(+ .error)` and `:has(~ .warning)` store sibling combinators.
- Nested `:has()` rejects.
- The list rejects empty input and trailing commas.
- The parser remains strict; any malformed or unsupported item rejects the whole pseudo-class.

Implementation guidance:

- Reuse `parse_rule_selector` for implied descendant relative selectors.
- For explicit leading combinators, consume the leading combinator and parse the following compound selector plus any later combinator chain.
- Do not duplicate all top-level selector parsing logic if a small helper can share the loop used by `parse_rule_selector`.
- Keep `||` unsupported.

- [ ] **Step 3: Add nested `:has` guard**

Avoid threading broad parser state through the crate. A narrow `parse_pseudo_class_with_has_policy(input, allow_has: bool)` helper is acceptable:

- top-level and ordinary selector-list contexts call with `allow_has = true`
- `:has` relative selector parsing calls with `allow_has = false`
- when `allow_has = false`, the `has` function token returns `invalid_selector`
- the `allow_has = false` policy must propagate through nested selector-list pseudos and nth-child `of` selector filters parsed inside `:has`
- `.card:has(:is(:has(.nested)))` and `.card:has(:nth-child(odd of :has(.nested)))` must reject

If a simpler local approach is found that preserves the same invariant without a broad flag bag, use it.

- [ ] **Step 4: Run focused tests**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-css has_accepts_strict_relative_selector_lists
cargo test -p surgeist-css has_rejects_invalid_relative_selector_entries_strictly
cargo test -p surgeist-css rejects_unsupported_relative_or_combinator_selector_forms
cargo test -p surgeist-css rejects_invalid_combinator_selectors
```

Expected: pass after updating old rejection tests to reflect newly supported `:has(.field > .icon)` and `:has(> .icon)`.

- [ ] **Step 5: Report**

Report changed files, tests run, any old rejection tests updated, and `git status --short --branch`. Workers do not commit.

Coordinator gate after clean reviewer:

```sh
cargo fmt --check
cargo test -p surgeist-css has_
cargo test -p surgeist-css combinator
git diff --check
git add src/parser/selectors.rs src/tests.rs
git commit -m "Parse strict relative selectors for has"
```

## Task 4: Parse `of S` For `:nth-child` And `:nth-last-child`

**Files:**
- Modify: `/Users/codex/Development/surgeist-css/src/parser/selectors.rs`
- Modify: `/Users/codex/Development/surgeist-css/src/tests.rs`

- [ ] **Step 1: Add failing structural tests**

Add tests:

```rust
#[test]
fn nth_child_accepts_strict_of_selector_lists() {
    let sheet =
        parse_sheet("li:nth-child(2n+1 of li.important, .row[hidden]) { color: black; }")
            .unwrap();
    let CssSelector::Compound(selector) = style_rule(&sheet.rules()[0]).selector() else {
        panic!("expected compound selector");
    };
    let [CssPseudoClass::NthChild(pattern)] = selector.pseudo_classes() else {
        panic!("expected nth-child pseudo-class");
    };
    assert!(matches!(pattern.pattern(), CssNthPattern::AnPlusB(value) if value.a() == 2 && value.b() == 1));
    let selector_list = pattern.selector_list().expect("expected of selector list");
    assert_eq!(selector_list.selectors().len(), 2);
    assert!(matches!(selector_list.selectors()[0], CssSelector::Compound(_)));
    assert!(matches!(selector_list.selectors()[1], CssSelector::Compound(_)));

    let sheet = parse_sheet(".item:nth-last-child(even of .selected ~ .tail) { color: black; }")
        .unwrap();
    let CssSelector::Compound(selector) = style_rule(&sheet.rules()[0]).selector() else {
        panic!("expected compound selector");
    };
    let [CssPseudoClass::NthLastChild(pattern)] = selector.pseudo_classes() else {
        panic!("expected nth-last-child pseudo-class");
    };
    assert_eq!(pattern.pattern(), CssNthPattern::Even);
    assert!(matches!(
        pattern.selector_list().unwrap().selectors()[0],
        CssSelector::Complex(_)
    ));
}
```

Add rejection tests:

```rust
#[test]
fn nth_child_of_selector_lists_reject_invalid_entries_strictly() {
    assert!(parse_sheet(":nth-child(odd of) { color: black; }").is_err());
    assert!(parse_sheet(":nth-child(odd of .valid, .bad..selector) { color: black; }").is_err());
    assert!(parse_sheet(":nth-child(odd of ::before) { color: black; }").is_err());
    assert!(parse_sheet(":nth-of-type(odd of .item) { color: black; }").is_err());
    assert!(parse_sheet(":nth-last-of-type(even of .item) { color: black; }").is_err());
}
```

Run:

```sh
cargo test -p surgeist-css nth_child_accepts_strict_of_selector_lists
cargo test -p surgeist-css nth_child_of_selector_lists_reject_invalid_entries_strictly
```

Expected before implementation: `of` forms fail.

- [ ] **Step 2: Implement nth-child parser split**

Add helpers:

- `parse_nth_child_pattern(input) -> CssNthChildPattern`
- `parse_nth_an_plus_b(input) -> CssNthPattern` or keep `parse_nth_pattern` for the existing `An+B` logic

Required behavior:

- `nth-child` and `nth-last-child` parse an existing `CssNthPattern`.
- If the next non-whitespace token is ident `of`, parse a strict `CssPseudoSelectorList` using the complex selector-list parser from Task 2.
- If no `of` appears, return `CssNthChildPattern::new(pattern, None)`.
- `nth-of-type` and `nth-last-of-type` continue to call the existing `An+B` parser and then require exhaustion, so `of` remains rejected for those functions.

- [ ] **Step 3: Keep nth parser strict**

Do not broaden `An+B` itself except as needed to keep current accepted tests passing. Unsupported current patterns such as unsupported `of` syntax, malformed dimensions, trailing tokens, and invalid selector entries must reject.

- [ ] **Step 4: Run focused tests**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-css nth_child_accepts_strict_of_selector_lists
cargo test -p surgeist-css nth_child_of_selector_lists_reject_invalid_entries_strictly
cargo test -p surgeist-css parses_nth_child_patterns
cargo test -p surgeist-css rejects_unsupported_nth_patterns_and_of_selector_forms
cargo test -p surgeist-css rejects_trailing_tokens_in_nth_functions
```

Expected: pass after updating older rejection tests that intentionally rejected all `of` forms.

- [ ] **Step 5: Report**

Report changed files, tests run, and `git status --short --branch`. Workers do not commit.

Coordinator gate after clean reviewer:

```sh
cargo fmt --check
cargo test -p surgeist-css nth
git diff --check
git add src/parser/selectors.rs src/tests.rs
git commit -m "Parse strict nth-child selector filters"
```

## Task 5: Acceptance Matrix, Strictness Matrix, And README

**Files:**
- Modify: `/Users/codex/Development/surgeist-css/src/tests.rs`
- Modify: `/Users/codex/Development/surgeist-css/README.md`

- [ ] **Step 1: Add practical acceptance matrix**

Add a matrix test that covers the feature at user-facing CSS level:

```rust
#[test]
fn selector_argument_surface_accepts_full_supported_strict_forms() {
    for css in [
        ":not(.field .icon, button.primary:hover) { color: black; }",
        ":is(.card > .title, button.primary:hover, [data-state=\"open\"].active) { color: black; }",
        ":where(.toolbar + .panel, .stack ~ .item) { color: black; }",
        ".card:has(.field > .icon) { color: black; }",
        ".card:has(> .icon, + .error, ~ .warning) { color: black; }",
        "li:nth-child(2n+1 of li.important, .row[hidden]) { color: black; }",
        "li:nth-last-child(even of .item.selected) { color: black; }",
    ] {
        parse_sheet(css).unwrap_or_else(|error| panic!("{css} should parse: {error:?}"));
    }
}
```

- [ ] **Step 2: Add strict no-recovery matrix**

Add a rejection matrix:

```rust
#[test]
fn selector_argument_surface_rejects_invalid_entries_without_recovery() {
    for css in [
        ":is(.valid, .bad..selector) { color: black; }",
        ":where(.valid, .col || .cell) { color: black; }",
        ":not(.valid, ::before) { color: black; }",
        ".card:has() { color: black; }",
        ".card:has(:has(.nested)) { color: black; }",
        ".card:has(:is(:has(.nested))) { color: black; }",
        ".card:has(:nth-child(odd of :has(.nested))) { color: black; }",
        ".card:has(::before) { color: black; }",
        ".card:has(.valid, .bad..selector) { color: black; }",
        ":nth-child(odd of) { color: black; }",
        ":nth-child(odd of .valid, .bad..selector) { color: black; }",
        ":nth-of-type(odd of .item) { color: black; }",
    ] {
        assert!(parse_sheet(css).is_err(), "{css} should reject strictly");
    }
}
```

- [ ] **Step 3: Update existing broad selector matrices**

Update any older tests that list supported/unsupported functional pseudo-class forms so they reflect the new support:

- Supported now:
  - `.field:has(> .icon)`
  - `.field:has(.field > .icon)`
  - `.field:not(.field .icon)`
- Still unsupported:
  - column combinator `||`
  - pseudo-elements
  - namespaces
  - nested `:has`
  - malformed selector entries

- [ ] **Step 4: Update README**

Add or adjust selector paragraph:

```md
Selector-list pseudo-class arguments are parsed as strict authored selector syntax. `:is`, `:where`, and `:not` preserve supported complex selector lists; `:has` preserves supported relative selector lists including leading child and sibling combinators; and `:nth-child` / `:nth-last-child` preserve optional `of` selector filters. Unlike browser forgiving selector lists, any unsupported or malformed selector argument rejects the whole sheet.
```

- [ ] **Step 5: Run matrix and full checks**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-css selector_argument_surface
cargo test -p surgeist-css practical_pseudo_class_matrix_accepts_supported_and_rejects_unsupported_forms
cargo test -p surgeist-css strict_no_recovery_whole_sheet_rejects_every_invalid_surface
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Expected: pass.

- [ ] **Step 6: Report**

Report changed files, checks run, and `git status --short --branch`. Workers do not commit.

Coordinator gate after clean reviewer:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
git add README.md src/tests.rs
git commit -m "Test strict selector argument support"
```

## Final Coordinator Verification

After all task-scoped worker/reviewer cycles are clean and committed:

```sh
git status --short --branch
git diff --stat origin/main..HEAD
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Then assign a final clean-context holistic reviewer. The reviewer must inspect:

- `AGENTS.md`
- `guidance/surgeist-rust-modeling-guide.md`
- this plan
- `src/syntax.rs`
- `src/parser/selectors.rs`
- `src/tests.rs`
- `README.md`

Reviewer checklist:

- Selector argument support matches the plan and the strict Surgeist interpretation.
- `:is`, `:where`, and `:not` accept full supported complex selector lists.
- `:has` accepts full supported relative selector lists and rejects nested `:has`.
- `:nth-child` and `:nth-last-child` accept optional strict `of` selector lists.
- `:nth-of-type` and `:nth-last-of-type` do not accidentally accept `of`.
- Invalid entries reject the whole sheet; no forgiving browser recovery was introduced.
- Pseudo-elements, namespaces, column combinator, universal selectors, selector matching, specificity calculation, and cascade behavior were not added accidentally.
- Public APIs are typed, inspectable, and do not expose mutable internals.
- Rust modeling guide is followed.

Completion is only after the final reviewer returns clean and the final local checks pass.

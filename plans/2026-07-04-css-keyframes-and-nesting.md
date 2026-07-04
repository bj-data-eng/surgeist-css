# CSS Keyframes And Nesting Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add strict authored CSS support for `@keyframes` and native CSS nesting, with nesting flattened inside `surgeist-css` because it is context-free CSS syntax sugar.

**Architecture:** `@keyframes` remains authored CSS syntax owned by this crate: the parser records typed keyframe names, selectors, blocks, declarations, and source locations without running animations. CSS nesting is not exposed as a long-lived public syntax node; nested style and conditional group rules are flattened into existing `CssRule` structures while preserving source order by emitting declaration runs around nested items.

**Tech Stack:** Rust 2024, `cssparser`, crate-local parser modules, crate-local authored syntax types, `cargo fmt`, `cargo test -p surgeist-css`, `cargo clippy -p surgeist-css --all-targets -- -D warnings`.

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
- Keep new keyframe models CSS-owned and authored/parser-facing.
- Flatten nesting in this crate because it is syntax sugar that does not require cascade, runtime state, selector matching, layout, or environment information.
- Do not evaluate animations, keyframe timing, media queries, container queries, selectors, pseudo-classes, cascade, or variables in this crate.
- Do not load files, fetch URLs, resolve imports, activate fonts, match selectors, or compute styles.
- Breaking public API changes are allowed when they improve authored CSS modeling.
- External/root tests must exercise public integration behavior only; do not preserve accidental APIs solely for compatibility.

## Specification References

Use these as grammar references, but implement only the strict subset explicitly listed in this plan:

- CSS Animations Level 1, especially `@keyframes`, `<keyframes-name>`, and `<keyframe-selector>`: <https://www.w3.org/TR/css-animations-1/>
- CSS Nesting Module Level 1, especially relative nested style rules and the nesting selector `&`: <https://www.w3.org/TR/css-nesting-1/>
- Selectors Level 4, for selector forms already modeled by this crate: <https://www.w3.org/TR/selectors-4/>

## Current Baseline

- `CssRule` currently has `Import`, `FontFace`, `Style`, `Media`, and `Container`.
- `parse_sheet` currently rejects `@keyframes`.
- `StrictRuleParser::parse_block` for style rules currently parses declarations only; nested qualified rules and nested at-rules inside style rules reject.
- `@media` and `@container` can contain nested group rules through `parse_nested_group_rules`.
- Selectors currently support tag, id/key, class, compound selectors, practical pseudo-classes, attribute selectors, and complex selectors with descendant, child, next-sibling, and subsequent-sibling combinators.
- Functional pseudo-class selector-list arguments intentionally remain restricted and must not become complex selector escape hatches.
- Animation declaration values already exist for animation-related properties, but `animation-name` currently models `none` and custom identifiers only.

## Scope

Implement authored parsing and typed syntax for:

- Top-level `@keyframes` rules.
- Nested `@keyframes` inside `@media` and `@container` group rules.
- Keyframe names as either custom identifiers or quoted strings.
- `animation-name` string names so string-named keyframes can be referenced by authored CSS.
- Keyframe selectors:
  - `from`
  - `to`
  - percentage selectors from `0%` through `100%`
  - comma-separated selector lists such as `0%, 50%, 100%`
- Keyframe blocks containing strict declarations parsed with this crate's existing property-specific declaration validation.
- Native CSS nesting in style rule blocks, flattened into ordinary `CssRule::Style`, `CssRule::Media`, and `CssRule::Container` output.
- Nested style rules inside top-level style rules.
- Nested style rules inside `@media` and `@container`.
- Nested `@media` and `@container` inside style rules, flattened while carrying the parent selector context into the nested group.
- Nested selector forms:
  - implicit descendant nesting, such as `.card { .title { color: black; } }`
  - leading relative combinators, such as `.card { > .icon { color: black; } }`
  - leading nesting selector `&`, such as `.button { &:hover { color: black; } }`
  - leading `&` followed by an existing supported combinator, such as `.tabs { & > .tab { color: black; } }`
  - leading `&` with appended class, attribute, and pseudo-class simple selectors, such as `.button { &.active[aria-current=true]:hover { color: black; } }`

Do not implement in this pass:

- Keyframe timeline range names from scroll-driven animations.
- Vendor-prefixed keyframe at-rules such as `@-webkit-keyframes`.
- Keyframe selector values without a percent unit, such as `0`.
- Keyframe selectors outside `0%..=100%`.
- Browser recovery for invalid keyframe blocks, duplicate keyframe selectors, invalid keyframe declarations, or invalid nested rules.
- Keyframe cascade/effect evaluation, animation interpolation, timing resolution, or animation-name to keyframes matching.
- `@keyframes` inside style rules, `@font-face`, `@import`, or `@keyframes`.
- Nested `@import`, `@font-face`, or `@keyframes` inside style rules.
- `@supports`, `@layer`, `@scope`, `@property`, `@page`, `@namespace`, `@charset`, or `@starting-style`.
- Nested selectors with `&` anywhere except the start of the nested selector.
- Multiple `&` occurrences in one nested selector.
- Nested selectors that require unsupported selector grammar, including pseudo-elements, namespaces, column combinator `||`, or complex selector-list pseudo-class arguments.

## Modeling Rules

- `@keyframes` is authored syntax. It records the animation name and keyframe blocks; it does not run animations.
- Keyframe blocks store declarations using existing `CssDeclaration` values so property-specific strict validation stays centralized.
- Keyframe selector percentages must use a dedicated bounded type, not a loose `f32`.
- Duplicate keyframe offsets reject. Browsers may cascade duplicate keyframe selectors, but Surgeist strict CSS should avoid hidden recovery and last-one-wins behavior.
- `from` and `0%` are the same semantic offset and conflict with each other in the same `@keyframes`; `to` and `100%` are likewise the same semantic offset.
- A keyframe block selector list must not be empty, and a keyframe block declaration list must not be empty.
- `@keyframes` must contain at least one keyframe block.
- `@keyframes` names must be typed separately from declaration values. Do not put keyframe rules into `CssValue`.
- Nesting flattening is an input transformation, not a selector matcher. It combines authored selector syntax into ordinary `CssSelector` values and never checks whether an element matches.
- Flattening must preserve source order. A style block with declarations before and after a nested item must emit separate parent `CssStyleRule` chunks around the nested output rather than merging all parent declarations into one rule.
- Flattening must keep declaration validation strict. Invalid declarations in any nested block reject the whole sheet.
- `CssValue` must not become a cross-property validation bag.

## Target Keyframes Model

Add `CssRule::Keyframes(CssKeyframesRule)`:

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum CssRule {
    Import(CssImportRule),
    FontFace(CssFontFaceRule),
    Keyframes(CssKeyframesRule),
    Style(CssStyleRule),
    Media(CssMediaRule),
    Container(CssContainerRule),
}
```

Add keyframe syntax types to `src/syntax.rs` near other at-rule models:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct CssKeyframesRule {
    name: CssKeyframesName,
    blocks: Vec<CssKeyframeBlock>,
    location: CssSourceLocation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssKeyframesName {
    Ident(CssCustomIdent),
    String(CssKeyframesString),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssKeyframesString {
    value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssKeyframeBlock {
    selectors: CssKeyframeSelectorList,
    declarations: Vec<CssDeclaration>,
    location: CssSourceLocation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssKeyframeSelectorList {
    selectors: Vec<CssKeyframeSelector>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CssKeyframeSelector {
    From,
    To,
    Percent(CssKeyframePercent),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssKeyframePercent {
    value: CssFiniteNumber,
}
```

Required public APIs:

```rust
impl CssKeyframesRule {
    pub fn try_new(
        name: CssKeyframesName,
        blocks: Vec<CssKeyframeBlock>,
        location: CssSourceLocation,
    ) -> Option<Self>;
    pub(crate) fn new(
        name: CssKeyframesName,
        blocks: Vec<CssKeyframeBlock>,
        location: CssSourceLocation,
    ) -> Self;
    pub const fn name(&self) -> &CssKeyframesName;
    pub fn blocks(&self) -> &[CssKeyframeBlock];
    pub const fn location(&self) -> CssSourceLocation;
}

impl CssKeyframesString {
    pub fn try_new(value: impl Into<String>) -> Option<Self>;
    pub(crate) fn new(value: impl Into<String>) -> Self;
    pub fn as_str(&self) -> &str;
}

impl CssKeyframeBlock {
    pub fn try_new(
        selectors: CssKeyframeSelectorList,
        declarations: Vec<CssDeclaration>,
        location: CssSourceLocation,
    ) -> Option<Self>;
    pub(crate) fn new(
        selectors: CssKeyframeSelectorList,
        declarations: Vec<CssDeclaration>,
        location: CssSourceLocation,
    ) -> Self;
    pub const fn selectors(&self) -> &CssKeyframeSelectorList;
    pub fn declarations(&self) -> &[CssDeclaration];
    pub const fn location(&self) -> CssSourceLocation;
}

impl CssKeyframeSelectorList {
    pub fn try_new(selectors: Vec<CssKeyframeSelector>) -> Option<Self>;
    pub(crate) fn new(selectors: Vec<CssKeyframeSelector>) -> Self;
    pub fn selectors(&self) -> &[CssKeyframeSelector];
}

impl CssKeyframeSelector {
    pub fn offset(self) -> CssKeyframePercent;
}

impl CssKeyframePercent {
    pub fn try_new(value: f32) -> Option<Self>;
    pub(crate) fn new(value: f32) -> Self;
    pub const fn value(self) -> CssFiniteNumber;
}
```

`CssKeyframesRule::try_new` must reject empty blocks and duplicate semantic offsets across all block selector lists.

`CssKeyframeSelectorList::try_new` must reject empty lists and duplicate semantic offsets in the same selector list.

`CssKeyframePercent::try_new` must reject non-finite values and values below `0.0` or above `100.0`.

Extend `CssAnimationName`:

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssAnimationName {
    None,
    Custom(CssCustomIdent),
    String(CssKeyframesString),
}
```

`animation-name` and the `animation` shorthand should accept quoted string names and preserve them as `CssAnimationName::String`.

## Target Nesting Flattening Behavior

The parser should continue returning `CssSheet { rules: Vec<CssRule> }`, not a nested rule tree.

For an input:

```css
.card {
  color: black;

  .title {
    color: blue;
  }

  background: white;
}
```

the parser should emit three `CssRule::Style` rules in order:

```text
.card { color: black; }
.card .title { color: blue; }
.card { background: white; }
```

For an input:

```css
.button, .link {
  &:hover {
    color: blue;
  }
}
```

the parser should emit two style rules:

```text
.button:hover { color: blue; }
.link:hover { color: blue; }
```

For an input:

```css
.card {
  @media (min-width: 600px) {
    color: black;

    > .title {
      color: blue;
    }
  }
}
```

the parser should emit one `CssRule::Media` containing flattened rules equivalent to:

```text
.card { color: black; }
.card > .title { color: blue; }
```

Nested style rules inside top-level `@media` and `@container` should flatten inside the containing group, not escape to the top level.

## File Map

- `plans/2026-07-04-css-keyframes-and-nesting.md`
  - This plan.
- `src/syntax.rs`
  - Add `CssRule::Keyframes`, keyframe rule models, keyframe selector models, keyframe name string model, animation-name string variant, accessors, checked constructors.
  - Add crate-visible selector composition helpers only if they fit better on the selector model than in parser code.
- `src/parser/keyframes.rs`
  - New parser module for `@keyframes` prelude, keyframe selector lists, keyframe blocks, and keyframe declaration blocks.
- `src/parser/nesting.rs`
  - New parser module for style rule block parsing and nesting flattening.
  - Own internal nested selector parsing and parent-selector composition.
- `src/parser/selectors.rs`
  - Add narrow parser helpers for nested selector pieces if `nesting.rs` cannot reuse existing parsing without duplication.
  - Keep functional pseudo-class argument parsing restricted.
- `src/parser/timing.rs`
  - Extend animation-name parsing to accept `CssKeyframesString`.
  - Extend animation shorthand parsing to accept string animation names.
- `src/parser/mod.rs`
  - Wire `keyframes` and `nesting` modules into `StrictRuleParser`.
  - Replace style rule declaration-only block parsing with the flattening style block parser.
- `src/tests.rs`
  - Add keyframes model/parser tests, nesting flattening tests, strict rejection tests, and public API inspection tests.
- `README.md`
  - Document that keyframes are parsed as authored animation syntax and are not evaluated.
  - Document that CSS nesting is flattened by this crate as syntax sugar while preserving source order.

## Task 1: Model Keyframes Syntax

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/tests.rs`

- [ ] Check status before work:

```sh
git status --short --branch
```

Expected: clean except previous committed work, on `main`.

- [ ] Add failing tests for keyframe constructors and accessors in `src/tests.rs`:

```rust
#[test]
fn keyframes_rule_accessors_expose_authored_structure() {
    let name = CssKeyframesName::Ident(CssCustomIdent::new("fade"));
    let selector = CssKeyframeSelectorList::try_new(vec![CssKeyframeSelector::From]).unwrap();
    let declaration = CssDeclaration::new(
        CssProperty::Opacity,
        CssValue::Opacity(CssOpacity::try_new(0.0).unwrap()),
        CssSourceLocation::new(1, 1),
    );
    let block = CssKeyframeBlock::try_new(
        selector,
        vec![declaration.clone()],
        CssSourceLocation::new(2, 3),
    )
    .unwrap();
    let rule = CssKeyframesRule::try_new(
        name,
        vec![block],
        CssSourceLocation::new(1, 1),
    )
    .unwrap();

    assert_eq!(rule.name(), &CssKeyframesName::Ident(CssCustomIdent::new("fade")));
    assert_eq!(rule.location(), CssSourceLocation::new(1, 1));
    let [block] = rule.blocks() else {
        panic!("expected one keyframe block");
    };
    assert_eq!(block.location(), CssSourceLocation::new(2, 3));
    assert_eq!(block.selectors().selectors(), &[CssKeyframeSelector::From]);
    assert_eq!(block.declarations(), &[declaration]);
    assert_eq!(CssKeyframeSelector::From.offset().value().value(), 0.0);
    assert_eq!(CssKeyframeSelector::To.offset().value().value(), 100.0);
}

#[test]
fn keyframes_constructors_reject_invalid_states() {
    let location = CssSourceLocation::new(1, 1);
    let name = CssKeyframesName::Ident(CssCustomIdent::new("fade"));
    let declaration = CssDeclaration::new(
        CssProperty::Opacity,
        CssValue::Opacity(CssOpacity::try_new(1.0).unwrap()),
        location,
    );
    let from = CssKeyframeSelectorList::try_new(vec![CssKeyframeSelector::From]).unwrap();

    assert_eq!(CssKeyframesString::try_new(""), None);
    assert_eq!(CssKeyframesString::try_new("   "), None);
    assert_eq!(CssKeyframePercent::try_new(-0.1), None);
    assert_eq!(CssKeyframePercent::try_new(100.1), None);
    assert_eq!(CssKeyframePercent::try_new(f32::NAN), None);
    assert_eq!(CssKeyframeSelectorList::try_new(Vec::new()), None);
    assert_eq!(
        CssKeyframeSelectorList::try_new(vec![
            CssKeyframeSelector::From,
            CssKeyframeSelector::Percent(CssKeyframePercent::new(0.0)),
        ]),
        None
    );
    assert_eq!(CssKeyframeBlock::try_new(from.clone(), Vec::new(), location), None);
    assert_eq!(CssKeyframesRule::try_new(name.clone(), Vec::new(), location), None);

    let duplicate_a = CssKeyframeBlock::try_new(from.clone(), vec![declaration.clone()], location)
        .unwrap();
    let duplicate_b = CssKeyframeBlock::try_new(from, vec![declaration], location).unwrap();
    assert_eq!(
        CssKeyframesRule::try_new(name, vec![duplicate_a, duplicate_b], location),
        None
    );
}
```

- [ ] Run the focused tests and verify they fail because the keyframe types do not exist yet:

```sh
cargo test -p surgeist-css keyframes_
```

Expected: compile failure naming missing keyframe types or variants.

- [ ] Implement keyframe model types and accessors in `src/syntax.rs`:
  - Add `CssRule::Keyframes(CssKeyframesRule)`.
  - Add the keyframe structs/enums listed in "Target Keyframes Model".
  - Keep fields private.
  - Add checked constructors exactly for the invariants listed in "Modeling Rules".
  - Use `CssFiniteNumber` inside `CssKeyframePercent`.
  - Implement duplicate offset checks by comparing `CssKeyframeSelector::offset().value().value()`.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css keyframes_rule_accessors
cargo test -p surgeist-css keyframes_constructors
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Worker reports changed files, tests run, and `git status --short --branch`.
- [ ] Assign a separate reviewer for this task.
- [ ] Coordinator commits after review is clean:

```sh
git add src/syntax.rs src/tests.rs
git commit -m "Model CSS keyframes rules"
```

## Task 2: Parse Strict Keyframes Rules

**Files:**
- Create: `src/parser/keyframes.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/parser/timing.rs`
- Modify: `src/tests.rs`
- Modify: `README.md`

- [ ] Check status before work:

```sh
git status --short --branch
```

- [ ] Add parser tests in `src/tests.rs`:

```rust
fn keyframes_rule(rule: &CssRule) -> &CssKeyframesRule {
    match rule {
        CssRule::Keyframes(rule) => rule,
        unexpected => panic!("expected keyframes rule, got {unexpected:?}"),
    }
}

#[test]
fn keyframes_rule_parser_accepts_strict_blocks() {
    let sheet = parse_sheet(
        r#"@keyframes fade {
            from { opacity: 0; transform: translateX(0px); }
            50% { opacity: 0.5; }
            to { opacity: 1; transform: translateX(10px); }
        }"#,
    )
    .unwrap();
    let [rule] = sheet.rules() else {
        panic!("expected one keyframes rule");
    };
    let rule = keyframes_rule(rule);

    assert_eq!(rule.name(), &CssKeyframesName::Ident(CssCustomIdent::new("fade")));
    assert_eq!(rule.blocks().len(), 3);
    assert_eq!(
        rule.blocks()[0].selectors().selectors(),
        &[CssKeyframeSelector::From]
    );
    assert_eq!(
        rule.blocks()[1].selectors().selectors(),
        &[CssKeyframeSelector::Percent(CssKeyframePercent::new(50.0))]
    );
    assert_eq!(rule.blocks()[0].declarations()[0].property(), &CssProperty::Opacity);
}

#[test]
fn keyframes_rule_parser_accepts_string_names_and_selector_lists() {
    let sheet = parse_sheet(
        r#"@keyframes "fade in" {
            0%, 100% { opacity: 1; }
        }
        .panel { animation-name: "fade in"; animation: "fade in" 120ms ease; }"#,
    )
    .unwrap();
    let [keyframes, style] = sheet.rules() else {
        panic!("expected keyframes and style rules");
    };

    assert_eq!(
        keyframes_rule(keyframes).name(),
        &CssKeyframesName::String(CssKeyframesString::new("fade in"))
    );
    assert_eq!(
        keyframes_rule(keyframes).blocks()[0].selectors().selectors(),
        &[
            CssKeyframeSelector::Percent(CssKeyframePercent::new(0.0)),
            CssKeyframeSelector::Percent(CssKeyframePercent::new(100.0)),
        ]
    );

    let declarations = style_rule(style).declarations();
    assert_eq!(declarations[0].property(), &CssProperty::AnimationName);
    assert_eq!(declarations[1].property(), &CssProperty::Animation);

    let CssValue::AnimationName(names) = declarations[0].value() else {
        panic!("expected animation-name value");
    };
    assert_eq!(
        names.names(),
        &[CssAnimationName::String(CssKeyframesString::new("fade in"))]
    );

    let CssValue::Animation(animations) = declarations[1].value() else {
        panic!("expected animation shorthand value");
    };
    assert_eq!(
        animations.items()[0].name(),
        Some(&CssAnimationName::String(CssKeyframesString::new("fade in")))
    );
}

#[test]
fn keyframes_rule_parser_accepts_keyframes_inside_conditional_groups() {
    let sheet = parse_sheet(
        r#"@media screen {
            @keyframes fade { from { opacity: 0; } to { opacity: 1; } }
        }
        @container sidebar (inline-size > 30rem) {
            @keyframes slide { 0% { transform: translateX(0px); } 100% { transform: translateX(10px); } }
        }"#,
    )
    .unwrap();

    let [media, container] = sheet.rules() else {
        panic!("expected media and container rules");
    };
    let [media_keyframes] = media_rule(media).rules() else {
        panic!("expected keyframes inside media");
    };
    let [container_keyframes] = container_rule(container).rules() else {
        panic!("expected keyframes inside container");
    };

    assert_eq!(
        keyframes_rule(media_keyframes).name(),
        &CssKeyframesName::Ident(CssCustomIdent::new("fade"))
    );
    assert_eq!(
        keyframes_rule(container_keyframes).name(),
        &CssKeyframesName::Ident(CssCustomIdent::new("slide"))
    );
}

#[test]
fn keyframes_rule_parser_rejects_invalid_blocks() {
    for css in [
        "@keyframes fade;",
        "@keyframes { from { opacity: 0; } }",
        "@keyframes none { from { opacity: 0; } }",
        "@keyframes fade { }",
        "@keyframes fade { 0 { opacity: 0; } }",
        "@keyframes fade { -1% { opacity: 0; } }",
        "@keyframes fade { 101% { opacity: 0; } }",
        "@keyframes fade { from, 0% { opacity: 0; } }",
        "@keyframes fade { from { opacity: 0; } 0% { opacity: 1; } }",
        "@keyframes fade { from { made-up: value; } }",
        "@keyframes fade { from { opacity: 0 !important; } }",
        "@keyframes fade { from { .nested { opacity: 0; } } }",
        ".panel { @keyframes fade { from { opacity: 0; } } }",
    ] {
        assert!(parse_sheet(css).is_err(), "{css} should reject");
    }
}
```

- [ ] Run the focused tests and verify they fail before implementation:

```sh
cargo test -p surgeist-css keyframes_rule_parser
```

- [ ] Create `src/parser/keyframes.rs`:
  - Parse prelude as `CssKeyframesName`.
  - Accept custom identifiers except CSS-wide/reserved animation names that `CssCustomIdent::try_new` rejects. `none` must reject as a keyframes name because existing animation-name reserves it.
  - Accept quoted strings as `CssKeyframesName::String`.
  - Parse keyframe blocks with a `QualifiedRuleParser` whose prelude is `CssKeyframeSelectorList`.
  - Parse keyframe declarations with existing strict property parsing.
  - Reject nested at-rules and nested qualified rules in keyframe blocks.
  - Reject empty declaration blocks.
  - Reject duplicate semantic offsets through `CssKeyframesRule::try_new`.
  - Reject `!important` in keyframe declarations. If the current declaration parser does not reject it, introduce a keyframe-specific declaration parser wrapper that does.

- [ ] Modify `src/parser/mod.rs`:
  - Add `mod keyframes;`.
  - Add `use keyframes::parse_keyframes_rule;`.
  - Add `StrictAtRulePrelude::Keyframes(CssKeyframesName)`.
  - Wire `@keyframes` in `parse_prelude`.
  - Return `Err(())` from `rule_without_block` for keyframes.
  - Return `CssRule::Keyframes(parse_keyframes_rule(...))` from `parse_block`.
  - Allow `@keyframes` at top level and inside `@media`/`@container`.

- [ ] Modify `src/parser/timing.rs`:
  - Allow quoted string animation names in `parse_animation_name`.
  - Preserve them as `CssAnimationName::String(CssKeyframesString)`.
  - Ensure `animation-name: "";` rejects.
  - Ensure the `animation` shorthand accepts string names and still rejects malformed or duplicate components.

- [ ] Update the existing advanced CSS surface matrix in `src/tests.rs`:
  - Remove `@keyframes fade { from { opacity: 0; } to { opacity: 1; } }` from `advanced_css_surface_matrix_rejects_unsupported_forms`.
  - Add a supported `@keyframes` example to `advanced_css_surface_matrix_accepts_supported_forms`.
  - Keep `@supports`, late `@import`, unsupported `@import supports(...)`, unsupported selector forms, and unsupported container scroll-state cases rejected.

- [ ] Update `README.md` with:

```md
Keyframes are parsed as authored `@keyframes` rules. `surgeist-css` validates keyframe names, selector offsets, and declarations, but does not evaluate animations, match animation names to rules, interpolate values, or run animation timelines.
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css keyframes_rule_parser
cargo test -p surgeist-css parses_transition_properties_and_preserves_comma_lists
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Worker reports changed files, tests run, and `git status --short --branch`.
- [ ] Assign a separate reviewer for this task.
- [ ] Coordinator commits after review is clean:

```sh
git add README.md src/parser/keyframes.rs src/parser/mod.rs src/parser/timing.rs src/syntax.rs src/tests.rs
git commit -m "Parse strict CSS keyframes rules"
```

## Task 3: Add Selector Composition Helpers For Nesting

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/tests.rs`

- [ ] Check status before work:

```sh
git status --short --branch
```

- [ ] Add focused tests in `src/tests.rs` for selector composition helpers:

```rust
#[test]
fn nesting_selector_composition_preserves_parent_and_child_structure() {
    let parent = CssSelector::Class("card".to_owned());
    let child = CssSelector::Class("title".to_owned());

    let descendant = CssSelector::combine_descendant(parent.clone(), child.clone()).unwrap();
    let CssSelector::Complex(descendant) = descendant else {
        panic!("expected descendant complex selector");
    };
    assert_eq!(descendant.first().classes(), &["card".to_owned()]);
    let [part] = descendant.rest() else {
        panic!("expected one descendant part");
    };
    assert_eq!(part.combinator(), CssSelectorCombinator::Descendant);
    assert_eq!(part.selector().classes(), &["title".to_owned()]);

    let appended = CssSelector::append_to_subject(
        parent,
        CssCompoundSelector::new(
            None,
            None,
            vec!["active".to_owned()],
            Vec::new(),
            vec![CssPseudoClass::Hover],
        ),
    )
    .unwrap();
    let CssSelector::Compound(appended) = appended else {
        panic!("expected compound selector");
    };
    assert_eq!(appended.classes(), &["card".to_owned(), "active".to_owned()]);
    assert_eq!(appended.pseudo_classes(), &[CssPseudoClass::Hover]);
}
```

- [ ] Run and verify the tests fail because helpers do not exist:

```sh
cargo test -p surgeist-css nesting_selector_composition
```

- [ ] Add crate-owned selector composition helpers in `src/syntax.rs`:
  - `CssSelector::combine_descendant(parent: CssSelector, child: CssSelector) -> Option<CssSelector>`
  - `CssSelector::combine_with_combinator(parent: CssSelector, combinator: CssSelectorCombinator, child: CssCompoundSelector) -> Option<CssSelector>`
  - `CssSelector::append_to_subject(parent: CssSelector, suffix: CssCompoundSelector) -> Option<CssSelector>`
  - Keep these helpers public only if tests and downstream public API need them; otherwise use `pub(crate)` and test through flattening. Prefer `pub(crate)` if possible.
  - Convert simple selectors into `CssCompoundSelector` or `CssComplexSelector` without string parsing.
  - For `append_to_subject`, append suffix simple selectors to the rightmost subject compound of the parent selector. Reject suffixes that contain a tag or key/id because appending those to an existing subject can create invalid compound selector states.
  - Preserve parent complex selector chains when appending or adding a child combinator.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css nesting_selector_composition
cargo test -p surgeist-css combinator_selectors_are_structurally_inspectable
cargo test -p surgeist-css attribute_selectors_are_structurally_inspectable
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Worker reports changed files, tests run, and `git status --short --branch`.
- [ ] Assign a separate reviewer for this task.
- [ ] Coordinator commits after review is clean:

```sh
git add src/syntax.rs src/tests.rs
git commit -m "Add selector composition helpers"
```

## Task 4: Flatten Nested Style Rules

**Files:**
- Create: `src/parser/nesting.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/parser/selectors.rs` only if narrow parser helper reuse is needed.
- Modify: `src/tests.rs`
- Modify: `README.md`

- [ ] Check status before work:

```sh
git status --short --branch
```

- [ ] Add tests for basic nesting flattening in `src/tests.rs`:

```rust
#[test]
fn nesting_flattens_descendant_and_parent_selectors_in_source_order() {
    let sheet = parse_sheet(
        r#".card {
            color: black;
            .title { color: blue; }
            background-color: white;
            &:hover { opacity: 0.8; }
        }"#,
    )
    .unwrap();

    let [base_before, title, base_after, hover] = sheet.rules() else {
        panic!("expected four flattened rules");
    };

    assert_eq!(style_rule(base_before).selector(), &CssSelector::Class("card".to_owned()));
    assert_eq!(style_rule(base_before).declarations()[0].property(), &CssProperty::Color);

    let CssSelector::Complex(title_selector) = style_rule(title).selector() else {
        panic!("expected descendant selector");
    };
    assert_eq!(title_selector.first().classes(), &["card".to_owned()]);
    assert_eq!(title_selector.rest()[0].combinator(), CssSelectorCombinator::Descendant);
    assert_eq!(title_selector.rest()[0].selector().classes(), &["title".to_owned()]);

    assert_eq!(style_rule(base_after).selector(), &CssSelector::Class("card".to_owned()));
    assert_eq!(style_rule(base_after).declarations()[0].property(), &CssProperty::BackgroundColor);

    let CssSelector::Compound(hover_selector) = style_rule(hover).selector() else {
        panic!("expected compound hover selector");
    };
    assert_eq!(hover_selector.classes(), &["card".to_owned()]);
    assert_eq!(hover_selector.pseudo_classes(), &[CssPseudoClass::Hover]);
}

#[test]
fn nesting_flattens_selector_lists_and_relative_combinators() {
    let sheet = parse_sheet(
        r#".button, .link {
            &.active[aria-current=true] { color: black; }
            > .icon { opacity: 1; }
        }"#,
    )
    .unwrap();

    assert_eq!(sheet.rules().len(), 4);
    for rule in sheet.rules() {
        assert!(matches!(rule, CssRule::Style(_)));
    }
}
```

- [ ] Run and verify the focused tests fail before implementation:

```sh
cargo test -p surgeist-css nesting_flattens
```

- [ ] Create `src/parser/nesting.rs`:
  - Add `parse_style_rule_block(parent_selectors: Vec<CssSelector>, input: &mut Parser) -> Result<Vec<CssRule>, ParseError<Error>>`.
  - Use a rule-body parser that accepts declarations, nested qualified rules, and nested `@media`/`@container`.
  - Preserve source order by buffering declaration runs:
    - When a declaration is parsed, append it to the current declaration buffer.
    - Before emitting a nested rule or nested group, flush the current declaration buffer into `CssRule::Style` rules for every parent selector.
    - After all items are parsed, flush any remaining declarations.
  - Reuse the existing strict declaration parsing so property validation stays in one place.
  - Nested qualified rules must produce flattened selectors from every parent selector and every nested selector.
  - Nested selector parsing should support:
    - no `&`: combine as descendant unless the nested selector starts with a combinator
    - leading `>` / `+` / `~`: combine parent with that combinator
    - leading `&` alone: parent selector
    - leading `&` plus class/attribute/pseudo-class suffix: append suffix to parent subject
    - leading `&` plus combinator chain: parent followed by the chain
  - Reject unsupported nested selectors with typed selector errors:
    - `&` not at the start
    - multiple `&`
    - `&&`
    - namespaces
    - pseudo-elements
    - column combinator `||`
    - unsupported complex pseudo-class arguments

- [ ] Modify `src/parser/mod.rs`:
  - Add `mod nesting;`.
  - Replace the style rule `parse_block` declaration-only implementation with `nesting::parse_style_rule_block(selectors, input)`.
  - Keep `StrictRuleParser::mark_non_import_top_level_rule()` behavior for top-level style rules after parsing succeeds.
  - Keep top-level and group-rule parsing strict.

- [ ] Update `README.md` with:

```md
CSS nesting is parsed as syntax sugar and flattened into ordinary style and conditional group rules while preserving source order. `surgeist-css` does not evaluate selector matches or cascade results during flattening.
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css nesting_flattens
cargo test -p surgeist-css parses_combinator_selectors
cargo test -p surgeist-css parses_attribute_selector
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Worker reports changed files, tests run, and `git status --short --branch`.
- [ ] Assign a separate reviewer for this task.
- [ ] Coordinator commits after review is clean:

```sh
git add README.md src/parser/nesting.rs src/parser/mod.rs src/parser/selectors.rs src/syntax.rs src/tests.rs
git commit -m "Flatten CSS nested style rules"
```

## Task 5: Flatten Nested Conditional Group Rules

**Files:**
- Modify: `src/parser/nesting.rs`
- Modify: `src/parser/mod.rs`
- Modify: `src/tests.rs`

- [ ] Check status before work:

```sh
git status --short --branch
```

- [ ] Add tests for nested conditional groups in style rules:

```rust
#[test]
fn nesting_flattens_media_and_container_inside_style_rules() {
    let sheet = parse_sheet(
        r#".card {
            color: black;
            @media (min-width: 600px) {
                background-color: white;
                > .title { color: blue; }
            }
            @container sidebar (inline-size > 30rem) {
                &:hover { opacity: 0.9; }
            }
        }"#,
    )
    .unwrap();

    let [base, media, container] = sheet.rules() else {
        panic!("expected base, media, and container rules");
    };
    assert_eq!(style_rule(base).selector(), &CssSelector::Class("card".to_owned()));

    let media = media_rule(media);
    let [media_base, media_title] = media.rules() else {
        panic!("expected two flattened media rules");
    };
    assert_eq!(style_rule(media_base).selector(), &CssSelector::Class("card".to_owned()));
    let CssSelector::Complex(title_selector) = style_rule(media_title).selector() else {
        panic!("expected complex title selector");
    };
    assert_eq!(title_selector.rest()[0].combinator(), CssSelectorCombinator::Child);

    let container = container_rule(container);
    let [container_hover] = container.rules() else {
        panic!("expected one flattened container rule");
    };
    let CssSelector::Compound(hover_selector) = style_rule(container_hover).selector() else {
        panic!("expected hover compound selector");
    };
    assert_eq!(hover_selector.classes(), &["card".to_owned()]);
    assert_eq!(hover_selector.pseudo_classes(), &[CssPseudoClass::Hover]);
}

#[test]
fn nesting_inside_media_and_container_stays_inside_group() {
    let sheet = parse_sheet(
        r#"@media (prefers-color-scheme: dark) {
            .card { .title { color: white; } }
        }
        @container sidebar (inline-size > 30rem) {
            .card { &:hover { opacity: 0.9; } }
        }"#,
    )
    .unwrap();

    let [media, container] = sheet.rules() else {
        panic!("expected media and container");
    };
    assert!(matches!(media, CssRule::Media(_)));
    assert!(matches!(container, CssRule::Container(_)));
    assert_eq!(media_rule(media).rules().len(), 1);
    assert_eq!(container_rule(container).rules().len(), 1);
}
```

- [ ] Run and verify tests fail before implementation:

```sh
cargo test -p surgeist-css nesting_flattens_media
cargo test -p surgeist-css nesting_inside_media
```

- [ ] Extend `src/parser/nesting.rs`:
  - Add nested at-rule support for `@media` and `@container` inside style blocks.
  - Parse their preludes with existing `parse_media_query_list` and `parse_container_prelude`.
  - Parse their blocks with the same parent selector context.
  - Emit `CssRule::Media` and `CssRule::Container` at the current output position.
  - Reject `@import`, `@font-face`, and `@keyframes` inside style blocks.

- [ ] Ensure `parse_nested_group_rules` still supports nested style rules inside group rules by using the same style block flattening path.

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css nesting_flattens_media
cargo test -p surgeist-css nesting_inside_media
cargo test -p surgeist-css media_rule_parser_accepts_nested_media_rule
cargo test -p surgeist-css container_rule_parser_accepts_unnamed_named_and_style_conditions
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Worker reports changed files, tests run, and `git status --short --branch`.
- [ ] Assign a separate reviewer for this task.
- [ ] Coordinator commits after review is clean:

```sh
git add src/parser/nesting.rs src/parser/mod.rs src/tests.rs
git commit -m "Flatten nested conditional CSS rules"
```

## Task 6: Strict Nesting And Keyframes Rejection Matrix

**Files:**
- Modify: `src/tests.rs`

- [ ] Check status before work:

```sh
git status --short --branch
```

- [ ] Add strict rejection matrix tests:

```rust
#[test]
fn keyframes_and_nesting_reject_browser_recovery_forms() {
    let rejected = [
        "@-webkit-keyframes fade { from { opacity: 0; } }",
        "@keyframes fade { 0 { opacity: 0; } }",
        "@keyframes fade { 50% { opacity: 0; } 50% { opacity: 1; } }",
        "@keyframes fade { from { @media screen { opacity: 0; } } }",
        "@keyframes fade { from { .nested { opacity: 0; } } }",
        ".card { & & { color: black; } }",
        ".card { .theme & { color: black; } }",
        ".card { && { color: black; } }",
        ".card { &::before { color: black; } }",
        ".card { svg|a { color: black; } }",
        ".card { .col || .cell { color: black; } }",
        ".card { @import url(\"theme.css\"); }",
        ".card { @font-face { font-family: Inter; src: url(\"inter.woff2\"); } }",
        ".card { @keyframes fade { from { opacity: 0; } } }",
    ];

    for css in rejected {
        assert!(parse_sheet(css).is_err(), "{css} should reject");
    }
}

#[test]
fn keyframes_and_nesting_accept_practical_surface_matrix() {
    let accepted = [
        r#"@keyframes fade { from { opacity: 0; } to { opacity: 1; } }"#,
        r#"@keyframes "fade in" { 0%, 100% { opacity: 1; } }"#,
        ".card { color: black; .title { color: blue; } }",
        ".card { &:hover { opacity: 0.9; } }",
        ".card { > .title[aria-current=true] { color: blue; } }",
        ".card { @media (min-width: 600px) { &:hover { opacity: 0.9; } } }",
        "@media screen { .card { .title { color: black; } } }",
        "@container sidebar (inline-size > 30rem) { .card { &:hover { opacity: 1; } } }",
    ];

    for css in accepted {
        assert!(parse_sheet(css).is_ok(), "{css} should parse");
    }
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css keyframes_and_nesting
cargo test -p surgeist-css strict_no_recovery
cargo test -p surgeist-css advanced_css_surface_matrix
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Worker reports changed files, tests run, and `git status --short --branch`.
- [ ] Assign a separate reviewer for this task.
- [ ] Coordinator commits after review is clean:

```sh
git add src/tests.rs
git commit -m "Add keyframes and nesting strictness tests"
```

## Task 7: Final Public API And Flattening Review Tests

**Files:**
- Modify: `src/tests.rs`

- [ ] Check status before work:

```sh
git status --short --branch
```

- [ ] Add a public API inspection test:

```rust
#[test]
fn keyframes_and_flattened_nesting_are_structurally_accessible() {
    let sheet = parse_sheet(
        r#"@keyframes fade {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        .card {
            color: black;
            &:hover { opacity: 0.9; }
            @media (min-width: 600px) {
                > .title { color: blue; }
            }
        }"#,
    )
    .unwrap();

    let [keyframes, base, hover, media] = sheet.rules() else {
        panic!("expected keyframes and flattened style output");
    };

    let CssRule::Keyframes(keyframes) = keyframes else {
        panic!("expected keyframes rule");
    };
    assert_eq!(keyframes.name(), &CssKeyframesName::Ident(CssCustomIdent::new("fade")));
    assert_eq!(keyframes.blocks().len(), 2);
    assert_eq!(
        keyframes.blocks()[0].selectors().selectors(),
        &[CssKeyframeSelector::From]
    );

    assert_eq!(style_rule(base).selector(), &CssSelector::Class("card".to_owned()));

    let CssSelector::Compound(hover_selector) = style_rule(hover).selector() else {
        panic!("expected flattened hover selector");
    };
    assert_eq!(hover_selector.classes(), &["card".to_owned()]);
    assert_eq!(hover_selector.pseudo_classes(), &[CssPseudoClass::Hover]);

    let CssRule::Media(media) = media else {
        panic!("expected media rule");
    };
    let [nested] = media.rules() else {
        panic!("expected one nested flattened rule");
    };
    let CssSelector::Complex(selector) = style_rule(nested).selector() else {
        panic!("expected complex nested title selector");
    };
    assert_eq!(selector.rest()[0].combinator(), CssSelectorCombinator::Child);
}
```

- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css structurally_accessible
cargo test -p surgeist-css keyframes_and_flattened_nesting
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Worker reports changed files, tests run, and `git status --short --branch`.
- [ ] Assign a separate reviewer for this task.
- [ ] Coordinator commits after review is clean:

```sh
git add src/tests.rs
git commit -m "Add keyframes and nesting API tests"
```

## Task 8: Final Checks And Holistic Review

**Files:**
- Inspect all changed files.

- [ ] Run:

```sh
git status --short --branch
git diff --stat origin/main..HEAD
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

- [ ] Assign a clean-context holistic reviewer with no conversation history. Reviewer must inspect:
  - `AGENTS.md`
  - `guidance/surgeist-rust-modeling-guide.md`
  - this plan
  - `src/syntax.rs`
  - `src/parser/mod.rs`
  - `src/parser/keyframes.rs`
  - `src/parser/nesting.rs`
  - `src/parser/selectors.rs`
  - `src/parser/timing.rs`
  - `src/tests.rs`
  - `README.md`

- [ ] Reviewer must verify:
  - `@keyframes` is implemented as authored typed CSS syntax
  - keyframe names, selector lists, offsets, blocks, and declarations are strict and inspectable
  - string keyframe names are supported consistently in `@keyframes`, `animation-name`, and `animation`
  - CSS nesting is flattened in this crate
  - flattening preserves source order by splitting declaration runs around nested items
  - nested style rules inside `@media` and `@container` remain inside their group
  - nested `@media` and `@container` inside style rules carry parent selector context
  - unsupported at-rules and selector forms still reject
  - no browser recovery was added
  - no animation evaluation, selector matching, cascade computation, query evaluation, resource loading, or font activation was added
  - public APIs are typed and inspectable
  - modeling follows `guidance/surgeist-rust-modeling-guide.md`

- [ ] Completion requires the holistic reviewer to return `APPROVED` with no unresolved findings.

## Completion Signal

Report:

- plan commit SHA
- implementation commit SHAs
- final test count from `cargo test -p surgeist-css`
- checks run
- final holistic reviewer result
- whether the repo is pushed

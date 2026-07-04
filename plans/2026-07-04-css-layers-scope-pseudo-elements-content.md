# CSS Layers, Scope, Pseudo-Elements, Lists, Counters, And Content Plan

## Goal

Implement strict authored CSS syntax support for `@layer`, `@scope`,
`::selection`, `::before`, `::after`, `::marker`, `::backdrop`, and useful
list/counter/content properties in `surgeist-css`.

Completion requires task-scoped worker/reviewer cycles, committed logical
points, passing focused crate checks, and a final clean-context holistic review
against `guidance/surgeist-rust-modeling-guide.md`.

## References

- CSS Cascading and Inheritance Level 5 defines cascade layers and the
  `@layer` statement/block forms.
  <https://www.w3.org/TR/css-cascade-5/>
- CSS Cascading and Inheritance Level 6 defines scoped styles and the `@scope`
  syntax as an authored rule with optional scope-start and scope-end selector
  lists.
  <https://www.w3.org/TR/css-cascade-6/>
- CSS Pseudo-Elements Level 4 defines pseudo-elements as authored selector
  syntax using `::name`, including `::before`, `::after`, `::marker`, and
  `::selection`.
  <https://www.w3.org/TR/css-pseudo-4/>
- CSS Generated Content Level 3 and CSS Lists and Counters Level 3 define the
  content, list, and counter syntax this crate should model symbolically.
  <https://www.w3.org/TR/css-content-3/>
  <https://www.w3.org/TR/css-lists-3/>

## Scope Rules

- Stay inside this crate.
- Coordinator does not write implementation code.
- Workers do not commit.
- No branches.
- Use one implementation worker per scoped task or tightly coupled task group.
- Use a separate reviewer for each worker change before committing.
- Commit each clean scoped task as a logical point.
- Do not expose internals merely for external tests. Public API exists only
  where downstream integration needs an intentional typed front door.
- Keep all new syntax authored/parser-facing. Do not add runtime cascade,
  selector matching, scope matching, counter evaluation, generated-content
  layout, or renderer semantics.
- Preserve strict parsing. Invalid or unsupported CSS must reject the whole
  sheet rather than browser-recovering around bad rules or declarations.
- Follow `guidance/surgeist-rust-modeling-guide.md`: use typed semantic
  structures, private fields with constructors/accessors, narrow phase-honest
  APIs, and symbolic values for work that belongs to later phases.

## Existing Shape

- `CssRule` already models `Import`, `FontFace`, `Keyframes`, `Style`, `Media`,
  and `Container`.
- `CssLayerName` already exists for `@import layer(...)` and validates dotted
  identifier components.
- `CssSelector`, `CssCompoundSelector`, and `CssComplexSelector` already model
  compound and complex selectors, attributes, and pseudo-classes.
- `parse_nested_group_rules` parses nested group-rule blocks, and
  `parse_style_rule_block` flattens native CSS nesting.
- `CssProperty`/`CssValue` are strict property-specific authored syntax; new
  list/counter/content support should extend that pattern instead of using
  opaque strings.

## Modeling Decisions

### Cascade Layers

Add two authored rule shapes rather than overloading one type:

```rust
pub enum CssRule {
    LayerStatement(CssLayerStatementRule),
    LayerBlock(CssLayerBlockRule),
    // existing variants...
}

pub struct CssLayerStatementRule {
    names: CssLayerNameList,
    location: CssSourceLocation,
}

pub struct CssLayerBlockRule {
    name: Option<CssLayerName>,
    rules: Vec<CssRule>,
    location: CssSourceLocation,
}
```

`CssLayerNameList` is a non-empty list of `CssLayerName`.

This crate records layer declarations and layer-contained rules only. It must
not compute cascade order or declaration precedence.

Strict syntax support:

- Accept `@layer reset, theme.components;`.
- Accept `@layer theme { ... }`.
- Accept anonymous `@layer { ... }`.
- Accept nested `@layer` blocks in top-level, conditional group rules, scope
  rules, and nested style-rule conditional blocks where the existing parser
  accepts nested group rules.
- Reject empty statement lists, empty layer-name components, trailing commas,
  invalid names, blocks on statement-only forms, and semicolon-only forms with
  no layer names.
- Treat top-level layer statements and blocks as non-import rules for the
  existing `@import` ordering rule.

### Scoped Styles

Add an authored scope rule:

```rust
pub enum CssRule {
    Scope(CssScopeRule),
    // existing variants...
}

pub struct CssScopeRule {
    root: Option<CssScopeSelectorList>,
    limit: Option<CssScopeSelectorList>,
    rules: CssScopedRuleList,
    location: CssSourceLocation,
}
```

`CssScopeSelectorList` is a non-empty selector list wrapper for scope
boundaries. Reuse the existing strict selector parser with a boundary policy:
compound/complex selectors, attributes, and pseudo-classes are allowed, but
pseudo-elements are invalid in scope roots and limits.

Scoped rule blocks need their own typed rule-list boundary so scoped-only
relative selectors cannot be constructed as ordinary top-level `CssRule`
values:

```rust
pub struct CssScopedRuleList {
    rules: Vec<CssScopedRule>,
}

pub enum CssScopedRule {
    Style(CssScopedStyleRule),
    Media(CssScopedMediaRule),
    Container(CssScopedContainerRule),
    LayerStatement(CssScopedLayerStatementRule),
    LayerBlock(CssScopedLayerBlockRule),
    Scope(CssScopeRule),
}

pub struct CssScopedStyleRule {
    selectors: CssScopedStyleSelectorList,
    declarations: Vec<CssDeclaration>,
}

pub struct CssScopedStyleSelectorList {
    selectors: Vec<CssScopedStyleSelector>,
}

pub enum CssScopedStyleSelector {
    Selector(CssSelector),
    Relative(CssRelativeSelector),
}
```

`CssScopedStyleSelector` represents an item from the scoped
`<relative-selector-list>`. Selectors without an explicit leading combinator
are stored as `Selector(CssSelector)`. Selectors with an explicit leading
combinator are stored as `Relative(CssRelativeSelector)`. Do not rewrite
leading-combinator selectors into `:scope`-rooted selectors in this crate; that
would lose authored syntax and move scope interpretation into the parser.
Add `CssPseudoClass::Scope` for authored `:scope` and preserve it as an
ordinary pseudo-class. Add a scoped selector simple-selector marker for authored
`&` that can appear inside scoped compound/complex selectors, such as
`& > .x` or `main & p`, without rewriting it to `:scope`. This crate does not
evaluate either form against scope roots.

Scoped variants of media/container/layer rules mirror their ordinary group-rule
shape. Scoped media/container/layer block rules carry `CssScopedRuleList`
children. Scoped layer statements carry a `CssLayerNameList` and source
location so valid layer-ordering declarations inside scope blocks remain typed
without leaking scoped-only rules into top-level `CssRule` values.

Top-level and ordinary non-scoped nested rules must continue rejecting
leading-combinator relative style selectors unless they are part of existing CSS
nesting syntax.

Strict syntax support:

- Accept `@scope (.card) { ... }`.
- Accept `@scope (.card, [data-scope]) to (.stop, .boundary) { ... }`.
- Accept `@scope to (.stop) { ... }`.
- Accept `@scope { ... }` as spec-authored syntax with no explicit root.
- Accept relative scoped style selectors inside the block, such as
  `@scope (.card) { > .label { color: red; } }`, as typed
  `CssScopedStyleRule` values without flattening them.
- Accept scoped selector lists that mix implicit and explicit scope-relative
  items, such as `@scope (.card) { .title, > .action { color: red; } }`.
- Reject missing parentheses around explicit roots/limits, empty selector
  lists, malformed selectors, dangling `to`, and extra tokens after the limit.
- Reject pseudo-elements in scope roots and limits, e.g.
  `@scope (.card::before) { ... }`, while allowing pseudo-elements in style
  selectors inside the block after Task 4 adds pseudo-element parsing.
- Model scoping roots and limits only. Do not evaluate whether a rule matches
  inside scope, and do not compute scoping proximity.

### Pseudo-Elements

Add typed pseudo-elements:

```rust
pub enum CssPseudoElement {
    Before,
    After,
    Marker,
    Selection,
    Backdrop,
}
```

Attach pseudo-elements to `CssCompoundSelector` as an optional terminal field:

```rust
pub struct CssCompoundSelector {
    // existing fields...
    pseudo_elements: Option<CssPseudoElementSequence>,
}

pub struct CssPseudoElementSequence {
    pseudo_elements: Vec<CssPseudoElement>,
}
```

This keeps the model honest: pseudo-elements are not pseudo-classes and cannot
float as arbitrary selector nodes. A sequence wrapper avoids baking in the false
general invariant that CSS pseudo-elements can never chain, while still making
the supported chains explicit and guarded by constructors.

Strict syntax support:

- Accept `::before`, `::after`, `::marker`, `::selection`, and `::backdrop`.
- Accept pseudo-elements after ordinary compound selector content, e.g.
  `.button.primary:hover::before`, `li[data-kind="task"]::marker`, and
  `dialog::backdrop`.
- Accept pseudo-elements inside complex selectors only as the terminal simple
  selector for the whole complex selector. For this strict pass,
  `.a::before .b`, `.a::before > .b`, and any later compound after a
  pseudo-element must reject.
- Accept only these pseudo-element sequence shapes:
  - single `::before`, `::after`, `::marker`, `::selection`, or `::backdrop`;
  - `::before::marker`;
  - `::after::marker`.
- Reject single-colon pseudo-element spellings such as `:before`.
- Reject unknown pseudo-elements.
- Reject pseudo-classes, attributes, classes, ids, tags, or any unsupported
  pseudo-element continuation after a pseudo-element in the same compound.
- Reject any pseudo-element chain other than the explicitly supported
  `::before::marker` and `::after::marker` forms.

The crate does not restrict declarations by pseudo-element. That selector-aware
property filtering belongs to a future style/adapter validation phase if needed.

### Lists, Counters, And Content

Add strict property support for:

- `content`
- `list-style-type`
- `list-style-position`
- `list-style-image`
- `list-style`
- `counter-reset`
- `counter-increment`
- `counter-set`

Add typed authored syntax:

```rust
pub enum CssContent {
    Normal,
    None,
    Items(CssContentList),
}

pub struct CssContentList {
    items: Vec<CssContentItem>,
}

pub enum CssContentItem {
    String(CssContentString),
    Url(CssUrl),
    Counter(CssCounterFunction),
    Counters(CssCountersFunction),
    Attr(CssAttributeName),
    OpenQuote,
    CloseQuote,
    NoOpenQuote,
    NoCloseQuote,
}

pub struct CssCounterFunction {
    name: CssCounterName,
    style: Option<CssCounterStyle>,
}

pub struct CssCountersFunction {
    name: CssCounterName,
    separator: CssContentString,
    style: Option<CssCounterStyle>,
}

pub enum CssCounterStyle {
    BuiltIn(CssBuiltInCounterStyle),
    Named(CssCustomIdent),
}

pub enum CssBuiltInCounterStyle {
    Disc,
    Circle,
    Square,
    Decimal,
    DecimalLeadingZero,
    LowerAlpha,
    UpperAlpha,
    LowerLatin,
    UpperLatin,
    LowerRoman,
    UpperRoman,
}

pub enum CssListStyleType {
    None,
    CounterStyle(CssCounterStyle),
    String(CssContentString),
}

pub enum CssListStylePosition {
    Inside,
    Outside,
}

pub enum CssListStyleImage {
    None,
    Url(CssUrl),
}

pub struct CssListStyle {
    style_type: Option<CssListStyleType>,
    position: Option<CssListStylePosition>,
    image: Option<CssListStyleImage>,
}

pub enum CssCounterChanges {
    None,
    Changes(CssCounterChangeList),
}

pub struct CssCounterChangeList {
    changes: Vec<CssCounterChange>,
}

pub struct CssCounterChange {
    name: CssCounterName,
    value: Option<i32>,
}
```

Use private fields and public accessors. Constructors should reject empty lists,
invalid names, and invalid strings. `CssCounterName` should be a custom-ident
newtype that rejects CSS-wide keywords and the `<counter-name>` exclusion
`none`; do not broadly reject names such as `list-item` unless the syntax
definition specifically excludes them.

Strict value support:

- `content`: accept only `normal`, `none`, or a non-empty sequence of supported
  content items. Reject mixed keyword/list forms like `none "x"`.
- Content items: string literals, `url(...)`, `counter(name)`,
  `counter(name, style)`, `counters(name, "separator")`,
  `counters(name, "separator", style)`, `attr(name)`, `open-quote`,
  `close-quote`, `no-open-quote`, and `no-close-quote`.
- Explicitly reject valid-but-unsupported generated-content forms in this pass,
  including slash alternative text, `contents`, gradients/image-set and other
  non-URL image functions, target-counter functions, leaders, and quotes
  property integration. These must not be accepted as opaque strings.
- Counter styles: accept the built-ins above plus valid named identifiers.
  Reject CSS-wide keywords, `none` where it is not valid, malformed functions,
  missing separators, extra function arguments, and non-identifier counter
  names.
- `list-style-type`: accept `none`, built-in/custom counter styles, or a string
  marker.
- `list-style-position`: accept `inside | outside`.
- `list-style-image`: accept `none | url(...)`.
- `list-style`: follow the existing shorthand modeling style used by `border`
  and `outline`: store typed optional slots and reject an all-empty shorthand.
  Accept one optional type, one optional position, and one optional image in any
  order. Resolve the grammar ambiguity for `none` only within the shorthand
  parser after considering the other authored components: if `url(...)` is
  present and no explicit style type is present, `none` fills `style_type`; if
  an explicit list style type is present and no image is present, `none` fills
  `image`; if neither a style type nor image is present, `none` fills both
  slots, including forms such as `list-style: none` and
  `list-style: none inside`. Reject duplicate components, multi-`none` forms
  such as `list-style: none none` and `list-style: none none inside`, empty
  values, and unknown tokens.
- `counter-reset`, `counter-increment`, `counter-set`: accept `none` or a
  non-empty list of counter-name with optional integer. Preserve the optional
  integer instead of resolving property-specific defaults in the parser.
- Variable-dependent values remain accepted through the existing variable
  front door without attempting partial validation.

## Task 1: Plan Review

Coordinator actions:

- [ ] Run `git status --short --branch` before review/integration.
- [ ] Save this plan in `plans/`.
- [ ] Assign a clean-context reviewer to review only the plan against
  `AGENTS.md`, the Rust modeling guide, and the stated feature scope.
- [ ] Reconcile plan review findings before assigning implementation.
- [ ] Run `git status --short --branch`, `git diff --stat`, and review the
  detailed plan diff before committing.
- [ ] Commit the reviewed plan.

Plan reviewer prompt:

Review `plans/2026-07-04-css-layers-scope-pseudo-elements-content.md` only.
Check that it follows `AGENTS.md`, stays inside `surgeist-css`, keeps authored
CSS syntax separate from runtime cascade/scope/counter evaluation, and follows
`guidance/surgeist-rust-modeling-guide.md`. Report blockers first. Do not edit
files.

## Shared Task Execution Preamble

For every implementation task below, the coordinator must:

- [ ] Run `git status --short --branch` before assigning the worker.
- [ ] Identify the crate name from `Cargo.toml` and read `README.md` before
  assigning the first worker if that has not already been done in the current
  coordinator context.
- [ ] Confirm the scoped work belongs in `surgeist-css` and does not require
  sibling crate edits.
- [ ] Give the worker only the scoped prompt, relevant files, commands, and
  constraints. Tell the worker they are not alone in the codebase, must not
  revert others' work, and must adapt to any existing changes they encounter.
- [ ] Wait for the worker result and reported status/checks.
- [ ] Assign a separate clean-context reviewer for that scoped change.
- [ ] Reconcile reviewer findings through follow-up worker/reviewer cycles.
- [ ] Run `git status --short --branch`, `git diff --stat`, and review the
  relevant detailed diff before committing the logical point.

Scoped implementation reviewer prompt:

Review only the scoped worker diff for the current task. Check the diff against
this plan, `AGENTS.md`, strict whole-sheet parsing, crate boundaries, test
coverage, and `guidance/surgeist-rust-modeling-guide.md`. Report blockers
first. Do not edit files.

## Task 2: Model Layer And Scope Rules

Worker scope:

- Add `CssRule` variants and syntax types for `@layer` statement/block and
  `@scope`.
- Reuse `CssLayerName` and add `CssLayerNameList`.
- Add `CssScopeSelectorList`.
- Add `CssScopedRuleList`, `CssScopedRule`, `CssScopedStyleRule`,
  `CssScopedStyleSelectorList`, and `CssScopedStyleSelector` so scoped blocks
  can model relative selectors without weakening top-level style-rule parsing or
  relying on a global scoped-only `CssRule` variant.
- Provide accessors and constructor guards.
- Update test helpers that exhaustively match `CssRule`.
- Add focused model/parser tests where possible without completing every parser
  branch. Pseudo-element-in-scope coverage is intentionally deferred until Task
  4 adds pseudo-elements to the shared selector parser.

Reviewer scope:

- Check for phase honesty, constructor/accessor quality, naming, and public API
  shape.
- Ensure no cascade/scope runtime behavior was added.

Checks before commit:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Commit message:

```text
Model CSS layer and scope rules
```

## Task 3: Parse `@layer` And `@scope`

Worker scope:

- Extend `StrictAtRulePrelude` and `StrictRuleParser` to parse `@layer` and
  `@scope`.
- Support top-level and nested group-rule blocks.
- Parse `@scope` blocks with a scoped rule parser that accepts existing normal
  selector syntax and leading-combinator relative selectors as
  `CssScopedStyleRule`, including comma-separated scoped selector lists, while
  rejecting relative selectors outside scoped blocks.
- Parse `@media`, `@container`, nested `@layer` statements/blocks, and nested
  `@scope` inside scoped blocks into `CssScopedRule` variants so scope does not
  get lost across nested group rules.
- Parse `@scope` roots/limits with a selector policy that rejects
  pseudo-elements.
- Extend `NestedStyleRuleParser` so nested style blocks can contain `@layer`
  and `@scope` group rules while preserving existing nesting flattening.
- Keep `@import` ordering strict.
- Add strict accept/reject tests for statement/block layers, anonymous layer
  blocks, scope roots/limits, limit-only scope, relative scoped style rules,
  scoped `&`, authored `:scope`, nested rules, and malformed syntax.

Reviewer scope:

- Check strictness: no browser recovery, no silent rule dropping, no accidental
  broad `@` rule support.
- Check source locations and nested rule order.

Checks before commit:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Commit message:

```text
Parse CSS layer and scope rules
```

## Task 4: Model And Parse Pseudo-Elements

Worker scope:

- Add `CssPseudoElement` and terminal pseudo-element storage/accessors on
  `CssCompoundSelector`.
- Add `CssPseudoElementSequence` with constructor guards for exactly the
  supported single-pseudo-element and generated-marker chain forms.
- Update selector conversion/composition helpers so nesting and complex
  selectors preserve pseudo-elements.
- Require `CssComplexSelector` constructors and composition helpers to reject
  any selector part after a compound with pseudo-elements, so terminal
  pseudo-element invariants are enforced by the model as well as by parsing.
- Parse the five requested double-colon pseudo-elements.
- Enforce terminal-only pseudo-element sequences. Accept `::before::marker` and
  `::after::marker`; reject all other chains.
- Update tests for simple, compound, complex, functional-pseudo argument, and
  nested-selector cases.
- Functional pseudo-class selector arguments must reject pseudo-elements in
  this pass. Add rejection tests for `:is(::before)`, `:where(.x::after)`,
  `:not(::marker)`, and `:has(::backdrop)` unless a later plan explicitly adds
  pseudo-element-bearing functional selector arguments.
- Add an integration test proving the shared selector parser allows
  pseudo-element style selectors inside `@scope` rule blocks, such as
  `@scope (.card) { .label::before { color: red; } }`.
- Add a rejection test proving pseudo-elements remain invalid in `@scope`
  roots/limits, such as `@scope (.card::before) { .label { color: red; } }`.

Reviewer scope:

- Check that pseudo-elements are not modeled as pseudo-classes.
- Check that invalid ordering is rejected and no selector internals are exposed
  just for tests.

Checks before commit:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Commit message:

```text
Add strict pseudo-element selectors
```

## Task 5: Model List, Counter, And Content Values

Worker scope:

- Add `CssProperty` variants and `CssValue` variants for the listed
  properties.
- Add typed syntax models for content lists, content items, counter names,
  counter functions, counter styles, list-style components, list-style
  shorthand, and counter change lists.
- Use private fields, `try_new` constructors, and accessors.
- Reuse existing `CssUrl`, `CssAttributeName`, and `CssCustomIdent` where they
  are semantically correct.
- Add unit tests for constructor guards and value shape accessors.

Reviewer scope:

- Check for broad bags or stringly typed modeling.
- Check names against the Rust modeling guide.
- Check that defaults are preserved as authored optional fields rather than
  prematurely resolved.

Checks before commit:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Commit message:

```text
Model CSS content and counter values
```

## Task 6: Parse List, Counter, And Content Values

Worker scope:

- Add a parser module for content/list/counter values.
- Extend the supported-property registry and declaration parser.
- Add accepted and rejected tests for every new longhand/shorthand and function
  form.
- Include strict leakage tests:
  - `content: normal "x"` rejects.
  - `content: counter()` rejects.
  - `content: counters(item)` rejects.
  - `content: attr()` rejects.
  - `content: "x" / "alt"` rejects.
  - `content: contents` rejects.
  - `content: linear-gradient(red, blue)` rejects.
  - `content: target-counter(attr(href), page)` rejects.
  - `list-style-position: center` rejects.
  - `list-style-image: red` rejects.
  - `list-style: inside outside` rejects.
  - `list-style: none none` rejects.
  - `list-style: none none inside` rejects.
  - `list-style: none inside` accepts and exposes both type-none, image-none,
    and inside position.
  - `list-style-image: linear-gradient(red, blue)` rejects.
  - `list-style: symbols(cyclic "*" "+") inside` rejects.
  - `content: counter(item, symbols(cyclic "*" "+"))` rejects.
  - `counter-reset: none item` rejects.
  - `counter-increment: 1` rejects.
  - `counter-set: inherit 1` rejects unless parsed as the existing global
    keyword alone.

Reviewer scope:

- Check property-specific strict parsing and registry coverage.
- Check that variable-dependent values keep using the existing front door.

Checks before commit:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Commit message:

```text
Parse CSS content and counter properties
```

## Task 7: Documentation And Public API Tests

Worker scope:

- Update `README.md` to describe authored support and non-goals for layers,
  scope, pseudo-elements, generated content, lists, and counters.
- Add public API inspection tests proving callers can structurally inspect:
  - layer statements and blocks;
  - scope roots/limits;
  - scoped style selectors, including relative scoped selectors;
  - scoped nested group rules without seeing scoped-only rules at the top level;
  - terminal pseudo-elements in compound/complex selectors;
  - `content` lists and counter functions;
  - list-style shorthand slots;
  - counter change lists.
- Add strict whole-sheet rejection tests proving invalid rules/declarations do
  not recover around bad input.

Reviewer scope:

- Check docs are accurate, not overpromising runtime support.
- Check tests exercise public API rather than private internals.

Checks before commit:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Commit message:

```text
Document generated content syntax support
```

## Final Holistic Review

After all scoped tasks are committed:

- [ ] Assign a final clean-context reviewer with only the final diff, plan,
  `AGENTS.md`, and `guidance/surgeist-rust-modeling-guide.md`.
- [ ] Reviewer must inspect naming, Rust modeling, strictness, API boundaries,
  tests, docs, and crate scope.
- [ ] Reconcile all reviewer findings through worker/reviewer cycles if needed.
- [ ] Run final checks:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
git status --short --branch
```

Do not declare the goal complete until the final holistic review is clean and
all final checks pass.

# Validated Known-Unsupported CSS Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement validated parser support for every property name and length unit currently listed as known-unsupported in `src/validation.rs`.

**Architecture:** `surgeist-css` owns authored CSS syntax and parse-time validation only. Support means a known item is removed from the known-unsupported registry only after the parser accepts an intentionally modeled property-specific syntax surface and rejects values that belong to a different property family. Do not turn `CssValue` into a broad cross-property bag; introduce small authored syntax types for property families where they preserve invariants.

**Tech Stack:** Rust 2024, `cssparser = 0.37.0`, crate-local tests in `src/lib.rs`, coordinator workflow from `AGENTS.md`, modeling guidance from `guidance/surgeist-rust-modeling-guide.md`.

---

## Scope Rules

- Work only in `/Users/codex/Development/surgeist-css`.
- Do not edit sibling crates or root submodule pointers.
- Workers must not commit. Coordinator commits after each clean worker/reviewer cycle.
- No branches.
- Keep successful syntax authored/parser-facing. Do not add `surgeist-style`.
- Every property in the known-unsupported registry must end in exactly one of these states:
  - parsed as a typed `CssProperty` with typed authored `CssValue` data and tests, or
  - removed from this plan only by explicit coordinator/user decision with the reason documented here.
- The completion target for this goal is no remaining `KnownUnsupported` property name or length unit in `src/validation.rs`.

## Definition Of Validated Support

For each length unit:

- `classify_length_unit` recognizes the unit as supported rather than known-unsupported.
- Plain lengths preserve the numeric value and unit as authored syntax.
- `calc(...)` components preserve the numeric value and unit as authored syntax.
- Existing `px`, `%`, `0`, `auto`, intrinsic keywords, `normal`, and calc behavior remains source-compatible where possible.
- Unknown units still produce an unknown-unit typed error.

For each property:

- `CssProperty` has an intentional variant.
- `parse_value` routes the property to a dedicated parser or a deliberately shared property-family parser.
- `property_for_supported_name` and `SUPPORTED_PROPERTY_NAMES` stay in sync.
- The property accepts global CSS keywords as whole declaration values.
- Tests cover at least one accepted value and one rejected leakage value for the property or its family.
- Shorthands and longhands preserve enough authored structure for root adapters to lower later without string parsing.

## Modeling Constraints

- Prefer small family value enums such as text alignment, border style, transition timing, background repeat, cursor, visibility, transform function, and animation direction over stuffing unrelated values into `CssLength` or `CssValue::Number`.
- A property family may share a parser only when the allowed grammar is genuinely shared. Examples: `margin-top` through `margin-left`; `padding-top` through `padding-left`; `border-top-style` through `border-left-style`.
- Do not add broad catch-all variants such as `CssValue::Tokens(Vec<String>)` for supported properties.
- Keep symbolic values symbolic. Do not resolve `em`, viewport units, container units, percentages, transforms, filters, or timing functions to pixels or numbers.
- Keep constructor invariants private when a syntax type has ordering, non-empty, range, or pairing rules.

## Task 1: Length Units

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] Add `CssLengthUnit` with variants for `Px`, font-relative units (`em`, `rem`, `ex`, `rex`, `cap`, `rcap`, `ch`, `rch`, `ic`, `ric`, `lh`, `rlh`), viewport units (`vw`, `vh`, `vi`, `vb`, `vmin`, `vmax`, `svw`, `svh`, `svi`, `svb`, `svmin`, `svmax`, `lvw`, `lvh`, `lvi`, `lvb`, `lvmin`, `lvmax`, `dvw`, `dvh`, `dvi`, `dvb`, `dvmin`, `dvmax`), container query units (`cqw`, `cqh`, `cqi`, `cqb`, `cqmin`, `cqmax`), and absolute physical units (`cm`, `mm`, `q`, `in`, `pc`, `pt`).
- [ ] Add a typed authored dimension representation for non-zero lengths in `CssLength` and `CssCalcLength`. Keep `CssLength::Px(f32)` and `CssCalcLength::Px(f32)` working or add compatibility helpers if replacing them would cause avoidable churn.
- [ ] Change `classify_length_unit` so every unit above is supported, while unknown units remain unknown.
- [ ] Parse all supported units in plain property length values and nested calc components.
- [ ] Add tests proving representative units parse in plain values and calc: `1em`, `2rem`, `3vw`, `4svh`, `5lvw`, `6dvb`, `7cqi`, `8cm`, `9pt`.
- [ ] Add tests proving unknown units still report unknown units for plain lengths and calc.
- [ ] Run `cargo test -p surgeist-css length_unit` or the closest focused subset, then report exact commands and status.

## Task 2: Spacing, Insets, Borders, Radius, Shadow

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] Add supported properties: `inset`, `top`, `right`, `bottom`, `left`, `z-index`, `box-decoration-break`, all margin longhands, all padding longhands, border shorthands/side shorthands, border width/color/style side longhands, border radius shorthand/longhands, and `box-shadow`.
- [ ] Model insets with `auto` plus the same length/percentage/calc family as box-size values.
- [ ] Model `z-index` as `auto` or integer authored syntax; reject non-integers and lengths.
- [ ] Model `box-decoration-break` as `slice | clone`.
- [ ] Reuse existing margin/padding/border-width component validation for matching longhands.
- [ ] Model border styles as a typed enum with common CSS border line styles: `none`, `hidden`, `dotted`, `dashed`, `solid`, `double`, `groove`, `ridge`, `inset`, `outset`.
- [ ] Model border shorthands without requiring browser recovery. Accept any order of width, style, and color components, require at least one component, reject duplicates, and preserve missing components as `None`.
- [ ] Model border radius values as one or two length-percentage values per corner and shorthand expansion with optional slash radii.
- [ ] Model `box-shadow` as `none` or a non-empty comma-separated list with optional `inset`, two required offsets, optional blur/spread, and optional color.
- [ ] Add accept/reject tests for each family, including leakage rejects: `padding-top: auto`, `border-width: 10%`, `border-style: 10px`, `border-color: solid`, `border-radius: auto`, `box-shadow: auto`, and `z-index: 1.5`.
- [ ] Run focused spacing/border tests, then report exact commands and status.

## Task 3: Positioning, Float/Clear, Alignment, Visibility

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] Extend `position` to include `static`, `fixed`, and `sticky` while preserving existing `relative` and `absolute`.
- [ ] Add supported properties `float`, `clear`, `align-content`, `justify-content`, `place-content`, `place-items`, `place-self`, `visibility`, and `content-visibility`.
- [ ] Model float as `left | right | none` and clear as `left | right | both | none`.
- [ ] Model content distribution/alignment with typed values. Accept currently useful CSS Box Alignment keywords: `normal`, `stretch`, `start`, `end`, `center`, `flex-start`, `flex-end`, `space-between`, `space-around`, `space-evenly`, `baseline`, `first baseline`, `last baseline`, plus existing safe alignment support where already accepted.
- [ ] Model `place-*` shorthands as one or two alignment values, preserving expansion semantics as authored syntax.
- [ ] Model visibility as `visible | hidden | collapse`.
- [ ] Model content visibility as `visible | hidden | auto`.
- [ ] Add accept/reject tests for each family, including leakage rejects: `float: center`, `clear: start`, `align-content: left`, `justify-content: auto`, `place-items: auto`, `visibility: auto`, and `content-visibility: collapse`.
- [ ] Run focused alignment/visibility tests, then report exact commands and status.

## Task 4: Grid And Flex Extras

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] Add supported grid properties: `grid-template-rows`, `grid-template-columns`, `grid-template-areas`, `grid-template`, `grid-auto-rows`, `grid-auto-columns`, `grid-auto-flow`, `grid-row-start`, `grid-row-end`, `grid-column-start`, `grid-column-end`, `grid-row`, `grid-column`, `grid-area`, and `grid`.
- [ ] Add supported flex/track properties: `order`, `flex`, `justify-tracks`, and `align-tracks`.
- [ ] Model integer grid lines, named custom idents, `span` line values, `auto`, track sizes, track lists, `repeat(...)`, `minmax(...)`, `fit-content(...)`, and `fr` units as authored syntax.
- [ ] Model `grid-template-areas` as `none` or string rows. Validate that all non-dot area tokens form rectangles.
- [ ] Model `grid-auto-flow` as `row | column` with optional `dense`.
- [ ] Model `grid-row`, `grid-column`, and `grid-area` shorthands with slash-separated authored components.
- [ ] Model `order` as an integer.
- [ ] Model `flex` as `none`, `auto`, or numeric grow/shrink plus basis using existing flex-basis validation.
- [ ] Model `justify-tracks` and `align-tracks` with the same alignment typed family used for content alignment unless local Surgeist semantics require a narrower set.
- [ ] Add accept/reject tests for representative values and leakage rejects: `order: 1.2`, `grid-auto-flow: left`, `grid-template-areas` non-rectangular areas, `grid-row: 1 / / 2`, `flex: solid`, `justify-tracks: auto`.
- [ ] Run focused grid/flex tests, then report exact commands and status.

## Task 5: Typography And Text

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] Add supported properties: `writing-mode`, `text-align`, `text-align-last`, `text-indent`, `vertical-align`, `font-family`, `font`, `font-weight`, `font-style`, `font-stretch`, `font-variant`, `font-feature-settings`, `letter-spacing`, `text-wrap`, `white-space`, `word-break`, `overflow-wrap`, `text-overflow`, `text-decoration`, `text-decoration-line`, `text-decoration-color`, `text-decoration-style`, `text-decoration-thickness`, and `text-transform`.
- [ ] Model writing mode, text alignment, wrapping, breaking, overflow, decoration, transform, font style/stretch/variant/feature settings as typed enums/structs.
- [ ] Model font families as a non-empty comma-separated list of quoted strings or ident sequences, preserving authored names.
- [ ] Model font weight as keyword or numeric 1 through 1000.
- [ ] Model font shorthand as an authored struct with optional style/variant/weight/stretch, required size, optional line-height after slash, and required family list.
- [ ] Reuse property-specific length validation for `text-indent`, `vertical-align`, `letter-spacing`, and decoration thickness; keep keyword alternatives property-specific.
- [ ] Add accept/reject tests for each family, including leakage rejects: `font-size: auto`, `font-weight: 1001`, `font-style: bold`, `font-family:`, `letter-spacing: auto`, `text-decoration-style: 2px`, and `text-transform: wrap`.
- [ ] Run focused typography/text tests, then report exact commands and status.

## Task 6: Backgrounds, Interaction, Effects, Transitions, Animations

**Files:**
- Modify: `src/syntax.rs`
- Modify: `src/validation.rs`
- Modify: `src/lib.rs`

- [ ] Add supported background properties: `background-image`, `background-position`, `background-size`, `background-repeat`, `background-origin`, `background-clip`, and `background-attachment`.
- [ ] Add supported interaction/focus properties: `cursor`, `pointer-events`, `user-select`, `outline`, `outline-color`, `outline-style`, and `outline-width`.
- [ ] Add supported transform/effect/mask properties: `transform`, `transform-origin`, `translate`, `rotate`, `scale`, `filter`, `backdrop-filter`, `clip-path`, `mask`, `mask-image`, `mask-size`, `mask-position`, and `mask-repeat`.
- [ ] Add supported transition properties: `transition-property`, `transition-duration`, `transition-delay`, `transition-timing-function`, and `transition`.
- [ ] Add supported animation properties: `animation-name`, `animation-duration`, `animation-delay`, `animation-timing-function`, `animation-iteration-count`, `animation-direction`, `animation-fill-mode`, `animation-play-state`, and `animation`.
- [ ] Model URLs as authored strings and accept `none` where CSS allows image/effect absence.
- [ ] Model positions, sizes, repeat styles, box keywords, attachment keywords, cursor keywords, pointer/user-select keywords, outline shorthands, transform functions, filter functions, basic shapes, mask shorthands, time values, easing functions, transition shorthands, and animation shorthands as typed authored syntax.
- [ ] For comma-separated transition and animation lists, preserve list structure and reject empty items.
- [ ] Add accept/reject tests for each family, including leakage rejects: `background-size: solid`, `cursor: 10px`, `pointer-events: grab`, `outline-width: 10%`, `transform: red`, `filter: 10px`, `transition-duration: 10px`, `animation-iteration-count: -1`, and `animation-play-state: visible`.
- [ ] Run focused background/interaction/effects/timing tests, then report exact commands and status.

## Task 7: Registry Exhaustion And Final Verification

**Files:**
- Modify: `src/validation.rs`
- Modify: tests in `src/lib.rs`

- [ ] Remove now-supported names from `KNOWN_UNSUPPORTED_PROPERTY_NAMES`.
- [ ] Remove now-supported units from `KNOWN_UNSUPPORTED_LENGTH_UNITS`.
- [ ] Add tests asserting the known-unsupported registries are empty or deleted.
- [ ] Add a broad parse smoke test that includes one declaration for every newly supported property.
- [ ] Run:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
git status --short --branch
```

- [ ] Coordinator assigns a final clean-context holistic reviewer against this plan, `AGENTS.md`, and `guidance/surgeist-rust-modeling-guide.md`.
- [ ] Coordinator reviews `git diff --stat` and detailed diff, commits the final logical point, and pushes if publication is needed.

# P01-I02-C03 Finite Numeric, Timing, And Typed Math Foundation

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C03` |
| Owning repository | `surgeist-css` |
| Status | `complete` |
| Cycle base | `a80ff9339f21ad041b159de72a03942ffb11ac50` |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `e290a7fef9bf6b6e9bde764140e5f7fac34156bb8f644d999e6bba58dc92ca2b`, especially P01.9 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `c6a9984521e23d5c010c3890902b70730db42eda092ad0e77f7d9e8e6168dfa1`, sections 3.1, 4.4, 8.1, 10, 11 findings 2.11/2.12, and 12.2-12.6 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `09ecbf2dcaafbd402b24642f1244ce0be3568fd8a85b993c0218e2e7c0deac6d`, the `O-VALUES3` numeric/math rows and timing-property rows |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `fb02bf326ae06414ac7b50e58d791962db9973cea3b8ae73b9a1d372276f645c`, entry `I02-C03` |
| Bounded outcome | Publish finite scalar invariants, distinct authored duration and delay domains, and typed calculation trees for the selected numeric roots; integrate them into existing numeric and timing consumers without changing any I01 accepted syntax or recovery contract. |

## 2. Boundary, Impacts, And Resolved Architecture

C02 candidate `a80ff9339f21ad041b159de72a03942ffb11ac50` is published and read back on the authority remote. Its active atomic owners and C03 reserved rows have no unresolved source or catalog finding. C03 closes findings 2.11 and 2.12 only. Positions, dedicated transform/easing/filter/shape grammars, colors, Grid, typography, and later property families remain in their ordered cycles.

The authored CSS syntax phase remains the owner. Numeric and calculation values remain symbolic; this crate neither resolves relative units nor computes layout, used values, animation timelines, or cross-crate style data. Calculation type checking uses authored dimensional categories only. It does not canonicalize units, combine unlike dimensions, or expose evaluated values.

All current scalar construction paths reject NaN and positive or negative infinity. Direct public construction of the frozen legacy `CssGridFlowTolerance` enum is the sole exception: its raw payload remains source-compatible, while parser-produced projections and the current accessor are always finite. `CssFiniteNumber` and semantic wrappers remain the primitive boundary. Unchecked crate-private construction follows equivalent validation; no input-dependent panic, `unwrap`, `expect`, or `unreachable!` is permitted.

The sole C03 legacy raw-scalar compatibility payload is `CssGridFlowTolerance::Percent(f32)`, and that public enum remains unchanged. New non-exhaustive `CssGridFlowToleranceValue::{Normal, Infinite, Length(CssLength), Percent(CssFiniteNumber)}` is the checked current model. `CssGridFlowTolerancePropertyValue` stores its parser-owned current value plus an optional legacy projection and exposes `value() -> &CssGridFlowToleranceValue`; `i01_subset()` retains its exact existing signature.

Every I01 parse has both representations. One shared checked post-`unit_value * 100.0` conversion protects ordinary and retained legacy-calc percentage paths before either representation is built. New typed length calculation syntax may use only the current value and return no I01 projection.

Timing uses separate authored domains:

- `CssDurationLiteral` is finite and non-negative; `CssDelayLiteral` is finite
  and signed. Both reuse `CssTimeUnit` and expose checked construction plus
  `value()` and `unit()`.
- non-exhaustive `CssDuration::{Literal(CssDurationLiteral),
  Calculation(CssTimeCalculation)}` and
  `CssDelay::{Literal(CssDelayLiteral), Calculation(CssTimeCalculation)}` are
  exact authored choices. Private-field `CssDurationList` and `CssDelayList`
  reject an empty list and expose `values()` slices.
- `CssAnimationIterationNumber` remains the finite non-negative literal branch;
  current non-exhaustive `CssAnimationIterationValue` has `Infinite`, `Number`,
  and `Calculation(CssNumberCalculation)` branches; private-field
  `CssAnimationIterationValueList` rejects empty and exposes `values()`.
- parser-owned `CssTransitionValue`/`CssTransitionValueList` and
  `CssAnimationValue`/`CssAnimationValueList` expose exact duration and delay
  accessors. Their private fields prevent a duration/delay swap. The first time
  in a shorthand is always a duration and the second is always a signed delay;
  a negative first time is invalid even when a later non-negative time exists.

`CssTransitionValue` exposes `property()`, `duration() -> Option<&CssDuration>`, `delay() -> Option<&CssDelay>`, and `timing_function()`. `CssAnimationValue` exposes the existing eight component names, with `duration() -> Option<&CssDuration>`, `delay() -> Option<&CssDelay>`, and `iteration_count() -> Option<&CssAnimationIterationValue>`; each current list exposes `values()`. Construction of these aggregate values is parser-owned.

Exactly seven wrappers gain current-value accessors: `CssTransitionDurationPropertyValue::durations()`, `CssTransitionDelayPropertyValue::delays()`, `CssAnimationDurationPropertyValue::durations()`, `CssAnimationDelayPropertyValue::delays()`, `CssAnimationIterationCountPropertyValue::iteration_counts()`, `CssTransitionPropertyValue::transitions()`, and `CssAnimationPropertyValue::animations()`. They return respectively `&CssDurationList`, `&CssDelayList`, `&CssDurationList`, `&CssDelayList`, `&CssAnimationIterationValueList`, `&CssTransitionValueList`, and `&CssAnimationValueList`.

Their `i01_subset()` projections remain byte-for-byte compatible for every I01 input. Newly accepted negative-delay or calculation syntax returns `None` only when it cannot be represented by the I01 payload. Existing `CssTime`, `CssTimeList`, `CssTransition`, `CssTransitionList`, `CssAnimation`, `CssAnimationList`, and `CssAnimationComponents` remain available as compatibility models and keep their existing signatures and observable values for the I01 subset. New parser paths do not place an invalid or lossy state into those compatibility types.

Typed calculations use one private expression implementation with seven public private-field root wrappers:
`CssNumberCalculation`, `CssIntegerCalculation`, `CssPercentageCalculation`, `CssLengthCalculation`,
`CssAngleCalculation`, `CssTimeCalculation`, and `CssFrequencyCalculation`. Each exposes
`expression() -> CssCalculationExpressionRef<'_>` and `result_type() -> CssCalculationType`. The non-exhaustive
`CssCalculationType` variants are `Integer`, `Number`, `Percentage`, `Length`,
`LengthPercentage`, `Angle`, `AnglePercentage`, `Time`, `TimePercentage`,
`Frequency`, and `FrequencyPercentage`.

`CssCalculationExpressionRef<'a>` is non-exhaustive with `Value(CssCalculationValueRef)`,
`Sum(CssCalculationSumRef<'a>)`, `Product(CssCalculationProductRef<'a>)`, and `Negate`, `Group`, and `NestedCalc` each carrying `CssCalculationUnaryRef<'a>`. The non-exhaustive leaf
view has `Integer(i32)`, `Number(CssFiniteNumber)`,
`Percentage(CssFiniteNumber)`, `Length(CssLengthDimension)`,
`Angle(CssAngleLiteral)`, `Time(CssDelayLiteral)`, and
`Frequency(CssFrequencyLiteral)`. New non-exhaustive
`CssAngleUnit::{Degrees, Gradians, Radians, Turns}` and
`CssFrequencyUnit::{Hertz, Kilohertz}` plus private-field checked
`CssAngleLiteral` and `CssFrequencyLiteral` preserve the authored unit and expose
`value()`/`unit()`.

`CssCalculationSumRef<'a>::term(usize) -> Option<CssCalculationSumTermRef<'a>>` and
`CssCalculationProductRef<'a>::factor(usize) -> Option<CssCalculationProductFactorRef<'a>>`; both expose `len() -> usize`. Term/factor views expose
`operator() -> Option<CssCalculationSumOperator>` or
`Option<CssCalculationProductOperator>` (`None` first) and
`expression() -> CssCalculationExpressionRef<'a>`. The non-exhaustive operators
are `Add`/`Subtract` and `Multiply`/`Divide`.
`CssCalculationUnaryRef<'a>::operand() -> CssCalculationExpressionRef<'a>`.
No owned typed-expression node or typed compound constructor is public. Exact leaf construction is
`CssNumberCalculation::try_literal(f32)`, `CssIntegerCalculation::literal(i32)`,
`CssPercentageCalculation::try_literal(f32)`, `CssLengthCalculation::{try_dimension(f32, CssLengthUnit), try_percentage(f32)}`,
`CssAngleCalculation::try_literal(f32, CssAngleUnit)`, `CssTimeCalculation::try_literal(f32, CssTimeUnit)`, and
`CssFrequencyCalculation::try_literal(f32, CssFrequencyUnit)`; fallible forms
return `Option<Self>`. Compound trees are parser-owned. Invalid dimensional
trees are unconstructable while every parser-produced node is inspectable
without Debug parsing.

Existing `CssCalcLength::sum`, `CssCalcLengthTerm::{add, sub}`, and
`CssCalcOperator::{Add, Subtract}` remain public compatibility construction.
They create only the old sum model; `CssCalcLength::Typed` is parser-owned.

Sum operands must have the same category except selected percentage promotion
to the applicable mixed percentage category. A product contains at most one
dimensioned factor and all other factors are numbers. A divisor is a number
tree; a fully numeric divisor that evaluates to positive or negative zero is
rejected, as is non-finite numeric arithmetic. Integer sums/products remain
integer; division or a non-integer number promotes to number. Negation, grouping,
and nested `calc()` retain the operand category. These checks use no layout or
environment context.

Literal non-negative property contexts reject a negative literal at its first
responsible token. A well-typed calculation remains authored and representable
when its eventual numeric range belongs to computed-value processing; a parser
must not reject it merely by finding a negative component. Existing simple I01
length sums retain their exact `CssCalcLength` compatibility projection and C01
fixture observable. Newly supported products, groups, nested calculations, or
other roots use the current typed model and return no I01 projection when the
old payload cannot represent them.

The public API effect is additive after C01: new private-field types, checked
constructors, borrowed non-exhaustive views, enum variants, and
property-specific accessors. Existing signatures are unchanged. Dependencies,
features, build logic, generated artifacts, and leaf MSRV are unchanged. Root
owns facade reexports, generated API audit refresh, integration tests/docs, and
gitlink promotion after the published handoff. All owned Rust remains free of
`unsafe`.

Integration tests apply authored CSS through crate-root parsers or call checked
public constructors. Owning `src/parser/values.rs` `#[cfg(test)]` cases may call
its private typed-root parser with authored tokens and assert the semantic tree
or structured error, providing compound angle/frequency evidence without test
surface. No test reads or parses Rust source, asserts symbols or code
placement, compares owner/test/catalog sets or counts, mutates an inventory as a
completeness proxy, checks call sequences, or encodes plan/review/publication
state. The C01 fixture is immutable and must not be regenerated or edited.

## 3. Tasks

### T1 Make Every Numeric Scalar Construction Path Finite

- **Files/area:** numeric scalar models in `src/syntax.rs`; the
  `CssGridFlowTolerancePropertyValue` representation in `src/properties.rs`;
  their existing parser call sites; `tests/numeric_domains.rs`,
  `tests/public_surface.rs`, and focused crate-private value tests. No
  calculation grammar, timing shorthand, catalog, fixture, manifest, or docs
  edit.
- **Dependency:** published/read-back C02 base only.
- **Outcome:** audit every current public and parser-reachable numeric wrapper
  and make NaN and both infinities unconstructable, explicitly excluding direct
  construction of the frozen raw Grid enum. Preserve sign/range boundaries,
  validate unchecked parser paths, and return diagnostics rather than panic.
  Restore the exact legacy
  `CssGridFlowTolerance::Percent(f32)` payload and implement the exact dual
  current/compatibility property representation and `value()` accessor above.
- **RED evidence:** checked constructors and real property parses first expose at
  least one currently admitted non-finite path, with named finite edge cases as
  characterization. The RED asserts the public construction or parse outcome,
  never the constructor's source shape or number of wrappers.
- **Acceptance:** representative number, integer, percentage, length, angle,
  time, frequency/resolution, opacity, ratio, keyframe, font, Grid, flex, and
  iteration-count paths preserve finite values and reject non-finite values;
  public construction and matching of the unchanged legacy Grid percent payload
  compiles, while a parser-produced Grid wrapper exposes the checked current
  percent and matching finite I01 projection; finite `3.5e38%` and
  `calc(3.5e38%)` inputs whose percentage conversion overflows are rejected by
  the shared checked conversion with exact diagnostics and sibling retention in
  both feature modes;
  error code, typed payload, position, span, and recovery action are exact for
  parser cases; C01 observables remain byte-identical.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test numeric_domains`; `cargo test -p surgeist-css --offline --no-default-features --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  repeat those four commands with `--features app-strict`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `fix: enforce finite CSS numeric domains`.

### T2 Publish The Typed Calculation Tree

- **Files/area:** shared authored calculation types in `src/syntax.rs`, parser
  and owning `#[cfg(test)]` cases in `src/parser/values.rs`, crate-root
  exports/rustdoc, and `tests/typed_calculations.rs`; no property schema, timing, catalog, fixture,
  manifest, or later math-function edit.
- **Dependency:** T1 independently clean.
- **Outcome:** implement the seven exact root wrappers and borrowed node/leaf
  views from section 2. Parse `calc()` sums, products, divisions, negation,
  groups, and nested `calc()` with precedence and complete input consumption.
  Enforce dimensional promotion, root equality, finite arithmetic, and the
  number-only zero-divisor rule. Preserve authored symbolic values and unit
  identity; do not evaluate dimensioned results.
- **RED evidence:** checked public leaf/tree boundaries and real existing
  property consumers first reject or cannot represent named valid products,
  groups, and nested calculations; invalid mixed sums, dimension-by-dimension
  products, non-number divisors, zero divisors, non-finite values, missing
  operands, and trailing tokens first demonstrate exact failure.
- **Acceptance:** each of number, integer, percentage, length, angle, time, and
  frequency roots has a public positive, negative, signed boundary, and
  overflow/non-finite case; borrowed inspection distinguishes every required
  node kind and result category; invalid trees cannot be publicly constructed;
  owning `src/parser/values.rs` cases parse valid and invalid compound trees for
  all seven roots through the private root parser;
  nested depth 255 succeeds, 256 reaches the existing limit, and 257 recovers
  without panic or loss of later siblings where a parser consumer exists.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test typed_calculations`; `cargo test -p surgeist-css --offline --no-default-features --lib parser::values`; `cargo test -p surgeist-css --offline --no-default-features --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  repeat those five commands with `--features app-strict`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `feat: add typed CSS calculation trees`.

### T3 Integrate Typed Math With Existing Numeric Property Consumers

- **Files/area:** `src/parser/values.rs`, affected value types in
  `src/parser/layout.rs`, `src/parser/grid.rs`, `src/syntax.rs`, affected wrappers in
  `src/properties.rs`, and `tests/typed_calculations.rs`,
  `tests/property_schema.rs`, and `tests/structured_errors.rs`; no timing,
  catalog, fixture, manifest, or later-cycle property grammar edit.
- **Dependency:** T2 independently clean.
- **Outcome:** accept typed calculations in this exact current consumer set:
  `opacity`, `flex-grow`, `flex-shrink`, `flex`, `order`, `z-index`,
  `aspect-ratio`, and `grid-flow-tolerance`; plus the existing length grammar
  reached by `width`, `height`, `min-width`, `min-height`, `max-width`,
  `max-height`, `flex-basis`, all three gaps, four Grid track-list properties,
  `font-size`, `line-height`, `font`, `text-indent`, `vertical-align`,
  `letter-spacing`, `text-decoration`, `text-decoration-thickness`, inset/margin/
  padding shorthands and longhands, border and border-width shorthands/longhands,
  border radii, `box-shadow`, `outline`, `outline-width`, background/mask
  position and size, `transform-origin`, and `translate`. Timing is T4. Angle,
  frequency, Media resolution, keyframe percentages, font-feature numeric values,
  and dedicated C05 function internals receive only the T1/T2 model here and
  remain property-parser integration work for their owning later cycles.
- **Scalar model:** add non-exhaustive
  `CssOpacityValue::{Literal(CssOpacity), Calculation(CssNumberCalculation)}`,
  `CssNonNegativeNumberValue::{Literal(CssNonNegativeNumber), Calculation(CssNumberCalculation)}`,
  `CssPositiveNumberValue::{Literal(CssPositiveNumber), Calculation(CssNumberCalculation)}`,
  `CssAspectRatioValue::{Literal(CssAspectRatio), Calculation(CssNumberCalculation)}`,
  `CssIntegerValue::{Literal(i32), Calculation(CssIntegerCalculation)}`,
  `CssZIndexValue::{Auto, Integer(CssIntegerValue)}`, and current
  `CssFlexValue::{None, Auto, Components(CssFlexComponents)}`. Private-field
  `CssFlexComponents` exposes `grow() -> &CssNonNegativeNumberValue`,
  `shrink() -> Option<&CssNonNegativeNumberValue>`, and `basis() -> Option<&CssLength>`.
  Private-field `CssPositiveNumber::try_new(f32)` enforces finite positive literals. Exact wrapper accessors are `CssOpacityPropertyValue::value()`,
  `CssFlexGrowPropertyValue::factor()`, `CssFlexShrinkPropertyValue::factor()`, `CssOrderPropertyValue::value()`, `CssZIndexPropertyValue::value()`,
  `CssAspectRatioPropertyValue::ratio()`, and `CssFlexPropertyValue::value()`;
  they return the corresponding current types above. Aspect-ratio literals stay
  finite and positive; a number calculation's range is deferred even with a
  negative component. Literal aspect ratios retain the exact I01 projection,
  while typed calculations and all other new calc syntax return none.
- **Grid compatibility:** integrate typed length/percentage calculations through
  T1's `CssGridFlowToleranceValue` and existing
  `CssGridFlowTolerancePropertyValue::value()`; do not change the legacy
  `CssGridFlowTolerance` enum or the wrapper's I01 accessor.
- **Compatibility:** preserve each I01 simple length sum as the same
  `CssCalcLength` projection; additive `CssCalcLength::Typed` carries new length
  trees. Existing wrappers above need no parallel accessor because their payload
  already contains `CssLength`/`CssCalcLength`; their `i01_subset()` stays `Some`.
  Literal range checks remain parse-time, while calculation ranges are deferred
  even when a calculation contains a syntactically negative component.
- **RED evidence:** authored property values first fail for valid product/group/
  nested calculations and for a well-typed calculation in a non-negative
  context, while one-token invalid type/operator/divisor mutations demonstrate
  the expected structured diagnostic and recovery.
- **Acceptance:** `CssCalcLength::Typed` and the named scalar payloads expose
  exact typed trees; frozen inputs retain `Some` projections and new syntax
  returns `None`; public aspect-ratio evidence distinguishes a rejected
  non-positive literal from an accepted typed number calculation and inspects
  its tree; non-negative literal versus calculation behavior is distinct in
  public outcomes; sibling retention, non-BMP source
  coordinates, recovery spans/actions, and default/`app-strict` parity are exact;
  the C01 fixture bytes and results remain unchanged.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test typed_calculations`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`;
  `cargo test -p surgeist-css --offline --no-default-features --test source_coordinates`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  repeat those five commands with `--features app-strict`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `feat: integrate typed CSS numeric calculations`.

### T4 Separate Duration, Delay, And Iteration Timing Domains

- **Files/area:** timing models in `src/syntax.rs`, `src/parser/timing.rs`, the
  seven affected property-wrapper representations in `src/properties.rs`, and
  `tests/timing_domains.rs`, `tests/property_schema.rs`,
  `tests/structured_errors.rs`, and `tests/source_coordinates.rs`; no easing
  function redesign, catalog, fixture, manifest, or docs edit.
- **Dependency:** T3 independently clean.
- **Outcome:** implement the exact duration, delay, current transition, current
  animation, list, and property-specific accessor model from section 2. Parse
  non-negative duration literals, signed delay literals, well-typed symbolic
  time calculations for either domain with range deferred, finite iteration
  literals, `infinite`, and typed number calculations. Enforce
  first-time duration/second-time delay shorthand assignment for every token
  order without conflating the domains.
- **RED evidence:** negative `transition-delay`/`animation-delay` and valid time
  calculations first fail; negative first shorthand times and third time values
  demonstrate rejection; non-finite iteration and timing values demonstrate
  checked/public and parser rejection.
- **Acceptance:** longhand lists and shorthands expose exact current semantic
  values; positive I01 inputs retain exact compatibility projections and Debug
  observables; new negative-delay/calculation inputs return `None` from only the
  lossy I01 projection; invalid first-time, duplicate-time, empty-list,
  non-finite, type-mismatch, separator, and trailing-token cases have exact
  diagnostic payload/position/span/action and retain valid siblings; repeated
  failures and depth boundaries remain panic-free and feature-parity clean.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test timing_domains`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`;
  `cargo test -p surgeist-css --offline --no-default-features --test source_coordinates`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  repeat those five commands with `--features app-strict`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `feat: separate CSS timing domains`.

### T5 Promote Exact C03 Metadata And Publish The Numeric Handoff

- **Files/area:** `src/conformance.rs`, owning shared-value inventory in
  `src/parser/values.rs`, timing/property inventories, README/crate rustdoc,
  focused public catalog consumers, and new SHA-free
  `plans/handoffs/P01-I02-C03-numeric-timing-math-foundation.md`; no grammar
  beyond T1-T4, fixture, manifest, root, sibling, or generated-artifact edit.
- **Dependency:** T4 independently clean.
- **Outcome:** promote these exact O-VALUES3 IDs:
  `official.value.integer`, `official.value.number`,
  `official.value.percentage`, `official.value.length`,
  `official.value.length-percentage`, `official.value.time`, and
  `official.value.resolution` are `Complete`; `official.value.dimension` is
  `Partial` for selected typed unit families versus other dimensions;
  `official.value.angle`, `official.value.angle-percentage`,
  `official.value.time-percentage`, `official.value.frequency`, and
  `official.value.frequency-percentage` are `Partial` for the public typed model
  versus later property consumers named in T3; `official.value.calc` is `Partial`
  for the C03 grammar/consumer set versus those later consumer integrations.
  Each gains exact O-VALUES3 source/production and one owning inventory identity.
- **Timing metadata:** exact `Complete` triples are
  `baseline.property.transition-duration`/I-TRANSITIONS1/`#propdef-transition-duration`, `baseline.property.transition-delay`/I-TRANSITIONS1/`#propdef-transition-delay`,
  `baseline.property.animation-duration`/I-ANIMATIONS1/`#propdef-animation-duration`, `baseline.property.animation-delay`/I-ANIMATIONS1/`#propdef-animation-delay`, and `baseline.property.animation-iteration-count`/I-ANIMATIONS1/`#propdef-animation-iteration-count`.
  Exact `Partial` triples are `baseline.property.transition`/I-TRANSITIONS1/
  `#propdef-transition` and `baseline.property.animation`/I-ANIMATIONS1/
  `#propdef-animation`; their subset is I01 components plus C03 timing/math and
  their remainder is C05 easing/function closure. All seven use property-schema
  identity, `crate::parser::timing`, `timing_domains` behavior, and named
  `conformance_catalog` public metadata cases. Every other timing row is unchanged. Document current
  versus I01 compatibility access, literal-range versus calculation-range phase,
  symbolic typed calculations, downstream exclusions, and exact root follow-up.
- **Evidence:** named public metadata cases pair each promoted or changed ID with a real
  authored parser outcome or checked public value outcome. Direct coordinator
  and reviewer reconciliation proves source/catalog/owner/case truthfulness;
  Rust tests do not compare their sets or counts. Documentation uses compiling
  crate-root examples and deterministic writing checks, never substring tests.
- **RED evidence:** public metadata cases first expose the reserved/absent or
  stale status for each implemented row. Documentation gaps are established by
  direct artifact comparison and receive no artificial failing test.
- **Acceptance:** every promoted or changed row's status, subset/remainder, source,
  production, implementation owner, and named behavior are exact; no unimplemented
  row is exposed; README/rustdoc examples compile; the handoff names public
  additions, promoted IDs, exact I01 preservation evidence, root-owned facade/API
  work, and no SHA/review/publication/command-manifest state; fixture SHA-256
  remains `98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8`.
- **Deferred reserved rows:** `official.value.syntax-token-stream`,
  `component-value`, `simple-block`, `function`, `declaration-value`, `any-value`,
  `css-wide-keyword`, `custom-ident`, `ident`, `string`, `url`, and
  `url-modifier` remain private reserved coverage and receive no C03 status
  claim. `official.value.position` likewise remains reserved for C04. T5
  reconciles but neither deletes nor promotes any of these deferred rows.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`; `cargo test -p surgeist-css --offline --no-default-features --test catalog_inventory`; `cargo test -p surgeist-css --offline --no-default-features --test numeric_domains`; `cargo test -p surgeist-css --offline --no-default-features --test typed_calculations`;
  `cargo test -p surgeist-css --offline --no-default-features --test timing_domains`; `cargo test -p surgeist-css --offline --no-default-features --test public_surface`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  repeat those seven commands with `--features app-strict`;
  `cargo test -p surgeist-css --offline --no-default-features --doc`; `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`;
  `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps`; `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps`;
  `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`; `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n 'TO''DO|TB''D|FIX''ME|\?''\?''\?' README.md src/lib.rs plans/handoffs/P01-I02-C03-numeric-timing-math-foundation.md`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `docs: publish the CSS numeric foundation`.

## 4. Exact Completion Gate

After every implementation task has a clean task review and the status-only
completion commit is made, run from a process-clean repository:

```sh
cargo check -p surgeist-css --offline --no-default-features && cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --doc && cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo check -p surgeist-css --offline --no-default-features --features app-strict && cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc && cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps
cargo fmt --check
git diff --check a80ff9339f21ad041b159de72a03942ffb11ac50..HEAD
rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs
! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .
```

Directly inspect every changed test for a real public or owning-crate stimulus
and the absence of source/code/count/owner-set/coordination proxies. Confirm the
C01 fixture SHA-256 is unchanged. Apply the canonical Surgeist task-review,
cycle-status, holistic-review, landing, and publication contracts to the exact
cycle range; no force push, non-fast-forward push, or history rewrite is allowed.

Run full `cargo clean --offline` after the cycle and confirm `target`
is absent. Before and after each task/review/gate transition, identify processes
whose working directory or command belongs to this repository, stop only stale
processes owned by this work, and leave no `surgeist-css` Cargo/test process
running. The handoff is
`plans/handoffs/P01-I02-C03-numeric-timing-math-foundation.md`. A required frozen
I01 semantic change, second breaking I02 API change, unsafe, dependency or
feature addition, external acquisition, root/sibling edit, unresolved source
ownership, or inability to preserve the C01 fixture is a genuine blocker.

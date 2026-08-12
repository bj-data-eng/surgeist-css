# P01-I01 Implementation Sequence

## 1. Authority And Initiative Boundary

This sequence orders implementation of
`plans/specs/P01-I01-browser-recovery-authored-api-foundation.md`, reviewed
`CLEAN` at normalized SHA-256
`76b76a50a613aea26e1b790749a780f7d05efdfe57711c6b8dbf9a9fca2359d7`.
The owning repository is `surgeist-css`. The initiative is intentionally
breaking and ends with a published leaf candidate and root-owned migration
handoff; no cycle edits root or a sibling repository.

The sequence covers I01 exactly once. I02 grammar closure and I03 corpus work
remain outside every cycle below. Only the next ready cycle receives a detailed
cycle plan.

## 2. Ordered Cycles

### I01-C01 Source And Diagnostic Foundation

- **Owning repository:** `surgeist-css`.
- **Specification sections:** 4, 5, 7, 10, 11, 12.5, 13 findings 2.22,
  2.23, and 2.25.
- **Bounded outcome:** the crate uses the final semantic byte/line/UTF-16 source
  model and structured error taxonomy, exposes the final recovery-report and
  diagnostic value types without yet changing ordinary parser signatures, and
  prohibits owned unsafe at the crate root.
- **Prerequisites and entry:** the reviewed I01 specification and this reviewed
  sequence; cycle base is the current published leaf `main` plus the reviewed
  planning packet.
- **Exit evidence:** all existing strict parser behavior uses the new source and
  structured-error boundary; coordinate/non-BMP, error-code/detail, span/action,
  construction, public-consumer, and no-unsafe evidence is clean; cycle cleanup
  removes build artifacts and leaves no stale crate process.
- **Handoff:** publish the additive/breaking diagnostic foundation for C02. Root
  does not promote this incomplete initiative candidate.

### I01-C02 Coupled Declaration Foundation

- **Owning repository:** `surgeist-css`.
- **Specification sections:** 5, 8.1 through 8.4, 9, 9.2 property inventory,
  10, 12.3, 13 findings 2.6, 2.19, and 2.20.
- **Bounded outcome:** all 179 baseline properties are generated from one schema;
  declarations couple each known property to its value type, preserve custom and
  substitution-dependent authored values, carry importance and semantic source
  positions, and expose no mismatched or unchecked construction path.
- **Prerequisites and entry:** C01 is published and its source/error/report
  contracts remain valid.
- **Exit evidence:** the strict stylesheet parser and all property parsers use
  the coupled model; schema/source equality, known/custom/global/substitution,
  cross-property rejection, importance, keyframe-context, and constructor
  evidence is clean; cycle cleanup removes build artifacts and leaves no stale
  crate process.
- **Handoff:** publish the declaration foundation for recovery parsing in C03.
  Root does not promote this incomplete initiative candidate.

### I01-C03 Structural Stylesheet Recovery

- **Owning repository:** `surgeist-css`.
- **Specification sections:** 1, 4 ordinary sheet API, 5, 6.1 structural rows,
  6.2, 6.4, 7.3, 10, 12.1, 13 finding 2.15.
- **Bounded outcome:** `parse_sheet` returns a report and owns deterministic
  structural recovery for at-rules, qualified rules, declarations, descriptors,
  and keyframe blocks, including leading encoding handling, balanced boundaries,
  child/parent ordering, progress, and nesting enforcement.
- **Prerequisites and entry:** C02 is published; final source, diagnostic,
  report, and declaration models are available.
- **Exit evidence:** valid siblings survive every structural recovery context;
  retained nodes remain valid; every drop has exact typed error, position, span,
  action, and order; adversarial ordinary input does not unwind; cycle cleanup
  removes build artifacts and leaves no stale crate process.
- **Handoff:** publish the report-based structural parser for specialized
  recovery and the secondary front door in C04. Root does not promote this
  incomplete initiative candidate.

### I01-C04 Specialized Recovery And Application Strictness

- **Owning repository:** `surgeist-css`.
- **Specification sections:** 4, 6.1 specialized rows, 6.3, 6.4, 7, 8.3,
  12.1, 12.2, 13 findings 2.5 and 2.6.
- **Bounded outcome:** forgiving selector-member recovery, malformed-media
  `Never` sentinels, implicit EOF closure, legacy-token handling, and nesting
  limit behavior are complete; style attributes use the shared recovering
  declaration core; `app-strict` is a one-pass additive wrapper with invariant
  ordinary behavior.
- **Prerequisites and entry:** C03 is published and all structural report
  semantics are stable.
- **Exit evidence:** every recovery action and both ordinary front doors satisfy
  their matrices; default/feature ordinary outputs are identical; strict
  validation accepts exactly clean reports and returns the complete non-empty
  diagnostic set otherwise; cycle cleanup removes build artifacts and leaves no
  stale crate process.
- **Handoff:** publish the complete I01 parser behavior for catalog and product
  closure in C05. Root does not promote this incomplete initiative candidate.

### I01-C05 Catalog, Public Evidence, And Candidate Closure

- **Owning repository:** `surgeist-css`.
- **Specification sections:** 1 through 14, with primary focus on 9, 11, 12.4,
  12.5, 13 findings 2.18, 2.21, and 2.24, and initiative acceptance.
- **Bounded outcome:** the independent finite I01 catalog, public metadata
  queries, crate/README guidance, doctests, and tracked public-consumer evidence
  cover the final parser surface; all I01 acceptance predicates and allocated
  historical findings are closed.
- **Prerequisites and entry:** C04 is published; parser signatures, recovery
  behavior, declarations, diagnostics, positions, and feature semantics are
  final for I01.
- **Exit evidence:** catalog, implementation inventory, and independent vectors
  agree bidirectionally; all 179 property records and every frozen non-property
  row are accounted for; configured verification and repository-wide no-unsafe
  evidence are clean; cycle cleanup removes build artifacts and leaves no stale
  crate process.
- **Handoff:** publish the immutable final I01 leaf candidate with an exact
  breaking API/root-adapter migration record. I02 may be specified JIT only after
  remote readback and candidate handoff complete.

## 3. Dependency And Stop Contract

The order is strict: source/error/report semantics precede declaration migration;
both precede parser recovery; structural recovery precedes specialized recovery
and strict validation; the catalog closes only the settled final surface. A
later cycle may add evidence for an earlier contract but may not reinterpret it.

Stop and reconcile the I01 specification and affected sequence entries if a
cycle requires a second grammar, a changed production dependency, a public raw
token escape, an invalid retained node, a different recovery action/coordinate
meaning, a retired inventory row, root or sibling mutation, or owned `unsafe`.

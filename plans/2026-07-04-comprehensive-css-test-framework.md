# Comprehensive CSS Test Framework

## Goal

Build a more comprehensive unit testing framework for the expanded
`surgeist-css` strict validation surface. The framework should make it cheap to
add property-specific accept/reject matrices and should prove that this crate
does not perform browser-style recovery: any invalid selector, declaration,
property, value, list item, shorthand, or rule body must fail the whole
`parse_sheet` call.

This plan is crate-local and follows `AGENTS.md`: the coordinator assigns
workers and reviewers, workers do not commit, and each clean task lands as a
traceable logical commit.

## Non-Goals

- Do not implement new CSS property support.
- Do not change successful authored syntax semantics except where a test reveals
  an existing strictness bug that must be fixed.
- Do not introduce browser recovery, lossy cleanup, silent declaration drops, or
  permissive parsing.
- Do not edit sibling repos or root submodule pointers.

## Standards

- Use `guidance/surgeist-rust-modeling-guide.md`.
- Keep test helpers typed and behavior-focused. Avoid helpers that hide which
  property/value pair is being tested.
- Prefer table-driven cases with explicit expected property, expected error
  kind, and a short case label.
- For any strictness bug found while adding tests, write the failing test first,
  verify the failure, then make the smallest parser/model fix.
- Keep public crate API unchanged unless a reviewer confirms a test-only
  structure cannot exercise the required invariant any other way.

## Framework Shape

Add a test-only harness that can express:

- accepted declaration cases: property name, authored value, expected
  `CssProperty`, and optional structured assertion on `CssValue`
- rejected declaration cases: property name, authored value, expected
  `ErrorKind` shape, and whether the property name should be recognized
- whole-sheet strictness cases: mixed valid/invalid declarations, invalid
  selectors, invalid at-rules, malformed blocks, trailing value junk, empty list
  items, and invalid shorthand pieces
- registry coverage cases: every supported property has at least one accepted
  case and at least one targeted rejected case unless the plan documents a
  property-specific reason
- unit matrix cases: every supported `CssLengthUnit` parses in ordinary length
  and calc contexts where the property grammar allows dimensions; unknown units
  still reject

The harness may live in `src/lib.rs` under `#[cfg(test)]`, or in a
`#[cfg(test)]` child module such as `src/test_support.rs` if that keeps the
suite readable while preserving access to crate-private internals.

## Task 1: Harness Foundation And Registry Coverage

Create the test-only helper layer and the first registry coverage checks.

Requirements:

- Add typed test case helpers for accept/reject declaration cases.
- Add helper(s) that parse a single declaration and return its `CssDeclaration`.
- Add helper(s) that assert whole-sheet rejection without accepting partial
  parse results.
- Add a registry coverage test proving every supported property in
  `src/validation.rs` has at least one accepted case in the new framework.
- Keep `KNOWN_UNSUPPORTED_PROPERTY_NAMES` empty and tested.
- Do not move all existing tests in this task; migrate or add only enough cases
  to prove the framework works.

Required checks:

```sh
cargo fmt --check
cargo test -p surgeist-css coverage
cargo test -p surgeist-css strict
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 2: Property Family Acceptance Matrices

Populate broad accepted-value matrices for the supported property families.

Minimum family coverage:

- CSS-wide globals, including `all`
- box sizing and display/layout keywords
- box size values, margins, padding, insets, gap, border widths, and radius
- color, background, border color/style, outline, shadow
- positioning, float/clear, visibility, alignment, order/flex
- grid template/track/line/shorthand forms
- typography and text families
- interaction, transform, filters/effects, masks
- transition and animation longhands/shorthands
- supported length units in ordinary and calc contexts

Requirements:

- Use the framework from Task 1 rather than adding ad hoc one-off tests.
- Preserve representative structured assertions for important authored syntax
  shapes, not just `parse_sheet(...).is_ok()`.
- Keep tests readable enough that a future property addition shows exactly
  which matrix entry to add.

Required checks:

```sh
cargo fmt --check
cargo test -p surgeist-css acceptance
cargo test -p surgeist-css unit_matrix
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 3: Property-Specific Rejection Matrices

Populate broad rejection matrices proving strict property-specific validation.

Minimum rejection coverage:

- wrong keyword family leakage across every property family
- wrong unit/domain leakage, including percent where forbidden and auto where
  forbidden
- negative numeric values where the model requires non-negative values
- malformed calc/function arguments
- malformed comma lists and empty items
- invalid shorthand combinations and duplicate shorthand components
- duplicate-axis positions and other modeled public constructor invariants
- unsupported but syntactically valid CSS keywords remain rejected unless this
  crate explicitly supports them

Requirements:

- Use typed `ErrorKind` shape assertions when stable.
- Add at least one rejection case for every supported property, or document a
  precise reason in the test table for why a property is global-keyword-only or
  otherwise covered by a family-level rejection.
- If a new test finds a permissive parser path, fix it with TDD and add a
  focused regression case.

Required checks:

```sh
cargo fmt --check
cargo test -p surgeist-css rejection
cargo test -p surgeist-css leakage
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 4: Strict No-Recovery Whole-Sheet Tests

Lock down the application-UI strictness contract.

Required cases:

- valid declaration before invalid declaration fails the whole sheet
- invalid declaration before valid declaration fails the whole sheet
- unknown property fails the whole sheet
- unsupported at-rule fails the whole sheet
- invalid selector fails the whole sheet
- malformed declaration block fails the whole sheet
- trailing junk after a value fails the whole sheet
- invalid comma-list item fails the whole sheet
- invalid shorthand component fails the whole sheet
- no test should assert browser-like recovery or partial result retention

Required checks:

```sh
cargo fmt --check
cargo test -p surgeist-css no_recovery
cargo test -p surgeist-css strict
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 5: Final Audit And Documentation

Review the complete test framework and update documentation if needed.

Requirements:

- Add README wording that this crate rejects invalid application CSS instead of
  recovering like a browser, if that wording is not already explicit enough.
- Ensure the suite count meaningfully reflects the expanded surface. Prefer
  many table rows over a few giant tests when separate failures would be easier
  to diagnose.
- Run a final coverage audit over `CssProperty`, `SUPPORTED_PROPERTY_NAMES`,
  `CssLengthUnit`, and parser-family tests.
- Assign a final clean-context holistic reviewer.

Final checks:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
git status --short --branch
```

Completion requires all task-scoped worker/reviewer cycles to be clean, final
checks to pass, and final holistic review to approve.

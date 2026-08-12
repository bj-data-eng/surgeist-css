# P01-I02-C01 Evolution Boundary And Equivalence Foundation

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C01` |
| Owning repository | `surgeist-css` |
| Status | `in_progress` |
| Cycle base | `57b71354e83f70ff0665241eedfebe269f754fa4` |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `e290a7fef9bf6b6e9bde764140e5f7fac34156bb8f644d999e6bba58dc92ca2b`, especially P01.9 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `cd6f00a46ab4c5eeccf1a0c2e312eb329d1c365fdb30b4fd99cae0cdd3ddaec8`, sections 1-3, 10, and 12.1 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `02955c03b5cc404aeb6e6a1724402d570d1736d06ccef74dc631084540188116`, I01/base identity only; no ledger grammar/status change in C01 |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `1e1750cc0d21b9b08bee0f084c5b28c943c0a15949c2618bcce387c73c57ef7b`, entry `I02-C01` |
| Bounded outcome | Freeze a finite public I01 report oracle, perform the one authorized I02 public evolution/declaration representation repair without changing accepted/recovered input semantics, and publish a clean migration foundation for additive grammar cycles. |

## 2. Boundary, Compatibility, And Order

C01 is intentionally breaking and is the only breaking I02 cycle. It changes
public exhaustiveness and property-declaration inspection, not parser language,
recovery, diagnostics, positions, declarations' semantic coupling, features,
dependencies, or ownership. It adds no official grammar, source/catalog status,
property name, rule, descriptor, selector, query, value spelling, recovery
action, diagnostic code, dependency, feature, build script, generator, corpus,
root/sibling edit, external software, or unsafe.

All committed tests obey the installed Surgeist testing reference: they apply a
real parser or compiler stimulus and assert public or crate-owned behavior. No
test reads Rust source, plans, handoffs, review state, command manifests, or code
shape as proxy evidence. Task and holistic reviewers may inspect implementation
structure directly; coordinator checks may inspect exact documentation,
manifest, migration, and safety artifacts outside the Rust test suite.

Tasks are strictly serial. T1 must be independently reviewed and committed while
production is still the published I01 representation. T2 starts from reviewed
T1. T3 starts from reviewed T2. T4 starts from reviewed T3. No task may absorb a
later task's work, and a production-language difference found by the oracle is a
stop/reconcile event rather than a fixture update.

## 3. Tasks

### T1 Freeze The I01 Observable Equivalence Oracle

- **Files/area:** new `tests/fixtures/i01-c01-observables.tsv`, a new focused
  integration test; no production source or existing expectation edits.
- **Outcome:** hand-author the finite behavioral corpus required by specification
  3.2 before representation edits. Every TSV row fixes a stable scenario label,
  entry point, feature mode, authored input, clean state, ordered retained
  rule/property stable IDs, retained authored value slice and importance where
  applicable, and every ordered diagnostic's code, `ErrorKind` root, stable
  payload identity, byte/line/UTF-16 position, complete span endpoints, and
  recovery action. The corpus contains no test-owner/name mapping or test,
  execution, or comparison counts.
- **Independence and format:** the fixture is not emitted by the parser or
  generated from catalog/parser tables. The reader rejects unknown columns,
  duplicate scenario labels, malformed escapes, absent required observables,
  invalid field values, retained/authored declaration mismatches, and
  noncanonical order. Malformed-schema checks remain within the declared fixture
  contract; no source/test manifest, identity fingerprint, or coordinated
  omission meta-test exists.
- **RED evidence:** focused tests first fail because the fixture and reader are
  absent; a malformed fixture proves schema/required-observable
  rejection before the completed corpus is installed.
- **Acceptance:** every row passes against the unmodified I01 public front door
  in its recorded feature mode; default and `app-strict` cases agree on ordinary
  report observables; full existing tests remain unchanged and green; the diff
  contains no `src/`, manifest, README, or existing expected-value edit.
- **Commands:**
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict`;
  `cargo fmt --check`; `git diff --check`;
  `rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `test: freeze I01 public report observables`.

### T2 Repair Public Enum Evolution Boundaries

- **Files/area:** every owned Rust public enum and rustdoc/public consumer tests;
  no parser branch or value representation change and no source-text inventory
  test or fixture.
- **Outcome:** exactly `CssImportance` and `CssSupportStatus` are closed; every
  other public enum at the task head is `#[non_exhaustive]`. The task reviewer
  inspects the complete owned source set, including nested visibility,
  macro-generated enums, and feature-gated public enums. Enums added by T3/T4
  follow the same rule and receive direct review in their owning task range.
- **Evidence:** external compile-fail doctests prove representative evolving
  enums cannot be exhaustively matched; positive consumers use wildcards.
  Closed-enum consumers exhaustively match both `CssImportance` branches and
  all three `CssSupportStatus` branches. These tests exercise compiler-visible
  API behavior; they do not parse or count source declarations. Exact complete
  attribute coverage is implementation-review evidence, not a behavioral-test
  oracle.
- **RED evidence:** representative external exhaustive matches compile before
  the attributes are added and fail to compile afterward for the intended
  non-exhaustiveness reason.
- **Acceptance:** task review confirms the exact complete source set and exact
  two closed exceptions; no enum variant/field/parser change; public rustdoc
  remains warning-clean; the T1 oracle and all I01 tests pass in both feature
  modes.
- **Commands:**
  `cargo test -p surgeist-css --offline --no-default-features --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --doc`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`;
  `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps`;
  `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps`;
  `cargo test -p surgeist-css --offline --no-default-features`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `refactor: repair CSS enum evolution boundaries`.

### T3 Migrate Property-Coupled Declaration Inspection

- **Files/area:** `src/properties.rs`, declaration/value models in
  `src/syntax.rs`, shared declaration dispatch in `src/parser/mod.rs`, focused
  declaration/property tests, public consumers, and removal of the broad
  test-only `CssValue` conversion in `src/test_support.rs` and its callers.
- **Outcome:** extend every exact 179-row `property_schema!` entry with a unique
  `Css<SchemaVariant>PropertyValue` wrapper identifier. Each public private-field
  wrapper stores the authored value slice and a private I01 representation,
  exposes `as_css()`, and exposes
  `i01_subset() -> Option<&I01PayloadType>`; every C01 parsed ordinary value
  returns `Some`. The authored slice excludes parser-owned boundary trivia and
  the terminal importance annotation while preserving interior spelling,
  escapes, comments, case, commas, and block text.
- **Declaration shape:** replace public enum `CssKnownDeclaration` with a
  private-field parser-owned struct and private generated coupled discriminator.
  It exposes `property()`,
  `declared_value() -> CssKnownDeclaredValueRef<'_>`, and mutually exclusive
  `property_value()`, `global()`, and `substitution_dependent()` accessors.
  Public non-exhaustive `CssKnownDeclaredValueRef` has exactly `Property`,
  `Global`, and `SubstitutionDependent`; generated public non-exhaustive
  `CssKnownPropertyValueRef` has exactly one variant per schema row carrying
  that row's wrapper. Global/substitution branches never construct wrappers.
- **Coupling and construction:** property identity is derived from the private
  active discriminator; there is no separately mutable property field, public
  constructor, broad value bag, duplicate `V2` variant, or mismatch state.
  New official grammar may later replace a wrapper's private representation and
  return `None` from accurately named `i01_subset` without public break.
- **Test migration:** delete the broad crate-private `CssValue` adapter and its
  manual exhaustive conversion. Tests assert the exact property wrapper or
  private validated representation. The production schema macro generates the
  coupled property/wrapper/view/parser arms, while an independent 179-property
  public dispatch vector exercises every branch and asserts its concrete wrapper
  behavior. External consumers cover all 179 property view branches with
  wildcards and all three declared-value branches. Compile-fail tests prove
  private declaration/wrapper construction. Tests do not read Rust source or
  enforce implementation style through tokens, symbols, counts, or placement;
  reviewers inspect absence of a replacement broad value bag directly.
- **RED evidence:** new public consumer/schema tests fail on the old declaration
  enum and absent wrappers/views; a mismatch-construction compile-fail test is
  retained through GREEN.
- **Acceptance:** T1 oracle observables are identical in both feature modes;
  all 179 positive/boundary vectors retain exact typed/global/substitution,
  authored-text, importance, and diagnostic behavior; no parser language,
  catalog row/status, source, dependency, feature, or recovery delta occurs.
- **Commands:**
  `cargo test -p surgeist-css --offline coupled_declaration`;
  `cargo test -p surgeist-css --offline declaration_importance`;
  `cargo test -p surgeist-css --offline authored_declaration_value`;
  `cargo test -p surgeist-css --offline --test property_schema`;
  `cargo test -p surgeist-css --offline --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --test public_surface`;
  `cargo test -p surgeist-css --offline --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict`;
  `cargo test -p surgeist-css --offline --no-default-features --doc`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`;
  `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`;
  `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `refactor: stabilize CSS declaration value inspection`.

### T4 Public Migration, Final Inventory, And Cycle Audit

- **Files/area:** README/crate rustdoc only as required by the breaking API,
  external public consumers, and new SHA-free
  `plans/handoffs/P01-I02-C01-css-evolution-migration.md`.
- **Outcome:** document the exact old-to-new declaration matching pattern,
  wrapper authored-text and `i01_subset` semantics, wildcard requirements, the
  two closed enums, and the fact that parsing/recovery is unchanged. The static
  migration record enumerates every affected public enum and declaration API,
  the 179-wrapper generation rule, all field meanings, and root-owned facade/
  adapter/API-artifact/docs/test/gitlink work. It contains no final/local SHA and
  receives no post-review edit.
- **Audit:** the coordinator and task reviewer map every C01 acceptance predicate
  to exact source, compiler, and behavioral evidence. They directly inspect the
  exact final public-enum policy, 179-row schema coupling, migration
  completeness/SHA-freedom, unchanged `Cargo.toml`, crate-root unsafe
  prohibition, and configured command evidence. Rust tests remain limited to
  T1 oracle behavior, public consumers, catalog equality, and concrete property
  dispatch; they do not encode plans, handoffs, completion state, source text,
  or command manifests.
- **RED evidence:** public documentation examples and migration deliverables are
  absent before T4; their deterministic coordinator checks and public consumer
  commands pass after they are written.
- **Acceptance:** public examples compile without private modules or Debug/
  Display control flow; every C01 predicate maps to exact direct review,
  compiler, or behavioral evidence; no source edit follows holistic review.
- **Commands:**
  `cargo test -p surgeist-css --offline --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --test public_surface`;
  `cargo test -p surgeist-css --offline --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --test property_schema`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test i01_c01_observables`;
  `cargo fmt --check`; `git diff --check`;
  `rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`;
  followed by the exact cycle-wide completion gate below.
- **Intended commit:** `docs: record P01 I02 C01 evolution migration`.

## 4. Exact Completion Gate

After all task reviews and the administrative status-only completion commit, the
coordinator runs:

```sh
cargo check -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --doc
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo check -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
cargo test -p surgeist-css --offline --test i01_c01_observables
cargo test -p surgeist-css --offline --test public_surface
cargo test -p surgeist-css --offline --test catalog_inventory
cargo test -p surgeist-css --offline --test property_schema
cargo fmt --check
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps
git diff --check
rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs
! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .
! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'
cargo clean
test ! -d target
! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'
git status --short
```

The exact cycle range receives fresh holistic review. Publication then follows
the canonical authority-query/fetch, base reconciliation, local-main gate,
immutable lease-push, fresh fetch/query, ref-equality/readback, cleanliness,
target-absence, and process-hygiene gate. The candidate handoff reports full
SHAs, task/review spans, migration, command evidence, and root-only actions.
Only then may C02 be planned.

Stop if T1 cannot finitely encode a required observable, a C01 edit changes
accepted/recovered language or frozen report semantics, a third closed enum is
required, a property wrapper cannot preserve coupling/authored inspection, a
second breaking cycle becomes necessary, or any ownership/safety/dependency/
external-software boundary would be crossed. Also stop if any proposed test uses
source text, tokens, AST shape, symbols, call sites, code/file/test counts or
placement, or coordination records as proxy behavioral evidence.

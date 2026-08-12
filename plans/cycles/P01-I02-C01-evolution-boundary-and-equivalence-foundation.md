# P01-I02-C01 Evolution Boundary And Equivalence Foundation

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C01` |
| Owning repository | `surgeist-css` |
| Status | `reviewed` |
| Cycle base | `57b71354e83f70ff0665241eedfebe269f754fa4` |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `e290a7fef9bf6b6e9bde764140e5f7fac34156bb8f644d999e6bba58dc92ca2b`, especially P01.9 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `b861cc0df4d3d1d03f857d7ea4ff47e7f64faeaf20d1dc549614c14d3b186d49`, sections 1-3, 10, and 12.1 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `02955c03b5cc404aeb6e6a1724402d570d1736d06ccef74dc631084540188116`, I01/base identity only; no ledger grammar/status change in C01 |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `cd037ef80427fa5e378b634268ed135ba0f8e80ac0c8f0de5796a3ec59c14c5c`, entry `I02-C01` |
| Bounded outcome | Freeze a finite public I01 report oracle, perform the one authorized I02 public evolution/declaration representation repair without changing accepted/recovered input semantics, and publish a clean migration foundation for additive grammar cycles. |

## 2. Boundary, Compatibility, And Order

C01 is intentionally breaking and is the only breaking I02 cycle. It changes
public exhaustiveness and property-declaration inspection, not parser language,
recovery, diagnostics, positions, declarations' semantic coupling, features,
dependencies, or ownership. It adds no official grammar, source/catalog status,
property name, rule, descriptor, selector, query, value spelling, recovery
action, diagnostic code, dependency, feature, build script, generator, corpus,
root/sibling edit, external software, or unsafe.

Tasks are strictly serial. T1 must be independently reviewed and committed while
production is still the published I01 representation. T2 starts from reviewed
T1. T3 starts from reviewed T2. T4 starts from reviewed T3. No task may absorb a
later task's work, and a production-language difference found by the oracle is a
stop/reconcile event rather than a fixture update.

## 3. Tasks

### T1 Freeze The I01 Observable Equivalence Oracle

- **Files/area:** new `tests/fixtures/i01-c01-observables.tsv`, a new focused
  integration test and only narrowly required test-manifest modules; no
  production source or existing expectation edits.
- **Outcome:** hand-author the exact finite case union required by specification
  3.2: all 219 catalog positive/boundary identities, every case from the I01
  focused integration files named in the static I01 migration trace, and the
  default/`app-strict` public-surface cases. Stable case IDs point back to their
  owning test/vector identity. Every TSV row fixes entry point, feature mode,
  authored input, clean state, ordered retained rule/property stable IDs,
  retained authored value slice and importance where applicable, and every
  ordered diagnostic's code, `ErrorKind` root, stable payload identity,
  byte/line/UTF-16 position, complete span endpoints, and recovery action.
- **Independence and format:** the fixture is not emitted by the parser or
  generated from catalog/parser tables. A hand-authored manifest maps the exact
  source case IDs to fixture rows. The reader rejects unknown columns, duplicate
  IDs, malformed escapes, absent required observables, and noncanonical order.
  Mutation tests remove one case, one observable field, and one repeated
  diagnostic and must fail with the responsible case ID.
- **RED evidence:** focused tests first fail because the fixture/manifest and
  reader are absent; a deliberately incomplete fixture proves exact-union and
  mutation failures before the completed fixture is installed.
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

- **Files/area:** every owned Rust public enum, rustdoc/compile-fail public tests,
  and `tests/fixtures/i02-c01-public-enums.tsv`; no parser branch or value
  representation change.
- **Outcome:** mechanically inventory every `pub enum` in owned Rust at the T2
  head by stable `path:item` identity. Exactly `CssImportance` and
  `CssSupportStatus` are closed; every other public enum is
  `#[non_exhaustive]`. Nested/public visibility, macro-generated enums, feature-
  gated public enums, and the enums added later by T3/T4 participate in the
  final cycle inventory.
- **Evidence:** a source-structure test compares the hand-authored inventory to
  owned source in both directions and validates the exact exception set.
  Crate-root consumer compile-fail doctests prove representative evolving enums
  cannot be exhaustively matched; positive consumers use wildcards. Closed-enum
  consumers exhaustively match both `CssImportance` branches and all three
  `CssSupportStatus` branches. Omission/extra/wrong-exception mutations fail by
  stable item ID.
- **RED evidence:** the base source inventory reports every exhaustive evolving
  enum as a stable failure before attributes are added.
- **Acceptance:** exact inventory closure, no enum variant/field/parser change,
  public rustdoc remains warning-clean, the T1 oracle and all I01 tests pass in
  both feature modes.
- **Commands:**
  `cargo test -p surgeist-css --offline --no-default-features --test public_enum_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test public_enum_inventory`;
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
  private validated representation. A generated schema inventory proves all
  179 wrapper/view/property/parser IDs agree; external consumers cover all 179
  property view branches with wildcards and all three declared-value branches.
  Compile-fail tests prove private declaration/wrapper construction.
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
  external public consumers, final enum/wrapper inventories, focused C01 audit,
  and new SHA-free `plans/handoffs/P01-I02-C01-css-evolution-migration.md`.
- **Outcome:** document the exact old-to-new declaration matching pattern,
  wrapper authored-text and `i01_subset` semantics, wildcard requirements, the
  two closed enums, and the fact that parsing/recovery is unchanged. The static
  migration record enumerates every affected public enum and declaration API,
  the 179-wrapper generation rule, all field meanings, and root-owned facade/
  adapter/API-artifact/docs/test/gitlink work. It contains no final/local SHA and
  receives no post-review edit.
- **Audit:** stable tests name every C01 acceptance predicate, verify exact final
  public-enum and 179-wrapper inventories, mutation guards, migration
  completeness/SHA-freedom, T1 oracle completeness, default/feature public
  consumers, unchanged `Cargo.toml`, I01 catalog/status equality, crate-root
  unsafe prohibition, and the exact configured command-evidence manifest.
- **RED evidence:** the audit fails by stable predicate ID before final docs,
  migration record, and T3-added enum inventory entries exist.
- **Acceptance:** public examples compile without private modules or Debug/
  Display control flow; every C01 predicate maps to exact source and behavioral
  test evidence; no source edit follows holistic review.
- **Commands:**
  `cargo test -p surgeist-css --offline initiative_i02_c01_audit_`;
  `cargo test -p surgeist-css --offline --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --test public_enum_inventory`;
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
cargo test -p surgeist-css --offline initiative_i02_c01_audit_
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
external-software boundary would be crossed.

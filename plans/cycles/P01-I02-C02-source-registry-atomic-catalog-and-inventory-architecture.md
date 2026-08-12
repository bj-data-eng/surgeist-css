# P01-I02-C02 Source Registry, Atomic Catalog, And Inventory Architecture

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C02` |
| Owning repository | `surgeist-css` |
| Status | `in_progress` |
| Cycle base | `606ae77156d3085b6a8a551bc1f8d50c3ab885df` |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `e290a7fef9bf6b6e9bde764140e5f7fac34156bb8f644d999e6bba58dc92ca2b`, especially P01.9 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `cd6f00a46ab4c5eeccf1a0c2e312eb329d1c365fdb30b4fd99cae0cdd3ddaec8`, sections 4, 10, and 12.2-12.3 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `02955c03b5cc404aeb6e6a1724402d570d1736d06ccef74dc631084540188116`, sections 1-5 |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `1e1750cc0d21b9b08bee0f084c5b28c943c0a15949c2618bcce387c73c57ef7b`, entry `I02-C02` |
| Bounded outcome | Publish the immutable source registry, atomic baseline catalog, official coverage/exclusion slots, and one-owner implementation inventories required for later additive grammar cycles without changing accepted or recovered CSS. |

## 2. Boundary, Impacts, And Resolved Architecture

C01 candidate `606ae77156d3085b6a8a551bc1f8d50c3ab885df` is published and read back on
the authority remote. C02 is additive. It changes conformance metadata and
crate-private ownership inventories, not parser language, rule/declaration
models, diagnostics, recovery actions or spans, property coupling, features,
dependencies, or repository ownership. The complete C01 observable fixture at
SHA-256 `98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8`
must remain byte-identical and green in both feature modes.

Public additions are `CssSpecificationSourceId`, non-exhaustive
`CssSpecificationTier`, source-registry lookup, non-exhaustive
`CssExclusionReason`, `CssConformanceExclusionId`, exclusion metadata/lookup,
new `CssSpecificationSource` accessors, and
`CssFeatureMetadata::baseline_alias_targets()`. All new public fields are
private, identifiers are semantic newtypes with `as_str()`, and evolving enums
are non-exhaustive. Existing `url()` versus `repository_provenance()` XOR,
`feature_catalog()`, `feature_metadata()`, `property_metadata()`, stable I01 IDs,
and closed `CssSupportStatus` signatures remain source-compatible.

`CssFeatureMetadata` receives a private disposition distinguishing an atomic
parser-facing feature from a baseline aggregate alias. The four ledger-section
4 I01 aggregate IDs remain queryable aliases with immutable target slices and
their truthful aggregate I01 status, but receive no implementation owner or
behavior-vector owner. Every other I01 record migrates directly to one atomic
source/tier/production identity. Atomic target records added for already
accepted I01 behavior receive explicit public-parser cases before they may be
`Complete` or `Partial`.

Official ledger rows not yet recognized by the parser become crate-private
reserved coverage slots with their exact ID, kind, dated source, production,
future owning cycle/module, and no active feature ID. A reserved slot is not a
public feature record, does not carry `CssSupportStatus`, and does not change an
unknown spelling into recognized syntax. Its owning later cycle must add the
grammar or recognition, public feature row, implementation inventory identity,
and independent parser cases atomically. Existing recognized-but-unimplemented
rows may remain `RecognizedUnsupported`; existing supported subsets remain
`Partial`; only source-proven complete base productions remain `Complete`.

Official exclusions use a separate public immutable exclusion catalog and a
private `Excluded(CssExclusionReason)` coverage disposition. An exclusion is
never returned by `feature_metadata`, never owns parser dispatch or a behavior
vector, and never changes an authored spelling's diagnostic classification.
This preserves the additive API boundary while making every ledger row exactly
one active parser-facing slot, reserved parser-facing slot, or exclusion.

Every implementation-owning module exposes a crate-private static inventory of
the atomic stable IDs it implements. `property_schema!` remains the property
inventory authority; rules, declarations, descriptors, selectors, media
types/features, qualified rules, shared values/functions, and preserved
container extensions receive kind-specific inventories in their existing
owners. One central crate-private view may borrow those slices but may not
generate the catalog, ledger mapping, or parser cases.

Tests assert declared public metadata and real parser outcomes for explicit
stable IDs. They do not parse Rust source, compare catalog/implementation/test
owner sets or counts, mutate one inventory to prove completeness, encode plan
or review state, or treat file/code/test quantity as behavior. The coordinator
and reviewers directly reconcile ledger rows, sources, catalog records,
reserved slots, exclusions, implementation inventories, and independent cases.

No dependency, feature, build script, generator, generated artifact, or leaf
MSRV is added. The leaf manifest still declares no `rust-version`; root retains
integration-MSRV ownership. Root owns facade exposure, API-artifact refresh,
integration docs/tests, and gitlink promotion after the published C02 handoff.
All owned Rust remains free of `unsafe`.

## 3. Tasks

### T1 Establish The Behavior-Focused Catalog Evidence Boundary

- **Files/area:** `src/conformance.rs` test module,
  `tests/conformance_catalog.rs`, `tests/catalog_inventory.rs`, and
  `tests/initiative_i01_audit.rs`; no production model, parser, fixture, docs,
  or manifest edit.
- **Dependency:** published/read-back C01 base only.
- **Outcome:** remove or rewrite every assertion whose evidence is a catalog,
  property, implementation, or test count; owner-set equality; source/code
  inspection; mutation-based completeness proxy; or initiative/command state.
  Retain named public metadata assertions and real public-parser positive,
  invalid, and recovery outcomes. Existing explicit property cases remain
  independent authored inputs and do not become generated catalog evidence.
- **RED evidence:** not applicable to this behavior-preserving test-evidence
  correction. First record focused characterization GREEN for the named public
  parser outcomes, then make the minimal evidence refactor and prove those same
  outcomes unchanged. Direct diff inspection, not a new meta-test, proves proxy
  removal.
- **Acceptance:** no prohibited proxy assertion remains in the scoped tests;
  public metadata lookups, property positive/negative parsing, unsupported-rule
  diagnostics, recovery, and the complete C01 oracle remain behaviorally green;
  no production source outside the `#[cfg(test)]` block changes.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --test initiative_i01_audit`;
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test initiative_i01_audit`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict`;
  `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`;
  `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`;
  `cargo fmt --check`; `git diff --check`;
  `rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `test: keep CSS catalog evidence behavior-focused`.

### T2 Publish The Dated Source And Exclusion Metadata Model

- **Files/area:** `src/conformance.rs`, crate-root reexports/rustdoc, and focused
  public conformance consumers; no parser module or manifest edit.
- **Dependency:** T1 independently clean.
- **Outcome:** add private-field `CssSpecificationSourceId`, non-exhaustive
  `CssSpecificationTier`, and additive `CssSpecificationSource::{id, module,
  level, tier}` accessors. Add exact `specification_sources()` and
  `specification_source(id)` front doors containing every section 4.2 official
  source, every section 4.3 preserved extension source, and the three exact
  repository sources. Dated URLs and repository revision/path values are
  immutable and exact; lookup is case-sensitive and performs no trimming.
- **Exclusions:** add private-field `CssConformanceExclusionId`, non-exhaustive
  `CssExclusionReason`, private-field `CssExclusionMetadata`, and exact
  `conformance_exclusions()`/`conformance_exclusion(id)` front doors. Accessors
  expose ID, source, production, reason, and optional superseding stable IDs.
  Install the ledger-section 2.3-2.4 and section 5 exclusions without changing
  parser recognition or diagnostics.
- **RED evidence:** external consumers first fail to compile on the absent new
  public types/accessors/lookups; exact named source and exclusion cases then
  fail because no registry exists. Tests use the public artifact contract, not
  source text or inventory counts.
- **Acceptance:** all exact named registry cases expose the reviewed values;
  private construction compile-fails; public wildcard consumers compile;
  existing source XOR behavior remains; C01 parser observables are unchanged.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features --doc`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`;
  `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps`;
  `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps`;
  `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`;
  `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`;
  `cargo fmt --check`; `git diff --check`;
  `rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `feat: publish CSS source and exclusion registries`.

### T3 Atomize The Preserved I01 Catalog

- **Files/area:** `src/conformance.rs`, focused catalog/public consumers, and
  existing explicit public-parser cases; no parser dispatch or fixture edit.
- **Dependency:** T2 independently clean.
- **Outcome:** remap every one of the exact 219 I01 IDs to its dated official,
  preserved extension, or exact repository source and tier. Retain every ID and
  truthful status/subset/remainder/diagnostic identity. Convert exactly the four
  ledger-section 4 mixed IDs into private-disposition baseline aliases and add
  their exact immutable atomic targets. `baseline_alias_targets()` returns the
  target slice only for those aliases and an empty slice for atomic records.
- **Atomic additions:** add catalog metadata only for atomic targets whose I01
  accepted or boundary behavior already exists. Each addition names one source,
  tier, kind, production, implementation owner, and explicit public parser case.
  No later-cycle grammar, property spelling, diagnostic recognition, or status
  promotion is pulled into C02.
- **RED evidence:** independently authored public metadata and parser cases first
  fail on old repository/generic provenance and absent atomic target/alias APIs;
  the complete C01 behavior fixture is characterization evidence and must never
  be updated to obtain GREEN.
- **Acceptance:** all 219 I01 IDs remain exact-look-up compatible; each alias
  target union covers its prior accepted/boundary behavior; every atomic target
  case asserts public metadata plus a real positive, negative, or recovery
  outcome; the T1 behavior-focused evidence boundary remains intact; parser
  reports and fixture bytes are unchanged.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict`;
  `cargo test -p surgeist-css --offline --no-default-features --doc`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`;
  `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`;
  `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `refactor: atomize the I01 CSS catalog`.

### T4 Install Official Coverage Slots And Source Audit Records

- **Files/area:** conformance-owned private ledger/coverage tables and focused
  public exclusion/source tests; no parser behavior or property schema edit.
- **Dependency:** T3 independently clean.
- **Outcome:** encode every ledger section 2-3 unit and section 5 exclusion once
  as an exact private coverage record. Active records borrow an atomic feature
  ID; future records reserve their exact official ID, kind, source, production,
  intended owning module/cycle, and evidence boundary without a feature/status
  claim; excluded records borrow exact exclusion metadata. Encode the one legacy
  shorthand as a reserved C12 property-alias slot, not a schema alias or parser
  recognition change.
- **Source audit:** install the per-source informative audit row and exact
  supersession ownership from ledger sections 2.3-2.4 and 5. A source item is
  active/reserved or excluded, never both. The private tables neither read nor
  generate the persisted ledger.
- **Evidence:** tests exercise public source/exclusion records by stable ID and
  accessors. Complete 162/167 unit coverage, predecessor/exclusion equations,
  uniqueness, and lack of overlap are direct coordinator/task-review evidence,
  never a Rust owner-set/count/mutation test.
- **Acceptance:** direct reconciliation accounts for every ledger row with the
  exact dated source, kind, production, owner, future cycle when reserved, and
  truthful current disposition; public feature lookup exposes no reserved or
  excluded row; C01 behavior remains identical.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict`;
  `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`;
  `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `feat: install official CSS coverage slots`.

### T5 Assign One Atomic Implementation Owner And Behavioral Boundary

- **Files/area:** `src/properties.rs`, existing parser owner modules, a private
  conformance inventory view, and narrowly required explicit public-parser
  cases; no public parser-model or grammar change.
- **Dependency:** T4 independently clean.
- **Outcome:** expose crate-private stable-ID inventories from the module that
  implements each active atomic property, rule, declaration, descriptor,
  selector, media, qualified-rule, shared-value/function, and container
  extension production. Property identity continues to originate in
  `property_schema!`; no second property table or generated catalog appears.
  Baseline aliases, reserved slots, and exclusions have no implementation owner.
- **Behavioral boundary:** retain or add independently authored cases only where
  an active atomic implementation lacks public positive, invalid/boundary, or
  recovery evidence. A case names its stable ID and authored input, calls a
  public parser, and asserts returned syntax or complete structured diagnostics;
  it does not identify a source file/test owner or participate in a set join.
- **Acceptance:** direct review proves every active atomic parser path has one
  owning inventory entry and no alias/reserved/excluded entry does; explicit
  parser cases support every status claim; no inventory generates or validates
  another inventory; all C01 observables remain identical.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --test property_schema`;
  `cargo test -p surgeist-css --offline --no-default-features --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`;
  `cargo test -p surgeist-css --offline --no-default-features --test stylesheet_recovery`;
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test property_schema`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test structured_errors`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test stylesheet_recovery`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict`;
  `cargo test -p surgeist-css --offline --no-default-features --doc`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`;
  `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`;
  `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `refactor: assign atomic CSS implementation owners`.

### T6 Publish The C02 Reconciliation Record

- **Files/area:** README/crate rustdoc and new SHA-free
  `plans/handoffs/P01-I02-C02-conformance-reconciliation.md`; no production or
  test change unless the preceding direct reconciliation exposes a defect.
- **Dependency:** T5 independently clean and direct reconciliation complete.
- **Documentation outcome:** document the source/tier registry, exact support
  versus alias versus reserved versus exclusion meanings, atomic lookup, and
  unchanged parser/recovery behavior. The static reconciliation record maps the
  official ledger sections to source registry, catalog, reserved slots,
  exclusions, implementation inventories, and behavioral case areas; records
  the four alias target slices and root-owned follow-up; and contains no Git SHA,
  review/publication state, command manifest, or test-owner/count proxy.
- **Evidence:** deterministic writing checks prove exact paths, terminology,
  source IDs, aliases, exclusions, and absence of placeholders/workflow state.
  Direct coordinator/task review—not a Rust meta-test—performs the final ledger,
  catalog, owner, and case reconciliation.
- **Acceptance:** repository-wide tests obey `testing.md`; public examples compile
  using crate-root APIs; exact C01 fixture SHA and parser reports are unchanged;
  docs make no Complete claim for a reserved slot; root follow-up is explicit.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test conformance_catalog`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --test i01_c01_observables`;
  `cargo check -p surgeist-css --offline --no-default-features`;
  `cargo test -p surgeist-css --offline --no-default-features`;
  `cargo test -p surgeist-css --offline --no-default-features --doc`;
  `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`;
  `cargo check -p surgeist-css --offline --no-default-features --features app-strict`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`;
  `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`;
  `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps`;
  `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps`;
  `cargo fmt --check`; `git diff --check`;
  `! rg -n 'TODO|TBD|FIXME|\?\?\?' README.md src/lib.rs plans/handoffs/P01-I02-C02-conformance-reconciliation.md`;
  `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commit:** `docs: record the atomic CSS conformance foundation`.

## 4. Exact Completion Gate

After every task is independently clean and the status-only completion commit is
made, run:

```sh
cargo check -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --doc
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo check -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
cargo test -p surgeist-css --offline --test conformance_catalog
cargo test -p surgeist-css --offline --test catalog_inventory
cargo test -p surgeist-css --offline --test property_schema
cargo test -p surgeist-css --offline --test public_surface
cargo test -p surgeist-css --offline --test i01_c01_observables
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

The exact cycle range then receives holistic review. Publication follows the
canonical authority query/fetch, fast-forward reconciliation, local-main gate,
immutable leased push, fresh remote readback, ref equality, cleanliness,
target-absence, and process-hygiene requirements. The candidate handoff reports
full planning/task/review SHAs, exact source/catalog/slot/exclusion/inventory/case
reconciliation, public additions, unchanged parser behavior, command evidence,
and root-only actions. Only a published/read-back C02 candidate permits C03 JIT
planning.

Stop if any C02 change would alter accepted/recovered input, diagnostics,
positions, spans, recovery actions, the C01 fixture, existing stable-ID meaning,
closed support statuses, dependencies/features/MSRV, another repository, or
unsafe. Also stop if an official future row cannot be represented truthfully
without a premature support claim, or if a proposed test uses source text,
tokens, AST shape, symbols, calls, code/file/test counts or placement, owner-set
comparison, coordination records, or mutation of inventories as behavioral
evidence.

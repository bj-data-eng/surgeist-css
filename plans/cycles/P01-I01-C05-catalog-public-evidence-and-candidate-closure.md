# P01-I01-C05 Catalog, Public Evidence, And Candidate Closure

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I01-C05` |
| Owning repository | `surgeist-css` |
| Status | `reviewed` |
| Cycle base | `ccf80046fadcb4ad38ef228f63e0f51a993d369d` |
| Reviewed specification | `plans/specs/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `76b76a50a613aea26e1b790749a780f7d05efdfe57711c6b8dbf9a9fca2359d7`, sections 1 through 14, primarily 9, 11, 12.4, 12.5, 13 findings 2.18, 2.21, and 2.24, and initiative acceptance |
| Reviewed sequence | `plans/sequences/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `f3a65df04c5c5a4f6f02212fe4d69959b75bba1cdcf2fd12e5bfb012f2c4ec94`, entry `I01-C05 Catalog, Public Evidence, And Candidate Closure` |
| Bounded outcome | An independent 219-record I01 catalog, public metadata, product documentation, and tracked public-consumer evidence prove the final parser surface and close every I01 predicate without changing settled grammar. |

## 2. Boundary And Impacts

The published C04 base implements final I01 parsing, recovery, declarations,
diagnostics, positions, and feature behavior. C05 owns only catalog truth,
metadata lookup, implementation/catalog/vector cross-checks, rustdoc/README/
public-consumer closure, a static migration note, and the final candidate audit.
It does not add grammar, dependencies, features, root/sibling edits, generated
API artifacts, or I02/I03 work.

Public API is additive metadata plus documentation. Dependency/feature/MSRV are
unchanged (`default=[]`, `app-strict=[]`, edition 2024, no leaf rust-version).
Root owns API artifacts. No owned unsafe is permitted. The C04 base is published
and read back; `target` and target-tree processes are absent.

## 3. Tasks

### T1 Independent Catalog Types And 40 Non-Property Records

- **Files/area:** new `src/conformance.rs`, crate-root reexports, focused unit
  tests, and `tests/conformance_catalog.rs`.
- **Outcome:** hand-author exactly 40 independent non-property records: 26
  rule/descriptor/shared-value rows plus 14 selector/query rows from section
  9.2. Public private-field `CssFeatureMetadata`,
  `CssSpecificationSource`, closed `CssSupportStatus`, and non-exhaustive
  `CssFeatureKind` implement the complete record shape. `feature_catalog()`
  returns the final immutable slice (40 in T1, 219 after T2) and
  `feature_metadata(id)` performs exact lookup. Metadata exposes `id`, `kind`,
  `spelling`, `source`, `production`, `status`, `supported_subset`, and
  `unsupported_remainder`, plus
  `recognized_unsupported_code() -> Option<CssErrorCode>`; source exposes immutable URL or exact
  `4b288d6:<path>` provenance. Subset options are both `Some` exactly for Partial.
  The diagnostic-code option is `Some` exactly for RecognizedUnsupported; its
  full diagnostic identity is that root code plus the record's stable feature
  ID, which must equal the `CssFeatureId` carried by the emitted structured
  unsupported diagnostic.
- **RED evidence:** the public integration test first fails on absent APIs. A
  separately hand-authored 40-row table names exact ID/kind/spelling/source/
  production/status/subset invariants.
- **Acceptance:** exact set equality, uniqueness, accessor/rustdoc/private-
  construction, status semantics, positive vectors, and negative boundary
  vectors pass. Catalog records are not generated from parser/schema branches.
- **Commands:** `cargo test -p surgeist-css --offline conformance_catalog_`;
  `cargo test -p surgeist-css --offline --test conformance_catalog`;
  `cargo check -p surgeist-css --offline`.
- **Dependencies:** C04 only.
- **Intended commit:** `feat: add independent CSS conformance catalog`.

### T2 179 Property Records And Three-Way Closure

- **Files/area:** property records in `src/conformance.rs`, schema implementation
  inventory in `src/properties.rs`, separately hand-authored vector manifests
  under `tests/`, focused tests, and `tests/catalog_inventory.rs`.
- **Outcome:** add exactly 179 independent property records for 219 total.
  `property_metadata` performs ASCII-case-insensitive canonical/alias lookup and
  returns None for custom/unknown names. `CssPropertyMetadata` exposes
  `feature`, `property`, `canonical_name`, and `aliases`.
- **Independent owners:** (1) the hand-authored catalog owns metadata truth;
  (2) crate-private parser/property implementation inventories own dispatch IDs;
  (3) separately hand-authored test manifests own positive/negative vector IDs.
  None is generated from another; tests compare all three bidirectionally.
- **RED evidence:** inventory tests first fail at 40 versus 219 and absent
  property lookup. Omission/extra/duplicate mutation guards prove drift detection.
- **Acceptance:** exact 40 + 179 closure; property name/ID/alias uniqueness;
  mixed-case/custom/unknown behavior; every Complete/Partial record has an
  implementation and positive vector; every Partial/RecognizedUnsupported has
  a negative boundary vector; every RecognizedUnsupported vector asserts both
  emitted root code and payload feature ID equal its record's diagnostic
  identity; no inventory/vector lacks a catalog record.
- **Commands:** `cargo test -p surgeist-css --offline catalog_inventory_`;
  `cargo test -p surgeist-css --offline --test catalog_inventory`;
  `cargo test -p surgeist-css --offline --test property_schema`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict`.
- **Dependencies:** T1 and the C02 schema.
- **Intended commit:** `feat: publish CSS property support metadata`.

### T3 Public Documentation And Consumer Closure

- **Files/area:** crate docs in `src/lib.rs`, rustdoc on final public items,
  `README.md`, crate-root-only integrations, and `tests/public_surface.rs`.
- **Outcome:** docs explain browser recovery, clean reports, diagnostics/spans/
  actions, coordinates, importance, custom/substitution preservation, support
  status, style attributes, `app-strict`, and non-responsibilities. Minimal sheet
  and style examples compile. Public tests prove wildcard matching, private
  construction, every final accessor, default validator absence, enabled
  validators, non-BMP coordinates, and all ten recovery actions.
- **RED evidence:** public-surface tests/doctests fail on missing metadata and
  final guidance; deterministic README checks reject stale strict-only claims.
- **Acceptance:** examples never infer validity from empty syntax or use
  Display/Debug as control flow; default/feature doctests and tracked public
  consumers are green with warnings denied.
- **Commands:** `cargo test -p surgeist-css --offline --test public_surface`;
  `cargo test -p surgeist-css --offline --no-default-features --doc`;
  `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`;
  `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps`.
- **Dependencies:** T1 and T2.
- **Intended commit:** `docs: close I01 CSS public guidance`.

### T4 Initiative Audit And Static Migration Note

- **Files/area:** focused audit tests, source/docs corrections only when settled
  I01 behavior is unchanged, and a SHA-free note under `plans/handoffs/`.
- **Outcome:** audit every section 14 predicate and findings 2.5, 2.6, 2.15,
  2.18–2.25 against exact source/tests. The static migration note enumerates
  removed/renamed APIs, candidate field meanings, and root-owned pointer/facade/
  API-artifact/docs/test actions, but contains no final/local SHA and receives no
  post-review edit. The canonical full-SHA `CRATE_CANDIDATE` report is emitted
  only after holistic review, publication, cleanup, and remote readback.
- **RED evidence:** `initiative_i01_audit_` fails by stable predicate/finding ID
  for missing evidence rather than relying only on prose search.
- **Acceptance:** every predicate maps to exact source plus a passing test; the
  migration note is complete/SHA-free; no semantic parser or cross-repo change
  occurs and no source edit follows holistic review.
- **Commands:** `cargo test -p surgeist-css --offline initiative_i01_audit_`;
  `cargo test -p surgeist-css --offline --test catalog_inventory`; full matrix.
- **Dependencies:** T1–T3.
- **Intended commit:** `docs: record final I01 CSS migration`.

## 4. Completion

Every task and the exact cycle receive fresh independent reviews. Exact final
commands are:

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
cargo test -p surgeist-css --offline --test public_surface
cargo test -p surgeist-css --offline initiative_i01_audit_
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

Publication follows the canonical gate: query/fetch authority `main`, require
the recorded C04 base or reconcile/re-review, rerun the matrix on local main,
query again, lease-push the immutable candidate, fetch/query fresh, and prove
local/tracking/observed main equality and candidate reachability. No source edit
follows holistic review. The canonical candidate report then supplies full base/
head/task ranges/reviews/commands/API/root-action evidence. Only after this
`READY_FOR_ROOT` handoff may I02 be specified JIT.

Stop if catalog truth requires grammar change, records lack reviewed provenance,
public guidance contradicts behavior, root/API artifacts would be edited here,
external software is missing, or unsafe would be required.

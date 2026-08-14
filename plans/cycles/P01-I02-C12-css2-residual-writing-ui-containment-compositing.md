# P01-I02-C12 CSS2 Residual, Writing, UI, Containment, And Compositing Properties

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C12` |
| Owning repository | `surgeist-css` |
| Status | `reviewed` |
| Cycle base | `371d4bb13f30b31abd9d4e85a9c95dccb9af05e2` |
| Published prerequisite | C11 `371d4bb13f30b31abd9d4e85a9c95dccb9af05e2`, fetched and read back |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `87f6a94b893ffa416c6ff451575f0d5a21b4aa136e7bcd391cd6c0ce8810a2ae` |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `3f93c7f6c3656ebe0b33b8bf9c32e458f306f294d1a969a86df375a5858b1710`, sections 4.2-4.4, 8-10, 11 finding 2.2, 12 |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, semantic SHA-256 `56bcf0340320339454e4ae1aa0b45a7ad2e37e03930e9e3bf7665f8ce4cbb15a`, entry `I02-C12` |
| Bounded outcome | Complete the 27 reserved C12 property/alias/shared-value rows across CSS2 residuals, Writing Modes 3, UI3, Containment 1, Transforms 1, and Compositing 1 with public grammar, recovery, metadata, and documentation evidence. |

## 2. Boundary And Current Evidence

C12 owns only authored property syntax, typed authored values, parser recovery,
public accessors, official metadata, vectors, and docs. It excludes cascade,
layout, painting, pagination, hit testing, resource loading, evaluation,
serialization, CSSOM, root adapters, and generated API artifacts. Existing
extension properties remain truthful and are not promoted by this cycle.

The reserved C12 coverage records are exactly 27: 25 property/alias rows and
two shared-value rows. The property/alias rows are:

- CSS2 residuals: `border-collapse`, `border-spacing`, `caption-side`, `clip`,
  `empty-cells`, `orphans`, `page-break-after`, `page-break-before`,
  `page-break-inside`, `quotes`, `table-layout`, `widows`, and `word-spacing`.
- Writing Modes 3: `text-combine-upright`, `text-orientation`, and
  `unicode-bidi`, plus the explicit CSS2 alias
  `glyph-orientation-vertical -> text-orientation`.
- UI3: `caret-color`, `outline-offset`, and `resize`.
- Containment 1: `contain`.
- Transforms 1: `transform-box`.
- Compositing 1: `background-blend-mode`, `isolation`, and `mix-blend-mode`.

The two shared-value rows are O-BOX3 `official.value.box-edge-keywords` and
O-COMPOSITING1 `official.value.blend-mode`. Their keyword/list grammars,
adjacent-grammar rejection, recovery vectors, exact source fragments, and
Complete metadata are part of this cycle; they are not silently subsumed by a
property row.

Selected sources and fragments are already frozen in `src/conformance.rs` and
the reviewed ledger: O-CSS2, O-WRITING3, O-UI3, O-CONTAIN1, O-TRANSFORMS1,
and O-COMPOSITING1. No source URL, property spelling, alias target, or owner
is inferred from parser implementation.

All public additions are additive: private fields, checked constructors or
parser-owned values, and non-exhaustive evolving enums. Existing projections
remain source-compatible. No dependency, feature, manifest, MSRV, root,
sibling, generated-artifact, or unsafe change is authorized.

## 3. Grammar Contract

Each property receives its complete selected grammar, including global and
substitution branches, keyword families, numeric boundaries, list/shorthand
separators, adjacent-grammar rejection, exact typed accessors, and recovery.
The CSS2 residuals cover table models, clipping, page-break keywords, quotes,
and word spacing without implementing layout or pagination semantics. Writing
Modes covers directionality/text-combination values and the legacy alias maps
explicitly to `text-orientation`. UI covers caret color, outline offset, and
resize domains. Containment is the selected Containment 1 `contain` grammar;
Transforms covers only `transform-box`; Compositing covers blend-mode lists,
isolation, and background-blend-mode lists, plus the shared O-COMPOSITING1
blend-mode value. The O-BOX3 edge-keyword value is independently closed as a
shared-value record and is not attached to any particular C12 property.

Invalid values must report the property-specific `InvalidPropertyValue` (or
the existing typed diagnostic), exact responsible position/span, and
`DropDeclaration`, retaining later siblings. Unknown properties remain
`UnknownProperty`. Valid duplicate declarations preserve ordinary cascade-free
source order. Every new recursive/list grammar exercises EOF, repeated
failures, non-BMP coordinates, and 255/256/257 depth boundaries where
applicable, with validator parity in both feature modes.

## 4. Tasks

Every worker records `task_base_sha` at assignment and makes exactly two
commits: a base-compilable public behavioral or named-metadata RED, then its
implementation/docs commit. Tests use only public CSS front doors or directly
named public metadata; they never parse Rust source/files, inspect symbols,
tokens, ASTs, registrations, call sites, owner sets/counts, workflow state,
or incidental calls.

After each focused loop run:

```sh
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --doc
cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
cargo fmt --check
git diff --check "${task_base_sha}..HEAD"
```

Each task receives a fresh independent task review before the next dependent
task begins.

### T1 CSS2 residual table, page, generated, and text properties

- **Area:** the 13 CSS2 residual canonicals listed above, the shared O-BOX3
  box-edge keyword value, their typed values,
  declaration recovery, and public vectors; no pagination/layout semantics.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test residual_property_grammars css2_residual_properties_retain_typed_values -- --exact`; at least one valid selected property is still dropped or recognized unsupported.
- **Acceptance:** complete value domains, globals/substitutions, exact negative
  diagnostics, aliases where applicable, sibling/EOF/non-BMP recovery, and
  public accessors for all 13 rows.
- **Commits:** `test: specify CSS2 residual properties`; `feat: add CSS2 residual property grammars`.

### T2 Writing Modes and legacy orientation alias

- **Dependency:** T1 independently CLEAN.
- **Area:** `text-combine-upright`, `text-orientation`, `unicode-bidi`, and
  explicit `glyph-orientation-vertical` mapping; typography parser/value models.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test writing_modes_grammars writing_modes_and_legacy_alias_are_typed -- --exact`.
- **Acceptance:** exact keyword/number domains, alias identity and authored
  mapping, whole-property globals, invalid-value recovery, source coordinates,
  validator parity, and no name-equivalent schema alias shortcut.
- **Commits:** `test: specify writing modes properties`; `feat: add writing modes property grammars`.

### T3 UI, containment, transforms, and compositing properties

- **Dependency:** T1 independently CLEAN; shared typed value primitives available.
- **Area:** `caret-color`, `outline-offset`, `resize`, `contain`, `transform-box`,
  `background-blend-mode`, `isolation`, `mix-blend-mode`, and the shared
  O-COMPOSITING1 blend-mode value.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test ui_containment_compositing_grammars residual_ui_containment_and_compositing_properties_are_typed -- --exact`.
- **Acceptance:** complete selected-source grammar, ordered list/separator and
  keyword domains, exact invalid recovery and sibling retention, globals and
  substitutions, coordinates, and strict parity. No paint/layout semantics.
- **Commits:** `test: specify UI containment and compositing properties`; `feat: add UI containment and compositing property grammars`.

### T4 Cross-family recovery and boundary matrix

- **Dependency:** T2 and T3 independently CLEAN.
- **Area:** narrowly required shared declaration/value recovery seams and
  public tests spanning all 27 rows, repeated failures, nested contexts,
  non-BMP coordinates, EOF, and 255/256/257 boundaries.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test c12_property_recovery c12_residual_recovery_preserves_siblings_and_boundaries -- --exact`.
- **Acceptance:** one diagnostic per smallest invalid unit, exact offsets/spans,
  property identity, sibling retention, ordinary/app-strict parity, no panic,
  and no weakening or masking of existing I01/C01 fixtures.
- **Commits:** `test: specify C12 property recovery`; `feat: harden C12 property recovery`.

### T5 Official metadata, catalog vectors, docs, and handoff

- **Dependency:** T1-T4 independently CLEAN.
- **Area:** activate exactly the 27 C12 official property/alias/shared-value
  rows, direct named metadata/vector tests, README/rustdoc/doctests, and
  SHA-free handoff
  `plans/handoffs/P01-I02-C12-css2-residual-writing-ui-containment-compositing.md`.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog c12_property_metadata_is_truthful -- --exact`; behavior is present while named rows remain Reserved.
- **Acceptance:** exact source/fragments, Complete status for every canonical,
  explicit alias, and both shared-value records, truthful
  supersession/exclusion totals, no unsupported metadata lies, public
  docs/consumer examples, and handoff containing only
  product fixture digests (no Git SHA).
- **Commits:** `test: specify C12 property metadata`; `docs: publish C12 property closure`.

## 5. Completion, Publication, And Blockers

After all five task ranges are independently CLEAN, make a separate status-only
`complete` commit. Run this exact final gate:

```sh
cargo check -p surgeist-css --offline --no-default-features
cargo check -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --doc
cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features --features app-strict
cargo fmt --check
git diff --check 371d4bb13f30b31abd9d4e85a9c95dccb9af05e2..HEAD
shasum -a 256 tests/fixtures/i01-c01-observables.tsv
git diff --unified=0 371d4bb13f30b31abd9d4e85a9c95dccb9af05e2..HEAD -- tests/fixtures/i01-c01-observables.tsv
git status --short --branch
ps -axo pid=,command=
```

Also run the canonical owned-Rust executable-unsafe scan. After holistic
review, run `cargo clean --offline`, prove `target/` absent, the worktree
clean, and no `surgeist-css` Cargo/Rust process. Fetch C11 and publish with the
canonical non-force fast-forward lease gate, then fetch/read back and prove
local, tracking, and remote main equality. Any new frozen-oracle contradiction,
unresolved source ownership, unsafe requirement, external acquisition,
root/sibling mutation, or scope expansion returns to P01 reconciliation.

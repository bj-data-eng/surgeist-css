# P01-I02-C07 Grid Repetition And Keyframe Structure

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C07` |
| Owning repository | `surgeist-css` |
| Status | `in_progress` |
| Cycle base | `4ec24e2bd09bbd937b85e059980970ec4ddcfc6e` |
| Published prerequisite | C06 `597265b574be01c88a3ce559cc2bc07e02791da3`, fetched and read back before reconciliation |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `fc53da090fdef779582bb4480d3e7943816470977af6abefe3468b2c10d3e064`, P01.6, P01.7, P01.10 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `b7b8aa699c05943ab3972087e8e64c91b887ad1f7b27114ec8014dd7e7c1005a`, sections 3.1, 3.3, 4.3-4.4, 5, 8.2 Grid, 10, 11 findings 2.14/2.17, and 12 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `09ecbf2dcaafbd402b24642f1244ce0be3568fd8a85b993c0218e2e7c0deac6d`; no C07 official-ledger delta |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `1ed8ab6f8a49da55f1dd1caab6df484444ba0b07ab2f070b105ba750300ad139`, entry `I02-C07` |
| Reconciliation review | Exact range `597265b574be01c88a3ce559cc2bc07e02791da3..4ec24e2bd09bbd937b85e059980970ec4ddcfc6e`, fresh planning review `CLEAN` |
| Bounded outcome | Close findings 2.14 and 2.17 with structurally valid current Grid repetition and source-ordered empty/duplicate keyframe structures, applying only the seven reviewed source-backed oracle corrections. |

## 2. Boundary And Decisions

The exact dated sources are Reliable Grid 2 at
<https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/> and Interop Animations 1
at <https://www.w3.org/TR/2023/WD-css-animations-1-20230302/>. Moving drafts are
discovery aids only. C07 owns Grid repeat structure, the six existing repeat
consumers, keyframe empty/duplicate structure, their recovery consequences,
the reviewed fixture correction, exact extension metadata, focused docs, and a
leaf handoff. It does not complete Grid 2, subgrid/name-repeat, keyframe
percentage calculations, empty string animation names, animation-property
ignoring inside keyframes, cascade, interpolation, timeline evaluation, layout,
serialization, root adapters, sibling code, or generated API artifacts.

Existing public `CssGridTrackBreadth`, `CssGridTrackSize`,
`CssGridTrackComponent`, `CssGridTrackList`, `CssGridRepeatCount`,
`CssGridRepeat`, `CssGridTemplate`, and `CssGrid` remain source-compatible I01
payloads. Their permissive public construction shapes are not the current
parser-owned validity boundary. Add distinct private-field, non-exhaustive
current authored models for track breadth/size, fixed size, non-recursive repeat
content, integer track repeat, integer fixed repeat, auto repeat, general versus
auto track lists, and the two aggregate Grid values. Property wrappers expose a
`current()` accessor and retain `i01_subset()` with an exact projection for every
conforming previously accepted value. Rejected nonconforming inputs produce no
declaration or projection. No parallel property variant or signature change is
allowed.

The Grid parser implements these structural languages:

- integer track-repeat content contains one or more track sizes with optional
  line names and never another repeat;
- integer fixed-repeat and auto-repeat content contains one or more fixed sizes
  with optional line names and never another repeat;
- fixed size distinguishes fixed breadth, `minmax(fixed, track-breadth)`, and
  `minmax(inflexible, fixed)`; a bare fraction, intrinsic size, `auto`, or
  `fit-content()` is not fixed, while `minmax(10px, 1fr)` and
  `minmax(auto, 10px)` are fixed and `minmax(auto, 1fr)` is not;
- an auto track list contains exactly one auto-repeat and only fixed sizes or
  fixed repeats around it; a general track list contains integer track repeats
  but no auto-repeat;
- `grid-auto-rows` and `grid-auto-columns` accept track sizes, never repeat.

Non-negative literal fixed breadths and existing typed length/percentage math
retain the C03 literal-versus-computed range boundary. C07 does not evaluate a
calculation. Failures use the existing `InvalidPropertyValue` identity,
first-responsible parser position, complete declaration span,
`DropDeclaration`, sibling retention, progress, and ordinary/`app-strict`
parity.

Keyframe rules and declaration blocks may be empty. Duplicate selector blocks,
duplicate equivalent offsets across blocks, and repeated equivalent selectors
within one selector list remain in authored order; the crate does not sort,
merge, cascade, or deduplicate them. Dropping an invalid declaration may leave a
valid empty block and rule, so obsolete parent `DropKeyframeBlock` and
`DropAtRule` diagnostics disappear. A genuinely invalid selector or structural
child still drops the smallest invalid block, but an otherwise valid rule with
zero retained blocks remains. Existing keyframe names, literal percentage
selectors, declaration importance meaning, positions, and compatibility types
remain unchanged. `baseline.rule.keyframes` stays truthfully `Partial` for the
unselected Animations grammar named above.

## 3. Exact Oracle Correction

The fixture before C07 has SHA-256
`98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8`.
The reviewed replacement has SHA-256
`99bbb897710969949d7b596d14fbd352d5d3121a6c4cf663b8ca100154057f8b`.
Both have 975 lines. Only the following stable IDs change; their entry point,
feature mode, and authored input remain byte-identical, and every other row is
byte-identical:

| Stable scenario ID | Exact replacement observable |
| --- | --- |
| `catalog.property.baseline.property.grid.positive` | not clean; no retained declaration/value/projection; one `InvalidPropertyValue` for `baseline.property.grid`, responsible `Dimension:1fr` at byte/UTF-16 column 46, full span `0..50`, `DropDeclaration` |
| `focused.property-schema.baseline.property.grid.important` | retain only the style rule; no declaration/value/projection; the same property error at byte/column 54, full span `8..70`, `DropDeclaration` |
| `focused.property-schema.baseline.property.grid.ordinary` | retain only the style rule; no declaration/value/projection; the same property error at byte/column 54, full span `8..59`, `DropDeclaration` |
| `focused.importance.05` | retain `baseline.rule.keyframes`; retain only the existing `InvalidDeclarationAnnotation` at byte/column 36, span `25..47`, `DropDeclaration` |
| `focused.importance.06` | retain `baseline.rule.keyframes`; retain only the existing keyframe-custom annotation error at byte/column 36, span `25..47`, `DropDeclaration` |
| `focused.nested-structural.keyframes-child-loss` | retain keyframes, later style rule, and blue color declaration/projection; retain only the existing `UnknownProperty:mystery` at byte/column 25, span `25..36`, `DropDeclaration` |
| `focused.structural.misc.03` | retain keyframes, later style rule, and red color declaration/projection; retain only the existing `UnknownProperty:mystery` at byte/column 25, span `25..36`, `DropDeclaration` |

The behavior task commits hand-author these exact rows before production changes,
so the public fixture reader and focused tests execute RED on the task base.
No Rust test asserts a digest, reads repository source, derives an expectation,
masks a corrected row, or compares owner sets/counts. Task review compares the
fixture diff directly and rejects any eighth changed row. Duplicate stale test
oracles are replaced with source-backed behavior, never merely deleted.

## 4. Metadata Delta

No official source, ledger count, exclusion, baseline alias, dependency, or
feature changes. Add one atomic shared-value record:

| Stable ID | Kind/source/production | Status, subset, and remainder | Owner and named behavior |
| --- | --- | --- | --- |
| `ext.value.grid-repeat` | Value; `R-GRID2`; `#repeat-notation` | `Partial`: non-recursive integer track/fixed repeats and one fixed-size auto-repeat; remainder is subgrid name-repeat and other unselected Grid 2 forms | `crate::parser::grid`; `grid_repeat_models_reject_invalid_cross_products` |

Add `ext.value.grid-repeat` to the grid parser's shared-value implementation
inventory. The existing repeat consumers
`baseline.property.grid-template-rows`, `.grid-template-columns`,
`.grid-template`, `.grid-auto-rows`, `.grid-auto-columns`, and `.grid` remain
`Partial`, source `R-GRID2`, with their subset narrowed to the C07 structural
repeat grammar and their remainder naming untouched Grid grammar. The existing
`baseline.rule.keyframes` remains `Partial`, source `I-ANIMATIONS1`, production
`#keyframes`; its subset adds empty and duplicate authored structures and its
remainder names calculation selectors, string-name and declaration-processing
grammar not selected by C07. `official.value.calc` and animation property rows
do not change.

## 5. Tasks

At assignment start and before any test or production edit, each worker runs
`task_base_sha="$(git rev-parse HEAD)"`, records that immutable SHA in its task
result, and keeps it as the task-base value. After the task-specific focused
loop, each worker runs this exact common GREEN tail with that recorded value:

```sh
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
cargo fmt --check
git diff --check "${task_base_sha}..HEAD"
```

Tests parse authored CSS only through the public crate API; parsing or
inspecting Rust source, files, symbols, test or catalog registration, owner
sets/counts, workflow state, or call counts is prohibited repository-wide.

### T1 Publish Structurally Valid Grid Repetition

- **Files/area:** current Grid models in `src/syntax.rs`; `src/parser/grid.rs`;
  the six exact property wrappers/aggregate representations and dispatch in
  `src/properties.rs`; the three Grid fixture rows; stale Grid behavioral
  oracles; new `tests/grid_repetition.rs` plus `numeric_domains`,
  `property_schema`, `public_surface`, `structured_errors`,
  `source_coordinates`, and `i01_c01_observables`. No keyframe/catalog/docs edit.
- **RED:** first commit is base-compilable public behavior using only existing
  APIs and fails because nested repeat or `repeat(auto-fit, 1fr)` is retained;
  the three hand-authored fixture replacements fail on the same base. New-symbol
  compile evidence may follow only after this executable RED. Exact RED command:
  `cargo test -p surgeist-css --offline --no-default-features --test grid_repetition grid_repeat_models_reject_invalid_cross_products -- --exact`.
  Exact fixture-reader RED command:
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables authored_css_cases_match_frozen_public_report_observables -- --exact`.
- **Acceptance:** all structural languages in section 2; one-token invalid
  mutations; all direct/aggregate consumers; exact current/I01 projection;
  calculation and 255/256/257 depth boundaries; non-BMP coordinates; complete
  diagnostics, repeated progress, sibling retention, and strict parity; exactly
  three fixture rows changed at this task.
- **Focused targets:** `grid_repetition`, `numeric_domains`, `property_schema`,
  `public_surface`, `structured_errors`, `source_coordinates`,
  `i01_c01_observables`, executed exactly as:

  ```sh
  for target in grid_repetition numeric_domains property_schema public_surface structured_errors source_coordinates i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in grid_repetition numeric_domains property_schema public_surface structured_errors source_coordinates i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  ```
- **Intended commits:** `test: specify structurally valid Grid repetition`;
  `feat: type Grid repetition`.

### T2 Preserve Empty And Duplicate Keyframes

- **Files/area:** keyframe structure in `src/syntax.rs` and
  `src/parser/keyframes.rs`; the four keyframe fixture rows; replace only stale
  keyframe behavior oracles; new `tests/keyframe_structures.rs` plus
  `nested_structural_recovery`, `specialized_recovery_boundaries`,
  `declaration_importance`, `public_surface`, `structured_errors`,
  `source_coordinates`, `app_strict_parity`, and `i01_c01_observables`. No Grid,
  calculation-selector, catalog, or docs edit.
- **RED:** first commit is base-compilable public behavior and fails because an
  empty rule/block or duplicate offset is rejected; the four exact fixture
  replacements also execute RED before production changes. Exact RED command:
  `cargo test -p surgeist-css --offline --no-default-features --test keyframe_structures keyframes_preserve_empty_and_duplicate_authored_structure -- --exact`.
  Exact fixture-reader RED command:
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables authored_css_cases_match_frozen_public_report_observables -- --exact`.
- **Acceptance:** empty rules and blocks; duplicate blocks and list-local/cross-
  block offsets in authored order; declaration and invalid-block smallest-unit
  recovery; retained empty parents; positions, depth, EOF, repeated failures,
  non-BMP, ordinary/strict parity; final fixture digest exactly
  `99bbb897710969949d7b596d14fbd352d5d3121a6c4cf663b8ca100154057f8b`, with
  only the seven reviewed IDs changed from C06.
- **Focused targets:** `keyframe_structures`, `nested_structural_recovery`,
  `specialized_recovery_boundaries`, `declaration_importance`, `public_surface`,
  `structured_errors`, `source_coordinates`, `app_strict_parity`,
  `i01_c01_observables`, executed exactly as:

  ```sh
  for target in keyframe_structures nested_structural_recovery specialized_recovery_boundaries declaration_importance public_surface structured_errors source_coordinates app_strict_parity i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in keyframe_structures nested_structural_recovery specialized_recovery_boundaries declaration_importance public_surface structured_errors source_coordinates app_strict_parity i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  ```
- **Intended commits:** `test: specify empty and duplicate keyframes`;
  `feat: preserve authored keyframe structure`.

### T3 Publish Grid And Keyframe Metadata, Docs, And Handoff

- **Files/area:** `src/conformance.rs`; inventory-only Grid parser declaration;
  explicit named public metadata tests; README and crate rustdoc; new SHA-free
  `plans/handoffs/P01-I02-C07-grid-repetition-and-keyframe-structure.md`. No
  grammar/model/property/fixture/manifest/root/sibling/artifact edit.
- **RED:** base-compilable explicit metadata behavior fails because
  `ext.value.grid-repeat` is absent and the six property/keyframe subset and
  remainder boundaries are stale, while paired independent parser behavior
  already passes. No set/count/completeness proxy. Exact RED command:
  `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog grid_and_keyframe_metadata_matches_preserved_boundaries -- --exact`.
- **Acceptance:** section 4 exact IDs/source/fragments/status/owner/named behavior;
  unrelated rows unchanged; docs state compatibility/current boundaries,
  source-backed oracle correction, and exclusions; handoff contains no SHA,
  review/publication/completion state, or command manifest.
- **Focused targets:** `conformance_catalog`, `catalog_inventory`,
  `grid_repetition`, `keyframe_structures`, `property_schema`, `public_surface`,
  `structured_errors`, `source_coordinates`, and `i01_c01_observables`, followed
  by doctests and warnings-denied rustdoc, executed exactly as:

  ```sh
  for target in conformance_catalog catalog_inventory grid_repetition keyframe_structures property_schema public_surface structured_errors source_coordinates i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in conformance_catalog catalog_inventory grid_repetition keyframe_structures property_schema public_surface structured_errors source_coordinates i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  cargo test -p surgeist-css --offline --no-default-features --doc
  cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
  RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features
  RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features --features app-strict
  ! rg -n 'TO''DO|TB''D|FIX''ME|\?''\?''\?' README.md src/lib.rs plans/handoffs/P01-I02-C07-grid-repetition-and-keyframe-structure.md
  ```
- **Intended commits:** `test: specify Grid and keyframe metadata`;
  `docs: publish Grid and keyframe structure`.

## 6. Final Cycle Gate And Publication

Immediately after T3 is independently clean, the coordinator makes the separate
status-only commit changing this plan from `in_progress` to `complete`. With
that commit already in the candidate range and before holistic review, the
coordinator runs this exact gate at the candidate head:

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
git diff --check 4ec24e2bd09bbd937b85e059980970ec4ddcfc6e..HEAD
shasum -a 256 tests/fixtures/i01-c01-observables.tsv
git diff --unified=0 4ec24e2bd09bbd937b85e059980970ec4ddcfc6e..HEAD -- tests/fixtures/i01-c01-observables.tsv
rg -n 'unsafe|unsafe_code' --glob '*.rs' src tests
! rg -n 'TO''DO|TB''D|FIX''ME|\?''\?''\?' README.md src/lib.rs plans/handoffs/P01-I02-C07-grid-repetition-and-keyframe-structure.md
git status --short --branch
ps -axo pid=,command=
```

The fixture checksum output must be exactly
`99bbb897710969949d7b596d14fbd352d5d3121a6c4cf663b8ca100154057f8b`;
direct diff inspection must show only the seven section 3 IDs. The Rust scan is
classified directly: the crate-level `forbid(unsafe_code)` and authored CSS
keyword strings are not executable unsafe; any other match blocks review. The
process listing must contain no repository Cargo, rustc, rustdoc, or
`surgeist_css` process. The worktree must be clean. A fresh holistic reviewer
then reviews exact range
`4ec24e2bd09bbd937b85e059980970ec4ddcfc6e..HEAD`.

Only after holistic `CLEAN`, run:

```sh
cargo clean --offline
test ! -e target
git status --short --branch
ps -axo pid=,command=
candidate_sha="$(git rev-parse HEAD)"
test "$candidate_sha" = "$(git rev-parse main)"
git merge-base --is-ancestor 597265b574be01c88a3ce559cc2bc07e02791da3 "$candidate_sha"
git fetch origin main
test "$(git rev-parse refs/remotes/origin/main)" = "597265b574be01c88a3ce559cc2bc07e02791da3"
git push --force-with-lease=refs/heads/main:597265b574be01c88a3ce559cc2bc07e02791da3 origin "${candidate_sha}:refs/heads/main"
git fetch origin main
test "$(git rev-parse HEAD)" = "$candidate_sha"
test "$(git rev-parse refs/remotes/origin/main)" = "$candidate_sha"
test "$(git ls-remote origin refs/heads/main | awk '{print $1}')" = "$candidate_sha"
```

Before the push, `candidate_sha` is set to and checked against the immutable
post-review local `HEAD`; fetched `origin/main` must still equal the published
C06 lease SHA. Afterward all three readbacks must equal `candidate_sha`, the
worktree stays clean, `target` stays absent, and no stale process remains.

## 7. Completion

C07 completes only after all three tasks are independently `CLEAN`, the exact
seven-row fixture correction and replacement digest are verified directly,
findings 2.14/2.17 have public behavioral evidence, all current Grid repeat
states are valid by construction, empty/duplicate keyframes retain authored
order, metadata remains truthful, all unaffected I01 and prior I02 evidence is
green, no prohibited proxy/dependency/feature/artifact/unsafe delta exists, a
fresh holistic review is `CLEAN`, cycle status is complete, `cargo clean
--offline` leaves no `target` or stale process, and the immutable candidate is
lease-published, fetched, and read back before C08 planning.

# P01-I02-C13 Backgrounds, Borders, Images, And Gradients

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C13` |
| Owning repository | `surgeist-css` |
| Status | `complete` |
| Cycle base | `ac164473416ccc2608f95a1a4f7e51ee638df3cc` |
| Published prerequisite | C12 `ac164473416ccc2608f95a1a4f7e51ee638df3cc`, fetched and read back |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `87f6a94b893ffa416c6ff451575f0d5a21b4aa136e7bcd391cd6c0ce8810a2ae` |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `3f93c7f6c3656ebe0b33b8bf9c32e458f306f294d1a969a86df375a5858b1710`, sections 4.2-4.4, 8.1-8.2, 9-10, 11 finding 2.2, 12 |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, semantic SHA-256 `56bcf0340320339454e4ae1aa0b45a7ad2e37e03930e9e3bf7665f8ce4cbb15a`, entry `I02-C13` |
| Bounded outcome | Complete exactly 27 reserved Backgrounds 3, Borders 3, Images 3, and gradient shared-value records, and promote the existing Partial Backgrounds/Borders property rows listed below, with layered/shorthand, separator, mutation, recovery, public-accessor, metadata, and documentation evidence. |

## 2. Boundary And Current Evidence

C13 owns authored CSS syntax, typed authored values, parser recovery, public
accessors, official metadata, vectors, and docs. It excludes resource loading,
image decoding, painting, layout, cascade, serialization, CSSOM, root adapters,
and generated API artifacts. Existing color, position, numeric, shadow, and
residual foundations are published prerequisites and must remain compatible.

The exact 27 reserved records in the base registry are:

- Properties: `border-image`, `border-image-outset`, `border-image-repeat`,
  `border-image-slice`, `border-image-source`, `border-image-width`,
  `image-orientation`, `image-rendering`, and `object-fit`.
- Shared values: `background-layer`, `background-image`, `repeat-style`,
  `background-attachment`, `background-size`, `line-style`, `line-width`,
  `image`, `gradient`, `linear-gradient`, `radial-gradient`,
  `repeating-linear-gradient`, `repeating-radial-gradient`, `color-stop-list`,
  `side-or-corner`, `radial-shape`, `radial-size`, and `radial-extent`.

These source IDs and fragments are frozen in `src/conformance.rs` and the
reviewed ledger. No source, spelling, owner, or status is inferred from parser
implementation. All public additions are additive with private fields,
checked constructors or parser-owned values, and non-exhaustive evolving
enums. No dependency, feature, manifest, MSRV, root, sibling, generated, or
unsafe change is authorized.

The existing Partial property rows that C13 must also make Complete are
`baseline.property.background`, `baseline.property.background-color`,
`baseline.property.background-image`, `baseline.property.background-size`,
`baseline.property.background-repeat`, `baseline.property.background-origin`,
`baseline.property.background-clip`, `baseline.property.background-attachment`,
`baseline.property.border`, `baseline.property.border-top`,
`baseline.property.border-right`, `baseline.property.border-bottom`,
`baseline.property.border-left`, `baseline.property.border-width`,
`baseline.property.border-top-width`, `baseline.property.border-right-width`,
`baseline.property.border-bottom-width`, `baseline.property.border-left-width`,
`baseline.property.border-color`, `baseline.property.border-top-color`,
`baseline.property.border-right-color`, `baseline.property.border-bottom-color`,
`baseline.property.border-left-color`, `baseline.property.border-style`,
`baseline.property.border-top-style`, `baseline.property.border-right-style`,
`baseline.property.border-bottom-style`, `baseline.property.border-left-style`,
`baseline.property.border-radius`, `baseline.property.border-top-left-radius`,
`baseline.property.border-top-right-radius`,
`baseline.property.border-bottom-right-radius`, and
`baseline.property.border-bottom-left-radius`.
Already-Complete `baseline.property.background-position`,
`official.property.object-position`, and `baseline.property.box-shadow` remain
Complete and are preserved, not reclassified.

## 3. Grammar Contract

Implement complete selected Backgrounds 3, Borders 3, and Images 3 authored
grammars: layered background shorthand, one-to-four border image components,
image values, gradients, stops and hints, object sizing/positioning, and image
rendering/orientation syntax. In each background layer, `/ <bg-size>` is
permitted only immediately after that layer's `<position>`; `<bg-color>` is
permitted only in the final layer. Preserve exact box/origin/clip arity,
authored layer order, and comma separators; reject adjacent grammar families
instead of accepting a sibling grammar by fallback.

Invalid values report the property-specific `InvalidPropertyValue` (or existing
typed diagnostic), exact responsible position/span, and `DropDeclaration`,
retaining later siblings. Unknown properties remain `UnknownProperty`. Globals
and substitution-dependent values stay distinct from typed values. Each new
list or recursive grammar covers EOF, repeated failures, non-BMP coordinates,
and 255/256/257 boundaries where applicable, with validator parity in both
feature modes. No test parses Rust source/files or asserts implementation
symbols, registrations, owner sets/counts, workflow state, or call counts.

## 4. Tasks

Every worker records `task_base_sha` and makes exactly two commits: a
base-compilable public behavioral RED, then implementation. After each focused
loop every task runs this exact matrix:

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

Each exact task range receives a fresh independent review before its dependent
task starts.

### T1 Image and gradient typed foundation

- **Area:** shared `image`, `gradient`, linear/radial/repeating gradient,
  `color-stop-list`, `side-or-corner`, `radial-shape`, `radial-size`, and
  `radial-extent` records; typed functions and image source distinctions.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test image_gradient_grammars c13_images_and_gradients_retain_typed_structure -- --exact`.
- **Acceptance:** image URL/none/function distinctions, stop and hint ordering,
  linear/radial/repeating forms, radial extent rules, globals/substitutions,
  exact invalid recovery and public accessors; no image loading or painting.
- **Commits:** `test: specify C13 image foundation`; `feat: add C13 image and gradient foundation`.

### T2 Background layers and shared background values

- **Dependency:** T1 independently CLEAN.
- **Area:** layered background declarations and shared `background-layer`,
  `background-image`, `repeat-style`, `background-attachment`, and
  `background-size` records; preserve existing color/position projections.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test background_grammars c13_background_layers_retain_typed_structure -- --exact`.
- **Acceptance:** authored layer order, comma separators, per-layer position/
  slash-size coupling, final-layer-only color, box/origin/clip arity,
  globals/substitutions, exact invalid recovery and public accessors; no image
  loading or painting.
- **Commits:** `test: specify C13 background layers`; `feat: add C13 background layers`.

### T3 Border images and image properties

- **Dependency:** T1 independently CLEAN; shared numeric/color/function models available.
- **Area:** six `border-image*` properties, shared `line-style`/`line-width`,
  and `image-orientation`, `image-rendering`, `object-fit`.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test border_image_grammars c13_border_images_retain_typed_structure -- --exact`.
- **Acceptance:** one-to-four arity, slice fill, repeat/outset/width domains,
  image source/value distinctions, orientation angles/flip, rendering and
  object-fit keywords, exact recovery and strict parity.
- **Commits:** `test: specify C13 border images`; `feat: add C13 border image grammars`.

### T4 Cross-family recovery, separators, and boundary matrix

- **Dependency:** T2 and T3 independently CLEAN.
- **Area:** narrowly required shared list/layer recovery seams and public tests
  spanning all 27 records, repeated failures, nested contexts, EOF, non-BMP,
  and 255/256/257 boundaries.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test c13_property_recovery c13_layer_separator_recovery_preserves_siblings_and_boundaries -- --exact`.
- **Acceptance:** exact smallest-unit diagnostics, responsible offsets/spans,
  layer/source order, sibling retention, validator parity, and no fixture
  weakening or masking.
- **Commits:** `test: specify C13 property recovery`; `feat: harden C13 property recovery`.

### T5 Official metadata, vectors, docs, and handoff

- **Dependency:** T1-T4 independently CLEAN.
- **Area:** activate exactly the 27 C13 property/value records, promote the
  listed existing Partial Backgrounds/Borders properties, direct named
  metadata/vector tests, README/rustdoc/doctests, and SHA-free handoff
  `plans/handoffs/P01-I02-C13-backgrounds-borders-images-gradients.md`.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog c13_background_image_metadata_is_truthful -- --exact`.
- **Acceptance:** exact source/fragments and Complete status for all 27 records
  and every listed Partial property, while preserving already-Complete rows;
  truthful aggregate totals, public docs/consumer examples, and handoff with
  product fixture digest(s) only, never Git SHAs.
- **Commits:** `test: specify C13 property metadata`; `docs: publish C13 property closure`.

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
git diff --check ac164473416ccc2608f95a1a4f7e51ee638df3cc..HEAD
test "$(shasum -a 256 tests/fixtures/i01-c01-observables.tsv | cut -d' ' -f1)" = "7c2cf7d79368d76d94cc0b383be70cc404d4c69d7caa72eedba6f0762e0b2356"
test -z "$(git diff --unified=0 ac164473416ccc2608f95a1a4f7e51ee638df3cc..HEAD -- tests/fixtures/i01-c01-observables.tsv)"
git status --short --branch
ps -axo pid=,command=
```

Also run this canonical owned-Rust executable-unsafe scan and require no
matches:

```sh
test -n "$(git ls-files '*.rs')"
c13_owned_rust="$(git ls-files '*.rs')"
if printf '%s\n' "$c13_owned_rust" | xargs rg -n --pcre2 --color never '\bunsafe\s*(fn|trait|impl|extern|mod|\{|union)|\bstatic\s+mut\b|#\s*\[\s*(allow|deny)\s*\(\s*unsafe_code' ; then exit 1; else test $? -eq 1; fi
```

After holistic review, run
`cargo clean --offline`, prove `target/` absent, the worktree clean, and no
leaf Cargo/Rust process. Fetch C12 and publish with the non-force fast-forward
lease gate, then fetch/read back and prove local/tracking/remote equality.
Any frozen-oracle contradiction, unresolved source ownership, unsafe or
external acquisition, root/sibling mutation, or scope expansion returns to P01
reconciliation.

# P01-I02-C04 Property-Specific Position Grammars

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C04` |
| Owning repository | `surgeist-css` |
| Status | `draft` |
| Cycle base | `966229264fdbfd101aa8688ca23c650875d2617a` |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `e290a7fef9bf6b6e9bde764140e5f7fac34156bb8f644d999e6bba58dc92ca2b`, P01.9 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `c6a9984521e23d5c010c3890902b70730db42eda092ad0e77f7d9e8e6168dfa1`, sections 3.1, 4.2-4.4, 8.2 positions, 9-10, 11 finding 2.9, and 12 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `09ecbf2dcaafbd402b24642f1244ce0be3568fd8a85b993c0218e2e7c0deac6d`, O-VALUES3 `#position`, O-BACKGROUNDS3 `#background-position`, O-IMAGES3 `object-position`, O-TRANSFORMS1 `transform-origin`, and preserved S-MASKING1 `mask-position` |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `fb02bf326ae06414ac7b50e58d791962db9973cea3b8ae73b9a1d372276f645c`, entry `I02-C04` |
| Bounded outcome | Publish distinct, valid-by-construction generic, background-layer, mask-layer, object, and transform-origin position models and parsers with exact property access, diagnostics, compatibility projections, and conformance metadata. |

## 2. Boundary And Source Reconciliation

C03 candidate `966229264fdbfd101aa8688ca23c650875d2617a` is published and read back. Its finite numeric and typed length-percentage calculation domains are stable C04 inputs. C04 closes finding 2.9 only. Function-specific position use in gradients, transforms, filters, and basic shapes remains on its existing legacy validation path for C05 or C13; C04 must not broaden those grammars.

The exact dated authored-syntax sources are O-VALUES3 `<position>` at `#position`, O-BACKGROUNDS3 `<bg-position>#` at `#background-position`, O-IMAGES3 `object-position` at `#propdef-object-position`, O-TRANSFORMS1 `transform-origin` at `#propdef-transform-origin`, and S-MASKING1 `mask-position` at `#propdef-mask-position`. The current private reservation for `official.property.object-position` says C13, but the reviewed sequence explicitly assigns the property and its generic position grammar to C04. C04 corrects that stale cycle reservation; it does not change O-IMAGES3 source ownership or claim `object-fit`, `image-rendering`, or `image-orientation`.

Generic `<position>` accepts exactly:

- one keyword or length-percentage, with the omitted axis centered;
- a valid two-axis pair, with two keywords reorderable but keyword/length order fixed;
- two edge-offset pairs, one horizontal and one vertical, in either pair order.

It rejects three components. In the two-component branch the first component is horizontal and the second vertical unless both are reorderable keywords: `left 50px` and `center 50px` are complete 2D positions, while `top 50px` and `bottom 50px` are not generic positions. An edge offset is permitted only after its matching non-center edge keyword; `center` never takes an offset. Duplicate axes, two horizontal or two vertical sides, a trailing offset, and partial four-component forms are invalid. Parsing is greedy where another grammar follows.

Background `<bg-position>` has the same one-, two-, and four-component forms plus its specified three-component form: exactly one edge carries an offset and the other axis is a center or unoffset edge. `background-position` is a nonempty comma list of these values. S-MASKING1 `mask-position` is instead a nonempty comma list of generic `<position>` values and therefore rejects three components.

O-IMAGES3 `object-position` is one generic `<position>`. O-TRANSFORMS1 `transform-origin` accepts its specified one- or two-component 2D position followed by an optional z `<length>`. The directed greedy rule is exact: `left 50px` and `center 50px` are complete 2D positions with no z; `top 50px` and `bottom 50px` are one-token vertical 2D positions followed by z; `left top 50px` also has z. The same distinction applies to a typed calculation with result `Length`. A percentage or mixed length-percentage in the z slot is invalid even though it is valid in a 2D axis. The z value also rejects keywords, a second z value, and non-finite values. Range evaluation of a well-typed length calculation remains deferred.

The crate continues to preserve authored symbolic lengths; it does not resolve percentages, physical axes, writing modes, boxes, object sizes, painting, transforms, or layout.

## 3. Resolved Public Model

Existing `CssHorizontalPositionKeyword`, `CssVerticalPositionKeyword`, `CssPositionComponent`, `CssPosition`, and `CssPositionList` remain exact I01 compatibility types with unchanged public signatures and Debug observables. Public construction of those legacy shapes does not define current grammar validity. Parser-produced compatibility projections are created only after current grammar validation.

New private-field `CssPositionOffset` wraps one position-valid `CssLength`. `try_new(CssLength) -> Option<Self>` accepts finite length, percentage, zero, or typed/legacy length-percentage calculation states and rejects `auto`, `normal`, intrinsic keywords, and other non-position branches; `value() -> &CssLength` exposes it. The parser uses the same invariant without input-driven panic.

New non-exhaustive axis enums are:

- `CssHorizontalPosition::{Left, Center, Right, Offset(CssPositionOffset), LeftOffset(CssPositionOffset), RightOffset(CssPositionOffset)}`;
- `CssVerticalPosition::{Top, Center, Bottom, Offset(CssPositionOffset), TopOffset(CssPositionOffset), BottomOffset(CssPositionOffset)}`.

`Offset` means an offset from the start edge without an authored edge keyword. Edge-offset variants retain the authored origin. All aggregate construction below is parser-owned so invalid cross-axis combinations are unconstructable.

Private-field `CssPositionValue` exposes `horizontal() -> &CssHorizontalPosition` and `vertical() -> &CssVerticalPosition`. It is the current generic `<position>` model. Private-field `CssBackgroundPosition` exposes the same two accessors but is distinct because it admits the property-only three-component syntax. `CssBackgroundPositionList` and `CssMaskPositionList` reject empty vectors and expose `positions()` slices; each mask entry is a distinct private-field `CssMaskPosition` exposing `value() -> &CssPositionValue`.

Private-field `CssObjectPosition` exposes `value() -> &CssPositionValue`. Private-field `CssTransformOriginZ` exposes `value() -> &CssLength` and `try_new(CssLength) -> Option<Self>` with the exact z-length invariant. Private-field `CssTransformOrigin` exposes `horizontal()`, `vertical()`, and `z() -> Option<&CssTransformOriginZ>`.

Exactly four property-specific current accessors are required:

- `CssBackgroundPositionPropertyValue::positions() -> &CssBackgroundPositionList`;
- `CssMaskPositionPropertyValue::positions() -> &CssMaskPositionList`;
- `CssObjectPositionPropertyValue::position() -> &CssObjectPosition`;
- `CssTransformOriginPropertyValue::origin() -> &CssTransformOrigin`.

The first, second, and fourth wrappers store a parser-owned current value plus an optional frozen I01 projection and retain their exact `i01_subset()` signatures. Every I01 input remains `Some` with identical authored/Debug observables. Newly accepted typed calculations or exact forms not representable by the frozen payload return `None`; no lossy projection is fabricated. `object-position` is a new additive schema row, wrapper, borrowed property-value view variant, property identity, and ordinary/global/substitution branch with no I01 projection requirement.

Existing mask shorthand parsing must validate its position component with the new generic parser but may retain its I01 `CssMaskList` representation; no new mask-shorthand semantic accessor or non-position mask grammar is claimed. Existing background shorthand remains outside C04. Existing function validators continue to call their explicit legacy position parser.

The public API effect is additive after C01: private-field structs, non-exhaustive enums, a new official property row/view, checked leaf wrappers, and property-specific accessors. Existing signatures, dependencies, features, build logic, generated artifacts, and leaf MSRV are unchanged. Root owns facade reexports, generated API artifacts, integration docs/tests, and gitlink promotion. All owned Rust remains free of `unsafe`.

Integration tests use crate-root checked constructors and public parse/validate entry points. Owning parser unit tests may parse authored CSS tokens into private helpers and assert semantic values or structured errors. No test reads or parses Rust source, asserts code/token/symbol placement, compares source/test/catalog owner sets or counts, mutates an inventory as completeness evidence, checks incidental call sequences, or encodes plan/review/publication state. The C01 fixture is immutable.

## 4. Tasks

### T1 Publish The Exact Generic Position Model And Parser

- **Files/area:** position types in `src/syntax.rs`; generic parser in `src/parser/background.rs` or a narrowly extracted owning value parser; crate-root rustdoc/exports; `tests/position_grammars.rs`, `tests/structured_errors.rs`, and owning parser tests. No property schema, background/mask list specialization, transform-origin, catalog, fixture, manifest, or docs edit.
- **Dependency:** published/read-back C03 base only.
- **Outcome:** implement `CssPositionOffset`, both exact axis enums, `CssPositionValue`, and a complete generic `<position>` parser. Preserve legacy models/projections. Keep function validators on the legacy parser.
- **RED:** a behavioral test-only commit first shows current acceptance of invalid `left right`, `50% left`, three-component, dangling-offset, center-offset, and duplicate-axis mutations or absence of the current public model; named positives cover every one/two/four grammar branch and pair order.
- **Acceptance:** public model inspection distinguishes start/end/free offsets and edge origins; generic consumers accept `left|center <length-percentage>` as x/y and reject `top|bottom <length-percentage>` as a completed position, including typed calculation counterparts; one-token mutation rows assert exact property/root/token/position/span/action and valid sibling retention in default and `app-strict`; non-BMP coordinates and typed calc depth 255/256/257 are exact; parser-produced legacy projections stay identical; C05/C13 function inputs retain their pre-C04 outcomes.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test position_grammars`; `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`; `cargo test -p surgeist-css --offline --no-default-features --test source_coordinates`; `cargo test -p surgeist-css --offline --no-default-features --test public_surface`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`; `cargo test -p surgeist-css --offline --no-default-features --lib parser::background`; repeat those six with `--features app-strict`; `cargo fmt --check`; `git diff --check`; `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commits:** `test: specify generic CSS positions`; `feat: add typed generic CSS positions`.

### T2 Separate Background And Mask Position Lists

- **Files/area:** current list/layer models in `src/syntax.rs`; `src/parser/background.rs`, mask position call sites in `src/parser/effects.rs`, affected property wrapper representations in `src/properties.rs`; `tests/position_grammars.rs`, `tests/property_schema.rs`, `tests/structured_errors.rs`. No object-position, transform-origin, function grammar, catalog, fixture, manifest, or docs edit.
- **Dependency:** T1 independently clean.
- **Outcome:** implement exact `CssBackgroundPosition`, `CssBackgroundPositionList`, `CssMaskPosition`, and `CssMaskPositionList`; add the two exact wrapper accessors. Background accepts nonempty `<bg-position>#`, including valid three-component layers; mask accepts nonempty generic `<position>#` and rejects every three-component layer. Mask shorthand uses generic position validation without broadening other components.
- **RED:** behavioral test-only commit first contrasts the same three-component mutation under background and mask, plus empty/trailing/double comma, cross-layer recovery, edge-offset order, slash/trailing-token, and typed calculation cases.
- **Acceptance:** each list and each layer is publicly inspectable; background/mask cannot share a component-count shortcut; all one-token mutation rows have exact typed diagnostics/coordinates/spans/actions, sibling retention, repeated-failure progress, and strict parity; I01 inputs retain exact projections and the fixture.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test position_grammars`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`; `cargo test -p surgeist-css --offline --no-default-features --test source_coordinates`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`; repeat those five with `--features app-strict`; `cargo test -p surgeist-css --offline --no-default-features`; `cargo test -p surgeist-css --offline --no-default-features --features app-strict`; `cargo fmt --check`; `git diff --check`; `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commits:** `test: specify layered CSS positions`; `feat: separate layered CSS position grammars`.

### T3 Add Object Position And Exact Transform Origin

- **Files/area:** `CssObjectPosition`, `CssTransformOriginZ`, and `CssTransformOrigin` in `src/syntax.rs`; parsers in `src/parser/background.rs`/`src/parser/effects.rs`; additive `object-position` schema row and the two affected wrappers in `src/properties.rs`; exact source routing in `src/conformance.rs`; `tests/position_grammars.rs`, `tests/property_schema.rs`, `tests/structured_errors.rs`, and `tests/source_coordinates.rs`. No other Images 3 property, transform function, catalog promotion, fixture, manifest, or docs edit.
- **Dependency:** T2 independently clean.
- **Outcome:** add official `object-position` with ordinary/global/substitution parsing and exact current accessor. Implement transform-origin greedy 2D parsing plus optional z length and exact accessor while preserving all I01 projections. Correct only the stale object-position C13 reservation ownership needed for the additive schema row; T4 promotes status.
- **RED:** behavioral test-only commit first shows missing object-position, transform-origin z ambiguity, invalid percentage/mixed-calculation z, fourth component, repeated z, and property-coupling boundaries; valid rows cover every one/two component order and z/no-z branch.
- **Acceptance:** object and transform models expose exact axes/z; `left|center 50px` has no z, while `top|bottom 50px` and `left top 50px` expose z; typed `calc(1px * 2)` follows the same directed split and typed percentage/mixed calculations are rejected in z; new object syntax has exact known-property diagnostics and recovery; all invalid mutations assert typed payload/token/position/span/action/sibling/strict parity; global and substitution branches remain distinct; no other Images 3 property is recognized early.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test position_grammars`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test coupled_declarations`; `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`; `cargo test -p surgeist-css --offline --no-default-features --test source_coordinates`; `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`; repeat those seven with `--features app-strict`; `cargo test -p surgeist-css --offline --no-default-features`; `cargo test -p surgeist-css --offline --no-default-features --features app-strict`; `cargo fmt --check`; `git diff --check`; `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commits:** `test: specify object and transform origins`; `feat: add property-specific CSS origins`.

### T4 Promote Position Metadata And Publish The Handoff

- **Files/area:** `src/conformance.rs`, owning position/property inventories, named public metadata cases, README/crate rustdoc, and new SHA-free `plans/handoffs/P01-I02-C04-property-specific-position-grammars.md`. No grammar, fixture, manifest, root, sibling, or generated-artifact edit.
- **Dependency:** T3 independently clean.
- **Outcome:** promote `official.value.position` Complete with O-VALUES3/`#position`, `official.value.background-position` Complete with O-BACKGROUNDS3/`#background-position`, `baseline.property.background-position` Complete with O-BACKGROUNDS3/`#propdef-background-position`, `official.property.object-position` Complete with O-IMAGES3/`#propdef-object-position`, `baseline.property.transform-origin` Complete with O-TRANSFORMS1/`#propdef-transform-origin`, and `baseline.property.mask-position` Complete with S-MASKING1/`#propdef-mask-position`. Each has one exact owner and individually named public behavior evidence. Other background, image, transform, and mask rows remain unchanged.
- **Evidence:** behavior-paired named metadata cases, direct source/catalog/owner reconciliation, compiling docs, and deterministic artifact checks only. No inventory/set/count proxy test. Document grammar distinctions, compatibility/current access, symbolic offsets, downstream exclusions, and exact root follow-up. The handoff contains no SHA, review, publication, or command-manifest state.
- **RED:** a test-only commit first records that the six named rows are reserved, absent, or carry the stale C13 reservation and therefore fail their exact Complete/source/owner/evidence assertions even though their paired public background-position, mask-position, object-position, and transform-origin cases pass after T1-T3; no test derives evidence from source text, inventories, set/count joins, or coordination state.
- **Acceptance:** every promoted row's source/production/status/owner/evidence is exact; no deferred row is exposed; docs compile; fixture SHA remains `98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8`.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`; `cargo test -p surgeist-css --offline --no-default-features --test catalog_inventory`; `cargo test -p surgeist-css --offline --no-default-features --test position_grammars`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test public_surface`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`; repeat those six with `--features app-strict`; `cargo test -p surgeist-css --offline --no-default-features --doc`; `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`; `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps`; `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps`; `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`; `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`; `cargo fmt --check`; `git diff --check`; `! rg -n 'TO''DO|TB''D|FIX''ME|\?''\?''\?' README.md src/lib.rs plans/handoffs/P01-I02-C04-property-specific-position-grammars.md`; `! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .`.
- **Intended commits:** `test: specify CSS position metadata`; `docs: publish property-specific CSS positions`.

## 5. Exact Completion Gate

After every task has a clean task review and the status-only completion commit is made, run from a process-clean repository:

```sh
cargo check -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --doc
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo check -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps
cargo fmt --check
git diff --check 966229264fdbfd101aa8688ca23c650875d2617a..HEAD
rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs
! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .
test "$(shasum -a 256 tests/fixtures/i01-c01-observables.tsv | awk '{print $1}')" = 98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8
```

Directly inspect every changed test for real public or owning-private behavior and absence of source/code/count/owner-set/inventory/call-sequence/coordination proxies.

Apply the canonical status, holistic-review, landing, and publication contracts to the exact cycle range without history rewrite or forced/non-fast-forward push. After post-review gates run:

```sh
cargo clean --offline
test ! -e target
test -z "$(git status --porcelain)"
repo_path=$(pwd -P)
excluded_pids=" $$ "
ancestor_pid=$PPID
while test "$ancestor_pid" -gt 1 2>/dev/null; do
  excluded_pids="$excluded_pids$ancestor_pid "
  ancestor_pid=$(ps -p "$ancestor_pid" -o ppid= 2>/dev/null | tr -d ' ')
  test -n "$ancestor_pid" || break
done
for pid in $(ps -U "$(id -u)" -o pid=); do
  case "$excluded_pids" in *" $pid "*) continue ;; esac
  process_cwd=$(lsof -a -p "$pid" -d cwd -Fn 2>/dev/null | sed -n 's/^n//p')
  process_command=$(ps -p "$pid" -o command= 2>/dev/null || true)
  if test "$process_cwd" = "$repo_path" || printf '%s\n' "$process_command" | rg -q --fixed-strings "$repo_path"; then
    printf 'repository process remains: %s %s %s\n' "$pid" "$process_cwd" "$process_command" >&2
    exit 1
  fi
done
```

The pass condition is exit zero with no reported repository process; processes whose cwd and command resolve only to a sibling repository are untouched. The handoff path is `plans/handoffs/P01-I02-C04-property-specific-position-grammars.md`. A frozen I01 semantic change, second breaking I02 API change, unsafe, dependency/feature addition, external acquisition, unresolved source ownership, root/sibling edit, or inability to preserve the fixture is a blocker.

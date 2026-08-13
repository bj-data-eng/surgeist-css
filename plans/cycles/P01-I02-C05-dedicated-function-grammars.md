# P01-I02-C05 Dedicated Function Grammars

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C05` |
| Owning repository | `surgeist-css` |
| Status | `complete` |
| Cycle base | `6b1eb1a0db6e1a26a7d8974dbd2405a874d07fdb` |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `e290a7fef9bf6b6e9bde764140e5f7fac34156bb8f644d999e6bba58dc92ca2b`, P01.9 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `c6a9984521e23d5c010c3890902b70730db42eda092ad0e77f7d9e8e6168dfa1`, sections 3.1, 4.3-4.4, 8.2 functions, 9-10, 11 finding 2.10, and 12 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `09ecbf2dcaafbd402b24642f1244ce0be3568fd8a85b993c0218e2e7c0deac6d`, O-BACKGROUNDS3 shadow, O-TRANSFORMS1 functions, and O-EASING1 functions |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `fb02bf326ae06414ac7b50e58d791962db9973cea3b8ae73b9a1d372276f645c`, entry `I02-C05` |
| Bounded outcome | Replace the selected generic authored-argument validators with dedicated typed transform, easing, shadow/filter, and basic-shape grammars while preserving the exact I01 projections and publishing truthful atomic metadata. |

## 2. Boundary And Source Reconciliation

C04 candidate `6b1eb1a0db6e1a26a7d8974dbd2405a874d07fdb` is published and read back. Its typed numeric/math and property-specific position models are stable C05 inputs. C05 closes finding 2.10 only. Color functions remain C06; gradient functions remain C13; transform-origin remains C04; individual `translate`, `rotate`, and `scale` property grammars retain their I01 behavior; rendering, interpolation, matrix multiplication, filter evaluation, path geometry, and layout resolution remain excluded.

The exact sources are O-TRANSFORMS1 `#transform-functions`, `#two-d-transform-functions`, and `#transform-function-lists`; O-EASING1 `#easing-functions`, `#cubic-bezier-easing-functions`, and `#step-easing-functions`; O-BACKGROUNDS3 `#box-shadow`; I-FILTER1 filter-function and `drop-shadow()` grammar; S-SHAPES1 `#funcdef-basic-shape-inset`, `#funcdef-basic-shape-circle`, `#funcdef-basic-shape-ellipse`, and `#funcdef-basic-shape-polygon` as imported by S-MASKING1 `clip-path`; and the exact I01-preserved I-TRANSFORMS2 3D function subset and X-FILTER2-BASE `backdrop-filter` subset. O-TRANSFORMS1 owns only `matrix`, `translate`, `translateX`, `translateY`, `scale`, `scaleX`, `scaleY`, `rotate`, `skew`, `skewX`, and `skewY`. I-TRANSFORMS2 owns the already-preserved `matrix3d`, `perspective`, `rotate3d`, `rotateX/Y/Z`, `scale3d`, `scaleZ`, `translate3d`, and `translateZ` functions; C05 does not broaden that source beyond I01.

Transform commas and arities are exact. `matrix()` has six comma-separated finite numbers. Two-dimensional translate/scale/skew take one value or two comma-separated values; the axis functions and rotate take exactly one. Typed length-percentage, number, and angle calculations remain symbolic. Three-dimensional functions retain their exact comma-separated arity and dimension domains. `perspective()` accepts exactly `none` or one non-negative length, including unitless zero; it rejects percentages and negative lengths. `scale3d()` and `scaleZ()` accept their exact `<number> | <percentage>` operands and preserve the authored scalar kind.

Easing keywords remain distinct authored branches. `cubic-bezier(x1, y1, x2, y2)` has exactly four comma-separated finite numbers and requires both x coordinates in the closed unit interval; y coordinates are finite but otherwise unbounded. `steps(N, position?)` requires a positive integer, exact comma use, and one of `jump-start`, `jump-end`, `jump-none`, `jump-both`, `start`, or `end`; `jump-none` requires at least two intervals. Lists remain nonempty and comma-separated.

Box shadow and filter drop shadow are distinct. Box shadow permits optional `inset`, optional one color, two required lengths, optional non-negative blur, and optional signed spread, in grammar-permitted order and comma-separated nonempty lists. `drop-shadow()` permits one optional color and two or three lengths only: no `inset`, no spread, and a non-negative optional blur. Filter amount functions use their selected non-negative number/percentage domains and exact optional/default arity; `hue-rotate()` takes one angle and `blur()` one non-negative length. URL and function lists preserve authored order. `backdrop-filter` remains an exact preserved baseline subset, not an implied Filter Effects 2 claim.

The selected Shapes 1 functions receive distinct typed models and bind their imported `<radial-size>` exactly. Circle accepts an omitted radius, any of `closest-side`, `farthest-side`, `closest-corner`, or `farthest-corner`, or one non-negative `<length>` (not a percentage), plus an optional exact C04 generic position. Ellipse accepts an omitted radius, any of the same four extent keywords, or exactly two non-negative `<length-percentage>` radii, plus an optional position. Inset has one-to-four length-percentages and optional `round` radii with the exact slash boundary. Polygon has optional fill rule, optional `round <length>` with a non-negative checked authored length, the mandatory comma before its nonempty point list, and exactly two length-percentages per comma-separated point. The dated S-SHAPES1 source also contains `path`, `shape`, `rect`, and `xywh`; they are not silently recognized or claimed Complete in C05.

## 3. Public Model And Evidence Contract

Existing `CssTransformArguments`, `CssFilterArguments`, `CssBasicShapeArguments`, `CssEasingArguments`, their legacy enums, public constructors, and `as_css()`/I01 wrapper projections remain source-compatible. Parser-produced compatibility projections preserve every frozen I01 debug/report observable. They are not the new invariant boundary.

C05 adds parallel current authored models with private fields and parser-owned or checked construction: `CssTransformFunctionValue` and a nonempty current transform list; typed cubic-bezier/step values and a nonempty current easing list; typed filter amounts, filter-function values, `CssDropShadow`, and a nonempty current filter list; and typed inset/circle/ellipse/polygon values under a current basic-shape/clip-path model. Existing property wrappers gain exact `current()` accessors (or an equally narrow property-named accessor where `current()` already exists) for transform, transition/animation timing functions and shorthands, filter/backdrop-filter, and clip-path. Box shadow's existing `CssShadow`/`CssBoxShadowList` is the current model and is strengthened only where its source grammar is presently violated.

All new evolving enums are `#[non_exhaustive]`; public fields stay private; scalar and list constructors reject invalid states; no authored symbolic numeric, percentage, calculation, position, URL, or color is resolved. The API effect is additive after C01. No existing signature, dependency, feature, build logic, fixture, generated artifact, or MSRV changes. Root owns facade reexports, generated API artifacts, integration tests/docs, and gitlink promotion.

Every integration test uses crate-root public APIs. Owning parser unit tests may parse authored tokens into private helpers and assert semantic values or structured errors. No test reads/parses Rust source, asserts code/token/symbol placement, compares source/test/catalog owner sets or counts, mutates inventories as completeness evidence, checks incidental calls, or encodes plan/review/publication state. Every behavior task begins with an executable focused behavioral RED using APIs available at its task base; a separate compile RED may supplement but never replace it. The C01 fixture is immutable.

### Exact C05 Metadata Delta

Every ID below is one distinct public catalog record. “Complete” means only the exact named production; it never promotes the whole source. The named behavior cases are stable test names to be authored independently of the catalog implementation.

| Stable ID(s) | Source / exact production | Disposition and exact subset/remainder | Owner | Named public behavior evidence |
| --- | --- | --- | --- | --- |
| `ext.value.transform.matrix3d` | I-TRANSFORMS2 `#funcdef-matrix3d` | Complete: exact 16-number function | `crate::parser::effects` | `transform_matrix3d_exposes_sixteen_finite_components` |
| `ext.value.transform.perspective` | I-TRANSFORMS2 `#funcdef-perspective` | Complete: exactly `none` or one non-negative length, including zero; percentages/negative lengths rejected | `crate::parser::effects` | `transform_perspective_accepts_none_and_zero_and_rejects_invalid_dimensions` |
| `ext.value.transform.rotate3d`, `ext.value.transform.rotate-x`, `ext.value.transform.rotate-y`, `ext.value.transform.rotate-z` | I-TRANSFORMS2 `#funcdef-rotate3d`, `#funcdef-rotatex`, `#funcdef-rotatey`, `#funcdef-rotatez` | Four separate Complete records: exact vector/angle and axis-angle functions | `crate::parser::effects` | `transform_three_dimensional_rotations_are_typed` |
| `ext.value.transform.scale3d`, `ext.value.transform.scale-z` | I-TRANSFORMS2 `#funcdef-scale3d`, `#funcdef-scalez` | Two separate Complete records: exact finite `<number> | <percentage>` operands with authored kind retained | `crate::parser::effects` | `transform_three_dimensional_scales_preserve_number_and_percentage_operands` |
| `ext.value.transform.translate3d`, `ext.value.transform.translate-z` | I-TRANSFORMS2 `#funcdef-translate3d`, `#funcdef-translatez` | Two separate Complete records: x/y length-percentage and z length-only domains | `crate::parser::effects` | `transform_three_dimensional_translations_keep_z_length_only` |
| `ext.value.filter-function-list` | I-FILTER1 `#FilterProperty` | Complete: nonempty ordered list of the ten functions below or URL; no Filter Effects 2 claim | `crate::parser::effects` | `filter_function_list_preserves_typed_authored_order` |
| `ext.value.filter.blur` → `#funcdef-filter-blur`; `ext.value.filter.brightness` → `#funcdef-filter-brightness`; `ext.value.filter.contrast` → `#funcdef-filter-contrast`; `ext.value.filter.grayscale` → `#funcdef-filter-grayscale`; `ext.value.filter.hue-rotate` → `#funcdef-filter-hue-rotate`; `ext.value.filter.invert` → `#funcdef-filter-invert`; `ext.value.filter.opacity` → `#funcdef-filter-opacity`; `ext.value.filter.saturate` → `#funcdef-filter-saturate`; `ext.value.filter.sepia` → `#funcdef-filter-sepia` | I-FILTER1, with the exact ID-to-fragment mapping in the first cell | Nine separate Complete records with each exact optional/default scalar domain | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.drop-shadow` | I-FILTER1 `#funcdef-filter-drop-shadow` | Complete: optional color plus two/three lengths; no inset/spread | `crate::parser::effects` | `drop_shadow_rejects_box_shadow_only_components` |
| `ext.value.basic-shape` | S-SHAPES1 `#typedef-basic-shape` | Partial: supports typed `inset`, `circle`, `ellipse`, `polygon`; remainder is `path`, `shape`, `rect`, and `xywh` | `crate::parser::effects` | `clip_path_distinguishes_selected_and_deferred_shape_functions` |
| `ext.value.basic-shape.inset`, `ext.value.basic-shape.circle`, `ext.value.basic-shape.ellipse`, `ext.value.basic-shape.polygon` | S-SHAPES1 respective `#funcdef-basic-shape-*` anchors | Four separate Complete records including polygon `round <length>` | `crate::parser::effects` | `every_selected_basic_shape_has_typed_public_components` |
| `baseline.property.backdrop-filter` | X-FILTER2-BASE existing baseline production | Retain Partial: exact I01 filter-list subset becomes typed; remainder is every Filter Effects 2 behavior not present at `bc5394f:src/parser/effects.rs` | `crate::parser::effects` | `backdrop_filter_preserves_exact_typed_baseline_subset` |
| `baseline.property.clip-path` | S-MASKING1 `#propdef-clip-path` | Retain Partial: `none`, URL, and four selected typed Shapes 1 functions; remainder is reference-box combinations and unselected shape functions | `crate::parser::effects` | `clip_path_selected_subset_and_remainder_are_distinct` |

The thirteen existing O-TRANSFORMS1 records, four O-EASING1 records, and `official.value.shadow` become Complete with their existing exact IDs/fragments. `baseline.property.transform`, `baseline.property.box-shadow`, `baseline.property.filter`, `baseline.property.transition-timing-function`, and `baseline.property.animation-timing-function` become Complete. `baseline.property.transition`, `baseline.property.animation`, `baseline.property.backdrop-filter`, and `baseline.property.clip-path` remain Partial with exact existing or table-defined boundaries; C05 does not promote them by association.

## 4. Tasks

### T1 Publish Typed Transform Functions

- **Files/area:** transform current models in `src/syntax.rs`; transform parsing in `src/parser/effects.rs`; exact transform wrapper representation/access in `src/properties.rs`; crate-root exports; new `tests/function_grammars.rs` plus `tests/property_schema.rs`, `tests/structured_errors.rs`, `tests/source_coordinates.rs`, and owning parser tests. No easing/filter/shadow/shape/catalog/fixture/manifest/docs edit.
- **Dependency:** published/read-back C04 only.
- **Outcome:** parse every selected O-TRANSFORMS1 2D and preserved I-TRANSFORMS2 3D function into a dedicated typed value and nonempty ordered list while retaining the legacy kind/authored-arguments projection.
- **RED:** executable public tests first show accepted wrong separators/arities/domains and absence of typed observables without referring to missing symbols; named positives cover every selected function and list-order branch, `perspective(none)`, `perspective(0)`, and number/percentage 3D-scale operands.
- **Acceptance:** exact comma/arity rules, finite typed scalar/math payloads, length-percentage versus length-only versus angle domains, `perspective(none)`/non-negative length including zero, typed number-or-percentage `scale3d`/`scaleZ`, invalid negative/percentage perspective and empty/extra/missing members, depth 255/256/257, non-BMP positions, full typed diagnostics/spans/actions/sibling recovery/repeated progress/strict parity; transform `none`, global, and substitution branches remain distinct; no individual transform-property or transform-origin behavior changes.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test function_grammars`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`; `cargo test -p surgeist-css --offline --no-default-features --test source_coordinates`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`; repeat those five with `--features app-strict`; `cargo test -p surgeist-css --offline --no-default-features`; `cargo test -p surgeist-css --offline --no-default-features --features app-strict`; `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`; `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`; `cargo fmt --check`; `git diff --check`.
- **Intended commits:** `test: specify typed transform functions`; `feat: add typed transform functions`.

### T2 Publish Exact Easing Functions

- **Files/area:** easing current models in `src/syntax.rs`; `src/parser/timing.rs` and narrowly shared typed helpers in `src/parser/effects.rs`; timing wrapper current access in `src/properties.rs`; `tests/function_grammars.rs`, `tests/timing_domains.rs`, `tests/property_schema.rs`, `tests/structured_errors.rs`, and `tests/source_coordinates.rs`. No transform/filter/shadow/shape/catalog/fixture/manifest/docs edit.
- **Dependency:** T1 independently clean.
- **Outcome:** replace generic easing argument bags on parser-produced current paths with typed keyword, cubic-bezier, step-position, and step-count values across timing longhands and shorthands.
- **RED:** executable behavioral tests first show acceptance of out-of-range x coordinates, nonpositive/noninteger steps, invalid `jump-none` count, wrong separators/arity, and missing typed current observables.
- **Acceptance:** all keyword and alias branches, x closed-unit bounds, unbounded finite y, every step position, `jump-none` rule, exact comma lists, shorthand consumer propagation, typed math policy from the selected grammar, full diagnostics/recovery/depth/non-BMP/strict parity, and I01 projections.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test function_grammars`; `cargo test -p surgeist-css --offline --no-default-features --test timing_domains`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`; `cargo test -p surgeist-css --offline --no-default-features --test source_coordinates`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`; repeat those six with `--features app-strict`; `cargo test -p surgeist-css --offline --no-default-features`; `cargo test -p surgeist-css --offline --no-default-features --features app-strict`; `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`; `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`; `cargo fmt --check`; `git diff --check`.
- **Intended commits:** `test: specify typed easing functions`; `feat: add typed easing functions`.

### T3 Separate Box Shadow And Filter Functions

- **Files/area:** shadow/filter current models in `src/syntax.rs`; `src/parser/box_model.rs` and filter paths in `src/parser/effects.rs`; box-shadow/filter/backdrop-filter wrapper access in `src/properties.rs`; `tests/function_grammars.rs`, `tests/property_schema.rs`, `tests/structured_errors.rs`, `tests/source_coordinates.rs`, and `tests/typed_calculations.rs`. No transform/easing/shape/catalog/fixture/manifest/docs edit.
- **Dependency:** T2 independently clean.
- **Outcome:** complete exact box-shadow parsing and give every filter function a typed current payload, with a distinct drop-shadow model that cannot contain inset or spread.
- **RED:** executable behavioral tests first show the current drop-shadow acceptance of a box-shadow-only mutation or another exact selected grammar failure, plus generic payload-only observability; named positives cover every filter and shadow order/list branch.
- **Acceptance:** exact defaults/arity/domains, color and keyword uniqueness, signed offsets/spread versus non-negative blur, drop-shadow restrictions, URL/function order, empty/unknown/repeated/trailing mutations, typed calculations, full diagnostics/recovery/depth/non-BMP/strict parity, filter/backdrop-filter/box-shadow ordinary-global-substitution distinction, and unchanged I01 projections.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test function_grammars`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`; `cargo test -p surgeist-css --offline --no-default-features --test source_coordinates`; `cargo test -p surgeist-css --offline --no-default-features --test typed_calculations`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`; repeat those six with `--features app-strict`; `cargo test -p surgeist-css --offline --no-default-features`; `cargo test -p surgeist-css --offline --no-default-features --features app-strict`; `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`; `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`; `cargo fmt --check`; `git diff --check`.
- **Intended commits:** `test: specify typed shadow and filter functions`; `feat: add typed shadow and filter functions`.

### T4 Publish Typed Basic Shapes

- **Files/area:** selected current basic-shape/clip-path models in `src/syntax.rs`; shape parsing in `src/parser/effects.rs` using C04 generic positions and C03 typed lengths; exact clip-path wrapper current access in `src/properties.rs`; `tests/function_grammars.rs`, `tests/property_schema.rs`, `tests/structured_errors.rs`, and `tests/source_coordinates.rs`. No new shape function/property, transform/easing/filter/shadow/catalog/fixture/manifest/docs edit.
- **Dependency:** T3 independently clean.
- **Outcome:** replace the four selected generic basic-shape argument bags on parser-produced current paths with typed inset, circle, ellipse, and polygon values while retaining legacy projections.
- **RED:** executable behavior first shows acceptance of exact radius/arity/separator/position mutations, including circle percentage, negative circle/ellipse radii, one/three ellipse radii, missing/wrong polygon comma, negative/percentage polygon rounding, and malformed polygon points, plus missing current inspection; positives cover all four radial extent keywords for circle and ellipse.
- **Acceptance:** valid omitted/default and explicit branches; circle permits all four extent keywords or one non-negative length and rejects percentage; ellipse permits all four extent keywords or exactly two non-negative length-percentages; C04 position reuse; inset expansion/slash rules; polygon fill-rule, optional `round <length>`, mandatory pre-list comma, and point-list grammar; full typed diagnostics/recovery/depth/non-BMP/strict parity, ordinary/global/substitution distinction, no early recognition of `path`, `shape`, `rect`, or `xywh`, and fixture preservation.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test function_grammars`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test structured_errors`; `cargo test -p surgeist-css --offline --no-default-features --test source_coordinates`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`; repeat those five with `--features app-strict`; `cargo test -p surgeist-css --offline --no-default-features`; `cargo test -p surgeist-css --offline --no-default-features --features app-strict`; `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`; `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`; `cargo fmt --check`; `git diff --check`.
- **Intended commits:** `test: specify typed basic-shape functions`; `feat: add typed basic-shape functions`.

### T5 Promote Function Metadata And Publish The Handoff

- **Files/area:** `src/conformance.rs`, owning shared-value inventories, named public metadata cases, README/crate rustdoc, and new SHA-free `plans/handoffs/P01-I02-C05-dedicated-function-grammars.md`. No grammar/model/schema/fixture/manifest/root/sibling/generated-artifact edit.
- **Dependency:** T4 independently clean.
- **Outcome:** apply exactly the metadata table in section 3: promote the thirteen O-TRANSFORMS1 rows, four O-EASING1 rows, `official.value.shadow`, and five named Complete property rows; add every listed source-specific extension record; retain the four named property/aggregate rows Partial with their exact supported subset and remainder.
- **RED:** a test-only commit first proves the named official rows are reserved/Partial and the new exact preserved-extension rows absent while their individually paired public behaviors pass; no set/count/inventory/source-code or coordination proxy is used.
- **Acceptance:** every promoted/added row exactly matches the section 3 table's source, fragment, disposition, supported subset/remainder, one implementation owner, and named public behavior case; other rows unchanged; polygon round behavior is paired with its atomic row; docs name compatibility/current access and exclusions; handoff contains no SHA, review, publication, or command-manifest state; fixture digest remains `98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8`.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog`; `cargo test -p surgeist-css --offline --no-default-features --test catalog_inventory`; `cargo test -p surgeist-css --offline --no-default-features --test function_grammars`; `cargo test -p surgeist-css --offline --no-default-features --test property_schema`; `cargo test -p surgeist-css --offline --no-default-features --test public_surface`; `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables`; repeat those six with `--features app-strict`; `cargo test -p surgeist-css --offline --no-default-features --doc`; `cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc`; `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps`; `RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps`; `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`; `cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings`; `cargo fmt --check`; `git diff --check`; `! rg -n 'TO''DO|TB''D|FIX''ME|\?''\?''\?' README.md src/lib.rs plans/handoffs/P01-I02-C05-dedicated-function-grammars.md`.
- **Intended commits:** `test: specify CSS function metadata`; `docs: publish dedicated CSS functions`.

## 5. Exact Completion Gate

After all five tasks have CLEAN task reviews and the status-only completion commit is made, run from a stale-build-process-free repository:

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
git diff --check 6b1eb1a0db6e1a26a7d8974dbd2405a874d07fdb..HEAD
rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs
! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .
test "$(shasum -a 256 tests/fixtures/i01-c01-observables.tsv | awk '{print $1}')" = 98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8
```

Directly inspect every changed test for real public or owning-private behavior and absence of source/code/count/owner-set/inventory/call-sequence/coordination proxies. Apply canonical status, holistic-review, landing, and publication contracts to exact cycle range without history rewrite or non-fast-forward push. After post-review gates:

```sh
cargo clean --offline
test ! -e target
test -z "$(git status --porcelain)"
repo_path=$(pwd -P)
for pid in $(pgrep -f 'cargo|rustc|rustdoc|surgeist_css' || true); do
  test "$pid" = "$$" && continue
  process_cwd=$(lsof -a -p "$pid" -d cwd -Fn 2>/dev/null | sed -n 's/^n//p')
  process_command=$(ps -p "$pid" -o command= 2>/dev/null || true)
  if test "$process_cwd" = "$repo_path" || printf '%s\n' "$process_command" | rg -q --fixed-strings "$repo_path"; then
    printf 'stale surgeist-css build process remains: %s %s %s\n' "$pid" "$process_cwd" "$process_command" >&2
    exit 1
  fi
done
```

The pass condition is exit zero with no reported repository build/test process; sibling-repository processes are untouched. The handoff path is `plans/handoffs/P01-I02-C05-dedicated-function-grammars.md`. A frozen I01 semantic change, second breaking I02 API change, unsafe, dependency/feature addition, external acquisition, unresolved source ownership, root/sibling edit, or fixture change is a blocker.

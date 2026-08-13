# P01-I02-C06 Color 4 And Preserved Color 5 Authored Grammar

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C06` |
| Owning repository | `surgeist-css` |
| Status | `draft` |
| Cycle base | `b6f2cfa00b9d547c926204195e105e722c0c0c42` |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `e290a7fef9bf6b6e9bde764140e5f7fac34156bb8f644d999e6bba58dc92ca2b`, P01.6-P01.9 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `c6a9984521e23d5c010c3890902b70730db42eda092ad0e77f7d9e8e6168dfa1`, sections 2, 3.1-3.2, 4.2-4.4, 8.1-8.2 colors, 9-10, 11 finding 2.13, and 12 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `09ecbf2dcaafbd402b24642f1244ce0be3568fd8a85b993c0218e2e7c0deac6d`, O-COLOR4 rows and exclusions |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `fb02bf326ae06414ac7b50e58d791962db9973cea3b8ae73b9a1d372276f645c`, entry `I02-C06` |
| Bounded outcome | Complete the selected Color 4 authored grammar and `color`/`opacity` properties, replace untyped preserved relative-color channels with per-space typed environments, and retain `color-mix()` as a truthful separately sourced Color 5 subset. |

## 2. Boundary And Reconciled Decisions

C05 candidate `b6f2cfa00b9d547c926204195e105e722c0c0c42` is published and read back. C03 typed number, percentage, angle, and math trees and C05 function/recovery infrastructure are stable inputs. The exact conformance sources are O-COLOR4 at <https://www.w3.org/TR/2026/CRD-css-color-4-20260326/> and the preserved I-COLOR5 subset at <https://www.w3.org/TR/2026/WD-css-color-5-20260618/>. Moving aliases and editor drafts are discovery aids only.

This cycle owns authored syntax, typed values, public inspection, recovery diagnostics, metadata, focused docs, and its leaf handoff. It does not evaluate relative channels or calculations, convert color spaces, normalize hues, clamp specified opacity, mix/interpolate colors, resolve system/current colors, load custom profiles, gamut-map, serialize, render, or lower into a sibling. It adds no `alpha()`, `light-dark()`, `device-cmyk()`, custom `color()` profile, `@color-profile`, contrast function, gradient, image, or new property grammar. Gradients remain C13.

`CssColor` and its existing public constructors/accessors remain the frozen I01 compatibility payload. They are not widened into a misleading authored-current model. Add a distinct parser-owned `CssAuthoredColor` current model with private fields and non-exhaustive choice types. Direct color property wrappers and every existing color-bearing current aggregate store a current color plus an optional exact `CssColor` I01 projection. Existing `color()`/`i01_subset()` compatibility accessors retain their signatures and I01 results; new `current()` or `current_color()` accessors expose `CssAuthoredColor`. Newly accepted syntax that the frozen payload cannot represent returns no I01 projection. This is additive after C01.

The current color model preserves authored branch and channel kind without resolving it: `currentcolor`, transparent, hex, named, modern/deprecated system keyword, RGB, HSL, HWB, Lab, LCH, Oklab, Oklch, predefined `color()`, Color 5 `color-mix()`, and the eight preserved relative families. Checked scalar wrappers reject non-finite values. `none`, number, percentage, angle, and typed calculation branches remain distinct wherever the source grammar distinguishes them. Exact spelling/trivia remains available through owning wrappers' `as_css()`; nested values preserve semantic components rather than serialization text.

Color 4 legacy comma RGB/HSL syntax remains distinct from modern space syntax and may not contain `none` or relative `from`. Modern syntax has exact slash and homogeneous legacy separator rules. Out-of-range finite authored color components and opacity values remain valid specified syntax; this crate does not perform the source-defined computed-value clamping. `opacity` accepts number or percentage, including signed/out-of-range finite literals and typed calculations of the corresponding root. Its current enum grows additively; `CssOpacity` and the I01 projection remain the old closed-unit literal subset.

Relative colors are limited to the eight families already preserved by I01: `rgb`/`rgba`, `hsl`/`hsla`, `hwb`, `lab`, `lch`, `oklab`, `oklch`, and `color()` in a predefined RGB or XYZ space. Custom-profile `color()` and `alpha()` are excluded. Every relative result has exactly three channels and optional alpha. Channel environments are closed:

| Family | Direct slot domains | Allowed origin identifiers in direct/math expressions |
| --- | --- | --- |
| RGB | number, percentage, `none`, typed math | `r`, `g`, `b`, `alpha` |
| HSL | hue: number/angle/`none`; saturation/lightness: number/percentage/`none`; typed math | `h`, `s`, `l`, `alpha` |
| HWB | hue: number/angle/`none`; whiteness/blackness: number/percentage/`none`; typed math | `h`, `w`, `b`, `alpha` |
| Lab/Oklab | number, percentage, `none`, typed math | `l`, `a`, `b`, `alpha` |
| LCH/Oklch | lightness/chroma: number/percentage/`none`; hue: number/angle/`none`; typed math | `l`, `c`, `h`, `alpha` |
| predefined RGB `color()` | number, percentage, `none`, typed math | `r`, `g`, `b`, `alpha` |
| XYZ `color()` | number, percentage, `none`, typed math | `x`, `y`, `z`, `alpha` |
| alpha slot | number, percentage, `none`, typed math | the selected family's identifiers, including `alpha` |

The typed relative expression records its family environment and result domain. Arbitrary identifiers, dimensions unrelated to a hue angle, wrong-space component names, malformed products/sums, and non-finite literals are invalid. The frozen valid I01 relative examples, including the established hue-angle spelling, retain their compatibility projections; typing must not reinterpret or evaluate them.

The preserved `color-mix()` implementation remains the I01 two-component form with required `in <supported-space>`, optional hue interpolation only for polar spaces, and an optional trailing percentage on each color. Color 5's optional interpolation clause, percentage-before-color ordering, one or three-plus component forms, custom spaces, and later functions remain a documented unsupported remainder. The current model uses checked construction; the existing permissive compatibility constructors remain isolated from parser-owned current state.

All parser failures remain `InvalidColorSyntax` with typed color detail, first-responsible source position, complete declaration span, `DropDeclaration`, valid sibling retention, repeated progress, and ordinary/`app-strict` parity. Recursive relative origins, color-mix components, and calculations obey the established 256 nesting limit without input-driven panic.

## 3. Impacts

| Area | Effect |
| --- | --- |
| Public API | Additive: current authored-color/channel/expression types and current accessors; additive non-exhaustive opacity variants. Existing `CssColor`, property wrapper signatures, I01 projections, and compatibility accessors remain source-compatible. |
| Dependencies/features/MSRV | Unchanged: existing `cssparser = 0.37.0`, `cssparser-color = 0.5.0`, edition 2024, `default=[]`, and `app-strict=[]`; no leaf MSRV added. |
| Generated artifacts | None in the leaf. Root owns generated API artifacts after the published candidate is promoted. |
| Docs/examples | README and crate rustdoc explain authored-current versus I01 color inspection, preserved Color 5 limits, and excluded evaluation. |
| Root follow-up | Promote the published gitlink, expose root-facade additions, refresh root-owned API artifacts, and adapt root tests/docs in a later root-owned cycle. |
| Safety | No Surgeist-owned `unsafe`, unsafe allowance, dependency acquisition, build script, generator, fixture mutation, root edit, or sibling edit. |

## 4. Exact Metadata Delta

Promote all seventeen existing O-COLOR4 rows to `Complete` without changing their stable IDs, source, or ledger production anchors: `official.value.color`, `alpha`, `hue`, `rgb`, `hex-color`, `named-color`, `system-color`, `deprecated-system-color`, `transparent`, `currentcolor`, `hsl`, `hwb`, `lab`, `lch`, `oklab`, `oklch`, and `predefined-color`. Promote `baseline.property.color` and `baseline.property.opacity` to `Complete` for their exact O-COLOR4 property grammars. Other color-valued property records do not change status in C06.

Add these I-COLOR5 records. Each public behavior case is authored independently of metadata and exercises the public parser.

| Stable ID | Production | Status and exact boundary | Named public behavior evidence |
| --- | --- | --- | --- |
| `ext.value.relative-color` | `#relative-colors,#relative-syntax` | Partial: eight preserved relative families with predefined `color()` spaces; remainder is `alpha()`, custom-profile parameters, and other unselected Color 5 color functions | `relative_color_selected_families_and_deferred_remainder_are_distinct` |
| `ext.value.relative-color.rgb` | `#relative-RGB` | Complete for preserved modern RGB/RGBA relative syntax and typed `r/g/b/alpha` environment | `relative_rgb_channels_reject_foreign_identifiers_and_dimensions` |
| `ext.value.relative-color.hsl` | `#relative-HSL` | Complete for preserved modern HSL/HSLA relative syntax and typed `h/s/l/alpha` environment | `relative_hsl_channels_keep_hue_and_percentage_domains_distinct` |
| `ext.value.relative-color.hwb` | `#relative-HWB` | Complete for preserved HWB relative syntax and typed `h/w/b/alpha` environment | `relative_hwb_channels_use_only_hwb_environment` |
| `ext.value.relative-color.lab` | `#relative-Lab` | Complete for preserved Lab relative syntax and typed `l/a/b/alpha` environment | `relative_lab_channels_use_only_lab_environment` |
| `ext.value.relative-color.oklab` | `#relative-Oklab` | Complete for preserved Oklab relative syntax and typed `l/a/b/alpha` environment | `relative_oklab_channels_use_only_oklab_environment` |
| `ext.value.relative-color.lch` | `#relative-LCH` | Complete for preserved LCH relative syntax and typed `l/c/h/alpha` environment | `relative_lch_channels_keep_hue_domain_distinct` |
| `ext.value.relative-color.oklch` | `#relative-OkLCh` | Complete for preserved Oklch relative syntax and typed `l/c/h/alpha` environment | `relative_oklch_channels_keep_hue_domain_distinct` |
| `ext.value.relative-color.predefined` | `#relative-color-function` | Complete for predefined RGB/XYZ relative `color()` spaces; custom profiles excluded by the aggregate remainder | `relative_predefined_color_channels_follow_space_environment` |
| `ext.value.color-mix` | `#funcdef-color-mix` | Partial: required interpolation method, exactly two colors, optional trailing percentages, predefined/polar spaces; remainder is the other dated Color 5 forms named in section 2 | `color_mix_preserved_subset_rejects_cross_space_hue_methods` |

All ten extension rows use source `I-COLOR5`, kind `Value`, owner `crate::parser::values`, and the exact dated source above. O-COLOR4 exclusions remain unchanged: Color 5 syntax is sourced only by these extension rows; quirky color and downstream processing remain excluded.

## 5. Tasks

For every task, “focused targets” means one exact command per listed target: `cargo test -p surgeist-css --offline --no-default-features --test <target>` and the same command with `--features app-strict`. Every task then runs `cargo test -p surgeist-css --offline --no-default-features`, the same full test with `--features app-strict`, `cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings`, the same Clippy command with `--features app-strict`, `cargo fmt --check`, and `git diff --check <task-base>..HEAD`. `<target>` and `<task-base>` are command metavariables resolved respectively from the task's finite target list and recorded assignment base SHA; they are not artifact placeholders.

### T1 Complete Authored Opacity

- **Files/area:** opacity current types in `src/syntax.rs`; parsing in `src/parser/layout.rs`; exact wrapper projection in `src/properties.rs`; new `tests/color_grammars.rs` plus `tests/numeric_domains.rs`, `tests/typed_calculations.rs`, `tests/property_schema.rs`, `tests/structured_errors.rs`, and `tests/source_coordinates.rs`. No color parser/catalog/docs edit.
- **Outcome:** accept the full finite specified `<number> | <percentage>` opacity grammar, preserve out-of-range authored values and number/percentage calculations, and retain the old I01 closed-unit number projection only where exact.
- **RED:** first commit is a base-compilable public behavioral test that executes and fails because valid `opacity: 150%` or an out-of-range finite number is dropped. Missing new API symbols may appear only in a later supplemental compile RED.
- **Acceptance:** signed/out-of-range finite literals; percentage mapping remains authored, not computed; number/percentage typed math; non-finite and unrelated dimensions rejected; ordinary/global/substitution branches; exact recovery/position/span/action/sibling retention; depth and strict parity; frozen fixture unchanged.
- **Commands:** focused targets `color_grammars`, `numeric_domains`, `typed_calculations`, `property_schema`, `structured_errors`, `source_coordinates`, followed by the exact common task matrix above.
- **Intended commits:** `test: specify Color 4 opacity`; `feat: complete authored opacity`.

### T2 Publish Authored RGB, HSL, HWB, And Keyword Colors

- **Files/area:** current color primitives/models in `src/syntax.rs`; dedicated current/compatibility parsing in `src/parser/values.rs`; direct color wrappers and parser dispatch in `src/properties.rs`; exact existing color consumer plumbing only where required; `tests/color_grammars.rs`, `property_schema.rs`, `structured_errors.rs`, `source_coordinates.rs`, `public_surface.rs`, and owning parser tests. No Lab/LCH/relative/mix/catalog/docs edit.
- **Dependency:** T1 independently clean.
- **Outcome:** typed current branches for current/transparent/hex/named/modern and deprecated system colors plus exact legacy/modern RGB, HSL, and HWB syntax, while every existing I01 value keeps its exact projection.
- **RED:** base-compilable behavior first shows rejection of a valid deprecated system color or a valid typed-math RGB/HSL/HWB channel; compile-only current inspection follows separately.
- **Acceptance:** complete named/hex widths, both system sets, exact legacy comma/homogeneous component rules, modern spaces/slash/`none`, finite out-of-range components, typed number/percentage/hue math, wrong separator/unit/arity mutations, nested/recovery/depth/non-BMP/strict parity, and no early perceptual/relative/mix change.
- **Commands:** focused targets `color_grammars`, `property_schema`, `structured_errors`, `source_coordinates`, `public_surface`, `i01_c01_observables`, followed by the exact common task matrix above.
- **Intended commits:** `test: specify authored RGB and keyword colors`; `feat: add authored RGB and keyword colors`.

### T3 Complete Lab, LCH, Oklab, Oklch, And Predefined Colors

- **Files/area:** remaining absolute current models and `src/parser/values.rs`; affected current/compatibility color consumers and wrappers; the same focused public/error/coordinate targets plus `tests/typed_calculations.rs`. No relative/mix/catalog/docs edit.
- **Dependency:** T2 independently clean.
- **Outcome:** exact typed absolute perceptual and predefined `color()` functions, preserving missing channels, component kinds, out-of-range finite state, predefined space identity, alpha, and typed calculations.
- **RED:** base-compilable behavior first shows rejection of a valid typed-math Lab/LCH/Oklab/Oklch/predefined channel or another exact dated Color 4 positive; new current symbols remain supplemental.
- **Acceptance:** all seven named predefined RGB spaces plus `xyz`/D50/D65 alias semantics; exact three channels and optional alpha; number/percentage/angle domain by slot; `none`; finite and calc boundaries; invalid space/unit/separator/arity; recursive recovery/depth/non-BMP/strict parity; I01 projection exactness.
- **Commands:** focused targets `color_grammars`, `property_schema`, `structured_errors`, `source_coordinates`, `public_surface`, `i01_c01_observables`, `typed_calculations`, followed by the exact common task matrix above.
- **Intended commits:** `test: specify perceptual and predefined colors`; `feat: complete Color 4 functions`.

### T4 Type Preserved Relative-Color Environments

- **Files/area:** relative current types in `src/syntax.rs`; relative parsing and typed expressions in `src/parser/values.rs`; current/compatibility propagation to every existing color consumer; `tests/color_grammars.rs`, `property_schema.rs`, `structured_errors.rs`, `source_coordinates.rs`, `typed_calculations.rs`, `public_surface.rs`, and owning parser tests. No color-mix/catalog/docs edit.
- **Dependency:** T3 independently clean.
- **Outcome:** close finding 2.13 with the exact per-family table in section 2 and preserve all valid I01 relative projections.
- **RED:** base-compilable public behavior first proves `rgb(from red bogus bogus bogus)` or an unrelated dimension is currently retained; the intended test executes and fails before typed API tests.
- **Acceptance:** all eight preserved families and every channel environment; direct and typed-math keyword references; wrong-space identifier/dimension rejection; exact from/origin/arity/slash rules; recursive origins, depth 255/256/257, non-BMP coordinates, diagnostic context and sibling recovery, strict parity; `alpha()` and custom profiles remain rejected; no evaluation.
- **Commands:** focused targets `color_grammars`, `property_schema`, `structured_errors`, `source_coordinates`, `typed_calculations`, `public_surface`, `i01_c01_observables`, followed by the exact common task matrix above.
- **Intended commits:** `test: reject untyped relative color channels`; `feat: type relative color environments`.

### T5 Strengthen The Preserved Color-Mix Subset And Consumer Projection

- **Files/area:** current color-mix types and checked construction in `src/syntax.rs`; `src/parser/values.rs`; current/compatibility fields and accessors for the finite existing `parse_color` consumer call sites in `src/parser/background.rs`, `src/parser/box_model.rs`, `src/parser/typography.rs`, and `src/properties.rs`; `tests/color_grammars.rs`, `function_grammars.rs`, `property_schema.rs`, `structured_errors.rs`, `source_coordinates.rs`, and `public_surface.rs`. No catalog/docs/fixture/manifest edit.
- **Dependency:** T4 independently clean.
- **Outcome:** expose a valid-by-construction current color-mix subset and ensure every current consumer carries new colors without falsely returning an I01 projection.
- **RED:** base-compilable public behavior first shows acceptance of a hue interpolation method in a rectangular space or another exact invalid preserved-subset mutation; public current-consumer inspection follows separately.
- **Acceptance:** required `in`, supported spaces, polar-only hue methods, exact two components/commas, optional trailing checked percentages, nested authored colors/order, unsupported remainder rejection, all direct and aggregate color consumers, frozen I01 debug/report behavior, global/substitution separation, full recovery/depth/non-BMP/strict parity.
- **Commands:** focused targets `color_grammars`, `function_grammars`, `property_schema`, `structured_errors`, `source_coordinates`, `public_surface`, `i01_c01_observables`, followed by the exact common task matrix above.
- **Intended commits:** `test: specify the preserved color-mix subset`; `feat: publish current color consumers`.

### T6 Publish Color Metadata, Documentation, And Handoff

- **Files/area:** `src/conformance.rs`, exact shared-value inventories, `tests/conformance_catalog.rs`, named public metadata/behavior cases, README/crate rustdoc, and new SHA-free `plans/handoffs/P01-I02-C06-color4-and-preserved-color5-authored-grammar.md`. No grammar/model/schema/fixture/manifest/root/sibling/generated-artifact edit.
- **Dependency:** T5 independently clean.
- **Outcome:** apply exactly section 4, document the current/compatibility boundary and exclusions, and publish the leaf handoff.
- **RED:** base-compilable public metadata tests first show the seventeen official rows/property rows still reserved or Partial and the ten exact extension IDs absent while paired parser behavior passes. No source/set/count/inventory proxy.
- **Acceptance:** exact IDs/kinds/sources/fragments/status/subset/remainder/owner/named behavior; unrelated rows unchanged; O-COLOR4 exclusions retained; docs accurate; handoff has no SHA/review/publication/completion/command-manifest state; fixture digest unchanged.
- **Commands:** focused targets `conformance_catalog`, `catalog_inventory`, `color_grammars`, `function_grammars`, `property_schema`, `public_surface`, `structured_errors`, `source_coordinates`, `i01_c01_observables`, followed by the exact common task matrix above; then doctests and warnings-denied rustdoc in both modes and `! rg -n 'TO''DO|TB''D|FIX''ME|\?''\?''\?' README.md src/lib.rs plans/handoffs/P01-I02-C06-color4-and-preserved-color5-authored-grammar.md`.
- **Intended commits:** `test: specify CSS color metadata`; `docs: publish authored CSS colors`.

## 6. Completion

Cycle acceptance is observable when all six tasks are independently `CLEAN`; every selected Color 4 branch and preserved Color 5 channel has typed public positive and exact invalid mutation evidence; all existing color consumers distinguish current values from I01 projections; section 4 metadata is exact; ordinary recovery and `app-strict` agree; the frozen fixture digest remains `98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8`; no prohibited test proxy, dependency/feature/artifact delta, or owned `unsafe` exists; and the SHA-free handoff is present.

The final command set is:

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
git diff --check b6f2cfa00b9d547c926204195e105e722c0c0c42..HEAD
rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs
test "$(shasum -a 256 tests/fixtures/i01-c01-observables.tsv | awk '{print $1}')" = 98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8
```

Build an explicit tracked/non-ignored owned-Rust manifest and apply the canonical Surgeist unsafe regex. Inspect every changed test for real public or owning-private behavior and the repository-wide prohibition on source/code/count/owner-set/inventory/call-sequence/coordination proxies. Apply the canonical task, holistic, landing, and publication gates. After the post-review gate, run `cargo clean --offline`, require `target` absent and a clean worktree, and require no repository `cargo`, `rustc`, `rustdoc`, or `surgeist_css` process before push/readback. Publish local `main`, read back the authority remote, and return the C06 crate-candidate handoff for root and the C07 entry.

A frozen I01 semantic change, second breaking I02 API change, acceptance/evaluation of an excluded Color 5 family, unsafe, dependency/feature addition, external acquisition, fixture change, root/sibling edit, or unresolved source ownership is a blocker.

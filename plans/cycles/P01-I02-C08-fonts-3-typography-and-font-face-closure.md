# P01-I02-C08 Fonts 3 Typography And Font-Face Closure

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C08` |
| Owning repository | `surgeist-css` |
| Status | `complete` |
| Cycle base | `21e33f121fd414c55bb229f0eab25ab41cfa7325` |
| Published prerequisite | C07 `21e33f121fd414c55bb229f0eab25ab41cfa7325`, fetched and read back |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `8e865db87cc4f68b91319664ce19ee63bcbd8df4a75943880539d11cbfadfc89`, P01.6-P01.7 and P01.11 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `be98886dc9a957a18f1dd7553b88ef8396c211001c89902de23ca5802443dc70`, sections 3.1, 3.4, 4.2-4.4, 5 font-face, 8.2 typography, 9-10, 11 finding 2.16, and 12 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `09ecbf2dcaafbd402b24642f1244ce0be3568fd8a85b993c0218e2e7c0deac6d`; exact Fonts rows |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `5efdd22358e3f35feb7764d89c226ef22bc48aa5b6ab58c776a5eb99e1080896`, entry `I02-C08` |
| Reconciliation commits | `595cbe9925a4801b1c73d0f6ab8898e5603397a6` and `2d0b5efa58c3186aa21b1e696436c787a3c088d7`; P01, I02, and sequence reviews `CLEAN` |
| Bounded outcome | Complete the selected Fonts 3 property, descriptor, source, and OpenType-tag grammars; preserve selected Fonts 4 deltas separately; apply only the reviewed one-row C08 oracle correction. |

## 2. Boundary And Decisions

The normative source is the dated Fonts 3 Recommendation at
<https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/>. Preserved deltas bind to
the dated Fonts 4 draft at
<https://www.w3.org/TR/2026/WD-css-fonts-4-20260422/>. Moving drafts are
discovery aids only. C08 owns the sixteen ledger font properties, the ten
official Fonts 3 non-property rows, selected existing Fonts 4 deltas, focused
recovery/public evidence, documentation, and a leaf handoff. It excludes font
loading/fetching/matching, feature resolution, fallback, shaping, CSSOM,
computed-value evaluation, cascade, serialization, root adapters, siblings,
and generated API artifacts.

All models are authored-phase and parser-owned or checked. New fields are
private and new enums non-exhaustive. Existing public types, constructors,
variants, property wrappers, `as_css()`, `i01_subset()`, descriptor accessors,
and parser entry-point signatures remain source-compatible. Where the I01 type
cannot represent complete syntax, a distinct current model and accessor is
added; the exact I01 subset still projects and new-only syntax returns `None`.
No parallel property identity or unvalidated public construction path is added.

The complete property grammar is fixed as follows:

- family names distinguish quoted names, unquoted identifier sequences, and
  the five generic keywords. CSS-wide keywords are valid only as the whole
  declaration; quoted global-looking names remain names. Unquoted generics are
  forbidden in `@font-face` family/local-name contexts;
- `font-size` accepts the seven absolute keywords, `larger`, `smaller`, and
  non-negative length-percentage literals or typed calculations. Shared
  line-height accepts `normal`, non-negative number, and non-negative
  length-percentage; its own metadata remains for its later owning cycle;
- `font` accepts the explicit Fonts 3 shorthand and the six system-font
  keywords only when they form the entire value. Its explicit branch preserves
  optional style, CSS2 small-caps variant, weight, stretch, size, optional
  line-height, and nonempty family list;
- `font-kerning` is `auto|normal|none`; `font-size-adjust` is `none` or a
  non-negative number; `font-synthesis` is `none` or the unordered nonempty
  unique set `weight||style`;
- the five variant longhands implement their exact `normal`/`none`, mutually
  exclusive keyword-group, duplicate, and unordered grammars. The shorthand is
  `normal`, `none`, or the compatible union of ligature, position, caps,
  numeric, and East Asian components, rejecting every group conflict;
- OpenType tags contain exactly four decoded ASCII characters. Escaped ASCII
  remains valid; non-ASCII and supplementary characters do not. Feature
  indices are integers greater than or equal to zero; omitted/`on`/`off` keep
  their authored distinction.

Add checked current `CssOpenTypeTag` and `CssFontFeatureIndex`, and current
property models for font size, line height, explicit/system font shorthand,
feature settings, synthesis, and variant groups. Property wrappers expose a
semantically named borrowed current accessor and retain the I01 projection.
The eight missing ledger properties add exact schema rows and wrappers:
`official.property.font-kerning`, `.font-size-adjust`, `.font-synthesis`,
`.font-variant-caps`, `.font-variant-east-asian`,
`.font-variant-ligatures`, `.font-variant-numeric`, and
`.font-variant-position`.

Font-face stores every valid descriptor occurrence in authored order. Add a
non-exhaustive borrowed `CssFontFaceDescriptorRef` and
`CssFontFaceDescriptors::occurrences()`. Existing typed accessors return the
last valid occurrence; `font_feature_settings()` is added. Invalid/unknown/
important occurrences recover independently with `DropDescriptor` and never
erase a valid occurrence. A rule is retained when at least one valid effective
`font-family` and `src` remain; otherwise child diagnostics precede the existing
parent `DropAtRule` diagnostic.

Fonts 3 descriptor values include one non-generic family; a nonempty source
list of `local()` or URL sources; URL `format(<string>#)` with arbitrary strings;
the nine Fonts 3 stretch keywords; normal/bold/100-through-900 weight; normal,
italic, or oblique style; unicode ranges; and feature settings. Add checked
current source-format strings and `formats()` while preserving the singular
recognized `format()` compatibility view. Preserve the already-selected Fonts
4 keyword format and `tech()` hints, 1-through-1000 numeric/increasing weight
ranges, oblique degree ranges, non-negative percentage stretch ranges, and
`font-display`; do not add unselected Fonts 4 descriptors or values.

## 3. Exact Oracle Correction

The fixture before C08 has SHA-256
`99bbb897710969949d7b596d14fbd352d5d3121a6c4cf663b8ca100154057f8b`.
Its `focused.structured-errors.12` row preserves this authored input:

```css
@font-face { font-family: One; font-family: Two; src: url(test.woff2); }
```

Replace only its clean/retained/diagnostic observables: the report becomes
clean, retains `rule:baseline.rule.font-face`, and has no diagnostic. The
replacement fixture SHA-256 is
`67e69813d808ffda40e7c159fde719fbadd0447f8e4105788b0bb593931fac89`.
The behavior task hand-authors this row before production changes, so the public
fixture reader executes RED. No Rust test asserts a digest, reads Rust source,
derives expected behavior from production, masks the row, or compares owner
sets/counts. Task review directly verifies exactly one row changed and public
tests separately prove occurrence order `[One, Two]` and effective value `Two`.

## 4. Impacts

The public API effect is additive current authored models, accessors, property
identities, and metadata; frozen compatibility signatures remain. Dependencies,
features, manifests, build logic, and leaf-generated artifacts are unchanged.
The leaf declares no independent MSRV; C08 uses the existing edition/toolchain
contract and does not change the compatible MSRV owned by root integration.
T7 updates leaf docs/examples and the product handoff. After publication, root
alone may promote the gitlink, regenerate API audit artifacts, and verify its
MSRV. No root or sibling file changes in this cycle. All owned Rust remains
free of `unsafe`.

## 5. Metadata Delta

Promote all sixteen O-FONTS3 property ledger rows and all ten O-FONTS3
non-property ledger rows to `Complete` with direct named behavior. Promote
`baseline.descriptor.font-display` to `Complete` under I-FONTS4 and retain
`later.rule.font-feature-values` as `RecognizedUnsupported`. Add separate
I-FONTS4 `Partial` atomic rows for the preserved property numeric-weight delta,
descriptor weight ranges, descriptor oblique ranges, descriptor stretch
ranges, and modern source hints. Their exact stable IDs are respectively
`ext.property.font-weight-range`, `ext.descriptor.font-weight-range`,
`ext.descriptor.font-style-oblique-range`,
`ext.descriptor.font-stretch-range`, and `ext.value.font-source-modern-hints`.
Each subset names only the behavior in section 2 and each remainder names the
unselected Fonts 4 grammar. No source, ledger count, exclusion, baseline alias,
dependency, feature, or existing stable-ID meaning changes.

## 6. Tasks

At assignment start each worker records `task_base_sha="$(git rev-parse HEAD)"`.
After its focused loop it runs this exact common GREEN tail:

```sh
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
cargo fmt --check
git diff --check "${task_base_sha}..HEAD"
```

Every test parses authored CSS only through public or crate-front-door behavior.
Parsing or inspecting Rust source, files, symbols, registrations, owner
sets/counts, workflow state, or incidental call counts is prohibited.

### T1 Complete Family, Size, Line-Height, And Font Shorthand

- **Dependency:** this plan independently clean.
- **Area:** `src/syntax.rs`, `src/parser/typography.rs`, font-size/line-height/
  family/font representations in `src/properties.rs`; new
  `tests/font_property_grammars.rs` plus `property_schema`, `public_surface`,
  `structured_errors`, `source_coordinates`, and `typed_calculations`.
- **RED:** existing public API must fail because `font-size:medium` or
  `font:menu` is dropped. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test font_property_grammars font_size_family_line_height_and_shorthand_follow_fonts3 -- --exact`.
- **Acceptance:** section 2 exact keywords/branches; global/generic/quoted and
  signed boundaries; system-keyword position; shorthand order/duplicates;
  current/I01 views; exact diagnostics, sibling retention, non-BMP coordinates,
  typed math, and strict parity.
- **Focused:** both feature modes for `font_property_grammars property_schema public_surface structured_errors source_coordinates typed_calculations`.

  ```sh
  for target in font_property_grammars property_schema public_surface structured_errors source_coordinates typed_calculations; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in font_property_grammars property_schema public_surface structured_errors source_coordinates typed_calculations; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  ```
- **Commits:** `test: specify Fonts 3 core font properties`; `feat: complete Fonts 3 core font properties`.

### T2 Check OpenType Tags And Feature Indices

- **Dependency:** T1 independently clean.
- **Area:** shared feature models/parser in `src/syntax.rs` and
  `src/parser/typography.rs`; property wrapper in `src/properties.rs`;
  `font_property_grammars`, `property_schema`, `public_surface`,
  `structured_errors`, `source_coordinates`.
- **RED:** existing public API accepts a non-ASCII four-scalar tag or negative
  index. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test font_property_grammars opentype_tags_and_indices_enforce_ascii_and_nonnegative_domains -- --exact`.
- **Acceptance:** ASCII/escape/supplementary/length boundaries; omitted/on/off/
  zero/positive distinctions; checked current construction; I01 compatibility;
  list recovery, exact positions/spans/action, sibling and strict parity.
- **Focused:** both modes for `font_property_grammars property_schema public_surface structured_errors source_coordinates`.

  ```sh
  for target in font_property_grammars property_schema public_surface structured_errors source_coordinates; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in font_property_grammars property_schema public_surface structured_errors source_coordinates; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  ```
- **Commits:** `test: specify checked OpenType features`; `feat: check OpenType features`.

### T3 Add Kerning, Size-Adjust, And Synthesis

- **Dependency:** T2 independently clean.
- **Area:** three schema rows/wrappers, models and typography parsers;
  `font_property_grammars`, `property_schema`, `public_surface`,
  `numeric_domains`, `structured_errors`, `source_coordinates`.
- **RED:** base-compilable public parsing reports each canonical spelling as
  `UnknownProperty`. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test font_property_grammars kerning_size_adjust_and_synthesis_follow_fonts3 -- --exact`.
- **Acceptance:** all section 2 branches, order independence, duplicate/group/
  negative/adjacent mutations, globals/substitution, typed public inspection,
  exact recovery, sibling retention, strict parity.
- **Focused:** both modes for `font_property_grammars property_schema public_surface numeric_domains structured_errors source_coordinates`.

  ```sh
  for target in font_property_grammars property_schema public_surface numeric_domains structured_errors source_coordinates; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in font_property_grammars property_schema public_surface numeric_domains structured_errors source_coordinates; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  ```
- **Commits:** `test: specify Fonts 3 control properties`; `feat: add Fonts 3 control properties`.

### T4 Complete Variant Longhands And Shorthand

- **Dependency:** T3 independently clean.
- **Area:** five schema rows/wrappers; variant models/parsers and existing
  shorthand; `font_property_grammars`, `property_schema`, `public_surface`,
  `structured_errors`, `source_coordinates`.
- **RED:** public parsing reports the five canonical longhands unknown and drops
  a combined valid shorthand. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test font_property_grammars font_variant_longhands_and_shorthand_enforce_keyword_groups -- --exact`.
- **Acceptance:** every keyword/group, unordered combination, duplicate and
  conflict mutation, `normal`/`none` isolation, globals/substitution, current/
  I01 views, diagnostics, siblings, non-BMP, strict parity.
- **Focused:** both modes for `font_property_grammars property_schema public_surface structured_errors source_coordinates`.

  ```sh
  for target in font_property_grammars property_schema public_surface structured_errors source_coordinates; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in font_property_grammars property_schema public_surface structured_errors source_coordinates; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  ```
- **Commits:** `test: specify Fonts 3 variant properties`; `feat: complete Fonts 3 variant properties`.

### T5 Complete Font Source And Local-Name Grammar

- **Dependency:** T4 independently clean.
- **Area:** source/current-format models in `src/syntax.rs`; source/family/local
  parsers in `src/parser/font_face.rs` and shared typography family parsing;
  new `tests/font_face_grammars.rs` plus `public_surface`, `structured_errors`,
  `source_coordinates`, `specialized_recovery_boundaries`.
- **RED:** public parsing drops `format("woff2","opentype")` or arbitrary
  string `format("zebra")`. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test font_face_grammars font_sources_preserve_fonts3_formats_and_selected_fonts4_hints -- --exact`.
- **Acceptance:** nonempty ordered sources; URL/local; arbitrary Fonts 3 string
  format list; exact selected keyword format/tech hints and ordering; empty/
  separator/unknown-ident mutations; quoted versus unquoted global/generic
  names; current/compatibility views; EOF closures, diagnostics, siblings,
  255/256/257 component depth, non-BMP, strict parity.
- **Focused:** both modes for `font_face_grammars public_surface structured_errors source_coordinates specialized_recovery_boundaries`.

  ```sh
  for target in font_face_grammars public_surface structured_errors source_coordinates specialized_recovery_boundaries; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in font_face_grammars public_surface structured_errors source_coordinates specialized_recovery_boundaries; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  ```
- **Commits:** `test: specify Fonts 3 font sources`; `feat: complete Fonts 3 font sources`.

### T6 Preserve Descriptor Occurrences And Complete Descriptor Values

- **Dependency:** T5 independently clean.
- **Area:** font-face aggregate/views in `src/syntax.rs`; descriptor parser and
  inventory in `src/parser/font_face.rs`; exact fixture row; replace stale
  duplicate oracles; `font_face_grammars`, `block_item_recovery`,
  `structured_errors`, `source_coordinates`, `structural_recovery_adversarial`,
  `app_strict_parity`, `public_surface`, `i01_c01_observables`.
- **RED:** first commit hand-authors the exact fixture row and adds existing-API
  behavior that fails because valid duplicates diagnose/drop and the effective
  family is `One`. Exact commands:
  `cargo test -p surgeist-css --offline --no-default-features --test font_face_grammars font_face_preserves_occurrences_and_uses_last_valid_descriptor -- --exact` and
  `cargo test -p surgeist-css --offline --no-default-features --test i01_c01_observables authored_css_cases_match_frozen_public_report_observables -- --exact`.
- **Acceptance:** authored order and effective-last for every descriptor;
  valid-invalid-valid and required-set behavior; Fonts 3 weight/stretch/style/
  unicode/feature values; selected Fonts 4 ranges/display; unknown, invalid,
  important, EOF, repeated-failure, depth, non-BMP, parent/sibling recovery;
  validators and strict parity; replacement fixture digest exact and only the
  reviewed row changed.
- **Focused:** both modes for `font_face_grammars block_item_recovery structured_errors source_coordinates structural_recovery_adversarial app_strict_parity public_surface i01_c01_observables`.

  ```sh
  for target in font_face_grammars block_item_recovery structured_errors source_coordinates structural_recovery_adversarial app_strict_parity public_surface i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in font_face_grammars block_item_recovery structured_errors source_coordinates structural_recovery_adversarial app_strict_parity public_surface i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  ```
- **Commits:** `test: specify complete Fonts 3 descriptors`; `feat: preserve complete Fonts 3 descriptors`.

### T7 Publish Fonts Metadata, Docs, And Handoff

- **Dependency:** T1-T6 independently clean.
- **Area:** `src/conformance.rs`; parser inventories only; direct named metadata
  tests; README, crate rustdoc, and SHA-free
  `plans/handoffs/P01-I02-C08-fonts-3-typography-and-font-face-closure.md`.
- **RED:** paired grammar behavior passes but direct named metadata remains
  Partial/Reserved. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog fonts3_and_preserved_fonts4_metadata_are_truthful -- --exact`.
- **Acceptance:** section 5 exact rows/source/fragments/status/subset/remainder;
  no set/count proxy; docs expose current/I01, recovery, and downstream limits;
  handoff records only the two product fixture digests among otherwise SHA-free
  product facts; doctests and warning-denied rustdoc pass.
- **Focused:** both modes for `conformance_catalog catalog_inventory font_property_grammars font_face_grammars property_schema public_surface structured_errors source_coordinates i01_c01_observables`; then both doctest and warning-denied rustdoc modes.

  ```sh
  for target in conformance_catalog catalog_inventory font_property_grammars font_face_grammars property_schema public_surface structured_errors source_coordinates i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --test "$target"; done
  for target in conformance_catalog catalog_inventory font_property_grammars font_face_grammars property_schema public_surface structured_errors source_coordinates i01_c01_observables; do cargo test -p surgeist-css --offline --no-default-features --features app-strict --test "$target"; done
  cargo test -p surgeist-css --offline --no-default-features --doc
  cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
  RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features
  RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features --features app-strict
  ```
- **Commits:** `test: specify Fonts 3 and Fonts 4 metadata`; `docs: publish Fonts 3 typography and font-face closure`.

## 7. Final Gate, Publication, And Completion

After all seven task ranges are independently `CLEAN`, make the separate
status-only `complete` commit, then run:

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
git diff --check 21e33f121fd414c55bb229f0eab25ab41cfa7325..HEAD
shasum -a 256 tests/fixtures/i01-c01-observables.tsv
git diff --unified=0 21e33f121fd414c55bb229f0eab25ab41cfa7325..HEAD -- tests/fixtures/i01-c01-observables.tsv
rg -n 'unsafe|unsafe_code' --glob '*.rs' src tests
git status --short --branch
ps -axo pid=,command=
```

The fixture output must be `67e69813d808ffda40e7c159fde719fbadd0447f8e4105788b0bb593931fac89`;
direct diff shows only section 3's row. Classify the Rust scan directly:
crate-level `forbid(unsafe_code)` and authored CSS strings are not executable
unsafe; any other match blocks. No repository Cargo/rustc/rustdoc/
`surgeist_css` process may remain. A fresh holistic reviewer then reviews exact
range `21e33f121fd414c55bb229f0eab25ab41cfa7325..HEAD`.

Only after holistic `CLEAN` run `cargo clean --offline`, prove `target` absent,
the worktree/process state clean, and candidate ancestry from C07. Fetch
`origin/main`, require it still equals the C07 lease SHA, push the immutable
candidate with explicit force-with-lease to `origin/main`, fetch again, and
require local `HEAD`, `main`, `origin/main`, and `git ls-remote` to equal the
candidate. C08 completes only after the sixteen official properties and ten
official non-property rows are Complete, selected Fonts 4 deltas remain
truthful, the exact one-row fixture correction is directly verified, all prior
evidence is green, task and holistic reviews are clean, cleanup passes, and the
published/read-back candidate is handed off with root-only follow-up. A
material contract contradiction, second public break, unsafe requirement,
external acquisition, or root/sibling mutation is a blocker.

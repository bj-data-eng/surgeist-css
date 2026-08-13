# P01-I02-C09 Media, Supports, Import, And Prelude Ordering

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C09` |
| Owning repository | `surgeist-css` |
| Status | `draft` |
| Cycle base | `129de7267726277b73d2cc15f1168c44c34ffcbc` |
| Published prerequisite | C08 `129de7267726277b73d2cc15f1168c44c34ffcbc`, fetched and read back |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `3a2f997f5c0a07566d6620b031b5010defb19d57ed0a878de57069cd97c4efe5`, especially P01.12 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `243379a1c1d5004904675e509d6bd23c6750cc8652bb4bfe9acad0214c6ab2c2`, sections 3.1, 3.5, 4.2-4.4, 5, 7, 10-12 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `626d176a734d48c3a6202c189daeadc5ff93253c20ac6681d91f93b01ab11b0d`; media, conditional, Cascade, and imported-value rows |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `6970425207846e2a5430779c06d0503ec7a1dac8b195b534e9a95aeefbd1fab1`, entry `I02-C09` |
| Bounded outcome | Complete Media Queries 3 and Conditional Rules 3 authored syntax, add typed Cascade 4/5 import conditions, install the top-level prelude phase machine, preserve selected later-level query syntax truthfully, and apply only the reviewed eight-row oracle correction. |

## 2. Sources, Boundary, And Decisions

The normative source revisions are:

- Media Queries 3: <https://www.w3.org/TR/2024/REC-mediaqueries-3-20240521/>;
- Conditional Rules 3: <https://www.w3.org/TR/2024/CRD-css-conditional-3-20240815/>;
- Cascade 4 core import grammar:
  <https://www.w3.org/TR/2022/CR-css-cascade-4-20220113/>;
- Cascade 5 import-layer and ordering deltas:
  <https://www.w3.org/TR/2022/CR-css-cascade-5-20220113/>;
- Conditional 4 selector-test delta:
  <https://www.w3.org/TR/2025/CRD-css-conditional-4-20250904/>;
- the imported `<general-enclosed>` production only, at immutable csswg-drafts
  commit `720ea2863696971ea6a6744e0f23acbb3e6936bd`, file
  `css-values-4/Overview.bs`.

Moving aliases and editor drafts are discovery aids only. This cycle owns
authored syntax and browser-style recovery. It excludes media/supports
evaluation, device matching, cascade, substitution, selector matching,
resource loading, CSSOM, namespace syntax, root adapters, sibling crates, and
generated API artifacts.

All new public models have private fields; new enums are non-exhaustive;
parser-produced source positions cannot be forged. Existing parser entry
points, `CssParseReport`, diagnostics/actions, `CssMediaQuery` variants,
`CssMediaQueryList::try_new`, existing media feature variants and accessors,
I01 projections, and ordinary/`app-strict` relationship remain source
compatible. Existing crate-private constructors may change with their models.

### 2.1 Media Query Model

Implement every MQ3 type: `all`, `aural`, `braille`, `embossed`, `handheld`,
`print`, `projection`, `screen`, `speech`, `tty`, and `tv`. Add the missing
feature families and their min/max forms where the source permits them:
width/height, device-width/device-height, orientation, aspect-ratio,
device-aspect-ratio, color, color-index, monochrome, resolution, scan, and
grid. Every feature supports its MQ3 boolean form except a min/max-prefixed
name; boolean state uses an additive `CssMediaFeatureQuery::Boolean` branch and
a non-exhaustive feature-name enum rather than changing existing variant
payloads.

Use an additive positive-integer MQ3 ratio model; do not weaken or repurpose
the existing general `CssRatio`. MQ3 resolution accepts positive `dpi` and
`dpcm`; preserved `dppx` remains accepted but is separately sourced as
`ext.media.resolution.dppx` under `R-MEDIA4`. Preserve existing one-sided MQ4
comparison forms and all already-catalogued MQ4/MQ5 discrete features without
claiming them as MQ3.

An empty media-query list is valid and retained for `@media {}`. The parser may
construct an empty `CssMediaQueryList`; public `try_new` remains the checked
nonempty constructor and `queries()` exposes the empty slice. This is authored
syntax only; the crate does not evaluate the list.

Unknown MQ3 types and syntactically complete unknown feature names or values
are valid defined-false authored syntax and emit no diagnostic. Add
parser-owned private-field models preserving exact authored spelling/text,
first non-trivia position, and a non-exhaustive reason enum. `CssMediaType`
gains the unit `Unknown` classification while the positioned typed query
exposes the exact unknown spelling. `CssMediaConditionKind` gains a distinct
defined-false branch for the complete parenthesized expression. These states
are not `CssNeverMediaQuery`; `Never` remains only a malformed list-member
replacement paired with `ReplaceMediaQueryWithNever`.

Reserved type spellings `layer`, `not`, `and`, `only`, and `or`, unexpected
tokens, empty comma members, invalid min/max boolean forms, and structurally
malformed members still recover as `Never`. The recognized deferred MQ4
`scripting` feature keeps its frozen `InvalidMediaQuery`/`Never` behavior.
Balanced unsupported values—including `width: calc(1px)`, negative lengths,
unknown units, and unknown keywords—are defined-false, not malformed.

### 2.2 Supports And General-Enclosed

Add positioned `CssSupportsCondition` plus a non-exhaustive kind with
declaration, selector, general-enclosed, `Not`, `And`, and `Or` branches. Lists
are checked nonempty multi-item models. Mixing `and` and `or` at one level is
malformed; grouping makes the mix valid. The condition parser does not import
the adjacent editor-draft generic boolean grammar.

`CssSupportsDeclaration` preserves exact authored declaration text, property
spelling, importance, and position. It attempts the authoritative property
schema and exposes an optional property-specific known declaration view, but it
is not inserted into a style declaration list. Unknown properties, unsupported
or invalid property values, and empty-but-syntactic declaration values remain
valid supports tests without ordinary declaration diagnostics. Custom
properties preserve authored values. No branch evaluates whether the condition
matches a user agent.

`CssGeneralEnclosed` retains an exact balanced function or parenthesis unit and
position. Recognized declaration/group/selector forms are attempted before the
fallback. Function, parenthesis, string, comment, escape, and nested-block
boundaries receive exact 255/256/257 depth evidence; malformed or over-depth
units recover at the containing conditional rule boundary with the established
diagnostic/action model.

The separate `selector(<complex-selector>)` branch delegates to the current
complex-selector parser. A currently supported selector is typed; other
balanced selector-function content remains authored general-enclosed/false
syntax rather than causing parent loss. `ext.supports.selector` is Partial in
C09 with the exact current selector subset/remainder; C10 expands that same row
after namespaces and complete Selectors 3.

Add `CssRule::Supports(CssSupportsRule)` for stylesheet and conditional-group
rule lists. Add `CssScopedRule::Supports(CssScopedSupportsRule)` for scoped
lists. Nested style rules retain a `CssRule::Supports` with the established
nested style-rule semantics. Supports may nest with media, supports, container,
layer, and scope where style rules are allowed; import, font-face, and
keyframes placement restrictions remain. Invalid children recover without
dropping a valid conditional parent; child diagnostics precede a parent-drop
diagnostic when the parent itself is invalid.

### 2.3 Import Clauses And Top-Level Phases

`CssImportRule` adds a private optional `CssImportSupports` field and a borrowed
`supports()` accessor. The exact prelude order is target, optional `layer` or
`layer(name)`, optional `supports(...)`, optional media list. The import
supports wrapper accepts a full supports condition or a bare declaration whose
wrapper supplies the declaration parentheses. Duplicated, swapped, or trailing
clauses are invalid. Existing target/layer/media accessors and target parsing
remain unchanged.

Replace `imports_allowed` with the following internal phase machine; keep the
encoding declaration's independent first-rule/one-shot check:

| Current phase | Successful initial layer statement | Successful import | Successful namespace (C10 hook) | Successful body rule |
| --- | --- | --- | --- | --- |
| `Initial` | `Initial` | `Imports` | `Namespaces` | `Body` |
| `Imports` | `Body` | `Imports` | `Namespaces` | `Body` |
| `Namespaces` | `Body` | invalid, unchanged | `Namespaces` | `Body` |
| `Body` | `Body` | invalid, unchanged | invalid, unchanged | `Body` |

Only successful rule parsing transitions the phase. A valid initial layer
statement therefore permits a following import. A layer statement after an
import or namespace closes both later imports and namespaces. Failed or
recovered malformed rules do not mutate phase. C09 reserves the explicit
namespace phase and transition hook but does not recognize, parse, retain,
diagnose differently, or promote `@namespace`; C10 remains its sole grammar
owner and activates that hook.

## 3. Exact Oracle Correction

The fixture before C09 has SHA-256
`67e69813d808ffda40e7c159fde719fbadd0447f8e4105788b0bb593931fac89`.
Only these stable IDs and authored inputs change:

1. `catalog.non-property.baseline.media.range-feature.boundary`;
2. `catalog.non-property.baseline.media.type.boundary`;
3. `catalog.non-property.baseline.rule.media.boundary`;
4. `focused.specialized.media-position`;
5. `focused.structured-errors.08`;
6. `catalog.non-property.later.rule.supports.boundary`;
7. `focused.structured-errors.01`;
8. `catalog.non-property.baseline.rule.import.boundary`.

The first five become clean through T1/T2, retaining their existing media rule
and children while removing the obsolete query diagnostic. The two supports
rows become clean through T3 and retain respectively the supports rule plus its
nested style/color and the empty supports rule. The import row becomes clean
through T4 and retains the import. The hand-authored replacement fixture has
SHA-256
`728f73c13d57dc526a02f58a68c672faf7e8e0fd1911ac30d76aba5e63be0b9d`.
Every task edits its rows before production so the public fixture reader runs
RED. T5 and final review verify exactly eight changed rows and byte identity for
all others. No Rust test asserts either digest, reads Rust source, derives an
expectation from production, masks a row, or compares owner sets/counts.

## 4. Metadata Delta

Promote all fifteen `O-MEDIA3` rows and all three `O-CONDITIONAL3` rows to
`Complete` with direct behavior. Promote the Cascade 4
`baseline.rule.import` row—including `supports()`—to `Complete`. Preserve all selected MQ4/MQ5 query
rows at their current exact source/tier/status unless this cycle completes their
already-supported subset.

Add or activate these distinct atomic rows without changing a baseline alias's
meaning:

- `ext.media.resolution.dppx` at `R-MEDIA4#resolution`;
- `ext.supports.general-enclosed` at immutable `X-VALUES4` provenance;
- `ext.supports.selector` at `R-CONDITIONAL4#at-supports`, `Partial`;
- `ext.import.layer` at `R-CASCADE5#at-import`;
- `ext.stylesheet.prelude-order` at the exact Cascade 5 ordering fragment.

The official Cascade 4 import row owns `supports()`; it never absorbs the
Cascade 5 layer or ordering deltas. General-enclosed never absorbs
the sibling generic boolean grammar. Conditional 3 never absorbs selector().
MQ3 never absorbs dppx, comparison syntax, or later media features.

## 5. Impacts

The public API effect is additive: current MQ3 feature/value models,
defined-false models, supports conditions/rules, scoped supports, import
supports, and accessors. Dependencies, features, manifests, build logic, and
leaf-generated artifacts do not change. The leaf declares no independent MSRV;
root alone owns compatible-pin verification, facade/adapters, gitlink promotion,
and API generation after publication. All owned Rust remains free of `unsafe`.

## 6. Tasks

At assignment start each worker records `task_base_sha="$(git rev-parse HEAD)"`.
Every task uses two commits: the first adds a base-compilable public behavioral
RED and runs its exact command to failure; only then may production symbols or
behavior change. New-symbol API assertions are added after that executable RED.
Every test parses authored CSS through public/front-door behavior or directly
inspects public metadata. No test parses or inspects Rust source, files, tokens,
ASTs, symbols, registrations, call sites, owner sets/counts, workflow state, or
incidental invocation counts.

After its focused loop every task runs this common GREEN tail:

```sh
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
cargo fmt --check
git diff --check "${task_base_sha}..HEAD"
```

Each exact two-commit task range then receives a fresh independent
`surgeist-task-reviewer` and must be `CLEAN` before the next task begins.

### T1 Complete Named MQ3 Types And Features

- **Dependency:** this plan independently clean.
- **Area:** `src/syntax.rs`, `src/parser/queries.rs`; new
  `tests/media_query_grammars.rs`; exact fixture rows 1-2; focused updates to
  `public_surface`, `structured_errors`, `source_coordinates`,
  `specialized_list_recovery`, `typed_calculations`, and stale direct MQ tests.
- **RED:** hand-author rows 1-2 and public behavior proving `speech` and
  `device-width` are clean. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test media_query_grammars mq3_named_types_and_features_follow_exact_domains -- --exact`.
- **Acceptance:** eleven types; every feature and permitted prefix/boolean form;
  positive-integer ratio; dpi/dpcm and separately preserved dppx; scan/grid
  keywords/integer domain; empty query list; preserved MQ4 comparison/discrete
  syntax; current/compatibility views; malformed domain boundaries, exact
  diagnostics, sibling retention, positions, strict parity.
- **Focused:** both feature modes for `media_query_grammars public_surface structured_errors source_coordinates specialized_list_recovery typed_calculations i01_c01_observables`.
- **Commits:** `test: specify complete MQ3 named grammar`; `feat: complete MQ3 named grammar`.

### T2 Preserve MQ3 Defined-False Syntax

- **Dependency:** T1 independently clean.
- **Area:** defined-false/query models in `src/syntax.rs`; classification and
  recovery in `src/parser/queries.rs`; exact fixture rows 3-5;
  `media_query_grammars`, `structured_errors`, `source_coordinates`,
  `specialized_list_recovery`, `app_strict_parity`, `public_surface`,
  `i01_c01_observables`.
- **RED:** hand-author rows 3-5 and existing-API behavior proving a balanced
  unknown feature/value is clean while a reserved type and `scripting` still
  diagnose/recover. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test media_query_grammars defined_false_media_syntax_is_not_malformed_recovery -- --exact`.
- **Acceptance:** unknown type spelling and modifiers; unknown/prefixed feature;
  unknown keyword/unit/function and negative known values; exact authored text
  and positions; clean/no-diagnostic defined-false; `Never` plus diagnostic for
  reserved/malformed/empty members; frozen scripting boundary; comma-local
  recovery, EOF, repeated failures, non-BMP, 255/256/257 balanced nesting,
  validator and strict parity.
- **Focused:** both modes for `media_query_grammars structured_errors source_coordinates specialized_list_recovery app_strict_parity public_surface i01_c01_observables`.
- **Commits:** `test: specify MQ3 defined-false syntax`; `feat: preserve MQ3 defined-false syntax`.

### T3 Add Supports Conditions And Conditional Rules

- **Dependency:** T2 independently clean.
- **Area:** supports models in `src/syntax.rs`; new `src/parser/supports.rs` and
  integration in parser/nesting/scoped coordinators; narrowly required selector
  parser visibility only; exact fixture rows 6-7; new
  `tests/supports_grammars.rs`; `block_item_recovery`, `nested_structural_recovery`,
  `structural_recovery_adversarial`, `structured_errors`, `source_coordinates`,
  `specialized_recovery_boundaries`, `app_strict_parity`, `public_surface`,
  `i01_c01_observables`.
- **RED:** hand-author rows 6-7 and public behavior proving valid `@supports`
  is currently unsupported/dropped. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test supports_grammars supports_conditions_and_group_rules_follow_conditional3 -- --exact`.
- **Acceptance:** declaration/custom/unknown/invalid-value tests; importance and
  empty values; not/and/or/grouping and no mixed operator; exact
  general-enclosed function/parenthesis retention; typed current selector
  subset and balanced fallback; top-level, conditional, nested-style, and scoped
  contexts; invalid placements; child/parent/sibling recovery; EOF, repeated,
  non-BMP, depth, positions, validators, strict parity.
- **Focused:** both modes for `supports_grammars block_item_recovery nested_structural_recovery structural_recovery_adversarial structured_errors source_coordinates specialized_recovery_boundaries app_strict_parity public_surface i01_c01_observables`.
- **Commits:** `test: specify Conditional 3 supports grammar`; `feat: add Conditional 3 supports grammar`.

### T4 Add Import Supports And Prelude Phases

- **Dependency:** T3 independently clean.
- **Area:** import model and top-level phase machine in `src/syntax.rs` and
  `src/parser/mod.rs`; shared supports parser; exact fixture row 8; new
  `tests/conditional_ordering.rs`; `stylesheet_recovery`, `structured_errors`,
  `source_coordinates`, `specialized_list_recovery`, `app_strict_parity`,
  `public_surface`, `i01_c01_observables`; replace only directly contradictory
  stale ordering assertions.
- **RED:** hand-author row 8 and public behavior proving import supports and an
  initial layer statement followed by import are rejected on base. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test conditional_ordering import_conditions_and_prelude_phases_follow_cascade -- --exact`.
- **Acceptance:** target/layer/supports/media exact order and accessors; bare
  declaration versus full condition; duplicates/swaps/trailing mutation;
  malformed condition recovery; complete table in section 2.3 through currently
  active inputs; explicit dormant namespace hook inspected directly by review;
  only-success transitions; encoding interaction; top-level-only import;
  diagnostics/actions/spans/order, siblings, EOF/non-BMP, strict parity; final
  fixture digest and exact eight-row diff.
- **Focused:** both modes for `conditional_ordering supports_grammars media_query_grammars stylesheet_recovery structured_errors source_coordinates specialized_list_recovery app_strict_parity public_surface i01_c01_observables`.
- **Commits:** `test: specify conditional import ordering`; `feat: add conditional import ordering`.

### T5 Publish Conditional Metadata, Docs, And Handoff

- **Dependency:** T1-T4 independently clean.
- **Area:** `src/conformance.rs`; parser inventories only; direct named metadata
  tests; README, crate rustdoc/doctests, and SHA-free
  `plans/handoffs/P01-I02-C09-media-supports-import-and-prelude-ordering.md`.
- **RED:** paired grammar behavior passes while direct named media/conditional/
  import metadata remains Partial/Reserved. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog media_conditional_and_import_metadata_are_truthful -- --exact`.
- **Acceptance:** section 4 exact IDs/source/fragments/status/subset/remainder;
  direct review reconciles all 15+3 official rows and distinct extension rows;
  no set/count proxy; docs show defined-false versus Never, supports inspection,
  import clauses/order, recovery, and downstream exclusions; handoff records only
  the two product fixture digests among otherwise SHA-free product facts;
  doctests and warning-denied rustdoc pass.
- **Focused:** both modes for `conformance_catalog catalog_inventory media_query_grammars supports_grammars conditional_ordering public_surface structured_errors source_coordinates i01_c01_observables`; then:

  ```sh
  cargo test -p surgeist-css --offline --no-default-features --doc
  cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
  RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features
  RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features --features app-strict
  ```
- **Commits:** `test: specify conditional metadata`; `docs: publish media and conditional closure`.

## 7. Final Gate, Publication, And Completion

After all five task ranges are independently `CLEAN`, make the separate
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
git diff --check 129de7267726277b73d2cc15f1168c44c34ffcbc..HEAD
shasum -a 256 tests/fixtures/i01-c01-observables.tsv
git diff --unified=0 129de7267726277b73d2cc15f1168c44c34ffcbc..HEAD -- tests/fixtures/i01-c01-observables.tsv
rg -n 'unsafe|unsafe_code' --glob '*.rs' src tests
git status --short --branch
ps -axo pid=,command=
```

The fixture output must be
`728f73c13d57dc526a02f58a68c672faf7e8e0fd1911ac30d76aba5e63be0b9d`;
direct diff must show only section 3's eight rows. Classify the Rust scan
directly: crate-level `forbid(unsafe_code)` and authored CSS strings are not
executable unsafe; any executable match blocks. No repository Cargo/rustc/
rustdoc/`surgeist_css` process may remain. A fresh holistic reviewer then
reviews exact range
`129de7267726277b73d2cc15f1168c44c34ffcbc..HEAD`.

After holistic `CLEAN`, follow the installed `surgeist-agent` canonical
publication gate for cleanup, immutable landing, publication, and readback;
this plan does not restate or modify that workflow authority. The user-required
cycle cleanup includes `cargo clean --offline`, proof that `target` is absent,
and a clean/process-free repository state before publication. C09 completes
only after all MQ3 and Conditional 3 rows are Complete, Cascade 4/5 import
ownership is truthful, the exact eight-row correction is verified, every task
and holistic review is clean, cleanup passes, and the published/read-back
candidate is handed off with root-only follow-up. Another fixture change,
public break, unsafe requirement, external acquisition, or root/sibling
mutation is a blocker requiring P01 reconciliation.

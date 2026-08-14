# P01-I02-C11 Counter Styles And Page Rules

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C11` |
| Owning repository | `surgeist-css` |
| Status | `in_progress` |
| Cycle base | `104ab4d9c2166dfe5d9179500e50da9a84026bbd` |
| Published prerequisite | C10 `104ab4d9c2166dfe5d9179500e50da9a84026bbd`, fetched and read back |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `87f6a94b893ffa416c6ff451575f0d5a21b4aa136e7bcd391cd6c0ce8810a2ae`, P01.14 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `3f93c7f6c3656ebe0b33b8bf9c32e458f306f294d1a969a86df375a5858b1710`, sections 3.1, 3.7, 4.2-4.4, 5, 10-12 |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, semantic SHA-256 `56bcf0340320339454e4ae1aa0b45a7ad2e37e03930e9e3bf7665f8ce4cbb15a`, entry `I02-C11` |
| Bounded outcome | Retain valid Counter Styles 3 definitions and descriptors, retain CSS2 `@page` rules and page pseudo selectors, preserve descriptor/declaration recovery and exact placement, and apply only the reviewed two-row oracle correction. |

## 2. Boundary And Current Evidence

The immutable sources are Counter Styles 3 at the dated O-COUNTERSTYLES3
revision and CSS2 at the dated O-CSS2 revision named by the reviewed ledger.
C11 owns only authored syntax, parser recovery, public AST/accessors, and
truthful metadata. It excludes counter evaluation, generated marker rendering,
pagination, page cascade, margin boxes, selector matching, specificity,
resource loading, serialization, CSSOM, and root adapters.

At the cycle base, `CssRule` has no counter-style or page variants; both rules
are recognized unsupported. Counter value primitives already exist in the
generated-content parser and must be reused or extended without changing their
I01 projections. The selector parser already handles ordinary pseudo-classes;
`:left`, `:right`, and `:first` are not yet owned by a retained `@page`
selector model. Ordinary declaration parsing and descriptor occurrence models
provide recovery and authored-order precedent. Unknown at-rules remain distinct
from these newly supported rules.

All changes are additive. New public fields remain private; constructors are
checked or parser-owned; evolving enums are non-exhaustive. Existing counter
value and declaration projections remain source-compatible. No dependency,
feature, manifest, build logic, generated leaf artifact, API artifact, MSRV,
root, or unsafe change is authorized. T5 updates README, rustdoc, and doctests;
the handoff is SHA-free except for the two product fixture digests.

### 2.1 Counter-Style Contract

Add `CssRule::CounterStyle(CssCounterStyleRule)` with checked
`CssCounterStyleName`, ordered descriptor occurrences, effective-last accessors,
and parser position. Support `system`, `negative`, `prefix`, `suffix`, `range`,
`pad`, `fallback`, `symbols`, `additive-symbols`, and `speak-as` with exact
Counter Styles 3 domains, including `extends`, infinite/comma-separated
ranges, and strictly descending additive weights. Reserved names are rejected;
`extends` inherits effective descriptors and forbids `symbols` and
`additive-symbols`.

The rule is top-level and block-form only. Valid duplicate descriptors are
retained in authored occurrence order and effective-last accessors select the
last valid occurrence. Invalid descriptor values/names use
`InvalidDescriptorValue` or `UnknownDescriptor` with `DropDescriptor`; only an
invalid effective combination uses `InvalidDescriptorCombination` with
`DropAtRule`. A malformed prelude, missing block, nested placement, EOF,
non-BMP source, and 255/256/257 nested component boundary reports the smallest
established recovery unit while retaining valid siblings. A valid rule advances
the top-level body phase; invalid or ignored input does not.

### 2.2 Page Contract

Add `CssRule::Page(CssPageRule)` with optional checked page selector
`CssPageSelector::{Left,Right,First}` (and the authored default form), ordered
ordinary declarations, private fields, accessors, and parser position. `@page`
is top-level and block-form only; its body accepts only `margin` and
`margin-top/right/bottom/left` with CSS2 length, percentage, `auto`, and
negative-value domains, excluding `em` and `ex`. Known non-margin declarations
use `InvalidPropertyValue`/`DropDeclaration`; unknown declarations use
`UnknownProperty`/`DropDeclaration`; invalid margin values retain later
siblings with the existing typed property diagnostic. Margin-box nested at-rules
remain unsupported and are not promoted by C11. Page selector placement,
malformed names, duplicate selectors, unknown page pseudos, block/EOF closure,
siblings, coordinates, validators, and strict parity are covered publicly.

## 3. Exact Oracle Correction

The fixture before C11 has SHA-256
`96be045dc181fe5fc258e76b09458b441139504a3cae13c41897995ab3ae8f5d`.
Only these stable IDs and authored inputs change:

1. `catalog.non-property.later.rule.counter-style.boundary` becomes clean,
   retains `rule:later.rule.counter-style`, and removes only the obsolete
   unsupported diagnostic.
2. `catalog.non-property.later.rule.page.boundary` becomes clean, retains
   `rule:later.rule.page`, and removes only the obsolete unsupported diagnostic.

All entry-point, feature-mode, authored-input, values, authored-declarations,
and unrelated fixture fields remain byte-identical. The hand-authored
two-row replacement has SHA-256
`7c2cf7d79368d76d94cc0b383be70cc404d4c69d7caa72eedba6f0762e0b2356`.
No Rust test asserts a digest, derives an expected row from production, masks a
corrected row, or compares source/test/catalog owner sets or counts.

## 4. Tasks

At assignment start each worker records `task_base_sha="$(git rev-parse HEAD)"`.
Each task uses two commits: a base-compilable public behavioral or named-
metadata RED, then its implementation/docs commit. No production symbol or
behavior precedes the RED. Tests parse authored CSS through public front doors
or directly inspect named public metadata; they never inspect Rust source,
files, tokens, ASTs, symbols, registrations, call sites, owner sets/counts,
workflow state, test existence/count/placement, or incidental call counts.

After each focused loop every task runs this exact matrix:

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

Each exact task range receives a fresh independent task review before the
dependent task starts.

### T1 Counter-Style Rule And Core Descriptors

- **Area:** counter-style rule/name models, parser dispatch and block recovery,
  core `system`/`symbols`/`prefix`/`suffix` descriptors, exact fixture row 1,
  and only the named `later.rule.counter-style` metadata/vector expectation.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test counter_style_grammars counter_style_rules_retain_valid_core_definitions -- --exact`; valid authored CSS is still dropped as `UnsupportedAtRule` on the C10 base.
- **Acceptance:** typed rule and occurrence accessors, valid cyclic/numeric/alphabetic
  systems, symbol lists, required descriptor relationships, body placement,
  source positions, exact recovery, and truthful Complete metadata for the rule.
- **Commits:** `test: specify counter-style rules`; `feat: add counter-style rules`.

### T2 Counter-Style Descriptor Domains And Recovery

- **Dependency:** T1 independently CLEAN.
- **Area:** remaining Counter Styles 3 descriptors/value models and parser
  grammar; `counter_style_grammars`, `structured_errors`, `source_coordinates`,
  `specialized_recovery_boundaries`, `app_strict_parity`, and public surface.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test counter_style_grammars counter_style_descriptors_enforce_domains_order_and_recovery -- --exact`; valid `negative`, `range`, `pad`, `fallback`, `additive-symbols`, and `speak-as` forms still recover or drop.
- **Acceptance:** exact domain/conflict matrices, valid ordered duplicates with
  effective-last behavior, rule-level recovery only for invalid effective
  combinations, extends inheritance/prohibition, infinite/comma ranges,
  strictly descending additive weights, symbols/additive-symbols, malformed
  values, EOF/non-BMP, depth 255/256/257, sibling retention, coordinates,
  validator parity, and all ten
  descriptor/value rows remain truthful until T5 metadata closure.
- **Commits:** `test: specify counter-style descriptor domains`; `feat: complete counter-style descriptors`.

### T3 Page Rules And Page Selectors

- **Dependency:** T1 independently CLEAN; T2 descriptor primitives available.
- **Area:** page rule/selector models and parser dispatch in syntax/parser,
  exact fixture row 2, selector tests, declaration recovery, and named
  `later.rule.page`/`official.selector.page-pseudo` metadata/vector expectations.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test page_rule_grammars page_rules_and_pseudos_retain_valid_authored_structure -- --exact`; valid `@page`, `@page :left`, `:right`, and `:first` are still unsupported/dropped.
- **Acceptance:** default/left/right/first selectors, only page-context margin
  declarations and CSS2 length/percentage/auto/negative domains, explicit
  `em`/`ex` rejection, distinct known-non-margin versus unknown declaration
  diagnostics/actions, importance, top-level-after-valid-prelude placement
  (including valid page after imports and rejection of later imports), later
  import rejection, nested margin-box rejection, duplicates,
  malformed/EOF/non-BMP/repeated recovery, public accessors, strict parity, and
  the clean page fixture row.
- **Commits:** `test: specify page rules and selectors`; `feat: add page rules and selectors`.

### T4 Shared Recovery, Placement, And Boundary Matrix

- **Dependency:** T2 and T3 independently CLEAN.
- **Area:** narrowly required shared rule-context/recovery seams; authored CSS
  recovery tests across counter-style/page/conditional/nested contexts, exact
  coordinates and 255/256/257 boundaries, and the final two-row fixture diff.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test counter_style_grammars c11_rule_recovery_preserves_siblings_and_boundaries -- --exact`; existing shared recovery loses a valid sibling or misclassifies placement.
- **Acceptance:** one diagnostic per smallest invalid unit, source order,
  responsible offsets/spans, child/parent retention, top-level-only rules,
  unknown-versus-recognized-unsupported distinction, ordinary/app-strict
  parity, and exact final fixture SHA.
- **Commits:** `test: specify counter-style and page recovery`; `feat: harden C11 rule recovery`.

### T5 Official Metadata, Docs, And Handoff

- **Dependency:** T1-T4 independently CLEAN.
- **Area:** remaining Counter Styles 3 official metadata/implementation
  inventories, CSS2 page selector metadata, direct named metadata tests,
  README/rustdoc/doctests, and SHA-free C11 handoff.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog counter_styles_and_page_metadata_are_truthful -- --exact`; paired behavior passes while named official rows remain Reserved.
- **Acceptance:** all 16 O-COUNTERSTYLES3 rows and both O-CSS2 rows are Complete
  with exact source/fragments, no unsupported-code lies, aggregate aliases
  remain truthful, docs cover public models/recovery/exclusions, and handoff
  records only the two product fixture digests. Run focused matrices, full
  tests/doctests, warning-denied rustdoc, both Clippy modes, fmt, and diff gates.
- **Commits:** `test: specify counter-style and page metadata`; `docs: publish C11 counter-style and page closure`.

## 5. Completion, Publication, And Blockers

After all five task ranges are independently CLEAN, set this plan `complete`
in a separate status-only commit. Run:

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
git diff --check 104ab4d9c2166dfe5d9179500e50da9a84026bbd..HEAD
shasum -a 256 tests/fixtures/i01-c01-observables.tsv
git diff --unified=0 104ab4d9c2166dfe5d9179500e50da9a84026bbd..HEAD -- tests/fixtures/i01-c01-observables.tsv
git status --short --branch
ps -axo pid=,command=
```

Also run the canonical owned-Rust executable-unsafe scan, then
`cargo clean --offline`, and prove `target` absent, the worktree clean, and no
`surgeist-css` Cargo/Rust process. After holistic review, use the canonical
lease publication/readback gate. The final fixture must be
`7c2cf7d79368d76d94cc0b383be70cc404d4c69d7caa72eedba6f0762e0b2356`, and the
direct fixture diff must contain only the two rows in section 3. A new frozen
oracle contradiction, unsafe requirement, external acquisition, root/sibling
mutation, margin-box promotion, or unresolved source ownership returns to P01
reconciliation.

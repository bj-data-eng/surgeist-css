# P01-I02-C10 Namespaces And Complete Selectors 3

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C10` |
| Owning repository | `surgeist-css` |
| Status | `reviewed` |
| Cycle base | `019906900cab8295d8c33a28eb53a76b39cd85ee` |
| Published prerequisite | C09 `019906900cab8295d8c33a28eb53a76b39cd85ee`, fetched and read back |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `db899aea31b168128b4d8bd5c4be58057a9860e0de4d0d4b00f049955b16eb22`, P01.13 |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `0d26ae87704ecc16c09a59f3684a3af15edfdc5082259ca2bec0135377d97f62`, sections 3.1, 3.6, 4.2-4.4, 5, 6, 10, 11 findings 2.1/2.3/2.8, and 12 |
| Reviewed ledger | `plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256 `626d176a734d48c3a6202c189daeadc5ff93253c20ac6681d91f93b01ab11b0d`; 20 `O-SELECTORS3` and 2 `O-NAMESPACES3` rows |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `29dc39ea41961e5418bfaf380080689d18100769bf0e808a5f77d37f04a1bd6f`, entry `I02-C10` |
| Bounded outcome | Retain Namespaces 3 rules, make every selector consumer namespace-aware, complete the dated Selectors 3 authored grammar and public AST, preserve selected later selector extensions truthfully, and apply only the reviewed six-row oracle correction. |

## 2. Boundary And Current Evidence

The immutable normative revisions are Selectors 3 Recommendation
<https://www.w3.org/TR/2018/REC-selectors-3-20181106/> and Namespaces 3
Recommendation
<https://www.w3.org/TR/2014/REC-css-namespaces-3-20140320/>. The ledger fixes
their exact 20+2 official rows. No moving source, Selectors 4 completion claim,
selector matching, specificity application, cascade, URI normalization or
loading, namespace resolution, serialization, generic fragment entry point, or
CSSOM behavior enters C10.

At the cycle base, `CssRule` has no namespace variant; `@namespace` is
recognized unsupported; the C09 top-level machine contains a dormant namespace
hook but cannot distinguish a prior initial layer; selector parsing rejects `|`
namespace syntax and universal `*`; `CssCompoundSelector` overwrites repeated
IDs; and required `:link`, `:visited`, `:target`, `:lang()`, `::first-line`,
`::first-letter`, and legacy single-colon pseudo-elements are absent. Existing
attribute matchers, four combinators, most Selectors 3 pseudo-classes, selected
Selectors 4 syntax, nesting, scope, forgiving `:is()`/`:where()`, and
unforgiving consumers already work and must not regress.

All changes are additive. Existing `CssSelector::Tag`, `CssSelector::Key`,
`CssCompoundSelector::tag()`, and `key()` remain source-compatible I01
projections. New public fields stay private; constructors are checked or
parser-owned; evolving enums remain non-exhaustive. `key()` retains the last-ID
projection while `ids()` exposes every authored ID. No dependency, feature,
manifest, build logic, generated leaf artifact, or leaf MSRV changes. Root
alone owns facade/adapters, API generation/artifacts, integration tests, and the
gitlink after leaf publication. All owned Rust remains free of `unsafe`.
T5 updates README, crate rustdoc, and doctests for the new public syntax and
recovery contract; no standalone example target changes.

### 2.1 Namespace Rule And Phase Contract

Add `CssRule::Namespace(CssNamespaceRule)`, checked
`CssNamespacePrefix`, literal `CssNamespaceName`, and parser-produced position.
The prefix is one decoded CSS identifier and is case-sensitive. The namespace
name preserves a string or `url()` token literally; empty strings and strings
that are not valid URIs are valid. URI normalization and loading are excluded.

The refined phases are `Initial`, `InitialLayers`, `Imports`,
`ImportsAfterInitialLayers`, `Namespaces`, and `Body`. Initial layers still
permit imports but permanently prohibit namespaces. Only `Initial` and
`Imports` admit a namespace; success enters `Namespaces`, which admits only
further namespaces before a layer/body transition. A successful body rule or a
layer after import/namespace enters `Body`. Invalid and ignored rules do not
mutate phase or active bindings. Namespace rules are semicolon top-level rules;
block form, nested placement, late placement, malformed prefix/name, extra
tokens, and missing semicolon recover as one dropped at-rule.

Every syntactically valid declaration remains in authored rule order. The last
declaration for an exact case-sensitive named prefix or the default is active
for following selectors. Encoding remains independent. The Cascade 5
`ext.stylesheet.prelude-order` row continues to own the initial-layer/import
delta; O-NAMESPACES3 owns namespace-before-layer ordering.

### 2.2 Namespace-Aware Selector Contract

Add non-exhaustive `CssNamespaceConstraint::{Default, ExplicitNone, Any,
Named(CssNamespacePrefix)}` and private-field `CssQualifiedSelectorName` with
`namespace()`, `local_name()`, and `is_universal()`. Add
`CssCompoundSelector::type_selector()` and `ids()`, and add an attribute
namespace accessor while preserving its local `name()` projection.

The sheet-local active environment reaches top-level and nested style rules,
media/supports/container/layer/scope groups, scope boundaries, nested style
selectors, selector-list pseudo-classes, and `@supports selector()`. For type
and universal selectors, no explicit separator means `Default` when active and
`Any` otherwise; `*|` is `Any`, `|` is `ExplicitNone`, and a declared exact
prefix is `Named`. Unqualified attributes are always `ExplicitNone` and never
inherit the default. An undeclared named prefix invalidates the selector.
Forgiving consumers drop that member with `DropSelectorListItem`; unforgiving
consumers preserve their established smallest-unit recovery. In
`selector()`, balanced content outside the typed subset remains
`GeneralEnclosed` without a recovery diagnostic.

### 2.3 Selectors 3 And Preserved Extension Contract

Add `Link`, `Visited`, `Target`, and checked `Lang(CssLanguageRange)` to
`CssPseudoClass`; add `FirstLine` and `FirstLetter` to `CssPseudoElement`.
Single-colon `before`, `after`, `first-line`, and `first-letter` map to those
same typed pseudo-elements; later pseudo-elements remain double-colon-only.
Accept universal/type selectors, every existing attribute matcher, ordered
repeated IDs/classes, the complete structural/UI/dynamic pseudo set, selector
groups, and all four Selectors 3 combinators. Pseudo-elements remain terminal,
subject to the already catalogued generated-marker extension sequence.

`ext.supports.selector` remains `Partial`. Its typed subset becomes complete
Selectors 3 plus the exact existing I01 attribute-case, extension-state,
extension-functional, and selected pseudo-element rows enumerated in
specification section 5. Its remainder includes `||`, unselected Selectors 4
pseudo-classes/elements, and syntax outside those atomic rows; balanced
remainder content is general-enclosed. Existing `:not()` list breadth,
`:is()`/`:where()` forgiveness, `:has()`, nth-child `of`, nesting, and scope
behavior remain under their existing extension ownership.

## 3. Exact Oracle Correction

The fixture before C10 has SHA-256
`95518fbabb04cd5b96bc9505a4d96681d444042498d681f28b3db4f3d8a2f0d3`.
Only these stable IDs and their expected public observables change:

1. `catalog.non-property.later.rule.namespace.boundary` becomes clean, retains
   `rule:later.rule.namespace`, and removes the obsolete unsupported diagnostic;
2. `catalog.non-property.baseline.selector.extension-state.boundary` becomes
   clean and retains the style rule plus red color declaration;
3. `catalog.non-property.baseline.selector.functional.boundary` receives the
   same clean style/color observables;
4. `catalog.non-property.baseline.selector.pseudo-class.boundary` receives the
   same clean style/color observables; and
5. `catalog.non-property.baseline.selector.pseudo-element.boundary` receives
   the same clean style/color observables; and
6. `focused.stylesheet-recovery.11` keeps both surrounding style rules and
   declarations, but classifies the intervening valid late namespace as an
   invalidly placed supported rule rather than an unsupported rule.

The hand-authored replacement fixture has SHA-256
`96be045dc181fe5fc258e76b09458b441139504a3cae13c41897995ab3ae8f5d`.
The undeclared-prefix `catalog.non-property.baseline.selector.complex.boundary`
row and every other byte remain unchanged. Row 6 preserves its non-clean flag,
retained/value/authored fields, `DropAtRule`, position, and span; only the
diagnostic changes to `InvalidAtRulePlacement:namespace:after imports and before
every layer or body rule`. T1 authors rows 1 and 6 before namespace production,
yielding interim fixture SHA-256
`174b6cc8db6181c42176c96a214e2f8cc210247f6c430c5191a0347bb5f31b72`;
T4 authors rows 2-5 before selector production. No Rust test asserts
a digest, derives an expected row from production, masks a row, or compares
source/test/catalog owner sets or counts.

## 4. Tasks

At assignment start each worker records `task_base_sha="$(git rev-parse HEAD)"`.
Each task uses two commits except reconciled T1, which uses its existing row-1
RED, a second test-only row-6 recovery RED, and one production commit. A task's
test commit adds a base-compilable public behavioral RED and runs its exact
command to the intended failure; no new production symbol or behavior precedes
it. New-symbol API assertions follow that executable RED.
Every test parses authored CSS through the public front door or directly
inspects named public metadata. No test parses or inspects Rust source, files,
tokens, ASTs, symbols, registrations, call sites, owner sets/counts, workflow
state, test existence/count/placement, or incidental invocation counts.

After its focused loop every task runs:

```sh
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
cargo fmt --check
git diff --check "${task_base_sha}..HEAD"
```

Each exact task range receives a fresh independent
`surgeist-task-reviewer` and must be `CLEAN` before the dependent task starts.

### T1 Retain Namespace Rules And Enforce Prelude Ordering

- **Dependency:** this plan independently clean.
- **Area:** namespace models in `src/syntax.rs`; at-rule parsing, active binding
  state, and refined phases in `src/parser/mod.rs`; new
  `tests/selector_namespace_grammars.rs`; exact fixture rows 1 and 6; focused updates
  to `conditional_ordering`, `stylesheet_recovery`, `structured_errors`,
  `source_coordinates`, `app_strict_parity`, `public_surface`, and
  `i01_c01_observables`.
- **RED 1 (already recorded):** commit
  `6c0d4c01e1bff041af1f1e33831d46e7a26428d1` authors row 1 and a
  base-compilable public behavior proving a valid namespace remains recognized
  unsupported. Its exact failing command is
  `cargo test -p surgeist-css --offline --no-default-features --test selector_namespace_grammars namespace_rules_obey_namespaces3_prelude_ordering -- --exact`.
- **RED 2:** before any production commit, author fixture row 6 and an
  existing-public-API assertion that the late namespace reports
  `InvalidAtRulePlacement`, not `UnsupportedAtRule`. Run
  `cargo test -p surgeist-css --offline --no-default-features --test selector_namespace_grammars late_namespace_is_a_placement_error_not_an_unsupported_rule -- --exact`
  to its intended base failure. The combined test-only fixture has exact
  SHA-256 `174b6cc8db6181c42176c96a214e2f8cc210247f6c430c5191a0347bb5f31b72`.
- **Acceptance:** string/url and empty/invalid-URI literals; decoded checked
  prefixes; default/named and duplicate authored order; six-state table,
  encoding/import/layer/body interactions; success-only transition and binding;
  block/nested/late/malformed recovery; exact action/payload/span/position,
  sibling retention, EOF/non-BMP, validators, strict parity; public variant and
  private-field accessor evidence.
- **Focused:** both modes for `selector_namespace_grammars conditional_ordering stylesheet_recovery structured_errors source_coordinates app_strict_parity public_surface i01_c01_observables`.
- **Commits:** existing `test: specify namespace rule ordering`; then
  `test: specify namespace placement recovery`; then
  `feat: add namespace rule ordering`.

### T2 Add Namespace-Qualified Type, Universal, And Attribute Selectors

- **Dependency:** T1 independently clean.
- **Area:** current qualified-name/constraint and ordered-ID-ready models in
  `src/syntax.rs`; top-level namespace-aware parsing in
  `src/parser/selectors.rs` and `src/parser/mod.rs`;
  `selector_namespace_grammars`, `public_surface`, `structured_errors`,
  `source_coordinates`, `specialized_list_recovery`, and directly stale
  selector assertions only.
- **RED:** public authored rules containing declared `svg|a`, `svg|*`, `*|a`,
  `|a`, qualified attributes, and default-qualified unprefixed types still drop
  on the T1 base. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test selector_namespace_grammars namespace_qualified_type_universal_and_attribute_selectors_use_active_bindings -- --exact`.
- **Acceptance:** all four constraints and local ident/universal accessors;
  compatibility projections; default never applies to attributes; case-sensitive
  active prefixes, escapes, redeclarations, explicit-none/any/default; universal
  omission boundaries; undeclared prefix and token/whitespace mutations; exact
  diagnostics/recovery/positions, groups and siblings, strict parity.
- **Focused:** both modes for `selector_namespace_grammars public_surface structured_errors source_coordinates specialized_list_recovery app_strict_parity i01_c01_observables`.
- **Commits:** `test: specify namespace-qualified selectors`; `feat: add namespace-qualified selectors`.

### T3 Propagate Namespace Bindings Through Every Selector Consumer

- **Dependency:** T2 independently clean.
- **Area:** selector context plumbing across `src/parser/mod.rs`,
  `src/parser/selectors.rs`, `src/parser/supports.rs`, and narrowly required
  nesting/scoped coordinators; `selector_namespace_grammars`, `supports_grammars`,
  `nested_structural_recovery`, `structural_recovery_adversarial`,
  `specialized_list_recovery`, `specialized_recovery_boundaries`,
  `block_item_recovery`, `source_coordinates`, and `app_strict_parity`.
- **RED:** a declared prefix works in a top-level style rule after T2 but not yet
  across nested conditional/style/scope and typed `selector()` consumers. Exact
  command:
  `cargo test -p surgeist-css --offline --no-default-features --test selector_namespace_grammars namespace_bindings_reach_every_selector_consumer_without_changing_recovery -- --exact`.
- **Acceptance:** one sheet-local environment reaches every section 2.2
  consumer; typed `selector()` with active names; undeclared/balanced content
  becomes general-enclosed there; forgiving member-local versus unforgiving
  whole-unit recovery elsewhere; namespace-aware nesting composition and scope
  constraints; repeated failures, child/parent order, EOF/non-BMP, 255/256/257
  existing recursive boundary, validators, strict parity.
- **Focused:** both modes for `selector_namespace_grammars supports_grammars nested_structural_recovery structural_recovery_adversarial specialized_list_recovery specialized_recovery_boundaries block_item_recovery source_coordinates app_strict_parity public_surface i01_c01_observables`.
- **Commits:** `test: specify namespace selector contexts`; `feat: propagate namespace selector contexts`.

### T4 Complete Selectors 3 Pseudos, Legacy Forms, And Repeated IDs

- **Dependency:** T3 independently clean.
- **Area:** pseudo/language/ordered-ID models in `src/syntax.rs`; exact grammar in
  `src/parser/selectors.rs`; exact fixture rows 2-5;
  `selector_namespace_grammars`, `public_surface`, `structured_errors`,
  `source_coordinates`, `specialized_list_recovery`,
  `nested_structural_recovery`, and directly stale selector assertions only.
- **RED:** author fixture rows 2-5 and public behavior proving required
  Selectors 3 spellings still drop on the T3 base. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test selector_namespace_grammars selectors3_pseudos_legacy_forms_and_repeated_ids_are_typed -- --exact`.
- **Acceptance:** complete official pseudo-class/element matrix; checked
  `:lang()` identifier and escapes; four legacy single-colon spellings and
  rejection for later pseudo-elements; terminal/sequence rules; ordered
  repeated IDs with last-ID compatibility; combinator/group/attribute
  preservation; exact invalid-token/arity/placement diagnostics, siblings,
  repeated failures, EOF/non-BMP, strict parity; final exact six-row fixture
  diff and digest.
- **Focused:** both modes for `selector_namespace_grammars public_surface structured_errors source_coordinates specialized_list_recovery nested_structural_recovery app_strict_parity conformance_catalog i01_c01_observables`.
- **Commits:** `test: specify complete Selectors 3 syntax`; `feat: complete Selectors 3 syntax`.

### T5 Publish Selector And Namespace Metadata, Docs, And Handoff

- **Dependency:** T1-T4 independently clean.
- **Area:** `src/conformance.rs`; selector/rule implementation inventories only;
  direct named metadata tests; README and crate rustdoc/doctests; SHA-free
  `plans/handoffs/P01-I02-C10-namespaces-and-complete-selectors-3.md`.
- **RED:** paired grammar behavior passes while direct named official metadata
  remains Reserved/RecognizedUnsupported. Exact command:
  `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog selectors3_and_namespace_metadata_are_truthful -- --exact`.
- **Acceptance:** exactly 20 O-SELECTORS3 and 2 O-NAMESPACES3 rows Complete with
  exact source/fragments; aggregate aliases remain truthful; namespace no longer
  has unsupported code; `ext.supports.selector` remains Partial with section
  2.3's exact subset/remainder and typed/general-enclosed public evidence; no
  set/count proxy; docs cover public models, phase refinement, constraints,
  recovery, selected extensions, and downstream exclusions; handoff records
  only the two product fixture digests among otherwise SHA-free product facts;
  doctests and warning-denied rustdoc pass.
- **Focused:** both modes for `conformance_catalog catalog_inventory selector_namespace_grammars supports_grammars conditional_ordering public_surface structured_errors source_coordinates specialized_list_recovery app_strict_parity i01_c01_observables`; then:

  ```sh
  cargo test -p surgeist-css --offline --no-default-features --doc
  cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
  RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features
  RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-deps --no-default-features --features app-strict
  ```
- **Commits:** `test: specify selector and namespace metadata`; `docs: publish selector and namespace closure`.

## 5. Completion, Publication, And Blockers

After all five task ranges are independently `CLEAN`, make the separate
status-only `complete` commit and run:

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
git diff --check 019906900cab8295d8c33a28eb53a76b39cd85ee..HEAD
shasum -a 256 tests/fixtures/i01-c01-observables.tsv
git diff --unified=0 019906900cab8295d8c33a28eb53a76b39cd85ee..HEAD -- tests/fixtures/i01-c01-observables.tsv
git status --short --branch
ps -axo pid=,command=
```

The fixture output must be
`96be045dc181fe5fc258e76b09458b441139504a3cae13c41897995ab3ae8f5d`;
the direct diff must show only section 3's six rows. Build the canonical owned
Rust manifest from tracked and nonignored untracked `*.rs`, run the exact
`surgeist-agent` executable-unsafe regex over it, verify
`#![forbid(unsafe_code)]`, and run both Clippy unsafe-denial matrices. Classify
textual authored CSS `unsafe` keywords as non-executable; any executable match
blocks.

A fresh `surgeist-holistic-reviewer` reviews the exact cycle-base-to-head range.
After `CLEAN`, rerun the full gate at the reviewed head, run the user-required
`cargo clean --offline`, prove `target` absent and the repository clean and
process-free, then follow the canonical lease publication/readback gate. C10
completes only when the 20+2 official rows are Complete, the Partial selector
extension is exact, the six-row correction is verified, all reviews and gates
are clean, and the published candidate handoff names root-only follow-up.

Another fixture change, a second breaking API change, unsafe requirement,
external acquisition, root/sibling mutation, Selectors 4 completion claim, or
missing active-prefix/ordering decision is a blocker requiring P01
reconciliation.

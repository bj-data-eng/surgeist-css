# P01-I01-C02 Coupled Declaration Foundation

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I01-C02` |
| Owning repository | `surgeist-css` |
| Status | `reviewed` |
| Cycle base | `5ac518b44e2a72a52cc40b938fb5c77b9429fb07` |
| Reviewed specification | `plans/specs/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `76b76a50a613aea26e1b790749a780f7d05efdfe57711c6b8dbf9a9fca2359d7`, sections 5, 8.1 through 8.4, 9 property inventory, 10, 12.3, and 13 findings 2.6, 2.19, and 2.20 |
| Reviewed sequence | `plans/sequences/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `f3a65df04c5c5a4f6f02212fe4d69959b75bba1cdcf2fd12e5bfb012f2c4ec94`, entry `I01-C02 Coupled Declaration Foundation` |
| Bounded outcome | One crate-private schema owns all 179 baseline properties, their typed values, and dispatch; declarations couple identity to value, preserve authored custom and substitution-dependent values, carry importance and semantic source positions, and expose no mismatched or unchecked construction path. |

## 2. Boundary

The published C01 base provides final semantic positions, structured errors, and
recovery report value types while keeping `parse_sheet` strict. Its declaration
surface still stores an independently constructible internal `CssProperty` and
`CssValue` pair, repeats the property inventory across `src/syntax.rs`,
`src/validation.rs`, and the parser dispatch in `src/parser/mod.rs`, approximates
custom-property names with character checks, and has no declaration importance
or context-specific declaration-list types.

This cycle owns the final I01 property schema and authored declaration model and
migrates the existing strict stylesheet, keyframe, and descriptor parsing paths
to that model. It does not change the ordinary `parse_sheet` signature, perform
browser recovery, add the style-attribute front door or `app-strict` feature,
add the independent conformance catalog or its public metadata queries, extend
the frozen I01 grammar, change C01 diagnostic meanings, edit root/siblings, or
add a dependency. Section 9 is in scope only for the 179-property schema and
crate-private implementation inventory; the independent catalog remains C05.

The base is published and read back with local `main`, `origin/main`, and remote
`main` all equal to the recorded cycle base. `target` is absent and no process is
executing from this repository's target tree at intake. The two pinned safe
dependencies are already present in `Cargo.lock`; every Cargo command that can
resolve, build, test, document, or lint dependencies uses offline mode. A cache
miss is a tooling blocker, not permission to acquire software.

## 3. Impacts

| Area | C02 classification |
| --- | --- |
| Public API | Breaking/additive: replace `CssProperty`/`CssValue` pairing with property-coupled declarations, declared-value phases, importance, semantic property-name views, and declaration-list wrappers. `parse_sheet` remains strict. |
| Dependencies/features | Unchanged; no dependency or feature is added. |
| Generated artifacts | None; the property schema expands in source at compile time, and root-owned API artifacts remain untouched. |
| Docs/examples | Rustdoc and public-consumer/compile-fail evidence cover every new or materially changed declaration item; README closure remains C05. |
| MSRV | No leaf `rust-version` is introduced; edition remains 2024. |
| Root follow-up | None for this incomplete I01 candidate; the final breaking facade/API migration is handed off after C05. |
| Unsafe | C01's crate-root prohibition remains; no owned target may contain or enable unsafe. |

## 4. Tasks

### T1 Single Property Schema And Generated Identity

- **Files/area:** new `src/properties.rs`, property-name classification in
  `src/validation.rs`, property dispatch ownership in `src/parser/mod.rs`,
  crate-root/module wiring, focused unit tests, and
  `tests/property_schema.rs`.
- **Outcome:** one crate-private declarative schema has exactly one entry for
  each of the 179 frozen canonical names. Each entry owns canonical spelling,
  aliases, stable `baseline.property.<canonical-name>` identity, the exact
  property-specific authored value type, and parser function. It generates
  `CssKnownProperty`, ASCII-case-insensitive canonical/alias lookup, generated
  property-specific dispatch, and one crate-private implementation inventory.
  No hand-maintained supported-property table or parallel property dispatch
  remains at cycle exit.
- **RED evidence:** independent tests first fail because no schema, generated
  identity, stable property IDs, or implementation inventory exists. The
  independent frozen-name table asserts exactly 179 unique names and exact set
  equality; focused vectors cover canonical and mixed-case lookup, aliases when
  present, unknown/custom exclusion, unique IDs, and one dispatch path per row.
- **Acceptance:** the schema is the only property identity/dispatch authority;
  every frozen row maps bidirectionally among authored name, known identity,
  stable ID, value type, parser, and implementation inventory; no parser branch
  or supported name exists outside it. The schema is not used as the independent
  conformance catalog and exposes no raw dependency token or public constructor.
- **Commands:** `cargo test -p surgeist-css --offline property_schema_`;
  `cargo test -p surgeist-css --offline --test property_schema`;
  `cargo check -p surgeist-css --offline`.
- **Dependencies:** C01 only.
- **Intended commit:** `feat: generate CSS property identity from one schema`.

### T2 Property-Coupled Known Declarations

- **Files/area:** schema-generated declaration items in `src/properties.rs`,
  declaration/value shapes in `src/syntax.rs`, corresponding error contexts in
  `src/error.rs`, strict property parsing in `src/parser/mod.rs`, focused unit
  tests, rustdoc, and `tests/coupled_declarations.rs`.
- **Outcome:** `CssDeclaration`, non-exhaustive `CssDeclarationBody`, generated
  non-exhaustive `CssKnownDeclaration`, generic non-exhaustive
  `CssDeclaredValue<T>`, dedicated `all` declared-value handling,
  `CssPropertyNameRef`, and their private-field accessors implement section 8.2.
  Every generated known-declaration variant owns
  `CssDeclaredValue<PropertySpecificType>` and derives its property from the
  active variant. The independent `CssProperty`/`CssValue` pair and every
  property/value cross-product construction route are removed.
- **RED evidence:** public and focused tests first fail because declarations do
  not expose `body()`, `known()`, `custom()`, `property_name()`, or typed
  declared-value views and because the old broad pair permits mismatched states.
  Named vectors parse adjacent properties whose value types differ, global
  keywords, and `all`; compile-fail doctests attempt cross-property pairing and
  direct parser-node construction.
- **Acceptance:** every known declaration carries only its schema-selected
  value type; `property()` and property-name views are derived, never stored in
  parallel; `all` cannot carry an ordinary typed value; fields and parser-owned
  construction remain private; non-exhaustive matching is demonstrated without
  string or `Debug` control flow. Existing strict valid/error behavior remains
  equivalent except for the intentional public model migration.
- **Commands:** `cargo test -p surgeist-css --offline coupled_`;
  `cargo test -p surgeist-css --offline --test coupled_declarations`;
  `cargo test -p surgeist-css --offline --doc`;
  `cargo check -p surgeist-css --offline`.
- **Dependencies:** T1.
- **Intended commit:** `feat: couple CSS properties to declared values`.

### T3 Authored Custom And Substitution-Dependent Values

- **Files/area:** authored declaration values and custom-name types in
  `src/syntax.rs`/`src/properties.rs`, shared token consumption in
  `src/parser/variables.rs` and `src/parser/mod.rs`, structured error contexts,
  focused unit tests, rustdoc, and `tests/authored_declaration_values.rs`.
- **Outcome:** non-exhaustive `CssCustomDeclaration` and
  `CssCustomPropertyDeclaredValue` distinguish exact authored token streams from
  whole-value CSS-wide keywords; `CssSubstitutionDependentValue` preserves a
  syntactically admissible known-property value whose grammar is deferred by a
  substitution function. Custom-property names use CSS identifier tokenization
  after `--`, and the retained UTF-8 slices implement section 8.4 without
  substitution, resolution, or dependency-graph promises.
- **RED evidence:** focused/public tests first fail because empty custom values,
  CSS-valid non-ASCII/escaped names, exact authored trivia, whole-value global
  branches, and generic substitution-dependent views are absent or represented
  by the old broad value enum. Vectors cover empty and whitespace-only custom
  values; interior whitespace/comments/escapes/case/commas/balanced functions;
  malformed names/tokens/balancing; `var()` and other specified substitution
  functions with fallbacks; and typed known values on either side.
- **Acceptance:** a custom name cannot attach to a known value; custom value
  emptiness is representable; only boundary trivia and a valid terminal
  importance annotation may be removed; known substitution-dependent values
  retain the complete authored value through `as_css()`; invalid lexical or
  structural input stays an exact C01 structured error. No authored value claims
  computed validity, substitution, cascade, matching, resolution, or loading.
- **Commands:** `cargo test -p surgeist-css --offline authored_declaration_`;
  `cargo test -p surgeist-css --offline --test authored_declaration_values`;
  `cargo test -p surgeist-css --offline --doc`;
  `cargo check -p surgeist-css --offline`.
- **Dependencies:** T1 and T2.
- **Intended commit:** `feat: preserve authored CSS declaration values`.

### T4 Importance, Contexts, And Declaration Collections

- **Files/area:** declaration/list models in `src/syntax.rs` and
  `src/properties.rs`, the one private declaration boundary across
  `src/parser/mod.rs`, `src/parser/keyframes.rs`, and descriptor parsing,
  structured annotation errors in `src/error.rs`, focused unit tests, rustdoc,
  and `tests/declaration_importance.rs`.
- **Outcome:** closed `CssImportance::{Normal, Important}` belongs to ordinary
  `CssDeclaration`; private-field `CssDeclarationList` and distinct
  `CssKeyframeDeclarationList`/`CssKeyframeDeclaration` enforce ordinary versus
  keyframe states and expose the required ordered-collection accessors. One
  private declaration boundary receives an exact ordinary, keyframe, or
  descriptor context, recognizes exactly one valid terminal `!important`, and
  preserves semantic source positions. No keyframe declaration or descriptor
  exposes an importance field. The existing `CssFontFaceDescriptors` semantic
  field aggregate remains the C02 retained descriptor model, but each required
  or optional slot stores a private-field `CssDescriptorOccurrence<T>` whose
  public `value()` and `position()` accessors expose the typed descriptor value
  and descriptor-name start. Aggregate accessors return those occurrences, and
  aggregate/occurrence construction is crate-private so positions cannot be
  forged. C02 adds no ordered public descriptor list: C03 owns descriptor-unit
  recovery and any collection migration needed for recovered authored order.
- **RED evidence:** focused/public tests first fail on absent importance/list
  APIs and current acceptance of unmodeled annotations. The matrix covers normal
  and ASCII-case-insensitive important values; whitespace/comments around `!`
  and `important`; optional final semicolon; bare/misspelled/duplicate/nonterminal
  annotations; ordinary, custom, keyframe, and descriptor contexts; a bad
  declaration between valid siblings; and exact position/error context. Every
  `@font-face` descriptor rejects terminal or malformed `!important` with
  `InvalidDeclarationAnnotation`; its `CssDeclarationContextRef::Descriptor`
  carries decoded `font-face` and the canonical descriptor name, the error
  position is the first `!`, and the encountered summary is the authored `!`
  delimiter. A retained occurrence's position is the descriptor-name start.
- **Acceptance:** importance is removed only after terminal syntactic
  recognition and belongs to the declaration, not its value; malformed
  annotations produce `InvalidDeclarationAnnotation` at `!`; keyframes and all
  descriptors reject even a syntactically terminal annotation through their
  exact structured contexts; all retained declarations and descriptor
  occurrences have semantic positions and the correct retained model. Every
  list provides `as_slice()`, `iter()`, `len()`, and `is_empty()`; no mutable or
  unchecked construction path exists. All 179 schema rows and strict
  stylesheet/property parsers use the final coupled model, and public rustdoc
  names phase, invariant, and relevant non-responsibilities.
- **Commands:** `cargo test -p surgeist-css --offline declaration_importance_`;
  `cargo test -p surgeist-css --offline --test declaration_importance`;
  `cargo test -p surgeist-css --offline --doc`;
  `cargo clippy -p surgeist-css --offline --all-targets -- -F unsafe-code -D warnings`.
- **Dependencies:** T1, T2, and T3.
- **Intended commit:** `feat: model CSS declaration importance and context`.

## 5. Completion

C02 is accepted when all four ordered task ranges are independently reviewed
`CLEAN`; the one schema owns all 179 property identities, value types, and
dispatch paths; every strict stylesheet, keyframe, and descriptor declaration
uses its final C02 context-specific model and shared boundary; the C02-applicable
section 12.3 rows—known,
custom, global, substitution-dependent, all schema entries, cross-property
rejection, importance, ordinary/keyframe context, and invalid declarations
between valid sheet siblings—and public construction evidence are clean;
sheet/style-attribute parity remains explicitly allocated to C04; C01
diagnostics/positions remain compatible; a fresh
holistic cycle review is `CLEAN`; and the cycle is landed and published with
remote readback. The candidate handoff names it as an incomplete I01 declaration
foundation and makes C03 the only next ready cycle.

Final commands, in order:

```sh
cargo check -p surgeist-css --offline
cargo test -p surgeist-css --offline
cargo test -p surgeist-css --offline --doc
cargo clippy -p surgeist-css --offline --all-targets -- -F unsafe-code -D warnings
cargo fmt --check
rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs
! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .
! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'
cargo clean
test ! -d target
! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'
```

The unsafe scan succeeds only with no matches. Before `cargo clean`, any process
executing from this repository's target tree is stale for this completed cycle
and must be terminated safely, then the check rerun; no unrelated process is
targeted. After landing on local `main`, the complete command set and cleanup
tail run again immediately before remote readback and handoff.

Stop before implementation when the reviewed packet cannot be committed without
unowned work, an offline command would acquire missing software, the frozen
property set differs from exactly 179 names, the schema cannot be independent
from the later conformance catalog, a C01 semantic contract must change, a
second grammar/public raw token/unchecked constructor is required, or work would
cross into recovery, C05 catalog, root/sibling, dependency/feature, or owned
unsafe scope. A contradiction returns to specification/sequence reconciliation
rather than being decided inside a worker task.

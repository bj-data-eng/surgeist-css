# P01-I02-C14 Flexbox, Multicolumn, And Initiative Closure

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I02-C14` |
| Owning repository | `surgeist-css` |
| Status | `in_progress` |
| Cycle base | `3b067893117eb8453259b7138633c01933eca16e` |
| Published prerequisite | C13 `3b067893117eb8453259b7138633c01933eca16e`, fetched and read back |
| Reviewed P01 | `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256 `87f6a94b893ffa416c6ff451575f0d5a21b4aa136e7bcd391cd6c0ce8810a2ae` |
| Reviewed specification | `plans/specs/P01-I02-css-snapshot-2026-grammar-closure.md`, SHA-256 `3f93c7f6c3656ebe0b33b8bf9c32e458f306f294d1a969a86df375a5858b1710`, sections 4, 9-12 |
| Reviewed sequence | `plans/sequences/P01-I02-css-snapshot-2026-grammar-closure.md`, semantic SHA-256 `56bcf0340320339454e4ae1aa0b45a7ad2e37e03930e9e3bf7665f8ce4cbb15a`, entry `I02-C14` |
| Bounded outcome | Close Flexbox 1, Multicolumn 1, the remaining seven official Partial value rows and 17 Reserved records, exact 162/167/219 reconciliation, I02 acceptance mapping, and SHA-free final handoff. |

## 2. Exact remaining ownership

The base registry has exactly 17 C14 Reserved records: Flexbox `flex-flow`;
Multicolumn `column-count`, `column-fill`, `column-rule`,
`column-rule-color`, `column-rule-style`, `column-rule-width`, `column-span`,
`column-width`, and `columns`; and the generic authored-shell/value records
`at-rule`, `generic` qualified-rule, `generic` declaration, `stylesheet`,
`rule-list`, `declaration-list`, and `style-block`. Existing Complete rows and
truthful extensions remain unchanged. The remaining official Partial value rows
are `official.value.dimension`, `official.value.angle`,
`official.value.angle-percentage`, `official.value.time-percentage`,
`official.value.frequency`, `official.value.frequency-percentage`, and
`official.value.calc`; C14 closes each selected grammar with public positive,
boundary, adjacent-grammar rejection, and recovery vectors. The retained
`later.rule.font-feature-values` RecognizedUnsupported record remains explicitly
unsupported with its existing public negative evidence. C14 also audits every official row and
the exact 162 property, 167 non-property, and 219 baseline totals; it does not
invent new grammar beyond the selected sources or implement cascade/layout.

All changes are additive, parser-owned or checked public models, with private
fields and non-exhaustive evolving enums. No dependency, feature, manifest,
MSRV, root, sibling, generated artifact, unsafe, source-parsing test, or
initiative-state proxy is authorized.

## 3. Tasks

Every task records `task_base_sha="$(git rev-parse HEAD)"` immediately before
its RED commit; that commit is the actual parent of the two-commit task range.
T1 and T2 each use the same published C13 base independently and do not depend
on one another’s implementation output. T3 and later use the clean head after
their declared prerequisites. Every task makes a base-compilable public
behavioral or named-metadata RED commit followed by implementation. Tests parse
authored CSS/public metadata only. Each task runs this exact matrix before
independent review:

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

Each exact range receives a fresh independent review before the next task.

### T1 Flexbox flow closure

- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test flexbox_grammars c14_flex_flow_retain_typed_structure -- --exact`.
- **Area/acceptance:** complete `flex-flow` shorthand, separator/order/domain
  mutations, globals/substitutions, exact recovery and public accessors; no
  flex layout or resolution semantics.
- **Commits:** `test: specify C14 flex-flow`; `feat: add C14 flex-flow grammar`.

### T2 Multicolumn closure

- **Dependency:** independently ready from the published C13 base; no T1 output is required.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test multicolumn_grammars c14_multicolumn_properties_retain_typed_structure -- --exact`.
- **Area/acceptance:** nine Multicolumn properties, shared rule/style/width
  domains, shorthand separators, globals/substitutions, exact recovery,
  coordinates, sibling retention, and strict parity; no pagination/layout.
- **Commits:** `test: specify C14 multicolumn properties`; `feat: add C14 multicolumn grammars`.

### T3 Generic authored shells and shared values

- **Dependency:** T1 and T2 CLEAN.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test generic_shell_grammars c14_generic_authored_shells_retain_structure -- --exact`.
- **Area/acceptance:** generic at-rule/qualified-rule/declaration, stylesheet,
  rule-list, declaration-list, style-block, and all selected authored list
  recovery; unknown-versus-generic distinction, exact spans, EOF/non-BMP,
  repeated failures, and no source/proxy inspection.
- **Commits:** `test: specify C14 generic authored shells`; `feat: add C14 generic authored shells`.

### T4 I02 ledger and acceptance reconciliation

- **Dependency:** T1-T3 CLEAN.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog c14_remaining_official_values_are_typed -- --exact`.
- **Area/acceptance:** public metadata and vectors close the seven listed
  official Partial values and directly exercise every retained Partial extension
  and the RecognizedUnsupported font-feature-values boundary. The coordinator
  performs the exact 162 property, 167 non-property, 219 baseline, source,
  exclusion, implementation-inventory, and fourteen-finding reconciliation as
  review evidence, never as a Rust test or state/count proxy.
- **Commits:** `test: specify C14 ledger reconciliation`; `feat: reconcile C14 official ledger`.

### T5 Docs, migration handoff, and initiative closure

- **Dependency:** T1-T4 CLEAN.
- **RED:** `cargo test -p surgeist-css --offline --no-default-features --test conformance_catalog c14_closure_metadata_and_docs_are_truthful -- --exact`.
- **Area/acceptance:** README/rustdoc/doctests, direct public metadata tests,
  and SHA-free `plans/handoffs/P01-I02-C14-flexbox-multicolumn-and-initiative-closure.md` recording exact totals, promotions, exclusions, and product fixture digest only.
- **Commits:** `test: specify C14 closure metadata`; `docs: publish C14 and I02 closure`.

## 4. Completion and publication

After all tasks are independently CLEAN, make a separate status-only `complete`
commit. Run this exact final gate:

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
git diff --check 3b067893117eb8453259b7138633c01933eca16e..HEAD
test -z "$(git diff --unified=0 3b067893117eb8453259b7138633c01933eca16e..HEAD -- tests/fixtures/i01-c01-observables.tsv)"
git status --short --branch
ps -axo pid=,command=
```

Run the canonical owned-Rust unsafe scan and prove its manifest has no
executable matches. Holistic review is followed by a fresh exact-range I02
initiative review covering the 162/167/219 and fourteen-finding reconciliation;
only after both are CLEAN run `cargo clean --offline`, prove `target/` absent,
then fetch C13 and publish with the non-force fast-forward gate/readback. Only
after that verified publication may P01-I03 be JIT-planned.

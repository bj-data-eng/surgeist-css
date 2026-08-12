# P01-I01-C03 Structural Stylesheet Recovery

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I01-C03` |
| Owning repository | `surgeist-css` |
| Status | `reviewed` |
| Cycle base | `4697cdeb9e288d42761a01d407f12e27c238e154` |
| Reviewed specification | `plans/specs/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `76b76a50a613aea26e1b790749a780f7d05efdfe57711c6b8dbf9a9fca2359d7`, sections 1, 4 ordinary sheet API, 5, 6.1 structural rows, 6.2, 6.4, 7.3, 10, 12.1, and 13 finding 2.15 |
| Reviewed sequence | `plans/sequences/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `f3a65df04c5c5a4f6f02212fe4d69959b75bba1cdcf2fd12e5bfb012f2c4ec94`, entry `I01-C03 Structural Stylesheet Recovery` |
| Bounded outcome | `parse_sheet` returns a report and deterministically recovers structural at-rule, qualified-rule, declaration, descriptor, and keyframe-block failures while preserving valid siblings, exact diagnostics, balanced boundaries, source order, progress, and the structural nesting limit. |

## 2. Boundary

The published C02 base has final C01 source/error/report value types and final
property-coupled declaration models, but `parse_sheet(&str)` still returns
`Result<CssSheet>` and every `RuleBodyParser` failure aborts its containing
parse. The report constructors are crate-private foundations rather than a
populated recovery path. `CssSheet` has no encoding metadata, and the parser has
no CSS-owned recovery-unit coordinator, progress invariant, or shared depth
budget.

This cycle changes only the ordinary stylesheet front door to
`parse_sheet(&str) -> CssParseReport<CssSheet>` and implements the five
structural recovery rows allocated by the sequence: at-rules, qualified rules,
declarations, descriptors, and keyframe blocks. It also owns leading legacy
`@charset` metadata/recovery, structural diagnostic ordering, balanced recovery
spans, parent representability, loop progress, and the 256-depth budget for
rule blocks and the component/function nesting reached by these structural
paths. It does not implement forgiving selector-member recovery, media `Never`
replacement, general implicit-EOF retention, CDO/CDC diagnostics, style-
attribute parsing, `app-strict`, the conformance catalog, I02 grammar, or root/
sibling work. C04 owns those specialized recovery actions and completes the
depth/EOF/legacy matrix across its additional paths.

The base is published and read back with local `main`, `origin/main`, and remote
`main` equal to the recorded cycle base. `target` is absent and no process runs
from this repository's target tree. Existing pinned dependencies remain the
only dependencies; all resolving/building/testing/documenting/linting commands
use offline mode. Missing cached tooling is a blocker, never installation
authority.

## 3. Impacts

| Area | C03 classification |
| --- | --- |
| Public API | Breaking/additive: `parse_sheet` returns `CssParseReport<CssSheet>`; sheet encoding metadata and recovery-populated diagnostics/actions become observable. |
| Dependencies/features | Unchanged; no feature is added. |
| Generated artifacts | None; root owns API artifacts and remains untouched. |
| Docs/examples | Rustdoc and public-consumer tests explain report cleanliness, valid retained syntax, diagnostic ordering, spans/actions, and non-responsibilities; README closure remains C05. |
| MSRV | No leaf `rust-version`; edition remains 2024. |
| Root follow-up | None for this incomplete candidate; final facade/API migration follows C05. |
| Unsafe | Existing crate-root prohibition remains; no owned target may contain or enable unsafe. |

## 4. Tasks

### T1 Report Front Door, Top-Level Units, And Encoding

- **Files/area:** `src/parser/mod.rs`, report construction in `src/report.rs`,
  sheet/encoding models in `src/syntax.rs`, crate reexports/rustdoc, focused unit
  tests, and `tests/stylesheet_recovery.rs`.
- **Outcome:** `parse_sheet` has the final ordinary report signature. A single
  top-level recovery coordinator uses `cssparser` bounded rule iteration to
  retain valid rules in source order and turn failed at-rules or qualified rules
  into exactly one `CssRecoveryDiagnostic` with `DropAtRule` or
  `DropQualifiedRule`. `CssSheet` exposes optional private-field
  `CssEncodingDeclaration` metadata plus rules; the leading `@charset` matrix in
  section 6.2 is recognized/recovered before ordinary rules without decoding
  already-UTF-8 input.
- **RED evidence:** public tests first fail on the old `Result` signature,
  absent encoding API, and all-or-nothing top-level parsing. Independent vectors
  place a valid rule before and after unknown, recognized-unsupported,
  misplaced, malformed prelude/body, semicolon-form, and balanced block-form
  at-rules and malformed qualified rules. Encoding vectors cover empty input;
  valid leading form; BOM/whitespace/comment handling; missing semicolon;
  unquoted/empty label; duplicate and non-leading forms.
- **Acceptance:** retained syntax is valid and ordered; each drop has its exact
  C01 code/detail, first-responsible position, complete non-empty balanced span,
  and action; semicolons/braces inside nested balanced content do not split a
  unit. A valid encoding is metadata once and never a rule; invalid encoding
  never becomes metadata or trivia. Empty input returns a clean empty report.
  No strict alias or second parser remains.
- **Commands:** `cargo test -p surgeist-css --offline stylesheet_recovery_`;
  `cargo test -p surgeist-css --offline --test stylesheet_recovery`;
  `cargo test -p surgeist-css --offline --doc`;
  `cargo check -p surgeist-css --offline`.
- **Dependencies:** C02 only.
- **Intended commit:** `feat: recover top-level CSS stylesheet units`.

### T2 Declaration And Descriptor Recovery

- **Files/area:** declaration recovery in `src/parser/mod.rs` and nesting rule
  adapters, descriptor recovery in `src/parser/font_face.rs`, report/error/span
  adapters, focused unit tests, and `tests/block_item_recovery.rs`.
- **Outcome:** each style/scoped declaration list owns one declaration recovery
  coordinator, and `@font-face` owns a distinct descriptor coordinator. Unknown,
  unsupported, invalid-value, invalid-annotation, and malformed declarations
  drop only their top-level semicolon/block-end unit with `DropDeclaration`;
  unknown/unsupported/invalid/duplicate descriptors drop only their descriptor
  unit with `DropDescriptor`. Later valid siblings remain eligible. The C02
  declaration and descriptor occurrence models remain the only retained models.
- **RED evidence:** independent matrices first show current parent-aborting
  behavior. Each error class is placed between valid siblings, with nested
  functions/blocks containing misleading semicolons. Vectors assert ordinary,
  custom, nested-style, and every font-face descriptor context; missing final
  semicolon at block end; repeated failures; and a child drop that makes a
  required `@font-face` aggregate unrepresentable.
- **Acceptance:** every failed unit advances to its own exact boundary and emits
  one typed diagnostic with complete span/action; no failed declaration or
  descriptor leaks a partial value. Valid siblings retain authored order and
  positions. If required descriptors are absent after child recovery, child
  diagnostics precede the `DropAtRule` parent diagnostic and no partial
  `CssFontFaceRule` survives. Optional descriptor loss alone retains the parent.
- **Commands:** `cargo test -p surgeist-css --offline block_item_recovery_`;
  `cargo test -p surgeist-css --offline --test block_item_recovery`;
  `cargo check -p surgeist-css --offline`.
- **Dependencies:** T1.
- **Intended commit:** `feat: recover CSS declarations and descriptors`.

### T3 Nested Rules And Keyframe Block Recovery

- **Files/area:** nested/scoped rule coordinators in `src/parser/mod.rs` and
  `src/parser/nesting.rs`, keyframe block coordinator in
  `src/parser/keyframes.rs`, authored parent models where representation requires
  a private migration, focused unit tests, and
  `tests/nested_structural_recovery.rs`.
- **Outcome:** every structural nested rule context independently recovers its
  owned at-rule/qualified-rule units without crossing the current balanced
  block. `@keyframes` drops an invalid selector or malformed keyframe block with
  `DropKeyframeBlock` and continues with later blocks. A parent is retained only
  when its final authored model remains representable; child-then-parent
  diagnostics preserve discovery order at tied/overlapping positions.
- **RED evidence:** tables first show aborting nested behavior. A valid sibling
  precedes and follows failures inside layer/media/container/scope/style nesting
  and keyframes. Cases include nested balanced delimiters with misleading braces
  or commas, invalid keyframe selector/declaration/block, repeated child failures,
  empty surviving collections where grammar permits them, and child loss that
  invalidates the smallest parent.
- **Acceptance:** recovery never reinterprets failed-unit tokens as siblings,
  crosses a balanced context, or leaks children from a dropped parent. Each unit
  produces the correct exact action/span/error and valid nodes remain in authored
  order with semantic positions. Diagnostic order is first responsible byte,
  with discovery order for ties and child before newly unrepresentable parent.
- **Commands:** `cargo test -p surgeist-css --offline nested_structural_`;
  `cargo test -p surgeist-css --offline --test nested_structural_recovery`;
  `cargo check -p surgeist-css --offline`.
- **Dependencies:** T1 and T2.
- **Intended commit:** `feat: recover nested CSS rules and keyframes`.

### T4 Progress, Structural Depth, And Panic-Freedom Closure

- **Files/area:** shared parser recovery state/depth budget under `src/parser/`,
  structural recovery call sites from T1 through T3, diagnostic ordering/report
  finalization, focused unit tests, rustdoc, and
  `tests/structural_recovery_adversarial.rs`.
- **Outcome:** every C03 recovery loop records its starting byte and either
  retains a node, advances at least one byte, or terminates bounded input. One
  shared depth counter enforces 256 nested structural rule/component/function
  levels reached by C03: at the limit, it drops the smallest balanced enclosing
  structural unit using `NestingLimit` and `StopAtNestingLimit`, or the remaining
  bounded input at EOF. Report finalization orders diagnostics by responsible
  byte with stable discovery-order ties. Ordinary `&str` input cannot unwind.
- **RED evidence:** focused tests first fail on missing progress/depth state.
  Vectors cover zero-progress dependency errors, repeated malformed units,
  empty/bad tokens, arbitrary Unicode/non-BMP coordinates, depth 255/256/257 for
  structural blocks and component/functions reached from declarations, smallest
  enclosing-unit discard, child/parent ties, and bounded `catch_unwind` tables
  over adversarial ordinary input.
- **Acceptance:** no loop spins or skips a valid later sibling; all C03 structural
  depth paths share one budget and exact typed diagnostic; no partial over-limit
  node survives. Diagnostic ordering is deterministic without prose sorting or
  deduplication. C04 may extend the same counter to specialized selector/media/
  style-attribute paths but may not reinterpret C03 behavior. All changed public
  items have phase/invariant/non-responsibility rustdoc and the final ordinary
  front door has no `Result` compatibility path.
- **Commands:** `cargo test -p surgeist-css --offline structural_recovery_`;
  `cargo test -p surgeist-css --offline --test structural_recovery_adversarial`;
  `cargo test -p surgeist-css --offline --doc`;
  `cargo clippy -p surgeist-css --offline --all-targets -- -F unsafe-code -D warnings`.
- **Dependencies:** T1, T2, and T3.
- **Intended commit:** `feat: enforce CSS structural recovery progress`.

## 5. Completion

C03 is accepted when all four task ranges are independently reviewed `CLEAN`;
`parse_sheet` has the final report signature; every structural row allocated to
C03 retains valid siblings with exact typed diagnostics, positions, spans,
actions, and ordering; encoding and parent-representability matrices are clean;
all structural loops make progress and enforce the shared depth budget; ordinary
input does not unwind; a fresh holistic review is `CLEAN`; and the cycle is
landed/published with remote readback. The handoff names C03 as an incomplete I01
parser candidate and makes C04 the only next ready cycle.

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

The unsafe scan succeeds only with no matches. Any process executing from this
repository's target tree before cleanup is stale for this completed cycle and
is safely terminated by exact path before rerunning the probe; unrelated
processes are never targeted. The complete main gate and cleanup tail run again
after landing and immediately before remote readback.

Stop before implementation if the reviewed packet cannot be committed without
unowned work, offline commands would acquire software, a C01/C02 semantic
contract must change, recovery requires a second grammar/raw dependency token/
invalid retained node, the 256 limit cannot be shared with C04, or work crosses
into specialized C04 recovery, C05 catalog, I02 grammar, dependency/feature,
root/sibling, or owned unsafe scope. Such a contradiction returns to planning
reconciliation rather than being decided by a worker.

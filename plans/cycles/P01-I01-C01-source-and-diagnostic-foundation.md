# P01-I01-C01 Source And Diagnostic Foundation

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I01-C01` |
| Owning repository | `surgeist-css` |
| Status | `reviewed` |
| Cycle base | `4b288d6467d91f2fc33eac78ef0b0b725154195d` |
| Reviewed specification | `plans/specs/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `76b76a50a613aea26e1b790749a780f7d05efdfe57711c6b8dbf9a9fca2359d7`, sections 4, 5, 7, 10, 11, 12.5, and 13 findings 2.22, 2.23, 2.25 |
| Reviewed sequence | `plans/sequences/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `f3a65df04c5c5a4f6f02212fe4d69959b75bba1cdcf2fd12e5bfb012f2c4ec94`, entry `I01-C01 Source And Diagnostic Foundation` |
| Bounded outcome | The existing strict parser uses the final semantic source-coordinate and structured-error boundary; final recovery report/diagnostic value types are exposed without changing ordinary parser signatures; crate-owned Rust forbids unsafe. |

## 2. Boundary

The published base exposes `Error` with prose-heavy `ErrorKind` variants and raw
`u32` line/column fields, `CssSourceLocation` with a public arbitrary
constructor, and strict `parse_sheet(&str) -> Result<CssSheet>`. It has no
recovery report/action/span types and no crate-root unsafe prohibition. Existing
parser modules translate `cssparser` failures through `src/error.rs`; syntax
nodes in `src/syntax.rs` carry the baseline location type.

This cycle owns only the final source, error, recovery-diagnostic value
foundation and its migration through the still-strict parser. It does not change
either ordinary parser signature, implement browser recovery, add style-
attribute or strict-validation entry points, migrate declarations/properties,
add the support catalog, add a Cargo feature/dependency, or edit root/siblings.

The safe pinned dependency is already present in `Cargo.lock`; every Cargo
command that resolves, builds, tests, or lints dependencies uses offline mode.
`cargo fmt --check` only invokes the already-installed Rustfmt component over
local source and performs no dependency resolution. A missing cached dependency
or tool is a tooling blocker, not permission to acquire software. The initial
stale-process check at cycle intake found no process executing from this
repository's target tree.

## 3. Impacts

| Area | C01 classification |
| --- | --- |
| Public API | Breaking/additive: replace raw locations and prose-only errors; add source/span, report, recovery-diagnostic/action types. `parse_sheet` remains strict until C03. |
| Dependencies/features | Unchanged; no feature added. |
| Generated artifacts | None; root-owned API artifacts remain untouched. |
| Docs/examples | Rustdoc and focused public-consumer coverage for changed/new public types; README initiative guidance remains C05. |
| MSRV | No leaf `rust-version` is introduced; edition remains 2024. |
| Root follow-up | None for this incomplete initiative candidate; final breaking migration is handed off after C05. |
| Unsafe | Add `#![forbid(unsafe_code)]`; no owned target may contain or enable unsafe. |

## 4. Tasks

### T1 Semantic Source Coordinates And Spans

- **Files/area:** `src/source.rs`, location-bearing types and accessors in
  `src/syntax.rs`, crate-root reexports, focused unit tests, and
  `tests/source_coordinates.rs`.
- **Outcome:** private-field `CssByteOffset`, `CssLineIndex`,
  `CssUtf16ColumnIndex`, `CssSourcePosition`, and `CssSourceSpan` implement the
  exact section 7.1 convention. Every currently location-bearing retained node
  exposes `position()` and no public arbitrary position constructor or obsolete
  `CssSourceLocation`/`location()` alias remains.
- **RED evidence:** public/focused tests first fail because the semantic types,
  byte offset, zero-based UTF-16 conversion, span invariant, and `position()`
  surface do not exist; named cases include empty input at byte/line/column zero,
  first/later columns, LF/CRLF, multiline comments, a CSS escape whose authored
  byte width differs from its decoded spelling, and a supplementary Unicode
  scalar whose UTF-8 and UTF-16 widths differ.
- **Acceptance:** dependency conversion is total, byte and UTF-16 coordinates
  differ correctly, and every named vector asserts the exact byte offset,
  zero-based line, and zero-based UTF-16 column; spans are ordered/private, all
  baseline parser tests compile against semantic positions, and public
  construction cannot forge positions.
- **Commands:** `cargo test -p surgeist-css --offline source_`; `cargo test -p surgeist-css --offline --test source_coordinates`; `cargo check -p surgeist-css --offline`.
- **Dependencies:** none.
- **Intended commit:** `feat: add semantic CSS source coordinates`.

### T2 Structured Error Taxonomy

- **Files/area:** `src/error.rs`, direct parser error-construction call sites,
  error-focused unit tests, and `tests/structured_errors.rs`.
- **Outcome:** `Error`, `CssErrorCode`, `ErrorKind`, semantic detail values,
  token summaries, grammar/production identifiers, and declaration contexts
  implement section 7.2. The still-strict parser maps every current failure to
  one exact root code/detail and semantic position; display prose is not control
  flow.
- **RED evidence:** focused tests first fail on missing exact codes/details and
  on current free-form reason variants. Each implemented root category has a
  named public accessor vector, including EOF versus encountered token and
  unknown versus recognized-unsupported identity where C01 source can supply it.
- **Acceptance:** every reachable current error path uses the one-to-one
  code/kind mapping, no catch-all reason variant remains, positions identify the
  responsible token, display is one-based, and public non-exhaustive matching is
  demonstrated without `Debug` parsing.
- **Commands:** `cargo test -p surgeist-css --offline error_`; `cargo test -p surgeist-css --offline --test structured_errors`; `cargo check -p surgeist-css --offline`.
- **Dependencies:** T1.
- **Intended commit:** `feat: structure CSS parser diagnostics`.

### T3 Recovery Diagnostic Value Foundation And Unsafe Prohibition

- **Files/area:** `src/report.rs`, `src/lib.rs`, focused unit tests,
  `tests/diagnostic_foundation.rs`, and rustdoc on all C01 public items.
- **Outcome:** private-field `CssParseReport<T>`, `CssRecoveryDiagnostic`,
  `CssRecoveryAction`, and `CssValidationFailure` implement sections 4 and 7.3
  as final authored/diagnostic value types. Crate root forbids unsafe. Ordinary
  parser signatures remain strict and no production-only test hook is exposed.
- **RED evidence:** focused tests first fail because report decomposition,
  cleanliness, diagnostic error/span/action accessors, non-empty validation
  failure, and the non-exhaustive action surface do not exist.
- **Acceptance:** crate-private construction enforces report/diagnostic/failure
  invariants; public consumers compile against every accessor and wildcard
  evolution boundary; rustdoc identifies phase and non-responsibilities; the
  crate and all tests compile with unsafe forbidden.
- **Commands:** `cargo test -p surgeist-css --offline report_`; `cargo test -p surgeist-css --offline --test diagnostic_foundation`; `cargo test -p surgeist-css --offline --doc`; `cargo clippy -p surgeist-css --offline --all-targets -- -F unsafe-code -D warnings`.
- **Dependencies:** T1 and T2.
- **Intended commit:** `feat: expose CSS recovery diagnostic foundation`.

## 5. Completion

C01 is accepted when all three task ranges are independently reviewed `CLEAN`,
the still-strict parser and every location-bearing public node use the final C01
model, the configured checks below pass on the integrated range, a fresh holistic
cycle review is `CLEAN`, and the cycle is landed/published with remote readback.
The candidate handoff names it as an incomplete I01 foundation and makes C02 the
only next ready cycle.

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
executing from the target tree is stale for this completed cycle and must be
terminated safely, then the check rerun; no unrelated process is targeted.
After landing on local `main`, the applicable main gates run again, followed by
the same two process checks and `cargo clean` tail immediately before remote
readback and handoff. Cycle completion therefore leaves neither build artifacts
nor a target-tree process.

Stop before implementation when the reviewed packet cannot be committed without
including unowned work, an offline Cargo command would acquire missing software,
or C01 would require an ordinary parser signature change, declaration migration,
new dependency/feature, root/sibling edit, or owned unsafe. A source/error design
contradiction returns to specification reconciliation rather than being decided
inside a worker task.

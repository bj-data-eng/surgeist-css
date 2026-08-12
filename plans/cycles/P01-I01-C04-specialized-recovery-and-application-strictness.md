# P01-I01-C04 Specialized Recovery And Application Strictness

## 1. Header

| Field | Value |
| --- | --- |
| Cycle ID | `P01-I01-C04` |
| Owning repository | `surgeist-css` |
| Status | `complete` |
| Cycle base | `51bfe8a4dc45ae4030c10072d1882821cacab51e` |
| Reviewed specification | `plans/specs/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `76b76a50a613aea26e1b790749a780f7d05efdfe57711c6b8dbf9a9fca2359d7`, sections 4, 6.1 specialized rows, 6.3, 6.4, 7, 8.3, 12.1, 12.2, and 13 findings 2.5 and 2.6 |
| Reviewed sequence | `plans/sequences/P01-I01-browser-recovery-authored-api-foundation.md`, SHA-256 `f3a65df04c5c5a4f6f02212fe4d69959b75bba1cdcf2fd12e5bfb012f2c4ec94`, entry `I01-C04 Specialized Recovery And Application Strictness` |
| Bounded outcome | Specialized selector/media/EOF/legacy recovery completes the ordinary stylesheet matrix; style attributes reuse the ordinary declaration core; `app-strict` validates one ordinary report without changing parser output. |

## 2. Boundary

The published C03 base has the final stylesheet report signature and structural
recovery for rules, declarations, descriptors, and keyframe blocks, including
shared progress/depth enforcement. CDO/CDC are still silently skipped for
compatibility; recognized forgiving selector members and malformed media-list
members still abort their owning production; implicit EOF closure has no
diagnostic action; there is no style-attribute front door or Cargo feature.

This cycle owns the five remaining specialized actions:
`DropSelectorListItem`, `ReplaceMediaQueryWithNever`,
`RetainWithImplicitClosure`, `IgnoreLegacyToken`, and the extension of
`StopAtNestingLimit` to specialized paths. It owns
`parse_style_attribute(&str) -> CssParseReport<CssDeclarationList>` and exactly
the empty-default `app-strict` feature with one-pass sheet/style validators. It
does not add grammar beyond the frozen I01 selection, add the independent
catalog/metadata, rewrite the C03 structural coordinator, edit README/root/
siblings, or introduce dependencies.

The published base is verified on all three main refs; `target` is absent and
no repository-target process exists. All resolving/building/testing/documenting
commands use offline mode. No external software acquisition is authorized.

## 3. Impacts

| Area | C04 classification |
| --- | --- |
| Public API | Breaking/additive: migrate media condition/query values to parser-positioned private-field models so every query can expose exact provenance; add `Never`, style-attribute reports, and feature-gated validators. Ordinary sheet signatures remain final. |
| Dependencies/features | No dependency delta; add exactly `default = []` and `app-strict = []`. |
| Generated artifacts | None; root-owned API artifacts remain untouched. |
| Docs/examples | Rustdoc/doctests for all specialized states/actions and both feature modes; README product closure remains C05. |
| MSRV | No leaf `rust-version` is introduced; edition remains 2024. |
| Root follow-up | None until the final C05 candidate. |
| Unsafe | Existing prohibition remains absolute. |

## 4. Tasks

### T1 Forgiving Selectors And Media Never Sentinels

- **Files/area:** selector/media models and parsers, recovery plumbing, focused
  tests, and `tests/specialized_list_recovery.rs`.
- **Outcome:** invalid members of already-recognized `:is()`/`:where()` lists
  alone drop with `DropSelectorListItem`; every other selector list remains
  unforgiving. Malformed media-query-list members become parser-owned
  `CssNeverMediaQuery` values in authored order with exactly one
  `ReplaceMediaQueryWithNever` diagnostic. `CssMediaQuery` exposes the exact
  non-exhaustive Condition/Typed/Never shape, `position()`, and
  `is_guaranteed_false()` semantics.
- **Positioned media model:** the baseline unit-heavy public
  `CssMediaCondition` enum becomes a private-field parser-produced
  `CssMediaCondition { kind, position }` with `kind()` and `position()`;
  `CssMediaConditionKind` is the public non-exhaustive semantic union of the
  former condition variants. `CssTypedMediaQuery` likewise stores its parser-
  produced first-nontrivia position privately and exposes `position()`.
  `CssMediaCondition::position()` likewise means the first non-trivia token of
  the condition production, including a leading logical operator when it owns
  the condition.
  `CssMediaQuery::position()` delegates to Condition/Typed/Never, and no public
  constructor can forge any position. This intentional breaking migration is
  the minimum C04 provenance change and does not reinterpret condition grammar.
- **RED evidence:** independent comma-list tables first show parent abort.
  Vectors cover first/middle/last/only/empty forgiving members, nested balanced
  commas, `:not()`/ordinary selector rejection, and malformed/empty media
  members between valid members with Unicode positions. Separate exact vectors
  assert first-nontrivia positions and `CssMediaQuery::position()` delegation
  for a condition-only query (including a leading logical operator), a typed
  query with modifier/media type, and a `Never` member; at least one uses a
  preceding supplementary Unicode scalar to distinguish byte and UTF-16 units.
- **Acceptance:** recovery never crosses a comma member; selector survivors and
  `Never` sentinels preserve authored order; exact error/position/full member
  span/action/order is asserted; a clean parse never constructs `Never`; no
  public sentinel constructor exists. Condition, typed, and Never positions all
  match the first-nontrivia rule and their delegated query position exactly.
- **Commands:** `cargo test -p surgeist-css --offline specialized_list_`;
  `cargo test -p surgeist-css --offline --test specialized_list_recovery`;
  `cargo check -p surgeist-css --offline`.
- **Dependencies:** C03.
- **Intended commit:** `feat: recover specialized CSS list members`.

### T2 Implicit EOF, Legacy Tokens, And Specialized Depth Closure

- **Files/area:** stylesheet/specialized parser recovery and shared depth state,
  report actions, focused tests, and `tests/specialized_recovery_boundaries.rs`.
- **Outcome:** representable CSS Syntax implicit EOF closures retain their node
  and emit `RetainWithImplicitClosure` with a zero-width EOF span; non-
  representable missing-token cases still drop. Top-level CDO/CDC now emit
  `IgnoreLegacyToken` with exact non-empty token spans while retaining later
  rules. C03's depth budget covers selector/media specialized recursion without
  changing the 256 boundary or structural behavior.
- **Finite EOF allocation:** C04 emits one action for each missing closer in
  exactly these retained contexts: the final block of a style rule; the final
  block of `@layer`, `@media`, `@container`, `@scope`, `@font-face`, or
  `@keyframes`; a final keyframe block; and a final balanced function,
  parenthesis, square-bracket, or curly-bracket component inside an otherwise
  valid declaration value, selector, media query, descriptor value, or keyframe
  declaration. Each uses its completed owning node, `UnexpectedEnd` at EOF, and
  `[EOF, EOF)`. Nested missing closers emit one action per completed owner in
  innermost-to-outermost discovery order at the tied EOF position. Every non-
  enumerated EOF case preserves C03 behavior. Named dropped counterparts are an
  unterminated string, bad URL, missing required at-rule prelude, missing
  selector before a block, missing declaration colon/value, and an owner whose
  required child remains absent.
- **RED evidence:** tables first show silent legacy tokens, absent EOF actions,
  and uncovered specialized depth. Cases cover each representable owning grammar
  and one non-representable counterpart, CDO/CDC before/between rules, depth
  255/256/257 in selector/media functions, misleading delimiters, and EOF at an
  over-limit unit.
- **Acceptance:** every implicit-closure diagnostic corresponds to a retained
  valid node; only that action has zero-width recovery span; each legacy token
  has one exact diagnostic; first over-limit specialized unit uses exact
  NestingLimit/StopAtNestingLimit without changing C03 outputs.
- **Commands:** `cargo test -p surgeist-css --offline specialized_boundary_`;
  `cargo test -p surgeist-css --offline --test specialized_recovery_boundaries`;
  `cargo check -p surgeist-css --offline`.
- **Dependencies:** T1.
- **Intended commit:** `feat: complete CSS specialized recovery boundaries`.

### T3 Style-Attribute Recovery Front Door

- **Files/area:** shared declaration coordinator, style-attribute adapter,
  crate reexports/rustdoc, focused tests, and `tests/style_attribute_recovery.rs`.
- **Outcome:** `parse_style_attribute` has the final report signature and reuses
  the one ordinary declaration core. Empty/trivia-only input is clean and empty;
  an optional final semicolon is accepted. Invalid declaration units—including
  at-rules, qualified rules, colonless segments, and malformed separators—drop
  independently with `DropDeclaration`; later valid declarations survive.
- **RED evidence:** public tests first fail on absent API. A parity table runs
  ordinary/custom/global/substitution/importance and every declaration error
  class through a style-rule block and style attribute; specialized invalid-item
  vectors assert exact spans/actions/order and no rule node.
- **Acceptance:** equivalent declaration sources produce identical retained
  declaration values and diagnostics modulo source offsets; no second property
  grammar or public raw token surface exists; style input never returns a rule.
- **Commands:** `cargo test -p surgeist-css --offline style_attribute_`;
  `cargo test -p surgeist-css --offline --test style_attribute_recovery`;
  `cargo test -p surgeist-css --offline --test declaration_importance`;
  `cargo test -p surgeist-css --offline --test authored_declaration_values`.
- **Dependencies:** T2 and C02 declaration model.
- **Intended commit:** `feat: parse recovering CSS style attributes`.

### T4 One-Pass Application Strictness And Feature Parity

- **Files/area:** `Cargo.toml`, feature-gated validators in crate front door,
  report validation conversion, rustdoc/doctests, focused tests, and
  `tests/app_strict_parity.rs`.
- **Outcome:** manifest has exactly empty-default `app-strict`; when enabled,
  `validate_sheet` and `validate_style_attribute` invoke their ordinary parser
  exactly once, return syntax for clean reports, and otherwise return the full
  non-empty `CssValidationFailure`. Ordinary default/feature reports are
  structurally identical and no feature-dependent grammar/model/dispatch exists.
- **RED evidence:** default public behavior table is captured first; feature
  tests then fail on absent APIs. Shared sheet/style vectors cover clean,
  recovered, multiple diagnostics, `Never`, implicit closure, legacy token, and
  structural/specialized depth. Feature-gated crate-local unit tests own a
  `#[cfg(test)]` crate-private atomic counter at the ordinary parser boundary;
  each resets it, invokes one validator, and asserts a final count of one for
  clean and recovered sheet/style input. Public integration tests cannot access
  the counter and prove only observable parity.
- **Acceptance:** validation never reparses, truncates, reorders, or changes
  diagnostics; failure accessors expose the identical full set; default mode
  contains no validator symbols; the real feature matrix is green and ordinary
  outputs match exactly.
- **Commands:** `cargo test -p surgeist-css --offline --no-default-features
  app_strict_one_pass_`; `cargo test -p surgeist-css --offline
  --no-default-features --features app-strict app_strict_one_pass_`;
  `cargo test -p surgeist-css --offline --no-default-features --features
  app-strict --test app_strict_parity`; and the exact completion matrix below.
- **Dependencies:** T1–T3.
- **Intended commit:** `feat: add one-pass strict CSS validation`.

## 5. Completion

C04 is accepted when all task ranges are independently `CLEAN`; all ten I01
recovery actions are reachable only in their specified contexts; stylesheet and
style-attribute matrices are exact; ordinary outputs are feature-invariant;
strict validation is one-pass and complete; final default/feature gates and a
fresh holistic review are clean; and the cycle publishes with remote readback.
C05 is then the only ready cycle.

The crate-root rustdoc includes default-only `compile_fail` examples under
`cfg(not(feature = "app-strict"))` that attempt to import both validators; the
feature-enabled public integration target imports and invokes them. Final gates
are exactly:

```sh
cargo check -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features
cargo test -p surgeist-css --offline --no-default-features --doc
cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
cargo check -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --features app-strict
cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
cargo fmt --check
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps
RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps
rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs
! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .
! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'
cargo clean
test ! -d target
! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'
```

After holistic `CLEAN`, authority `main` must still equal the recorded C03
base; the immutable C04 head is pushed by explicit lease, and fresh readback
proves local `main == origin/main == observed remote main == C04 head`, candidate
reachability, clean status, target absence, and the C05-only handoff.

Stop for planning reconciliation if work needs a second grammar, changes a C01–
C03 meaning, makes malformed syntax a non-`Never` retained authored claim,
changes ordinary output under the feature, adds a dependency/catalog/README/
root/sibling work, or introduces owned unsafe.

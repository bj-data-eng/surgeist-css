# Mechanical `lib.rs` Refactor

## Goal

Split the oversized `src/lib.rs` into focused modules while preserving the
public API and parser behavior exactly. This is a mechanical refactor, not a
grammar, validation, or model redesign.

`lib.rs` should remain the public crate front door: module declarations,
re-exports, and the public `parse_sheet` API. Parser implementation, error
plumbing, and tests should move into named modules that make future CSS work
easier to coordinate.

## Non-Goals

- Do not add CSS property support.
- Do not loosen or tighten grammar unless a moved test exposes an accidental
  behavior change caused by the refactor.
- Do not change public API shape, error kinds, source locations, or authored
  syntax types.
- Do not move `src/syntax.rs` model definitions in this pass unless a reviewer
  confirms the move is purely mechanical and needed to complete the parser
  split.
- Do not edit sibling repos or root submodule pointers.

## Standards

- Follow `AGENTS.md`.
- Use `guidance/surgeist-rust-modeling-guide.md`.
- Workers do code changes; coordinator integrates after clean review.
- Keep each task mechanical and commit it as a traceable logical point.
- Run focused checks after each task and full checks before final review.
- Because this is a refactor, the existing tests are the red/green guardrail:
  each worker should run a baseline or focused test before/after the move and
  report both.

## Target Module Shape

Preferred final shape:

```text
src/lib.rs
src/error.rs
src/parser/mod.rs
src/parser/selectors.rs
src/parser/layout.rs
src/parser/grid.rs
src/parser/box_model.rs
src/parser/typography.rs
src/parser/background.rs
src/parser/effects.rs
src/parser/timing.rs
src/parser/values.rs
src/tests.rs
src/test_support.rs
src/syntax.rs
src/validation.rs
```

If a worker finds this exact split creates unnecessary churn, they may keep a
larger module, but must explain why and keep the change mechanical.

## Task 1: Move Tests Out Of `lib.rs`

Move the `#[cfg(test)] mod tests` body from `src/lib.rs` into `src/tests.rs`
and wire it with `#[cfg(test)] mod tests;`.

Requirements:

- Preserve every existing test name and assertion.
- Keep `src/test_support.rs` as the shared test helper module.
- Do not change parser behavior or public API.
- `lib.rs` should shrink substantially.

Checks:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 2: Move Error Types And Error Helpers

Move `ErrorKind`, `Error`, display/error impls, `Result`, and parser error
helper functions into `src/error.rs` where practical.

Requirements:

- Public re-exports from `lib.rs` must remain compatible.
- Error source line/column behavior must remain unchanged.
- Parser internals may use `crate::error::*` or explicit imports.
- Keep this task focused on error plumbing only.

Checks:

```sh
cargo fmt --check
cargo test -p surgeist-css strict
cargo test -p surgeist-css rejection
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 3: Create Parser Front Door Module

Move `parse_sheet`, parser structs, declaration dispatch, and shared parser
utility glue into `src/parser/mod.rs`.

Requirements:

- `lib.rs` should still expose `pub fn parse_sheet(input: &str) -> Result<CssSheet>`
  directly or via `pub use parser::parse_sheet`.
- Keep parser behavior unchanged.
- Do not split property-family parsers yet unless required for compilation.

Checks:

```sh
cargo fmt --check
cargo test -p surgeist-css strict
cargo test -p surgeist-css coverage
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 4: Split Parser Family Modules

Mechanically move parser functions from `src/parser/mod.rs` into focused child
modules.

Suggested ownership:

- `selectors.rs`: selector parsing
- `layout.rs`: display, box-sizing, position, direction, overflow, flex basics,
  float/clear, alignment, visibility, z-index, order/flex
- `grid.rs`: grid track, area, line, shorthand parsing
- `box_model.rs`: edges, border, radius, shadow, colors where coupled to
  border/background shorthand
- `typography.rs`: font and text families
- `background.rs`: images, CSS positions, background, cursor, outline
- `effects.rs`: transforms, filters, clip-path, masks
- `timing.rs`: transitions and animations
- `values.rs`: lengths, calc, numbers, custom idents, colors, shared primitive
  parsers

Requirements:

- Keep visibility as narrow as possible, but do not fight Rust privacy so much
  that the move becomes semantic.
- Avoid broad prelude modules that hide dependencies.
- No behavior changes.

Checks:

```sh
cargo fmt --check
cargo test -p surgeist-css acceptance
cargo test -p surgeist-css rejection
cargo test -p surgeist-css no_recovery
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

## Task 5: Final Cleanup And Holistic Review

Audit the final module layout and trim any leftover accidental coupling.

Requirements:

- `src/lib.rs` should be small and read like a crate front door.
- `src/parser/mod.rs` should not simply become the new 9k-line file if the
  family split is practical.
- Update README only if module structure documentation is useful.
- Assign final clean-context holistic review.

Final checks:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
git status --short --branch
```

Completion requires every task worker/reviewer cycle to be clean, final checks
to pass, and final holistic review to approve.

VERDICT: NOT CLEAN

SCOPE: `surgeist-css` at `318864d1074d8d723a3a925528343c8a3d8c7253` (`main`), reviewed for CSS Snapshot 2026 compatibility, strict whole-input syntax handling, crate-boundary correctness, API quality, and local verification health. Implementation, root-facade integration, dependency changes, commits, and publication were out of scope.

EVIDENCE CHECKED: `AGENTS.md`, `Cargo.toml`, `README.md`, the public front door in `src/lib.rs`, all owned Rust source and tracked tests under `src/`, the local Cargo target/feature inventory, and the official W3C specifications linked below. Three independent zero-history review passes covered standards completeness, strict-parser defects, and API/test quality; the coordinator reproduced and consolidated their actionable findings.

# CSS Snapshot 2026 compatibility review

## Compatibility interpretation

The minimum compatibility baseline used here is the set of specifications that the [CSS Snapshot 2026 §2.1](https://www.w3.org/TR/css-2026/#css-official) defines as CSS in 2026, restricted to this crate's documented ownership of authored CSS syntax. Cascade application, variable substitution, value resolution, selector matching, and resource loading remain outside this crate. Syntax owned by those specifications—including declarations, rule structure, selectors, media queries, and authored values—remains in scope even when its later semantic processing belongs elsewhere.

Modules listed as substantially interoperable or as safe-to-use pre-CR exceptions in Snapshot 2026 were checked where the crate already claims support, such as Grid, transitions, animations, relative color syntax, and `color-mix()`. The review does not require the crate to implement every experimental draft. It does require claimed parsers to accept valid syntax, reject invalid syntax, preserve authored distinctions needed downstream, and reject the entire parse operation when strict parsing encounters an error, as promised by `README.md`.

The crate does not meet even the §2.1 minimum baseline today. The compatibility gaps are therefore not limited to draft or aspirational CSS.

## Findings

### [Important] Snapshot-defined at-rule families are absent

Location: `src/parser/mod.rs:107-200`, `src/syntax.rs:37-47`, and the rejection expectations in `src/tests.rs`.

The top-level parser recognizes only a narrow rule set: imports, layers, font faces, keyframes, style rules, media, container, and scope rules. It rejects stable syntax families required by the Snapshot baseline, including `@supports`, `@namespace`, `@counter-style`, `@page`, and `@font-feature-values`. In particular, Conditional Rules 3, Namespaces 3, Counter Styles 3, CSS2 page rules, and Fonts 3 are all part of the official 2026 definition. Some valid `@supports` input is deliberately asserted to fail in the current tests.

Impact: a stylesheet using ordinary CSS Snapshot 2026 syntax cannot be represented, so the crate's strict whole-sheet front door rejects conforming CSS.

Required remediation: add typed rule models and strict parsers for every in-boundary Snapshot at-rule family, with positive, negative, nested, ordering, and round-trip/preservation coverage. Track intentionally deferred post-Snapshot rule families separately rather than treating all unknown at-rules alike.

References: [Conditional Rules 3 `@supports`](https://www.w3.org/TR/css-conditional-3/#at-ruledef-supports), [Namespaces 3](https://www.w3.org/TR/css3-namespace/#declaration), [Counter Styles 3](https://www.w3.org/TR/css-counter-styles-3/#the-counter-style-rule).

### [Important] The declaration surface is far short of Snapshot 2026 and overstates support for several listed properties

Location: `src/validation.rs:31-213`, the property dispatch beginning at `src/parser/mod.rs:770`, and property grammars including `src/parser/layout.rs:8-91` and `src/parser/background.rs`.

The allowlist contains 179 names, but broad portions of CSS2 and Snapshot modules are absent. Even basic valid forms such as `display: inline` and `overflow: auto` are rejected. Several names that are listed as supported implement only a small subset of their grammar: `background` is routed through a color-only parser, `border-color` accepts one color rather than its one-to-four-value grammar, and list-valued background box and gap forms are incomplete.

Impact: the public support inventory is not a reliable compatibility contract. Callers cannot distinguish a fully supported property from a property for which only one convenient spelling is recognized.

Required remediation: derive a property-by-property Snapshot matrix from the owning W3C grammars, distinguish complete from partial support in machine-readable metadata, and close each grammar with conformance vectors before labeling it supported. Start with CSS2 and the §2.1 modules before expanding draft-module breadth.

References: [CSS2 `display`](https://www.w3.org/TR/CSS2/visuren.html#display-prop), [CSS2 `overflow`](https://www.w3.org/TR/CSS2/visufx.html#overflow-clipping), [Backgrounds and Borders 3](https://www.w3.org/TR/css-backgrounds-3/).

### [Important] The selector parser is not Selectors Level 3 complete

Location: `src/parser/selectors.rs:299-419`, `src/parser/selectors.rs:567-759`, and rejection cases around `src/tests.rs:3685-3749`.

The parser omits or rejects baseline forms including the universal selector, namespace-qualified selectors, `:link`, `:visited`, `:target`, `:lang()`, `::first-line`, and `::first-letter`. Namespace support is also blocked by the missing `@namespace` rule. Tests currently encode some valid selectors as expected failures.

Impact: valid Snapshot stylesheets fail at the selector boundary before any downstream matching concern arises.

Required remediation: implement the complete Selectors 3 syntax and authored AST distinctions, then add the Snapshot-approved Selectors 4 features as explicitly versioned extensions. Test tokenization-sensitive namespace, escape, pseudo-class, pseudo-element, and functional-pseudo cases.

Reference: [Selectors Level 3](https://www.w3.org/TR/selectors-3/).

### [Important] Media Queries Level 3 syntax is incomplete

Location: `src/parser/queries.rs:253-268`, `src/parser/queries.rs:336-456`, `src/parser/queries.rs:493-524`, and tests around `src/tests.rs:4980-5014`.

Only `all`, `screen`, and `print` media types are accepted. Valid Level 3 media types such as `tv` are rejected, and the feature grammar lacks required device features and boolean forms.

Impact: valid Snapshot media rules and import media lists are rejected despite media queries being part of the official baseline.

Required remediation: implement the complete Media Queries 3 type and feature grammar first, then layer later range/context syntax without weakening Level 3 validation. Add exhaustive feature arity, prefix, unit, boolean, comma-list, and malformed-query tests.

Reference: [Media Queries Level 3](https://www.w3.org/TR/mediaqueries-3/#media1).

### [Important] There is no style-attribute declaration-list entry point

Location: `src/lib.rs:17-19` and `src/parser/mod.rs`.

The only public parse front door consumes a stylesheet. CSS Style Attributes is part of the Snapshot definition, but callers cannot strictly parse the declaration-list syntax used by a `style` attribute without wrapping it in an invented rule and changing locations/context.

Impact: the crate cannot claim Snapshot-level authored CSS syntax coverage for one of the official CSS environments.

Required remediation: expose a strict, typed declaration-list/style-attribute parser sharing the same property parsers and diagnostic model, with tests for empty lists, semicolon handling, custom properties, `!important`, comments, and malformed trailing input.

Reference: [CSS Style Attributes](https://www.w3.org/TR/css-style-attr/#syntax).

### [Important] Ordinary declarations lose or reject `!important`

Location: `src/syntax.rs:1927-1971`, `src/parser/mod.rs:703-740`, `src/parser/mod.rs:956-958`, and `src/parser/variables.rs:42-52`.

`CssDeclaration` has no importance field. Ordinary declarations leave `!important` unconsumed and fail the whole declaration, while raw/custom-property paths can absorb the tokens into authored text rather than separating the declaration flag. `cssparser` requires declaration parsers to explicitly extract importance; the current implementation does not do so consistently.

Impact: valid CSS Cascade syntax is either rejected or misrepresented, and downstream cascade code cannot recover the authored importance bit reliably.

Required remediation: parse `!important` at the declaration boundary, store it as an explicit typed flag, exclude it from custom-property/raw value payloads, and test case-insensitivity, whitespace/comments, invalid suffixes, and both stylesheet and style-attribute contexts.

Reference: [Cascade 4 importance](https://www.w3.org/TR/css-cascade-4/#importance).

### [Important] Layer statements incorrectly make following imports invalid, and import conditions are incomplete

Location: `src/parser/mod.rs:100-103`, `src/parser/mod.rs:216-218`, and `src/tests.rs:936-947`.

The parser marks any prior `@layer` statement as ending the import phase, and a test explicitly rejects `@layer base; @import "late.css";`. Cascade 5 permits layer statements before imports and excludes them when determining whether an import is late. The import parser also rejects the valid `supports(...)` import condition.

Impact: the crate rejects valid ordering and conditions in a rule family it otherwise claims to support.

Required remediation: model the specification's import-order state precisely—distinguishing layer statements from other rules—and add typed import support conditions with strict nesting and fallback/media parsing.

Reference: [CSS Cascading and Inheritance Level 5](https://www.w3.org/TR/css-cascade-5/).

### [Important] Repeated ID selectors are silently overwritten

Location: `src/parser/selectors.rs:378-383`.

When a compound selector contains more than one ID selector, assigning the later ID overwrites the earlier one. Repeated ID selectors are syntactically valid and each occurrence contributes to the selector and its specificity; `#first#second` must not become merely `#second`.

Impact: successful parsing changes authored meaning and downstream specificity/matching behavior.

Required remediation: preserve ID selectors as an ordered collection, or reject them only if the adopted selector grammar actually forbids the form (Selectors does not). Add AST and specificity-facing regression tests for repeated identical and distinct IDs.

### [Important] Shared position parsing accepts invalid arities and keyword combinations

Location: `src/parser/background.rs:73-88` and `src/syntax.rs:5336-5346`.

The shared position model mainly enforces a non-empty, at-most-four component list and a limited duplicate-side check. It consequently accepts forms such as three naked lengths, four-component `transform-origin`, and repeated center components that do not match the owning property grammars.

Impact: strict parsing certifies invalid authored values, and reuse across background, mask, transform, and shape-related properties spreads the defect.

Required remediation: replace the generic component-count validator with property-specific grammar states for `<position>`, background-position layers, mask-position layers, and transform-origin, including valid edge-offset pairings and z-offset constraints.

### [Important] Generic function validators accept invalid transform, easing, shape, and filter syntax

Location: `src/parser/effects.rs:85-127`, `src/parser/effects.rs:221-337`, and `src/parser/effects.rs:427-463`.

Representative accepted-invalid forms include a comma-less `matrix(1 0 0 1 0 0)`, `steps(1, jump-none)`, `cubic-bezier()` with x coordinates outside `[0, 1]`, `circle(closest-corner)`, and `drop-shadow(inset 1px 2px)`. The validators mostly count broadly typed arguments without enforcing the grammar-specific separators, domains, and keyword sets.

Impact: multiple claimed properties violate the crate's strict-syntax guarantee.

Required remediation: give every function a dedicated grammar parser with exact separator handling and semantic token-domain checks that the syntax specification makes parse-time requirements. Add a table of one-token mutations around every valid vector.

References: [Easing Functions](https://www.w3.org/TR/css-easing-1/), [Transforms](https://www.w3.org/TR/css-transforms-1/).

### [Important] Time values conflate durations with delays and admit non-finite numbers

Location: `src/parser/timing.rs:29-55`, `src/parser/timing.rs:178`, `src/parser/timing.rs:279-298`, `src/parser/timing.rs:434`, `src/parser/mod.rs:939`, and `src/syntax.rs:6016-6036`.

The same non-negative `CssTime` path is used for durations and delays. Negative transition and animation delays are valid but are rejected. Conversely, exponent overflow such as `1e999s` is accepted as a non-finite `f32`; animation iteration-count has the same non-finite-number exposure.

Impact: valid timing syntax is rejected while invalid/unrepresentable numeric states enter the public AST.

Required remediation: split duration and delay grammar/domain types, reject every non-finite conversion, and use validated numeric wrappers for iteration counts. Cover negative zero, negative delay, overflow, NaN-impossible token paths, and shorthand/list interactions.

References: [CSS Transitions 1](https://www.w3.org/TR/css-transitions-1/), [CSS Animations 1](https://www.w3.org/TR/css-animations-1/).

### [Important] `calc()` and range handling contradict Values and Units Level 3

Location: `src/parser/values.rs:149-160`, `src/parser/values.rs:193-213`, `src/parser/values.rs:257-266`, and the explicit multiplication rejection near `src/tests.rs:4625`.

Bare negative box sizes can pass because the `BoxSize` grammar is excluded from the non-negative check. In the opposite direction, non-negative properties reject `calc()` merely because an authored component is negative, even though range restriction is applied after expression evaluation/computation rather than by scanning literal terms. The expression parser also implements only addition/subtraction and rejects valid product syntax such as `calc(10px * 2)`.

Impact: the parser both accepts invalid values and rejects valid Snapshot expressions, with behavior depending on spelling rather than the grammar's computed type.

Required remediation: represent math expressions with their typed grammar, support the Level 3 product/division rules, defer range restriction to the specification-defined phase, and enforce non-negativity for non-math literals in every applicable property grammar.

Reference: [CSS Values and Units Level 3](https://www.w3.org/TR/css-values-3/).

### [Important] Relative-color components are effectively untyped

Location: `src/parser/values.rs:598-629` and `src/parser/values.rs:661-683`.

Relative-color parsing accepts arbitrary identifiers and arbitrary dimensions as channel expressions. Inputs such as `rgb(from red bogus bogus bogus)` and channel values with unrelated units can therefore succeed even though the function's channel grammar defines a closed set of channel references, numbers/percentages, `none`, and math expressions.

Impact: a modern syntax family the crate claims to support is not strict and cannot provide trustworthy channel semantics downstream.

Required remediation: parameterize relative-color parsing by color space and channel slot, validate the allowed channel identifiers and numeric types for each function, and type `calc()` channel expressions against that environment.

Reference: [CSS Color Level 5 relative colors](https://drafts.csswg.org/css-color-5/#relative-colors).

### [Important] Grid `repeat()` validation permits forbidden nesting and flexible auto-repeat tracks

Location: `src/parser/grid.rs:62-80`, `src/parser/grid.rs:103-157`, `src/syntax.rs:3297-3405`, and tests around `src/tests.rs:9662-9677`.

The recursive track-list parser permits `repeat()` inside `repeat()`. It also accepts flexible tracks such as `1fr` in `auto-fill`/`auto-fit` repetitions where the auto-repeat grammar requires fixed-size tracks.

Impact: invalid Grid syntax is accepted and normalized into the same AST as conforming track lists.

Required remediation: use distinct AST/parser states for fixed repeat, auto repeat, fixed-size track lists, and general track lists; make nested repeat structurally unrepresentable.

Reference: [CSS Grid Layout](https://drafts.csswg.org/css-grid/).

### [Important] A malformed leading `@charset` bypasses strict rule parsing

Location: `src/parser/mod.rs:65-72` together with `cssparser 0.37.0`'s `StyleSheetParser` handling in `rules_and_declarations.rs:375-382`.

The dependency consumes the first `@charset` rule internally before delegating to this crate, regardless of the prelude's validity. As a result, malformed input such as `@charset bogus;` can be silently skipped and the remainder accepted.

Impact: the public whole-sheet parser can report success after discarding invalid leading syntax, directly violating its documented rejection contract.

Required remediation: validate the optional leading encoding declaration before constructing the dependency iterator, or use a parser path that exposes it. Test valid legacy spelling, malformed preludes, missing semicolons, comments/BOM interactions, and non-leading occurrences.

### [Important] Typography parsers accept invalid feature tags, indices, and global-keyword list members

Location: `src/parser/typography.rs:156-183`, `src/parser/typography.rs:396-422`, and `src/syntax.rs:4113-4152`.

`font-feature-settings` checks only character count for a four-character OpenType tag and accepts non-ASCII forms; it also accepts negative feature indices. `font-family` permits CSS-wide keywords as members of a comma-separated family list, even though those keywords apply only as the entire property value.

Impact: invalid authored font syntax is accepted and exposed as ordinary typed values.

Required remediation: validate feature tags against the four-ASCII-character grammar, enforce non-negative feature values, and separate whole-property global keywords before parsing family-list members. Add escaped-string, supplementary-character, signed-integer, and mixed-list tests.

Reference: [CSS Fonts Level 3](https://drafts.csswg.org/css-fonts-3/).

### [Important] Keyframe validation rejects valid duplicate and empty structures

Location: `src/syntax.rs:200-221`, `src/syntax.rs:407-418`, `src/parser/keyframes.rs:34-90`, and the duplicate-offset rejection in `src/tests.rs:3530-3548`.

The model rejects duplicate keyframe offsets across blocks and rejects empty keyframe structures/declaration blocks. CSS Animations allows multiple keyframe blocks at the same offset; their declarations cascade in source order. The rule-list and declaration-list grammars also admit empty lists.

Impact: valid animation stylesheets are rejected, and the AST cannot preserve a meaningful authored cascade case.

Required remediation: preserve duplicate blocks and source order, permit specification-valid empty lists, and leave declaration combination to the downstream cascade owner.

Reference: [CSS Animations Level 1 keyframes](https://www.w3.org/TR/css-animations-1/#keyframes).

### [Minor] The compatibility oracle is circular and currently classifies no standard property as known-unsupported

Location: `src/validation.rs:267-328`, especially `KNOWN_UNSUPPORTED_PROPERTY_NAMES`, and the support-list tests using the same local tables.

The coverage tests compare the parser's accepted set to the parser's own supported-property table. The known-unsupported registry is empty, so a Snapshot property absent from the allowlist is merely `Unknown`; no independent specification manifest makes the test fail when baseline syntax is missing.

Impact: tests can remain green while large, stable portions of CSS are absent, and diagnostics cannot tell a misspelled property from a recognized but deferred standard property.

Required remediation: create a versioned, source-linked Snapshot manifest independent of parser dispatch, record complete/partial/unsupported status and module level, and make coverage tests prove that every in-scope grammar item is deliberately classified.

### [Minor] Several public syntax types can represent values the parser would never accept

Location: public constructors/variants around `src/syntax.rs:3173`, `src/syntax.rs:3186`, `src/syntax.rs:7284`, and `src/syntax.rs:7920`.

Examples include `CssGridFlowTolerance::Percent(f32)` and raw-string selector variants such as `CssSelector::Tag(String)`, `Key(String)`, and `Class(String)`. Callers can construct negative/non-finite percentages or invalid token spellings directly, so the public type system does not preserve the parser's validity invariant.

Impact: downstream code must defensively revalidate allegedly typed syntax, and equality/debug behavior can encounter NaN or impossible identifiers.

Required remediation: use opaque validated wrappers and checked constructors, keep parser-only construction crate-private, and represent identifiers/tokens with types that establish their lexical invariants.

### [Minor] The declaration/value architecture has become a manually synchronized cross-product

Location: the design statement at `src/syntax.rs:1-7`, `CssDeclaration` at `src/syntax.rs:1927-1971`, the large `CssProperty`/`CssValue` enums beginning near `src/syntax.rs:2001` and `src/syntax.rs:2384`, and the manual dispatch beginning at `src/parser/mod.rs:770`.

The source explicitly says `CssValue` must not become a broad cross-property validation bag, but the implementation now synchronizes a large property enum, large value enum, validation table, and roughly one arm per property by hand. The declaration pair is only valid because parser dispatch maintains an external relationship that the types do not encode.

Impact: additions are easy to make inconsistently, partial support is hard to describe, and correctness depends on several distant tables staying aligned.

Required remediation: choose one authoritative property schema that generates or centrally declares name, typed value kind, completeness status, and parse function. Preserve property-specific types rather than growing an unchecked property/value cross-product.

### [Minor] The public API lacks item-level guidance and consumer-facing tests

Location: glob exports in `src/lib.rs:17-19`, the public types beginning at `src/syntax.rs:16`, `src/error.rs`, `src/parser/mod.rs`, `README.md`, and `src/tests.rs:1`.

The crate exports hundreds of public items with almost no item documentation, provides no end-to-end usage example, has no tracked external `tests/` suite, and runs zero doctests. Internal tests can access crate-private helpers and therefore do not establish that the public API is usable or sufficient for a consumer.

Impact: intended invariants, unsupported-vs-invalid behavior, source coordinate semantics, and exhaustive-match stability are unclear to users.

Required remediation: document the front door and public semantic types, add a minimal README/doctest example, and add external tests that use only public API for success, strict failure, authored preservation, and diagnostic inspection.

### [Minor] Source-location coordinates mix conventions without documenting them

Location: `src/error.rs:50-81`, `src/syntax.rs:1981-1989`, and the expectation near `src/tests.rs:5640`.

Dependency locations use zero-based lines and one-based UTF-16 columns. The crate forwards and displays these values without documenting either convention, producing values such as `(0, 1)` for the first character.

Impact: editors and diagnostic consumers can highlight the wrong row/column, particularly with supplementary Unicode characters.

Required remediation: document the exact coordinate basis and encoding or normalize both axes to one deliberate public convention. Add multiline and non-BMP regression tests.

### [Minor] Many negative tests permit diagnostic regressions

Location: shared rejection helpers around `src/test_support.rs:73-95` and broad alternative matches such as those around `src/tests.rs:4498`.

Several tests assert only that parsing failed, or accept either `InvalidSyntax` or `UnsupportedValue`, without checking the reason or location. That allows a valid unsupported feature to be accidentally reclassified as malformed syntax and lets an early unrelated failure satisfy the test.

Impact: the suite does not reliably protect the crate's error taxonomy or prove that the intended grammar boundary caused rejection.

Required remediation: make focused negative vectors assert exact error kind, stable reason category, and location; reserve broad failure-only helpers for fuzz/property tests where the cause is intentionally unconstrained.

### [Minor] The configured Clippy gate is currently red

Location: `src/syntax.rs:3636`.

`cargo --offline clippy --locked -p surgeist-css --all-targets -- -F unsafe-code -D warnings` fails on `clippy::question_mark`.

Impact: the repository does not satisfy its own documented verification inventory, despite check, tests, and formatting passing.

Required remediation: address the lint without suppressing the warning globally, then keep the exact configured command green.

### [Minor] Unsafe prohibition is not encoded at the crate root

Location: `src/lib.rs:1-22`.

The owned Rust source contains no `unsafe` token under the repository scan, and the configured Clippy command forbids unsafe code. The crate itself does not contain `#![forbid(unsafe_code)]`, so ordinary check/test builds do not enforce the invariant.

Impact: a future unsafe block can enter through a path that does not run the special Clippy flag.

Required remediation: add `#![forbid(unsafe_code)]` at the crate root and retain the repository-wide token scan and Clippy gate.

## Positive observations

- The documented crate boundary is coherent: authored syntax is kept separate from cascade application, substitution, resolution, selector matching, and resource loading.
- The parser consistently attempts whole-input exhaustion in many property-specific paths, and the test suite is already large enough to host focused conformance vectors.
- Owned Rust source is currently free of `unsafe` tokens.
- `cargo check`, all 289 unit tests, formatting, and doctests complete offline; no external acquisition was needed.

These observations do not offset the compatibility and strictness findings above, but they provide a useful base for remediation.

## Verification record

All commands were run from `/Users/codex/Development/surgeist-css` without network access or dependency acquisition.

| Command | Result |
| --- | --- |
| `cargo --offline metadata --locked --no-deps --format-version 1` | PASS; one library target, no declared features |
| `cargo --offline check --locked -p surgeist-css` | PASS |
| `cargo --offline test --locked -p surgeist-css` | PASS; 289 unit tests, 0 doctests |
| `cargo --offline clippy --locked -p surgeist-css --all-targets -- -F unsafe-code -D warnings` | FAIL; `clippy::question_mark` at `src/syntax.rs:3636` |
| `cargo fmt --check` | PASS |
| Repository-wide owned-Rust unsafe-token scan | PASS; no matches |

The worktree was clean before this review. The only intended review write is this report.

## Required next transition

Remediation is a multi-cycle, public-API-affecting effort and must enter the Surgeist canonical planning pipeline before product code changes. The first plan should establish an independent, versioned Snapshot 2026 conformance manifest and then sequence work in dependency order: shared lexical/numeric invariants; declaration importance and style-attribute parsing; Snapshot at-rules/selectors/media queries; property grammar completion; claimed post-Snapshot modules; public API hardening; and conformance/test-oracle enforcement.

No external-software blocker was identified. No implementation, commit, push, root integration, or cross-repository handoff was performed as part of this review.

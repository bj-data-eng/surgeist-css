# P01-I01 CSS Migration Record

This is the static, revision-independent migration record for the final I01 CSS
candidate. It is complete before task and holistic review so publication does
not require editing it. The canonical candidate handoff supplies all immutable
revision identifiers and publication evidence separately.

## Public API Migration

| Previous public contract | Candidate contract | Required caller migration |
| --- | --- | --- |
| `parse_sheet(&str) -> Result<CssSheet>` | `parse_sheet(&str) -> CssParseReport<CssSheet>` | Read retained syntax through `syntax()`, inspect `diagnostics()` or `is_clean()`, and use `into_parts()` when ownership of both values is needed. Do not infer cleanliness from an empty sheet. |
| No style-attribute entry point | `parse_style_attribute(&str) -> CssParseReport<CssDeclarationList>` | Parse authored `style` values directly; do not wrap them in a synthetic qualified rule. |
| Public crate alias `Result<T>` | No CSS parser result alias | Ordinary parsing is report-based. With `app-strict`, validators return the standard result type with `CssValidationFailure`. |
| `Error::message()`, `Error::line()`, and `Error::column()` | `Error::kind()`, `Error::code()`, and `Error::position()` | Branch on typed codes and detail payloads. Use display text only for people. Read coordinates from the semantic position types. |
| `CssSourceLocation` and `CssDeclaration::location()` | `CssSourcePosition` and `position()` on parser-produced nodes | Consume the byte offset, zero-based line index, and zero-based UTF-16 column index. |
| Raw declaration `line()` and `column()` accessors | `CssDeclaration::position()` | Read both axes from the returned position. The displayed position is one-based but typed accessors remain zero-based. |
| Independent `CssProperty` and `CssValue` declaration fields exposed by `property()` and `value()` | `CssDeclarationBody`, `CssKnownDeclaration`, `CssDeclaredValue<T>`, `CssCustomDeclaration`, and derived `property_name()` | Match the property-coupled known variant or custom branch; then inspect typed, global, or substitution-dependent views. Do not pair a property with an unrelated value type. |
| Custom and variable-dependent values mixed into the broad value enum | `CssCustomPropertyDeclaredValue`, `CssCustomPropertyValue`, and `CssSubstitutionDependentValue` | Preserve authored text through `as_css()` and defer substitution, dependency resolution, and post-substitution validation to their owning layer. |
| No declaration importance field | `CssImportance::{Normal, Important}` from `importance()` | Carry the authored importance bit into downstream cascade input. The annotation is not part of preserved authored value text. |
| Public construction of parser-owned aggregate states, including keyframe rules/blocks, font-face descriptor aggregates, typed media queries, declarations, sheets, and recovered media sentinels | Private parser construction with read-only accessors | Obtain these values from the ordinary parser. Continue using checked public constructors only for context-free semantic scalar or list types that expose such constructors. |
| Exhaustive matching on evolving public enums | All public enums except `CssImportance` and `CssSupportStatus` are non-exhaustive | Add a wildcard branch to downstream matches. The two closed enums may be matched exhaustively. |
| No support metadata query | `feature_catalog()`, `feature_metadata()`, and `property_metadata()` | Query the independent I01 catalog. Treat `Complete`, `Partial`, and `RecognizedUnsupported` as production-level metadata rather than parse-result validity. |

The two optional validators, `validate_sheet` and
`validate_style_attribute`, exist only with `app-strict`. Each accepts exactly a
clean ordinary report and otherwise returns all diagnostics in a non-empty
`CssValidationFailure`. Enabling the feature does not select a different parser
or grammar.

## Candidate Field Meanings

- `CssParseReport::syntax` is the valid retained authored tree after recovery;
  it is not proof that every source unit was accepted.
- `CssParseReport::diagnostics` contains every recovery diagnostic in
  first-responsible source order. `is_clean` means exactly that this slice is
  empty.
- `CssRecoveryDiagnostic::error` is the typed reason, `span` is the complete
  discarded, replaced, ignored, or implicitly closed unit, and `action` is the
  parser's recovery decision. These fields are related but not interchangeable.
- A source position contains an original-input UTF-8 byte offset, a zero-based
  line, and a zero-based UTF-16 column. A span starts inclusively and ends
  exclusively. Only a missing token or implicit end-of-input closure may use a
  zero-width span.
- `CssDeclaration::body` owns either a property-coupled known declaration or a
  custom declaration. `property_name` is derived from that body, `importance`
  owns the terminal annotation state, and `position` identifies the authored
  property name.
- `CssDeclaredValue::Value` is a property-specific authored value, `Global` is
  a whole-value CSS-wide keyword, and `SubstitutionDependent` preserves a value
  whose final grammar depends on substitution.
- A custom declaration preserves the case-sensitive custom-property name and
  either its authored token text or a whole-value CSS-wide keyword. The leaf
  does not apply cascade, substitute variables, or validate computed values.
- `CssMediaQuery::Never` is parser-owned recovered syntax for one malformed
  query-list member. It is guaranteed false, has exactly one corresponding
  replacement diagnostic, and is never produced by a clean parse.
- `CssFeatureMetadata` identifies one bounded parser-facing production. A
  partial record carries both its supported subset and its valid unsupported
  remainder. A recognized-unsupported record carries the emitted root error
  code, while the record ID supplies the diagnostic's stable feature identity.
- `CssPropertyMetadata` joins a catalog feature to its canonical generated
  property identity and aliases. Lookup is ASCII-case-insensitive for canonical
  names and aliases, and returns no record for custom or unknown names.

## Initiative Audit Trace

The focused `initiative_i01_audit_` tests name the stable predicate and finding
IDs in their test names and assertion failures. The broader evidence remains
independent and is listed here so each acceptance claim has an exact source and
test owner.

| Predicate or finding | Owning source | Passing evidence |
| --- | --- | --- |
| `P14.01` recovering front doors and all recovery boundaries | `src/parser/mod.rs`, `src/parser/recovery.rs`, and context parsers under `src/parser/` | `tests/initiative_i01_audit.rs`, `tests/stylesheet_recovery.rs`, `tests/style_attribute_recovery.rs`, `tests/nested_structural_recovery.rs`, `tests/specialized_list_recovery.rs`, and `tests/structural_recovery_adversarial.rs` |
| `P14.02` structured diagnostics, positions, spans, actions, and ordering | `src/error.rs`, `src/report.rs`, and `src/source.rs` | `tests/initiative_i01_audit.rs`, `tests/structured_errors.rs`, `tests/source_coordinates.rs`, and `tests/structural_recovery_adversarial.rs` |
| `P14.03` default/feature parity and one-pass strict wrappers | feature-gated validators in `src/lib.rs` and invocation boundary in `src/parser/mod.rs` | `tests/initiative_i01_audit.rs`, `tests/app_strict_parity.rs`, and the feature-gated crate tests in `src/lib.rs` |
| `P14.04` style attributes and declaration invariants | `src/properties.rs`, declaration types in `src/syntax.rs`, and the shared declaration core in `src/parser/mod.rs` | `tests/initiative_i01_audit.rs`, `tests/coupled_declarations.rs`, `tests/declaration_importance.rs`, `tests/authored_declaration_values.rs`, and `tests/property_schema.rs` |
| `P14.05` exact independent catalog and three-way evidence | `src/conformance.rs`, `src/properties.rs`, and the independent vector manifest in `tests/catalog_inventory/vectors.rs` | `tests/initiative_i01_audit.rs`, `tests/conformance_catalog.rs`, and `tests/catalog_inventory.rs` |
| `P14.06` all allocated historical findings closed without later-initiative claims | the exact finding-owned sources named in the rows below | all focused finding rows below plus the complete default and `app-strict` matrices |
| `P14.07` public guidance, doctests, and external consumers | crate documentation in `src/lib.rs`, `README.md`, and crate-root reexports | `tests/initiative_i01_audit.rs`, `tests/public_surface.rs`, default/feature doctests, and warning-denied rustdoc |
| `P14.08` no unsafe, unchanged dependency/feature shape, and clean configured verification | `Cargo.toml` and crate-root prohibition in `src/lib.rs` | `tests/initiative_i01_audit.rs`, both warning-denied Clippy configurations, format, dependency-offline builds, and the complete owned-Rust unsafe scan |
| `P14.09` immutable published candidate and root handoff | this static migration record and the canonical publication workflow | task review, holistic review, final matrix, publication readback, and the separate canonical candidate handoff; publication state is intentionally not encoded in a product test |
| `F2.5` style-attribute entry point | `src/parser/mod.rs` and crate-root reexport | `tests/initiative_i01_audit.rs` and `tests/style_attribute_recovery.rs` |
| `F2.6` declaration importance | declaration core in `src/parser/mod.rs` and declaration types in `src/syntax.rs` | `tests/initiative_i01_audit.rs` and `tests/declaration_importance.rs` |
| `F2.15` leading encoding handling | sheet preflight in `src/parser/mod.rs` | `tests/initiative_i01_audit.rs` and the encoding matrix in `tests/stylesheet_recovery.rs` |
| `F2.18` non-circular compatibility evidence | `src/conformance.rs` and independent implementation inventory in `src/properties.rs` | `tests/initiative_i01_audit.rs`, `tests/conformance_catalog.rs`, and `tests/catalog_inventory.rs` |
| `F2.19` parser-produced invalid states | private fields and parser-owned constructors in `src/syntax.rs` | `tests/initiative_i01_audit.rs`, `tests/public_surface.rs`, and compile-fail doctests for private construction |
| `F2.20` property/value cross-product | schema in `src/properties.rs` and coupled declarations in `src/syntax.rs` | `tests/initiative_i01_audit.rs`, `tests/coupled_declarations.rs`, and `tests/property_schema.rs` |
| `F2.21` public guidance and consumer tests | `src/lib.rs` and `README.md` | `tests/initiative_i01_audit.rs`, `tests/public_surface.rs`, and default/feature doctests |
| `F2.22` coordinate convention | `src/source.rs` | `tests/initiative_i01_audit.rs`, `tests/source_coordinates.rs`, and non-BMP public-consumer vectors |
| `F2.23` precise negative diagnostics | `src/error.rs`, `src/report.rs`, and recovery contexts | `tests/initiative_i01_audit.rs`, `tests/structured_errors.rs`, and all focused recovery matrices |
| `F2.24` configured Clippy gate | lint-clean owned Rust source | both configured warning-denied Clippy commands |
| `F2.25` crate-root unsafe prohibition | `src/lib.rs` | crate-root attribute check, both compiler-enforced Clippy commands, and the complete owned-Rust scan |

## Audit Artifact Contract

The manifests below declare exact evidence identities. They do not claim that a
command, review, publication, or remote readback has already succeeded. The leaf
coordinator executes every command after task and holistic review, records each
result in the canonical candidate handoff, and then performs publication and
fresh remote readback under the canonical publication gate. This static record
remains unchanged throughout those later transitions.

### Configured Command Evidence Manifest V1
```text
P14.07.PUBLIC-SURFACE|cargo test -p surgeist-css --offline --test public_surface
P14.07.DOCTEST.DEFAULT|cargo test -p surgeist-css --offline --no-default-features --doc
P14.07.DOCTEST.APP-STRICT|cargo test -p surgeist-css --offline --no-default-features --features app-strict --doc
P14.07.RUSTDOC.DEFAULT|RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --no-deps
P14.07.RUSTDOC.APP-STRICT|RUSTDOCFLAGS='-D warnings' cargo doc -p surgeist-css --offline --no-default-features --features app-strict --no-deps
P14.08.CHECK.DEFAULT|cargo check -p surgeist-css --offline --no-default-features
P14.08.TEST.DEFAULT|cargo test -p surgeist-css --offline --no-default-features
P14.08.CHECK.APP-STRICT|cargo check -p surgeist-css --offline --no-default-features --features app-strict
P14.08.TEST.APP-STRICT|cargo test -p surgeist-css --offline --no-default-features --features app-strict
P14.08.CONFORMANCE-CATALOG|cargo test -p surgeist-css --offline --test conformance_catalog
P14.08.CATALOG-INVENTORY|cargo test -p surgeist-css --offline --test catalog_inventory
P14.08.INITIATIVE-AUDIT|cargo test -p surgeist-css --offline initiative_i01_audit_
P14.08.FORMAT|cargo fmt --check
F2.24.CLIPPY.DEFAULT|cargo clippy -p surgeist-css --offline --no-default-features --all-targets -- -F unsafe-code -D warnings
F2.24.CLIPPY.APP-STRICT|cargo clippy -p surgeist-css --offline --no-default-features --features app-strict --all-targets -- -F unsafe-code -D warnings
F2.25.CRATE-ROOT-PROHIBITION|rg -n '^#!\[forbid\(unsafe_code\)\]$' src/lib.rs
F2.25.OWNED-RUST-SCAN|! rg -n --glob '*.rs' 'unsafe[[:space:]]*(\{|fn|trait|impl|extern)|#!?\[[[:space:]]*unsafe|#!?\[[^]]*(allow|expect)\(unsafe_code\)' .
P14.09.DIFF-CHECK|git diff --check
P14.09.PROCESS-CHECK.BEFORE-CLEAN|! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'
P14.09.CLEAN|cargo clean
P14.09.TARGET-ABSENT|test ! -d target
P14.09.PROCESS-CHECK.AFTER-CLEAN|! pgrep -f '/Users/codex/Development/surgeist-css/target/(debug|release)/'
P14.09.STATUS|git status --short
```

The exact command text is part of this migration artifact's contract. Omitting
an identity, changing a flag, or substituting a broader summary invalidates the
artifact. The command manifest records required future execution; it is not a
stored result and does not replace fresh coordinator evidence.

## Root-Owned Follow-Up

After receiving the canonical immutable candidate handoff, root owns every
integration mutation:

### Root Follow-Up Manifest V1
```text
ROOT.01.PUBLISHED-CANDIDATE|Verify the selected candidate is reachable from the leaf authority's published main, check it against root's committed MSRV, and deliberately update the crates/surgeist-css gitlink.
ROOT.02.FACADE-ADAPTERS|Migrate facade reexports and CSS-to-Surgeist adapters to reports, structured diagnostics, semantic positions, and property-coupled declarations.
ROOT.03.FEATURE-FORWARDING|Decide root feature forwarding for app-strict while keeping ordinary parser access report-based.
ROOT.04.AUTHORED-VALUES|Preserve custom and substitution-dependent authored values until the root-owned cascade/substitution layer resolves them, and carry declaration importance into root-owned cascade input.
ROOT.05.API-ARTIFACTS|Run root's committed API generator and update only root-owned API audit artifacts.
ROOT.06.DOCUMENTATION|Update root documentation and examples for retained syntax, diagnostics, clean reports, coordinates, style attributes, and the removal of whole-sheet rejection semantics.
ROOT.07.INTEGRATION-TESTS|Cover clean and recovered sheets and style attributes, exact diagnostics, non-BMP coordinates, importance, authored-value preservation, property coupling, metadata lookup, and forwarded strict validation.
ROOT.08.ROOT-GATES|Run root's complete workspace, feature, lint, format, API-artifact, dependency, MSRV, unsafe, and publication gates before reporting promotion.
```

Root owns the pointer, facade, adapters, generated API artifacts, root docs, and
root integration tests. The leaf candidate must not edit any of them.

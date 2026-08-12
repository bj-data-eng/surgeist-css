# surgeist-css

Browser-recovering CSS ingestion for Surgeist. This crate parses CSS-facing input into CSS-owned authored syntax contracts. Root-owned Surgeist adapters lower parsed CSS syntax into typed style data.

The ordinary `parse_sheet` and `parse_style_attribute` entry points use browser recovery: each returns a report containing valid retained syntax and every structured recovery diagnostic in source order. Unsupported or malformed source units are dropped, replaced, retained with an implicit closure, ignored, or stopped at the documented boundary; valid siblings remain eligible. A clean report means that no recovery diagnostic was produced. An empty retained tree alone does not establish that the source was clean, so consumers should inspect `is_clean()` or `diagnostics()` rather than infer validity from syntax length.

```rust
use surgeist_css::{CssRecoveryAction, CssRule, parse_sheet};

let report = parse_sheet(
    ".before { color: red; } @unknown value; .after { color: blue; }",
);
assert_eq!(report.syntax().rules().len(), 2);
assert!(matches!(report.syntax().rules()[0], CssRule::Style(_)));
assert_eq!(
    report.diagnostics()[0].action(),
    CssRecoveryAction::DropAtRule,
);
```

Style attributes use the same ordinary declaration parser as style-rule blocks. Declarations retain their authored order, source position, coupled property/value type, and terminal `!important` state.

```rust
use surgeist_css::{CssImportance, CssPropertyNameRef, parse_style_attribute};

let report = parse_style_attribute("color: red; mystery: 1; width: 2px !important");
assert_eq!(report.syntax().len(), 2);
assert_eq!(report.diagnostics().len(), 1);
let width = &report.syntax()[1];
assert_eq!(width.importance(), CssImportance::Important);
assert!(matches!(width.property_name(), CssPropertyNameRef::Known(_)));
```

Each diagnostic exposes a typed error and stable root code, the first responsible source position, the complete recovery-unit span, and one `CssRecoveryAction`. Source byte offsets index the original UTF-8 input; line and column indices are zero-based, and columns count UTF-16 code units. Display text is for people, not control flow—match typed variants with a wildcard for future non-exhaustive cases.

CSS custom properties preserve case-sensitive names and authored value text, including interior trivia. Known-property values whose grammar depends on `var(...)` remain substitution-dependent authored values. The crate recognizes terminal `!important` but does not apply cascade or perform custom-property substitution or post-substitution validation.

The independent support catalog reports an exact support status for each bounded I01 production: `Complete`, `Partial`, or `RecognizedUnsupported`. Partial records document both the accepted subset and valid-but-unsupported remainder. A clean use of a partial production's supported subset is accepted; status is metadata about the whole named production, not a parse-result validity flag.

Enable the additive `app-strict` feature to expose `validate_sheet` and `validate_style_attribute`. Each validator runs the ordinary parser once, returns retained syntax only for a clean report, and otherwise returns the complete non-empty diagnostic sequence. Enabling the feature does not change ordinary parsing or recovery.

This crate owns authored CSS syntax, intrinsic grammar validation, recovery boundaries, diagnostics, and support metadata. It does not apply cascade or inheritance, substitute or resolve variables, evaluate queries, match selectors, resolve URLs or resources, perform layout or painting, serialize a CSSOM, or lower CSS into sibling Surgeist types. Root-owned integration owns cross-crate lowering and generated API audit artifacts.

CSS custom properties are parsed as authored syntax. Custom property names are case-sensitive, `var(...)` references and fallback token text remain symbolic, and supported properties containing `var(...)` parse as variable-dependent authored values. This crate does not resolve variables, run cascade substitution, or validate post-substitution values.

Colors are parsed as authored CSS color syntax. `surgeist-css` accepts named colors, alpha hex, modern color functions, `currentcolor`, system colors, `color-mix()`, and relative color syntax as typed syntax, but does not resolve system colors, substitute variables, evaluate relative channels, mix colors, convert color spaces, or adapt colors to a renderer gamut. Downstream style/render layers own those resolution steps.

Pseudo-classes for UI interaction, form state, structure, selector-list filtering, and overlay state are parsed as authored selector syntax. This crate does not evaluate pseudo-class matches; runtime matching belongs to downstream Surgeist layers with node and interaction state.

Selector-list pseudo-class arguments are parsed as authored selector syntax with bounded recovery. In recognized `:is()` and `:where()` lists, an invalid member is dropped with `DropSelectorListItem` while the other members remain in authored order. Other selector lists are unforgiving: `:not()` preserves supported complex selector lists, `:has()` preserves supported relative selector lists including leading child and sibling combinators, and `:nth-child()` / `:nth-last-child()` preserve optional `of` selector filters, but an invalid member causes the containing qualified rule to be dropped with `DropQualifiedRule`. Later sibling rules remain eligible for parsing.

Media queries are parsed as authored conditions on `@media` group rules. `surgeist-css` does not evaluate media query matches; environment-dependent matching belongs to downstream Surgeist layers.

Container queries are parsed as authored conditions on `@container` group rules. `surgeist-css` does not evaluate container query matches; container-dependent matching belongs to downstream Surgeist layers.

Imports are parsed as authored `@import` contracts only. `surgeist-css` preserves import targets, layer clauses, and media conditions, but does not resolve paths, load files, or merge imported sheets; root/style-owned Surgeist integration performs loading and composition.

Cascade layers are parsed as authored `@layer` statements and blocks, including named and anonymous layer blocks. `surgeist-css` records layer names and layer-contained rules, but does not compute cascade order, declaration precedence, or runtime cascade effects.

Scoped styles are parsed as authored `@scope` rules with optional roots, limits, scoped style selectors, and scoped nested group rules. Relative scoped selectors remain structurally distinct from ordinary selectors. `surgeist-css` does not perform scope matching, selector matching, or scoping proximity calculations.

Pseudo-elements are parsed as terminal authored selector syntax for the supported `::before`, `::after`, `::marker`, `::selection`, and `::backdrop` forms. The parser records them on selector compounds, but does not filter declarations by pseudo-element or perform generated box/layout behavior.

Generated content, list markers, and counters are parsed as typed authored property values for `content`, list-style longhands and shorthand, and counter change properties. Strings, URLs, attribute references, quote keywords, counter functions, list-style slots, and counter change lists remain symbolic. `surgeist-css` does not lay out generated content or list markers, resolve marker images, or evaluate/reset/increment counters.

Font faces are parsed as authored `@font-face` descriptor blocks only. `surgeist-css` validates supported descriptors and preserves font source hints, unicode ranges, and variation ranges, but does not perform font lookup, loading, matching, or resource validation; downstream Surgeist layers own those steps.

Keyframes are parsed as authored `@keyframes` rules. `surgeist-css` validates keyframe names, selector offsets, and declarations, but does not evaluate animations, match animation names to rules, interpolate values, or run animation timelines.

CSS nesting is parsed as syntax sugar and flattened into ordinary style and conditional group rules while preserving source order. `surgeist-css` does not evaluate selector matches or cascade results during flattening.

# surgeist-css

Strict CSS ingestion for Surgeist. This crate parses CSS-facing input into CSS-owned syntax contracts. Root-owned Surgeist adapters lower parsed CSS syntax into typed style data.

Unlike a browser CSS parser, this crate does not recover from invalid application CSS or silently drop bad declarations. Unsupported selectors, at-rules, properties, values, malformed lists, and invalid rule bodies reject the whole sheet.

CSS custom properties are parsed as authored syntax. Custom property names are case-sensitive, `var(...)` references and fallback token text remain symbolic, and supported properties containing `var(...)` parse as variable-dependent authored values. This crate does not resolve variables, run cascade substitution, or validate post-substitution values.

Pseudo-classes for UI interaction, form state, structure, selector-list filtering, and overlay state are parsed as authored selector syntax. This crate does not evaluate pseudo-class matches; runtime matching belongs to downstream Surgeist layers with node and interaction state.

Media queries are parsed as authored conditions on `@media` group rules. `surgeist-css` does not evaluate media query matches; environment-dependent matching belongs to downstream Surgeist layers.

Container queries are parsed as authored conditions on `@container` group rules. `surgeist-css` does not evaluate container query matches; container-dependent matching belongs to downstream Surgeist layers.

Imports are parsed as authored `@import` contracts only. `surgeist-css` preserves import targets, layer clauses, and media conditions, but does not resolve paths, load files, or merge imported sheets; root/style-owned Surgeist integration performs loading and composition.

Font faces are parsed as authored `@font-face` descriptor blocks only. `surgeist-css` validates supported descriptors and preserves font source hints, unicode ranges, and variation ranges, but does not perform font lookup, loading, matching, or resource validation; downstream Surgeist layers own those steps.

Keyframes are parsed as authored `@keyframes` rules. `surgeist-css` validates keyframe names, selector offsets, and declarations, but does not evaluate animations, match animation names to rules, interpolate values, or run animation timelines.

CSS nesting is parsed as syntax sugar and flattened into ordinary style and conditional group rules while preserving source order. `surgeist-css` does not evaluate selector matches or cascade results during flattening.

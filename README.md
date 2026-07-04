# surgeist-css

Strict CSS ingestion for Surgeist. This crate parses CSS-facing input into CSS-owned syntax contracts. Root-owned Surgeist adapters lower parsed CSS syntax into typed style data.

Unlike a browser CSS parser, this crate does not recover from invalid application CSS or silently drop bad declarations. Unsupported selectors, at-rules, properties, values, malformed lists, and invalid rule bodies reject the whole sheet.

CSS custom properties are parsed as authored syntax. Custom property names are case-sensitive, `var(...)` references and fallback token text remain symbolic, and supported properties containing `var(...)` parse as variable-dependent authored values. This crate does not resolve variables, run cascade substitution, or validate post-substitution values.

Pseudo-classes for UI interaction, form state, structure, selector-list filtering, and overlay state are parsed as authored selector syntax. This crate does not evaluate pseudo-class matches; runtime matching belongs to downstream Surgeist layers with node and interaction state.

Media queries are parsed as authored conditions on `@media` group rules. `surgeist-css` does not evaluate media query matches; environment-dependent matching belongs to downstream Surgeist layers.

Imports are parsed as authored `@import` contracts only. `surgeist-css` preserves import targets, layer clauses, and media conditions, but does not resolve paths, load files, or merge imported sheets; root/style-owned Surgeist integration performs loading and composition.

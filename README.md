# surgeist-css

Strict CSS ingestion for Surgeist. This crate parses CSS-facing input into CSS-owned syntax contracts. Root-owned Surgeist adapters lower parsed CSS syntax into typed style data.

Unlike a browser CSS parser, this crate does not recover from invalid application CSS or silently drop bad declarations. Unsupported selectors, at-rules, properties, values, malformed lists, and invalid rule bodies reject the whole sheet.

CSS custom properties are parsed as authored syntax. Custom property names are case-sensitive, `var(...)` references and fallback token text remain symbolic, and supported properties containing `var(...)` parse as variable-dependent authored values. This crate does not resolve variables, run cascade substitution, or validate post-substitution values; unsupported pseudo-classes beyond `:root` still reject the sheet.

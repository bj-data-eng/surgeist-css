# surgeist-css

Strict CSS ingestion for Surgeist. This crate parses CSS-facing input into CSS-owned syntax contracts. Root-owned Surgeist adapters lower parsed CSS syntax into typed style data.

Unlike a browser CSS parser, this crate does not recover from invalid application CSS or silently drop bad declarations. Unsupported selectors, at-rules, properties, values, malformed lists, and invalid rule bodies reject the whole sheet.

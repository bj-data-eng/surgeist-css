# P01-I02-C12 CSS2 Residual, Writing, UI, Containment, And Compositing

## Boundary

`surgeist-css` owns the selected strict authored property grammars, typed values,
legacy shorthand mapping, source coordinates, declaration recovery, conformance
metadata, and public parser evidence. It does not apply cascade or substitution;
resolve layout, pagination, painting, hit testing, writing modes, containment, or
blending; load resources; serialize CSSOM; or lower syntax into another Surgeist
crate.

## Public Authored Surface

The complete C12 surface consists of thirteen CSS2 residual canonicals;
Writing Modes 3 `text-combine-upright`, `text-orientation`, and `unicode-bidi`;
UI3 `caret-color`, `outline-offset`, and `resize`; Containment 1 `contain`;
Transforms 1 `transform-box`; and Compositing 1 `background-blend-mode`,
`isolation`, and `mix-blend-mode`.

Every canonical uses its private-field `Css<Property>PropertyValue` wrapper and
typed current accessor. Ordinary values, whole-property globals, and
substitution-dependent values remain distinct declaration branches. Valid
duplicates retain authored order. Invalid values identify the responsible
property and smallest declaration recovery unit while later siblings remain
eligible.

`glyph-orientation-vertical` is an explicit legacy shorthand with its own
restricted grammar and metadata. It maps `auto`, `0`, `0deg`, `90`, and `90deg`
to parser-produced `text-orientation` values. It is not present in
`CssKnownProperty::TextOrientation.aliases()` and does not use the schema's
name-equivalent alias path.

## Complete Official Metadata

Every row below is a public `Complete` atomic record with no supported subset,
unsupported remainder, recognized-unsupported code, or aggregate-alias target.

| Stable ID | Source | Exact fragment |
| --- | --- | --- |
| `official.property.border-collapse` | `O-CSS2` | `tables.html#propdef-border-collapse` |
| `official.property.border-spacing` | `O-CSS2` | `tables.html#propdef-border-spacing` |
| `official.property.caption-side` | `O-CSS2` | `tables.html#propdef-caption-side` |
| `official.property.clip` | `O-CSS2` | `visufx.html#propdef-clip` |
| `official.property.empty-cells` | `O-CSS2` | `tables.html#propdef-empty-cells` |
| `official.property.orphans` | `O-CSS2` | `page.html#propdef-orphans` |
| `official.property.page-break-after` | `O-CSS2` | `page.html#propdef-page-break-after` |
| `official.property.page-break-before` | `O-CSS2` | `page.html#propdef-page-break-before` |
| `official.property.page-break-inside` | `O-CSS2` | `page.html#propdef-page-break-inside` |
| `official.property.quotes` | `O-CSS2` | `generate.html#propdef-quotes` |
| `official.property.table-layout` | `O-CSS2` | `tables.html#propdef-table-layout` |
| `official.property.widows` | `O-CSS2` | `page.html#propdef-widows` |
| `official.property.word-spacing` | `O-CSS2` | `text.html#propdef-word-spacing` |
| `official.property.text-combine-upright` | `O-WRITING3` | `#propdef-text-combine-upright` |
| `official.property.text-orientation` | `O-WRITING3` | `#propdef-text-orientation` |
| `official.property.unicode-bidi` | `O-WRITING3` | `#propdef-unicode-bidi` |
| `official.property-alias.glyph-orientation-vertical` | `O-WRITING3` | `#propdef-glyph-orientation-vertical` |
| `official.property.caret-color` | `O-UI3` | `#propdef-caret-color` |
| `official.property.outline-offset` | `O-UI3` | `#propdef-outline-offset` |
| `official.property.resize` | `O-UI3` | `#propdef-resize` |
| `official.property.contain` | `O-CONTAIN1` | `#propdef-contain` |
| `official.property.transform-box` | `O-TRANSFORMS1` | `#propdef-transform-box` |
| `official.property.background-blend-mode` | `O-COMPOSITING1` | `#propdef-background-blend-mode` |
| `official.property.isolation` | `O-COMPOSITING1` | `#propdef-isolation` |
| `official.property.mix-blend-mode` | `O-COMPOSITING1` | `#propdef-mix-blend-mode` |
| `official.value.box-edge-keywords` | `O-BOX3` | `#keywords` |
| `official.value.blend-mode` | `O-COMPOSITING1` | `#blending,#blendingseparable,#blendingnonseparable` |

The two shared-value records remain independent coverage units. The box-edge
record owns the seven selected edge keywords; the blend-mode record owns the full
sixteen-keyword domain used by the single-value and comma-list consumers.

## Inventory And Exclusion Reconciliation

The immutable official inventory remains exactly 162 property units: 161
canonical properties plus the custom-property declaration family. C12 activates
24 canonical property rows and one normative legacy shorthand; it does not
promote the background/image, multicolumn, or flexbox work allocated to later
cycles. The non-property inventory remains exactly 167 units, of which C12
activates the two independent shared-value rows.

The public exclusion registry remains exactly 131 rows. Its property-facing
partition still contains 50 superseded CSS2 property definitions, 20 informative
CSS2 Appendix A properties, and the two current-production-less
`glyph-orientation-horizontal` and `ime-mode` spellings. C12 does not replace an
exclusion with metadata, duplicate a superseded predecessor, or claim excluded
downstream semantics.

## Public Vector And Recovery Evidence

`tests/catalog_inventory/vectors.rs` supplies independent ordinary positive and
property-specific negative vectors for all 24 canonical properties.
`tests/conformance_catalog.rs` names every C12 ID, source, fragment, spelling,
kind, status, and public parser stimulus, including the explicit shorthand and
both shared-value records. The family-specific public tests cover keyword and
numeric boundaries, list separators and arity, globals and substitutions,
adjacent-grammar rejection, exact diagnostics, duplicates, EOF, non-BMP source
coordinates, repeated failures, depth boundaries, sibling retention, and
ordinary/`app-strict` parity.

The frozen product fixture is unchanged. Its SHA-256 digest before and after C12
is `7c2cf7d79368d76d94cc0b383be70cc404d4c69d7caa72eedba6f0762e0b2356`.
No fixture row is replaced, weakened, or masked by this cycle.

## Root-Owned Follow-Up

Root `surgeist` owns the integration work:

1. select and promote the intended `surgeist-css` gitlink;
2. expose the C12 property/value types, the additive
   `CssFeatureKind::PropertyAlias` variant, and all 27 metadata records through
   the facade;
3. preserve the authored-syntax boundary and keep cascade, substitution,
   layout, pagination, painting, hit testing, containment, blending, writing-mode
   resolution, CSSOM, and cross-crate lowering outside this leaf;
4. refresh root-owned generated API audit artifacts with the root generator;
5. update root integration tests and documentation for the public types,
   explicit shorthand mapping, recovery distinctions, official metadata, and
   exact exclusions.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and generated API artifacts. This leaf handoff does not authorize
changes in root or sibling repositories.

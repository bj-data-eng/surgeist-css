# P01-I02-C14 Flexbox, Multicolumn, And Initiative Closure

## Boundary

`surgeist-css` owns strict authored Flexbox 1 `flex-flow`, all nine
Multicolumn 1 properties, generic Syntax 3 authored shells, the selected
Values 3 grammars, public metadata, and recovery. It does not apply cascade or
substitution; evaluate selectors or queries; load resources; perform layout,
pagination, painting, or animation; serialize CSSOM; or lower syntax into
another Surgeist crate.

## Public Surface And Reserved Closures

C14 closes all 31 records that entered the cycle as `Reserved` by adding them
to the public catalog as `Complete` metadata. The first 17 are ten property
records and seven generic shell records. The other 14 are official shared-value
records. This distinction preserves the exact catalog-cardinality
reconciliation below.

C14 activates these ten property records as public `Complete` metadata:
`official.property.flex-flow`, `official.property.column-count`,
`official.property.column-fill`, `official.property.column-rule`,
`official.property.column-rule-color`, `official.property.column-rule-style`,
`official.property.column-rule-width`, `official.property.column-span`,
`official.property.column-width`, and `official.property.columns`.

It activates seven public `Complete` non-property records:
`official.rule.at-rule`, `official.qualified-rule.generic`,
`official.declaration.generic`, `official.value.stylesheet`,
`official.value.rule-list`, `official.value.declaration-list`, and
`official.value.style-block`.

C14 closes these fourteen formerly `Reserved` official shared-value records as
public `Complete` metadata: `official.value.syntax-token-stream`,
`official.value.component-value`, `official.value.simple-block`,
`official.value.function`, `official.value.declaration-value`,
`official.value.any-value`, `official.value.an-plus-b`,
`official.value.unicode-range`, `official.value.css-wide-keyword`,
`official.value.custom-ident`, `official.value.ident`,
`official.value.string`, `official.value.url`, and
`official.value.url-modifier`.

## Partial Promotions And Retained Boundaries

C14 promotes these seven existing Values 3 records from `Partial` to
`Complete`: `official.value.dimension`, `official.value.angle`,
`official.value.angle-percentage`, `official.value.time-percentage`,
`official.value.frequency`, `official.value.frequency-percentage`, and
`official.value.calc`.

The following preserved extension records remain `Partial`, each with explicit
supported-subset and unsupported-remainder metadata:

- `ext.value.relative-color`
- `ext.value.color-mix`
- `ext.value.grid-repeat`
- `ext.value.basic-shape`
- `ext.descriptor.font-weight-range`
- `ext.descriptor.font-style-oblique-range`
- `ext.descriptor.font-stretch-range`
- `ext.value.font-source-modern-hints`
- `ext.property.font-weight-range`
- `ext.supports.selector`
- `ext.media.range.width`
- `ext.media.range.height`
- `ext.media.range.resolution`
- `ext.media.range.color`
- `ext.media.range.monochrome`

`later.rule.font-feature-values` remains `RecognizedUnsupported` with its typed
unsupported-at-rule diagnostic. These retained boundaries are not initiative
closure gaps.

All stable IDs retain their previous meaning. The additions are additive: no
public type, stable ID, dependency, feature, or manifest entry is renamed or
removed. C13 ended with exactly 456 public catalog records. C14 adds all 31
formerly `Reserved` records—17 shell/property records plus 14 shared-value
records—so the public support catalog contains exactly 487 records. Promoting
the seven existing Partial records does not change cardinality.

## Exact Inventory And Exclusions

Catalog cardinality is distinct from the inventories below. The immutable
official inventory reconciles to exactly 162 property units: 161
canonical properties plus the `--*` custom-property family. The one normative
`glyph-orientation-vertical` legacy shorthand remains separate. The
non-property inventory reconciles to exactly 167 units. Together those official
inventories account for the complete selected source ledger without changing
unit identity. All 219 preserved I01 feature records retain their
classifications.

The public exclusion registry reconciles to exactly 131 rows. No exclusion was
added, removed, or reclassified. The registry still contains the 50 superseded
CSS2 property definitions, 20 informative CSS2 Appendix A properties, and the
two current-production-less `glyph-orientation-horizontal` and `ime-mode`
spellings. The remaining 59 rows continue to exclude the selected official
source areas outside authored syntax, including Flexbox and Multicolumn layout,
pagination, and rendering algorithms; the Flexbox WebKit legacy aliases;
Syntax 3 fragment APIs and serialization; and other downstream loading,
evaluation, matching, cascade, CSSOM, and rendering concerns.

## Product Fixture

The I01 observable fixture is unchanged. Its SHA-256 digest is
`7c2cf7d79368d76d94cc0b383be70cc404d4c69d7caa72eedba6f0762e0b2356`.
No fixture row was replaced, weakened, or masked.

## Root-Owned Follow-Up

Root `surgeist` owns the gitlink selection, facade composition, cross-crate
adapters, integration tests and documentation, and generated API audit
artifacts. Root must expose the additive typed values and public metadata while
preserving the authored-symbolic boundary and the exact inventory and exclusion
facts above. This leaf handoff does not authorize root or sibling edits.

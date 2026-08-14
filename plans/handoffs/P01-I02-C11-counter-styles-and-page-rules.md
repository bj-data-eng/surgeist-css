# P01-I02-C11 Counter Styles And Page Rules

## Boundary

`surgeist-css` owns strict authored Counter Styles 3 definitions, CSS2 page
rules and pseudo-page selectors, their public syntax models, intrinsic grammar
validation, structured recovery, source coordinates, and conformance metadata.
It does not register, inherit, resolve, or evaluate counter styles; render list
markers; paginate; match pages; apply page cascade; support page margin boxes;
load resources; serialize CSSOM; or lower syntax into another Surgeist crate.

## Counter-Style Surface And Recovery

`CssRule::CounterStyle` exposes a parser-produced `CssCounterStyleRule` with a
checked case-sensitive `CssCounterStyleName`, typed
`CssCounterStyleDescriptors`, and position. Valid descriptor occurrences remain
in authored order, while the named accessors select the effective last valid
`system`, `negative`, `prefix`, `suffix`, `range`, `pad`, `fallback`, `symbols`,
`additive-symbols`, or `speak-as` occurrence.

The typed domains include fixed and symbolic systems, symbolic `extends`
references, one- or two-symbol negative forms, infinite and comma-separated
ranges, nonnegative padding, checked fallback names, nonempty symbol lists,
strictly descending nonnegative additive weights, and the complete selected
speech choices. Reserved names are rejected. Individual invalid or unknown
descriptors recover with `DropDescriptor`; an invalid effective descriptor
combination drops the at-rule. Malformed, statement-form, nested, EOF, and depth
boundary cases preserve the smallest established recovery unit and later valid
siblings.

## Page Surface And Recovery

`CssRule::Page` exposes a parser-produced `CssPageRule` with an optional
`CssPageSelector::{Left, Right, First}`, ordered valid declarations, and
position. Absence of a selector represents the authored default page form.

Page bodies accept only `margin` and the four margin longhands with CSS2
absolute lengths, percentages, `auto`, zero, and negative values; `em` and `ex`
are excluded. Known non-margin, unknown, and invalid margin declarations retain
their distinct typed diagnostics and recover with `DropDeclaration`. Page rules
are top-level block rules. Invalid selectors, statement forms, nested placement,
margin-box rules, EOF, and depth boundaries recover without consuming later
valid siblings.

## Official Metadata

Every row below is a `Complete` atomic public feature with no subset,
unsupported remainder, recognized-unsupported code, or aggregate-alias target.

| Stable ID | Source | Exact fragment |
| --- | --- | --- |
| `later.rule.counter-style` | `O-COUNTERSTYLES3` | `#the-counter-style-rule` |
| `official.descriptor.counter-style.system` | `O-COUNTERSTYLES3` | `#counter-style-system` |
| `official.descriptor.counter-style.negative` | `O-COUNTERSTYLES3` | `#counter-style-negative` |
| `official.descriptor.counter-style.prefix` | `O-COUNTERSTYLES3` | `#counter-style-prefix` |
| `official.descriptor.counter-style.suffix` | `O-COUNTERSTYLES3` | `#counter-style-suffix` |
| `official.descriptor.counter-style.range` | `O-COUNTERSTYLES3` | `#counter-style-range` |
| `official.descriptor.counter-style.pad` | `O-COUNTERSTYLES3` | `#counter-style-pad` |
| `official.descriptor.counter-style.fallback` | `O-COUNTERSTYLES3` | `#counter-style-fallback` |
| `official.descriptor.counter-style.symbols` | `O-COUNTERSTYLES3` | `#counter-style-symbols` |
| `official.descriptor.counter-style.additive-symbols` | `O-COUNTERSTYLES3` | `#counter-style-symbols` |
| `official.descriptor.counter-style.speak-as` | `O-COUNTERSTYLES3` | `#counter-style-speak-as` |
| `official.value.counter-style` | `O-COUNTERSTYLES3` | `#the-counter-style-rule` |
| `official.value.counter-style-name` | `O-COUNTERSTYLES3` | `#the-counter-style-rule` |
| `official.value.symbol` | `O-COUNTERSTYLES3` | `#counter-style-symbols` |
| `official.value.symbols-function` | `O-COUNTERSTYLES3` | `#symbols-function` |
| `official.value.symbols-type` | `O-COUNTERSTYLES3` | `#symbols-function` |
| `later.rule.page` | `O-CSS2` | `page.html#page-box` |
| `official.selector.page-pseudo` | `O-CSS2` | `page.html#page-selectors` |

The preserved aggregate aliases keep their existing source, support status,
and exact target slices. Counter and page metadata adds no alias and does not
claim parser dispatch for excluded evaluation or rendering behavior.

## Source-Backed Product Fixture Delta

The product fixture before C11 has SHA-256
`96be045dc181fe5fc258e76b09458b441139504a3cae13c41897995ab3ae8f5d`.
The source-backed two-row replacement has SHA-256
`7c2cf7d79368d76d94cc0b383be70cc404d4c69d7caa72eedba6f0762e0b2356`.

Exactly these stable rows change:

- `catalog.non-property.later.rule.counter-style.boundary` becomes clean,
  retains `rule:later.rule.counter-style`, and removes only the obsolete
  unsupported-rule diagnostic; and
- `catalog.non-property.later.rule.page.boundary` becomes clean, retains
  `rule:later.rule.page`, and removes only the obsolete unsupported-rule
  diagnostic.

Their entry points, feature modes, authored inputs, and all unrelated fixture
fields remain byte-identical. Every other fixture row remains byte-identical.

## Root-Owned Follow-Up

Root `surgeist` owns the integration work:

1. select and promote the intended `surgeist-css` gitlink;
2. expose the counter-style, page-rule, page-selector, descriptor/value, and
   conformance metadata additions through the facade;
3. keep counter evaluation, marker rendering, pagination, page matching,
   cascade, resource loading, CSSOM, and cross-crate lowering outside this leaf;
4. refresh root-owned generated API audit artifacts with the root generator;
5. update root integration tests and documentation for the public models,
   recovery distinctions, official metadata closure, and exclusions.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and generated API artifacts. This leaf handoff does not authorize
changes in root or sibling repositories.

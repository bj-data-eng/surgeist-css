# P01-I02-C09 Media, Supports, Import, And Prelude Ordering

## Boundary

`surgeist-css` owns strict authored media queries, supports conditions and
conditional group rules, import clauses, top-level grammar ordering, structured
recovery, and truthful conformance metadata. It does not evaluate conditions,
match selectors, resolve URLs, load resources, apply cascade or substitution,
compute layer order, expose CSSOM behavior, or lower syntax into another
Surgeist crate.

## Media Queries 3 And Recovery

The public authored model covers all eleven Media Queries 3 media types and the
complete width, height, device-width, device-height, orientation, aspect-ratio,
device-aspect-ratio, color, color-index, monochrome, resolution, scan, and grid
feature families, including permitted boolean and min/max forms. Media Queries 3
resolution owns positive `dpi` and `dpcm`; `dppx` remains a separate Media
Queries 4 delta. Existing one-sided comparison forms and selected later discrete
features retain their existing source and support boundaries.

A syntactically complete unknown type, feature, or balanced feature value is
retained as defined-false authored syntax without a diagnostic. Reserved types,
malformed structure, empty comma members, invalid min/max boolean forms, and the
deferred `scripting` feature recover as `CssMediaQuery::Never` with
`ReplaceMediaQueryWithNever`. Recovery is comma-local and preserves later valid
members and the containing media rule.

## Supports Inspection And Recovery

Supports conditions expose declaration, `selector()`, general-enclosed, `not`,
`and`, and `or` branches. Declaration tests preserve authored spelling, value,
importance, position, and an optional property-specific known-declaration view.
They are inspection nodes and are not inserted into a style declaration list.
Balanced unknown property/value syntax remains a valid authored test.

The selector branch exposes the complex-selector subset currently implemented.
Other balanced selector-function content falls back to general-enclosed authored
syntax. Namespace-qualified and remaining Selectors 3 syntax stays assigned to
the next selector cycle. General-enclosed owns only the selected immutable
production; it does not import the adjacent generic boolean grammar.

Supports group rules are retained at top level, in conditional groups, in nested
style contexts, and in scoped rule lists where style rules are allowed. Invalid
children recover without dropping a valid parent. A malformed parent prelude is
dropped after any earlier child diagnostics, and later siblings remain eligible.

## Import Clauses And Prelude Phases

`CssImportRule` preserves a target, optional layer, optional supports condition,
and optional media list in that exact order. The supports wrapper accepts either
a full condition or a bare declaration with implied parentheses. Duplicate,
swapped, or trailing clauses invalidate only that import.

The top-level authored phase transitions are:

| Current phase | Initial layer statement | Import | Namespace hook | Body rule |
| --- | --- | --- | --- | --- |
| Initial | Initial | Imports | Namespaces | Body |
| Imports | Body | Imports | Namespaces | Body |
| Namespaces | Body | invalid, unchanged | Namespaces | Body |
| Body | Body | invalid, unchanged | invalid, unchanged | Body |

Only successfully parsed rules advance the phase. The namespace transition is a
dormant hook until namespace grammar is implemented; this cycle does not retain
or reclassify `@namespace`. Encoding keeps its independent first-rule and
one-shot checks.

## Conformance Metadata

All fifteen `O-MEDIA3` parser-facing rows and all three `O-CONDITIONAL3` rows are
`Complete`. `O-CASCADE4` owns complete `baseline.rule.import`, including its
target, `supports()`, and media grammar.

Distinct additions preserve later-source ownership:

| Stable ID | Source fragment | Product disposition |
| --- | --- | --- |
| `ext.media.resolution.dppx` | `R-MEDIA4#resolution` | `Complete` |
| `ext.supports.general-enclosed` | immutable `X-VALUES4` repository provenance, `css-values-4/Overview.bs#general-enclosed` | `Complete` |
| `ext.supports.selector` | `R-CONDITIONAL4#at-supports` | `Partial`: current typed complex-selector subset; namespace-qualified and remaining Selectors 3 syntax stays outside this row until the selector cycle |
| `ext.import.layer` | `R-CASCADE5#at-import` | `Complete` |
| `ext.stylesheet.prelude-order` | `R-CASCADE5#at-import` ordering fragment | `Complete` |

There is no separate import-supports extension row: Cascade 4 owns that clause.
Conditional 3 does not absorb `selector()`, Media Queries 3 does not absorb
`dppx`, comparisons, or later discrete features, and general-enclosed does not
claim generic boolean grammar.

## Source-Backed Product Fixture Delta

The product fixture before this cycle has digest
`67e69813d808ffda40e7c159fde719fbadd0447f8e4105788b0bb593931fac89`.
The replacement product fixture has digest
`95518fbabb04cd5b96bc9505a4d96681d444042498d681f28b3db4f3d8a2f0d3`.

Exactly these eight stable rows change from a diagnostic-bearing report to a
clean report while preserving their authored input and applicable retained
syntax:

- `catalog.non-property.baseline.media.range-feature.boundary`;
- `catalog.non-property.baseline.media.type.boundary`;
- `catalog.non-property.baseline.rule.media.boundary`;
- `focused.specialized.media-position`;
- `focused.structured-errors.08`;
- `catalog.non-property.later.rule.supports.boundary`;
- `focused.structured-errors.01`;
- `catalog.non-property.baseline.rule.import.boundary`.

Every other fixture row remains byte-identical.

## Root-Owned Follow-Up

Root `surgeist` owns the integration work:

1. select and promote the intended `surgeist-css` gitlink;
2. expose the current media, supports, import, prelude-ordering, and conformance
   metadata surface through the facade;
3. keep query and supports evaluation, selector matching, URL resolution,
   resource loading, cascade, and substitution outside this leaf;
4. refresh root-owned generated API audit artifacts with the root generator;
5. update root integration documentation and tests for the source split,
   defined-false versus malformed recovery, supports inspection, import clause
   ordering, and downstream exclusions.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and generated API artifacts. This leaf handoff does not authorize
changes in root or sibling repositories.

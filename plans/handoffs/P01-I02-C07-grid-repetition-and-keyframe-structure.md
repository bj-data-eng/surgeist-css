# P01-I02-C07 Grid Repetition And Keyframe Structure

## Boundary

`surgeist-css` owns strict authored Grid repetition syntax and authored
`@keyframes` structure. It preserves structural Grid constraints, current versus
I01 compatibility views, source-ordered keyframe rules and blocks, and recovery
diagnostics. It does not perform Grid layout, cascade or declaration processing,
animation-name matching, interpolation, timeline evaluation, serialization, or
cross-crate lowering.

## Current Grid Surface And I01 Compatibility

The current Grid property wrappers expose parser-owned structural values through
`current()`:

- `grid-template-rows` and `grid-template-columns` expose a general track list or
  an auto track list containing exactly one automatic repeat;
- `grid-template` preserves its current rows and columns through the same track
  list model;
- `grid-auto-rows` and `grid-auto-columns` expose nonempty track-size lists and
  reject `repeat()`;
- `grid` preserves its current template or auto-flow aggregate.

Integer track-repeat and fixed-repeat content is nonempty and non-recursive.
Automatic repetition uses fixed-size content, and its surrounding tracks and
integer repeats are fixed-size. A current typed calculation may have no exact I01
representation. Every existing wrapper retains `i01_subset()` with its frozen
signature; root consumers must treat it as compatibility data rather than the
current validity boundary.

## Authored Keyframe Structure

Empty keyframe rules and blocks are retained. Duplicate blocks, duplicate
equivalent offsets across blocks, and repeated equivalent selectors within one
selector list remain in authored order. The leaf does not sort, merge, cascade,
or deduplicate them. Dropping an invalid declaration may leave a valid empty
block and rule. A genuinely invalid selector drops the smallest invalid block,
while an otherwise valid rule with no retained blocks remains present.

## Partial Metadata Boundaries

`ext.value.grid-repeat` is a `Partial` `R-GRID2` value record for
`#repeat-notation`, owned by `crate::parser::grid` and paired with the named
public behavior `grid_repeat_models_reject_invalid_cross_products`. Its supported
subset is non-recursive integer track and fixed repetition plus one fixed-size
automatic repetition where the consumer permits it. Subgrid name-repeat and
other unselected Grid 2 forms remain unsupported.

`baseline.property.grid-template-rows`,
`baseline.property.grid-template-columns`, `baseline.property.grid-template`,
`baseline.property.grid-auto-rows`, `baseline.property.grid-auto-columns`, and
`baseline.property.grid` remain `Partial` `R-GRID2` records. Their supported
subset names the structural repetition boundary above; subgrid name-repeat and
other unselected Grid 2 property grammar remain outside it.

`baseline.rule.keyframes` remains a `Partial` `I-ANIMATIONS1` record for
`#keyframes`. Its supported subset includes empty and duplicate authored
structures. Calculation selectors, string-name grammar, and declaration-
processing grammar not selected by this boundary remain unsupported. The
`official.value.calc` and animation-property records are unchanged.

## Source-Backed Fixture Correction

The product fixture artifact before this boundary had SHA-256
`98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8`.
Its source-backed replacement fixture artifact has SHA-256
`99bbb897710969949d7b596d14fbd352d5d3121a6c4cf663b8ca100154057f8b`.
These are digests of the public-observable fixture artifacts, not repository,
revision identifiers. The correction rejects a structurally invalid Grid
shorthand cross-product and retains keyframe parents that are valid after
declaration or child recovery.

## Root-Owned Follow-Up

Root `surgeist` must:

1. deliberately promote the selected `surgeist-css` candidate gitlink;
2. expose the current Grid models, Grid property wrappers, keyframe structures,
   and conformance records through the facade;
3. update root-owned adapters to consume `current()` and treat `i01_subset()` as
   compatibility data only;
4. preserve authored Grid and keyframe ordering until the owning layout,
   cascade, or animation layer;
5. refresh root-owned generated API audit artifacts with the root generator;
6. update root integration documentation and tests for Grid structural
   repetition, current/I01 projection, empty and duplicate keyframes, recovery,
   and every retained Partial boundary.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and all generated API artifacts. This leaf handoff does not
authorize mutations in root or sibling repositories.

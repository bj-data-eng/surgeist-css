# P01-I02-C02 Conformance Reconciliation

## Purpose And Boundary

This static record maps the official production ledger to the owning C02
artifacts. The ledger remains the independent coverage authority. The public
catalog, private reserved slots, public exclusions, implementation inventories,
and independently authored parser cases remain separate artifacts; none is
generated from another.

C02 adds metadata and ownership boundaries. It does not change parser language,
retained syntax, diagnostics, source positions, recovery spans, recovery
actions, declaration coupling, ordinary versus `app-strict` behavior, features,
or dependencies. The C01 observable fixture remains the parser-behavior
preservation authority.

## Source And Tier Registry

`src/conformance.rs` owns `SPECIFICATION_SOURCES` and exposes it through
`specification_sources()` and exact `specification_source()` lookup. Every
record has one stable ID, module, level, tier, and exactly one URL or repository
provenance value. A tier classifies provenance only and never implies parser
support.

| Tier | Registered source IDs |
| --- | --- |
| `Snapshot2026Official` | `O-CSS2`, `O-SYNTAX3`, `O-STYLE-ATTR`, `O-MEDIA3`, `O-CONDITIONAL3`, `O-SELECTORS3`, `O-NAMESPACES3`, `O-CASCADE4`, `O-VALUES3`, `O-VARIABLES1`, `O-BOX3`, `O-COLOR4`, `O-BACKGROUNDS3`, `O-IMAGES3`, `O-FONTS3`, `O-WRITING3`, `O-MULTICOL1`, `O-FLEXBOX1`, `O-UI3`, `O-CONTAIN1`, `O-TRANSFORMS1`, `O-COMPOSITING1`, `O-EASING1`, `O-COUNTERSTYLES3` |
| `Snapshot2026Reliable` | `R-MEDIA4`, `R-SCROLLBARS1`, `R-GRID1`, `R-GRID2`, `R-CASCADE5`, `R-CONDITIONAL4` |
| `Snapshot2026Stable` | `S-DISPLAY3`, `S-WRITING4`, `S-BREAK3`, `S-ALIGN3`, `S-SHAPES1`, `S-TEXT3`, `S-TEXTDECOR3`, `S-MASKING1` |
| `Snapshot2026Interop` | `I-TRANSITIONS1`, `I-ANIMATIONS1`, `I-FILTER1`, `I-SIZING3`, `I-TRANSFORMS2`, `I-LISTS3`, `I-POSITION3`, `I-FONTS4`, `I-COLOR5`, `I-SELECTORS4`, `I-CONTAIN2`, `I-NESTING1` |
| `SurgeistExtension` | `X-CONTAIN3`, `X-CONDITIONAL5`, `X-CASCADE6`, `X-PSEUDO4`, `X-VALUES4`, `X-MEDIA5`, `X-OVERFLOW3`, `X-SIZING4`, `X-TEXT4`, `X-TEXTDECOR4`, `X-UI4`, `X-CONTENT3`, `X-FULLSCREEN`, `X-FILTER2-BASE`, `X-DISPLAY-MODE-BASE`, `X-GRID-TOLERANCE-BASE`, `I01-BASE-SELECTORS`, `I01-BASE-QUERIES` |
| `LaterStandard` | Publicly representable tier with no registered C02 source record. |

`specification_source()`, `feature_metadata()`, and
`conformance_exclusion()` are atomic ID lookups: matching is case-sensitive,
and whitespace and aliases are not normalized. `property_metadata()` is the
separate authored-property lookup and accepts canonical names and declared
property aliases with ASCII-case-insensitive matching.

## Exact Conformance Dispositions

| Disposition | Owning artifact | Meaning |
| --- | --- | --- |
| Atomic parser-facing support | `FEATURE_CATALOG` in `src/conformance.rs` | One feature ID, kind, spelling, source, tier, production, and truthful `CssSupportStatus`. `Complete` covers the entire named production, `Partial` states both the accepted subset and valid remainder, and `RecognizedUnsupported` names the diagnostic identity. |
| Preserved baseline alias | Four `CssConformanceDisposition::BaselineAlias` records in `FEATURE_CATALOG` | A queryable I01 aggregate with an immutable atomic target slice. It carries its preserved aggregate boundary but owns no parser dispatch or implementation inventory entry. |
| Reserved official slot | `OFFICIAL_PROPERTY_COVERAGE`, `OFFICIAL_NON_PROPERTY_COVERAGE`, and `OFFICIAL_LEGACY_PROPERTY_ALIAS_COVERAGE` in `src/conformance.rs` | Private future grammar metadata with an official ID, kind, source, production, future module/cycle, and evidence boundary. It is not a public feature record, has no support status, and does not make an authored spelling recognized. |
| Official exclusion | `CONFORMANCE_EXCLUSIONS` and borrowed `OFFICIAL_EXCLUDED_COVERAGE` records in `src/conformance.rs` | A public audit fact with `InformativeOnly`, `SupersededWithoutCurrentProduction`, or `OutsideAuthoredSyntaxBoundary`. It has no support status, parser dispatch, implementation inventory entry, or parser case. |

## Official Ledger Mapping

| Ledger area | Registry, catalog, slots, and exclusions | Implementation inventory | Behavioral case area |
| --- | --- | --- | --- |
| Section 1, contract and counting unit | `SPECIFICATION_SOURCES`, `FEATURE_CATALOG`, `OFFICIAL_PROPERTY_COVERAGE`, `OFFICIAL_NON_PROPERTY_COVERAGE`, `OFFICIAL_LEGACY_PROPERTY_ALIAS_COVERAGE`, and `OFFICIAL_EXCLUDED_COVERAGE` in `src/conformance.rs` form the mutually exclusive reconciliation view. | `ATOMIC_IMPLEMENTATION_INVENTORY` in `src/conformance.rs` borrows the property and parser inventories without generating catalog records or cases. | `tests/conformance_catalog.rs`, `tests/catalog_inventory.rs`, `tests/catalog_inventory/vectors.rs`, and `tests/initiative_i01_audit.rs` apply explicit public metadata and parser stimuli. `tests/i01_c01_observables.rs` applies the frozen observable fixture. |
| Sections 2.1-2.2, canonical official properties and CSS2 fragments | Active schema-backed property IDs are atomic `FEATURE_CATALOG` records; future canonical properties are private `OFFICIAL_PROPERTY_COVERAGE` reserved records. The custom-property family is shared through `baseline.declaration.custom-property`. | `property_schema!` and `property_implementation_inventory()` in `src/properties.rs` own active known-property dispatch identities; `src/parser/variables.rs` owns the custom-property declaration identity. | `tests/catalog_inventory.rs` and `tests/catalog_inventory/vectors.rs` exercise ordinary, global, substitution-dependent, negative, and recovery outcomes through `parse_style_attribute`. `tests/initiative_i01_audit.rs` and the C01 observable fixture preserve declaration recovery. |
| Sections 2.3-2.4, legacy shorthand, supersession, and property exclusions | `OFFICIAL_LEGACY_PROPERTY_ALIAS_COVERAGE` reserves `official.property-alias.glyph-orientation-vertical` without a support status. Superseded CSS2 property definitions, informative Appendix A properties, `glyph-orientation-horizontal`, and `ime-mode` are public `CONFORMANCE_EXCLUSIONS` records and are borrowed by `OFFICIAL_EXCLUDED_COVERAGE`. | Active property identities remain in `src/properties.rs`; the reserved legacy shorthand has no implementation inventory entry. | Existing active property cases remain in `tests/catalog_inventory.rs`; exclusion metadata is exercised through public lookup in `tests/conformance_catalog.rs` and `tests/public_surface.rs`. No reserved slot is presented as parser behavior. |
| Section 3, official non-property units | Each ledger row maps to an active atomic feature or a private record in `OFFICIAL_NON_PROPERTY_COVERAGE`. Sources come from `SPECIFICATION_SOURCES`; active records come from `FEATURE_CATALOG`. | `atomic_implementation_inventories()` in `src/parser/mod.rs` combines rule, qualified-rule, declaration, descriptor, selector, media, shared-value, and preserved container-extension inventories from their owning parser modules. | `tests/conformance_catalog.rs` exercises named metadata plus positive, negative, unsupported, and recovery outcomes. `tests/initiative_i01_audit.rs`, `tests/public_surface.rs`, and the C01 observable fixture preserve front-door behavior and structured diagnostics. |
| Section 4, baseline aggregate aliases | The four alias records remain in `FEATURE_CATALOG`; their exact target slices are listed below. Targets are atomic records. Aliases are neither reserved slots nor exclusions. | Only atomic targets appear in implementation inventories; aggregate alias IDs do not. | `tests/conformance_catalog.rs` applies public parser stimuli for each explicit atomic target and the malformed-media recovery target. |
| Section 5, exact non-property exclusions | `CONFORMANCE_EXCLUSIONS` owns exact source area, reason, and optional superseding IDs. `OFFICIAL_EXCLUDED_COVERAGE` borrows those records and adds no second metadata owner. | Excluded items have no implementation inventory entry. | `tests/conformance_catalog.rs` and `tests/public_surface.rs` exercise exclusion lookup and typed public metadata; exclusions do not receive synthetic parser cases. |

## Baseline Alias Target Slices

| Baseline alias | Immutable atomic targets |
| --- | --- |
| `baseline.selector.pseudo-element` | `official.selector.generated`, `ext.pseudo-element.marker`, `ext.pseudo-element.selection`, `ext.pseudo-element.backdrop`, `ext.pseudo-element.generated-marker` |
| `baseline.media.query-list` | `official.media.query-list-core`, `ext.media.condition-syntax`, `ext.media.malformed-member-never` |
| `baseline.media.range-feature` | `official.media.feature.width`, `official.media.feature.height`, `official.media.feature.resolution`, `official.media.feature.color`, `official.media.feature.monochrome`, `ext.media.range.width`, `ext.media.range.height`, `ext.media.range.resolution`, `ext.media.range.color`, `ext.media.range.monochrome` |
| `baseline.media.discrete-feature` | `official.media.feature.orientation`, `ext.media.hover`, `ext.media.any-hover`, `ext.media.pointer`, `ext.media.any-pointer`, `ext.media.prefers-color-scheme`, `ext.media.prefers-reduced-motion`, `ext.media.prefers-reduced-transparency`, `ext.media.prefers-contrast`, `ext.media.forced-colors`, `ext.media.display-mode` |

## Atomic Implementation Inventory Map

| Authored boundary | Inventory source |
| --- | --- |
| Known properties | `property_schema!` and `property_implementation_inventory()` in `src/properties.rs` |
| Rule, qualified-rule, declaration, and container-rule shell identities | `IMPLEMENTED_RULES`, `IMPLEMENTED_QUALIFIED_RULES`, `IMPLEMENTED_DECLARATIONS`, and `IMPLEMENTED_CONTAINER_EXTENSIONS` in `src/parser/mod.rs` |
| Font-face rule and descriptors | `IMPLEMENTED_RULES` and `IMPLEMENTED_DESCRIPTORS` in `src/parser/font_face.rs` |
| Keyframes rule | `IMPLEMENTED_RULES` in `src/parser/keyframes.rs` |
| Nesting selector | `IMPLEMENTED_SELECTORS` in `src/parser/nesting.rs` |
| Media and container-query productions | `IMPLEMENTED_MEDIA` and `IMPLEMENTED_CONTAINER_EXTENSIONS` in `src/parser/queries.rs` |
| Selector productions | `IMPLEMENTED_SELECTORS` in `src/parser/selectors.rs` |
| Custom-property declaration and substitution-dependent value | `IMPLEMENTED_DECLARATIONS` and `IMPLEMENTED_SHARED_VALUES` in `src/parser/variables.rs` |

The central view borrows these slices for direct reconciliation. It does not
generate catalog entries, reserved slots, exclusions, or behavioral cases.

## Root-Owned Follow-Up

Root `surgeist` owns all cross-repository integration. It must deliberately
promote the `surgeist-css` gitlink; expose `CssSpecificationSourceId`,
`CssSpecificationTier`, `CssSpecificationSource`,
`CssConformanceExclusionId`, `CssConformanceSupersedingId`,
`CssExclusionReason`, `CssExclusionMetadata`, `specification_sources()`,
`specification_source()`, `conformance_exclusions()`, and
`conformance_exclusion()` through the root facade; retain
`CssFeatureMetadata::baseline_alias_targets()` on the reexported feature
metadata; refresh the root-owned generated API audit artifacts with the root
generator; and update root integration documentation and tests for these
additions. The leaf does not edit root files, sibling crates, or generated API
artifacts.

# P01-I01 CSS Snapshot 2026 Remediation Specification

## 1 Outcome

`surgeist-css` shall accept and model every syntax-owned feature in the official
CSS Snapshot 2026 definition that applies to a parsed stylesheet or style
attribute. It shall also preserve the finite non-official inventory in 3.5,
frozen from source and documentation at initiative base
`ae44d858308e4f73c17e91c4c8768c43ce6ceb82`. Every construct retained by the
browser-compatible parser shall satisfy its complete grammar, every discarded
or implicitly repaired source construct shall produce a typed recovery
diagnostic, and every public model shall prevent construction of states that the
parser itself would reject. The additive `app-strict` feature shall expose
validation entry points that return a syntax result only when the same parse
produces no recovery diagnostics.

The source review at
`plans/P01-implement-full-css-spec/P01-css-snapshot-2026-review.md`
(`sha256:5ddd3eebb4fc3664759021605d3884a0c795947e0ef4e427d3dfc5e77469199d`),
scoped to commit `318864d1074d8d723a3a925528343c8a3d8c7253`, identifies 25 actionable findings.
This specification closes all 25 as one initiative. Completion requires a fresh
holistic review of the implemented initiative; the historical review remains
unchanged as evidence about its original commit.

The normative profile edition is the W3C Group Note of 26 March 2026 at
https://www.w3.org/TR/2026/NOTE-css-2026-20260326/. Its sections 2.1 through
2.4 determine tier membership. 3.1 freezes the exact dated module revision used
for each grammar. Moving `/TR/` aliases and editor's drafts are discovery aids
only and are not conformance inputs. A module revision published after the Note
remains in the Note's tier only when it is the same named module level; it does
not import a later module level or change tier membership.

Where an existing public claim uses a later module, the later grammar supersedes
the lower-level production only for that claimed feature. Existing examples are
Grid, cascade layers, container queries, selector-list pseudo-classes, masking,
transitions, animations, filters, CSS nesting, scope, Color 5 authored forms,
and generated-content extensions.

## 2 Ownership And Non-Goals

`surgeist-css` owns authored CSS syntax, intrinsic lexical and grammar validity,
CSS Syntax recovery boundaries, authored source positions, recovery diagnostics,
and machine-readable support classification. Every syntax type introduced by
this initiative is in the authored phase; parse reports are recovered-authored
results, and types whose names explicitly say metadata, diagnostic, or
validation failure belong to those corresponding support phases.

The following remain outside this repository and this initiative:

- cascade ordering or application, including layer precedence;
- inheritance application;
- custom-property substitution or post-substitution validation;
- evaluation of `@supports`, media, container, or style queries;
- selector matching, specificity application, scope proximity, or pseudo-state;
- resource loading, URL resolution, import composition, font loading, or image
  decoding;
- numeric evaluation beyond intrinsic Values 3 calculation type-checking and
  the required pure-numeric zero-divisor check, unit resolution, color
  conversion, color mixing, gamut mapping, animation interpolation, or renderer
  adaptation;
- root adapters, root API artifacts, sibling repositories, and root gitlinks;
- unchecked raw nodes, silent recovery, preservation of malformed source as a
  syntax node, or treating a recovered output as proof that its original source
  was wholly valid.

The implementation shall add exactly one Cargo feature, `app-strict`, with no
feature dependencies and with browser-compatible parsing available when the
feature is disabled or enabled. It shall add no dependency, build script,
generator, CI rule, policy file, reusable external tool, or external test-corpus
copy. It shall use the already pinned `cssparser = 0.37.0` and
`cssparser-color = 0.5.0` through safe APIs. Surgeist-owned Rust shall contain no
`unsafe` code. Importing or vendoring an external conformance corpus remains a
separate acquisition decision; this initiative only makes later fixture adapters
possible without changing production parser semantics.

## 3 Compatibility Contract

### 3.1 Profiles

The crate shall distinguish these specification tiers:

| Tier | Source-profile meaning |
| --- | --- |
| `Snapshot2026Official` | A section 2.1 normative source |
| `Snapshot2026Reliable` | A section 2.2 source |
| `Snapshot2026Stable` | A section 2.3 source |
| `Snapshot2026Interop` | A section 2.4 source |
| `SurgeistExtension` | A later or baseline source explicitly selected by this crate |
| `LaterStandard` | A known standards-track source outside the selected profile |

A tier classifies only the source and selected profile; it never asserts parser
support. `CssConformanceDisposition` independently records complete, partial,
recognized-unsupported, or excluded status for the exact production. Therefore
`later.font-feature-values` retains its `Snapshot2026Interop` tier while being
`RecognizedUnsupported`. `LaterStandard` is reserved for an outside-profile
source and does not itself imply a support status. Completion requirements come
from 3.2/3.5/3.6, not from the tier enum. Metadata must name the exact module,
level, and production and must not label an entire module complete when only one
production is implemented.

The public `#[non_exhaustive]` `CssSpecificationTier` enum shall contain exactly
the six variants in this table at initiative completion. Property metadata
exposes it through `CssPropertyMetadata::tier()`. Structured unsupported-feature
diagnostics expose the stable feature ID, module and level, and tier, so
`later.font-feature-values` is publicly distinguishable from an unknown at-rule
without adding a general non-property metadata query API.

The complete official module registry is below. Every `O-*` key has tier
`Snapshot2026Official`; the authored syntax column is the only part owned here.

| Source key | Module and level | Pinned dated revision | In-boundary authored syntax |
| --- | --- | --- | --- |
| `O-CSS2` | CSS Level 2 Revision 1 | https://www.w3.org/TR/2011/REC-CSS2-20110607/ | Core rules, page syntax, selectors and properties not superseded below |
| `O-SYNTAX3` | CSS Syntax 3 | https://www.w3.org/TR/2021/CRD-css-syntax-3-20211224/ | Tokenization, rule/declaration structure and encoding declaration validation |
| `O-STYLE-ATTR` | CSS Style Attributes | https://www.w3.org/TR/2013/REC-css-style-attr-20131107/ | Strict declaration-list style attributes |
| `O-MEDIA3` | Media Queries 3 | https://www.w3.org/TR/2024/REC-mediaqueries-3-20240521/ | Media query lists, media types and media features |
| `O-CONDITIONAL3` | Conditional Rules 3 | https://www.w3.org/TR/2024/CRD-css-conditional-3-20240815/ | `@media` nesting and `@supports` conditions |
| `O-SELECTORS3` | Selectors 3 | https://www.w3.org/TR/2018/REC-selectors-3-20181106/ | Complete Selectors 3 grammar |
| `O-NAMESPACES3` | Namespaces 3 | https://www.w3.org/TR/2014/REC-css-namespaces-3-20140320/ | `@namespace` and namespace-qualified selectors |
| `O-CASCADE4` | Cascade 4 | https://www.w3.org/TR/2022/CR-css-cascade-4-20220113/ | CSS-wide keywords and declaration importance only |
| `O-VALUES3` | Values and Units 3 | https://www.w3.org/TR/2024/CRD-css-values-3-20240322/ | Primitive values, units and typed math syntax |
| `O-VARIABLES1` | Custom Properties 1 | https://www.w3.org/TR/2022/CR-css-variables-1-20220616/ | Custom declarations and symbolic `var()` references |
| `O-BOX3` | Box Model 3 | https://www.w3.org/TR/2024/REC-css-box-3-20240411/ | Margin, padding and box-model value syntax |
| `O-COLOR4` | Color 4 | https://www.w3.org/TR/2026/CRD-css-color-4-20260715/ | Authored colors, system colors and opacity syntax |
| `O-BACKGROUNDS3` | Backgrounds and Borders 3 | https://www.w3.org/TR/2024/CRD-css-backgrounds-3-20240311/ | Background, border, radius, image-border and shadow syntax |
| `O-IMAGES3` | Images 3 | https://www.w3.org/TR/2023/CRD-css-images-3-20231218/ | Image values, gradients, object sizing/positioning and rendering syntax |
| `O-FONTS3` | Fonts 3 | https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/ | Font properties and the Level 3 `@font-face` grammar |
| `O-WRITING3` | Writing Modes 3 | https://www.w3.org/TR/2019/REC-css-writing-modes-3-20191210/ | Direction and writing-mode syntax |
| `O-MULTICOL1` | Multi-column Layout 1 | https://www.w3.org/TR/2024/CR-css-multicol-1-20240516/ | Column property and shorthand syntax |
| `O-FLEXBOX1` | Flexible Box Layout 1 | https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/ | Flex properties and shorthands |
| `O-UI3` | Basic User Interface 3 | https://www.w3.org/TR/2026/REC-css-ui-3-20260407/ | Cursor, outline and related property syntax |
| `O-CONTAIN1` | Containment 1 | https://www.w3.org/TR/2024/REC-css-contain-1-20240625/ | `contain` property syntax |
| `O-TRANSFORMS1` | Transforms 1 | https://www.w3.org/TR/2019/CR-css-transforms-1-20190214/ | Transform functions and origin syntax |
| `O-COMPOSITING1` | Compositing and Blending 1 | https://www.w3.org/TR/2024/CRD-compositing-1-20240321/ | Blend-mode and isolation property syntax |
| `O-EASING1` | Easing Functions 1 | https://www.w3.org/TR/2023/CRD-css-easing-1-20230213/ | Easing function value syntax |
| `O-COUNTERSTYLES3` | Counter Styles 3 | https://www.w3.org/TR/2021/CR-css-counter-styles-3-20210727/ | `@counter-style` and counter-style value syntax |

The exact non-official source registry used by 3.5 and 3.6 is:

| Source key | Module and level | Tier | Pinned dated revision or baseline source |
| --- | --- | --- | --- |
| `R-MEDIA4` | Media Queries 4 | `Snapshot2026Reliable` | https://www.w3.org/TR/2026/CRD-mediaqueries-4-20260219/ |
| `R-SCROLLBARS1` | Scrollbars 1 | `Snapshot2026Reliable` | https://www.w3.org/TR/2021/CR-css-scrollbars-1-20211209/ |
| `R-GRID1` | Grid 1 | `Snapshot2026Reliable` | https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/ |
| `R-GRID2` | Grid 2 | `Snapshot2026Reliable` | https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/ |
| `R-CASCADE5` | Cascade 5 | `Snapshot2026Reliable` | https://www.w3.org/TR/2022/CR-css-cascade-5-20220113/ |
| `R-CONDITIONAL4` | Conditional Rules 4 | `Snapshot2026Reliable` | https://www.w3.org/TR/2025/CRD-css-conditional-4-20250904/ |
| `S-DISPLAY3` | Display 3 | `Snapshot2026Stable` | https://www.w3.org/TR/2026/CRD-css-display-3-20260605/ |
| `S-WRITING4` | Writing Modes 4 | `Snapshot2026Stable` | https://www.w3.org/TR/2019/CR-css-writing-modes-4-20190730/ |
| `S-BREAK3` | Fragmentation 3 | `Snapshot2026Stable` | https://www.w3.org/TR/2018/CR-css-break-3-20181204/ |
| `S-ALIGN3` | Box Alignment 3 | `Snapshot2026Stable` | https://www.w3.org/TR/2026/WD-css-align-3-20260130/ |
| `S-SHAPES1` | Shapes 1 | `Snapshot2026Stable` | https://www.w3.org/TR/2025/CRD-css-shapes-1-20250612/ |
| `S-TEXT3` | Text 3 | `Snapshot2026Stable` | https://www.w3.org/TR/2026/CRD-css-text-3-20260608/ |
| `S-TEXTDECOR3` | Text Decoration 3 | `Snapshot2026Stable` | https://www.w3.org/TR/2022/CRD-css-text-decor-3-20220505/ |
| `S-MASKING1` | Masking 1 | `Snapshot2026Stable` | https://www.w3.org/TR/2021/CRD-css-masking-1-20210805/ |
| `I-TRANSITIONS1` | Transitions 1 | `Snapshot2026Interop` | https://www.w3.org/TR/2026/WD-css-transitions-1-20260108/ |
| `I-ANIMATIONS1` | Animations 1 | `Snapshot2026Interop` | https://www.w3.org/TR/2023/WD-css-animations-1-20230302/ |
| `I-FILTER1` | Filter Effects 1 | `Snapshot2026Interop` | https://www.w3.org/TR/2018/WD-filter-effects-1-20181218/ |
| `I-SIZING3` | Sizing 3 | `Snapshot2026Interop` | https://www.w3.org/TR/2021/WD-css-sizing-3-20211217/ |
| `I-TRANSFORMS2` | Transforms 2 | `Snapshot2026Interop` | https://www.w3.org/TR/2021/WD-css-transforms-2-20211109/ |
| `I-LISTS3` | Lists and Counters 3 | `Snapshot2026Interop` | https://www.w3.org/TR/2020/WD-css-lists-3-20201117/ |
| `I-POSITION3` | Positioned Layout 3 | `Snapshot2026Interop` | https://www.w3.org/TR/2025/WD-css-position-3-20251007/ |
| `I-FONTS4` | Fonts 4 | `Snapshot2026Interop` | https://www.w3.org/TR/2026/WD-css-fonts-4-20260422/ |
| `I-COLOR5` | Color 5 | `Snapshot2026Interop` | https://www.w3.org/TR/2026/WD-css-color-5-20260618/ |
| `I-SELECTORS4` | Selectors 4 | `Snapshot2026Interop` | https://www.w3.org/TR/2026/WD-selectors-4-20260122/ |
| `I-CONTAIN2` | Containment 2 | `Snapshot2026Interop` | https://www.w3.org/TR/2022/WD-css-contain-2-20220917/ |
| `I-NESTING1` | Nesting 1 | `Snapshot2026Interop` | https://www.w3.org/TR/2026/WD-css-nesting-1-20260122/ |
| `X-CONDITIONAL5` | Conditional Rules 5 | `SurgeistExtension` | https://www.w3.org/TR/2025/WD-css-conditional-5-20251030/ |
| `X-CASCADE6` | Cascade 6 | `SurgeistExtension` | https://www.w3.org/TR/2024/WD-css-cascade-6-20240906/ |
| `X-PSEUDO4` | Pseudo-elements 4 | `SurgeistExtension` | https://www.w3.org/TR/2025/WD-css-pseudo-4-20250627/ |
| `X-VALUES4` | Values and Units 4 | `SurgeistExtension` | https://www.w3.org/TR/2024/WD-css-values-4-20240312/ |
| `X-MEDIA5` | Media Queries 5 | `SurgeistExtension` | https://www.w3.org/TR/2026/WD-mediaqueries-5-20260219/ |
| `X-OVERFLOW3` | Overflow 3 | `SurgeistExtension` | https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/ |
| `X-SIZING4` | Sizing 4 | `SurgeistExtension` | https://www.w3.org/TR/2021/WD-css-sizing-4-20210520/ |
| `X-TEXT4` | Text 4 | `SurgeistExtension` | https://www.w3.org/TR/2026/WD-css-text-4-20260608/ |
| `X-TEXTDECOR4` | Text Decoration 4 | `SurgeistExtension` | https://www.w3.org/TR/2022/WD-css-text-decor-4-20220504/ |
| `X-UI4` | Basic User Interface 4 | `SurgeistExtension` | https://www.w3.org/TR/2026/WD-css-ui-4-20260120/ |
| `X-CONTENT3` | Generated Content 3 | `SurgeistExtension` | https://www.w3.org/TR/2025/WD-css-content-3-20251204/ |
| `X-FULLSCREEN` | Fullscreen | `SurgeistExtension` | https://www.w3.org/TR/2012/WD-fullscreen-20120703/ |
| `X-FILTER2-BASE` | Filter Effects 2 `backdrop-filter` subset | `SurgeistExtension` | `ae44d858308e4f73c17e91c4c8768c43ce6ceb82:src/parser/effects.rs` |
| `X-DISPLAY-MODE-BASE` | authored `display-mode` media feature | `SurgeistExtension` | `ae44d858308e4f73c17e91c4c8768c43ce6ceb82:src/parser/queries.rs` |
| `X-GRID-TOLERANCE-BASE` | authored `grid-flow-tolerance` property | `SurgeistExtension` | `ae44d858308e4f73c17e91c4c8768c43ce6ceb82:src/parser/grid.rs` |

Across both registries, only authored syntax assigned by 3.5, 3.6, and 6-11
is owned here. Cascade, evaluation, matching, loading, layout, painting, and
rendering semantics remain out of scope even when their specifications define
the grammar.

The Snapshot-linked Fonts 3 specification moved `@font-feature-values` to Fonts
4. This initiative therefore does not implement or model that at-rule. The
parser-facing catalog shall classify stable ID `later.font-feature-values` as a
`Snapshot2026Interop` `RecognizedUnsupported` at-rule sourced by `I-FONTS4`, and
parsing it shall return `UnsupportedAtRule`. A matching implementation-
classification record and strict diagnostic vector make this exclusion explicit
without turning the rest of Fonts 4 into an implementation target.

### 3.2 Support Status

`CssSupportStatus` shall be a closed public enum with `Complete`, `Partial`, and
`RecognizedUnsupported` variants. `Partial` is available for honest intermediate
development metadata but no official in-boundary Snapshot feature may remain
`Partial` or `RecognizedUnsupported` at initiative completion. Every 3.6
baseline property and every 3.5 row marked `Complete` must also be complete.
Only an exact row explicitly marked `RecognizedUnsupported`, currently
`later.font-feature-values`, may finish in that state; its presence does not
imply support for another production from the same module.

Support status applies only to an author-facing parser production. The
conformance catalog shall use this one crate-private closed disposition model:

```rust
enum CssConformanceDisposition {
    ParserFacing(CssSupportStatus),
    Excluded(CssExclusionReason),
}

enum CssExclusionReason {
    InformativeOnly,
    SupersededWithoutCurrentProduction,
    OutsideAuthoredSyntaxBoundary,
}
```

An `Excluded` entry has no current author-facing grammar spelling owned by this
crate and therefore has no public support status. `InformativeOnly` is limited
to notes, examples and indexes with no normative grammar production.
`SupersededWithoutCurrentProduction` is limited to a lower-level or removed
production wholly replaced by a cataloged current production and with no
independently current spelling. `OutsideAuthoredSyntaxBoundary` is limited to
cascade application, evaluation, matching, loading, layout, painting, CSSOM/IDL
and other semantics excluded by 2. Any current author-facing property,
descriptor, at-rule, selector or value spelling must instead be `ParserFacing`;
if it is deliberately not parsed, its status is `RecognizedUnsupported` and it
receives an exact public unsupported diagnostic.

An unknown spelling is different from a recognized feature that is not
implemented. Public diagnostics and support queries shall preserve that
difference. Exclusion entries are audit facts, not hidden aliases for unknown or
unsupported parser spellings, and are not exposed by property metadata.

### 3.3 Independent Conformance Catalog

`src/conformance.rs` shall contain a versioned, source-linked Snapshot 2026
catalog independent of parser dispatch. `PARSER_CONFORMANCE` shall enumerate
every in-boundary official property, descriptor, at-rule, qualified rule,
selector feature, media type, media feature and shared value, plus every 3.5
and 3.6 parser production. `CONFORMANCE_EXCLUSIONS` shall enumerate each exact
non-parser item required by the fixed official source audit. Both slices use one
record shape and their union is `CONFORMANCE_CATALOG`. Each entry shall name:

- one globally unique stable production identifier;
- exactly one feature kind: property, descriptor, at-rule, qualified rule,
  selector production, media type, media feature, or shared value production;
- exactly one 3.1 source key and one exact production name or fragment;
- the one specification tier implied by that source key;
- exactly one 3.2 `CssConformanceDisposition`.

A production identifier owns one grammar production or one explicitly bounded
delta to an earlier production. It never combines an at-rule with its
descriptors, a property family with shared values, two module levels, or
official syntax with a later extension. A later delta may reference an official
base production, but the two entries have distinct IDs, sources, tiers, vectors,
and acceptance claims. A property production may reference independently
inventoried shared values without either entry owning the other's grammar.

Official section 2.1 entries use only `O-*` sources. 3.5 is the complete
non-official non-property inventory and contains no section 2.1 production and
no property production. 3.6 is the complete baseline property ownership map;
each listed name expands to exactly one `baseline.property.<canonical-name>`
entry. Newly added official properties use
`official.property.<canonical-name>`. No accepted parser path may exist without
one of those disjoint `ParserFacing` ownership records. Exclusion IDs use
`excluded.<source-key>.<production>` and name the one current owning or
superseding catalog entry when the reason requires it. Repository-baseline
sources in 3.1 are permitted only for `SurgeistExtension` productions that have
no immutable dated standards publication; their commit and path are the
normative source.

The catalog is the conformance oracle. It shall not invoke parser tables to
derive its inventory. Source tests shall compare parser-facing entries to
implementation metadata and shall compare the fixed official source-item
inventory to the union of parser-facing and exclusion entries. Implementation
metadata shall not generate or mutate either catalog slice.

Each implementation-owning module shall also expose a crate-private,
kind-specific implementation inventory. The property schema supplies the
property inventory. At-rule dispatch, nested qualified-rule parsing, selector
parsing, media-type parsing, media-feature parsing, descriptor parsing, and
shared-value parsing shall respectively supply `AT_RULE_IMPLEMENTATION`,
`QUALIFIED_RULE_IMPLEMENTATION`, `SELECTOR_IMPLEMENTATION`,
`MEDIA_TYPE_IMPLEMENTATION`, `MEDIA_FEATURE_IMPLEMENTATION`,
`DESCRIPTOR_IMPLEMENTATION`, and `SHARED_VALUE_IMPLEMENTATION` typed record
slices. Tier is metadata, not a second feature kind; there is no parallel
`EXTENSION_IMPLEMENTATION` slice. Every implementation record shall contain its
stable catalog identifier, implemented support state, and owning parser/schema
discriminator. These records live with the parser or schema declarations they
describe and may be generated from those implementation declarations; they
shall never be generated from `src/conformance.rs` or from tests.

Independent tests shall compare, by feature kind, parser-facing catalog IDs,
implementation IDs, and independently authored vector IDs in all directions.
Every `ParserFacing` entry shall have exactly one implementation record and at
least one vector record; every implementation and vector record shall name a
parser-facing entry; and support states shall agree. This three-way check applies
to properties, descriptors, at-rules, selector features, media types, media
features, qualified rules, and shared values. Every parser-facing ID appears in
exactly one kind comparison. Every `Excluded` entry shall instead have exactly
one source-audit record, no implementation/vector ID, and a reason/source/owner
combination permitted by 3.2. The official source-item inventory must equal the
disjoint union of official parser-facing IDs and exclusion IDs. These checks
prevent the parser tables, tests, or exclusions from becoming a circular or
silent conformance oracle.

### 3.4 Public Metadata

The public front door shall expose read-only metadata sufficient for callers to
ask whether a recognized property spelling is complete, partial, recognized but
unsupported, or unknown. This includes official properties and the explicitly
preserved baseline properties in 3.6. `CssPropertyMetadata` shall use
private fields and accessors for canonical name, aliases, module, level, tier,
and support status. The exact lookup is:

```rust
pub fn property_metadata(name: &str) -> Option<&'static CssPropertyMetadata>;
```

`CssPropertyMetadata` shall expose `canonical_name() -> &'static str`,
`aliases() -> &'static [&'static str]`, `module() -> &'static str`,
`level() -> &'static str`, `tier() -> CssSpecificationTier`, and
`status() -> CssSupportStatus`. Lookup is ASCII-case-insensitive for recognized
non-custom property names, returns the same static record for an alias and its
canonical spelling, and returns `None` for custom property names and unknown
names. Its tier accessor distinguishes official Snapshot tiers from
`SurgeistExtension`.

At initiative completion every returned property record has status `Complete`;
`None` remains the unknown/custom result. `Partial` and
`RecognizedUnsupported` remain explicit status vocabulary for a future catalog
revision, not hidden final property entries or reasons to retain unreachable
parse-error variants.

The public metadata is generated from the implementation's authoritative
property schema. The independent conformance catalog remains a separate test
oracle.

The public diagnostic surface also includes one read-only
`CssUnsupportedFeatureMetadata` value for every parser-facing catalog entry
whose final status is `RecognizedUnsupported`. It has private fields and exposes
`production_id() -> &CssProductionId`,
`module() -> &'static str`,
`level() -> &'static str`, and
`tier() -> CssSpecificationTier`. It is generated from the same implementation
classification record as the unsupported dispatch arm and cross-checked against
the independent catalog; there is no public general non-property metadata lookup.
At initiative completion the only such value is `later.font-feature-values`.

### 3.5 Frozen Non-Official Production Inventory

The following is the complete non-section-2.1, non-property preservation target
at initiative base `ae44d858308e4f73c17e91c4c8768c43ce6ceb82`. Every row is
one parser-facing catalog entry and one primary vector record. Official Selectors 3 forms,
Custom Properties 1, Color 4 system colors, base `@import`, and `::before` /
`::after` are deliberately absent because their sole owners are official entries.

| Stable production ID | Kind | Exact production slice | Source | Final status |
| --- | --- | --- | --- | --- |
| `ext.at-rule.layer-statement` | at-rule | Empty `@layer <layer-name-list>;` statement | `R-CASCADE5` | Complete |
| `ext.at-rule.layer-block` | at-rule | Named or anonymous `@layer` block | `R-CASCADE5` | Complete |
| `ext.import.layer-clause` | shared value | `@import` `layer` / `layer(<layer-name>)` clause | `R-CASCADE5` | Complete |
| `ext.import.supports-clause` | shared value | `@import` `supports()` clause | `R-CASCADE5` | Complete |
| `ext.value.revert-layer` | shared value | Whole-value `revert-layer` global keyword | `R-CASCADE5` | Complete |
| `ext.supports.selector` | shared value | `selector(<complex-real-selector>)` support test | `R-CONDITIONAL4` | Complete |
| `ext.at-rule.container` | at-rule | Named or anonymous `@container` group rule | `X-CONDITIONAL5` | Complete |
| `ext.container.size-feature` | shared value | Width, height, inline-size, block-size, aspect-ratio and orientation container conditions | `X-CONDITIONAL5` | Complete |
| `ext.container.style-exists` | shared value | Custom-property existence style query | `X-CONDITIONAL5` | Complete |
| `ext.container.style-equals` | shared value | Custom-property equality style query | `X-CONDITIONAL5` | Complete |
| `ext.at-rule.scope` | at-rule | `@scope` with optional root and limit and the 6.3 nested-rule set | `X-CASCADE6` | Complete |
| `ext.nesting.style-rule` | qualified rule | Nesting 1 nested style rule, `&` composition and deterministic flattening | `I-NESTING1` | Complete |
| `ext.at-rule.keyframes` | at-rule | `@keyframes` rule and keyframe selector blocks | `I-ANIMATIONS1` | Complete |
| `later.font-feature-values` | at-rule | Fonts 4 `@font-feature-values` recognition only | `I-FONTS4` | RecognizedUnsupported |
| `ext.selector.is` | selector | Strict complex selector list in `:is()` | `I-SELECTORS4` | Complete |
| `ext.selector.where` | selector | Strict complex selector list in `:where()` | `I-SELECTORS4` | Complete |
| `ext.selector.not-complex` | selector | Selectors 4 complex selector list delta for `:not()` | `I-SELECTORS4` | Complete |
| `ext.selector.has` | selector | Strict relative selector list in `:has()` | `I-SELECTORS4` | Complete |
| `ext.selector.nth-child-of` | selector | `:nth-child(<an+b> of <complex-real-selector-list>)` | `I-SELECTORS4` | Complete |
| `ext.selector.nth-last-child-of` | selector | `:nth-last-child(<an+b> of <complex-real-selector-list>)` | `I-SELECTORS4` | Complete |
| `ext.selector.attribute-case` | selector | `i` and `s` valued-attribute selector modifiers | `I-SELECTORS4` | Complete |
| `ext.selector.scope-pseudo` | selector | `:scope` | `I-SELECTORS4` | Complete |
| `ext.selector.focus-visible` | selector | `:focus-visible` | `I-SELECTORS4` | Complete |
| `ext.selector.focus-within` | selector | `:focus-within` | `I-SELECTORS4` | Complete |
| `ext.selector.required` | selector | `:required` | `I-SELECTORS4` | Complete |
| `ext.selector.optional` | selector | `:optional` | `I-SELECTORS4` | Complete |
| `ext.selector.valid` | selector | `:valid` | `I-SELECTORS4` | Complete |
| `ext.selector.invalid` | selector | `:invalid` | `I-SELECTORS4` | Complete |
| `ext.selector.placeholder-shown` | selector | `:placeholder-shown` | `I-SELECTORS4` | Complete |
| `ext.selector.default` | selector | `:default` | `I-SELECTORS4` | Complete |
| `ext.selector.indeterminate` | selector | `:indeterminate` | `I-SELECTORS4` | Complete |
| `ext.selector.read-only` | selector | `:read-only` | `I-SELECTORS4` | Complete |
| `ext.selector.read-write` | selector | `:read-write` | `I-SELECTORS4` | Complete |
| `ext.selector.in-range` | selector | `:in-range` | `I-SELECTORS4` | Complete |
| `ext.selector.out-of-range` | selector | `:out-of-range` | `I-SELECTORS4` | Complete |
| `ext.selector.modal` | selector | `:modal` | `I-SELECTORS4` | Complete |
| `ext.selector.fullscreen` | selector | `:fullscreen` | `I-SELECTORS4` | Complete |
| `ext.selector.popover-open` | selector | `:popover-open` | `I-SELECTORS4` | Complete |
| `ext.selector.nesting-anchor` | selector | `&` in Nesting 1 selector composition | `I-NESTING1` | Complete |
| `ext.selector.scope-anchor` | selector | `&` in scoped style selectors | `X-CASCADE6` | Complete |
| `ext.pseudo-element.marker` | selector | Terminal `::marker` | `X-PSEUDO4` | Complete |
| `ext.pseudo-element.selection` | selector | Terminal `::selection` | `X-PSEUDO4` | Complete |
| `ext.pseudo-element.backdrop` | selector | Terminal `::backdrop` | `X-FULLSCREEN` | Complete |
| `ext.pseudo-element.generated-marker` | selector | Allowed `::before::marker` and `::after::marker` sequence | `X-PSEUDO4` | Complete |
| `ext.media.range.width` | media feature | MQ4 range syntax delta for width | `R-MEDIA4` | Complete |
| `ext.media.range.height` | media feature | MQ4 range syntax delta for height | `R-MEDIA4` | Complete |
| `ext.media.range.resolution` | media feature | MQ4 range syntax delta for resolution | `R-MEDIA4` | Complete |
| `ext.media.range.color` | media feature | MQ4 range syntax delta for color | `R-MEDIA4` | Complete |
| `ext.media.range.monochrome` | media feature | MQ4 range syntax delta for monochrome | `R-MEDIA4` | Complete |
| `ext.media.hover` | media feature | `hover` | `R-MEDIA4` | Complete |
| `ext.media.any-hover` | media feature | `any-hover` | `R-MEDIA4` | Complete |
| `ext.media.pointer` | media feature | `pointer` | `R-MEDIA4` | Complete |
| `ext.media.any-pointer` | media feature | `any-pointer` | `R-MEDIA4` | Complete |
| `ext.media.prefers-color-scheme` | media feature | `prefers-color-scheme` | `X-MEDIA5` | Complete |
| `ext.media.prefers-reduced-motion` | media feature | `prefers-reduced-motion` | `X-MEDIA5` | Complete |
| `ext.media.prefers-reduced-transparency` | media feature | `prefers-reduced-transparency` | `X-MEDIA5` | Complete |
| `ext.media.prefers-contrast` | media feature | `prefers-contrast` | `X-MEDIA5` | Complete |
| `ext.media.forced-colors` | media feature | `forced-colors` | `X-MEDIA5` | Complete |
| `ext.media.display-mode` | media feature | Authored `display-mode` identifier set at the initiative base | `X-DISPLAY-MODE-BASE` | Complete |
| `ext.grid.subgrid` | shared value | Grid 2 `subgrid` track-list branch | `R-GRID2` | Complete |
| `ext.value.basic-shape` | shared value | Shapes 1 `inset()`, `circle()`, `ellipse()` and `polygon()` subset accepted by shape-bearing properties | `S-SHAPES1` | Complete |
| `ext.color.color-mix` | shared value | `color-mix()` | `I-COLOR5` | Complete |
| `ext.color.relative-rgb` | shared value | Relative `rgb(from ...)` | `I-COLOR5` | Complete |
| `ext.color.relative-hsl` | shared value | Relative `hsl(from ...)` | `I-COLOR5` | Complete |
| `ext.color.relative-hwb` | shared value | Relative `hwb(from ...)` | `I-COLOR5` | Complete |
| `ext.color.relative-lab` | shared value | Relative `lab(from ...)` | `I-COLOR5` | Complete |
| `ext.color.relative-lch` | shared value | Relative `lch(from ...)` | `I-COLOR5` | Complete |
| `ext.color.relative-oklab` | shared value | Relative `oklab(from ...)` | `I-COLOR5` | Complete |
| `ext.color.relative-oklch` | shared value | Relative `oklch(from ...)` | `I-COLOR5` | Complete |
| `ext.color.relative-color` | shared value | Relative `color(from ... <space> ...)` | `I-COLOR5` | Complete |
| `ext.font-face.font-display` | descriptor | `font-display` descriptor | `I-FONTS4` | Complete |
| `ext.font-face.font-style-range` | descriptor | Range form of `font-style` descriptor | `I-FONTS4` | Complete |
| `ext.font-face.font-weight-range` | descriptor | Range form of `font-weight` descriptor | `I-FONTS4` | Complete |
| `ext.font-face.font-stretch-range` | descriptor | Range form of `font-stretch` descriptor | `I-FONTS4` | Complete |
| `ext.font-face.tech-hint` | descriptor | `tech()` hint in a `src` item | `I-FONTS4` | Complete |
| `ext.length-unit.font-relative` | shared value | `rex`, `cap`, `rcap`, `rch`, `ic`, `ric`, `lh`, `rlh` | `X-VALUES4` | Complete |
| `ext.length-unit.logical-viewport` | shared value | `vi`, `vb` | `X-VALUES4` | Complete |
| `ext.length-unit.small-viewport` | shared value | `svw`, `svh`, `svi`, `svb`, `svmin`, `svmax` | `X-VALUES4` | Complete |
| `ext.length-unit.large-viewport` | shared value | `lvw`, `lvh`, `lvi`, `lvb`, `lvmin`, `lvmax` | `X-VALUES4` | Complete |
| `ext.length-unit.dynamic-viewport` | shared value | `dvw`, `dvh`, `dvi`, `dvb`, `dvmin`, `dvmax` | `X-VALUES4` | Complete |
| `ext.length-unit.container-query` | shared value | `cqw`, `cqh`, `cqi`, `cqb`, `cqmin`, `cqmax` | `X-VALUES4` | Complete |

No other section 2.2-2.4 or later non-property feature is implied. An unlisted
baseline non-official production shall be retired with a breaking migration note
and rejection vector; it may not survive as an implicit compatibility promise.

### 3.6 Frozen Baseline Property Ownership

At the initiative base, `src/validation.rs` recognizes exactly 179 non-custom
property names. The table below assigns every name to one and only one source
key. Each comma-delimited name is a separate parser-facing catalog entry with stable ID
`baseline.property.<canonical-name>`, feature kind `property`, the source key's
one tier, and final status `Complete`. Grouping is presentation-only and does not
create family catalog entries. A property's source is the highest preserved
production selected by this initiative and supersedes the lower official
production only for that property. Shared 3.5 value productions remain separate
referenced entries.

| Source key | Canonical property names |
| --- | --- |
| `O-CASCADE4` | `all` |
| `S-DISPLAY3` | `display` |
| `O-UI3` | `box-sizing`, `cursor`, `text-overflow`, `outline`, `outline-color`, `outline-style`, `outline-width` |
| `I-POSITION3` | `position`, `inset`, `top`, `right`, `bottom`, `left`, `z-index` |
| `O-WRITING3` | `direction` |
| `S-WRITING4` | `writing-mode` |
| `X-OVERFLOW3` | `overflow`, `overflow-x`, `overflow-y` |
| `O-CSS2` | `float`, `clear`, `visibility`, `vertical-align` |
| `S-ALIGN3` | `align-content`, `justify-content`, `align-items`, `align-self`, `justify-items`, `justify-self`, `place-content`, `place-items`, `place-self`, `gap`, `row-gap`, `column-gap`, `justify-tracks`, `align-tracks` |
| `X-CONTENT3` | `content` |
| `I-CONTAIN2` | `content-visibility` |
| `I-LISTS3` | `list-style-type`, `list-style-position`, `list-style-image`, `list-style`, `counter-reset`, `counter-increment`, `counter-set` |
| `I-SIZING3` | `width`, `height`, `min-width`, `min-height`, `max-width`, `max-height`, `flex-basis` |
| `X-GRID-TOLERANCE-BASE` | `grid-flow-tolerance` |
| `R-GRID1` | `grid-template-rows`, `grid-template-columns`, `grid-template-areas`, `grid-template`, `grid-auto-rows`, `grid-auto-columns`, `grid-auto-flow`, `grid-row-start`, `grid-row-end`, `grid-column-start`, `grid-column-end`, `grid-row`, `grid-column`, `grid-area`, `grid` |
| `O-FONTS3` | `font-size`, `line-height`, `font-family`, `font`, `font-weight`, `font-style`, `font-stretch`, `font-variant`, `font-feature-settings` |
| `S-TEXT3` | `text-align`, `text-align-last`, `text-indent`, `letter-spacing`, `white-space`, `word-break`, `overflow-wrap`, `text-transform` |
| `X-TEXT4` | `text-wrap` |
| `S-TEXTDECOR3` | `text-decoration`, `text-decoration-line`, `text-decoration-color`, `text-decoration-style` |
| `X-TEXTDECOR4` | `text-decoration-thickness` |
| `S-BREAK3` | `box-decoration-break` |
| `O-BOX3` | `margin`, `margin-top`, `margin-right`, `margin-bottom`, `margin-left`, `padding`, `padding-top`, `padding-right`, `padding-bottom`, `padding-left` |
| `O-BACKGROUNDS3` | `border`, `border-top`, `border-right`, `border-bottom`, `border-left`, `border-width`, `border-top-width`, `border-right-width`, `border-bottom-width`, `border-left-width`, `background`, `background-color`, `border-color`, `border-top-color`, `border-right-color`, `border-bottom-color`, `border-left-color`, `background-image`, `background-position`, `background-size`, `background-repeat`, `background-origin`, `background-clip`, `background-attachment`, `border-style`, `border-top-style`, `border-right-style`, `border-bottom-style`, `border-left-style`, `border-radius`, `border-top-left-radius`, `border-top-right-radius`, `border-bottom-right-radius`, `border-bottom-left-radius`, `box-shadow` |
| `O-COLOR4` | `color`, `opacity` |
| `O-FLEXBOX1` | `flex-direction`, `flex-wrap`, `flex-grow`, `flex-shrink`, `order`, `flex` |
| `X-SIZING4` | `aspect-ratio` |
| `R-SCROLLBARS1` | `scrollbar-width` |
| `X-UI4` | `pointer-events`, `user-select` |
| `O-TRANSFORMS1` | `transform`, `transform-origin` |
| `I-TRANSFORMS2` | `translate`, `rotate`, `scale` |
| `I-FILTER1` | `filter` |
| `X-FILTER2-BASE` | `backdrop-filter` |
| `S-MASKING1` | `clip-path`, `mask`, `mask-image`, `mask-size`, `mask-position`, `mask-repeat` |
| `I-TRANSITIONS1` | `transition-property`, `transition-duration`, `transition-delay`, `transition-timing-function`, `transition` |
| `I-ANIMATIONS1` | `animation-name`, `animation-duration`, `animation-delay`, `animation-timing-function`, `animation-iteration-count`, `animation-direction`, `animation-fill-mode`, `animation-play-state`, `animation` |

New official properties required by 10 use `official.property.*` IDs and their
one `O-*` source. They do not alter or alias the 179 baseline IDs. Coverage shall
assert that the baseline table contains 179 unique names, exactly matches the
base source inventory, and has no name in 3.5.

## 4 Authoritative Declaration Model

### 4.1 One Property Schema

`src/properties.rs` shall own one declarative `css_properties!` schema. The macro
is crate-private and shall contain one entry per canonical recognized non-custom
property, whether official or an explicitly preserved Surgeist extension. Every
recognized alias appears in exactly one entry. Every final entry is `Complete`
and carries canonical name, aliases, module and level, official-or-extension
tier, the `Complete` support status, an exact typed value, and a parser function.
3.2 permits other status values in the public metadata vocabulary, but the
final property inventory contains none; test-only or unreachable schema forms
shall not be created merely to exercise them. The schema shall generate:

- the validated `CssKnownProperty` identifier;
- canonical and alias name lookup;
- one `CssKnownDeclaration` variant per entry;
- the property-coupled parsed value type for each generated variant;
- property-specific parse dispatch for every entry;
- `CssPropertyMetadata` implementation records;
- the mapping from a parsed declaration back to `CssKnownProperty`.

`CssKnownProperty` and `CssKnownDeclaration` shall be public
`#[non_exhaustive]` enums. Adding a property therefore adds variants without
granting downstream code an exhaustive-match promise. Their fields and all
component fields remain private or validated-by-construction, and stable
accessors expose canonical property identity and the coupled authored value.
External consumer examples and tests shall match these enums with a wildcard and
shall use accessors for ordinary inspection.

Property parser functions shall return their property-specific type, never a
broad cross-property value enum. Unknown non-custom names return
`UnknownProperty`; every recognized name has a complete parser, and malformed or
out-of-grammar values return `InvalidPropertyValue`. There is no final
`UnsupportedProperty` or `UnsupportedPropertyValue` error path because the
catalog contains no corresponding parser-facing production. A future initiative
that adds such a production must add its public diagnostic category and evidence
at the same time. The schema shall not contain test vectors; vectors remain
independent evidence.

No second supported-property table, manual property enum, broad `CssValue`
dispatch, or hand-synchronized name map may remain.

### 4.2 Property-Coupled Values

The independent `CssProperty` plus `CssValue` pair shall be removed. The public
model shall use these names and this variant structure:

```rust
pub struct CssDeclaration {
    body: CssDeclarationBody,
    importance: CssImportance,
    position: CssSourcePosition,
}

pub enum CssDeclarationBody {
    Known(CssKnownDeclaration),
    Custom(CssCustomDeclaration),
}

pub enum CssDeclaredValue<T> {
    Value(T),
    Global(CssGlobalKeyword),
    VariableDependent(CssVariableDependentValue),
}

pub enum CssCustomPropertyDeclaredValue {
    Value(CssCustomPropertyValue),
    Global(CssGlobalKeyword),
}
```

Every `CssKnownDeclaration` variant shall carry
`CssDeclaredValue<PropertySpecificType>`. `all` shall instead carry a dedicated
`CssAllDeclarationValue` that permits only a global keyword or a variable-
dependent authored value. A custom declaration shall couple one validated
`CssCustomPropertyName` with one `CssCustomPropertyDeclaredValue`. The dedicated
custom-property union preserves whole-value CSS-wide keyword semantics while its
`Value` branch retains the permissive custom-property token stream, including an
empty stream and valid `var()` references.

`CssDeclaration::property_name()` shall derive a borrowed property-name view from
the body. It shall not read a separately stored discriminator. The public
`#[non_exhaustive]` `CssPropertyNameRef<'a>` enum has exactly
`Known(CssKnownProperty)` and `Custom(&'a CssCustomPropertyName)` variants.

`CssDeclarationBody` and `CssPropertyNameRef<'a>` shall be
`#[non_exhaustive]`; callers use `CssDeclaration::known()`,
`CssDeclaration::custom()`, `CssDeclaration::property_name()`, and
`CssDeclaration::importance()` accessors rather than relying on exhaustive
matches. `CssDeclaredValue<T>` is non-exhaustive for future authored value forms
and exposes `value() -> Option<&T>`,
`global() -> Option<CssGlobalKeyword>`, and
`variable_dependent() -> Option<&CssVariableDependentValue>`.
`CssKnownDeclaration::property() -> CssKnownProperty` derives the canonical
property from its active typed variant. `CssCustomDeclaration` exposes
`name() -> &CssCustomPropertyName` and
`value() -> &CssCustomPropertyDeclaredValue`.
`CssCustomPropertyDeclaredValue` is public and non-exhaustive and exposes
`value() -> Option<&CssCustomPropertyValue>` and
`global() -> Option<CssGlobalKeyword>`. `CssAllDeclarationValue` is a public
`#[non_exhaustive]` enum with `Global(CssGlobalKeyword)` and
`VariableDependent(CssVariableDependentValue)` variants and corresponding
optional accessors; a concrete non-global `all` value remains unrepresentable.
`CssImportance` remains an intentionally exhaustive two-state enum.

All fields are private. Constructors that could create mismatched property/value
pairs do not exist. Parser-only constructors remain crate-private and accept only
already validated component types.

### 4.3 Importance

`CssImportance` shall be a closed public enum with `Normal` and `Important`.
`Normal` is the real default. Importance belongs to the declaration, not the
property value or raw custom-property token text.

The declaration boundary shall remove one terminal, ASCII-case-insensitive
`!important` annotation after allowing CSS whitespace and comments in the places
the grammar permits. It shall reject misspellings, duplicate annotations, tokens
after the annotation, a bare `!`, and an annotation embedded in another token.
The removed annotation shall never appear in custom-property or variable-
dependent authored text.

### 4.4 Declaration Lists And Style Attributes

`CssDeclarationList` shall be a public private-field wrapper over an ordered list
of declarations. Empty lists are valid. Style rules, scoped rules, page rules,
and the style-attribute parser shall reuse the same strict ordinary-declaration
parser and list type.

Keyframe blocks shall instead own a public private-field
`CssKeyframeDeclarationList` of `CssKeyframeDeclaration` values. A keyframe
declaration contains a validated `CssDeclarationBody` and source position but no
importance field, so important keyframe declarations are unrepresentable. One
private declaration parser core shall accept an exact
`CssDeclarationParseContext::{Ordinary, Keyframe}` discriminator. `Ordinary`
constructs `CssDeclaration` with either importance state. `Keyframe` constructs
`CssKeyframeDeclaration` only after confirming the parsed annotation is absent;
a terminal `!important` returns `InvalidDeclarationAnnotation` at the `!` and
rejects the complete sheet. Empty keyframe declaration lists remain valid.

The public front door shall add:

```rust
pub fn parse_style_attribute(input: &str) -> Result<CssDeclarationList>;
```

It shall consume the complete input. It shall accept an empty attribute, comments,
optional final semicolons, custom properties, variables, global keywords, and
importance. It shall reject qualified rules, at-rules, malformed separators,
trailing non-declaration tokens, and every invalid declaration without dropping
the invalid item.

### 4.5 Custom Properties And Variables

Custom-property values and known-property values containing `var()` shall retain
an opaque authored token stream plus a typed reference tree. They shall not
attempt substitution, dependency-cycle resolution, shorthand expansion, or
post-substitution property validation. Their exact public private-field surface
is:

```rust
pub struct CssCustomPropertyValue { /* authored token stream plus references */ }
pub struct CssVariableDependentValue { /* authored token stream plus references */ }
pub struct CssVariableReference { /* custom name plus optional fallback */ }
pub struct CssVariableFallback { /* authored fallback plus nested references */ }
```

`CssCustomPropertyValue` exposes `as_css() -> &str`, `is_empty() -> bool`, and
`references() -> &[CssVariableReference]`. Its authored stream may be empty,
matching the `<declaration-value>?` grammar; empty is a valid authored value and
is not the unobservable guaranteed-invalid initial value.
`CssVariableDependentValue` exposes `as_css() -> &str` and
`references() -> &[CssVariableReference]`; its reference slice is non-empty by
construction. Each `CssVariableReference` exposes
`name() -> &CssCustomPropertyName` and
`fallback() -> Option<&CssVariableFallback>`. A fallback exposes
`as_css() -> &str`, `is_empty() -> bool`, and
`references() -> &[CssVariableReference]`. `None` distinguishes `var(--x)` from
an authored empty fallback: both `var(--x,)` and `var(--x, )` produce `Some`
whose `is_empty()` is true.

At each value or fallback level, `references()` contains the directly authored
`var()` calls in source order. References nested inside a fallback are inspected
only through that parent reference's fallback, preserving the authored tree
without a second flattened index. Names are decoded validated
`CssCustomPropertyName` values; fallback text preserves every token after the
first comma, including additional commas.

`as_css()` is the exact original UTF-8 source slice after parser-owned boundary
CSS whitespace/comments are removed. It preserves interior whitespace,
comments, escapes, spelling, case, commas, and nested block text. The declaration
parser removes one valid terminal `!important` annotation, together with only
the boundary whitespace/comments that separate it, before constructing either
authored stream; that outer annotation never appears in the enclosing
`as_css()`. Tokens with the same spelling inside a nested fallback are ordinary
fallback content and remain preserved. Malformed `var()`, a bad token, an
unmatched closer, a top-level semicolon, or a non-annotation top-level `!`
rejects the complete parse. All constructors and stored fields are private.

Vectors shall distinguish empty custom values, whitespace-only values, exact
case/interior preservation, no fallback, empty fallback, comma-containing
fallback, nested references, multiple direct references, malformed references,
and terminal importance removal in both sheet and style-attribute entry points.
Public consumer tests shall traverse every accessor without using `Debug` or a
private module.

## 5 Diagnostics And Source Positions

### 5.1 Position Convention

`CssSourcePosition` shall use zero-based line indexes and zero-based UTF-16 code
unit column indexes. `CssLineIndex` and `CssUtf16ColumnIndex` shall be semantic
newtypes with private fields, value accessors, equality, ordering, and hashing.
The exact public shape is:

```rust
pub struct CssSourcePosition {
    line: CssLineIndex,
    column: CssUtf16ColumnIndex,
}
```

`CssSourcePosition` exposes `line() -> CssLineIndex` and
`column() -> CssUtf16ColumnIndex`. `CssLineIndex::value() -> u32` and
`CssUtf16ColumnIndex::value() -> u32` are the only scalar inspection accessors.
All three types are copyable value types; their fields and parser-conversion
constructors remain private.

Recovery diagnostics shall use a public private-field `CssSourceSpan` with an
inclusive `start() -> CssSourcePosition` and exclusive
`end() -> CssSourcePosition`. Its constructor remains private and guarantees
`start <= end` in source order. A zero-width span is permitted only for an
implicit end-of-input closure or a missing token; discarded authored text uses a
non-empty span covering the complete recovery unit.

Converting a `cssparser::SourceLocation` shall preserve its line and subtract one
from its one-based UTF-16 column with `saturating_sub(1)`. This total dependency-
boundary conversion maps the dependency's contract-violating zero column to
zero rather than panicking or wrapping; every contract-conforming column maps
exactly. Unit tests shall construct zero, one, later-column, multiline, and
non-BMP cases and assert the exact zero-based result. Public integration tests
shall inspect both typed accessors and assert that a supplementary Unicode scalar
before an error advances the column by two UTF-16 code units.

`Error` and every successful syntax node that currently carries a location shall
use `CssSourcePosition`. Human `Display` output shall render one-based line and
column numbers while public accessors continue to expose typed zero-based values.
The crate-level docs shall state the encoding and basis explicitly.

### 5.2 Stable Error Categories

`Error` shall expose this public stable coarse category through
`Error::code() -> CssErrorCode` in addition to detailed `ErrorKind`:

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CssErrorCode {
    UnexpectedEnd,
    UnexpectedToken,
    InvalidEncodingDeclaration,
    InvalidAtRulePlacement,
    InvalidAtRulePrelude,
    InvalidAtRuleBody,
    UnsupportedAtRule,
    InvalidMediaQuery,
    InvalidSelector,
    UndeclaredNamespacePrefix,
    UnknownProperty,
    InvalidPropertyValue,
    InvalidDeclarationAnnotation,
    UnknownDescriptor,
    InvalidDescriptorValue,
    InvalidDescriptorCombination,
    InvalidColorSyntax,
}
```

`ErrorKind` shall be a public `#[non_exhaustive]` structured enum with exactly
the same root variants and names at initiative completion. Every variant carries
the one detail type in this matrix:

| `ErrorKind` variant | Public private-field detail type and exact accessors |
| --- | --- |
| `UnexpectedEnd` | `CssUnexpectedEndError`: `expected() -> &CssGrammarExpectation` |
| `UnexpectedToken` | `CssUnexpectedTokenError`: `expected() -> &CssGrammarExpectation`, `encountered() -> &CssTokenSummary` |
| `InvalidEncodingDeclaration` | `CssEncodingDeclarationError`: `expected() -> &CssGrammarExpectation`, `encountered() -> Option<&CssTokenSummary>` |
| `InvalidAtRulePlacement` | `CssAtRulePlacementError`: `at_rule() -> &CssIdentifier`, `expected_context() -> &CssGrammarExpectation` |
| `InvalidAtRulePrelude`, `InvalidAtRuleBody` | `CssAtRuleSyntaxError`: `at_rule() -> &CssIdentifier`, `production_id() -> &CssProductionId`, `expected() -> &CssGrammarExpectation`, `encountered() -> Option<&CssTokenSummary>` |
| `UnsupportedAtRule` | `CssUnsupportedAtRuleError`: `at_rule() -> &CssIdentifier`, `metadata() -> Option<&'static CssUnsupportedFeatureMetadata>` |
| `InvalidMediaQuery` | `CssMediaQueryError`: `feature() -> Option<&CssIdentifier>`, `expected() -> &CssGrammarExpectation`, `encountered() -> Option<&CssTokenSummary>` |
| `InvalidSelector` | `CssSelectorError`: `production_id() -> Option<&CssProductionId>`, `expected() -> &CssGrammarExpectation`, `encountered() -> Option<&CssTokenSummary>` |
| `UndeclaredNamespacePrefix` | `CssNamespacePrefixError`: `prefix() -> &CssNamespacePrefix` |
| `UnknownProperty` | `CssUnknownPropertyError`: `property_name() -> &CssIdentifier` |
| `InvalidPropertyValue` | `CssPropertyValueError`: `property() -> CssKnownProperty`, `expected() -> &CssGrammarExpectation`, `encountered() -> Option<&CssTokenSummary>` |
| `InvalidDeclarationAnnotation` | `CssDeclarationAnnotationError`: `context() -> CssDeclarationAnnotationContextRef<'_>`, `encountered() -> &CssTokenSummary` |
| `UnknownDescriptor` | `CssUnknownDescriptorError`: `at_rule() -> &CssIdentifier`, `descriptor() -> &CssIdentifier` |
| `InvalidDescriptorValue` | `CssDescriptorValueError`: `at_rule() -> &CssIdentifier`, `descriptor() -> &CssIdentifier`, `expected() -> &CssGrammarExpectation`, `encountered() -> Option<&CssTokenSummary>` |
| `InvalidDescriptorCombination` | `CssDescriptorCombinationError`: `at_rule() -> &CssIdentifier`, `descriptor() -> &CssIdentifier`, `conflicting_descriptors() -> &[CssIdentifier]` |
| `InvalidColorSyntax` | `CssColorSyntaxError`: `component() -> Option<&CssIdentifier>`, `expected() -> &CssGrammarExpectation`, `encountered() -> Option<&CssTokenSummary>` |

`CssDeclarationAnnotationContextRef<'a>` shall be a public
`#[non_exhaustive]` borrowed enum with `Property(CssPropertyNameRef<'a>)`,
`Keyframe`, `SupportsDeclaration(&'a CssSupportsPropertyName)`,
`FontFaceDescriptor(&'a CssIdentifier)`, and
`CounterStyleDescriptor(&'a CssIdentifier)` variants. It reports the authored
grammar that rejected the annotation; it does not turn descriptors into ordinary
declarations.

`CssProductionId` and `CssGrammarExpectation` are public private-field semantic
newtypes with `as_str() -> &'static str`. They are created only from static
catalog and grammar records. `CssTokenSummary` is a public private-field
diagnostic value with `kind() -> CssTokenKind` and `authored() -> &str`;
`CssTokenKind` is a public `#[non_exhaustive]` semantic enum with `Identifier`,
`Function`, `AtKeyword`, `Hash`, `String`, `Url`, `Number`, `Percentage`,
`Dimension`, `Delimiter`, `Colon`, `Semicolon`, `Comma`, `Whitespace`, `Comment`,
`OpenBlock`, `CloseBlock`, `Cdo`, `Cdc`, `BadString`, and `BadUrl` variants. An absent
`encountered()` value means end of input, never an unavailable diagnostic. The
optional metadata on `UnsupportedAtRule` is present for a cataloged
unsupported at-rule and absent for an unknown valid at-keyword. The optional
media feature, selector production, and color component are absent only for a
failure of the enclosing grammar rather than one named subproduction.

All detail fields and constructors are private. No detail type combines unrelated
optional property, descriptor, selector, media, and color fields into a generic
diagnostic bag. The existing free-form catch-all CSS reason variant shall be
removed. `Error` exposes `kind() -> &ErrorKind`,
`code() -> CssErrorCode`, and `position() -> CssSourcePosition`; its source
position is the first responsible token, or the expected end position for
premature input exhaustion. The total mapping is one-to-one by root variant:
every `ErrorKind::X` maps to `CssErrorCode::X` in `Error::code()`. No error path
may bypass that mapping.

Both enums are non-exhaustive so a later release may add a new structured detail
and, when no existing category is truthful, a new code without invalidating
wildcard-compatible callers. Existing code meanings and mappings shall not be
repurposed. Public examples and integration tests shall demonstrate wildcard
matching and control flow through `Error::code()`.

Dynamic prose remains for display. Tests and downstream control flow shall match
typed categories and structured payloads, never reason strings. Unknown
descriptor errors point at the descriptor name; invalid descriptor
values point at the first responsible value token; descriptor annotations point
at `!`; and an invalid cross-descriptor combination points at the effective
descriptor whose choice makes the combination invalid. Specific property/value
errors preserve the canonical property identity.

### 5.3 Validated Recovery Report

Browser-compatible parsing shall return a `CssParseReport<T>` containing one
validated syntax result and every recovery diagnostic in source order. The
public private-field generic report exposes `syntax() -> &T`,
`diagnostics() -> &[CssRecoveryDiagnostic]`, `is_clean() -> bool`, and
`into_parts() -> (T, Vec<CssRecoveryDiagnostic>)`. It executes no caller callback,
logs nothing, and has no API that labels a recovered source as valid merely
because its retained tree is valid.

`CssRecoveryDiagnostic` has private fields and exposes
`error() -> &Error`, `span() -> CssSourceSpan`, and
`action() -> CssRecoveryAction`. `CssRecoveryAction` is public and
`#[non_exhaustive]` with these initiative-completion variants:

```rust
pub enum CssRecoveryAction {
    DropDeclaration,
    DropDescriptor,
    DropQualifiedRule,
    DropAtRule,
    DropKeyframeBlock,
    DropSelectorListItem,
    ReplaceMediaQueryWithNever,
    RetainWithImplicitClosure,
    IgnoreLegacyToken,
    StopAtNestingLimit,
}
```

The diagnostic's `Error` points to the first responsible token while its span
covers the complete discarded, replaced, retained-after-repair, or ignored
source unit. A report may contain multiple diagnostics with nested spans; their
order is the first responsible source position followed by the order in which
the recovery algorithm discovers equal-position failures. Diagnostics are never
deduplicated by display text.

Every syntax node in a report independently satisfies the same private
constructor invariants as a clean parse. There is no `Raw`, unknown-token,
unchecked-text, invalid-node, or partially parsed branch. Authored text remains
public only where its grammar explicitly owns token preservation, including
custom-property values, variable fallbacks, general-enclosed conditions, and
defined-false media expressions; those parsers still reject their grammar's bad
tokens and unbalanced structure.

Grammar-defined feature detection is not recovery. Unknown media types and
syntactically complete unknown media features or media-feature values, plus
Conditional 3 general-enclosed supports tests, are accepted as authored
conditions because their owning grammars explicitly define those forms. They
produce no recovery diagnostic.

### 5.4 Browser-Compatible Recovery Boundaries

The default parser shall implement the recovery behavior of the pinned CSS
Syntax and owning module grammars. It shall use these exact recovery units:

| Failure context | Required browser-compatible result |
| --- | --- |
| Unknown, unsupported, misplaced, or malformed top-level or nested at-rule | Consume through the at-rule's specification-defined semicolon or balanced block and emit `DropAtRule`; later sibling rules remain eligible. |
| Invalid style-rule selector list or malformed qualified-rule structure | Consume the complete balanced qualified rule and emit `DropQualifiedRule`; no declaration from that rule escapes. |
| Unknown property, unsupported property, invalid property value, invalid annotation, or malformed declaration | Consume through the declaration's top-level semicolon or containing block end and emit `DropDeclaration`; the containing style, page, keyframe, or style-attribute list remains eligible. |
| Unknown or invalid descriptor | Consume through the descriptor boundary and emit `DropDescriptor`; the at-rule remains only when its surviving authored descriptor set is itself representable under the owning grammar. |
| Invalid keyframe selector or malformed keyframe block | Consume the complete keyframe block and emit `DropKeyframeBlock`; later blocks remain eligible. Invalid declarations inside a valid block use declaration recovery instead. |
| Invalid member of Selectors 4 `<forgiving-selector-list>` | In `:is()` and `:where()` only, consume that comma-delimited member and emit `DropSelectorListItem`; preserve remaining members, including an empty valid result. Every other selector list, including style rules, `:not()`, `:has()`, nth `of`, scope boundaries, and nesting, is unforgiving and invalidates its containing selector or rule. |
| Syntactically malformed Media Queries 3 query-list member | Consume through that member's comma boundary while respecting balanced blocks, retain a typed guaranteed-false query sentinel, and emit `ReplaceMediaQueryWithNever`; later list members remain eligible. |
| End of input with an open rule, block, function, or string form that CSS Syntax implicitly closes and whose owning grammar is otherwise complete | Retain the validated result and emit `RetainWithImplicitClosure` with a zero-width end-of-input span. If the completed owning grammar still fails, apply its ordinary drop rule instead. |
| Top-level legacy CDO/CDC token ignored by CSS Syntax | Emit `IgnoreLegacyToken` for each ignored token and continue. Comments and whitespace are not diagnostics. |

Recovery shall never reinterpret tokens from one failed unit as part of a later
unit, cross a balanced block boundary, or retain children from a dropped parent.
When dropping a child leaves a parent unable to satisfy its own modeled grammar,
the parser emits the child diagnostic first, then a diagnostic for the smallest
unrepresentable parent and drops that parent. Valid siblings before and after it
remain in source order.

Every recovery loop records its starting byte position. One iteration must
produce a node, advance by at least one source byte, or terminate the current
bounded input. The parser shall enforce a documented nesting limit before
recursive descent; reaching it emits `StopAtNestingLimit`, discards the smallest
balanced enclosing construct available, and never constructs a partial node.
Input-driven paths shall not use `unwrap`, `expect`, unchecked indexing, or an
`unreachable!` assumption over dependency output. Allocation failure and process
abort are outside the Rust panic contract; all ordinary `&str` input is otherwise
required to return a report without unwinding.

### 5.5 Additive Application-Strict Validation

Cargo feature `app-strict` shall add, rather than alter, public validation entry
points. Enabling it shall not change the signature, recovery behavior, syntax
tree, diagnostics, or ordering of either browser-compatible parsing function.
This additive rule prevents Cargo feature unification from silently changing a
dependency's parsing semantics.

The feature exposes `validate_sheet` and `validate_style_attribute`. Each runs
the same browser-compatible parser exactly once. A clean report returns its
syntax value. A report with one or more diagnostics discards the syntax value and
returns `CssValidationFailure`, whose private non-empty diagnostic vector is
exposed through `diagnostics() -> &[CssRecoveryDiagnostic]`,
`first() -> &CssRecoveryDiagnostic`, and
`into_diagnostics() -> Vec<CssRecoveryDiagnostic>`. No second grammar, recovery
implementation, or feature-dependent syntax variant is permitted.

## 6 Stylesheet State And At-Rule Ordering

### 6.1 Leading Encoding Declaration

Before constructing `cssparser::StyleSheetParser`, `parse_sheet` shall inspect the
optional leading encoding declaration. It shall validate the complete legacy
`@charset "<label>";` form, preserve the non-empty label in a typed
`CssEncodingDeclaration`, and include it as optional sheet metadata rather than
as an ordinary rule. It shall not perform byte decoding because the API already
receives UTF-8 Rust text.

A malformed leading `@charset`, missing semicolon, unquoted label, duplicate
declaration, or non-leading `@charset` is an error. Leading BOM and comment
behavior shall follow CSS Syntax 3. No malformed form may be silently consumed by
the dependency.

### 6.2 Top-Level Phase Machine

The top-level parser shall use a private phase enum rather than one
`imports_allowed` boolean. It shall use exactly these dynamic phases:

- `InitialPrelude`: neither an import nor namespace has appeared; empty layer
  statements are transparent and both families remain possible;
- `Imports`: at least one import has appeared; further imports must be
  consecutive, and namespaces may follow them directly;
- `Namespaces`: at least one namespace has appeared; only further consecutive
  namespaces remain possible;
- `Body`: imports and namespaces are permanently closed.

The exact transition table is:

| Current phase | `@import` | empty `@layer` statement | `@namespace` | any other valid rule, including an `@layer` block |
| --- | --- | --- | --- | --- |
| `InitialPrelude` | accept, move to `Imports` | accept, stay | accept, move to `Namespaces` | accept, move to `Body` |
| `Imports` | accept, stay | accept, move to `Body` | accept, move to `Namespaces` | accept, move to `Body` |
| `Namespaces` | reject placement | accept, move to `Body` | accept, stay | accept, move to `Body` |
| `Body` | reject placement | accept, stay | reject placement | accept, stay |

Thus initial empty layer statements may precede either family:
`@layer a; @import "x.css";`, `@layer a; @namespace "u";`, and
`@layer a; @import "x.css"; @namespace "u";` are valid. Once an import or
namespace has appeared, an intervening layer statement is itself valid but
closes both prelude families. Consequently `@import "x.css"; @layer a;
@import "y.css";`, `@import "x.css"; @layer a; @namespace "u";`, and
`@namespace "u"; @layer a; @namespace x "v";` reject at the final prelude
rule. Imports after namespaces, and either family after a body rule or layer
block, are always invalid. Imports and namespaces are top-level only.

The ordering vector table shall cover repeated initial layer statements,
consecutive imports, consecutive namespaces, import-then-namespace, each of the
three accepted initial-layer examples above, a layer statement that validly ends
the prelude when followed only by body rules, and every rejected permutation
named above. Placement errors point at the rejected import or namespace, not at
the preceding valid layer statement.

The phase transition occurs only after a rule parses successfully. Failed rules
do not leave partially advanced state.

### 6.3 Rule Model

`CssRule` shall gain typed variants for:

- `Namespace(CssNamespaceRule)`;
- `Supports(CssSupportsRule)`;
- `CounterStyle(CssCounterStyleRule)`;
- `Page(CssPageRule)`.

Existing variants remain authored and typed. Group rules shall use a shared
strict nested-rule parser configured by the exact allowed nested at-rule set.
Rules restricted by the matrix shall fail outside their legal contexts. Empty group-rule bodies are
accepted where their grammar permits them.

The shared parser shall use these finite private contexts. `Group` means a
`@media`, `@supports`, `@container`, `@layer` block, or `@scope` body not nested
through a style rule. `NestedGroupInStyle` means the same group family with a
nearest ancestor style selector carried for Nesting 1 flattening.

| Authored child family | Top level | Group | Style block | Nested group in style | `@keyframes` body | Keyframe block | `@font-face` block | `@counter-style` block | `@page` block |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `@import` | 6.2 phase only | reject | reject | reject | reject | reject | reject | reject | reject |
| `@namespace` | 6.2 phase only | reject | reject | reject | reject | reject | reject | reject | reject |
| `@layer` statement | accept | accept | reject | accept | reject | reject | reject | reject | reject |
| Group block: `@media`, `@supports`, `@container`, `@layer`, `@scope` | accept | accept | accept and flatten | accept and flatten | reject | reject | reject | reject | reject |
| Global body at-rule: `@font-face`, `@keyframes`, `@counter-style`, `@page` | accept | accept | reject | accept without selector prefixing | reject | reject | reject | reject | reject |
| Style qualified rule | accept | accept | accept and flatten | accept and flatten | reject | reject | reject | reject | reject |
| Ordinary declaration | reject | reject | accept | accept with carried selector | reject | reject | reject | reject | accept |
| Keyframe selector block | reject | reject | reject | reject | accept | reject | reject | reject | reject |
| Keyframe declaration | reject | reject | reject | reject | reject | accept | reject | reject | reject |
| Font-face descriptor | reject | reject | reject | reject | reject | reject | accept | reject | reject |
| Counter-style descriptor | reject | reject | reject | reject | reject | reject | reject | accept | reject |

An encoding declaration is handled only by 6.1 and is never a matrix child.
Conditional group rules and layer blocks accept every otherwise top-level body
rule, matching Conditional 3/Cascade 5; scope uses the same rule-list allowance,
while its parser state changes descendant selector interpretation. A group
nested through a style additionally accepts direct declarations under Nesting 1.
The `@page` block deliberately accepts only ordinary declarations because later
margin-box rules are outside the catalog.

Nesting flattening is deterministic and source-order preserving. A style block
is split into maximal declaration runs around nested rules; each non-empty run
emits one `CssStyleRule` with the carried parent selector at that source position.
A nested style rule combines its selector with the nearest carried parent and is
emitted in place. A nested group block is emitted in place and parses its body as
`NestedGroupInStyle`; direct declarations and nested style rules inside it receive
the carried selector, while global body at-rules and layer statements do not.
Empty declaration runs emit no rule. Repeating this transition recursively
eliminates every Nesting 1 `&` and leaves no nested child field on
`CssStyleRule`, while preserving the public group-rule tree.

Every rejected at-rule cell returns `InvalidAtRulePlacement` at its `@` token.
Late top-level imports/namespaces use the same code and the 6.2 position rule.
A recognized-unsupported at-rule is checked for legal placement first:
`@font-feature-values` at top level, in `Group`, or in
`NestedGroupInStyle` returns `UnsupportedAtRule` with metadata, while the same
spelling directly in a style block returns `InvalidAtRulePlacement`. An unknown
valid at-keyword returns `UnsupportedAtRule` with no metadata in any
rule-list/style-block context.
Disallowed non-at-rule child families return `InvalidAtRuleBody` at their first
token. Descriptor/value grammar failures retain their more specific 5.2 code.

Table-driven vectors shall exercise every cell, every group kind in both group
columns, nested groups at two depths, global rules inside a nested group, mixed
declaration runs before/between/after nested rules, empty runs, and exact output
order/selectors/positions after flattening. No recovery drops a disallowed child.

## 7 Selectors And Namespaces

### 7.1 Identifier And Name Types

`CssIdentifier` shall be an opaque validated decoded CSS identifier. Its fallible
constructor shall validate through `cssparser` tokenization and complete-input
exhaustion. It shall reject empty strings, non-ident tokens, and non-finite or
structural spellings. Selector tags, IDs, classes, namespace prefixes, attribute
identifiers, custom identifiers, counter names, and other identifier-bearing
public models shall use this type or a stricter semantic wrapper.

Compact public selector variants containing arbitrary `String` values shall be
removed. A selector shall be represented only as a compound or complex selector
whose construction validates all components.

### 7.2 Namespace Rules And Qualified Names

`CssNamespaceRule` shall preserve an optional validated prefix, one
`CssNamespaceTarget`, and source position. The target is a public
`#[non_exhaustive]` enum with `String(CssNamespaceValue)` and
`Url(CssNamespaceValue)` variants so the authored token form remains inspectable.
Its payload newtype has private storage. `CssNamespaceValue` is a decoded string
that deliberately permits the empty value; `as_str()` exposes it and
`is_null_namespace()` is true exactly when it is empty. Thus `@namespace "";`,
`@namespace url("");`, and their prefixed forms are valid declarations of the
null namespace. Empty targets are not confused with a missing target.

The parser shall maintain the active prefix set needed to validate selector
syntax. It shall not resolve or load the URI. String and URL targets, including
both null-namespace spellings, receive accepted AST vectors; missing targets,
malformed URL syntax, and trailing tokens receive exact rejection vectors.

Type, universal, and attribute selectors shall use `CssQualifiedName` with one of
these namespace constraints:

- default namespace where the selector grammar applies it;
- explicit no namespace (`|name`);
- any namespace (`*|name`);
- named prefix (`prefix|name`).

An undeclared named prefix is an `UndeclaredNamespacePrefix` error. Attribute
selectors do not inherit the default namespace. Namespace declaration order,
empty/default prefixes, escapes, and URI/string targets follow Namespaces 3.

### 7.3 Complete Selectors 3 Surface

The parser and authored AST shall support every Selectors 3 simple selector,
combinator, pseudo-class, and pseudo-element, including:

- type and universal selectors with namespace forms;
- ordered repeated ID selectors and ordered repeated class selectors;
- every attribute matcher and permitted case syntax;
- `:link`, `:visited`, `:target`, and `:lang()`;
- all Selectors 3 structural and UI state pseudo-classes;
- `::first-line` and `::first-letter` plus legacy single-colon spellings allowed
  by Selectors 3;
- descendant, child, adjacent sibling, and subsequent sibling combinators.

`CssCompoundSelector` shall store IDs as an ordered collection. It shall never
overwrite an earlier occurrence. Type/universal selector cardinality and
pseudo-element terminal rules shall be unconstructable through public APIs.

The successful public selector tree shall use exactly this structural shape;
the current compact `CssSelector::{Tag, Key, Class, PseudoClass, Compound,
Complex}` representation and the scoped-selector duplicate family shall be
removed:

```rust
pub struct CssSelectorList { /* non-empty ordered CssComplexSelector values */ }
pub struct CssStyleSelectorList { /* non-empty ordered CssStyleSelector values */ }
pub struct CssComplexSelector { /* first compound plus ordered parts */ }
pub struct CssComplexSelectorPart { /* combinator plus following compound */ }
pub struct CssCompoundSelector { /* optional type plus ordered simple selectors
                                    plus optional terminal pseudo sequence */ }

#[non_exhaustive]
pub enum CssTypeSelector {
    Named(CssQualifiedName),
    Universal(CssNamespaceConstraint),
}

#[non_exhaustive]
pub enum CssSimpleSelector {
    Id(CssIdentifier),
    Class(CssIdentifier),
    Attribute(CssAttributeSelector),
    PseudoClass(CssPseudoClass),
    ScopeAnchor,
}

#[non_exhaustive]
pub enum CssStyleSelector {
    Absolute(CssComplexSelector),
    ScopeRelative(CssRelativeSelector),
}
```

`CssSelectorList::as_slice() -> &[CssComplexSelector]` exposes list order.
`CssStyleSelectorList::as_slice() -> &[CssStyleSelector]` is the one selector
contract used by `CssStyleRule`: outside `@scope` every item is `Absolute`, while
every style rule under an active innermost `@scope`, including through a nested
group or layer, stores every item as `ScopeRelative`. This replaces the prior
scoped-style rule/list duplicate types without erasing the scoped-relative
grammar distinction.
`CssComplexSelector` exposes `first() -> &CssCompoundSelector` and
`parts() -> &[CssComplexSelectorPart]`; each part exposes
`combinator() -> CssSelectorCombinator` and
`compound() -> &CssCompoundSelector`. `CssCompoundSelector` exposes
`type_selector() -> Option<&CssTypeSelector>`,
`simple_selectors() -> &[CssSimpleSelector]`, and
`pseudo_elements() -> Option<&CssPseudoElementSequence>`. The simple-selector
slice preserves the authored interleaving of IDs, classes, attributes, and
pseudo-classes. A Nesting 1 `&` is a parser-owned transient consumed by the
required flattening and never survives in this successful tree; `ScopeAnchor`
is retained only where Cascade 6 makes it authored scope syntax.

`CssQualifiedName` exposes `namespace() -> &CssNamespaceConstraint` and
`local_name() -> &CssIdentifier`. `CssNamespaceConstraint` is a public
`#[non_exhaustive]` enum with `Default`, `NoNamespace`, `Any`, and
`Named(CssNamespacePrefix)` variants. A universal selector stores only its namespace
constraint. `CssIdentifier` exposes its decoded value through `as_str() -> &str`;
`CssString` is the corresponding private-field decoded CSS string value with the
same accessor.

Attribute selector state shall not be split across independent optional matcher,
value, and modifier fields. It uses this validated shape:

```rust
pub struct CssAttributeSelector { /* qualified name plus test */ }

#[non_exhaustive]
pub enum CssAttributeTest {
    Exists,
    Value(CssAttributeValueTest),
}

pub struct CssAttributeValueTest { /* operator, value, modifier */ }

#[non_exhaustive]
pub enum CssAttributeValue {
    Identifier(CssIdentifier),
    String(CssString),
}
```

`CssAttributeSelector` exposes `name() -> &CssQualifiedName` and
`test() -> &CssAttributeTest`. `CssAttributeValueTest` exposes
`operator() -> CssAttributeMatchOperator`, `value() -> &CssAttributeValue`, and
`modifier() -> CssAttributeModifier`. The operator enum has `Equals`,
`Includes`, `DashMatch`, `Prefix`, `Suffix`, and `Substring`; the modifier enum
has `Default`, `AsciiCaseInsensitive`, and `ExplicitSensitive`. Both are public
and non-exhaustive under 12.1.

Functional pseudo-class payloads use the same full tree. `:is()`, `:where()`,
and the Selectors 4 form of `:not()` carry `CssSelectorList`; `:has()` carries
`CssRelativeSelectorList`; `:lang()` carries `CssLanguageRangeList`; and the
nth-child variants carry `CssNthChildPattern`, whose
`selector_list() -> Option<&CssSelectorList>` exposes the optional `of` list.
Every accepted no-argument pseudo-class named by 7.3/7.4 has a same-named
public `CssPseudoClass` unit variant. `CssRelativeSelectorList::as_slice()`
returns `&[CssRelativeSelector]`; each relative selector exposes
`leading_combinator() -> Option<CssSelectorCombinator>` and
`selector() -> &CssComplexSelector`. `None` means the grammar's implicit
descendant relation and is not rewritten to an explicit token. Language ranges
are non-empty validated values with `as_str() -> &str` and the list has the
standard ordered-collection accessors.

`CssPseudoElement` has one public variant for every accepted pseudo-element in
7.3 and 3.5. `CssPseudoElementSequence::as_slice() -> &[CssPseudoElement]`
preserves the accepted authored sequence. Selector aggregate fields and
constructors remain private; only context-free scalar identifier/string
validation may have a public fallible constructor.

### 7.4 Existing Selectors 4 Extensions

The existing strict support for `:is`, `:where`, `:not`, `:has`, nth-child `of`
filters, scope anchors, attributes, combinators, and documented UI/overlay
pseudo-classes shall remain. Functional selector lists shall use the full
compound/complex selector representation already required by the project.

No selector-list context is forgiving. One malformed or unsupported argument
rejects the complete sheet. Relative selectors preserve their optional leading
combinator. Pseudo-elements remain forbidden in argument contexts where the
owning selector grammar forbids them.

### 7.5 Scope Selector Contexts

`@scope` shall use an explicit selector-context stack separate from the nesting
rule-list context in 6.3. Scope start and limit lists are unforgiving
`<selector-list>` values; one invalid member rejects the complete sheet. Every
pseudo-element sequence anywhere in either list is `InvalidSelector` at the
first pseudo-element token. The finite binding matrix is:

| Authored selector context | Accepted grammar | `&` binding | `:scope` binding | Successful public form |
| --- | --- | --- | --- | --- |
| Scope start with no enclosing style or scope | `<selector-list>` | unbound and therefore invalid | ordinary outer selector context | `CssSelectorList` |
| Scope start inside an outer scope, but not a style rule | `<selector-list>` relative to the outer scope | outer scope start, or `:scope` when that start was omitted | outer scope root | composed `CssSelectorList` |
| Scope start inside a style rule | `<selector-list>` relative to the nearest style selector | nearest carried parent style selector | innermost outer scope root when one exists; otherwise ordinary outer selector context | composed `CssSelectorList` |
| Scope limit | `<selector-list>` | current scope start, or `:scope` when that start was omitted | current scope root | `CssSelectorList` |
| Direct style rule in a scope | `<relative-selector-list>` | current scope start, or `:scope` when that start was omitted | current scope root | every item is `CssStyleSelector::ScopeRelative` |
| Style rule reached through groups in a scope | `<relative-selector-list>` | same current scope binding | same current scope root | every item is `CssStyleSelector::ScopeRelative` |
| Nested style rule below a scoped style rule | Nesting 1 nested selector list | nearest parent style selector | current scope root | combined and flattened `CssStyleSelector::ScopeRelative` |
| Style rule in a nested scope body | `<relative-selector-list>` | innermost new scope start, or its `:scope` default | innermost new scope root | every item is `CssStyleSelector::ScopeRelative` |

A nested scope start is always parsed in its outer row before the new scope is
pushed. Its limit and body then use the new innermost scope. Consequently a
scope nested through a style rule composes a start-list `&` with the carried
parent selector, while a scope nested only through another scope interprets
`&` against the outer scope start. Once the nested rule is entered, neither
outer binding leaks into its limit or scoped style selectors.

Composition applies even when the nested scope start contains no explicit
anchor. Each such selector receives the nesting context's implicit descendant
relation, the parser computes the ordered outer-by-inner selector product, and
`CssScopeRule::root()` exposes the resulting non-relative `CssSelectorList`.
When the outer base is itself a `ScopeRelative` style selector, composition
first materializes its implicit `:scope` descendant relation; when an enclosing
scope omitted its start, the materialized base is `:scope`. This deterministic
scope-nesting flattening preserves selector order and specificity-bearing
components, leaves the `CssScopeRule` tree intact, and performs no matching or
scope-proximity calculation.

An `&` bound to a parent style selector is a Nesting 1 transient and is removed
by 6.3 flattening, including when it occurs in a nested scope start. An `&`
bound to a scope start is authored scope syntax and remains
`CssSimpleSelector::ScopeAnchor`. `:scope` always remains the same-named
`CssPseudoClass` node; its current-root meaning is conveyed by the containing
`CssScopeRule` tree rather than rewritten into another selector. Scoped style
selectors with or without an explicit leading combinator remain
`ScopeRelative`; no spelling is reclassified as `Absolute` merely because it
contains `:scope` or a scope anchor. Outside every scope, successful style-rule
items are `Absolute` after Nesting 1 flattening.

Vectors shall cover every matrix row, omitted and explicit starts, omitted and
explicit limits, unbound top-level `&`, pseudo-elements at every list depth,
leading combinators, `&` and `:scope` separately and together, groups inside
scopes, scopes inside groups, scopes inside styles, two nested scopes, and a
nested style inside each. Public assertions shall inspect root/limit lists,
`Absolute` versus `ScopeRelative`, surviving scope anchors, eliminated nesting
anchors, and exact source order and positions.

## 8 Media, Supports, Import, And Container Conditions

### 8.1 Media Queries Level 3

The media-query parser shall implement all Media Queries 3 media types and media
features. Known media types include `all`, `aural`, `braille`, `embossed`,
`handheld`, `print`, `projection`, `screen`, `speech`, `tty`, and `tv` according
to the final Level 3 grammar. Other valid identifiers are preserved as
`CssMediaType::Unknown(CssIdentifier)` except the exact ASCII-case-insensitive
reserved set `layer`, `not`, `and`, `only`, and `or`. Using any member of that set
as the media type returns `InvalidMediaQuery` at that identifier and rejects the
complete sheet; it never constructs `Unknown`. Every other valid CSS identifier
constructs `Unknown` and carries the specification-defined false disposition.

The complete Level 3 feature set includes width, height, device width, device
height, orientation, aspect ratio, device aspect ratio, color, color index,
monochrome, resolution, scan, and grid. Each feature shall support exactly its
allowed value type, min/max prefixes, and boolean form. `grid` accepts its
boolean form and, when valued, only an integer token whose mathematical value is
zero (including `-0` and `+0`) or one (including `+1`); it accepts no min/max
prefix. Every other valued `grid` form is malformed. Ratios have positive finite
integer components. Resolution
uses positive finite values and the Level 3 units. Negative and non-finite values
cannot inhabit a known feature value; when their outer Level 3 expression syntax
is complete they follow the defined-false unknown-value contract below.

Media Queries 3 distinguishes a malformed query from a syntactically complete
query containing an unknown feature or unknown feature value. The authored model
shall preserve that distinction with this semantic shape:

```rust
#[non_exhaustive]
pub enum CssMediaFeatureExpression {
    Known(CssKnownMediaFeatureExpression),
    DefinedFalse(CssDefinedFalseMediaExpression),
}

#[non_exhaustive]
pub enum CssDefinedFalseMediaReason {
    UnknownFeature,
    UnknownValue,
}
```

`CssDefinedFalseMediaExpression` has private fields and accessors for the reason,
original feature identifier including any prefix, optional non-empty balanced
CSS2 `expr` token sequence, and source position. `UnknownFeature` covers an
identifier not in the implementation inventory, including a disallowed min/max
form such as `min-orientation`; its optional value is preserved. `UnknownValue`
carries the known canonical feature plus the complete balanced value whenever
the outer expression matches the Level 3 syntax but the value is not one the
feature defines, including a negative value in a non-negative domain. The query
AST retains the expression in source order and is not normalized to `not all`.
This crate does not evaluate it, but the typed reason records the specification's
defined-false disposition for downstream consumers.

The `grid` feature is the explicit Level 3 exception to that unknown-value path.
`(grid: 2)`, `(grid: -1)`, `(grid: 1.0)`, dimensions, percentages, identifiers,
multiple terms, and all other values outside the exact integer-zero-or-one set
return `InvalidMediaQuery`; none may construct `DefinedFalse(UnknownValue)`.
Vectors shall include `(grid)`, `0`, `-0`, `+0`, `1`, and `+1` as accepted forms
and one case from every malformed category above.

Unexpected tokens, unbalanced delimiters, a colon without a non-empty CSS2
`expr`, a min/max feature without its required value, malformed `and`/comma
structure, and trailing input remain parse errors for the complete sheet. They
shall never fall back to `DefinedFalse`, and this crate shall not perform the
browser recovery that discards a malformed query. These paths return
`InvalidMediaQuery` with the first deterministic malformed position. Vectors
shall distinguish all five reserved media-type identifiers,
unknown type, unknown feature, unknown value, and malformed syntax, and shall
inspect the preserved authored payload of each accepted unknown form.

Existing documented later media features and range syntax remain typed
extensions. Their parser is layered after the complete Level 3 grammar and may
not weaken Level 3 arity, prefix, separator, or unit rules.

The public authored condition tree has this exact branch structure:

```rust
#[non_exhaustive]
pub enum CssMediaQuery {
    Typed(CssTypedMediaQuery),
    Condition(CssMediaCondition),
}

#[non_exhaustive]
pub enum CssMediaCondition {
    Not(Box<CssMediaCondition>),
    And(CssMediaConditionList),
    Or(CssMediaConditionList),
    Parenthesized(Box<CssMediaCondition>),
    Feature(CssMediaFeatureExpression),
}
```

`CssMediaQueryList::as_slice() -> &[CssMediaQuery]` and
`CssMediaConditionList::as_slice() -> &[CssMediaCondition]` expose non-empty
source order. `CssTypedMediaQuery` exposes
`modifier() -> Option<CssMediaQueryModifier>`,
`media_type() -> &CssMediaType`,
`condition() -> Option<&CssMediaCondition>`, and
`position() -> CssSourcePosition`. Every `CssMediaType` branch is public,
including `Unknown(CssIdentifier)`. `CssKnownMediaFeatureExpression` is a
public `#[non_exhaustive]` enum with one typed payload variant per parser-facing
media-feature catalog entry; no generic token payload may represent a known
feature. Every such payload exposes its canonical feature identity, exact
boolean/plain/min/max/range authored form, grammar-specific typed value when
present, and source position. The defined-false payload separately exposes
`reason() -> CssDefinedFalseMediaReason`,
`feature() -> &CssIdentifier`,
`value() -> Option<&CssMediaExpressionTokens>`, and
`position() -> CssSourcePosition`; its token accessor is authored diagnostic
preservation, not a known-feature escape path.

### 8.2 Supports Conditions

`CssSupportsRule` shall preserve one typed `CssSupportsCondition`, nested rules,
and source position. Conditions shall model `not`, homogeneous `and`, homogeneous
`or`, parenthesized conditions, declaration tests, and the existing claimed
Conditional 4 `selector()` extension when included by the conformance catalog.
Mixing unparenthesized `and` and `or` is invalid.

The public condition enum and list are exactly:

```rust
#[non_exhaustive]
pub enum CssSupportsCondition {
    Not(Box<CssSupportsCondition>),
    And(CssSupportsConditionList),
    Or(CssSupportsConditionList),
    Parenthesized(Box<CssSupportsCondition>),
    Declaration(CssSupportsDeclaration),
    Selector(CssComplexSelector),
    GeneralEnclosed(CssSupportsGeneralEnclosed),
}
```

`CssSupportsConditionList::as_slice() -> &[CssSupportsCondition]` exposes a
non-empty homogeneous operand list. `Parenthesized` is retained as authored
structure instead of being normalized away. The `Selector` payload is the full
7 tree and remains strict.

The condition enum shall also contain
`GeneralEnclosed(CssSupportsGeneralEnclosed)` for Conditional 3's forward-
compatible `<general-enclosed>` production. The private-field public payload
distinguishes `Function { name, payload }` from `Parenthesized { payload }`.
Each payload is an optional balanced CSS Syntax 3 `<any-value>`; absence models
the production's empty form rather than an empty sentinel string. The function
name and source position remain inspectable. It is an authored condition with no
support truth value computed by this crate.

The parser shall choose known condition productions before the general-enclosed
branch according to the Conditional 3 grammar. A syntactically complete unknown
function or otherwise-unclaimed parenthesized form is accepted as
general-enclosed;
unbalanced input, a malformed declaration test, a malformed recognized
`selector()` form, invalid boolean operator structure, or trailing tokens is an
error and shall not be laundered through the fallback. Independent vectors shall
cover empty and non-empty payloads for both general-enclosed shapes and one-token
malformed mutations of each.

A declaration test is not an ordinary declaration. The private-field public
`CssSupportsDeclaration` shall preserve a validated property name, a required
colon, and one `CssSupportsDeclarationValue`. The value is a public
`#[non_exhaustive]` enum with `Empty` and
`Tokens(CssSupportsDeclarationTokens)` variants. The token payload has private
storage, is non-empty, satisfies CSS Syntax 3 `<declaration-value>`, and is
available through an authored-text accessor. This makes an empty value distinct
from a missing colon or property. `Empty` means no component value remains after
CSS whitespace and comments; surrounding trivia does not manufacture a token
payload.

The declaration test shall not invoke an ordinary property parser as an
acceptance gate. It shall preserve the declaration grammar's optional terminal
annotation through a closed public
`CssSupportsAnnotation` enum with `Absent` and `Important` variants.
`CssSupportsDeclaration` exposes
`property_name() -> &CssSupportsPropertyName`,
`value() -> &CssSupportsDeclarationValue`,
`annotation() -> CssSupportsAnnotation`, and
`position() -> CssSourcePosition`. `CssSupportsDeclarationTokens` exposes
`authored() -> &str`. One terminal ASCII-case-
insensitive `!important`, with CSS whitespace/comments in permitted positions,
is removed from the value before constructing `Empty` or `Tokens` and stored as
`Important`. It is semantically inert for the support test and is deliberately
distinct from ordinary declaration importance.

Thus `@supports (display:)` and `@supports (display: !important)`, a recognized
property with an unsupported value, a recognized unsupported property, and an
unknown property are all valid capability tests when their declaration-test
syntax is complete. A missing property, missing colon, bare `!`, misspelled or
duplicate annotation, nonterminal top-level `!`, malformed token, or trailing
condition token is invalid. The model preserves accepted tests for downstream
evaluation and computes no support truth value. This grammar-defined capability
test is not browser recovery.

`CssSupportsGeneralEnclosed` is a public `#[non_exhaustive]` enum with
`Function(CssSupportsGeneralEnclosedFunction)` and
`Parenthesized(CssSupportsGeneralEnclosedParenthesized)` variants. The function
payload exposes `name() -> &CssIdentifier`,
`payload() -> Option<&CssAnyValueTokens>`, and
`position() -> CssSourcePosition`; the parenthesized payload exposes the latter
two accessors. `CssAnyValueTokens::authored() -> &str` is the only raw authored
payload view and its absence, rather than an empty string, represents the empty
grammar form.

`CssSupportsPropertyName` is a public `#[non_exhaustive]` enum with
`Known(CssKnownProperty)`, `Custom(CssCustomPropertyName)`, and
`Unknown(CssIdentifier)` variants. This capability-test-only type preserves all
three syntactically valid cases without weakening `CssPropertyNameRef`, whose
ordinary-declaration invariant excludes unknown properties.

### 8.3 Import Conditions

`CssImportRule` shall preserve, in grammar order, target, optional layer clause,
optional `supports()` condition, optional media query list, and source position.
The supports clause shall use this exact public authored branch model:

```rust
#[non_exhaustive]
pub enum CssImportSupportsClause {
    BareDeclaration(CssSupportsDeclaration),
    Condition(CssSupportsCondition),
}
```

`supports(display: flex)` is `BareDeclaration`; the bare declaration parser
applies the same property-name, required-colon, and empty-or-token value model as
a declaration test, including its terminal inert annotation.
`supports(display:)` is therefore a valid `BareDeclaration` with an `Empty`
value, and `supports(display: !important)` adds `Important`.
`supports((display: flex))` and
`supports(not (display: flex))` are `Condition` and reuse the complete 8.2
condition grammar. The two branches remain distinguishable because this crate
models authored grammar, even though Cascade 5 defines the bare declaration as
semantically equivalent to adding parentheses. A malformed attempt in either
branch shall not fall through to the other or to general-enclosed.

The only accepted clause order is target, optional layer clause, optional
supports clause, optional media query list. Invalid ordering, duplicate clauses,
malformed conditions, missing bare-declaration properties or colons, and trailing
tokens reject the sheet. Import targets remain authored contracts; no loading
occurs. Vectors shall cover both supports branches, empty and non-empty values,
absent and important annotations, their equivalent declaration payload, all
clause-order boundaries, and malformed one-token mutations.

### 8.4 Container Conditions

The existing container-query extension shall expose the same authored boolean
structure rather than flattened flags:

```rust
#[non_exhaustive]
pub enum CssContainerCondition {
    Not(Box<CssContainerCondition>),
    And(CssContainerConditionList),
    Or(CssContainerConditionList),
    Parenthesized(Box<CssContainerCondition>),
    SizeFeature(CssContainerSizeFeatureExpression),
    Style(CssContainerStyleQuery),
}

#[non_exhaustive]
pub enum CssContainerStyleQuery {
    Exists(CssCustomPropertyName),
    Equals(CssContainerStyleDeclaration),
}
```

`CssContainerConditionList::as_slice() -> &[CssContainerCondition]` exposes a
non-empty homogeneous list. `CssContainerSizeFeatureExpression` is a public
`#[non_exhaustive]` enum with one grammar-specific typed payload variant for
each cataloged size feature; those payloads expose canonical feature identity,
exact boolean/plain/range authored form, typed value, and source position.
`CssContainerStyleDeclaration` exposes
`name() -> &CssCustomPropertyName`,
`value() -> &CssCustomPropertyValue`, and
`position() -> CssSourcePosition`. It is a style-query payload, not an ordinary
declaration, and cannot carry `!important`. Parenthesized structure and source
operand order remain inspectable; evaluation against a query container remains
downstream.

## 9 Snapshot At-Rules

### 9.1 Counter Styles

`CssCounterStyleRule` shall preserve a validated non-reserved counter-style name
and typed descriptor set. It shall support the Counter Styles 3 descriptors
`system`, `negative`, `prefix`, `suffix`, `range`, `pad`, `fallback`, `symbols`,
`additive-symbols`, and `speak-as` with descriptor-specific value types.

The public descriptor occurrence model is exact:

```rust
pub struct CssCounterStyleDescriptor { /* typed value plus position */ }

#[non_exhaustive]
pub enum CssCounterStyleDescriptorValue {
    System(CssCounterSystem),
    Negative(CssCounterNegative),
    Prefix(CssCounterPrefix),
    Suffix(CssCounterSuffix),
    Range(CssCounterRangeValue),
    Pad(CssCounterPad),
    Fallback(CssCounterStyleName),
    Symbols(CssCounterSymbols),
    AdditiveSymbols(CssCounterAdditiveSymbols),
    SpeakAs(CssCounterSpeakAs),
}
```

`CssCounterStyleDescriptorKind` is a public `#[non_exhaustive]` enum with the
same ten variant names. `CssCounterStyleDescriptor` derives its kind from its
value and exposes `kind() -> CssCounterStyleDescriptorKind`,
`value() -> &CssCounterStyleDescriptorValue`, and
`position() -> CssSourcePosition`; it does not store an independently mutable
kind discriminator.

Every counter descriptor payload is publicly inspectable through this exact
private-construction model:

```rust
#[non_exhaustive]
pub enum CssCounterSystem {
    Cyclic,
    Numeric,
    Alphabetic,
    Symbolic,
    Additive,
    Fixed(CssCounterFixedSystem),
    Extends(CssCounterStyleName),
}

#[non_exhaustive]
pub enum CssCounterSymbol {
    String(CssString),
    Image(CssImage),
    Identifier(CssCustomIdent),
}

#[non_exhaustive]
pub enum CssCounterRangeValue {
    Auto,
    Explicit(CssCounterRangeList),
}

#[non_exhaustive]
pub enum CssCounterRangeLowerBound {
    NegativeInfinity,
    Integer(CssInteger),
}

#[non_exhaustive]
pub enum CssCounterRangeUpperBound {
    Integer(CssInteger),
    PositiveInfinity,
}

#[non_exhaustive]
pub enum CssCounterSpeakAs {
    Auto,
    Bullets,
    Numbers,
    Words,
    SpellOut,
    Reference(CssCounterStyleName),
}
```

`CssCounterFixedSystem::start() -> Option<CssInteger>` preserves whether the
optional starting integer was authored. `CssCounterNegative` exposes
`prefix() -> &CssCounterSymbol` and
`suffix() -> Option<&CssCounterSymbol>`; `CssCounterPrefix` and
`CssCounterSuffix` each expose `symbol() -> &CssCounterSymbol`.
`CssCounterRangeList::as_slice() -> &[CssCounterRange]`; each range exposes
`lower() -> CssCounterRangeLowerBound` and
`upper() -> CssCounterRangeUpperBound`, and construction enforces lower not
greater than upper. `CssCounterPad` exposes
`width() -> CssNonNegativeInteger` and `symbol() -> &CssCounterSymbol`.
`CssCounterSymbols::as_slice() -> &[CssCounterSymbol]` is non-empty.
`CssCounterAdditiveSymbols::as_slice() -> &[CssCounterAdditiveSymbol]` is
non-empty; each item exposes `weight() -> CssNonNegativeInteger` and
`symbol() -> &CssCounterSymbol`, and the enclosing type guarantees strictly
descending weights. `CssCounterStyleName` exposes
`as_identifier() -> &CssIdentifier`. No payload has a raw-token fallback or a
public unchecked constructor.

`CssCounterStyleDescriptors` shall preserve every individually valid descriptor
in source order, including duplicates, as an ordered private-field collection of
typed `CssCounterStyleDescriptor` values. Its
`as_slice() -> &[CssCounterStyleDescriptor]` accessor exposes that order. Its
`effective(CssCounterStyleDescriptorKind) -> Option<&CssCounterStyleDescriptor>`
accessor scans from the end and
returns the last value of that kind, implementing Counter Styles 3's last-valid-
descriptor rule without deleting authored entries. An omitted `system`
descriptor has the intrinsic initial value `symbolic` for the post-block checks.

Syntax validity and whether a rule defines a counter style are distinct authored
facts. `CssCounterStyleRule` shall store this public read-only status:

```rust
#[non_exhaustive]
pub enum CssCounterStyleDefinitionStatus {
    DefinesStyle,
    Ineffective(CssCounterStyleIneffective),
}
```

`CssCounterStyleRule` exposes `name() -> &CssCounterStyleName`,
`descriptors() -> &CssCounterStyleDescriptors`,
`definition_status() -> &CssCounterStyleDefinitionStatus`, and
`position() -> CssSourcePosition`.

`CssCounterStyleIneffective` has private fields and exposes
`system() -> &CssCounterSystem`,
`required_descriptor() -> CssCounterStyleRequiredDescriptorKind`,
`minimum_items() -> CssCounterSymbolCount`, and
`actual_items() -> CssCounterSymbolCount`. The required-descriptor kind is a
public `#[non_exhaustive]` enum with `Symbols` and `AdditiveSymbols` variants;
the count is a private-field non-negative integer newtype. The status is
`Ineffective` exactly when the effective system and effective required
descriptor have these counts:

| Effective system | Required effective descriptor | Minimum items |
| --- | --- | --- |
| `cyclic`, `symbolic`, or `fixed` | `symbols` | 1 |
| `numeric` or `alphabetic` | `symbols` | 2 |
| `additive` | `additive-symbols` | 1 |
| `extends` | none | 0; `DefinesStyle` |

Missing and too-short required descriptors therefore preserve a syntactically
valid rule as `Ineffective`; they do not produce a parse error or invent a
counter-resolution result. If the effective system is `extends`, the presence
of any valid `symbols` or `additive-symbols` occurrence anywhere in the authored
descriptor list is instead an invalid at-rule and returns
`InvalidDescriptorCombination` at the effective `system` descriptor position.

Descriptor parsing uses a dedicated private `CounterStyle` context, not the
ordinary or keyframe property parser. Under this crate's strict no-recovery
contract, an unknown descriptor name returns `UnknownDescriptor`, an invalid
value in any occurrence returns `InvalidDescriptorValue`, and any `!important` annotation
returns `InvalidDeclarationAnnotation` at the `!`. None is silently ignored,
even though a browser would recover by discarding that descriptor. An invalid
duplicate rejects the complete sheet rather than revealing an earlier value.
CSS-wide keywords are accepted only where the descriptor's own grammar names
them; they are not applied through the ordinary declaration global-keyword path.

Vectors shall prove source-order preservation, duplicate same-kind values, a
later value changing the effective result, the default symbolic system, every
row of the effectiveness table at below/equal/above bounds, `extends` both with
and without each prohibited descriptor, unknown names, invalid values,
annotation rejection, and an invalid duplicate rejecting
the sheet. External tests shall inspect both status branches and all ineffective
payload accessors. Symbol lists, additive weights, ranges, pad width, fallback
names, and fixed-system integers shall use finite/bounded semantic types. No
counter rendering, fallback resolution, or name-cycle resolution occurs.

### 9.2 Page Rules

`CssPageRule` shall model the CSS2 page-rule selector grammar, page pseudo-class,
ordered declaration list, and source position. The selector model shall include
the named or pseudo-page forms required by the exact Snapshot-linked CSS2
production and reject unsupported later margin-box syntax unless the conformance
catalog explicitly includes it. Empty declaration lists are valid. Page layout
and pagination remain downstream concerns.

`CssPageSelector` has private optional name and pseudo-class fields and exposes
`name() -> Option<&CssIdentifier>` and
`pseudo_class() -> Option<CssPagePseudoClass>`; both absent represents the
grammar's bare `@page` selector. `CssPagePseudoClass` is a public
`#[non_exhaustive]` enum with `Left`, `Right`, and `First`. `CssPageRule` exposes
`selector() -> &CssPageSelector`,
`declarations() -> &CssDeclarationList`, and
`position() -> CssSourcePosition`. No public constructor can create duplicate
pseudo-classes, multiple page names, or a margin-box child rule.

### 9.3 Existing At-Rules

Existing `@font-face`, `@keyframes`, `@media`, `@container`, `@layer`, `@scope`,
`@import`, and nesting behavior shall remain strict. Their placement and nested
rule permissions shall use the 6.3 parser context table rather than
scattered fallback branches.

`@font-face` shall use the same ordered occurrence discipline as counter styles,
but its own descriptor-specific types:

```rust
pub struct CssFontFaceDescriptor { /* typed value plus position */ }

#[non_exhaustive]
pub enum CssFontFaceDescriptorValue {
    FontFamily(CssFontFaceFamily),
    Src(CssFontFaceSourceList),
    FontStyle(CssFontFaceStyle),
    FontWeight(CssFontFaceWeight),
    FontStretch(CssFontFaceStretch),
    UnicodeRange(CssUnicodeRangeList),
    FontFeatureSettings(CssFontFaceFeatureSettings),
    FontDisplay(CssFontDisplay),
}
```

`CssFontFaceDescriptorKind` is a public `#[non_exhaustive]` enum with the same
eight variant names. An occurrence exposes
`kind() -> CssFontFaceDescriptorKind`,
`value() -> &CssFontFaceDescriptorValue`, and
`position() -> CssSourcePosition`, deriving the kind from the value.
`CssFontFaceDescriptors` preserves every valid occurrence and duplicate in source
order and exposes `as_slice() -> &[CssFontFaceDescriptor]` plus
`effective(CssFontFaceDescriptorKind) -> Option<&CssFontFaceDescriptor>`, which
returns the last occurrence. The descriptor-specific `font-feature-settings`
type excludes ordinary-property-only global values.

Every font-face descriptor payload has this exact public read-only shape:

```rust
#[non_exhaustive]
pub enum CssFontFaceSource {
    Url(CssFontFaceUrlSource),
    Local(CssFontLocalName),
}

#[non_exhaustive]
pub enum CssFontFaceStyle {
    Normal,
    Italic,
    Oblique(Option<CssFontFaceObliqueRange>),
}

#[non_exhaustive]
pub enum CssFontFaceFeatureSettings {
    Normal,
    Features(CssFontFeatureList),
}

#[non_exhaustive]
pub enum CssFontFeatureValue {
    On,
    Off,
    Index(CssNonNegativeInteger),
}

#[non_exhaustive]
pub enum CssFontDisplay {
    Auto,
    Block,
    Swap,
    Fallback,
    Optional,
}

#[non_exhaustive]
pub enum CssFontFormatHintValue {
    Identifier(CssIdentifier),
    String(CssString),
}

pub struct CssFontFormatHint { /* authored value plus optional known format */ }

#[non_exhaustive]
pub enum CssKnownFontFormat {
    Woff,
    Woff2,
    TrueType,
    OpenType,
    Collection,
    EmbeddedOpenType,
    Svg,
}

#[non_exhaustive]
pub enum CssFontTechHint {
    Variations,
    ColorCOLRv0,
    ColorCOLRv1,
    ColorSVG,
    ColorSbix,
    ColorCBDT,
    FeaturesOpenType,
    FeaturesAAT,
    FeaturesGraphite,
    Incremental,
}
```

`CssFontFaceFamily::as_str() -> &str` and
`CssFontLocalName::as_str() -> &str` expose decoded names.
`CssFontFaceSourceList::as_slice() -> &[CssFontFaceSource]` is non-empty.
`CssFontFaceUrlSource` exposes `url() -> &CssUrl`,
`format_hints() -> &[CssFontFormatHint]`, and
`tech_hints() -> &[CssFontTechHint]`; `CssUrl::as_str() -> &str` preserves the
unresolved authored URL. `CssFontFormatHint` exposes
`authored() -> &CssFontFormatHintValue` and
`known() -> Option<CssKnownFontFormat>` so grammar-valid future format strings
remain authored without pretending a capability result.

`CssFontFaceWeight` exposes `start() -> CssFontFaceWeightValue` and
`end() -> Option<CssFontFaceWeightValue>`; `CssFontFaceStretch` has the same
shape with `CssFontFaceStretchValue`. `CssFontFaceObliqueRange` exposes
`start() -> CssAngle` and `end() -> Option<CssAngle>`. Each bounded scalar has
the 11.1 exact numeric accessor and construction invariant.
`CssUnicodeRangeList::as_slice() -> &[CssUnicodeRange]`; each range exposes
`start() -> CssUnicodeCodePoint` and `end() -> CssUnicodeCodePoint` and enforces
ordered Unicode scalar bounds. `CssFontFeatureList::as_slice() -> &[CssFontFeature]`;
each feature exposes `tag() -> CssOpenTypeTag` and
`value() -> Option<CssFontFeatureValue>`, where absence preserves an omitted
value rather than rewriting it to `On`. All list order and duplicates permitted by the
descriptor grammar are retained, and no payload exposes unparsed descriptor
tokens.

The effective `font-family` and `src` descriptors are both required. If either
is absent, the complete at-rule returns `InvalidAtRuleBody` at the closing brace
with an expectation identifying the missing descriptor or descriptors; no
partially usable font-face node is returned. Unknown, invalid-value, and
annotated descriptor occurrences follow the exact 9.1 strict
descriptor policy and 5.2 categories, including rejection of an invalid later
duplicate rather than fallback to an earlier occurrence. `CssFontFaceRule`
exposes `descriptors() -> &CssFontFaceDescriptors` and
`position() -> CssSourcePosition`. Loading, source selection, font matching, and
resource capability checks remain downstream.

Keyframes specifically shall preserve duplicate offset blocks in source order,
allow empty rule bodies, allow empty declaration blocks, and allow repeated
selectors where the grammar permits. Each block shall use the 4.4 keyframe
declaration context and `CssKeyframeDeclarationList`; any `!important`
annotation is a strict whole-sheet `InvalidDeclarationAnnotation` failure.
Vectors shall cover ordinary keyframe declarations, empty blocks, and the exact
annotation position. The parser shall not combine declarations or apply keyframe
cascade semantics.

`CssKeyframesRule` exposes `name() -> &CssKeyframesName`,
`blocks() -> &[CssKeyframeBlock]`, and `position() -> CssSourcePosition`.
Each block exposes `selectors() -> &CssKeyframeSelectorList`,
`declarations() -> &CssKeyframeDeclarationList`, and
`position() -> CssSourcePosition`. Both list wrappers have the standard ordered-
collection accessors. `CssKeyframeDeclaration` exposes
`body() -> &CssDeclarationBody`,
`known() -> Option<&CssKnownDeclaration>`,
`custom() -> Option<&CssCustomDeclaration>`,
`property_name() -> CssPropertyNameRef<'_>`, and
`position() -> CssSourcePosition`; it has no importance accessor or importance
field.

## 10 Official Property Grammar Closure

### 10.1 Inventory Rule

Every normative property or descriptor reachable from a section 2.1 module and
owned by authored syntax shall appear in `PARSER_CONFORMANCE`.
This includes CSS2 properties not superseded out of the profile and all properties
introduced by the Snapshot-listed Box, Color, Backgrounds and Borders, Images,
Fonts, Writing Modes, Multi-column, Flexbox, Basic UI, Containment, Transforms,
Compositing, and Counter Styles modules.

The implementation schema shall contain every final `Complete` property. The
fixed official source-item inventory shall put every non-parser item into
`CONFORMANCE_EXCLUSIONS` with one 3.2 reason, source and owner. A current
author-facing spelling may never use an exclusion reason: it is either complete
or explicitly `RecognizedUnsupported` and receives the corresponding parser
diagnostic. A superseded or removed spelling with no current production is an
exclusion rather than a recognized current property, so input using it remains
unknown. No official in-boundary property may remain partial or unsupported at
initiative completion.

### 10.2 Grammar Requirements

Each complete property parser shall implement its exact property value grammar,
including:

- every keyword and CSS-wide keyword allowed at the whole-property boundary;
- exact value kinds, units, ranges, and finite-number policy;
- one-to-four edge expansion forms where defined;
- comma, slash, and whitespace separators as defined, without interchange;
- layered lists and their final-layer-only components;
- shorthand slot order, optionality, ambiguity resolution, and duplicate
  rejection;
- global keywords only as the complete value, never as list members;
- symbolic `var()` preservation without pretending post-substitution validity is
  known;
- strict complete-input exhaustion.

Representative known defects are mandatory closure cases: `display: inline`,
`overflow: auto`, full `background`, one-to-four `border-color`, list-valued
background boxes, two-value gaps, and every catalog property absent at the
initiative base.

### 10.3 Authored Shorthands

Shorthands shall remain authored shorthand syntax rather than being expanded into
longhands in this crate. Each shorthand receives a dedicated typed value whose
private constructor enforces slot uniqueness and grammar. Expansion, cascade,
initial values, and logical-to-physical mapping require downstream context and
remain out of scope.

## 11 Shared Value Modeling

### 11.1 Validated Scalars And Identifiers

All public floating-point values shall use `CssFiniteNumber` or a stricter wrapper.
Non-negative, positive, unit-interval, percentage, opacity, ratio, integer, and
count domains shall use distinct private-field types. No public or crate-private
unchecked constructor may accept an unverified external float. Parser token
conversion shall reject infinities produced by exponent overflow.

`CssFiniteNumber::try_new(f32)` is the single float validation boundary. It
rejects NaN and both infinities and canonicalizes every positive or negative zero
to positive `0.0` before storage. Its value accessor therefore never returns
negative zero. It implements `PartialEq`/`Eq` by canonical stored value,
`PartialOrd`/`Ord` with `f32::total_cmp`, and `Hash` from the canonical
`to_bits()` value; these contracts agree because NaN, infinities, and negative
zero are absent. `CssFiniteNumber::value() -> f32` is its exact scalar accessor;
it exposes no arithmetic operators.

Every stricter float-domain wrapper stores a `CssFiniteNumber`, validates only
its additional domain at construction, and delegates equality, ordering, and
hashing to that inner value. Unit-bearing authored structs include the unit in
those operations, so semantically convertible spellings such as `1s` and
`1000ms` remain distinct authored values. Integer and count wrappers use their
ordinary mathematical equality/order/hash after range validation. Named vectors
shall exercise equality, `HashSet`, `BTreeSet`, both zero spellings, range edges,
and rejected non-finite inputs for every wrapper family.

This crate exposes no Serde implementation, stable binary/wire format, or
numeric `Display` contract, and this initiative shall not add one. `Debug` is
diagnostic only. A future CSS serializer must be a separate authored-syntax
contract and may not infer interchange stability from these wrappers.

Identifiers shall use `CssIdentifier` or a stricter newtype. OpenType tags shall
use `CssOpenTypeTag`, which contains exactly four ASCII bytes in the Fonts 3
allowed range. Feature indices shall use a non-negative integer wrapper.

`CssUrl` shall be a public private-field decoded, unresolved URL value exposing
`as_str() -> &str` and `is_empty() -> bool`. Empty is a valid value: `url()`,
`url( )`, `url("")`, and `url('')` all construct a `CssUrl` whose `is_empty()`
is true. Syntax whitespace around an unquoted URL value is not part of the
decoded string, while whitespace inside a quoted string is preserved, so
`url(" ")` is non-empty. An omitted URL token/function, unterminated quoted
form, bad-url token, forbidden unquoted character, or trailing component is a
syntax error and is never represented as an empty value. `CssUrl` performs no
base-URL resolution, loading, media-type inference, or capability check.

Every URL-bearing grammar, including Images 3 `<image>`, font-face `src`,
cursor/list/background/mask values, and URL forms of `@import`, shall reuse this
one boundary. The string form of `@import` uses its own
`CssImportString` token-form payload but likewise permits and preserves an empty
decoded string. Tests shall cover all four empty URL spellings, quoted
whitespace, an empty import string, an empty import URL, an empty font source,
an empty image URL, and each distinct missing/malformed case.

The shared scalar accessors used by the public descriptor and calculation views
are exact: `CssInteger::value() -> i32`,
`CssNonNegativeInteger::value() -> u32`,
`CssPercentage::value() -> CssFiniteNumber`,
`CssUnicodeCodePoint::value() -> u32`,
`CssCounterSymbolCount::value() -> usize`,
`CssFontFaceWeightValue::value() -> CssFiniteNumber`, and
`CssFontFaceStretchValue::value() -> CssFiniteNumber`.
`CssLengthDimension`, `CssAngle`, `CssTime`, and `CssFrequency` each expose
`number() -> CssFiniteNumber` and a domain-specific `unit()` accessor returning
`CssLengthUnit`, `CssAngleUnit`, `CssTimeUnit`, or `CssFrequencyUnit`.
`CssOpenTypeTag::bytes() -> [u8; 4]` exposes the validated tag without a string
round-trip. Stricter wrappers may return one of these semantic scalars rather
than a primitive, but shall name that exact accessor in their owning model.

### 11.2 Math Expressions

Values and Units 3 `calc()` coverage shall be complete for every numeric domain
it names. Every grammar slot that accepts `<length>`, `<frequency>`, `<angle>`,
`<time>`, `<percentage>`, `<number>`, or `<integer>` shall accept either the
slot's literal type or a calculation whose resolved type is valid for that slot.
Where the owning grammar explicitly accepts a compatible type-percentage form,
the calculation may combine percentages with that dimension. Values 3 does not
permit `calc()` as `<resolution>` and does not define `fr` as a calculation
dimension; both remain literal-only and are rejected as calculation leaves.

The public authored calculation surface shall contain these private-field domain
wrappers:

- `CssNumberCalculation` and `CssIntegerCalculation`;
- `CssPercentageCalculation`;
- `CssLengthCalculation` and `CssLengthPercentageCalculation`;
- `CssAngleCalculation` and `CssAnglePercentageCalculation`;
- `CssTimeCalculation` and `CssTimePercentageCalculation`;
- `CssFrequencyCalculation` and `CssFrequencyPercentageCalculation`.

There is deliberately no number-percentage wrapper. A percentage is compatible
with a dimension only when the property grammar defines that contextual
relationship. A pure dimension wrapper rejects percentage leaves, and a pure
percentage wrapper rejects dimension leaves.

The wrappers may share one structural implementation parameterized by a
crate-sealed math-domain marker. External code cannot implement a new domain or
construct nodes. The exact public leaf assignment is:

| Calculation wrapper | Public leaf type and variants |
| --- | --- |
| `CssNumberCalculation` | `CssNumberCalculationLeaf::{Number(CssFiniteNumber), Integer(CssInteger)}` |
| `CssIntegerCalculation` | `CssInteger` |
| `CssPercentageCalculation` | `CssPercentage` |
| `CssLengthCalculation` | `CssLengthDimension` |
| `CssLengthPercentageCalculation` | `CssLengthPercentageCalculationLeaf::{Length(CssLengthDimension), Percentage(CssPercentage)}` |
| `CssAngleCalculation` | `CssAngle` |
| `CssAnglePercentageCalculation` | `CssAnglePercentageCalculationLeaf::{Angle(CssAngle), Percentage(CssPercentage)}` |
| `CssTimeCalculation` | `CssTime` |
| `CssTimePercentageCalculation` | `CssTimePercentageCalculationLeaf::{Time(CssTime), Percentage(CssPercentage)}` |
| `CssFrequencyCalculation` | `CssFrequency` |
| `CssFrequencyPercentageCalculation` | `CssFrequencyPercentageCalculationLeaf::{Frequency(CssFrequency), Percentage(CssPercentage)}` |

Every named leaf enum is public and `#[non_exhaustive]`. Relative-color channel
calculations defined in 11.6 use separate slot-specific leaves. No calculation
tree shares a broad cross-domain leaf enum.

The read-only tree view exposes exact closed punctuation enums
`CssCalculationSumOperator::{Add, Subtract}` and
`CssCalculationProductOperator::{Multiply, Divide}` and this structure:

```rust
pub struct CssCalculationSumRef<'a, L> { /* borrowed private sum */ }
pub struct CssCalculationProductRef<'a, L> { /* borrowed private product */ }
pub struct CssCalculationSumTerm<L> { /* private operator and product */ }
pub struct CssCalculationProductOperation<L> { /* private operator and value */ }

#[non_exhaustive]
pub enum CssCalculationValueRef<'a, L> {
    Leaf(&'a L),
    Parenthesized(CssCalculationSumRef<'a, L>),
    NestedCalculation(CssCalculationSumRef<'a, L>),
}
```

Every domain wrapper's `root()` returns
`CssCalculationSumRef<'_, ItsLeaf>`. `CssCalculationSumRef` exposes
`first() -> CssCalculationProductRef<'_, L>` and
`terms() -> &[CssCalculationSumTerm<L>]`. Each sum term exposes
`operator() -> CssCalculationSumOperator` and
`product() -> CssCalculationProductRef<'_, L>`.
`CssCalculationProductRef` exposes
`first() -> CssCalculationValueRef<'_, L>` and
`operations() -> &[CssCalculationProductOperation<L>]`. Each product operation
exposes `operator() -> CssCalculationProductOperator` and
`value() -> CssCalculationValueRef<'_, L>`.

`Parenthesized` represents authored parentheses and `NestedCalculation`
represents an authored nested `calc()` function; neither is normalized into the
other. The borrowed views, term/operation wrappers, and their fields have no
public constructors or mutation APIs and cannot be converted back into a domain
wrapper. This is the only public structural inspection path; adapters shall not
parse `Debug` or authored text.

Every domain tree follows the exact Values 3 structure and type algorithm:

- a sum has a first product and ordered add/subtract terms;
- a product has a first value and ordered multiply/divide operations;
- nested `calc()` and parenthesized grouping remain explicit authored nodes;
- `+` and `-` require equal resolved types, except number plus integer promotes
  the result to number;
- `*` requires at least one number-or-integer scalar; integer times integer
  resolves to integer, a scalar-times-scalar product with at least one number
  resolves to number, and a valid scalar-times-dimension or
  scalar-times-percentage product resolves to the non-scalar operand's type;
- `/` requires a number-or-integer right operand; an integer left operand resolves
  to number and every other valid division preserves the left type;
- `CssIntegerCalculation` accepts only an integer root, while
  `CssNumberCalculation` accepts a number or integer root because an integer is a
  valid `<number>`; every other wrapper requires its named root type after any
  contextual percentage typing, so a number-producing division cannot inhabit
  `CssIntegerCalculation`;
- a percentage leaf is assigned its contextually compatible dimension type only
  in the corresponding type-percentage wrapper;
- unitless zero remains number or integer inside `calc()` and never becomes a
  dimension leaf;
- a purely numeric divisor is evaluated in source order with Values 3 precedence
  by a private checked `f64` evaluator over the already validated finite `f32`
  leaves; an exact positive or negative zero divisor and any non-finite
  intermediate are invalid at the responsible operator, while this evaluator
  does not normalize the authored tree or evaluate any property range;
- whitespace is required around sum `+` and `-`; product whitespace follows the
  Values 3 grammar; separators and complete-input exhaustion remain strict.

Property-specific numeric value types shall be authored unions whose variants
are `Literal` and `Calculation`, using only the calculation wrapper valid for
that property slot. A generic union may be used internally only with a sealed
domain/range marker; public property types shall not expose combinations that
their grammar cannot accept. If `var()` makes the numeric expression dependent
on substitution, the enclosing declaration uses
`CssDeclaredValue::VariableDependent` rather than placing an untyped escape leaf
in a calculation tree.

Literal non-math values are checked against the property's range during parsing.
Calculation leaves must be finite and the expression must type-check, but the
calculation is not rejected merely because a term or statically apparent result
is outside the target range. Values 3 range checking and clamping occur at the
computed/used-value phase, which this crate does not own. Vectors shall cover
every wrapper, every operator/type promotion, contextual percentages, nested
groups, unitless zero, division by expression-derived zero, cross-domain
rejection, and literal-versus-calculation range behavior. Lexical, operator,
resolved-type, non-finite-intermediate, and divide-by-zero failures return the
owning property's `InvalidPropertyValue` at the first responsible token.

### 11.3 Positions

The generic component-count-only `CssPosition` model shall be replaced by grammar
types that make these domains distinct:

- a validated standard `<position>` with legal keyword/offset pairings;
- background-position layers;
- mask-position layers;
- transform-origin with its two-dimensional origin and optional z offset.

Each model shall encode valid arity and keyword axes. Three naked lengths,
duplicate center components, orphaned offsets, contradictory horizontal or
vertical edges, and four-component transform origins are invalid. Shared parsing
helpers may tokenize components but shall return only property-specific validated
types.

### 11.4 Timing

`CssDuration` shall contain a finite non-negative time. `CssDelay` shall contain a
finite signed time. These scalar wrappers represent literal time tokens only;
their authored property unions are exact:

```rust
pub enum CssDurationValue {
    Literal(CssDuration),
    Calculation(CssTimeCalculation),
}

pub enum CssDelayValue {
    Literal(CssDelay),
    Calculation(CssTimeCalculation),
}

pub enum CssAnimationIterationValue {
    Infinite,
    Literal(CssAnimationIterationNumber),
    Calculation(CssNumberCalculation),
}
```

These public enums are `#[non_exhaustive]` and have no unchecked constructors.
Duration, delay, and iteration list/shorthand types remain distinct and contain
their corresponding authored union, not a common time/number bag. Literal
transition and animation durations reject negatives; literal delays accept
negatives; literal iteration numbers reject negatives; and the exact ASCII-case-
insensitive keyword `infinite` constructs the `Infinite` branch only for
animation iteration count and its shorthand slot. It is not a number and never
enters calculation arithmetic. Negative zero is accepted at the token boundary
and canonicalized to positive zero by `CssFiniteNumber`, so it compares, orders,
and hashes identically to positive zero.

Well-typed `calc(-1s)`, `calc(1s - 2s)`, and `calc(-1)` are preserved in the
appropriate calculation variants even for non-negative target grammars; their
range handling is deferred under 11.2. No literal or calculation leaf may
contain infinity or NaN, and invalid dimensional mixing or division by zero is a
parse error.

### 11.5 Function-Specific Models

Generic argument-count wrappers shall not certify transform, easing, shape, or
filter functions. Every accepted function shall have a dedicated variant or
private-field struct enforcing exact separators, arity, keyword domain, and
numeric bounds.

The shared Images 3 `<image>` payload used by properties and
`CssCounterSymbol::Image` shall have this exact public top-level shape:

```rust
#[non_exhaustive]
pub enum CssImage {
    Url(CssUrl),
    LinearGradient(CssLinearGradient),
    RepeatingLinearGradient(CssLinearGradient),
    RadialGradient(CssRadialGradient),
    RepeatingRadialGradient(CssRadialGradient),
}

#[non_exhaustive]
pub enum CssLinearGradientDirection {
    Angle(CssGradientAngle),
    To(CssSideOrCorner),
}

#[non_exhaustive]
pub enum CssGradientAngle {
    Zero,
    Dimension(CssAngle),
    Calculation(CssAngleCalculation),
}

#[non_exhaustive]
pub enum CssSideOrCorner {
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[non_exhaustive]
pub enum CssRadialGradientGeometry {
    Omitted,
    Position(CssPosition),
    Circle(CssCircleGradientGeometry),
    Ellipse(CssEllipseGradientGeometry),
    Extent(CssRadialExtentGeometry),
}

#[non_exhaustive]
pub enum CssRadialShape {
    Circle,
    Ellipse,
}

#[non_exhaustive]
pub enum CssRadialExtent {
    ClosestCorner,
    ClosestSide,
    FarthestCorner,
    FarthestSide,
}

#[non_exhaustive]
pub enum CssGradientStopItem {
    ColorStop(CssGradientColorStop),
    Hint(CssLengthPercentageValue),
}

#[non_exhaustive]
pub enum CssLengthPercentageValue {
    Length(CssLengthDimension),
    Percentage(CssPercentage),
    Calculation(CssLengthPercentageCalculation),
}

#[non_exhaustive]
pub enum CssNonNegativeLengthValue {
    Literal(CssNonNegativeLengthDimension),
    Calculation(CssLengthCalculation),
}

#[non_exhaustive]
pub enum CssNonNegativeLengthPercentageValue {
    Length(CssNonNegativeLengthDimension),
    Percentage(CssNonNegativePercentage),
    Calculation(CssLengthPercentageCalculation),
}
```

`CssLinearGradient` exposes
`direction() -> Option<&CssLinearGradientDirection>` and
`stops() -> &CssGradientStopList`; `CssRadialGradient` exposes
`geometry() -> &CssRadialGradientGeometry` and the same stop-list accessor.
`CssGradientAngle` preserves the authored unitless-zero form rather than
rewriting it as a dimension.

`CssCircleGradientGeometry` exposes
`radius() -> Option<&CssNonNegativeLengthValue>`,
`position() -> Option<&CssPosition>`, and
`shape_was_authored() -> bool`; `CssEllipseGradientGeometry` exposes
`radii() -> Option<&CssNonNegativeLengthPercentagePair>`,
`position() -> Option<&CssPosition>`, and the same authored-shape flag.
`CssRadialExtentGeometry` exposes
`shape() -> Option<CssRadialShape>`,
`extent() -> CssRadialExtent`, and
`position() -> Option<&CssPosition>`. The shape enum has `Circle` and
`Ellipse`; the extent enum has `ClosestCorner`, `ClosestSide`,
`FarthestCorner`, and `FarthestSide`. These variants make a circle with two
radii, an ellipse with one radius, or any other invalid shape/size pairing
unrepresentable.

The non-negative radius wrappers use the same literal-versus-calculation rule
as 11.2: literal length and percentage branches are range-checked, while a
well-typed calculation is preserved for later computed-value range handling.
`CssNonNegativeLengthDimension::value() -> CssLengthDimension` and
`CssNonNegativePercentage::value() -> CssPercentage` expose the validated
literals. `CssNonNegativeLengthPercentagePair` exposes
`horizontal() -> &CssNonNegativeLengthPercentageValue` and
`vertical() -> &CssNonNegativeLengthPercentageValue`.

`CssGradientStopList::as_slice() -> &[CssGradientStopItem]` preserves authored
order and guarantees at least two color stops, a color stop at each end, no
adjacent hints, and at most one hint between adjacent stops.
`CssGradientColorStop` exposes `color() -> &CssColor` and
`position() -> Option<&CssLengthPercentageValue>`. All image, gradient,
geometry, stop, and range-bearing fields and constructors are private. No image
payload contains raw component values, and URL resolution, image loading,
gradient fixup, interpolation, and rendering remain downstream.

Mandatory cases include comma-separated matrix arguments, unit-interval
cubic-bezier x coordinates, positive step counts and the stricter `jump-none`
domain, shape-specific radius keywords, and a drop-shadow grammar that rejects
`inset` and unsupported spread syntax.

### 11.6 Relative Colors

Relative color functions shall be parameterized by their color space and channel
slot. Each slot accepts only its allowed channel references, number/percentage or
angle domains, `none` where specified, and a math expression typed for that
channel environment. Alpha has its own channel environment. Arbitrary identifiers
and unrelated dimensions are invalid. The model remains symbolic; it does not
evaluate channels or convert colors.

### 11.7 Grid Repetition

Grid repetition shall use structurally distinct fixed-repeat and auto-repeat
models. Repeat content types shall exclude repeat components, making nested
`repeat()` unconstructable. Auto-repeat shall accept only fixed-size track forms;
flexible `fr` tracks are invalid there. General track lists may contain validated
repeat nodes but cannot construct recursive repeat content.

### 11.8 Typography

`font-feature-settings` shall require valid four-ASCII-character OpenType tags and
non-negative feature values. `font-family` shall parse a CSS-wide keyword only at
the whole-property boundary; a family list cannot contain an unquoted global
keyword member. Quoted family names remain valid even when their text matches a
global keyword. Escapes and supplementary Unicode characters shall follow the
identifier/string grammar without being mistaken for four-byte OpenType tags.

## 12 Public API And Documentation

### 12.1 Front Door

`src/lib.rs` shall retain an intentional front door and reexport only authored
syntax, metadata, and diagnostic types needed by consumers. Parser implementation
modules, schema macros, conformance fixtures, and test helpers remain private.
The crate root shall contain `#![forbid(unsafe_code)]`.

The supported public parsing functions shall be:

- `parse_sheet(&str) -> Result<CssSheet>`;
- `parse_style_attribute(&str) -> Result<CssDeclarationList>`.

`CssSheet` exposes `encoding() -> Option<&CssEncodingDeclaration>` and
`rules() -> &[CssRule]`; `CssEncodingDeclaration` exposes `label() -> &str` and
`position() -> CssSourcePosition`. The rule enum is exactly this public
non-exhaustive authored choice:

```rust
#[non_exhaustive]
pub enum CssRule {
    Import(CssImportRule),
    Namespace(CssNamespaceRule),
    LayerStatement(CssLayerStatementRule),
    LayerBlock(CssLayerBlockRule),
    FontFace(CssFontFaceRule),
    Keyframes(CssKeyframesRule),
    Style(CssStyleRule),
    Media(CssMediaRule),
    Container(CssContainerRule),
    Scope(CssScopeRule),
    Supports(CssSupportsRule),
    CounterStyle(CssCounterStyleRule),
    Page(CssPageRule),
}
```

`CssRule::position() -> CssSourcePosition` delegates to the active typed payload.
Each payload exposes this exact minimum contract:

| Public rule payload | Required read-only accessors |
| --- | --- |
| `CssImportRule` | `target() -> &CssImportTarget`, `layer() -> Option<&CssImportLayer>`, `supports() -> Option<&CssImportSupportsClause>`, `media() -> Option<&CssMediaQueryList>`, `position()` |
| `CssNamespaceRule` | `prefix() -> Option<&CssNamespacePrefix>`, `target() -> &CssNamespaceTarget`, `position()` |
| `CssLayerStatementRule` | `names() -> &CssLayerNameList`, `position()` |
| `CssLayerBlockRule` | `name() -> Option<&CssLayerName>`, `rules() -> &[CssRule]`, `position()` |
| `CssFontFaceRule` | `descriptors() -> &CssFontFaceDescriptors`, `position()` |
| `CssKeyframesRule` | `name() -> &CssKeyframesName`, `blocks() -> &[CssKeyframeBlock]`, `position()` |
| `CssStyleRule` | `selectors() -> &CssStyleSelectorList`, `declarations() -> &CssDeclarationList`, `position()` |
| `CssMediaRule` | `queries() -> &CssMediaQueryList`, `rules() -> &[CssRule]`, `position()` |
| `CssContainerRule` | `name() -> Option<&CssContainerName>`, `condition() -> &CssContainerCondition`, `rules() -> &[CssRule]`, `position()` |
| `CssScopeRule` | `root() -> Option<&CssSelectorList>`, `limit() -> Option<&CssSelectorList>`, `rules() -> &[CssRule]`, `position()` |
| `CssSupportsRule` | `condition() -> &CssSupportsCondition`, `rules() -> &[CssRule]`, `position()` |
| `CssCounterStyleRule` | `name() -> &CssCounterStyleName`, `descriptors() -> &CssCounterStyleDescriptors`, `definition_status() -> &CssCounterStyleDefinitionStatus`, `position()` |
| `CssPageRule` | `selector() -> &CssPageSelector`, `declarations() -> &CssDeclarationList`, `position()` |

Every abbreviated `position()` in the matrix has the exact return type
`CssSourcePosition`. Group rules reuse `CssRule` for their validated child
payloads; the parser context makes forbidden variants unconstructable in each
group. The public `CssScopedRule`, `CssScopedStyleRule`,
`CssScopedMediaRule`, `CssScopedContainerRule`, and scoped layer duplicate
families shall be removed rather than maintained beside the canonical tree.

`CssImportTarget` remains a public non-exhaustive `Url(CssImportUrl)` or
`String(CssImportString)` choice. `CssImportUrl` wraps the shared `CssUrl` and
exposes `url() -> &CssUrl`; `CssImportString` exposes `as_str() -> &str` and
`is_empty() -> bool`. Both branches permit an empty decoded target while a
missing or malformed target remains an `InvalidAtRulePrelude` error.
`CssImportLayer` has `Anonymous` and `Named(CssLayerName)` branches.
`CssNamespaceTarget` has `String(CssNamespaceValue)` and
`Url(CssNamespaceValue)` branches. `CssNamespacePrefix` is a private-field
semantic newtype exposing `as_identifier() -> &CssIdentifier`; namespace
constraints and namespace diagnostics reuse it instead of an untyped string.

`CssDeclaration` exposes
`body() -> &CssDeclarationBody`,
`known() -> Option<&CssKnownDeclaration>`,
`custom() -> Option<&CssCustomDeclaration>`,
`property_name() -> CssPropertyNameRef<'_>`,
`importance() -> CssImportance`, and
`position() -> CssSourcePosition`. The declaration and keyframe list contracts
are in 4.4, the selector contracts in 7, condition contracts in 8, descriptor
contracts in 9, property metadata in 3.4, and calculation views in 11.2.
Those sections are part of this exact public inspection surface, not optional
implementation guidance.

Every ordered collection wrapper exposes `as_slice()`, `iter()`, `len()`, and
`is_empty()` over its one semantic element type. It exposes no mutable slice,
unchecked insertion, public field, or placeholder default; a non-empty grammar
uses no public empty constructor. Parser-produced sheets, rules, declaration
aggregates, selector aggregates, conditions, descriptor occurrences/lists, and
diagnostics have private construction. Public fallible constructors are limited
to context-free scalar values whose complete invariant can be checked without a
parser context.

Every successful syntax node that carries a source position shall expose it as
`position() -> CssSourcePosition`; the obsolete `location()` name and
`CssSourceLocation` type shall not remain as compatibility aliases. Read-only
payload accessors return borrowed validated semantic types rather than raw
parser tokens or strings, except where authored text is itself the modeled
contract.

Every public enum in the final front door shall be `#[non_exhaustive]` except
this exact intentionally closed allowlist:

- `CssSupportStatus::{Complete, Partial, RecognizedUnsupported}`;
- `CssImportance::{Normal, Important}`;
- `CssSupportsAnnotation::{Absent, Important}`;
- `CssCalculationSumOperator::{Add, Subtract}`;
- `CssCalculationProductOperator::{Multiply, Divide}`.

The closed enums represent a fixed support state, exact binary annotation, or
grammar punctuation set; adding a variant is intentionally breaking. All other
public enums, including `CssRule`, selector and pseudo-selector enums, media and
supports condition enums, declaration/value enums, diagnostic enums, and
calculation value-view enums, require wildcard-compatible downstream matching.
There is no retained exhaustive legacy enum solely for an external test.

No production-visible API shall exist solely for tests. External tests shall use
only these public functions and public accessors.

### 12.2 Documentation

Every public semantic type and public function introduced or materially changed
by this initiative shall have item-level documentation that states its authored
phase, invariant, and downstream non-responsibilities where ambiguity exists.
The crate-level docs and README shall include one minimal stylesheet example and
one style-attribute example that compile as doctests.

Documentation shall explicitly describe strict whole-input failure, support
metadata, source position coordinates, importance, variables, at-rule authored
contracts, and the absence of loading/evaluation/matching/cascade behavior.

### 12.3 Compatibility

This initiative is intentionally breaking. Removal of `CssValue`, changes to
`CssProperty`, selector models, time/math/position types, source-location
accessors, and new declaration coupling require root adapter migration after the
leaf candidate is published. The leaf shall not retain invalid legacy APIs for
root tests or external callers. Root integration tests may inspect only public
API and must adapt in the root-owned follow-up.

No Cargo dependency, feature, edition, package identity, or leaf-owned generated
artifact changes are expected. The leaf manifest currently declares edition 2024
and no `rust-version`; this initiative shall not invent a leaf MSRV. The root-owned
integration follow-up shall read root's committed MSRV at the selected candidate
revision and verify that the breaking public surface and existing dependencies
compile under it before pointer promotion.

## 13 Test And Conformance Evidence

### 13.1 Independent Property Vectors

For every parser-facing catalog property marked complete, independent table-driven tests shall
provide at least:

- one non-global valid value that exercises the property's own grammar;
- every property-specific keyword family;
- `inherit` and one other CSS-wide keyword as whole values;
- one variable-dependent value preserving authored text and references;
- one invalid token or keyword from an adjacent property grammar;
- boundary/range vectors for every numeric domain;
- separator, arity, and trailing-token negatives for lists or shorthands.

Using only `inherit` as the accepted case does not count as grammar coverage. The
test inventory shall compare its property names to the independent conformance
catalog and implementation metadata in both directions.

### 13.2 Rule And Selector Vectors

Every non-property parser-facing catalog entry shall have an independent
kind-specific test record keyed by its stable catalog identifier. At-rule records
shall include accepted preservation plus negative, empty-body, malformed-prelude,
malformed-body, placement, nesting, and source-order cases as applicable.
Selector records shall include an accepted AST case and every applicable exact
diagnostic mutation for each Selectors 3 category, namespace form, combinator,
pseudo-class, pseudo-element, escape-sensitive form, and repeated ID case.
Functional pseudo-selector records shall prove strict complex/relative list
support without forgiving entries.

Media type and media feature entries shall each have an accepted preservation
case and exact invalid arity, value-domain, prefix, unit, boolean, and separator
cases that apply to that feature. Descriptor entries shall each have an accepted
typed-value case plus authored duplicate/effective-last, placement,
required-companion, and value-domain cases that apply. Extension entries shall
each cite at least one accepted preservation vector and one strict rejection
vector for every bounded grammar dimension named by the inventory row. 3.5
entries participate only in the kind named by their row; Snapshot tier never
creates a second extension-vector inventory. The
`later.font-feature-values` vector instead proves exact
`RecognizedUnsupported` classification and `UnsupportedAtRule` diagnostics.

Counter-style descriptor vectors shall additionally cover every 9.1
definition-status row and the strict `extends` conflicts. Font-face vectors shall
cover both required descriptors singly and together, every optional descriptor,
source-order duplicates, and a missing `font-family`, missing `src`, or both as
`InvalidAtRuleBody`. For both descriptor contexts, unknown, invalid-valued,
annotated, and invalid-later-duplicate cases shall
assert the distinct 5.2 category and exact responsible position.

Coverage tests shall compare parser-facing catalog identifiers to these records in both
directions and shall compare both sets to the corresponding crate-private
kind-specific implementation inventory. No parser-facing entry may be marked
`Complete` without exactly one implementation record and its kind-specific
vector record, and no implementation or vector record may silently name a
feature absent from the catalog. Qualified-rule and shared-value records receive
the same three-way comparison as the other kinds.

### 13.3 Value Grammar Vectors

Position, transform, easing, filter, shape, timing, relative-color, Grid,
typography, background, border, image, font, multicolumn, flex, containment, and
compositing grammars shall each have named valid vectors and one-token mutation
tables. Calculation vectors shall independently cover number, integer,
percentage, length, angle, time, and frequency roots; every type-percentage
wrapper; all operator/type promotions; nested `calc()` and grouped sums;
unitless zero; expression-derived zero divisors; checked-evaluation overflow;
cross-domain rejection; and literal-versus-calculation range handling. Other
numeric tests shall cover zero, negative zero, negative values, range edges,
exponent overflow, and every allowed unit family. Timing vectors shall also
cover `infinite` in the animation iteration longhand and shorthand, reject it in
duration/delay and unrelated numeric slots, and inspect the dedicated
`CssAnimationIterationValue::Infinite` branch.

### 13.4 Exact Diagnostics

Focused negative tests shall assert exact `CssErrorCode`, structured `ErrorKind`
payload, canonical property or feature identifier, and `CssSourcePosition`.
Helpers that accept multiple unrelated syntax categories shall be removed.
Broad failure-only assertions are allowed only in fuzz/property-style tests whose
purpose is panic freedom or total rejection rather than a specific grammar
boundary.

The diagnostic matrix shall include every 5.2 root variant. Tests shall inspect
every accessor on its detail payload, distinguish absent encountered tokens from
real tokens, distinguish an unknown at-keyword from the cataloged unsupported
`@font-feature-values`, and prove the
one-to-one `ErrorKind`/`CssErrorCode` mapping. Descriptor tests shall separately
prove name, value, combination, and annotation positions; selector/media/color
tests shall cover both named-subproduction and enclosing-grammar payloads.

### 13.5 Public Consumer Tests

Tracked integration tests under `tests/` shall use only public API. They shall
cover successful sheet parsing, style-attribute parsing, strict whole-input
failure, declaration importance, custom-property preservation, selector/at-rule
inspection, support metadata, and diagnostic position inspection. Internal unit
tests remain responsible for exhaustive grammar vectors and private invariants.

The same integration suite shall exercise the public enum evolution contract:
consumer matches over `CssRule`, `CssKnownProperty`, `CssKnownDeclaration`,
`CssDeclarationBody`, `CssDeclaredValue<T>`, `CssPropertyNameRef<'_>`,
`CssSupportsPropertyName`, `CssTypeSelector`, `CssSimpleSelector`,
`CssStyleSelector`,
`CssAttributeTest`, `CssAttributeValue`, `CssPseudoClass`, `CssPseudoElement`,
`CssMediaQuery`, `CssMediaCondition`, `CssMediaFeatureExpression`,
`CssContainerCondition`, `CssContainerStyleQuery`, `CssSupportsCondition`,
`CssSupportsGeneralEnclosed`, `CssImportSupportsClause`, both descriptor-value
enums, `CssCounterStyleDefinitionStatus`, `CssImage`,
`CssLinearGradientDirection`, `CssGradientAngle`, `CssSideOrCorner`,
`CssRadialGradientGeometry`, `CssRadialShape`, `CssRadialExtent`,
`CssGradientStopItem`, `CssLengthPercentageValue`,
`CssNonNegativeLengthValue`, `CssNonNegativeLengthPercentageValue`,
`CssCalculationValueRef<'_, _>`, `CssErrorCode`, `ErrorKind`, and every public
diagnostic context enum include a wildcard. The five intentionally closed enums
in 12.1 receive exhaustive two- or three-branch matches instead.

Using only public API, the suite shall inspect:

- a leading encoding declaration and all thirteen `CssRule` payload branches,
  including every accessor in the 12.1 rule matrix and nested group-rule order;
- a complex selector with namespace-qualified type and attribute names,
  repeated interleaved IDs/classes, every attribute-test field, a functional
  complex list, a relative selector with and without an explicit leading
  combinator, an nth-child `of` list, and a pseudo-element sequence;
- each media, container, and supports boolean branch, known and defined-false
  media payloads, all three supports property-name branches, both
  general-enclosed shapes, and both import-supports branches;
- font-face and counter-style source-order descriptor slices, effective-last
  lookup, every descriptor-value variant, every nested symbol/range/source/hint/
  font-setting accessor specified in 9, URL and gradient image branches through
  the 11.5 public view, and both counter definition-status branches with all
  ineffective payload accessors;
- every 5.2 error detail family through `Error::kind()`, `code()`, and
  `position()`, including token summaries, both source-position scalar
  accessors, `@font-feature-values` metadata ID/module/level/tier, and absent
  metadata for an unknown at-keyword;
- every calculation leaf assignment plus a nested subtraction containing both
  `Parenthesized` and `NestedCalculation` through all ordered sum/product/term/
  operation accessors, and
  property metadata through the exact 3.4 lookup.

No external test shall reach through module privacy, name a private module path,
parse `Debug`, depend on implementation-only discriminators, or force retention
of an obsolete public API.

### 13.6 Verification Obligations

The current configured Clippy failure shall be fixed without a lint suppression.
All package tests, doctests, public integration tests, formatting, configured
warnings-denied Clippy checks, and repository-wide no-unsafe evidence shall pass.
Exact execution commands and publication evidence belong to current cycle plans
and the canonical workflow, not this design contract.

## 14 Finding Closure Matrix

| Review finding | Required closure | Primary evidence |
| --- | --- | --- |
| Missing Snapshot at-rules | 3.1, 6, and 9 typed rules | Catalog-indexed rule grammar matrices |
| Incomplete/overstated properties | 3, 4, 10 | Independent catalog and per-property vectors |
| Incomplete Selectors 3 | 7 | Complete selector vectors |
| Incomplete Media Queries 3 | 8.1 | Type/feature/boolean/malformed vectors |
| No style-attribute parser | 4.4 | Public integration tests |
| Missing `!important` | 4.3 | Sheet/style/custom-property vectors |
| Layer/import ordering and import conditions | 6.2 and 8.3 | Ordering matrix |
| Repeated IDs overwritten | 7.3 | Ordered AST assertions |
| Position grammar leakage | 11.3 | Property-specific mutation tables |
| Generic function leakage | 11.5 | Dedicated function vectors |
| Duration/delay and non-finite conflation | 11.4 | Domain and overflow vectors |
| Calc/range errors | 11.2 | Typed math and literal/calc range vectors |
| Untyped relative colors | 11.6 | Per-space channel vectors |
| Invalid Grid repeat | 11.7 | Structural AST and rejection vectors |
| Malformed charset bypass | 6.1 | Leading-input matrix |
| Typography invalid forms | 11.8 | Tag/index/family vectors |
| Keyframe duplicate/empty rejection | 9.3 | Ordered duplicate and empty vectors |
| Circular compatibility oracle | 3.3 and 13.1-13.2 | Independent bidirectional coverage for every feature kind |
| Public invalid states | 7.1 and 11.1 | Constructor/property tests |
| Property/value cross-product | 4 | Compile-time coupled declaration model |
| Missing public guidance/tests | 12 and 13.5 | Doctests and external tests |
| Mixed location convention | 5.1 | Multiline/non-BMP tests |
| Weak negative diagnostics | 5.2 and 13.4 | Exact category/location assertions |
| Red Clippy gate | 13.6 | Configured Clippy command |
| Missing crate-root unsafe prohibition | 12.1 and 13.6 | Crate attribute, Clippy, scan |

## 15 Initiative Acceptance

The initiative is complete only when all of these predicates hold:

1. Every one of the 25 review findings maps to implemented source and focused
   evidence in 14.
2. The independent Snapshot catalog contains every in-boundary official item and
   every 3.5/3.6 production, with no unclassified entry or moving source URL.
3. Every parser-facing catalog ID has exactly one kind, source, tier,
   implementation record, and primary vector record; every exclusion has one
   allowed 3.2 reason and source-audit record but no implementation/vector ID;
   3.5 contains no official or property production, and 3.6 contains exactly
   the 179 unique baseline names.
4. Every official in-boundary Snapshot item is `Complete`; no partial support is
   advertised as complete.
5. Both public parse functions reject their complete input on every malformed or
   unsupported ordinary construct and perform no browser recovery.
6. All public authored models use private fields and checked construction so
   parser-invalid strings, non-finite numbers, literal range violations,
   mismatched declarations, nested repeats, and invalid grammar combinations are
   unconstructable while 11.2 calculation range deferral remains representable.
7. The public API and docs accurately expose importance, source positions,
   support metadata, at-rule/selector/value syntax, and downstream ownership
   boundaries.
8. Root-facing breakage is reported as a candidate handoff; no root or sibling
   file is changed by this initiative.
9. All verification obligations in 13.6 pass with no Surgeist-owned unsafe and no
   weakened diagnostic or conformance oracle.

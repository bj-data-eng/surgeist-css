# P01-I02 CSS Snapshot 2026 Grammar Closure

## 1. Authority, Base, And Outcome

This is the JIT implementation contract for `P01-I02` in `surgeist-css`.
It is subordinate to the reviewed P01 program at semantic SHA-256
`e7219f734010db9e5772f587c6a394f5eef3020c0512240229f5e5f77829d986`
and incorporates its I02-entry and source-contradiction reconciliations. The
initiative base is the published, fetchable I01 candidate
`bc5394ff5855109dd1d224d29278d6ab601cef4f`; the P01 reconciliation commit
`537b97b46e0ab8625413b005f74eb79eeaa0ac64` is planning-only evidence and is
also part of the I02 implementation range.

I02 shall complete every authored-syntax production owned by this crate from
the official CSS Snapshot 2026 profile, preserve and truthfully classify the
finite I01 extension baseline, and close historical findings 2.1-2.4, 2.7-2.14,
2.16, and 2.17. Ordinary parsing remains browser-recovering and panic-free;
`app-strict` remains an additive one-pass rejection view over the same report.
Every retained public node is valid by construction, and every recovery remains
observable through the I01 typed diagnostic model.

The normative profile is the W3C Group Note of 26 March 2026:
<https://www.w3.org/TR/2026/NOTE-css-2026-20260326/>. Its section 2.1 defines
the official target. Moving `/TR/` aliases and editor drafts are discovery aids,
not conformance inputs. Exact dated module revisions are frozen in section 4.2.

The historical review at
`plans/P01-implement-full-css-spec/P01-css-snapshot-2026-review.md`, SHA-256
`5ddd3eebb4fc3664759021605d3884a0c795947e0ef4e427d3dfc5e77469199d`,
is evidence about source commit `318864d1074d8d723a3a925528343c8a3d8c7253`.
The deleted legacy all-in-one remediation document at commit `4b288d6` is
design evidence only. Its strict-whole-sheet front door, old type sketches, and
all-25-findings acceptance are not active requirements.

## 2. Ownership And Non-Goals

The leaf owns authored CSS token/grammar validity, parser recovery boundaries,
typed authored models, source coordinates, recovery diagnostics, support
metadata, focused tests, documentation, and its published candidate.

I02 excludes cascade/inheritance application, shorthand application, custom
property substitution, post-substitution validation, selector matching or
specificity application, query evaluation, namespace/URL/import/font/image
loading, unit resolution, layout, painting, color conversion, animation
interpolation, serialization, a generic fragment parser, and the I03 corpus
harness. Root owns facade/adapters, API artifacts, integration tests, and the
gitlink. `surgeist-generator` owns corpus import and neutral expectation
generation. I02 edits neither owner and invokes no sibling generator.

No dependency, feature, build script, CI rule, external corpus, or external
software is added. `cssparser = 0.37.0`, `cssparser-color = 0.5.0`, edition
2024, `default=[]`, and `app-strict=[]` remain fixed. No leaf MSRV is invented.
All owned Rust remains free of `unsafe`.

## 3. Frozen I01 Semantics And Required Foundation Repair

### 3.1 Semantics That May Not Change

The following I01 contracts are frozen through I02:

- `parse_sheet(&str) -> CssParseReport<CssSheet>` and
  `parse_style_attribute(&str) -> CssParseReport<CssDeclarationList>`;
- feature-gated `validate_sheet` and `validate_style_attribute`, each parsing
  once and rejecting exactly non-clean reports;
- the meanings of `CssParseReport`, `CssRecoveryDiagnostic`, the ten
  `CssRecoveryAction` variants, `ErrorKind`/`CssErrorCode`, first-responsible
  error positions, complete recovery spans, and stable diagnostic ordering;
- zero-based UTF-8 byte offsets, zero-based lines, zero-based UTF-16 columns,
  and exclusive span ends;
- declaration importance, custom-property and substitution-dependent authored
  preservation, property/value coupling, and keyframe declarations without
  importance;
- structural nesting limit 256, progress guarantees, balanced-boundary
  recovery, child-before-unrepresentable-parent diagnostics, and no input-driven
  panic;
- parser-owned aggregate construction, private fields, wildcard-compatible
  evolving enums, and ordinary/`app-strict` parity.

Adding grammar changes which inputs recover and which valid nodes are retained;
it does not redefine an existing action, diagnostic root, position, or report.

### 3.2 Mandatory First Cycle: Evolution Boundary Repair

Before any grammar-closure cycle, I02-C01 shall perform the one breaking repair
authorized by P01.9. It is the only breaking I02 cycle.

The exact enum inventory is defined mechanically and exhaustively: every public
enum in owned Rust at base `bc5394f` shall be `#[non_exhaustive]` except the two
deliberately closed semantic state sets `CssImportance` and
`CssSupportStatus`. A tracked source inventory shall name every enum, its base
path, and whether the exception applies; compile-fail public tests shall prove
the evolving set requires wildcard matches and that the two exceptions remain
exhaustively matchable. Adding the attribute to an existing public enum is part
of the authorized break. No variant is removed merely to satisfy the inventory.

`CssKnownDeclaration` shall cease to expose one publicly matchable variant per
property. It becomes a private-field parser-owned struct whose
`property() -> CssKnownProperty` is derived from one private coupled value
discriminator. Downstream callers cannot pair a value obtained from one
property with another property.

The authoritative `property_schema!` remains the sole name/alias/identity/
parser/coupling declaration. Each of its exact 179 base rows shall additionally
name one public private-field property-specific wrapper with the deterministic
name `Css<SchemaVariant>PropertyValue` and one private representation type. The
wrapper set is therefore exactly the 179 schema IDs at `bc5394f`, not a
prose-selected subset. Each wrapper exposes exact authored
`as_css() -> &str` and
`i01_subset() -> Option<&I01PayloadType>`. In C01 every parsed ordinary value
returns `Some`; a later grammar cycle may add property-specific semantic
accessors and return `None` only for newly supported syntax that the I01 payload
could not represent. The accurately named compatibility accessor never claims
complete grammar coverage. New official properties add new schema rows and
wrappers additively.

The frozen I01 payload types reached through `i01_subset()` are compatibility
projections, not the construction boundary for the current authored model.
Their existing public constructors and enum payloads remain source-compatible,
including raw scalar payloads that predate checked numeric wrappers. A current
property-specific accessor must expose a valid-by-construction model, and the
parser may create an I01 projection only after validating the same invariant.
This distinction is the additive repair path when changing a compatibility
payload would otherwise require a second breaking cycle.

For C03 the affected legacy raw-scalar inventory is exactly
`CssGridFlowTolerance::Percent(f32)`. Its name, variants, payloads, derives,
construction syntax, and `CssGridFlowTolerancePropertyValue::i01_subset() ->
Option<&CssGridFlowTolerance>` signature remain unchanged. Add a distinct
non-exhaustive current `CssGridFlowToleranceValue::{Normal, Infinite,
Length(CssLength), Percent(CssFiniteNumber)}` and
`CssGridFlowTolerancePropertyValue::value() -> &CssGridFlowToleranceValue`.
The property wrapper stores the parser-owned current value and an optional I01
compatibility projection. Every I01 input stores both; the percent projection
is formed only after the post-`unit_value * 100.0` result is checked finite.
New typed-calculation syntax stores only the current value and therefore returns
`None` from `i01_subset()`. Public evidence constructs and pattern-matches the
unchanged legacy `Percent(f32)`, inspects a finite parser-produced current
`Percent(CssFiniteNumber)`, proves its matching finite I01 projection, and
checks exact diagnostic position, span, action, and sibling retention when a
finite token overflows during percentage conversion. No other C03 legacy
payload requires a parallel checked representation: the remaining numeric
public fields are private or already use checked wrappers, and their existing
public signatures remain unchanged.

Public inspection uses two exact non-exhaustive borrowed views generated from
the same schema:

```rust
pub struct CssKnownDeclaration { /* private coupled discriminator */ }

#[non_exhaustive]
pub enum CssKnownPropertyValueRef<'a> {
    // one generated variant per schema row, carrying that row's wrapper
}

#[non_exhaustive]
pub enum CssKnownDeclaredValueRef<'a> {
    Property(CssKnownPropertyValueRef<'a>),
    Global(CssGlobalKeyword),
    SubstitutionDependent(&'a CssSubstitutionDependentValue),
}
```

`CssKnownDeclaration::declared_value() -> CssKnownDeclaredValueRef<'_>` returns
exactly one branch. `Property` contains the schema-matched wrapper;
`Global` and `SubstitutionDependent` never construct a property wrapper.
Convenience accessors `property_value()`, `global()`, and
`substitution_dependent()` return the corresponding optional view and are
mutually exclusive. Existing `CssDeclaredValue<T>` may remain for private
implementation and component APIs, but no public constructor or parallel `V2`
declaration variant may recreate the old cross-product. Public compile-fail
tests prove private construction; consumers cover all three branches and all
179 generated property-view variants with wildcard-compatible matches.

C01 shall retire the test-only broad `CssValue` conversion in
`src/test_support.rs`; focused tests inspect property-specific public or private
validated values instead. It shall produce a superseding, SHA-free I02 migration
record naming every changed public enum/declaration access pattern and the exact
root follow-up.

The finite C01 equivalence oracle is a tracked, hand-authored
`tests/fixtures/i01-c01-observables.tsv` created and independently reviewed from
the published base before representation edits. It is a behavioral corpus, not
a source-test inventory: it contains no test owner/name mapping, test count,
placement assertion, execution/comparison counter, plan state, or coordination
manifest. Each row fixes a stable scenario label, entry point, feature mode,
authored input, clean state, ordered retained rule/property IDs, retained
authored value text and importance where applicable, and every ordered
diagnostic's code/root/stable payload identity/byte-line-column position/span/
action. The fixture reader validates this declared data schema; each applicable
row is applied to the public parser and compared field-for-field. Malformed
fixture-schema and missing-observable checks are permitted, but no test infers
completeness from source/test identity or counts. C01 compares the post-repair
public report to every retained row in both feature modes; the only allowed
difference is the documented compile-time API shape. The complete pre-existing
default and `app-strict` I01 behavioral matrices also remain green. A second
required breaking change stops I02 and returns to P01.

### 3.3 C07 Source-Backed Oracle Correction

P01.10 reconciles the one discovered conflict between the C01 behavioral oracle
and findings 2.14/2.17. C07 shall replace only the expected observables for
these seven stable scenario IDs while preserving their authored inputs:

- Grid: `catalog.property.baseline.property.grid.positive`,
  `focused.property-schema.baseline.property.grid.important`, and
  `focused.property-schema.baseline.property.grid.ordinary` recover from the
  invalid flexible auto-repeat instead of retaining it;
- keyframes: `focused.importance.05`, `focused.importance.06`,
  `focused.nested-structural.keyframes-child-loss`, and
  `focused.structural.misc.03` retain the valid now-empty keyframe block and
  rule after the invalid declaration is dropped, without obsolete
  `DropKeyframeBlock` or `DropAtRule` diagnostics.

The existing fixture SHA-256 before C07 is
`98bda43ab3c0d1be1c6663ad36afeca33ca03c2cac742fc5a5e3c9983084ece8`.
The C07 plan freezes the hand-authored replacement rows and their new digest;
its task review verifies the exact seven-row diff. No Rust test asserts either
digest, derives expected values from production, masks the corrected cases, or
weakens comparison. Every unaffected row remains byte-for-byte identical and
all non-contradictory I01 behavioral suites remain green.

This is the first post-C01 oracle correction authorized by the reviewed P01.
It changes parser outcomes solely where the dated grammar proves the C01
expectation nonconforming; section 3.1 contracts and compatibility signatures
remain frozen. Any additional contradiction stops for P01 reconciliation.

### 3.4 C08 Source-Backed Oracle Correction

P01.11 reconciles the second discovered conflict between the C01 behavioral
oracle and Fonts 3 descriptor semantics. C08 shall replace only the expected
observables for `focused.structured-errors.12`, preserving its stable ID,
stylesheet entry point, both-feature applicability, and authored input:

```css
@font-face { font-family: One; font-family: Two; src: url(test.woff2); }
```

The C01/C07 expectation recovers from the second `font-family` descriptor with
`InvalidDescriptorCombination` and `DropDescriptor`. The dated Fonts 3
Recommendation section 4.1 requires the last declaration to be effective when
a descriptor occurs multiple times. The replacement expectation is therefore
a clean report retaining the valid `@font-face` rule, with both valid family
occurrences preserved in authored order and `Two` exposed by effective lookup.

The fixture SHA-256 before C08 is
`99bbb897710969949d7b596d14fbd352d5d3121a6c4cf663b8ca100154057f8b`.
Replacing exactly that one row yields SHA-256
`67e69813d808ffda40e7c159fde719fbadd0447f8e4105788b0bb593931fac89`.
The C08 task review verifies the one-row diff. No Rust test asserts either
digest, derives the expectation from production, masks the corrected case, or
weakens fixture comparison. Every other row remains byte-for-byte identical.

This correction does not change a section 3.1 parser, report, diagnostic,
coordinate, recovery-action, or feature-parity contract. Invalid descriptor
occurrences still recover with `DropDescriptor`; valid duplicates are retained
and effective-last exactly as section 5 already requires. Any further frozen
oracle contradiction returns to P01 reconciliation before implementation.

## 4. Conformance Profile And Catalog

### 4.1 Tiers, Sources, And Dispositions

Add public non-exhaustive `CssSpecificationTier` with
`Snapshot2026Official`, `Snapshot2026Reliable`, `Snapshot2026Stable`,
`Snapshot2026Interop`, `SurgeistExtension`, and `LaterStandard`. Tier describes
the source/profile only and never implies parser support.

Retain closed `CssSupportStatus::{Complete, Partial, RecognizedUnsupported}`.
Add a separate private conformance disposition for parser-facing support versus
`Excluded(CssExclusionReason)`, where the non-exhaustive reason enum contains
`InformativeOnly`, `SupersededWithoutCurrentProduction`, and
`OutsideAuthoredSyntaxBoundary`. An exclusion is never a hidden unknown or
unsupported authored spelling.

Every source has a stable ID, module, level, tier, and immutable dated URL or
exact repository revision/path. `CssSpecificationSource::url()` and
`repository_provenance()` retain their I01 XOR meaning; additive accessors expose
source ID/module/level/tier. No status is inferred from a URL.

### 4.2 Official Dated Source Registry

Every row below has tier `Snapshot2026Official`. Only its authored-syntax
productions within section 2 are owned.

| ID | Module | Dated normative revision | In-boundary authored syntax |
| --- | --- | --- | --- |
| `O-CSS2` | CSS 2.1 | <https://www.w3.org/TR/2011/REC-CSS2-20110607/> | core rule/property syntax not superseded below, including page syntax |
| `O-SYNTAX3` | CSS Syntax 3 | <https://www.w3.org/TR/2021/CRD-css-syntax-3-20211224/> | tokens, rules, declarations, recovery, encoding declaration |
| `O-STYLE-ATTR` | Style Attributes | <https://www.w3.org/TR/2013/REC-css-style-attr-20131107/> | style declaration lists |
| `O-MEDIA3` | Media Queries 3 | <https://www.w3.org/TR/2024/REC-mediaqueries-3-20240521/> | types, features, query lists |
| `O-CONDITIONAL3` | Conditional Rules 3 | <https://www.w3.org/TR/2024/CRD-css-conditional-3-20240815/> | `@media`, `@supports`, conditions |
| `O-SELECTORS3` | Selectors 3 | <https://www.w3.org/TR/2018/REC-selectors-3-20181106/> | complete selector grammar |
| `O-NAMESPACES3` | Namespaces 3 | <https://www.w3.org/TR/2014/REC-css-namespaces-3-20140320/> | `@namespace`, qualified selector names |
| `O-CASCADE4` | Cascade 4 | <https://www.w3.org/TR/2022/CR-css-cascade-4-20220113/> | global keywords and importance syntax only |
| `O-VALUES3` | Values and Units 3 | <https://www.w3.org/TR/2024/CRD-css-values-3-20240322/> | primitive values, units, typed math |
| `O-VARIABLES1` | Custom Properties 1 | <https://www.w3.org/TR/2022/CR-css-variables-1-20220616/> | custom declarations and symbolic `var()` |
| `O-BOX3` | Box Model 3 | <https://www.w3.org/TR/2024/REC-css-box-3-20240411/> | margin, padding, box values |
| `O-COLOR4` | Color 4 | <https://www.w3.org/TR/2026/CRD-css-color-4-20260326/> | authored colors and opacity |
| `O-BACKGROUNDS3` | Backgrounds and Borders 3 | <https://www.w3.org/TR/2024/CRD-css-backgrounds-3-20240311/> | backgrounds, borders, radii, image borders, shadows |
| `O-IMAGES3` | Images 3 | <https://www.w3.org/TR/2023/CRD-css-images-3-20231218/> | images, gradients, object sizing/positioning/rendering syntax |
| `O-FONTS3` | Fonts 3 | <https://www.w3.org/TR/2018/REC-css-fonts-3-20180920/> | font properties and `@font-face` |
| `O-WRITING3` | Writing Modes 3 | <https://www.w3.org/TR/2019/REC-css-writing-modes-3-20191210/> | direction and writing modes |
| `O-MULTICOL1` | Multi-column 1 | <https://www.w3.org/TR/2024/CR-css-multicol-1-20240516/> | column properties and shorthands |
| `O-FLEXBOX1` | Flexbox 1 | <https://www.w3.org/TR/2025/CRD-css-flexbox-1-20251014/> | flex properties and shorthands |
| `O-UI3` | Basic UI 3 | <https://www.w3.org/TR/2018/REC-css-ui-3-20180621/> | cursor, outline, resize and related syntax |
| `O-CONTAIN1` | Containment 1 | <https://www.w3.org/TR/2024/REC-css-contain-1-20240625/> | `contain` property |
| `O-TRANSFORMS1` | Transforms 1 | <https://www.w3.org/TR/2019/CR-css-transforms-1-20190214/> | transform functions and origin |
| `O-COMPOSITING1` | Compositing 1 | <https://www.w3.org/TR/2024/CRD-compositing-1-20240321/> | blend modes and isolation syntax |
| `O-EASING1` | Easing 1 | <https://www.w3.org/TR/2023/CRD-css-easing-1-20230213/> | easing functions |
| `O-COUNTERSTYLES3` | Counter Styles 3 | <https://www.w3.org/TR/2021/CR-css-counter-styles-3-20210727/> | `@counter-style` and counter values |

`@font-feature-values` is not a Fonts 3 production. It remains the preserved
Fonts 4 `later.rule.font-feature-values` record with
`RecognizedUnsupported`; it does not block official completion.

### 4.3 Preserved Extension Registry

The exact preservation set is every one of the 219 feature IDs returned by
`feature_catalog()` at base `bc5394f`, with its spelling/kind and accepted I01
subset. Official productions are remapped to one `O-*` source. Non-official
productions are remapped to the exact dated source and tier below, or retain the
three explicit repository sources. No unlisted I01 behavior becomes an implied
extension.

| Source ID | Tier | Exact dated source |
| --- | --- | --- |
| `R-MEDIA4` | Reliable | <https://www.w3.org/TR/2026/CRD-mediaqueries-4-20260219/> |
| `R-SCROLLBARS1` | Reliable | <https://www.w3.org/TR/2021/CR-css-scrollbars-1-20211209/> |
| `R-GRID1` | Reliable | <https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/> |
| `R-GRID2` | Reliable | <https://www.w3.org/TR/2025/CRD-css-grid-2-20250326/> |
| `R-CASCADE5` | Reliable | <https://www.w3.org/TR/2022/CR-css-cascade-5-20220113/> |
| `R-CONDITIONAL4` | Reliable | <https://www.w3.org/TR/2025/CRD-css-conditional-4-20250904/> |
| `S-DISPLAY3` | Stable | <https://www.w3.org/TR/2026/CRD-css-display-3-20260605/> |
| `S-WRITING4` | Stable | <https://www.w3.org/TR/2019/CR-css-writing-modes-4-20190730/> |
| `S-BREAK3` | Stable | <https://www.w3.org/TR/2018/CR-css-break-3-20181204/> |
| `S-ALIGN3` | Stable | <https://www.w3.org/TR/2026/WD-css-align-3-20260130/> |
| `S-SHAPES1` | Stable | <https://www.w3.org/TR/2025/CRD-css-shapes-1-20250612/> |
| `S-TEXT3` | Stable | <https://www.w3.org/TR/2026/CRD-css-text-3-20260608/> |
| `S-TEXTDECOR3` | Stable | <https://www.w3.org/TR/2022/CRD-css-text-decor-3-20220505/> |
| `S-MASKING1` | Stable | <https://www.w3.org/TR/2021/CRD-css-masking-1-20210805/> |
| `I-TRANSITIONS1` | Interop | <https://www.w3.org/TR/2026/WD-css-transitions-1-20260108/> |
| `I-ANIMATIONS1` | Interop | <https://www.w3.org/TR/2023/WD-css-animations-1-20230302/> |
| `I-FILTER1` | Interop | <https://www.w3.org/TR/2018/WD-filter-effects-1-20181218/> |
| `I-SIZING3` | Interop | <https://www.w3.org/TR/2021/WD-css-sizing-3-20211217/> |
| `I-TRANSFORMS2` | Interop | <https://www.w3.org/TR/2021/WD-css-transforms-2-20211109/> |
| `I-LISTS3` | Interop | <https://www.w3.org/TR/2020/WD-css-lists-3-20201117/> |
| `I-POSITION3` | Interop | <https://www.w3.org/TR/2025/WD-css-position-3-20251007/> |
| `I-FONTS4` | Interop | <https://www.w3.org/TR/2026/WD-css-fonts-4-20260422/> |
| `I-COLOR5` | Interop | <https://www.w3.org/TR/2026/WD-css-color-5-20260618/> |
| `I-SELECTORS4` | Interop | <https://www.w3.org/TR/2026/WD-selectors-4-20260122/> |
| `I-CONTAIN2` | Interop | <https://www.w3.org/TR/2022/WD-css-contain-2-20220917/> |
| `I-NESTING1` | Interop | <https://www.w3.org/TR/2026/WD-css-nesting-1-20260122/> |
| `X-CONTAIN3` | Surgeist extension | <https://www.w3.org/TR/2022/WD-css-contain-3-20220818/> |
| `X-CONDITIONAL5` | Surgeist extension | <https://www.w3.org/TR/2025/WD-css-conditional-5-20251030/> |
| `X-CASCADE6` | Surgeist extension | <https://www.w3.org/TR/2024/WD-css-cascade-6-20240906/> |
| `X-PSEUDO4` | Surgeist extension | <https://www.w3.org/TR/2025/WD-css-pseudo-4-20250627/> |
| `X-VALUES4` | Surgeist extension | `720ea2863696971ea6a6744e0f23acbb3e6936bd:css-values-4/Overview.bs` |
| `X-MEDIA5` | Surgeist extension | <https://www.w3.org/TR/2026/WD-mediaqueries-5-20260219/> |
| `X-OVERFLOW3` | Surgeist extension | <https://www.w3.org/TR/2025/WD-css-overflow-3-20251007/> |
| `X-SIZING4` | Surgeist extension | <https://www.w3.org/TR/2021/WD-css-sizing-4-20210520/> |
| `X-TEXT4` | Surgeist extension | <https://www.w3.org/TR/2026/WD-css-text-4-20260608/> |
| `X-TEXTDECOR4` | Surgeist extension | <https://www.w3.org/TR/2022/WD-css-text-decor-4-20220504/> |
| `X-UI4` | Surgeist extension | <https://www.w3.org/TR/2026/WD-css-ui-4-20260120/> |
| `X-CONTENT3` | Surgeist extension | <https://www.w3.org/TR/2025/WD-css-content-3-20251204/> |
| `X-FULLSCREEN` | Surgeist extension | <https://www.w3.org/TR/2012/WD-fullscreen-20120703/> |
| `X-FILTER2-BASE` | Surgeist extension | `bc5394f:src/parser/effects.rs` |
| `X-DISPLAY-MODE-BASE` | Surgeist extension | `bc5394f:src/parser/queries.rs` |
| `X-GRID-TOLERANCE-BASE` | Surgeist extension | `bc5394f:src/parser/grid.rs` |

`R-SCROLLBARS1` has module `CSS Scrollbars Styling`, level `1`, and owns only
the preserved `baseline.property.scrollbar-width` record. CSS Snapshot 2026
section 2.2 classifies that source as Reliable. `X-CONTAIN3` has module
`CSS Containment`, level `3`, and owns only the preserved
`baseline.rule.container`, `baseline.container.condition`, and
`baseline.container.size-feature` query productions. CSS Snapshot 2026 does
not classify Containment 3; its exact 18 August 2022 Working Draft is therefore
a Surgeist extension source rather than an Interop source. `I-CONTAIN2` remains
the owner only of preserved Level 2 productions and must not be used as
provenance for container-query or style-query syntax.

### 4.4 Independent Inventory And Status Rules

The immutable official coverage universe is
`plans/specs/P01-I02-css-snapshot-2026-official-ledger.md`, SHA-256
`746e8fe722eea56c2b6d3d8072480a91fb2473a6c25296cd466556f4bef51ced`.
It enumerates exactly 162 property units, 167 non-property units, the one
normative legacy-shorthand alias, all supersession mappings, and the complete
selected exclusion remainder. The implementation may consume its stable IDs in
tests but shall not generate the catalog, parser inventories, or vectors from
the ledger file.

The hand-authored conformance catalog remains independent of parser dispatch.
It grows from the exact 219-row I01 baseline to include every official
parser-facing property, descriptor, at-rule, qualified rule, selector, media
type/feature, and shared value production, plus exact exclusions for official
source items outside section 2.

Each feature has one stable ID, kind, spelling, source ID, exact production,
tier, and disposition. Existing IDs retain their bounded meaning. New official
IDs use `official.<kind>.<canonical-production>`; an official production already
owned by an exact baseline ID retains that ID and changes provenance/status
without duplication. Exclusions use `excluded.<source-id>.<production>`.

The four mixed I01 aggregate IDs named in ledger section 4 become immutable
`BaselineAlias` records with repository provenance and exact atomic target
slices. They remain queryable through `feature_metadata`; additive
`baseline_alias_targets()` exposes the target IDs. They have no implementation
or vector row after atomic migration and do not count as parser-facing coverage.
Every other I01 ID migrates directly. The target union for each alias must equal
its I01 accepted/boundary behavior, preventing a source/tier lie.

Every implementation-owning module exposes a crate-private stable-ID inventory
for its kind. Properties continue through `property_schema!`; rules,
descriptors, selectors, media types/features, qualified rules, and shared values
gain their own implementation inventories. Separately hand-authored behavioral
cases exercise positive, negative, and recovery outcomes through the public
parser and may assert the public metadata associated with their explicit stable
ID. Rust tests do not compare catalog, implementation, and test-vector owner
sets/counts or use omission/extra/duplicate/status mutations as completeness
proxies. The coordinator and reviewers reconcile the official ledger, public
catalog, implementation ownership, and behavioral evidence directly. No one
owner generates either of the other two.

At initiative completion:

- every official parser-facing row is `Complete` and has exact vectors;
- every official source item is exactly one parser-facing row or one justified
  exclusion, never both;
- preserved extensions are `Complete`, `Partial`, or
  `RecognizedUnsupported` truthfully, with exact subset/remainder or diagnostic
  identity; except for the exact seven section 3.3 source corrections, no
  preserved I01 accepted vector regresses;
- unknown spellings remain distinct from recognized unsupported spellings;
- exact I01 baseline tests become subset-preservation tests rather than replacing
  `219`, `179`, and `40` with ungrounded new totals.

## 5. Rules, Ordering, Descriptors, And Keyframes

Add typed `CssRule` variants and private-field models for `@namespace`,
`@supports`, `@counter-style`, and `@page`. Each implements its complete selected
grammar, placement/nesting contexts, descriptors, recovery boundaries, source
positions, and public accessors. `@font-feature-values` remains recognized
unsupported with exact metadata.

Replace the top-level `imports_allowed` boolean with a phase machine for initial
layer statements, consecutive imports, consecutive namespaces, and body rules.
Initial empty `@layer` statements do not close imports; an intervening layer
statement after an import or namespace does. Namespace declarations are
top-level only. Phase transitions occur only after successful rule parsing.

`@import` gains typed optional `layer`, `supports()`, and media clauses in the
specification order. The official Cascade 4 core, reliable Cascade 5 layer
delta, and reliable Cascade 5 supports delta are distinct atomic catalog rows.
The supports clause shares the `@supports` condition parser.
Conditional 3's imported `general-enclosed` grammar is bound to the immutable
`X-VALUES4` repository revision and path in section 4.3 as its own atomic delta.
That revision contributes only `<general-enclosed>`; its sibling generic boolean
grammar is not imported into this profile. The `@media` rule
shell and `baseline.rule.media` record bind to `O-CONDITIONAL3`; their imported
`<media-query-list>` grammar binds to the `O-MEDIA3` core plus separately
catalogued `R-MEDIA4` deltas. No moving imported URL becomes conformance input.
Unknown/general-enclosed tests are retained as authored syntax where the owning
grammar requires, not recovered as malformed.

Counter-style and page descriptors use source-order occurrence lists,
effective-last lookup where defined, exact required/conflicting descriptor
validation, and `DropDescriptor` recovery. Page blocks reuse ordinary
declarations and do not add later margin-box rules. Font-face closure includes
the complete Fonts 3 descriptor grammar while preserving selected Fonts 4
descriptor extensions truthfully.

Keyframes accept empty rule/block lists and preserve duplicate selector blocks
and duplicate offsets in authored order. Important keyframe declarations remain
unrepresentable and recover as established by I01. Invalid selectors/blocks use
the existing smallest-unit actions.

## 6. Selectors And Namespaces

Implement the complete Selectors 3 authored grammar: universal/type selectors;
namespace-qualified type, universal, and attribute names; ordered repeated IDs
and classes; all matchers; `:link`, `:visited`, `:target`, `:lang()`; the full
structural/UI pseudo-class set; `::first-line` and `::first-letter` including
allowed legacy single-colon spelling; and all four combinators.

The namespace model preserves default, explicit-none, any, and named-prefix
constraints. Named prefixes must have an earlier active declaration;
attributes do not inherit the default namespace. URI resolution is out of
scope. Namespace declaration order, null namespace, escapes, and undeclared
prefix diagnostics receive exact vectors.

`CssCompoundSelector` stores IDs as an ordered collection and never overwrites
an occurrence. Existing Selectors 4/nesting/scope extensions retain their I01
recovery distinctions: only `:is()` and `:where()` are forgiving; `:not()`,
`:has()`, nth `of`, style-rule, scope, and nesting lists remain unforgiving.

## 7. Media, Supports, And Conditional Syntax

Implement all Media Queries 3 types (`all`, `aural`, `braille`, `embossed`,
`handheld`, `print`, `projection`, `screen`, `speech`, `tty`, `tv`) and the
complete feature set: width/height, device width/height, orientation, aspect
ratios, color, color-index, monochrome, resolution, scan, and grid. Enforce each
feature's exact boolean/value/min/max/unit/domain grammar. Preserve syntactically
complete unknown media types/features/values in typed defined-false authored
nodes where MQ3 requires; malformed comma members retain `Never` plus
`ReplaceMediaQueryWithNever`.

`@supports` implements declaration, selector, `not`, `and`, `or`, grouping, and
general-enclosed conditions with the grammar's no-mixed-operator rule. A
declaration test parses through the authoritative property schema but does not
apply substitution or expose it as an ordinary declaration. Conditional group
rules reuse the established rule-context and nesting recovery coordinators.

Existing MQ4 range and container-query extensions remain typed and truthful.
No query is evaluated by this crate.

## 8. Shared Value And Historical Defect Closure

### 8.1 Numeric And Math Foundation

All current numeric wrappers reject non-finite values. Frozen I01 compatibility
payloads retain their existing signatures, but no parser path may put a
non-finite value into them and every property-specific current accessor exposes
only checked scalar state. Split non-negative durations from signed delays and
use finite animation iteration counts with a distinct `infinite` branch.
Shorthands assign first/second time values by grammar without conflating their
domains.

Replace sum-only calc modeling with typed sum/product/negation/group/nested-calc
trees for number, integer, percentage, length, angle, time, and frequency roots.
Enforce dimensional promotion and pure-number zero-divisor rules without doing
layout/unit resolution. Literal non-negative constraints are checked at parse;
well-typed calculations remain representable when range enforcement belongs to
computed-value processing.

### 8.2 Positions, Functions, Colors, Grid, And Typography

Use distinct models/parsers for generic `<position>`, layered background/mask
positions, and transform-origin including its optional z offset. Enforce exact
axis, center, edge-offset, arity, and per-property list grammar.

Replace generic authored function-argument validators with dedicated typed
transform, easing, filter, shadow, and basic-shape grammars. Exact separators,
arities, keyword domains, unit-interval cubic-bezier x coordinates, step-count/
`jump-none` rules, and shape/drop-shadow restrictions are parse invariants.

Relative colors are parameterized by color space and channel slot. Each slot
accepts only its allowed channel identifiers and numeric/percentage/angle/math
domain; arbitrary identifiers and unrelated dimensions are invalid. Conversion
and evaluation remain out of scope.

Grid uses distinct fixed-repeat, auto-repeat, and repeat-content models. Repeat
content cannot contain repeat; auto-repeat accepts fixed-size tracks only.

Typography requires four ASCII-character OpenType tags, non-negative feature
values, and whole-property-only global keywords for family lists. Escapes,
quoted global-looking names, supplementary characters, and signed boundaries
receive exact tests.

## 9. Official Property Grammar Closure

The immutable ledger named in section 4.4 is independent of
`property_schema!`. Each official canonical property and reviewed alias is
either superseded/excluded exactly there or has exactly one schema row and
property-specific wrapper. The one legacy `glyph-orientation-vertical`
shorthand uses an explicit alias kind/parser/mapping rather than the schema's
name-equivalent alias array. The `--*` declaration-family row remains outside
`CssKnownProperty`. The ledger may not be derived from the parser.

Each official property is complete, including every longhand/shorthand branch,
list layer, separator, keyword, numeric range, and authored distinction in its
selected grammar. Required missing families include CSS2 residual properties,
full backgrounds/borders/images/gradients, multicolumn, containment,
compositing/blending, UI3, writing modes, fonts, flexbox, and every official
property exposed by the fixed indexes. Existing extension properties remain in
the same schema and retain truthful status.

For every Complete property, independent vectors include a non-global typed
positive, all property keyword families, global and substitution-dependent
values, adjacent-grammar rejection, numeric boundaries, list/shorthand
separator and arity mutations, and exact recovery diagnostics. `inherit` alone
never counts as positive grammar evidence.

No recognized official property is `Unknown`. A deliberately unsupported
non-official property may use `UnsupportedProperty` only with an exact catalog
row/feature ID and a metadata representation that cannot panic when no
`CssKnownProperty` exists. Custom properties remain outside property metadata.

## 10. Public API, Recovery, And Documentation Evidence

All new public fields are private; constructors are checked or parser-owned.
New evolving enums are non-exhaustive. Public tests consume crate-root exports
only and never parse `Debug`/`Display` for control flow.

Every grammar negative asserts exact `CssErrorCode`, structured payload, stable
feature/property identity when applicable, first-responsible position, complete
recovery span, and action. Tests cover sibling retention, nested parent loss,
EOF closure, repeated failures, non-BMP coordinates, and depth 255/256/257 for
new recursive grammars. Panic-freedom adversarial tests require progress without
unchecked indexing, `unwrap`, `expect`, or input-dependent `unreachable!`.

README, rustdoc, doctests, and external public consumers document and exercise
the complete official profile, preserved extension statuses, recovered versus
clean reports, the breaking C01 declaration/value migration, selectors,
namespaces, rules/descriptors, queries, calculations, and property-specific
inspection. They reiterate the excluded downstream semantics.

I01 behavioral tests remain baseline-preservation evidence, subject only to the
seven source-backed C07 corrections in section 3.3 and the one source-backed
C08 correction in section 3.4. At I02 completion,
the coordinator and reviewers map every acceptance item to direct source
inspection, compiler-visible API evidence, behavioral tests, or deterministic
checks of declared product artifacts as appropriate. No Rust test encodes an
initiative predicate, plan/task/review/publication state, command manifest, or
source/code shape as completion evidence.

## 11. Finding Closure Matrix

| Finding | Required I02 closure |
| --- | --- |
| `2.1` | sections 4, 5: official at-rules, descriptors, contexts, recovery |
| `2.2` | sections 4 and 9: independent official property ledger and complete property grammars |
| `2.3` | section 6: complete Selectors 3 |
| `2.4` | section 7: complete Media Queries 3 |
| `2.7` | sections 5 and 7: top-level phase machine and import conditions |
| `2.8` | section 6: ordered repeated IDs |
| `2.9` | section 8.2: property-specific positions |
| `2.10` | section 8.2: dedicated function grammars |
| `2.11` | section 8.1: duration/delay/finite numeric domains |
| `2.12` | section 8.1: typed math and range phase |
| `2.13` | section 8.2: typed relative-color channels |
| `2.14` | section 8.2: structurally valid Grid repeat |
| `2.16` | section 8.2: typography tags/indices/global-list rules |
| `2.17` | section 5: duplicate/empty keyframe structures |

I02 also preserves all I01 finding evidence after the reviewed section 3.3
correction. If closing a row would change another frozen section 3.1 semantic,
require unsafe, cross an ownership boundary, or require another breaking cycle,
stop and reconcile P01.

## 12. Initiative Acceptance

I02 is complete only when all predicates hold:

1. C01 performs the exact one-time section 3.2 break before grammar work and a
   superseding migration record covers every affected public type/root action.
2. Every official source item is exactly one Complete parser-facing row or one
   justified exclusion; every parser-facing row has one kind/source/tier, a
   reviewed implementation mapping, and independently authored public-parser
   behavioral evidence. Completeness is reconciled directly from the ledger and
   owning artifacts, not inferred by a Rust test from owner identity sets or
   counts.
3. The exact 219-row I01 feature baseline remains classified; the seven C07
   scenario IDs in section 3.3 and the one C08 scenario ID in section 3.4 carry
   their reviewed source-backed replacement observables and every other
   accepted vector does not regress; extension status and provenance are
   truthful.
4. All fourteen allocated findings in section 11 have implemented source and
   focused exact evidence, while the reconciled I01 evidence stays green.
5. Both ordinary front doors retain valid siblings and report every recovery;
   both validators reject exactly non-clean reports under either feature graph.
6. All current public authored states are valid by construction; non-finite
   numbers, mismatched property values, recursive repeats, invalid function
   domains, namespace errors, and invalid grammar combinations are
   unconstructable. Frozen I01 compatibility payloads may retain legacy public
   construction shapes, but parser-produced projections always satisfy the
   current invariant and property-specific current access never exposes an
   invalid compatibility state.
7. Public docs and consumers expose the complete official grammar and exact
   metadata/recovery/downstream boundaries without relying on private modules.
8. Default and `app-strict` check/test/doctest/Clippy matrices, focused catalog
   and behavioral suites, warning-denied rustdoc, formatting, diff checks,
   dependency/feature checks, no-unsafe scans, process hygiene, and `cargo clean`
   all pass.
9. Every cycle and the exact initiative receive fresh independent review; the
   immutable candidate is lease-published to leaf `main`, fetched/read back, and
   handed off with full SHAs and root-only follow-up. No root or sibling file is
   changed.

Only after this handoff may P01-I03 be specified JIT.

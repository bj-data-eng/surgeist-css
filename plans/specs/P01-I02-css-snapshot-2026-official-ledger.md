# P01-I02 Official Production Ledger

## 1. Contract And Counting Unit

This is the one-hop immutable coverage ledger owned by the P01-I02
specification. Its sources are the 24 exact dated official revisions in I02
section 4.2. A coverage unit is one independently testable authored property,
rule, declaration, descriptor, selector production, media type/feature family,
or shared value/function family. Grammar helper nonterminals remain with their
author-facing production. This convention yields exactly 162 property units
(161 ordinary properties plus the `--*` custom-property family) and 167
non-property units.

Each canonical property uses `#propdef-<canonical>` in its owning module unless
an exact CSS2 chapter/fragment is supplied below. Each non-property row supplies
its exact fragment family. The stable ID for a property already present in the
179-row schema at `bc5394f` is its existing `baseline.property.<canonical>`;
otherwise it is `official.property.<canonical>`. `--*` retains
`baseline.declaration.custom-property`. The only normative property alias is
the special legacy shorthand in section 2.3; ordinary schema aliases remain
name-equivalent and empty at the initiative base.

The totals are an acceptance invariant. Catalog, implementation, vector, and
source-audit inventories compare against these rows without deriving them from
parser dispatch.

## 2. Official Property Units

### 2.1 Canonical Inventory

| Source | Exact canonical property units |
| --- | --- |
| `O-CSS2` | `border-collapse`, `border-spacing`, `bottom`, `caption-side`, `clear`, `clip`, `content`, `counter-increment`, `counter-reset`, `display`, `empty-cells`, `float`, `height`, `left`, `letter-spacing`, `line-height`, `list-style`, `list-style-image`, `list-style-position`, `list-style-type`, `max-height`, `max-width`, `min-height`, `min-width`, `orphans`, `overflow`, `page-break-after`, `page-break-before`, `page-break-inside`, `position`, `quotes`, `right`, `table-layout`, `text-align`, `text-decoration`, `text-indent`, `text-transform`, `top`, `vertical-align`, `visibility`, `white-space`, `widows`, `width`, `word-spacing`, `z-index` |
| `O-CASCADE4` | `all` |
| `O-VARIABLES1` | `--*` custom-property declaration family |
| `O-BOX3` | `margin`, `margin-bottom`, `margin-left`, `margin-right`, `margin-top`, `padding`, `padding-bottom`, `padding-left`, `padding-right`, `padding-top` |
| `O-COLOR4` | `color`, `opacity` |
| `O-BACKGROUNDS3` | `background`, `background-attachment`, `background-clip`, `background-color`, `background-image`, `background-origin`, `background-position`, `background-repeat`, `background-size`, `border`, `border-bottom`, `border-bottom-color`, `border-bottom-left-radius`, `border-bottom-right-radius`, `border-bottom-style`, `border-bottom-width`, `border-color`, `border-image`, `border-image-outset`, `border-image-repeat`, `border-image-slice`, `border-image-source`, `border-image-width`, `border-left`, `border-left-color`, `border-left-style`, `border-left-width`, `border-radius`, `border-right`, `border-right-color`, `border-right-style`, `border-right-width`, `border-style`, `border-top`, `border-top-color`, `border-top-left-radius`, `border-top-right-radius`, `border-top-style`, `border-top-width`, `border-width`, `box-shadow` |
| `O-IMAGES3` | `image-orientation`, `image-rendering`, `object-fit`, `object-position` |
| `O-FONTS3` | `font`, `font-family`, `font-feature-settings`, `font-kerning`, `font-size`, `font-size-adjust`, `font-stretch`, `font-style`, `font-synthesis`, `font-variant`, `font-variant-caps`, `font-variant-east-asian`, `font-variant-ligatures`, `font-variant-numeric`, `font-variant-position`, `font-weight` |
| `O-WRITING3` | `direction`, `text-combine-upright`, `text-orientation`, `unicode-bidi`, `writing-mode` |
| `O-MULTICOL1` | `column-count`, `column-fill`, `column-rule`, `column-rule-color`, `column-rule-style`, `column-rule-width`, `column-span`, `column-width`, `columns` |
| `O-FLEXBOX1` | `align-content`, `align-items`, `align-self`, `flex`, `flex-basis`, `flex-direction`, `flex-flow`, `flex-grow`, `flex-shrink`, `flex-wrap`, `justify-content` |
| `O-UI3` | `box-sizing`, `caret-color`, `cursor`, `outline`, `outline-color`, `outline-offset`, `outline-style`, `outline-width`, `resize`, `text-overflow` |
| `O-CONTAIN1` | `contain` |
| `O-TRANSFORMS1` | `transform`, `transform-box`, `transform-origin` |
| `O-COMPOSITING1` | `background-blend-mode`, `isolation`, `mix-blend-mode` |

All other official sources have zero property units. The table contains 161
ordinary canonical names plus `--*`.

### 2.2 Exact CSS2 Fragments

CSS2 owners use these chapter groups and `#propdef-<canonical>`:

- `tables.html`: `border-collapse`, `border-spacing`, `caption-side`,
  `empty-cells`, `table-layout`;
- `visuren.html`: `bottom`, `clear`, `display`, `float`, `left`, `position`,
  `right`, `top`, `z-index`;
- `visufx.html`: `clip`, `overflow`, `visibility`;
- `generate.html`: `content`, `counter-increment`, `counter-reset`, `list-style`,
  `list-style-image`, `list-style-position`, `list-style-type`, `quotes`;
- `visudet.html`: `height`, `line-height`, `max-height`, `max-width`,
  `min-height`, `min-width`, `vertical-align`, `width`;
- `text.html`: `letter-spacing`, `text-align`, `text-decoration`,
  `text-indent`, `text-transform`, `white-space`, `word-spacing`;
- `page.html`: `orphans`, `page-break-after`, `page-break-before`,
  `page-break-inside`, `widows`.

### 2.3 Alias, Base Delta, And Supersession

`glyph-orientation-vertical` is one normative optional legacy shorthand at
`O-WRITING3:#propdef-glyph-orientation-vertical`. It maps its restricted grammar
to `text-orientation`; it is not a name-equivalent schema alias. The property
schema shall add an explicit `LegacyShorthand` alias kind, parser, and mapping.
`glyph-orientation-horizontal` and UI3 `ime-mode` have no current CSS production
and are exact `SupersededWithoutCurrentProduction` exclusions.

Exactly these 52 ordinary official properties are absent from the base schema:

```text
border-collapse border-spacing caption-side clip empty-cells orphans
page-break-after page-break-before page-break-inside quotes table-layout widows
word-spacing border-image border-image-outset border-image-repeat
border-image-slice border-image-source border-image-width image-orientation
image-rendering object-fit object-position font-kerning font-size-adjust
font-synthesis font-variant-caps font-variant-east-asian
font-variant-ligatures font-variant-numeric font-variant-position
text-combine-upright text-orientation unicode-bidi column-count column-fill
column-rule column-rule-color column-rule-style column-rule-width column-span
column-width columns flex-flow caret-color outline-offset resize contain
transform-box background-blend-mode isolation mix-blend-mode
```

The other 109 ordinary units already have `baseline.property.*` rows and are
`Partial` at the base. The other 70 base schema rows are preserved non-official
extensions. The `--*` family remains outside `CssKnownProperty` and property
metadata by design.

Later official modules supersede same-named CSS2 property definitions exactly
for these 50 names: all ten `O-BOX3` properties; `color`; `background`,
`background-attachment`, `background-color`, `background-image`,
`background-position`, `background-repeat`, `border`, `border-bottom`,
`border-bottom-color`, `border-bottom-style`, `border-bottom-width`,
`border-color`, `border-left`, `border-left-color`, `border-left-style`,
`border-left-width`, `border-right`, `border-right-color`,
`border-right-style`, `border-right-width`, `border-style`, `border-top`,
`border-top-color`, `border-top-style`, `border-top-width`, and `border-width`;
`font`, `font-family`, `font-size`, `font-style`, `font-variant`, and
`font-weight`; `direction` and `unicode-bidi`; and `cursor`, `outline`,
`outline-color`, `outline-style`, and `outline-width`.
Each predecessor is an exact
`excluded.O-CSS2.property.<canonical>`
`SupersededWithoutCurrentProduction` row whose owner is the later canonical
unit. CSS2 properties remaining in the `O-CSS2` row are not superseded by a
non-official extension; for example `page-break-*` remains official while its
`break-*` relationship belongs to the preserved `S-BREAK3` extension.

### 2.4 Property Exclusions

CSS2 Appendix A is explicitly informative. Its exact 20 property-index names
are `azimuth`, `cue`, `cue-after`, `cue-before`, `elevation`, `pause`,
`pause-after`, `pause-before`, `pitch`, `pitch-range`, `play-during`, `richness`,
`speak`, `speak-header`, `speak-numeral`, `speak-punctuation`, `speech-rate`,
`stress`, `voice-family`, and `volume`. Each is
`excluded.O-CSS2.informative-property.<name>` with `InformativeOnly` and its
`aural.html#propdef-<name>` fragment.

`image-orientation` remains parser-facing despite being deprecated, optional,
and at-risk in the selected Images 3 revision. `glyph-orientation-horizontal`
and `ime-mode` use the exclusions in 2.3, not recognized-unsupported rows.

The property-index equation is fixed:

```text
233 indexed spellings = 162 current canonical units
                      + 1 normative legacy-shorthand alias
                      + 50 superseded CSS2 definitions
                      + 20 informative CSS2 Appendix A exclusions
```

## 3. Official Non-Property Units

Every semicolon-delimited assignment below is one coverage row. Parenthesized
aliases are part of the named row rather than another row.

| Source | Count | Exact fragments and stable IDs |
| --- | ---: | --- |
| `O-CSS2` | 2 | `page.html#page-box`: `later.rule.page=@page`; `page.html#page-selectors`: `official.selector.page-pseudo=:left|:right|:first` |
| `O-SYNTAX3` | 17 | `#tokenization,#parser-entry-points,#consume-at-rule,#consume-qualified-rule,#consume-declaration,#consume-component-value,#consume-simple-block,#consume-function,#declaration-rule-list,#any-value,#the-anb-type,#urange-syntax,#style-rules,#at-rules,#charset-rule`: `foundation.encoding.charset`, `baseline.rule.style`, `official.rule.at-rule`, `official.qualified-rule.generic`, `official.declaration.generic`, `official.value.syntax-token-stream`, `official.value.component-value`, `official.value.simple-block`, `official.value.function`, `official.value.stylesheet`, `official.value.rule-list`, `official.value.declaration-list`, `official.value.style-block`, `official.value.declaration-value`, `official.value.any-value`, `official.value.an-plus-b`, `official.value.unicode-range` |
| `O-STYLE-ATTR` | 1 | `#syntax`: `foundation.declaration-list.style-attribute` |
| `O-MEDIA3` | 15 | `#syntax,#media1,#width,#height,#device-width,#device-height,#orientation,#aspect-ratio,#device-aspect-ratio,#color,#color-index,#monochrome,#resolution,#scan,#grid`: `official.media.query-list-core`, `baseline.media.type`, `official.media.feature.width`, `official.media.feature.height`, `official.media.feature.device-width`, `official.media.feature.device-height`, `official.media.feature.orientation`, `official.media.feature.aspect-ratio`, `official.media.feature.device-aspect-ratio`, `official.media.feature.color`, `official.media.feature.color-index`, `official.media.feature.monochrome`, `official.media.feature.resolution`, `official.media.feature.scan`, `official.media.feature.grid` |
| `O-CONDITIONAL3` | 3 | `#contents,#placement,#at-media,#at-supports`: `baseline.rule.media`, `later.rule.supports`, `official.rule.conditional-group-context` |
| `O-SELECTORS3` | 20 | `#grouping,#type-selectors,#universal-selector,#attribute-representation,#attribute-substrings,#class-html,#id-selectors,#dynamic-pseudos,#target-pseudo,#lang-pseudo,#UIstates,#structural-pseudos,#negation,#first-line,#first-letter,#gen-content,#descendant-combinators,#child-combinators,#adjacent-sibling-combinators,#general-sibling-combinators`: `official.selector.group`, `official.selector.type`, `official.selector.universal`, `official.selector.attribute-presence-value`, `official.selector.attribute-substring`, `official.selector.class`, `official.selector.id`, `official.selector.dynamic`, `official.selector.target`, `official.selector.lang`, `official.selector.ui-state`, `official.selector.structural`, `official.selector.negation`, `official.selector.first-line`, `official.selector.first-letter`, `official.selector.generated`, `official.selector.combinator.descendant`, `official.selector.combinator.child`, `official.selector.combinator.next-sibling`, `official.selector.combinator.subsequent-sibling` |
| `O-NAMESPACES3` | 2 | `#declaration,#syntax,#scope,#prefixes,#css-qnames`: `later.rule.namespace`, `official.selector.namespace-qualified-name` |
| `O-CASCADE4` | 3 | `#at-import,#importance,#defaulting-keywords`: `baseline.rule.import`, `foundation.declaration.importance`, `official.value.css-wide-keyword` |
| `O-VALUES3` | 20 | `#custom-idents,#strings,#urls,#url-modifiers,#integers,#numbers,#dimensions,#percentages,#lengths,#mixed-percentages,#angles,#time,#frequency,#resolution,#position,#calc-notation,#calc-syntax,#calc-type-checking`: `official.value.custom-ident`, `official.value.ident`, `official.value.string`, `official.value.url`, `official.value.url-modifier`, `official.value.integer`, `official.value.number`, `official.value.dimension`, `official.value.percentage`, `official.value.length`, `official.value.length-percentage`, `official.value.angle`, `official.value.angle-percentage`, `official.value.time`, `official.value.time-percentage`, `official.value.frequency`, `official.value.frequency-percentage`, `official.value.resolution`, `official.value.position`, `official.value.calc` |
| `O-VARIABLES1` | 2 | `#defining-variables,#syntax,#using-variables`: `baseline.declaration.custom-property`, `baseline.value.substitution-dependent` |
| `O-BOX3` | 1 | `#keywords`: `official.value.box-edge-keywords` |
| `O-COLOR4` | 17 | `#color-type,#alpha-syntax,#hue-syntax,#rgb-functions,#hex-notation,#named-colors,#css-system-colors,#transparent-color,#currentcolor-color,#the-hsl-notation,#the-hwb-notation,#specifying-lab-lch,#specifying-oklab-oklch,#color-function`: `official.value.color`, `official.value.alpha`, `official.value.hue`, `official.value.rgb`, `official.value.hex-color`, `official.value.named-color`, `official.value.system-color`, `official.value.deprecated-system-color`, `official.value.transparent`, `official.value.currentcolor`, `official.value.hsl`, `official.value.hwb`, `official.value.lab`, `official.value.lch`, `official.value.oklab`, `official.value.oklch`, `official.value.predefined-color` |
| `O-BACKGROUNDS3` | 9 | `#layering,#background-image,#background-repeat,#background-attachment,#background-position,#background-size,#border-style,#border-width,#box-shadow`: `official.value.background-layer`, `official.value.background-image`, `official.value.repeat-style`, `official.value.background-attachment`, `official.value.background-position`, `official.value.background-size`, `official.value.line-style`, `official.value.line-width`, `official.value.shadow` |
| `O-IMAGES3` | 11 | `#image-values,#gradients,#linear-gradients,#radial-gradients,#repeating-gradients,#color-stop-syntax`: `official.value.image`, `official.value.gradient`, `official.value.linear-gradient`, `official.value.radial-gradient`, `official.value.repeating-linear-gradient`, `official.value.repeating-radial-gradient`, `official.value.color-stop-list`, `official.value.side-or-corner`, `official.value.radial-shape`, `official.value.radial-size`, `official.value.radial-extent` |
| `O-FONTS3` | 10 | `#font-face-rule,#font-family-desc,#src-desc,#font-prop-desc,#unicode-range-desc,#font-rend-desc`: `baseline.rule.font-face`, `baseline.descriptor.font-family`, `baseline.descriptor.src`, `baseline.descriptor.font-style`, `baseline.descriptor.font-weight`, `baseline.descriptor.font-stretch`, `baseline.descriptor.unicode-range`, `official.descriptor.font-feature-settings`, `official.value.font-source`, `official.value.opentype-tag` |
| `O-TRANSFORMS1` | 13 | `#transform-functions,#two-d-transform-functions,#transform-function-lists`: `official.value.transform-list`, `official.value.transform-function`, `official.value.transform.matrix`, `official.value.transform.translate`, `official.value.transform.translate-x`, `official.value.transform.translate-y`, `official.value.transform.scale`, `official.value.transform.scale-x`, `official.value.transform.scale-y`, `official.value.transform.rotate`, `official.value.transform.skew`, `official.value.transform.skew-x`, `official.value.transform.skew-y` |
| `O-COMPOSITING1` | 1 | `#blending,#blendingseparable,#blendingnonseparable`: `official.value.blend-mode` |
| `O-EASING1` | 4 | `#easing-functions,#cubic-bezier-easing-functions,#step-easing-functions`: `official.value.easing-function`, `official.value.cubic-bezier-easing`, `official.value.step-easing`, `official.value.step-position` |
| `O-COUNTERSTYLES3` | 16 | `#the-counter-style-rule,#counter-style-system,#counter-style-negative,#counter-style-prefix,#counter-style-suffix,#counter-style-range,#counter-style-pad,#counter-style-fallback,#counter-style-symbols,#counter-style-speak-as,#symbols-function`: `later.rule.counter-style`, `official.descriptor.counter-style.system`, `official.descriptor.counter-style.negative`, `official.descriptor.counter-style.prefix`, `official.descriptor.counter-style.suffix`, `official.descriptor.counter-style.range`, `official.descriptor.counter-style.pad`, `official.descriptor.counter-style.fallback`, `official.descriptor.counter-style.symbols`, `official.descriptor.counter-style.additive-symbols`, `official.descriptor.counter-style.speak-as`, `official.value.counter-style`, `official.value.counter-style-name`, `official.value.symbol`, `official.value.symbols-function`, `official.value.symbols-type` |

`O-WRITING3`, `O-MULTICOL1`, `O-FLEXBOX1`, `O-UI3`, and
`O-CONTAIN1` have zero non-property units. The count equation is:

```text
12 rule/qualified-rule + 4 declaration + 17 descriptor + 22 selector
+ 15 media type/feature + 97 shared value/function = 167
```

Conditional 3's imported `general-enclosed` grammar is frozen to
`X-VALUES4` at <https://www.w3.org/TR/2024/WD-css-values-4-20240312/> as the
atomic `ext.supports.general-enclosed` delta. Conditional 3 `@media` binds to
`O-MEDIA3`, not its moving Media Queries reference. Color 4 does not own
`color-mix()` or relative colors; those retain `I-COLOR5`. Fonts 3 does not own
`font-display` or `@font-feature-values`; those retain `I-FONTS4`.
`@import layer` and `@import supports()` are separate `R-CASCADE5` atomic
deltas, never part of the `O-CASCADE4` row.

## 4. Baseline Aggregate Aliases

Four I01 IDs combine slices that acquire different source/tier owners in I02.
They remain queryable immutable baseline aliases with repository provenance and
do not count as parser-facing conformance rows:

| I01 alias ID | Exact atomic targets |
| --- | --- |
| `baseline.selector.pseudo-element` | `official.selector.generated`, `ext.pseudo-element.marker`, `ext.pseudo-element.selection`, `ext.pseudo-element.backdrop`, `ext.pseudo-element.generated-marker` |
| `baseline.media.query-list` | `official.media.query-list-core`, `ext.media.condition-syntax`, `ext.media.malformed-member-never` |
| `baseline.media.range-feature` | official width/height/resolution/color/monochrome rows plus `ext.media.range.width`, `ext.media.range.height`, `ext.media.range.resolution`, `ext.media.range.color`, `ext.media.range.monochrome` |
| `baseline.media.discrete-feature` | `official.media.feature.orientation` plus `ext.media.hover`, `ext.media.any-hover`, `ext.media.pointer`, `ext.media.any-pointer`, `ext.media.prefers-color-scheme`, `ext.media.prefers-reduced-motion`, `ext.media.prefers-reduced-transparency`, `ext.media.prefers-contrast`, `ext.media.forced-colors`, `ext.media.display-mode` |

Add a private `BaselineAlias` conformance disposition and public read-only
`baseline_alias_targets() -> &[CssFeatureId]`. Alias records have no
implementation/vector row of their own after atomic migration; their target
union must equal their I01 positive/boundary behavior. All other I01 IDs retain
their one atomic meaning and migrate directly to the selected official or
extension source.

## 5. Exact Non-Property Exclusions

Every source's examples, explicitly non-normative notes, status/TOC,
changelogs, acknowledgments, indexes, bibliography, test inventories, and
conformance boilerplate form one `InformativeOnly` source-audit row. The exact
normative exclusions are:

| Exclusion ID | Exact source area | Reason and owner |
| --- | --- | --- |
| `excluded.O-CSS2.superseded-syntax` | `syndata.html;grammar.html` | `SupersededWithoutCurrentProduction`; `O-SYNTAX3` |
| `excluded.O-CSS2.superseded-media` | `media.html` | superseded by `O-MEDIA3`,`O-CONDITIONAL3` |
| `excluded.O-CSS2.superseded-selectors` | `selector.html` | superseded by `O-SELECTORS3`,`O-NAMESPACES3` |
| `excluded.O-CSS2.superseded-cascade-values` | `cascade.html;syndata.html` | superseded by `O-CASCADE4`,`O-VALUES3`,`O-VARIABLES1` |
| `excluded.O-CSS2.non-authored-semantics` | `visuren.html;visufx.html;tables.html;page.html#outside-page-box,#page-breaks,#page-cascade` | layout/rendering/cascade/pagination algorithms |
| `excluded.O-SYNTAX3.fragment-api` | `#parse-rule,#parse-declaration,#parse-component-value,#parse-list-of-component-values,#parse-comma-separated-list-of-component-values` | no public generic fragment parser |
| `excluded.O-SYNTAX3.serialization` | `#serialization` | serialization excluded |
| `excluded.O-STYLE-ATTR.interpretation` | `#interpret` | cascade interpretation excluded |
| `excluded.O-MEDIA3.evaluation` | `#media0,#media1` evaluation portions | query/device evaluation excluded |
| `excluded.O-CONDITIONAL3.evaluation-api` | `#processing,#the-cssmediarule-interface,#the-csssupportsrule-interface,#apis` | evaluation/CSSOM excluded |
| `excluded.O-SELECTORS3.matching-specificity` | `#selectors,#specificity,#first-formatted-line,#application-in-css` | matching/specificity/formatting excluded |
| `excluded.O-NAMESPACES3.uri-matching` | semantic portions of `#scope,#css-qnames` | URI resolution/matching excluded |
| `excluded.O-CASCADE4.processing` | `#import-processing,#value-stages,#filtering,#cascading,#initial-values,#inheriting` | loading/cascade/computed processing excluded |
| `excluded.O-VALUES3.metasyntax` | `#value-defs` | specification metasyntax is not authored input |
| `excluded.O-VALUES3.computation` | `#calc-computed-value,#calc-range,#calc-serialize,#relative-urls` | evaluation/range/serialization/URL resolution excluded |
| `excluded.O-VARIABLES1.substitution` | `#cycles,#invalid-variables,#variables-in-shorthands,#apis` | substitution/dependency/CSSOM excluded |
| `excluded.O-BOX3.layout` | `#box-model,#fragmentation` | box/layout semantics excluded |
| `excluded.O-COLOR4.color5-syntax` | Color 5 references | superseded by selected `I-COLOR5` extension rows |
| `excluded.O-COLOR4.quirky-color` | `#quirky-color` | HTML presentational quirk, not CSS declaration grammar |
| `excluded.O-COLOR4.processing` | conversion/interpolation/gamut/resolution/serialization/sample-code sections | downstream processing excluded |
| `excluded.O-BACKGROUNDS3.painting` | serialization/painting/corner/border-image/shadow algorithms | downstream rendering excluded |
| `excluded.O-IMAGES3.processing` | object negotiation/sizing/interpolation/serialization algorithms | loading/rendering excluded |
| `excluded.O-FONTS3.processing` | loading/fetching/matching/feature-resolution/object-model sections | loading/rendering/CSSOM excluded |
| `excluded.O-FONTS3.font-display` | no Fonts 3 production | superseded by selected `I-FONTS4` extension |
| `excluded.O-FONTS3.font-feature-values` | no Fonts 3 production | superseded by selected `I-FONTS4` unsupported row |
| `excluded.O-WRITING3.layout` | bidi/inline/abstract/principal-flow/text-combine algorithms | layout excluded |
| `excluded.O-MULTICOL1.layout` | model/pseudo-algorithm/stacking/overflow sections | layout excluded |
| `excluded.O-FLEXBOX1.layout` | box/items/lines/layout/pagination/axis algorithms | layout excluded |
| `excluded.O-FLEXBOX1.webkit-legacy` | `#webkit-aliases` | prefixed aliases have no official canonical production |
| `excluded.O-UI3.behavior` | ellipsis/input/default-style behavior sections | UI behavior excluded |
| `excluded.O-CONTAIN1.semantics` | containment-type/optimization sections | layout/paint effects excluded |
| `excluded.O-TRANSFORMS1.processing` | rendering/SVG/animation/interpolation/matrix algorithms | downstream processing excluded |
| `excluded.O-COMPOSITING1.processing` | Canvas/formula/backdrop/group/advanced-compositing sections | compositing evaluation excluded |
| `excluded.O-EASING1.evaluation` | easing output/serialization sections | evaluation/serialization excluded |
| `excluded.O-COUNTERSTYLES3.processing` | counter algorithms/predefined rendering/APIs/sample sheet | rendering/speech/CSSOM excluded |

All rows without an explicit reason above are `OutsideAuthoredSyntaxBoundary`.
Together with sections 2.3 and 2.4, these exclusions are the complete selected
official source-audit remainder.

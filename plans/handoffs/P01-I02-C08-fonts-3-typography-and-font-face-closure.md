# P01-I02-C08 Fonts 3 Typography And Font-Face Closure

## Boundary

`surgeist-css` owns strict authored Fonts syntax, typed property and descriptor
values, source-order font-face occurrences, and recovery diagnostics. It does
not load or match fonts, resolve fallback or OpenType feature application, shape
glyphs, apply cascade or substitution, evaluate computed values, expose CSSOM,
serialize, render, or lower into another Surgeist crate.

## Current Surface And I01 Compatibility

The current property surface implements the sixteen Fonts 3 properties:
`font`, `font-family`, `font-feature-settings`, `font-kerning`, `font-size`,
`font-size-adjust`, `font-stretch`, `font-style`, `font-synthesis`,
`font-variant`, `font-variant-caps`, `font-variant-east-asian`,
`font-variant-ligatures`, `font-variant-numeric`, `font-variant-position`, and
`font-weight`. Family values preserve quoted names, identifier sequences, and
generic keywords. OpenType tags are exactly four decoded ASCII characters, and
feature indices are non-negative.

Property wrappers expose parser-owned current values through their semantic
accessors. Their existing `i01_subset()` signatures remain the compatibility
view: every frozen I01 value retains its projection, while current-only syntax
returns `None` when the older payload cannot represent it. Root consumers must
not treat an absent I01 projection as invalid current syntax.

## Font-Face And Recovery

`CssFontFaceDescriptors::occurrences()` exposes every valid descriptor
occurrence in authored order. Effective typed accessors return the last valid
occurrence. Fonts 3 family, source, style, weight, stretch, unicode-range, and
feature-settings descriptors retain typed values. Source lists preserve
`local()` and URL order plus arbitrary nonempty Fonts 3 string format hints.

An invalid, unknown, or important descriptor occurrence is dropped with
`DropDescriptor` and does not erase valid neighbors. A font-face rule is retained
only when at least one valid effective `font-family` and `src` remain. When that
required set is absent, child diagnostics precede the parent `DropAtRule`
diagnostic.

## Source And Metadata Split

The sixteen property rows and these ten non-property rows cite the dated
`O-FONTS3` source and report `Complete`:

- `baseline.rule.font-face`;
- `baseline.descriptor.font-family`, `baseline.descriptor.src`,
  `baseline.descriptor.font-style`, `baseline.descriptor.font-weight`,
  `baseline.descriptor.font-stretch`, and
  `baseline.descriptor.unicode-range`;
- `official.descriptor.font-feature-settings`, `official.value.font-source`,
  and `official.value.opentype-tag`.

Selected additions cite the dated `I-FONTS4` source separately:

| Stable ID | Production | Product disposition |
| --- | --- | --- |
| `ext.property.font-weight-range` | `#font-weight-prop` | `Partial`: integer values from 1 through 1000; other unselected Fonts 4 property grammar remains unsupported |
| `ext.descriptor.font-weight-range` | `#font-weight-desc` | `Partial`: numbers from 1 through 1000 and increasing two-value ranges; other unselected descriptor grammar remains unsupported |
| `ext.descriptor.font-style-oblique-range` | `#font-style-desc` | `Partial`: one or two increasing `-90deg` through `90deg` oblique angles; other unselected descriptor grammar remains unsupported |
| `ext.descriptor.font-stretch-range` | `#font-stretch-desc` | `Partial`: non-negative percentage values and increasing two-value ranges; other unselected descriptor grammar remains unsupported |
| `ext.value.font-source-modern-hints` | `#font-face-src-parsing` | `Partial`: `woff`, `woff2`, `truetype`, `opentype`, `collection`, `embedded-opentype`, and `svg` `format()` hints plus `variations`, `color-colrv0`, `color-colrv1`, `color-svg`, `color-sbix`, `color-cbdt`, `features-opentype`, `features-aat`, `features-graphite`, and `incremental` `tech()` hints; other unselected hints remain unsupported |

`baseline.descriptor.font-display` is `Complete` under `I-FONTS4`.
`later.rule.font-feature-values` remains `RecognizedUnsupported` with
`UnsupportedAtRule`. The Fonts 3 processing, font-display, and
font-feature-values exclusions remain unchanged source-audit facts.

Parser ownership is split by behavior: property dispatch remains in the
property schema with typography parsing; shared OpenType-tag and numeric-weight
extension parsing belongs to `crate::parser::typography`; font-face rule,
descriptor, source-list, range, and modern-hint parsing belongs to
`crate::parser::font_face`.

## Source-Backed Product Fixture Delta

The product fixture artifact before the correction has SHA-256
`99bbb897710969949d7b596d14fbd352d5d3121a6c4cf663b8ca100154057f8b`.
The replacement product fixture artifact has SHA-256
`67e69813d808ffda40e7c159fde719fbadd0447f8e4105788b0bb593931fac89`.

Exactly one product row changes. Its identity, mode, and authored source remain:

```text
focused.structured-errors.12 | sheet | both | @font-face { font-family: One; font-family: Two; src: url(test.woff2); }
```

Its observable delta is:

```text
clean: false -> true
retained: rule:baseline.rule.font-face -> rule:baseline.rule.font-face
position: - -> -
span: - -> -
diagnostic: InvalidDescriptorCombination/InvalidDescriptorCombination:font-face:font-family:font-family/DropDescriptor@31:0:31>31:0:31-48:0:48:48 -> -
```

## Root-Owned Follow-Up

Root `surgeist` owns the integration work:

1. select and promote the intended `surgeist-css` gitlink;
2. expose the current font property, font-face occurrence, descriptor, source,
   OpenType, and conformance metadata surface through the facade;
3. update adapters to consume current values and treat `i01_subset()` only as
   compatibility data;
4. preserve authored ordering and symbolic values until the owning font,
   shaping, cascade, or computed-value layer;
5. refresh root-owned generated API audit artifacts with the root generator;
6. update root integration documentation and tests for the Fonts 3 profile,
   Fonts 4 source split, recovery boundaries, and downstream exclusions.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and generated API artifacts. This leaf handoff does not authorize
changes in root or sibling repositories.

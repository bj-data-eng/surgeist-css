# P01-I02-C04 Property-Specific Position Grammars

## Boundary

`surgeist-css` owns strict authored generic positions, background-position
layers, mask-position layers, object positions, transform origins, and the six
completed conformance records named below. Values remain authored and symbolic.
This crate does not apply cascade, substitute variables, resolve percentages or
writing modes, choose positioning boxes, compute object sizes, perform layout or
painting, apply transforms, load resources, or lower values into sibling
Surgeist crates.

## Public Position Surface

`CssPositionOffset` accepts the position-valid authored length-percentage
domain. The non-exhaustive `CssHorizontalPosition` and
`CssVerticalPosition` enums distinguish centered axes, free offsets, named
edges, and offsets authored from named edges. Parser-owned
`CssPositionValue` exposes the exact horizontal and vertical axes while keeping
invalid cross-axis combinations unconstructable.

The four property accessors preserve their grammar boundaries:

- `CssBackgroundPositionPropertyValue::positions()` returns a nonempty
  `CssBackgroundPositionList`. Each layer exposes exact horizontal and vertical
  axes and admits the background-only three-component form.
- `CssMaskPositionPropertyValue::positions()` returns a nonempty
  `CssMaskPositionList`. Each `CssMaskPosition::value()` is one generic
  `CssPositionValue`, so a mask layer does not admit the background-only
  three-component form.
- `CssObjectPositionPropertyValue::position()` returns one
  `CssObjectPosition`; `CssObjectPosition::value()` exposes its generic
  `CssPositionValue`.
- `CssTransformOriginPropertyValue::origin()` returns one
  `CssTransformOrigin` with explicit horizontal and vertical axes plus an
  optional `CssTransformOriginZ`. The z value accepts only an authored length
  without a percentage component.

Generic `<position>` accepts its exact one-, two-, and four-component forms.
Background layers additionally accept the specified three-component form. The
transform-origin parser keeps `left 50px` and `center 50px` as complete 2D
positions, while `top 50px`, `bottom 50px`, and `left top 50px` direct the final
length to the z axis.

## Completed Conformance Metadata

| Stable ID | Source | Production | Owner |
| --- | --- | --- | --- |
| `official.value.position` | `O-VALUES3` | `#position` | `crate::parser::background` shared-position parser |
| `official.value.background-position` | `O-BACKGROUNDS3` | `#background-position` | `crate::parser::background` background-layer parser |
| `baseline.property.background-position` | `O-BACKGROUNDS3` | `#propdef-background-position` | `CssKnownProperty::BackgroundPosition` property schema row |
| `official.property.object-position` | `O-IMAGES3` | `#propdef-object-position` | `CssKnownProperty::ObjectPosition` property schema row |
| `baseline.property.transform-origin` | `O-TRANSFORMS1` | `#propdef-transform-origin` | `CssKnownProperty::TransformOrigin` property schema row |
| `baseline.property.mask-position` | `S-MASKING1` | `#propdef-mask-position` | `CssKnownProperty::MaskPosition` property schema row |

Every row is `Complete`, atomic, and paired with a named public behavior case.
No other background, image, transform, or mask catalog row changes disposition.

## Compatibility And Exclusions

Background-position, mask-position, and transform-origin retain their exact
`i01_subset()` signatures. Every frozen I01 input keeps its authored and debug
projection. A newly accepted typed calculation or exact current form returns
`None` when the older payload cannot represent it without loss.
`object-position` is additive and has no I01 projection requirement.

Position syntax inside gradients, transform functions, filters, and basic
shapes remains on its existing function-specific grammar path. This record does
not complete `background`, background images or sizes, borders, gradients,
`object-fit`, image rendering or orientation, transform functions, mask
shorthand semantics beyond its validated position component, or any downstream
resolution behavior.

## Root-Owned Follow-Up

Root `surgeist` must:

1. deliberately promote the selected `surgeist-css` candidate gitlink;
2. expose the current position types, property wrappers, borrowed property-value
   branch for `object-position`, and conformance records through the facade;
3. update root-owned adapters to consume the property-specific accessors and
   preserve symbolic offsets until the owning contextual-resolution layer;
4. treat `i01_subset()` only as compatibility data and handle `None` without
   treating current syntax as invalid;
5. refresh root-owned generated API audit artifacts with the root generator;
6. update root integration documentation and tests for generic versus
   background layers, mask layers, object positions, transform-origin z
   behavior, compatibility projections, and downstream exclusions.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and all generated API artifacts. This leaf record does not
authorize mutations in root or sibling repositories.

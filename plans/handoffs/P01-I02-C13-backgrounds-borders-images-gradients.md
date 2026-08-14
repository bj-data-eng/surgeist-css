# P01-I02-C13 Backgrounds, Borders, Images, And Gradients

## Boundary

`surgeist-css` owns strict authored Backgrounds 3, Borders 3, and Images 3
grammar, typed authored values, declaration recovery, source coordinates,
official metadata, and public parser evidence. It does not apply cascade or
substitution; resolve URLs, resources, percentages, layout, or geometry; load or
decode images; paint; serialize CSSOM; or lower syntax into another Surgeist
crate.

## Public Authored Surface

Background shorthands preserve ordered layers, comma separators, image,
position and immediately coupled slash-size, repeat, attachment, origin/clip
boxes, and final-layer-only color. Longhands preserve their authored lists and
share the same typed value families.

Image values distinguish `none`, URLs, linear/radial gradients, and their
repeating forms. Gradient values preserve directions, radial shapes and sizes,
positions, stops, optional double positions, and interleaved hints. Border-image
shorthand and longhands preserve source, one-to-four slice/width/outset
components, `fill`, and one- or two-axis repeat choices. `image-orientation`,
`image-rendering`, `object-fit`, and the already-published `object-position`
retain authored typed choices without resolving or rendering them.

Ordinary property values, whole-property globals, and substitution-dependent
values remain distinct declaration branches. Invalid values identify the
responsible property and declaration span, recover with `DropDeclaration`, and
leave later siblings eligible.

## Complete Official Metadata

Every row below is a public `Complete` atomic record with no supported subset,
unsupported remainder, recognized-unsupported code, or aggregate-alias target.

| Stable ID | Source | Exact fragment |
| --- | --- | --- |
| `official.property.border-image` | `O-BACKGROUNDS3` | `#propdef-border-image` |
| `official.property.border-image-outset` | `O-BACKGROUNDS3` | `#propdef-border-image-outset` |
| `official.property.border-image-repeat` | `O-BACKGROUNDS3` | `#propdef-border-image-repeat` |
| `official.property.border-image-slice` | `O-BACKGROUNDS3` | `#propdef-border-image-slice` |
| `official.property.border-image-source` | `O-BACKGROUNDS3` | `#propdef-border-image-source` |
| `official.property.border-image-width` | `O-BACKGROUNDS3` | `#propdef-border-image-width` |
| `official.property.image-orientation` | `O-IMAGES3` | `#propdef-image-orientation` |
| `official.property.image-rendering` | `O-IMAGES3` | `#propdef-image-rendering` |
| `official.property.object-fit` | `O-IMAGES3` | `#propdef-object-fit` |
| `official.value.background-layer` | `O-BACKGROUNDS3` | `#layering` |
| `official.value.background-image` | `O-BACKGROUNDS3` | `#background-image` |
| `official.value.repeat-style` | `O-BACKGROUNDS3` | `#background-repeat` |
| `official.value.background-attachment` | `O-BACKGROUNDS3` | `#background-attachment` |
| `official.value.background-size` | `O-BACKGROUNDS3` | `#background-size` |
| `official.value.line-style` | `O-BACKGROUNDS3` | `#border-style` |
| `official.value.line-width` | `O-BACKGROUNDS3` | `#border-width` |
| `official.value.image` | `O-IMAGES3` | `#image-values` |
| `official.value.gradient` | `O-IMAGES3` | `#gradients` |
| `official.value.linear-gradient` | `O-IMAGES3` | `#linear-gradients` |
| `official.value.radial-gradient` | `O-IMAGES3` | `#radial-gradients` |
| `official.value.repeating-linear-gradient` | `O-IMAGES3` | `#repeating-gradients` |
| `official.value.repeating-radial-gradient` | `O-IMAGES3` | `#repeating-gradients` |
| `official.value.color-stop-list` | `O-IMAGES3` | `#color-stop-syntax` |
| `official.value.side-or-corner` | `O-IMAGES3` | `#linear-gradients` |
| `official.value.radial-shape` | `O-IMAGES3` | `#radial-gradients` |
| `official.value.radial-size` | `O-IMAGES3` | `#radial-gradients` |
| `official.value.radial-extent` | `O-IMAGES3` | `#radial-gradients` |

The Backgrounds/Borders property rows promoted from Partial to Complete are
`background`, `background-color`, `background-image`, `background-size`,
`background-repeat`, `background-origin`, `background-clip`,
`background-attachment`, `border`, the four physical border shorthands, the
five width rows, five color rows, five style rows, `border-radius`, and the four
physical corner-radius rows. Existing `background-position`, `object-position`,
and `box-shadow` records remain Complete.

## Inventory And Vector Reconciliation

C13 activates exactly 27 records: nine properties and eighteen non-property
shared values. The public feature catalog grows from 429 to 456 records. Status
promotion of 33 existing property records does not change catalog cardinality.

The immutable official inventory remains exactly 162 property units: 161
canonical properties plus the custom-property declaration family. The one
normative legacy shorthand and 167 non-property units are unchanged. C13 does
not add or remove an official ledger unit or exclusion row.

`tests/catalog_inventory/vectors.rs` supplies independent ordinary positive and
property-specific negative vectors for all nine newly activated properties.
`tests/conformance_catalog.rs` names every C13 ID, spelling, kind, source,
fragment, status, and public parser stimulus, and directly proves all 33
promotions plus preservation of the three already-Complete rows. Family tests
cover layer and list separators, arity, stops and hints, globals and
substitutions, repeated recovery, EOF, non-BMP coordinates, 255/256/257
boundaries, sibling retention, and ordinary/application-strict parity.

The frozen product fixture is unchanged. Its SHA-256 digest before and after C13
is `7c2cf7d79368d76d94cc0b383be70cc404d4c69d7caa72eedba6f0762e0b2356`.
No fixture row is replaced, weakened, or masked by this cycle.

## Root-Owned Follow-Up

Root `surgeist` owns the integration work:

1. select and promote the intended `surgeist-css` gitlink;
2. expose the additive C13 typed property/value and metadata surface through the
   facade;
3. preserve authored symbolic values and keep cascade, substitution, resource
   loading, decoding, geometry, layout, painting, CSSOM, and cross-crate
   lowering outside this leaf;
4. refresh root-owned generated API audit artifacts with the root generator;
5. update root integration tests and documentation for typed inspection,
   recovery distinctions, exact official metadata, and inventory totals.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and generated API artifacts. This leaf handoff does not authorize
changes in root or sibling repositories.

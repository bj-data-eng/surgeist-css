# P01-I02-C06 Color 4 And Preserved Color 5 Authored Grammar

## Boundary

`surgeist-css` owns strict parsing and typed inspection of authored Color 4
values plus the explicitly preserved Color 5 relative-color and `color-mix()`
subsets. It preserves authored components, calculations, color-space identity,
channel environments, separators, ordering, and recovery diagnostics. It does
not apply cascade or substitution; clamp computed values; resolve
`currentcolor` or system colors; evaluate calculations or relative channels;
convert, gamut-map, contrast-adjust, mix, serialize, or render colors; load
color profiles; or lower values into another Surgeist crate.

## Public Authored Surface

Color-bearing property wrappers expose a checked `CssAuthoredColor` through
`current()`, while the opacity wrapper exposes `CssOpacityValue` through
`value()`. The current model distinguishes current, transparent,
hexadecimal, named, current and deprecated system, RGB, HSL, HWB, Lab, LCH,
Oklab, Oklch, predefined `color()`, relative-color, and `color-mix()` branches.
Component slots retain their exact authored number, percentage, angle, `none`,
or typed-calculation domain. `CssOpacityValue` preserves finite numbers and
percentages, including signed and out-of-range specified values and their typed
calculations.

Every existing wrapper keeps `i01_subset()` with its frozen signature. Every
frozen I01 input retains its exact compatibility payload. Current syntax returns
`None` when the older `CssColor` or `CssOpacity` model cannot represent it
without loss; root consumers must not treat that absence as a parse failure.

## Color 4 Metadata

The following `O-COLOR4` value records are `Complete` and owned by
`crate::parser::values`: `official.value.color`, `official.value.alpha`,
`official.value.hue`, `official.value.rgb`, `official.value.hex-color`,
`official.value.named-color`, `official.value.system-color`,
`official.value.deprecated-system-color`, `official.value.transparent`,
`official.value.currentcolor`, `official.value.hsl`, `official.value.hwb`,
`official.value.lab`, `official.value.lch`, `official.value.oklab`,
`official.value.oklch`, and `official.value.predefined-color`.

`baseline.property.color` and `baseline.property.opacity` are `Complete` for
their exact Color 4 property grammars. Other color-valued property records keep
their existing dispositions. Color 4 exclusions for Color 5 syntax, quirky
color, and downstream processing remain separate source-audit facts.

## Preserved Color 5 Metadata

All records below use the dated `I-COLOR5` source, have kind `Value`, and are
owned by `crate::parser::values`.

| Stable ID | Production | Status and boundary |
| --- | --- | --- |
| `ext.value.relative-color` | `#relative-colors,#relative-syntax` | `Partial`: the eight selected families are supported; `alpha()`, custom-profile parameters, and other unselected Color 5 functions remain unsupported |
| `ext.value.relative-color.rgb` | `#relative-RGB` | `Complete` for the selected RGB/RGBA environment |
| `ext.value.relative-color.hsl` | `#relative-HSL` | `Complete` for the selected HSL/HSLA environment |
| `ext.value.relative-color.hwb` | `#relative-HWB` | `Complete` for the selected HWB environment |
| `ext.value.relative-color.lab` | `#relative-Lab` | `Complete` for the selected Lab environment |
| `ext.value.relative-color.oklab` | `#relative-Oklab` | `Complete` for the selected Oklab environment |
| `ext.value.relative-color.lch` | `#relative-LCH` | `Complete` for the selected LCH environment |
| `ext.value.relative-color.oklch` | `#relative-OkLCh` | `Complete` for the selected Oklch environment |
| `ext.value.relative-color.predefined` | `#relative-color-function` | `Complete` for predefined RGB/XYZ relative `color()` spaces; custom profiles remain excluded by the aggregate boundary |
| `ext.value.color-mix` | `#funcdef-color-mix` | `Partial`: required interpolation method, exactly two colors, optional trailing percentages, and predefined or polar spaces; other dated Color 5 forms remain unsupported |

Relative colors retain exactly three result channels plus optional alpha and use
closed per-family identifier environments. The preserved `color-mix()` subset
permits hue interpolation methods only in polar spaces. `alpha()`, custom color
profiles, `light-dark()`, `device-cmyk()`, and other unselected Color 5
functions remain outside this boundary.

## Root-Owned Follow-Up

Root `surgeist` owns the integration work:

1. promote the selected `surgeist-css` gitlink;
2. expose the authored current color and opacity surface through the facade;
3. update root-owned adapters to consume `current()` while retaining
   `i01_subset()` only as compatibility data;
4. keep authored calculations, relative channels, current/system colors, and
   color mixes symbolic until their owning contextual layer;
5. refresh root-owned generated API audit artifacts with the root generator;
6. update root integration tests and documentation for Color 4 breadth,
   preserved Color 5 boundaries, compatibility projection, and excluded
   evaluation behavior.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and generated API artifacts. This leaf handoff does not authorize
changes in root or sibling repositories.

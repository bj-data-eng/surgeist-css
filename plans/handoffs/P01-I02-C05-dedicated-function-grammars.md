# P01-I02-C05 Dedicated Function Grammars

## Boundary

`surgeist-css` owns strict authored transform, easing, box-shadow, filter, and
selected basic-shape syntax. The current models preserve typed components,
authored order, exact separators, and exact dimension domains. They do not
perform transform multiplication, easing evaluation, interpolation, filter or
shadow rendering, URL resolution, shape geometry, layout, painting, cascade,
substitution, or cross-crate lowering.

## Public Function Surface

The current property accessors are separate from the frozen I01 compatibility
view:

- `CssTransformPropertyValue::current()` exposes `CssTransformValue` and its
  nonempty `CssTransformFunctionValueList`.
- transition and animation timing-function wrappers expose nonempty current
  `CssEasingValueList` values; the transition and animation shorthands carry
  the same typed easing values in their current items.
- `CssFilterPropertyValue::current()` and
  `CssBackdropFilterPropertyValue::current()` expose `CssFilterValue` and its
  nonempty ordered `CssFilterFunctionValueList`.
- `CssBoxShadowPropertyValue::current()` exposes `CssBoxShadow`; filter
  `drop-shadow()` instead exposes `CssDropShadow`, which cannot contain `inset`
  or spread.
- `CssClipPathPropertyValue::current()` exposes an optional `CssClipPathValue`
  with typed `inset`, `circle`, `ellipse`, and `polygon` branches. Polygon
  exposes its optional non-negative `round <length>` component.

Every wrapper retains `i01_subset()` with its existing signature. A value
accepted by the current grammar may return `None` when the frozen I01 payload
cannot represent it. Root consumers must use the current accessor for current
syntax and treat the I01 payload only as compatibility data.

## Official Atomic Metadata

Every row below is `Complete`, has atomic disposition, and is owned by the named
shared-value parser inventory.

| Stable ID | Source | Production | Owner |
| --- | --- | --- | --- |
| `official.value.transform-list` | `O-TRANSFORMS1` | `#transform-function-lists` | `crate::parser::effects` |
| `official.value.transform-function` | `O-TRANSFORMS1` | `#transform-functions` | `crate::parser::effects` |
| `official.value.transform.matrix` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.translate` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.translate-x` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.translate-y` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.scale` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.scale-x` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.scale-y` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.rotate` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.skew` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.skew-x` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.transform.skew-y` | `O-TRANSFORMS1` | `#two-d-transform-functions` | `crate::parser::effects` |
| `official.value.easing-function` | `O-EASING1` | `#easing-functions` | `crate::parser::timing` |
| `official.value.cubic-bezier-easing` | `O-EASING1` | `#cubic-bezier-easing-functions` | `crate::parser::timing` |
| `official.value.step-easing` | `O-EASING1` | `#step-easing-functions` | `crate::parser::timing` |
| `official.value.step-position` | `O-EASING1` | `#step-easing-functions` | `crate::parser::timing` |
| `official.value.shadow` | `O-BACKGROUNDS3` | `#box-shadow` | `crate::parser::effects` |

## Preserved-Extension Atomic Metadata

All rows below are atomic. The aggregate basic-shape row is `Partial`; every
other row is `Complete`. The named public behavior is the independently
observable parser case paired with the metadata record.

| Stable ID | Source | Production | Owner | Named public behavior |
| --- | --- | --- | --- | --- |
| `ext.value.transform.matrix3d` | `I-TRANSFORMS2` | `#funcdef-matrix3d` | `crate::parser::effects` | `transform_matrix3d_exposes_sixteen_finite_components` |
| `ext.value.transform.perspective` | `I-TRANSFORMS2` | `#funcdef-perspective` | `crate::parser::effects` | `transform_perspective_accepts_none_and_zero_and_rejects_invalid_dimensions` |
| `ext.value.transform.rotate3d` | `I-TRANSFORMS2` | `#funcdef-rotate3d` | `crate::parser::effects` | `transform_three_dimensional_rotations_are_typed` |
| `ext.value.transform.rotate-x` | `I-TRANSFORMS2` | `#funcdef-rotatex` | `crate::parser::effects` | `transform_three_dimensional_rotations_are_typed` |
| `ext.value.transform.rotate-y` | `I-TRANSFORMS2` | `#funcdef-rotatey` | `crate::parser::effects` | `transform_three_dimensional_rotations_are_typed` |
| `ext.value.transform.rotate-z` | `I-TRANSFORMS2` | `#funcdef-rotatez` | `crate::parser::effects` | `transform_three_dimensional_rotations_are_typed` |
| `ext.value.transform.scale3d` | `I-TRANSFORMS2` | `#funcdef-scale3d` | `crate::parser::effects` | `transform_three_dimensional_scales_preserve_number_and_percentage_operands` |
| `ext.value.transform.scale-z` | `I-TRANSFORMS2` | `#funcdef-scalez` | `crate::parser::effects` | `transform_three_dimensional_scales_preserve_number_and_percentage_operands` |
| `ext.value.transform.translate3d` | `I-TRANSFORMS2` | `#funcdef-translate3d` | `crate::parser::effects` | `transform_three_dimensional_translations_keep_z_length_only` |
| `ext.value.transform.translate-z` | `I-TRANSFORMS2` | `#funcdef-translatez` | `crate::parser::effects` | `transform_three_dimensional_translations_keep_z_length_only` |
| `ext.value.filter-function-list` | `I-FILTER1` | `#FilterProperty` | `crate::parser::effects` | `filter_function_list_preserves_typed_authored_order` |
| `ext.value.filter.blur` | `I-FILTER1` | `#funcdef-filter-blur` | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.brightness` | `I-FILTER1` | `#funcdef-filter-brightness` | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.contrast` | `I-FILTER1` | `#funcdef-filter-contrast` | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.grayscale` | `I-FILTER1` | `#funcdef-filter-grayscale` | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.hue-rotate` | `I-FILTER1` | `#funcdef-filter-hue-rotate` | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.invert` | `I-FILTER1` | `#funcdef-filter-invert` | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.opacity` | `I-FILTER1` | `#funcdef-filter-opacity` | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.saturate` | `I-FILTER1` | `#funcdef-filter-saturate` | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.sepia` | `I-FILTER1` | `#funcdef-filter-sepia` | `crate::parser::effects` | `every_filter_amount_function_has_exact_typed_domain` |
| `ext.value.filter.drop-shadow` | `I-FILTER1` | `#funcdef-filter-drop-shadow` | `crate::parser::effects` | `drop_shadow_rejects_box_shadow_only_components` |
| `ext.value.basic-shape` | `S-SHAPES1` | `#typedef-basic-shape` | `crate::parser::effects` | `clip_path_distinguishes_selected_and_deferred_shape_functions` |
| `ext.value.basic-shape.inset` | `S-SHAPES1` | `#funcdef-basic-shape-inset` | `crate::parser::effects` | `every_selected_basic_shape_has_typed_public_components` |
| `ext.value.basic-shape.circle` | `S-SHAPES1` | `#funcdef-basic-shape-circle` | `crate::parser::effects` | `every_selected_basic_shape_has_typed_public_components` |
| `ext.value.basic-shape.ellipse` | `S-SHAPES1` | `#funcdef-basic-shape-ellipse` | `crate::parser::effects` | `every_selected_basic_shape_has_typed_public_components` |
| `ext.value.basic-shape.polygon` | `S-SHAPES1` | `#funcdef-basic-shape-polygon` | `crate::parser::effects` | `every_selected_basic_shape_has_typed_public_components` |

The `ext.value.basic-shape` supported subset is typed `inset`, `circle`,
`ellipse`, and `polygon`. Its valid-but-unsupported remainder is `path`,
`shape`, `rect`, and `xywh`. The polygon atomic row includes
`round <length>`.

## Property Metadata

The following property rows are `Complete`: `baseline.property.transform`,
`baseline.property.box-shadow`, `baseline.property.filter`,
`baseline.property.transition-timing-function`, and
`baseline.property.animation-timing-function`.

The following rows retain atomic `Partial` disposition and exact independent
boundaries:

| Stable ID | Source | Production | Supported subset | Valid-but-unsupported remainder |
| --- | --- | --- | --- | --- |
| `baseline.property.transition` | `I-TRANSITIONS1` | `#propdef-transition` | I01 shorthand components plus typed durations, signed delays, iteration/calculation syntax, and typed easing functions | Other valid forms of the cited shorthand production |
| `baseline.property.animation` | `I-ANIMATIONS1` | `#propdef-animation` | I01 shorthand components plus typed durations, signed delays, iteration/calculation syntax, and typed easing functions | Other valid forms of the cited shorthand production |
| `baseline.property.backdrop-filter` | `X-FILTER2-BASE` | `#propdef-backdrop-filter` | Exact preserved I01 filter-function-list subset with typed current values | Every Filter Effects 2 behavior absent from that preserved subset |
| `baseline.property.clip-path` | `S-MASKING1` | `#propdef-clip-path` | `none`, URL, and typed `inset`, `circle`, `ellipse`, and `polygon` | Reference-box combinations and `path`, `shape`, `rect`, and `xywh` |

No aggregate or unselected function/property behavior gains support by
association with these rows.

## Root-Owned Follow-Up

Root `surgeist` must:

1. select the intended `surgeist-css` candidate gitlink;
2. expose the current function models and property accessors through the facade;
3. update root-owned adapters to consume current typed values and keep
   `i01_subset()` as compatibility data only;
4. preserve symbolic values until the owning contextual or rendering layer;
5. refresh root-owned generated API audit artifacts with the root generator;
6. update root integration documentation and tests for typed transform, easing,
   shadow/filter, and selected-shape access plus every retained Partial boundary.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and generated API artifacts. This leaf handoff does not authorize
mutations in root or sibling repositories.

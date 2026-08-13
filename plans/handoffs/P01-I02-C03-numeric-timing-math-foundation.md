# P01-I02-C03 Numeric, Timing, And Math Foundation

## Boundary

`surgeist-css` now owns finite authored numeric values, distinct duration and
delay domains, typed authored calculation trees, their selected property
consumers, and the catalog records listed below. Values remain symbolic at this
boundary. This crate does not apply cascade, substitute variables, resolve
relative units, evaluate computed ranges, run animation timelines, perform
layout, or lower CSS values into sibling Surgeist types.

## Public Additions

The finite numeric boundary adds `CssFiniteNumber`, checked scalar construction,
`CssGridFlowToleranceValue`, and
`CssGridFlowTolerancePropertyValue::value()`. The frozen
`CssGridFlowTolerance::Percent(f32)` compatibility payload is unchanged.

Typed calculations add:

- `CssNumberCalculation`, `CssIntegerCalculation`,
  `CssPercentageCalculation`, `CssLengthCalculation`, `CssAngleCalculation`,
  `CssTimeCalculation`, and `CssFrequencyCalculation`;
- `CssCalculationType`, `CssCalculationExpressionRef`,
  `CssCalculationValueRef`, `CssCalculationSumRef`,
  `CssCalculationSumTermRef`, `CssCalculationProductRef`,
  `CssCalculationProductFactorRef`, `CssCalculationUnaryRef`,
  `CssCalculationSumOperator`, and `CssCalculationProductOperator`;
- `CssAngleUnit`, `CssAngleLiteral`, `CssFrequencyUnit`, and
  `CssFrequencyLiteral`;
- the additive typed branch on `CssCalcLength` and the current scalar property
  models `CssPositiveNumber`, `CssOpacityValue`, `CssNonNegativeNumberValue`,
  `CssPositiveNumberValue`, `CssAspectRatioValue`, `CssIntegerValue`,
  `CssZIndexValue`, `CssFlexValue`, and `CssFlexComponents`.

The scalar current-value accessors are `CssOpacityPropertyValue::value()`,
`CssFlexGrowPropertyValue::factor()`,
`CssFlexShrinkPropertyValue::factor()`, `CssOrderPropertyValue::value()`,
`CssZIndexPropertyValue::value()`, `CssAspectRatioPropertyValue::ratio()`, and
`CssFlexPropertyValue::value()`.

Timing adds `CssDurationLiteral`, `CssDelayLiteral`, `CssDuration`, `CssDelay`,
`CssDurationList`, `CssDelayList`, `CssAnimationIterationValue`,
`CssAnimationIterationValueList`, `CssTransitionValue`,
`CssTransitionValueList`, `CssAnimationValue`, and `CssAnimationValueList`.
`CssTransitionValue` exposes `property()`, `duration()`, `delay()`, and
`timing_function()`; `CssAnimationValue` exposes `name()`, `duration()`,
`delay()`, `timing_function()`, `iteration_count()`, `direction()`,
`fill_mode()`, and `play_state()`.

The exact seven timing wrapper accessors are
`CssTransitionDurationPropertyValue::durations()`,
`CssTransitionDelayPropertyValue::delays()`,
`CssAnimationDurationPropertyValue::durations()`,
`CssAnimationDelayPropertyValue::delays()`,
`CssAnimationIterationCountPropertyValue::iteration_counts()`,
`CssTransitionPropertyValue::transitions()`, and
`CssAnimationPropertyValue::animations()`.

## Promoted Shared-Value Metadata

Every row below cites `O-VALUES3` and is owned by the shared-value inventory in
`crate::parser::values`.

| Stable ID | Status | Production | Supported subset and remainder |
| --- | --- | --- | --- |
| `official.value.integer` | `Complete` | `#integers` | Entire named production |
| `official.value.number` | `Complete` | `#numbers` | Entire named production |
| `official.value.percentage` | `Complete` | `#percentages` | Entire named production |
| `official.value.length` | `Complete` | `#lengths` | Entire named production |
| `official.value.length-percentage` | `Complete` | `#mixed-percentages` | Entire named production |
| `official.value.time` | `Complete` | `#time` | Entire named production |
| `official.value.resolution` | `Complete` | `#resolution` | Entire named production |
| `official.value.dimension` | `Partial` | `#dimensions` | Selected typed length, angle, time, frequency, and resolution dimensions; other valid CSS dimension families remain for their owning later grammar cycles |
| `official.value.angle` | `Partial` | `#angles` | Public typed angle model and calculation root; angle property consumers remain for their owning later grammar cycles |
| `official.value.angle-percentage` | `Partial` | `#mixed-percentages` | Public typed angle and percentage calculation models; angle-percentage property consumers remain for their owning later grammar cycles |
| `official.value.time-percentage` | `Partial` | `#mixed-percentages` | Public typed time and percentage calculation models; time-percentage property consumers remain for their owning later grammar cycles |
| `official.value.frequency` | `Partial` | `#frequency` | Public typed frequency model and calculation root; frequency property consumers remain for their owning later grammar cycles |
| `official.value.frequency-percentage` | `Partial` | `#mixed-percentages` | Public typed frequency and percentage calculation models; frequency-percentage property consumers remain for their owning later grammar cycles |
| `official.value.calc` | `Partial` | `#calc-notation,#calc-syntax,#calc-type-checking` | Typed C03 roots and integrated property consumers; angle, frequency, Media resolution, keyframe percentage, font-feature numeric, and C05 function-owned integrations remain in their owning later cycles |

The private reserved rows for `official.value.syntax-token-stream`,
`official.value.component-value`, `official.value.simple-block`,
`official.value.function`, `official.value.declaration-value`,
`official.value.any-value`, `official.value.css-wide-keyword`,
`official.value.custom-ident`, `official.value.ident`,
`official.value.string`, `official.value.url`, and
`official.value.url-modifier` remain deferred. `official.value.position` remains
deferred to C04.

## Timing Property Metadata

All seven rows retain their property-schema identity and are implemented by
`crate::parser::timing`. Their named public metadata cases are in
`conformance_catalog`; their semantic behavior cases are in `timing_domains`.

| Stable ID | Source | Status | Production | Supported subset and remainder |
| --- | --- | --- | --- | --- |
| `baseline.property.transition-duration` | `I-TRANSITIONS1` | `Complete` | `#propdef-transition-duration` | Entire named production |
| `baseline.property.transition-delay` | `I-TRANSITIONS1` | `Complete` | `#propdef-transition-delay` | Entire named production |
| `baseline.property.animation-duration` | `I-ANIMATIONS1` | `Complete` | `#propdef-animation-duration` | Entire named production |
| `baseline.property.animation-delay` | `I-ANIMATIONS1` | `Complete` | `#propdef-animation-delay` | Entire named production |
| `baseline.property.animation-iteration-count` | `I-ANIMATIONS1` | `Complete` | `#propdef-animation-iteration-count` | Entire named production |
| `baseline.property.transition` | `I-TRANSITIONS1` | `Partial` | `#propdef-transition` | I01 components plus C03 duration, signed delay, iteration, and typed calculation syntax; C05 easing and function grammar closure remains unsupported |
| `baseline.property.animation` | `I-ANIMATIONS1` | `Partial` | `#propdef-animation` | I01 components plus C03 duration, signed delay, iteration, and typed calculation syntax; C05 easing and function grammar closure remains unsupported |

No other timing catalog row changes status or boundary in this cycle.

## Compatibility Evidence

Every I01 input continues to expose its exact compatibility projection through
`i01_subset()`. Newly accepted syntax returns `None` only when the frozen I01
payload cannot represent it. The retained evidence is:

- `i01_c01_observables` keeps the frozen fixture byte-identical and preserves
  all recorded parser, recovery, diagnostic, and debug observables;
- `numeric_domains` preserves direct construction and matching of
  `CssGridFlowTolerance::Percent(f32)` while parser-produced values expose a
  checked current percentage and matching finite I01 projection;
- `typed_calculations` preserves the exact simple length-sum projection while
  exposing new typed products, groups, nested calculations, and scalar current
  accessors;
- `timing_domains` preserves positive I01 timing projections and their exact
  debug observables while proving that signed delays and typed calculations use
  the current duration, delay, iteration, transition, and animation domains.

Literal-only range constraints are enforced during authored parsing and checked
construction. A well-typed calculation is retained when its eventual range is
owned by computed-value processing. Typed trees preserve authored structure and
units without evaluating dimensioned results.

## Root-Owned Follow-Up

Root `surgeist` must:

1. promote the `surgeist-css` gitlink deliberately;
2. expose the public additions above through the root facade without adding a
   leaf-to-sibling dependency;
3. update root-owned lowering adapters to consume current accessors and treat
   `i01_subset()` only as compatibility data;
4. refresh the root-owned generated API audit artifacts with the root generator;
5. update root integration tests and documentation for finite construction,
   symbolic calculation inspection, duration versus delay, literal-range versus
   calculation-range phase, and the absence of cascade, resolution, layout, and
   timeline evaluation in this crate.

# P01-I02-C01 CSS Evolution Migration

This is the current, revision-independent authority for the C01
declaration-inspection and public-enum evolution migration. It supersedes those
subjects in `plans/handoffs/P01-I01-css-migration.md`; the earlier record remains
the authority for unrelated I01 migration history. Candidate provenance and
execution evidence belong in the separate candidate handoff, not in this
product migration record.

## Compatibility Boundary

C01 changes how external consumers inspect parser-produced known declarations
and match evolving public enums. It does not change the accepted CSS language,
catalog identities or support states, recovery boundaries or actions,
diagnostics or coordinates, dependency or feature shape, or crate ownership.
`surgeist-css` still owns strict authored CSS parsing; cascade, substitution,
contextual resolution, selector matching, resource loading, cross-crate lowering,
and generated API audit artifacts remain outside this leaf.

## Declaration Migration

The former public `CssKnownDeclaration` enum exposed one variant per property,
with each variant carrying `CssDeclaredValue<T>`. Callers commonly selected both
property identity and declared-value state by matching nested public enums:

```rust,ignore
match known {
    CssKnownDeclaration::Width(CssDeclaredValue::Value(value)) => use_width(value),
    CssKnownDeclaration::Width(CssDeclaredValue::Global(keyword)) => use_global(keyword),
    CssKnownDeclaration::Width(CssDeclaredValue::SubstitutionDependent(value)) => {
        defer_width(value)
    }
    _ => {}
}
```

`CssKnownDeclaration` is now a parser-owned struct with a private coupled value
discriminator. Property identity is derived by `property()` from that active
discriminator; it is not stored as a separately mutable field. There is no
public constructor, property/value mismatch state, broad value bag, or duplicate
`V2` branch.

Match the new borrowed declared-value and property-value views instead:

```rust,ignore
match known.declared_value() {
    CssKnownDeclaredValueRef::Property(property) => match property {
        CssKnownPropertyValueRef::Width(width) => {
            use_authored(width.as_css());
            if let Some(i01_width) = width.i01_subset() {
                use_i01_width(i01_width);
            }
        }
        _ => {}
    },
    CssKnownDeclaredValueRef::Global(keyword) => use_global(keyword),
    CssKnownDeclaredValueRef::SubstitutionDependent(value) => defer_value(value),
    _ => {}
}
```

The shorter accessor migration is exact:

| Previous inspection | Current inspection |
| --- | --- |
| Match `CssKnownDeclaration::<Property>(...)` | Read `known.property()` for canonical identity, then inspect `known.declared_value()` or a convenience accessor |
| Match `CssDeclaredValue::Value(value)` or call `value()` | Match `CssKnownDeclaredValueRef::Property(property)`, then match a concrete `CssKnownPropertyValueRef::<Property>(wrapper)` |
| Match `CssDeclaredValue::Global(keyword)` or call `global()` on the nested value | Call `known.global()` or match `CssKnownDeclaredValueRef::Global(keyword)` |
| Match `CssDeclaredValue::SubstitutionDependent(value)` or call `substitution_dependent()` on the nested value | Call `known.substitution_dependent()` or match `CssKnownDeclaredValueRef::SubstitutionDependent(value)` |
| Consume a property parser payload directly | Use the concrete wrapper's `as_css()` and, only when I01 compatibility is required, `i01_subset()` |

`declared_value()` has exactly three current semantic branches: `Property`,
`Global`, and `SubstitutionDependent`. The `property_value()`, `global()`, and
`substitution_dependent()` convenience accessors are mutually exclusive views
of those branches: exactly one can be present for a known declaration.
`CssDeclaredValue<T>` is no longer a public known-declaration inspection type.

Both new borrowed view enums, `CssKnownDeclaredValueRef` and
`CssKnownPropertyValueRef`, are non-exhaustive. Match their concrete branches
with a wildcard. The property view has one generated variant for each schema row
and each variant borrows that row's concrete wrapper.

## Wrapper And Payload Model

Every one of the exact 179 `property_schema!` rows supplies a unique
`Css<SchemaVariant>PropertyValue` wrapper identifier. The schema generates the
corresponding wrapper, property-view variant, private coupled declaration
variant, and parser dispatch arm together. Each wrapper has private construction
and two public inspection methods:

- `as_css() -> &str` returns the exact authored ordinary value. It excludes
  parser-owned boundary trivia and the terminal importance annotation, while
  preserving interior spelling, escapes, comments, case, commas, and block text.
- `i01_subset() -> Option<&I01PayloadType>` returns the frozen I01 compatibility
  payload. Every ordinary value parsed by the current grammar returns `Some`.
  A later grammar may return `None` only for newly supported syntax that the I01
  payload cannot represent; `None` does not mean that current syntax failed
  validation.

The generated `CssAllPropertyValue` exists so the schema and public view remain
one-to-one. Current `all` parsing produces only the `Global` or
`SubstitutionDependent` declared-value branches and therefore does not construct
an ordinary `CssAllPropertyValue`.

The `overflow` row has an intentional name and shape distinction:

- `CssOverflowPropertyValue` is now the generated private-field authored wrapper
  for the `overflow` property. Its inspection surface is `as_css()` plus
  `i01_subset()`.
- `CssOverflowI01PropertyValue` is the renamed I01 parser payload. It is the
  non-exhaustive enum with `Single(CssOverflow)` and `Pair(CssOverflowAxes)`
  variants.
- Consequently,
  `CssOverflowPropertyValue::i01_subset() -> Option<&CssOverflowI01PropertyValue>`.
  Downstream code that formerly matched `CssOverflowPropertyValue::Single` or
  `Pair` must match those variants on the returned I01 payload instead.

## Field Meanings And Downstream Boundaries

- `CssKnownDeclaration`'s private discriminator jointly owns canonical property
  identity and its declared value. `property()` derives identity from it.
- `CssKnownDeclaredValueRef::Property` borrows an ordinary, already parsed,
  property-specific wrapper. `Global` carries a whole-value CSS-wide keyword.
  `SubstitutionDependent` borrows exact authored syntax whose final grammar
  depends on later substitution.
- A property wrapper's private authored field owns the exact ordinary value
  slice. Its private representation owns the current property parser payload.
  Neither can be forged or paired with a different property by a public caller.
- `CssDeclaration::importance()` remains the terminal annotation state.
  Importance is not included in wrapper `as_css()` text and is not applied by
  this crate.
- Root adapters may consume property identity, the declared-value branch,
  authored ordinary text, the I01 compatibility payload, and importance. Root
  must keep substitution-dependent syntax symbolic until the layer with
  substitution context owns validation and resolution.

The finite behavioral corpus continues to record authored inputs, clean state,
retained syntax identities, authored declaration text and importance, typed
diagnostics, positions, spans, and recovery actions in both feature modes. It is
the equivalence boundary for C01 representation changes; it does not turn source
layout, test identities, invocation counts, or command state into product
behavior.

## Public Enum Evolution Policy

Exactly `CssImportance` and `CssSupportStatus` are closed public enums.
Downstream consumers may exhaustively match their two and three branches,
respectively. Every other public enum is non-exhaustive and downstream matches
must include a wildcard. The two C01 view enums added with the declaration model,
`CssKnownDeclaredValueRef` and `CssKnownPropertyValueRef`, follow that evolving
policy.

The original enum-boundary source diff newly made the following exact 139 public
enums in `src/syntax.rs` non-exhaustive:

- `CssImportTarget`
- `CssImportLayer`
- `CssKeyframesName`
- `CssKeyframeSelector`
- `CssFontFaceSource`
- `CssFontFaceStyle`
- `CssFontDisplay`
- `CssFontFormatHint`
- `CssFontTechHint`
- `CssScopedRule`
- `CssScopedStyleSelector`
- `CssMediaQueryModifier`
- `CssMediaType`
- `CssContainerCondition`
- `CssContainerFeatureQuery`
- `CssContainerStyleQuery`
- `CssMediaFeatureQuery`
- `CssQueryComparison`
- `CssOrientation`
- `CssColorSchemePreference`
- `CssReducedMotionPreference`
- `CssReducedTransparencyPreference`
- `CssContrastPreference`
- `CssForcedColorsMode`
- `CssHoverCapability`
- `CssPointerCapability`
- `CssDisplayMode`
- `CssResolutionUnit`
- `CssScrollbarWidth`
- `CssDisplay`
- `CssBoxSizing`
- `CssLayoutPosition`
- `CssDirection`
- `CssOverflow`
- `CssFlexDirection`
- `CssFlexWrap`
- `CssFloat`
- `CssClear`
- `CssAlignment`
- `CssAlignItems`
- `CssPlaceAlignment`
- `CssVisibility`
- `CssContentVisibility`
- `CssContent`
- `CssContentItem`
- `CssCounterStyle`
- `CssBuiltInCounterStyle`
- `CssListStyleType`
- `CssListStylePosition`
- `CssListStyleImage`
- `CssCounterChanges`
- `CssGridFlowTolerance`
- `CssGridTrackBreadth`
- `CssGridTrackSize`
- `CssGridTrackComponent`
- `CssGridRepeatCount`
- `CssGridTemplateAreaCell`
- `CssGridTemplateAreas`
- `CssGridTemplate`
- `CssGridAutoFlowAxis`
- `CssGridLine`
- `CssGrid`
- `CssOrder`
- `CssFlex`
- `CssZIndex`
- `CssBoxDecorationBreak`
- `CssWritingMode`
- `CssTextAlign`
- `CssTextAlignLast`
- `CssVerticalAlign`
- `CssFontFamilyNameKind`
- `CssFontWeight`
- `CssFontStyle`
- `CssFontStretch`
- `CssFontVariant`
- `CssFontFeatureSettings`
- `CssFontFeatureValue`
- `CssLetterSpacing`
- `CssTextWrap`
- `CssWhiteSpace`
- `CssWordBreak`
- `CssOverflowWrap`
- `CssTextOverflow`
- `CssTextDecorationLineComponent`
- `CssTextDecorationStyle`
- `CssTextDecorationThickness`
- `CssTextTransform`
- `CssLengthUnit`
- `CssLength`
- `CssBorderStyle`
- `CssBoxShadow`
- `CssImageLayer`
- `CssHorizontalPositionKeyword`
- `CssVerticalPositionKeyword`
- `CssPositionComponent`
- `CssBackgroundSizeComponent`
- `CssBackgroundSize`
- `CssBackgroundRepeatStyle`
- `CssBackgroundRepeat`
- `CssBackgroundBox`
- `CssBackgroundAttachment`
- `CssCursorKeyword`
- `CssCursor`
- `CssPointerEvents`
- `CssUserSelect`
- `CssOutlineStyle`
- `CssOutlineWidth`
- `CssTransformFunctionKind`
- `CssTransform`
- `CssTranslate`
- `CssRotate`
- `CssScale`
- `CssFilterFunction`
- `CssFilter`
- `CssBasicShape`
- `CssClipPath`
- `CssTimeUnit`
- `CssEasing`
- `CssTransitionProperty`
- `CssAnimationName`
- `CssAnimationIterationCount`
- `CssAnimationDirection`
- `CssAnimationFillMode`
- `CssAnimationPlayState`
- `CssColor`
- `CssPredefinedColorSpace`
- `CssSystemColor`
- `CssColorInterpolationSpace`
- `CssHueInterpolationMethod`
- `CssRelativeColorFunction`
- `CssSelector`
- `CssSelectorCombinator`
- `CssPseudoClass`
- `CssPseudoElement`
- `CssNthPattern`
- `CssAttributeMatcher`
- `CssAttributeCaseSensitivity`
- `CssCalcLength`
- `CssCalcOperator`

Completeness of this list is owned by direct inspection of that task's source
diff and the final public source, including generated and feature-gated public
enums. It is deliberately not enforced by a Rust test that parses source text or
counts declarations.

## Non-Effects

C01 adds no CSS grammar, property, alias, source record, catalog record, support
status, diagnostic code, recovery action, dependency, feature, target, build
script, or external software. Existing clean and recovered inputs retain the
same authored nodes, property identities, importance, diagnostics, positions,
spans, actions, and feature-mode behavior. Application-strict validators consume
ordinary parsing semantics and reports; their internal parser invocation count
is not a public contract, and they do not select a second grammar.

## Root-Owned Follow-Up

After selecting the leaf candidate, root `surgeist` owns all integration work:

1. Verify the selected leaf candidate and deliberately update the
   `crates/surgeist-css` gitlink under root's promotion workflow.
2. Update facade reexports for `CssKnownDeclaration`,
   `CssKnownDeclaredValueRef`, `CssKnownPropertyValueRef`, all generated property
   wrappers, and `CssOverflowI01PropertyValue`; remove public reliance on
   `CssDeclaredValue<T>` and property variants of `CssKnownDeclaration`.
3. Update every root-owned exhaustive match on evolving CSS enums to include a
   wildcard. Preserve exhaustive matches only for `CssImportance` and
   `CssSupportStatus`.
4. Migrate root adapters to `property()`, `declared_value()` or the mutually
   exclusive convenience accessors, then match the concrete property wrapper
   with a wildcard. Consume `as_css()` for exact authored ordinary values and
   carry `CssDeclaration::importance()` separately.
5. Preserve substitution-dependent authored values until the root-owned
   substitution/resolution layer has the required context. Use `i01_subset()`
   only as an explicitly bounded compatibility view and handle `None` for later
   syntax without treating it as parse failure.
6. Distinguish the authored `CssOverflowPropertyValue` wrapper from its
   `CssOverflowI01PropertyValue` payload in facade signatures and adapters.
7. Run root's committed API generator and update only root-owned API audit
   artifacts. This leaf owns no generated API report.
8. Update root documentation and integration tests for wildcard evolution,
   declaration views, concrete wrappers, authored values, importance, global and
   substitution-dependent branches, and the overflow rename.
9. Run root's configured workspace, feature, lint, format, API-artifact,
   dependency, MSRV, unsafe, integration, promotion, and publication gates.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and all generated API artifacts. The leaf migration record does not
authorize mutations in root or sibling repositories.

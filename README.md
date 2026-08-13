# surgeist-css

Browser-recovering CSS ingestion for Surgeist. This crate parses CSS-facing input into CSS-owned authored syntax contracts. Root-owned Surgeist adapters lower parsed CSS syntax into typed style data.

The ordinary `parse_sheet` and `parse_style_attribute` entry points use browser recovery: each returns a report containing valid retained syntax and every structured recovery diagnostic in source order. Unsupported or malformed source units are dropped, replaced, retained with an implicit closure, ignored, or stopped at the documented boundary; valid siblings remain eligible. A clean report means that no recovery diagnostic was produced. An empty retained tree alone does not establish that the source was clean, so consumers should inspect `is_clean()` or `diagnostics()` rather than infer validity from syntax length.

```rust
use surgeist_css::{CssRecoveryAction, CssRule, parse_sheet};

let report = parse_sheet(
    ".before { color: red; } @unknown value; .after { color: blue; }",
);
assert_eq!(report.syntax().rules().len(), 2);
assert!(matches!(report.syntax().rules()[0], CssRule::Style(_)));
assert_eq!(
    report.diagnostics()[0].action(),
    CssRecoveryAction::DropAtRule,
);
```

Style attributes use the same ordinary declaration parser as style-rule blocks. Declarations retain their authored order, source position, coupled property/value type, and terminal `!important` state.

```rust
use surgeist_css::{CssImportance, CssPropertyNameRef, parse_style_attribute};

let report = parse_style_attribute("color: red; mystery: 1; width: 2px !important");
assert_eq!(report.syntax().len(), 2);
assert_eq!(report.diagnostics().len(), 1);
let width = &report.syntax()[1];
assert_eq!(width.importance(), CssImportance::Important);
assert!(matches!(width.property_name(), CssPropertyNameRef::Known(_)));
```

## Declaration inspection and API evolution

`CssKnownDeclaration` is a parser-owned, private-field value. Its `property()`
identity is derived from the active coupled value, so callers cannot construct or
mutate a property/value mismatch. `declared_value()` returns exactly one
`CssKnownDeclaredValueRef` branch: `Property`, `Global`, or
`SubstitutionDependent`. The convenience accessors `property_value()`,
`global()`, and `substitution_dependent()` are mutually exclusive views of those
same three branches.

Ordinary property values are borrowed through the non-exhaustive
`CssKnownPropertyValueRef`. Match the concrete property wrapper and retain a
wildcard for future variants:

```rust
use surgeist_css::{
    CssImportance, CssKnownDeclaredValueRef, CssKnownPropertyValueRef,
    parse_style_attribute,
};

let report = parse_style_attribute("width: calc(100% - 12px) !important");
let declaration = &report.syntax()[0];
assert_eq!(declaration.importance(), CssImportance::Important);
let known = declaration.known().expect("known declaration");

match known.declared_value() {
    CssKnownDeclaredValueRef::Property(property) => match property {
        CssKnownPropertyValueRef::Width(width) => {
            assert_eq!(width.as_css(), "calc(100% - 12px)");
            assert!(width.i01_subset().is_some());
        }
        _ => panic!("expected width"),
    },
    CssKnownDeclaredValueRef::Global(_)
    | CssKnownDeclaredValueRef::SubstitutionDependent(_) => {
        panic!("expected an ordinary property value")
    }
    _ => panic!("future declared-value branch"),
}
```

Every one of the 179 schema rows has a generated
`Css<SchemaVariant>PropertyValue` wrapper. `as_css()` returns the exact authored
ordinary value, preserving its interior spelling and trivia while excluding
parser-owned boundary trivia and the terminal importance annotation.
`i01_subset()` exposes the compatibility payload only when the value belongs to
the frozen I01 representation. Every I01 input retains its exact `Some`
projection; newly accepted I02 syntax returns `None` when the I01 payload cannot
represent it.

The `overflow` row illustrates the wrapper/payload distinction. The generated
`CssOverflowPropertyValue` is the authored property wrapper, while
`CssOverflowI01PropertyValue` is the renamed I01 payload containing the
`Single` and `Pair` shapes.

## Finite numeric values, timing domains, and symbolic calculations

Current numeric models reject NaN and both infinities at checked construction
boundaries. Duration literals are additionally non-negative, while delay
literals are signed. Range checks that belong to the authored literal are
immediate; a well-typed calculation remains symbolic when its eventual range
belongs to computed-value processing.

```rust
use surgeist_css::{
    CssDelay, CssDuration, CssDurationLiteral, CssKnownPropertyValueRef,
    CssTimeUnit, parse_style_attribute,
};

assert!(CssDurationLiteral::try_new(-1.0, CssTimeUnit::Seconds).is_none());

let report = parse_style_attribute(concat!(
    "transition-duration: calc(-1s + 2s); ",
    "transition-delay: -250ms",
));
assert!(report.is_clean());

let CssKnownPropertyValueRef::TransitionDuration(duration) = report.syntax()[0]
    .known()
    .expect("known duration")
    .property_value()
    .expect("ordinary duration")
else {
    panic!("expected transition-duration");
};
assert!(matches!(
    duration.durations().values()[0],
    CssDuration::Calculation(_)
));
assert!(duration.i01_subset().is_none());

let CssKnownPropertyValueRef::TransitionDelay(delay) = report.syntax()[1]
    .known()
    .expect("known delay")
    .property_value()
    .expect("ordinary delay")
else {
    panic!("expected transition-delay");
};
assert!(matches!(
    delay.delays().values()[0],
    CssDelay::Literal(value) if value.value() == -250.0
));
```

The current accessors expose `CssDuration`, `CssDelay`, typed iteration values,
and typed calculation trees. `i01_subset()` remains the frozen compatibility
view: every I01 timing value retains its exact projection, while newly accepted
signed-delay or calculation syntax returns `None` when the older payload cannot
represent it. Calculation roots preserve authored units and expression shape;
this crate does not resolve relative units, evaluate computed ranges, run
animation timelines, or lower values into sibling Surgeist crates.

## Property-specific authored positions

Current position values preserve authored symbolic offsets and expose both axes
without resolving percentages, calculations, writing modes, positioning boxes,
object sizes, layout, painting, or transforms. `CssPositionOffset` accepts only
the position-valid length-percentage domain and retains whether an offset was
free or authored against a named edge.

The property grammars and accessors are deliberately distinct:

- `CssObjectPositionPropertyValue::position()` exposes one generic
  `CssPositionValue` through `CssObjectPosition::value()`.
- `CssMaskPositionPropertyValue::positions()` exposes a nonempty list whose
  `CssMaskPosition` layers each contain one generic `CssPositionValue`.
- `CssBackgroundPositionPropertyValue::positions()` exposes a distinct
  nonempty layer list that additionally admits the background-only
  three-component form.
- `CssTransformOriginPropertyValue::origin()` exposes explicit horizontal and
  vertical axes plus an optional `CssTransformOriginZ`; the z component is a
  checked authored length and cannot contain a percentage.

```rust
use surgeist_css::{
    CssHorizontalPosition, CssKnownPropertyValueRef, CssLength,
    parse_style_attribute,
};

let report = parse_style_attribute(concat!(
    "background-position: left 10px top; ",
    "object-position: right 5% bottom 2px; ",
    "transform-origin: top 50px",
));
assert!(report.is_clean());

let CssKnownPropertyValueRef::BackgroundPosition(background) = report.syntax()[0]
    .known().expect("known background position")
    .property_value().expect("ordinary background position")
else { panic!("expected background-position") };
assert!(matches!(
    background.positions().positions()[0].horizontal(),
    CssHorizontalPosition::LeftOffset(offset)
        if matches!(offset.value(), CssLength::Px(value) if value.value() == 10.0)
));

let CssKnownPropertyValueRef::ObjectPosition(object) = report.syntax()[1]
    .known().expect("known object position")
    .property_value().expect("ordinary object position")
else { panic!("expected object-position") };
assert!(matches!(
    object.position().value().horizontal(),
    CssHorizontalPosition::RightOffset(_)
));

let CssKnownPropertyValueRef::TransformOrigin(transform) = report.syntax()[2]
    .known().expect("known transform origin")
    .property_value().expect("ordinary transform origin")
else { panic!("expected transform-origin") };
assert!(matches!(
    transform.origin().z().map(|z| z.value()),
    Some(CssLength::Px(value)) if value.value() == 50.0
));
```

The background, mask, and transform wrappers keep `i01_subset()` as a frozen
compatibility view. Every I01 value retains its exact projection; newly accepted
current syntax returns `None` when the older payload cannot represent it without
loss. `object-position` is additive and has no I01 projection. Position use
inside gradients, transforms, filters, and basic shapes remains on its separate
function grammar boundary.

`CssImportance` and `CssSupportStatus` are deliberately closed and may be
matched exhaustively. Every other public enum is non-exhaustive and requires a
wildcard in downstream matches. This declaration inspection migration changes
neither accepted input nor parsing, recovery, or diagnostic behavior.

Each diagnostic exposes a typed error and stable root code, the first responsible source position, the complete recovery-unit span, and one `CssRecoveryAction`. Source byte offsets index the original UTF-8 input; line and column indices are zero-based, and columns count UTF-16 code units. Display text is for people, not control flow—match typed variants with a wildcard for future non-exhaustive cases.

CSS custom properties preserve case-sensitive names and authored value text, including interior trivia. Known-property values whose grammar depends on `var(...)` remain substitution-dependent authored values. The crate recognizes terminal `!important` but does not apply cascade or perform custom-property substitution or post-substitution validation.

The independent support catalog reports an exact support status for each bounded I01 production: `Complete`, `Partial`, or `RecognizedUnsupported`. Partial records document both the accepted subset and valid-but-unsupported remainder. A clean use of a partial production's supported subset is accepted; status is metadata about the whole named production, not a parse-result validity flag.

## Conformance sources and atomic records

The conformance source registry assigns every selected dated specification or
preserved repository baseline a stable `CssSpecificationSourceId`, module,
level, and `CssSpecificationTier`. The tier classifies provenance only; it does
not imply parser support. A source has exactly one immutable URL or repository
provenance value. `specification_source`, `feature_metadata`, and
`conformance_exclusion` perform exact, case-sensitive lookup without trimming
or aliasing.

```rust
use surgeist_css::{
    CssExclusionReason, CssSpecificationTier, CssSupportStatus,
    conformance_exclusion, feature_metadata, specification_source,
};

let color = specification_source("O-COLOR4").expect("dated Color 4 source");
assert_eq!(color.tier(), CssSpecificationTier::Snapshot2026Official);
assert!(specification_source("o-color4").is_none());

let importance = feature_metadata("foundation.declaration.importance")
    .expect("atomic parser-facing record");
assert_eq!(importance.status(), CssSupportStatus::Complete);
assert!(importance.baseline_alias_targets().is_empty());

let pseudo_elements = feature_metadata("baseline.selector.pseudo-element")
    .expect("preserved aggregate alias");
assert_eq!(
    pseudo_elements.baseline_alias_targets()[0].as_str(),
    "official.selector.generated",
);

let processing = conformance_exclusion("excluded.O-IMAGES3.processing")
    .expect("official source exclusion");
assert_eq!(
    processing.reason(),
    CssExclusionReason::OutsideAuthoredSyntaxBoundary,
);
```

An atomic feature record is parser-facing and carries one truthful
`CssSupportStatus`. The four preserved baseline aggregate aliases remain
queryable and expose their immutable atomic target slices, but they do not own
parser dispatch. A private reserved coverage slot identifies a later grammar
boundary only: it is not a feature record, has no support status, and does not
make its spelling recognized. An exclusion is a public source-audit fact for an
informative, superseded, or out-of-boundary source item; it likewise carries no
support status and never changes parser diagnostics. Adding registry metadata,
aliases, reserved slots, exclusions, or implementation inventories does not
change accepted CSS, retained syntax, diagnostics, positions, spans, or
recovery actions.

Enable the additive `app-strict` feature to expose `validate_sheet` and `validate_style_attribute`. Each validator consumes ordinary parsing semantics and its report, returns retained syntax only for a clean report, and otherwise returns the complete non-empty diagnostic sequence. The validators do not select a second grammar, and enabling the feature does not change ordinary parsing or recovery.

This crate owns authored CSS syntax, intrinsic grammar validation, recovery boundaries, diagnostics, and support metadata. It does not apply cascade or inheritance, substitute or resolve variables, evaluate queries, match selectors, resolve URLs or resources, perform layout or painting, serialize a CSSOM, or lower CSS into sibling Surgeist types. Root-owned integration owns cross-crate lowering and generated API audit artifacts.

CSS custom properties are parsed as authored syntax. Custom property names are case-sensitive, `var(...)` references and fallback token text remain symbolic, and supported properties containing `var(...)` parse as variable-dependent authored values. This crate does not resolve variables, run cascade substitution, or validate post-substitution values.

Colors are parsed as authored CSS color syntax. `surgeist-css` accepts named colors, alpha hex, modern color functions, `currentcolor`, system colors, `color-mix()`, and relative color syntax as typed syntax, but does not resolve system colors, substitute variables, evaluate relative channels, mix colors, convert color spaces, or adapt colors to a renderer gamut. Downstream style/render layers own those resolution steps.

Pseudo-classes for UI interaction, form state, structure, selector-list filtering, and overlay state are parsed as authored selector syntax. This crate does not evaluate pseudo-class matches; runtime matching belongs to downstream Surgeist layers with node and interaction state.

Selector-list pseudo-class arguments are parsed as authored selector syntax with bounded recovery. In recognized `:is()` and `:where()` lists, an invalid member is dropped with `DropSelectorListItem` while the other members remain in authored order. Other selector lists are unforgiving: `:not()` preserves supported complex selector lists, `:has()` preserves supported relative selector lists including leading child and sibling combinators, and `:nth-child()` / `:nth-last-child()` preserve optional `of` selector filters, but an invalid member causes the containing qualified rule to be dropped with `DropQualifiedRule`. Later sibling rules remain eligible for parsing.

Media queries are parsed as authored conditions on `@media` group rules. `surgeist-css` does not evaluate media query matches; environment-dependent matching belongs to downstream Surgeist layers.

Container queries are parsed as authored conditions on `@container` group rules. `surgeist-css` does not evaluate container query matches; container-dependent matching belongs to downstream Surgeist layers.

Imports are parsed as authored `@import` contracts only. `surgeist-css` preserves import targets, layer clauses, and media conditions, but does not resolve paths, load files, or merge imported sheets; root/style-owned Surgeist integration performs loading and composition.

Cascade layers are parsed as authored `@layer` statements and blocks, including named and anonymous layer blocks. `surgeist-css` records layer names and layer-contained rules, but does not compute cascade order, declaration precedence, or runtime cascade effects.

Scoped styles are parsed as authored `@scope` rules with optional roots, limits, scoped style selectors, and scoped nested group rules. Relative scoped selectors remain structurally distinct from ordinary selectors. `surgeist-css` does not perform scope matching, selector matching, or scoping proximity calculations.

Pseudo-elements are parsed as terminal authored selector syntax for the supported `::before`, `::after`, `::marker`, `::selection`, and `::backdrop` forms. The parser records them on selector compounds, but does not filter declarations by pseudo-element or perform generated box/layout behavior.

Generated content, list markers, and counters are parsed as typed authored property values for `content`, list-style longhands and shorthand, and counter change properties. Strings, URLs, attribute references, quote keywords, counter functions, list-style slots, and counter change lists remain symbolic. `surgeist-css` does not lay out generated content or list markers, resolve marker images, or evaluate/reset/increment counters.

Font faces are parsed as authored `@font-face` descriptor blocks only. `surgeist-css` validates supported descriptors and preserves font source hints, unicode ranges, and variation ranges, but does not perform font lookup, loading, matching, or resource validation; downstream Surgeist layers own those steps.

Keyframes are parsed as authored `@keyframes` rules. `surgeist-css` validates keyframe names, selector offsets, and declarations, but does not evaluate animations, match animation names to rules, interpolate values, or run animation timelines.

CSS nesting is parsed as syntax sugar and flattened into ordinary style and conditional group rules while preserving source order. `surgeist-css` does not evaluate selector matches or cascade results during flattening.

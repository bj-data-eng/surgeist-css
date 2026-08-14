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

## Dedicated authored function grammars

Current property accessors expose dedicated typed function families while
`i01_subset()` remains the frozen compatibility view. `transform.current()`
returns `CssTransformValue`, timing-function wrappers expose current
`CssEasingValue` lists, `filter.current()` and `backdrop-filter.current()` return
`CssFilterValue`, `box-shadow.current()` returns `CssBoxShadow`, and
`clip-path.current()` returns an optional `CssClipPathValue`. A current value can
be valid when its I01 projection is `None`; consumers must not treat the
compatibility view as the current grammar.

```rust
use surgeist_css::{
    CssBasicShapeValue, CssClipPathValue, CssFilterFunctionValue, CssFilterValue,
    CssKnownPropertyValueRef, CssTransformFunctionValue, CssTransformValue,
    parse_style_attribute,
};

let report = parse_style_attribute(concat!(
    "transform: translate3d(10%, 2px, 4em) rotate(45deg); ",
    "filter: blur(2px) drop-shadow(red 1px 2px 3px); ",
    "clip-path: polygon(round 2px, 0 0, 100% 0)",
));
assert!(report.is_clean());

let CssKnownPropertyValueRef::Transform(transform) = report.syntax()[0]
    .known().expect("known transform")
    .property_value().expect("ordinary transform")
else { panic!("expected transform") };
assert!(matches!(
    transform.current(),
    CssTransformValue::Functions(functions)
        if matches!(functions.functions()[0], CssTransformFunctionValue::Translate3d(_))
));

let CssKnownPropertyValueRef::Filter(filter) = report.syntax()[1]
    .known().expect("known filter")
    .property_value().expect("ordinary filter")
else { panic!("expected filter") };
assert!(matches!(
    filter.current(),
    CssFilterValue::Functions(functions)
        if matches!(functions.functions()[1], CssFilterFunctionValue::DropShadow(_))
));

let CssKnownPropertyValueRef::ClipPath(clip) = report.syntax()[2]
    .known().expect("known clip path")
    .property_value().expect("ordinary clip path")
else { panic!("expected clip-path") };
assert!(matches!(
    clip.current(),
    Some(CssClipPathValue::BasicShape(CssBasicShapeValue::Polygon(polygon)))
        if polygon.round().is_some()
));
```

The typed transform family covers the selected two-dimensional Transforms 1
functions and the preserved I01 three-dimensional subset with exact arity,
separator, and dimension domains. Easing values distinguish keywords,
`cubic-bezier()`, and `steps()`. Box shadows and filter `drop-shadow()` use
different models, so filter shadows cannot contain `inset` or spread. Filter
lists preserve URL/function order and typed function-specific operands. The
selected basic-shape family exposes `inset()`, `circle()`, `ellipse()`, and
`polygon()`, including polygon `round <length>`.

These are authored syntax values. This crate does not multiply transform
matrices, interpolate or evaluate easing, render shadows or filters, resolve
URLs, compute shape geometry, perform layout or painting, or lower values into
sibling crates. `path()`, `shape()`, `rect()`, `xywh()`, and clip-path
reference-box combinations remain outside the selected shape subset.
`transition`, `animation`, `backdrop-filter`, and `clip-path` therefore retain
their explicit Partial catalog boundaries; support for one typed function does
not promote an aggregate or an unselected production.

## Authored colors and frozen I01 compatibility

The current color model preserves the authored Color 4 grammar rather than a
computed color. It distinguishes named, transparent, current, hexadecimal,
current and deprecated system, legacy and modern RGB/HSL, HWB, Lab/LCH,
Oklab/Oklch, and predefined `color()` branches. Finite specified components
remain authored even when they are outside a computed range, and typed
calculations remain symbolic. The current opacity model likewise preserves a
finite number or percentage, including signed and out-of-range specified
values.

Color-bearing property wrappers expose the current value through `current()`,
and the opacity wrapper exposes its current `CssOpacityValue` through `value()`.
Their `i01_subset()` remains a separate frozen compatibility projection: every
frozen I01 input keeps its exact projection, while a newly accepted current
value returns `None` when the old `CssColor` or `CssOpacity` model cannot
represent it without loss. A missing I01 projection does not make the current
value invalid.

```rust
use surgeist_css::{
    CssAuthoredSystemColor, CssKnownPropertyValueRef, CssOpacityValue,
    parse_style_attribute,
};

let report = parse_style_attribute("color: ActiveBorder; opacity: 150%");
assert!(report.is_clean());

let CssKnownPropertyValueRef::Color(color) = report.syntax()[0]
    .known().expect("known color")
    .property_value().expect("ordinary color")
else { panic!("expected color") };
assert_eq!(
    color.current().system(),
    Some(CssAuthoredSystemColor::ActiveBorder),
);
assert!(color.i01_subset().is_none());

let CssKnownPropertyValueRef::Opacity(opacity) = report.syntax()[1]
    .known().expect("known opacity")
    .property_value().expect("ordinary opacity")
else { panic!("expected opacity") };
assert!(matches!(opacity.value(), CssOpacityValue::Percentage(value)
    if value.value() == 150.0));
assert!(opacity.i01_subset().is_none());
```

The preserved Color 5 surface is intentionally narrower: relative colors cover
`rgb`/`rgba`, `hsl`/`hsla`, `hwb`, `lab`, `lch`, `oklab`, `oklch`, and
predefined RGB/XYZ `color()` spaces with closed per-family channel
environments. `color-mix()` requires an interpolation method and exactly two
colors, accepts optional trailing percentages, and permits hue interpolation
methods only in polar spaces. This crate does not provide `alpha()`, custom
color profiles, `light-dark()`, or `device-cmyk()`.

These values remain authored syntax. This crate does not clamp computed color
or opacity values, resolve `currentcolor` or system colors, evaluate relative
channels or calculations, perform color conversion or gamut mapping, resolve a
mix, apply contrast, serialize computed colors, or lower colors into a sibling
crate.

## Authored Grid repetition and keyframe structure

The six Grid repetition consumers expose a parser-owned current value through
`current()` while preserving their existing `i01_subset()` compatibility view.
Current Grid track lists distinguish general lists from lists containing exactly
one automatic repetition. Integer repetition is non-recursive; automatic
repetition and every surrounding track use fixed sizes; and `grid-auto-rows` and
`grid-auto-columns` accept track sizes rather than `repeat()`. A conforming value
that uses newly typed calculation structure can therefore be current even when
its frozen I01 projection is absent.

Keyframe rules preserve authored structure rather than a merged animation
timeline. Empty rules and blocks remain present. Repeated selector blocks,
equivalent offsets in different blocks, and repeated equivalent selectors within
one list remain in source order without sorting, merging, or deduplication. When
an invalid declaration is dropped, its now-empty block and rule remain; an
invalid selector still drops the smallest invalid keyframe block. These recovery
observables replace older expectations that accepted structurally invalid Grid
cross-products or discarded valid empty keyframe parents.

The Grid repetition value, the six Grid property records, and the keyframe rule
record remain `Partial`. Subgrid name-repeat and other unselected Grid 2 property
grammar remain unsupported. Calculation keyframe selectors, string names, and
unselected declaration-processing grammar remain outside the keyframe boundary.
This crate does not perform Grid layout, cascade declarations, evaluate or
interpolate keyframes, run timelines, or lower either syntax family into sibling
Surgeist crates.

## Fonts 3 typography and font-face

The current authored font surface completes the sixteen Fonts 3 property
grammars, including family/global boundaries, checked four-ASCII-character
OpenType tags, non-negative feature indices, the explicit and system `font`
branches, synthesis, and the five variant longhands. Each generated property
wrapper exposes its typed current value through the property-specific accessor
while retaining `i01_subset()` as a separate compatibility projection. A current
value such as `font: menu` or `font-weight: 725` can be valid even when the frozen
I01 payload cannot represent it.

```rust
use surgeist_css::{
    CssFontValue, CssKnownPropertyValueRef, CssSystemFont,
    parse_style_attribute,
};

let report = parse_style_attribute("font: menu; font-weight: 725");
assert!(report.is_clean());
let CssKnownPropertyValueRef::Font(font) = report.syntax()[0]
    .known().expect("known font")
    .property_value().expect("ordinary font")
else { panic!("expected font") };
assert!(matches!(font.font(), CssFontValue::System(CssSystemFont::Menu)));
assert!(font.i01_subset().is_none());
```

`@font-face` retains every valid descriptor occurrence in authored order;
effective typed accessors return the last valid occurrence. Fonts 3 family,
source, weight, style, stretch, unicode-range, and feature-settings grammar is
catalogued separately from the selected Fonts 4 additions: `font-display`,
numeric property weight, descriptor weight/style/stretch ranges, and keyword
`format()`/`tech()` source hints. An invalid or unknown descriptor is dropped
with a `DropDescriptor` diagnostic without erasing valid neighbors. A rule is
retained only when a valid effective `font-family` and `src` remain, and parent
loss follows child diagnostics.

Fonts 3 rows cite the dated `O-FONTS3` source and are `Complete`. The five
selected atomic Fonts 4 deltas cite `I-FONTS4` and remain `Partial` with explicit
subset and remainder text; `font-display` is `Complete`, while
`@font-feature-values` remains `RecognizedUnsupported`. These authored models do
not load or match fonts, resolve fallback or feature application, shape glyphs,
apply cascade or substitution, evaluate computed values, expose CSSOM, serialize,
or lower into another Surgeist crate.

`CssImportance` and `CssSupportStatus` are deliberately closed and may be
matched exhaustively. Every other public enum is non-exhaustive and requires a
wildcard in downstream matches. This declaration inspection migration changes
neither accepted input nor parsing, recovery, or diagnostic behavior.

Each diagnostic exposes a typed error and stable root code, the first responsible source position, the complete recovery-unit span, and one `CssRecoveryAction`. Source byte offsets index the original UTF-8 input; line and column indices are zero-based, and columns count UTF-16 code units. Display text is for people, not control flow—match typed variants with a wildcard for future non-exhaustive cases.

CSS custom properties preserve case-sensitive names and authored value text, including interior trivia. Known-property values whose grammar depends on `var(...)` remain substitution-dependent authored values. The crate recognizes terminal `!important` but does not apply cascade or perform custom-property substitution or post-substitution validation.

The independent support catalog reports an exact support status for each declared conformance production: `Complete`, `Partial`, or `RecognizedUnsupported`. Partial records document both the accepted subset and valid-but-unsupported remainder. A clean use of a partial production's supported subset is accepted; status is metadata about the whole named production, not a parse-result validity flag.

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

Pseudo-classes for UI interaction, form state, structure, selector-list filtering, and overlay state are parsed as authored selector syntax. This crate does not evaluate pseudo-class matches; runtime matching belongs to downstream Surgeist layers with node and interaction state.

Selector-list pseudo-class arguments are parsed as authored selector syntax with bounded recovery. In recognized `:is()` and `:where()` lists, an invalid member is dropped with `DropSelectorListItem` while the other members remain in authored order. Other selector lists are unforgiving: `:not()` preserves supported complex selector lists, `:has()` preserves supported relative selector lists including leading child and sibling combinators, and `:nth-child()` / `:nth-last-child()` preserve optional `of` selector filters, but an invalid member causes the containing qualified rule to be dropped with `DropQualifiedRule`. Later sibling rules remain eligible for parsing.

## Namespaces and complete Selectors 3 syntax

`CssRule::Namespace` retains a top-level `@namespace` declaration with its
optional decoded, case-sensitive `CssNamespacePrefix`, literal
`CssNamespaceName`, and parser-produced position. Namespace names preserve the
authored string or `url()` token value, including empty strings and strings that
are not valid URIs. The crate does not normalize, resolve, or load the value.

Selector type, universal, and attribute names expose
`CssNamespaceConstraint`. `Named` contains an earlier active prefix;
`ExplicitNone` represents `|`; `Any` represents `*|`; and `Default` represents
an unqualified type or universal selector while a default declaration is
active. Without an active default, an unqualified type or universal selector is
`Any`. Unqualified attributes are always `ExplicitNone`. A
`CssQualifiedSelectorName` distinguishes an identifier returned by
`local_name()` from universal `*` reported by `is_universal()`.

```rust
use surgeist_css::{
    CssNamespaceConstraint, CssPseudoElement, CssRule, CssSelector, parse_sheet,
};

let report = parse_sheet(concat!(
    "@namespace svg \"urn:svg\";",
    "svg|a#first#second[|lang]::first-line { color: red; }",
));
assert!(report.is_clean());
let [CssRule::Namespace(namespace), CssRule::Style(style)] =
    report.syntax().rules()
else {
    panic!("expected namespace and style rules");
};
assert_eq!(namespace.prefix().expect("named prefix").as_str(), "svg");
assert_eq!(namespace.name().as_str(), "urn:svg");

let CssSelector::Compound(selector) = style.selector() else {
    panic!("expected compound selector");
};
let qualified = selector.type_selector().expect("qualified type selector");
assert!(matches!(
    qualified.namespace(),
    CssNamespaceConstraint::Named(prefix) if prefix.as_str() == "svg"
));
assert_eq!(qualified.local_name(), Some("a"));
assert_eq!(selector.ids(), ["first", "second"]);
assert_eq!(selector.key().map(String::as_str), Some("second"));
let [attribute] = selector.attributes() else {
    panic!("expected one attribute selector");
};
assert_eq!(attribute.namespace(), &CssNamespaceConstraint::ExplicitNone);
assert!(matches!(
    selector
        .pseudo_elements()
        .expect("pseudo-element sequence")
        .pseudo_elements(),
    [CssPseudoElement::FirstLine]
));
```

The top-level parser distinguishes six authored phases. `Initial` admits imports
and namespaces. An initial layer statement enters `InitialLayers`, which still
admits imports but permanently prohibits namespaces. An import enters `Imports`
from `Initial` or `ImportsAfterInitialLayers` from `InitialLayers`; only the
former admits namespaces. A namespace enters `Namespaces`, where only further
namespaces remain valid before a layer or body transition. A successful layer
after an import or namespace, or any successful body rule, enters `Body`.
Malformed, ignored, nested, or misplaced rules never change the phase or active
bindings.

Declarations and active bindings remain in authored order; the last declaration
for an exact named prefix or the default affects following selectors. An
undeclared named prefix invalidates its selector. Forgiving `:is()` and
`:where()` lists drop only that member with `DropSelectorListItem`; unforgiving
style, scope, nesting, `:not()`, `:has()`, and nth `of` consumers drop their
established containing unit. Malformed, block-form, nested, or late namespace
rules recover as one `DropAtRule` and leave later siblings eligible.

The authored selector model covers complete Selectors 3, including universal
and type selectors, all attribute matchers, repeated IDs and classes in order,
the structural/UI/dynamic pseudo-class families, `:lang()`, all four
combinators, and `::first-line`/`::first-letter`. The legacy single-colon
spellings for `before`, `after`, `first-line`, and `first-letter` map to the same
typed pseudo-elements. Selected extensions remain separately owned: attribute
`i`/`s`, the existing extension-state and functional pseudo-classes, nesting and
scope, and the marker/selection/backdrop pseudo-element rows. Matching,
specificity, cascade, namespace URI resolution, CSSOM serialization, and
cross-crate lowering remain downstream exclusions.

## Counter Styles 3 and CSS2 page rules

`CssRule::CounterStyle` retains a checked, case-sensitive
`CssCounterStyleName`, the parser-produced rule position, and typed
`CssCounterStyleDescriptors`. Every valid descriptor occurrence remains in
authored order; the named `system`, `negative`, `prefix`, `suffix`, `range`,
`pad`, `fallback`, `symbols`, `additive_symbols`, and `speak_as` accessors select
the effective last valid occurrence. The model preserves symbolic
`extends` names, infinite range bounds, nonempty symbol lists, and strictly
descending additive weights without registering, resolving, inheriting, or
evaluating a counter style.

An invalid or unknown counter-style descriptor is dropped individually with a
typed `DropDescriptor` diagnostic, preserving valid neighboring descriptors.
An invalid effective combination, such as `system: extends` with an authored
`symbols` definition, drops the complete at-rule. Counter-style rules are
top-level block rules; malformed preludes, statement forms, and nested placement
drop the smallest established at-rule unit and leave later siblings eligible.

`CssRule::Page` retains the default page form or one of the finite
`CssPageSelector::{Left, Right, First}` choices, valid declarations in authored
order, and the parser-produced position. Page bodies accept only `margin` and
the four margin longhands with CSS2 lengths other than `em` and `ex`,
percentages, `auto`, zero, and negative values. Known non-margin, unknown, and
invalid margin declarations receive their existing typed declaration
diagnostics and are dropped individually. Page rules are likewise top-level and
block-form only; margin-box nested at-rules remain unsupported.

```rust
use surgeist_css::{CssCounterStyleSystem, CssPageSelector, CssRule, parse_sheet};

let report = parse_sheet(concat!(
    "@counter-style digits { system: numeric; symbols: \"0\" \"1\"; suffix: \".\"; } ",
    "@page :left { margin-left: -12mm; margin-right: 10%; }",
));
assert!(report.is_clean());
let [CssRule::CounterStyle(counter), CssRule::Page(page)] = report.syntax().rules() else {
    panic!("expected counter-style and page rules");
};
assert_eq!(counter.name().as_str(), "digits");
assert!(matches!(
    counter.descriptors().system().map(|value| value.value()),
    Some(CssCounterStyleSystem::Numeric)
));
assert_eq!(counter.descriptors().occurrences().count(), 3);
assert_eq!(page.selector(), Some(CssPageSelector::Left));
assert_eq!(page.declarations().len(), 2);
```

All sixteen Counter Styles 3 non-property rows and the two CSS2 page rows are
public `Complete` atomic metadata with their dated official source fragments.
They have no partial remainder, recognized-unsupported code, or aggregate-alias
targets. The crate does not paginate, match page selectors, apply page cascade,
render generated markers, resolve counter inheritance, expose CSSOM, or lower
these authored models into another Surgeist crate.

## CSS2 residual, writing, UI, containment, and compositing properties

The C12 property family adds complete authored grammars for thirteen CSS2
residual properties, Writing Modes 3 `text-combine-upright`,
`text-orientation`, and `unicode-bidi`, UI3 `caret-color`, `outline-offset`, and
`resize`, Containment 1 `contain`, Transforms 1 `transform-box`, and Compositing
1 `background-blend-mode`, `isolation`, and `mix-blend-mode`. Their property
wrappers preserve exact authored CSS and expose typed current values without
performing cascade, layout, pagination, painting, hit testing, containment
semantics, blending, or writing-mode resolution.

`glyph-orientation-vertical` is the selected Writing Modes legacy shorthand,
not a name-equivalent schema alias. Its restricted `auto`, `0`, `0deg`, `90`,
and `90deg` grammar maps to a parser-produced `text-orientation` declaration.
The schema therefore keeps `CssKnownProperty::TextOrientation.aliases()` empty,
while the conformance catalog exposes the explicit
`official.property-alias.glyph-orientation-vertical` record.

```rust
use surgeist_css::{
    CssBlendMode, CssFeatureKind, CssKnownProperty, CssKnownPropertyValueRef,
    CssSupportStatus, feature_metadata, parse_style_attribute,
};

let report = parse_style_attribute(concat!(
    "border-spacing: 2px 3px; ",
    "glyph-orientation-vertical: 90; ",
    "background-blend-mode: multiply, luminosity",
));
assert!(report.is_clean());
assert_eq!(
    report.syntax()[1].known().expect("legacy shorthand").property(),
    CssKnownProperty::TextOrientation,
);
let CssKnownPropertyValueRef::BackgroundBlendMode(blending) = report.syntax()[2]
    .known().expect("known blending property")
    .property_value().expect("ordinary value")
else { panic!("expected background blend modes") };
assert_eq!(
    blending.modes().modes(),
    &[CssBlendMode::Multiply, CssBlendMode::Luminosity],
);
let alias = feature_metadata("official.property-alias.glyph-orientation-vertical")
    .expect("legacy alias metadata");
assert_eq!(alias.kind(), CssFeatureKind::PropertyAlias);
assert_eq!(alias.status(), CssSupportStatus::Complete);
```

Exactly 27 C12 official rows are public `Complete` atomic records: 24 canonical
properties, the explicit legacy shorthand, and the independent
`official.value.box-edge-keywords` and `official.value.blend-mode` shared-value
records. This activation does not inflate the immutable ledger or promote later
work: it remains 162 property units (161 canonical properties plus the custom
property family), one normative legacy shorthand, and 167 non-property units.
The unchanged 131-row exclusion registry still includes exactly 50 superseded
CSS2 property definitions, 20 informative CSS2 Appendix A properties, and the
two current-production-less `glyph-orientation-horizontal` and `ime-mode`
spellings; the remaining exclusions cover exact non-property or downstream
source areas.

## Backgrounds, border images, and gradients

Backgrounds 3 and Images 3 values remain authored and symbolic. Background
shorthands preserve layer order, per-layer position/size coupling, repeats,
attachments, boxes, and a final-layer color. Image values distinguish `none`,
URLs, and typed linear, radial, and repeating gradients. Border-image values
preserve their source, slice, width, outset, and repeat components without
loading an image or resolving any geometry.

```rust
use surgeist_css::{
    CssGradient, CssImageValue, CssKnownPropertyValueRef, CssSupportStatus,
    feature_metadata, parse_style_attribute,
};

let report = parse_style_attribute(concat!(
    "background-image: linear-gradient(to right, red 0%, 40%, blue); ",
    "border-image: url(frame.png) 10 fill / 2 / 1 round",
));
assert!(report.is_clean(), "{:?}", report.diagnostics());

let CssKnownPropertyValueRef::BackgroundImage(images) = report.syntax()[0]
    .known().expect("known background image")
    .property_value().expect("ordinary background image")
else { panic!("expected background-image") };
assert!(matches!(
    images.images().images(),
    [CssImageValue::Gradient(CssGradient::Linear(_))]
));

let CssKnownPropertyValueRef::BorderImage(border) = report.syntax()[1]
    .known().expect("known border image")
    .property_value().expect("ordinary border image")
else { panic!("expected border-image") };
assert!(border.border_image().slice().expect("slice").fill());

let gradient = feature_metadata("official.value.linear-gradient")
    .expect("linear-gradient metadata");
assert_eq!(gradient.source().id().as_str(), "O-IMAGES3");
assert_eq!(gradient.status(), CssSupportStatus::Complete);
```

C13 activates exactly 27 public `Complete` atomic records: nine properties and
eighteen shared values. It also promotes the existing complete Backgrounds 3
property grammars that previously carried Partial metadata. The already-Complete
`background-position`, `object-position`, and `box-shadow` rows remain Complete.
The public support catalog consequently contains 456 records. Activation and
promotion do not add official ledger units: the inventory remains 162 property
units (161 canonical properties plus the custom-property family), one normative
legacy shorthand, and 167 non-property units.

These models do not fetch or decode images, resolve URLs, apply cascade or
substitution, compute background or border geometry, paint, serialize CSSOM, or
lower values into another Surgeist crate.

## Flexbox, multicolumn, and official grammar closure

Flexbox 1 `flex-flow` and all nine Multicolumn 1 properties now expose complete
authored grammars and typed current values. `flex-flow` preserves the authored
direction/wrap combination; `columns` preserves its independently optional
width and count; and `column-rule` preserves width, style, and color without
performing layout, pagination, or painting.

```rust
use surgeist_css::{
    CssColumnCount, CssFlexDirection, CssKnownPropertyValueRef, CssSupportStatus,
    feature_metadata, parse_style_attribute,
};

let report = parse_style_attribute("flex-flow: column wrap; columns: 3 12em");
assert!(report.is_clean(), "{:?}", report.diagnostics());

let CssKnownPropertyValueRef::FlexFlow(flow) = report.syntax()[0]
    .known().expect("known flex-flow")
    .property_value().expect("ordinary flex-flow")
else { panic!("expected flex-flow") };
assert_eq!(flow.flow().direction(), CssFlexDirection::Column);

let CssKnownPropertyValueRef::Columns(columns) = report.syntax()[1]
    .known().expect("known columns")
    .property_value().expect("ordinary columns")
else { panic!("expected columns") };
assert!(matches!(columns.columns().count(), CssColumnCount::Count(_)));

let metadata = feature_metadata("official.property.flex-flow")
    .expect("public Flexbox metadata");
assert_eq!(metadata.source().id().as_str(), "O-FLEXBOX1");
assert_eq!(metadata.status(), CssSupportStatus::Complete);

let shared = feature_metadata("official.value.syntax-token-stream")
    .expect("public Syntax 3 value metadata");
assert_eq!(shared.status(), CssSupportStatus::Complete);

let extension = feature_metadata("ext.value.relative-color")
    .expect("preserved extension metadata");
assert_eq!(extension.status(), CssSupportStatus::Partial);
assert!(extension.supported_subset().is_some());
assert!(extension.unsupported_remainder().is_some());

let unsupported = feature_metadata("later.rule.font-feature-values")
    .expect("recognized unsupported rule metadata");
assert_eq!(
    unsupported.status(),
    CssSupportStatus::RecognizedUnsupported,
);
```

The generic Syntax 3 at-rule, qualified-rule, declaration, stylesheet,
rule-list, declaration-list, and style-block records become public `Complete`
atomic metadata. C14 also makes these fourteen formerly `Reserved` shared
values public `Complete` metadata: `syntax-token-stream`, `component-value`,
`simple-block`, `function`, `declaration-value`, `any-value`, `an-plus-b`,
`unicode-range`, `css-wide-keyword`, `custom-ident`, `ident`, `string`, `url`,
and `url-modifier`.

Together, the ten Flexbox/Multicolumn property records, seven generic shell
records, and fourteen shared-value records are the 31 public catalog additions
that entered C14 as `Reserved`. C14 separately promotes the seven Values 3
records for `dimension`, `angle`, `angle-percentage`, `time-percentage`,
`frequency`, `frequency-percentage`, and `calc()` from `Partial` to `Complete`.

The preserved extension records `ext.value.relative-color`,
`ext.value.color-mix`, `ext.value.grid-repeat`, `ext.value.basic-shape`,
`ext.descriptor.font-weight-range`, `ext.descriptor.font-style-oblique-range`,
`ext.descriptor.font-stretch-range`, `ext.value.font-source-modern-hints`,
`ext.property.font-weight-range`, `ext.supports.selector`,
`ext.media.range.width`, `ext.media.range.height`,
`ext.media.range.resolution`, `ext.media.range.color`, and
`ext.media.range.monochrome` remain `Partial`, with both subset and remainder
metadata. The `@font-feature-values` rule remains `RecognizedUnsupported` with
its typed unsupported-at-rule diagnostic.

The C13 public support catalog contained exactly 456 records. C14 adds the 31
records above, so the public support catalog contains exactly 487 records. That
catalog cardinality is distinct from the immutable official inventory of
exactly 162 property units (161 canonical properties plus the custom-property
family), one normative legacy shorthand, and 167 non-property units. All 219
preserved I01 baseline records retain their classifications, and the exclusion
registry remains exactly 131 rows.

The crate still stops at strict authored syntax. Cascade, substitution,
selector matching, query evaluation, resource loading, layout, pagination,
painting, serialization, and cross-crate lowering remain downstream concerns.

## Media, supports, and import preludes

Media Queries 3 types and features are retained as authored query syntax. A
balanced unknown type, feature, or feature value is defined-false syntax: it is
preserved with its exact authored text and emits no diagnostic. This is distinct
from `CssMediaQuery::Never`, which replaces a reserved or structurally malformed
comma member and is paired with `ReplaceMediaQueryWithNever`. The replacement is
comma-local, so later query members and the containing `@media` rule remain
eligible.

```rust
use surgeist_css::{
    CssMediaConditionKind, CssMediaQuery, CssRecoveryAction, CssRule, parse_sheet,
};

let report = parse_sheet("@media (future-mode: active), ???, print {}");
let [CssRule::Media(media)] = report.syntax().rules() else {
    panic!("expected retained media rule");
};
assert!(matches!(
    media.query().queries(),
    [
        CssMediaQuery::Condition(condition),
        CssMediaQuery::Never(_),
        CssMediaQuery::Typed(_),
    ] if matches!(condition.kind(), CssMediaConditionKind::DefinedFalse(_))
));
assert_eq!(
    report.diagnostics()[0].action(),
    CssRecoveryAction::ReplaceMediaQueryWithNever,
);
```

`@supports` conditions expose declaration tests, `not`/`and`/`or` grouping,
complete Selectors 3 plus the selected existing selector extensions as the typed
`selector()` subset, and exact balanced general-enclosed fallback syntax. The
typed subset does not include `||`, unselected Selectors 4 pseudo-classes or
pseudo-elements, or syntax outside the named extension rows. Declaration tests
preserve authored property/value text and importance;
their optional known-declaration view is inspection data, not a declaration
inserted into a style block. Invalid children recover within a valid conditional
parent, while a malformed supports prelude drops that parent and leaves later
siblings eligible.

```rust
use surgeist_css::{CssRule, CssSupportsConditionKind, parse_sheet};

let report = parse_sheet(concat!(
    "@supports (display: grid) and (color: red) {}",
    "@supports selector(.card > .item:hover) {}",
    "@supports future-layout(mode) {}",
));
assert!(report.is_clean());
let [
    CssRule::Supports(declarations),
    CssRule::Supports(selector),
    CssRule::Supports(fallback),
] = report.syntax().rules()
else {
    panic!("expected supports rules");
};
assert!(matches!(
    declarations.condition().kind(),
    CssSupportsConditionKind::And(_)
));
assert!(matches!(
    selector.condition().kind(),
    CssSupportsConditionKind::Selector(_)
));
assert!(matches!(
    fallback.condition().kind(),
    CssSupportsConditionKind::GeneralEnclosed(value)
        if value.authored() == "future-layout(mode)"
));
```

An `@import` prelude is retained in exact target, optional `layer` or
`layer(name)`, optional `supports(...)`, optional media-list order. A successful
initial layer statement permits a following import. Once an import is followed
by another layer statement, a namespace phase, or a body rule, later imports are
invalid; only successful rules advance the phase. Duplicated, swapped, or
trailing import clauses drop that import without preventing later siblings from
being parsed.

These models are authored syntax only. `surgeist-css` does not evaluate media or
supports conditions, match selectors, resolve URLs, load imported resources,
apply cascade or substitution, compute layer order, or lower syntax into root or
sibling types. Environment matching, resource loading, composition, and
cross-crate adapters remain downstream responsibilities.

Container queries are parsed as authored conditions on `@container` group rules. `surgeist-css` does not evaluate container query matches; container-dependent matching belongs to downstream Surgeist layers.

Imports are parsed as authored `@import` contracts only. `surgeist-css` preserves import targets, layer clauses, supports conditions, and media conditions, but does not resolve paths, load files, or merge imported sheets; root/style-owned Surgeist integration performs loading and composition.

Cascade layers are parsed as authored `@layer` statements and blocks, including named and anonymous layer blocks. `surgeist-css` records layer names and layer-contained rules, but does not compute cascade order, declaration precedence, or runtime cascade effects.

Scoped styles are parsed as authored `@scope` rules with optional roots, limits, scoped style selectors, and scoped nested group rules. Relative scoped selectors remain structurally distinct from ordinary selectors. `surgeist-css` does not perform scope matching, selector matching, or scoping proximity calculations.

Pseudo-elements are parsed as terminal authored selector syntax for the supported `::before`, `::after`, `::first-line`, `::first-letter`, `::marker`, `::selection`, and `::backdrop` forms. The Selectors 3 legacy single-colon spellings map to the same typed before, after, first-line, and first-letter values. The parser records pseudo-elements on selector compounds, but does not filter declarations by pseudo-element or perform generated box/layout behavior.

Generated content, list markers, and counters are parsed as typed authored property values for `content`, list-style longhands and shorthand, and counter change properties. Strings, URLs, attribute references, quote keywords, counter functions, list-style slots, and counter change lists remain symbolic. `surgeist-css` does not lay out generated content or list markers, resolve marker images, or evaluate/reset/increment counters.

Font faces are parsed as authored `@font-face` descriptor blocks only. `surgeist-css` validates supported descriptors and preserves font source hints, unicode ranges, and variation ranges, but does not perform font lookup, loading, matching, or resource validation; downstream Surgeist layers own those steps.

Keyframes are parsed as authored `@keyframes` rules. `surgeist-css` validates keyframe names, selector offsets, and declarations, but does not evaluate animations, match animation names to rules, interpolate values, or run animation timelines.

CSS nesting is parsed as syntax sugar and flattened into ordinary style and conditional group rules while preserving source order. `surgeist-css` does not evaluate selector matches or cascade results during flattening.

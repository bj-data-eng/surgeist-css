# P01-I02-C10 Namespaces And Complete Selectors 3

## Boundary

`surgeist-css` owns strict authored namespace declarations, namespace-qualified
selector names, complete Selectors 3 syntax, selector recovery, and truthful
conformance metadata. It does not resolve namespace URIs, match selectors,
calculate specificity, apply cascade or substitution, load resources, expose a
mutable CSSOM, serialize CSS, or lower syntax into another Surgeist crate.

## Namespace Rules And Prelude Phases

`CssRule::Namespace` exposes a private-field `CssNamespaceRule` with optional
decoded `CssNamespacePrefix`, literal `CssNamespaceName`, and parser-produced
position accessors. Prefixes are case-sensitive CSS identifiers. Names preserve
the string or `url()` token value literally; empty and non-URI strings remain
valid authored values.

The top-level phase machine is `Initial`, `InitialLayers`, `Imports`,
`ImportsAfterInitialLayers`, `Namespaces`, and `Body`. Initial layers still
permit imports but permanently prohibit namespaces. Only `Initial` and
`Imports` admit a namespace. Consecutive namespace declarations remain valid
until a successful layer or body rule enters `Body`. Only successful rules
advance the phase or active bindings.

Declarations remain in authored order. The last declaration for the exact
case-sensitive named prefix or the default is active for following selectors.
Malformed, block-form, nested, and late namespace rules recover as one
`DropAtRule`; invalid rules do not change the phase or namespace environment.

## Namespace-Aware Selector Surface

`CssNamespaceConstraint` exposes `Default`, `ExplicitNone`, `Any`, and
`Named(CssNamespacePrefix)`. `CssQualifiedSelectorName` distinguishes a local
identifier from universal `*` through `local_name()` and `is_universal()`.
`CssCompoundSelector::type_selector()` exposes the qualified model and `ids()`
preserves every ID in authored order. The existing `tag()` and last-ID `key()`
views remain compatibility projections. Attribute selectors retain their local
`name()` and add a namespace accessor.

An active default applies only to unqualified type and universal selectors.
Without one, those selectors use `Any`; unqualified attributes always use
`ExplicitNone`. `*|` is `Any`, `|` is `ExplicitNone`, and a declared exact
prefix is `Named`. Namespace bindings reach top-level and nested style rules,
conditional and layer groups, containers, scope boundaries, selector-list
pseudo-classes, nesting, and `@supports selector()`.

An undeclared prefix invalidates its selector. Forgiving `:is()` and `:where()`
lists drop that member with `DropSelectorListItem`; unforgiving style, scope,
nesting, `:not()`, `:has()`, and nth `of` consumers preserve their established
whole-unit recovery. Later valid siblings remain eligible.

## Complete Selectors 3 And Selected Extensions

The authored model covers type and universal selectors, all attribute matchers,
ordered repeated IDs and classes, all four combinators, the complete Selectors 3
structural/UI/dynamic pseudo-class families, `:link`, `:visited`, `:target`,
checked `:lang()`, and first-line/first-letter pseudo-elements. Legacy
single-colon `before`, `after`, `first-line`, and `first-letter` map to the same
typed pseudo-elements as their double-colon forms.

Selected extensions keep their existing ownership: attribute `i`/`s` modifiers;
the extension-state pseudo-classes; `:is()`, `:where()`, `:has()`, selector-list
`:not()`, and nth-child `of`; nesting and scope; and the marker, selection,
backdrop, and generated-marker pseudo-element rows. Pseudo-elements remain
terminal under the existing selected generated-marker sequence.

## Official Metadata

Every row below is a `Complete` atomic public feature with no subset, remainder,
recognized-unsupported diagnostic, or aggregate-alias targets.

| Stable ID | Source | Exact fragment |
| --- | --- | --- |
| `official.selector.group` | `O-SELECTORS3` | `#grouping` |
| `official.selector.type` | `O-SELECTORS3` | `#type-selectors` |
| `official.selector.universal` | `O-SELECTORS3` | `#universal-selector` |
| `official.selector.attribute-presence-value` | `O-SELECTORS3` | `#attribute-representation` |
| `official.selector.attribute-substring` | `O-SELECTORS3` | `#attribute-substrings` |
| `official.selector.class` | `O-SELECTORS3` | `#class-html` |
| `official.selector.id` | `O-SELECTORS3` | `#id-selectors` |
| `official.selector.dynamic` | `O-SELECTORS3` | `#dynamic-pseudos` |
| `official.selector.target` | `O-SELECTORS3` | `#target-pseudo` |
| `official.selector.lang` | `O-SELECTORS3` | `#lang-pseudo` |
| `official.selector.ui-state` | `O-SELECTORS3` | `#UIstates` |
| `official.selector.structural` | `O-SELECTORS3` | `#structural-pseudos` |
| `official.selector.negation` | `O-SELECTORS3` | `#negation` |
| `official.selector.first-line` | `O-SELECTORS3` | `#first-line` |
| `official.selector.first-letter` | `O-SELECTORS3` | `#first-letter` |
| `official.selector.generated` | `O-SELECTORS3` | `#gen-content` |
| `official.selector.combinator.descendant` | `O-SELECTORS3` | `#descendant-combinators` |
| `official.selector.combinator.child` | `O-SELECTORS3` | `#child-combinators` |
| `official.selector.combinator.next-sibling` | `O-SELECTORS3` | `#adjacent-sibling-combinators` |
| `official.selector.combinator.subsequent-sibling` | `O-SELECTORS3` | `#general-sibling-combinators` |
| `later.rule.namespace` | `O-NAMESPACES3` | `#declaration,#syntax` |
| `official.selector.namespace-qualified-name` | `O-NAMESPACES3` | `#scope,#prefixes,#css-qnames` |

The preserved selector aggregate aliases remain `Partial` and continue to point
at their exact atomic targets. Namespace rules have no unsupported diagnostic.

## Partial Supports Selector Boundary

`ext.supports.selector` remains `Partial` at
`R-CONDITIONAL4#at-supports`. Its typed subset is complete Selectors 3 plus the
selected extension rows named above. The `||` combinator, unselected Selectors 4
pseudo-classes and pseudo-elements, and syntax outside those atomic extension
rows remain outside the typed subset. Balanced remainder content is retained as
`CssSupportsConditionKind::GeneralEnclosed` without a recovery diagnostic. This
is not a complete Selectors 4 claim.

## Source-Backed Product Fixture Delta

The product fixture before this cycle has SHA-256
`95518fbabb04cd5b96bc9505a4d96681d444042498d681f28b3db4f3d8a2f0d3`.
The source-backed replacement product fixture has SHA-256
`96be045dc181fe5fc258e76b09458b441139504a3cae13c41897995ab3ae8f5d`.

Exactly these six stable rows change:

- `catalog.non-property.later.rule.namespace.boundary` becomes clean and
  retains `rule:later.rule.namespace`;
- `catalog.non-property.baseline.selector.extension-state.boundary` becomes
  clean and retains its style rule and red color declaration;
- `catalog.non-property.baseline.selector.functional.boundary` receives the
  same clean style/color observables;
- `catalog.non-property.baseline.selector.pseudo-class.boundary` receives the
  same clean style/color observables;
- `catalog.non-property.baseline.selector.pseudo-element.boundary` receives the
  same clean style/color observables; and
- `focused.stylesheet-recovery.11` keeps both surrounding style rules and
  declarations, authored values, `DropAtRule`, position, and span, while its
  namespace diagnostic becomes `InvalidAtRulePlacement` with expected context
  `after imports and before every layer or body rule`.

Every other fixture row remains byte-identical.

## Root-Owned Follow-Up

Root `surgeist` owns the integration work:

1. select and promote the intended `surgeist-css` gitlink;
2. expose the namespace, qualified-selector, pseudo-class, pseudo-element, and
   conformance metadata additions through the facade;
3. update root-owned adapters without moving selector matching, specificity,
   namespace resolution, cascade, resource loading, or CSSOM behavior into this
   leaf;
4. refresh root-owned generated API audit artifacts with the root generator;
5. update root integration tests and documentation for the refined phases,
   namespace constraints, recovery distinctions, complete Selectors 3 surface,
   and retained Partial supports-selector boundary.

Root owns facade composition, cross-crate adapters, integration tests and docs,
the gitlink, and generated API artifacts. This leaf handoff does not authorize
changes in root or sibling repositories.

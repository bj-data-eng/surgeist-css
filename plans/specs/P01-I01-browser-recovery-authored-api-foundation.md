# P01-I01 Browser Recovery And Authored API Foundation

## 1. Authority And Outcome

This is the JIT initiative specification for `P01-I01` in
`surgeist-css`. It is governed by
`plans/specs/P01-css-syntax-conformance-program.md`, normalized SHA-256
`6f45b5e60e47960a28d0d292c71d2d1c172cb0817129e0453a6a2c7638600b2e`.
P01 owns the program split and later-initiative boundary; this document owns the
complete desired state for I01 only.

I01 shall replace the strict whole-sheet front door with a browser-recovery
authored-syntax front door for stylesheets and style attributes. Each ordinary
parse returns a valid retained tree and all typed recovery diagnostics in source
order. An additive `app-strict` feature shall validate by running that same
parser once and rejecting a non-clean report. The initiative also establishes
the property-coupled declaration model, source-coordinate convention, recovery
span model, and independent support catalog that I02 can extend without changing
their meaning.

I01 is intentionally breaking. It does not claim complete CSS Snapshot 2026
grammar. At completion, every parser-facing production in the I01 catalog is
truthfully `Complete`, `Partial`, or `RecognizedUnsupported` at an exact bounded
production, and every unsupported or invalid source unit encountered by an
ordinary parser produces a typed diagnostic. A clean report means that every
input unit fell inside the accepted subset of a `Complete` or `Partial`
production and required no recovery. Accepted subsets of partial productions are
therefore clean and accepted by `app-strict`; the metadata status still states
that the broader named production is incomplete.

## 2. Baseline Evidence And Normative Inputs

The implementation baseline is leaf commit
`4b288d6467d91f2fc33eac78ef0b0b725154195d`. Its manifest pins
`cssparser = 0.37.0` and `cssparser-color = 0.5.0`, exposes only strict
`parse_sheet`, stores declarations as an independent `CssProperty`/`CssValue`
pair, exposes mixed raw line/column locations, and has no Cargo features or
tracked public integration-test directory.

The historical review at
`plans/P01-implement-full-css-spec/P01-css-snapshot-2026-review.md`, SHA-256
`5ddd3eebb4fc3664759021605d3884a0c795947e0ef4e427d3dfc5e77469199d`,
is scoped to source commit
`318864d1074d8d723a3a925528343c8a3d8c7253`. P01 allocates findings 2.5,
2.6, 2.15, and 2.18 through 2.25 to I01. The deleted legacy remediation
specification at baseline commit `4b288d6`, SHA-256
`2cdf3d1536e913c539d5f63c82889e353350baaf87ddd4796867d6eb0b89dbba`,
is design evidence only.

These immutable sources decide I01 behavior:

| Source | Pinned revision | I01-owned use |
| --- | --- | --- |
| CSS Syntax 3 | https://www.w3.org/TR/2021/CRD-css-syntax-3-20211224/ | tokenization, rule/declaration consumption, error recovery, CDO/CDC, EOF closure, encoding-declaration recognition |
| CSS Style Attributes | https://www.w3.org/TR/2013/REC-css-style-attr-20131107/ | style-attribute declaration-list front door |
| CSS Cascade 4 | https://www.w3.org/TR/2022/CR-css-cascade-4-20220113/ | CSS-wide keywords and declaration importance syntax only |
| CSS Variables 1 | https://www.w3.org/TR/2022/CR-css-variables-1-20220616/ | custom-property authored values and substitution-dependent value preservation only |
| Media Queries 3 | https://www.w3.org/TR/2024/REC-mediaqueries-3-20240521/ | malformed query-list member recovery |
| Selectors 4 | https://www.w3.org/TR/2026/WD-selectors-4-20260122/ | forgiving list recovery for the already-recognized `:is()` and `:where()` extensions only |
| Baseline source | `4b288d6:src/`, `README.md`, and `Cargo.toml` | finite grammar spellings and later extensions that I01 must classify truthfully |

The dated sources define recovery and the named foundation syntax only. They do
not pull their complete property, selector, rule, media, or value inventories
into I01. Moving standards aliases and editor drafts are discovery aids, not
I01 conformance inputs.

## 3. Ownership And Non-Goals

`surgeist-css` owns authored CSS syntax, intrinsic lexical and grammar validity,
CSS recovery boundaries, retained authored source positions, recovery
diagnostics, the I01 support catalog, focused tests and documentation, and the
published leaf candidate.

I01 excludes:

- CSS Snapshot 2026 grammar closure, complete official source inventory, and
  property-by-property conformance completion, which belong to I02;
- the CSSTree corpus, fixture adapters, generated expectations, and generator
  execution, which belong to I03 and `surgeist-generator`;
- cascade application, inheritance application, shorthand application,
  custom-property substitution, dependency-cycle resolution, and
  post-substitution validation;
- query evaluation, selector matching, specificity application, pseudo-state,
  scope proximity, URL resolution, resource loading, unit resolution, layout,
  painting, color conversion, interpolation, and renderer behavior;
- source-preserving serialization, a CSSOM mutation API, and a public generic
  token or fragment parser;
- root adapters, root integration tests, root API artifacts, root gitlinks, and
  any sibling source or metadata;
- a new dependency, build script, external executable, corpus copy, CI rule,
  policy mirror, or leaf-owned generated API artifact.

I01 retains every baseline-recognized production selected in section 9.2. If
one cannot be bounded truthfully, implementation stops for specification
reconciliation; it may not retire the production, silently accept a broader
grammar, or classify a partially implemented production as complete.

## 4. Public Front Door And Compatibility

The crate root shall contain `#![forbid(unsafe_code)]`, keep implementation
modules private, and reexport only public authored syntax, diagnostics, source
types, reports, validation types, and support metadata.

The ordinary front door is exactly:

```rust
pub fn parse_sheet(input: &str) -> CssParseReport<CssSheet>;
pub fn parse_style_attribute(input: &str) -> CssParseReport<CssDeclarationList>;
```

Neither function returns `Result`, invokes a callback, logs, panics for ordinary
`&str` input, or hides diagnostics. `CssParseReport<T>` has private fields and
exposes:

```rust
pub fn syntax(&self) -> &T;
pub fn diagnostics(&self) -> &[CssRecoveryDiagnostic];
pub fn is_clean(&self) -> bool;
pub fn into_parts(self) -> (T, Vec<CssRecoveryDiagnostic>);
```

`is_clean()` is exactly `diagnostics().is_empty()`. The report has no method
named `is_valid`; retained syntax is valid by construction even when the source
required recovery.

The manifest shall add exactly this feature shape:

```toml
[features]
default = []
app-strict = []
```

With `app-strict` enabled, the crate additionally exposes:

```rust
pub fn validate_sheet(input: &str)
    -> Result<CssSheet, CssValidationFailure>;
pub fn validate_style_attribute(input: &str)
    -> Result<CssDeclarationList, CssValidationFailure>;
```

Each validator calls its ordinary parser exactly once. It returns the syntax for
a clean report and otherwise returns `CssValidationFailure`, whose private,
non-empty diagnostic vector is exposed by `diagnostics()`, `first()`, and
`into_diagnostics()`. Enabling the feature changes no ordinary function,
retained tree, diagnostic, span, action, or ordering.

The following baseline contracts are removed or changed without compatibility
aliases:

- `parse_sheet(&str) -> Result<CssSheet>` becomes the report API above;
- `parse_style_attribute` is new;
- the public `Result<T>` alias is no longer an ordinary parser contract;
- `CssSourceLocation`, `location()`, and raw declaration `line()`/`column()`
  accessors become the source model in section 7;
- the independent `CssProperty` plus `CssValue` declaration representation
  becomes the coupled model in section 8;
- public parser-produced constructors and mutable construction paths are
  removed;
- public extensible enums become `#[non_exhaustive]` as specified in section 5.

The leaf candidate handoff shall enumerate the final removed and renamed public
items and instruct root to migrate adapters and regenerate root-owned API
artifacts after selecting the immutable candidate. I01 itself changes no root
file.

## 5. Authored Model Invariants And Evolution

The phases are explicit:

```text
UTF-8 source
  -> cssparser tokenization and bounded consumption
  -> CSS-owned recovery decision
  -> validated retained authored syntax + ordered diagnostics
  -> optional application-strict acceptance
```

`cssparser` owns safe tokenization, source cursors, delimited parsing, balanced
consumption, and iterator recovery opportunities. It does not own the public
error taxonomy, recovery action, full recovery span, diagnostic ordering,
parent-retention decision, support status, or CSS-owned syntax model.

Every parser-produced field is private. Public construction is limited to
context-free semantic scalar types whose complete invariant can be checked at
the boundary; fallible constructors return `Option` or a typed error. There is
no public unchecked constructor, `new_unchecked`, raw-invalid node, placeholder
default for a non-empty grammar, mutable slice, or independently constructible
property/value pair. Source positions on parser-produced nodes cannot be forged
through public constructors.

Every ordered collection wrapper exposes `as_slice()`, `iter()`, `len()`, and
`is_empty()` over one semantic element type. A grammar requiring a non-empty
list has no public empty constructor. Empty sheets, style attributes,
declaration lists, keyframe blocks, and descriptor lists remain representable
where authored grammar permits them; downstream effectiveness is not encoded as
syntax rejection.

All public enums are `#[non_exhaustive]` except this closed allowlist:

- `CssImportance::{Normal, Important}`;
- `CssSupportStatus::{Complete, Partial, RecognizedUnsupported}`.

Closed enums represent deliberately complete state sets. Downstream examples
and public tests use wildcard-compatible matches for every other enum. I02 may
add variants and private-field types, but may not reinterpret an I01 variant,
status, action, coordinate, or parser function.

`CssSheet` exposes `encoding() -> Option<&CssEncodingDeclaration>` and
`rules() -> &[CssRule]`. `CssRule` remains the non-exhaustive union of the nine
rule families retained by I01 and exposes `position()`. Every rule payload,
declaration, descriptor occurrence, and keyframe block exposes
`position() -> CssSourcePosition`; ordered child accessors return borrowed
validated semantic values. I01 may rename or consolidate baseline duplicate
wrapper types, but it may not expose dependency tokens or mutable child
collections as the replacement surface.

No input-driven path uses `unwrap`, `expect`, unchecked indexing, an
`unreachable!` assumption over dependency output, or recursion without the
limit in section 6. Allocation failure and process abort are outside the Rust
unwinding contract; all ordinary `&str` input is otherwise panic-free.

## 6. Browser-Recovery Behavior

### 6.1 Recovery Units

The parsers use `cssparser` recovery iterators and balanced consumption but make
one CSS-owned decision for every failure:

| Failure context | Retained result | Diagnostic action and span |
| --- | --- | --- |
| Unknown, recognized-unsupported, misplaced, or malformed top-level/nested at-rule | No node for that at-rule; later siblings remain eligible | `DropAtRule`; span covers the complete semicolon-terminated or balanced at-rule |
| Invalid selector list or malformed qualified-rule structure | No node or declarations from the rule; later siblings remain eligible | `DropQualifiedRule`; span covers the complete balanced qualified rule |
| Unknown/unsupported property, invalid value/annotation, or malformed declaration | No declaration; containing declaration list and later declarations remain eligible | `DropDeclaration`; span ends at its top-level semicolon or containing block end |
| Unknown/unsupported/invalid descriptor | No descriptor; the at-rule retains every other valid descriptor | `DropDescriptor`; span covers exactly the descriptor recovery unit |
| Invalid keyframe selector or malformed keyframe block | No node or declarations from that block; later blocks remain eligible | `DropKeyframeBlock`; span covers the complete balanced block |
| Invalid member of an already-recognized `:is()` or `:where()` forgiving selector list | Other members remain in authored order, including a grammar-permitted empty result | `DropSelectorListItem`; span covers the comma-delimited member |
| Malformed Media Queries 3 query-list member | A typed guaranteed-false query sentinel occupies the authored list position | `ReplaceMediaQueryWithNever`; span covers the comma-delimited member |
| CSS Syntax EOF implicit closure whose completed owning grammar is representable | Valid completed node is retained | `RetainWithImplicitClosure`; zero-width span at EOF |
| Top-level CDO or CDC ignored by CSS Syntax | No syntax node | `IgnoreLegacyToken`; non-empty token span |
| Configured nesting limit reached | No partial node; discard the smallest balanced enclosing recovery unit | `StopAtNestingLimit`; span covers that unit or the remaining input at EOF |

All other selector lists are unforgiving in I01. If recovery of a child leaves a
parent unrepresentable by its authored model, the child diagnostic is emitted
first, followed by the parent's diagnostic, and the smallest unrepresentable
parent is dropped. A dropped parent never leaks retained children.

Recovery never crosses a balanced block or list-member boundary and never
reinterprets tokens from a failed unit as a later sibling. Each loop records its
starting byte offset and must produce a node, advance by at least one byte, or
terminate its bounded input. The nesting limit is 256 nested rule blocks,
component-value blocks, and functions; the stylesheet root is depth zero.

Comments and whitespace are not recovery. Grammar-defined feature detection is
not recovery: syntactically valid unknown media types/features and supported
general-enclosed forms remain typed authored conditions without diagnostics
where the I01 catalog marks that exact owning production complete.

The recovered media state has this exact public shape:

```rust
#[non_exhaustive]
pub enum CssMediaQuery {
    Condition(CssMediaCondition),
    Typed(CssTypedMediaQuery),
    Never(CssNeverMediaQuery),
}

pub struct CssNeverMediaQuery {
    position: CssSourcePosition,
}
```

`CssMediaQuery::is_guaranteed_false()` is true only for `Never`.
`CssMediaQuery::position()` delegates to the active branch, and
`CssNeverMediaQuery::position()` is the first non-trivia position of the
malformed comma-delimited member, or its end position when empty. The sentinel
has no public constructor and is a valid recovered-syntax state, not an authored
claim that the malformed member was valid. Exactly one
`ReplaceMediaQueryWithNever` diagnostic exists for each sentinel, in the same
list order; its span, not the sentinel, owns the complete malformed source unit.
A clean parse never constructs `Never`, so `app-strict` always rejects a report
containing one.

### 6.2 Stylesheet And Encoding Behavior

`parse_sheet("")` returns an empty clean sheet. Valid rules remain in source
order. A rule before or after a recovered unit is retained unchanged.

Before ordinary rule parsing, the parser recognizes the optional leading legacy
`@charset "<label>";` form. A valid leading form is retained once as optional
`CssEncodingDeclaration` sheet metadata; its label is non-empty and it does not
perform byte decoding because the input is already Rust UTF-8. Leading BOM,
whitespace, and comments follow CSS Syntax 3.

A malformed leading form, missing semicolon, unquoted label, duplicate, or
non-leading `@charset` emits `InvalidEncodingDeclaration` with `DropAtRule` and
does not become metadata. Parsing then resumes at the next valid recovery
boundary. A malformed form is never silently consumed as dependency trivia.

I01 preserves the baseline's successfully parsed rule families only within the
exact grammar classified by its catalog: import, layer, font-face, keyframes,
style, media, container, scope, and nesting forms. It does not add Snapshot
rule families allocated to I02. A valid spelling outside I01 support is
`RecognizedUnsupported` when cataloged and otherwise unknown; both are dropped
with distinct typed error payloads.

### 6.3 Style-Attribute Behavior

`parse_style_attribute("")` and whitespace/comment-only input return an empty
clean declaration list. The parser accepts an optional final semicolon and
reuses the same ordinary declaration core as style-rule blocks.

Invalid declaration candidates are dropped independently. At-rules, qualified
rules, colonless segments, malformed separators, and other non-declaration
items in a style attribute are diagnosed as invalid declaration units with
`DropDeclaration`; later valid declarations remain eligible. A style attribute
never retains a rule node.

### 6.4 Diagnostic Ordering

Diagnostics are ordered by first responsible byte offset. Ties retain discovery
order, so a child precedes a parent made unrepresentable by that child. Nested
spans are permitted. Diagnostics are never sorted by display text, grouped by
category, or deduplicated.

## 7. Diagnostics, Positions, And Spans

### 7.1 Source Coordinates

Every diagnostic and retained syntax node that owns a source position uses:

```rust
pub struct CssSourcePosition {
    byte_offset: CssByteOffset,
    line: CssLineIndex,
    column: CssUtf16ColumnIndex,
}

pub struct CssSourceSpan {
    start: CssSourcePosition,
    end: CssSourcePosition,
}
```

`CssByteOffset::value() -> usize`, `CssLineIndex::value() -> u32`, and
`CssUtf16ColumnIndex::value() -> u32` are the scalar accessors.
`CssSourcePosition` exposes `byte_offset()`, `line()`, and `column()`.
`CssSourceSpan` exposes inclusive `start()` and exclusive `end()` and guarantees
source order. All five types are copyable, comparable, hashable private-field
values with no public arbitrary constructor.

Offsets are UTF-8 byte offsets into the original `&str`. Lines and columns are
zero-based; columns count UTF-16 code units. Dependency conversion preserves
`cssparser::SourceLocation::line`, converts its one-based UTF-16 column with
`saturating_sub(1)`, and combines it with the dependency cursor's byte offset.
A dependency-contract-violating zero column therefore maps to zero without
panic or wraparound. Human `Display` renders one-based line and column while
typed accessors retain the zero-based convention.

A discarded/replaced/ignored authored unit has a non-empty span. A zero-width
span is allowed only for a missing token or implicit EOF closure. Tests cover
empty input, first/later columns, CRLF and LF lines, comments, escapes, and a
supplementary Unicode scalar whose UTF-8 byte width and UTF-16 column width
differ.

### 7.2 Error Taxonomy

`Error` remains the typed failure value inside a recovery diagnostic and
exposes `kind() -> &ErrorKind`, `code() -> CssErrorCode`, and
`position() -> CssSourcePosition`. `CssErrorCode` and `ErrorKind` are public
non-exhaustive enums with a one-to-one root mapping for these I01 categories:

| Code/root variant | Required structured payload |
| --- | --- |
| `UnexpectedEnd` | static grammar expectation |
| `UnexpectedToken` | expectation and encountered token summary |
| `InvalidEncodingDeclaration` | expectation and optional encountered token |
| `InvalidAtRulePlacement` | at-rule name and expected context |
| `InvalidAtRulePrelude` | at-rule name, production ID, expectation, optional token |
| `InvalidAtRuleBody` | at-rule name, production ID, expectation, optional token |
| `UnknownAtRule` | authored at-rule name |
| `UnsupportedAtRule` | at-rule name and required catalog metadata |
| `InvalidQualifiedRule` | production ID, expectation, optional token |
| `InvalidSelector` | optional production ID, expectation, optional token |
| `InvalidMediaQuery` | optional feature name, expectation, optional token |
| `UnknownProperty` | authored property name |
| `UnsupportedProperty` | authored name and catalog metadata |
| `InvalidPropertyValue` | canonical known property, expectation, optional token |
| `InvalidDeclarationAnnotation` | declaration context and encountered token |
| `UnknownDescriptor` | owning at-rule and authored descriptor name |
| `UnsupportedDescriptor` | owning at-rule, descriptor name, and catalog metadata |
| `InvalidDescriptorValue` | owning at-rule, descriptor, expectation, optional token |
| `InvalidDescriptorCombination` | owning at-rule, responsible descriptor, and conflicting descriptors |
| `InvalidColorSyntax` | optional component, expectation, optional token |
| `NestingLimit` | configured limit and enclosing production ID |

Static `CssProductionId` and `CssGrammarExpectation` semantic values expose
`as_str()`. Identifiers and property names are decoded authored semantic values,
not formatted debug strings. `CssTokenSummary` exposes a non-exhaustive token
kind and the exact authored source slice. An absent encountered token means EOF,
never unavailable diagnostics.

Each root variant carries one public private-field detail type named for that
row: `CssUnexpectedEndError`, `CssUnexpectedTokenError`,
`CssEncodingDeclarationError`, `CssAtRulePlacementError`,
`CssAtRuleSyntaxError`, `CssUnknownAtRuleError`,
`CssUnsupportedAtRuleError`, `CssQualifiedRuleError`, `CssSelectorError`,
`CssMediaQueryError`, `CssUnknownPropertyError`,
`CssUnsupportedPropertyError`, `CssPropertyValueError`,
`CssDeclarationAnnotationError`, `CssUnknownDescriptorError`,
`CssUnsupportedDescriptorError`, `CssDescriptorValueError`,
`CssDescriptorCombinationError`, `CssColorSyntaxError`, and
`CssNestingLimitError`. Each detail exposes one read-only accessor for every
field named in its table row and no unrelated optional fields. The prelude and
body variants share `CssAtRuleSyntaxError` because their root variant supplies
the phase; no other two roots share a detail type.

`CssDeclarationContextRef<'_>` is a non-exhaustive borrowed enum distinguishing
ordinary known/custom properties, keyframe declarations, and descriptors. It
prevents an annotation diagnostic from pretending every block item is an
ordinary property.

There is no free-form catch-all variant. Dynamic prose exists only for
`Display`; tests and callers match codes and structured payloads. Existing root
variant meanings may not be repurposed when I02 adds a genuinely new category.

### 7.3 Recovery Diagnostic

`CssRecoveryDiagnostic` has private fields and exposes:

```rust
pub fn error(&self) -> &Error;
pub fn span(&self) -> CssSourceSpan;
pub fn action(&self) -> CssRecoveryAction;
```

`CssRecoveryAction` is public and non-exhaustive with exactly the ten actions in
section 6.1 at I01 completion. The error position is the first responsible token
or missing-token position; the span is the entire recovery unit. Action, error,
and span are therefore related but not interchangeable.

## 8. Declarations And Authored Values

### 8.1 One Property Schema

`src/properties.rs` shall own one crate-private declarative property schema. One
entry per recognized non-custom property owns canonical spelling, aliases,
stable catalog production ID, exact support status/subset, property-specific
authored value type, and parser function. It generates:

- `CssKnownProperty` and canonical/alias lookup;
- one `CssKnownDeclaration` variant per property;
- property-specific parse dispatch;
- `CssPropertyMetadata` and mapping back from a declaration;
- a crate-private implementation inventory used for bidirectional catalog
  evidence.

There is no second supported-property table, manual parallel enum, broad
`CssValue` dispatch, or parser-derived conformance catalog. Test vectors remain
independent from the schema.

### 8.2 Property-Coupled Declaration Model

The independent baseline `CssProperty`/`CssValue` pair is removed. The public
private-field model is:

```rust
pub struct CssDeclaration {
    body: CssDeclarationBody,
    importance: CssImportance,
    position: CssSourcePosition,
}

#[non_exhaustive]
pub enum CssDeclarationBody {
    Known(CssKnownDeclaration),
    Custom(CssCustomDeclaration),
}

#[non_exhaustive]
pub enum CssDeclaredValue<T> {
    Value(T),
    Global(CssGlobalKeyword),
    SubstitutionDependent(CssSubstitutionDependentValue),
}
```

Every generated known-declaration variant carries
`CssDeclaredValue<PropertySpecificType>`. The `all` property uses a dedicated
type permitting only a global keyword or substitution-dependent authored text.
`CssKnownDeclaration::property()` derives its property from the active variant.

`CssDeclaration` exposes `body()`, `known()`, `custom()`, `property_name()`,
`importance()`, and `position()`. `CssPropertyNameRef<'_>` is a non-exhaustive
borrowed enum with known and custom branches and is derived from the body, never
stored independently. Generic declared values expose optional typed/global/
substitution-dependent views.

`CssCustomDeclaration` couples a validated `CssCustomPropertyName` with
`CssCustomPropertyDeclaredValue`, whose non-exhaustive branches distinguish a
preserved authored token stream from a whole-value CSS-wide keyword. Empty and
whitespace-only custom-property value streams remain representable according to
CSS Variables; no public constructor can attach a custom value to a known name.
The exact branches are `Value(CssCustomPropertyValue)` and
`Global(CssGlobalKeyword)`, with corresponding optional accessors. A custom
property name is validated using CSS identifier tokenization after the required
`--` prefix, not an ASCII/alphanumeric approximation.

### 8.3 Importance And Shared Declaration Parsing

`CssImportance` is exactly `Normal` or `Important`; normal is the default.
Importance belongs to the declaration, not its value.

The shared declaration boundary recognizes one terminal,
ASCII-case-insensitive `!important`, allowing grammar-permitted whitespace and
comments between its tokens. It removes the annotation and only its separating
boundary trivia before constructing preserved authored value text. A bare `!`,
misspelling, duplicate annotation, or tokens after the annotation is an
`InvalidDeclarationAnnotation` and drops that declaration.

`CssDeclarationList` is an ordered private-field wrapper used by style rules
and style attributes. Keyframe blocks use a distinct ordered
`CssKeyframeDeclarationList` of `CssKeyframeDeclaration` values with no
importance field. The one private declaration core receives an exact ordinary,
keyframe, or descriptor context. `!important` in a keyframe declaration is
diagnosed at `!` and only that declaration is dropped.

### 8.4 Custom And Substitution-Dependent Values

Custom-property values preserve their exact UTF-8 source slice after removal of
parser-owned boundary trivia and a valid terminal importance annotation. They
preserve interior whitespace, comments, escapes, case, commas, functions, and
balanced block text. I01 performs no substitution or computed-value validation.

When a known-property value contains a syntactically admissible substitution
function that defers property grammar, the parser retains the complete authored
value as `CssSubstitutionDependentValue` rather than forcing it through the
property-specific value type. It exposes `as_css()` and does not promise a
resolved value or dependency graph. Custom-property authored values similarly
expose `as_css()` and `is_empty()`.

Bad strings/URLs, unmatched top-level closers, top-level semicolons, or
unbalanced structures use declaration recovery. A substitution function whose
arguments are unusable only after substitution remains valid authored syntax;
it is not silently treated as a completed property-specific value.

## 9. Independent Support Catalog

`src/conformance.rs` shall own a source-linked catalog independent of parser
dispatch. It contains one atomic parser-facing record for every grammar
production spelling recognized at I01 completion: rules, descriptors,
properties, selectors, media forms, and shared values. A record names:

- a globally unique stable `CssFeatureId`;
- one non-exhaustive `CssFeatureKind`;
- exact authored spelling or named grammar production;
- immutable source URL and production/section, or the exact `4b288d6` source
  path for a baseline-only extension;
- `CssSupportStatus`;
- for `Partial`, non-empty static descriptions of the supported subset and the
  valid-but-unsupported remainder;
- for `RecognizedUnsupported`, the exact diagnostic identity used when its
  spelling is encountered.

`Complete` means the whole identified production is accepted/recovered as its
own grammar requires. `Partial` means the exact named supported subset is
retained cleanly; every source form outside that subset is recovered with a
typed invalid, unknown, or recognized-unsupported diagnostic, depending on what
the bounded I01 parser can distinguish. `RecognizedUnsupported` means the exact
spelling is known but never retained as that production. An unknown spelling has
no record. `app-strict` accepts a diagnostic-free use of a partial supported
subset and rejects every recovered remainder.

Public `feature_catalog()` returns the immutable record slice and
`feature_metadata(id: &str)` performs exact stable-ID lookup.
`property_metadata(name: &str)` performs ASCII-case-insensitive canonical/alias
lookup for recognized non-custom properties and returns the same underlying
record plus canonical identity. Unknown and syntactically custom names return
`None` from property metadata; parse diagnostics still distinguish them.

`CssFeatureMetadata` exposes `id()`, `kind()`, `spelling()`, `source()`,
`production()`, `status()`, `supported_subset()`, and
`unsupported_remainder()`. The two subset accessors return `Option<&'static
str>` and are both `Some` exactly for `Partial`. `CssSpecificationSource`
exposes its immutable URL or exact repository commit/path provenance.
`CssPropertyMetadata` exposes `feature()`, `property()`, `canonical_name()`, and
`aliases()`. Metadata records and their component semantic IDs have no public
constructor.

The catalog is hand-authored from the sources in section 2 and cannot be
generated from property schema or parser branches. Crate-private implementation
inventories identify every dispatch path. Tests compare catalog records,
implementation inventories, and independent vector IDs in both directions.
Every `Complete` or `Partial` record has an implementation and positive vector;
every `Partial` or `RecognizedUnsupported` record has a diagnostic vector for
its unsupported boundary; no implementation or vector lacks a catalog record.

I02 shall extend this record shape and the same bidirectional checks to the
complete selected Snapshot profile. It may add records, enum variants, and
typed values, but it may not replace this catalog with parser-derived evidence.

### 9.2 Frozen I01 Production Inventory

The I01 selection is frozen from commit `4b288d6`. No baseline-recognized row is
retired. For every `Partial` row, the supported subset is exactly the positive
parser behavior at the cited baseline path, migrated to sections 4 through 8,
plus only the explicitly specified I01 deltas: browser recovery, style-
attribute entry, importance, substitution-dependent preservation, source
modeling, and checked declaration construction. Its unsupported remainder is the
rest of the row's cited dated grammar. I02, not I01, closes that remainder.

The rule and descriptor inventory is:

| Stable ID | Kind/spelling | Source | I01 status |
| --- | --- | --- | --- |
| `baseline.rule.import` | `@import` | `4b288d6:src/parser/mod.rs` and CSS Syntax 3 | Partial |
| `baseline.rule.layer-statement` | `@layer ...;` | `4b288d6:src/parser/mod.rs` | Partial |
| `baseline.rule.layer-block` | `@layer {...}` | `4b288d6:src/parser/mod.rs` | Partial |
| `baseline.rule.font-face` | `@font-face` | `4b288d6:src/parser/font_face.rs` | Partial |
| `baseline.rule.keyframes` | `@keyframes` | `4b288d6:src/parser/keyframes.rs` | Partial |
| `baseline.rule.style` | style and nested qualified rules | `4b288d6:src/parser/mod.rs` and `nesting.rs` | Partial |
| `baseline.rule.media` | `@media` | `4b288d6:src/parser/mod.rs` and `queries.rs` | Partial |
| `baseline.rule.container` | `@container` | `4b288d6:src/parser/mod.rs` and `queries.rs` | Partial |
| `baseline.rule.scope` | `@scope` | `4b288d6:src/parser/mod.rs` and `selectors.rs` | Partial |
| `foundation.encoding.charset` | optional leading legacy `@charset` metadata | CSS Syntax 3 and section 6.2 | Complete |
| `foundation.declaration-list.style-attribute` | style-attribute declaration-list structure | CSS Style Attributes and section 6.3 | Complete |
| `foundation.declaration.importance` | terminal declaration `!important` annotation | CSS Cascade 4 and section 8.3 | Complete |
| `baseline.declaration.custom-property` | custom-property names and authored token streams | `4b288d6:src/parser/variables.rs` and CSS Variables 1 | Partial |
| `baseline.value.substitution-dependent` | preserved known-property values containing substitution functions | `4b288d6:src/parser/variables.rs` and CSS Variables 1 | Partial |
| `later.rule.namespace` | `@namespace` | Namespaces 3 grammar allocated to I02 | RecognizedUnsupported |
| `later.rule.supports` | `@supports` | Conditional Rules 3 grammar allocated to I02 | RecognizedUnsupported |
| `later.rule.counter-style` | `@counter-style` | Counter Styles 3 grammar allocated to I02 | RecognizedUnsupported |
| `later.rule.page` | `@page` | CSS 2 grammar allocated to I02 | RecognizedUnsupported |
| `later.rule.font-feature-values` | `@font-feature-values` | Fonts 4 grammar outside I01 | RecognizedUnsupported |
| `baseline.descriptor.font-family` | `font-family` in `@font-face` | `4b288d6:src/parser/font_face.rs` | Partial |
| `baseline.descriptor.src` | `src` in `@font-face` | `4b288d6:src/parser/font_face.rs` | Partial |
| `baseline.descriptor.font-weight` | `font-weight` in `@font-face` | `4b288d6:src/parser/font_face.rs` | Partial |
| `baseline.descriptor.font-style` | `font-style` in `@font-face` | `4b288d6:src/parser/font_face.rs` | Partial |
| `baseline.descriptor.font-stretch` | `font-stretch` in `@font-face` | `4b288d6:src/parser/font_face.rs` | Partial |
| `baseline.descriptor.font-display` | `font-display` in `@font-face` | `4b288d6:src/parser/font_face.rs` | Partial |
| `baseline.descriptor.unicode-range` | `unicode-range` in `@font-face` | `4b288d6:src/parser/font_face.rs` | Partial |

The selector and query inventory is finite at the public entry-point grammar;
property-owned shared values remain part of their property row because I01 has
no public fragment parser. Grouping spellings in a row defines that row's exact
supported subset and does not claim the rest of the standards production.

| Stable ID | Exact baseline-recognized spelling group | Source | I01 status |
| --- | --- | --- | --- |
| `baseline.selector.complex` | type, universal, ID, class; presence and six valued attribute matchers; descendant, child, next-sibling, subsequent-sibling combinators | `4b288d6:src/parser/selectors.rs` | Partial |
| `baseline.selector.pseudo-class` | `:root`, `:hover`, `:active`, `:focus`, `:disabled`, `:enabled`, `:checked`, `:first-child`, `:last-child`, `:only-child`, `:empty`, `:first-of-type`, `:last-of-type`, `:only-of-type` | same | Partial |
| `baseline.selector.functional` | `:nth-child()`, `:nth-last-child()`, `:nth-of-type()`, `:nth-last-of-type()`, `:not()` | same | Partial |
| `baseline.selector.extension-state` | `:scope`, `:focus-visible`, `:focus-within`, `:required`, `:optional`, `:valid`, `:invalid`, `:placeholder-shown`, `:default`, `:indeterminate`, `:read-only`, `:read-write`, `:in-range`, `:out-of-range`, `:modal`, `:fullscreen`, `:popover-open` | same and Selectors 4 source in section 2 | Partial |
| `baseline.selector.extension-functional` | `:is()`, `:where()`, complex `:not()`, `:has()`, and nth-child `of` lists | same and Selectors 4 source in section 2 | Partial |
| `baseline.selector.attribute-case` | `i` and `s` attribute-selector modifiers | same and Selectors 4 source in section 2 | Partial |
| `baseline.selector.pseudo-element` | `::before`, `::after`, `::marker`, `::selection`, `::backdrop`, and generated `::marker` sequences | same | Partial |
| `baseline.selector.nesting` | nesting `&`, scoped selector anchors, and scoped relative selectors | same and `4b288d6:src/parser/nesting.rs` | Partial |
| `baseline.media.query-list` | typed/condition query lists, `not`/`only`, `and`/`or`/`not`, range and colon forms, and malformed-member `Never` recovery | `4b288d6:src/parser/queries.rs` and Media Queries 3 | Partial |
| `baseline.media.type` | `all`, `screen`, `print` | `4b288d6:src/parser/queries.rs` | Partial |
| `baseline.media.range-feature` | width, height, resolution, color, monochrome and their `min-`/`max-` names | same | Partial |
| `baseline.media.discrete-feature` | orientation, prefers-color-scheme, prefers-reduced-motion, prefers-reduced-transparency, prefers-contrast, forced-colors, hover, any-hover, pointer, any-pointer, display-mode | same | Partial |
| `baseline.container.condition` | `and`/`or`/`not`, size features, and custom-property style existence/equality | same | Partial |
| `baseline.container.size-feature` | width, height, inline-size, block-size, aspect-ratio, orientation and applicable `min-`/`max-` names | same | Partial |

The property inventory contains exactly the 179 canonical non-custom names
recognized by `4b288d6:src/validation.rs`. Each comma-delimited name below is a
separate `baseline.property.<canonical-name>` record with kind `Property`,
status `Partial`, and the property-specific parser reached from
`4b288d6:src/parser/mod.rs`. The supported subset also includes whole-value CSS-
wide keywords and syntactically admissible substitution-dependent authored
values under section 8. Grouping is presentation-only.

| Group | Canonical property names |
| --- | --- |
| Core/cascade | `all`, `display`, `box-sizing`, `position`, `direction`, `overflow`, `overflow-x`, `overflow-y`, `float`, `clear`, `visibility`, `content-visibility` |
| Flex/alignment | `flex-direction`, `flex-wrap`, `align-content`, `justify-content`, `align-items`, `align-self`, `justify-items`, `justify-self`, `place-content`, `place-items`, `place-self`, `gap`, `row-gap`, `column-gap`, `flex-basis`, `flex-grow`, `flex-shrink`, `order`, `flex`, `justify-tracks`, `align-tracks` |
| Generated content/lists | `content`, `list-style-type`, `list-style-position`, `list-style-image`, `list-style`, `counter-reset`, `counter-increment`, `counter-set` |
| Sizing/grid | `width`, `height`, `min-width`, `min-height`, `max-width`, `max-height`, `grid-flow-tolerance`, `grid-template-rows`, `grid-template-columns`, `grid-template-areas`, `grid-template`, `grid-auto-rows`, `grid-auto-columns`, `grid-auto-flow`, `grid-row-start`, `grid-row-end`, `grid-column-start`, `grid-column-end`, `grid-row`, `grid-column`, `grid-area`, `grid`, `aspect-ratio` |
| Typography/text | `font-size`, `line-height`, `writing-mode`, `text-align`, `text-align-last`, `text-indent`, `vertical-align`, `font-family`, `font`, `font-weight`, `font-style`, `font-stretch`, `font-variant`, `font-feature-settings`, `letter-spacing`, `text-wrap`, `white-space`, `word-break`, `overflow-wrap`, `text-overflow`, `text-decoration`, `text-decoration-line`, `text-decoration-color`, `text-decoration-style`, `text-decoration-thickness`, `text-transform` |
| Position/box | `inset`, `top`, `right`, `bottom`, `left`, `z-index`, `box-decoration-break`, `margin`, `margin-top`, `margin-right`, `margin-bottom`, `margin-left`, `padding`, `padding-top`, `padding-right`, `padding-bottom`, `padding-left` |
| Border/background/color | `border`, `border-top`, `border-right`, `border-bottom`, `border-left`, `border-width`, `border-top-width`, `border-right-width`, `border-bottom-width`, `border-left-width`, `color`, `background`, `background-color`, `border-color`, `border-top-color`, `border-right-color`, `border-bottom-color`, `border-left-color`, `background-image`, `background-position`, `background-size`, `background-repeat`, `background-origin`, `background-clip`, `background-attachment`, `border-style`, `border-top-style`, `border-right-style`, `border-bottom-style`, `border-left-style`, `border-radius`, `border-top-left-radius`, `border-top-right-radius`, `border-bottom-right-radius`, `border-bottom-left-radius`, `box-shadow`, `opacity` |
| UI/effects | `scrollbar-width`, `cursor`, `pointer-events`, `user-select`, `outline`, `outline-color`, `outline-style`, `outline-width`, `transform`, `transform-origin`, `translate`, `rotate`, `scale`, `filter`, `backdrop-filter`, `clip-path`, `mask`, `mask-image`, `mask-size`, `mask-position`, `mask-repeat` |
| Transitions/animations | `transition-property`, `transition-duration`, `transition-delay`, `transition-timing-function`, `transition`, `animation-name`, `animation-duration`, `animation-delay`, `animation-timing-function`, `animation-iteration-count`, `animation-direction`, `animation-fill-mode`, `animation-play-state`, `animation` |

Inventory tests assert 179 unique property names, exact equality with the
baseline source list, unique stable IDs across all tables, and no surviving
baseline parser dispatch outside these rows. Adding, retiring, or changing the
status of a row is specification reconciliation, not an implementation choice.

## 10. Production Structure

The design requires these responsibility boundaries; filenames may be split
further without changing ownership:

| Area | Responsibility |
| --- | --- |
| `src/lib.rs` | unsafe prohibition, intentional public reexports, feature-gated validators |
| `src/source.rs` | semantic source positions, offsets, spans, dependency conversion |
| `src/error.rs` | error codes, structured kinds/details, display formatting |
| `src/report.rs` | parse reports, recovery diagnostics/actions, validation failure |
| `src/conformance.rs` | independent stable feature catalog and public lookup |
| `src/properties.rs` | one property schema, generated coupled declaration identity and dispatch inventory |
| `src/syntax.rs` or semantic submodules | private-field authored trees and accessors |
| `src/parser/` | context-specific parsing, recovery-unit ownership, progress/nesting enforcement |
| `tests/` | public-consumer and feature-unification evidence using crate-root API only |

The parser shall have one recovery coordinator per structural context rather
than a second strict parser. Contexts may share helpers only when their recovery
boundary and parent-retention rule are identical. Parser adapters translate
dependency errors immediately into owned structured errors while the input
cursor and authored slice are still available.

The dependency order is architectural: source/error/report types and authored
invariants define the parser target; the parser establishes recovery behavior;
strict validation wraps the completed ordinary parser; the independent catalog
and public tests prove the complete I01 surface. An implementation sequence may
divide these into cycle-sized outcomes but may not create a temporary public
strict mode or a second grammar.

## 11. Documentation, Dependencies, And Product Effects

Crate docs and README shall describe browser recovery, the meaning of clean
reports, the `app-strict` wrapper, source coordinates, importance, support
statuses, custom/substitution-dependent preservation, and the crate's
non-responsibilities. They shall contain compiling minimal stylesheet and style-
attribute examples that inspect both retained syntax and diagnostics. Strict
examples are feature-gated.

Every new or materially changed public item has rustdoc stating its authored,
diagnostic, metadata, or validation phase; its invariant; and relevant
downstream non-responsibilities. Examples never infer validity from an empty
retained tree and never match a non-exhaustive enum without a wildcard.

I01 retains edition 2024, package identity, repository URL, and the two pinned
production dependencies. It adds no dependency and does not invent a leaf
`rust-version`. The root-owned integration follow-up must check the root's
committed MSRV against the published candidate before gitlink promotion.

Source remains authoritative. Root owns the only API generator and all generated
API audit artifacts. No I01 check requires a runtime network, browser,
subprocess, source checkout, or sibling repository.

## 12. Required Evidence Matrices

### 12.1 Recovery And Panic Freedom

Focused tables cover every section 6.1 row at top level and every applicable
nested context, with a valid sibling before and after the failed unit. Assertions
inspect retained node order, exact error code/payload, first-responsible
position, complete span, action, and diagnostic order. Additional vectors cover
child-then-parent failure, balanced delimiters containing misleading semicolons
or commas, repeated failures, zero progress attempts, the nesting boundary and
one level beyond it, implicit EOF closure, bad tokens, empty input, arbitrary
Unicode, and no unwind for adversarial ordinary `&str` input.

### 12.2 Parser/Validator Parity

The same sheet and style-attribute vector tables run with default features and
with `app-strict`. Ordinary reports are structurally identical in both builds.
Validators return the identical syntax for clean reports and all identical
diagnostics for recovered reports. Tests prove the parser is invoked once and
there is no feature-dependent dispatch table or syntax variant.

### 12.3 Declarations

Independent tables cover known, custom, global, and substitution-dependent
values; every property schema entry; a value valid only for an adjacent
property; empty/whitespace/interior custom values; terminal importance with
case/comments/whitespace; malformed and duplicate annotations; ordinary versus
keyframe context; invalid declarations between valid siblings; and sheet/style-
attribute parity. Compile-time/public tests demonstrate that a known property
cannot be paired with another property's value.

### 12.4 Catalog

Bidirectional tests compare exact stable IDs among the independent catalog,
implementation inventories, and independent positive/negative vector tables.
They prove status semantics, required Partial subset text, recognized-
unsupported diagnostics, alias identity, unknown distinctions, immutable source
references, unique IDs, and absence of parser-derived catalog construction.

### 12.5 Public Consumers And Coordinates

Tracked integration tests use only crate-root public API. They cover a clean and
recovered sheet, a clean and recovered style attribute, importance, custom and
substitution-dependent preservation, report decomposition, every diagnostic
accessor, metadata lookup, source byte/line/UTF-16 coordinates, non-exhaustive
matching, and feature-gated validation failure. They do not parse `Debug`, use
private modules, or retain obsolete APIs.

All public examples compile as doctests. Formatting, package tests, doctests,
configured warning-denied Clippy for all targets, feature/dependency inspection,
and repository-wide no-unsafe evidence are clean. Exact execution commands and
publication evidence belong to the current cycle plans and canonical workflow,
not this design contract.

## 13. Allocated Finding Closure

| Historical finding | I01 closure | Primary evidence |
| --- | --- | --- |
| 2.5 style-attribute entry point | report-based declaration-list API in sections 4 and 6.3 | public sheet/style parity vectors |
| 2.6 declaration importance | closed importance state and shared boundary parser in 8.3 | known/custom/keyframe annotation matrix |
| 2.15 leading encoding recovery | typed metadata plus malformed-form recovery in 6.2 | leading/BOM/comment/duplicate/non-leading matrix |
| 2.18 circular compatibility oracle | independent atomic catalog and three-way comparison in 9 | bidirectional catalog tests for the complete I01 surface |
| 2.19 public invalid states | private construction and phase invariants in 5 | constructor and public-consumer evidence |
| 2.20 property/value cross-product | generated property-coupled declaration model in 8 | schema coverage and compile-time API shape |
| 2.21 missing public guidance/tests | section 11 and tracked public tests | doctests and integration suite |
| 2.22 mixed location convention | byte/zero-based line/UTF-16 convention in 7.1 | multiline/non-BMP exact vectors |
| 2.23 weak negative diagnostics | structured taxonomy, spans, actions, and matrices | exact recovery assertions |
| 2.24 configured Clippy failure | no suppression; warning-denied configured target is clean | configured Clippy evidence |
| 2.25 missing unsafe prohibition | crate-root prohibition and full owned-source evidence | attribute, Clippy, and source scan |

I02 must preserve these closures while extending grammar and the independent
catalog. It owns the remaining historical grammar findings listed in P01.

## 14. Initiative Acceptance And Stop Conditions

I01 is complete only when all of these predicates hold:

1. Both ordinary functions always return a `CssParseReport`, retain only valid
   authored nodes, recover at every section 6 boundary, and never silently omit
   an invalid or unsupported source unit.
2. Every diagnostic has one structured error/code, exact first-responsible
   position, complete span, recovery action, and deterministic source order.
3. Default and `app-strict` ordinary parsing are identical; each validator is a
   one-pass clean-report wrapper with a non-empty typed failure.
4. Style attributes, importance, custom properties, substitution-dependent
   values, and property-coupled declarations satisfy section 8 with no public
   mismatched state.
5. Every I01 parser-facing production has exactly one truthful independent
   catalog record and required implementation/vector evidence; no valid
   unsupported remainder is labeled complete or merely unknown.
6. All allocated findings in section 13 are closed, while no I02 grammar or I03
   corpus completion is claimed.
7. Public docs, doctests, and integration tests expose the breaking surface and
   non-responsibilities; no obsolete alias is retained solely for compatibility.
8. Owned source and tests contain no `unsafe`; all configured verification is
   clean without lint suppression, new dependencies, acquired software, or
   leaf-owned generated API artifacts.
9. The reviewed implementation is landed and published on leaf `main`, remotely
   read back, and handed off as an immutable breaking CSS candidate with exact
   root follow-up and no root or sibling mutation.

Stop and reconcile this specification before sequencing if browser recovery
would require an invalid retained node, a second parser, a changed dependency,
an unbounded support claim, a public raw-token escape, or a materially different
root migration. Stop during implementation if `cssparser` cannot expose a
required recovery boundary or complete source extent through its pinned safe
API; diagnostic instrumentation may be designed in owned parser adapters, but
the dependency contract may not be guessed or bypassed with `unsafe`.

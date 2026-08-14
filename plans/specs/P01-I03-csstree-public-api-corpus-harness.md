# P01-I03 CSSTree Public-API Corpus Harness

## 1 Authority, outcome, and entry state

This is the JIT specification for `P01-I03` in `surgeist-css`. It is governed
by `plans/specs/P01-css-syntax-conformance-program.md`, semantic SHA-256
`87f6a94b893ffa416c6ff451575f0d5a21b4aa136e7bcd391cd6c0ce8810a2ae`, and the
completed I02 grammar closure at leaf commit
`a8a6f00bd9f49464dfdef24f0feba9fdff705189`.

I03 adds independent, broad evidence for the browser-recovering public CSS
front doors. It commits the pinned CSSTree neutral corpus, a CSS-owned
per-case oracle, and an in-process integration harness that executes every
imported case through public APIs. It does not change production parsing,
public API, recovery semantics, or the I02 catalog to make a corpus case pass.

The fixed provider is the already-published `surgeist-generator` crate at
`83a216880884a5a364258ffaaeaf93d228c0bc53`, used exactly as published. Its
completed grouped-case cycle proves the pinned `fixtures/ast` census of 74
files, 935 cases, 721 upstream-parsed cases, and 214 upstream-rejected cases.
The existing read-only checkout `tmp/csstree` is pinned to
`88e3d965c0b1628642a30a841745b410d6835052`; its `fixtures/ast` tree is
`bfadc7a7a8d93dce59a27fa7df3bb0f6f6a623d8`.

The generator is a provider, not a leaf dependency: it is invoked only during
artifact maintenance and cycle verification through its public CSS command.
Ordinary Cargo tests consume committed artifacts and never invoke the generator,
Node.js, JavaScript, a browser, Git, a subprocess, a network, or a checkout.

## 2 Ownership and non-goals

`surgeist-css` owns the committed attribution, the adopted corpus root and its
maintenance, CSS-owned oracle, complete-input adapter registry, integration
test, and documentation. `surgeist-generator` owns the import/generation/check
implementation and the neutral artifact contract that governs that adopted
root. Root owns facade integration and generated API artifacts. No sibling,
root, or generator source is edited by this initiative.

I03 excludes parser grammar repair, public type additions, serializer design,
cascade, substitution, selector matching, layout, upstream AST or diagnostic
prose, and generic fragment parsing. A case that exposes a defect in a grammar
claimed complete by I02 remains active failing evidence; it may not be
reclassified unsupported. A case without a truthful complete-input adapter may
execute only as explicitly classified unsupported panic-freedom evidence.

No production dependency, feature, build script, CI rule, browser, JavaScript
runtime, external acquisition, or executable `unsafe` is permitted. Before
editing the manifest, the worker must perform the read-only cache proof
`test -d /Users/codex/.cargo/registry/src/index.crates.io-*/serde-1.0.228 &&
test -d /Users/codex/.cargo/registry/src/index.crates.io-*/serde_json-1.0.145`.
Only after that proof may test-only JSON deserialization add the exact
generator-compatible entries `serde = { version = "=1.0.228", features = ["derive"] }`
and `serde_json = "=1.0.145"`. A cache miss is a tooling blocker,
not permission to fetch.

## 3 Immutable corpus contract

The adopted, leaf-owned corpus root governed by the generator contract is
`tests/corpus/csstree/`:

```text
tests/corpus/csstree/
├── LICENSE
├── README.md
├── corpus.toml
├── source/
│   ├── .surgeist-source.json
│   └── 74 pinned fixture JSON files
└── expectations/
    ├── generation-reports/all.json
    └── 74 canonical neutral expectation JSON files
```

The CSS-owned oracle is `tests/csstree/oracle.json`; test support is outside
the generated tree. The upstream MIT notice is copied unchanged from the pinned
checkout. `corpus.toml` uses generator schema 1 and records the canonical
repository, revision, `fixtures/ast`, expected file count 74, expected case
count 935, distinct one-component roots, and the canonical report path.

All neutral generated cases remain `active` with no neutral reason. Their
upstream outcome is only `parsed` or `rejected`; upstream AST objects, error
classes, source locators, and diagnostic wording are not imported. The generator
case ID is opaque to CSS and is accepted only after validating the generated
schema. Grouped members and singleton members must remain a bijective 935-case
inventory; the CSS harness does not reproduce ID derivation.

The pinned context totals are:

| Context | Files | Parsed | Rejected | Total |
| --- | ---: | ---: | ---: | ---: |
| `atrule` | 14 | 116 | 14 | 130 |
| `atrulePrelude` | 1 | 2 | 0 | 2 |
| `block` | 1 | 29 | 0 | 29 |
| `declaration` | 4 | 73 | 4 | 77 |
| `declarationList` | 3 | 19 | 0 | 19 |
| `mediaQuery` | 4 | 43 | 6 | 49 |
| `rule` | 5 | 33 | 0 | 33 |
| `selector` | 21 | 196 | 121 | 317 |
| `selectorList` | 1 | 2 | 8 | 10 |
| `stylesheet` | 4 | 48 | 28 | 76 |
| `value` | 16 | 160 | 33 | 193 |
| **Total** | **74** | **721** | **214** | **935** |

The generated report digest binds the oracle to the exact neutral artifacts.
Any source, expectation, report, ID, context, input, or option change makes the
oracle stale and requires an explicit reviewed corpus migration.

## 4 CSS-owned oracle and semantic phases

`tests/csstree/oracle.json` is canonical pretty JSON with one final LF and
schema version 1. It records the provider repository, source revision, source
tree, expectation schema version, generation-report SHA-256, and exactly one
sorted record per generated case ID. Each record repeats the fixture path and
context, names one closed probe, and names exactly one expected outcome. No
prefix, context, or fixture fallback supplies a missing record.

The private record schema is complete and closed. A `Probe` is one tagged
variant:

- `active { entry_point, adapter, extractor, property_or_descriptor, options,
  payload }`; or
- `panic_freedom { entry_point: style_attribute, adapter:
  custom_property_containment, payload }`.

`entry_point` is `sheet` or `style_attribute`; `adapter` is one of the exact
registry identities in §5; `property_or_descriptor` is required only for a
value probe; and `options` is the closed upstream-options object. Every payload
stores `prefix`, `suffix`, and the exact input byte length. The harness
constructs `prefix + input + suffix`, requires the input bytes to occur exactly
once, and computes the payload span as
`prefix.len()..prefix.len()+input.len()`; the oracle never stores or infers a
second span. The `panic_freedom` adapter is exactly the fixed prefix
`--surgeist-corpus-probe:` followed by the input bytes and a fixed `;`, passed
to `parse_style_attribute`; it
preserves the payload bytes and makes no conformance claim. A wrapper identity
has no implicit defaults and is rejected when illegal for the case context.

Every adapter registry identity names one closed `Extractor` variant and its
public path/count semantics. The extractor inventory is:
`SheetRules` = `CssSheet::rules().len()`;
`TopLevelRuleKind(kind)` = the count of that `CssRule` variant in
`CssSheet::rules()` for rule-level probes only;
`StyleDeclarations` = the first retained top-level `CssRule::Style`'s
`CssStyleRule::declarations().len()`, or zero when no such rule exists;
`StyleSelector` = one when that first style rule exists (its public
`CssStyleRule::selector()` is then read), otherwise zero;
`DeclarationList` = `CssDeclarationList::len()`;
`MediaQueries` = the first top-level `CssRule::Media`'s
`CssMediaRule::query().queries().len()`, or zero when absent;
`MediaChildren` = the first top-level media rule's `CssMediaRule::rules().len()`,
or zero when absent;
`SupportsChildren` = the first top-level supports rule's
`CssSupportsRule::rules().len()`, or zero when absent;
`ContainerChildren` = the first top-level container rule's
`CssContainerRule::rules().len()`, or zero when absent;
`ScopeChildren` = the first top-level scope rule's
`CssScopeRule::rules().rules().len()`, or zero when absent;
`LayerBlockChildren` = the first top-level layer-block rule's
`CssLayerBlockRule::rules().len()`, or zero when absent;
`FontFaceDescriptor(kind)` = the first top-level `CssRule::FontFace`'s
`CssFontFaceRule::descriptors().occurrences()` count matching the named public
`CssFontFaceDescriptorRef` variant, or zero when absent; and
`KnownDeclaration(index, property)` = one only when the indexed public
`CssDeclaration::property_name()` is the named known property, otherwise zero.
`TopLevelRuleKind(FontFace)` is forbidden for descriptor probes; descriptor
probes must use `FontFaceDescriptor(kind)` so dropped descriptors cannot appear
retained merely because their outer rule survived.
The oracle's retained-syntax field names the extractor and exact expected
number (or the explicit `nonempty`/`empty` relation); no outer-wrapper count
may stand in for a payload extractor.

Each non-unsupported expected record stores one exact `Observation`: a public
retained-syntax count rule (`exact`, `nonempty`, or `empty`), `is_clean`, and an
ordered diagnostic vector. Each diagnostic stores a closed `CssErrorCode` name,
a closed `CssRecoveryAction` name, the exact responsible byte offset, the exact
exclusive span start/end, and an exact multiplicity of one. The vector is
compared in report order; extra or missing diagnostics fail the case. The
oracle also stores a payload relation (`intersects` or `ends_at`) as a load-time
sanity check, but never replaces the exact byte offsets and spans.

The only closed unsupported execution policies are:

- `full_observation`: invoke the selected public parser, compare the exact
  oracle observation, and apply the same strict-parity rule as an active case;
- `panic_freedom_only`: invoke the selected public parser under
  `catch_unwind`, require no unwind, and do not assert conformance. Its tagged
  probe is the §4 `custom_property_containment` adapter and is permitted only
  when the registry marks the original fragment as lacking a truthful complete
  adapter. Under `app-strict`, invoke `validate_style_attribute` on that exact
  constructed source, require `Ok` iff the ordinary report is clean, and when
  it is `Err` require diagnostics equal to the ordinary report. Its result is
  still included in mismatch aggregation.

`Clean` requires `is_clean` and an empty exact vector; `Recovered` and
`StrictRejected` require a nonempty exact vector and the recorded retained
syntax rule. Unsupported records always carry one finite reason and one of the
two policies; `full_observation` carries the exact observation, while
`panic_freedom_only` carries no conformance expectation.

The canonical public observation is
`{syntax_count, is_clean, diagnostics: [{code, action, byte_offset, span_start, span_end}]}`.
Both default and `app-strict` builds compare ordinary parser results to this
same committed observation baseline. In the strict build, the validator's
`CssValidationFailure::diagnostics()` must additionally equal the ordinary
diagnostics, including order, code, action, position, and span. This common
baseline is the deterministic cross-feature parity evidence; no comparison of
separately generated test output is required.

The private harness separates these phases:

1. persisted JSON records;
2. validated case identities, contexts, options, input, and outcome;
3. a complete CSS source plus payload byte span;
4. an observed public parse/validation result;
5. a typed comparison producing zero or more mismatches.

The generated neutral disposition and CSS oracle outcome are separate. The
generator must contain 935 `active` cases with no neutral reason. The CSS oracle
then partitions the same 935 IDs among `Clean`, `Recovered`, `StrictRejected`,
and `Unsupported`; `ExpectedFail` and `Quarantined` are not valid oracle states.

The closed expected outcomes are:

- `Clean`: browser report is clean and `app-strict` accepts it;
- `Recovered`: browser report contains the exact ordered diagnostic predicate
  list, its retained-syntax rule, and `app-strict` rejects with equal
  diagnostics;
- `StrictRejected`: browser report contains the exact ordered typed rejection
  predicate list, its retained-syntax rule, and `app-strict` rejects with equal
  diagnostics;
- `Unsupported`: the case still executes, but its reason belongs to the finite
  classes below and it is not conformance coverage.

`ExpectedFail` and `Quarantined` are invalid completion states, not escape
hatches. At completion their counts are zero. Unsupported reasons are exactly:
outside the selected profile; vendor/host-specific syntax; an upstream parser
option without a public CSS equivalent; a generic fragment without a truthful
supported property/descriptor; or AST-construction behavior with no public
stylesheet meaning. “Not implemented,” “flaky,” and “currently failing” are
never valid reasons.

Upstream options are a closed model covering the observed keys `atrule`,
`parseAtrulePrelude`, `parseCustomProperty`, `parseRulePrelude`, `parseValue`,
and `property`. Every observed combination is consumed by a probe or explicitly
classified unsupported; unknown keys and types fail corpus loading.

## 5 Complete-input adapter registry

Every active probe preserves its upstream input bytes contiguously in one
complete stylesheet or style-attribute source and records the payload span.
Wrappers may add only fixed ASCII syntax selected by the named probe; they may
not repair, normalize, escape, reorder, or delete payload bytes.

| Context | Adapter requirement |
| --- | --- |
| `stylesheet` | Parse the source directly as a stylesheet. |
| `rule` | Use a complete top-level rule or one explicitly named legal group-rule wrapper. |
| `atrule` | Use a legal top-level at-rule or named legal wrapper for restricted forms. |
| `declarationList` | Place the list in one fixed valid style-rule body. |
| `declaration` | Place one declaration in one fixed valid style-rule body. |
| `block` | Place block content at a named valid block location. |
| `selectorList` | Use the payload as a complete style-rule selector list. |
| `selector` | Use a fixture-specific complete selector probe; otherwise unsupported. |
| `mediaQuery` | Use the payload in one fixed `@media` rule. |
| `atrulePrelude` | Select a supported at-rule wrapper from the typed `atrule` option. |
| `value` | Name the supported property or descriptor that owns the value grammar. |

The registry is exhaustive over all 74 fixture paths and has no default. Each
path record explicitly names its adapter, entry point, extractor, and whether
the path may use the tagged `panic_freedom` variant; no path silently inherits
the custom-property containment adapter. Value
probes may use properties and `@font-face` descriptors already exposed by the
public API; a custom property is valid only for authored-token preservation and
never claims ordinary property validity. Canonical CSS is retained for schema
drift detection but is not compared to Surgeist serialization, which I03 does
not own.

## 6 Public behavior and recovery oracle

The integration test imports only the public `surgeist_css` front door. Each
case is invoked inside `catch_unwind`; an unwind becomes a typed mismatch and
does not stop later cases. Every active case executes exactly once.

For `Clean`, the report must contain valid syntax and match the oracle's exact
clean observation. For `Recovered` or `StrictRejected`, the report must match
the oracle's exact ordered diagnostic vector, each diagnostic must satisfy its
payload relation, and the retained-syntax rule must match. An upstream-rejected
case must not unwind, must not retain invalid syntax, and must have a
payload-accounting diagnostic. Unsupported `panic_freedom_only` cases still
must not unwind. Under `app-strict`, the same public parse report must be
rejected exactly when it is non-clean, and
`CssValidationFailure::diagnostics()` must equal the ordinary diagnostic slice;
no second test grammar is permitted.

Default and `app-strict` runs must observe identical browser-compatible syntax
and diagnostics. Exact positions, spans, action ordering, nesting, and public
model invariants remain owned by focused tests; the corpus checks typed
behavior, not implementation details or display strings.

## 7 Inventory, loading, and failure reporting

One `OnceLock`-backed validated corpus is shared by context-family tests. The
loader reads the report, resolves its sorted expectation inventory, deserializes
the 74 files, hashes and deserializes the oracle, and proves schema, digest,
context, neutral disposition, case-ID, path, option, adapter, and 935-entry
bijection invariants before calling the parser.

It asserts 74 files; 935 unique cases; the 721/214 parsed/rejected totals; all
eleven context totals; 935 neutral `active` records with no neutral reason;
the four CSS oracle outcomes partitioning those same 935 IDs; zero expected-
fail and quarantined oracle records; one adapter per fixture; and report/oracle
digest agreement. It does not verify Git, reconstruct sidecars, inspect Rust source,
inspect tests or catalogs, infer owner sets/counts from implementation, or use
test-only production hooks.

Each context-family test executes every case, collects all mismatches, sorts
them by opaque case ID, and fails once with the complete typed list. A mismatch
includes case ID/path, context, upstream and oracle outcomes, probe, observed
result or panic marker, relevant codes/actions, and escaped adapted source and
payload span. No first-failure assertion hides later cases.

## 8 Dependencies, features, and artifacts

Production dependencies, public API, MSRV, and features remain unchanged. The
only permitted new dependencies are the exact test-only
`serde = { version = "=1.0.228", features = ["derive"] }` and
`serde_json = "=1.0.145"` entries named above. After the manifest
change, `cargo test --offline` must resolve both; the local lock, when present,
is then checked with `cargo test --locked --offline`. If either command cannot
resolve the exact versions, stop before acquisition. No generated Rust source,
serializer, snapshot framework, browser, subprocess, or network dependency is
allowed.

The committed corpus and oracle are generated/maintained as one artifact set.
The generator's `import-csstree`, `generate`, and `check-corpus` commands run
only against the existing pinned checkout and a disposable owner/corpus root
during maintenance. Ordinary Cargo tests consume only committed files.

## 9 Initiative acceptance

I03 is accepted only when:

1. the exact pinned source and neutral artifact set are committed with MIT
   attribution and generator report/provenance intact;
2. the generator candidate's offline `check-corpus` passes against that set;
3. all 935 cases execute once through public APIs, with 935 neutral `active`
   cases and the four CSS oracle outcomes partitioning those same IDs; there
   are zero expected-fail/quarantined cases;
4. every active case satisfies its explicit clean, recovered, or strict oracle;
5. every unsupported case executes panic-freedom with one finite truthful reason;
6. all context and parsed/rejected totals match the pinned census;
7. default and `app-strict` browser observations are identical and strict
   validation rejects every non-clean report;
8. mismatches aggregate deterministically; and
9. no production API/dependency/unsafe/source-parsing test was introduced.

Any claimed-complete corpus mismatch, unknown fixture shape, untruthful adapter,
license change, parser/public-API requirement, acquisition pressure, or unsafe
match is a stop condition. It returns to the owning specification or provider;
it never authorizes weakening the oracle or changing production behavior.

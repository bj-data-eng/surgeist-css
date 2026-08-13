# P01 CSS Syntax Conformance Program

## P01.1 Authority And Outcome

This document is the authoritative program contract and initiative index for
P01 in `surgeist-css`. Repository source and `AGENTS.md` continue to own current
crate facts, while the installed `surgeist-agent` skill owns planning,
implementation, review, landing, publication, and handoff workflow. Detailed
desired-state decisions belong to the one current initiative specification;
this program does not replace one.

P01 shall evolve `surgeist-css` into a browser-compatible, panic-free parser of
CSS-owned authored syntax with truthful CSS Snapshot 2026 conformance metadata,
an additive application-strict validation feature, and broad independent public
API corpus evidence. Every retained syntax node shall be valid by construction.
Every recovery shall be observable through a typed diagnostic; recovery is never
silent acceptance of the original source.

P01 is complete only through the three ordered initiatives in P01.5. No single
initiative may claim the complete program outcome.

## P01.2 Fixed Evidence And Decisions

The compatibility baseline is the review at
`plans/P01-implement-full-css-spec/P01-css-snapshot-2026-review.md`, SHA-256
`5ddd3eebb4fc3664759021605d3884a0c795947e0ef4e427d3dfc5e77469199d`,
scoped to source commit
`318864d1074d8d723a3a925528343c8a3d8c7253`. Its 25 findings remain the finite
remediation ledger.

The following legacy artifacts at commit
`4b288d6467d91f2fc33eac78ef0b0b725154195d` are design evidence only and are
not active planning packets:

- `plans/P01-implement-full-css-spec/initiatives/P01-I01-css-snapshot-2026-remediation.md`,
  SHA-256
  `2cdf3d1536e913c539d5f63c82889e353350baaf87ddd4796867d6eb0b89dbba`;
- `plans/P01-implement-full-css-spec/initiatives/P01-I02-csstree-public-api-corpus-harness.md`,
  SHA-256
  `684825e26a7aa6896df2532d463e70d82c49d8e7b284d4c7bc1b863c7cd50b12`.

P01 fixes these program-level decisions:

1. The ordinary stylesheet and style-attribute parsers use CSS Syntax and owning
   grammar recovery. Each returns validated retained syntax plus every typed
   recovery diagnostic in source order.
2. The additive `app-strict` feature exposes validation entry points that run the
   same browser-compatible parser once and reject every report containing a
   recovery diagnostic. Feature unification never changes ordinary parser
   behavior.
3. `cssparser = 0.37.0` supplies safe tokenization, positions, delimited parsing,
   balanced consumption, and iterator recovery boundaries. `surgeist-css` owns
   diagnostic categories, recovery actions, complete source spans, ordering,
   context-sensitive parent retention, and all public models.
4. CSS Snapshot 2026 is pinned by dated normative sources in the current
   initiative specification. Moving standards URLs and editor drafts are not
   conformance inputs.
5. Existing intentionally selected later syntax remains in scope only through a
   finite, source-linked inventory. An initiative may retire an untruthful legacy
   claim with an explicit breaking record; it may not preserve it implicitly.
6. P01 introduces no `unsafe`. No Surgeist-owned Rust target may contain or
   enable `unsafe` code.

## P01.3 Ownership And Program Boundary

`surgeist-css` owns authored CSS syntax, intrinsic lexical and grammar validity,
CSS recovery boundaries, authored source positions, recovery diagnostics,
support metadata, focused conformance tests, and its published leaf candidates.

P01 excludes:

- cascade ordering or application, inheritance application, and shorthand
  application;
- custom-property substitution, dependency resolution, and post-substitution
  property validation;
- evaluation of supports, media, container, or style queries;
- selector matching, specificity application, pseudo-state, and scope proximity;
- URL resolution, imports or font loading, image decoding, and other resource
  loading;
- unit resolution, layout, painting, color conversion or gamut mapping,
  animation interpolation, and renderer behavior;
- root adapters, root API artifacts, root integration tests, root gitlinks, and
  sibling implementation;
- an authored-syntax serializer or a public generic fragment parser;
- acquiring or executing external software without the exact authorization
  required by the active Surgeist workflow.

Root `surgeist` owns every cross-crate adapter, facade change, API artifact, and
gitlink promotion. `surgeist-generator` owns corpus import, neutral expectation
generation, reports, and offline corpus verification. A leaf initiative returns
a published candidate handoff and never edits either owner.

## P01.4 Durable Initiative Boundaries

P01 uses three initiatives because it contains three independently releasable
contracts in different phases.

### P01-I01 Browser Recovery And Authored API Foundation

I01 owns the intentionally breaking production front door and model foundation:

- browser-compatible `CssParseReport` entry points for sheets and style
  attributes;
- typed errors, recovery diagnostics, recovery actions, source positions, and
  source spans;
- additive `app-strict` validation;
- declaration importance, style-attribute declaration lists, custom-property
  preservation, and property-coupled declarations;
- one truthful support/property metadata foundation for the grammar implemented
  at I01 completion;
- checked construction, non-exhaustive evolution boundaries, public consumer
  tests, crate documentation, `#![forbid(unsafe_code)]`, and clean configured
  verification.

I01 makes no Snapshot-completeness claim. It must classify every grammar spelling
it recognizes at its completion as complete, partial, or recognized unsupported,
and ordinary parsing must recover with a typed diagnostic from unsupported or
invalid constructs. I01 shall leave I02 able to add completed grammar without
redesigning parser semantics, diagnostics, positions, or validation. P01.9
supersedes only the published claim that every required authored-value evolution
boundary and declaration payload is already sufficient; its one required repair
preserves property/value coupling while replacing insufficient partial payload
shapes.

### P01-I02 CSS Snapshot 2026 Grammar Closure

I02 owns production grammar and conformance completion over the published I01
foundation:

- the independently inventoried, dated Snapshot 2026 profile and preserved
  extension inventory;
- stylesheet ordering, rules, descriptors, selectors, namespaces, media and
  conditional syntax;
- official properties and property-specific authored values;
- calculations, positions, timing, functions, relative colors, Grid,
  typography, keyframes, and other shared value grammar identified by the
  review;
- exact positive, negative, recovery, metadata, and public-API vectors proving
  that every in-boundary official production is complete.

I02 may add public variants and new validated authored types. Its first cycle
shall perform the bounded evolution-boundary repair recorded in P01.9 before any
grammar-closure cycle: add missing non-exhaustive boundaries and replace I01
property/value payload shapes that cannot represent their complete owning
grammar without semantic distortion. It shall not change the I01 parser
signatures, report meaning, recovery actions, diagnostic ordering,
source-coordinate convention, property/value coupling, or feature-unification
behavior. It adds no corpus harness and invokes no sibling generator.

### P01-I03 CSSTree Public API Corpus Harness

I03 owns fixture-phase independent evidence only:

- the pinned, license-complete CSSTree corpus and neutral generated expectations;
- a CSS-owned per-case oracle and complete-input adapter registry;
- a public-API-only integration harness that executes every imported case;
- deterministic inventory, classification, panic-freedom, mismatch aggregation,
  default-feature, and `app-strict` parity evidence;
- offline corpus verification through a reviewed, published, fetchable
  `surgeist-generator` candidate.

I03 shall not change production parsing behavior or public API to make a case
convenient. Claimed-complete I02 grammar failures remain parser defects; cases
may not be weakened, quarantined, or relabeled unsupported to obtain a pass.

## P01.5 Short Initiative Sequence

| Order | Initiative | Entry state | Exit state and handoff |
| --- | --- | --- | --- |
| 1 | `P01-I01` Browser Recovery And Authored API Foundation | P01 is reviewed `CLEAN`; current source and the historical review are reconciled in a JIT I01 specification | I01 implementation is reviewed, published on leaf `main`, and returned as a breaking CSS candidate with exact root follow-up |
| 2 | `P01-I02` CSS Snapshot 2026 Grammar Closure | I01 is published and fetchable; P01 and I01 completion evidence remain valid; a JIT I02 specification is reviewed `CLEAN` | Every in-boundary official grammar is complete, preserved extensions are truthful, all allocated review findings are closed, and the published leaf candidate is handed off |
| 3 | `P01-I03` CSSTree Public API Corpus Harness | I02 is published and fetchable; a compatible generator candidate and complete handoff are verified; a JIT I03 specification is reviewed `CLEAN` | The pinned corpus and public harness are committed, all cases execute under their truthful oracle, offline verification passes, and the published leaf candidate is handed off |

Only the next initiative specification may be authored. A later specification is
written after the prior initiative is reviewed, landed on leaf `main`, published,
read back, and handed off. Each multi-cycle initiative receives its own reviewed
implementation sequence; only its next cycle plan is written. No row above is an
implementation sequence or authorization to pre-author future task detail.

## P01.6 Review-Finding Allocation

Each historical finding has one primary closure initiative. A later initiative
must preserve earlier closure evidence.

| Review finding | Primary closure |
| --- | --- |
| 2.5 style-attribute entry point | I01 |
| 2.6 declaration importance | I01 |
| 2.15 leading encoding recovery | I01 |
| 2.18 independent compatibility oracle foundation | I01 |
| 2.19 public invalid states | I01 |
| 2.20 property/value cross-product | I01 |
| 2.21 public guidance and consumer tests | I01 |
| 2.22 source-coordinate convention | I01 |
| 2.23 exact negative diagnostics | I01 |
| 2.24 configured Clippy failure | I01 |
| 2.25 crate-root unsafe prohibition | I01 |
| 2.1 missing Snapshot at-rules | I02 |
| 2.2 incomplete or overstated properties | I02 |
| 2.3 incomplete Selectors 3 | I02 |
| 2.4 incomplete Media Queries 3 | I02 |
| 2.7 layer/import ordering and import conditions | I02 |
| 2.8 repeated ID selectors | I02 |
| 2.9 position grammar leakage | I02 |
| 2.10 generic function grammar leakage | I02 |
| 2.11 timing domains and non-finite values | I02 |
| 2.12 calculation and range handling | I02 |
| 2.13 relative-color channel typing | I02 |
| 2.14 Grid repetition invariants | I02 |
| 2.16 typography grammar | I02 |
| 2.17 keyframe duplicate and empty structures | I02 |

I01 establishes the independent catalog mechanism required by finding 2.18. I02
must extend that same independent catalog to the complete selected profile and
prove bidirectional coverage; it may not replace it with parser-derived evidence.
I03 supplements, but does not own closure of, the 25 findings.

## P01.7 Compatibility, Dependencies, And Artifacts

I01 is intentionally breaking. Its candidate handoff shall identify removed or
changed public types and the root-owned adapter/API-artifact migration. I02 is
additive relative to sound I01 evolution boundaries, except for the one bounded
repair authorized by P01.9 and any later contradiction that returns to P01
reconciliation before implementation. I03 is production-API internal-only.

I01 and I02 shall retain the existing production dependencies unless their JIT
specification establishes a new dependency as a material design decision and the
active workflow separately authorizes any required acquisition. I03 may use only
reviewed test-only dependencies and generated corpus artifacts defined by its JIT
specification. No initiative adds a build script, CI rule, policy mirror, or
leaf-owned API audit artifact under this program contract.

Source is authoritative. Root owns API generation and generated API audit
artifacts. Corpus source, expectations, reports, and license material belong to
`surgeist-css` only after I03 adopts them through the generator's public contract.

## P01.8 Program Acceptance And Stop Conditions

P01 is complete only when:

1. I01, I02, and I03 each satisfy a reviewed JIT specification, every required
   initiative sequence and current cycle plan, task reviews, holistic cycle
   reviews, configured checks, publication, remote readback, and candidate
   handoff under the active Surgeist workflow.
2. All 25 historical findings are closed in their primary initiative and remain
   closed through the final I03 candidate.
3. Ordinary parsing is browser-compatible and panic-free for ordinary `&str`
   input, returns only valid retained syntax, and reports every recovery through
   typed ordered diagnostics.
4. Application-strict validation is an additive feature over exactly the same
   parse and rejects every non-clean report.
5. Every selected in-boundary Snapshot production is truthfully complete, every
   preserved extension is explicitly sourced, and no parser path escapes the
   independent catalog.
6. The pinned independent corpus executes completely through public APIs with no
   expected failures, quarantine, runtime generator, source checkout, browser,
   JavaScript, subprocess, or network requirement in ordinary tests.
7. The final leaf candidate and each intermediate immutable candidate are
   fetchable from the authority remote, with no root or sibling mutation by this
   repository.
8. All owned Rust remains free of `unsafe`; configured tests, doctests, formatting,
   warning-denied Clippy checks, dependency/feature checks, and generated-artifact
   verification applicable to each initiative are clean.

Stop and reconcile P01 before a later initiative when an earlier public contract
must materially change, a selected standards source contradicts the fixed profile,
the generator handoff is incomplete, a claimed-complete corpus case fails, an
ownership boundary would be crossed, external software acquisition would be
required without exact permission, or completion would retain or introduce
Surgeist-owned `unsafe`.

## P01.9 I02 Entry Reconciliation

The published I01 candidate at
`bc5394ff5855109dd1d224d29278d6ab601cef4f` satisfies its parser, recovery,
diagnostic, declaration, catalog-foundation, verification, and publication
predicates. Its static migration record nevertheless overstates one evolution
property: it says every public enum except `CssImportance` and
`CssSupportStatus` is non-exhaustive, while current source retains many
exhaustive public value enums required by I02 grammar closure. Representative
blocking types include media, selector, calculation, timing, Grid, and relative-
color models. Several coupled property variants also carry a payload type that
models only the I01 partial subset—for example `background`, `border-color`,
`gap`, timing, and position-bearing properties—and cannot express the complete
owning grammar by merely adding a sibling enum variant.

P01 rejects two superficially additive workarounds: leaving the published
partial payload as the parser's complete representation, and adding parallel
`V2` property variants for the same canonical property. Either would make the
authored model or property coupling untruthful. P01 therefore authorizes exactly
one intentionally breaking I02 foundation cycle with this boundary:

1. add `#[non_exhaustive]` to evolving public enums that I02 or a later
   compatible grammar addition can extend, while retaining only deliberately
   finite semantic states as closed;
2. replace insufficient I01 partial property/value payloads with one truthful
   property-specific authored model per canonical property, retaining the
   authoritative property schema and derived property/value coupling;
3. preserve the public `parse_sheet`, `parse_style_attribute`, `validate_sheet`,
   and `validate_style_attribute` signatures and their one-pass behavior;
4. preserve `CssParseReport`, typed diagnostics, source positions/spans,
   recovery-action meanings, diagnostic ordering, parser-owned construction,
   custom/substitution preservation, declaration importance, and ordinary/
   `app-strict` feature parity;
5. publish a superseding I02 migration record that names every affected public
   type and all root-owned facade, adapter, API-artifact, documentation, and test
   work. The I01 record remains immutable historical evidence and is not edited.

The I02 JIT specification must prove the exact affected type inventory and
replacement shapes before that cycle is planned. After the bounded foundation
cycle, the remaining I02 work is additive against the repaired evolution
surface. Any need to break a frozen item in points 3 or 4, weaken I01 evidence,
or create a second breaking cycle stops implementation and returns to P01
reconciliation. This repair does not authorize I03 production-API changes.

## P01.10 C07 Source-Contradiction Reconciliation

C06 is published and read back at
`597265b574be01c88a3ce559cc2bc07e02791da3`. JIT discovery for I02-C07 proved
that seven C01 oracle rows encode the two defects that reviewed findings 2.14
and 2.17 allocate to C07:

- `catalog.property.baseline.property.grid.positive`,
  `focused.property-schema.baseline.property.grid.important`, and
  `focused.property-schema.baseline.property.grid.ordinary` accept
  `repeat(auto-fit, 1fr)`, although dated Grid 2 restricts auto-repeat content
  to fixed sizes;
- `focused.importance.05`, `focused.importance.06`,
  `focused.nested-structural.keyframes-child-loss`, and
  `focused.structural.misc.03` cascade an invalid declaration into loss of its
  now-empty keyframe block and rule, although dated Animations 1 admits empty
  keyframe block and rule lists.

Preserving those observations would preserve the allocated defects and make the
P01.6 closure claim false. P01 therefore authorizes one source-backed behavioral
oracle correction in C07. The seven case IDs and authored inputs remain; their
expected retained syntax, projections, and diagnostics change only as required
by the dated Grid 2 and Animations 1 grammars. Every other C01 case and observable
remains byte-for-byte unchanged. C07 records the old and replacement fixture
digests in its reviewed plan and handoff, never in Rust tests, and independently
reviews the exact fixture diff. Existing duplicate stale assertions outside the
fixture are replaced with the same source-backed behavior rather than deleted or
masked.

This correction is not another public-API breaking cycle. The parser entry-point
signatures, report and recovery-action meanings, diagnostic ordering and source
coordinates, property/value coupling, and ordinary/`app-strict` relationship in
P01.9 points 3 and 4 remain frozen. Current Grid models are added alongside
source-compatible I01 payloads; parser-produced projections exist only for
conforming values. Keyframe structure becomes permissive exactly where the
owning grammar admits empty or duplicate authored occurrences, without sorting,
merging, cascading, or evaluating them. Any further C01-oracle contradiction or
public-contract break stops again for a new P01 reconciliation.

## P01.11 C08 Source-Contradiction Reconciliation

C07 is published and read back at
`21e33f121fd414c55bb229f0eab25ab41cfa7325`. JIT discovery for I02-C08 proved
that `focused.structured-errors.12` encodes a descriptor-duplicate behavior
that contradicts both the selected dated Fonts 3 Recommendation and the
reviewed C08 outcome. Its authored source contains two valid `font-family`
descriptors followed by the required valid `src` descriptor. The C01 oracle
expects the second family descriptor to recover with
`InvalidDescriptorCombination` and `DropDescriptor`, retaining the first
family. Fonts 3 section 4.1 instead requires every authored occurrence to be
parsed and makes the last declaration effective when one descriptor occurs
multiple times.

Preserving that observation would make Fonts 3 descriptor completion and the
P01.6 finding 2.1 closure claim false. P01 therefore authorizes one additional
source-backed behavioral-oracle correction in C08. The stable case ID, entry
point, feature mode, and authored input remain unchanged. Its expected report
becomes clean and retains the valid `@font-face` rule without the obsolete
duplicate diagnostic. The C08 plan records the old and replacement fixture
digests outside Rust tests, and task review verifies this exact one-row diff.
Every other fixture row remains byte-for-byte unchanged. Existing stale direct
duplicate-descriptor assertions are replaced with public behavior that proves
authored occurrence order and effective-last lookup; they are not deleted or
masked.

This correction changes no parser entry-point signature, report or recovery
meaning, diagnostic ordering or coordinate contract, property/value coupling,
or ordinary/`app-strict` relationship. The additive descriptor model preserves
valid occurrences in authored order and exposes the last valid occurrence
through the existing effective typed accessors. Invalid descriptor occurrences
still recover at the descriptor boundary and do not erase an earlier or later
valid occurrence. Any further C01-oracle contradiction or public-contract break
stops again for a new P01 reconciliation.

## P01.12 C09 Imported-Grammar Source Reconciliation

C08 is published and read back at
`129de7267726277b73d2cc15f1168c44c34ffcbc`. JIT discovery for I02-C09 proved
that the selected `X-VALUES4` URL cannot own the Conditional 3
`<general-enclosed>` delta allocated to C09. The immutable 12 March 2024 Values
4 Working Draft contains no `<general-enclosed>` production; csswg-drafts added
the generic boolean and `<general-enclosed>` grammar on 17 June 2024 in commit
`720ea2863696971ea6a6744e0f23acbb3e6936bd`, file
`css-values-4/Overview.bs`.

P01 therefore preserves the stable `X-VALUES4` source identity and Surgeist
extension tier while replacing its non-owning dated URL with that exact
repository revision and path. The selected production is only
`<general-enclosed>` as imported by the dated Conditional 3 grammar; no other
editor-draft Values 4 syntax enters the profile. This is a provenance correction,
not a moving-source exception: the commit and path are immutable and the
implementation remains bounded by the reviewed I02 atomic ledger row.

The correction changes no frozen I01 parser behavior, public contract, fixture,
official Snapshot source, dependency, feature, or repository boundary. C09 must
prove the exact function/parenthesis balanced-token grammar, authored retention,
and malformed-condition recovery through public parser behavior. Any need to
consume later boolean grammar beyond this one imported production, or any
further unresolved source ownership, stops again for P01 reconciliation.

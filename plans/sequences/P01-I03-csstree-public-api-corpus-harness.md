# P01-I03 CSSTree Public-API Corpus Harness Sequence

## 1 Sequence identity and authority

This is the reviewed implementation sequence for `P01-I03` in
`surgeist-css`. It is subordinate to the clean specification
`plans/specs/P01-I03-csstree-public-api-corpus-harness.md`, normalized semantic
SHA-256 `fe170c19763956c8160bc4d7fe398c3582ccc657cc8a81dbd9af492e29741e18`,
and the governing P01 program at semantic SHA-256
`87f6a94b893ffa416c6ff451575f0d5a21b4aa136e7bcd391cd6c0ce8810a2ae`.

The provider prerequisite is the already-published `surgeist-generator`
candidate `83a216880884a5a364258ffaaeaf93d228c0bc53`, used without sibling
edits. The leaf entry candidate is I02 commit
`a8a6f00bd9f49464dfdef24f0feba9fdff705189`. The pinned source checkout and
generator census are fixed by the specification: revision
`88e3d965c0b1628642a30a841745b410d6835052`, tree
`bfadc7a7a8d93dce59a27fa7df3bb0f6f6a623d8`, 74 files, 935 cases, 721 parsed,
and 214 rejected.

This sequence orders implementation boundaries only. It contains no task
steps, code outlines, test matrices, commit choreography, or later-cycle plan.
Only the next cycle plan may be authored after its predecessor is published,
read back, and handed off.

## 2 Shared sequence invariants

- The committed corpus is leaf-owned but governed by the generator's neutral
  artifact contract. The generator remains the sole owner of import, neutral
  generation, provenance, reports, and offline checking.
- Production parser behavior, public API, dependency direction, and I02
  semantics remain unchanged. Root and sibling repositories are not edited.
- Every corpus case has one explicit opaque ID, one context, one adapter/probe,
  and one oracle outcome. No default adapter, source-derived test, metadata
  proxy, owner-set/count proxy, subprocess, JavaScript, browser, network, or
  production test hook is permitted. The existing pinned `tmp/csstree`
  checkout is allowed only for provider-backed artifact maintenance; ordinary
  Cargo tests and the public harness never inspect a checkout or source tree.
- Neutral generated disposition and CSS oracle outcome remain separate: all 935
  neutral cases are active, while the four CSS outcomes partition the same IDs.
- Claimed-complete cases remain active evidence when they fail. Unsupported is
  finite, explicit, executed, and never a mask for an implementation defect.
- The complete I03 feature and artifact contract remains the specification's
  authority; each cycle preserves its exact public observation baseline,
  payload spans, typed diagnostics, strict parity, attribution, and no-unsafe
  invariants.

## 3 Ordered cycles

### P01-I03-C01 — Adopt the pinned neutral corpus

- **Owning repository:** `surgeist-css`.
- **Specification sections:** §§1–3, §8, and the artifact portions of §9.
- **Prerequisites:** clean I03 specification; published I02 leaf candidate;
  published generator candidate; existing pinned `tmp/csstree` checkout.
- **Entry state:** the leaf has no committed CSSTree corpus or I03 consumer;
  the provider's public import/generate/check contract is available as-is.
- **Bounded outcome:** commit the exact manifest, attribution, source sidecar,
  74 imported fixture files, 74 neutral expectation files, and full report;
  add the private validated neutral-record/fixture inventory boundary without
  invoking the parser or inferring conformance from implementation metadata.
- **Exit evidence:** the adopted artifacts prove the fixed source identity,
  canonical bytes/provenance, 74/935/721/214 census, active-neutral status,
  and a loader-ready schema for the next cycle. The published generator
  candidate's offline `check-corpus` passes against the adopted artifact set.
- **Handoff:** publish the C01 candidate and return its artifact/provenance
  digest and clean state to C02. No oracle or parser harness is claimed yet.

### P01-I03-C02 — Define explicit probes and the CSS oracle

- **Owning repository:** `surgeist-css`.
- **Specification sections:** §§4–5, §7, and §8.
- **Prerequisites:** published/read-back C01 candidate; unchanged clean
  specification and provider artifact contract.
- **Entry state:** neutral records load and the complete fixture inventory is
  immutable, but no CSS-owned adapter registry or per-case outcome authority
  exists.
- **Bounded outcome:** add the closed tagged active/panic-freedom probe model,
  exhaustive 74-path adapter registry, public extractor semantics, exact
  observation baseline, payload-span rules, and one explicit oracle record for
  every case ID. This cycle does not execute the full corpus or alter parser
  production code.
- **Exit evidence:** loader validation proves the oracle/report bijection,
  exact source/context/options binding, neutral/oracle count separation,
  finite unsupported reasons/policies, and complete adapter coverage.
- **Handoff:** publish the C02 candidate and hand the immutable corpus/oracle
  contract to C03. Any untruthful adapter or claimed-complete mismatch remains
  an active stop condition.

### P01-I03-C03 — Execute the public harness and close I03

- **Owning repository:** `surgeist-css`.
- **Specification sections:** §§4–9.
- **Prerequisites:** published/read-back C02 candidate; unchanged I02 public
  parser contract; unchanged provider candidate and pinned artifacts.
- **Entry state:** every case has a validated neutral record, explicit probe,
  extractor, and expected observation, but no complete public execution or
  cross-feature evidence is committed.
- **Bounded outcome:** add the context-family public integration harness with
  panic capture, deterministic mismatch aggregation, exact default and
  `app-strict` observation/parity checks, and the maintenance documentation
  needed to reproduce the provider-backed artifact check.
- **Exit evidence:** all 935 cases execute exactly once; active and
  unsupported outcomes partition the IDs; exact context/census and report
  digest checks pass; the provider's offline `check-corpus` is revalidated;
  the default/app-strict common baseline agrees; focused crate checks remain
  green; and no source-parsing test or production change was introduced.
- **Handoff:** after the final cycle review, cleanup, publication, and remote
  readback, return the immutable I03 leaf candidate with provider SHA, corpus
  census, artifact digests, public-only harness evidence, feature parity,
  no-unsafe result, and root-only follow-up notes.

## 4 Sequence completion

I03 is complete only after C03 is landed on leaf `main`, its configured final
checks pass, the candidate is published and read back from the leaf authority
remote, and the complete provider-backed handoff
is recorded. A source/tree drift, license change, missing offline artifact,
untruthful adapter, claimed-complete corpus mismatch, public-API requirement,
unauthorized acquisition, or unsafe finding returns work to the owning cycle or
specification; it never authorizes narrowing the corpus or weakening the oracle.

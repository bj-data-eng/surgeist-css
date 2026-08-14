# Pinned CSSTree corpus

This directory adopts the neutral CSS fixture corpus generated from CSSTree
commit `88e3d965c0b1628642a30a841745b410d6835052`. The pinned
`fixtures/ast` tree is `bfadc7a7a8d93dce59a27fa7df3bb0f6f6a623d8` and contains
935 cases across 74 fixture files (721 upstream-parsed and 214
upstream-rejected cases).

CSSTree is distributed under the MIT License. `LICENSE` is the unchanged
license notice copied from that pinned checkout.

The source sidecar, imported fixtures, neutral expectations, generation report,
and manifest are produced by the published `surgeist-generator` candidate
`83a216880884a5a364258ffaaeaf93d228c0bc53`. They are an indivisible generated
artifact set and must not be edited by hand.

Corpus maintenance runs the provider's offline `import-csstree`, `generate`, and
`check-corpus` commands against the pinned checkout in a disposable owner root,
then adopts only the generated manifest, source, expectations, and report along
with the unchanged license. Ordinary `surgeist-css` tests consume the committed
artifacts and do not invoke the generator or access an upstream checkout.

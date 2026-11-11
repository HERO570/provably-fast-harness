# Source brief: whole-proof optimization

These are primary sources selected for the current frontier work. They inform hypotheses only; every patch still requires byte-identical proofs, independent verification, and evaluator timing.

1. [Rayon FAQ](https://github.com/rayon-rs/rayon/blob/main/FAQ.md) — documents work stealing and `join` scheduling. Hypothesis: preserve deterministic append order while exposing only independent, sufficiently coarse tasks to Rayon; avoid moving protocol-visible variable creation across task boundaries.
2. [Rayon implementation](https://github.com/rayon-rs/rayon) — authoritative implementation and API surface for scoped parallelism and parallel iterators. Hypothesis: use indexed parallel work with deterministic reduction/append points, not cross-trace speculative construction when allocation order is proof-visible.
3. [RFC 7693: BLAKE2](https://www.rfc-editor.org/rfc/rfc7693) — normative BLAKE2s specification. Hypothesis: SIMD changes must preserve lane layout, rotation semantics, and little-endian interpretation; optimize transpose/compression only behind differential tests.
4. [Cache-oblivious vs. cache-conscious algorithms](https://doi.org/10.1145/1248377.1248394) — primary study of memory locality and FFT-like workloads. Hypothesis: tune buffer lifetime and transform tiling around the proof's largest FFT/commit phases, measuring cache misses and peak RSS together.
5. [Stwo source repository](https://github.com/starkware-libs/stwo) — upstream architecture and production constraints. Hypothesis: keep verifier soundness and serializer behavior unchanged; focus on prover-side scheduling, reuse, and SIMD paths with accept/reject tests.

## Immediate experiment

The first source-informed scheduler prototype moved the first Merkle query across traces and changed proof bytes, so it is rejected. The next safe experiment must retain the original trace-major variable-creation order and optimize only work that is already independent inside that order. The allocator-budget variant was byte-identical but slower locally (1.0043x), so it is not a submission candidate.

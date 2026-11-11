# High-signal prover sources for the next point-seeking cycle

These are implementation-oriented sources already represented in the challenge-2 graph. They are more actionable than generic STARK references because each maps to a concrete prover hot path.

1. **Circle FFT fusion and cache-resident passes** — [Plonky3 PR #1796](https://github.com/Plonky3/Plonky3/pull/1796), graph source `rgr1_5d37217b47771b9be1a22b138cbc58195754a0923eb9f37da6aa3db269d15890`.
   - Target: `crates/stwo/src/prover/backend/simd/circle.rs`, `poly/circle/ops.rs`.
   - Test: fuse only mathematically independent FFT stages; compare coefficients and full proof bytes.

2. **Blocked Merkle construction with register-tile transposes** — [Plonky3 PR #2335](https://github.com/Plonky3/Plonky3/pull/2335), graph source `rgr1_bfe2441d01fd08feb9a7df925d71d4f69f6a1acbc0882011ea61af62ecbc7332`.
   - Target: `crates/stwo/src/prover/vcs_lifted/prover.rs` and SIMD Blake2s batching.
   - Test: preserve row/leaf ordering exactly while processing bounded subtrees; use differential Merkle-root tests.

3. **Reuse quotient inverses** — [Plonky3 PR #549](https://github.com/Plonky3/Plonky3/pull/549), graph source `rgr1_b1e23e261821c6e39df9c5beb32c82a925a020780c0475d75d0f812c78d7c694`.
   - Target: `crates/stwo/src/prover/pcs/quotient_ops.rs` and composition evaluation.
   - Test: cache denominator inverses only within one commitment/composition lifetime; verify no cross-statement state leaks.

4. **Row-oriented coset LDE** — [Plonky3 PR #440](https://github.com/Plonky3/Plonky3/pull/440), graph source `rgr1_fa0ccf9bcf74705ebb4a9037ab6b6c7b82f8b0d4fb86fe5b1c805cc64e089904`.
   - Target: `crates/stwo/src/prover/backend/simd/circle.rs` and column layout paths.
   - Test: compare bit-reversed evaluation buffers and all commitment roots before considering a whole-proof screen.

5. **Cache-blocked bit reversal** — [Stwo issue #813](https://github.com/starkware-libs/stwo/issues/813), graph source `rgr1_d9126ef0f2cd470b8fe0e46dd667979324c8917939757ca0488d85532b636ce2`.
   - Target: bit-reversal/permutation code in the SIMD circle/column backend.
   - Test: benchmark only the largest transforms first; retain the portable `std::simd` path and exact output ordering.

## Priority

Start with blocked Merkle construction and cache-blocked bit reversal. They address large, repeated memory walks and can be isolated without changing protocol parameters. Treat FFT fusion and row-oriented LDE as higher-risk alternatives. Do not stack any of them until an isolated screen shows a real whole-proof gain.

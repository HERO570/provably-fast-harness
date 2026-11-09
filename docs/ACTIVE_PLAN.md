# Active research plan

This plan is based on the current proving tree, not the bare pinned commit. The frontier already contains substantial work in `crates/stwo/src/prover/pcs/mod.rs`, `crates/stwo/src/prover/backend/simd/{circle,column}.rs`, `crates/prover/src/prover.rs`, and the Cairo claim generator. New work must avoid re-implementing those changes.

## Track 1 — reproduce and audit the retained-column stack

**Code surfaces**

- `crates/stwo/src/prover/pcs/mod.rs`
- `crates/stwo/src/prover/air/component_prover.rs`
- `crates/stwo/src/prover/backend/simd/column.rs`
- `crates/stwo/src/prover/backend/simd/circle.rs`
- `crates/prover/src/prover.rs`

**Questions**

- Which pair-only columns retain stripe 0 through the circuit composition?
- Are any retained columns live through the Cairo peak unnecessarily?
- Can residency be selected by last-read time and measured recomputation cost?

**Success condition**

At least 1% whole-proof improvement, byte-identical proof, and no more than the frontier peak memory. Otherwise record the measured refutation.

## Track 2 — fill a remaining idle window

**Code surfaces**

- `crates/prover/src/prover.rs`
- `crates/prover/src/witness/cairo_claim_generator.rs`
- `crates/circuit_prover/src/prover.rs`
- `crates/constraint-framework/src/prover/component_prover.rs`

**Candidates**

- Verifier-fill pair FFTs while fill tasks are open.
- FRI-input sums in parallel pieces, with a constant-producing fallback.
- Cairo base interpolation only where the writer pool has real idle workers.
- Circuit `eq`/`qm31_ops` trace production overlapped with gate traces.

**Guardrail**

Do not parallelize work merely because it is divisible. Require CPU-busy evidence that the work is exposed on the critical path and a live-set calculation before editing.

## Track 3 — setup and cached decommitment

**Code surfaces**

- `crates/privacy_prove/src/pvfast_workload.rs`
- `crates/stwo_cairo_adapter/src/` (if included by the source manifest)
- `crates/prover/src/cairo_preprocessed_cache.rs`
- `crates/stwo/src/prover/backend/simd/circle.rs`

**Candidates**

- Parse large JSON arrays concurrently with twiddle/precompute construction.
- Share inverse fold layers across aligned cached-column row runs.
- Read sourced columns directly from cached stripes instead of regrowing them.

**Fallback rule**

Any escaped, duplicated, malformed, or non-whole-array input must use the original parser. Differential tests must compare the resulting `ProverInput` exactly.

## Track 4 — low-priority arithmetic

Benchmark only after Tracks 1–3 are exhausted:

- LogUp prefix-sum tails.
- Lazy-reduced SIMD dot products.
- Batch twiddle inversion.
- Fused circle-to-line folds.

These are worthwhile only when phase evidence shows they can move whole-proof wall time.

## Current execution order

1. Run the frontier locally and collect phase/memory evidence.
2. Audit which Track 1 mechanisms are already present and which are partial.
3. Add one Track 2 or Track 3 mechanism at a time.
4. Run byte-identity and verifier checks before timing.
5. Use alternating fresh-input pairs and discard changes whose confidence interval does not clear 1%.
6. Record the lead and result in the graph before submitting.


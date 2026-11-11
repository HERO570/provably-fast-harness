# Protocol: parallel shared evaluation-basis construction

## Hypothesis

`SimdBackend::eval_at_point_group` builds the shared high-word basis sequentially before its parallel coefficient dots. Filling that same basis in independent fixed chunks will remove an exposed serial QM31 multiplication chain without changing dot-product reduction order, proof bytes, or peak memory.

## Mechanism

- Surface: `crates/stwo/src/prover/backend/simd/circle.rs`
- Compute the existing twiddle-step recurrence once.
- Split the already allocated basis into fixed chunks.
- For each chunk, compute its exact starting factor with `twiddle_at`, then fill consecutive entries with `advance_twiddle`.
- Keep the existing sequential construction for non-parallel builds.
- Keep coefficient dot products and their Rayon reduction unchanged.

## Correctness gate

1. Add a differential test comparing the parallel basis to the existing doubling construction across random points, log sizes, and chunk boundaries.
2. Run the relevant Stwo prover tests in release mode.
3. Build the complete candidate with evaluator flags.
4. Require byte-identical proofs and independent verification on every measured statement.

## Performance gate

- Initial gross-regression screen: two alternating fresh-input pairs.
- Promising candidate: ten alternating fresh-input pairs.
- Do not submit unless the ratio plausibly clears 0.99 on a quiet or evaluator-like host.
- No new proof-resident allocation is allowed; the basis vector already exists, and the added twiddle-step vector is `O(log n)`.

## Lineage check

Grouped coefficient dots were previously reported as a whole-proof null on an older frontier. This experiment changes a different substage—the serial shared-basis producer—and is justified only because frontier `69645ee0` made the grouped dot itself much faster, potentially exposing setup as a larger fraction. If the complete proof remains neutral, record it as a current-frontier refutation.

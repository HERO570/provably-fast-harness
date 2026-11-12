# Parallel shared evaluation-basis construction — 2026-11-12

## Hypothesis

`SimdBackend::eval_at_point_group` builds a shared QM31 basis sequentially. Filling fixed slots in 4096-entry Rayon chunks, using the existing exact twiddle-step recurrence, could remove an exposed serial tail.

## Correctness

- Added a standalone differential test for seeds 1 and 21903 at log sizes 9, 12, 16, 17, and 21, including the 4096-entry chunk boundary.
- Test passed.
- Whole-proof candidate compiled with evaluator flags.
- Both fresh proofs were byte-identical to frontier and independently verified.

## Measurement

Darwin arm64, 8 logical CPUs; the local result is only a gross-regression signal, not an evaluator prediction.

- Warm-up: frontier 7.12 s, candidate 7.70 s.
- Pair 1: frontier 7.46 s, candidate 7.49 s, ratio 1.0030.
- Pair 2: frontier 6.93 s, candidate 7.02 s, ratio 1.0123.
- Two-pair geometric ratio: **1.0076**, 95% interval 0.9505–1.0683.

## Decision

Stop and do not submit. The implementation is exact, but adding Rayon chunk starts and a twiddle-step vector did not beat the frontier even before a full ten-pair screen. A future attempt would need x86 evidence or a different mechanism that removes setup overhead.

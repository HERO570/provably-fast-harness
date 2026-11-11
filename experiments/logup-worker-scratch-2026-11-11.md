# Worker-local LogUp scratch reuse — 2026-11-11

## Hypothesis

`LogupTraceGenerator::write_frac_cols` allocates four 128-row `Vec<PackedSecureField>` buffers for every Rayon chunk. Reusing those buffers per worker with `for_each_init` should reduce allocator traffic without changing arithmetic or output order.

## Patch

- Surface: `crates/constraint-framework/src/prover/logup.rs`
- Candidate delta versus frontier: approximately 3.7 KB
- Mechanism: allocate numerator, denominator, inverse, and running-sum buffers once per worker per call, then reuse their prefixes for every assigned chunk.
- Memory model: no retained proof columns and no changed data lifetime; about 128 KiB scratch per active worker, as before, but without per-task heap allocation.

## Correctness

- Candidate compiled with evaluator flags.
- Every measured proof was byte-identical to frontier for the same fresh statement.
- Independent `privacy_verify` accepted every candidate proof.

## Measurement

Host: Darwin arm64, 8 logical CPUs. The ten-pair run warned that load average was 14, so the confidence interval is wide and cannot predict the Linux x86-64 evaluator.

- Initial two-pair screen: 0.9219, CI 0.5125–1.6584.
- Predeclared ten-pair gate: **1.0005**, 95% CI **0.9398–1.0651**.
- Pair ratios: 0.9738, 0.9618, 1.1435, 0.8722, 0.8882, 1.0544, 1.0113, 0.9918, 1.1178, 1.0246.
- Peak memory: unavailable from the local timer.

Median phase changes were small and inconsistent. Cairo interaction was -1.4%, circuit interaction -0.7%, and circuit base commit -2.2%, while several unrelated phases moved in both directions. This is consistent with host noise, not a robust whole-proof gain.

## Decision

Do not submit. Preserve the result as an inconclusive/neutral local experiment and move to a distinct serial evaluation-basis mechanism. An x86 retest is justified only if it can run cheaply alongside a stronger candidate.

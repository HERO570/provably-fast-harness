# Research cycle — 2026-11-11

## Evaluator result

- Job: `2697f895-6f89-4405-bed7-5de329dda1e6`
- Lead: `rgr1_a006a75428e5e2d23da79945fddfb1003af8197f51975ef3039ab7387e3029e3`
- Result: `VALID_NOT_FASTER / STOPPED_NOT_FASTER`
- Ratio: `1.0448` (95% CI `1.0361–1.0537`), six clean pairs
- Peak: 2.78 GiB versus frontier 2.77 GiB
- Interpretation: increasing lifted-Merkle `FUSED_LEVELS` from 5 to 6 preserved proof bytes and verification, but was 4.5% slower. Do not retry this mechanism on the same frontier.

The regression showed lower IPC (1.58 versus 1.62), more branch misses (0.93 versus 0.80 per 1,000 instructions), and larger tail regressions than commitment-phase gains. The change was reverted.

## Primary sources proposed

Five non-duplicate implementation sources were proposed to challenge 2:

1. `1a57387e-20b9-4783-b393-e88aa16bdc9d` — Plonky3 PR #2057, contiguous staging for same-height multi-matrix Merkle hashing.
2. `b6b9e82e-b138-4418-bef4-d98e8c41f579` — Plonky3 PR #2194, retaining SIMD lane groups beyond scratch budgets.
3. `9308b6f0-9bca-4b8a-9a92-79cff6a9c48e` — Plonky3 PR #831, efficient PCS coset interpolation.
4. `a8aa5a60-8cd7-48ed-81a7-5d3b2a0bd4d9` — Plonky3 PR #869, Rayon square-transpose acceleration.
5. `e2def24a-7337-495f-95db-0c3e2a755b52` — Plonky3 PR #890, improved small-batch DFT parallelism.

Sources improve the research graph and provenance. They do not themselves establish a speed result; evaluator points require a valid patch that clears the 1% whole-proof threshold.

## Local screen

A one-line grouped-OODS task-grain experiment changed the Rayon chunk from 4,096 to 2,048 packed words. The intent was to increase task count in log-20 tails where evaluator profiles showed idle cores.

- Proof bytes: identical
- Independent verification: passed
- Host: Darwin arm64, load average 33 on 8 CPUs
- Warm-up ratio: 1.3690
- Pair ratios before stopping: 1.6592, 1.3310

The screen was stopped early and reverted. The host was too loaded for precise estimates, but the regression was far too large to justify an evaluator job.

## Current direction

Avoid generic readiness scheduling: the graph already contains several overlapping or refuted attempts. The strongest remaining direction is a narrow, bounded transform change that performs multiple FFT rounds within each task, reducing inter-thread transfers while preserving circle ordering and proof bytes. Any implementation must first pass differential transform tests and a 10-pair fresh-statement screen on a quiet host.

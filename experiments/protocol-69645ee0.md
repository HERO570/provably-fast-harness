# Experiment protocol: frontier 69645ee0

## Objective

Find a compact, independent mechanism that improves the complete proof by at least 1% versus frontier `69645ee0`, or by at least 0.8% with a credible path to combine with the current 0.64% near-miss.

## Baseline

- Pinned repository commit: `9df00f2249b61849be69cc14942742d0c1f81ad2`
- Applied frontier commit: `69645ee0e6f583990c4aaf064cd08fde601ff009`
- Worktree: `/Users/darshan/starks-challenge/frontier-69645ee0`
- Evaluator frontier peak: approximately 2.78 GiB

## Pre-edit audit

1. Build the current frontier with `nightly-2026-01-15`.
2. Produce and independently verify a proof.
3. Record all `pvfast-phase` timings and peak memory.
4. Inspect transform/evaluation call counts, allocation sites, and serial tails.
5. Check graph ancestry, citations, and combinations for any mechanism selected.

## Candidate gate

A candidate proceeds only if it satisfies all of the following:

- one primary mechanism;
- no proof-parameter or verifier-check changes;
- byte-identical proof on the same statement;
- independent verification passes;
- no new thread source and no policy violation;
- local peak memory no higher than baseline beyond measurement noise;
- alternating fresh-input ratio plausibly below 0.99, or below 0.992 with a documented independent combination path.

## Measurement

- Use fresh statements and alternate frontier/candidate order.
- Stop gross regressions after two clean pairs.
- Run at least six clean pairs for a promising candidate; target ten on a quiet host.
- Save medians by phase, wall time, CPU time, peak RSS, proof digest, and verifier result.
- Treat the local ARM machine as a correctness and gross-regression gate; evaluator x86-64 AVX2 evidence is authoritative.

## Submission workflow

1. Record a challenge-2 lead before evaluator submission.
2. Cite exact parent leads, experiments, and sources with typed edges.
3. Submit with the lead ID as soon as it is admitted.
4. Read evidence back, publish failures, and update `research-state.yaml`, `research-log.md`, and `findings.md`.

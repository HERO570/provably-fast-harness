# Active research plan

Last updated 2026-11-17. Frontier: branch `mainnet-season` at commit `69645ee0e6f583990c4aaf064cd08fde601ff009`, tree `07cb0abaa4d0a4b7d78a8e8050aa29b3c5a4a689`; pinned HEAD `9df00f2249b61849be69cc14942742d0c1f81ad2`.

## Current work

An isolated pure-frontier candidate reuses row-target metadata within each striped decommit call. Independent source review cleared the bounded cache and fallback; three focused tests passed on Nov 17 with unchanged source hashes. One native release build is running, followed conditionally by ten fresh alternating proof pairs using the unchanged client. See the [experiment record](EXPERIMENT_PER_DECOMMIT_TARGETS.md) for graph identifiers, bounds, and evidence limits.

The old quotient diagnostic remains preserved separately. Its join/lift call-graph review found no supported redundant computation; its patch size and profiling results do not describe the new candidate.

## Next steps

1. Finish the existing source-bound release build and ten-pair measurement pipeline. Stop on build/test failure, mismatched proofs, verifier failure, source drift, or unavailable memory evidence.
2. Review the complete local result: exact bytes, independent verification, time ratio and confidence interval, median peak memory, and phase timings. ARM timing remains advisory for the Linux AVX2 evaluator.
3. If the evidence passes, exclude the hash-verified temporary helper, run canonical policy preflight, and submit using proposal `20583edf-7fdc-435a-93ff-c31eb1ae458e`. Read the evaluator result back into the graph. Otherwise record the negative or unresolved result without an unchanged rerun.
4. Keep one build owner and freeze source during measurements. Preserve the separate old diagnostic and failed candidates. The derivative inverse-scatter-map lead is untested; do not stack it into this experiment before measuring the isolated change.
5. After each material experiment or evaluator result, update the dated ledger, findings, research state, and append-only research log in GitHub with identifiers, evidence limits, and the next decision. Review the diff before committing; keep proof inputs/artifacts, credentials, binaries, and large snapshots out of the public repository.

## Guardrails

- Keep frontier and exact lineage explicit; no promotion or speed points are confirmed.
- Treat official evaluator results and local diagnostics as separate evidence classes.
- Do not claim standings points for this STWO session or promise future points.
- The old lookup run's peak memory is unavailable. The authorized `/usr/bin/time -l true` timer probe now succeeds; macOS denied `kern.clockrate` during the earlier diagnosis. No client change is indicated.
- Unknown or unpublished raw logs remain unknown; do not reconstruct them from summaries.

See the [2026-11-16 dated evidence ledger](../experiments/research-update-2026-11-16.md).

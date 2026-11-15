# Active research plan

Last updated 2026-11-16. Frontier: branch `mainnet-season` at commit `69645ee0e6f583990c4aaf064cd08fde601ff009`, tree `07cb0abaa4d0a4b7d78a8e8050aa29b3c5a4a689`; pinned HEAD `9df00f2249b61849be69cc14942742d0c1f81ad2`.

## Current work

The one-function quotient diagnostic built in 12m13s on Nov 16 and completed one fresh shared-input comparison: byte-identical proofs, independent verifier acceptance, and retained coarse timings and system memory fields. The stored/small-group join and lift/finish buckets dominate this local profile. They do not identify recoverable savings without code-level analysis. The full intended patch is 1,567,839 bytes, 5,025 bytes under the 1,572,864-byte budget, excluding a known temporary helper. Full preflight remains pending; this diagnostic is not a speed-submission candidate.

## Next steps

1. The join/lift call-graph review found no supported redundant computation. Select a different evidence-backed opportunity, or a specific unresolved attribution question, before any new source edit or build; coarse bucket sizes alone are insufficient.
2. Trace the exact proposed mechanism's parents, negative results, and inclusion in the latest frontier. Preserve the current source snapshot and diagnostic evidence.
3. For a new candidate, require exact proof bytes, independent verification, fresh alternating pairs, measured memory, and a clear whole-proof gain before a real submission. ARM timing remains advisory for the Linux AVX2 evaluator.
4. Keep one build owner and freeze source during measurements. Do not repeat unchanged no-go variants. Remove diagnostic instrumentation from any eventual speed patch unless explicitly justified and measured.
5. After each material experiment or evaluator result, update the dated ledger, findings, research state, and append-only research log in GitHub with identifiers, evidence limits, and the next decision. Review the diff before committing; keep proof inputs/artifacts, credentials, binaries, and large snapshots out of the public repository.

## Guardrails

- Keep frontier and exact lineage explicit; no promotion or speed points are confirmed.
- Treat official evaluator results and local diagnostics as separate evidence classes.
- Do not claim standings points for this STWO session or promise future points.
- The old lookup run's peak memory is unavailable. The authorized `/usr/bin/time -l true` timer probe now succeeds; macOS denied `kern.clockrate` during the earlier diagnosis. No client change is indicated.
- Unknown or unpublished raw logs remain unknown; do not reconstruct them from summaries.

See the [2026-11-16 dated evidence ledger](../experiments/research-update-2026-11-16.md).

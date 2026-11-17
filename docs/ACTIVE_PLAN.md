# Active research plan

Last updated 2026-11-17. Frontier: branch `mainnet-season` at commit `69645ee0e6f583990c4aaf064cd08fde601ff009`, tree `07cb0abaa4d0a4b7d78a8e8050aa29b3c5a4a689`; pinned HEAD `9df00f2249b61849be69cc14942742d0c1f81ad2`.

## Current work

The target-reuse candidate completed its [replacement ten-pair screen](../experiments/target-reuse-result-2026-11-17.md). All proofs were exact and independently verified; exit status was 0. Time ratio 1.0087 (95% interval 0.9726–1.0462) and median peak +0.6% establish no qualifying local gain. Coordinator and independent review agree that this is local inconclusive evidence, not an evaluator refutation. The source and attributable binary remain preserved; no submission is justified by this run.

The old quotient diagnostic remains preserved separately. Its join/lift call-graph review found no supported redundant computation; its patch size and profiling results do not describe the new candidate.

The [Nov 17 Stwo operator audit](../experiments/stwo-operator-audit-2026-11-17.md) changes the selection target: distinguish the promoted source frontier from the best published eligible result, and read exact supported LogUp/writer-stack evidence before selecting another mechanism. The saved record target is below 0.9722888964 candidate/frontier; refresh it before use. The new offline helper passes eight tests. No proving source, proof run or evaluator job changed during this audit. A separate source review confirms the Cairo cache guard samples words; mutation/fallback coverage remains a correctness question, with no accepted-invalid-proof finding.

## Next steps

The [operational experiment plan](NEXT_EXPERIMENT.md) and [untested inverse-scatter design](INVERSE_SCATTER_HYPOTHESIS.md) expand the steps below for independent reproduction and review.

1. Identify an existing quiet Linux x86-64 AVX2 host suitable for profiling and paired measurement. The bounded Herdr machine inventory is empty; this does not rule out unlisted infrastructure. No new host provisioning or spending is authorized.
2. Refresh the official frontier and published record target. Obtain the exact supported fixed-table LogUp/writer-hand-off stack payload and check source inclusion/lineage before choosing a distinct increment. Profile the chosen path on the matched host before another patch build. Mixed coarse phase medians on loaded ARM do not identify recoverable savings.
3. Keep inverse scatter-map reuse as a conditional source-level hypothesis rather than the default next patch. Another contributor now reports a speed-unestablished local screen (graph sequence 1065); exact novelty and evaluator effect remain unknown. Review actual workload counts, combined metadata capacity and resident-suffix invariants before any implementation. It has no transferred support.
4. Keep the current candidate, full log, earlier partial capture, old diagnostic and failed candidates separate. No unchanged ARM rerun or evaluator submission follows from this screen. A future qualifying patch must pass the full proof, memory, timing and final policy gates before submission.
5. After each material experiment or evaluator result, update the dated ledger, findings, research state, and append-only research log in GitHub with identifiers, evidence limits, and the next decision. Review the diff before committing; keep proof inputs/artifacts, credentials, binaries, and large snapshots out of the public repository.

## Guardrails

- Keep frontier and exact lineage explicit; no promotion or speed points are confirmed.
- Treat official evaluator results and local diagnostics as separate evidence classes.
- Do not claim standings points for this STWO session or promise future points.
- The old lookup run's peak memory is unavailable. The authorized `/usr/bin/time -l true` timer probe now succeeds; macOS denied `kern.clockrate` during the earlier diagnosis. No client change is indicated.
- Unknown or unpublished raw logs remain unknown; do not reconstruct them from summaries.

See the [2026-11-16 dated evidence ledger](../experiments/research-update-2026-11-16.md).

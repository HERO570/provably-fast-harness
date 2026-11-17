# Research target: sub-1.3 GiB mainnet peak

Requested 2026-11-17. This is a proposed memory experiment, not a measured result. Proof parameters, verifier checks and proof-byte identity remain fixed. The target means whole-process peak under the evaluator's memory measurement, not an allocator counter or configured budget.

## Why investigate it

The captured mainnet frontier peak is 2,981,707,776 bytes, about 2.78 GiB, in the latest saved best-result comparison. The workload sets a 3,000 MiB allocator budget. Historical contributor measurements found much of the peak in idle cached blocks, so reclaiming idle memory is a plausible first lever. They do not prove the current frontier has the same live/idle split or can reach 1.3 GiB.

The current mainnet board scores time. A memory reduction alone is still useful research and an operational improvement, but does not establish a new time record. The earlier approximately 1.04 GiB result belongs to another season/settings; it neither proves nor disproves sub-1.3 GiB at today's mainnet parameters. See [scoring guidance](SCORING.md).

## What the current source establishes

- [`pvfast_workload.rs`](https://github.com/starknet-innovation/proving/blob/69645ee0e6f583990c4aaf064cd08fde601ff009/crates/privacy_prove/src/pvfast_workload.rs#L42) sets `LargeBlockCache::set_idle_budget(3000 << 20)`.
- [`column.rs`](https://github.com/starknet-innovation/proving/blob/69645ee0e6f583990c4aaf064cd08fde601ff009/crates/stwo/src/prover/backend/simd/column.rs#L982) retains freed blocks subject to live-plus-idle accounting. Live allocations remain permitted even when they exceed the budget. Small-allocation balances are buffered per thread. The budget is therefore not a hard RSS or peak limit.
- Cairo/circuit stripe pairs remain live through composition to avoid repeated transforms. Pair release after composition and other precompute lifetimes already exist on the frontier. Turning them off may lower live memory while adding regrowth and page faults; it is not a free reduction.
- The [historical step-three report](https://stwo.provably.fast/threads/bt1_30d412f8dcf8bfedb3a2b482) describes cap-only memory reductions with nearly unchanged local time, plus first-touch penalties in other low-memory/retention combinations. Its reported successful page-reuse/trim combination is evidence about that older stack, not an isolated component or a current-frontier prediction.

No current phase-by-phase live-memory peak or rigorous 1.3 GiB feasibility bound has been established by this audit.

## First experiment: attribute the live/idle boundary

1. Preserve the existing candidate and create a separately identified diagnostic against the exact current frontier. Change the existing idle budget to zero, leaving stripe policies, proof arithmetic and parameters unchanged. Zero budget requests release of freed large blocks; it does not discard live proof data or guarantee a process cap.
2. If needed, expose bounded snapshots of the existing live/idle counters at coarse phase boundaries. Avoid adding hot-path counters or treating approximate allocator totals as the kernel's process peak. Keep profiling overhead separate from any claimed performance measurement.
3. Use fresh matched Linux x86-64 AVX2 baseline/diagnostic builds on an existing suitably quiet host. One shared-input pair can attribute the memory components; compare proof bytes and verify both proofs independently. Capture peak memory, phase time, page faults and raw logs. No host is currently configured in our bounded inventory; no provisioning or spending is implied.
4. If idle bytes fall near zero while peak stays above 1.3 GiB, identify live overlap or non-cache retention before another budget tweak. Consider one precisely identified lifetime/reuse change, with its memory estimate and recomputation cost. If the diagnostic reaches the target but is slower, report that tradeoff and use the evidence to select a less aggressive phase-aware policy.
5. Only a subsequent complete fresh-input alternating run with the challenge's correctness, memory, timing and policy checks can qualify a patch. Publish the diagnostic even if the target is missed or time regresses. A one-pair attribution run is not a performance acceptance gate.

This plan adds the user's memory target to the research queue. No allocator/source change, build, proof run, new memory record or time gain has been made by this source-only assessment.

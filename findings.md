# Research Findings

## Current update — 2026-11-16

Frontier branch `mainnet-season` is at `69645ee0e6f583990c4aaf064cd08fde601ff009` (tree `07cb0abaa4d0a4b7d78a8e8050aa29b3c5a4a689`), last checked Nov 16; pinned HEAD is `9df00f2249b61849be69cc14942742d0c1f81ad2`. No speed points or promotion are confirmed. Official four-small-Cairo-trace-writer batching was inconclusive (0.9924, CI 0.9869–0.9980; 2.78 vs 2.77 GiB). The official four-writer plus one-pass Pedersen combination was valid but not faster enough (0.9949, CI 0.9907–0.9992; ten clean pairs; 2.78 vs 2.78 GiB), despite a favorable local ARM screen (0.9764, CI 0.9540–0.9993). Local writer, decommit, and lookup screens are detailed in the [2026-11-16 ledger](experiments/research-update-2026-11-16.md); they are not official results. The corrected lookup variant's admitted lead is `rgr1_b08ed0a74faf5a9cf17568064f2374d54b640a876d0b9e2e44bd3292b4b6bfdd`, but its local screen remains NO-GO.

One Nov 16 quotient diagnostic built and completed a fresh shared-input comparison: exact proof bytes, pinned verifier acceptance, working authorized timer, and retained coarse bucket timings. Stored/small-group join and lift/finish were the largest quotient intervals on this ARM run. They are not isolated operation costs or a promised speedup. Single-run peak footprints were 3,053,768,784 bytes for frontier and 3,041,087,664 for candidate; these do not establish median-memory acceptance. Follow-up read-only call-graph review found no supported redundant computation, so no optimization or further build has been selected from this diagnostic.

Evidence limits remain: detailed local writer logs are not uploaded; the later dependency-group result is conversation-reported; peak memory is unavailable for the earlier corrected lookup screen; no new aggregate run count is inferred. Authorized macOS timer probing now works after the sandbox `kern.clockrate` restriction was identified. Standings points in the ledger are from STARKS math cross-site work, not this STWO session.

## Research Question

Historical section, as of 2026-11-13; current evidence is summarized above and in the [2026-11-16 ledger](experiments/research-update-2026-11-16.md).

Which byte-identical, verifier-valid optimization can improve the current Stwo whole-proof frontier by at least 1% without increasing peak memory by more than 1%?

## Current Understanding

The current frontier is already highly optimized and cumulative gains have reduced pinned-base time by about 13.8%. Broad parallelism is no longer automatically useful: deeper Merkle fusion and finer Rayon tasks increased overhead and lost whole-proof time. The strongest scoring strategy is therefore a small, independently measurable mechanism that removes work from an exposed phase and can stack with another near-threshold patch.

The newest public near-miss is about 0.64% faster than frontier `69645ee0`, with slightly lower memory. It combines a full-coverage circuit cache check, lane-parallel BLAKE2s, vectorized decommit folds, and a zero-column FFT skip. It is useful as a future combination parent, but it cannot earn a speed result by itself because the evaluator requires at least a 1% gain beyond noise.

## Key Results

- `FUSED_LEVELS=6`: verifier-valid, byte-identical, but 4.48% slower on the evaluator. The extra fusion reduced IPC and increased branch misses.
- Grouped-OODS chunk 4096→2048: verifier-valid locally but grossly slower on a loaded ARM host. Task overhead dominated.
- Worker-local LogUp scratch reuse: exact and verified, but neutral over ten pairs at 1.0005 (95% CI 0.9398–1.0651) on an overloaded ARM host. The early two-pair 0.9219 signal was noise and was not submitted.
- Parallel shared evaluation-basis construction: differential test passed and proofs stayed exact, but both gross-screen pairs regressed (1.0030 and 1.0123; ratio 1.0076). Stop; Rayon setup and per-chunk starts cost more than the exposed serial chain on this host.
- Sonic parsing of the prepared `ProverInput` byte slice: all ten proofs were byte-identical and independently verified, but the median candidate/frontier ratio was 1.0140 (95% interval 0.9442–1.0890); local measurements were noisy and memory was unavailable on Darwin arm64. This isolated parser change did not establish a gain and was not submitted.
- Frontier `69645ee0`: current public baseline, cumulative ratio 0.8622706 versus the pinned commit.
- Rukafe near-miss on that frontier: ratio 0.9936471, 95% CI 0.9900004–0.9973072; valid but below the 1% scoring threshold.

## Patterns and Insights

- Strong operators rebase before every experiment, cite exact lineage, submit only focused mechanisms, and publish negative evidence quickly.
- Changes that merely increase available parallelism often lose because the remaining tasks are too small or memory movement dominates.
- Work elimination and lifetime reduction are more reliable than adding task waves.
- Near-threshold gains should be combined only when their mechanisms and memory peaks are demonstrably independent.
- Two-pair screens can produce spectacular false positives under load; the ten-pair gate prevented a low-value evaluator submission.

## Lessons and Constraints

- Do not retry lifted-Merkle fusion level 6 against the same frontier.
- Do not retry finer grouped-OODS task grains without new evidence about evaluator occupancy.
- Do not submit source records as though they were speed results; points that affect the performance leaderboard require evaluator-validated patches.
- Do not submit worker-local LogUp scratch reuse by itself on the current frontier without new Linux x86 evidence; the available whole-proof screen is neutral.
- Do not retry parallel basis construction on this frontier without a new mechanism that removes its per-chunk setup or demonstrates x86 occupancy; the current ARM screen is a refutation of this patch shape.
- Do not submit the isolated Sonic parser swap from H3.1 on this evidence; it was not faster on the ten-pair local screen, and the measured supported parser result was a broader stack.
- Every candidate must preserve exact proof bytes and pass independent verification.
- Peak memory may not exceed the current frontier by more than 1%.
- The patch budget is tight, so prefer compact edits and delete dead experimental code.

## Open Questions

- Which phase on frontier `69645ee0` still has an exposed serial or low-occupancy interval large enough to move wall time?
- Can interpolation/evaluation share a transform or denominator computation across existing groups without changing order?
- Can an independent transform gain stack with the 0.64% near-miss while preserving the memory envelope?

## Historical Optimization Trajectory (as of 2026-11-13)

Pinned base → cumulative frontier 0.86227. At that date, the recorded experiments had not improved that frontier; the next milestone was a qualifying current-frontier local result before evaluator time. Subsequent official and local results, including their limits, are in the [2026-11-16 ledger](experiments/research-update-2026-11-16.md).

## Target metadata reuse — 2026-11-17

The complete ten-pair screen preserved exact proofs and independent verification, but did not establish a speed gain: ratio 1.0087, 95% interval 0.9726–1.0462, reported median peak +0.6%. This busy ARM64 result is locally inconclusive, not an official refutation. The coordinator and independent reviewer withheld submission clearance. [Full evidence](experiments/target-reuse-result-2026-11-17.md).

Source inspection confirms repeated per-column inverse-index searches, but does not establish that they occupy enough whole-proof time to justify another build. The next research bottleneck is quiet evaluator-like Linux AVX2 profiling. The configured-machine inventory was empty; no resources were provisioned. Graph admission, source posts, and citations do not directly mint Stwo performance points in the inspected scoring engine.

## 2026-11-18: arithmetic screens and corrected lineage

The [new report](experiments/logup-arithmetic-2026-11-18/README.md) publishes a small kernel-level denominator reuse saving and all unresolved checks. Three adapted production-module tests pass; a constructed zero-denominator trace case panics before comparison with the original path. Full Cairo lib-tests fail to compile at an unchanged frontier initializer. Static preflight PASS does not clear these correctness gaps. No new whole-proof result or evaluator job.

Full graph payloads identify the exact two supported fixed-table zero-multiplicity stacks and link the newer one to digest a273aff7…. These are tested-stack results, not evidence for one-sided skipping, denominator delta or our separate deferred-dot witness prototype. That external prototype passes 7,168 raw-coordinate cases but is untimed and unintegrated. Preserve both artifacts, resolve test semantics, and establish material work before building a full speed candidate. Memory-only research remains secondary to the user's speed priority.

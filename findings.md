# Research Findings

## Research Question

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

## Optimization Trajectory

Pinned base → cumulative frontier 0.86227. Our scored experiments have not improved that frontier yet: one evaluator run was 1.0448 and one local task-grain screen was discarded. The next milestone is a current-frontier local result below 0.99 with exact proofs and stable memory before consuming evaluator time.

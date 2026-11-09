# Seeded experiment backlog

These are research leads distilled from the current public mainnet graph and leaderboard. They are hypotheses for our harness, not claims that every mechanism is already a winning patch.

## A. Retained stripe-0 decommitment

**Source:** Anurag's supported experiment `rgr1_e11b7eaec7c2ba68a1b0683f6ade53ff952f49e258e8bf1260ed49921c203285`.

**Mechanism:** Keep stripe 0 of circuit pair-only columns already committed and route samples/quotients through the resident path instead of regrowing coefficients with per-column IFFTs.

**Why adopt:** Evaluator-supported result around `0.9663` of the frontier with lower peak memory.

**Test:** Compare whole commitment proofs against the regrown reference; check resident and sourced columns, decommitment, byte identity, and peak memory.

## B. Cached-column run planning

**Source:** Anurag's supported experiment `rgr1_c2f4a8cfedbb6e8c59ee846a8e348c1dbf5d3b9c7b65af7042c639b20b6524f9`.

**Mechanism:** Group aligned cached-column rows so shared inverse fold layers run once per chunk, while sourced columns read cached stripes directly.

**Why adopt:** Best visible mainnet result, measured around `0.9561` with approximately frontier-level memory.

**Test:** Compare run-plan rows with the normal FFT extension, omitted Merkle layers, weighted dots, and malformed cache fallbacks.

## C. Input parse overlap

**Mechanism:** Parse large JSON arrays on rayon workers while twiddles and precomputes are built; fall back to the ordinary parser for unusual input.

**Why adopt:** Cheap setup win, but only if parsing is on the critical path and the fallback is exact.

**Test:** Differential-parse normal, escaped, duplicate-key, malformed, and split-array inputs.

## D. LogUp tail parallelism

**Source:** Vibhanshu's experiment `rgr1_efdba382cf540886429824b59f449dc27c636f673044c8e...`.

**Mechanism:** Parallel prefix-sum sweeps and chunked finalization.

**Why adopt cautiously:** Large phase-level savings did not shorten the whole proof on its own.

**Test:** First establish that the tail is critical-path work; then measure with and without overlap.

## E. Writer and verifier-fill overlap

**Mechanism:** Move interpolation or FRI-input work into dependency-bound writer/fill windows only when those windows have idle workers.

**Why adopt cautiously:** Writer interpolation helped only after the writer pool had spare capacity; stacking it after other scheduling changes could put it back on the critical path.

**Test:** Use phase markers and CPU-busy samples before and after the change.

## F. SIMD batch inversion and lazy reduction

**Mechanism:** Batch inversions and reduce SIMD accumulators less frequently while preserving canonical reduction points.

**Why adopt cautiously:** The isolated kernel improvements were near the evaluator noise floor.

**Test:** Extreme field values, portable `std::simd`, AVX2 path, and exact field outputs.


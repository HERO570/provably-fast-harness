# Research Log

Chronological, append-only record of research decisions and results.

| # | Date | Type | Summary |
|---|------|------|---------|
| 1 | 2026-11-09 | bootstrap | Created a separate whole-proof research harness and mapped strong-operator practices: current-frontier rebasing, graph lineage checks, phase evidence, one-mechanism patches, and immediate publication of negative results. |
| 2 | 2026-11-10 | inner-loop | Evaluator job `2697f895-6f89-4405-bed7-5de329dda1e6`: lifted-Merkle `FUSED_LEVELS=6` was valid but 4.48% slower (95% CI 3.61–5.37% slower), peak 2.78 versus 2.77 GiB. Reverted and published as a refutation. |
| 3 | 2026-11-11 | inner-loop | Grouped-OODS task-grain 4096→2048 preserved proof bytes and verification but was grossly slower in a loaded local screen. Reverted; no evaluator time spent. |
| 4 | 2026-11-11 | bootstrap | Five primary implementation sources were admitted to challenge 2, covering contiguous Merkle staging, retained SIMD groups, efficient coset interpolation, Rayon transpose, and small-batch DFT scheduling. |
| 5 | 2026-11-11 | outer-loop | Frontier advanced to `69645ee0` (cumulative ratio 0.86227). A separate worktree was created at the pinned base with the frontier applied. Rukafe's newest valid patch measured 0.99365, so copying it cannot score; next work must add an independent gain. |
| 6 | 2026-11-11 | pivot | Adopted a two-loop protocol: reproduce the current frontier first, then test one bounded transform/evaluation or serial-window mechanism at a time. Promotion gate remains <0.99 ratio, exact proof bytes, independent verification, and ≤1% peak-memory increase. |

# Experiment plan: per-decommit row-target metadata reuse

## Graph proposal state

Updated 2026-11-17. Proposal `20583edf-7fdc-435a-93ff-c31eb1ae458e` was admitted as Challenge 2 lead `rgr1_dadb7c21852f1ecfc477e9077beb7c15681f0a6021c35648604900de2ad58346` (sequence 1045), titled “Reuse per-decommit row-target metadata,” credited to `hero570`.

It has a `DERIVED_FROM` edge to `rgr1_ea084ee5484191ae60a9dbde8c6074879eaef5f3fb8ebe2e7966191b025ac420`, with the note: “Hoists row-target metadata separately from arithmetic caching and scheduling.” Campaign digest: `b8f27680440392bcc81f93088c3ed3a801ff89f5d65047d2241c7f495642f17b`.

The original snapshot request failed. A later successful proposal receipt and live graph read supersede that failure; no duplicate proposal was made. The live cited-by view contains derivative lead `rgr1_4cc20fc794e175c3124c58479e1e4cc0a3c3ad67436b4cd00bdf685d9a93ff4d` (sequence 1052), concerning shared targets and inverse scatter maps. This is a citation, not an experiment or a supporting verdict. No performance result or points are confirmed for this lead.

## Hypothesis

On pure frontier `69645ee0e6f583990c4aaf064cd08fde601ff009`, reusing per-call decommit row-target metadata reduces repeated target-list construction without changing arithmetic. Source review supplied for this task reports that `rows_of` returns identical lists by `log_size` within a call, with repeated construction before SIMD `PrefixPlan` lookup.

Candidate location: isolated `candidate-decommit-targets-69645ee0`. Expected whole-proof speed and memory effects are unknown; no measured gain is claimed.

## Mechanism and bounds

Borrow cached all-row and suffix lists. Collect used sizes in a fixed stack bitmap rather than a heap vector that scales with polynomial count. Limit metadata reuse to a 64 KiB per-call budget using a checked conservative estimate and actual capacity accounting, including nested vectors and cache containers. Empty, unsupported, overflow, or over-budget cases retain the original construction path. Preserve query order, duplicate handling, sorted/deduplicated target rows, and resident-stripe suffix semantics. Arithmetic, proof parameters, backend interfaces, and scheduling are unchanged.

The increment is confined to `crates/stwo/src/prover/pcs/mod.rs` and a new standalone `target_rows_cache_tests.rs`. The source baseline was verified against all 1,779 entries in official frontier commit `69645ee0e6f583990c4aaf064cd08fde601ff009`; the pinned Git HEAD and empty index were retained. Existing failed candidates and baseline binaries remain separate.

## Review and test gates

Independent design and actual-diff review cleared controlled compilation and focused unit tests. The reviewer caught an omitted live key-vector allocation before compilation; the fixed stack bitmap resolves that issue. Static whitespace checks passed. On Nov 17 the pinned focused command passed **3 tests, 0 failures** both before and after the test-module policy correction:

```sh
cargo +nightly-2026-01-15 test --locked -p stwo --features prover,parallel --lib target_rows_cache_tests
```

Coverage includes cached/fallback target equivalence, duplicate and out-of-order rows, multiple sizes, resident suffixes, and estimate/capacity budget fallback. These tests do not establish whole-proof identity or speed. Corrected source hashes remained unchanged after the second test run:

| File | SHA-256 |
|---|---|
| `pcs/mod.rs` | `3498174c5c12be54837060520faaa3c1b3cbdeada708521d0a4cbc4fd2397c2b` |
| `pcs/target_rows_cache_tests.rs` | `30874fe7e525fa8d3d203f97b97e05900fd97661709a7464ffd4e12e0e0d6909` |

The remaining stage is the unchanged client's ten fresh alternating pairs against the preserved frontier binaries. Require byte-identical proofs, independent verification, valid peak-memory measurements, and a clear whole-proof time gain. ARM local results are advisory for the Linux x86-64 AVX2 evaluator. Final policy preflight and parent review precede evaluator submission using the existing proposal ID.

## Current execution state

The first native release build succeeded in **15m50s**. Its binary was preserved, SHA-256 `02098105e18d8f0e306e119d4d5fb3f6117949b086c2ab8fe9e1a03d5861b165`, and was not benchmarked because of the policy issue below. Flags were `-C target-cpu=native -C opt-level=3 -C codegen-units=1` on Darwin ARM64.

The static source checker initially flagged the added `#[cfg(test)]` module declaration as `PROTECTED_CFG_TEST_REGION`. After the build ended, the approved correction removed that new attribute and moved imports inside the three standalone test functions. The existing protected tests and scanner stayed unchanged. Independent actual-diff review and the unchanged static source/promotion checkers now report **zero violations**. The corrected focused run passed **3/3**; its retained local log SHA-256 is `d8ddb4817c20e7958c33a6099760aba5da617e4f302bf51101b746901ad4b51a`.

The corrected read-only canonical serializer reports **1,536,778 bytes across 42 candidate paths**, excluding the known temporary helper, leaving **36,086 bytes** below the 1,572,864-byte limit. Canonical patch digest: `e5a3aae5f43763b77d035c1575c22bef8042a918d881fb97d95c74800507b43c`. Editable paths, base blobs, protected tests, manifests, compile inputs, tokens, and promotion checks pass. This excludes the helper and remains distinct from final full preflight. Full source-policy checks now belong before the first build in the [playbook](RESEARCH_PLAYBOOK.md).

The corrected same-flags release build succeeded in **15m52s**. The binary SHA-256 is `dbf583dd7ecb580b152c881eff314424d9c69bd4674da2a9963a313b12a1b83f`; the retained build-log SHA-256 is `fe3ba6af683576b8c56b6f6af7c869b9e737b2efdb4c9e00a7403228d349680b`. Post-build source hashes still match the corrected hashes above. Measurement will use this binary, not the preserved pre-correction artifact.

Normal outside-sandbox execution approval cleared and the first measurement started. Its intended live file was not written because tee was attached to the final exit command rather than to the benchmark invocation. Later the worker panes returned to shells and no benchmark process remained. Recovery retained 24 original execution-output chunks, SHA-256 `7230364aa56b2e9044be3573375d30e64d00aa73c82f8f1b9c9fca012b9c091a`. The retained evidence contains the warm-up and measured pairs 1–4, followed by empty poll results; it contains no pairs 5–10, aggregate interval, median-memory summary, or exit status. A lost execution-process ID does not establish completion.

| Measured pair | Frontier seconds | Candidate seconds | Candidate/frontier | Frontier peak GiB | Candidate peak GiB |
|---|---:|---:|---:|---:|---:|
| 1 | 15.63 | 15.75 | 1.0073 | 2.89 | 2.88 |
| 2 | 21.94 | 18.95 | 0.8636 | 2.84 | 2.87 |
| 3 | 40.38 | 63.05 | 1.5613 | 2.71 | 2.74 |
| 4 | 107.37 | 70.25 | 0.6543 | 2.69 | 2.85 |

All four captured measured pairs were byte-identical and independently verified. The warm-up was also exact and verified and is excluded from measured conclusions. This incomplete, mixed evidence establishes neither a speedup nor a refutation. There is no complete ten-pair verdict.

One replacement ten-pair run is authorized with the same reviewed source, attributable binaries, flags, client, and tools. It will use fresh inputs and direct unbuffered client-output logging with pipefail. Its results will remain separate from the partial run; prior pairs will not be selected or combined into it. At the latest observation, its owner is awaiting normal execution permission for a bounded host/hash/timer precheck. No rebuild or candidate change is planned. The macOS system timer needs access denied inside the sandbox.

No complete performance result or evaluator submission is recorded yet. The temporary build helper is outside the permitted submission surface and must be excluded from the final patch. Keep inputs, preimages, proofs, binaries, credentials, and private raw logs out of this public repository.

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

Independent design and actual-diff review cleared controlled compilation and focused unit tests. The reviewer caught an omitted live key-vector allocation before compilation; the fixed stack bitmap resolves that issue. Static whitespace checks passed. On Nov 17 the pinned focused command passed **3 tests, 0 failures**:

```sh
cargo +nightly-2026-01-15 test --locked -p stwo --features prover,parallel --lib target_rows_cache_tests
```

Coverage includes cached/fallback target equivalence, duplicate and out-of-order rows, multiple sizes, resident suffixes, and estimate/capacity budget fallback. These tests do not establish whole-proof identity or speed. Source hashes remained unchanged after the tests:

| File | SHA-256 |
|---|---|
| `pcs/mod.rs` | `44d4eb6dfc85cd7b44856859d5a4e0fb7207c5573c7149ea9f367788e240305c` |
| `pcs/target_rows_cache_tests.rs` | `6c32742c752cb58f7e0a00be6e2a43eaa41704a9166e4f5e7dc2c5c6316b88ef` |

The authorized next stages are one attributable native release prover build, then the unchanged client's ten fresh alternating pairs against the preserved frontier binaries. Require byte-identical proofs, independent verification, valid peak-memory measurements, and a clear whole-proof time gain. ARM local results are advisory for the Linux x86-64 AVX2 evaluator. Final policy preflight and parent review precede evaluator submission using the existing proposal ID.

## Current execution state

Focused tests passed. The release-build/measurement pipeline is in progress; no full-proof result or evaluator submission is recorded here yet. The temporary build helper is outside the permitted submission surface and must be excluded from the final patch. Keep inputs, preimages, proofs, binaries, credentials, and private raw logs out of this public repository.

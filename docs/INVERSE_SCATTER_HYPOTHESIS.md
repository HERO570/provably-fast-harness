# Inverse scatter indices for striped decommitment

## Context

Lead `rgr1_dadb7c21852f1ecfc477e9077beb7c15681f0a6021c35648604900de2ad58346` describes reuse of per-decommit row-target metadata. Its cited derivative, `rgr1_4cc20fc794e175c3124c58479e1e4cc0a3c3ad67436b4cd00bdf685d9a93ff4d`, is titled “Share striped-opening targets and inverse scatter maps across columns on 69645ee0.” The derivative payload and verdict are unavailable in our retained evidence; this note does not claim to reproduce or validate its implementation.

The existing candidate experiment measured **row-target-list reuse**: within one striped decommit operation, target row lists can be built once per target size and borrowed by columns. A further **inverse-index extension is untested**. Frontier source at `69645ee0` repeatedly maps each requested position to a stripe target and then binary-searches that stripe’s sorted row list in several per-column fast paths. An inverse index could precompute those lookup results once per target size and reuse them across columns. It would not remove polynomial evaluation, regrowth, field arithmetic, or row reads.

## Minimal mapping design

Within one decommit invocation, positions and stripe width are fixed. Build one immutable mapping per target log size, aligned one-for-one with `positions`; each entry stores the full sorted-stripe index and row index within that stripe’s sorted, deduplicated row list. Alignment preserves duplicate positions: duplicate entries remain duplicate output positions even when they reference the same row slot. `log_size` is required because the row transform changes with the target size. The plan must not escape the invocation or be reused with different positions or stripe width.

The prefix-evaluation path evaluates only stripes at or above its `resident` prefix count. Keep indices in the full stripe list, compute `suffix_start = partition_point(stripes < resident)` once for that column, and use `full_stripe_index - suffix_start` for nonresident targets. Positions below `resident` continue to read resident values directly. If an implementation instead stores suffix-relative indices, `resident` (or the equivalent suffix start) must be part of that map’s key; reusing such a map across different resident prefixes is incorrect.

## Cost and memory limits

Let `C_l` be the number of columns using target log size `l`, and `P` the number of positions. Current fast paths repeat up to `C_l * P` target-list scans (at most the stripe count per scan) and `C_l * P` row binary searches. Prefix columns do the lookups only for nonresident positions. The proposed map replaces those repeated searches with one-time construction plus indexed access. This is a work-count proxy only: no retained measurement establishes that these searches are a meaningful hotspot, and no speedup percentage is claimed.

For `S` target sizes, a two-`usize` entry costs `2 * size_of::<usize>() * P * S` bytes of map capacity, plus vector headers and allocation capacity. The existing row-target cache also consumes metadata. Any implementation must use checked size arithmetic and fallible reservation, and enforce the per-invocation 64 KiB budget across the **combined live target lists and inverse maps**, including capacity overhead and temporary construction allocations. Separately account for the sum of allocations across all simultaneously live decommit invocations in the whole-proof peak; a per-invocation cap is not a global cap, and it cannot assume there is only one cache per worker. On overflow, allocation failure, or budget excess, discard/avoid optional cached plans and use the existing builder/search fallback. This is a proposed design constraint, not an implemented or measured memory result.

## Evidence status

The source contains repeated lookups, making the extension a concrete hypothesis. The inverse-index extension itself is unimplemented and untested; exact derivative payload, verdict, novelty, workload counts, and evaluator-host bottleneck evidence are not established. The local target-list-reuse measurement was noisy and inconclusive, so it supplies no gain claim for either mechanism. Publication does not authorize implementation or measurement. See the [conditional experiment plan](NEXT_EXPERIMENT.md) and [completed local evidence](../experiments/target-reuse-result-2026-11-17.md).

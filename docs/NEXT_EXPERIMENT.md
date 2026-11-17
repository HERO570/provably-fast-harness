# Next experiment: quiet Linux profiling and one bounded decision

The [complete target-reuse screen](../experiments/target-reuse-result-2026-11-17.md) passed ten exact, independently verified pairs but established no speed gain: ratio 1.0087, 95% interval 0.9726–1.0462, median peak +0.6%. It ran on busy Darwin ARM64 hardware. The existing Sol High coordinator and Luna workers reviewed the result, source-level follow-up, and environment inventory. Their next step is a suitable measurement environment, followed by a measured choice rather than another speculative build.

The [measured source increment](../experiments/patches/README.md) is published with a verified base and exact postimage hashes, so this path can be reproduced without reconstructing the patch from a description.

This plan and the design notes were also published in the [existing Provably thread](https://stwo.provably.fast/threads/bt1_777ece046670d1dc46e006ec), reply `bp1_030c15e2d971de0f6aa68e76`, with the measured patch attached. This is a research follow-up, not an evaluator job or a new speed result.

## 1. Establish a quiet existing Linux host

Use Linux x86-64 with AVX2 and enough cores to exercise the evaluator's 16-worker context. Record CPU model, physical/logical core layout, available memory, toolchain, load, and peak-memory reporter. Arrange a measurement window without competing CPU-heavy work. The configured Herdr inventory is empty; it does not prove that no unlisted infrastructure exists. A host alias/access scope is the missing handoff. No rental or provisioning is part of this plan.

## 2. Re-establish source and binary identity on that host

Refresh official `mainnet-season` and record its exact commit/tree; do not assume `69645ee0` remains current. Check whether the target-reuse increment is already included or whether a changed frontier invalidates the old comparison. Keep the challenge's pinned HEAD, unstaged frontier diff, allowed paths, and untracked additions intact.

Build fresh **Linux** frontier tools and candidate binaries with matching pinned toolchain and flags. The existing Mach-O ARM64 executables cannot serve as Linux comparator artifacts. Preserve a verifier built from the required pinned source, record source/executable hashes, and check the system timer before measuring. Run complete static source-policy checks before the first build; keep temporary harness helpers out of any eventual upload.

## 3. Measure the preserved candidate and attribute the frontier's work

A quiet, different target architecture is a reason for one controlled re-evaluation of the preserved candidate; it is not an unchanged rerun on the same loaded ARM host. Use fresh statements, alternating order, exact byte comparisons, pinned independent verification, a complete live log and exit status, phase timings, median peaks and the time-ratio interval. Preserve all prior data separately.

Profile the frontier in a separate untimed run so profiler overhead does not enter the ratio. Inspect actual striped-decommit target construction and index-search work. Collect positions `P`, used target sizes `U`, selected stripe count `T`, eligible columns by size `C_l`, selected rows per stripe, and simultaneously live invocation counts. Use existing symbols/tracing where possible; any diagnostic build must have separate source/binary attribution. Coarse tail medians do not isolate these costs, and per-column CPU totals do not add directly to whole-proof wall time.

## 4. Choose one falsifiable outcome

- If the preserved candidate shows a qualifying whole-proof gain within the memory envelope, proceed through final policy checks and evaluator submission. A new inverse-map patch is unnecessary for that decision.
- If it does not, but attributable search/construction work could materially affect the complete proof, review the [inverse-scatter hypothesis](INVERSE_SCATTER_HYPOTHESIS.md) as one bounded next patch. Record its lineage accurately, including the related derivative whose exact payload is unavailable; do not claim transferred support or reproduction.
- If even an optimistic bound on removable critical-path work cannot meet the challenge's threshold, stop this direction. A source-level repeated operation alone is insufficient justification for another build.

The optional target-list and inverse-map metadata must share a checked per-invocation 64 KiB budget, including vector capacities and temporary construction storage. Separately account for the sum across all live invocations in the whole-proof peak; it is not bounded by assuming one cache per worker. Overflow, unsupported shape or allocation/budget failure must use the original construction/search path.

## 5. Validate and publish the selected change

For an inverse-map implementation, first compare cached/fallback row outputs across all affected paths. Cover unsorted/duplicate positions, omitted leaves, multiple sizes, different resident prefixes, empty/full suffixes, budget/overflow fallback, and serial/parallel execution. Preserve arithmetic, proof parameters, column order and lookup dependencies.

Then repeat the full fresh-input paired proof gate on the suitable host. Require exact bytes, independent verification, a clear whole-proof gain, and peak memory within the challenge envelope. Before an upload, remove only the hash-verified temporary helper and run full canonical client preflight. Submit against the correct admitted lead for the actual tested patch; a new extension needs its own accurately cited research record. Read the evaluator result back and preserve its exact scope.

Publish useful negative or unresolved results too, with hardware/load conditions and missing evidence. Workshop updates are research contributions, not speed results or automatic points. Do not spend evaluator jobs on the current local screen or describe it as an official refutation.

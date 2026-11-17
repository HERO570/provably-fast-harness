# Stwo operator and scoring audit — 2026-11-17

This audit focuses on [Stwo](https://stwo.provably.fast/). It separates official evaluator rows, graph experiment verdicts, contributor reports, public repository documentation and local inference. Existing coordinator and workers shared one retained snapshot; no new prover build, proof run or evaluator submission was made for this audit.

## What can advance each board

The [People table](https://stwo.provably.fast/#/people?view=table) sorts contribution activity by default. Its Total adds leads, experiments, sources, workshop discussion and visible evaluated results. Cited-by is displayed separately. These are bounded UI windows, not weighted merit or performance points. The source uses at most ten 100-record feed pages; when the initial workshop event walk exhausts forty pages, it switches to the newest 4,000 sequence range. Older activity can leave these windows. Grouped counts from the People API are not the table's exact counts. Source: [Stwo UI](https://stwo.provably.fast/v2.js), [profile/data contracts](https://stwo.provably.fast/presentation.js).

The performance leaderboard and [global points engine](https://provably.fast/points.js) are separate. The mainnet time board counts successive strict improvements among eligible published `MEASURED_IMPROVEMENT` results. Posting a source, receiving a citation or running another job does not itself create those record points. Useful, reviewed research should still be published so others can reproduce it.

## Actual Stwo contribution top ten in the reconstructed snapshot

This follows the retained UI's default Total sort. It is not a browser-observed live rank, a full-history ranking or the cross-site standings top ten.

| Rank | Handle | Leads | Experiments | Sources | Discussion | Results | Total |
|---:|---|---:|---:|---:|---:|---:|---:|
| 1 | nadai2010 | 79 | 27 | 11 | 0 | 12 | 129 |
| 2 | realadii | 62 | 17 | 33 | 0 | 11 | 123 |
| 3 | tanctl | 87 | 5 | 1 | 0 | 5 | 98 |
| 4 | rukafe0x | 55 | 23 | 0 | 0 | 11 | 89 |
| 5 | fkiene | 60 | 8 | 5 | 0 | 1 | 74 |
| 6 | ryun1 | 32 | 21 | 0 | 0 | 16 | 69 |
| 7 | odinfree | 44 | 12 | 2 | 0 | 7 | 65 |
| 8 | saniksin | 39 | 16 | 0 | 0 | 7 | 62 |
| 9 | hero570 | 31 | 6 | 12 | 6 | 5 | 60 |
| 10 | lokesh0652h | 51 | 1 | 0 | 0 | 0 | 52 |

There are 92 consented profiles in both identity inputs. The reconstruction uses exactly 1,000 feed items (sequences 66–1065), the latest event-sequence window `(9179, 13179]`, all 170 listed threads, and 111 normalized result rows including memory-only record additions. The event pages contain 3,999 unique events; only 14 are thread/reply events surviving the UI's listed-thread filter. Hence zero Discussion in a row does not mean no workshop participation. References were not retrieved, so cited-by and citation-only zero-total rows are unavailable; they do not change these positive-total top-ten positions. [Machine-readable counts and input hashes](results/stwo-contribution-table-2026-11-17.json) preserve the reconstruction's scope. All input snapshots are public-route captures, with no claim of an atomic server snapshot.

Our reconstructed activity rank is ninth, while our new local target-reuse screen still has no qualifying performance gain. Fifth place's Total 74 is an activity comparison, not a speed or points target. We should publish useful reproducible work rather than optimize post volume. Strong recent mainnet record-setters need not appear in this historical/windowed top ten.

Every member of that top ten was checked against the retained feed/result evidence:

| Handle | Paths in the retained work | Evidence limits / harness |
|---|---|---|
| nadai2010 | Retained stripe pairs, allocator/live-set accounting, circuit coefficient lifetimes, host-specific kernel analysis. | Latest retained feed sequence 798 supports its tested circuit-pair stack on an older frontier. Public race README practices are documented below; this does not attribute them to a particular result. |
| realadii | Fused IFFT, doubled FFT store runs, grouped writers and tiled Merkle passes. | Latest stack/interpolation experiments 649/643 refute those tested variants; a separate coefficient-fold experiment supports its own lead. Latest captured result is valid but not faster. Harness/model are self-reported `claude-code` / `claude-fable-5-1`. |
| tanctl | Composition-window sizing and direct coefficient-fold isolation. | Experiment 500 refutes its wider-window lead. Latest captured result is valid but not faster; harness/model unknown. |
| rukafe0x | Validity-check locality, hosted attribution, instruction census and scheduling/allocator nulls. | Current-frontier experiment 1054 supports its tested tiled-check stack. Earlier CPU/VM analyses are contributor reports, not current-mainnet bottleneck proof. |
| fkiene | Cached regrown composition coefficients and 90-bit composition-window sweeps. | Experiment 472 refutes its window variant; older coefficient-cache claims are not affirmative evidence for the window change. Latest captured result is valid but not faster. Self-reported model is Claude Haiku 5.5; harness unknown. |
| ryun1 | Memory-oriented tree/commit geometry and composition lookahead. | Older experiments 402/395 support their own stacks; those gains do not transfer to today's mainnet settings. Older memory result self-reports `claude-code+auto-research-toolkit`; public toolkit documentation is a separate source. |
| odinfree | Allocator/interaction-pool composition and a checked Cairo preprocessing-cache proposal. | Latest lead 799 has no verdict in this capture. Supporting experiment 770 tests a different allocator stack. Latest captured mainnet result is valid but not faster; `codex` / `mistral-large-4` are self-reported. |
| saniksin | Double-stripe retention, small-block reservation and idle-budget packages. | Experiment 767 supports its exact Package 45 lead on an older frontier. A separate mainnet result is valid but not faster. Harness `claude-code` is self-reported; model unknown. |
| hero570 | Writer batching, lookup composition and per-decommit target reuse. | Our target-reuse lead is admitted; its local ten-pair screen is unresolved and unsubmitted to the evaluator. Public patch, logs, plan and workshop follow-ups are linked below. |
| lokesh0652h | Twiddle batch inversion, transcript allocation and FFT hypotheses. | Experiment 904 refutes its four-way twiddle variant; nearby leads lack supporting verdicts. No retained evaluated result or harness attribution for this handle. |

The top ten is not a list of ten currently active mainnet record-setters: several latest retained contributions predate mainnet or use earlier settings. Binharold's current record work and Anurag's current-frontier result are assessed separately below. Unknown or self-reported harness fields do not establish private skills, model quality or a reproducible agent workflow.

The saved official results were observed at **2026-11-17 12:22:02 UTC**. Their source frontier is `69645ee0e6f583990c4aaf064cd08fde601ff009`, campaign ratio **0.8622706158865119**. The best qualifying published ratio is **0.8383761455505968**, by Binharold. Thus the strict frontier-relative record target in this capture is **below 0.9722888964**, more than **2.7711%** faster. A qualifying 1% frontier gain could still fail to set a new points record. This is snapshot arithmetic, not a promise about future scoring or proof performance. Source: [official results](https://provably.fast/api/participation/internal/results), [reproducible fixture](results/mainnet-record-target-snapshot-2026-11-17.json), [calculated threshold](results/mainnet-record-target-2026-11-17.json).

## Currently supported research paths

The retained Challenge 2 feed ends at sequence 1065. The following graph verdicts support the exact tested leads/stacks. A title and nearby result row do not establish which component produced a particular gain.

| Operator | Retained evidence | What it suggests investigating |
|---|---|---|
| Binharold | Experiments 1057 and 1059 support successive Blake/long-Cairo writer hand-off changes; 1061 and 1064 support zero-multiplicity fixed-table LogUp work skipping and its extension. | Identify concrete arithmetic or dependency work removed, reproduce the exact supported stack, then seek a distinct increment. Do not copy an already published stack and claim a new mechanism. |
| Rukafe0x | Experiment 1054 supports a tiled validity-check stack on `69645ee0`. | Examine actual aligned-column read locality and the full validity predicate; tile dimensions and isolated gain are unavailable in this retained feed. |
| Anurag629 | Captured current-frontier result measured 0.97262 against `69645ee0`, with 95% interval 0.96872–0.97654. | Use same-frontier comparisons. Its interval overlaps Binharold's 0.97229 result, so this snapshot does not establish a significant head-to-head difference. |
| Lyskey1 | The retained PoW-kernel experiment 1047 refutes its tested lead; earlier scheduling/rotation experiments also have scoped negative verdicts. | Read negative evidence before repeating a plausible microkernel or scheduling change. A microbenchmark win on another CPU does not establish whole-proof gain. |

Exact current graph IDs:

- Binharold 1064: `rgr1_8db519d619dd6557327c0e473b127f335f3e194ba91cc5c7cb3529c5dc1ac779`, tests/supports lead `rgr1_c391297c5b6a77745d3fa5e0861267f985a0dab784c20c40de075ef8bc34ab89`.
- Binharold 1061: `rgr1_5815cd8c28fbc1367100095d76fc1fd2c65092a8c1a8f55e1f81bce2147e4355`, tests/supports lead `rgr1_1056aa9e675c7755edc101bd8cd3a1b6d74eddba9e63aca612d34eb0cbfedf00`.
- Binharold 1059: `rgr1_e74405afa7ea2b30d4fd31dd1a89e594d24b21d364aa6ab6dfd257aebf60d130`, tests/supports lead `rgr1_48643c05094f8197a0bd39ed1563f8d847e8997a229c31df22e46aaeae977dd2`.
- Binharold 1057: `rgr1_f7b6f1cccebe9e2cbcfea58a95f49aef884463906675c15ee5c8cabba05d6b3c`, tests/supports lead `rgr1_4cda805d17ebfa9d71fdce4669b9dd7917d6bbf1a54f86166f54d85bb9687d82`.
- Rukafe0x 1054: `rgr1_80f5afdf2f45649e468f88a7a5a0740e1d49ac7d4a41f8e339180d6fd430cbe3`, tests/supports lead `rgr1_f78a9898c9d08a96fb40368dd610ab3b705a7198f1b16189fc57c47d98f6c1d3`.

Read these through `pvfast remote graph ... --challenge 2`. Public feed source: [Challenge 2 contribution feed](https://stwo.provably.fast/api/participation/feed?campaign_digest=b8f27680440392bcc81f93088c3ed3a801ff89f5d65047d2241c7f495642f17b&limit=100). `SUPPORTS` belongs to the tested lead against its frontier; it does not transfer to our patch or prove an isolated component helps.

Sequence 1065 is another contributor's local shared-target/scatter-slot screen, explicitly speed-unestablished, derived from `rgr1_4cc20fc794e175c3124c58479e1e4cc0a3c3ad67436b4cd00bdf685d9a93ff4d`. It has no evaluator verdict in this capture. Our optional inverse-index extension therefore cannot be presented as an established novel or supported direction merely because its local implementation differs.

## Practices supported by actual public artifacts

The [Nadai Falcon/Stwo race README](https://github.com/Nadai2010/falcon-stwo-race) documents equivalent compiler/workload settings, explicit setup-inclusive timing, verifier gating and frontier-aware live reproduction. Its side-by-side demonstration is not a substitute for the challenge's fresh-input alternating benchmark. We adopt explicit endpoint/build identities and reproducible run records.

The [Ryun research-toolkit README](https://github.com/Ryun1/auto-research-toolkit) documents validated measurement records, separate goal/metric/target/policy inputs, noise rules, independent confirmation, isolated workspaces and budgeted stopping. We adopt those checks in our workflow. We did not install or run the toolkit. Public documentation does not prove a particular Stwo result used those files or that adopting them will improve a score.

Nadai's [step-three thread](https://stwo.provably.fast/threads/bt1_30d412f8dcf8bfedb3a2b482) distinguishes local CPU attribution, live versus idle allocator memory, evaluator outcomes and failed variants. Its follow-up reports describe first-touch costs erasing local FFT savings. Rukafe's [instruction-census thread](https://stwo.provably.fast/threads/bt1_cb2eacbc6237d650972dee39) separates VM occupancy and kernel costs from evaluator behavior and closes ideas with bounded work estimates. These are contributor reports; their historical percentages are not current-frontier bottleneck measurements.

Result metadata often self-reports `claude-code`; some models are absent. Ryun's older memory result self-reports `claude-code+auto-research-toolkit`. We found no basis to infer a private skill, agent team or current model from a repository's existence. The Aeon README/instructions were unavailable in the bounded review, so no specific Aeon practice is adopted from its description alone.

## Correctness caveat: sampled Cairo cache guard

Binharold's [cache report](https://stwo.provably.fast/threads/bt1_5f5f3a3f6c2bb00dd6a910ab) explicitly says its sampled fingerprint can miss unsampled trace mutations and that no deliberate mutation test was performed. The current frontier source confirms a first-1,024-words-plus-every-256th-later-word pattern in [`crates/prover/src/cairo_preprocessed_cache.rs`](https://github.com/starknet-innovation/proving/blob/69645ee0e6f583990c4aaf064cd08fde601ff009/crates/prover/src/cairo_preprocessed_cache.rs#L139).

This is a cache-selection/fallback completeness concern, not evidence of an accepted invalid proof. The author's expected verifier rejection has not been reproduced in this audit. The circuit's separate public-preprocessed cache hashes full source columns and must not be conflated with this Cairo guard. We will not adopt a weaker check as a speed technique. A focused mutation/fallback audit should cover sampled and unsampled interior/tail words, shape changes and stale-key cases before any claim that all changed traces miss the cache.

## Changes made and next decision

1. Added an offline [record-target helper](../scripts/stwo_record_target.py), [eight regression tests](../scripts/test_stwo_record_target.py), an official mainnet fixture and saved calculation. It reproduces the status filter, last-value-wins digest handling and stable evaluation ordering. The helper is a snapshot threshold tool, not the official scoring engine or an all-time-history guarantee.
2. Corrected [scoring guidance](../docs/SCORING.md) to distinguish the People table, measured results and global points. Useful findings, including unresolved results, are published through the research routes independently of evaluator eligibility.
3. Prioritize exact supported LogUp/writer-stack evidence and a distinct removable-work mechanism over another speculative metadata/SIMD patch. Obtain the exact source payload and check inclusion before implementation; titles alone are insufficient. An existing quiet Linux x86-64 AVX2 host with matched fresh binaries is still needed for useful performance attribution. No host provisioning or spending is implied.
4. Keep the already [published target-reuse screen](target-reuse-result-2026-11-17.md) intact: 10/10 exact verified pairs, ratio 1.0087, interval 0.9726–1.0462, median peak +0.6%, busy ARM64. It establishes no qualifying gain; no unchanged rerun or evaluator submission follows from this audit.

Only harness documentation and offline tooling changed. No new proof-performance points, promotion, graph admission or evaluator verdict is claimed.

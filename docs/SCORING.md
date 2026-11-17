# How this work earns points

Checked 2026-11-17 against the public [points engine](https://provably.fast/points.js) and [hero570 standings API](https://provably.fast/api/checks/standings?who=hero570).

## Three separate rankings

The [Stwo People contribution table](https://stwo.provably.fast/#/people?view=table) is an activity ranking. Its default Total is the unweighted sum of leads, experiments, sources, workshop discussion and visible evaluated results. Cited-by counts are separate and do not enter Total. These counts use the UI's bounded feed/event/result windows, not an all-time contribution ledger. The [site implementation](https://stwo.provably.fast/v2.js) labels this activity rather than merit.

The Stwo Results leaderboard measures proof performance. The separate global standings allocate points for qualifying record work across sites. A useful admitted source or workshop report may contribute to the People table without minting performance points. A People-table rank, a global rank and a measured time ratio must never be substituted for one another.

## Stwo performance records

The speed-season branch of `stwoBoards` selects published Stwo result rows whose status is `MEASURED_IMPROVEMENT`, orders them by evaluation time, and keeps only successive improvements to the season's best score. Those record-setting results create the work entries used for Stwo points; attribution requires a recognized GitHub identity. Merely submitting a job or obtaining a valid proof is insufficient. The current mainnet season is included through `/api/participation/internal/results`: the `remote submit --internal` command does not exclude it from the public board or points engine.

The mainnet challenge's measured proof, timing, memory, guard-program, and promotion checks still apply. A lead being admitted, a source being published, or a graph citation appearing does not establish a qualifying performance result. The current Stwo points implementation does not directly mint points for those graph events.

## Amount and timing

The scoring engine allocates 1,000 points per site per UTC week across that site's open boards. A board's unused allocation waits until qualifying work occurs. Within a week's record work, points are weighted by the contribution's share of the measured score improvement. The amount depends on other work in that week and the board's available allocation; it is not a fixed reward per submission or per 1% gain. Standings use a rolling 13-week window.

Our operational route is therefore: isolate a new improvement against the current frontier, validate exact proofs and memory, obtain an official qualifying result, and check its appearance in the public score ledger. More sources, lines changed, or agent activity do not substitute for that result.

## Source frontier versus published record

In the Nov 17 captured mainnet results, the promoted source frontier is `69645ee0e6f583990c4aaf064cd08fde601ff009`, with campaign ratio **0.8622706158865119**. The best published qualifying result is Binharold's digest `a273aff79b9f7e52d7eee2669888825938bbf7b4e27f8103a97701c5d6799b53`, ratio **0.8383761455505968**. These ratios share the campaign baseline. Dividing them gives **0.9722888964**: a new record would need more than **2.7711%** improvement over that source frontier, as well as a qualifying evaluator outcome. A 1% improvement over the source frontier alone would still trail this published record. This comparison does not establish promotion of the published patch.

Reproduce the captured score threshold offline:

```bash
python3 scripts/stwo_record_target.py experiments/results/mainnet-record-target-snapshot-2026-11-17.json
python3 -m unittest discover -s scripts -p test_stwo_record_target.py -v
```

The helper follows the time-board status filter, last-value-wins digest deduplication and stable evaluation-time ordering. It reports a threshold from a bounded saved result window, not guaranteed complete history, a points prediction or validation of a candidate. Refresh the official results and source frontier before a new experiment; do not hard-code this dated threshold. See the [saved calculation](../experiments/results/mainnet-record-target-2026-11-17.json).

## Dated account snapshot

The Nov 17 API response showed hero570 at **place 19**, with **96 earned points** in the last 13 weeks and all time, **26 this week**, and **84 pending**. Its `sites` field listed only `starks`; this snapshot provides no Stwo points attribution. These points must not be attributed to this Stwo experiment. Pending points are not earned points.

No Stwo points or promotion are confirmed for the [per-decommit target-reuse experiment](EXPERIMENT_PER_DECOMMIT_TARGETS.md). Its complete local ten-pair screen passed exact proofs and verification but established no qualifying speed gain; no evaluator job was submitted.

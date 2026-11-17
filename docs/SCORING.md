# How this work earns points

Checked 2026-11-17 against the public [points engine](https://provably.fast/points.js) and [hero570 standings API](https://provably.fast/api/checks/standings?who=hero570).

## Stwo performance records

The speed-season branch of `stwoBoards` selects published Stwo result rows whose status is `MEASURED_IMPROVEMENT`, orders them by evaluation time, and keeps only successive improvements to the season's best score. Those record-setting results create the work entries used for Stwo points; attribution requires a recognized GitHub identity. Merely submitting a job or obtaining a valid proof is insufficient. The current mainnet season is included through `/api/participation/internal/results`: the `remote submit --internal` command does not exclude it from the public board or points engine.

The mainnet challenge's measured proof, timing, memory, guard-program, and promotion checks still apply. A lead being admitted, a source being published, or a graph citation appearing does not establish a qualifying performance result. The current Stwo points implementation does not directly mint points for those graph events.

## Amount and timing

The scoring engine allocates 1,000 points per site per UTC week across that site's open boards. A board's unused allocation waits until qualifying work occurs. Within a week's record work, points are weighted by the contribution's share of the measured score improvement. The amount depends on other work in that week and the board's available allocation; it is not a fixed reward per submission or per 1% gain. Standings use a rolling 13-week window.

Our operational route is therefore: isolate a new improvement against the current frontier, validate exact proofs and memory, obtain an official qualifying result, and check its appearance in the public score ledger. More sources, lines changed, or agent activity do not substitute for that result.

## Dated account snapshot

The Nov 17 API response showed hero570 at **place 19**, with **96 earned points** in the last 13 weeks and all time, **26 this week**, and **84 pending**. Its `sites` field listed only `starks`; this snapshot provides no Stwo points attribution. These points must not be attributed to this Stwo experiment. Pending points are not earned points.

No Stwo points or promotion are confirmed for the [per-decommit target-reuse experiment](EXPERIMENT_PER_DECOMMIT_TARGETS.md). Its graph lead is admitted and its focused tests pass; the whole-proof performance result is pending.

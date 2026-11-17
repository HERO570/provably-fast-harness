# Research update — 2026-11-16

Dated evidence ledger, recorded 2026-11-16. Frontier last checked Nov 16: branch `mainnet-season` at commit `69645ee0e6f583990c4aaf064cd08fde601ff009`, tree `07cb0abaa4d0a4b7d78a8e8050aa29b3c5a4a689`; pinned HEAD `9df00f2249b61849be69cc14942742d0c1f81ad2`. All time ratios below are candidate/frontier; intervals are 95%. Do not infer missing raw measurements or aggregate run totals. No proof inputs, preimages, proof bodies, executables, source archives, or credentials are published here.

## Official evaluator results

- Four-small-Cairo-trace-writer batching, job `6eaeae2c-a2ea-4688-84cf-0fcdab4c1d7f`: `INCONCLUSIVE`, `STOPPED_UNRESOLVED`; candidate/frontier ratio 0.9924, CI 0.9869–0.9980; peak 2.78 vs 2.77 GiB. No qualifying promotion.
- Combined four-writer batching plus one-pass Pedersen, official experiment `rgr1_fc910092286ad7d45655548f76154d7a75ae19cbf2db7c66476a4a99c1f3de84`, job `4d480c53-d78e-43da-9381-71d96d25768d`: `VALID_NOT_FASTER`, `STOPPED_NOT_FASTER`; ratio 0.9949, CI 0.9907–0.9992; 10 clean pairs; peak 2.78 vs 2.78 GiB; all proofs exact and verified. Associated local ARM screen: 10 pairs, ratio 0.9764, CI 0.9540–0.9993; peak 2.84 vs 2.84 GiB; exact and verified.
- No speed points or promotion are confirmed.

## Local screens, not official speed results

- Unsubmitted local screens (not children of the official experiment):
  - Six trace-writer variant: 10 exact, verified ARM pairs; ratio 1.1052, CI 0.9664–1.2639; peak −0.2%.
  - Shared leaf-sum ablation: 10 exact, verified ARM pairs; ratio 1.0189, CI 0.9648–1.0759; peak +0.4%.
  - Small interaction writer batch: 10 exact, verified ARM pairs; ratio 1.0197, CI 0.9741–1.0675; peak +0.7%.
  Each was a local NO-GO, reverted, and not submitted. Detailed raw logs are not in the public repository. Their admitted lead records are respectively `rgr1_8064253453c96d143adbf122a34e2a3536b2b7bfc6327f5891f8200da7fee17f`, `rgr1_50cd2f1820ea5711e095811953583429fe8f9658bf7c5f74a50b6e546deb7b9d`, and `rgr1_af936dea1781f527e3bf4b2a5ca7683450ac266aba09553322304ed430514397`.
- Dependency-group decommit scheduler: local 10-pair ratio 0.9988, CI 0.9916–1.0060; peak −0.4%; exact and verified. Removed and not submitted. Raw later summary is conversation-reported.
- Corrected eight-table small-lookup inline variant (`range_check6/8/4_3/4_4`, `xor4/7/8/9`, threshold 4096 packed blocks): built; 10 exact, verified ARM pairs; ratio 1.0625, CI 0.9244–1.2213; peak unavailable. NO-GO, unsubmitted. The first implementation's slice-shadowing remainder-bounds bug was caught by proof warmup and fixed before those 10 pairs.
- Current admitted lead: `rgr1_b08ed0a74faf5a9cf17568064f2374d54b640a876d0b9e2e44bd3292b4b6bfdd`.
- REGROWN_BATCH 16→32 was rejected before implementation: it removes no per-column work and adds scratch whose numerical bound cannot be established from available dimensions/group distribution.

## Diagnostic and constraints

- The diagnostic instruments `compute_fri_quotients_with_regrowth` in `crates/stwo/src/prover/pcs/quotient_ops.rs`. That file matched the frontier before instrumentation. One function received a 5,561-byte canonical patch increment containing four coarse timers and aggregate metadata/leg markers; an independent reviewer cleared the change. The release build succeeded in 12m13s on Nov 16, using `nightly-2026-01-15` and `RUSTFLAGS='-C target-cpu=native -C opt-level=3 -C codegen-units=1'`.
- **Completed:** one fresh statement was prepared once, then used for exactly one frontier proof and one instrumented candidate proof. Both exited 0; full-byte `cmp` exited 0, and the pinned independent verifier exited 0 with `PRIVACY_VERIFY ACCEPTED`. This was a single ARM64 correctness/attribution diagnostic, with no warm-up or alternating timing pairs. It does not establish a speed gain or evaluator acceptance.
- Runtime checks found 19 valid phase markers for frontier and 23 for candidate, no malformed markers, and raw stderr sizes of 1,331 and 1,941 bytes. Both are below the challenge's marker and stderr limits. Exactly two candidate diagnostic summaries were retained, one per leg.
- The unchanged timer parser selected `peak memory footprint`: frontier 3,053,768,784 bytes; candidate 3,041,087,664 bytes. Maximum RSS was separately 3,338,387,456 and 3,253,583,872 bytes. These are single-run ARM readings, **not paired medians or a passed memory gate**.
- Intended whole patch is 1,567,839 bytes, 5,025 bytes below the 1,572,864-byte limit, excluding a known temporary helper. Full preflight remains pending.
- Memory diagnosis: sandbox macOS denied `sysctl kern.clockrate`. The already-authorized `/usr/bin/time -l true` succeeds; no client changes are shipped.
- Official whole-proof brief currently uses `submit --internal`; `MEASURED_IMPROVEMENT` goes public. The old private-pilot exclusion does not automatically apply. Do not promise points.

### Retained diagnostic measurements

| Leg | Setup (ms) | Stored/small-group join (ms) | Large groups (ms) | Lift/finish (ms) | Regrown groups | Regrown columns | Largest column group |
|---|---:|---:|---:|---:|---:|---:|---:|
| Cairo | 0.580 | 46.722 | 0.010 | 66.201 | 11 | 3,154 | 782 |
| Circuit | 0.065 | 76.131 | 1.568 | 67.055 | 6 | 156 | 84 |

Maximum group-domain size was 8,388,608 in each leg. These are coarse wall intervals; the join includes parallel work, and the buckets do not isolate pure source-evaluation cost or recoverable whole-proof savings. The next step is read-only inspection of the join and lift/finish paths for a specific repeated operation. No optimization edit, additional build, or speed submission is justified by the bucket sizes alone.

That read-only follow-up is now complete: **no supported avoidable computation was found**. The join overlaps stored accumulation with coefficient/source combined-or-regrown paths, so its wall time is not their sum or a branch profile. Lift/finish includes sample-point grouping, linear-term lifting, quotient combination, denominator inversion, and four coordinate transforms. The observed counts do not identify the path mix or redundant identical work. Removing a sort would require proving ordered emission through the parallel collections; that property was not established. This closes the diagnostic with useful attribution and no justified speed patch, rather than authorizing another tuning build.

Locally retained artifact identities (contents are not uploaded):

| Artifact | SHA-256 |
|---|---|
| ARM64 diagnostic executable | `915f2a64c0320f19c206691856c4ca0d0dbb807dfbd70a764c7a0f3d33dd0198` |
| Frontier executable | `a90d8fc9b89e1d6c0152bd5b0b3eee1d1b5431dee4c19f8f919cc5380af736d8` |
| Unchanged client `measure.py` | `375847f44473011fc555d42f6e3676c819be7bd68422b78e19da8e1b57cc9f87` |
| Approved full diagnostic diff | `3addee8d959a9a2dd08de92a59a5794de9c50e03bb7ffc010e56d8d96dbe271b` |
| Build log | `ea1b9f3cc401110dbeba4550b3ff0925a44ba27d5396a41767a37f3648fdd6f9` |
| Single-input diagnostic transcript | `37ab1b55acb0630814a7cbfca47a223f94397555df6a11ca2b696610e9f1a9b4` |

Source and executable identities were checked before and after the run. The source snapshot contains 1,781 inventory entries; only the authorized quotient file differed from the pre-diagnostic snapshot. The client, tools, and temporary helper remained unchanged.

## Bounded public-feed observations

Observed STWO feed sequences 945–1044: strong method is exact frontier and lineage, a specific mechanism, independent evaluator testing, and preserving negative results. Binharold supports parent prefix `rgr1_61420ca2` (full ID unavailable); experiment `rgr1_edae165292acfe0de99125a84d5d3624eff3e9c5333a099047c06a076cb06b5a` supports only the full coefficient-fold/paired-stripe stack. New FFT lead at seq 1044 is pending. Verifier-fill variants seq 1007 and 1013 are refuted.

Current team approach: Sol 6.1 High orchestrates; Luna workers own scoped implementation, research, or audit; one build owner and source freeze. Validate timers first; do not repeat no-go builds; require exact proof bytes, independent verification, fresh alternating pairs, and memory before a real submission.

Standings snapshot Nov 15 20:43 UTC: hero570 place 19, WITH 69 points, 62 this week, 84 pending. These points came from STARKS math cross-site work, not this STWO session; do not describe this as “19 of 69 people.” Top-ten cross-site snapshot:

| Place | Handle | Points |
|---:|---|---:|
| 1 | nadai2010 | 2789 |
| 2 | rukafe0x | 2446 |
| 3 | anurag629 | 1506 |
| 4 | omarespejel | 1409 |
| 5 | odinfree | 1147 |
| 6 | ryun1 | 720 |
| 7 | binharold | 700 |
| 8 | adrienlacombe | 685 |
| 9 | krissemmy | 632 |
| 10 | ardianreffa | 480 |

Sources: [whole-proof agent brief](https://provably.fast/data/whole-proof-agent.md), [points engine](https://provably.fast/points.js), [standings](https://provably.fast/standings), [hero570 standings API](https://provably.fast/api/checks/standings?who=hero570), [proving repository](https://github.com/starknet-innovation/proving). These ranks and points are a dated cross-site snapshot, not a ranking of STWO speed results. Full top-ten harness/GitHub implementations were not established; the operator conclusions above are limited to the public feed inspected.

## Follow-up bookkeeping — 2026-11-17

- The per-decommit row-target metadata hypothesis was admitted as Challenge 2 `LEAD` `rgr1_dadb7c21852f1ecfc477e9077beb7c15681f0a6021c35648604900de2ad58346` (sequence 1045), proposal `20583edf-7fdc-435a-93ff-c31eb1ae458e`. It derives from `rgr1_ea084ee5484191ae60a9dbde8c6074879eaef5f3fb8ebe2e7966191b025ac420`; campaign digest `b8f27680440392bcc81f93088c3ed3a801ff89f5d65047d2241c7f495642f17b`. Admission and a derivative citation are not evaluator support or confirmed points.
- Corrected static review from t1b is CLEAR for 42 canonical paths: projected size 1,536,778 B with 36,086 B headroom; no policy, path, or promotion violations.
- First release artifact SHA-256 is `02098105e18d8f0e306e119d4d5fb3f6117949b086c2ab8fe9e1a03d5861b165`; that artifact is preserved and unmeasured. Corrected source SHA-256: `3498174c5c12be54837060520faaa3c1b3cbdeada708521d0a4cbc4fd2397c2b`. Corrected test SHA-256: `30874fe7e525fa8d3d203f97b97e05900fd97661709a7464ffd4e12e0e0d6909`.
- Corrected focused tests passed 3/3, with unchanged source hashes. One same-flags release rebuild is running, to be followed by ten fresh alternating pairs if attributable and successful. No full-proof performance result or promotion is claimed. See the [current experiment record](../docs/EXPERIMENT_PER_DECOMMIT_TARGETS.md).

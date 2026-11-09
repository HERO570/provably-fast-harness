# Provably Fast Research Harness

A reproducible, evidence-first harness for whole-proof Stwo optimization.

This repository turns the strongest public techniques from the mainnet challenge into a repeatable workflow. It is intentionally separate from the pinned `proving` tree: the challenge patch remains subject to its own editable-file and proof-identity rules, while this repo stores orchestration, measurements, hypotheses, and research context.

## Goals

- Optimize the complete proof, not an isolated kernel.
- Preserve byte-identical proofs and verifier acceptance.
- Keep peak memory at or below the active frontier.
- Make critical-path, CPU occupancy, and memory-lifetime evidence visible.
- Record every hypothesis before evaluator time is spent.
- Make successful combinations easy to reproduce and failed ideas hard to repeat.

## Adopted ideas

The initial playbook incorporates the best publicly evidenced directions:

1. Retain stripe 0 for pair-only columns when memory headroom makes recomputation unnecessary.
2. Plan cached-column decommitment in aligned runs so shared fold work is performed once.
3. Parallelize input parsing and setup only when it overlaps real precompute work.
4. Schedule writer interpolation, verifier fill, LogUp tails, and FRI-input sums into otherwise idle windows.
5. Treat retained coefficients, preprocessed trees, and regrown columns as an explicit memory-lifetime problem.
6. Benchmark SIMD batch inversion, lazy reductions, and fused folds only after proving they are on the whole-proof critical path.
7. Use a strict verifier/evidence gate: fresh inputs, alternating order, byte identity, proof verification, portable tests, and memory checks.

## Quick start

```bash
./scripts/mark_phases.sh /absolute/path/to/proving
./scripts/run_experiment.sh /absolute/path/to/proving baseline-name
```

The scripts are deliberately conservative. They do not change the proving tree, weaken checks, change proof parameters, or submit jobs automatically.

## Agent roles

The recommended harness has five independent reviewers:

- `roles/critical_path_profiler.md` — phase time, CPU occupancy, and serial windows.
- `roles/memory_lifetime_optimizer.md` — residency, release points, and retain-versus-regrow tradeoffs.
- `roles/overlap_scheduler.md` — rayon task overlap without increasing peak memory.
- `roles/kernel_simd_specialist.md` — SIMD, LogUp, FFT, inversion, and hashing candidates.
- `roles/verifier_evidence_reviewer.md` — byte identity, verification, portability, and graph provenance.

## Research protocol

Read `docs/RESEARCH_PLAYBOOK.md` before changing the prover. New ideas belong in `templates/lead.json` and `experiments/leader-techniques.md` before evaluator time is used.

## Current status

The first seeded research pass is documented in `experiments/leader-techniques.md`. The harness currently provides phase markers, a conservative local measurement wrapper, evidence templates, and role prompts. The remote GitHub publication step is pending GitHub authentication for the local account.

## Sources

- [Mainnet evaluator results](https://stwo.provably.fast/api/participation/internal/results)
- [Public challenge graph](https://provably.fast/api/participation/graph?campaign_digest=2b481067083737b89704e6b4b7f77a6a7accf0bb66aa91104275a01592fb38f8&cursor=0&limit=100)
- [Stwo proving repository](https://github.com/starknet-innovation/proving)
- [OpenZeppelin Cairo PQ verifier benchmarks](https://github.com/OpenZeppelin/cairo-pq-verifiers)


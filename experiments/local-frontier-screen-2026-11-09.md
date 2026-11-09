# Local frontier screen — 2026-11-09

## Setup

- Candidate tree: `/Users/darshan/starks-challenge/candidate`
- Frontier tools tree: `/Users/darshan/starks-challenge/pf-tools`
- Pinned commit: `9df00f2249b61849be69cc14942742d0c1f81ad2`
- Host: macOS arm64, 8 CPUs
- Measurement: `pvfast-participant-1.50.0/measure.py`
- Pairs: 10, alternating order, fresh statement per pair

## Result

- Every pair produced a byte-identical proof.
- Every candidate proof was accepted by the independent verifier.
- Geometric-mean time ratio: `0.9995`.
- Local 95% interval: `0.9430–1.0593`.
- Peak memory: unavailable because the macOS sandbox denied the system time utility's RSS query.

This is a correctness pass and a timing tie, not a promotion signal. The machine is not evaluator-equivalent, and the interval is far too wide to establish a 1% improvement.

## Phase signal

The candidate was faster in Cairo interaction (`-12.7%` median) but slower in circuit base commitment (`+12.3%`) and slightly slower in circuit interaction and interaction commitment (`+3.8%` each). The next experiment should therefore target circuit-side scheduling or residency, not add more Cairo-only overlap.

## Next action

Run the ablation matrix for retained columns, preprocessing cache, pooling, overlap, and SIMD changes. Then prototype a bounded circuit-side readiness schedule with fixed output ordering and a measured live-set bound.


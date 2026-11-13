# H3.1 — Sonic parser for prepared prover input

Status: local screen did not establish a speed gain; not submitted.

## Hypothesis

Replacing `serde_json::from_slice::<ProverInput>` with `sonic_rs::from_slice::<ProverInput>` in `pvfast_workload` may reduce the time spent parsing the large prepared input without changing the proof statement or bytes. This is an isolated parser hypothesis inspired by, but not a reproduction of, binharold's supported parser stack (`rgr1_507fe23508d81ddfe1d100705b70105f9eb7740b989e23885b87180cceb50860`).

## Candidate

- Frontier: `69645ee0` applied to pinned `9df00f2249b61849be69cc14942742d0c1f81ad2`.
- Change: add the already locked `sonic-rs` workspace dependency to `privacy-prove`; use its byte-slice parser for the prepared `ProverInput`.
- No proof parameters or data were changed.
- Candidate tree: `/Users/darshan/starks-challenge/candidate-69645ee0-logup` (disposable); frontier tree left untouched.
- Build: `cargo +nightly-2026-01-15 build --release --locked -p privacy-prove --bin privacy_prove_stdout`, with evaluator `RUSTFLAGS`.

## Measurement

Command: official `measure.py --tools pf-tools-69645ee0 --candidate candidate-69645ee0-logup --pairs 10`.

Host: Darwin arm64; evaluator target is Linux x86-64 AVX2 / 16 cores. The harness alternated sides over fresh statements and checked proof-byte identity plus independent verification.

- Warm-up ratio: 1.0215.
- Pair ratios: 1.0430, 1.1184, 1.1332, 1.1513, 1.0274, 0.9752, 0.8602, 1.0456, 0.9202, 0.9108.
- Aggregate candidate/frontier ratio: 1.0140; 95% interval `[0.9442, 1.0890]`.
- Every proof was byte-identical and independently verified.
- Peak memory: unavailable under the Mac timer; must be measured on Linux.
- Phase timings were mixed and noisy, with no consistent parser-specific whole-proof signal.

## Decision

Do not submit. The run neither demonstrates the required whole-proof speedup nor measures memory. Reconsider only with Linux x86-64 evidence or as a separately justified component of a broader supported stack whose exact patch is available.

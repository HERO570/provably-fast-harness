# Reproducing the measured target-list cache

[target-reuse-on-69645ee0.patch](target-reuse-on-69645ee0.patch) is the exact two-file source increment for the [completed local experiment](../target-reuse-result-2026-11-17.md). It is published for reproduction, not as a demonstrated speed improvement. It does **not** implement the separate inverse-scatter hypothesis.

- Expected source base: official `69645ee0e6f583990c4aaf064cd08fde601ff009`.
- Patch SHA-256: `7cdd9d7c5282d39d29fb9610fcad612dc843ad4452aecaea50dc72ff441a06cc`.
- Also attached to [the Provably follow-up](https://stwo.provably.fast/threads/bt1_777ece046670d1dc46e006ec), post `bp1_030c15e2d971de0f6aa68e76`, as content-addressed blob `cas1_7cdd9d7c5282d39d29fb9610fcad612dc843ad4452aecaea50dc72ff441a06cc` (15,265 bytes).
- Scope: `crates/stwo/src/prover/pcs/mod.rs` and new `crates/stwo/src/prover/pcs/target_rows_cache_tests.rs` only.
- No build helpers, inputs, preimages, proof files, binaries, credentials, or full frontier diff are included.

The exported new-file header was corrected before publication. Read-only `git apply --check` passed on the verified frontier worktree. Applying the artifact to a temporary two-file reproduction also yielded exactly the measured postimages:

| Path | Expected SHA-256 after application |
|---|---|
| `crates/stwo/src/prover/pcs/mod.rs` | `3498174c5c12be54837060520faaa3c1b3cbdeada708521d0a4cbc4fd2397c2b` |
| `crates/stwo/src/prover/pcs/target_rows_cache_tests.rs` | `30874fe7e525fa8d3d203f97b97e05900fd97661709a7464ffd4e12e0e0d6909` |

Neither live proving checkout nor its index was changed by these artifact checks. No new proof or performance experiment was run. The original source base, measured compiler flags, tests, results, and evidence limits are recorded in the linked experiment report.

Apply only to file contents matching the stated base. For a challenge submission, keep HEAD at the pinned commit and include the current frontier changes as required by the client; this small increment is **not** the complete canonical upload patch against pinned HEAD. It does not establish current-frontier novelty or Linux performance by itself.

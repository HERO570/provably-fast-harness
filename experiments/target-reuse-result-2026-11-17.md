# Target metadata reuse: complete local result

The per-invocation striped-decommit row-target cache passed ten fresh alternating proof pairs, but did not establish a whole-proof speed improvement. **Do not submit this patch to the evaluator on this evidence.** This is a completed local inconclusive screen, not an official graph `REFUTES` verdict.

Published workshop account: [Local target-plan reuse: correct proofs, no resolved speed gain](https://stwo.provably.fast/threads/bt1_777ece046670d1dc46e006ec), post `bp1_9d6a4dc1877ba35c2560f142`. The thread is pinned to our admitted lead; the creation receipt did not include a new graph discussion-record ID. This is a workshop publication, not an evaluator submission or points award.

The [exact measured two-file increment and postimage hashes](patches/README.md) are available for reproduction. The patch passes an applicability check against the stated frontier and reconstructs the measured source bytes in a temporary copy. This publication adds no new timing evidence.

| Check | Complete replacement run |
|---|---|
| Measured pairs | 10, fresh inputs, alternating order |
| Exact proof bytes and pinned independent verification | 10/10 passed |
| Command exit status | 0, recorded by the completed command session |
| Candidate/frontier geometric-mean time ratio | 1.0087 |
| 95% time-ratio interval | 0.9726–1.0462 |
| Frontier median peak | 2.84 GiB, range 2.83–2.88 |
| Candidate median peak | 2.86 GiB, range 2.83–2.89 |
| Reported median peak delta | +0.6% |
| Host | Darwin ARM64, 8 CPUs, load average 17 |

The point estimate is 0.87% slower, and the interval includes both improvement and regression. The reported memory increase is within the local 1% envelope; that does not establish evaluator-host memory. Percentages come directly from the client, rather than being recomputed from rounded GiB values. The Linux x86-64 AVX2/16-core evaluator differs from this busy local host.

The [complete 37-line log](results/target-reuse-arm64-2026-11-17.log) is published verbatim after checking it contains only timing, memory, verification status, host cautions, and phase summaries. It contains no proof inputs, preimages, proof bodies, credentials, or private source. SHA-256: `6893c3b65776c6d5d0fae7b36d9b2724115aa42eff153463e0772dffae43cf55`. Exit status is supported separately by the retained local attribution report; it is not a line in this log.

## Source and measurement identity

The baseline source was official frontier `69645ee0e6f583990c4aaf064cd08fde601ff009`, with candidate Git HEAD left pinned at `9df00f2249b61849be69cc14942742d0c1f81ad2` and changes unstaged. Toolchain: `nightly-2026-01-15`; baseline and candidate flags: `-C target-cpu=native -C opt-level=3 -C codegen-units=1`. The corrected focused tests passed 3/3; the corrected native build succeeded in 15m52s.

The unchanged client's command was:

```sh
set -o pipefail
python3 -u measure.py --tools /path/to/pf-tools-69645ee0 --candidate /path/to/candidate-decommit-targets-69645ee0 --pairs 10 --time /usr/bin/time 2>&1 | tee /path/to/new-measurement.log
```

The complete command used absolute local paths. The placeholders above describe the invocation without publishing personal filesystem paths. Source, executable, client, and fixed tool identities matched before and after execution:

| Artifact | SHA-256 |
|---|---|
| Candidate executable | `dbf583dd7ecb580b152c881eff314424d9c69bd4674da2a9963a313b12a1b83f` |
| Baseline executable | `a90d8fc9b89e1d6c0152bd5b0b3eee1d1b5431dee4c19f8f919cc5380af736d8` |
| Unchanged client `measure.py` | `375847f44473011fc555d42f6e3676c819be7bd68422b78e19da8e1b57cc9f87` |
| Candidate `pcs/mod.rs` | `3498174c5c12be54837060520faaa3c1b3cbdeada708521d0a4cbc4fd2397c2b` |
| Standalone tests | `30874fe7e525fa8d3d203f97b97e05900fd97661709a7464ffd4e12e0e0d6909` |

The earlier incomplete capture remains preserved separately. Its four measured pairs were not selected, combined, or counted in this replacement run. The candidate, temporary build helper, baseline binaries, and old failed experiments remain unchanged. No evaluator job was submitted; full final submission preflight was not run.

## Lineage and next decision

Admitted lead: `rgr1_dadb7c21852f1ecfc477e9077beb7c15681f0a6021c35648604900de2ad58346`, proposal `20583edf-7fdc-435a-93ff-c31eb1ae458e`. It is `DERIVED_FROM` the striped-decommit/run-planning lead `rgr1_ea084ee5484191ae60a9dbde8c6074879eaef5f3fb8ebe2e7966191b025ac420`. Lineage is context, not transferred performance support.

Both the coordinator and independent reviewer read the complete result and withheld speed clearance. Coarse phase medians are mixed; they do not isolate target-list construction or repeated index searches. No unchanged rerun, new build, or additional mechanism follows from this result alone.

The strongest next step is profiling the current frontier on a quiet Linux x86-64 AVX2 host, then measuring whether target construction or inverse scatter-map work is large enough to matter. The bounded Herdr inventory returned no configured machines; that does not rule out unlisted infrastructure. Source review identified a precise inverse-index reuse hypothesis, but its workload counts, bottleneck share, exact related payload, and measured benefit remain unknown. It is not stacked into this candidate.

No Stwo points or promotion are confirmed. See [scoring rules](../docs/SCORING.md) and the [experiment history](../docs/EXPERIMENT_PER_DECOMMIT_TARGETS.md).

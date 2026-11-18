# Twenty more Stwo references — batch 3, 2026-11-18

Twenty distinct primary references, checked against all 115 SOURCE records that preceded this batch. The complete source-feed pagination was reconciled with retained full records and the [first twenty](../stwo-sources-2026-11-18/README.md) plus [next twenty-one](../stwo-sources-batch2-2026-11-18/README.md). Identifier normalization covers DOI/arXiv/ePrint forms, with title and work-level review as well. No known duplicate is counted here.

Current receipt summary: 20 ADMITTED. The [machine-readable catalog](catalog.json) records actual IDs, decisions, relevance and limits. Submission, graph admission and measured evaluator outcomes remain separate; no performance points or speedup is claimed.

Eighteen primary URLs were retrieved successfully and hashed. Two papers use registered DOI metadata, checked against primary abstracts; the hash then identifies the metadata response, not a full paper. Review is bounded to the stated abstracts/manual sections and targeted passages. PDFs, private inputs, proof artifacts, binaries and credentials are not republished.

## Selection and immediate uses

- **Arithmetic/work:** sumcheck tradeoffs and small-value/equality-factor techniques are context for examining actual Stwo factors. Harvey provides reduction-range prior art; barycentric interpolation informs evaluation identities; prefix scans require exact associative/representation reasoning. No protocol substitution is proposed.
- **Memory/scheduling:** Cilk-5 illustrates common-path task overhead, while three different allocator designs expose locality, cross-thread ownership and fragmentation questions. They are not new allocator dependencies or evidence that Stwo should change its pool.
- **Attribution:** distinguish CPU samples, off-CPU blocking, runnable delay, pressure and frequency policy. No diagnostic tool is run, no host setting changes and no benchmark pair is selected or discarded in this task.
- **Correctness:** an exact cache equivalence relation is different from a hash/fingerprint; generated differential cases must be well-defined and independently checked. Sources do not establish a vulnerability or clear our existing denominator test gap.

The 2025 sumcheck tradeoff reference covers the earlier 2024/524 family; that predecessor is not counted separately. The extended 2026/587 work combines 2025/1117, 2024/1046 and 2024/1210, and those incorporated papers are also excluded from the count. This avoids turning versions of related work into activity volume.

## Index

| # | Reference | Stwo question |
|---:|---|---|
| 1 | [Time-Space Trade-Offs for Sumcheck](https://eprint.iacr.org/2025/1473) | Bounded sumcheck memory/work tradeoffs |
| 2 | [Speeding Up Sum-Check Proving (Extended Version)](https://eprint.iacr.org/2026/587) | Small-value and structured-factor arithmetic |
| 3 | [Barycentric Lagrange Interpolation](https://people.maths.ox.ac.uk/trefethen/barycentric.pdf) | Opening evaluation and denominator reuse |
| 4 | [Faster arithmetic for number-theoretic transforms](https://arxiv.org/abs/1205.2926) | Lazy modular reduction invariants |
| 5 | [Prefix Sums and Their Applications](https://www.cs.cmu.edu/~scandal/papers/CMU-CS-90-190.html) | Parallel scan work and dependencies |
| 6 | [The Implementation of the Cilk-5 Multithreaded Language](https://doi.org/10.1145/277650.277725) | Work-first scheduling overhead |
| 7 | [Mimalloc: Free List Sharding in Action](https://www.microsoft.com/en-us/research/publication/mimalloc-free-list-sharding-in-action/) | Locality and bounded free-list design |
| 8 | [snmalloc: A Message Passing Allocator](https://www.microsoft.com/en-us/research/wp-content/uploads/2020/04/snmalloc.pdf) | Cross-thread buffer ownership and frees |
| 9 | [A Scalable Concurrent malloc(3) Implementation for FreeBSD](https://people.freebsd.org/~jasone/jemalloc/bsdcan2006/jemalloc.pdf) | Fragmentation, locality and application-wide allocator cost |
| 10 | [perf-sched Linux manual: scheduling latency and timehist](https://man7.org/linux/man-pages/man1/perf-sched.1.html) | Runnable versus blocked worker time |
| 11 | [Flame Graphs: visualization and interpretation of profiled stacks](https://www.brendangregg.com/flamegraphs.html) | Attributable sampled hot paths |
| 12 | [Off-CPU Analysis](https://www.brendangregg.com/offcpuanalysis.html) | Blocked-time attribution |
| 13 | [Linux Pressure Stall Information](https://docs.kernel.org/accounting/psi.html) | Host contention evidence |
| 14 | [Linux CPU Performance Scaling](https://docs.kernel.org/admin-guide/pm/cpufreq.html) | Frequency-policy context for measurement |
| 15 | [Rust Performance Book: Bounds Checks](https://nnethercote.github.io/perf-book/bounds-checks.html) | Safe hot-loop representation |
| 16 | [llvm-exegesis: LLVM Machine Instruction Benchmark](https://llvm.org/docs/CommandGuide/llvm-exegesis.html) | Measured instruction-cost calibration |
| 17 | [Rust Unstable Book: sanitizer instrumentation and supported targets](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html) | Native memory/race diagnostics |
| 18 | [Finding and Understanding Bugs in C Compilers](https://doi.org/10.1145/1993498.1993532) | Defined-input differential testing methodology |
| 19 | [SIMD Compression and the Intersection of Sorted Integers](https://arxiv.org/abs/1401.6399) | Sorted target-index search work |
| 20 | [Type-Safe Modular Hash-Consing](https://usr.lmf.cnrs.fr/~jcf/publis/hash-consing2.pdf) | Exact cache-key equivalence and sharing |

## 1. Time-Space Trade-Offs for Sumcheck

Primary reference: [https://eprint.iacr.org/2025/1473](https://eprint.iacr.org/2025/1473)

**Source covers:** Primary streaming sumcheck algorithms and time-space bounds for multilinear claims and products.

**Stwo use and limits:** Provides a framework for pricing retained tables versus recomputation in Stwo GKR/sumcheck work. Confirm the actual polynomial form and available enumeration before transferring any algorithm. Reduced space can cost time, so neither the paper's asymptotics nor memory reductions establish a mainnet speed win. Protocol changes and reordered transcript messages remain disallowed; preserve exact round outputs and the whole-proof memory/time gates. The earlier 2024/524 family is not counted as another source here.

**Concrete use:** Count actual retained entries and repeated round work before proposing a bounded tradeoff.

**Receipt:** ADMITTED; proposal `084d5695-a686-42e0-867f-ad1451894f28`; record `rgr1_1608f3fea805198f682495d8f04b56107bfb0ef9e3d6360cea187ac80fb91b1b`.

## 2. Speeding Up Sum-Check Proving (Extended Version)

Primary reference: [https://eprint.iacr.org/2026/587](https://eprint.iacr.org/2026/587)

**Source covers:** Primary algorithms for sumcheck products, small-value inputs, streaming and equality-polynomial structure.

**Stwo use and limits:** A useful comparison for the external witness-dot prototype: determine whether actual Stwo factors lie in a small subfield or have reusable structure before replacing repeated extension-field arithmetic. The paper's Jolt results do not transfer to Stwo, and small-value assumptions must hold for every accepted input. Preserve the current protocol, all terms, raw/canonical representations and verifier checks. This extended work is counted once, not once per incorporated precursor.

**Concrete use:** Identify and prove any applicable value/range invariant before timing a separate arithmetic kernel.

**Receipt:** ADMITTED; proposal `de465a14-9934-4f78-9816-c44d7d2cb5da`; record `rgr1_5bffd59984e76cb40c8a169bf9e84941aa41a2d9e232261734187dd6acf32c10`.

## 3. Barycentric Lagrange Interpolation

Primary reference: [https://people.maths.ox.ac.uk/trefethen/barycentric.pdf](https://people.maths.ox.ac.uk/trefethen/barycentric.pdf)

**Source covers:** Primary barycentric interpolation formulas and precomputed-weight organization.

**Stwo use and limits:** Mathematical background for repeated evaluation at opening/sample points in Stwo PCS. Standard polynomial barycentric formulas are not automatically the circle-basis formulas; derive the exact field/domain identity and handle evaluation at nodes before considering reused weights or inverse denominators. Floating-point stability and reported numerical performance are not finite-field speed evidence. Cache keys must include the full relevant domain/sample context and obey the memory bound.

**Concrete use:** Compare exact evaluation identities and repeated denominator work on the actual opening path.

**Receipt:** ADMITTED; proposal `f6c33cc3-ba99-4caf-b4cd-10c32b843c24`; record `rgr1_4fc3a78987a1c076890c01f401b589e0fff961706a755c72f97aabcf4e7936e9`.

## 4. Faster arithmetic for number-theoretic transforms

Primary reference: [https://arxiv.org/abs/1205.2926](https://arxiv.org/abs/1205.2926)

**Source covers:** Primary use of redundant modular representations to reduce transform arithmetic cost.

**Stwo use and limits:** Relevant prior art for reducing normalization work in M31/QM31 SIMD and transform kernels. Stwo uses circle transforms and specific packed/raw representations, so ordinary NTT formulas and the paper's timing are not drop-in results. Prove every intermediate-width bound, reduction point and final representation, retain the portable path and compare exact outputs. No new intrinsic path or parameter change follows from the reference alone.

**Concrete use:** Write range/canonicalization proofs before testing a delayed-reduction rewrite.

**Receipt:** ADMITTED; proposal `cb7b87dd-467f-4c71-8d46-1b64120eda76`; record `rgr1_41b011e29655b0dd4ac515dd3002911d90aec9b36e04c423f3caea4dfe330328`.

## 5. Prefix Sums and Their Applications

Primary reference: [https://www.cs.cmu.edu/~scandal/papers/CMU-CS-90-190.html](https://www.cs.cmu.edu/~scandal/papers/CMU-CS-90-190.html)

**Source covers:** Author-hosted prefix-sum report and associative scan as a parallel building block.

**Stwo use and limits:** Useful for studying LogUp accumulations and cumulative-product chains, where apparent serial dependencies may admit a bounded scan organization. The operator must satisfy the necessary algebraic conditions and raw packed output conventions must still match the existing implementation. Preserve segmented/column boundaries and account for all scratch and concurrent scans. It does not prove a current Stwo bottleneck or authorize a different runtime or new threads outside Rayon.

**Concrete use:** Measure the exposed chain and derive an exact-output scan with explicit scratch accounting.

**Receipt:** ADMITTED; proposal `f821013c-ef91-45fc-b307-571f40ab8362`; record `rgr1_747b97b40914e5b50dd2747a2a71e48421c37ef1b22c73ef64bfd259468d968f`.

## 6. The Implementation of the Cilk-5 Multithreaded Language

Primary reference: [https://doi.org/10.1145/277650.277725](https://doi.org/10.1145/277650.277725)

**Source covers:** Primary implementation study of the work-first principle and work/critical-path overhead.

**Stwo use and limits:** Complements the already-recorded work-stealing theorem with a concrete implementation lesson for bounded Cairo writer/interpolation scheduling: additional parallel tasks can increase common-path work even when they expose more concurrency. Evaluate actual Rayon task granularity and critical completion, without replacing Rayon with Cilk or transferring historical spawn-cost constants. Maintain lookup dependencies, indexed commitment order and the frontier memory cap.

**Concrete use:** Count added task/setup overhead and the actually shortened dependency path together.

**Receipt:** ADMITTED; proposal `8a13b9f9-af18-447c-bfc7-650eb5701bec`; record `rgr1_7ff5344ef1bef1dbcfa60f3464504b1c81a589b5e3d9bb7b9e41e60e478046bf`.

## 7. Mimalloc: Free List Sharding in Action

Primary reference: [https://www.microsoft.com/en-us/research/publication/mimalloc-free-list-sharding-in-action/](https://www.microsoft.com/en-us/research/publication/mimalloc-free-list-sharding-in-action/)

**Source covers:** Primary allocator design using page-local sharded free lists and maintenance cadence.

**Stwo use and limits:** Design context for Stwo's existing column/large-block pools: distinguish fast reuse, maintenance work and retained-page costs before changing pool organization. This is not a proposal to link mimalloc, add a crate or replace the evaluator allocator. A pool variant must account for total live and idle capacity across tasks and measure the complete proof; the paper's allocator results do not predict Stwo time or peak.

**Concrete use:** Check whether a proposed reuse fast path adds maintenance or retained-memory costs elsewhere.

**Receipt:** ADMITTED; proposal `5e032bcf-eac6-4ed6-bef7-348fc6829f23`; record `rgr1_481cad2cc164a2f274dccf0bd17b2c09fcda66d244358b96bc74319ef0e2801b`.

## 8. snmalloc: A Message Passing Allocator

Primary reference: [https://www.microsoft.com/en-us/research/wp-content/uploads/2020/04/snmalloc.pdf](https://www.microsoft.com/en-us/research/wp-content/uploads/2020/04/snmalloc.pdf)

**Source covers:** Primary batched return-to-origin allocator design for producer/consumer workloads.

**Stwo use and limits:** Relevant to buffers allocated by one Rayon worker and released by another during writer/FFT/commit phases. It motivates attributing cross-thread return traffic and contention before changing the existing pool, not adding snmalloc or a new runtime. Batched returns may extend idle-buffer lifetime; quantify total retained capacity and all concurrent owners against the frontier guard. No current cross-thread-free hotspot or improvement is established.

**Concrete use:** Trace allocation, last use and return ownership before evaluating bounded return batching.

**Receipt:** ADMITTED; proposal `251f5974-b548-4a18-8828-475edb9bd8e0`; record `rgr1_025e385b500cdd5712176e60f573b9923a43571b3f1d45bfe20bdf9e5f9161c6`.

## 9. A Scalable Concurrent malloc(3) Implementation for FreeBSD

Primary reference: [https://people.freebsd.org/~jasone/jemalloc/bsdcan2006/jemalloc.pdf](https://people.freebsd.org/~jasone/jemalloc/bsdcan2006/jemalloc.pdf)

**Source covers:** Primary jemalloc design discussion and its application-level measurement rationale.

**Stwo use and limits:** A contrasting allocator design reference for analyzing Stwo pool retention, arenas, cache locality and fragmentation. It emphasizes application behavior rather than allocator-call microbenchmarks, matching the whole-proof objective. This does not authorize adding jemalloc or changing the package set. Historical FreeBSD results and arena counts are not Linux-mainnet constants; measure any change's full proof time and actual peak, including scratch and idle memory.

**Concrete use:** Separate internal/external waste from reuse benefits before changing pool policy.

**Receipt:** ADMITTED; proposal `3bb8746f-f35e-48db-a071-2def62c737b1`; record `rgr1_bd2439b778c0552b071883a958c16298d21571235c030012aaa9e024a55a19c2`.

## 10. perf-sched Linux manual: scheduling latency and timehist

Primary reference: [https://man7.org/linux/man-pages/man1/perf-sched.1.html](https://man7.org/linux/man-pages/man1/perf-sched.1.html)

**Source covers:** Primary tool manual for scheduler tracing, latency and per-task timeline reports.

**Stwo use and limits:** Can help distinguish an idle Rayon window caused by dependency/blocking from runnable work delayed by host contention. Use only an available supported Linux host and keep tracing separate from acceptance pairs. Kernel scheduler events do not by themselves identify the prover's task DAG, so correlate source/phase context before changing writer scheduling. No system policy, evaluator harness or prover file access is changed in this source task.

**Concrete use:** Attribute an observed idle window before adding tasks or overlap.

**Receipt:** ADMITTED; proposal `7ebed374-d334-43b1-983e-061403260a60`; record `rgr1_396dd871c2efc52d3fe53e8ae1ff75a23fdc28e3dd8be539257990b94c9e2a01`.

## 11. Flame Graphs: visualization and interpretation of profiled stacks

Primary reference: [https://www.brendangregg.com/flamegraphs.html](https://www.brendangregg.com/flamegraphs.html)

**Source covers:** Originator's guidance on stack-sample visualization, variants and interpretation.

**Stwo use and limits:** A method for retaining readable per-leg CPU profiles and checking whether a proposed optimization touches meaningful executing work. Flame width represents the profiled quantity, not automatically wall-clock latency or a critical-path saving. Separate on-CPU, off-CPU and differential views, preserve binary/source identity and use sampling as advisory evidence. Do not infer a mainnet speedup from a wide frame or replace the exact-proof measurement gate.

**Concrete use:** Build a phase-scoped profile packet that names the cost and its measurement unit.

**Receipt:** ADMITTED; proposal `18ee9525-55e3-489b-bb6b-6ddf1817c988`; record `rgr1_1b1da4f8e0621e153afb279a602a22199be11038904b8ddf446b0d746f94be92`.

## 12. Off-CPU Analysis

Primary reference: [https://www.brendangregg.com/offcpuanalysis.html](https://www.brendangregg.com/offcpuanalysis.html)

**Source covers:** Author's methodology for analyzing time blocked off CPU and its context.

**Stwo use and limits:** Complements CPU profiles when a Stwo proof's wall time contains waiting on allocation, joins, locks or host resources. Off-CPU stacks alone do not prove removable work, and runnable scheduler delay is a different question; correlate with the actual task and phase boundaries. Collect diagnostic data outside the submitted code and do not mix tracing overhead into acceptance timing. No current lock or wait is asserted from the guide alone.

**Concrete use:** Separate blocking, runnable delay and true compute before choosing an overlap experiment.

**Receipt:** ADMITTED; proposal `98d30089-01ef-4a4f-8c79-ba87080d90ab`; record `rgr1_a22b690414f93a7264ec23d57400e118a1c0e479939a2c6c4d2a73de1ad9be49`.

## 13. Linux Pressure Stall Information

Primary reference: [https://docs.kernel.org/accounting/psi.html](https://docs.kernel.org/accounting/psi.html)

**Source covers:** Kernel definitions of CPU, memory and I/O pressure and stall reporting.

**Stwo use and limits:** Useful for documenting whether an available Linux benchmark host experiences resource pressure during a diagnostic session, rather than interpreting load average alone. Host-wide/cgroup pressure does not identify which prover operation caused a stall or excuse selecting favorable pairs. Keep collection outside the patch, retain the prescribed estimator and complete ordered pairs, and do not pause other workloads or alter host policy without authorization.

**Concrete use:** Record contention context separately from candidate/frontier timing conclusions.

**Receipt:** ADMITTED; proposal `19e37684-09e4-4b8d-a750-942a743519c9`; record `rgr1_1c95dd1c96849ebf664d6f86c9e08bc59c79555b19f314d903b65ae64fc19769`.

## 14. Linux CPU Performance Scaling

Primary reference: [https://docs.kernel.org/admin-guide/pm/cpufreq.html](https://docs.kernel.org/admin-guide/pm/cpufreq.html)

**Source covers:** Kernel explanation of performance states, scaling policies and governors.

**Stwo use and limits:** Helps interpret build/kernel timing differences across runs and hosts when CPU frequency behavior may differ. The source is context for reporting the actual environment, not a plan to force a governor, disable turbo or change evaluator configuration. Frequency observations do not prove a code change faster; preserve matched binaries, alternating proof order and confidence/memory checks. No host setting is changed in this task.

**Concrete use:** Document frequency-policy context when diagnosing unexpected timing variation.

**Receipt:** ADMITTED; proposal `c6dd8cb2-6e0c-49c1-93e1-49083181ae44`; record `rgr1_6b936d1a4d9c19172cf01a21ba9a1b9e948062bd929d509faff3ce2e32b5abbb`.

## 15. Rust Performance Book: Bounds Checks

Primary reference: [https://nnethercote.github.io/perf-book/bounds-checks.html](https://nnethercote.github.io/perf-book/bounds-checks.html)

**Source covers:** Author's guidance on safe iteration and range information that can help eliminate bounds checks.

**Stwo use and limits:** Relevant to repeated row, lane and coefficient indexing in Stwo witness, opening and verifier-fill loops. Establish actual compiled check overhead before replacing indexing with safe slices/iterators and preserve padding, lengths and failure behavior. The chapter's unsafe fallback is not permission for new unsafe outside the allowed SIMD surface. Changes must remain exact and pass full proof timing; source-level elegance is not a measured saving.

**Concrete use:** Inspect generated hot loops, then test one safe bounds-information rewrite if justified.

**Receipt:** ADMITTED; proposal `04e12779-c605-43e9-8502-f63379690b2a`; record `rgr1_b0b75d54fb87525d8e208b282e32837673827e9db813e3c231d57899bf68bb5a`.

## 16. llvm-exegesis: LLVM Machine Instruction Benchmark

Primary reference: [https://llvm.org/docs/CommandGuide/llvm-exegesis.html](https://llvm.org/docs/CommandGuide/llvm-exegesis.html)

**Source covers:** Primary tool documentation for measuring instruction scheduling properties on supported targets.

**Stwo use and limits:** Complements llvm-mca's model with possible same-host latency/throughput measurements for exact x86 forms used in M31/QM31 or shuffle kernels. CPU and tool support, measurement noise and instruction setup matter. This is a separate diagnostic, not an evaluator-host result or whole-proof prediction, and no generated benchmark/runtime code enters the candidate. Do not provision hardware or change the evaluator build to use it.

**Concrete use:** Validate a suspected instruction-model discrepancy before designing another SIMD rewrite.

**Receipt:** ADMITTED; proposal `7a35ab59-985b-4a6f-9ee8-b4254c7191dc`; record `rgr1_e6f57e368e949d5db3e53d0bb109a911277aabf214ea65f8e2b61a4505d271c1`.

## 17. Rust Unstable Book: sanitizer instrumentation and supported targets

Primary reference: [https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html)

**Source covers:** Official sanitizer options, support constraints and runtime-overhead warnings.

**Stwo use and limits:** Potential offline diagnostics for pooled trace buffers and permitted SIMD unsafe paths where a portable interpreter may not cover native behavior. Verify pinned-toolchain and target support; a clean instrumented run is neither a proof of soundness nor complete race/UB coverage. Sanitizer binaries and their timings differ from release acceptance builds. No new dependency, flags in the evaluator build, protected-test edit or diagnostic run occurs here.

**Concrete use:** Choose a supported bounded native safety check before performance measurement.

**Receipt:** ADMITTED; proposal `f2dd116f-f4d9-4b8a-88e4-7fcd3bfd3690`; record `rgr1_9283409676142b814c8b1b0281f7c6382567824dc895115de08dd5262a5dea54`.

## 18. Finding and Understanding Bugs in C Compilers

Primary reference: [https://doi.org/10.1145/1993498.1993532](https://doi.org/10.1145/1993498.1993532)

**Source covers:** Primary Csmith research on differential compiler testing with generated well-defined programs.

**Stwo use and limits:** Methodological reference for ensuring raw-field, denominator and portable/SIMD differential cases satisfy stated preconditions and use an independent original/oracle. C programs and Csmith are not a drop-in Rust or proof generator, and compiler diversity is not independent proof verification. Apply the validity/oracle discipline without adding a package or replacing the challenge's fresh-input byte-equality/verifier checks. No Csmith run or newly found compiler bug is claimed.

**Concrete use:** Reject invalid fixtures and preserve minimized, independently checked differential cases.

**Receipt:** ADMITTED; proposal `d11c7ac7-4401-4e4e-b5c6-b6c221eaf824`; record `rgr1_f07057e8a321b39a5bd29508211b24b225013f87ddb64ad32e052f6bc63b622c`.

## 19. SIMD Compression and the Intersection of Sorted Integers

Primary reference: [https://arxiv.org/abs/1401.6399](https://arxiv.org/abs/1401.6399)

**Source covers:** Primary SIMD list-intersection and galloping algorithms with evaluation of end-to-end list processing.

**Stwo use and limits:** Prior art for repeated sorted row/target searches in decommit_striped and an unimplemented inverse-scatter extension. Stwo lists may be much shorter than retrieval posting lists, so setup and extra metadata can erase any kernel benefit. Preserve original query duplicates/order, sorted target identity and resident-prefix mapping; do not introduce a compression format or assume the paper's gains transfer. Respect portable-path and memory limits.

**Concrete use:** Count actual list lengths/searches and compare a bounded identical mapping before implementation.

**Receipt:** ADMITTED; proposal `7c39e236-219d-46c9-b8bf-5416afe28b23`; record `rgr1_a491db6db95066f7f4930eae52bb52db3ff3220657b37712b51d3e1c338bb610`.

## 20. Type-Safe Modular Hash-Consing

Primary reference: [https://usr.lmf.cnrs.fr/~jcf/publis/hash-consing2.pdf](https://usr.lmf.cnrs.fr/~jcf/publis/hash-consing2.pdf)

**Source covers:** Primary hash-consing design using explicit equality to share structurally equivalent values.

**Stwo use and limits:** Useful correctness context for preprocessing and target-plan caches: distinguish a hash used to find a candidate from the full equivalence relation that authorizes reuse. A sampled fingerprint must not be assumed to prove all relevant inputs equal; define complete statement/domain/configuration keys and adversarial mutation tests. OCaml weak tables and the paper's library are not a Rust implementation plan. No accepted-invalid-proof finding or cache speedup is established by this reference.

**Concrete use:** State and independently test the exact equality required for every cache-hit path.

**Receipt:** ADMITTED; proposal `3c568228-3e26-4cd8-8a42-38534e897c86`; record `rgr1_af91e7a4445865631a718cc44b21876b86f2624466a47b05e84d70d384132ed1`.

## Publication and implementation boundary

The publication destination is one index in the existing [Stwo workshop thread](https://stwo.provably.fast/threads/bt1_80663a8c1d05345a9ed49ac0), with actual status and attachment hashes recorded in publication.json when available. Mutable reference pages may change; the local retrieval hashes identify only the observed bytes, not independent verification of every claim.

No prover source, build, test, proof input, protocol setting or measurement changed in this source task. Candidate work remains subject to the original safety/edit surface, portable correctness, fresh alternating byte-identical/independently verified proof pairs, whole-proof timing, memory and final policy gates. The next useful step is to apply the exact-equivalence and valid-input lessons, then establish material exposed work before a new speed patch.

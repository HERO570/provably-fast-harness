# Twenty primary sources for Stwo research — 2026-11-18

This is a bounded source catalog for [Stwo Challenge 2](https://stwo.provably.fast/), prepared at the user’s request. All twenty primary URLs were retrieved successfully, hashed, read at the stated scope and checked against the 74 existing source records captured from the terminal source-feed page. No exact normalized URL duplicates were found; title/work-level review also excluded existing references. This is a point-in-time duplicate check, not a guarantee about later submissions.

All twenty catalog references are now admitted as SOURCE records in Challenge 2. The full Agner tables PDF was separately rejected for snapshot size; entry 10 snapshots the author index/metadata page, and the rejected PDF receipt is retained rather than presented as an admitted PDF.

Each card separates what the source actually covers from our proposed Stwo use. It does not claim a measured speedup, a completed implementation or a paper-wide formal audit. No prover source, build, proof input, parameters or benchmark changed in this source-collection task. The actual submission/admission receipts live in [catalog.json](catalog.json); proposing is distinct from admission, and admission is distinct from an evaluator verdict or performance points.

## Use these first

- **Correctness:** entries 4–5 expose different batch-inversion zero contracts. ark-ff filters zero entries; p3-field 0.4.3 explicitly documents a panic if an input is zero and describes independent cumulative-product chains. Neither permits changing Stwo’s contract. Resolve our zero-denominator fixture by comparing valid original/candidate behavior.
- **Critical path:** entry 14 (Coz) explains why optimizing a busy function may fail to accelerate a parallel proof. Use it to establish whether witness work is exposed before another production patch.
- **Compiled work:** entries 7–10 and 20 support compiler diagnostics, assembly and matched-x86 cost analysis. Keep the Mac kernel screen separate from Linux evaluator evidence.
- **Memory and transforms:** use entries 11–12 and 17–19 only after attribution; bound scratch, retained capacity and concurrent ownership before adding another subsystem.

Protocol references (1–3) document invariants. They do not authorize a protocol replacement or lower security settings in the byte-identical speed lane. Tools and allocator papers are references for offline analysis/design, not a plan to add crates, file/network access or a new runtime to the prover. Keep instrumented runs separate from acceptance timing.

## Catalog

| # | Source | Research question |
|---:|---|---|
| 1 | [Improving logarithmic derivative lookups using GKR](https://eprint.iacr.org/2023/1284) | Lookup correctness and cost |
| 2 | [Fast Reed-Solomon Interactive Oracle Proofs of Proximity](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ICALP.2018.14) | FRI invariants |
| 3 | [DEEP-FRI: Sampling Outside the Box Improves Soundness](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITCS.2020.5) | Quotient and opening invariants |
| 4 | [ark-ff batch inversion: serial and parallel implementation with zero filtering](https://docs.rs/ark-ff/latest/src/ark_ff/fields/mod.rs.html) | Batch-inversion zero semantics |
| 5 | [Plonky3 p3-field 0.4.3 batch multiplicative inversion implementation](https://docs.rs/crate/p3-field/0.4.3/source/src/batch_inverse.rs) | Batch-inversion dependency chains |
| 6 | [BLAKE2: simpler, smaller, fast as MD5](https://eprint.iacr.org/2013/322) | Merkle hashing implementation |
| 7 | [LLVM Auto-Vectorization: loop and SLP vectorizers, diagnostics and cost model](https://llvm.org/docs/Vectorizers.html) | Portable SIMD/compiler analysis |
| 8 | [LLVM optimization remarks: passed, missed and analysis records](https://llvm.org/docs/Remarks.html) | Inlining and compiler evidence |
| 9 | [uops.info: Characterizing Latency, Throughput, and Port Usage of Instructions on Intel Microarchitectures](https://arxiv.org/abs/1810.04610) | x86 instruction cost |
| 10 | [Agner Fog x86 optimization manuals: author index and instruction-table metadata](https://www.agner.org/optimize/) | SIMD dependency and shuffle analysis |
| 11 | [The Design and Implementation of FFTW3](https://www.fftw.org/~fftw/fftw-paper-ieee.pdf) | Transform locality and bounded planning |
| 12 | [SPIRAL: Code Generation for DSP Transforms](https://www.cs.cmu.edu/~mmv/papers/05SpiralIEEE.pdf) | Transform factorization and SIMD layout |
| 13 | [Rigorous Benchmarking in Reasonable Time](https://kar.kent.ac.uk/33611/) | Benchmark variance and confidence |
| 14 | [Coz: Finding Code that Counts with Causal Profiling](https://arxiv.org/abs/1608.03676) | Critical-path attribution |
| 15 | [STABILIZER: Statistically Sound Performance Evaluation](https://people.cs.umass.edu/~emery/pubs/stabilizer-asplos13.pdf) | Measurement bias |
| 16 | [perf-stat Linux manual: performance counters, repetition and counter scheduling](https://man7.org/linux/man-pages/man1/perf-stat.1.html) | Host-matched work attribution |
| 17 | [Valgrind DHAT manual: allocation lifetime, turnover, access and copy profiling](https://valgrind.org/docs/manual/dh-manual.html) | Allocation and copy attribution |
| 18 | [Linux kernel false-sharing guide: detection, field layout and mitigation tradeoffs](https://docs.kernel.org/kernel-hacking/false-sharing.html) | Writer/lookup contention |
| 19 | [Hoard: A Scalable Memory Allocator for Multithreaded Applications](https://people.cs.umass.edu/~emery/pubs/berger-asplos2000.pdf) | Pool scalability and memory blowup |
| 20 | [llvm-mca: LLVM Machine Code Analyzer](https://llvm.org/docs/CommandGuide/llvm-mca.html) | Assembly resource-pressure screening |

## 1. Improving logarithmic derivative lookups using GKR

Primary source: [https://eprint.iacr.org/2023/1284](https://eprint.iacr.org/2023/1284)

**Source covers:** Fractional-sumcheck construction and auxiliary-column tradeoffs.

**Stwo relevance and limits:** Primary construction behind fractional lookup sumchecks. Use it to audit which witness terms and denominators must be preserved while investigating paired lookup arithmetic in range_check_18.rs, range_check_20.rs and the constraint-framework LogUp generator. Protocol or column-count changes are not permitted in the byte-identical mainnet speed lane; this source informs invariants, not permission to change the proof.

**Concrete use:** Check numerator/denominator identities and pole assumptions before optimizing the existing witness implementation.

**Provably:** ADMITTED; record `rgr1_545e26f400815e6af13e6c021e1e55cd7c0805850de4c4a122ccbca0b753b124`; proposal `659d8d62-6113-4cf4-8db3-d9fd657691df`.

## 2. Fast Reed-Solomon Interactive Oracle Proofs of Proximity

Primary source: [https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ICALP.2018.14](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ICALP.2018.14)

**Source covers:** Original FRI construction and prover/verifier work model.

**Stwo relevance and limits:** Primary FRI protocol reference for auditing fold order and commitment/query dependencies in crates/stwo/src/prover/fri.rs. It helps distinguish implementation work that may be fused or streamed from transcript-visible work that must stay fixed. Its Reed-Solomon description is not a drop-in Circle STARK implementation; keep the challenge's rounds, queries, parameters and bytes unchanged.

**Concrete use:** Document fold/transcript invariants for any FRI allocation or scheduling experiment.

**Provably:** ADMITTED; record `rgr1_71b00f9f1b5e5be57a8e64aaa9afec420435a38bb5e997ffc13a89a6694848cc`; proposal `1c7d9c18-7904-4ae3-a16a-1d949ec6a63c`.

## 3. DEEP-FRI: Sampling Outside the Box Improves Soundness

Primary source: [https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITCS.2020.5](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ITCS.2020.5)

**Source covers:** Out-of-domain sampling and its soundness role.

**Stwo relevance and limits:** Explains the role of out-of-domain evaluations and quotient construction, directly relevant to pcs/quotient_ops.rs, sample evaluation and opening-plan reuse. Use it to preserve the exact sample set, denominators and transcript order while removing repeated computation. It is a soundness reference, not evidence that a new cache or arithmetic variant is fast, and does not authorize lowering security parameters.

**Concrete use:** Use as an invariant checklist for denominator reuse and sampled opening changes.

**Provably:** ADMITTED; record `rgr1_1dee379beb0ba95f3e3079942e3cf5ddce4caa02e7a755effbc544e776090e63`; proposal `61a6feff-8766-4e0c-80e8-a2cdb2404447`.

## 4. ark-ff batch inversion: serial and parallel implementation with zero filtering

Primary source: [https://docs.rs/ark-ff/latest/src/ark_ff/fields/mod.rs.html](https://docs.rs/ark-ff/latest/src/ark_ff/fields/mod.rs.html)

**Source covers:** Concrete serial prefix-product inversion and parallel chunking implementation.

**Stwo relevance and limits:** A primary reference implementation for comparing batch-inversion zero conventions with Stwo's packed inverse path. Our constructed zero-denominator trace test panicked before the original-side comparison; this source helps formulate tests that distinguish zero filtering from an inverse-that-requires-nonzero contract. Do not import arkworks dependencies or silently change Stwo's zero behavior; the exact local original/candidate behavior must be measured separately.

**Concrete use:** Compare explicit zero contracts and add valid differential cases without weakening checks.

**Provably:** ADMITTED; record `rgr1_a752fa3a3e1f6c464ae9a80499fd768be9ed66dfbc061260d71783b3620a9e7f`; proposal `6cf44ead-9ae4-4bbb-aa61-c0bcf00d8730`.

## 5. Plonky3 p3-field 0.4.3 batch multiplicative inversion implementation

Primary source: [https://docs.rs/crate/p3-field/0.4.3/source/src/batch_inverse.rs](https://docs.rs/crate/p3-field/0.4.3/source/src/batch_inverse.rs)

**Source covers:** Batched finite-field inversion implementation and stated input contract.

**Stwo relevance and limits:** Versioned primary code for a second batch-inversion design, relevant to the dependency chains in constraint-framework/prover/logup.rs and SIMD denominator batches. Compare multiplication count, independent chains, scratch lifetime and zero-input preconditions against Stwo before proposing a bounded implementation experiment. Different field representations and zero contracts mean its performance or semantics cannot be transferred directly; no new crate is added.

**Concrete use:** Inspect batching shape and zero preconditions, then count actual work in the Stwo path.

**Provably:** ADMITTED; record `rgr1_cba39101365e8983b62ee53ecc3ba673f26f1d54b600263b978f2dcbba5dea4f`; proposal `53941d7c-2f3c-4a3c-85e8-cc6b5057056c`.

## 6. BLAKE2: simpler, smaller, fast as MD5

Primary source: [https://eprint.iacr.org/2013/322](https://eprint.iacr.org/2013/322)

**Source covers:** Original BLAKE2 design and implementation-oriented parallelism discussion.

**Stwo relevance and limits:** Complements the already-recorded RFC7693 specification with the designers' performance rationale. Relevant to simd/blake2s.rs and lifted Merkle hashing: optimize independent-message scheduling and vector layout while keeping the existing BLAKE2s parameter block, compression rounds, transcript and tree layout identical. BLAKE2sp/bp are different constructions and cannot replace the accepted hash; reported paper speeds are not Stwo measurements.

**Concrete use:** Identify implementation-only scheduling/layout candidates for differential compression tests.

**Provably:** ADMITTED; record `rgr1_9eb5f2c16000065bd23e362d8f1b32f72bdc6ba09917e4fa256f76f31d3457ed`; proposal `a5b65e1c-e8f9-44ca-8795-13e6668203d3`.

## 7. LLVM Auto-Vectorization: loop and SLP vectorizers, diagnostics and cost model

Primary source: [https://llvm.org/docs/Vectorizers.html](https://llvm.org/docs/Vectorizers.html)

**Source covers:** Loop versus SLP transformations and missed-vectorization diagnostics.

**Stwo relevance and limits:** Helps test whether paired-denominator or ordinary Packed witness-dot rewrites actually change generated vector work, rather than just source-level combine calls. Map diagnostics to range-check witness loops and SIMD field arithmetic using the pinned compiler; inspect assembly on the relevant host. Clang example flags are not automatically Rust flags, and architecture/compiler differences prohibit transferring the Mac kernel gain to Linux AVX2.

**Concrete use:** Compare actual vectorized loops and generated operations before a full prover build.

**Provably:** ADMITTED; record `rgr1_9578fdab3b5797a81126e64c60386e566037967126d815c5ae75fd11180ff84e`; proposal `05ca9086-0818-420f-a085-0c1d008abf43`.

## 8. LLVM optimization remarks: passed, missed and analysis records

Primary source: [https://llvm.org/docs/Remarks.html](https://llvm.org/docs/Remarks.html)

**Source covers:** Optimization diagnostic classes and serialized optimization records.

**Stwo relevance and limits:** Makes compiler-work claims auditable for cross-crate witness relation combines and verifier-fill helpers: distinguish successful inlining/CSE from missed transformations, then pair diagnostics with actual disassembly. This addresses the gap between a source call reduction and real instruction savings. Use only supported pinned-toolchain diagnostics for local analysis; do not change evaluator build flags or submit diagnostic artifacts as speed evidence.

**Concrete use:** Preserve attributable optimization remarks and compare the named hot functions.

**Provably:** ADMITTED; record `rgr1_6b802e99f7883f005cb35051d94468d2323e4214a8cc27b8994137961f46f025`; proposal `ad28fa94-c084-4f3e-9717-5fa24caf1634`.

## 9. uops.info: Characterizing Latency, Throughput, and Port Usage of Instructions on Intel Microarchitectures

Primary source: [https://arxiv.org/abs/1810.04610](https://arxiv.org/abs/1810.04610)

**Source covers:** Measured x86 instruction latency, throughput and operand/port modeling.

**Stwo relevance and limits:** Primary measurement methodology for pricing the AVX2 shuffles, integer multiplies and reduction chains in Stwo's M31/QM31, Blake2s and FFT kernels. It directly guards against extrapolating an ARM-only kernel screen to the evaluator. Select the actual CPU model and instruction forms; tables and microbenchmarks are advisory and cannot establish whole-proof speed, memory or proof correctness.

**Concrete use:** Cost the exact generated x86 instruction sequence after matched compilation.

**Provably:** ADMITTED; record `rgr1_6d92d60db533becab2432b91eb3941936ca31e157a412de4417cac1e8b26778a`; proposal `34aad218-61f9-4fe1-b352-1baeaf2eb29f`.

## 10. Agner Fog x86 optimization manuals: author index and instruction-table metadata

Primary source: [https://www.agner.org/optimize/](https://www.agner.org/optimize/)

**Source covers:** Author-maintained index describing instruction timing tables, CPU coverage, release metadata and measurement resources.

**Stwo relevance and limits:** The author index identifies the instruction-latency, throughput and micro-operation tables used to analyze M31/QM31 dependency chains and FFT/Blake2s shuffle costs. This graph reference snapshots only the index and metadata, not the full tables PDF, which exceeds the service snapshot limit. Consult the linked original tables for exact CPU/instruction forms. No table-derived speedup, new dependency or removal of the portable std::simd path is implied.

**Concrete use:** Separate latency-bound chains from throughput-bound independent lanes.

**Provably:** ADMITTED; record `rgr1_c3d3f3112c4c8bbc2350f4cb325d38b733874d555ba9fc798d44ee7ebcab0638`; proposal `eef5bcbc-2252-46d7-bb79-720e332a0c83`.

## 11. The Design and Implementation of FFTW3

Primary source: [https://www.fftw.org/~fftw/fftw-paper-ieee.pdf](https://www.fftw.org/~fftw/fftw-paper-ieee.pdf)

**Source covers:** Adaptive transform decomposition, codelets and hardware-aware planning.

**Stwo relevance and limits:** Primary design study for selecting cache-resident transform decompositions, directly relevant to simd/fft/rfft.rs, ifft.rs and circle.rs. Transfer the practice of comparing bounded decomposition choices, not floating-point FFT formulas or FFTW dependency code, to the finite-field circle transform. Preserve exact coefficient/evaluation order and account for planning/setup inside the timed proof; no runtime autotuning win is assumed.

**Concrete use:** Design a small offline ablation of transform passes after profiling their whole-proof share.

**Provably:** ADMITTED; record `rgr1_e867e4b8f12a653a67733ebce37b2ba16c358d41051312324a258709355f8b4d`; proposal `d9107284-cf20-40dc-9c18-49909c2be308`.

## 12. SPIRAL: Code Generation for DSP Transforms

Primary source: [https://www.cs.cmu.edu/~mmv/papers/05SpiralIEEE.pdf](https://www.cs.cmu.edu/~mmv/papers/05SpiralIEEE.pdf)

**Source covers:** Algebraic transform descriptions and search over equivalent implementations.

**Stwo relevance and limits:** Informs how to enumerate a bounded family of equivalent radix/layer and lane-layout kernels for Stwo's circle FFT rather than guessing a larger SIMD rewrite. Its DSP formulas are not automatically valid over the circle/M31 domain; derive the exact finite-field identity and run portable differential tests. Use offline design only, with no generated compile inputs or new packages in the challenge patch, and measure the full proof.

**Concrete use:** Compare a few proven equivalent layer groupings with explicit scratch and shuffle counts.

**Provably:** ADMITTED; record `rgr1_183534dc9d3ed0a7afb8ebec9a707d56b9080ec25bb1cc85c24e334764cd31c3`; proposal `9572acbf-9187-4386-9916-0df971adcef6`.

## 13. Rigorous Benchmarking in Reasonable Time

Primary source: [https://kar.kent.ac.uk/33611/](https://kar.kent.ac.uk/33611/)

**Source covers:** Performance comparison methodology with multiple sources of variation.

**Stwo relevance and limits:** Directly addresses our noisy alternating whole-proof screen whose interval crossed parity. Use the paper to distinguish within-run noise from build/run-level variability and to report effect-size intervals and experimental units honestly. It supplements evidence interpretation, not the evaluator's prescribed estimator, fresh statements, run order or acceptance threshold. No extra repetitions should be used to select only favorable pairs.

**Concrete use:** Predeclare the comparison and preserve complete ordered measurements and intervals.

**Provably:** ADMITTED; record `rgr1_38d572d90c1f4bd9115cf4b4cf3f954db05dd48aa8b1eec0b1c4065de5910504`; proposal `71dfc359-821a-4d91-a12a-76b8dd7e0728`.

## 14. Coz: Finding Code that Counts with Causal Profiling

Primary source: [https://arxiv.org/abs/1608.03676](https://arxiv.org/abs/1608.03676)

**Source covers:** Causal profiling distinguishes time consumption from achievable program speedup.

**Stwo relevance and limits:** Our quotient join/lift timers covered overlapping work without identifying removable critical-path cost. This primary paper explains why a long-running function is not necessarily a profitable optimization target in a parallel prover. Apply its attribution questions to Cairo interaction and verifier-fill overlap; any tool experiment must remain outside the submitted patch and its perturbed timings are not acceptance measurements. No new library or runtime is added.

**Concrete use:** Establish whether accelerating the selected work can shorten whole-proof completion.

**Provably:** ADMITTED; record `rgr1_441f035789559aa77fc923ee308cb33ca50d90ee9a35703590fb6ff4b0a71558`; proposal `7a629f74-37d8-46db-95a3-3ad5d5647fdb`.

## 15. STABILIZER: Statistically Sound Performance Evaluation

Primary source: [https://people.cs.umass.edu/~emery/pubs/stabilizer-asplos13.pdf](https://people.cs.umass.edu/~emery/pubs/stabilizer-asplos13.pdf)

**Source covers:** Memory-layout bias and statistical evaluation of performance changes.

**Stwo relevance and limits:** Explains why layout changes and one host's microbenchmark can make a small speed claim unreliable, pertinent to our sub-percent/noisy local screens. Record binary/source/build identity and distinguish kernel results from whole-proof evidence. Do not introduce Stabilizer instrumentation, randomized allocators or layout changes into the evaluator build; use the paper as a warning about inference, retaining the unchanged challenge harness and all pairs.

**Concrete use:** Audit layout/build differences before attributing a small timing difference to the mechanism.

**Provably:** ADMITTED; record `rgr1_0d20da145e5ef8e0c915274d3b96dad77f3191f3c5953067046eddd3e1c0de1e`; proposal `7fa4abe3-9c58-497b-8e91-55b9aa55cd3c`.

## 16. perf-stat Linux manual: performance counters, repetition and counter scheduling

Primary source: [https://man7.org/linux/man-pages/man1/perf-stat.1.html](https://man7.org/linux/man-pages/man1/perf-stat.1.html)

**Source covers:** Counter collection, repeat/interval options and reporting details.

**Stwo relevance and limits:** Supports diagnosing whether a Stwo whole-proof change removes instructions, reduces cache stalls or merely changes CPU occupancy on an available Linux x86 host. Match the evaluator's separate untimed-counter-run concept: preserve native binary identity, check event availability/multiplexing and keep counted runs separate from timing pairs. Counter improvements are advisory; exact proof checks, timing and peak-memory gates still decide.

**Concrete use:** Collect a bounded same-host instruction/cycle/cache comparison outside timed pairs.

**Provably:** ADMITTED; record `rgr1_bcb12f275064efb296b799f38ed06e52a7239ee935577588cabedc2185c909c2`; proposal `70c1e94b-b6b2-4397-b409-0c3ec33c495b`.

## 17. Valgrind DHAT manual: allocation lifetime, turnover, access and copy profiling

Primary source: [https://valgrind.org/docs/manual/dh-manual.html](https://valgrind.org/docs/manual/dh-manual.html)

**Source covers:** Heap lifetime/access diagnostics and separate copy profiling.

**Stwo relevance and limits:** Concrete method for testing whether proposed target/scatter metadata caches remove meaningful allocation or copies, and whether pooled trace scratch is retained after its last use. DHAT is an advisory profiling run, not the evaluator's wall time or peak-memory metric; custom mmap-backed allocations and SIMD support need coverage validation. Do not add instrumentation/dependencies to the submitted prover or treat instrumented memory as the acceptance peak.

**Concrete use:** Attribute avoidable allocation/copy work before retaining another cache.

**Provably:** ADMITTED; record `rgr1_02c454b5c7ead19679fabcca857b4a80cc960b8c858a2735dc1bc42c96998aca`; proposal `5887417b-489f-47a0-a7a3-f61594f3a497`.

## 18. Linux kernel false-sharing guide: detection, field layout and mitigation tradeoffs

Primary source: [https://docs.kernel.org/kernel-hacking/false-sharing.html](https://docs.kernel.org/kernel-hacking/false-sharing.html)

**Source covers:** False-sharing mechanism, perf-c2c/pahole diagnosis and mitigation costs.

**Stwo relevance and limits:** Relevant to Cairo writer overlap, shared lookup-state atomics and global buffer-pool bookkeeping. The guide provides a way to test whether cache-line contention limits useful parallel work before adding padding, sharding or scheduling changes. It also warns that mitigation increases space; any Stwo change must preserve lookup dependencies, deterministic output and the frontier memory cap. No current contention hotspot or speedup is established by this source.

**Concrete use:** Locate actual shared write/read cache lines before changing layout or task ownership.

**Provably:** ADMITTED; record `rgr1_fde43331e0d50722c13ced86a8ddc3173567ca02cf9a73ccb7dd202254fadd69`; proposal `e4d56059-3477-4e25-a45d-990d86067537`.

## 19. Hoard: A Scalable Memory Allocator for Multithreaded Applications

Primary source: [https://people.cs.umass.edu/~emery/pubs/berger-asplos2000.pdf](https://people.cs.umass.edu/~emery/pubs/berger-asplos2000.pdf)

**Source covers:** Allocator false sharing, ownership and memory-efficiency analysis.

**Stwo relevance and limits:** Primary design reference for evaluating Stwo's existing global column/large-block pools under Rayon: contention reduction must not multiply retained memory across workers or producer/consumer paths. Use its memory-blowup analysis as a question for pool-sharding proposals, not as a plan to link Hoard or change dependencies. Actual live/capacity accounting and the whole-proof 1.01x memory guard are required; Hoard's measurements do not transfer.

**Concrete use:** Bound total retained/free blocks across concurrent tasks before any pool redesign.

**Provably:** ADMITTED; record `rgr1_f8840fe0acc065746e0c6e3d741015da4007d0b1c40892e838859b5d9537b4b8`; proposal `6a63311b-ab68-492c-97fc-ea574e94c0f1`.

## 20. llvm-mca: LLVM Machine Code Analyzer

Primary source: [https://llvm.org/docs/CommandGuide/llvm-mca.html](https://llvm.org/docs/CommandGuide/llvm-mca.html)

**Source covers:** Processor-model analysis of assembly throughput, dependencies and resource pressure.

**Stwo relevance and limits:** A bounded offline gate for comparing original versus proposed M31/QM31 or FFT assembly before another costly full prover build. Analyze matched-target instruction sequences, especially multiply/reduction chains and shuffle pressure. LLVM scheduling models omit important real cache/OS/parallel effects, so the result is not a performance verdict and cannot substitute for exact/verified fresh whole-proof pairs or peak-memory measurement. No tool code enters the candidate.

**Concrete use:** Reject rewrites with no modeled work reduction, then verify promising ones on the actual host.

**Provably:** ADMITTED; record `rgr1_f85bc5edac4b93addc3e8d52ecca7a67cf9a65eabcc2c867a908bd0dc3b11cf5`; proposal `ca289d7d-8666-465d-ac1b-d88434621920`.

## Provenance and publication

The source-cache bodies and PDFs remain outside the public repository. Only original relevance notes, primary links, retrieval hashes and submission receipts are shared. The [JSON catalog](catalog.json) binds each receipt to its exact URL and records the source bytes observed on November 18; mutable latest/manual pages may change later. A successful fetch verifies reachability and the recorded bytes, not the truth of every source claim.

The existing [Stwo workshop thread](https://stwo.provably.fast/threads/bt1_80663a8c1d05345a9ed49ac0) is the publication destination for the bounded index. See publication.json when posted. This collection earns no claimed performance points; the next speed candidate still needs correct, byte-identical verified proofs, memory within the frontier limit and a qualifying whole-proof time result.

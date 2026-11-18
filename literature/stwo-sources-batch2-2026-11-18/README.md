# Twenty-one additional Stwo sources — batch 2, 2026-11-18

This batch contains 21 distinct primary references, checked against all 94 preexisting Challenge 2 SOURCE records, including the [previous twenty](../stwo-sources-2026-11-18/README.md). The complete pre-submission source feed had no next page; its 94 IDs were reconciled against retained full records and the previous catalog. Both URL/title checks and work-level review excluded duplicates and alternate mirrors.

Current receipt summary: 21 ADMITTED. Exact proposal IDs, decisions and admitted record IDs are in [catalog.json](catalog.json). Submission, admission, local correctness and evaluator performance are separate evidence classes. No performance points are claimed.

All 21 reference identities were verified: 19 through retrieved primary URLs and two through registered DOI metadata. Hashes identify the actual retrieved page, document or metadata response, not necessarily a DOI landing page. Review scope is bounded: primary abstracts/manual sections and relevant passages were read; the scanned transpose paper’s first page was rendered and visually inspected. The Merkle author preprint host failed TLS verification; certificate validation was not disabled. The submitted reference instead uses the verified publisher abstract/metadata page and registered DOI. Roofline is cited through its institutional metadata page, with its linked paper consulted separately. No third-party PDFs, page images, proof inputs, binaries or credentials are republished.

## What to use first

- **Correctness fixtures:** entries 16–20 help define synchronization, initialization and valid generated input contracts before interpreting a panic or a cache-hit result. They do not assert a discovered vulnerability or a passed test.
- **Actual work:** entries 1–4 and 21 separate plausible compute/bandwidth/cache limits from measured critical-path work. No bottleneck follows from a reference alone.
- **Probe/build integrity:** entries 10–15 address optimization barriers, source-to-binary attribution, compiler settings, loop dependencies and alias analysis. Preserve the pinned evaluator settings and original controls.
- **Algorithmic context:** entries 5–9 support GKR, traversal, field and locality reasoning. They do not authorize a replacement protocol, changed transcript or lower security parameters.

This task collected and published sources only. It performed no candidate-source edit, build, test, proof run or infrastructure provisioning. Tool references describe optional separate diagnostics, not new packages or instrumentation in the challenge patch. The denominator-delta correctness gate remains incomplete; no source admission clears it.

## Index

| # | Primary reference | Stwo question |
|---:|---|---|
| 1 | [Roofline: An Insightful Visual Performance Model for Floating-Point Programs and Multicore Architectures](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2008/EECS-2008-134.html) | Compute versus bandwidth attribution |
| 2 | [STREAM Benchmark Reference Information: sustainable bandwidth and run rules](https://www.cs.virginia.edu/stream/ref.html) | Host memory-bandwidth calibration |
| 3 | [A Top-Down method for performance analysis and counters architecture](https://doi.org/10.1109/ispass.2014.6844459) | Dominant CPU bottleneck classification |
| 4 | [Cache-Efficient Matrix Transposition](https://doi.org/10.1109/hpca.2000.824350) | FFT and commitment data-layout locality |
| 5 | [Time-Optimal Interactive Proofs for Circuit Evaluation](https://arxiv.org/abs/1304.3812) | Structured GKR prover work |
| 6 | [Libra: Succinct Zero-Knowledge Proofs with Optimal Prover Computation](https://www.cs.yale.edu/homes/cpap/published/libra-crypto19.pdf) | General-circuit sumcheck organization |
| 7 | [Merkle Tree Traversal in Log Space and Time](https://link.springer.com/chapter/10.1007/978-3-540-24676-3_32) | Authentication-path memory and recomputation |
| 8 | [Handbook of Applied Cryptography, Chapter 14: Efficient Implementation](https://cacr.uwaterloo.ca/hac/about/chap14.pdf) | Modular arithmetic bounds and cost accounting |
| 9 | [What Every Programmer Should Know About Memory](https://www.akkadia.org/drepper/cpumemory.pdf) | Cache, TLB and memory-access patterns |
| 10 | [Google Benchmark User Guide: preventing optimization and interpreting benchmark runs](https://github.com/google/benchmark/blob/main/docs/user_guide.md) | Standalone kernel experiment design |
| 11 | [Rust std::hint::black_box: best-effort optimization barrier and its limits](https://doc.rust-lang.org/std/hint/fn.black_box.html) | Rust kernel benchmark validity |
| 12 | [Cargo Build Cache: target directories, artifacts and dependency tracking](https://doc.rust-lang.org/cargo/reference/build-cache.html) | Build freshness and artifact attribution |
| 13 | [rustc Codegen Options: codegen-units, optimization, LTO and target CPU](https://doc.rust-lang.org/rustc/codegen-options/index.html) | Comparable compiler settings |
| 14 | [LLVM Loop Fusion: dependence requirements, safety and profitability](https://llvm.org/docs/LoopFusion.html) | Removing repeated row passes safely |
| 15 | [LLVM Alias Analysis Infrastructure: alias and ModRef dependence queries](https://llvm.org/docs/AliasAnalysis.html) | Memory-dependence reasoning |
| 16 | [Rustonomicon Atomics: ordering, synchronization and relaxed operations](https://doc.rust-lang.org/nomicon/atomics.html) | Lookup-state concurrency correctness |
| 17 | [Rust MaybeUninit: initialization, validity and safe construction invariants](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html) | Uninitialized trace-buffer safety |
| 18 | [Miri: Rust interpreter for detecting undefined behavior and its limitations](https://github.com/rust-lang/miri) | Portable helper correctness diagnostics |
| 19 | [Rust Fuzz Book: Structure-Aware Fuzzing](https://rust-fuzz.github.io/book/cargo-fuzz/structure-aware-fuzzing.html) | Adversarial cache and parser inputs |
| 20 | [Proptest Strategy Basics: generated inputs and shrinking](https://proptest-rs.github.io/proptest/proptest/tutorial/strategy-basics.html) | Algebraic and boundary-case test design |
| 21 | [Valgrind Cachegrind manual: instruction tracing, differencing and simulation limits](https://valgrind.org/docs/manual/cg-manual.html) | Attributable instruction-work comparison |

## 1. Roofline: An Insightful Visual Performance Model for Floating-Point Programs and Multicore Architectures

Primary source: [https://www2.eecs.berkeley.edu/Pubs/TechRpts/2008/EECS-2008-134.html](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2008/EECS-2008-134.html)

**Source covers:** Original operational-intensity and compute/bandwidth ceiling model.

**Stwo relevance and limits:** Helps classify Stwo's FFT, coefficient fold and Merkle passes before choosing arithmetic versus memory-traffic optimizations. The paper models floating-point kernels; finite-field operation costs and actual bytes moved must be measured separately, so its numerical ceilings do not transfer to M31/QM31. Apply only as a diagnostic framework on the actual host, not as proof that the current prover is bandwidth-bound or that a kernel gain shortens the complete proof.

**Concrete use:** Estimate actual bytes and field work per profiled pass before selecting a patch.

**Receipt:** ADMITTED; proposal `d8a45762-ea18-4341-8ab3-463992e371fb`; record `rgr1_4c7e1fca0780cdf51c90c91f614649683232efdca9d46dd128e60576154c4d3e`.

## 2. STREAM Benchmark Reference Information: sustainable bandwidth and run rules

Primary source: [https://www.cs.virginia.edu/stream/ref.html](https://www.cs.virginia.edu/stream/ref.html)

**Source covers:** Author-maintained bandwidth benchmark definition, byte accounting and run rules.

**Stwo relevance and limits:** Provides a way to calibrate sustainable memory bandwidth on an existing matched Linux host before interpreting large Stwo column sweeps. Array size, cache residency and byte-count conventions matter; STREAM numbers are not Stwo throughput or a process peak-memory metric. Do not run a calibration concurrently with proof pairs, provision new infrastructure, add a prover dependency, or substitute this benchmark for the unchanged fresh-input proof harness.

**Concrete use:** Use a separate host calibration to test the plausibility of a bandwidth ceiling.

**Receipt:** ADMITTED; proposal `677e891b-8c1d-47b8-8779-39fef0f123b5`; record `rgr1_b51a1e5fd83d2d9894c0e1cdf235d73b5b2bf7874b23ed8a1369325ba124f9d7`.

## 3. A Top-Down method for performance analysis and counters architecture

Primary source: [https://doi.org/10.1109/ispass.2014.6844459](https://doi.org/10.1109/ispass.2014.6844459)

**Source covers:** Original structured performance-counter methodology; DOI identity and author publication listing verified.

**Stwo relevance and limits:** Primary paper behind Top-Down analysis, relevant to classifying front-end, speculation and back-end limits in a profiled Stwo hot phase. This reference uses registered DOI metadata and the author publication listing; it is not a claim of a complete paper audit or a current Stwo bottleneck. Events and formulas depend on the actual processor, so an ARM kernel result cannot establish Intel categories. Keep diagnostic counter runs separate from proof acceptance timing and preserve the pinned build.

**Concrete use:** Classify the exposed hot phase with supported same-host counters before a rewrite.

**Receipt:** ADMITTED; proposal `12df0dcd-6de6-423b-92b1-6a52489e123c`; record `rgr1_b8510ea8c1e359b684d58aa46aa30629136a3c34b5b9ad7399427a25e6c5f53e`.

## 4. Cache-Efficient Matrix Transposition

Primary source: [https://doi.org/10.1109/hpca.2000.824350](https://doi.org/10.1109/hpca.2000.824350)

**Source covers:** Primary study of transpose algorithms under cache organization and associativity.

**Stwo relevance and limits:** Relevant to coefficient-layout transposes, bit-reversal-related memory walks and Blake2s packing in Stwo. It motivates testing bounded tiles against actual cache conflicts rather than assuming a theoretically optimal transpose wins. Preserve the exact row/column/lane permutation and both portable and SIMD outputs; count temporary buffers and concurrent tiles. The paper's matrix workloads and machines do not establish a current mainnet gain.

**Concrete use:** Compare exact permutation outputs and memory traffic for one profiled transpose site.

**Receipt:** ADMITTED; proposal `875e62f3-a820-407a-978f-53ebc481b04b`; record `rgr1_cfd779e6a45817e441efa5bd55bf10614709494db59aa8dfb7b263fea1aae690`.

## 5. Time-Optimal Interactive Proofs for Circuit Evaluation

Primary source: [https://arxiv.org/abs/1304.3812](https://arxiv.org/abs/1304.3812)

**Source covers:** Linear-time GKR refinements for sufficiently regular circuit wiring.

**Stwo relevance and limits:** Primary algorithmic background for examining repeated wiring/sumcheck work in Stwo's existing GKR prover. First establish whether the local circuit and round computations have the required regularity; the paper's asymptotic or reported speedups are not transferred. Only algebraically equivalent work removal within the existing transcript could fit the byte-identical speed lane; do not replace its protocol, verifier or security settings.

**Concrete use:** Map repeated GKR calculations to the actual round schedule before proposing reuse.

**Receipt:** ADMITTED; proposal `f8b0f864-e7d1-4b90-817d-23d9a5123e31`; record `rgr1_07dc61a4ff0751ed7797580bfde74b7069578bef1b19c37df014a3243a7753d5`.

## 6. Libra: Succinct Zero-Knowledge Proofs with Optimal Prover Computation

Primary source: [https://www.cs.yale.edu/homes/cpap/published/libra-crypto19.pdf](https://www.cs.yale.edu/homes/cpap/published/libra-crypto19.pdf)

**Source covers:** Primary linear-time GKR prover and zero-knowledge construction.

**Stwo relevance and limits:** Offers a different general-circuit work organization to study alongside the regular-wiring GKR reference when auditing Stwo sumcheck passes. This is algorithmic context only: Libra uses a different proof system and setup, so its commitments, masking or protocol cannot replace the accepted Stwo proof. Any transferable calculation must preserve every existing challenge, round polynomial and verifier check, with direct differential and whole-proof validation.

**Concrete use:** Identify whether a local repeated sumcheck pass admits an identical-output calculation.

**Receipt:** ADMITTED; proposal `95e85955-d139-42cc-9c98-3268a17e8570`; record `rgr1_88f19a43421448405a802f35a2a6193471cb1ff9cbc5491651a4b0f0a1b4bdc1`.

## 7. Merkle Tree Traversal in Log Space and Time

Primary source: [https://link.springer.com/chapter/10.1007/978-3-540-24676-3_32](https://link.springer.com/chapter/10.1007/978-3-540-24676-3_32)

**Source covers:** Primary Merkle traversal time/space tradeoff algorithm.

**Stwo relevance and limits:** A reference for reasoning about retained nodes versus recomputation in Stwo Merkle decommitment. The paper's traversal workload differs from the prover's sampled multiquery openings; analyze the actual query order and shared nodes before adapting any schedule. Preserve tree shape, hashes, authentication paths and serialized order. No logarithmic-memory claim for the complete prover follows, and extra recomputation may fail the time guard.

**Concrete use:** Price a bounded node-lifetime plan against the actual multiquery opening workload.

**Receipt:** ADMITTED; proposal `53d182f3-f5fd-4c60-a95e-fbddae4e6501`; record `rgr1_02d6a6819a9b30e0cc96ae757c1a80a895af8214b065e0ac79dba8f52c6ba2a0`.

## 8. Handbook of Applied Cryptography, Chapter 14: Efficient Implementation

Primary source: [https://cacr.uwaterloo.ca/hac/about/chap14.pdf](https://cacr.uwaterloo.ca/hac/about/chap14.pdf)

**Source covers:** Author-hosted modular arithmetic, reduction, inversion and exponentiation algorithms.

**Stwo relevance and limits:** Background for checking intermediate-width bounds and operation counts in proposed M31/QM31 deferred reductions and inverses. General multiple-precision or Montgomery algorithms are not automatically appropriate for the Mersenne31 representation. Derive the exact lane bounds and preserve raw/canonical zero conventions; this chapter provides no Stwo speed evidence. Reference the original URL only; do not republish its chapter text or add an arithmetic package to the patch.

**Concrete use:** Write explicit range and representation invariants before any reduction rewrite.

**Receipt:** ADMITTED; proposal `f09371e6-2186-4340-b713-53d36da4e069`; record `rgr1_5ad6522d99f8d226c9e155702e70fb87b2b709da0f9a558a1bbeb042c92be3d1`.

## 9. What Every Programmer Should Know About Memory

Primary source: [https://www.akkadia.org/drepper/cpumemory.pdf](https://www.akkadia.org/drepper/cpumemory.pdf)

**Source covers:** Author's explanation of memory hierarchy, locality and software access patterns.

**Stwo relevance and limits:** Informs hypotheses about strided column reads, retained stripes, first touch and scratch-buffer reuse in the Stwo proof. Its historical hardware timings are not current evaluator constants; confirm cache/TLB behavior on the matched host. A local memory model is not a measured critical path or peak-memory bound. Share only a citation and original notes, not the copyrighted PDF, and retain the existing proof and memory gates.

**Concrete use:** Connect an actual memory-walk profile to one bounded locality experiment.

**Receipt:** ADMITTED; proposal `ef3801a3-3437-47aa-97ac-30f478079cc6`; record `rgr1_ab2b79e7a3b162f4a8c98fd5e4849016a0a68d0196666116622022034931207c`.

## 10. Google Benchmark User Guide: preventing optimization and interpreting benchmark runs

Primary source: [https://github.com/google/benchmark/blob/main/docs/user_guide.md](https://github.com/google/benchmark/blob/main/docs/user_guide.md)

**Source covers:** Benchmark fixtures, optimization barriers, repetitions and timing controls.

**Stwo relevance and limits:** Useful for reviewing our standalone arithmetic probes for constant folding, invariant hoisting, setup inclusion and checksum overhead. The guide's C++ barriers are not identical to Rust black_box semantics, and a microbenchmark never substitutes for full Stwo proof timings. Adopt experimental questions only: do not add Google Benchmark or alter the evaluator harness/build. Preserve the complete paired result rather than selecting favorable iterations.

**Concrete use:** Audit the probe so baseline and candidate consume equivalent inputs and timed work.

**Receipt:** ADMITTED; proposal `9307e46c-7ab5-45f9-9459-6120ca14d8fd`; record `rgr1_8d4f18b7165d109a3b3db19f00ea8cc9a56721848897748f3f09b50edc29248d`.

## 11. Rust std::hint::black_box: best-effort optimization barrier and its limits

Primary source: [https://doc.rust-lang.org/std/hint/fn.black_box.html](https://doc.rust-lang.org/std/hint/fn.black_box.html)

**Source covers:** Official optimization-barrier contract, examples and limitations.

**Stwo relevance and limits:** Directly relevant to our paired-denominator and witness-dot probes, which use black_box. It helps distinguish prevented dead-code elimination from an actual guarantee that the compiler preserves an intended instruction sequence; inspect generated code as well. It is not a constant-time or security primitive. Keep pinned compiler/build identities, and do not use this reference to claim the synthetic loop represents whole-proof scheduling or a speed win.

**Concrete use:** Check barrier placement and assembly when interpreting an isolated arithmetic saving.

**Receipt:** ADMITTED; proposal `630a6b8a-b483-4def-afab-b2482f2c4fef`; record `rgr1_6fbdc735839d61ed49d5081913b370adbe71d865ec191249d1375e5ac5d4a1b0`.

## 12. Cargo Build Cache: target directories, artifacts and dependency tracking

Primary source: [https://doc.rust-lang.org/cargo/reference/build-cache.html](https://doc.rust-lang.org/cargo/reference/build-cache.html)

**Source covers:** Official Cargo build-cache structure and tracking behavior.

**Stwo relevance and limits:** Addresses this project's prior stale-binary and incompatible cached-library problems. Use the documented target/dependency boundaries when attributing a benchmark executable to source, toolchain, flags and dependencies; do not assume a filename or timestamp proves compatibility. It does not guarantee our E0463 cause or a specific cache layout across versions. No cache clearing, artifact reuse, build or candidate mutation is performed in this source task.

**Concrete use:** Bind each future measurement to a successful build and explicit binary/source hashes.

**Receipt:** ADMITTED; proposal `6a0b56b2-ce82-4745-917e-be57a18cacc8`; record `rgr1_085802e6050b2b732e15fd16b64ec88374ca06284af02445845cc916ed7ebf1c`.

## 13. rustc Codegen Options: codegen-units, optimization, LTO and target CPU

Primary source: [https://doc.rust-lang.org/rustc/codegen-options/index.html](https://doc.rust-lang.org/rustc/codegen-options/index.html)

**Source covers:** Official code-generation option semantics and exact-compiler help guidance.

**Stwo relevance and limits:** Supports checking that baseline and candidate use the pinned nightly and the same native flags, especially codegen-units, optimization and target CPU. Current online documentation may describe a newer compiler; verify options against nightly-2026-01-15. This is provenance guidance, not authorization to change evaluator flags, add target_feature attributes or claim LTO helps. Separate compiler-setting experiments from source-mechanism evidence.

**Concrete use:** Record and compare the actual compiler invocations for both proof binaries.

**Receipt:** ADMITTED; proposal `4549c72b-75dc-49c1-88b0-9d706ee551b5`; record `rgr1_a3c5ea8c3c94f1c89a2c99509f4e0865cb7431f76ed73dad4de824bcad03b578`.

## 14. LLVM Loop Fusion: dependence requirements, safety and profitability

Primary source: [https://llvm.org/docs/LoopFusion.html](https://llvm.org/docs/LoopFusion.html)

**Source covers:** Compiler loop-fusion legality and profitability conditions.

**Stwo relevance and limits:** Relevant when considering a fused Stwo witness row pass or adjacent field loops to reduce memory traffic. Establish iteration-space and dependence equivalence, preserve all lookup-state updates and exact output order, and account for register pressure or lost parallelism. The compiler document does not prove a particular manual fusion safe or fast; pinned LLVM support may differ. Do not force evaluator flags or weaken checks to obtain a fused loop.

**Concrete use:** List data dependencies and generated-code effects for one proposed row-pass fusion.

**Receipt:** ADMITTED; proposal `bb0aada7-24b9-465a-aaf8-44e5c8394324`; record `rgr1_f76fd96cd56dd13c43a5b7a917aa07fdb7a81c81c540c17e0653de191aca66ce`.

## 15. LLVM Alias Analysis Infrastructure: alias and ModRef dependence queries

Primary source: [https://llvm.org/docs/AliasAnalysis.html](https://llvm.org/docs/AliasAnalysis.html)

**Source covers:** Compiler alias and read/write-effect analysis concepts.

**Stwo relevance and limits:** Helps explain why apparently independent coefficient, target-plan or witness-buffer operations may resist hoisting/fusion. Use it to frame actual ownership/read-write questions and inspect the pinned compiler's output. Do not add false noalias assumptions, reinterpret mutable slices or use new unsafe code outside the allowed SIMD surface. An alias-analysis model does not prove application-level scheduling dependencies or a whole-proof gain.

**Concrete use:** Confirm real buffer independence before reordering or sharing work across loops.

**Receipt:** ADMITTED; proposal `37b1860c-09c7-4aec-a120-c5623670e88e`; record `rgr1_ec4ed2a797bb9be664adba154c0b5be550ffddeea57c93c0293128aa955b40da`.

## 16. Rustonomicon Atomics: ordering, synchronization and relaxed operations

Primary source: [https://doc.rust-lang.org/nomicon/atomics.html](https://doc.rust-lang.org/nomicon/atomics.html)

**Source covers:** Official Rust memory-ordering and synchronization guidance.

**Stwo relevance and limits:** Directly relevant to Cairo writer overlap and shared lookup-state counters. Distinguish commutative arithmetic updates from publication of non-atomic data; a relaxed counter alone does not establish completion or visibility. Preserve all lookup dependencies and deterministic commitment order, using only allowed Rayon execution. This reference neither validates the current code nor authorizes weaker ordering without a complete correctness argument and testing.

**Concrete use:** Document the happens-before edges required by any writer handoff experiment.

**Receipt:** ADMITTED; proposal `9d26ea19-8e51-410d-a165-47b81619188a`; record `rgr1_53f03b2159513396053ebee511e34dde14d386586ff0dcfc05a055e5ab1ea756`.

## 17. Rust MaybeUninit: initialization, validity and safe construction invariants

Primary source: [https://doc.rust-lang.org/std/mem/union.MaybeUninit.html](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html)

**Source covers:** Official rules for uninitialized storage and assume-init obligations.

**Stwo relevance and limits:** Relevant to Stwo column pools and LogupTraceGenerator::uninitialized: prove every subsequently read lane/element is initialized on every branch, including empty and fallback cases. The type does not make uninitialized reads safe, and zeroing is not valid for every Rust type. Online APIs may exceed the pinned toolchain. This is review guidance, not permission for new unsafe outside the challenge's allowed surface or edits to protected tests.

**Concrete use:** Audit write-before-read coverage for each proposed pooled-buffer path.

**Receipt:** ADMITTED; proposal `8e1a70a8-3395-4449-b8ce-de5b81879c2c`; record `rgr1_dbbd0aabd07f52b93f12fa6c4a6a4bb96e2ca702e5fafa8fa471f6d2e9499b53`.

## 18. Miri: Rust interpreter for detecting undefined behavior and its limitations

Primary source: [https://github.com/rust-lang/miri](https://github.com/rust-lang/miri)

**Source covers:** Primary tool documentation for dynamic undefined-behavior checks and unsupported cases.

**Stwo relevance and limits:** A possible offline diagnostic for small portable buffer/ownership helpers before performance testing, especially uninitialized reads and invalid aliasing. SIMD intrinsics, foreign calls and concurrency support must be checked; passing a reduced Miri test is not proof of native SIMD correctness or verifier soundness. No installation, test run, new dependency or evaluator instrumentation is part of this source collection.

**Concrete use:** Evaluate supported portable helper coverage while retaining native differential and proof tests.

**Receipt:** ADMITTED; proposal `72c10322-4433-42f3-a6e6-60434e2f272b`; record `rgr1_3c9543b120bfa9f1d6e28f5e871b766e8ae6b15f908a22fcf349ec52f7a360ee`.

## 19. Rust Fuzz Book: Structure-Aware Fuzzing

Primary source: [https://rust-fuzz.github.io/book/cargo-fuzz/structure-aware-fuzzing.html](https://rust-fuzz.github.io/book/cargo-fuzz/structure-aware-fuzzing.html)

**Source covers:** Primary guidance on generating structured inputs for coverage-guided testing.

**Stwo relevance and limits:** Useful for exercising target/scatter plans and preprocessing-cache fingerprints beyond a few valid examples. Generate mutations that reach the intended cache or lookup path, including unsampled changes, duplicates and boundary cases, then compare with the uncached/original implementation. This does not establish a cache bug or authorize accepting invalid proofs. Fuzz tooling stays outside the challenge patch; no new package, test-region edit or run is performed here.

**Concrete use:** Design valid structured mutations whose expected behavior is independently checked.

**Receipt:** ADMITTED; proposal `b57abc65-0009-4851-921c-37deb6c2ed51`; record `rgr1_751a4055c7da575682a2c1cc9aa2028ffb9283289ebdf30dd6be96623f9025be`.

## 20. Proptest Strategy Basics: generated inputs and shrinking

Primary source: [https://proptest-rs.github.io/proptest/proptest/tutorial/strategy-basics.html](https://proptest-rs.github.io/proptest/proptest/tutorial/strategy-basics.html)

**Source covers:** Primary property-testing strategy and shrinking concepts.

**Stwo relevance and limits:** Informs generation of raw zero/P field representations, zero/nonzero denominator cases, duplicated positions and empty/resident suffix boundaries for differential Stwo tests. State each operation's preconditions first, so an invalid fixture panic is not mistaken for a regression; preserve a minimal failing case. Apply the testing method without adding a new crate to the pinned package set, and retain independent originals rather than implementation-mirroring assertions.

**Concrete use:** Build a compact table of valid invariants and minimized counterexamples before another patch.

**Receipt:** ADMITTED; proposal `b0000ed2-cc96-4fd7-a1af-2ae80f88961e`; record `rgr1_dba65fb31bec6b8eb037398bfa1d89b6198a9bb7dd5cc7f4951f0d872c218b17`.

## 21. Valgrind Cachegrind manual: instruction tracing, differencing and simulation limits

Primary source: [https://valgrind.org/docs/manual/cg-manual.html](https://valgrind.org/docs/manual/cg-manual.html)

**Source covers:** Primary instruction-count, annotation/differencing and optional cache/branch simulation guidance.

**Stwo relevance and limits:** Complements DHAT's heap view with a bounded instruction-work comparison for suspected redundant Stwo loops. Validate tool coverage and use a controlled diagnostic workload; simulated caches do not establish real coherence, TLB, parallel scheduling or evaluator behavior. Instrumented wall time and memory are not acceptance metrics. Keep profiling tools outside the submitted code and follow with exact/verified native whole-proof timing and peak checks.

**Concrete use:** Compare attributed work counts to verify that a proposed optimization removes actual work.

**Receipt:** ADMITTED; proposal `c6159ac4-689b-4959-9e88-18901dc719c1`; record `rgr1_7049dbe7ec3d7c8905860252899a3b6cc31a733821bfe43a7c7f001eb2dd7372`.

## Publication and next action

The existing [Stwo workshop thread](https://stwo.provably.fast/threads/bt1_80663a8c1d05345a9ed49ac0) is the destination for one linked index, with receipt in publication.json. Catalog/source hashes identify the bytes observed, not a claim that every paper statement was independently proved; mutable documentation may change.

Next, apply the testing references to a valid original-versus-candidate zero-case comparison, then use attributable compiler/work evidence to select a material optimization. Any future patch still needs portable correctness, fresh alternating exact/independently verified whole-proof pairs, timing, memory and final policy checks. This source count is not a speed result or point award.

Publication verified: workshop reply `bp1_2f5ca4c6121842293b9b4f36` in the linked thread. Its immutable attachments preserve the earlier 19-admitted/2-proposed checkpoint at commit `734ab36`; this GitHub catalog now records all 21 as admitted. [Publication receipt](publication.json) binds both attachment hashes and the final status check.

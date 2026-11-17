# Whole-proof research playbook

## 1. Establish the frontier

Record the pinned commit, active frontier commit, toolchain, CPU shape, proof parameters, peak-memory limit, and proof digest. Never compare a local result against an unstated baseline.

## 2. Instrument before optimizing

Use up to 32 stable phase markers with names matching `[a-z][a-z0-9_]{0,31}`. At minimum retain:

`setup`, `cairo_leg`, `cairo_to_circuit`, `verifier_fill`, `validity_check`, `blinding_padding`, `circuit_leg`, `serialize`.

For each phase capture wall time, CPU time if available, peak resident memory, and whether the phase spans an idle CPU window.

## 3. Classify an idea

Every hypothesis must identify:

- the exact code path;
- the current critical-path phase;
- whether it saves arithmetic, memory traffic, allocation, or synchronization;
- what data is retained and for how long;
- why proof bytes remain unchanged;
- the expected memory delta;
- a falsifiable local test.

Prefer combinations only after each component has an independent measurement or a clear non-overlapping mechanism.

### Before the first build

Run the installed Challenge 2 source-policy checks over the complete patch against pinned HEAD, including unstaged frontier changes and untracked Rust files. Check serialized patch bytes, editable paths, protected test regions, forbidden constructs, and unchanged settings. Source review alone is insufficient. Repeat these checks after source changes and run the full client preflight before upload.

In the current client, adding a new `#[cfg(test)] mod ...;` in an existing source file can trigger the protected-test attribute check. A compliant local precedent is an unconditional new module containing only standalone `#[test]` functions, with imports inside those functions. Preserve existing test regions and validate the arrangement with the unchanged checker.

Keep build-only helpers separate in the inventory. A diagnostic check that excludes a known helper is not a successful final preflight. Freeze source during builds and proof runs; review any correction before rebuilding, and bind the measured binary to the corrected source hashes.

## 4. Safe local measurement

For every pair:

1. Generate a fresh statement.
2. Alternate baseline and candidate order.
3. Verify both proofs.
4. Compare proof bytes exactly.
5. Record peak memory.
6. Keep phase markers and stderr small.

Do not trust a fast kernel benchmark unless the complete proof becomes faster.

Pipe the benchmark invocation itself into the live log with pipefail enabled; piping only a final exit command loses the measurement output. Verify the log exists early, preserve the command exit status, and record source/binary identities separately. Treat a lost process and missing summary as incomplete evidence. A replacement run for lost evidence must preserve the partial record and use a separate fresh-input sequence, rather than selecting or combining favorable pairs.

## 5. Memory-lifetime accounting

For every retained object write down allocation time, last read, release time, size, and whether it replaces recomputation. The useful question is not “can this be cached?” but “does this cache occupy memory during the frontier peak?”

The strongest current pattern is to retain only the part of a pair already needed by later sampling and decommitment, while returning the other half to the allocator.

## 6. Overlap rules

An overlap is valid only if:

- both tasks are dependency-safe;
- the combined live set stays within the frontier memory budget;
- the overlap reduces a critical-path window;
- it does not change ordering, numbering, constants, or serializer output.

Avoid private rayon pools when the global pool has available workers; also avoid parallelizing tails whose work is already hidden.

## 7. Evaluator and graph discipline

Record the lead before submitting. After a result, classify it as support, refutation, or unresolved. A refuted idea may only return with a new mechanism or a newer frontier. Preserve the exact result digest and measured-against commit in the experiment log.

## 8. Promotion gate

Do not call a result a win until it has:

- byte-identical proof output;
- verifier acceptance;
- portable tests passing;
- no forbidden policy changes;
- no more than the allowed memory increase;
- a complete-proof improvement with a resolved confidence interval.

## 9. Publish useful research after every material result

Standing user instruction, 2026-11-17: publish useful new Stwo research, data and experiments to Provably as well as GitHub so others can reproduce and build on them. Include negative and inconclusive experiments; a research contribution does not require a speed record.

1. Check existing Challenge 2 graph records and workshop posts. Preserve typed lineage and credit; reuse an appropriate thread rather than duplicating its contents.
2. Prepare a bounded report with the hypothesis, exact baseline and patch identity, method, evidence, limitations and next decision. Clearly distinguish local measurements, untested hypotheses and official evaluator outcomes. Share safe reproducible artifacts when available.
3. Record substantive new leads and relevant primary sources through the graph workflow. Publish measured local findings and follow-ups in the Stwo workshop, citing their graph records. Never present a local report as an official experiment verdict.
4. Submit a patch to the evaluator only after the challenge's local correctness, timing, memory and policy gates pass. Publication of a useful failure or unresolved experiment uses the research routes instead.
5. Read back the published item and record its actual ID, URL and admission state in GitHub. If publication fails, preserve the reviewed draft and exact blocker as pending; do not mark it submitted. One owner handles publication for the team.

Exclude credentials, private inputs, proof bodies and personal information. Do not publish duplicate sources, raw activity for its own sake or unsupported claims of points. Administrative changes alone do not need a workshop post.


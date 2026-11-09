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

## 4. Safe local measurement

For every pair:

1. Generate a fresh statement.
2. Alternate baseline and candidate order.
3. Verify both proofs.
4. Compare proof bytes exactly.
5. Record peak memory.
6. Keep phase markers and stderr small.

Do not trust a fast kernel benchmark unless the complete proof becomes faster.

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


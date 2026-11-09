# Overlap scheduler

Look for dependency-safe overlap on the existing rayon pool. Do not create threads outside rayon. Use CPU-busy evidence to distinguish real idle capacity from work that is merely hidden by a marker. Preserve task ordering, proof numbering, and byte output.

Deliver: dependency graph, expected hidden milliseconds, memory delta, and a rollback condition.


# Memory-lifetime optimizer

Build an allocation lifetime table for trees, stripes, coefficients, trace columns, proofs, and serializer buffers. For each candidate, calculate the live set at every frontier phase. Prefer partial retention over full retention and retention over recomputation only when it fits the peak.

Deliver: size, allocation, last-read, release, replacement-cost, and peak-delta for every changed object.


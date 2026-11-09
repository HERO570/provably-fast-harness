#!/bin/zsh
set -euo pipefail

if [[ $# -ne 2 ]]; then
  print -u2 "usage: $0 /absolute/path/to/proving experiment-name"
  exit 2
fi

root="$1"
name="$2"
if [[ ! -d "$root/.git" ]]; then
  print -u2 "not a git repository: $root"
  exit 2
fi

print "Experiment: $name"
print "Repository: $root"
print "Before running, record the pinned and frontier commits, then use the challenge's measure.py."
print "Required evidence: fresh inputs, alternating order, proof verification, byte identity, peak memory, and phase medians."
print "This wrapper intentionally does not submit to the evaluator."


#!/bin/zsh
set -euo pipefail

if [[ $# -ne 1 ]]; then
  print -u2 "usage: $0 /absolute/path/to/proving"
  exit 2
fi

root="$1"
if [[ ! -d "$root" ]]; then
  print -u2 "proving directory not found: $root"
  exit 2
fi

print "Phase-marker checklist for $root"
print "Add eprintln!(\"pvfast-phase NAME\") at stable boundaries:"
print "setup cairo_leg cairo_to_circuit verifier_fill validity_check blinding_padding circuit_leg serialize"
print "Keep markers under 32 total and stderr below 64 KiB."


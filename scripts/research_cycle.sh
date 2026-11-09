#!/bin/zsh
set -euo pipefail

if [[ $# -ne 2 ]]; then
  print -u2 "usage: $0 /absolute/path/to/proving /absolute/path/to/pvfast-participant"
  exit 2
fi

root="$1"
client="$2"

[[ -d "$root/.git" ]] || { print -u2 "not a proving git tree: $root"; exit 2; }
[[ -x "$client/pvfast" ]] || { print -u2 "pvfast client not found: $client/pvfast"; exit 2; }

print "== Frontier identity =="
git -C "$root" rev-parse HEAD
git -C "$root" status --short
print "== Current frontier diff =="
git -C "$root" diff --stat 9df00f2249b61849be69cc14942742d0c1f81ad2
print "== Phase marker checklist =="
print "Run the client's measure.py with fresh inputs and alternating order."
print "Require: proof verification, byte identity, phase medians, CPU occupancy, and peak memory."
print "== Research gate =="
print "Do not edit until docs/ACTIVE_PLAN.md identifies one non-duplicative lead."
print "Do not submit until the lead has been recorded in the challenge graph."


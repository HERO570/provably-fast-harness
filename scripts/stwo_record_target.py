#!/usr/bin/env python3
"""Offline Stwo mainnet strict-record target calculator."""

import argparse
import json
import math
import sys
from pathlib import Path
from typing import Any, Optional

MAINNET_CAMPAIGN = "stwo-privacy-mainnet-whole-proof-v1"


class DataError(ValueError):
    """Input does not contain a valid mainnet record snapshot."""


def _finite_number(value: Any) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)


def _valid_frontier(value: Any) -> tuple[str, float]:
    if not isinstance(value, dict):
        raise DataError("mainnet frontier must be an object")
    commit = value.get("commit")
    ratio = value.get("ratio")
    if not isinstance(commit, str) or not commit.strip():
        raise DataError("mainnet frontier is missing a commit")
    if not _finite_number(ratio) or ratio <= 0:
        raise DataError("mainnet frontier ratio must be positive and finite")
    return commit, float(ratio)


def _rows(container: dict[str, Any], label: str) -> list[dict[str, Any]]:
    collected = []
    for key in ("results", "recent"):
        rows = container.get(key, [])
        if not isinstance(rows, list):
            raise DataError(f"{label}.{key} must be an array")
        for index, row in enumerate(rows):
            if not isinstance(row, dict):
                raise DataError(f"{label}.{key}[{index}] must be an object")
            collected.append(row)
    return collected


def calculate(data: Any) -> dict[str, Any]:
    if not isinstance(data, dict):
        raise DataError("input JSON root must be an object")

    top_level_matches = data.get("campaign_id") == MAINNET_CAMPAIGN
    matching_seasons: list[tuple[str, dict[str, Any]]] = []
    seasons = data.get("seasons", [])
    if not isinstance(seasons, list):
        raise DataError("seasons must be an array")
    for index, season in enumerate(seasons):
        if not isinstance(season, dict):
            raise DataError(f"seasons[{index}] must be an object")
        if season.get("campaign_id") == MAINNET_CAMPAIGN:
            matching_seasons.append((f"seasons[{index}]", season))

    # The engine computes board rows from the matching season when seasons
    # exist; the top-level copy is a summary and must not override that source.
    containers = matching_seasons or ([ ("top-level", data) ] if top_level_matches else [])

    if not containers:
        raise DataError(f"no campaign_id {MAINNET_CAMPAIGN!r} found")
    frontier_values = [_valid_frontier(container.get("frontier")) for _, container in containers]
    if any(frontier != frontier_values[0] for frontier in frontier_values[1:]):
        raise DataError("matching mainnet seasons have conflicting frontiers")
    frontier_commit, frontier_ratio = frontier_values[0]

    # Match the engine's Map semantics: last duplicate value wins, while an
    # overwrite keeps the digest's original insertion position.
    rows_by_digest: dict[str, dict[str, Any]] = {}
    for label, container in containers:
        for row in _rows(container, label):
            digest = row.get("result_digest")
            if not isinstance(digest, str) or not digest.strip():
                raise DataError(f"{label} row is missing a non-empty result_digest")
            rows_by_digest[digest] = row

    eligible: list[dict[str, Any]] = []
    for row in rows_by_digest.values():
        evaluated_at = row.get("evaluated_at")
        ratio = row.get("ratio")
        if (
            row.get("status") == "MEASURED_IMPROVEMENT"
            and _finite_number(evaluated_at)
            and _finite_number(ratio)
            and ratio > 0
        ):
            eligible.append(row)

    # Python's sort is stable, matching the engine's evaluated_at-only sort.
    eligible.sort(key=lambda row: float(row["evaluated_at"]))
    best_ratio = 1.0  # Official engine initializes the strict-record best at 1.
    best_row = None
    for row in eligible:
        ratio = float(row["ratio"])
        if ratio < best_ratio:
            best_ratio = ratio
            best_row = row

    candidate_frontier_ratio = best_ratio / frontier_ratio
    fractional_gain_needed = 1.0 - candidate_frontier_ratio
    author = best_row.get("github_login") if best_row else None
    known_author = isinstance(author, str) and bool(author.strip())
    return {
        "campaign_id": MAINNET_CAMPAIGN,
        "frontier": {"commit": frontier_commit, "ratio": frontier_ratio},
        "best_published_qualifying": {
            "ratio": best_ratio,
            "result_digest": best_row.get("result_digest") if best_row else None,
            "author": author if known_author else None,
        },
        "attribution": "known" if known_author else "unknown",
        "strict_candidate_frontier_ratio_target": candidate_frontier_ratio,
        "fractional_gain_needed_from_frontier": fractional_gain_needed,
        "threshold_state": (
            "existing_eligible_record"
            if best_row is not None
            else "no_existing_eligible_record_engine_baseline_1.0"
        ),
        "source_window_limited": True,
        "all_time_best_guaranteed": False,
        "gain_interpretation": (
            "candidate must be strictly faster than the best qualifying ratio; equality does not advance the record"
            if best_row is not None
            else (
                "no existing eligible record; engine baseline is 1.0, so the threshold may exceed 1.0; "
                "a threshold alone is not validation"
            )
        ),
        "unique_digests_seen": len(rows_by_digest),
        "eligible_measured_improvements": len(eligible),
        "note": (
            "This is a score threshold calculation only; it makes no points, promotion, or award prediction. "
            "The captured source window does not guarantee an all-time best. A threshold alone is not validation."
        ),
    }


def main(argv: Optional[list[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("results_json", type=Path, help="saved official internal/results JSON; no network is used")
    args = parser.parse_args(argv)
    try:
        with args.results_json.open("r", encoding="utf-8") as handle:
            result = calculate(json.load(handle))
    except (OSError, json.JSONDecodeError, DataError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2
    print(json.dumps(result, indent=2, sort_keys=True, allow_nan=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

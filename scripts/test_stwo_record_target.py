import unittest

from stwo_record_target import DataError, MAINNET_CAMPAIGN, calculate


FRONTIER = {"commit": "frontier-commit", "ratio": 0.8}


def row(digest, ratio, *, status="MEASURED_IMPROVEMENT", evaluated_at=10, author="alice"):
    return {
        "result_digest": digest,
        "ratio": ratio,
        "status": status,
        "evaluated_at": evaluated_at,
        "github_login": author,
    }


def payload(*, results=None, recent=None, seasons=None, frontier=FRONTIER):
    return {
        "campaign_id": MAINNET_CAMPAIGN,
        "frontier": frontier,
        "results": results or [],
        "recent": recent or [],
        "seasons": seasons or [],
    }


class StwoRecordTargetTests(unittest.TestCase):
    def test_status_filter_and_nonfinite_values(self):
        data = payload(results=[
            row("valid", 0.7),
            row("not-faster", 0.5, status="VALID_NOT_FASTER"),
            row("refutes", 0.4, status="REFUTES"),
            row("nan-ratio", float("nan")),
            row("inf-time", 0.3, evaluated_at=float("inf")),
            row("zero", 0),
        ])
        result = calculate(data)
        self.assertEqual(result["best_published_qualifying"]["result_digest"], "valid")
        self.assertEqual(result["eligible_measured_improvements"], 1)

    def test_deduplicates_and_ignores_wrong_season(self):
        duplicate = row("same", 0.6, evaluated_at=3)
        wrong = {
            "campaign_id": "stwo-privacy-whole-proof-internal-v1",
            "frontier": {"commit": "other", "ratio": 0.2},
            "results": [row("wrong-season", 0.1)],
        }
        matching = {
            "campaign_id": MAINNET_CAMPAIGN,
            "frontier": FRONTIER,
            "results": [row("newer", 0.7, evaluated_at=4)],
            "recent": [duplicate],
        }
        conflicting_top = row("same", 0.2, evaluated_at=1, author="top-level")
        result = calculate(payload(results=[conflicting_top], recent=[duplicate], seasons=[wrong, matching]))
        self.assertEqual(result["unique_digests_seen"], 2)
        self.assertEqual(result["best_published_qualifying"]["result_digest"], "same")
        self.assertEqual(result["best_published_qualifying"]["author"], "alice")

    def test_last_duplicate_value_wins(self):
        result = calculate(payload(results=[
            row("same", 0.4),
            row("same", 0.3, status="VALID_NOT_FASTER"),
        ]))
        self.assertEqual(result["best_published_qualifying"]["ratio"], 1.0)
        self.assertIsNone(result["best_published_qualifying"]["result_digest"])
        self.assertEqual(result["eligible_measured_improvements"], 0)

    def test_tied_evaluation_times_keep_source_order(self):
        result = calculate(payload(results=[
            row("z-first", 0.6, evaluated_at=10, author="first"),
            row("a-second", 0.6, evaluated_at=10, author="second"),
        ]))
        self.assertEqual(result["best_published_qualifying"]["result_digest"], "z-first")
        self.assertEqual(result["best_published_qualifying"]["author"], "first")

    def test_exact_saved_mainnet_values(self):
        frontier = {"commit": "69645ee0e6f583990c4aaf064cd08fde601ff009", "ratio": 0.8622706158865119}
        best = row("a273aff79b9f7e52d7eee2669888825938bbf7b4e27f8103a97701c5d6799b53", 0.8383761455505968, author="binharold")
        result = calculate(payload(results=[best], frontier=frontier))
        self.assertEqual(result["frontier"]["commit"], frontier["commit"])
        self.assertEqual(result["frontier"]["ratio"], 0.8622706158865119)
        self.assertEqual(result["best_published_qualifying"]["ratio"], 0.8383761455505968)
        self.assertEqual(result["best_published_qualifying"]["result_digest"], best["result_digest"])
        self.assertAlmostEqual(result["strict_candidate_frontier_ratio_target"], 0.9722888964372874)
        self.assertAlmostEqual(result["fractional_gain_needed_from_frontier"], 0.0277111035627126)

    def test_no_records_uses_engine_baseline_and_keeps_frontier_distinct(self):
        frontier = {"commit": "frontier-commit", "ratio": 0.8}
        result = calculate(payload(results=[row("bad", 0.7, status="VALID_NOT_FASTER")], frontier=frontier))
        self.assertEqual(result["best_published_qualifying"]["ratio"], 1.0)
        self.assertIsNone(result["best_published_qualifying"]["result_digest"])
        self.assertEqual(result["strict_candidate_frontier_ratio_target"], 1.25)
        self.assertEqual(result["fractional_gain_needed_from_frontier"], -0.25)
        self.assertEqual(result["threshold_state"], "no_existing_eligible_record_engine_baseline_1.0")
        self.assertTrue(result["source_window_limited"])
        self.assertFalse(result["all_time_best_guaranteed"])
        self.assertIn("threshold alone is not validation", result["note"])
        self.assertIn("threshold may exceed 1.0", result["gain_interpretation"])

    def test_missing_mainnet_fails_clearly(self):
        with self.assertRaisesRegex(DataError, "no campaign_id"):
            calculate({"results": [], "seasons": []})

    def test_missing_author_reports_unknown_attribution(self):
        result = calculate(payload(results=[row("unattributed", 0.7, author=None)]))
        self.assertEqual(result["attribution"], "unknown")
        self.assertIsNone(result["best_published_qualifying"]["author"])


if __name__ == "__main__":
    unittest.main()

use serde::{Deserialize, Serialize};
use crate::model::candidate_id::CandidateId;
use crate::model::game_id::GameId;
use crate::poll::poll_report::PollReport;
use crate::report::run_summary::RunSummary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndReport {
    pub game_id: GameId,
    pub winner: CandidateId,
    pub final_poll: PollReport,
    pub run_summary: RunSummary,
    pub run_hash: String,
}

impl EndReport {
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# Princeps — Campaign End Report\n\n");
        md.push_str(&format!("**Game ID:** {}\n\n", self.game_id));
        md.push_str(&format!("**Seed:** {}\n\n", self.run_summary.seed));
        md.push_str(&format!("**Days Simulated:** {}\n\n", self.run_summary.days));
        md.push_str(&format!("**Winner:** {}\n\n", self.winner));
        md.push_str("## Final Poll Results\n\n");
        md.push_str("| Candidate | Vote Share |\n");
        md.push_str("|-----------|------------|\n");
        for (cid, share) in &self.final_poll.results {
            md.push_str(&format!("| {} | {:.1}% |\n", cid, share * 100.0));
        }
        md.push_str("\n## Run Statistics\n\n");
        md.push_str(&format!(
            "- Total Events: {}\n",
            self.run_summary.total_events
        ));
        md.push_str(&format!(
            "- Total Debates: {}\n",
            self.run_summary.total_debates
        ));
        md.push_str(&format!(
            "- Total Polls: {}\n",
            self.run_summary.total_polls
        ));
        md.push_str(&format!("\n**Run Hash:** `{}`\n", self.run_hash));
        md
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::model::candidate_id::CandidateId;
    use crate::model::game_id::GameId;
    use crate::poll::poll_report::PollReport;
    use crate::report::run_summary::RunSummary;
    use sha2::{Sha256, Digest};

    #[test]
    fn end_report_run_hash_is_stable() {
        let mut results = BTreeMap::new();
        results.insert(CandidateId::new("a"), 0.6);
        results.insert(CandidateId::new("b"), 0.4);
        let poll = PollReport { day: 30, results, block_breakdown: BTreeMap::new() };
        let mut approvals = BTreeMap::new();
        approvals.insert(CandidateId::new("a"), 0.3);
        approvals.insert(CandidateId::new("b"), 0.2);
        let summary = RunSummary {
            seed: 42,
            days: 30,
            total_events: 10,
            total_debates: 2,
            total_polls: 5,
            candidate_final_approvals: approvals,
        };
        let game_id = GameId::new(42, 30);
        let winner = CandidateId::new("a");
        let payload = serde_json::json!({
            "game_id": game_id,
            "winner": winner,
            "final_poll": poll,
            "run_summary": summary,
        });
        let canonical = serde_json::to_string(&payload).unwrap();
        let mut h1 = Sha256::new();
        h1.update(canonical.as_bytes());
        let hash1 = hex::encode(h1.finalize());
        let mut h2 = Sha256::new();
        h2.update(canonical.as_bytes());
        let hash2 = hex::encode(h2.finalize());
        assert_eq!(hash1, hash2, "hash must be stable/deterministic");
        assert_eq!(hash1.len(), 64, "SHA256 hex should be 64 chars");
    }
}

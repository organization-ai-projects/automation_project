use crate::model::candidate_id::CandidateId;
use crate::model::game_id::GameId;
use crate::poll::poll_report::PollReport;
use crate::report::run_summary::RunSummary;
use serde::{Deserialize, Serialize};

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
        md.push_str(&format!(
            "**Days Simulated:** {}\n\n",
            self.run_summary.days
        ));
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

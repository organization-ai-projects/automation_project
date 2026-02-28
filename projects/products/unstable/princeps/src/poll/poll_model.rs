use std::collections::BTreeMap;
use crate::model::candidate::Candidate;
use crate::model::candidate_id::CandidateId;
use crate::model::voter_block::VoterBlock;
use crate::poll::poll_report::PollReport;

pub struct PollModel;

impl PollModel {
    pub fn compute(
        day: u32,
        candidates: &[Candidate],
        voter_blocks: &[VoterBlock],
    ) -> PollReport {
        let total_size: u32 = voter_blocks.iter().map(|b| b.size).sum();
        let total_size = if total_size == 0 { 1 } else { total_size };

        let mut results: BTreeMap<CandidateId, f64> = BTreeMap::new();
        let mut block_breakdown: BTreeMap<String, BTreeMap<CandidateId, f64>> = BTreeMap::new();

        for block in voter_blocks {
            let block_support = Self::compute_block_support(candidates, block);
            let weight = block.size as f64 / total_size as f64;
            for (cid, support) in &block_support {
                *results.entry(cid.clone()).or_insert(0.0) += support * weight;
            }
            block_breakdown.insert(block.id.clone(), block_support);
        }

        // Normalize overall results
        let total: f64 = results.values().sum();
        if total > 0.0 {
            for v in results.values_mut() {
                *v /= total;
            }
        }

        PollReport {
            day,
            results,
            block_breakdown,
        }
    }

    fn compute_block_support(
        candidates: &[Candidate],
        block: &VoterBlock,
    ) -> BTreeMap<CandidateId, f64> {
        let mut support: BTreeMap<CandidateId, f64> = BTreeMap::new();

        for candidate in candidates {
            let mut score = candidate.approval;

            for (topic, preferred_pos) in &block.preferences {
                if let Some(cand_pos) = candidate.positions.get(topic) {
                    let diff = (*cand_pos - preferred_pos).abs() as f64;
                    let alignment = 1.0 - (diff / 10.0).min(1.0);
                    let sensitivity = block.sensitivities.get(topic).copied().unwrap_or(1.0);
                    score += alignment * sensitivity * 0.1;
                }
            }

            let charisma_w = block.sensitivities.get("charisma").copied().unwrap_or(0.5);
            score += candidate.charisma as f64 / 100.0 * charisma_w * 0.1;

            support.insert(candidate.id.clone(), score.max(0.0));
        }

        let total: f64 = support.values().sum();
        if total > 0.0 {
            for v in support.values_mut() {
                *v /= total;
            }
        }

        support
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::candidate::Candidate;
    use crate::model::voter_block::VoterBlock;

    #[test]
    fn poll_model_determinism() {
        let candidates = vec![
            Candidate::new("a", "Alice", 70, 60, 80, 30),
            Candidate::new("b", "Bob", 55, 75, 65, 45),
        ];
        let blocks = vec![
            VoterBlock::new("urban", "Urban", 50),
            VoterBlock::new("rural", "Rural", 50),
        ];
        let report1 = PollModel::compute(1, &candidates, &blocks);
        let report2 = PollModel::compute(1, &candidates, &blocks);
        assert_eq!(
            serde_json::to_string(&report1).unwrap(),
            serde_json::to_string(&report2).unwrap(),
            "poll model must be deterministic"
        );
    }
}

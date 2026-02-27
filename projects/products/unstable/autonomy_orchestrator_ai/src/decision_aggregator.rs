use crate::domain::{DecisionContribution, DecisionSummary, FinalDecision};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecisionAggregatorConfig {
    pub min_confidence_to_proceed: u8,
}

impl Default for DecisionAggregatorConfig {
    fn default() -> Self {
        Self {
            min_confidence_to_proceed: 70,
        }
    }
}

pub fn aggregate(
    contributions: &[DecisionContribution],
    cfg: &DecisionAggregatorConfig,
) -> DecisionSummary {
    if contributions.is_empty() {
        return DecisionSummary {
            final_decision: FinalDecision::Block,
            decision_confidence: 0,
            decision_rationale_codes: vec!["DECISION_NO_CONTRIBUTIONS".to_string()],
            contributions: Vec::new(),
            threshold: cfg.min_confidence_to_proceed,
        };
    }

    let mut proceed_score = 0u32;
    let mut block_score = 0u32;
    let mut escalate_score = 0u32;
    let mut proceed_max_conf = 0u8;
    let mut block_max_conf = 0u8;
    let mut escalate_max_conf = 0u8;
    let mut proceed_count = 0u32;
    let mut block_count = 0u32;
    let mut escalate_count = 0u32;

    for contribution in contributions {
        let score = u32::from(contribution.confidence) * u32::from(contribution.weight);
        match contribution.vote {
            FinalDecision::Proceed => {
                proceed_score += score;
                proceed_count += 1;
                proceed_max_conf = proceed_max_conf.max(contribution.confidence);
            }
            FinalDecision::Block => {
                block_score += score;
                block_count += 1;
                block_max_conf = block_max_conf.max(contribution.confidence);
            }
            FinalDecision::Escalate => {
                escalate_score += score;
                escalate_count += 1;
                escalate_max_conf = escalate_max_conf.max(contribution.confidence);
            }
        }
    }

    let mut rationale_codes = Vec::new();
    let mut candidates = vec![
        VoteStats {
            vote: FinalDecision::Proceed,
            score: proceed_score,
            max_confidence: proceed_max_conf,
            count: proceed_count,
            fail_closed_rank: 2,
        },
        VoteStats {
            vote: FinalDecision::Block,
            score: block_score,
            max_confidence: block_max_conf,
            count: block_count,
            fail_closed_rank: 0,
        },
        VoteStats {
            vote: FinalDecision::Escalate,
            score: escalate_score,
            max_confidence: escalate_max_conf,
            count: escalate_count,
            fail_closed_rank: 1,
        },
    ];

    let max_score = candidates.iter().map(|c| c.score).max().unwrap_or(0);
    candidates.retain(|c| c.score == max_score);
    if candidates.len() > 1 {
        let max_conf = candidates
            .iter()
            .map(|c| c.max_confidence)
            .max()
            .unwrap_or(0);
        candidates.retain(|c| c.max_confidence == max_conf);
    }
    if candidates.len() > 1 {
        let max_count = candidates.iter().map(|c| c.count).max().unwrap_or(0);
        candidates.retain(|c| c.count == max_count);
    }
    if candidates.len() > 1 {
        rationale_codes.push("DECISION_TIE_FAIL_CLOSED".to_string());
        candidates.sort_by_key(|c| c.fail_closed_rank);
    }

    let winner = candidates[0];
    let total_score = proceed_score + block_score + escalate_score;
    let decision_confidence = if total_score == 0 {
        0
    } else {
        let numerator = winner.score.saturating_mul(100);
        let rounded = numerator + (total_score / 2);
        (rounded / total_score) as u8
    };

    let mut final_decision = winner.vote;
    if final_decision == FinalDecision::Proceed
        && decision_confidence < cfg.min_confidence_to_proceed
    {
        final_decision = FinalDecision::Block;
        rationale_codes.push("DECISION_CONFIDENCE_BELOW_THRESHOLD".to_string());
    }
    if final_decision == FinalDecision::Escalate {
        rationale_codes.push("DECISION_ESCALATED".to_string());
    }

    DecisionSummary {
        final_decision,
        decision_confidence,
        decision_rationale_codes: rationale_codes,
        contributions: contributions.to_vec(),
        threshold: cfg.min_confidence_to_proceed,
    }
}

#[derive(Debug, Clone, Copy)]
struct VoteStats {
    vote: FinalDecision,
    score: u32,
    max_confidence: u8,
    count: u32,
    fail_closed_rank: u8,
}

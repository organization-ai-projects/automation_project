use crate::domain::{
    DecisionContribution, DecisionReliabilityFactor, DecisionReliabilityInput,
    DecisionReliabilityUpdate, DecisionSummary, FinalDecision,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionAggregatorConfig {
    pub min_confidence_to_proceed: u8,
    pub reliability_inputs: Vec<DecisionReliabilityInput>,
    pub memory_reliability_inputs: Vec<DecisionReliabilityInput>,
}

impl Default for DecisionAggregatorConfig {
    fn default() -> Self {
        Self {
            min_confidence_to_proceed: 70,
            reliability_inputs: Vec::new(),
            memory_reliability_inputs: Vec::new(),
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
            reliability_factors: Vec::new(),
            reliability_updates: Vec::new(),
            threshold: cfg.min_confidence_to_proceed,
        };
    }

    let mut proceed_score = 0u64;
    let mut block_score = 0u64;
    let mut escalate_score = 0u64;
    let mut proceed_max_conf = 0u8;
    let mut block_max_conf = 0u8;
    let mut escalate_max_conf = 0u8;
    let mut proceed_count = 0u32;
    let mut block_count = 0u32;
    let mut escalate_count = 0u32;
    let mut reliability_factors = Vec::<DecisionReliabilityFactor>::new();
    let mut used_reliability_input = false;
    let mut used_cold_start = false;
    let mut used_memory_input = false;

    // Pre-compute the set of (contributor_id, capability) pairs that have an explicit
    // reliability input to avoid O(n*m) per-contribution scan inside the loop.
    let explicit_input_keys: Vec<(&str, &str)> = cfg
        .reliability_inputs
        .iter()
        .map(|i| (i.contributor_id.as_str(), i.capability.as_str()))
        .collect();

    for contribution in contributions {
        let reliability_score = lookup_reliability_score(
            contribution,
            &cfg.reliability_inputs,
            &cfg.memory_reliability_inputs,
        );
        let has_explicit = explicit_input_keys
            .iter()
            .any(|&(id, cap)| id == contribution.contributor_id && cap == contribution.capability);
        let from_memory = !has_explicit
            && cfg.memory_reliability_inputs.iter().any(|input| {
                input.contributor_id == contribution.contributor_id
                    && input.capability == contribution.capability
            });
        if reliability_score == 50 && !from_memory {
            used_cold_start = true;
        } else if from_memory {
            used_memory_input = true;
        } else {
            used_reliability_input = true;
        }
        let reliability_factor = u16::from(50u8.saturating_add(reliability_score));
        let base_score = u64::from(contribution.confidence) * u64::from(contribution.weight);
        let score = base_score * u64::from(reliability_factor);

        reliability_factors.push(DecisionReliabilityFactor {
            contributor_id: contribution.contributor_id.clone(),
            capability: contribution.capability.clone(),
            reliability_score,
            reliability_factor,
            base_score,
            adjusted_score: score,
        });

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
    if used_reliability_input {
        rationale_codes.push("DECISION_RELIABILITY_WEIGHTED".to_string());
    }
    if used_cold_start {
        rationale_codes.push("DECISION_RELIABILITY_COLD_START".to_string());
    }
    if used_memory_input {
        rationale_codes.push("MEMORY_SIGNAL_APPLIED".to_string());
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
    let reliability_updates = build_reliability_updates(
        contributions,
        final_decision,
        &cfg.reliability_inputs,
        &cfg.memory_reliability_inputs,
    );

    DecisionSummary {
        final_decision,
        decision_confidence,
        decision_rationale_codes: rationale_codes,
        contributions: contributions.to_vec(),
        reliability_factors,
        reliability_updates,
        threshold: cfg.min_confidence_to_proceed,
    }
}

#[derive(Debug, Clone, Copy)]
struct VoteStats {
    vote: FinalDecision,
    score: u64,
    max_confidence: u8,
    count: u32,
    fail_closed_rank: u8,
}

fn lookup_reliability_score(
    contribution: &DecisionContribution,
    reliability_inputs: &[DecisionReliabilityInput],
    memory_reliability_inputs: &[DecisionReliabilityInput],
) -> u8 {
    reliability_inputs
        .iter()
        .find(|input| {
            input.contributor_id == contribution.contributor_id
                && input.capability == contribution.capability
        })
        .or_else(|| {
            memory_reliability_inputs.iter().find(|input| {
                input.contributor_id == contribution.contributor_id
                    && input.capability == contribution.capability
            })
        })
        .map(|input| input.score)
        .unwrap_or(50)
}

fn build_reliability_updates(
    contributions: &[DecisionContribution],
    final_decision: FinalDecision,
    reliability_inputs: &[DecisionReliabilityInput],
    memory_reliability_inputs: &[DecisionReliabilityInput],
) -> Vec<DecisionReliabilityUpdate> {
    contributions
        .iter()
        .map(|contribution| {
            let previous_score =
                lookup_reliability_score(contribution, reliability_inputs, memory_reliability_inputs);
            let mut delta: i16 = if contribution.vote == final_decision {
                2
            } else {
                -2
            };
            if contribution.confidence >= 80 {
                delta += if delta.is_positive() { 1 } else { -1 };
            }
            let new_score = clamp_score(previous_score, delta);
            let reason_code = if contribution.vote == final_decision {
                "RELIABILITY_REWARD_ALIGNMENT"
            } else {
                "RELIABILITY_PENALIZE_DIVERGENCE"
            };
            DecisionReliabilityUpdate {
                contributor_id: contribution.contributor_id.clone(),
                capability: contribution.capability.clone(),
                previous_score,
                new_score,
                reason_code: reason_code.to_string(),
            }
        })
        .collect()
}

fn clamp_score(previous_score: u8, delta: i16) -> u8 {
    let raw = i16::from(previous_score) + delta;
    raw.clamp(0, 100) as u8
}

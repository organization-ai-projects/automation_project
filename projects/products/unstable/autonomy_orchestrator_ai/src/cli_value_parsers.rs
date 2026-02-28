// projects/products/unstable/autonomy_orchestrator_ai/src/cli_value_parsers.rs
use crate::domain::{
    DecisionContribution, DecisionReliabilityInput, FinalDecision, ReviewVerdict, ReviewerVerdict,
};

pub fn parse_env_pair_cli(raw: &str) -> Result<(String, String), String> {
    let mut split = raw.splitn(2, '=');
    let key = split.next().unwrap_or_default().trim();
    let value = split.next();
    if key.is_empty() || value.is_none() {
        return Err(format!("Invalid env pair '{}', expected KEY=VALUE", raw));
    }
    Ok((key.to_string(), value.unwrap_or_default().to_string()))
}

pub fn parse_decision_contribution_cli(raw: &str) -> Result<DecisionContribution, String> {
    let mut contributor_id = None::<String>;
    let mut capability = None::<String>;
    let mut vote = None::<FinalDecision>;
    let mut confidence = None::<u8>;
    let mut weight = None::<u8>;
    let mut reason_codes = Vec::<String>::new();
    let mut artifact_refs = Vec::<String>::new();

    for segment in raw.split(',') {
        let (key, value) = segment
            .split_once('=')
            .ok_or_else(|| format!("Invalid decision contribution segment '{segment}'"))?;
        let key = key.trim();
        let value = value.trim();
        match key {
            "contributor_id" => contributor_id = Some(value.to_string()),
            "capability" => capability = Some(value.to_string()),
            "vote" => {
                vote = Some(parse_vote(value).ok_or_else(|| {
                    format!(
                        "Invalid decision vote '{}', expected proceed|block|escalate",
                        value
                    )
                })?)
            }
            "confidence" => {
                confidence = Some(value.parse::<u8>().map_err(|_| {
                    format!("Invalid confidence '{}', expected integer 0..100", value)
                })?)
            }
            "weight" => {
                weight =
                    Some(value.parse::<u8>().map_err(|_| {
                        format!("Invalid weight '{}', expected integer 1..100", value)
                    })?)
            }
            "reason_codes" => {
                reason_codes = split_pipe_list(value);
            }
            "artifact_refs" => {
                artifact_refs = split_pipe_list(value);
            }
            other => return Err(format!("Unknown decision contribution key '{}'", other)),
        }
    }

    let confidence = confidence
        .ok_or_else(|| "Missing decision contribution field 'confidence' (0..100)".to_string())?;
    if confidence > 100 {
        return Err(format!(
            "Invalid confidence '{}', expected integer 0..100",
            confidence
        ));
    }

    let weight = weight
        .ok_or_else(|| "Missing decision contribution field 'weight' (1..100)".to_string())?;
    if weight == 0 || weight > 100 {
        return Err(format!(
            "Invalid weight '{}', expected integer 1..100",
            weight
        ));
    }

    Ok(DecisionContribution {
        contributor_id: contributor_id
            .ok_or_else(|| "Missing decision contribution field 'contributor_id'".to_string())?,
        capability: capability
            .ok_or_else(|| "Missing decision contribution field 'capability'".to_string())?,
        vote: vote.ok_or_else(|| "Missing decision contribution field 'vote'".to_string())?,
        confidence,
        weight,
        reason_codes,
        artifact_refs,
    })
}

pub fn parse_reviewer_verdict_cli(raw: &str) -> Result<ReviewerVerdict, String> {
    let mut reviewer_id = None::<String>;
    let mut specialty = None::<String>;
    let mut verdict = None::<ReviewVerdict>;
    let mut confidence = None::<u8>;
    let mut weight = None::<u8>;
    let mut reason_codes = Vec::<String>::new();

    for segment in raw.split(',') {
        let (key, value) = segment
            .split_once('=')
            .ok_or_else(|| format!("Invalid reviewer verdict segment '{segment}'"))?;
        let key = key.trim();
        let value = value.trim();
        match key {
            "reviewer_id" => reviewer_id = Some(value.to_string()),
            "specialty" => specialty = Some(value.to_string()),
            "verdict" => {
                verdict = Some(match value {
                    "approve" => ReviewVerdict::Approve,
                    "reject" => ReviewVerdict::Reject,
                    other => {
                        return Err(format!(
                            "Invalid reviewer verdict '{}', expected approve|reject",
                            other
                        ));
                    }
                })
            }
            "confidence" => {
                confidence = Some(value.parse::<u8>().map_err(|_| {
                    format!("Invalid confidence '{}', expected integer 0..100", value)
                })?)
            }
            "weight" => {
                weight =
                    Some(value.parse::<u8>().map_err(|_| {
                        format!("Invalid weight '{}', expected integer 1..100", value)
                    })?)
            }
            "reason_codes" => {
                reason_codes = split_pipe_list(value);
            }
            other => return Err(format!("Unknown reviewer verdict key '{}'", other)),
        }
    }

    let confidence = confidence
        .ok_or_else(|| "Missing reviewer verdict field 'confidence' (0..100)".to_string())?;
    if confidence > 100 {
        return Err(format!(
            "Invalid confidence '{}', expected integer 0..100",
            confidence
        ));
    }

    let weight =
        weight.ok_or_else(|| "Missing reviewer verdict field 'weight' (1..100)".to_string())?;
    if weight == 0 || weight > 100 {
        return Err(format!(
            "Invalid weight '{}', expected integer 1..100",
            weight
        ));
    }

    Ok(ReviewerVerdict {
        reviewer_id: reviewer_id
            .ok_or_else(|| "Missing reviewer verdict field 'reviewer_id'".to_string())?,
        specialty: specialty
            .ok_or_else(|| "Missing reviewer verdict field 'specialty'".to_string())?,
        verdict: verdict.ok_or_else(|| "Missing reviewer verdict field 'verdict'".to_string())?,
        confidence,
        weight,
        reason_codes,
    })
}

fn parse_vote(raw: &str) -> Option<FinalDecision> {
    match raw {
        "proceed" => Some(FinalDecision::Proceed),
        "block" => Some(FinalDecision::Block),
        "escalate" => Some(FinalDecision::Escalate),
        _ => None,
    }
}

fn split_pipe_list(raw: &str) -> Vec<String> {
    raw.split('|')
        .filter_map(|v| {
            let value = v.trim();
            (!value.is_empty()).then(|| value.to_string())
        })
        .collect()
}

pub fn parse_decision_reliability_input_cli(raw: &str) -> Result<DecisionReliabilityInput, String> {
    let mut contributor_id = None::<String>;
    let mut capability = None::<String>;
    let mut score = None::<u8>;

    for segment in raw.split(',') {
        let (key, value) = segment
            .split_once('=')
            .ok_or_else(|| format!("Invalid decision reliability segment '{segment}'"))?;
        let key = key.trim();
        let value = value.trim();
        match key {
            "contributor_id" => contributor_id = Some(value.to_string()),
            "capability" => capability = Some(value.to_string()),
            "score" => {
                score = Some(value.parse::<u8>().map_err(|_| {
                    format!(
                        "Invalid reliability score '{}', expected integer 0..100",
                        value
                    )
                })?)
            }
            other => return Err(format!("Unknown decision reliability key '{}'", other)),
        }
    }

    let score =
        score.ok_or_else(|| "Missing decision reliability field 'score' (0..100)".to_string())?;
    if score > 100 {
        return Err(format!(
            "Invalid reliability score '{}', expected integer 0..100",
            score
        ));
    }

    Ok(DecisionReliabilityInput {
        contributor_id: contributor_id
            .ok_or_else(|| "Missing decision reliability field 'contributor_id'".to_string())?,
        capability: capability
            .ok_or_else(|| "Missing decision reliability field 'capability'".to_string())?,
        score,
    })
}

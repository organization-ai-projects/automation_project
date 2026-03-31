use crate::config::FeatureGateConfig;
use crate::decision::candidate_action::CandidateAction;
use crate::decision::decision_confidence::DecisionConfidence;
use crate::decision::decision_reason::{DecisionReason, ReasonCategory};
use crate::decision::decision_summary::{ActionScore, DecisionSummary};
use crate::decision::wait_thesis::WaitThesis;
use crate::portfolio::UnrealizedPnl;
use crate::risk::RiskScore;
use crate::sentiment::sentiment_engine::SentimentLabel;

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn synthesize(
        risk: &RiskScore,
        pnl: &UnrealizedPnl,
        sentiment: &SentimentLabel,
        gate: &FeatureGateConfig,
    ) -> DecisionSummary {
        if !gate.is_recommendation_allowed() {
            return DecisionSummary::gated();
        }

        let mut sell_score: f64 = 0.0;
        let mut hold_score: f64 = 0.0;
        let mut buy_score: f64 = 0.0;
        let mut reasons = Vec::new();
        let mut risks = Vec::new();
        let mut invalidations = Vec::new();
        let mut new_facts = Vec::new();

        if risk.thesis_break.score > 0.8 {
            sell_score += 0.4;
            reasons.push(DecisionReason::new(
                ReasonCategory::ThesisIntegrity,
                "Investment thesis is broken or severely weakened",
                0.4,
            ));
            risks.push("Thesis may be permanently impaired".to_string());
        } else if risk.thesis_break.score > 0.4 {
            hold_score += 0.2;
            reasons.push(DecisionReason::new(
                ReasonCategory::ThesisIntegrity,
                "Investment thesis is weakened but not broken",
                0.2,
            ));
        } else {
            hold_score += 0.1;
            buy_score += 0.1;
        }

        if risk.composite_score > 0.7 {
            sell_score += 0.3;
            risks.push("High composite risk score".to_string());
            invalidations.push("Risk score drops below 0.5".to_string());
        } else if risk.composite_score < 0.3 {
            buy_score += 0.2;
            new_facts.push("Risk score increases above 0.5".to_string());
        } else {
            hold_score += 0.15;
        }

        if pnl.is_loss() && pnl.drawdown_from_purchase < -0.3 {
            if risk.thesis_break.score < 0.5 {
                hold_score += 0.3;
                reasons.push(DecisionReason::new(
                    ReasonCategory::CostBasis,
                    "Significant unrealized loss but thesis intact - waiting preferred",
                    0.3,
                ));
            } else {
                sell_score += 0.2;
                reasons.push(DecisionReason::new(
                    ReasonCategory::CostBasis,
                    "Significant unrealized loss with weakened thesis",
                    0.2,
                ));
            }
        } else if pnl.is_gain() && risk.composite_score > 0.6 {
            sell_score += 0.2;
            reasons.push(DecisionReason::new(
                ReasonCategory::CostBasis,
                "Unrealized gain with elevated risk - consider taking profits",
                0.2,
            ));
        }

        match sentiment {
            SentimentLabel::Bearish => {
                if risk.thesis_break.score < 0.5 {
                    buy_score += 0.15;
                    reasons.push(DecisionReason::new(
                        ReasonCategory::Sentiment,
                        "Bearish sentiment with intact thesis - potential accumulation opportunity",
                        0.15,
                    ));
                }
            }
            SentimentLabel::Bullish => {
                hold_score += 0.1;
            }
            SentimentLabel::Neutral => {
                hold_score += 0.05;
            }
        }

        let action_scores = Self::build_action_scores(sell_score, hold_score, buy_score, &reasons);

        let recommended = action_scores
            .iter()
            .max_by(|a, b| {
                a.score
                    .partial_cmp(&b.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|a| a.action.clone())
            .unwrap_or(CandidateAction::Hold);

        let max_score = action_scores
            .iter()
            .map(|a| a.score)
            .reduce(f64::max)
            .unwrap_or(0.0);
        let total_score: f64 = action_scores.iter().map(|a| a.score).sum();
        let confidence_score = if total_score > 0.0 {
            max_score / total_score
        } else {
            0.0
        };

        let wait_thesis = if pnl.is_loss() && recommended != CandidateAction::Sell {
            Some(WaitThesis::hold_recommended(
                "Loss recovery more likely than immediate exit based on current analysis",
                confidence_score,
                None,
            ))
        } else if pnl.is_loss() && recommended == CandidateAction::Sell {
            Some(WaitThesis::exit_recommended(
                "Exit recommended despite unrealized loss",
            ))
        } else {
            None
        };

        DecisionSummary {
            recommended_action: recommended,
            confidence: DecisionConfidence::from_score(confidence_score),
            action_scores,
            primary_reasons: reasons,
            principal_risks: risks,
            invalidation_conditions: invalidations,
            new_facts_that_would_change: new_facts,
            wait_thesis,
            recommendation_gated: false,
        }
    }

    fn build_action_scores(
        sell_score: f64,
        hold_score: f64,
        buy_score: f64,
        reasons: &[DecisionReason],
    ) -> Vec<ActionScore> {
        CandidateAction::canonical_order()
            .into_iter()
            .map(|action| {
                let score = match action {
                    CandidateAction::Sell => sell_score,
                    CandidateAction::Hold => hold_score,
                    CandidateAction::BuyMore => buy_score,
                };
                let action_reasons: Vec<DecisionReason> =
                    reasons.iter().filter(|r| r.weight > 0.0).cloned().collect();
                ActionScore {
                    action,
                    score,
                    reasons: action_reasons,
                }
            })
            .collect()
    }
}

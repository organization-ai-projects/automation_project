use serde::{Deserialize, Serialize};

use crate::sentiment::{MarketFearScore, NarrativeShift};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentimentSummary {
    pub fear_score: Option<MarketFearScore>,
    pub narrative_shifts: Vec<NarrativeShift>,
    pub overall_sentiment: SentimentLabel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentimentLabel {
    Bearish,
    Neutral,
    Bullish,
}

pub struct SentimentEngine;

impl SentimentEngine {
    pub fn evaluate(
        fear_score: Option<MarketFearScore>,
        narrative_shifts: Vec<NarrativeShift>,
    ) -> SentimentSummary {
        let overall = Self::compute_overall(&fear_score, &narrative_shifts);
        SentimentSummary {
            fear_score,
            narrative_shifts,
            overall_sentiment: overall,
        }
    }

    fn compute_overall(
        fear_score: &Option<MarketFearScore>,
        shifts: &[NarrativeShift],
    ) -> SentimentLabel {
        let fear_bearish = fear_score.as_ref().is_some_and(|f| f.is_fearful());
        let shift_bearish = shifts.iter().any(|s| s.is_bearish_shift());

        if fear_bearish && shift_bearish {
            SentimentLabel::Bearish
        } else if fear_bearish || shift_bearish {
            SentimentLabel::Neutral
        } else {
            SentimentLabel::Bullish
        }
    }
}

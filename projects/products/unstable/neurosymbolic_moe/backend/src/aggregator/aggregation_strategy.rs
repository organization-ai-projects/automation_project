use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationStrategy {
    HighestConfidence,
    WeightedAverage,
    Majority,
    First,
    Custom(String),
}

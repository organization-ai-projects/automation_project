use crate::moe_core::{AggregatedOutput, ExpertOutput, MoeError};

use super::aggregation_strategy::AggregationStrategy;

#[derive(Debug, Clone)]
pub struct OutputAggregator {
    strategy: AggregationStrategy,
}

impl OutputAggregator {
    pub fn new(strategy: AggregationStrategy) -> Self {
        Self { strategy }
    }

    pub fn aggregate(
        &self,
        outputs: Vec<ExpertOutput>,
    ) -> Result<AggregatedOutput, MoeError> {
        if outputs.is_empty() {
            return Err(MoeError::AggregationFailed(
                "no outputs to aggregate".to_string(),
            ));
        }

        let selected = match &self.strategy {
            AggregationStrategy::First => outputs.first().cloned(),
            AggregationStrategy::HighestConfidence
            | AggregationStrategy::WeightedAverage
            | AggregationStrategy::Majority
            | AggregationStrategy::Custom(_) => {
                outputs
                    .iter()
                    .max_by(|a, b| {
                        a.confidence
                            .partial_cmp(&b.confidence)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .cloned()
            }
        };

        let strategy_name = match &self.strategy {
            AggregationStrategy::HighestConfidence => "highest_confidence".to_string(),
            AggregationStrategy::WeightedAverage => "weighted_average".to_string(),
            AggregationStrategy::Majority => "majority".to_string(),
            AggregationStrategy::First => "first".to_string(),
            AggregationStrategy::Custom(name) => format!("custom:{name}"),
        };

        Ok(AggregatedOutput {
            outputs,
            selected_output: selected,
            strategy: strategy_name,
        })
    }
}

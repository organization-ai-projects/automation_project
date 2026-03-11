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

    pub fn aggregate(&self, outputs: Vec<ExpertOutput>) -> Result<AggregatedOutput, MoeError> {
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
            | AggregationStrategy::Custom(_) => outputs
                .iter()
                .max_by(|a, b| {
                    a.confidence
                        .partial_cmp(&b.confidence)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .cloned(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moe_core::ExpertId;
    use std::collections::HashMap;

    fn make_output(id: &str, confidence: f64) -> ExpertOutput {
        ExpertOutput {
            expert_id: ExpertId::new(id),
            content: format!("output-{id}"),
            confidence,
            metadata: HashMap::new(),
            trace: Vec::new(),
        }
    }

    #[test]
    fn aggregate_highest_confidence() {
        let agg = OutputAggregator::new(AggregationStrategy::HighestConfidence);
        let outputs = vec![make_output("e1", 0.7), make_output("e2", 0.9)];
        let result = agg.aggregate(outputs).unwrap();
        let selected = result.selected_output.unwrap();
        assert_eq!(selected.expert_id.as_str(), "e2");
        assert_eq!(result.strategy, "highest_confidence");
    }

    #[test]
    fn aggregate_first_strategy() {
        let agg = OutputAggregator::new(AggregationStrategy::First);
        let outputs = vec![make_output("e1", 0.7), make_output("e2", 0.9)];
        let result = agg.aggregate(outputs).unwrap();
        let selected = result.selected_output.unwrap();
        assert_eq!(selected.expert_id.as_str(), "e1");
        assert_eq!(result.strategy, "first");
    }

    #[test]
    fn aggregate_empty_outputs_returns_error() {
        let agg = OutputAggregator::new(AggregationStrategy::HighestConfidence);
        let result = agg.aggregate(vec![]);
        assert!(result.is_err());
    }
}

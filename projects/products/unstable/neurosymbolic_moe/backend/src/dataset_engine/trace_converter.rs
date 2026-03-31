use std::collections::HashMap;

use crate::moe_core::{ExpertId, TaskId, TraceRecord};

use super::{DatasetEntry, Outcome};
use protocol::ProtocolId;

#[derive(Debug, Clone)]
pub struct TraceConverter;

impl TraceConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert(
        &self,
        traces: &[TraceRecord],
        input: &str,
        output: &str,
        outcome: Outcome,
    ) -> DatasetEntry {
        let task_id = traces
            .first()
            .map(|t| t.task_id.clone())
            .unwrap_or_else(TaskId::new);

        let expert_id = traces
            .iter()
            .find_map(|t| t.expert_id.clone())
            .unwrap_or_else(ExpertId::new);

        let timestamp = traces.iter().map(|t| t.timestamp).max().unwrap_or(0);

        let mut metadata = HashMap::new();
        metadata.insert("trace_count".to_string(), traces.len().to_string());
        if let Some(first) = traces.first() {
            metadata.insert("first_trace_id".to_string(), first.trace_id.clone());
        }
        if let Some(last) = traces.last() {
            metadata.insert("last_trace_id".to_string(), last.trace_id.clone());
        }

        let tags = self.extract_tags(traces);

        DatasetEntry {
            id: ProtocolId::generate(),
            task_id,
            expert_id,
            input: input.to_string(),
            output: output.to_string(),
            outcome,
            score: None,
            tags,
            created_at: timestamp,
            metadata,
        }
    }

    pub fn extract_tags(&self, traces: &[TraceRecord]) -> Vec<String> {
        let mut tags: Vec<String> = traces
            .iter()
            .map(|t| format!("phase:{:?}", t.phase))
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }
}

impl Default for TraceConverter {
    fn default() -> Self {
        Self::new()
    }
}

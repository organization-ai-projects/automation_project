//! projects/products/unstable/neurosymbolic_moe/backend/src/trace_logging/tests/trace_logger.rs
use std::collections::HashMap;

use crate::moe_core::{self, TracePhase, TraceRecord};
use crate::trace_logging::TraceLogger;
use protocol::ProtocolId;

fn task_id() -> moe_core::TaskId {
    moe_core::TaskId::from_protocol_id(ProtocolId::default())
}

fn make_record(task: u8, phase: TracePhase) -> TraceRecord {
    TraceRecord {
        trace_id: format!("tr-{task}"),
        task_id: task_id(),
        timestamp: 1,
        expert_id: None,
        phase,
        detail: "detail".to_string(),
        metadata: HashMap::new(),
    }
}

#[test]
fn log_and_count() {
    let mut logger = TraceLogger::new(100);
    logger.log(make_record(1, TracePhase::Routing));
    logger.log(make_record(2, TracePhase::ExpertExecution));
    assert_eq!(logger.count(), 2);
}

#[test]
fn max_traces_eviction() {
    let mut logger = TraceLogger::new(2);
    logger.log(make_record(1, TracePhase::Routing));
    logger.log(make_record(2, TracePhase::Routing));
    logger.log(make_record(3, TracePhase::Routing));
    assert_eq!(logger.count(), 2);
    assert!(logger.get_by_task(&task_id()).is_empty());
    assert_eq!(logger.get_by_task(&task_id()).len(), 1);
}

#[test]
fn get_by_task_returns_only_matching_traces() {
    let mut logger = TraceLogger::new(100);
    logger.log(make_record(1, TracePhase::Routing));
    logger.log(make_record(1, TracePhase::ExpertExecution));
    logger.log(make_record(2, TracePhase::Routing));
    assert_eq!(logger.get_by_task(&task_id()).len(), 2);
    assert_eq!(logger.get_by_task(&task_id()).len(), 1);
}

//! projects/products/unstable/neurosymbolic_moe/backend/src/evaluations/tests/expert_metrics.rs
use crate::evaluations::ExpertMetrics;
use crate::moe_core::ExpertId;

#[test]
fn expert_metrics_update_success_rate() {
    let mut metrics = ExpertMetrics::new(ExpertId::new());
    metrics.record_execution(true, 0.8, 12.0);
    metrics.record_execution(false, 0.2, 18.0);
    assert_eq!(metrics.total_executions, 2);
    assert!((metrics.success_rate() - 0.5).abs() < f64::EPSILON);
}

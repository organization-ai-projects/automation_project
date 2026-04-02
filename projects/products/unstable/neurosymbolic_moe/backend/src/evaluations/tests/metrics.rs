use crate::evaluations::{ExpertMetrics, RoutingMetrics};
use crate::moe_core::ExpertId;

#[test]
fn expert_metrics_record_and_success_rate() {
    let mut metrics = ExpertMetrics::new(ExpertId::new());
    assert_eq!(metrics.success_rate(), 0.0);

    metrics.record_execution(true, 0.9, 100.0);
    metrics.record_execution(true, 0.8, 200.0);
    metrics.record_execution(false, 0.5, 300.0);

    assert_eq!(metrics.total_executions, 3);
    assert_eq!(metrics.successful_executions, 2);
    assert_eq!(metrics.failed_executions, 1);
    assert!((metrics.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    assert!((metrics.average_confidence - (0.9 + 0.8 + 0.5) / 3.0).abs() < 1e-9);
}

#[test]
fn routing_metrics_record() {
    let mut metrics = RoutingMetrics::new();
    assert_eq!(metrics.accuracy(), 0.0);

    metrics.record_routing(2, false);
    metrics.record_routing(1, true);

    assert_eq!(metrics.total_routings, 2);
    assert_eq!(metrics.successful_routings, 1);
    assert_eq!(metrics.fallback_count, 1);
    assert!((metrics.accuracy() - 0.5).abs() < f64::EPSILON);
    assert!((metrics.average_experts_per_task - 1.5).abs() < f64::EPSILON);
}

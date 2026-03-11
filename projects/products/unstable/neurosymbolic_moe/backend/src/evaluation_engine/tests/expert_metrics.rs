use crate::evaluation_engine::ExpertMetrics;
use crate::moe_core::ExpertId;

#[test]
fn expert_metrics_update_success_rate() {
    let mut metrics = ExpertMetrics::new(ExpertId::new("expert-x"));
    metrics.record_execution(true, 0.8, 12.0);
    metrics.record_execution(false, 0.2, 18.0);
    assert_eq!(metrics.total_executions, 2);
    assert!((metrics.success_rate() - 0.5).abs() < f64::EPSILON);
}

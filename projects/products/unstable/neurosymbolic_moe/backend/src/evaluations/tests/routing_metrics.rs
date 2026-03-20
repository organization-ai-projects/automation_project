use crate::evaluations::RoutingMetrics;

#[test]
fn routing_metrics_update_accuracy() {
    let mut metrics = RoutingMetrics::new();
    metrics.record_routing(3, false);
    metrics.record_routing(1, true);
    assert_eq!(metrics.total_routings, 2);
    assert!((metrics.accuracy() - 0.5).abs() < f64::EPSILON);
}

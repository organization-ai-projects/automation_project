use crate::evaluation_engine::EvaluationEngine;
use crate::moe_core::ExpertId;

#[test]
fn records_and_reads_expert_metrics() {
    let mut engine = EvaluationEngine::new();
    let expert = ExpertId::new("e1");

    engine.record_expert_execution(expert.clone(), true, 0.9, 10.0);
    engine.record_expert_execution(expert.clone(), false, 0.4, 20.0);

    let metrics = engine
        .get_expert_metrics(&expert)
        .expect("metrics should exist for recorded expert");
    assert_eq!(metrics.total_executions, 2);
    assert_eq!(metrics.successful_executions, 1);
    assert_eq!(metrics.failed_executions, 1);
}

#[test]
fn records_and_reads_routing_metrics() {
    let mut engine = EvaluationEngine::new();
    engine.record_routing(2, false);
    engine.record_routing(1, true);

    let routing = engine.get_routing_metrics();
    assert_eq!(routing.total_routings, 2);
    assert_eq!(routing.successful_routings, 1);
    assert_eq!(routing.fallback_count, 1);
}

#[test]
fn best_and_worst_experts_are_reported() {
    let mut engine = EvaluationEngine::new();
    let expert_a = ExpertId::new("a");
    let expert_b = ExpertId::new("b");

    engine.record_expert_execution(expert_a.clone(), true, 0.9, 10.0);
    engine.record_expert_execution(expert_a, true, 0.8, 12.0);
    engine.record_expert_execution(expert_b.clone(), false, 0.1, 20.0);
    engine.record_expert_execution(expert_b, true, 0.6, 18.0);

    let best = engine
        .best_performing_expert()
        .expect("best performing expert should exist");
    let worst = engine
        .worst_performing_expert()
        .expect("worst performing expert should exist");

    assert!(best.success_rate() >= worst.success_rate());
}

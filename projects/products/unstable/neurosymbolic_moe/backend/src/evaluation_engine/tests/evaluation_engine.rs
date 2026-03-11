use crate::evaluation_engine::{
    EvaluationEngine, EvaluationGovernanceReport, ExpertRegression, RoutingRegression,
};
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

#[test]
fn detects_expert_and_routing_regressions_against_baseline() {
    let expert = ExpertId::new("regression-expert");

    let mut baseline = EvaluationEngine::new();
    baseline.record_expert_execution(expert.clone(), true, 0.9, 10.0);
    baseline.record_expert_execution(expert.clone(), true, 0.8, 12.0);
    baseline.record_routing(1, false);
    baseline.record_routing(1, false);

    let mut current = EvaluationEngine::new();
    current.record_expert_execution(expert.clone(), true, 0.8, 10.0);
    current.record_expert_execution(expert.clone(), false, 0.2, 12.0);
    current.record_routing(1, false);
    current.record_routing(1, true);

    let expert_regressions: Vec<ExpertRegression> =
        current.detect_expert_regressions(&baseline, 0.2);
    assert_eq!(expert_regressions.len(), 1);
    assert_eq!(expert_regressions[0].expert_id, expert);
    assert!(expert_regressions[0].delta < 0.0);
    assert!(
        expert_regressions[0].previous_success_rate > expert_regressions[0].current_success_rate
    );

    let routing_regression: RoutingRegression = current
        .detect_routing_regression(&baseline, 0.2)
        .expect("routing regression should be detected");
    assert!(routing_regression.delta < 0.0);
    assert!(routing_regression.previous_accuracy > routing_regression.current_accuracy);
}

#[test]
fn governance_report_flags_underperforming_experts() {
    let mut engine = EvaluationEngine::new();
    let expert_good = ExpertId::new("good");
    let expert_bad = ExpertId::new("bad");

    engine.record_expert_execution(expert_good, true, 0.9, 10.0);
    engine.record_expert_execution(expert_bad.clone(), false, 0.3, 20.0);
    engine.record_routing(2, true);

    let report: EvaluationGovernanceReport = engine.governance_report(0.8, 0.9);
    assert!(!report.ready_for_promotion);
    assert!(report.routing_accuracy_below_threshold);
    assert_eq!(report.underperforming_experts, vec![expert_bad]);
    assert!((report.min_expert_success_rate - 0.8).abs() < f64::EPSILON);
    assert!((report.min_routing_accuracy - 0.9).abs() < f64::EPSILON);
}

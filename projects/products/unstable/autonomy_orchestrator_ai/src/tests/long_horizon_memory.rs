// Integration tests for long-horizon memory: read/write stability and memory signal flow.
use crate::decision_aggregator::{DecisionAggregatorConfig, aggregate};
use crate::domain::{
    DecisionContribution, DecisionReliabilityInput, DecisionReliabilityUpdate, FinalDecision,
    MemoryPolicy, RunReport, TerminalState,
};
use crate::long_horizon_memory::{
    LongHorizonMemoryStore, derive_reliability_inputs, enforce_policy, record_run,
};

fn make_contribution(id: &str, vote: FinalDecision, confidence: u8) -> DecisionContribution {
    DecisionContribution {
        contributor_id: id.to_string(),
        capability: "governance".to_string(),
        vote,
        confidence,
        weight: 100,
        reason_codes: Vec::new(),
        artifact_refs: Vec::new(),
    }
}

fn failed_report_with_update(run_id: &str, score: u8) -> RunReport {
    let mut report = RunReport::new(run_id.to_string());
    report.terminal_state = Some(TerminalState::Failed);
    report.blocked_reason_codes = vec!["GATE_CI_NOT_SUCCESS".to_string()];
    report.decision_reliability_updates = vec![DecisionReliabilityUpdate {
        contributor_id: "agent_x".to_string(),
        capability: "governance".to_string(),
        previous_score: 50,
        new_score: score,
        reason_code: "RELIABILITY_REWARD_ALIGNMENT".to_string(),
    }];
    report
}

#[test]
fn memory_read_write_is_stable_across_multiple_records() {
    let mut store = LongHorizonMemoryStore::default();
    let policy = MemoryPolicy::default();

    for i in 0..5u8 {
        let report = failed_report_with_update(&format!("run_{i}"), 60 + i);
        record_run(&mut store, &report, u64::from(i) * 100);
    }

    assert_eq!(store.entries.len(), 5);
    assert_eq!(store.next_run_index, 5);

    let (inputs1, codes1) = derive_reliability_inputs(&store, &policy);
    let (inputs2, codes2) = derive_reliability_inputs(&store, &policy);
    assert_eq!(inputs1, inputs2, "derive_reliability_inputs must be deterministic");
    assert_eq!(codes1, codes2);
    assert!(codes1.contains(&"MEMORY_SIGNAL_APPLIED".to_string()));

    // Average of scores 60..64 = (60+61+62+63+64)/5 = 62
    assert_eq!(inputs1.len(), 1);
    assert_eq!(inputs1[0].contributor_id, "agent_x");
    assert_eq!(inputs1[0].score, 62);
}

#[test]
fn memory_signal_influences_aggregator_decision() {
    let mut store = LongHorizonMemoryStore::default();

    // Record 3 runs where agent_x had high reliability (score 90)
    for i in 0..3 {
        let report = failed_report_with_update(&format!("run_{i}"), 90);
        record_run(&mut store, &report, i);
    }

    let policy = MemoryPolicy::default();
    let (memory_inputs, _) = derive_reliability_inputs(&store, &policy);
    assert_eq!(memory_inputs[0].score, 90);

    // agent_x votes Proceed at moderate confidence; agent_y votes Block at same confidence
    let contributions = vec![
        make_contribution("agent_x", FinalDecision::Proceed, 70),
        make_contribution("agent_y", FinalDecision::Block, 70),
    ];

    // Without memory: tie broken fail-closed -> Block
    let summary_no_memory = aggregate(
        &contributions,
        &DecisionAggregatorConfig {
            min_confidence_to_proceed: 50,
            reliability_inputs: Vec::new(),
            memory_reliability_inputs: Vec::new(),
        },
    );
    assert_eq!(summary_no_memory.final_decision, FinalDecision::Block);

    // With memory boosting agent_x's reliability: should Proceed
    let summary_with_memory = aggregate(
        &contributions,
        &DecisionAggregatorConfig {
            min_confidence_to_proceed: 50,
            reliability_inputs: Vec::new(),
            memory_reliability_inputs: memory_inputs,
        },
    );
    assert_eq!(summary_with_memory.final_decision, FinalDecision::Proceed);
    assert!(
        summary_with_memory
            .decision_rationale_codes
            .contains(&"MEMORY_SIGNAL_APPLIED".to_string()),
        "MEMORY_SIGNAL_APPLIED should be in rationale codes"
    );
}

#[test]
fn explicit_reliability_input_takes_precedence_over_memory() {
    let mut store = LongHorizonMemoryStore::default();

    // Memory gives agent_x score 90 (high)
    let report = failed_report_with_update("run_1", 90);
    record_run(&mut store, &report, 100);

    let policy = MemoryPolicy::default();
    let (memory_inputs, _) = derive_reliability_inputs(&store, &policy);

    // Explicit input overrides memory (low score for agent_x)
    let explicit_inputs = vec![DecisionReliabilityInput {
        contributor_id: "agent_x".to_string(),
        capability: "governance".to_string(),
        score: 10,
    }];

    let contributions = vec![
        make_contribution("agent_x", FinalDecision::Proceed, 80),
        make_contribution("agent_y", FinalDecision::Block, 80),
    ];

    let summary = aggregate(
        &contributions,
        &DecisionAggregatorConfig {
            min_confidence_to_proceed: 60,
            reliability_inputs: explicit_inputs,
            memory_reliability_inputs: memory_inputs,
        },
    );
    // agent_x has low explicit reliability -> Block wins
    assert_eq!(summary.final_decision, FinalDecision::Block);
    // Should use explicit reliability (DECISION_RELIABILITY_WEIGHTED), not memory
    assert!(
        summary
            .decision_rationale_codes
            .contains(&"DECISION_RELIABILITY_WEIGHTED".to_string())
    );
    assert!(
        !summary
            .decision_rationale_codes
            .contains(&"MEMORY_SIGNAL_APPLIED".to_string()),
        "MEMORY_SIGNAL_APPLIED should NOT be set when explicit input overrides"
    );
}

#[test]
fn memory_decay_limits_window_for_reliability_derivation() {
    let mut store = LongHorizonMemoryStore::default();

    // 10 old runs with low score
    for i in 0..10 {
        let report = failed_report_with_update(&format!("old_{i}"), 20);
        record_run(&mut store, &report, i);
    }
    // 3 new runs with high score
    for i in 0..3 {
        let report = failed_report_with_update(&format!("new_{i}"), 90);
        record_run(&mut store, &report, 1000 + i);
    }

    // With a narrow window (last 3 runs only)
    let narrow_policy = MemoryPolicy {
        max_entries: 500,
        decay_window_runs: 3,
    };
    let (narrow_inputs, _) = derive_reliability_inputs(&store, &narrow_policy);
    assert_eq!(narrow_inputs.len(), 1);
    assert_eq!(narrow_inputs[0].score, 90, "narrow window should only see high-score runs");

    // Wide window includes old low-score runs
    let wide_policy = MemoryPolicy {
        max_entries: 500,
        decay_window_runs: 1000,
    };
    let (wide_inputs, _) = derive_reliability_inputs(&store, &wide_policy);
    // (20*10 + 90*3) / 13 = (200 + 270) / 13 = 470 / 13 = 36
    assert_eq!(wide_inputs[0].score, 36, "wide window should average all runs");
}

#[test]
fn policy_enforcement_then_derive_produces_consistent_results() {
    let mut store = LongHorizonMemoryStore::default();

    for i in 0u64..20 {
        let report = failed_report_with_update(&format!("run_{i}"), 70);
        record_run(&mut store, &report, i);
    }

    let policy = MemoryPolicy {
        max_entries: 500,
        decay_window_runs: 5,
    };

    let codes = enforce_policy(&mut store, &policy, 9999);
    assert!(codes.contains(&"MEMORY_ENTRY_DECAYED".to_string()));
    assert_eq!(store.entries.len(), 5);

    let (inputs, _) = derive_reliability_inputs(&store, &policy);
    assert_eq!(inputs[0].score, 70);
}

use auto_manager_ai::{ActionPlan, RunReport};
use common_json::{to_string_pretty, from_str};

#[test]
fn test_action_plan_serialization() {
    let plan = ActionPlan::new("Test plan".to_string());
    
    // Serialize to JSON
    let json = to_string_pretty(&plan).expect("Failed to serialize action plan");
    
    // Verify it's valid JSON
    assert!(json.contains("version"));
    assert!(json.contains("generated_at"));
    assert!(json.contains("actions"));
    assert!(json.contains("summary"));
    
    // Deserialize back
    let _deserialized: ActionPlan = from_str(&json).expect("Failed to deserialize action plan");
}

#[test]
fn test_run_report_serialization() {
    let report = RunReport::new("test_run_123".to_string());
    
    // Serialize to JSON
    let json = to_string_pretty(&report).expect("Failed to serialize run report");
    
    // Verify it's valid JSON
    assert!(json.contains("product"));
    assert!(json.contains("auto_manager_ai"));
    assert!(json.contains("version"));
    assert!(json.contains("run_id"));
    assert!(json.contains("test_run_123"));
    assert!(json.contains("status"));
    assert!(json.contains("output"));
    assert!(json.contains("policy_decisions"));
    assert!(json.contains("errors"));
    
    // Deserialize back
    let _deserialized: RunReport = from_str(&json).expect("Failed to deserialize run report");
}

#[test]
fn test_action_plan_round_trip() {
    let plan = ActionPlan::new("Round trip test".to_string());
    let json = to_string_pretty(&plan).expect("Failed to serialize");
    let deserialized: ActionPlan = from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(plan.version, deserialized.version);
    assert_eq!(plan.summary, deserialized.summary);
    assert_eq!(plan.actions.len(), deserialized.actions.len());
}

#[test]
fn test_run_report_round_trip() {
    let report = RunReport::new("round_trip_test".to_string());
    let json = to_string_pretty(&report).expect("Failed to serialize");
    let deserialized: RunReport = from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(report.product, deserialized.product);
    assert_eq!(report.version, deserialized.version);
    assert_eq!(report.run_id, deserialized.run_id);
    assert_eq!(report.status, deserialized.status);
}


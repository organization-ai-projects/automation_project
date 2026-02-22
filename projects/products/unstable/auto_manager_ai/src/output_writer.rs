// projects/products/unstable/auto_manager_ai/src/output_writer.rs

use crate::artifact_contract::{validate_action_plan_contract, validate_run_report_contract};
use crate::domain::{ActionPlan, RunReport};
use common_json::to_string_pretty;
use std::fs;
use std::path::Path;

/// Write outputs to the output directory
pub fn write_outputs(plan: &ActionPlan, report: &RunReport, out_dir: &Path) -> Result<(), String> {
    validate_action_plan_contract(plan).map_err(|e| e.render())?;
    validate_run_report_contract(report).map_err(|e| e.render())?;

    // Create output directory if it doesn't exist
    fs::create_dir_all(out_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;

    // Write action plan
    let action_plan_path = out_dir.join("action_plan.json");
    let action_plan_json =
        to_string_pretty(plan).map_err(|e| format!("Failed to serialize action plan: {:?}", e))?;
    fs::write(&action_plan_path, action_plan_json)
        .map_err(|e| format!("Failed to write action plan: {}", e))?;

    // Write run report
    let run_report_path = out_dir.join("run_report.json");
    let run_report_json =
        to_string_pretty(report).map_err(|e| format!("Failed to serialize run report: {:?}", e))?;
    fs::write(&run_report_path, run_report_json)
        .map_err(|e| format!("Failed to write run report: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        artifact_contract::ArtifactContractErrorCode,
        domain::{ActionPlan, RunReport},
        output_writer::write_outputs,
        tests::test_helpers::create_unique_temp_dir,
    };

    #[test]
    fn test_write_outputs() {
        let temp_dir = create_unique_temp_dir("auto_manager_ai_output_test");
        let out_dir = temp_dir.join("out");

        let plan = ActionPlan::new("Test".to_string());
        let report = RunReport::new("test_run".to_string());

        let result = write_outputs(&plan, &report, &out_dir);
        assert!(result.is_ok());

        assert!(out_dir.join("action_plan.json").exists());
        assert!(out_dir.join("run_report.json").exists());

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_write_outputs_rejects_non_compliant_action_plan_contract() {
        let temp_dir = create_unique_temp_dir("auto_manager_ai_output_contract_plan");
        let out_dir = temp_dir.join("out");

        let mut plan = ActionPlan::new("Test".to_string());
        plan.schema_version = "2".to_string();
        let report = RunReport::new("test_run".to_string());

        let result = write_outputs(&plan, &report, &out_dir);
        assert!(
            result
                .as_ref()
                .err()
                .map(|e| e.contains(ArtifactContractErrorCode::ActionPlanSchemaVersion.as_str()))
                .unwrap_or(false),
            "unexpected result: {:?}",
            result
        );
        assert!(!out_dir.join("action_plan.json").exists());
        assert!(!out_dir.join("run_report.json").exists());

        fs::remove_dir_all(&temp_dir).ok();
    }

    #[test]
    fn test_write_outputs_rejects_non_compliant_run_report_contract() {
        let temp_dir = create_unique_temp_dir("auto_manager_ai_output_contract_report");
        let out_dir = temp_dir.join("out");

        let plan = ActionPlan::new("Test".to_string());
        let mut report = RunReport::new("test_run".to_string());
        report.producer = "other".to_string();

        let result = write_outputs(&plan, &report, &out_dir);
        assert!(
            result
                .as_ref()
                .err()
                .map(|e| e.contains(ArtifactContractErrorCode::RunReportProducer.as_str()))
                .unwrap_or(false),
            "unexpected result: {:?}",
            result
        );
        assert!(!out_dir.join("action_plan.json").exists());
        assert!(!out_dir.join("run_report.json").exists());

        fs::remove_dir_all(&temp_dir).ok();
    }
}

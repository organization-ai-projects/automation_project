// projects/products/unstable/auto_manager_ai/src/output_writer.rs

use std::path::Path;
use std::fs;
use common_json::to_string_pretty;
use crate::domain::{ActionPlan, RunReport};

/// Write outputs to the output directory
pub fn write_outputs(
    plan: &ActionPlan,
    report: &RunReport,
    out_dir: &Path,
) -> Result<(), String> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(out_dir)
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    // Write action plan
    let action_plan_path = out_dir.join("action_plan.json");
    let action_plan_json = to_string_pretty(plan)
        .map_err(|e| format!("Failed to serialize action plan: {:?}", e))?;
    fs::write(&action_plan_path, action_plan_json)
        .map_err(|e| format!("Failed to write action plan: {}", e))?;

    // Write run report
    let run_report_path = out_dir.join("run_report.json");
    let run_report_json = to_string_pretty(report)
        .map_err(|e| format!("Failed to serialize run report: {:?}", e))?;
    fs::write(&run_report_path, run_report_json)
        .map_err(|e| format!("Failed to write run report: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn test_write_outputs() {
        let temp_dir = std::env::temp_dir().join(format!("auto_manager_ai_output_test_{}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()));
        
        let out_dir = temp_dir.join("out");
        
        let plan = ActionPlan::new("Test".to_string());
        let report = RunReport::new("test_run".to_string());
        
        let result = write_outputs(&plan, &report, &out_dir);
        assert!(result.is_ok());
        
        assert!(out_dir.join("action_plan.json").exists());
        assert!(out_dir.join("run_report.json").exists());
        
        fs::remove_dir_all(&temp_dir).ok();
    }
}

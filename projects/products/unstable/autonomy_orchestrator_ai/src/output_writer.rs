// projects/products/unstable/autonomy_orchestrator_ai/src/output_writer.rs
use crate::domain::RunReport;
use common_json::to_string_pretty;
use std::fs;
use std::path::Path;

pub fn write_run_report(report: &RunReport, out_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(out_dir).map_err(|e| {
        format!(
            "Failed to create output directory '{}': {}",
            out_dir.display(),
            e
        )
    })?;

    let report_path = out_dir.join("orchestrator_run_report.json");
    let report_json =
        to_string_pretty(report).map_err(|e| format!("Failed to serialize run report: {e:?}"))?;

    fs::write(&report_path, report_json).map_err(|e| {
        format!(
            "Failed to write run report '{}': {}",
            report_path.display(),
            e
        )
    })
}

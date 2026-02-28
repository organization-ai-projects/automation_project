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

/// Write the escalation queue artifact when the report contains escalation cases.
/// No file is written when there are no cases (normal autonomous path).
pub fn write_escalation_queue(report: &RunReport, out_dir: &Path) -> Result<(), String> {
    if report.escalation_cases.is_empty() {
        return Ok(());
    }

    fs::create_dir_all(out_dir).map_err(|e| {
        format!(
            "Failed to create output directory '{}': {}",
            out_dir.display(),
            e
        )
    })?;

    let queue_path = out_dir.join("escalation_queue.json");
    let queue_json = to_string_pretty(&report.escalation_cases)
        .map_err(|e| format!("Failed to serialize escalation queue: {e:?}"))?;

    fs::write(&queue_path, queue_json).map_err(|e| {
        format!(
            "Failed to write escalation queue '{}': {}",
            queue_path.display(),
            e
        )
    })
}

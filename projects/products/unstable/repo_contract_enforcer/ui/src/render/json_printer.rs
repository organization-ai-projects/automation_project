use crate::transport::ipc_client::Report;

pub struct JsonPrinter;

impl JsonPrinter {
    pub fn print_report(report: &Report) -> anyhow::Result<()> {
        let txt = Self::render_report(report)?;
        println!("{txt}");
        Ok(())
    }

    pub fn render_report(report: &Report) -> anyhow::Result<String> {
        Ok(common_json::to_string(report)?)
    }
}

#[cfg(test)]
mod tests {
    use super::JsonPrinter;
    use crate::transport::ipc_client::{Report, ReportMode, ReportSummary};

    #[test]
    fn render_report_is_canonical_compact_json() {
        let report = Report {
            repository_root: ".".to_string(),
            mode: ReportMode::Auto,
            violations: Vec::new(),
            summary: ReportSummary {
                stable_error_count: 0,
                stable_warning_count: 1,
                unstable_error_count: 0,
                unstable_warning_count: 2,
            },
            report_hash: "h".to_string(),
        };
        let rendered = JsonPrinter::render_report(&report).expect("render report");
        assert!(rendered.contains("\"mode\":\"auto\""));
        assert!(rendered.contains("\"stable_warning_count\":1"));
    }
}

use crate::transport::ipc_client::Report;

pub struct HumanPrinter;

impl HumanPrinter {
    pub fn print_report(report: &Report) {
        println!("{}", Self::render_report(report));
    }

    pub fn render_report(report: &Report) -> String {
        format!(
            "repo_contract_enforcer summary:\nstable errors={}, stable warnings={}, unstable errors={}, unstable warnings={}",
            report.summary.stable_error_count,
            report.summary.stable_warning_count,
            report.summary.unstable_error_count,
            report.summary.unstable_warning_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::HumanPrinter;
    use crate::transport::ipc_client::{Report, ReportMode, ReportSummary};

    #[test]
    fn render_report_is_stable_for_fixed_summary() {
        let report = Report {
            repository_root: ".".to_string(),
            mode: ReportMode::Auto,
            violations: Vec::new(),
            summary: ReportSummary {
                stable_error_count: 1,
                stable_warning_count: 2,
                unstable_error_count: 3,
                unstable_warning_count: 4,
            },
            report_hash: "abc".to_string(),
        };
        let rendered = HumanPrinter::render_report(&report);
        assert_eq!(
            rendered,
            "repo_contract_enforcer summary:\nstable errors=1, stable warnings=2, unstable errors=3, unstable warnings=4"
        );
    }
}

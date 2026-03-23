// projects/products/unstable/repo_contract_enforcer/ui/src/render/vscode_printer.rs
use crate::transport::{
    self,
    ipc_client::{Report, Severity},
};

pub struct VscodePrinter;

impl VscodePrinter {
    pub fn print_report(report: &Report) {
        println!("{}", Self::render_report(report));
    }

    pub fn render_report(report: &Report) -> String {
        if report.violations.is_empty() {
            return "repo_contract_enforcer: no violations".to_string();
        }

        let mut out = String::new();
        for violation in &report.violations {
            let line = violation.line.unwrap_or(1);
            let severity = match violation.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
            };
            let scope = match violation.scope {
                transport::ipc_client::Scope::Stable => "stable",
                transport::ipc_client::Scope::Unstable => "unstable",
                transport::ipc_client::Scope::Tool => "tool",
                transport::ipc_client::Scope::Other => "other",
            };
            let msg = format!(
                "{} [{} / {}] {}",
                violation.message, violation.rule_id, violation.violation_code, scope
            );
            out.push_str(&format!(
                "{}:{}:1: {}: {}\n",
                violation.path, line, severity, msg
            ));
        }

        if out.ends_with('\n') {
            out.pop();
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::VscodePrinter;
    use crate::transport::ipc_client::{
        Report, ReportMode, ReportSummary, Scope, Severity, Violation,
    };

    #[test]
    fn render_report_formats_gcc_style_lines() {
        let report = Report {
            repository_root: ".".to_string(),
            mode: ReportMode::Auto,
            violations: vec![Violation {
                rule_id: "determinism".to_string(),
                violation_code: "DET_UNWRAP_RISK".to_string(),
                severity: Severity::Warning,
                scope: Scope::Stable,
                path: "backend/src/main.rs".to_string(),
                message: "unwrap usage may panic".to_string(),
                line: Some(12),
            }],
            summary: ReportSummary {
                stable_error_count: 0,
                stable_warning_count: 1,
                unstable_error_count: 0,
                unstable_warning_count: 0,
            },
            report_hash: "h".to_string(),
        };

        let rendered = VscodePrinter::render_report(&report);
        assert_eq!(
            rendered,
            "backend/src/main.rs:12:1: warning: unwrap usage may panic [determinism / DET_UNWRAP_RISK] stable"
        );
    }
}

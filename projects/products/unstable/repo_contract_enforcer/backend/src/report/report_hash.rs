use crate::report::report::Report;
use crate::report::violation::Violation;
use anyhow::Result;
use sha2::{Digest, Sha256};

pub struct ReportHash;

impl ReportHash {
    pub fn compute(report: &Report) -> Result<String> {
        let canonical = canonical_report_bytes(report);
        let digest = Sha256::digest(canonical);
        Ok(hex::encode(digest))
    }
}

fn canonical_report_bytes(report: &Report) -> Vec<u8> {
    let mut out = String::new();
    out.push_str("repository_root=");
    out.push_str(&report.repository_root);
    out.push('\n');
    out.push_str("mode=");
    out.push_str(match report.mode {
        crate::config::enforcement_mode::EnforcementMode::Auto => "auto",
        crate::config::enforcement_mode::EnforcementMode::Strict => "strict",
        crate::config::enforcement_mode::EnforcementMode::Relaxed => "relaxed",
    });
    out.push('\n');
    out.push_str("summary=");
    out.push_str(&format!(
        "{},{},{},{}",
        report.summary.stable_error_count,
        report.summary.stable_warning_count,
        report.summary.unstable_error_count,
        report.summary.unstable_warning_count
    ));
    out.push('\n');
    out.push_str("violations=\n");
    for violation in &report.violations {
        append_violation(&mut out, violation);
        out.push('\n');
    }
    out.into_bytes()
}

fn append_violation(out: &mut String, v: &Violation) {
    out.push_str(&format!(
        "{}|{}|{}|{}|{}|{}|{}",
        v.rule_id as u8,
        v.violation_code as u8,
        v.severity as u8,
        v.scope as u8,
        v.path,
        v.message,
        v.line
            .map_or_else(|| "-".to_string(), |line| line.to_string())
    ));
}

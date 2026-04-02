//! projects/products/unstable/autonomous_dev_ai/backend/src/ops/utils.rs
use crate::{lifecycle::RunReport, ops::OpsAlert};

pub(crate) fn build_ops_alerts(report: &RunReport) -> Vec<OpsAlert> {
    let mut alerts = Vec::new();

    if report.final_state != "Done" {
        alerts.push(OpsAlert {
            severity: "high".to_string(),
            code: "RUN_NOT_DONE".to_string(),
            message: format!("run ended in non-success state '{}'", report.final_state),
        });
    }
    if report.policy_violations_total > 0 {
        alerts.push(OpsAlert {
            severity: "high".to_string(),
            code: "POLICY_VIOLATIONS".to_string(),
            message: format!(
                "{} policy violation(s) detected in this run",
                report.policy_violations_total
            ),
        });
    }
    if report.authz_denials_total > 0 {
        alerts.push(OpsAlert {
            severity: "medium".to_string(),
            code: "AUTHZ_DENIALS".to_string(),
            message: format!(
                "{} authorization denial(s) detected in this run",
                report.authz_denials_total
            ),
        });
    }
    if report.risk_gate_denies > 0 {
        alerts.push(OpsAlert {
            severity: "medium".to_string(),
            code: "RISK_GATE_DENIES".to_string(),
            message: format!("{} risk gate denial(s) recorded", report.risk_gate_denies),
        });
    }
    if report.total_failures > 0 {
        alerts.push(OpsAlert {
            severity: "medium".to_string(),
            code: "RUN_FAILURES".to_string(),
            message: format!(
                "{} failure(s) recorded in run memory",
                report.total_failures
            ),
        });
    }

    alerts
}

pub(crate) fn render_ops_alerts_markdown(alerts: &[OpsAlert]) -> String {
    if alerts.is_empty() {
        return "- No active alerts.".to_string();
    }

    let mut lines = String::new();
    for alert in alerts {
        lines.push_str(&format!(
            "- [{}] `{}` {}\n",
            alert.severity, alert.code, alert.message
        ));
    }
    lines
}

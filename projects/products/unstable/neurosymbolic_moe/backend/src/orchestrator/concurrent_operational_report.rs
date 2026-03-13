//! projects/products/unstable/neurosymbolic_moe/backend/src/orchestrator/concurrent_operational_report.rs
use serde::{Deserialize, Serialize};

use crate::orchestrator::{ConcurrentLockMetrics, OperationalReport};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrentOperationalReport {
    pub pipeline: OperationalReport,
    pub lock_metrics: ConcurrentLockMetrics,
    pub lock_contention_rate: f64,
    pub lock_timeout_rate: f64,
    pub write_guard_rejections: u64,
}

impl ConcurrentOperationalReport {
    pub fn slo_violations(
        &self,
        max_lock_contention_rate: f64,
        max_lock_timeout_rate: f64,
        min_runtime_import_successes: u64,
        max_total_import_rejections: u64,
        max_json_parse_failures: u64,
    ) -> Vec<String> {
        let mut violations = self.pipeline.slo_violations(
            min_runtime_import_successes,
            max_total_import_rejections,
            max_json_parse_failures,
        );
        if self.lock_contention_rate > max_lock_contention_rate {
            violations.push(format!(
                "lock contention rate {:.6} above maximum {:.6}",
                self.lock_contention_rate, max_lock_contention_rate
            ));
        }
        if self.lock_timeout_rate > max_lock_timeout_rate {
            violations.push(format!(
                "lock timeout rate {:.6} above maximum {:.6}",
                self.lock_timeout_rate, max_lock_timeout_rate
            ));
        }
        violations
    }

    pub fn slo_status(
        &self,
        max_lock_contention_rate: f64,
        max_lock_timeout_rate: f64,
        min_runtime_import_successes: u64,
        max_total_import_rejections: u64,
        max_json_parse_failures: u64,
    ) -> &'static str {
        if self
            .slo_violations(
                max_lock_contention_rate,
                max_lock_timeout_rate,
                min_runtime_import_successes,
                max_total_import_rejections,
                max_json_parse_failures,
            )
            .is_empty()
        {
            "OK"
        } else {
            "FAIL"
        }
    }

    pub fn to_prometheus_text(&self, prefix: &str) -> String {
        let p = if prefix.is_empty() {
            "moe_concurrent_pipeline".to_string()
        } else {
            prefix.to_string()
        };
        let mut text = self.pipeline.to_prometheus_text(&format!("{p}_pipeline"));
        text.push_str(&format!(
            "{p}_read_lock_acquisitions {}\n{p}_write_lock_acquisitions {}\n{p}_read_lock_contention {}\n{p}_write_lock_contention {}\n{p}_read_lock_timeouts {}\n{p}_write_lock_timeouts {}\n{p}_lock_contention_rate {:.6}\n{p}_lock_timeout_rate {:.6}\n{p}_write_guard_rejections {}\n",
            self.lock_metrics.read_lock_acquisitions,
            self.lock_metrics.write_lock_acquisitions,
            self.lock_metrics.read_lock_contention,
            self.lock_metrics.write_lock_contention,
            self.lock_metrics.read_lock_timeouts,
            self.lock_metrics.write_lock_timeouts,
            self.lock_contention_rate,
            self.lock_timeout_rate,
            self.write_guard_rejections,
        ));
        text
    }
}

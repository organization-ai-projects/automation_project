//! projects/products/unstable/autonomous_dev_ai/src/tools/tool_metric_snapshot.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ToolMetricSnapshot {
    pub(crate) executions: usize,
    pub(crate) failures: usize,
    pub(crate) avg_duration_ms: u128,
    pub(crate) p95_duration_ms: u128,
    pub(crate) max_duration_ms: u128,
}

impl ToolMetricSnapshot {
    /// Vérifie si l'outil a été exécuté avec succès (aucun échec).
    pub fn is_successful(&self) -> bool {
        self.failures == 0
    }

    /// Calcule le ratio d'échecs par rapport aux exécutions.
    pub fn failure_ratio(&self) -> f64 {
        if self.executions == 0 {
            0.0
        } else {
            self.failures as f64 / self.executions as f64
        }
    }
}

impl ToolMetricSnapshot {
    /// Génère un tableau Markdown des métriques des outils.
    pub fn render_markdown(tool_metrics: &HashMap<String, ToolMetricSnapshot>) -> String {
        if tool_metrics.is_empty() {
            return "- No tool metrics recorded.".to_string();
        }

        let mut rows: Vec<(&String, &ToolMetricSnapshot)> = tool_metrics.iter().collect();
        rows.sort_by(|a, b| a.0.cmp(b.0));

        let mut out = String::from(
            "| Tool | Executions | Failures | Avg ms | P95 ms | Max ms |\n|---|---:|---:|---:|---:|---:|\n",
        );
        for (tool, stats) in rows {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} | {} |\n",
                tool,
                stats.executions,
                stats.failures,
                stats.avg_duration_ms,
                stats.p95_duration_ms,
                stats.max_duration_ms
            ));
        }

        out
    }
}

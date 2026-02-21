use serde::{Deserialize, Serialize};

/// Simple sliding-window drift detector based on rolling confidence averages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftDetector {
    window: Vec<f64>,
    window_size: usize,
    /// Alert when rolling average drops below this threshold.
    drift_threshold: f64,
}

impl DriftDetector {
    pub fn new(window_size: usize, drift_threshold: f64) -> Self {
        Self {
            window: Vec::with_capacity(window_size),
            window_size,
            drift_threshold,
        }
    }

    /// Record a new confidence observation; returns true if drift is detected.
    pub fn observe(&mut self, confidence: f64) -> bool {
        if self.window.len() >= self.window_size {
            self.window.remove(0);
        }
        self.window.push(confidence);

        if self.window.len() < self.window_size {
            // Not enough samples yet
            return false;
        }

        let avg = self.window.iter().sum::<f64>() / self.window.len() as f64;
        avg < self.drift_threshold
    }

    pub fn rolling_average(&self) -> Option<f64> {
        if self.window.is_empty() {
            None
        } else {
            Some(self.window.iter().sum::<f64>() / self.window.len() as f64)
        }
    }
}

impl Default for DriftDetector {
    fn default() -> Self {
        Self::new(10, 0.6)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeEntry {
    pub date: String,
    pub volume: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeHistory {
    pub ticker: String,
    pub entries: Vec<VolumeEntry>,
}

impl VolumeHistory {
    pub fn new(ticker: impl Into<String>, entries: Vec<VolumeEntry>) -> Self {
        Self {
            ticker: ticker.into(),
            entries,
        }
    }

    pub fn average_volume(&self, n: usize) -> Option<f64> {
        let recent: Vec<&VolumeEntry> = self.entries.iter().rev().take(n).collect();
        if recent.is_empty() {
            return None;
        }
        let sum: u64 = recent.iter().map(|e| e.volume).sum();
        Some(sum as f64 / recent.len() as f64)
    }
}

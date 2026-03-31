use serde::{Deserialize, Serialize};

use crate::domain::contradiction_entry::ContradictionEntry;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContradictionMemory {
    entries: Vec<ContradictionEntry>,
}

impl ContradictionMemory {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn append(&mut self, entry: ContradictionEntry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[ContradictionEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn canonical_string(&self) -> String {
        let parts: Vec<String> = self
            .entries
            .iter()
            .map(|e| {
                let violation_ids: Vec<String> =
                    e.violations.iter().map(|v| v.rule.to_string()).collect();
                format!(
                    "tick={},violations=[{}],corrections={}",
                    e.tick,
                    violation_ids.join(","),
                    e.corrections.len()
                )
            })
            .collect();
        parts.join(";")
    }
}

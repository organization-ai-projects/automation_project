use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum DslOp {
    ReplaceRange {
        file: String,
        start: usize,
        end: usize,
        text: String,
    },
    ReplaceFirst {
        file: String,
        pattern: String,
        text: String,
    },
    InsertAfter {
        file: String,
        pattern: String,
        text: String,
    },
    DeleteRange {
        file: String,
        start: usize,
        end: usize,
    },
}

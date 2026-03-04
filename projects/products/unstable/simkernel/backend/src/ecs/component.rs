use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Component {
    Label(String),
    Counter(i64),
    Flag(bool),
    Queue(Vec<u64>),
    Custom(String, common_json::Value),
}

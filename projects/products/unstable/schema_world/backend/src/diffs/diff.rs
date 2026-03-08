use common_json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diff {
    pub added: Vec<Json>,
    pub removed: Vec<Json>,
    pub updated: Vec<Json>,
}

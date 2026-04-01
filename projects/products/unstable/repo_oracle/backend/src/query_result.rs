use serde::{Deserialize, Serialize};

use crate::query::Query;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub query: Query,
    pub matches: Vec<String>,
}

use serde::{Deserialize, Serialize};

use crate::model::company_id::CompanyId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyReport {
    pub company_id: CompanyId,
    pub name: String,
    pub final_budget: i64,
    pub store_count: u64,
}

use serde::{Deserialize, Serialize};

use crate::model::company_id::CompanyId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    id: CompanyId,
    name: String,
    budget: i64,
}

impl Company {
    pub fn new(id: CompanyId, name: String, budget: i64) -> Self {
        Self { id, name, budget }
    }

    pub fn id(&self) -> CompanyId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn budget(&self) -> i64 {
        self.budget
    }

    pub fn adjust_budget(&mut self, amount: i64) {
        self.budget += amount;
    }
}

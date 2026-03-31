use serde::{Deserialize, Serialize};

use crate::model::company_id::CompanyId;
use crate::model::store_id::StoreId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    id: StoreId,
    owner: CompanyId,
    name: String,
}

impl Store {
    pub fn new(id: StoreId, owner: CompanyId, name: String) -> Self {
        Self { id, owner, name }
    }

    pub fn id(&self) -> StoreId {
        self.id
    }

    pub fn owner(&self) -> CompanyId {
        self.owner
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

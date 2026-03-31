use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SupplierId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Supplier {
    id: SupplierId,
    name: String,
}

impl Supplier {
    pub fn new(id: SupplierId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> SupplierId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for SupplierId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "supplier-{}", self.0)
    }
}

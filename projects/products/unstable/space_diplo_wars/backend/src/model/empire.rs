use serde::{Deserialize, Serialize};

use crate::economy::resource_wallet::ResourceWallet;
use crate::model::empire_id::EmpireId;
use crate::tech::tech_tree::TechTree;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empire {
    pub id: EmpireId,
    pub name: String,
    pub home_system: String,
    pub resources: ResourceWallet,
    pub tech_tree: TechTree,
}

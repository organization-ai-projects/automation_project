use crate::diagnostics::SpaceEmpireError;
use crate::economy::ResourceKind;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceWallet {
    pub balances: BTreeMap<ResourceKind, u64>,
}

impl ResourceWallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn get(&self, kind: ResourceKind) -> u64 {
        *self.balances.get(&kind).unwrap_or(&0)
    }

    pub fn add(&mut self, kind: ResourceKind, amount: u64) {
        *self.balances.entry(kind).or_insert(0) += amount;
    }

    pub fn try_spend(&mut self, kind: ResourceKind, amount: u64) -> Result<(), SpaceEmpireError> {
        let balance = self.get(kind);
        if balance < amount {
            return Err(SpaceEmpireError::InsufficientResources(format!(
                "Need {amount} {:?}, have {balance}",
                kind
            )));
        }
        *self.balances.entry(kind).or_insert(0) -= amount;
        Ok(())
    }

    pub fn can_afford(&self, costs: &BTreeMap<ResourceKind, u64>) -> bool {
        costs
            .iter()
            .all(|(kind, &amount)| self.get(*kind) >= amount)
    }

    pub fn spend_all(
        &mut self,
        costs: &BTreeMap<ResourceKind, u64>,
    ) -> Result<(), SpaceEmpireError> {
        if !self.can_afford(costs) {
            return Err(SpaceEmpireError::InsufficientResources(
                "Cannot afford costs".to_string(),
            ));
        }
        for (&kind, &amount) in costs {
            self.try_spend(kind, amount)?;
        }
        Ok(())
    }
}

impl Default for ResourceWallet {
    fn default() -> Self {
        Self::new()
    }
}

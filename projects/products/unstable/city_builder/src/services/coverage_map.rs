use crate::map::TileId;
use crate::services::ServiceKind;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoverageMap {
    pub covered: BTreeMap<ServiceKind, BTreeSet<TileId>>,
}

impl CoverageMap {
    pub fn new() -> Self {
        Self {
            covered: BTreeMap::new(),
        }
    }

    pub fn is_covered(&self, kind: ServiceKind, tile: &TileId) -> bool {
        self.covered
            .get(&kind)
            .map(|s| s.contains(tile))
            .unwrap_or(false)
    }
}

impl Default for CoverageMap {
    fn default() -> Self {
        Self::new()
    }
}

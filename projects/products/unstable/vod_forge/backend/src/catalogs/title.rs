// projects/products/unstable/vod_forge/backend/src/catalogs/title.rs
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::catalogs::{Season, TitleId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: TitleId,
    pub name: String,
    pub year: u16,
    pub seasons: BTreeMap<u32, Season>,
}

// projects/products/unstable/vod_forge/backend/src/catalog/catalog.rs
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::catalogs::{Title, TitleId};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Catalog {
    pub titles: BTreeMap<TitleId, Title>,
}

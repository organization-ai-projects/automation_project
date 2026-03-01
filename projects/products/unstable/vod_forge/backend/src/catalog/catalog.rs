use crate::catalog::title::Title;
use crate::catalog::title_id::TitleId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Catalog {
    pub titles: BTreeMap<TitleId, Title>,
}

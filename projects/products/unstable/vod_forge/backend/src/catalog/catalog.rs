use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::catalog::title_id::TitleId;
use crate::catalog::title::Title;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Catalog {
    pub titles: BTreeMap<TitleId, Title>,
}

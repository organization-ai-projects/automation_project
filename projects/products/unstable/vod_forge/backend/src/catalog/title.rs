use crate::catalog::season::Season;
use crate::catalog::title_id::TitleId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: TitleId,
    pub name: String,
    pub year: u16,
    pub seasons: BTreeMap<u32, Season>,
}

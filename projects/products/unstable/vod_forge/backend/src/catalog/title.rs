use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::catalog::title_id::TitleId;
use crate::catalog::season::Season;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub id: TitleId,
    pub name: String,
    pub year: u16,
    pub seasons: BTreeMap<u32, Season>,
}

use crate::map::cell_id::CellId;
use crate::model::colonist_id::ColonistId;
use crate::model::item_id::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaulTask {
    pub item_id: ItemId,
    pub from: CellId,
    pub to: CellId,
    pub assigned_to: Option<ColonistId>,
}

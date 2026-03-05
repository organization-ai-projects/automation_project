use crate::map::cell_id::CellId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub id: CellId,
    pub passable: bool,
    pub resource: Option<String>,
}

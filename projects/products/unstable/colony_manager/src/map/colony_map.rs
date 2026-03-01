use crate::map::cell::Cell;
use crate::map::cell_id::CellId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColonyMap {
    pub cells: BTreeMap<CellId, Cell>,
    pub width: u32,
    pub height: u32,
}

impl ColonyMap {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = BTreeMap::new();
        for y in 0..height {
            for x in 0..width {
                let id = CellId(y * width + x);
                cells.insert(id, Cell { id, passable: true, resource: None });
            }
        }
        Self { cells, width, height }
    }
}

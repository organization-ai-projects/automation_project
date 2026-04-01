use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellCoord {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpatialGrid {
    pub cell_size: f64,
    pub cells: HashMap<CellCoord, Vec<u64>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f64) -> Self {
        Self {
            cell_size: if cell_size > 0.0 { cell_size } else { 1.0 },
            cells: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn cell_for(&self, x: f64, y: f64, z: f64) -> CellCoord {
        CellCoord {
            x: (x / self.cell_size).floor() as i64,
            y: (y / self.cell_size).floor() as i64,
            z: (z / self.cell_size).floor() as i64,
        }
    }

    pub fn insert(&mut self, id: u64, x: f64, y: f64, z: f64) {
        let coord = self.cell_for(x, y, z);
        self.cells.entry(coord).or_default().push(id);
    }

    pub fn neighbors(&self, coord: &CellCoord) -> Vec<u64> {
        let mut result = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let neighbor = CellCoord {
                        x: coord.x + dx,
                        y: coord.y + dy,
                        z: coord.z + dz,
                    };
                    if let Some(ids) = self.cells.get(&neighbor) {
                        result.extend_from_slice(ids);
                    }
                }
            }
        }
        result
    }
}

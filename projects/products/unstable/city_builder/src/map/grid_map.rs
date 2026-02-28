use super::{Tile, TileId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GridMap {
    pub width: u32,
    pub height: u32,
    pub tiles: BTreeMap<TileId, Tile>,
}

impl GridMap {
    pub fn new(width: u32, height: u32) -> Self {
        let mut tiles = BTreeMap::new();
        for y in 0..height {
            for x in 0..width {
                let id = TileId { x, y };
                tiles.insert(id, Tile::new(id));
            }
        }
        Self { width, height, tiles }
    }

    pub fn get(&self, id: &TileId) -> Option<&Tile> {
        self.tiles.get(id)
    }

    pub fn get_mut(&mut self, id: &TileId) -> Option<&mut Tile> {
        self.tiles.get_mut(id)
    }

    pub fn neighbors(&self, id: &TileId) -> Vec<TileId> {
        let mut result = Vec::new();
        let x = id.x;
        let y = id.y;
        if x > 0 { result.push(TileId { x: x - 1, y }); }
        if y > 0 { result.push(TileId { x, y: y - 1 }); }
        if x + 1 < self.width { result.push(TileId { x: x + 1, y }); }
        if y + 1 < self.height { result.push(TileId { x, y: y + 1 }); }
        result.sort();
        result
    }
}

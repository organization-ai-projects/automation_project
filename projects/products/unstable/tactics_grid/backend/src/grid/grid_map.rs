use super::position::Position;
use super::tile::Tile;
use std::collections::BTreeMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GridMap {
    pub width: u32,
    pub height: u32,
    tiles: BTreeMap<(i32, i32), Tile>,
}

impl GridMap {
    pub fn new(width: u32, height: u32) -> Self {
        let mut tiles = BTreeMap::new();
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                tiles.insert((x, y), Tile::Floor);
            }
        }
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn tile_at(&self, pos: &Position) -> Option<&Tile> {
        self.tiles.get(&(pos.x, pos.y))
    }

    pub fn set_tile(&mut self, pos: &Position, tile: Tile) {
        self.tiles.insert((pos.x, pos.y), tile);
    }

    pub fn in_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width as i32 && pos.y < self.height as i32
    }

    pub fn is_walkable(&self, pos: &Position) -> bool {
        self.tile_at(pos).is_some_and(|t| t.is_walkable())
    }
}

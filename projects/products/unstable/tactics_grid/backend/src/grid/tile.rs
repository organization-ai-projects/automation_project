#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    pub fn is_walkable(&self) -> bool {
        matches!(self, Tile::Floor)
    }
}

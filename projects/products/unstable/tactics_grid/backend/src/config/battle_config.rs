#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BattleConfig {
    pub max_turns: u32,
    pub grid_width: u32,
    pub grid_height: u32,
}

impl Default for BattleConfig {
    fn default() -> Self {
        Self {
            max_turns: 50,
            grid_width: 8,
            grid_height: 8,
        }
    }
}

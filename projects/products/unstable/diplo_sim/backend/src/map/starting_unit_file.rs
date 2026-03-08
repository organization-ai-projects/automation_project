use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct StartingUnitFile {
    pub faction_id: u32,
    pub territory_id: u32,
}

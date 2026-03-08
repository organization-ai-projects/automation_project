use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TerritoryFile {
    pub id: u32,
    pub name: String,
}

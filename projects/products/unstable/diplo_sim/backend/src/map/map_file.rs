use super::starting_unit_file::StartingUnitFile;
use super::territory_file::TerritoryFile;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MapFile {
    pub name: String,
    pub version: String,
    pub territories: Vec<TerritoryFile>,
    pub adjacencies: Vec<[u32; 2]>,
    pub starting_units: Vec<StartingUnitFile>,
}

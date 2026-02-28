use super::map_graph::MapGraph;
use super::territory::Territory;
use super::territory_id::TerritoryId;
use crate::diagnostics::error::DiploSimError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MapFile {
    pub name: String,
    pub version: String,
    pub territories: Vec<TerritoryFile>,
    pub adjacencies: Vec<[u32; 2]>,
    pub starting_units: Vec<StartingUnitFile>,
}

#[derive(Debug, Deserialize)]
pub struct TerritoryFile {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StartingUnitFile {
    pub faction_id: u32,
    pub territory_id: u32,
}

pub fn load_map_from_str(json: &str) -> Result<(MapGraph, Vec<StartingUnitFile>), DiploSimError> {
    let map_file: MapFile = common_json::from_str(json)
        .map_err(|e| DiploSimError::MapValidation(format!("JSON parse error: {e}")))?;

    validate_map_file(&map_file)?;

    let territories: Vec<Territory> = map_file
        .territories
        .iter()
        .map(|t| Territory {
            id: TerritoryId(t.id),
            name: t.name.clone(),
        })
        .collect();

    let adjacencies: Vec<[TerritoryId; 2]> = map_file
        .adjacencies
        .iter()
        .map(|a| [TerritoryId(a[0]), TerritoryId(a[1])])
        .collect();

    let graph = MapGraph {
        name: map_file.name,
        version: map_file.version,
        territories,
        adjacencies,
    };

    Ok((graph, map_file.starting_units))
}

pub fn load_map_from_file(path: &str) -> Result<(MapGraph, Vec<StartingUnitFile>), DiploSimError> {
    let json = std::fs::read_to_string(path)
        .map_err(|e| DiploSimError::Io(format!("Cannot read map file '{}': {}", path, e)))?;
    load_map_from_str(&json)
}

fn validate_map_file(map_file: &MapFile) -> Result<(), DiploSimError> {
    if map_file.territories.is_empty() {
        return Err(DiploSimError::MapValidation(
            "Map has no territories".to_string(),
        ));
    }
    let ids: std::collections::HashSet<u32> = map_file.territories.iter().map(|t| t.id).collect();
    if ids.len() != map_file.territories.len() {
        return Err(DiploSimError::MapValidation(
            "Duplicate territory IDs".to_string(),
        ));
    }
    for adj in &map_file.adjacencies {
        if !ids.contains(&adj[0]) || !ids.contains(&adj[1]) {
            return Err(DiploSimError::MapValidation(format!(
                "Adjacency references unknown territory: [{}, {}]",
                adj[0], adj[1]
            )));
        }
        if adj[0] == adj[1] {
            return Err(DiploSimError::MapValidation(format!(
                "Self-adjacency for territory {}",
                adj[0]
            )));
        }
    }
    for su in &map_file.starting_units {
        if !ids.contains(&su.territory_id) {
            return Err(DiploSimError::MapValidation(format!(
                "Starting unit references unknown territory {}",
                su.territory_id
            )));
        }
    }
    Ok(())
}

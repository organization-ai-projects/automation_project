use crate::data::move_data::MoveData;
use crate::data::move_id::MoveId;
use crate::data::species::Species;
use crate::data::species_id::SpeciesId;
use crate::data::type_chart::TypeChart;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct DataStore {
    pub species: BTreeMap<String, Species>,
    pub moves: BTreeMap<String, MoveData>,
    pub type_chart: TypeChart,
}

impl DataStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_species(&mut self, species: Species) {
        self.species.insert(species.id.0.clone(), species);
    }

    pub fn add_move(&mut self, move_data: MoveData) {
        self.moves.insert(move_data.id.0.clone(), move_data);
    }

    pub fn get_species(&self, id: &SpeciesId) -> Option<&Species> {
        self.species.get(&id.0)
    }

    pub fn get_move(&self, id: &MoveId) -> Option<&MoveData> {
        self.moves.get(&id.0)
    }
}

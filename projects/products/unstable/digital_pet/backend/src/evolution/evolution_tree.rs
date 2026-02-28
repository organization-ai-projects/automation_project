// projects/products/unstable/digital_pet/backend/src/evolution/evolution_tree.rs
use crate::config::sim_config::SimConfig;
use crate::evolution::evolution_node::EvolutionNode;
use crate::evolution::evolution_rule::EvolutionRule;
use crate::model::pet_species::PetSpecies;
use crate::model::pet_species_id::PetSpeciesId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTree {
    pub root: EvolutionNode,
    pub rules: Vec<EvolutionRule>,
}

impl EvolutionTree {
    pub fn from_config(_config: &SimConfig) -> Self {
        let baby = PetSpecies {
            id: PetSpeciesId::new("baby"),
            name: "BabyMon".to_string(),
            base_attack: 5, base_defense: 5, base_hp: 20,
        };
        let child_good = PetSpecies {
            id: PetSpeciesId::new("child_good"),
            name: "GoodMon".to_string(),
            base_attack: 15, base_defense: 12, base_hp: 40,
        };
        let child_bad = PetSpecies {
            id: PetSpeciesId::new("child_bad"),
            name: "BadMon".to_string(),
            base_attack: 10, base_defense: 8, base_hp: 30,
        };
        let adult = PetSpecies {
            id: PetSpeciesId::new("adult"),
            name: "AdultMon".to_string(),
            base_attack: 25, base_defense: 20, base_hp: 60,
        };
        let root = EvolutionNode::new(PetSpecies::egg())
            .with_children(vec![
                EvolutionNode::new(baby.clone()).with_children(vec![
                    EvolutionNode::new(child_good.clone()).with_children(vec![EvolutionNode::new(adult.clone())]),
                    EvolutionNode::new(child_bad.clone()),
                ]),
            ]);
        let rules = vec![
            EvolutionRule { from_species: "egg".into(), to_species: "baby".into(), min_ticks: 5, max_care_mistakes: 10, min_happiness: 0, min_discipline: 0 },
            EvolutionRule { from_species: "baby".into(), to_species: "child_good".into(), min_ticks: 20, max_care_mistakes: 2, min_happiness: 60, min_discipline: 60 },
            EvolutionRule { from_species: "baby".into(), to_species: "child_bad".into(), min_ticks: 20, max_care_mistakes: 0, min_happiness: 0, min_discipline: 0 },
            EvolutionRule { from_species: "child_good".into(), to_species: "adult".into(), min_ticks: 50, max_care_mistakes: 3, min_happiness: 50, min_discipline: 50 },
        ];
        Self { root, rules }
    }

    pub fn find_evolution(&self, species_id: &str, mistakes: usize, ticks: u64, happiness: u32, discipline: u32) -> Option<PetSpecies> {
        for rule in &self.rules {
            if rule.from_species == species_id
                && ticks >= rule.min_ticks
                && mistakes <= rule.max_care_mistakes
                && happiness >= rule.min_happiness
                && discipline >= rule.min_discipline
            {
                return self.find_species_in_tree(&self.root, &rule.to_species);
            }
        }
        None
    }

    fn find_species_in_tree<'a>(&'a self, node: &'a EvolutionNode, id: &str) -> Option<PetSpecies> {
        if node.species.id.0 == id {
            return Some(node.species.clone());
        }
        for child in &node.children {
            if let Some(s) = self.find_species_in_tree(child, id) {
                return Some(s);
            }
        }
        None
    }
}

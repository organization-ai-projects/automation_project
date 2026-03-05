use crate::determinism::deterministic_order::DeterministicOrder;
use crate::ecs::world::World;
use crate::inspect::query::Query;
use crate::inspect::query_report::QueryReport;

pub struct QueryEngine;

impl QueryEngine {
    pub fn execute(world: &World, query: &Query) -> QueryReport {
        let entity_count = world.entity_count();
        let component_count = world.component_count();
        let mut entities = world.entities_with_components();
        DeterministicOrder::sort_by_key(&mut entities, |entity| entity.id);
        let richest_entity = entities
            .iter()
            .max_by_key(|entity| entity.components.len())
            .map(|entity| format!("{}:{}", entity.id.0, entity.components.len()))
            .unwrap_or_else(|| "none".to_string());
        let mut entity_ids = world.entities_sorted();
        DeterministicOrder::sort_entities(&mut entity_ids);
        QueryReport {
            kind: query.kind.clone(),
            results: vec![
                format!("entity_count={}", entity_count),
                format!("component_count={}", component_count),
                format!("richest_entity={}", richest_entity),
                format!("sorted_entities={}", entity_ids.len()),
            ],
        }
    }
}

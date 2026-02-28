#![allow(dead_code)]
use crate::ecs::world::World;
use crate::inspect::query::Query;
use crate::inspect::query_report::QueryReport;

pub struct QueryEngine;

impl QueryEngine {
    pub fn execute(world: &World, query: &Query) -> QueryReport {
        let entity_count = world.entity_count();
        QueryReport {
            kind: query.kind.clone(),
            results: vec![format!("entity_count={}", entity_count)],
        }
    }
}

#![allow(dead_code)]
use crate::ecs::entity_id::EntityId;

pub struct DeterministicOrder;

impl DeterministicOrder {
    pub fn sort_entities(entities: &mut [EntityId]) {
        entities.sort_unstable();
    }

    pub fn sort_by_key<T, K: Ord>(items: &mut [T], key: impl Fn(&T) -> K) {
        items.sort_by_key(key);
    }
}

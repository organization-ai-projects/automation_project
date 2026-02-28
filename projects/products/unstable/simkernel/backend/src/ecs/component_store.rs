#![allow(dead_code)]
use crate::ecs::component::Component;
use crate::ecs::component_id::ComponentId;
use crate::ecs::entity_id::EntityId;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct ComponentStore {
    data: BTreeMap<(EntityId, ComponentId), Component>,
}

impl ComponentStore {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, entity: EntityId, component: ComponentId, value: Component) {
        self.data.insert((entity, component), value);
    }

    pub fn get(&self, entity: EntityId, component: ComponentId) -> Option<&Component> {
        self.data.get(&(entity, component))
    }

    pub fn get_mut(&mut self, entity: EntityId, component: ComponentId) -> Option<&mut Component> {
        self.data.get_mut(&(entity, component))
    }

    pub fn remove(&mut self, entity: EntityId, component: ComponentId) {
        self.data.remove(&(entity, component));
    }

    pub fn iter_entity(&self, entity: EntityId) -> impl Iterator<Item = (ComponentId, &Component)> {
        self.data.range((entity, ComponentId(0))..(EntityId(entity.0 + 1), ComponentId(0)))
            .map(|((_, cid), c)| (*cid, c))
    }

    pub fn len(&self) -> usize { self.data.len() }
}

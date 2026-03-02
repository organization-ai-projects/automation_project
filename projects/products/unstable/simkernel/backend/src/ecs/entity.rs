#![allow(dead_code)]
use crate::ecs::component::Component;
use crate::ecs::component_id::ComponentId;
use crate::ecs::entity_id::EntityId;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityId,
    pub components: Vec<(ComponentId, Component)>,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            components: Vec::new(),
        }
    }
}

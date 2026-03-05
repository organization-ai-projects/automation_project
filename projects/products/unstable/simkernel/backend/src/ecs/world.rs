use crate::ecs::component::Component;
use crate::ecs::component_id::ComponentId;
use crate::ecs::component_store::ComponentStore;
use crate::ecs::entity::Entity;
use crate::ecs::entity_id::EntityId;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Default)]
pub struct World {
    next_entity_id: u64,
    entities: BTreeSet<EntityId>,
    components: ComponentStore,
}

impl World {
    pub fn new() -> Self {
        Self {
            next_entity_id: 0,
            entities: BTreeSet::new(),
            components: ComponentStore::new(),
        }
    }

    pub fn spawn(&mut self) -> EntityId {
        let id = EntityId::new(self.next_entity_id);
        self.next_entity_id += 1;
        self.entities.insert(id);
        id
    }

    pub fn despawn(&mut self, id: EntityId) {
        self.entities.remove(&id);
    }

    pub fn insert_component(&mut self, entity: EntityId, cid: ComponentId, value: Component) {
        self.components.insert(entity, cid, value);
    }

    pub fn get_component(&self, entity: EntityId, cid: ComponentId) -> Option<&Component> {
        self.components.get(entity, cid)
    }

    pub fn get_component_mut(
        &mut self,
        entity: EntityId,
        cid: ComponentId,
    ) -> Option<&mut Component> {
        self.components.get_mut(entity, cid)
    }

    pub fn remove_component(&mut self, entity: EntityId, cid: ComponentId) {
        self.components.remove(entity, cid);
    }

    pub fn entities_sorted(&self) -> Vec<EntityId> {
        let mut v: Vec<EntityId> = self.entities.iter().copied().collect();
        v.sort_unstable();
        v
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    pub fn entities_with_components(&self) -> Vec<Entity> {
        self.entities_sorted()
            .into_iter()
            .map(|id| {
                let mut entity = Entity::new(id);
                entity.components = self
                    .components
                    .iter_entity(id)
                    .map(|(cid, component)| (cid, component.clone()))
                    .collect();
                entity
            })
            .collect()
    }
}

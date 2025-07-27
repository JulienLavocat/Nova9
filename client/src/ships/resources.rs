use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Resource, Debug, Default)]
pub struct ShipsRegistry {
    pub registry: HashMap<u64, Entity>,
}
impl ShipsRegistry {
    pub fn register(&mut self, ship_id: u64, entity: Entity) {
        self.registry.insert(ship_id, entity);
    }

    pub fn get(&self, ship_id: u64) -> Option<Entity> {
        self.registry.get(&ship_id).cloned()
    }

    pub fn remove(&mut self, ship_id: u64) {
        self.registry.remove(&ship_id);
    }
}

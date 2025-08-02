use bevy::{platform::collections::HashMap, prelude::*};
use spacetimedb_sdk::Identity;

#[derive(Component, Debug)]
pub struct ShipData {
    entity: Entity,
    pilot_id: Option<Identity>,
}

impl ShipData {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            pilot_id: None,
        }
    }

    pub fn set_pilot(&mut self, pilot_id: Identity) {
        self.pilot_id = Some(pilot_id);
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn pilot_id(&self) -> Option<Identity> {
        self.pilot_id
    }
}

#[derive(Resource, Debug, Default)]
pub struct ShipsRegistry {
    pub registry: HashMap<u64, ShipData>,
}

impl ShipsRegistry {
    pub fn register(&mut self, ship_id: u64, entity: Entity) {
        self.registry.insert(ship_id, ShipData::new(entity));
    }

    pub fn get(&self, ship_id: u64) -> Option<&ShipData> {
        self.registry.get(&ship_id)
    }

    pub fn get_mut(&mut self, ship_id: u64) -> Option<&mut ShipData> {
        self.registry.get_mut(&ship_id)
    }

    pub fn remove(&mut self, ship_id: u64) {
        self.registry.remove(&ship_id);
    }
}

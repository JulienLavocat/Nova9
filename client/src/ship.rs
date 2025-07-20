use bevy::{platform::collections::HashMap, prelude::*};
use bevy_spacetimedb::ReadInsertEvent;

use crate::{assets_loader::ModelAssets, bindings::Ship as ShipTable, materials::GameMaterial};

#[derive(Component, Debug)]
pub struct Ship;

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

pub struct ShipsPlugin;

impl Plugin for ShipsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShipsRegistry>()
            .add_systems(PreUpdate, (spawn_ship, despawn_ship).chain());
    }
}

fn spawn_ship(
    mut commands: Commands,
    mut events: ReadInsertEvent<ShipTable>,
    mut ships: ResMut<ShipsRegistry>,
    models_assets: Res<ModelAssets>,
) {
    for event in events.read() {
        let ship = &event.row;

        debug!("Spawning ship: {:?}", ship);

        let model = match ship.ship_type_id {
            1 => &models_assets.ship_bomber_01,
            _ => panic!("Unknown ship type: {}", ship.ship_type_id),
        };

        let entity = commands
            .spawn((
                Ship,
                SceneRoot(model.clone()),
                Transform::from_xyz(ship.x, ship.y, ship.z),
                GameMaterial::Ship,
            ))
            .id();

        ships.register(ship.id, entity);
    }
}

fn despawn_ship(
    mut commands: Commands,
    mut events: ReadInsertEvent<ShipTable>,
    mut ships: ResMut<ShipsRegistry>,
) {
    for event in events.read() {
        let ship = &event.row;
        debug!("Despawning ship: {:?}", ship);
        if let Some(entity) = ships.get(ship.id) {
            commands.entity(entity).despawn();
            ships.remove(ship.id);
        }
    }
}

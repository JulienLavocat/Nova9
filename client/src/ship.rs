use bevy::{platform::collections::HashMap, prelude::*};
use bevy_spacetimedb::{DeleteEvent, InsertEvent, ReadDeleteEvent, ReadInsertEvent};

use crate::{
    assets_loader::ModelAssets,
    bindings::{Ship as ShipTable, ShipPilot},
    materials::GameMaterial,
    spacetimedb::SpacetimeDB,
};

#[derive(Component, Debug)]
pub struct Ship {
    ship_id: u64,
}

#[derive(Component, Debug)]
pub struct ControlledShip;

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
            .add_systems(PreUpdate, (spawn_ship, despawn_ship).chain())
            .add_systems(
                PreUpdate,
                (on_ship_pilot_inserted, on_ship_pilot_removed).chain(),
            );
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
                Name::new(format!("Ship {}", ship.id)),
                Ship { ship_id: ship.id },
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
    mut events: ReadDeleteEvent<ShipTable>,
    mut ships: ResMut<ShipsRegistry>,
) {
    for event in events.read() {
        let ship = &event.row;
        debug!("Despawning ship: {:?}", ship);
        if let Some(entity) = ships.get(ship.id) {
            commands.entity(entity).despawn();
            ships.remove(ship.id);
        } else {
            warn!("Ship with ID {} not found for despawn", ship.id);
            commands.send_event(DeleteEvent { row: ship.clone() });
        }
    }
}

fn on_ship_pilot_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<ShipPilot>,
    ships: Res<ShipsRegistry>,
    stdb: SpacetimeDB,
) {
    for event in events.read() {
        // We don't care about pilots that are not us
        if event.row.player_id != stdb.identity() {
            continue;
        }

        let ship = &event.row;

        if let Some(ship_entity) = ships.get(ship.ship_id) {
            debug!("Assigning pilot to ship: {:?}", ship);
            commands.entity(ship_entity).insert(ControlledShip);
        } else {
            warn!(
                "Ship with ID {} not found for pilot assignment",
                ship.ship_id
            );
            commands.send_event(InsertEvent { row: ship.clone() });
        }
    }
}

fn on_ship_pilot_removed(
    mut commands: Commands,
    mut events: ReadDeleteEvent<ShipPilot>,
    ships: Res<ShipsRegistry>,
    stdb: SpacetimeDB,
) {
    for event in events.read() {
        // We don't care about pilots that are not us
        if event.row.player_id != stdb.identity() {
            continue;
        }

        let ship = &event.row;

        if let Some(ship_entity) = ships.get(ship.ship_id) {
            debug!("Removing pilot from ship: {:?}", ship);
            commands.entity(ship_entity).remove::<ControlledShip>();
        } else {
            warn!("Ship with ID {} not found for pilot removal", ship.ship_id);
        }
    }
}

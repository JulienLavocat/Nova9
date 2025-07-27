use bevy::prelude::*;
use bevy_spacetimedb::{DeleteEvent, ReadDeleteEvent, ReadInsertEvent};

use crate::{
    assets_loader::ModelAssets, bindings::Ship as ShipTable, materials::GameMaterial,
    ships::components::Ship,
};

use super::resources::ShipsRegistry;

pub struct ShipsLifecyclePlugin;

impl Plugin for ShipsLifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (spawn_ship, despawn_ship).chain());
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

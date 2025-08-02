use bevy::prelude::*;
use bevy_spacetimedb::{InsertEvent, ReadInsertEvent, ReadUpdateEvent, UpdateEvent};
use spacetimedb_sdk::Identity;

use crate::{bindings::ShipLocation, spacetimedb::SpacetimeDB};

use super::{components::Ship, resources::ShipsRegistry};

pub struct ShipLocationUpdatesPlugin;

impl Plugin for ShipLocationUpdatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_ship_location_inserted, on_ship_location_updated),
        );
    }
}

fn on_ship_location_inserted(
    mut events: ReadInsertEvent<ShipLocation>,
    mut commands: Commands,
    mut ship_transforms: Query<&mut Transform, With<Ship>>,
    ship_registry: Res<ShipsRegistry>,
) {
    for event in events.read() {
        let ship_location = &event.row;

        debug!("Received ship location update: {:?}", ship_location);

        if let Some(ship) = ship_registry.get(ship_location.ship_id) {
            debug!("{ship:?}");
            if let Ok(mut transform) = ship_transforms.get_mut(ship.entity()) {
                *transform = Transform::from_xyz(ship_location.x, ship_location.y, ship_location.z)
                    .with_rotation(Quat::from_xyzw(
                        ship_location.rot_x,
                        ship_location.rot_y,
                        ship_location.rot_z,
                        ship_location.rot_w,
                    ));
            } else {
                warn!(
                    "Failed to update transform for ship ID {}: Entity not found",
                    ship_location.ship_id
                );
            }
        } else {
            warn!(
                "Ship[{}] not found in registry for location update",
                ship_location.ship_id
            );

            commands.send_event(InsertEvent {
                row: ship_location.clone(),
            });
        }
    }
}

fn on_ship_location_updated(
    mut events: ReadUpdateEvent<ShipLocation>,
    mut commands: Commands,
    mut ship_transforms: Query<&mut Transform, With<Ship>>,
    ship_registry: Res<ShipsRegistry>,
    stdb: SpacetimeDB,
) {
    for event in events.read() {
        let ship_location = &event.new;

        if let Some(ship) = ship_registry.get(ship_location.ship_id) {
            // A bug might exists here where the pilot_id is not yet set (if this event is received
            // before the pilot is assigned).
            let pilot_id = ship.pilot_id().unwrap_or(Identity::ZERO);
            if pilot_id == stdb.identity() {
                continue;
            }

            if let Ok(mut transform) = ship_transforms.get_mut(ship.entity()) {
                *transform = Transform::from_xyz(ship_location.x, ship_location.y, ship_location.z)
                    .with_rotation(Quat::from_xyzw(
                        ship_location.rot_x,
                        ship_location.rot_y,
                        ship_location.rot_z,
                        ship_location.rot_w,
                    ));
            } else {
                warn!(
                    "Failed to update transform for ship ID {}: Entity not found",
                    ship_location.ship_id
                );
            }
        } else {
            warn!(
                "Ship[{}] not found in registry for location update",
                ship_location.ship_id
            );

            commands.send_event(UpdateEvent {
                old: event.old.clone(),
                new: event.new.clone(),
            });
        }
    }
}

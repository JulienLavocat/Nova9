use bevy::prelude::*;
use bevy_spacetimedb::{InsertUpdateEvent, ReadInsertUpdateEvent};

use crate::bindings::ShipLocation;

use super::{components::Ship, resources::ShipsRegistry};

pub struct ShipLocationUpdatesPlugin;

impl Plugin for ShipLocationUpdatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, on_ship_location_received);
    }
}

fn on_ship_location_received(
    mut events: ReadInsertUpdateEvent<ShipLocation>,
    mut commands: Commands,
    mut ship_transforms: Query<&mut Transform, With<Ship>>,
    ship_registry: Res<ShipsRegistry>,
) {
    for event in events.read() {
        let ship_location = &event.new;

        debug!("Received ship location update: {:?}", ship_location);

        if let Some(ship) = ship_registry.get(ship_location.ship_id) {
            if let Ok(mut transform) = ship_transforms.get_mut(ship) {
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

            commands.send_event(InsertUpdateEvent {
                old: event.old.clone(),
                new: event.new.clone(),
            });
        }
    }
}

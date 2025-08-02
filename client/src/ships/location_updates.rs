use bevy::prelude::*;
use bevy_spacetimedb::{InsertEvent, ReadInsertEvent, ReadUpdateEvent, UpdateEvent};
use spacetimedb_sdk::Identity;

use crate::{bindings::ShipLocation, spacetimedb::SpacetimeDB};

use super::{components::ControlledShip, resources::ShipsRegistry};

#[derive(Component, Debug)]
struct TargetShipLocation {
    pos: Vec3,
    rot: Quat,
}

pub struct ShipLocationUpdatesPlugin;

impl Plugin for ShipLocationUpdatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (on_ship_location_inserted, on_ship_location_updated),
        )
        .add_systems(PostUpdate, update_target_ship_locations);
    }
}

fn on_ship_location_inserted(
    mut events: ReadInsertEvent<ShipLocation>,
    mut commands: Commands,
    ship_registry: Res<ShipsRegistry>,
) {
    for event in events.read() {
        let ship_location = &event.row;

        if let Some(ship) = ship_registry.get(ship_location.ship_id) {
            commands.entity(ship.entity()).insert((
                TargetShipLocation {
                    pos: Vec3::new(ship_location.x, ship_location.y, ship_location.z),
                    rot: Quat::from_xyzw(
                        ship_location.rot_x,
                        ship_location.rot_y,
                        ship_location.rot_z,
                        ship_location.rot_w,
                    ),
                },
                Transform::from_translation(Vec3::new(
                    ship_location.x,
                    ship_location.y,
                    ship_location.z,
                ))
                .with_rotation(Quat::from_xyzw(
                    ship_location.rot_x,
                    ship_location.rot_y,
                    ship_location.rot_z,
                    ship_location.rot_w,
                )),
            ));
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
    mut ships: Query<&mut TargetShipLocation>,
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

            if let Ok(mut target_location) = ships.get_mut(ship.entity()) {
                target_location.pos = Vec3::new(ship_location.x, ship_location.y, ship_location.z);
                target_location.rot = Quat::from_xyzw(
                    ship_location.rot_x,
                    ship_location.rot_y,
                    ship_location.rot_z,
                    ship_location.rot_w,
                );
            } else {
                warn!(
                    "TargetShipLocation component not found for ship[{}]",
                    ship.entity()
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

fn update_target_ship_locations(
    mut ships: Query<(&mut Transform, &TargetShipLocation), Without<ControlledShip>>,
) {
    for (mut transform, target_location) in ships.iter_mut() {
        transform.translation = transform.translation.lerp(target_location.pos, 0.1);
        transform.rotation = transform.rotation.slerp(target_location.rot, 0.1);
    }
}

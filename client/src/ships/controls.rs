use bevy::prelude::*;
use bevy_enhanced_input::{
    action::Action,
    actions,
    prelude::{Actions, Bindings, Cardinal, InputAction},
};
use bevy_spacetimedb::{InsertEvent, ReadDeleteEvent, ReadInsertEvent};

use crate::{
    GameState,
    bindings::{ShipPilot, ShipTypesTableAccess, ShipsTableAccess},
    player::PlayerCamera,
    ships::components::ControlledShip,
    spacetimedb::SpacetimeDB,
};

use super::resources::ShipsRegistry;

#[derive(Component)]
pub struct OnPiloting;

#[derive(InputAction)]
#[action_output(Vec2)]
struct Move;

pub struct ShipControlsPlugin;

impl Plugin for ShipControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (on_ship_pilot_inserted, on_ship_pilot_removed)
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Update, apply_ship_controls)
        .add_systems(PostUpdate, debug_camera_rotation);
    }
}

fn on_ship_pilot_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<ShipPilot>,
    camera_transform: Single<Entity, With<PlayerCamera>>,
    ships: Res<ShipsRegistry>,
    stdb: SpacetimeDB,
) {
    let camera_entity = camera_transform.into_inner();

    for event in events.read() {
        // We don't care about pilots that are not us
        if event.row.player_id != stdb.identity() {
            continue;
        }

        let ship = &event.row;

        if let Some(ship_entity) = ships.get(ship.ship_id) {
            debug!("Assigning pilot to ship: {:?}", ship);
            commands.entity(ship_entity).insert((
                ControlledShip,
                OnPiloting,
                actions!(
                    OnPiloting[(
                        Action::<Move>::new(),
                        Bindings::spawn(Cardinal::wasd_keys())
                    )]
                ),
            ));

            let ship = stdb.db().ships().id().find(&ship.ship_id).unwrap();
            let ship_data = stdb
                .db()
                .ship_types()
                .id()
                .find(&ship.ship_type_id)
                .unwrap();

            commands.entity(camera_entity).insert((
                ChildOf(ship_entity),
                Transform::from_xyz(
                    ship_data.camera_offset_x,
                    ship_data.camera_offset_y,
                    ship_data.camera_offset_z,
                )
                .with_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    ship_data.camera_rotation_x,
                    ship_data.camera_rotation_y,
                    ship_data.camera_rotation_z,
                )),
            ));
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
            // We don't remove the relationship between the camera and the ship, as when we go back to being on foot,
            // the camera will be reattached to the player entity.
            commands
                .entity(ship_entity)
                .remove::<ControlledShip>()
                .remove_with_requires::<OnPiloting>()
                .despawn_related::<Actions<OnPiloting>>();
        } else {
            warn!("Ship with ID {} not found for pilot removal", ship.ship_id);
        }
    }
}

fn apply_ship_controls(
    move_action: Single<&Action<Move>>,
    mut ships: Single<&mut Transform, With<ControlledShip>>,
) {
    ships.translation += move_action.extend(0.0);
}

fn debug_camera_rotation(camera_transform: Single<&Transform, With<PlayerCamera>>) {
    debug!(
        "Camera pos: {} | Quat: {}",
        camera_transform.translation, camera_transform.rotation,
    );
}

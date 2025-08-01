use avian3d::prelude::{AngularVelocity, ExternalForce, ExternalTorque, LinearVelocity, RigidBody};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui::Window};
use bevy_spacetimedb::{InsertEvent, ReadDeleteEvent, ReadInsertEvent};

use crate::{
    GameState,
    bindings::{ShipPilot, ShipTypesTableAccess, ShipsTableAccess},
    player::PlayerCamera,
    ships::components::ControlledShip,
    spacetimedb::SpacetimeDB,
};

use super::{components::Ship, resources::ShipsRegistry};

#[derive(Component, Debug, Default, Reflect)]
pub struct FlightControls {
    pub thrust: f32,
    pub up_down: f32,
    pub strafe: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Component)]
pub struct OnPiloting;

#[derive(InputAction)]
#[action_output(f32)]
struct Thrust;

#[derive(InputAction)]
#[action_output(f32)]
struct Strafe;

#[derive(InputAction)]
#[action_output(f32)]
struct UpDown;

#[derive(InputAction)]
#[action_output(f32)]
struct Roll;

#[derive(InputAction)]
#[action_output(Vec2)]
struct PitchYaw;

pub struct ShipControlsPlugin;

impl Plugin for ShipControlsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FlightControls>()
            .add_input_context::<OnPiloting>()
            .add_systems(
                PreUpdate,
                (on_ship_pilot_inserted, on_ship_pilot_removed)
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (apply_inputs, apply_movement, debug_controls).chain(),
            );
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
                FlightControls::default(),
                OnPiloting,
                RigidBody::Dynamic,
                actions!(
                    OnPiloting[
                    (
                        Action::<Thrust>::new(),
                        Bindings::spawn(Bidirectional {
                            positive: Binding::from(KeyCode::KeyW),
                            negative: Binding::from(KeyCode::KeyS),
                        }
                    )
                    ),
                    (
                        Action::<Strafe>::new(),
                        Bindings::spawn(Bidirectional {
                            positive: Binding::from(KeyCode::KeyD),
                            negative: Binding::from(KeyCode::KeyA),
                        })
                    ),
                    (
                        Action::<UpDown>::new(),
                        Bindings::spawn(Bidirectional {
                            positive: Binding::from(KeyCode::Space),
                            negative: Binding::from(KeyCode::ControlLeft),
                        })
                    ),
                    (
                        Action::<Roll>::new(),
                        Negate::all(),
                        Bindings::spawn(Bidirectional {
                            positive: Binding::from(KeyCode::KeyE),
                            negative: Binding::from(KeyCode::KeyQ),
                        })
                    ),
                    (
                        Action::<PitchYaw>::new(),
                        Negate::all(),
                        bindings![(Binding::mouse_motion())]
                    ),
                ]),
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
                ),
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
                .remove::<RigidBody>()
                .remove_with_requires::<OnPiloting>()
                .despawn_related::<Actions<OnPiloting>>();
        } else {
            warn!("Ship with ID {} not found for pilot removal", ship.ship_id);
        }
    }
}

fn apply_inputs(
    thrust_action: Single<&ActionValue, With<Action<Thrust>>>,
    strafe_action: Single<&ActionValue, With<Action<Strafe>>>,
    up_down_action: Single<&ActionValue, With<Action<UpDown>>>,
    roll_action: Single<&ActionValue, With<Action<Roll>>>,
    pitch_yaw_action: Single<&ActionValue, With<Action<PitchYaw>>>,
    mut flight_controls: Single<&mut FlightControls, With<ControlledShip>>,
) -> Result {
    let pitch_yaw_action = pitch_yaw_action.as_axis2d();

    flight_controls.thrust = thrust_action.as_axis1d().clamp(-1.0, 1.0);
    flight_controls.strafe = strafe_action.as_axis1d().clamp(-1.0, 1.0);
    flight_controls.up_down = up_down_action.as_axis1d().clamp(-1.0, 1.0);
    flight_controls.roll = roll_action.as_axis1d().clamp(-1.0, 1.0);
    flight_controls.pitch = pitch_yaw_action.y.clamp(-1.0, 1.0);
    flight_controls.yaw = pitch_yaw_action.x.clamp(-1.0, 1.0);

    Ok(())
}

fn apply_movement(
    query: Single<
        (
            &mut ExternalTorque,
            &mut ExternalForce,
            &Transform,
            &FlightControls,
            &Ship,
        ),
        With<ControlledShip>,
    >,
    stdb: SpacetimeDB,
    time: Res<Time>,
) -> Result {
    // Main source: https://www.youtube.com/watch?v=fZvJvZA4nhY

    let (mut external_torque, mut external_force, transform, flight_controls, ship) =
        query.into_inner();
    let ship_data = stdb.db().ship_types().id().find(&ship.ship_type).unwrap();

    let roll_torque =
        transform.back() * flight_controls.roll * ship_data.roll_torque * time.delta_secs();
    external_torque.apply_torque(roll_torque);

    let pitch_torque =
        transform.right() * flight_controls.pitch * ship_data.pitch_torque * time.delta_secs();
    external_torque.apply_torque(pitch_torque);

    let yaw_torque =
        transform.up() * flight_controls.yaw * ship_data.yaw_torque * time.delta_secs();
    external_torque.apply_torque(yaw_torque);

    let thrust_force =
        transform.forward() * flight_controls.thrust * ship_data.thrust * time.delta_secs();
    external_force.apply_force(thrust_force);

    Ok(())
}

fn debug_controls(
    flight_controls: Single<
        (
            &FlightControls,
            &ExternalTorque,
            &AngularVelocity,
            &LinearVelocity,
        ),
        With<ControlledShip>,
    >,
    mut egui_context: EguiContexts,
) -> Result {
    let (flight_controls, external_torque, angular_velocity, linear_velocity) =
        flight_controls.into_inner();
    Window::new("Flight Controls").show(egui_context.ctx_mut()?, |ui| {
        ui.label(format!("Thrust: {}", flight_controls.thrust));
        ui.label(format!("Strafe: {}", flight_controls.strafe));
        ui.label(format!("Up/Down: {}", flight_controls.up_down));
        ui.label(format!("Roll: {}", flight_controls.roll));
        ui.label(format!("Pitch: {}", flight_controls.pitch));
        ui.label(format!("Yaw: {}", flight_controls.yaw));
    });

    Window::new("Ship Movement").show(egui_context.ctx_mut()?, |ui| {
        ui.label(format!(
            "External Torque: ({:.2}, {:.2}, {:.2})",
            external_torque.x, external_torque.y, external_torque.z
        ));
        ui.label(format!(
            "Angular Velocity: ({:.2}, {:.2}, {:.2}) -> {:.2} rad/s",
            angular_velocity.x,
            angular_velocity.y,
            angular_velocity.z,
            angular_velocity.length()
        ));
        ui.label(format!(
            "Linear Velocity: ({:.2}, {:.2}, {:.2}) -> {:.2} m/s",
            linear_velocity.x,
            linear_velocity.y,
            linear_velocity.z,
            linear_velocity.length()
        ));
    });

    Ok(())
}

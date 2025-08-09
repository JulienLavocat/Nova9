use avian3d::prelude::{ExternalForce, ExternalTorque, RigidBody};
use bevy::{
    prelude::*,
    window::{CursorGrabMode, Window},
};
use bevy_enhanced_input::prelude::*;
use bevy_spacetimedb::{InsertEvent, ReadDeleteEvent, ReadInsertEvent};

use crate::{
    GameState,
    bindings::{
        ShipPilot, ShipTableAccess, ShipTypeTableAccess, player_leave_ship, player_move_ship,
    },
    local_player::PlayerCamera,
    ships::components::ControlledShip,
    spacetimedb::SpacetimeDB,
};

use super::{components::Ship, resources::ShipsRegistry};

#[derive(Component, Debug, Default, Reflect)]
struct FlightControls {
    pub thrust: f32,
    pub vertical_thrust: f32,
    pub lateral_thrust: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Component, Debug)]
struct ShipLocationUpdate {
    timer: Timer,
    last_position: Vec3,
    last_rotation: Quat,
    position_threshold: f32,
    rotation_threshold: f32,
}

#[derive(Component)]
struct OnPiloting;

#[derive(InputAction)]
#[action_output(f32)]
struct Thrust;

#[derive(InputAction)]
#[action_output(f32)]
struct LateralThrust;

#[derive(InputAction)]
#[action_output(f32)]
struct VerticalThrust;

#[derive(InputAction)]
#[action_output(f32)]
struct Roll;

#[derive(InputAction)]
#[action_output(Vec2)]
struct PitchYaw;

#[derive(InputAction)]
#[action_output(bool)]
struct ToggleCaptureCursor;

#[derive(InputAction)]
#[action_output(bool)]
struct ExitShip;

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
            .add_systems(Update, (apply_inputs, apply_movement).chain())
            .add_systems(PostUpdate, send_location_updates)
            .add_observer(capture_cursor)
            .add_observer(exit_ship);
    }
}

fn on_ship_pilot_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<ShipPilot>,
    mut ships: ResMut<ShipsRegistry>,
    mut window: Single<&mut Window>,
    camera_transform: Single<Entity, With<PlayerCamera>>,
    stdb: SpacetimeDB,
) {
    let camera_entity = camera_transform.into_inner();

    for event in events.read() {
        let ship = &event.row;

        if let Some(ship_details) = ships.get_mut(ship.ship_id) {
            ship_details.set_pilot(ship.player_id);

            // We don't care about pilots that are not us for the moment
            if event.row.player_id != stdb.identity() {
                continue;
            }

            debug!("Assigning pilot to ship: {:?}", ship);

            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;

            commands.entity(ship_details.entity()).insert((
                ControlledShip,
                FlightControls::default(),
                OnPiloting,
                RigidBody::Dynamic,
                ShipLocationUpdate {
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    last_position: Vec3::ZERO,
                    last_rotation: Quat::IDENTITY,
                    position_threshold: 0.1,
                    rotation_threshold: 0.01,
                },
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
                        Action::<LateralThrust>::new(),
                        Negate::all(),
                        Bindings::spawn(Bidirectional {
                            positive: Binding::from(KeyCode::KeyQ),
                            negative: Binding::from(KeyCode::KeyE),
                        })
                    ),
                    (
                        Action::<VerticalThrust>::new(),
                        Bindings::spawn(Bidirectional {
                            positive: Binding::from(KeyCode::Space),
                            negative: Binding::from(KeyCode::ControlLeft),
                        })
                    ),
                    (
                        Action::<Roll>::new(),
                        Bindings::spawn(Bidirectional {
                            positive: Binding::from(KeyCode::KeyA),
                            negative: Binding::from(KeyCode::KeyD),
                        })
                    ),
                    (
                        Action::<PitchYaw>::new(),
                        Negate::all(),
                        bindings![(Binding::mouse_motion())]
                    ),
                    (
                        Action::<ToggleCaptureCursor>::new(),
                        Hold::new(0.2),
                        bindings![KeyCode::Escape]
                    ),
                    (
                        Action::<ExitShip>::new(),
                        Hold::new(0.5),
                        bindings![KeyCode::KeyF]
                    ),
                ]),
            ));

            let ship = stdb.db().ship().id().find(&ship.ship_id).unwrap();
            let ship_data = stdb.db().ship_type().id().find(&ship.ship_type_id).unwrap();

            commands.entity(camera_entity).insert((
                ChildOf(ship_details.entity()),
                Transform::from_xyz(
                    ship_data.camera_offset_x,
                    ship_data.camera_offset_y,
                    ship_data.camera_offset_z,
                ),
            ));
        } else {
            warn!("Ship[{}] not found for pilot assignment", ship.ship_id);
            commands.send_event(InsertEvent { row: ship.clone() });
        }
    }
}

fn on_ship_pilot_removed(
    mut commands: Commands,
    mut events: ReadDeleteEvent<ShipPilot>,
    ships: Res<ShipsRegistry>,
    stdb: SpacetimeDB,
    camera_transform: Single<Entity, With<PlayerCamera>>,
) {
    for event in events.read() {
        // We don't care about pilots that are not us
        if event.row.player_id != stdb.identity() {
            continue;
        }

        let ship = &event.row;

        if let Some(ship_data) = ships.get(ship.ship_id) {
            debug!("Removing pilot from ship: {:?}", ship);
            // We don't remove the relationship between the camera and the ship, as when we go back to being on foot,
            // the camera will be reattached to the player entity.
            commands
                .entity(ship_data.entity())
                .remove::<ControlledShip>()
                .remove::<RigidBody>()
                .remove::<ShipLocationUpdate>()
                .remove_with_requires::<OnPiloting>()
                .despawn_related::<Actions<OnPiloting>>();

            commands
                .entity(camera_transform.entity())
                .remove::<ChildOf>();
        } else {
            warn!("Ship[{}] not found for pilot removal", ship.ship_id);
        }
    }
}

fn apply_inputs(
    thrust_action: Single<&ActionValue, With<Action<Thrust>>>,
    lateral_thrust_action: Single<&ActionValue, With<Action<LateralThrust>>>,
    vertical_thrust_action: Single<&ActionValue, With<Action<VerticalThrust>>>,
    roll_action: Single<&ActionValue, With<Action<Roll>>>,
    pitch_yaw_action: Single<&ActionValue, With<Action<PitchYaw>>>,
    mut flight_controls: Single<&mut FlightControls, With<ControlledShip>>,
) -> Result {
    let pitch_yaw_action = pitch_yaw_action.as_axis2d();

    flight_controls.thrust = thrust_action.as_axis1d().clamp(-1.0, 1.0);
    flight_controls.lateral_thrust = lateral_thrust_action.as_axis1d().clamp(-1.0, 1.0);
    flight_controls.vertical_thrust = vertical_thrust_action.as_axis1d().clamp(-1.0, 1.0);
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
    window: Single<&Window>,
    stdb: SpacetimeDB,
    time: Res<Time>,
) -> Result {
    // Main source: https://www.youtube.com/watch?v=fZvJvZA4nhY

    if window.cursor_options.grab_mode == CursorGrabMode::None {
        return Ok(());
    }

    let (mut external_torque, mut external_force, transform, flight_controls, ship) =
        query.into_inner();
    let ship_data = stdb.db().ship_type().id().find(&ship.ship_type).unwrap();

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

    let vertical_thrust_force = transform.up()
        * flight_controls.vertical_thrust
        * ship_data.vertical_thrust
        * time.delta_secs();
    external_force.apply_force(vertical_thrust_force);

    let lateral_thrust_force = transform.right()
        * flight_controls.lateral_thrust
        * ship_data.lateral_thrust
        * time.delta_secs();
    external_force.apply_force(lateral_thrust_force);

    Ok(())
}

fn send_location_updates(
    ship: Single<(&Transform, &mut ShipLocationUpdate), With<ControlledShip>>,
    time: Res<Time>,
    stdb: SpacetimeDB,
) -> Result {
    let (ship_transform, mut update) = ship.into_inner();
    if !update.timer.tick(time.delta()).just_finished() {
        return Ok(());
    }

    let pos = ship_transform.translation;
    let rot = ship_transform.rotation;

    // If the position is greater than thresold or rotation has changed, we send an update
    let pos_diff = update.last_position.distance(pos);
    let rot_diff = update.last_rotation.angle_between(rot);
    if pos_diff < update.position_threshold && rot_diff < update.rotation_threshold {
        return Ok(());
    }

    stdb.reducers()
        .player_move_ship(pos.x, pos.y, pos.z, rot.x, rot.y, rot.z, rot.w)?;

    update.last_position = pos;
    update.last_rotation = rot;

    Ok(())
}

fn capture_cursor(
    _trigger: Trigger<Completed<ToggleCaptureCursor>>,
    mut window: Single<&mut Window>,
) {
    let grab = window.cursor_options.grab_mode == CursorGrabMode::None;
    window.cursor_options.grab_mode = if grab {
        CursorGrabMode::Confined
    } else {
        CursorGrabMode::None
    };
    window.cursor_options.visible = !grab;
}

fn exit_ship(_trigger: Trigger<Completed<ExitShip>>, stdb: SpacetimeDB) {
    stdb.reducers().player_leave_ship().unwrap();
}

// fn debug_controls(
//     flight_controls: Single<
//         (
//             &FlightControls,
//             &ExternalTorque,
//             &AngularVelocity,
//             &LinearVelocity,
//         ),
//         With<ControlledShip>,
//     >,
//     mut egui_context: EguiContexts,
// ) -> Result {
//     // let (flight_controls, external_torque, angular_velocity, linear_velocity) =
//     //     flight_controls.into_inner();
//     // EguiWindow::new("Flight Controls").show(egui_context.ctx_mut()?, |ui| {
//     //     ui.label(format!("Thrust: {}", flight_controls.thrust));
//     //     ui.label(format!("Strafe: {}", flight_controls.lateral_thrust));
//     //     ui.label(format!("Up/Down: {}", flight_controls.vertical_thrust));
//     //     ui.label(format!("Roll: {}", flight_controls.roll));
//     //     ui.label(format!("Pitch: {}", flight_controls.pitch));
//     //     ui.label(format!("Yaw: {}", flight_controls.yaw));
//     // });
//     //
//     // EguiWindow::new("Ship Movement").show(egui_context.ctx_mut()?, |ui| {
//     //     ui.label(format!(
//     //         "External Torque: ({:.2}, {:.2}, {:.2})",
//     //         external_torque.x, external_torque.y, external_torque.z
//     //     ));
//     //     ui.label(format!(
//     //         "Angular Velocity: ({:.2}, {:.2}, {:.2}) -> {:.2} rad/s",
//     //         angular_velocity.x,
//     //         angular_velocity.y,
//     //         angular_velocity.z,
//     //         angular_velocity.length()
//     //     ));
//     //     ui.label(format!(
//     //         "Linear Velocity: ({:.2}, {:.2}, {:.2}) -> {:.2} m/s",
//     //         linear_velocity.x,
//     //         linear_velocity.y,
//     //         linear_velocity.z,
//     //         linear_velocity.length()
//     //     ));
//     // });
//     //
//     Ok(())
// }

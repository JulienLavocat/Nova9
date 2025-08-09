use bevy::{prelude::*, window::CursorGrabMode};
use bevy_enhanced_input::prelude::*;
use log::debug;

use crate::GameState;

use super::LocalPlayerState;
#[derive(InputAction)]
#[action_output(Vec2)]
struct Move;

#[derive(InputAction)]
#[action_output(Vec2)]
struct Rotate;

#[derive(InputAction)]
#[action_output(f32)]
struct UpDown;

#[derive(InputAction)]
#[action_output(bool)]
struct Run;

#[derive(InputAction)]
#[action_output(bool)]
struct ToggleCaptureCursor;

#[derive(Component)]
pub struct PlayerFlyCam;

pub struct LocalPlayerFlycamPlugin;

impl Plugin for LocalPlayerFlycamPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<PlayerFlyCam>()
            .add_systems(
                PreUpdate,
                (on_flycam_added, on_flycam_removed)
                    .run_if(in_state(GameState::InGame))
                    .chain(),
            )
            .add_systems(
                Update,
                apply_movement.run_if(in_state(LocalPlayerState::OnFoot)),
            )
            .add_observer(rotate)
            .add_observer(capture_cursor);
    }
}

fn on_flycam_added(
    mut commands: Commands,
    mut window: Single<&mut Window>,
    query: Query<Entity, Added<PlayerFlyCam>>,
) {
    for event in query {
        debug!("Adding flycam to player: {event:?}");
        grab_cursor(&mut window, true);
        commands.entity(event).insert(actions!(
        PlayerFlyCam[(
            Action::<Move>::new(),
            Bindings::spawn(Cardinal::wasd_keys()),
        ),
        (
            Action::<Rotate>::new(),
            Negate::all(),
            Bindings::spawn(Spawn(Binding::mouse_motion()))
        ),
        (
            Action::<Run>::new(),
            bindings![(KeyCode::ShiftLeft)],
        ),
        (
            Action::<UpDown>::new(),
            Bindings::spawn(Bidirectional {
                positive: Binding::from(KeyCode::Space),
                negative: Binding::from(KeyCode::ControlLeft),
            })
        ),
        (
            Action::<ToggleCaptureCursor>::new(),
            bindings![(KeyCode::Escape, Hold::new(0.1))]
        )
        ]));
    }
}

fn on_flycam_removed(mut commands: Commands, mut query: RemovedComponents<PlayerFlyCam>) {
    for entity in query.read() {
        debug!("Removing flycam from player: {entity:?}");
        commands
            .entity(entity)
            .remove_with_requires::<PlayerFlyCam>()
            .despawn_related::<Actions<PlayerFlyCam>>();
    }
}

fn capture_cursor(
    _trigger: Trigger<Completed<ToggleCaptureCursor>>,
    mut window: Single<&mut Window>,
) {
    let grab = window.cursor_options.grab_mode == CursorGrabMode::None;
    grab_cursor(&mut window, grab);
}

fn apply_movement(
    move_action: Single<&ActionValue, With<Action<Move>>>,
    run_action: Single<&ActionValue, With<Action<Run>>>,
    up_down_action: Single<&ActionValue, With<Action<UpDown>>>,
    transform: Single<&mut Transform, With<PlayerFlyCam>>,
    time: Res<Time>,
) {
    let mut transform = transform.into_inner();
    let rotation = transform.rotation;

    let mut movement = move_action.as_axis2d().extend(0.0).xzy();
    movement.z = -movement.z;

    let up_down = up_down_action.as_axis1d();
    movement.y += up_down;

    let speed = if !run_action.as_bool() { 400.0 } else { 40.0 };

    transform.translation += rotation * movement * speed * time.delta_secs();
}

fn rotate(
    trigger: Trigger<Fired<Rotate>>,
    mut players: Query<&mut Transform, With<PlayerFlyCam>>,
    window: Single<&Window>,
    time: Res<Time>,
) {
    if window.cursor_options.visible {
        return;
    }

    let mut transform = players.get_mut(trigger.target()).unwrap();
    let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

    yaw += (trigger.value.x * time.delta_secs()).to_radians();
    pitch += (trigger.value.y * time.delta_secs()).to_radians();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

fn grab_cursor(window: &mut Window, grab: bool) {
    window.cursor_options.grab_mode = if grab {
        CursorGrabMode::Confined
    } else {
        CursorGrabMode::None
    };
    window.cursor_options.visible = !grab;
    debug!(
        "Cursor grab mode set to: {:?}, visible: {}",
        window.cursor_options.grab_mode, window.cursor_options.visible
    );
}

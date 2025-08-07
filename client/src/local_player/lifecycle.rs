use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent};
use log::debug;

use crate::{
    assets_loader::TextureAssets,
    bindings::{Player, PlayerLocation},
    local_player::get_player_camera,
    spacetimedb::SpacetimeDB,
};

use super::{LocalPlayerState, flycam::PlayerFlyCam};

const MAX_INTERACTION_DISTANCE: f32 = 40.0;

#[derive(Component)]
pub struct LocalPlayer;

pub struct LocalPlayerLifecyclePlugin;

impl Plugin for LocalPlayerLifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (on_local_player_inserted, on_player_location_inserted).chain(),
        )
        .add_systems(PostUpdate, on_player_location_deleted);
    }
}

fn on_local_player_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<Player>,
    mut images: ResMut<Assets<Image>>,
    texture_assets: Res<TextureAssets>,
    stdb: SpacetimeDB,
) {
    for event in events.read().filter(|e| e.row.id == stdb.identity()) {
        let player = &event.row;
        debug!("Inserting local player: {player:?}");
        commands.spawn((
            Name::new("Local Player"),
            get_player_camera(&texture_assets, &mut images),
            Visibility::Hidden,
            LocalPlayer,
            Transform {
                translation: Vec3::new(player.x, player.y, player.z),
                rotation: Quat::from_xyzw(player.rot_x, player.rot_y, player.rot_z, player.rot_w),
                ..default()
            },
            RayCaster::new(Vec3::ZERO, Dir3::NEG_Z).with_max_distance(MAX_INTERACTION_DISTANCE),
        ));
    }
}

fn on_player_location_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<PlayerLocation>,
    mut player_state: ResMut<NextState<LocalPlayerState>>,
    player_entity: Single<Entity, With<LocalPlayer>>,
    stdb: SpacetimeDB,
) {
    for event in events.read().filter(|e| e.row.player_id == stdb.identity()) {
        let player_location = &event.row;
        debug!("Inserting player location: {player_location:?}");
        commands.entity(player_entity.entity()).insert((
            PlayerFlyCam,
            Visibility::Visible,
            Transform {
                translation: Vec3::new(player_location.x, player_location.y, player_location.z),
                rotation: Quat::from_xyzw(
                    player_location.rot_x,
                    player_location.rot_y,
                    player_location.rot_z,
                    player_location.rot_w,
                ),
                ..default()
            },
        ));
        player_state.set(LocalPlayerState::OnFoot);
    }
}

fn on_player_location_deleted(
    mut commands: Commands,
    mut events: ReadDeleteEvent<PlayerLocation>,
    mut player_state: ResMut<NextState<LocalPlayerState>>,
    player_entity: Single<Entity, With<LocalPlayer>>,
    stdb: SpacetimeDB,
) {
    for _ in events.read().filter(|e| e.row.player_id == stdb.identity()) {
        debug!(
            "Removing player location for entity: {:?}",
            player_entity.entity()
        );
        commands
            .entity(player_entity.entity())
            .remove::<PlayerFlyCam>()
            .insert(Visibility::Hidden);
        player_state.set(LocalPlayerState::InShip);
    }
}

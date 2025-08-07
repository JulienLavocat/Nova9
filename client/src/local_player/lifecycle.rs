use bevy::prelude::*;
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent};
use log::debug;

use crate::{
    assets_loader::TextureAssets, bindings::PlayerLocation, local_player::get_player_camera,
    spacetimedb::SpacetimeDB,
};

use super::flycam::PlayerFlyCam;

#[derive(Component)]
pub struct LocalPlayerEntity;

pub struct LocalPlayerLifecyclePlugin;

impl Plugin for LocalPlayerLifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, on_player_location_inserted)
            .add_systems(PostUpdate, on_player_location_deleted);
    }
}

fn on_player_location_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<PlayerLocation>,
    texture_assets: Res<TextureAssets>,
    mut images: ResMut<Assets<Image>>,
    stdb: SpacetimeDB,
) {
    for event in events.read().filter(|e| e.row.player_id == stdb.identity()) {
        let player_location = &event.row;
        debug!("Inserting player location: {player_location:?}");
        commands.spawn((
            Name::new("Player Camera"),
            get_player_camera(&texture_assets, &mut images),
            PlayerFlyCam,
            LocalPlayerEntity,
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
    }
}

fn on_player_location_deleted(
    mut commands: Commands,
    mut events: ReadDeleteEvent<PlayerLocation>,
    player_entity: Single<Entity, With<LocalPlayerEntity>>,
    stdb: SpacetimeDB,
) {
    for _ in events.read().filter(|e| e.row.player_id == stdb.identity()) {
        debug!(
            "Removing player location for entity: {:?}",
            player_entity.entity()
        );
        commands.entity(player_entity.entity()).despawn();
    }
}

use bevy::{
    core_pipeline::{Skybox, bloom::Bloom},
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use bevy_flycam::FlyCam;

use crate::{
    GameState, assets_loader::TextureAssets, bindings::player_ready, spacetimedb::SpacetimeDB,
};

#[derive(Component)]
pub struct PlayerCamera;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (player_ready, spawn_player));
    }
}

fn player_ready(stdb: SpacetimeDB) {
    stdb.reducers().player_ready().unwrap();
}

fn spawn_player(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    texture_assets: Res<TextureAssets>,
) {
    let skybox_handle = texture_assets.skybox_black.clone();

    let image = images.get_mut(&skybox_handle).unwrap();
    // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
    // so they appear as one texture. The following code reconfigures the texture as necessary.
    if image.texture_descriptor.array_layer_count() == 1 {
        image.reinterpret_stacked_2d_as_array(image.height() / image.width());
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }

    // For now, the player is just a camera. In the future he will be a 3D model that can move
    commands.spawn((
        PlayerCamera,
        Name::new("Player Camera"),
        Camera3d::default(),
        Bloom::NATURAL,
        Skybox {
            image: skybox_handle,
            brightness: 1000.0,
            ..Default::default()
        },
    ));
}

use bevy::{
    core_pipeline::{Skybox, bloom::Bloom},
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use flycam::LocalPlayerFlycamPlugin;
use lifecycle::LocalPlayerLifecyclePlugin;

use crate::{
    GameState, assets_loader::TextureAssets, bindings::player_ready, spacetimedb::SpacetimeDB,
};

mod flycam;
mod lifecycle;

#[derive(Component)]
pub struct PlayerCamera;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LocalPlayerLifecyclePlugin, LocalPlayerFlycamPlugin))
            .add_systems(OnEnter(GameState::InGame), player_ready);
    }
}

fn player_ready(stdb: SpacetimeDB) {
    stdb.reducers().player_ready().unwrap();
}

fn get_player_camera(
    texture_assets: &TextureAssets,
    images: &mut Assets<Image>,
) -> (PlayerCamera, Camera3d, Bloom, Skybox) {
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

    (
        PlayerCamera,
        Camera3d::default(),
        Bloom::NATURAL,
        Skybox {
            image: skybox_handle,
            brightness: 1000.0,
            ..Default::default()
        },
    )
}

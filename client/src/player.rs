use bevy::{
    core_pipeline::{Skybox, bloom::Bloom},
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use bevy_flycam::FlyCam;

use crate::{
    GameState,
    assets_loader::{ModelAssets, TextureAssets},
    materials::GameMaterial,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    images: ResMut<Assets<Image>>,
    model_assets: Res<ModelAssets>,
) {
    commands.spawn((
        Camera3d::default(),
        FlyCam,
        Bloom::NATURAL,
        create_skybox(texture_assets, images),
    ));

    commands.spawn((
        SceneRoot(model_assets.ship_bomber_01.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GameMaterial::Ship,
    ));
}

pub fn create_skybox(
    texture_assets: Res<TextureAssets>,
    mut images: ResMut<Assets<Image>>,
) -> Skybox {
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

    Skybox {
        image: skybox_handle,
        brightness: 1000.0,
        ..Default::default()
    }
}

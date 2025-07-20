use std::f32::consts::PI;

use assets_loader::{AssetsLoaderPlugin, MaterialAssets, ModelAssets, TextureAssets};
use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
    scene::SceneInstanceReady,
};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use shaders::ShadersPlugin;

mod assets_loader;
mod shaders;
mod utils;

#[derive(Component)]
enum SyntyMaterial {
    Standard,
    SpaceStation,
}

#[derive(States, Clone, Debug, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    InGame,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShadersPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default())
        .init_state::<GameState>()
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(OnEnter(GameState::InGame), after_load)
        .add_observer(apply_synty_material)
        .run()
}

fn after_load(
    mut commands: Commands,
    model_assets: Res<ModelAssets>,
    texture_assets: Res<TextureAssets>,
    mut images: ResMut<Assets<Image>>,
) {
    let skybox_handle = texture_assets.skybox_black.clone();

    let image = images.get_mut(&skybox_handle).unwrap();
    // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
    // so they appear as one texture. The following code reconfigures the texture as necessary.
    if image.texture_descriptor.array_layer_count() == 1 {
        debug!(
            "skybox: {:?} is not a cubemap, reinterpreting as such",
            skybox_handle
        );
        image.reinterpret_stacked_2d_as_array(image.height() / image.width());
        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });
    }

    commands.spawn((
        Camera3d::default(),
        FlyCam,
        Skybox {
            image: skybox_handle,
            brightness: 1000.0,
            ..Default::default()
        },
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0.0, 2.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(210, 220, 240),
        brightness: 1.0,
        ..default()
    });

    commands.spawn((
        SceneRoot(model_assets.ship_bomber_01.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        SyntyMaterial::Standard,
    ));

    commands.spawn((
        SceneRoot(model_assets.ship_station_01.clone()),
        Transform::from_xyz(100.0, 0.0, 0.0),
        SyntyMaterial::SpaceStation,
    ));
}

fn apply_synty_material(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    material_targets: Query<&SyntyMaterial>,
    materials: Res<MaterialAssets>,
) {
    let material_target = material_targets.get(trigger.target());
    if material_target.ok().is_none() {
        return;
    }

    for descendants in children.iter_descendants(trigger.target()) {
        match material_target.unwrap() {
            SyntyMaterial::Standard => {
                commands
                    .entity(descendants)
                    .insert(MeshMaterial3d(materials.synty_mat_01.clone()));
            }
            SyntyMaterial::SpaceStation => {
                commands
                    .entity(descendants)
                    .remove::<MeshMaterial3d<StandardMaterial>>()
                    .insert(MeshMaterial3d(materials.spacestation.clone()));
            }
        };
    }
}

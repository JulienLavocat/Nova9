use std::f32::consts::PI;

use assets_loader::{AssetsLoaderPlugin, MaterialAssets, ModelAssets, TextureAssets};
use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin, PlayerPlugin};

mod assets_loader;

#[derive(Component)]
struct SyntyMaterialTarget;

#[derive(States, Clone, Debug, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    InGame,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(NoCameraPlayerPlugin)
        .add_systems(OnEnter(GameState::InGame), after_load)
        .run()
}

fn after_load(
    mut commands: Commands,
    material_assets: Res<MaterialAssets>,
    model_assets: Res<ModelAssets>,
    texture_assets: Res<TextureAssets>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let skybox_handle = texture_assets.skybox_04.clone();

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
}

// fn setup(
//     mut commands: Commands,
//     asset_server: ResMut<AssetServer>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
// commands.spawn((Camera3d::default(), FlyCam));
//
// commands.spawn((
//     DirectionalLight::default(),
//     Transform::from_xyz(0.0, 100.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
// ));
//
// let ship = asset_server.load(GltfAssetLabel::Scene(0).from_asset("SM_Ship_Bomber_01.glb"));
//
// commands.spawn(SceneRoot(ship));
//
// commands.spawn((
//     SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("SM_Ship_Bomber_02.glb"))),
//     Transform::from_xyz(0.0, 0.0, -20.0),
// ));
// commands.spawn((
//     SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("SM_Ship_Fighter_01.glb"))),
//     Transform::from_xyz(20.0, 0.0, 0.0),
// ));
// }

// fn change_material(
//     trigger: Trigger<SceneInstanceReady>,
//     mut commands: Commands,
//     children: Query<&Children>,
//     color_override: Query<&SyntyMaterialTarget>,
//     asset_server: Res<AssetServer>,
//     mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
//     mut asset_materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     if color_override.get(trigger.target()).ok().is_none() {
//         return;
//     }
//
//     // Iterate over all children recursively
//     for descendants in children.iter_descendants(trigger.target()) {
//         // Get the material of the descendant
//         if mesh_materials
//             .get(descendants)
//             .ok()
//             .and_then(|id| asset_materials.get_mut(id.id()))
//             .is_some()
//         {
//             let main_texture: Handle<Image> =
//                 asset_server.load("PolygonSciFiSpace_Texture_01_A.png");
//             let emissive_texture: Handle<Image> =
//                 asset_server.load("PolygonSciFiSpace_Emissive_01.png");
//             let main_material = StandardMaterial {
//                 base_color_texture: Some(main_texture),
//                 emissive_texture: Some(emissive_texture),
//                 metallic: 0.2,
//                 perceptual_roughness: 0.8,
//                 ..default()
//             };
//             commands
//                 .entity(descendants)
//                 .insert(MeshMaterial3d(asset_materials.add(main_material)));
//         }
//     }
// }

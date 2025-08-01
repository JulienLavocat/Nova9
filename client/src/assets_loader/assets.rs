use bevy::{pbr::ExtendedMaterial, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::shaders::DetailedMaterialExtension;

#[derive(Resource, AssetCollection, Default, Debug)]
pub struct ModelAssets {
    #[asset(path = "models/ships/bomber_01.glb#Scene0")]
    pub ship_bomber_01: Handle<Scene>,

    #[asset(path = "models/ships/station_01.glb#Scene0")]
    pub ship_station_01: Handle<Scene>,

    #[asset(path = "models/env/astroid_01.glb#Scene0")]
    pub asteroid_01: Handle<Scene>,
    #[asset(path = "models/env/astroid_02.glb#Scene0")]
    pub asteroid_02: Handle<Scene>,
    #[asset(path = "models/env/astroid_03.glb#Scene0")]
    pub asteroid_03: Handle<Scene>,
    #[asset(path = "models/env/astroid_04.glb#Scene0")]
    pub asteroid_04: Handle<Scene>,
    #[asset(path = "models/env/astroid_05.glb#Scene0")]
    pub asteroid_05: Handle<Scene>,
}

#[derive(Resource, AssetCollection, Default, Debug)]
pub struct MaterialAssets {
    #[asset(key = "mats.base_01")]
    pub base_01: Handle<StandardMaterial>,

    #[asset(key = "mats.spacestation")]
    pub spacestation: Handle<ExtendedMaterial<StandardMaterial, DetailedMaterialExtension>>,

    #[asset(key = "mats.spaceship")]
    pub spaceship: Handle<ExtendedMaterial<StandardMaterial, DetailedMaterialExtension>>,
}

#[derive(Resource, AssetCollection, Default, Debug)]
pub struct TextureAssets {
    #[asset(path = "textures/skyboxes/04/cubemap.png")]
    pub skybox_black: Handle<Image>,
}

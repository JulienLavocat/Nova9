use bevy::{pbr::ExtendedMaterial, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::shaders::DetailedMaterialExtension;

#[derive(Resource, AssetCollection, Default, Debug)]
pub struct ModelAssets {
    #[asset(path = "models/ships/bomber_01.glb#Scene0")]
    pub ship_bomber_01: Handle<Scene>,

    #[asset(path = "models/ships/station_01.glb#Scene0")]
    pub ship_station_01: Handle<Scene>,
    #[asset(path = "models/ships/station_02.glb#Scene0")]
    pub ship_station_02: Handle<Scene>,
    #[asset(path = "models/ships/station_03.glb#Scene0")]
    pub ship_station_03: Handle<Scene>,
    #[asset(path = "models/ships/station_04.glb#Scene0")]
    pub ship_station_04: Handle<Scene>,
    #[asset(path = "models/ships/station_05.glb#Scene0")]
    pub ship_station_05: Handle<Scene>,
    #[asset(path = "models/ships/station_06.glb#Scene0")]
    pub ship_station_06: Handle<Scene>,

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
pub struct CollisionAssets {
    #[asset(path = "colliders/ships/bomber_01_collision.glb#Mesh0/Primitive0")]
    pub ship_bomber_01: Handle<Mesh>,
    #[asset(path = "colliders/ships/station_01_collision.glb#Mesh0/Primitive0")]
    pub ship_station_01: Handle<Mesh>,
    #[asset(path = "colliders/ships/station_02_collision.glb#Mesh0/Primitive0")]
    pub ship_station_02: Handle<Mesh>,
    #[asset(path = "colliders/ships/station_03_collision.glb#Mesh0/Primitive0")]
    pub ship_station_03: Handle<Mesh>,
    #[asset(path = "colliders/ships/station_04_collision.glb#Mesh0/Primitive0")]
    pub ship_station_04: Handle<Mesh>,
    #[asset(path = "colliders/ships/station_05_collision.glb#Mesh0/Primitive0")]
    pub ship_station_05: Handle<Mesh>,
    #[asset(path = "colliders/ships/station_06_collision.glb#Mesh0/Primitive0")]
    pub ship_station_06: Handle<Mesh>,
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

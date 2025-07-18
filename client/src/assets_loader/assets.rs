use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Resource, AssetCollection, Debug)]
pub struct ModelAssets {
    #[asset(path = "models/SM_Ship_Bomber_01.glb")]
    pub ship_bomber_01: Handle<Gltf>,
}

#[derive(Resource, AssetCollection, Debug)]
pub struct MaterialAssets {
    #[asset(key = "mats.synty_mat_01")]
    pub synty_mat_01: Handle<StandardMaterial>,
}

#[derive(Resource, AssetCollection, Debug)]
pub struct TextureAssets {
    #[asset(path = "textures/skyboxes/01/cubemap.png")]
    pub skybox_01: Handle<Image>,

    #[asset(path = "textures/skyboxes/02/cubemap.png")]
    pub skybox_02: Handle<Image>,

    #[asset(path = "textures/skyboxes/03/cubemap.png")]
    pub skybox_03: Handle<Image>,

    #[asset(path = "textures/skyboxes/04/cubemap.png")]
    pub skybox_04: Handle<Image>,
}

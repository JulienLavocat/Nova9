use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    LoadingState, LoadingStateAppExt, config::ConfigureLoadingState,
};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::GameState;

mod assets;
mod custom_loader;

pub use assets::*;
use custom_loader::*;

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding AssetsLoaderPlugin");
        app.init_resource::<TextureAssets>()
            .init_resource::<ModelAssets>()
            .init_resource::<MaterialAssets>()
            .add_plugins(RonAssetPlugin::<CustomDynamicAssetCollection>::new(&[
                "manifest.ron",
            ]))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::InGame)
                    .register_dynamic_asset_collection::<CustomDynamicAssetCollection>()
                    .with_dynamic_assets_file::<CustomDynamicAssetCollection>(
                        "materials.manifest.ron",
                    )
                    .load_collection::<ModelAssets>()
                    .load_collection::<TextureAssets>()
                    .load_collection::<MaterialAssets>(),
            );
    }
}

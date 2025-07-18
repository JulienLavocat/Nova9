use bevy::{ecs::system::SystemState, platform::collections::HashMap, prelude::*};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
enum CustomDynamicAsset {
    StandardMaterial {
        base_color_texture: String,
        emissive_texture: Option<String>,
    },
}

impl DynamicAsset for CustomDynamicAsset {
    fn load(&self, asset_server: &AssetServer) -> Vec<UntypedHandle> {
        match self {
            Self::StandardMaterial {
                base_color_texture,
                emissive_texture,
            } => {
                let mut textures = Vec::new();

                textures.push(asset_server.load_untyped(base_color_texture).untyped());

                if let Some(emissive_texture) = emissive_texture {
                    textures.push(asset_server.load_untyped(emissive_texture).untyped());
                }

                textures
            }
        }
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, anyhow::Error> {
        match self {
            Self::StandardMaterial {
                base_color_texture,
                emissive_texture,
            } => {
                let mut system_state =
                    SystemState::<(ResMut<Assets<StandardMaterial>>, Res<AssetServer>)>::new(world);
                let (mut materials, asset_server) = system_state.get_mut(world);

                let base_color_texture = Some(asset_server.load(base_color_texture));
                let emissive_texture = emissive_texture.as_ref().map(|tex| asset_server.load(tex));

                let material = StandardMaterial {
                    base_color_texture,
                    emissive_texture,
                    ..Default::default()
                };

                Ok(DynamicAssetType::Single(materials.add(material).untyped()))
            }
        }
    }
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct CustomDynamicAssetCollection(HashMap<String, CustomDynamicAsset>);

impl DynamicAssetCollection for CustomDynamicAssetCollection {
    fn register(&self, dynamic_assets: &mut DynamicAssets) {
        for (key, asset) in self.0.iter() {
            dynamic_assets.register_asset(key, Box::new(asset.clone()));
        }
    }
}

use bevy::{
    ecs::system::SystemState, pbr::ExtendedMaterial, platform::collections::HashMap, prelude::*,
};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

use crate::shaders::SpaceStationMaterialExtension;

#[derive(Deserialize, Debug, Clone)]
enum CustomDynamicAsset {
    StandardMaterial {
        base_color_texture: String,
        emissive_texture: Option<String>,
        normal_texture: Option<String>,
    },
    SpaceStationMaterial {
        base_texture: String,
        details_texture: String,
        emissive_texture: String,
        details_amount: f32,
        emissive_color: [f32; 4],
    },
}

impl DynamicAsset for CustomDynamicAsset {
    fn load(&self, asset_server: &AssetServer) -> Vec<UntypedHandle> {
        match self {
            Self::StandardMaterial {
                base_color_texture,
                emissive_texture,
                normal_texture,
            } => {
                let mut textures = Vec::new();

                textures.push(asset_server.load_untyped(base_color_texture).untyped());

                if let Some(emissive_texture) = emissive_texture {
                    textures.push(asset_server.load_untyped(emissive_texture).untyped());
                }
                if let Some(normal_texture) = normal_texture {
                    textures.push(asset_server.load_untyped(normal_texture).untyped());
                }

                textures
            }
            Self::SpaceStationMaterial {
                base_texture,
                details_texture,
                emissive_texture,
                details_amount: _,
                emissive_color: _,
            } => {
                vec![
                    asset_server.load_untyped(base_texture).untyped(),
                    asset_server.load_untyped(details_texture).untyped(),
                    asset_server.load_untyped(emissive_texture).untyped(),
                ]
            }
        }
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, anyhow::Error> {
        match self {
            Self::StandardMaterial {
                base_color_texture,
                emissive_texture,
                normal_texture,
            } => {
                let mut system_state =
                    SystemState::<(ResMut<Assets<StandardMaterial>>, Res<AssetServer>)>::new(world);
                let (mut materials, asset_server) = system_state.get_mut(world);

                let base_color_texture = Some(asset_server.load(base_color_texture));
                let emissive_texture = emissive_texture.as_ref().map(|tex| asset_server.load(tex));
                let normal_map_texture = normal_texture.as_ref().map(|tex| asset_server.load(tex));

                let material = StandardMaterial {
                    base_color_texture,
                    emissive_texture,
                    normal_map_texture,
                    ..Default::default()
                };

                Ok(DynamicAssetType::Single(materials.add(material).untyped()))
            }
            Self::SpaceStationMaterial {
                base_texture,
                details_texture,
                details_amount,
                emissive_texture,
                emissive_color,
            } => {
                let mut system_state = SystemState::<(
                    ResMut<
                        Assets<ExtendedMaterial<StandardMaterial, SpaceStationMaterialExtension>>,
                    >,
                    Res<AssetServer>,
                )>::new(world);
                let (mut materials, asset_server) = system_state.get_mut(world);

                let texture = asset_server.load(base_texture);
                let details = asset_server.load(details_texture);
                let emissive = asset_server.load(emissive_texture);

                let base = StandardMaterial {
                    base_color_texture: Some(texture.clone()),
                    emissive_texture: Some(emissive.clone()),
                    emissive: LinearRgba::new(
                        emissive_color[0],
                        emissive_color[1],
                        emissive_color[2],
                        emissive_color[3],
                    ),
                    ..Default::default()
                };
                let extension = SpaceStationMaterialExtension {
                    texture,
                    details,
                    details_amount: *details_amount,
                };

                Ok(DynamicAssetType::Single(
                    materials
                        .add(ExtendedMaterial { base, extension })
                        .untyped(),
                ))
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

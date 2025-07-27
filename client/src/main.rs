use assets_loader::AssetsLoaderPlugin;
use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_flycam::{MovementSettings, NoCameraPlayerPlugin};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use materials::MaterialsPlugin;
use player::PlayerPlugin;
use shaders::ShadersPlugin;
use ships::ShipsPlugin;
use spacetimedb::SpacetimeDbPlugin;
use world::WorldPlugin;

mod assets_loader;
mod bindings;
mod materials;
mod player;
mod shaders;
mod ships;
mod spacetimedb;
mod world;

#[derive(States, Clone, Debug, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    WaitingForConnection,
    StaticDataLoading,
    InGame,
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(ShadersPlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins((NoCameraPlayerPlugin, EnhancedInputPlugin))
        .add_plugins((
            AssetsLoaderPlugin,
            SpacetimeDbPlugin,
            MaterialsPlugin,
            PlayerPlugin,
            ShipsPlugin,
            WorldPlugin,
        ))
        .insert_resource(MovementSettings {
            speed: 20.0,
            ..default()
        })
        .run()
}

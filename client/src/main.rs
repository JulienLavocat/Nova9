use assets_loader::AssetsLoaderPlugin;
use avian3d::{PhysicsPlugins, prelude::Gravity};
use bevy::{prelude::*, window::WindowMode};
use bevy_enhanced_input::EnhancedInputPlugin;
use bevy_flycam::{MovementSettings, NoCameraPlayerPlugin};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
#[cfg(feature = "dev")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
    #[cfg(feature = "dev")]
    let window = Window {
        title: "Nova 9 - dev".to_string(),
        mode: WindowMode::Windowed,
        ..default()
    };
    #[cfg(not(feature = "dev"))]
    let window = Window {
        title: "Nova 9".to_string(),
        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
        ..default()
    };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }))
    .init_state::<GameState>()
    .add_plugins(ShadersPlugin)
    .add_plugins(EguiPlugin::default())
    .add_plugins((
        NoCameraPlayerPlugin,
        EnhancedInputPlugin,
        PhysicsPlugins::default(),
        // PhysicsDebugPlugin::default(),
    ))
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
    .insert_resource(Gravity(Vec3::ZERO));

    #[cfg(feature = "dev")]
    app.add_plugins((
        WorldInspectorPlugin::default(),
        // PhysicsDebugPlugin::default(),
    ));

    app.run()
}

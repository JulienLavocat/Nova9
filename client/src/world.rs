use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{GameState, assets_loader::ModelAssets, materials::GameMaterial};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_world);
    }
}

fn spawn_world(mut commands: Commands, model_assets: Res<ModelAssets>) {
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(210, 220, 240),
        brightness: 1.0,
        ..default()
    });

    commands.spawn((
        SceneRoot(model_assets.ship_station_01.clone()),
        Transform::from_xyz(100.0, 0.0, 0.0),
        GameMaterial::Station,
    ));

    commands.spawn((
        SceneRoot(model_assets.asteroid_01.clone()),
        Transform::from_xyz(0.0, 0.0, -100.0),
        GameMaterial::Standard,
    ));
}

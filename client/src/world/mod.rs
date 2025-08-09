use asteroids::AsteroidsPlugin;
use avian3d::math::PI;
use bevy::prelude::*;
use stations::StationsPlugin;

use crate::{GameState, spacetimedb::SpacetimeDB};

mod asteroids;
mod stations;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StationsPlugin, AsteroidsPlugin))
            .add_systems(OnEnter(GameState::InGame), subscribe_to_world);
    }
}

fn subscribe_to_world(mut commands: Commands, stdb: SpacetimeDB) {
    commands.spawn((
        DirectionalLight {
            illuminance: 500.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 2000.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(210, 220, 240),
        ..default()
    });

    stdb.subscription_builder()
        .on_applied(|_| debug!("Subscribed to world"))
        .on_error(|_, err| {
            panic!("Failed to subscribe to world: {err}");
        })
        .subscribe_to_all_tables();
}

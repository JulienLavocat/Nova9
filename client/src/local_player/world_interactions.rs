use avian3d::prelude::RayHits;
use bevy::prelude::*;

use crate::{
    bindings::{player_enter_ship, player_spawn_ship},
    ships::Ship,
    spacetimedb::SpacetimeDB,
};

use super::{LocalPlayerState, lifecycle::LocalPlayer};

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (enter_ship_interaction, spawn_ship).run_if(in_state(LocalPlayerState::OnFoot)),
        );
    }
}

fn enter_ship_interaction(
    ray_caster: Single<&RayHits, With<LocalPlayer>>,
    input: Res<ButtonInput<KeyCode>>,
    ships: Query<&Ship>,
    stdb: SpacetimeDB,
) {
    for hit in ray_caster.iter() {
        if let Ok(ship) = ships.get(hit.entity)
            && input.just_pressed(KeyCode::KeyF)
        {
            debug!("Entering ship: {}", ship.id);
            stdb.reducers().player_enter_ship(ship.id).unwrap();
        }
    }
}

fn spawn_ship(
    player: Single<&Transform, With<LocalPlayer>>,
    stdb: SpacetimeDB,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::KeyR) {
        return;
    }

    // Spawn ship 20 units in front of the player
    let postion = player.translation + player.forward() * 20.0;
    stdb.reducers()
        .player_spawn_ship(
            postion.x,
            postion.y,
            postion.z,
            player.rotation.x,
            player.rotation.y,
            player.rotation.z,
            player.rotation.w,
        )
        .unwrap();
}

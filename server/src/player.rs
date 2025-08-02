use spacetimedb::{reducer, ReducerContext, Table};

use crate::tables::{ship_locations, ship_pilots, ships, Ship, ShipLocation, ShipPilot};

#[reducer]
fn player_ready(ctx: &ReducerContext) {
    // For now, we just spawn a ship for the player.
    let ship = ctx.db.ships().insert(Ship {
        id: 0,
        ship_type_id: 1,
        owner_id: ctx.sender,
    });

    ctx.db.ship_locations().insert(ShipLocation {
        ship_id: ship.id,
        x: 0.0,
        y: 0.0,
        z: 300.0,
        rot_x: 0.0,
        rot_y: 0.0,
        rot_z: 0.0,
        rot_w: 1.0,
    });

    ctx.db.ship_pilots().insert(ShipPilot {
        ship_id: ship.id,
        player_id: ctx.sender,
    });
}

#[reducer]
fn player_move_ship(
    ctx: &ReducerContext,
    x: f32,
    y: f32,
    z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    rot_w: f32,
) {
    if let Some(ship) = ctx.db.ship_pilots().player_id().find(ctx.sender) {
        ctx.db.ship_locations().ship_id().update(ShipLocation {
            ship_id: ship.ship_id,
            x,
            y,
            z,
            rot_x,
            rot_y,
            rot_z,
            rot_w,
        });
    }
}

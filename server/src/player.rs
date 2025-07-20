use spacetimedb::{reducer, ReducerContext, Table};

use crate::tables::{ship_pilots, ships, Ship, ShipPilot};

#[reducer]
fn player_ready(ctx: &ReducerContext) {
    // For now, we just spawn a ship for the player.
    let ship = ctx.db.ships().insert(Ship {
        id: 0,
        ship_type_id: 1,
        owner_id: ctx.sender,
        x: 300.0,
        y: 0.0,
        z: 0.0,
    });

    ctx.db.ship_pilots().insert(ShipPilot {
        ship_id: ship.id,
        player_id: ctx.sender,
    });
}

use log::debug;
use spacetimedb::{reducer, ReducerContext, Table};
use tables::{players, ship_locations, ship_pilots, ships, Player};

mod init;
mod player;
mod tables;

#[reducer(client_connected)]
fn on_connected(ctx: &ReducerContext) {
    debug!("Player connected: {}", ctx.sender);
    ctx.db.players().insert(Player { id: ctx.sender });
}

#[reducer(client_disconnected)]
fn on_disconnected(ctx: &ReducerContext) {
    debug!("Player disconnected: {}", ctx.sender);
    ctx.db.players().id().delete(ctx.sender);
    ctx.db.ship_pilots().player_id().delete(ctx.sender);

    ctx.db
        .ships()
        .owner_id()
        .filter(ctx.sender)
        .for_each(|ship| {
            ctx.db.ships().id().delete(ship.id);
            ctx.db.ship_locations().ship_id().delete(ship.id);
        });
}

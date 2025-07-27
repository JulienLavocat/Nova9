use spacetimedb::{reducer, ReducerContext, Table};
use tables::{players, ship_pilots, ships, Player};

mod init;
mod player;
mod tables;

#[reducer(client_connected)]
fn on_connected(ctx: &ReducerContext) {
    ctx.db.players().insert(Player { id: ctx.sender });
}

#[reducer(client_disconnected)]
fn on_disconnected(ctx: &ReducerContext) {
    ctx.db.players().id().delete(ctx.sender);
    ctx.db.ships().owner_id().delete(ctx.sender);
    ctx.db.ship_pilots().player_id().delete(ctx.sender);
}

use log::debug;
use spacetimedb::{reducer, ReducerContext};
use spacetimedsl::{dsl, Wrapper};
use tables::*;

mod init;
mod player;
mod tables;
mod world;

#[reducer(client_connected)]
fn on_connected(ctx: &ReducerContext) -> Result<(), String> {
    debug!("Player connected: {}", ctx.sender);

    dsl(ctx).create_player(
        ctx.sender, 0.0, 0.0, 0.0, // Initial position
        0.0, 0.0, 0.0, 1.0, // Initial rotation (identity quaternion)
    )?;
    Ok(())
}

#[reducer(client_disconnected)]
fn on_disconnected(ctx: &ReducerContext) -> Result<(), String> {
    debug!("Player disconnected: {}", ctx.sender);
    let dsl = dsl(ctx);

    let player_id = PlayerId::new(ctx.sender);
    dsl.delete_player_by_id(&player_id)?;
    dsl.delete_ships_by_owner_id(&player_id)?;

    Ok(())
}

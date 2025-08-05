use log::debug;
use spacetimedb::{reducer, ReducerContext};
use spacetimedsl::{dsl, Wrapper};
use tables::{
    CreatePlayerRow, DeletePlayerRowByIdentity, DeleteShipRowsByOwnerId,
    GetPlayerRowOptionByIdentity, PlayerIdentity,
};

mod init;
mod player;
mod tables;
mod world;

#[reducer(client_connected)]
fn on_connected(ctx: &ReducerContext) -> Result<(), String> {
    debug!("Player connected: {}", ctx.sender);

    dsl(ctx).create_player(ctx.sender)?;
    Ok(())
}

#[reducer(client_disconnected)]
fn on_disconnected(ctx: &ReducerContext) {
    debug!("Player disconnected: {}", ctx.sender);
    let dsl = dsl(ctx);
    let player = dsl
        .get_player_by_identity(PlayerIdentity::new(ctx.sender))
        .expect("Player should exist on disconnect");

    dsl.delete_player_by_identity(player.get_identity())
        .expect("Failed to delete player by identity");
    dsl.delete_ships_by_owner_id(player.get_id())
        .expect("Failed to delete ships for disconnected player");
}

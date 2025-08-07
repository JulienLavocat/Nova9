use spacetimedb::{reducer, ReducerContext};
use spacetimedsl::{dsl, Wrapper};

use crate::tables::*;

#[reducer]
fn player_ready(ctx: &ReducerContext) -> Result<(), String> {
    let dsl = dsl(ctx);

    // For now, we just spawn a ship for the player.
    let player_id = PlayerId::new(ctx.sender);
    let ship = dsl.create_ship(1, &player_id)?;
    dsl.create_ship_location(ship.get_id(), 0.0, 0.0, 800.0, 0.0, 0.0, 0.0, 1.0)?;
    dsl.create_ship_pilot(ship.get_id(), &player_id)?;

    Ok(())
}

#[reducer]
fn player_spawn_ship(
    ctx: &ReducerContext,
    x: f32,
    y: f32,
    z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    rot_w: f32,
) -> Result<(), String> {
    let dsl = dsl(ctx);

    // Create the ship and its location
    let player_id = PlayerId::new(ctx.sender);
    let ship = dsl.create_ship(1, &player_id)?;
    dsl.create_ship_location(ship.get_id(), x, y, z, rot_x, rot_y, rot_z, rot_w)?;
    dsl.create_ship_pilot(ship.get_id(), &player_id)?;

    Ok(())
}

#[reducer]
fn player_enter_ship(ctx: &ReducerContext, ship_id: u64) -> Result<(), String> {
    let dsl = dsl(ctx);

    let player_id = PlayerId::new(ctx.sender);
    // TODO: Check if the player is in range of the ship
    let ship = dsl.get_ship_by_id(ShipId::new(ship_id))?;
    dsl.delete_ship_pilot_by_player_id(&player_id)?;
    dsl.create_ship_pilot(ship.get_id(), &player_id)?;

    Ok(())
}

#[reducer]
fn player_leave_ship(ctx: &ReducerContext) -> Result<(), String> {
    let dsl = dsl(ctx);
    dsl.delete_ship_pilot_by_player_id(&PlayerId::new(ctx.sender))?;
    Ok(())
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
) -> Result<(), String> {
    let dsl = dsl(ctx);

    if let Ok(ship) = dsl.get_ship_pilot_by_player_id(&PlayerId::new(ctx.sender)) {
        dsl.update_ship_location_by_ship_id(ShipLocation::new(
            ship.get_ship_id(),
            x,
            y,
            z,
            rot_x,
            rot_y,
            rot_z,
            rot_w,
        ))
        .unwrap();
        return Ok(());
    }

    Err("Player is not piloting a ship".into())
}

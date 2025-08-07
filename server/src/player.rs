use spacetimedb::{reducer, ReducerContext};
use spacetimedsl::{dsl, Wrapper};

use crate::tables::*;

#[reducer]
fn player_ready(ctx: &ReducerContext) -> Result<(), String> {
    let dsl = dsl(ctx);

    let player = dsl.get_player_by_id(&PlayerId::new(ctx.sender))?;
    dsl.create_player_location(
        player.get_id(),
        *player.get_x(),
        *player.get_y(),
        *player.get_z(),
        *player.get_rot_x(),
        *player.get_rot_y(),
        *player.get_rot_z(),
        *player.get_rot_w(),
    )?;

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

    Ok(())
}

#[reducer]
fn player_enter_ship(ctx: &ReducerContext, ship_id: u64) -> Result<(), String> {
    let dsl = dsl(ctx);

    let player_id = PlayerId::new(ctx.sender);
    // TODO: Check if the player is in range of the ship
    let ship = dsl.get_ship_by_id(ShipId::new(ship_id))?;
    dsl.delete_ship_pilot_by_player_id(&player_id).ok();
    dsl.create_ship_pilot(ship.get_id(), &player_id)?;

    dsl.delete_player_location_by_player_id(player_id)?;

    Ok(())
}

#[reducer]
fn player_leave_ship(ctx: &ReducerContext) -> Result<(), String> {
    let dsl = dsl(ctx);
    let player_id = &PlayerId::new(ctx.sender);
    let ship_pilot = dsl.get_ship_pilot_by_player_id(player_id)?;
    let ship = dsl.get_ship_location_by_ship_id(ship_pilot.get_ship_id())?;

    dsl.delete_ship_pilot_by_player_id(player_id)?;
    // TODO: Properly find a safe position to spawn the player at
    // Possibly get it from the client and validate it based on the distance to the ship?
    dsl.create_player_location(
        player_id,
        *ship.get_x(),
        *ship.get_y() + 10.0,
        *ship.get_z(),
        *ship.get_rot_x(),
        *ship.get_rot_y(),
        *ship.get_rot_z(),
        *ship.get_rot_w(),
    )?;

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
        // TODO: Validate the new position and rotation
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

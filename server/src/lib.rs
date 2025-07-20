use spacetimedb::{reducer, ReducerContext, Table};
use tables::{players, ship_pilots, ship_types, ships, stations, Player, ShipType, Station};

mod player;
mod tables;

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    ctx.db.ship_types().insert(ShipType {
        id: 0,
        name: "Bomber".to_string(),
        speed: 100.0,
        rotation_speed: 0.5,
    });

    ctx.db.stations().insert(Station {
        id: 0,
        name: "Station Alpha".to_string(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
}

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

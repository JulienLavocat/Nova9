use spacetimedb::{reducer, ReducerContext, Table};
use tables::{stations, Station};

mod tables;

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    ctx.db.stations().insert(Station {
        id: 0,
        name: "Station Alpha".to_string(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
}

#[reducer]
fn tmp(_ctx: &ReducerContext) {}

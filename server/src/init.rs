use spacetimedb::{reducer, ReducerContext, Table};

use crate::tables::{ship_types, stations, ShipType, Station};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    ctx.db.ship_types().insert(ShipType {
        id: 0,
        name: "Bomber".to_string(),
        speed: 100.0,
        rotation_speed: 0.5,
        camera_offset_x: 0.0,
        camera_offset_y: 10.0,
        camera_offset_z: -40.0,
        camera_rotation_x: -180.0_f32.to_radians(),
        camera_rotation_y: -2.0_f32.to_radians(),
        camera_rotation_z: -180.0_f32.to_radians(),
    });

    ctx.db.stations().insert(Station {
        id: 0,
        name: "Station Alpha".to_string(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
}

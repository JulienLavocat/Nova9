use spacetimedb::{reducer, ReducerContext, Table};

use crate::tables::{ship_types, stations, ShipType, Station};

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    ctx.db.ship_types().insert(ShipType {
        id: 0,
        name: "Bomber".to_string(),

        camera_offset_x: 0.0,
        camera_offset_y: 10.0,
        camera_offset_z: 40.0,

        mass: 1.0,
        thrust: 10000.0,
        vertical_trhust: 50.0,
        lateral_thrust: 50.0,

        linear_damping: 5.0,
        angular_damping: 1.0,

        pitch_torque: 300.0,
        yaw_torque: 500.0,
        roll_torque: 5000.0,
    });

    ctx.db.stations().insert(Station {
        id: 0,
        name: "Station Alpha".to_string(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
}

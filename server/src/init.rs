use std::f32::consts::TAU;

use spacetimedb::{reducer, ReducerContext, Table};

use crate::tables::{asteroids, ship_types, stations, Asteroid, ShipType, Station};

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

        linear_damping: 1.0,
        angular_damping: 1.0,

        pitch_torque: 500.0,
        yaw_torque: 500.0,
        roll_torque: 500.0,
    });

    ctx.db.stations().insert(Station {
        id: 0,
        name: "Station Alpha".to_string(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    let center_x = 0.0;
    let center_y = 0.0;
    let center_z = 0.0;
    let number_of_asteroids = 100;
    let radius = 1500.0;
    for i in 0..number_of_asteroids {
        let angle = i as f32 / number_of_asteroids as f32 * TAU;

        ctx.db.asteroids().insert(Asteroid {
            id: 0,
            pos_x: center_x + radius * angle.cos(),
            pos_y: center_y,
            pos_z: center_z + radius * angle.sin(),
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
            rot_w: 1.0,
            asteroid_type: (i % 5) as u8,
            scale: 10.0,
        });
    }
}

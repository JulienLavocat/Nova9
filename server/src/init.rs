use std::{f32::consts::TAU, time::Duration};

use spacetimedb::{reducer, ReducerContext, Table};
use spacetimedsl::dsl;

use crate::{
    tables::{CreateAsteroidRow, CreateShipTypeRow, CreateStationRow},
    world::{station_rotation_update, StationRotationUpdate},
};

/// The speed at which stations rotate in the world, in radians per second.
const STATIONS_ROTATION_SPEED: f32 = 0.01;

#[reducer(init)]
pub fn init(ctx: &ReducerContext) {
    let dsl = dsl(ctx);

    dsl.create_ship_type(
        "Fighter", // Name
        0.0, 10.0, 40.0, // Camera offsets
        1.0,  // Mass
        1.0, 1.0, // Damping values
        10000.0, 1000.0, 1000.0, // Thrust values
        1500.0, 1500.0, 2000.0, // Torque values
    )
    .unwrap();

    dsl.create_station(
        "Station Alpha",
        0.0,
        0.0,
        0.0,
        STATIONS_ROTATION_SPEED,
        0.0,
        ctx.timestamp
            .to_duration_since_unix_epoch()
            .unwrap()
            .as_millis(),
    )
    .unwrap();

    let center_x = 0.0;
    let center_y = 0.0;
    let center_z = 0.0;
    let number_of_asteroids = 100;
    let radius = 1500.0;
    for i in 0..number_of_asteroids {
        let angle = i as f32 / number_of_asteroids as f32 * TAU;

        dsl.create_asteroid(
            center_x + radius * angle.cos(),
            center_y,
            center_z + radius * angle.sin(),
            0.0,           // rot_x
            0.0,           // rot_y
            0.0,           // rot_z
            1.0,           // rot_w
            (i % 5) as u8, // asteroid_type
            10.0,          // scale
        )
        .unwrap();
    }

    ctx.db
        .station_rotation_update()
        .insert(StationRotationUpdate::new(Duration::from_secs(5).into()));
}

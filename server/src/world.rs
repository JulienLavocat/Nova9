use std::{f32::consts::TAU, ops::Add};

use log::debug;
use spacetimedb::{reducer, table, ScheduleAt, Table};

use crate::tables::{stations, Station};

#[table(name = station_rotation_update, scheduled(world_update_stations_rotation))]
pub struct StationRotationUpdate {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    scheduled_at: ScheduleAt,
}

impl StationRotationUpdate {
    pub fn new(scheduled_at: ScheduleAt) -> Self {
        Self {
            scheduled_id: 0,
            scheduled_at,
        }
    }
}

#[reducer]
pub fn world_update_stations_rotation(
    ctx: &spacetimedb::ReducerContext,
    update: StationRotationUpdate,
) {
    let update_interval = match update.scheduled_at {
        ScheduleAt::Interval(interval) => interval.to_duration().unwrap(),
        ScheduleAt::Time(_) => panic!("Station rotation update should be scheduled as an interval"),
    };

    let reach_angle_at = ctx
        .timestamp
        .to_duration_since_unix_epoch()
        .unwrap()
        .add(update_interval)
        .as_millis();

    debug!("Updating station rotations");
    for station in ctx.db.stations().iter() {
        let target_angle = (station.target_angle
            + station.rotation_speed * update_interval.as_secs_f32())
        .rem_euclid(TAU);

        ctx.db.stations().id().update(Station {
            target_angle,
            reach_angle_at,
            ..station
        });
    }
}

use std::{f32::consts::TAU, ops::Add};

use spacetimedb::{reducer, table, ScheduleAt};
use spacetimedsl::dsl;

use crate::tables::{GetAllStationRows, UpdateStationRowById};

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
    let dsl = dsl(ctx);

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

    for mut station in dsl.get_all_stations() {
        let target_angle = (station.get_target_angle()
            + station.get_rotation_speed() * update_interval.as_secs_f32())
        .rem_euclid(TAU);

        station.set_target_angle(target_angle);
        station.set_reach_angle_at(reach_angle_at);
        dsl.update_station_by_id(station)
            .expect("Failed to update station rotation");
    }
}

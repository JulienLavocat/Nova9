use std::time::{SystemTime, UNIX_EPOCH};

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent, ReadUpdateEvent};

use crate::{assets_loader::ModelAssets, bindings::Station as StationRow, materials::GameMaterial};

#[derive(Component, Debug, Clone)]
pub struct Station {
    pub rotation_angle: f32,
    pub reach_angle_at: u128,
}

#[derive(Resource, Default, Debug, Clone)]
struct StationsRegistry {
    registry: HashMap<u64, Entity>,
}

impl StationsRegistry {
    fn register(&mut self, id: u64, entity: Entity) {
        self.registry.insert(id, entity);
    }

    fn get(&self, id: u64) -> Option<&Entity> {
        self.registry.get(&id)
    }

    fn remove(&mut self, id: u64) {
        self.registry.remove(&id);
    }
}

pub struct StationsPlugin;

impl Plugin for StationsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StationsRegistry>()
            .add_systems(PreUpdate, spawn_stations)
            .add_systems(Update, (update_stations, rotate_stations))
            .add_systems(PostUpdate, remove_stations);
    }
}

fn spawn_stations(
    mut events: ReadInsertEvent<StationRow>,
    mut commands: Commands,
    mut registry: ResMut<StationsRegistry>,
    model_assets: Res<ModelAssets>,
) {
    for event in events.read() {
        debug!("Spawning station: {:?}", event.row);
        let station = &event.row;

        let entity = commands
            .spawn((
                Station {
                    rotation_angle: station.target_angle,
                    reach_angle_at: station.reach_angle_at,
                },
                Name::new(format!("Station {}", station.id)),
                SceneRoot(model_assets.ship_station_01.clone()),
                Transform::from_xyz(station.x, station.y, station.z),
                GameMaterial::Station,
                children![
                    SceneRoot(model_assets.ship_station_02.clone()),
                    SceneRoot(model_assets.ship_station_03.clone()),
                    SceneRoot(model_assets.ship_station_04.clone()),
                    SceneRoot(model_assets.ship_station_05.clone()),
                    SceneRoot(model_assets.ship_station_06.clone())
                ],
            ))
            .id();
        registry.register(station.id, entity);
    }
}

fn update_stations(
    mut events: ReadUpdateEvent<StationRow>,
    mut commands: Commands,
    registry: ResMut<StationsRegistry>,
) {
    for event in events.read() {
        trace!("Updating station: {:?}", event.new);
        let station = &event.new;

        if let Some(entity) = registry.get(station.id) {
            commands.entity(*entity).insert(Station {
                rotation_angle: station.target_angle,
                reach_angle_at: station.reach_angle_at,
            });
        } else {
            warn!(
                "Tried to update station with id {} but it was not found in the registry",
                station.id
            );
        }
    }
}

fn remove_stations(
    mut commands: Commands,
    mut registry: ResMut<StationsRegistry>,
    mut events: ReadDeleteEvent<StationRow>,
) {
    for event in events.read() {
        debug!("Removing station: {:?}", event.row);
        let station = &event.row;

        if let Some(entity) = registry.get(station.id) {
            commands.entity(*entity).despawn();
            registry.remove(station.id);
        } else {
            warn!(
                "Tried to remove station with id {} but it was not found in the registry",
                station.id
            );
        }
    }
}

fn rotate_stations(mut query: Query<(&mut Transform, &Station)>, time: Res<Time>) {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    for (mut transform, station) in query.iter_mut() {
        let current_rotation = transform.rotation;
        let target_rotation = Quat::from_rotation_y(station.rotation_angle);

        // Compute time remaining until target time is reached (in seconds)
        let remaining =
            ((station.reach_angle_at + 100) as i128 - now_ms as i128).max(0) as f32 / 1000.0;

        let t = (time.delta_secs() / remaining).clamp(0.0, 1.0);
        transform.rotation = current_rotation.slerp(target_rotation, t);
    }
}

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent};

use crate::{assets_loader::ModelAssets, bindings::Station as StationRow, materials::GameMaterial};

#[derive(Component, Debug, Clone)]
pub struct Station;

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
                Station,
                Name::new(format!("Station {}", station.id)),
                SceneRoot(model_assets.ship_station_01.clone()),
                Transform::from_xyz(station.x, station.y, station.z),
                GameMaterial::Station,
            ))
            .id();
        registry.register(station.id, entity);
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

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent};

use crate::{assets_loader::ModelAssets, bindings::Asteroid, materials::GameMaterial};

#[derive(Resource, Default, Debug, Clone)]
struct AsteroidsRegistry {
    registry: HashMap<u64, Entity>,
}

impl AsteroidsRegistry {
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

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidsRegistry>()
            .add_systems(PreUpdate, spawn_asteroid)
            .add_systems(PostUpdate, remove_asteroid);
    }
}

fn spawn_asteroid(
    mut events: ReadInsertEvent<Asteroid>,
    mut commands: Commands,
    mut registry: ResMut<AsteroidsRegistry>,
    model_assets: Res<ModelAssets>,
) {
    for event in events.read() {
        trace!("Spawning asteroid: {:?}", event.row);
        let asteroid = &event.row;

        let transform = Transform::from_xyz(asteroid.pos_x, asteroid.pos_y, asteroid.pos_z)
            .with_rotation(Quat::from_xyzw(
                asteroid.rot_x,
                asteroid.rot_y,
                asteroid.rot_z,
                asteroid.rot_w,
            ))
            .with_scale(Vec3::splat(asteroid.scale));

        let model = match asteroid.asteroid_type {
            0 => model_assets.asteroid_01.clone(),
            1 => model_assets.asteroid_02.clone(),
            2 => model_assets.asteroid_03.clone(),
            3 => model_assets.asteroid_04.clone(),
            4 => model_assets.asteroid_05.clone(),
            _ => {
                panic!("Unknown asteroid type: {}", asteroid.asteroid_type);
            }
        };

        let entity = commands
            .spawn((
                Name::new(format!("Asteroid {}", asteroid.id)),
                SceneRoot(model),
                GameMaterial::Standard,
                transform,
            ))
            .id();
        registry.register(asteroid.id, entity);
    }
}

fn remove_asteroid(
    mut commands: Commands,
    mut registry: ResMut<AsteroidsRegistry>,
    mut events: ReadDeleteEvent<Asteroid>,
) {
    for event in events.read() {
        trace!("Removing asteroid: {:?}", event.row);
        let asteroid = &event.row;

        if let Some(entity) = registry.get(asteroid.id) {
            commands.entity(*entity).despawn();
            registry.remove(asteroid.id);
        } else {
            warn!(
                "Tried to remove asteroid with id {} but it was not found in the registry",
                asteroid.id
            );
        }
    }
}

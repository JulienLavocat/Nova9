use avian3d::prelude::{
    AngularDamping, Collider, ExternalForce, ExternalTorque, LinearDamping, Mass,
};
use bevy::prelude::*;
use bevy_spacetimedb::{DeleteEvent, ReadDeleteEvent, ReadInsertEvent};

use crate::{
    assets_loader::{CollisionAssets, ModelAssets},
    bindings::{Ship as ShipTable, ShipTypesTableAccess},
    materials::GameMaterial,
    ships::components::Ship,
    spacetimedb::SpacetimeDB,
};

use super::resources::ShipsRegistry;

pub struct ShipsLifecyclePlugin;

impl Plugin for ShipsLifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (spawn_ship, despawn_ship).chain());
    }
}

fn spawn_ship(
    mut commands: Commands,
    mut events: ReadInsertEvent<ShipTable>,
    mut ships: ResMut<ShipsRegistry>,
    models_assets: Res<ModelAssets>,
    collision_assets: Res<CollisionAssets>,
    meshes: Res<Assets<Mesh>>,
    stdb: SpacetimeDB,
) {
    for event in events.read() {
        let ship = &event.row;

        debug!("Spawning ship: {:?}", ship);

        let ship_type = stdb
            .db()
            .ship_types()
            .id()
            .find(&ship.ship_type_id)
            .unwrap();
        let model = match ship.ship_type_id {
            1 => &models_assets.ship_bomber_01,
            _ => panic!("Unknown ship type: {}", ship.ship_type_id),
        };

        let ship_spot_light = SpotLight {
            color: Color::WHITE,
            shadows_enabled: true,
            intensity: 100_000_000.0,
            range: 500.0,
            ..Default::default()
        };

        let mesh_handle = match ship.ship_type_id {
            1 => collision_assets.ship_bomber_01.clone(),
            _ => panic!("Unknown ship type: {}", ship.ship_type_id),
        };

        let mesh = meshes
            .get(&mesh_handle)
            .expect("Failed to get mesh for ship collider");
        // I hate this
        let mesh = mesh
            .clone()
            .rotated_by(Quat::from_rotation_y(-180.0_f32.to_radians()));

        let entity = commands
            .spawn((
                Name::new(format!("Ship {}", ship.id)),
                Ship {
                    ship_type: ship.ship_type_id,
                },
                Visibility::Visible,
                Mass(ship_type.mass),
                LinearDamping(ship_type.linear_damping),
                AngularDamping(ship_type.angular_damping),
                ExternalTorque::default().with_persistence(false),
                ExternalForce::default().with_persistence(false),
                Collider::trimesh_from_mesh(&mesh).unwrap(),
                Transform::default(),
                children![
                    (
                        SceneRoot(model.clone()),
                        Transform::default()
                            .with_rotation(Quat::from_rotation_y(-180.0_f32.to_radians())),
                        GameMaterial::Ship,
                    ),
                    (ship_spot_light, Transform::from_xyz(0.0, 0.0, -15.0))
                ],
            ))
            .id();

        ships.register(ship.id, entity);
    }
}

fn despawn_ship(
    mut commands: Commands,
    mut events: ReadDeleteEvent<ShipTable>,
    mut ships: ResMut<ShipsRegistry>,
) {
    for event in events.read() {
        let ship = &event.row;
        debug!("Despawning ship: {:?}", ship);
        if let Some(ship_data) = ships.get(ship.id) {
            commands.entity(ship_data.entity()).despawn();
            ships.remove(ship.id);
        } else {
            warn!("Ship[{}] not found for despawn", ship.id);
            commands.send_event(DeleteEvent { row: ship.clone() });
        }
    }
}

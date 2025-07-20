use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::assets_loader::MaterialAssets;

#[derive(Component)]
pub enum GameMaterial {
    Standard,
    Station,
    Ship,
}

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(apply_synty_material);
    }
}

fn apply_synty_material(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    material_targets: Query<&GameMaterial>,
    materials: Res<MaterialAssets>,
) {
    let material_target = material_targets.get(trigger.target());
    if material_target.ok().is_none() {
        return;
    }

    for descendants in children.iter_descendants(trigger.target()) {
        match material_target.unwrap() {
            GameMaterial::Standard => {
                commands
                    .entity(descendants)
                    .insert(MeshMaterial3d(materials.base_01.clone()));
            }
            GameMaterial::Station => {
                commands
                    .entity(descendants)
                    .remove::<MeshMaterial3d<StandardMaterial>>()
                    .insert(MeshMaterial3d(materials.spacestation.clone()));
            }
            GameMaterial::Ship => {
                commands
                    .entity(descendants)
                    .remove::<MeshMaterial3d<StandardMaterial>>()
                    .insert(MeshMaterial3d(materials.spaceship.clone()));
            }
        };
    }
}

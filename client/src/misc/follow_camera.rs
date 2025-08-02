use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct FollowCamera {
    /// The entity that the camera will follow
    target: Entity,
    /// The offset from the target's position
    offset: Vec3,
    /// The point the camera will look at, relative to the target's position
    look_at: Vec3,
    /// Damping factors for each axis to smooth the camera movement
    damping: Vec3,
}

impl FollowCamera {
    pub fn new(target: Entity, offset: Vec3, look_at: Vec3, damping: Vec3) -> Self {
        Self {
            target,
            offset,
            look_at,
            damping,
        }
    }
}

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, follow_camera);
    }
}

fn follow_camera(
    mut camera_query: Query<(&mut Transform, &FollowCamera)>,
    target_query: Query<&GlobalTransform>,
) -> Result {
    for (mut cam_transform, follow) in &mut camera_query {
        if let Ok(target_transform) = target_query.get(follow.target) {
            // The code below feels OK-ish but needs to be improved for a better feel when playing
            // to be usable
            // This is probably dues to the system's schedule being app.add_systems(PhysicsSchedule, follow_camera.after(PhysicsStepSet::Last))
            // let target_pos = target_transform.transform_point(follow.offset);
            // let look_at_pos = target_transform.transform_point(follow.look_at);
            //
            // cam_transform.translation = damp_vec3(
            //     cam_transform.translation,
            //     target_pos,
            //     follow.damping,
            //     time.delta_secs(),
            // );
            // cam_transform.look_at(look_at_pos, target_transform.up());

            let target_pos = target_transform.transform_point(follow.offset);
            let look_at_pos = target_transform.transform_point(follow.look_at);

            cam_transform.translation = target_pos;
            cam_transform.look_at(look_at_pos, target_transform.up());
        }
    }

    Ok(())
}

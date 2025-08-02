use bevy::prelude::*;
use follow_camera::FollowCameraPlugin;

pub mod follow_camera;

pub struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FollowCameraPlugin);
    }
}

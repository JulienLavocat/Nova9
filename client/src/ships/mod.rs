use bevy::prelude::*;
use controls::ShipControlsPlugin;
use lifecycle::ShipsLifecyclePlugin;
use resources::ShipsRegistry;

mod components;
mod controls;
mod lifecycle;
mod resources;

pub struct ShipsPlugin;

impl Plugin for ShipsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShipsRegistry>()
            .add_plugins(ShipsLifecyclePlugin)
            .add_plugins(ShipControlsPlugin);
    }
}

use bevy::prelude::*;
use controls::ShipControlsPlugin;
use lifecycle::ShipsLifecyclePlugin;
use location_updates::ShipLocationUpdatesPlugin;
use resources::ShipsRegistry;

mod components;
mod controls;
mod lifecycle;
mod location_updates;
mod resources;

pub use components::Ship;

pub struct ShipsPlugin;

impl Plugin for ShipsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShipsRegistry>()
            .add_plugins(ShipLocationUpdatesPlugin)
            .add_plugins(ShipsLifecyclePlugin)
            .add_plugins(ShipControlsPlugin);
    }
}

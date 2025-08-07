use crate::tables::PlayerId;
use spacetimedb::{table, Identity};
use spacetimedsl::dsl;

#[dsl(plural_name = player_locations)]
#[table(name = player_location, public)]
pub struct PlayerLocation {
    #[primary_key]
    #[use_wrapper(path = PlayerId)]
    player_id: Identity,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
}

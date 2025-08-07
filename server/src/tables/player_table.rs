use spacetimedb::{table, Identity};
use spacetimedsl::dsl;

#[dsl(plural_name = players)]
#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    #[create_wrapper]
    #[referenced_by(path = crate::tables, table = ship_pilot)]
    #[referenced_by(path = crate::tables, table = player_location)]
    id: Identity,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
}

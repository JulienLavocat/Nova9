use crate::tables::ShipId;
use spacetimedb::table;
use spacetimedsl::{dsl, Wrapper};

#[dsl(plural_name = ship_locations)]
#[table(name = ship_location, public)]
pub struct ShipLocation {
    #[primary_key]
    #[use_wrapper(path = ShipId)]
    #[foreign_key(path = crate::tables, table = ship, column = id, on_delete = Delete)]
    ship_id: u64,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
}

impl ShipLocation {
    pub fn new(
        ship_id: ShipId,
        x: f32,
        y: f32,
        z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
        rot_w: f32,
    ) -> Self {
        Self {
            ship_id: ship_id.value(),
            x,
            y,
            z,
            rot_x,
            rot_y,
            rot_z,
            rot_w,
        }
    }
}

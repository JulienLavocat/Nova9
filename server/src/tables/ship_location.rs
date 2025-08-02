use spacetimedb::table;

#[table(name = ship_locations, public)]
pub struct ShipLocation {
    #[primary_key]
    pub ship_id: u64,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
}

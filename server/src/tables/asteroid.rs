use spacetimedb::table;

#[table(name = asteroids, public)]
pub struct Asteroid {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
    pub asteroid_type: u8,
    pub scale: f32,
}

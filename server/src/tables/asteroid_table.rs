use spacetimedb::table;
use spacetimedsl::dsl;

#[dsl(plural_name = asteroids)]
#[table(name = asteroid, public)]
pub struct Asteroid {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    rot_w: f32,
    asteroid_type: u8,
    scale: f32,
}

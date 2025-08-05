use spacetimedb::table;
use spacetimedsl::dsl;

#[dsl(plural_name = stations)]
#[table(name = station, public)]
pub struct Station {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,
    name: String,
    x: f32,
    y: f32,
    z: f32,
    rotation_speed: f32,
    pub target_angle: f32,
    pub reach_angle_at: u128,
}

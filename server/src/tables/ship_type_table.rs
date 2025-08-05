use spacetimedb::table;
use spacetimedsl::dsl;

#[dsl(plural_name = ship_types)]
#[table(name = ship_type, public)]
pub struct ShipType {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,
    name: String,
    camera_offset_x: f32,
    camera_offset_y: f32,
    camera_offset_z: f32,
    mass: f32,
    linear_damping: f32,
    angular_damping: f32,
    thrust: f32,
    vertical_thrust: f32,
    lateral_thrust: f32,
    pitch_torque: f32,
    yaw_torque: f32,
    roll_torque: f32,
}

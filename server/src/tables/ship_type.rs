use spacetimedb::table;

#[table(name = ship_types, public)]
pub struct ShipType {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub camera_offset_x: f32,
    pub camera_offset_y: f32,
    pub camera_offset_z: f32,
    pub mass: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub thrust: f32,
    pub vertical_thrust: f32,
    pub lateral_thrust: f32,
    pub pitch_torque: f32,
    pub yaw_torque: f32,
    pub roll_torque: f32,
}

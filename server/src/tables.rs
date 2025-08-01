use spacetimedb::{table, Identity};

#[table(name = stations, public)]
pub struct Station {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

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
    pub vertical_trhust: f32,
    pub lateral_thrust: f32,
    pub pitch_torque: f32,
    pub yaw_torque: f32,
    pub roll_torque: f32,
}

#[table(name = players, public)]
pub struct Player {
    #[primary_key]
    pub id: Identity,
}

#[table(name = ships, public)]
pub struct Ship {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub ship_type_id: u64,
    #[index(btree)]
    pub owner_id: Identity,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[table(name = ship_pilots, public)]
pub struct ShipPilot {
    #[primary_key]
    pub ship_id: u64,
    #[unique]
    pub player_id: Identity,
}

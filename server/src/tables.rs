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
    pub speed: f32,
    pub rotation_speed: f32,
}

#[table(name = players, public)]
pub struct Player {
    #[primary_key]
    pub id: Identity,
}

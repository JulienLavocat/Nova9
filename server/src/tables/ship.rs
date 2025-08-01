use spacetimedb::{table, Identity};

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

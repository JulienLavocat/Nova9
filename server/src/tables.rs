use spacetimedb::table;

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

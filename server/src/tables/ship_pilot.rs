use spacetimedb::{table, Identity};

#[table(name = ship_pilots, public)]
pub struct ShipPilot {
    #[primary_key]
    pub ship_id: u64,
    #[unique]
    pub player_id: Identity,
}

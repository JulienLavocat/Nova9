use crate::tables::ShipId;
use spacetimedb::{table, Identity};
use spacetimedsl::dsl;

#[dsl(plural_name = ship_pilots)]
#[table(name = ship_pilot, public)]
pub struct ShipPilot {
    #[primary_key]
    #[use_wrapper(path = ShipId)]
    ship_id: u64,
    #[unique]
    #[create_wrapper]
    player_id: Identity,
}

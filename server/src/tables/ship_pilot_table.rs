use crate::tables::{PlayerId, ShipId};
use spacetimedb::{table, Identity};
use spacetimedsl::dsl;

#[dsl(plural_name = ship_pilots)]
#[table(name = ship_pilot, public)]
pub struct ShipPilot {
    #[primary_key]
    #[use_wrapper(path = ShipId)]
    #[foreign_key(path = crate::tables, table = ship, column = id, on_delete = Delete)]
    ship_id: u64,

    #[unique]
    #[use_wrapper(path = PlayerId)]
    #[foreign_key(path = crate::tables, table = player, column = id, on_delete = Delete)]
    player_id: Identity,
}

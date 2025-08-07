use crate::tables::PlayerId;
use spacetimedb::{table, Identity};
use spacetimedsl::dsl;

#[dsl(plural_name = ships)]
#[table(name = ship, public)]
pub struct Ship {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::tables, table = ship_pilot)]
    #[referenced_by(path = crate::tables, table = ship_location)]
    id: u64,

    // TODO: Set foreign keys
    ship_type_id: u64,

    #[index(btree)]
    #[use_wrapper(path = PlayerId)]
    owner_id: Identity,
}

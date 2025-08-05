use crate::tables::PlayerId;
use spacetimedb::table;
use spacetimedsl::dsl;

#[dsl(plural_name = ships)]
#[table(name = ship, public)]
pub struct Ship {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,

    ship_type_id: u64,

    #[index(btree)]
    #[use_wrapper(path = PlayerId)]
    owner_id: u64,
}

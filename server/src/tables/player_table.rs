use spacetimedb::{table, Identity};
use spacetimedsl::dsl;

#[dsl(plural_name = players)]
#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    #[create_wrapper]
    #[referenced_by(path = crate::tables, table = ship_pilot)]
    id: Identity,
}

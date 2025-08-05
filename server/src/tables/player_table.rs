use spacetimedb::{table, Identity};
use spacetimedsl::dsl;

#[dsl(plural_name = players)]
#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,

    #[unique]
    #[create_wrapper]
    identity: Identity,
}

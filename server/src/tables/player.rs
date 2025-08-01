use spacetimedb::{table, Identity};

#[table(name = players, public)]
pub struct Player {
    #[primary_key]
    pub id: Identity,
}

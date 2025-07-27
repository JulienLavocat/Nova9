use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Ship {
    pub ship_id: u64,
}

#[derive(Component, Debug)]
pub struct ControlledShip;

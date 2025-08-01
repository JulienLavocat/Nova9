use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Ship {
    pub ship_type: u64,
}

#[derive(Component, Debug)]
pub struct ControlledShip;

#[derive(Component, Debug)]
pub struct ShipRotationTarget;

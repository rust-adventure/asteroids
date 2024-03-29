use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum PlayerShipType {
    A,
    B,
    C,
}

impl PlayerShipType {
    pub fn base_atlas_index(&self) -> usize {
        match &self {
            PlayerShipType::A => 200,
            PlayerShipType::B => 207,
            PlayerShipType::C => 214,
        }
    }
    pub fn all_ships() -> Vec<PlayerShipType> {
        vec![
            PlayerShipType::A,
            PlayerShipType::B,
            PlayerShipType::C,
        ]
    }
    pub fn base_ship_speed(&self) -> BaseShipSpeed {
        match self {
            PlayerShipType::A => BaseShipSpeed {
                movement_speed: 500.0, // meters per second
                rotation_speed: f32::to_radians(360.0), /* degrees per second */
            },
            PlayerShipType::B => BaseShipSpeed {
                movement_speed: 500.0, // meters per second
                rotation_speed: f32::to_radians(360.0), /* degrees per second */
            },
            PlayerShipType::C => BaseShipSpeed {
                movement_speed: 500.0, // meters per second
                rotation_speed: f32::to_radians(360.0), /* degrees per second */
            },
        }
    }
}

pub struct BaseShipSpeed {
    /// linear speed in meters per second
    pub movement_speed: f32,
    /// rotation speed in radians per second
    pub rotation_speed: f32,
}

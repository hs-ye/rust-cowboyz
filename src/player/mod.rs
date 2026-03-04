pub mod actions;
pub mod inventory;
pub mod ship;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub money: u32,
    pub location: String,
    pub ship: ship::Ship,
    pub inventory: inventory::CargoHold,
}

// TODO refactor so we load player initial data from yaml config
impl Player {
    pub fn new() -> Self {
        Player {
            money: 1000, // Starting money
            location: "earth".to_string(), // Starting planet
            ship: ship::Ship::new(10.0, 10), // Default ship speed and cargo capacity (MVP specifies 10 units)
            inventory: inventory::CargoHold::new(10), // Default cargo hold capacity (MVP specifies 10 units)
        }
    }
}

pub mod actions;
pub mod inventory;
pub mod ship;

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
            ship: ship::Ship::new(10.0, 100), // Default ship speed and cargo capacity
            inventory: inventory::CargoHold::new(100), // Default cargo hold capacity
        }
    }
}

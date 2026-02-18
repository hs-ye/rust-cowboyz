use clap::{Parser, Subcommand};

use crate::setup::World;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display current game status, player status, and market information
    Status {},
    /// Buy goods from the current planet's market
    Buy {
        good_id: String,
        quantity: u32,
    },
    /// Sell goods to the current planet's market
    Sell {
        good_id: String,
        quantity: u32,
    },
    /// Travel to a different planet
    Travel {
        destination_planet_id: String,
    },
    /// Wait at the current location, advancing time
    Wait {
        months: u32,
    },
    /// Show information about a specific planet
    PlanetInfo {
        planet_id: String,
    },
    /// Exit the game
    Quit,
}

pub fn display_game_status(world: &World) -> String {
    format!("--- Game Status ---\nCurrent Turn: {} / Total Turns: {}\n",
           world.game_clock.current_turn, world.game_clock.total_turns)
}

pub fn display_player_status(world: &World) -> String {
    let mut goods_list = String::new();
    if world.player.inventory.goods.is_empty() {
        goods_list.push_str("  (empty)");
    } else {
        for (good_id, quantity) in world.player.inventory.get_goods_list() {
            goods_list.push_str(&format!("  {} x {}\n", good_id, quantity));
        }
    }

    format!("--- Player Status ---\nLocation: {}\nMoney: {}\nCargo: {}/{}\nGoods:\n{}",
           world.player.location, world.player.money, world.player.inventory.current_load(), world.player.inventory.capacity, goods_list)
}

pub fn display_market_status(world: &World) -> String {
    let mut market_list = String::new();
    let current_planet = world.planets.iter().find(|p| p.id == world.player.location);

    if let Some(planet) = current_planet {
        market_list.push_str("Good           Buy Price   Sell Price\n");
        market_list.push_str("---------------------------------------\n");
        for market_good in &planet.economy.market {
            market_list.push_str(&format!("{:<14} {:<12} {:<12}\n",
                                           market_good.good.id,
                                           market_good.buy_price,
                                           market_good.sell_price));
        }
    } else {
        market_list.push_str("Market information not available for current location.");
    }

    format!("--- Market Status ({}) ---\n{}", world.player.location, market_list)
}

pub fn display_travel_options(world: &World) -> String {
    let mut travel_list = String::new();
    
    let current_planet = world.planets.iter()
        .find(|p| p.id == world.player.location)
        .expect("Player is not at a valid planet");
        
    for planet in &world.planets {
        if planet.id != world.player.location {
            let travel_time = crate::simulation::travel::calculate_travel_time(current_planet, planet, world.player.ship.speed);
            travel_list.push_str(&format!("Travel to {} (Time: {} months)\n", planet.id, travel_time));
        }
    }

    format!("--- Available Destinations ---\n{}", travel_list)
}

pub fn display_planet_info(world: &World, planet_id: &str) -> String {
    let planet = world.planets.iter()
        .find(|p| p.id == planet_id)
        .ok_or_else(|| format!("Planet '{}' not found", planet_id))
        .expect("Planet not found");

    let mut market_list = String::new();
    market_list.push_str("Good           Buy Price   Sell Price\n");
    market_list.push_str("---------------------------------------\n");
    for market_good in &planet.economy.market {
        market_list.push_str(&format!("{:<14} {:<12} {:<12}\n",
                                       market_good.good.id,
                                       market_good.buy_price,
                                       market_good.sell_price));
    }

    format!("--- Market Status ({}) ---\n{}", planet.id, market_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::World;
    use crate::player::{Player, inventory::CargoHold, ship::Ship};
    use crate::simulation::{economy::{Good, MarketGood, PlanetEconomy}, orbits::{Planet, Position}};
    use crate::game_state::GameClock;

    // Helper function to create a mock World instance
    fn create_mock_world() -> World {
        let good_food = Good { id: "Food".to_string(), base_value: 10 };
        let good_water = Good { id: "Water".to_string(), base_value: 5 };

        let market_earth_food = MarketGood {
            good: good_food.clone(),
            buy_price: 8,
            sell_price: 12,
            supply: 1.0,
            demand: 1.0,
            is_produced: true,
            is_demanded: false,
        };
        let market_earth_water = MarketGood {
            good: good_water.clone(),
            buy_price: 4,
            sell_price: 6,
            supply: 1.0,
            demand: 1.0,
            is_produced: false,
            is_demanded: true,
        };

        let planet_earth = Planet {
            id: "Earth".to_string(),
            orbit_radius: 1.0,
            orbit_period: 12.0,
            position: Position { x: 1.0, y: 0.0 },
            economy: PlanetEconomy { market: vec![market_earth_food, market_earth_water] },
        };

        let planet_mars = Planet {
            id: "Mars".to_string(),
            orbit_radius: 1.5,
            orbit_period: 24.0,
            position: Position { x: -1.5, y: 0.0 },
            economy: PlanetEconomy { market: vec![] }, // Empty market for simplicity
        };

        let mut player_inventory = CargoHold::new(100);
        player_inventory.add_good("Food".to_string(), 5).unwrap();

        World {
            goods: vec![good_food, good_water],
            planets: vec![planet_earth, planet_mars],
            current_time: 0.0,
            player: Player {
                money: 1000,
                location: "Earth".to_string(),
                ship: Ship::new(10.0, 100),
                inventory: player_inventory,
            },
            game_clock: GameClock {
                current_turn: 1,
                total_turns: 100,
            },
        }
    }

    #[test]
    fn test_display_game_status() {
        let world = create_mock_world();
        let output = display_game_status(&world);
        assert!(output.contains("--- Game Status ---"));
        assert!(output.contains("Current Turn: 1 / Total Turns: 100"));
    }

    #[test]
    fn test_display_player_status() {
        let world = create_mock_world();
        let output = display_player_status(&world);
        assert!(output.contains("--- Player Status ---"));
        assert!(output.contains("Location: Earth"));
        assert!(output.contains("Money: 1000"));
        assert!(output.contains("Cargo: 5/100"));
        assert!(output.contains("Goods:"));
        assert!(output.contains("Food x 5"));
    }

    #[test]
    fn test_display_market_status() {
        let world = create_mock_world();
        let output = display_market_status(&world);
        assert!(output.contains("--- Market Status (Earth) ---"));
        assert!(output.contains("Good           Buy Price   Sell Price"));
        assert!(output.contains("Food"));
        assert!(output.contains("8"));
        assert!(output.contains("12"));
        assert!(output.contains("Water"));
        assert!(output.contains("4"));
        assert!(output.contains("6"));
    }

    #[test]
    fn test_display_travel_options() {
        let world = create_mock_world();
        let output = display_travel_options(&world);
        assert!(output.contains("--- Available Destinations ---"));
        assert!(output.contains("Travel to Mars (Time: "));
        assert!(!output.contains("Travel to Earth")); // Should not list current planet
    }

    // Placeholder command tests
    // These will be tested by calling the main CLI executable with assert_cmd
    // For now, we'll just ensure the commands are defined.
}

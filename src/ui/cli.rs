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
}

pub fn display_game_status(world: &World) {
    println!("--- Game Status ---");
    println!("Current Turn: {} / Total Turns: {}", world.game_clock.current_turn, world.game_clock.total_turns);
    println!("");
}

pub fn display_player_status(world: &World) {
    println!("--- Player Status ---");
    println!("Location: {}", world.player.location);
    println!("Money: {}", world.player.money);
    println!("Cargo: {}/{}", world.player.inventory.current_load(), world.player.inventory.capacity);
    println!("Goods:");
    if world.player.inventory.goods.is_empty() {
        println!("  (empty)");
    } else {
        for (good_id, quantity) in world.player.inventory.get_goods_list() {
            println!("  {} x {}", good_id, quantity);
        }
    }
    println!("");
}

pub fn display_market_status(world: &World) {
    println!("--- Market Status ({}) ---", world.player.location);
    let current_planet = world.planets.iter().find(|p| p.id == world.player.location);

    if let Some(planet) = current_planet {
        println!("Good           Buy Price   Sell Price");
        println!("---------------------------------------");
        for market_good in &planet.economy.market {
            println!("{:<14} {:<12} {:<12}",
                       market_good.good.id,
                       market_good.buy_price,
                       market_good.sell_price);
        }
    } else {
        println!("Market information not available for current location.");
    }
    println!("");
}

pub fn display_travel_options(world: &World) {
    println!("--- Available Destinations ---");
    for planet in &world.planets {
        if planet.id != world.player.location {
            // Placeholder for travel time calculation
            println!("Travel to {} (Time: TBD months)", planet.id);
        }
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::World;
    use crate::player::{Player, inventory::CargoHold, ship::Ship};
    use crate::simulation::{economy::{Good, MarketGood, PlanetEconomy}, orbits::{Planet, Position}};
    use crate::game_state::GameClock;
    use std::io::{self, Write};
    use std::sync::Mutex;

    // A global mutex to protect stdout during tests
    static STDOUT_MUTEX: Mutex<()> = Mutex::new(());

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

    // Helper to capture stdout
    fn capture_stdout<F>(f: F) -> String
    where
        F: FnOnce(),
    {
        let _guard = STDOUT_MUTEX.lock().unwrap(); // Acquire lock
        let original_stdout = io::stdout();
        let (pipe_read, pipe_write) = os_pipe::pipe().unwrap();
        let mut captured_output = String::new();

        // Redirect stdout to the write end of the pipe
        unsafe {
            libc::dup2(pipe_write.as_raw_fd(), 1);
        }

        // Execute the function that prints to stdout
        f();

        // Restore original stdout
        unsafe {
            libc::dup2(original_stdout.as_raw_fd(), 1);
        }

        // Read captured output from the read end of the pipe
        let mut reader = io::BufReader::new(pipe_read);
        reader.read_to_string(&mut captured_output).unwrap();

        captured_output
    }

    #[test]
    fn test_display_game_status() {
        let world = create_mock_world();
        let output = capture_stdout(|| {
            display_game_status(&world);
        });
        assert!(output.contains("--- Game Status ---"));
        assert!(output.contains("Current Turn: 1 / Total Turns: 100"));
    }

    #[test]
    fn test_display_player_status() {
        let world = create_mock_world();
        let output = capture_stdout(|| {
            display_player_status(&world);
        });
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
        let output = capture_stdout(|| {
            display_market_status(&world);
        });
        assert!(output.contains("--- Market Status (Earth) ---"));
        assert!(output.contains("Good           Buy Price   Sell Price"));
        assert!(output.contains("Food             8           12"));
        assert!(output.contains("Water            4           6"));
    }

    #[test]
    fn test_display_travel_options() {
        let world = create_mock_world();
        let output = capture_stdout(|| {
            display_travel_options(&world);
        });
        assert!(output.contains("--- Available Destinations ---"));
        assert!(output.contains("Travel to Mars (Time: TBD months)"));
        assert!(!output.contains("Travel to Earth")); // Should not list current planet
    }

    // Placeholder command tests
    // These will be tested by calling the main CLI executable with assert_cmd
    // For now, we'll just ensure the commands are defined.
}

// src/setup.rs

use crate::assets::config_loader;
use crate::simulation::{economy, orbits};

/// Initializes the game world from configuration files.
///
/// This function orchestrates the loading of game data from configuration files,
/// creates the initial game state, and sets the starting positions of the planets.
pub fn initialize_world(
    goods_config_path: &str,
    planets_config_path: &str,
) -> World {
    let goods = load_goods(goods_config_path);
    let planets = load_planets(planets_config_path, &goods);

    let mut world = World {
        goods,
        planets,
        current_time: 0.0, // Game starts at time 0
        player: crate::player::Player::new(),
        game_clock: crate::game_state::GameClock {
            current_turn: 1,
            total_turns: 100,
        },
    };

    world.initialize_positions();
    world
}

/// Loads and maps goods from the configuration file.
fn load_goods(path: &str) -> Vec<economy::Good> {
    config_loader::load_goods_config(path)
        .into_iter()
        .map(|config| economy::Good {
            id: config.name, // The `name` from config is the `id` in the game
            base_value: config.base_value,
        })
        .collect()
}

/// Loads planets from configuration and initializes their economies.
fn load_planets(path: &str, goods: &[economy::Good]) -> Vec<orbits::Planet> {
    let configs = config_loader::load_planets_config(path);

    configs
        .into_iter()
        .map(|config| {
            // Use the produces and demands from the config file
            let market = initialize_market(goods, &config.produces, &config.demands);

            orbits::Planet {
                id: config.id,
                orbit_radius: config.orbit_radius,
                orbit_period: config.orbit_period,
                position: orbits::Position { x: 0.0, y: 0.0 }, // Initial position is calculated later
                economy: economy::PlanetEconomy { market },
            }
        })
        .collect()
}

/// Initializes a planet's market based on its production and demand.
fn initialize_market(
    goods: &[economy::Good],
    produced: &[String],
    demanded: &[String],
) -> Vec<economy::MarketGood> {
    goods
        .iter()
        .map(|good| {
            let base_price = good.base_value;
            let (buy_price, sell_price) = if produced.contains(&good.id) {
                // Produced goods: Player sells to planet, so planet's buy price is low.
                ( (base_price as f64 * 0.8) as u32, (base_price as f64 * 0.6) as u32)
            } else if demanded.contains(&good.id) {
                // Demanded goods: Player buys from planet, so planet's sell price is high.
                ( (base_price as f64 * 1.4) as u32, (base_price as f64 * 1.6) as u32)
            } else {
                // Neutral goods
                (base_price, (base_price as f64 * 1.2) as u32)
            };

            economy::MarketGood {
                good: good.clone(), // Clone the good prototype
                buy_price,
                sell_price,
                supply: 1.0,
                demand: 1.0,
                is_produced: produced.contains(&good.id),
                is_demanded: demanded.contains(&good.id),
            }
        })
        .collect()
}

/// Represents the complete state of the game world.
pub struct World {
    pub goods: Vec<economy::Good>,
    pub planets: Vec<orbits::Planet>,
    pub current_time: f64, // In-game time, measured in months
    pub player: crate::player::Player,
    pub game_clock: crate::game_state::GameClock,
}

impl World {
    /// Calculates and sets the initial orbital positions of all planets.
    pub fn initialize_positions(&mut self) {
        for planet in &mut self.planets {
            planet.position = orbits::calculate_orbit_position(
                planet.orbit_radius,
                planet.orbit_period,
                self.current_time,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_initialize_world_from_config() {
        // 1. Create a temporary directory for our config files
        let dir = tempdir().expect("Failed to create temp dir");
        let goods_path = dir.path().join("goods.yaml");
        let planets_path = dir.path().join("planets.yaml");

        // 2. Create and write mock config files
        let mut goods_file = File::create(&goods_path).expect("Failed to create goods file");
        goods_file.write_all(b"
- name: Food
  base_value: 10
- name: Machinery
  base_value: 100
").expect("Failed to write goods file");

        let mut planets_file = File::create(&planets_path).expect("Failed to create planets file");
        planets_file.write_all(b"
- id: test_earth
  orbit_radius: 1.0
  orbit_period: 12.0
  produces: [Food]
  demands: [Machinery]
- id: test_mars
  orbit_radius: 1.5
  orbit_period: 24.0
  produces: [Machinery]
  demands: [Food]
").expect("Failed to write planets file");

        // 3. Call the function we are testing
        let world = initialize_world(
            goods_path.to_str().unwrap(),
            planets_path.to_str().unwrap(),
        );

        // 4. Assert the world state is correct
        assert_eq!(world.goods.len(), 2);
        assert_eq!(world.planets.len(), 2);
        assert_eq!(world.current_time, 0.0);

        // 5. Assert specific planet data is correct
        let earth = world.planets.iter().find(|p| p.id == "test_earth").expect("Planet 'test_earth' not found");
        assert_eq!(earth.orbit_radius, 1.0);

        let earth_food_market = earth.economy.market.iter().find(|mg| mg.good.id == "Food").unwrap();
        assert!(earth_food_market.is_produced);
        assert!(!earth_food_market.is_demanded);
        // Check that produced goods have lower prices
        assert_eq!(earth_food_market.buy_price, 8);
        assert_eq!(earth_food_market.sell_price, 6);

        let earth_machinery_market = earth.economy.market.iter().find(|mg| mg.good.id == "Machinery").unwrap();
        assert!(!earth_machinery_market.is_produced);
        assert!(earth_machinery_market.is_demanded);
        // Check that demanded goods have higher prices
        assert_eq!(earth_machinery_market.buy_price, 140);
        assert_eq!(earth_machinery_market.sell_price, 160);

        // 6. Assert that initial positions have been calculated
        for planet in world.planets {
            let expected_pos = orbits::calculate_orbit_position(planet.orbit_radius, planet.orbit_period, 0.0);
            assert_eq!(planet.position, expected_pos);
        }
    }
}

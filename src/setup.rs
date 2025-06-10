use crate::assets::config_loader;
use crate::simulation::{economy, orbits, travel};
use rand::{seq::SliceRandom, Rng};
use std::collections::HashMap;

/// Initializes the game world from configuration files
pub fn initialize_world(
    goods_config_path: &str,
    planets_config_path: &str,
) -> World {
    let goods: Vec<economy::goods> = load_goods(goods_config_path);
    let planets: Vec<_> = load_planets(planets_config_path, &goods);
    
    World {
        goods,
        planets,
        current_time: 0.0, // Game starts at time 0
    }
}

/// Loads goods from configuration
fn load_goods(path: &str) -> Vec<economy::Good> {
    config_loader::load_goods_config(path)
        .into_iter()
        .map(|config| economy::Good {
            id: config.id,
            base_value: config.base_value,
        })
        .collect()
}

/// Loads planets with randomized economies
fn load_planets(path: &str, goods: &[economy::Good]) -> Vec<orbits::Planet> {
    let configs = config_loader::load_planets_config(path);
    let mut rng = rand::thread_rng();
    
    configs
        .into_iter()
        .map(|config| {
            // Randomly assign produced and demanded goods
            let (produced, demanded) = randomize_economy(&mut rng, goods);
            
            // Create market with initial prices
            let market = initialize_market(goods, &produced, &demanded);
            
            orbits::Planet {
                id: config.id,
                orbit_radius: config.orbit_radius,
                orbit_period: config.orbit_period,
                position: orbits::Position { x: 0.0, y: 0.0 }, // Initial position calculated later
                economy: economy::PlanetEconomy {
                    produced_goods: produced,
                    demanded_goods: demanded,
                    market,
                },
            }
        })
        .collect()
}

/// Randomizes which goods a planet produces and demands
fn randomize_economy<R: Rng>(
    rng: &mut R,
    goods: &[economy::Good],
) -> (Vec<String>, Vec<String>) {
    let mut produced = Vec::new();
    let mut demanded = Vec::new();
    
    // Ensure at least one produced and one demanded good
    produced.push(goods.choose(rng).unwrap().id.clone());
    demanded.push(
        goods
            .iter()
            .find(|g| !produced.contains(&g.id))
            .unwrap_or(&goods[0])
            .id
            .clone(),
    );
    
    // Randomly add more goods (0-2 additional)
    for _ in 0..rng.gen_range(0..3) {
        if let Some(good) = goods.choose(rng) {
            produced.push(good.id.clone());
        }
    }
    
    for _ in 0..rng.gen_range(0..3) {
        if let Some(good) = goods.iter().find(|g| !produced.contains(&g.id)).and_then(|g| Some(g)) {
            demanded.push(good.id.clone());
        }
    }
    
    (produced, demanded)
}

/// Initializes market prices based on production/demand
fn initialize_market(
    goods: &[economy::Good],
    produced: &[String],
    demanded: &[String],
) -> HashMap<String, economy::MarketGood> {
    goods
        .iter()
        .map(|good| {
            let base_price = good.base_value;
            let (buy_price, sell_price) = if produced.contains(&good.id) {
                // Produced goods: low buy price, very low sell price
                (base_price * 0.8, base_price * 0.6)
            } else if demanded.contains(&good.id) {
                // Demanded goods: high buy price, very high sell price
                (base_price * 1.4, base_price * 1.6)
            } else {
                // Neutral goods
                (base_price, base_price * 1.2)
            };
            
            (
                good.id.clone(),
                economy::MarketGood {
                    buy_price,
                    sell_price,
                    supply: 1.0, // Initial supply level
                    demand: 1.0, // Initial demand level
                },
            )
        })
        .collect()
}

/// Represents the game world state
pub struct World {
    pub goods: Vec<economy::Good>,
    pub planets: Vec<orbits::Planet>,
    pub current_time: f64, // In-game time (months)
}

impl World {
    /// Initializes planetary positions based on current time
    pub fn initialize_positions(&mut self) {
        for planet in &mut self.planets {
            planet.position = orbits::calculate_position(
                planet.orbit_radius,
                planet.orbit_period,
                self.current_time,
            );
        }
    }
}
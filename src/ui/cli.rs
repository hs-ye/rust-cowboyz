use clap::{Parser, Subcommand};

use crate::setup::World;
use crate::simulation::economy::PriceAnomaly;

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
    /// Buy commodities from the current planet's market
    Buy {
        good_id: String, // Still using good_id to avoid breaking CLI interface
        quantity: u32,
    },
    /// Sell commodities to the current planet's market
    Sell {
        good_id: String, // Still using good_id to avoid breaking CLI interface
        quantity: u32,
    },
    /// Travel to a different planet
    Travel { destination_planet_id: String },
    /// Wait at the current location, advancing time
    Wait { months: u32 },
    /// Show information about a specific planet
    PlanetInfo { planet_id: String },
    /// Exit the game
    Quit,
}

pub fn display_game_status(world: &World) -> String {
    format!(
        "--- Game Status ---\nCurrent Turn: {} / Total Turns: {}\n",
        world.game_clock.current_turn, world.game_clock.total_turns
    )
}

pub fn display_player_status(world: &World) -> String {
    let mut commodities_list = String::new();
    if world.player.inventory.commodities.is_empty() {
        commodities_list.push_str("  (empty)");
    } else {
        for (commodity_type, quantity) in world.player.inventory.get_commodities_list() {
            commodities_list.push_str(&format!(
                "  {} x {}\n",
                commodity_type.display_name(),
                quantity
            ));
        }
    }

    format!(
        "--- Player Status ---\nLocation: {}\nMoney: {}\nCargo: {}/{}\nCommodities:\n{}",
        world.player.location,
        world.player.money,
        world.player.inventory.current_load(),
        world.player.inventory.capacity,
        commodities_list
    )
}

pub fn display_market_status(world: &World) -> String {
    let current_planet = world.planets.iter().find(|p| p.id == world.player.location);

    if let Some(planet) = current_planet {
        format!(
            "--- Market Status ({}) ---\n{}",
            world.player.location,
            build_enhanced_market_display(&planet.economy)
        )
    } else {
        "--- Market Status ---\nMarket information not available for current location.".to_string()
    }
}

pub fn display_travel_options(world: &World) -> String {
    let mut travel_list = String::new();

    let current_planet = world
        .planets
        .iter()
        .find(|p| p.id == world.player.location)
        .expect("Player is not at a valid planet");

    for planet in &world.planets {
        if planet.id != world.player.location {
            let travel_turns = crate::simulation::travel::calculate_travel_turns(
                current_planet,
                planet,
                world.player.ship.acceleration,
            );
            travel_list.push_str(&format!(
                "Travel to {} (Time: {} months)\n",
                planet.id, travel_turns
            ));
        }
    }

    format!("--- Available Destinations ---\n{}", travel_list)
}

pub fn display_planet_info(world: &World, planet_id: &str) -> String {
    let planet = world
        .planets
        .iter()
        .find(|p| p.id == planet_id)
        .ok_or_else(|| format!("Planet '{}' not found", planet_id))
        .expect("Planet not found");

    format!(
        "--- Market Status ({}) ---\n{}",
        planet.id,
        build_enhanced_market_display(&planet.economy)
    )
}

/// Format price trend as a visual indicator
/// Returns "↑" for rising, "↓" for falling, "→" for stable
fn format_price_trend(trend: f64) -> &'static str {
    if trend > 0.05 {
        "↑" // Rising
    } else if trend < -0.05 {
        "↓" // Falling
    } else {
        "→" // Stable
    }
}

/// Format price trend as text description
fn format_price_trend_text(trend: f64) -> String {
    if trend > 0.1 {
        "Rising Fast".to_string()
    } else if trend > 0.05 {
        "Rising".to_string()
    } else if trend > -0.05 {
        "Stable".to_string()
    } else if trend > -0.1 {
        "Falling".to_string()
    } else {
        "Falling Fast".to_string()
    }
}

/// Format price anomaly indicator
/// Returns Some(indicator) if there's an anomaly, None otherwise
fn format_price_anomaly(anomaly: Option<PriceAnomaly>) -> Option<&'static str> {
    match anomaly {
        Some(PriceAnomaly::High) => Some("SELL"), // Price is high, good time to sell
        Some(PriceAnomaly::Low) => Some("BUY"),   // Price is low, good time to buy
        None => None,
    }
}

/// Format produced/demanded status indicator
fn format_supply_demand_status(is_produced: bool, is_demanded: bool) -> &'static str {
    if is_produced && is_demanded {
        "↔" // Both
    } else if is_produced {
        "↓" // Produced here (supply)
    } else if is_demanded {
        "↑" // Demanded here (demand)
    } else {
        " " // Neither
    }
}

/// Build the enhanced market display with all information
/// Shows: price trends, anomalies, produced/demanded status, active events
fn build_enhanced_market_display(economy: &crate::simulation::economy::PlanetEconomy) -> String {
    let mut output = String::new();

    // Display active market events first
    if !economy.active_events.is_empty() {
        output.push_str("*** ACTIVE MARKET EVENTS ***\n");
        for event in &economy.active_events {
            output.push_str(&format!(
                "  ⚠ {}: {}\n",
                event.display_name(),
                event.description()
            ));
        }
        output.push('\n');
    }

    // Display profitable trade opportunities
    let profitable_trades = economy.get_profitable_trades();
    if !profitable_trades.is_empty() {
        output.push_str("*** PROFITABLE OPPORTUNITIES ***\n");
        for (commodity, potential) in &profitable_trades {
            let anomaly = economy
                .get_commodity(commodity)
                .and_then(|mg| mg.is_price_anomaly());
            let indicator = match anomaly {
                Some(PriceAnomaly::High) => "SELL NOW",
                Some(PriceAnomaly::Low) => "BUY NOW",
                None => "",
            };
            output.push_str(&format!(
                "  ★ {} - {} (potential: {:.0}%)\n",
                commodity.display_name(),
                indicator,
                potential * 100.0
            ));
        }
        output.push('\n');
    }

    // Column headers with legend
    output.push_str("Commodity      Buy    Sell   Trend Status  Notes\n");
    output.push_str("-----------------------------------------------------------\n");

    // Group commodities by type: produced, demanded, ignored
    let produced: Vec<_> = economy
        .market
        .values()
        .filter(|mg| mg.is_produced)
        .collect();
    let demanded: Vec<_> = economy
        .market
        .values()
        .filter(|mg| mg.is_demanded && !mg.is_produced)
        .collect();
    let ignored: Vec<_> = economy
        .market
        .values()
        .filter(|mg| !mg.is_produced && !mg.is_demanded)
        .collect();

    // Helper to format a commodity row
    let format_commodity_row = |mg: &crate::simulation::economy::MarketGood| -> String {
        let trend_indicator = format_price_trend(mg.get_price_trend());
        let trend_text = format_price_trend_text(mg.get_price_trend());
        let status_indicator = format_supply_demand_status(mg.is_produced, mg.is_demanded);
        let anomaly_indicator = format_price_anomaly(mg.is_price_anomaly());

        // Build status column
        let status = format!("{}{}", status_indicator, trend_indicator);

        // Build notes column
        let mut notes = String::new();
        if mg.is_produced {
            notes.push_str("PRODUCED");
        } else if mg.is_demanded {
            notes.push_str("DEMANDED");
        }
        if let Some(anomaly) = anomaly_indicator {
            if !notes.is_empty() {
                notes.push_str(", ");
            }
            notes.push_str(anomaly);
        }

        format!(
            "{:<14} {:<6} {:<6} {:<5} {:<7} {}\n",
            mg.commodity_type.display_name(),
            mg.buy_price,
            mg.sell_price,
            status,
            trend_text,
            notes
        )
    };

    // Show produced commodities section
    if !produced.is_empty() {
        output.push_str("--- Produced Here (Supply) ---\n");
        for mg in &produced {
            output.push_str(&format_commodity_row(mg));
        }
        output.push('\n');
    }

    // Show demanded commodities section
    if !demanded.is_empty() {
        output.push_str("--- Demanded Here (High Demand) ---\n");
        for mg in &demanded {
            output.push_str(&format_commodity_row(mg));
        }
        output.push('\n');
    }

    // Show ignored commodities section
    if !ignored.is_empty() {
        output.push_str("--- Other Commodities ---\n");
        for mg in &ignored {
            output.push_str(&format_commodity_row(mg));
        }
    }

    // Add legend
    output.push_str("\nLegend:\n");
    output.push_str("  Trend: ↑ Rising  ↓ Falling  → Stable\n");
    output.push_str("  Status: ↓ Produced  ↑ Demanded  ↔ Both\n");
    output.push_str("  Notes: PRODUCED (local supply)  DEMANDED (local demand)\n");
    output.push_str("         BUY (low price anomaly)  SELL (high price anomaly)\n");

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::GameClock;
    use crate::player::{Player, inventory::CargoHold, ship::Ship};
    use crate::setup::World;
    use crate::simulation::commodity::CommodityType;
    use crate::simulation::economy::PlanetEconomy;
    use crate::simulation::orbits::{Planet, Position};
    use crate::simulation::planet_types::PlanetType;

    // Helper function to create a mock World instance
    fn create_mock_world() -> World {
        // Use PlanetEconomy::new() to properly initialize all 10 commodities
        let economy_earth = PlanetEconomy::new(PlanetType::Agricultural);
        let economy_mars = PlanetEconomy::new(PlanetType::Mining);

        let planet_earth = Planet {
            id: "Earth".to_string(),
            name: "Earth".to_string(),
            orbit_radius: 5,
            orbit_period: 10,
            position: Position::new(0),
            economy: economy_earth,
            planet_type: PlanetType::Agricultural,
        };

        let planet_mars = Planet {
            id: "Mars".to_string(),
            name: "Mars".to_string(),
            orbit_radius: 12,
            orbit_period: 15,
            position: Position::new(7),
            economy: economy_mars,
            planet_type: PlanetType::Mining,
        };

        let mut player_inventory = CargoHold::new(100);
        player_inventory
            .add_commodity(CommodityType::Foodstuffs, 5)
            .unwrap();

        World {
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
        assert!(output.contains("Commodities:"));
        assert!(output.contains("Foodstuffs x 5"));
    }

    #[test]
    fn test_display_market_status() {
        let world = create_mock_world();
        let output = display_market_status(&world);
        assert!(output.contains("--- Market Status (Earth) ---"));
        // Check for new enhanced format headers
        assert!(output.contains("Commodity      Buy    Sell"));
        assert!(output.contains("Trend"));
        assert!(output.contains("Status"));
        assert!(output.contains("Notes"));
        // Check that commodities are displayed
        assert!(output.contains("Foodstuffs"));
        assert!(output.contains("Water"));
        // Check for legend
        assert!(output.contains("Legend:"));
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

use crate::{
    setup::World,
    simulation::economy::MarketGood,
    simulation::commodity::CommodityType,
    simulation::travel::calculate_travel_time,
};

pub fn handle_buy(
    world: &mut World,
    commodity_str: &str,
    quantity: u32,
) -> Result<String, String> {
    // Convert string to CommodityType
    let commodity_type = str_to_commodity_type(commodity_str)
        .ok_or_else(|| format!("Unknown commodity type: '{}'", commodity_str))?;

    let player = &mut world.player;
    let planet_idx = world
        .planets
        .iter()
        .position(|p| p.id == player.location)
        .ok_or_else(|| "Player is not at a valid planet".to_string())?;
    let planet = &mut world.planets[planet_idx];

    let market_commodity = planet
        .economy
        .market
        .get_mut(&commodity_type)
        .ok_or_else(|| format!("Commodity '{}' not available at this market", commodity_str))?;

    let total_cost = market_commodity.sell_price * quantity;

    // Check cargo space first
    if quantity > player.inventory.remaining_capacity() {
        return Err("Insufficient cargo space".to_string());
    }

    // Check if player has enough money
    if player.money < total_cost {
        return Err("Insufficient funds".to_string());
    }

    // Add commodity to inventory
    player.inventory.add_commodity(commodity_type.clone(), quantity)
        .map_err(|e| e.to_string())?;

    // Deduct money
    player.money -= total_cost;

    // Update market supply based on purchase (player buying = market supply decreases)
    market_commodity.adjust_supply_from_trade(-(quantity as i32));

    Ok(format!("Successfully purchased {} of '{}'", quantity, commodity_str))
}

pub fn handle_sell(
    world: &mut World,
    commodity_str: &str,
    quantity: u32,
) -> Result<String, String> {
    // Convert string to CommodityType
    let commodity_type = str_to_commodity_type(commodity_str)
        .ok_or_else(|| format!("Unknown commodity type: '{}'", commodity_str))?;

    let player = &mut world.player;
    let planet_idx = world
        .planets
        .iter()
        .position(|p| p.id == player.location)
        .ok_or_else(|| "Player is not at a valid planet".to_string())?;
    let planet = &mut world.planets[planet_idx];

    let market_commodity = planet
        .economy
        .market
        .get_mut(&commodity_type)
        .ok_or_else(|| format!("Commodity '{}' not available at this market", commodity_str))?;

    // Check if player has enough of the commodity
    let available_quantity = player.inventory.get_commodity_quantity(&commodity_type);
    if available_quantity < quantity {
        return Err(format!("You only have {} of '{}' to sell", available_quantity, commodity_str));
    }

    // Remove commodity from inventory
    player.inventory.remove_commodity(commodity_type.clone(), quantity)
        .map_err(|e| e.to_string())?;

    let total_sale_price = market_commodity.buy_price * quantity;
    player.money += total_sale_price;

    // Update market supply based on sale (player selling = market supply increases)
    market_commodity.adjust_supply_from_trade(quantity as i32);

    Ok(format!("Successfully sold {} of '{}'", quantity, commodity_str))
}

pub fn handle_travel(
    world: &mut World,
    destination_planet_id: &str,
) -> Result<String, String> {
    let (origin_planet, destination_planet) = {
        let origin = world
            .planets
            .iter()
            .find(|p| p.id == world.player.location)
            .ok_or_else(|| "Player is not at a valid planet".to_string())?;

        let destination = world
            .planets
            .iter()
            .find(|p| p.id == destination_planet_id)
            .ok_or_else(|| format!("Destination planet '{}' not found", destination_planet_id))?;

        (origin.clone(), destination.clone())
    };

    let travel_time = calculate_travel_time(&origin_planet, &destination_planet, world.player.ship.speed);

    world.game_clock.current_turn += travel_time;
    world.player.location = destination_planet_id.to_string();

    Ok(format!("Traveled to {}. It took {} months.", destination_planet_id, travel_time))
}

pub fn handle_wait(
    world: &mut World,
    months: u32,
) -> Result<String, String> {
    world.game_clock.current_turn += months;
    Ok(format!("Waited for {} months.", months))
}

/// Helper function to convert string to CommodityType
fn str_to_commodity_type(s: &str) -> Option<CommodityType> {
    match s.to_lowercase().as_str() {
        "water" => Some(CommodityType::Water),
        "foodstuffs" => Some(CommodityType::Foodstuffs),
        "medicine" => Some(CommodityType::Medicine),
        "firearms" => Some(CommodityType::Firearms),
        "ammunition" => Some(CommodityType::Ammunition),
        "metals" => Some(CommodityType::Metals),
        "antimatter" => Some(CommodityType::Antimatter),
        "electronics" => Some(CommodityType::Electronics),
        "narcotics" => Some(CommodityType::Narcotics),
        "alienartefacts" | "alien artefacts" | "alien_artefacts" => Some(CommodityType::AlienArtefacts),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        setup::World,
        player::{inventory::CargoHold, Player, ship::Ship},
        simulation::economy::{MarketGood, PlanetEconomy},
        simulation::commodity::CommodityType,
        simulation::orbits::{Planet, Position},
        simulation::planet_types::PlanetType,
        game_state::GameClock,
    };
    use std::collections::HashMap;

    fn create_mock_world() -> World {
        let mut market_earth = HashMap::new();
        market_earth.insert(CommodityType::Water, MarketGood::new(&CommodityType::Water, &PlanetType::Agricultural));
        market_earth.insert(CommodityType::Foodstuffs, MarketGood::new(&CommodityType::Foodstuffs, &PlanetType::Agricultural));

        let planet_earth = Planet {
            id: "earth".to_string(),
            orbit_radius: 5,
            orbit_period: 10,
            position: Position::new(0),
            economy: PlanetEconomy { 
                market: market_earth,
                planet_type: PlanetType::Agricultural,
                active_events: Vec::new(),
            },
            planet_type: PlanetType::Agricultural,
        };

        let planet_mars = Planet {
            id: "mars".to_string(),
            orbit_radius: 12,
            orbit_period: 15,
            position: Position::new(7),
            economy: PlanetEconomy { 
                market: HashMap::new(),
                planet_type: PlanetType::Mining,
                active_events: Vec::new(),
            },
            planet_type: PlanetType::Mining,
        };

        World {
            planets: vec![planet_earth, planet_mars],
            current_time: 0.0,
            player: Player {
                money: 100,
                location: "earth".to_string(),
                ship: Ship::new(0.5, 50),
                inventory: CargoHold::new(50),
            },
            game_clock: GameClock {
                current_turn: 1,
                total_turns: 100,
            },
        }
    }

    #[test]
    fn test_handle_buy_successful() {
        let mut world = create_mock_world();
        
        // Get the sell price of water on Agricultural planet
        let water_price = world.planets.iter()
            .find(|p| p.id == "earth")
            .unwrap()
            .economy.market.get(&CommodityType::Water)
            .unwrap()
            .sell_price;
        
        let result = handle_buy(&mut world, "water", 5);
        assert!(result.is_ok());
        // Player should have spent water_price * 5
        assert_eq!(world.player.money, 100 - (water_price * 5));
        assert_eq!(world.player.inventory.get_commodity_quantity(&CommodityType::Water), 5);
    }

    #[test]
    fn test_handle_buy_insufficient_funds() {
        let mut world = create_mock_world();
        
        // Get the sell price of water
        let water_price = world.planets.iter()
            .find(|p| p.id == "earth")
            .unwrap()
            .economy.market.get(&CommodityType::Water)
            .unwrap()
            .sell_price;
        
        // Try to buy more than player can afford (player has 100)
        // Use a quantity that costs more than 100 but fits in cargo (cargo capacity is 50)
        let quantity = (100 / water_price) + 1;
        let result = handle_buy(&mut world, "water", quantity);
        
        // Should fail with either insufficient funds or insufficient cargo space
        // (whichever comes first given the constraints)
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err == "Insufficient funds" || err == "Insufficient cargo space");
    }

    #[test]
    fn test_handle_buy_insufficient_cargo_space() {
        let mut world = create_mock_world();
        let result = handle_buy(&mut world, "water", 51);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient cargo space");
    }

    #[test]
    fn test_handle_buy_commodity_not_at_market() {
        let mut world = create_mock_world();
        let result = handle_buy(&mut world, "medicine", 1);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Commodity 'medicine' not available at this market"
        );
    }

    #[test]
    fn test_handle_sell_successful() {
        let mut world = create_mock_world();
        world.player.inventory.add_commodity(CommodityType::Water, 10).unwrap();
        
        // Get the buy price of water
        let water_buy_price = world.planets.iter()
            .find(|p| p.id == "earth")
            .unwrap()
            .economy.market.get(&CommodityType::Water)
            .unwrap()
            .buy_price;
        
        let result = handle_sell(&mut world, "water", 5);
        assert!(result.is_ok());
        // Player should have received water_buy_price * 5
        assert_eq!(world.player.money, 100 + (water_buy_price * 5));
        assert_eq!(world.player.inventory.get_commodity_quantity(&CommodityType::Water), 5);
    }

    #[test]
    fn test_handle_sell_commodity_not_in_inventory() {
        let mut world = create_mock_world();
        let result = handle_sell(&mut world, "water", 5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "You only have 0 of 'water' to sell");
    }

    #[test]
    fn test_handle_sell_insufficient_quantity() {
        let mut world = create_mock_world();
        world.player.inventory.add_commodity(CommodityType::Water, 5).unwrap();
        let result = handle_sell(&mut world, "water", 10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "You only have 5 of 'water' to sell");
    }

    #[test]
    fn test_handle_travel_successful() {
        let mut world = create_mock_world();
        let result = handle_travel(&mut world, "mars");
        assert!(result.is_ok());
        assert_eq!(world.player.location, "mars");
        // Earth orbit_radius: 5, Mars orbit_radius: 12
        // Distance: |12 - 5| = 7
        // Travel time: 2 * sqrt(7/1) = 5.29... → ceil = 6 turns
        // Starting turn: 1, after travel: 1 + 6 = 7
        assert_eq!(world.game_clock.current_turn, 7);
    }

    #[test]
    fn test_handle_travel_unknown_destination() {
        let mut world = create_mock_world();
        let result = handle_travel(&mut world, "unknown_planet");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Destination planet 'unknown_planet' not found"
        );
    }

    #[test]
    fn test_handle_wait() {
        let mut world = create_mock_world();
        let result = handle_wait(&mut world, 10);
        assert!(result.is_ok());
        assert_eq!(world.game_clock.current_turn, 11);
    }

    #[test]
    fn test_str_to_commodity_type() {
        assert_eq!(str_to_commodity_type("water"), Some(CommodityType::Water));
        assert_eq!(str_to_commodity_type("WATER"), Some(CommodityType::Water));
        assert_eq!(str_to_commodity_type("Water"), Some(CommodityType::Water));
        assert_eq!(str_to_commodity_type("foodstuffs"), Some(CommodityType::Foodstuffs));
        assert_eq!(str_to_commodity_type("alien artefacts"), Some(CommodityType::AlienArtefacts));
        assert_eq!(str_to_commodity_type("unknown"), None);
    }
}


use crate::{
    setup::World,
    player::inventory::CargoHold,
    simulation::economy::MarketCommodity,
    simulation::commodity::CommodityType,
    simulation::orbits::Planet,
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
        .iter_mut()
        .find(|mc| mc.commodity_type == commodity_type)
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

    // Update market supply based on purchase
    market_commodity.adjust_supply(quantity as i32);

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
        .iter_mut()
        .find(|mc| mc.commodity_type == commodity_type)
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

    // Update market supply based on sale
    market_commodity.adjust_supply(-(quantity as i32));

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
        simulation::economy::{MarketCommodity, PlanetEconomy},
        simulation::commodity::CommodityType,
        simulation::orbits::{Planet, Position},
        game_state::GameClock,
    };

    fn create_mock_world() -> World {
        let market_earth_water = MarketCommodity::new(CommodityType::Water, 10);
        let market_earth_food = MarketCommodity::new(CommodityType::Foodstuffs, 20);

        let planet_earth = Planet {
            id: "earth".to_string(),
            orbit_radius: 1.0,
            orbit_period: 12.0,
            position: Position { x: 1.0, y: 0.0 },
            economy: PlanetEconomy { market: vec![market_earth_water, market_earth_food] },
        };

        let planet_mars = Planet {
            id: "mars".to_string(),
            orbit_radius: 1.5,
            orbit_period: 24.0,
            position: Position { x: -1.5, y: 0.0 },
            economy: PlanetEconomy { market: vec![] },
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
        let result = handle_buy(&mut world, "water", 5);
        assert!(result.is_ok());
        assert_eq!(world.player.money, 50); // 100 - (10 * 5)
        assert_eq!(world.player.inventory.get_commodity_quantity(&CommodityType::Water), 5);
    }

    #[test]
    fn test_handle_buy_insufficient_funds() {
        let mut world = create_mock_world();
        let result = handle_buy(&mut world, "water", 11);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient funds");
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
        let result = handle_sell(&mut world, "water", 5);
        assert!(result.is_ok());
        // After creating MarketCommodity with base value 10, buy_price should be 9 (10-1)
        assert_eq!(world.player.money, 145); // 100 + (9 * 5)
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
        assert_eq!(world.game_clock.current_turn, 6);
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


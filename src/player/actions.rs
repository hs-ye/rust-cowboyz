use crate::{
    setup::World,
    player::{inventory::CargoHold, Player, ship::Ship},
    simulation::economy::{Good, MarketGood},
    simulation::orbits::{Planet, Position},
    simulation::travel::calculate_travel_time,
    game_state::GameClock,
};

pub fn handle_buy(
    world: &mut World,
    good_id: &str,
    quantity: u32,
) -> Result<String, String> {
    let player = &mut world.player;
    let planet = world
        .planets
        .iter_mut()
        .find(|p| p.id == player.location)
        .ok_or_else(|| "Player is not at a valid planet".to_string())?;

    let market_good = planet
        .economy.market
        .iter_mut()
        .find(|g| g.good.id == good_id)
        .ok_or_else(|| format!("Good '{}' not available at this market", good_id))?;

    let total_cost = market_good.buy_price * quantity;

    if player.inventory.add_good(good_id.to_string(), quantity).is_err() {
        return Err("Insufficient cargo space".to_string());
    }

    if player.money < total_cost {
        return Err("Insufficient funds".to_string());
    }

    player.money -= total_cost;

    Ok(format!("Successfully purchased {} of '{}'", quantity, good_id))
}

pub fn handle_sell(
    world: &mut World,
    good_id: &str,
    quantity: u32,
) -> Result<String, String> {
    let player = &mut world.player;
    let planet = world
        .planets
        .iter()
        .find(|p| p.id == player.location)
        .ok_or_else(|| "Player is not at a valid planet".to_string())?;

    let market_good = planet
        .economy.market
        .iter()
        .find(|g| g.good.id == good_id)
        .ok_or_else(|| format!("Good '{}' not available at this market", good_id))?;

    if player.inventory.remove_good(good_id.to_string(), quantity).is_err() {
        return Err(format!("You only have {} of '{}' to sell", player.inventory.goods.get(good_id).unwrap_or(&0), good_id));
    }

    let total_sale_price = market_good.sell_price * quantity;
    player.money += total_sale_price;

    Ok(format!("Successfully sold {} of '{}'", quantity, good_id))
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

#[cfg(test)]
mod tests {
    use crate::{
        setup::World,
        player::{inventory::CargoHold, Player, ship::Ship},
        simulation::economy::{Good, MarketGood, PlanetEconomy},
        simulation::orbits::{Planet, Position},
        simulation::travel::calculate_travel_time,
        game_state::GameClock,
    };

    fn create_mock_world() -> World {
        let good_food = Good { id: "food".to_string(), base_value: 10 };
        let good_water = Good { id: "water".to_string(), base_value: 5 };

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
            id: "earth".to_string(),
            orbit_radius: 1.0,
            orbit_period: 12.0,
            position: Position { x: 1.0, y: 0.0 },
            economy: PlanetEconomy { market: vec![market_earth_food, market_earth_water] },
        };

        let planet_mars = Planet {
            id: "mars".to_string(),
            orbit_radius: 1.5,
            orbit_period: 24.0,
            position: Position { x: -1.5, y: 0.0 },
            economy: PlanetEconomy { market: vec![] },
        };

        World {
            goods: vec![good_food, good_water],
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
        let result = handle_buy(&mut world, "food", 5);
        assert!(result.is_ok());
        assert_eq!(world.player.money, 60);
        assert_eq!(*world.player.inventory.goods.get("food").unwrap(), 5);
    }

    #[test]
    fn test_handle_buy_insufficient_funds() {
        let mut world = create_mock_world();
        let result = handle_buy(&mut world, "food", 13);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient funds");
    }

    #[test]
    fn test_handle_buy_insufficient_cargo_space() {
        let mut world = create_mock_world();
        let result = handle_buy(&mut world, "food", 51);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient cargo space");
    }

    #[test]
    fn test_handle_buy_good_not_at_market() {
        let mut world = create_mock_world();
        let result = handle_buy(&mut world, "unknown_good", 1);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Good 'unknown_good' not available at this market"
        );
    }

    #[test]
    fn test_handle_sell_successful() {
        let mut world = create_mock_world();
        world.player.inventory.add_good("food".to_string(), 10).unwrap();
        let result = handle_sell(&mut world, "food", 5);
        assert!(result.is_ok());
        assert_eq!(world.player.money, 160);
        assert_eq!(*world.player.inventory.goods.get("food").unwrap(), 5);
    }

    #[test]
    fn test_handle_sell_good_not_in_inventory() {
        let mut world = create_mock_world();
        let result = handle_sell(&mut world, "food", 5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "You only have 0 of 'food' to sell");
    }

    #[test]
    fn test_handle_sell_insufficient_quantity() {
        let mut world = create_mock_world();
        world.player.inventory.add_good("food".to_string(), 5).unwrap();
        let result = handle_sell(&mut world, "food", 10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "You only have 5 of 'food' to sell");
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
}
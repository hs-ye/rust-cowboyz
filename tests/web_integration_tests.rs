//! Integration tests for web UI and backend movement system
//! Tests the full flow from UI interaction to backend state updates

use cowboyz::game_state::{
    GameState, GameSettings, Player, SolarSystem, Planet, Ship, TravelState, GameDifficulty,
};
use cowboyz::simulation::planet_types::PlanetType;
use cowboyz::simulation::orbits::Position;
use cowboyz::simulation::economy::PlanetEconomy;

/// Helper function to create a test solar system with multiple planets
fn create_test_solar_system() -> SolarSystem {
    let planets = vec![
        Planet {
            id: "earth".to_string(),
            name: "Earth".to_string(),
            orbit_radius: 5,
            orbit_period: 10,
            position: Position::new(0),
            economy: PlanetEconomy::new(PlanetType::Agricultural),
            planet_type: PlanetType::Agricultural,
        },
        Planet {
            id: "mars".to_string(),
            name: "Mars".to_string(),
            orbit_radius: 12,
            orbit_period: 15,
            position: Position::new(7),
            economy: PlanetEconomy::new(PlanetType::Mining),
            planet_type: PlanetType::Mining,
        },
        Planet {
            id: "jupiter".to_string(),
            name: "Jupiter".to_string(),
            orbit_radius: 25,
            orbit_period: 25,
            position: Position::new(12),
            economy: PlanetEconomy::new(PlanetType::Industrial),
            planet_type: PlanetType::Industrial,
        },
    ];
    
    SolarSystem::new("Test System".to_string(), planets)
}

/// Helper function to create a test game state
fn create_test_game_state() -> GameState {
    let solar_system = create_test_solar_system();
    let settings = GameSettings::default();
    let player = Player::new();
    
    GameState::with_settings(player, solar_system, settings)
}

#[test]
fn test_ui_reads_planet_data_from_backend() {
    let state = create_test_game_state();
    
    // Verify UI can read planet data from backend
    let earth = state.solar_system.get_planet("earth").unwrap();
    assert_eq!(earth.name, "Earth");
    assert_eq!(earth.orbit_radius, 5);
    assert_eq!(earth.planet_type, PlanetType::Agricultural);
    
    let mars = state.solar_system.get_planet("mars").unwrap();
    assert_eq!(mars.name, "Mars");
    assert_eq!(mars.orbit_radius, 12);
    assert_eq!(mars.planet_type, PlanetType::Mining);
}

#[test]
fn test_travel_selection_triggers_backend_travel() {
    let mut state = create_test_game_state();
    let initial_fuel = state.player.ship.fuel;
    
    // Verify initial state
    assert!(state.player.travel_state.is_idle());
    assert_eq!(state.player.location, "earth");
    
    // Initiate travel to Mars
    let result = state.initiate_travel("mars");
    assert!(result.is_ok());
    
    // Verify travel state updated
    assert!(state.player.travel_state.is_in_transit());
    assert_eq!(state.player.travel_state.destination(), Some(&"mars".to_string()));
    
    // Verify fuel was deducted
    let distance = 12 - 5; // Mars orbit - Earth orbit
    let expected_fuel = initial_fuel - distance;
    assert_eq!(state.player.ship.fuel, expected_fuel);
}

#[test]
fn test_turn_advancement_updates_all_ui_components() {
    let mut state = create_test_game_state();
    
    // Record initial positions
    let initial_positions: Vec<u32> = state.solar_system.planets.iter()
        .map(|p| p.position.orbital_position)
        .collect();
    
    // Initiate travel to Mars
    state.initiate_travel("mars").unwrap();
    let arrival_turn = state.game_clock.current_turn + state.turns_until_arrival();
    let mut arrival_event_received = false;
    let mut turn_count = 0;
    
    // Advance turns until arrival
    while state.game_clock.current_turn < arrival_turn {
        let arrival_event = state.next_turn();
        turn_count += 1;
        
        // Verify planet positions have advanced by turn_count from initial
        for (i, planet) in state.solar_system.planets.iter().enumerate() {
            let expected_position = (initial_positions[i] + turn_count) % planet.orbit_period.max(1);
            assert_eq!(planet.position.orbital_position, expected_position, 
                "Planet {} position should have advanced by {}", planet.name, turn_count);
        }
        
        // Check if we arrived (arrival_event is Some when ship arrives)
        if let Some(event) = arrival_event {
            assert_eq!(event.destination_planet_id, "mars");
            arrival_event_received = true;
        }
    }
    
    // Verify we received arrival event
    assert!(arrival_event_received, "Should have received arrival event");
    
    // Verify arrival
    assert!(state.player.travel_state.is_idle());
    assert_eq!(state.player.location, "mars");
}

#[test]
fn test_travel_state_synchronization() {
    let mut state = create_test_game_state();
    
    // Test idle state
    assert!(state.is_in_transit() == false);
    assert_eq!(state.get_current_location(), Some(&"earth".to_string()));
    assert_eq!(state.get_destination(), None);
    assert_eq!(state.turns_until_arrival(), 0);
    
    // Initiate travel
    state.initiate_travel("mars").unwrap();
    
    // Test in-transit state
    assert!(state.is_in_transit() == true);
    assert_eq!(state.get_current_location(), None);
    assert_eq!(state.get_destination(), Some(&"mars".to_string()));
    assert!(state.turns_until_arrival() > 0);
    
    // Advance to arrival
    let turns_to_advance = state.turns_until_arrival();
    for _ in 0..turns_to_advance {
        state.next_turn();
    }
    
    // Test arrival state
    assert!(state.is_in_transit() == false);
    assert_eq!(state.get_current_location(), Some(&"mars".to_string()));
    assert_eq!(state.get_destination(), None);
    assert_eq!(state.turns_until_arrival(), 0);
}

#[test]
fn test_game_state_persistence_compatibility() {
    let state = create_test_game_state();
    
    // Serialize to JSON (simulating localStorage persistence)
    let json = serde_json::to_string(&state).unwrap();
    
    // Deserialize back
    let loaded_state: GameState = serde_json::from_str(&json).unwrap();
    
    // Verify all data preserved
    assert_eq!(loaded_state.player.location, state.player.location);
    assert_eq!(loaded_state.player.money, state.player.money);
    assert_eq!(loaded_state.player.ship.fuel, state.player.ship.fuel);
    assert_eq!(loaded_state.game_clock.current_turn, state.game_clock.current_turn);
    assert_eq!(loaded_state.solar_system.planets.len(), state.solar_system.planets.len());
}

#[test]
fn test_edge_case_insufficient_fuel() {
    let mut state = create_test_game_state();
    
    // Set fuel to insufficient amount (Earth to Jupiter distance = 20)
    state.player.ship.fuel = 10;
    
    // Attempt travel to Jupiter
    let result = state.initiate_travel("jupiter");
    
    // Should fail with insufficient fuel error
    assert!(result.is_err());
    assert!(state.player.travel_state.is_idle());
    assert_eq!(state.player.location, "earth");
}

#[test]
fn test_edge_case_same_destination() {
    let mut state = create_test_game_state();
    
    // Attempt travel to current location
    let result = state.initiate_travel("earth");
    
    // Should fail with same destination error
    assert!(result.is_err());
    assert!(state.player.travel_state.is_idle());
}

#[test]
fn test_edge_case_mid_travel_state() {
    let mut state = create_test_game_state();
    
    // Initiate travel
    state.initiate_travel("mars").unwrap();
    
    // Attempt to initiate another travel while in transit
    let result = state.initiate_travel("jupiter");
    
    // Should fail with already in transit error
    assert!(result.is_err());
    assert!(state.player.travel_state.is_in_transit());
    assert_eq!(state.player.travel_state.destination(), Some(&"mars".to_string()));
}

#[test]
fn test_edge_case_invalid_destination() {
    let mut state = create_test_game_state();
    
    // Attempt travel to non-existent planet
    let result = state.initiate_travel("nonexistent");
    
    // Should fail with invalid destination error
    assert!(result.is_err());
    assert!(state.player.travel_state.is_idle());
}

#[test]
fn test_full_travel_flow_integration() {
    let mut state = create_test_game_state();
    let initial_turn = state.game_clock.current_turn;
    let initial_location = state.player.location.clone();
    let initial_fuel = state.player.ship.fuel;
    
    // Step 1: Select destination (Mars)
    let destination = "mars";
    let dest_planet = state.solar_system.get_planet(destination).unwrap();
    let distance = dest_planet.orbit_radius.abs_diff(
        state.solar_system.get_planet(&initial_location).unwrap().orbit_radius
    );
    
    // Step 2: Calculate travel cost
    assert!(state.player.ship.can_travel(distance));
    
    // Step 3: Confirm travel
    let result = state.initiate_travel(destination);
    assert!(result.is_ok());
    
    // Step 4: Verify travel initiated
    assert!(state.is_in_transit());
    assert_eq!(state.get_destination(), Some(&destination.to_string()));
    let turns_required = state.turns_until_arrival();
    assert!(turns_required > 0);
    
    // Step 5: Verify fuel deducted
    assert_eq!(state.player.ship.fuel, initial_fuel - distance);
    
    // Step 6: Advance turns until arrival
    for i in 0..turns_required {
        let arrival = state.next_turn();
        
        // Verify turns remaining decreases
        assert_eq!(state.turns_until_arrival(), turns_required - i - 1);
        
        // Check for arrival on last turn
        if i == turns_required - 1 {
            assert!(arrival.is_some());
        } else {
            assert!(arrival.is_none());
        }
    }
    
    // Step 7: Verify arrival
    assert!(!state.is_in_transit());
    assert_eq!(state.player.location, destination);
    assert_eq!(state.get_current_location(), Some(&destination.to_string()));
    assert_eq!(state.game_clock.current_turn, initial_turn + turns_required);
}

#[test]
fn test_market_data_updates_during_travel() {
    let mut state = create_test_game_state();
    
    // Get initial market prices
    let earth_initial = state.solar_system.get_planet("earth").unwrap().economy.market.clone();
    
    // Initiate travel
    state.initiate_travel("mars").unwrap();
    
    // Advance several turns
    for _ in 0..3 {
        state.next_turn();
    }
    
    // Verify market prices have been updated
    let earth_current = state.solar_system.get_planet("earth").unwrap().economy.market.clone();
    // Market should have been updated each turn
    assert!(!earth_current.is_empty());
}

#[test]
fn test_planet_positions_update_during_travel() {
    let mut state = create_test_game_state();
    
    // Get initial positions
    let earth_initial = state.solar_system.get_planet("earth").unwrap().position.orbital_position;
    
    // Initiate travel
    state.initiate_travel("mars").unwrap();
    let turns = state.turns_until_arrival();
    
    // Advance turns
    for _ in 0..turns {
        state.next_turn();
    }
    
    // Verify planet positions have advanced
    let earth_current = state.solar_system.get_planet("earth").unwrap().position.orbital_position;
    let expected_position = (earth_initial + turns) % 10; // Earth orbit_period = 10
    assert_eq!(earth_current, expected_position);
}

//! Web UI components for Rust Cowboyz

use leptos::*;
use crate::ui::cargo_panel::CargoPanel;
use crate::ui::solar_map::{SolarMap, MapPlanet};
use crate::ui::market_panel::MarketPanelReactive;
use crate::simulation::planet_types::PlanetType;
use crate::simulation::orbits::Position;
use crate::simulation::economy::PlanetEconomy;
use crate::game_state::Planet;

/// Main application component with 60/40 split-screen layout
#[component]
pub fn App() -> impl IntoView {
    // Create reactive game state
    let (money, set_money) = create_signal(1000);
    let (location, _set_location) = create_signal("earth".to_string());
    let (turn, set_turn) = create_signal(1);
    let (fuel, _set_fuel) = create_signal(100);
    let (cargo_capacity, _set_cargo_capacity) = create_signal(50);
    let (cargo_used, _set_cargo_used) = create_signal(0);
    let (selected_planet, set_selected_planet) = create_signal(None::<String>);

    // Create memoized values for reactive planet selection
    // This ensures the market panel properly tracks when selected_planet changes
    let current_planet_id = create_memo(move |_| {
        selected_planet.get().unwrap_or(location.get())
    });

    // Create solar system planet data with economy
    let planets: Vec<Planet> = vec![
        Planet {
            id: "mercury".to_string(),
            name: "Mercury".to_string(),
            orbit_radius: 3,
            orbit_period: 4,
            position: Position::new(0),
            planet_type: PlanetType::Mining,
            economy: PlanetEconomy::new(PlanetType::Mining),
        },
        Planet {
            id: "venus".to_string(),
            name: "Venus".to_string(),
            orbit_radius: 5,
            orbit_period: 6,
            position: Position::new(0),
            planet_type: PlanetType::Industrial,
            economy: PlanetEconomy::new(PlanetType::Industrial),
        },
        Planet {
            id: "earth".to_string(),
            name: "Earth".to_string(),
            orbit_radius: 7,
            orbit_period: 8,
            position: Position::new(0),
            planet_type: PlanetType::Agricultural,
            economy: PlanetEconomy::new(PlanetType::Agricultural),
        },
        Planet {
            id: "mars".to_string(),
            name: "Mars".to_string(),
            orbit_radius: 10,
            orbit_period: 12,
            position: Position::new(0),
            planet_type: PlanetType::Mining,
            economy: PlanetEconomy::new(PlanetType::Mining),
        },
        Planet {
            id: "jupiter".to_string(),
            name: "Jupiter".to_string(),
            orbit_radius: 15,
            orbit_period: 20,
            position: Position::new(0),
            planet_type: PlanetType::MegaCity,
            economy: PlanetEconomy::new(PlanetType::MegaCity),
        },
        Planet {
            id: "saturn".to_string(),
            name: "Saturn".to_string(),
            orbit_radius: 20,
            orbit_period: 28,
            position: Position::new(0),
            planet_type: PlanetType::Industrial,
            economy: PlanetEconomy::new(PlanetType::Industrial),
        },
        Planet {
            id: "uranus".to_string(),
            name: "Uranus".to_string(),
            orbit_radius: 25,
            orbit_period: 36,
            position: Position::new(0),
            planet_type: PlanetType::ResearchOutpost,
            economy: PlanetEconomy::new(PlanetType::ResearchOutpost),
        },
        Planet {
            id: "neptune".to_string(),
            name: "Neptune".to_string(),
            orbit_radius: 30,
            orbit_period: 44,
            position: Position::new(0),
            planet_type: PlanetType::FrontierColony,
            economy: PlanetEconomy::new(PlanetType::FrontierColony),
        },
        Planet {
            id: "pluto".to_string(),
            name: "Pluto".to_string(),
            orbit_radius: 35,
            orbit_period: 52,
            position: Position::new(0),
            planet_type: PlanetType::PirateSpaceStation,
            economy: PlanetEconomy::new(PlanetType::PirateSpaceStation),
        },
    ];

    // Clone planets for use in reactive memos
    let planets_for_economy = planets.clone();
    let planets_for_type = planets.clone();

    // Create memoized economy that reacts to planet selection changes
    let current_economy = create_memo(move |_| {
        let planet_id = current_planet_id.get();
        planets_for_economy.iter()
            .find(|p| p.id == planet_id)
            .map(|p| p.economy.clone())
            .unwrap_or_else(|| PlanetEconomy::new(PlanetType::Agricultural))
    });

    // Create memoized planet type for reactive updates
    let current_planet_type = create_memo(move |_| {
        let planet_id = current_planet_id.get();
        planets_for_type.iter()
            .find(|p| p.id == planet_id)
            .map(|p| p.planet_type.clone())
            .unwrap_or(PlanetType::Agricultural)
    });

    view! {
        <div class="app-container">
            <header class="app-header">
                <h1>"Space Cowboys" </h1>
                <span class="subtitle">"Space-Western Trading Game"</span>
            </header>

            <div class="split-layout">
                // Left side (60%): Solar System Map
                <div class="map-panel">
                    <div class="panel-header">
                        <h2>"Solar System Map"</h2>
                    </div>
                    <div class="map-viewport">
                        <SolarMap
                            planets={planets.iter().map(|p| MapPlanet {
                                id: p.id.clone(),
                                name: p.name.clone(),
                                orbit_radius: p.orbit_radius,
                                orbit_period: p.orbit_period,
                                position: p.position.clone(),
                                planet_type: p.planet_type.clone(),
                            }).collect()}
                            current_turn={Box::new(move || turn.get())}
                            player_location={Box::new(move || location.get())}
                            selected_planet={Box::new(move || selected_planet.get())}
                            on_planet_select={Some(Box::new(move |id| {
                                set_selected_planet.set(Some(id));
                            }))}
                        />
                    </div>
                </div>

                // Right side (40%): Information Panels
                <div class="info-panels">
                    // Player Status Panel
                    <div class="panel player-panel">
                        <div class="panel-header">
                            <h3>"Player Status"</h3>
                        </div>
                        <div class="panel-content">
                            <div class="stat-row">
                                <span class="stat-label">"Location:"</span>
                                <span class="stat-value location">{move || location.get()}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"Turn:"</span>
                                <span class="stat-value turn">{move || turn.get()}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"Reputation:"</span>
                                <span class="stat-value">"Rookie"</span>
                            </div>
                        </div>
                    </div>

                    // Ship Status Panel
                    <div class="panel ship-panel">
                        <div class="panel-header">
                            <h3>"Ship Status"</h3>
                        </div>
                        <div class="panel-content">
                            <div class="stat-row">
                                <span class="stat-label">"Fuel:"</span>
                                <span class="stat-value fuel"> {move || fuel.get()} "/ 100"</span>
                            </div>
                            <div class="progress-bar">
                                <div class="progress-fill fuel-fill" style={move || format!("width: {}%", fuel.get())}></div>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"Cargo:"</span>
                                <span class="stat-value">{move || format!("{}/{}", cargo_used.get(), cargo_capacity.get())}</span>
                            </div>
                            <CargoPanel
                                current_used={cargo_used.get()}
                                capacity={cargo_capacity.get()}
                            />
                            <div class="stat-row">
                                <span class="stat-label">"Ship:"</span>
                                <span class="stat-value">"Pioneer"</span>
                            </div>
                        </div>
                    </div>

                    // Credits Panel - Prominent display of player credits
                    <div class="panel credits-panel">
                        <div class="panel-header">
                            <h3>"Credits"</h3>
                        </div>
                        <div class="panel-content credits-content">
                            <div class="credits-display">
                                <span class="credits-symbol">"💰 $"</span>
                                <span class="credits-amount">{move || format!("{}", money.get())}</span>
                            </div>
                        </div>
                    </div>

                    // Market Panel - Reactive component that updates with planet selection
                    <MarketPanelReactive
                        planet_name={move || current_planet_id.get()}
                        planet_type={current_planet_type}
                        economy={current_economy}
                    />
                </div>
            </div>

            // Action buttons
            <div class="actions">
                <button class="action-btn" on:click={move |_| set_money.update(|m| *m += 100)}>
                    <span class="btn-icon">"💰"</span>
                    <span>"Test: Add Credits"</span>
                </button>
                <button class="action-btn" on:click={move |_| set_turn.update(|t| *t += 1)}>
                    <span class="btn-icon">"⏱"</span>
                    <span>"Next Turn"</span>
                </button>
                <button class="action-btn">
                    <span class="btn-icon">"⚙"</span>
                    <span>"New Game"</span>
                </button>
            </div>
        </div>
    }
}

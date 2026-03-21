//! Web UI components for Rust Cowboyz

use leptos::*;
use crate::ui::solar_map::{SolarMap, MapPlanet};
use crate::simulation::planet_types::PlanetType;
use crate::simulation::orbits::Position;

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

    // Create solar system planet data
    let planets = vec![
        MapPlanet {
            id: "mercury".to_string(),
            name: "Mercury".to_string(),
            orbit_radius: 3,
            orbit_period: 4,
            position: Position::new(0),
            planet_type: PlanetType::Mining,
        },
        MapPlanet {
            id: "venus".to_string(),
            name: "Venus".to_string(),
            orbit_radius: 5,
            orbit_period: 6,
            position: Position::new(0),
            planet_type: PlanetType::Industrial,
        },
        MapPlanet {
            id: "earth".to_string(),
            name: "Earth".to_string(),
            orbit_radius: 7,
            orbit_period: 8,
            position: Position::new(0),
            planet_type: PlanetType::Agricultural,
        },
        MapPlanet {
            id: "mars".to_string(),
            name: "Mars".to_string(),
            orbit_radius: 10,
            orbit_period: 12,
            position: Position::new(0),
            planet_type: PlanetType::Mining,
        },
        MapPlanet {
            id: "jupiter".to_string(),
            name: "Jupiter".to_string(),
            orbit_radius: 15,
            orbit_period: 20,
            position: Position::new(0),
            planet_type: PlanetType::MegaCity,
        },
        MapPlanet {
            id: "saturn".to_string(),
            name: "Saturn".to_string(),
            orbit_radius: 20,
            orbit_period: 28,
            position: Position::new(0),
            planet_type: PlanetType::Industrial,
        },
        MapPlanet {
            id: "uranus".to_string(),
            name: "Uranus".to_string(),
            orbit_radius: 25,
            orbit_period: 36,
            position: Position::new(0),
            planet_type: PlanetType::ResearchOutpost,
        },
        MapPlanet {
            id: "neptune".to_string(),
            name: "Neptune".to_string(),
            orbit_radius: 30,
            orbit_period: 44,
            position: Position::new(0),
            planet_type: PlanetType::FrontierColony,
        },
        MapPlanet {
            id: "pluto".to_string(),
            name: "Pluto".to_string(),
            orbit_radius: 35,
            orbit_period: 52,
            position: Position::new(0),
            planet_type: PlanetType::PirateSpaceStation,
        },
    ];

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
                            planets={planets.clone()}
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
                                <span class="stat-label">"Credits:"</span>
                                <span class="stat-value credits"> {move || format!("${}", money.get())}</span>
                            </div>
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
                                <span class="stat-value"> {move || cargo_used.get()} "/ " {move || cargo_capacity.get()}</span>
                            </div>
                            <div class="progress-bar">
                                <div class="progress-fill cargo-fill" style={move || format!("width: {}%", (cargo_used.get() as f64 / cargo_capacity.get() as f64) * 100.0)}></div>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"Ship:"</span>
                                <span class="stat-value">"Pioneer"</span>
                            </div>
                        </div>
                    </div>

                    // Inventory Panel
                    <div class="panel inventory-panel">
                        <div class="panel-header">
                            <h3>"Inventory"</h3>
                        </div>
                        <div class="panel-content">
                            <div class="inventory-empty">
                                <p>"Cargo hold is empty"</p>
                            </div>
                            <div class="inventory-list">
                                // Placeholder inventory items
                            </div>
                        </div>
                    </div>

                    // Market Panel
                    <div class="panel market-panel">
                        <div class="panel-header">
                            <h3>"Market"</h3>
                            <span class="panel-subtitle">"Earth"</span>
                        </div>
                        <div class="panel-content">
                            <div class="market-table">
                                <div class="market-header">
                                    <span>"Item"</span>
                                    <span>"Buy"</span>
                                    <span>"Sell"</span>
                                </div>
                                <div class="market-row">
                                    <span>"Water"</span>
                                    <span class="buy-price">"$10"</span>
                                    <span class="sell-price">"$8"</span>
                                </div>
                                <div class="market-row">
                                    <span>"Food"</span>
                                    <span class="buy-price">"$25"</span>
                                    <span class="sell-price">"$20"</span>
                                </div>
                                <div class="market-row">
                                    <span>"Ore"</span>
                                    <span class="buy-price">"$50"</span>
                                    <span class="sell-price">"$40"</span>
                                </div>
                                <div class="market-row">
                                    <span>"Electronics"</span>
                                    <span class="buy-price">"$100"</span>
                                    <span class="sell-price">"$80"</span>
                                </div>
                            </div>
                        </div>
                    </div>
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

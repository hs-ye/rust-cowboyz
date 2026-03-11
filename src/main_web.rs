//! Web entry point for Rust Cowboyz
//!
//! This is the entry point for the web application compiled to WASM.

#![cfg(feature = "web")]
#![no_main]

use leptos::view;
use leptos::IntoView;
use leptos::component;
use leptos::create_signal;
use leptos::SignalSet;
use leptos::SignalGet;
use leptos::SignalUpdate;
use leptos::mount_to_body;
use leptos_meta::{Title, Meta};

use cowboyz::ui::travel_panel::{DestinationSelectionPanel, TravelPanel};
use cowboyz::ui::solar_map::{SolarMap, MapPlanet};
use cowboyz::game_state::{Planet, Ship};
use cowboyz::simulation::planet_types::PlanetType;
use cowboyz::simulation::orbits::Position;
use cowboyz::simulation::economy::PlanetEconomy;

/// Create sample planets for the solar system
fn create_sample_planets() -> Vec<Planet> {
    vec![
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
        Planet {
            id: "titan".to_string(),
            name: "Titan Station".to_string(),
            orbit_radius: 35,
            orbit_period: 30,
            position: Position::new(20),
            economy: PlanetEconomy::new(PlanetType::ResearchOutpost),
            planet_type: PlanetType::ResearchOutpost,
        },
        Planet {
            id: "pirate_haven".to_string(),
            name: "Pirate Haven".to_string(),
            orbit_radius: 45,
            orbit_period: 40,
            position: Position::new(5),
            economy: PlanetEconomy::new(PlanetType::PirateSpaceStation),
            planet_type: PlanetType::PirateSpaceStation,
        },
        Planet {
            id: "new_eden".to_string(),
            name: "New Eden".to_string(),
            orbit_radius: 55,
            orbit_period: 50,
            position: Position::new(30),
            economy: PlanetEconomy::new(PlanetType::FrontierColony),
            planet_type: PlanetType::FrontierColony,
        },
        Planet {
            id: "megacity_one".to_string(),
            name: "Mega City One".to_string(),
            orbit_radius: 8,
            orbit_period: 12,
            position: Position::new(3),
            economy: PlanetEconomy::new(PlanetType::MegaCity),
            planet_type: PlanetType::MegaCity,
        },
    ]
}

/// Main application component with 60/40 split-screen layout
#[component]
fn App() -> impl IntoView {
    // Create reactive game state
    let (money, set_money) = create_signal(1000);
    let (location, set_location) = create_signal("Earth".to_string());
    let (turn, set_turn) = create_signal(1);
    let (fuel, set_fuel) = create_signal(100);
    let (cargo_capacity, set_cargo_capacity) = create_signal(50);
    let (cargo_used, set_cargo_used) = create_signal(0);

    // Destination selection state
    let (show_destination_panel, set_show_destination_panel) = create_signal(false);
    let (show_travel_panel, set_show_travel_panel) = create_signal(false);
    let (selected_destination, set_selected_destination) = create_signal(Option::<String>::None);
    let (planets, set_planets) = create_signal(create_sample_planets());
    let (current_planet_id, set_current_planet_id) = create_signal("earth".to_string());
    let total_turns = 20; // Default game length

    view! {
        <Title text="Space Cowboys - Rust Cowboyz" />
        <Meta name="description" content="A space-western trading game built with Rust and Leptos" />

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
                        {move || {
                            let planets_val = planets.get();
                            let map_planets = planets_val.iter().map(|p| MapPlanet {
                                id: p.id.clone(),
                                name: p.name.clone(),
                                orbit_radius: p.orbit_radius,
                                orbit_period: p.orbit_period,
                                position: p.position,
                                planet_type: p.planet_type.clone(),
                            }).collect::<Vec<_>>();
                            let current_turn_val = turn.get();
                            let player_loc = location.get();
                            let selected_dest = selected_destination.get();
                            let current_planet_id_val = current_planet_id.get();

                            view! {
                                <SolarMap
                                    planets=map_planets
                                    current_turn=current_turn_val
                                    player_location=player_loc
                                    selected_planet=selected_dest
                                    on_planet_select=Some(Box::new(move |planet_id: String| {
                                        if planet_id != current_planet_id_val {
                                            set_selected_destination.set(Some(planet_id));
                                            set_show_travel_panel.set(true);
                                        }
                                    }))
                                />
                            }
                        }}
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
                <button class="action-btn" on:click={move |_| set_show_destination_panel.set(true)}>
                    <span class="btn-icon">"🚀"</span>
                    <span>"Travel"</span>
                </button>
                <button class="action-btn" on:click={move |_| set_money.update(|m| *m += 100)}>
                    <span class="btn-icon">"💰"</span>
                    <span>"Test: Add Credits"</span>
                </button>
                <button class="action-btn" on:click={move |_| {
                    // Advance turn counter
                    set_turn.update(|t| *t += 1);
                    // Advance planet positions
                    set_planets.update(|planets| {
                        for planet in planets.iter_mut() {
                            if planet.orbit_period > 0 {
                                planet.position.orbital_position = (planet.position.orbital_position + 1) % planet.orbit_period;
                            }
                        }
                    });
                }}>
                    <span class="btn-icon">"⏱"</span>
                    <span>"Next Turn"</span>
                </button>
                <button class="action-btn">
                    <span class="btn-icon">"⚙"</span>
                    <span>"New Game"</span>
                </button>
            </div>

            // Destination Selection Modal
            {move || {
                if show_destination_panel.get() {
                    let planets_val = planets.get();
                    let current_planet_id_val = current_planet_id.get();
                    let selected_dest = selected_destination.get();

                    view! {
                        <div class="modal-overlay" style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.8); z-index: 100; display: flex; align-items: center; justify-content: center; padding: 2rem;">
                            <DestinationSelectionPanel
                                planets=planets_val
                                current_planet_id=current_planet_id_val
                                selected_planet_id=selected_dest
                                on_select=Some(Box::new(move |planet_id: String| {
                                    set_selected_destination.set(Some(planet_id));
                                    set_show_destination_panel.set(false);
                                    set_show_travel_panel.set(true);
                                }))
                                on_cancel=Some(Box::new(move || {
                                    set_show_destination_panel.set(false);
                                }))
                            />
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}

            // Travel Confirmation Panel (shows time and fuel cost)
            {move || {
                if show_travel_panel.get() {
                    let planets_val = planets.get();
                    let selected_dest_id = selected_destination.get();
                    let current_id = current_planet_id.get();
                    let origin_planet = planets_val.iter().find(|p| p.id == current_id).cloned();
                    let dest_planet = selected_dest_id.and_then(|id| {
                        planets_val.iter().find(|p| p.id == id).cloned()
                    });
                    let current_fuel = fuel.get();
                    let current_turn_num = turn.get();

                    // Clone for closures
                    let origin_for_travel = origin_planet.clone();
                    let dest_for_travel = dest_planet.clone();

                    view! {
                        <div class="modal-overlay" style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.8); z-index: 101; display: flex; align-items: center; justify-content: center; padding: 2rem;">
                            <TravelPanel
                                origin_planet=origin_planet
                                destination_planet=dest_planet
                                player_fuel=current_fuel
                                ship_acceleration=1
                                current_turn=current_turn_num
                                total_turns=total_turns
                                on_travel_confirm=Box::new(move || {
                                    // Execute travel: consume fuel and advance turns
                                    if let Some(ref dest) = dest_for_travel {
                                        let distance = origin_for_travel.as_ref().map(|o| o.orbit_radius.abs_diff(dest.orbit_radius)).unwrap_or(0);
                                        let travel_turns = cowboyz::simulation::travel::calculate_travel_turns_from_radii(
                                            origin_for_travel.as_ref().unwrap().orbit_radius,
                                            dest.orbit_radius,
                                            1
                                        );
                                        set_fuel.update(|f: &mut u32| *f = f.saturating_sub(distance.max(1)));
                                        set_turn.update(|t| *t += travel_turns);
                                        set_location.set(dest.name.clone());
                                        set_current_planet_id.set(dest.id.clone());
                                        set_selected_destination.set(None);
                                        set_show_travel_panel.set(false);
                                    }
                                })
                                on_cancel=Box::new(move || {
                                    set_show_travel_panel.set(false);
                                    set_selected_destination.set(None);
                                })
                            />
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
        </div>
    }
}

// Note: The web entry point is now in src/lib.rs via the `start()` function
// with #[wasm_bindgen(start)] attribute. This file only contains the App component.
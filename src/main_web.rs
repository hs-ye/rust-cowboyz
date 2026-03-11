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
use leptos_meta::{Title, Meta};
use wasm_bindgen::JsCast;

use cowboyz::ui::travel_panel::{DestinationSelectionPanel, TravelPanel};
use cowboyz::ui::solar_map::{SolarMap, MapPlanet};
use cowboyz::game_state::{Planet, GameState, GameSettings, Player, SolarSystem, TravelState};
use cowboyz::simulation::planet_types::PlanetType;
use cowboyz::simulation::orbits::Position;
use cowboyz::simulation::economy::PlanetEconomy;
use cowboyz::assets::save_game::{save_game_to_browser, load_game_from_browser};

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

/// Create a new game state with default settings
fn create_new_game_state() -> GameState {
    let planets = create_sample_planets();
    let solar_system = SolarSystem::new("Sol System".to_string(), planets);
    let settings = GameSettings::default();
    let player = Player::new();
    
    GameState::with_settings(player, solar_system, settings)
}

/// Main application component with 60/40 split-screen layout
#[component]
fn App() -> impl IntoView {
    // Try to load saved game state, or create new one
    let initial_game_state = match load_game_from_browser() {
        Ok(state) => {
            web_sys::console::log_1(&"Loaded saved game state".into());
            state
        }
        Err(_) => {
            web_sys::console::log_1(&"Creating new game state".into());
            create_new_game_state()
        }
    };

    // Create reactive game state using a single RwSignal for the entire GameState
    let (game_state, set_game_state) = create_signal(initial_game_state);
    
    // UI state signals
    let (show_destination_panel, set_show_destination_panel) = create_signal(false);
    let (show_travel_panel, set_show_travel_panel) = create_signal(false);
    let (selected_destination, set_selected_destination) = create_signal(Option::<String>::None);
    let (travel_error, set_travel_error) = create_signal(Option::<String>::None);
    let (travel_notification, set_travel_notification) = create_signal(Option::<String>::None);

    // Helper function to save game state to localStorage
    let save_game = move |state: &GameState| {
        if let Err(e) = save_game_to_browser(state) {
            web_sys::console::error_1(&format!("Failed to save game: {}", e).into());
        }
    };

    // Helper function to get current planet name
    let get_current_planet_name = move || {
        let state = game_state.get();
        state.get_current_planet()
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    };

    // Helper function to check if in transit
    let is_in_transit = move || {
        game_state.get().is_in_transit()
    };

    // Helper function to get turns remaining
    let turns_remaining = move || {
        game_state.get().turns_until_arrival()
    };

    // Helper function to get destination name
    let get_destination_name = move || {
        let state = game_state.get();
        state.get_destination()
            .and_then(|dest_id| state.solar_system.get_planet(dest_id))
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    };

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
                            let state = game_state.get();
                            let map_planets = state.solar_system.planets.iter().map(|p| MapPlanet {
                                id: p.id.clone(),
                                name: p.name.clone(),
                                orbit_radius: p.orbit_radius,
                                orbit_period: p.orbit_period,
                                position: p.position,
                                planet_type: p.planet_type.clone(),
                            }).collect::<Vec<_>>();
                            let current_turn_val = state.game_clock.current_turn;
                            let player_loc = state.player.location.clone();
                            let selected_dest = selected_destination.get();
                            let current_planet_id_val = state.player.location.clone();

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
                                            set_travel_error.set(None);
                                        }
                                    }))
                                />
                            }
                        }}
                    </div>
                </div>

                // Right side (40%): Information Panels
                <div class="info-panels">
                    // Travel Status Panel (shown when in transit)
                    {move || {
                        if is_in_transit() {
                            let dest_name = get_destination_name();
                            let turns = turns_remaining();
                            view! {
                                <div class="panel travel-status-panel in-transit">
                                    <div class="panel-header">
                                        <h3>"🚀 In Transit"</h3>
                                    </div>
                                    <div class="panel-content">
                                        <div class="travel-progress">
                                            <div class="travel-route-display">
                                                <span class="origin">{get_current_planet_name()}</span>
                                                <span class="route-arrow">" → "</span>
                                                <span class="destination">{dest_name}</span>
                                            </div>
                                            <div class="turns-remaining">
                                                <span class="turns-label">"Turns Remaining: "</span>
                                                <span class="turns-value">{turns}</span>
                                            </div>
                                            <div class="progress-bar travel-progress-bar">
                                                <div class="progress-fill" style={move || {
                                                    let state = game_state.get();
                                                    if let TravelState::InTransit { departure_turn, arrival_turn, .. } = &state.player.travel_state {
                                                        let total = *arrival_turn - *departure_turn;
                                                        let remaining = state.turns_until_arrival();
                                                        let progress = if total > 0 {
                                                            ((total - remaining) as f64 / total as f64 * 100.0).min(100.0)
                                                        } else {
                                                            100.0
                                                        };
                                                        format!("width: {}%", progress)
                                                    } else {
                                                        "width: 0%".to_string()
                                                    }
                                                }}></div>
                                            </div>
                                            <div class="travel-status-message">
                                                <span class="status-icon">"⏱"</span>
                                                <span>"Traveling..."</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            view! { <div></div> }
                        }
                    }}

                    // Player Status Panel
                    <div class="panel player-panel">
                        <div class="panel-header">
                            <h3>"Player Status"</h3>
                        </div>
                        <div class="panel-content">
                            <div class="stat-row">
                                <span class="stat-label">"Credits:"</span>
                                <span class="stat-value credits"> {move || format!("${}", game_state.get().player.money)}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"Location:"</span>
                                <span class="stat-value location">{move || get_current_planet_name()}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"Turn:"</span>
                                <span class="stat-value turn">{move || game_state.get().game_clock.current_turn} " / " {move || game_state.get().game_clock.total_turns}</span>
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
                                <span class="stat-value fuel"> {move || game_state.get().player.ship.fuel} "/" {move || game_state.get().player.ship.max_fuel}</span>
                            </div>
                            <div class="progress-bar">
                                <div class="progress-fill fuel-fill" style={move || {
                                    let state = game_state.get();
                                    let pct = (state.player.ship.fuel as f64 / state.player.ship.max_fuel as f64 * 100.0).min(100.0);
                                    format!("width: {}%", pct)
                                }}></div>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"Cargo:"</span>
                                <span class="stat-value"> {move || game_state.get().player.cargo.total_cargo_space_used()} "/" {move || game_state.get().player.cargo.capacity}</span>
                            </div>
                            <div class="progress-bar">
                                <div class="progress-fill cargo-fill" style={move || {
                                    let state = game_state.get();
                                    let pct = if state.player.cargo.capacity > 0 {
                                        (state.player.cargo.total_cargo_space_used() as f64 / state.player.cargo.capacity as f64 * 100.0).min(100.0)
                                    } else {
                                        0.0
                                    };
                                    format!("width: {}%", pct)
                                }}></div>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"Hull:"</span>
                                <span class="stat-value"> {move || game_state.get().player.ship.hull} "/" {move || game_state.get().player.ship.max_hull}</span>
                            </div>
                            <div class="progress-bar">
                                <div class="progress-fill hull-fill" style={move || {
                                    let state = game_state.get();
                                    let pct = (state.player.ship.hull as f64 / state.player.ship.max_hull as f64 * 100.0).min(100.0);
                                    format!("width: {}%", pct)
                                }}></div>
                            </div>
                        </div>
                    </div>

                    // Inventory Panel
                    <div class="panel inventory-panel">
                        <div class="panel-header">
                            <h3>"Inventory"</h3>
                        </div>
                        <div class="panel-content">
                            {move || {
                                let state = game_state.get();
                                if state.player.cargo.is_empty() {
                                    view! {
                                        <div class="inventory-empty">
                                            <p>"Cargo hold is empty"</p>
                                        </div>
                                    }
                                } else {
                                    view! {
                                        <div class="inventory-list">
                                            {state.player.cargo.commodities.iter().map(|(commodity, qty)| {
                                                view! {
                                                    <div class="inventory-item">
                                                        <span class="item-name">{commodity.display_name()}</span>
                                                        <span class="item-qty">{*qty}</span>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }
                                }
                            }}
                        </div>
                    </div>

                    // Market Panel
                    <div class="panel market-panel">
                        <div class="panel-header">
                            <h3>"Market"</h3>
                            <span class="panel-subtitle">{move || get_current_planet_name()}</span>
                        </div>
                        <div class="panel-content">
                            {move || {
                                let state = game_state.get();
                                if let Some(planet) = state.get_current_planet() {
                                    let market = &planet.economy.market;
                                    if market.is_empty() {
                                        view! {
                                            <div class="market-empty">
                                                <p>"No commodities available"</p>
                                            </div>
                                        }
                                    } else {
                                        view! {
                                            <div class="market-table">
                                                <div class="market-header">
                                                    <span>"Item"</span>
                                                    <span>"Buy"</span>
                                                    <span>"Sell"</span>
                                                </div>
                                                {market.iter().map(|(commodity, data)| {
                                                    view! {
                                                        <div class="market-row">
                                                            <span>{commodity.display_name()}</span>
                                                            <span class="buy-price">{format!("${}", data.buy_price)}</span>
                                                            <span class="sell-price">{format!("${}", data.sell_price)}</span>
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        }
                                    }
                                } else {
                                    view! {
                                        <div class="market-empty">
                                            <p>"No market data available"</p>
                                        </div>
                                    }
                                }
                            }}
                        </div>
                    </div>
                </div>
            </div>

            // Action buttons
            <div class="actions">
                <button 
                    class="action-btn" 
                    disabled={move || is_in_transit()}
                    on:click={move |_| {
                        if !is_in_transit() {
                            set_show_destination_panel.set(true);
                            set_travel_error.set(None);
                        }
                    }}
                >
                    <span class="btn-icon">"🚀"</span>
                    <span>"Travel"</span>
                </button>
                <button class="action-btn" on:click={move |_| {
                    set_game_state.update(|state| {
                        state.player.money += 100;
                        save_game(state);
                    });
                }}>
                    <span class="btn-icon">"💰"</span>
                    <span>"Test: Add Credits"</span>
                </button>
                <button 
                    class="action-btn" 
                    on:click={move |_| {
                        set_game_state.update(|state| {
                            // Advance one turn and check for arrival
                            let arrival = state.next_turn();
                            
                            // Show arrival notification if ship arrived
                            if let Some(event) = arrival {
                                set_travel_notification.set(Some(format!(
                                    "Arrived at {}! Travel took {} turns.",
                                    event.destination_planet_id, event.travel_turns
                                )));
                                // Clear notification after 3 seconds
                                let window = web_sys::window().unwrap();
                                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                                    set_travel_notification.set(None);
                                }) as Box<dyn Fn()>);
                                window.set_timeout_with_callback_and_timeout_and_arguments_0(
                                    closure.as_ref().unchecked_ref(),
                                    3000
                                ).unwrap();
                                closure.forget();
                            }
                            
                            save_game(state);
                        });
                    }}
                >
                    <span class="btn-icon">"⏱"</span>
                    <span>"Next Turn"</span>
                </button>
                <button class="action-btn" on:click={move |_| {
                    // Create new game
                    let new_state = create_new_game_state();
                    set_game_state.set(new_state.clone());
                    save_game(&new_state);
                    set_selected_destination.set(None);
                    set_travel_error.set(None);
                }}>
                    <span class="btn-icon">"⚙"</span>
                    <span>"New Game"</span>
                </button>
            </div>

            // Travel Error Notification
            {move || {
                if let Some(error) = travel_error.get() {
                    view! {
                        <div class="notification error-notification">
                            <span class="notification-icon">"⚠"</span>
                            <span class="notification-message">{error}</span>
                            <button class="notification-close" on:click={move |_| set_travel_error.set(None)}>"✕"</button>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}

            // Travel Success Notification
            {move || {
                if let Some(notification) = travel_notification.get() {
                    view! {
                        <div class="notification success-notification">
                            <span class="notification-icon">"✓"</span>
                            <span class="notification-message">{notification}</span>
                            <button class="notification-close" on:click={move |_| set_travel_notification.set(None)}>"✕"</button>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}

            // Destination Selection Modal
            {move || {
                if show_destination_panel.get() {
                    let state = game_state.get();
                    let planets_val = state.solar_system.planets.clone();
                    let current_planet_id_val = state.player.location.clone();
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
                                    set_travel_error.set(None);
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
                    let state = game_state.get();
                    let selected_dest_id = selected_destination.get();
                    let current_id = state.player.location.clone();
                    let origin_planet = state.solar_system.get_planet(&current_id).cloned();
                    let dest_planet = selected_dest_id.as_ref().and_then(|id| {
                        state.solar_system.get_planet(id).cloned()
                    });
                    let current_fuel = state.player.ship.fuel;
                    let current_turn_num = state.game_clock.current_turn;
                    let total_turns = state.game_clock.total_turns;

                    // Clone for closures
                    let dest_id_for_travel = selected_dest_id.clone();

                    view! {
                        <div class="modal-overlay" style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.8); z-index: 101; display: flex; align-items: center; justify-content: center; padding: 2rem;">
                            <TravelPanel
                                origin_planet=origin_planet
                                destination_planet=dest_planet
                                player_fuel=current_fuel
                                ship_acceleration=state.player.ship.acceleration
                                current_turn=current_turn_num
                                total_turns=total_turns
                                on_travel_confirm=Box::new(move || {
                                    if let Some(ref dest_id) = dest_id_for_travel {
                                        set_game_state.update(|state| {
                                            // Use the backend initiate_travel method
                                            match state.initiate_travel(dest_id) {
                                                Ok(()) => {
                                                    // Travel initiated successfully
                                                    set_show_travel_panel.set(false);
                                                    set_selected_destination.set(None);
                                                    set_travel_error.set(None);
                                                    
                                                    // Save the updated state
                                                    save_game(state);
                                                    
                                                    web_sys::console::log_1(
                                                        &format!("Travel initiated to {}. Arrival in {} turns.", 
                                                            dest_id, state.turns_until_arrival()).into()
                                                    );
                                                }
                                                Err(e) => {
                                                    // Show error message
                                                    set_travel_error.set(Some(e.to_string()));
                                                    web_sys::console::error_1(
                                                        &format!("Travel failed: {}", e).into()
                                                    );
                                                }
                                            }
                                        });
                                    }
                                })
                                on_cancel=Box::new(move || {
                                    set_show_travel_panel.set(false);
                                    set_selected_destination.set(None);
                                    set_travel_error.set(None);
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

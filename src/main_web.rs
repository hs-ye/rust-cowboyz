//! Web entry point for Rust Cowboyz
//!
//! This is the entry point for the web application compiled to WASM.

use leptos::prelude::*;
use leptos_meta::{Title, Meta};
use crate::ui::solar_map::{SolarMap, MapPlanet};
use crate::ui::game_config_modal::{GameConfigModal, GameConfig};
use crate::ui::travel_panel::{TravelPanel, TravelAnimation};
use crate::simulation::planet_types::PlanetType;
use crate::game_state::{GameState, GameDifficulty, GameSettings, Player, Ship, CargoHold, SolarSystem, Planet, GameClock, validate_game_state, ValidationResult};
use crate::assets::save_game::{save_game_to_browser, load_game_from_browser, has_saved_game, LOCAL_STORAGE_KEY, SaveLoadError};

/// Calculate travel time using Brachistochrone model
/// Formula: travel_turns = 2 * sqrt(base_distance / acceleration)
fn calculate_travel_turns(origin_orbit_radius: u32, dest_orbit_radius: u32, ship_acceleration: u32) -> u32 {
    let base_distance = origin_orbit_radius.abs_diff(dest_orbit_radius);
    
    if base_distance == 0 {
        return 1;
    }

    let travel_turns = 2.0 * (base_distance as f64 / ship_acceleration.max(1) as f64).sqrt();
    std::cmp::max(travel_turns.ceil() as u32, 1)
}

/// Main application component with 60/40 split-screen layout
#[component]
fn App() -> impl IntoView {
    // Create reactive game state - using a stored value that wraps GameState
    let (game_state, set_game_state) = signal(GameState::new());
    
    // Derived signals for UI display
    let money = Signal::derive(move || game_state.get().player.money as i32);
    let location = Signal::derive(move || game_state.get().player.location.clone());
    let turn = Signal::derive(move || game_state.get().game_clock.current_turn);
    let fuel = Signal::derive(move || game_state.get().player.ship.fuel as i32);
    let cargo_capacity = Signal::derive(move || game_state.get().player.ship.cargo_capacity as i32);
    let cargo_used = Signal::derive(move || game_state.get().player.cargo.total_cargo_space_used() as i32);
    
    // Save status for UI feedback
    let (save_status, set_save_status) = signal(Option::<String>::None);
    let (load_error, set_load_error) = signal(Option::<String>::None);
    let (is_loading, set_is_loading) = signal(true);

    // Selected planet for info panel
    let (selected_planet, set_selected_planet) = signal(Option::<String>::None);

    // Game configuration modal state
    let (is_modal_open, set_is_modal_open) = signal(false);
    let (has_existing_game, set_has_existing_game) = signal(false);

    // Travel animation state
    let (is_traveling, set_is_traveling) = signal(false);
    let (travel_origin_name, set_travel_origin_name) = signal(String::new());
    let (travel_destination_name, set_travel_destination_name) = signal(String::new());

    // Initialize game state from localStorage on mount
    let initialize_game = move || {
        set_is_loading.set(true);
        
        // Check if there's a saved game
        if has_saved_game() {
            match load_game_from_browser() {
                Ok(loaded_state) => {
                    let validation = validate_game_state(&loaded_state);
                    if validation.is_valid {
                        set_game_state.set(loaded_state);
                        set_has_existing_game.set(true);
                        set_load_error.set(None);
                    } else {
                        set_load_error.set(Some(format!("Validation failed: {}", validation.errors.join(", "))));
                        // Start with new game if validation fails
                        set_game_state.set(GameState::new());
                    }
                }
                Err(e) => {
                    set_load_error.set(Some(format!("Failed to load game: {}", e)));
                    set_game_state.set(GameState::new());
                }
            }
        } else {
            set_game_state.set(GameState::new());
        }
        set_is_loading.set(false);
    };

    // Run initialization on mount
    on_mount(initialize_game);

    // Auto-save function - triggers on significant game actions
    let auto_save = move || {
        let current_state = game_state.get();
        match save_game_to_browser(&current_state) {
            Ok(()) => {
                set_save_status.set(Some("Auto-saved".to_string()));
                // Clear status after 2 seconds
                set_timeout(move || set_save_status.set(None), 2000);
            }
            Err(e) => {
                set_save_status.set(Some(format!("Auto-save failed: {}", e)));
            }
        }
    };

    // Manual save handler
    let on_manual_save = move |_| {
        let current_state = game_state.get();
        match save_game_to_browser(&current_state) {
            Ok(()) => {
                set_save_status.set(Some("Game saved!".to_string()));
                set_timeout(move || set_save_status.set(None), 2000);
            }
            Err(e) => {
                set_save_status.set(Some(format!("Save failed: {}", e)));
            }
        }
    };

    // Manual load handler
    let on_manual_load = move |_| {
        match load_game_from_browser() {
            Ok(loaded_state) => {
                let validation = validate_game_state(&loaded_state);
                if validation.is_valid {
                    set_game_state.set(loaded_state);
                    set_has_existing_game.set(true);
                    set_load_error.set(None);
                    set_save_status.set(Some("Game loaded!".to_string()));
                    set_timeout(move || set_save_status.set(None), 2000);
                } else {
                    set_load_error.set(Some(format!("Validation failed: {}", validation.errors.join(", "))));
                }
            }
            Err(e) => {
                set_load_error.set(Some(format!("Load failed: {}", e)));
            }
        }
    };

    // Handle new game button click
    let on_new_game_click = move |_| {
        set_is_modal_open.set(true);
    };

    // Handle game start from modal
    let on_game_start = move |config: GameConfig| {
        // Create new game state with configuration
        let difficulty = match config.difficulty.as_str() {
            "easy" => GameDifficulty::Easy,
            "hard" => GameDifficulty::Hard,
            _ => GameDifficulty::Normal,
        };
        
        let settings = GameSettings {
            difficulty,
            ..Default::default()
        };
        
        let total_turns = settings.difficulty.turn_limit();
        
        let new_state = GameState {
            version: "1.0.0".to_string(),
            player: Player {
                money: settings.difficulty.starting_money(),
                location: "earth".to_string(),
                ship: Ship::new(10.0, 50),
                cargo: CargoHold::new(50),
                visited_planets: vec!["earth".to_string()],
                total_trades: 0,
                total_earnings: 0,
            },
            solar_system: create_sample_solar_system(),
            game_clock: GameClock::new(total_turns),
            settings,
            transaction_history: Vec::new(),
            is_game_over: false,
            game_over_reason: None,
        };
        
        set_game_state.set(new_state);
        set_has_existing_game.set(true);
        set_selected_planet.set(None);
        
        // Auto-save after starting new game
        auto_save();
    };

    // Handle modal close
    let on_modal_close = move |_| {
        set_is_modal_open.set(false);
    };

    // Create sample planets for the solar map
    let planets = create_map_planets();

    // Handle planet selection - just select for travel, don't change location yet
    let on_planet_select = move |planet_id: String| {
        set_selected_planet.set(Some(planet_id.clone()));
    };

    // Handle travel confirmation
    let on_travel_confirm = move || {
        let destination_id = match selected_planet.get() {
            Some(id) => id,
            None => return,
        };

        let current_state = game_state.get();
        let origin_planet = current_state.solar_system.get_planet(&current_state.player.location);
        let dest_planet = current_state.solar_system.get_planet(&destination_id);

        match (origin_planet, dest_planet) {
            (Some(origin), Some(dest)) => {
                // Calculate travel cost
                let distance = origin.orbit_radius.abs_diff(dest.orbit_radius);
                let fuel_required = distance.max(1);
                let travel_turns = calculate_travel_turns(origin.orbit_radius, dest.orbit_radius, current_state.player.ship.acceleration);

                // Check if player has enough fuel
                if current_state.player.ship.fuel < fuel_required {
                    // Not enough fuel - could add a toast notification here
                    return;
                }

                // Set travel animation state
                set_travel_origin_name.set(origin.name.clone());
                set_travel_destination_name.set(dest.name.clone());
                set_is_traveling.set(true);

                // Perform travel after a short delay to show animation
                let game_state_clone = game_state.clone();
                let set_game_state_clone = set_game_state.clone();
                let set_selected_planet_clone = set_selected_planet.clone();
                let set_is_traveling_clone = set_is_traveling.clone();
                let auto_save_clone = auto_save.clone();

                set_timeout(move || {
                    set_game_state_clone.update(|state| {
                        // Consume fuel
                        state.player.ship.fuel = state.player.ship.fuel.saturating_sub(fuel_required);
                        // Update location
                        state.player.location = destination_id.clone();
                        // Advance game clock
                        state.advance_turns(travel_turns);
                    });
                    
                    // Clear selection and stop animation
                    set_selected_planet_clone.set(None);
                    set_is_traveling_clone.set(false);
                    
                    // Auto-save after travel
                    auto_save_clone();
                }, 2000);
            }
            _ => {
                // Invalid planets
            }
        }
    };

    // Handle travel cancel
    let on_travel_cancel = move || {
        set_selected_planet.set(None);
    };

    // Handle travel animation complete
    let on_travel_animation_complete = move || {
        set_is_traveling.set(false);
    };

    // Get selected planet info for display
    let selected_planet_info = move || {
        selected_planet.get().and_then(|id| {
            planets.iter().find(|p| p.id == id).cloned()
        })
    };

    // Handle turn advance with auto-save
    let on_advance_turn = move |_| {
        set_game_state.update(|state| {
            state.advance_turns(1);
        });
        // Auto-save after turn advance
        auto_save();
    };

    // Handle money change with auto-save
    let on_add_money = move |_| {
        set_game_state.update(|state| {
            state.player.money += 100;
        });
        // Auto-save after significant action
        auto_save();
    };

    view! {
        <Title text="太空牛仔 - Rust Cowboyz" />
        <Meta name="description" content="A space-western trading game built with Rust and Leptos" />

        <div class="app-container">
            <header class="app-header">
                <h1>"太空牛仔" </h1>
                <span class="subtitle">"Space-Western Trading Game"</span>
                // Save status indicator
                {move || {
                    save_status.get().map(|status| {
                        view! {
                            <span class="save-status">{status}</span>
                        }
                    })
                }}
            </header>

            // Loading state
            {move || {
                if is_loading.get() {
                    view! {
                        <div class="loading">"Loading game..."</div>
                    }
                } else {
                    view! { <></> }
                }
            }}

            // Load error display
            {move || {
                load_error.get().map(|error| {
                    view! {
                        <div class="error-message">{"Error: " {error}}</div>
                    }
                })
            }}

            <div class="split-layout">
                // Left side (60%): Solar System Map
                <div class="map-panel">
                    <div class="panel-header">
                        <h2>"太阳系地图" </h2>
                        <span class="panel-subtitle">"Solar System Map"</span>
                    </div>
                    <div class="map-viewport">
                        <SolarMap
                            planets=planets
                            current_turn=turn.get()
                            player_location=location.get()
                            selected_planet=selected_planet.get()
                            on_planet_select=Box::new(on_planet_select)
                        />
                    </div>
                </div>

                // Right side (40%): Information Panels
                <div class="info-panels">
                    // Player Status Panel
                    <div class="panel player-panel">
                        <div class="panel-header">
                            <h3>"玩家状态" </h3>
                            <span class="panel-subtitle">"Player Status"</span>
                            <div class="info-tooltip">
                                <span class="info-icon">?</span>
                                <div class="tooltip-content">
                                    "Track your credits, location, and game progress. Your reputation increases as you complete trades and missions."
                                </div>
                            </div>
                        </div>
                        <div class="panel-content">
                            <div class="stat-row">
                                <span class="stat-label">
                                    "资金 Credits:"
                                    <span class="tooltip">
                                        <span class="tooltip-icon">?</span>
                                        <span class="tooltip-text">"Use credits to buy goods, fuel, and ship upgrades. Earn money by trading between planets."</span>
                                    </span>
                                </span>
                                <span class="stat-value credits"> {move || format!("${}", money())}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">
                                    "位置 Location:"
                                    <span class="tooltip">
                                        <span class="tooltip-icon">?</span>
                                        <span class="tooltip-text">"Your current planet in the solar system. Click planets on the map to travel."</span>
                                    </span>
                                </span>
                                <span class="stat-value location">{location}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">
                                    "回合 Turn:"
                                    <span class="tooltip">
                                        <span class="tooltip-icon">?</span>
                                        <span class="tooltip-text">"Each turn represents time passing in the game. Some actions advance turns (travel, certain events)."</span>
                                    </span>
                                </span>
                                <span class="stat-value turn">{turn}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">
                                    "声望 Reputation:"
                                    <span class="tooltip">
                                        <span class="tooltip-icon">?</span>
                                        <span class="tooltip-text">"Build reputation by completing trades. Higher reputation unlocks better prices and exclusive goods."</span>
                                    </span>
                                </span>
                                <span class="stat-value">"新秀 Rookie"</span>
                            </div>
                        </div>
                    </div>

                    // Selected Planet Info Panel
                    <div class="panel selected-planet-panel">
                        <div class="panel-header">
                            <h3>"行星信息" </h3>
                            <span class="panel-subtitle">"Planet Info"</span>
                        </div>
                        <div class="panel-content">
                            {move || {
                                match selected_planet_info() {
                                    Some(planet) => {
                                        view! {
                                            <div class="planet-info">
                                                <div class="planet-name">{planet.name}</div>
                                                <div class="planet-type" style={format!("color: {}", get_planet_color_for_css(&planet.planet_type))}>
                                                    {planet.planet_type.display_name()}
                                                </div>
                                                <div class="planet-stats">
                                                    <div class="stat-row">
                                                        <span class="stat-label">"轨道半径 Orbit:"</span>
                                                        <span class="stat-value">{planet.orbit_radius}</span>
                                                    </div>
                                                    <div class="stat-row">
                                                        <span class="stat-label">"公转周期 Period:"</span>
                                                        <span class="stat-value">{planet.orbit_period} turns</span>
                                                    </div>
                                                    <div class="stat-row">
                                                        <span class="stat-label">"当前位置 Position:"</span>
                                                        <span class="stat-value">{calculate_position_at_turn(planet.orbit_period, turn.get())}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                    None => {
                                        view! {
                                            <div class="no-selection">
                                                <p>"点击地图选择行星"</p>
                                                <p class="hint">"Click map to select planet"</p>
                                            </div>
                                        }
                                    }
                                }
                            }}
                        </div>
                    </div>

                    // Travel Selection Panel
                    {move || {
                        let current_state = game_state.get();
                        let origin_planet = current_state.solar_system.get_planet(&current_state.player.location).cloned();
                        let dest_planet = selected_planet.get()
                            .and_then(|id| current_state.solar_system.get_planet(&id).cloned());
                        let player_fuel = current_state.player.ship.fuel;
                        let ship_accel = current_state.player.ship.acceleration;
                        let current_turn = current_state.game_clock.current_turn;
                        let total_turns = current_state.game_clock.total_turns;

                        view! {
                            <TravelPanel
                                origin_planet={origin_planet}
                                destination_planet={dest_planet}
                                player_fuel={player_fuel}
                                ship_acceleration={ship_accel}
                                current_turn={current_turn}
                                total_turns={total_turns}
                                on_travel_confirm={Box::new(on_travel_confirm)}
                                on_cancel={Box::new(on_travel_cancel)}
                            />
                        }
                    }}

                    // Ship Status Panel
                    <div class="panel ship-panel">
                        <div class="panel-header">
                            <h3>"飞船状态" </h3>
                            <span class="panel-subtitle">"Ship Status"</span>
                            <div class="info-tooltip">
                                <span class="info-icon">?</span>
                                <div class="tooltip-content">
                                    "Monitor your ship's fuel and cargo capacity. Refuel at stations, upgrade your cargo hold for more trading space."
                                </div>
                            </div>
                        </div>
                        <div class="panel-content">
                            <div class="stat-row">
                                <span class="stat-label">
                                    "燃料 Fuel:"
                                    <span class="tooltip">
                                        <span class="tooltip-icon">?</span>
                                        <span class="tooltip-text">"Fuel is consumed when traveling between planets. The farther the distance, the more fuel required."</span>
                                    </span>
                                </span>
                                <span class="stat-value fuel"> {fuel()} "/ 100"</span>
                            </div>
                            <div class="progress-bar">
                                <div class="progress-fill fuel-fill" style={move || format!("width: {}%", fuel())}></div>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">
                                    "货舱 Cargo:"
                                    <span class="tooltip">
                                        <span class="tooltip-icon">?</span>
                                        <span class="tooltip-text">"Your cargo hold capacity. Buy goods at low prices and sell at high prices to make profit."</span>
                                    </span>
                                </span>
                                <span class="stat-value"> {cargo_used()} "/ " {cargo_capacity()}</span>
                            </div>
                            <div class="progress-bar">
                                <div class="progress-fill cargo-fill" style={move || format!("width: {}%", (cargo_used() as f64 / cargo_capacity() as f64) * 100.0)}></div>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"飞船 Ship:"</span>
                                <span class="stat-value">"开拓者号 Pioneer"</span>
                            </div>
                        </div>
                    </div>

                    // Inventory Panel
                    <div class="panel inventory-panel">
                        <div class="panel-header">
                            <h3>"库存" </h3>
                            <span class="panel-subtitle">"Inventory"</span>
                        </div>
                        <div class="panel-content">
                            <div class="inventory-empty">
                                <p>"货舱为空"</p>
                                <p class="hint">"Cargo hold is empty"</p>
                            </div>
                            <div class="inventory-list">
                                // Placeholder inventory items
                            </div>
                        </div>
                    </div>

                    // Market Panel
                    <div class="panel market-panel">
                        <div class="panel-header">
                            <h3>"市场" </h3>
                            <span class="panel-subtitle">"Market - Earth"</span>
                            <div class="info-tooltip">
                                <span class="info-icon">?</span>
                                <div class="tooltip-content">
                                    "Buy goods at low prices and sell at high prices across different planets. Prices vary by planet type and economy."
                                </div>
                            </div>
                        </div>
                        <div class="panel-content">
                            <div class="market-table">
                                <div class="market-header">
                                    <span>"商品 Item"</span>
                                    <span>"买入 Buy"</span>
                                    <span>"卖出 Sell"</span>
                                </div>
                                <div class="market-row">
                                    <span>"水 Water"</span>
                                    <span class="buy-price">"$10"</span>
                                    <span class="sell-price">"$8"</span>
                                </div>
                                <div class="market-row">
                                    <span>"食物 Food"</span>
                                    <span class="buy-price">"$25"</span>
                                    <span class="sell-price">"$20"</span>
                                </div>
                                <div class="market-row">
                                    <span>"矿石 Ore"</span>
                                    <span class="buy-price">"$50"</span>
                                    <span class="sell-price">"$40"</span>
                                </div>
                                <div class="market-row">
                                    <span>"电子元件 Electronics"</span>
                                    <span class="buy-price">"$100"</span>
                                    <span class="sell-price">"$80"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Action buttons with save/load
            <div class="actions">
                <button 
                    class="action-btn" 
                    on:click={on_add_money}
                    title="Add test money (debug feature)"
                >
                    <span class="btn-icon">"💰"</span>
                    <span>"测试: 增加资金"</span>
                </button>
                <button 
                    class="action-btn" 
                    on:click={on_advance_turn}
                    title="Advance to next turn"
                >
                    <span class="btn-icon">"⏱"</span>
                    <span>"下一回合"</span>
                </button>
                <button 
                    class="action-btn save-btn" 
                    on:click={on_manual_save}
                    title="Save your current game progress to browser"
                >
                    <span class="btn-icon">"💾"</span>
                    <span>"保存游戏"</span>
                </button>
                <button 
                    class="action-btn load-btn" 
                    on:click={on_manual_load}
                    title="Load saved game from browser"
                >
                    <span class="btn-icon">"📂"</span>
                    <span>"加载游戏"</span>
                </button>
                <button 
                    class="action-btn" 
                    on:click={on_new_game_click}
                    title="Start a new game (warning: will overwrite current game)"
                >
                    <span class="btn-icon">"⚙"</span>
                    <span>"新游戏"</span>
                </button>
            </div>
        </div>

        // Game Configuration Modal
        {move || {
            if is_modal_open.get() {
                view! {
                    <GameConfigModal
                        on_close={on_modal_close}
                        on_start={on_game_start}
                        has_existing_game={has_existing_game.get()}
                    />
                }
            } else {
                view! { <></> }
            }
        }}

        // Travel Animation Overlay
        <TravelAnimation
            is_active={is_traveling.get()}
            origin_name={travel_origin_name.get()}
            destination_name={travel_destination_name.get()}
            on_complete={Box::new(on_travel_animation_complete)}
        />
    }
}

/// Helper function to create sample solar system
fn create_sample_solar_system() -> SolarSystem {
    SolarSystem::new(
        "Sol System".to_string(),
        vec![
            Planet::new(
                "earth".to_string(),
                "Earth".to_string(),
                5,
                10,
                PlanetType::Agricultural,
            ),
            Planet::new(
                "mars".to_string(),
                "Mars".to_string(),
                10,
                15,
                PlanetType::Mining,
            ),
            Planet::new(
                "jupiter".to_string(),
                "Jupiter".to_string(),
                18,
                20,
                PlanetType::Industrial,
            ),
            Planet::new(
                "venus".to_string(),
                "Venus".to_string(),
                7,
                8,
                PlanetType::MegaCity,
            ),
            Planet::new(
                "titan".to_string(),
                "Titan".to_string(),
                25,
                30,
                PlanetType::ResearchOutpost,
            ),
            Planet::new(
                "pirate_station".to_string(),
                "Pirate Station".to_string(),
                30,
                25,
                PlanetType::PirateSpaceStation,
            ),
            Planet::new(
                "frontier".to_string(),
                "Frontier Colony".to_string(),
                35,
                40,
                PlanetType::FrontierColony,
            ),
        ],
    )
}

/// Helper function to create map planets for UI
fn create_map_planets() -> Vec<MapPlanet> {
    vec![
        MapPlanet {
            id: "earth".to_string(),
            name: "Earth".to_string(),
            orbit_radius: 5,
            orbit_period: 10,
            position: crate::simulation::orbits::Position::start(),
            planet_type: PlanetType::Agricultural,
        },
        MapPlanet {
            id: "mars".to_string(),
            name: "Mars".to_string(),
            orbit_radius: 10,
            orbit_period: 15,
            position: crate::simulation::orbits::Position::new(5),
            planet_type: PlanetType::Mining,
        },
        MapPlanet {
            id: "jupiter".to_string(),
            name: "Jupiter".to_string(),
            orbit_radius: 18,
            orbit_period: 20,
            position: crate::simulation::orbits::Position::new(10),
            planet_type: PlanetType::Industrial,
        },
        MapPlanet {
            id: "venus".to_string(),
            name: "Venus".to_string(),
            orbit_radius: 7,
            orbit_period: 8,
            position: crate::simulation::orbits::Position::new(3),
            planet_type: PlanetType::MegaCity,
        },
        MapPlanet {
            id: "titan".to_string(),
            name: "Titan".to_string(),
            orbit_radius: 25,
            orbit_period: 30,
            position: crate::simulation::orbits::Position::new(15),
            planet_type: PlanetType::ResearchOutpost,
        },
        MapPlanet {
            id: "pirate_station".to_string(),
            name: "Pirate Station".to_string(),
            orbit_radius: 30,
            orbit_period: 25,
            position: crate::simulation::orbits::Position::new(20),
            planet_type: PlanetType::PirateSpaceStation,
        },
        MapPlanet {
            id: "frontier".to_string(),
            name: "Frontier Colony".to_string(),
            orbit_radius: 35,
            orbit_period: 40,
            position: crate::simulation::orbits::Position::new(25),
            planet_type: PlanetType::FrontierColony,
        },
    ]
}

/// Helper function to get planet color as CSS string
fn get_planet_color_for_css(planet_type: &PlanetType) -> String {
    match planet_type {
        PlanetType::Agricultural => "#4CAF50".to_string(),
        PlanetType::MegaCity => "#9C27B0".to_string(),
        PlanetType::Mining => "#FF9800".to_string(),
        PlanetType::PirateSpaceStation => "#F44336".to_string(),
        PlanetType::ResearchOutpost => "#2196F3".to_string(),
        PlanetType::Industrial => "#607D8B".to_string(),
        PlanetType::FrontierColony => "#795548".to_string(),
    }
}

/// Calculate orbital position at a given turn
fn calculate_position_at_turn(orbit_period: u32, turn: u32) -> String {
    if orbit_period == 0 {
        return "0".to_string();
    }
    let position = turn % orbit_period;
    format!("{}/{}", position, orbit_period)
}

/// Main entry point for the web application
fn main() {
    // Set up panic hook for better error reporting in browser console
    console_error_panic_hook::set_once();

    // Mount the application
    leptos::mount::mount_to_body(|| {
        view! {
            <App />
        }
    });
}
//! Travel Selection Panel Component
//!
//! Displays travel costs and allows player to confirm travel to selected planet.

#[cfg(feature = "web")]
use leptos::view;
#[cfg(feature = "web")]
use leptos::IntoView;
#[cfg(feature = "web")]
use leptos::component;
#[cfg(feature = "web")]
use leptos::create_signal;
#[cfg(feature = "web")]
use leptos::Effect;
#[cfg(feature = "web")]
use leptos::SignalUpdate;
#[cfg(feature = "web")]
use leptos::SignalGet;
#[cfg(feature = "web")]
use crate::game_state::Planet;

/// Travel cost information
#[derive(Clone, Debug)]
pub struct TravelCost {
    pub fuel_required: u32,
    pub turns_required: u32,
    pub can_afford: bool,
}

/// Calculate travel time using Brachistochrone model
/// Formula: travel_turns = 2 * sqrt(base_distance / acceleration)
fn calculate_travel_turns_internal(
    origin_orbit_radius: u32,
    dest_orbit_radius: u32,
    ship_acceleration: u32,
) -> u32 {
    let base_distance = origin_orbit_radius.abs_diff(dest_orbit_radius);
    
    if base_distance == 0 {
        return 1;
    }

    let travel_turns = 2.0 * (base_distance as f64 / ship_acceleration.max(1) as f64).sqrt();
    std::cmp::max(travel_turns.ceil() as u32, 1)
}

/// Calculate travel cost between two planets
#[cfg(feature = "web")]
pub fn calculate_travel_cost(
    origin_planet: &Planet,
    destination_planet: &Planet,
    ship_acceleration: u32,
    player_fuel: u32,
) -> TravelCost {
    let turns = calculate_travel_turns_internal(
        origin_planet.orbit_radius,
        destination_planet.orbit_radius,
        ship_acceleration,
    );

    // Fuel calculation: 1 fuel per unit of orbital distance
    let distance = origin_planet.orbit_radius.abs_diff(destination_planet.orbit_radius);
    let fuel_required = distance.max(1); // At least 1 fuel for any travel

    TravelCost {
        fuel_required,
        turns_required: turns,
        can_afford: player_fuel >= fuel_required,
    }
}

/// Travel Panel Component
#[cfg(feature = "web")]
#[component]
pub fn TravelPanel(
    origin_planet: Option<Planet>,
    destination_planet: Option<Planet>,
    player_fuel: u32,
    ship_acceleration: u32,
    current_turn: u32,
    total_turns: u32,
    on_travel_confirm: Box<dyn Fn()>,
    on_cancel: Box<dyn Fn()>,
) -> impl IntoView {
    // Create signals to trigger callbacks
    let (confirm_trigger, set_confirm_trigger) = create_signal(0u32);
    let (cancel_trigger, set_cancel_trigger) = create_signal(0u32);

    // Effect to handle confirm trigger
    let on_travel_confirm = on_travel_confirm;
    Effect::new(move |_| {
        let _ = confirm_trigger.get();
        on_travel_confirm();
    });

    // Effect to handle cancel trigger
    let on_cancel = on_cancel;
    Effect::new(move |_| {
        let _ = cancel_trigger.get();
        on_cancel();
    });

    // Clone values before creating closures
    let origin_for_travel = origin_planet.clone();
    let dest_for_travel = destination_planet.clone();
    
    // Calculate travel cost
    let travel_cost = move || {
        match (&origin_for_travel, &dest_for_travel) {
            (Some(origin), Some(dest)) => {
                Some(calculate_travel_cost(origin, dest, ship_acceleration, player_fuel))
            }
            _ => None,
        }
    };

    // Clone values for is_already_there
    let origin_for_there = origin_planet.clone();
    let dest_for_there = destination_planet.clone();
    
    // Check if we're already at the destination
    let is_already_there = move || {
        match (&origin_for_there, &dest_for_there) {
            (Some(origin), Some(dest)) => origin.id == dest.id,
            _ => false,
        }
    };

    // Clone for destination name
    let dest_for_name = destination_planet.clone();
    // Get destination name
    let destination_name = move || {
        dest_for_name.as_ref().map(|p| p.name.clone()).unwrap_or_default()
    };

    // Clone for destination type
    let dest_for_type = destination_planet.clone();
    // Get destination type display
    let destination_type = move || {
        dest_for_type.as_ref()
            .map(|p| p.planet_type.display_name())
            .unwrap_or_default()
    };

    // Button click handlers - set the trigger signals
    let on_confirm_click = move |_| {
        set_confirm_trigger.update(|v| *v += 1);
    };

    let on_cancel_click = move |_| {
        set_cancel_trigger.update(|v| *v += 1);
    };

    view! {
        <div class="panel travel-panel">
            <div class="panel-header">
                <h3>"航行选择" </h3>
                <span class="panel-subtitle">"Travel Selection"</span>
            </div>
            <div class="panel-content">
                {move || {
                    if is_already_there() {
                        view! {
                            <div class="travel-already-there">
                                <span class="travel-icon">"🚀"</span>
                                <p>"您已在此行星"</p>
                                <p class="hint">"You are already here"</p>
                            </div>
                        }
                    } else {
                        match travel_cost() {
                            Some(cost) => {
                                view! {
                                    <div class="travel-info">
                                        <div class="travel-destination">
                                            <span class="travel-icon">"🎯"</span>
                                            <div class="destination-details">
                                                <div class="destination-name">{destination_name()}</div>
                                                <div class="destination-type">{destination_type()}</div>
                                            </div>
                                        </div>

                                        <div class="travel-costs">
                                            <div class="cost-row">
                                                <span class="cost-label">"⏱ 航行时间 Travel Time:"</span>
                                                <span class="cost-value">{cost.turns_required} 回合 turns</span>
                                            </div>
                                            <div class="cost-row">
                                                <span class="cost-label">"⛽ 所需燃料 Fuel Required:"</span>
                                                <span class={if cost.can_afford { "cost-value fuel-ok" } else { "cost-value fuel-low" }}>
                                                    {cost.fuel_required}
                                                </span>
                                            </div>
                                            <div class="cost-row">
                                                <span class="cost-label">"🔋 当前燃料 Current Fuel:"</span>
                                                <span class="cost-value">{player_fuel}</span>
                                            </div>
                                        </div>

                                        <div class="fuel-indicator">
                                            <div class="fuel-bar">
                                                <div
                                                    class="fuel-fill"
                                                    style={format!(
                                                        "width: {}%; background: {}",
                                                        (player_fuel as f64 / cost.fuel_required.max(1) as f64 * 100.0).min(100.0),
                                                        if cost.can_afford { "var(--accent-green)" } else { "var(--accent-red)" }
                                                    )}
                                                ></div>
                                            </div>
                                            <div class="fuel-warning" style={if cost.can_afford { "display: none;" } else { "" }}>
                                                <span>"⚠ 燃料不足 Insufficient fuel!"</span>
                                            </div>
                                        </div>

                                        <div class="travel-turn-info">
                                            <span>"当前回合 Current Turn: "</span>
                                            <span class="turn-value">{current_turn}</span>
                                            <span>" / "</span>
                                            <span>{total_turns}</span>
                                            <span>" → "</span>
                                            <span class="turn-value">{current_turn + cost.turns_required}</span>
                                        </div>

                                        <div class="travel-actions">
                                            <button
                                                class="travel-btn confirm"
                                                disabled={!cost.can_afford}
                                                on:click={on_confirm_click}
                                            >
                                                <span class="btn-icon">"🚀"</span>
                                                <span>"确认航行 Confirm Travel"</span>
                                            </button>
                                            <button
                                                class="travel-btn cancel"
                                                on:click={on_cancel_click}
                                            >
                                                <span class="btn-icon">"✕"</span>
                                                <span>"取消 Cancel"</span>
                                            </button>
                                        </div>
                                    </div>
                                }
                            }
                            None => {
                                view! {
                                    <div class="travel-no-selection">
                                        <span class="travel-icon">"🛸"</span>
                                        <p>"选择目标行星以开始航行"</p>
                                        <p class="hint">"Select a destination planet to travel"</p>
                                    </div>
                                }
                            }
                        }
                    }
                }}
            </div>
        </div>
    }
}

/// Travel Animation Component
/// Shows a visual animation when traveling between planets
#[cfg(feature = "web")]
#[component]
pub fn TravelAnimation(
    is_active: bool,
    origin_name: String,
    destination_name: String,
    on_complete: Option<Box<dyn Fn()>>,
) -> impl IntoView {
    // Animation complete handler
    let on_animation_end = move |_| {
        if let Some(callback) = &on_complete {
            callback();
        }
    };

    view! {
        <div 
            class="travel-animation-overlay" 
            style={if is_active { "" } else { "display: none;" }}
            on:animationend={on_animation_end}
        >
            <div class="travel-animation-content">
                <div class="travel-ship">
                    <span class="ship-icon">"🚀"</span>
                </div>
                <div class="travel-route">
                    <span class="origin">{origin_name}</span>
                    <div class="route-line">
                        <div class="route-progress"></div>
                    </div>
                    <span class="destination">{destination_name}</span>
                </div>
                <div class="travel-status">
                    <span>"正在航行... Traveling..."</span>
                </div>
            </div>
        </div>
    }
}

#[cfg(all(test, feature = "web"))]
mod tests {
    use super::*;
    use crate::simulation::economy::PlanetEconomy;
    use crate::simulation::orbits::Position;
    use crate::simulation::planet_types::PlanetType;

    #[test]
    fn test_calculate_travel_cost() {
        let origin = Planet {
            id: "earth".to_string(),
            name: "Earth".to_string(),
            orbit_radius: 5,
            orbit_period: 10,
            position: Position::new(0),
            economy: PlanetEconomy::new(PlanetType::Agricultural),
            planet_type: PlanetType::Agricultural,
        };

        let dest = Planet {
            id: "mars".to_string(),
            name: "Mars".to_string(),
            orbit_radius: 12,
            orbit_period: 15,
            position: Position::new(7),
            economy: PlanetEconomy::new(PlanetType::Mining),
            planet_type: PlanetType::Mining,
        };

        // Distance = |12 - 5| = 7
        // Travel time = 2 * sqrt(7/1) = 5.29... → 6 turns
        let cost = calculate_travel_cost(&origin, &dest, 1, 10);
        
        assert_eq!(cost.fuel_required, 7);
        assert_eq!(cost.turns_required, 6);
        assert!(cost.can_afford); // 10 >= 7
    }

    #[test]
    fn test_calculate_travel_cost_insufficient_fuel() {
        let origin = Planet {
            id: "earth".to_string(),
            name: "Earth".to_string(),
            orbit_radius: 5,
            orbit_period: 10,
            position: Position::new(0),
            economy: PlanetEconomy::new(PlanetType::Agricultural),
            planet_type: PlanetType::Agricultural,
        };

        let dest = Planet {
            id: "mars".to_string(),
            name: "Mars".to_string(),
            orbit_radius: 12,
            orbit_period: 15,
            position: Position::new(7),
            economy: PlanetEconomy::new(PlanetType::Mining),
            planet_type: PlanetType::Mining,
        };

        // Distance = |12 - 5| = 7, need 7 fuel but only have 5
        let cost = calculate_travel_cost(&origin, &dest, 1, 5);
        
        assert!(!cost.can_afford);
    }
}
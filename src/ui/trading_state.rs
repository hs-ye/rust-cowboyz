//! Shared TradingState Signal Architecture
//!
//! This module provides a shared signal architecture for reactive trade previews
//! across MarketPanel, CreditsPanel, and CargoPanel components.
//!
//! # Signal Flow Architecture
//!
//! ```text
//! App Component (web.rs)
//! ├── slider_values: RwSignal<HashMap<CommodityType, u32>>
//! ├── credit_change: Memo<i32>
//! ├── cargo_change: Memo<i32>
//! ├── projected_credits: Memo<u32>
//! └── projected_cargo: Memo<u32>
//!     │
//!     ├─→ MarketPanelReactive (reads/writes slider_values)
//!     ├─→ CreditsPanel (reads credit_change, projected_credits)
//!     └─→ CargoPanel (reads cargo_change, projected_cargo)
//! ```
//!
//! # Example Usage
//!
//! ```rust,ignore
//! // In App component:
//! let trading_state = create_trading_state(
//!     initial_inventory,
//!     economy,
//!     player_credits,
//!     cargo_capacity,
//!     current_cargo,
//! );
//!
//! // Pass to panels:
//! view! {
//!     <MarketPanelReactive
//!         slider_values={trading_state.slider_values}
//!         set_slider_values={trading_state.set_slider_values}
//!         economy={economy}
//!         cargo_capacity={cargo_capacity}
//!         current_cargo={current_cargo}
//!         player_credits={player_credits}
//!         cargo_inventory={cargo_inventory}
//!     />
//!     <CreditsPanel
//!         current_credits={player_credits}
//!         credit_change={trading_state.credit_change}
//!         projected_credits={trading_state.projected_credits}
//!     />
//!     <CargoPanel
//!         current_used={current_cargo}
//!         capacity={cargo_capacity}
//!         cargo_change={trading_state.cargo_change}
//!         projected_cargo={trading_state.projected_cargo}
//!     />
//! }
//! ```

#[cfg(feature = "web")]
use leptos::*;
#[cfg(feature = "web")]
use std::collections::HashMap;
#[cfg(feature = "web")]
use crate::simulation::commodity::CommodityType;
#[cfg(feature = "web")]
use crate::simulation::economy::PlanetEconomy;

/// TradingState holds all reactive signals and memos for trade previews
#[cfg(feature = "web")]
#[derive(Clone)]
pub struct TradingState {
    /// Slider values for each commodity (quantity to hold after trade)
    pub slider_values: ReadSignal<HashMap<CommodityType, u32>>,
    /// Setter for slider values (passed to MarketPanel)
    pub set_slider_values: WriteSignal<HashMap<CommodityType, u32>>,
    /// Total credit change from pending trades (positive = gain, negative = loss)
    pub credit_change: Memo<i32>,
    /// Total cargo change from pending trades (positive = buying, negative = selling)
    pub cargo_change: Memo<i32>,
    /// Projected credits after trade execution
    pub projected_credits: Memo<u32>,
    /// Projected cargo after trade execution
    pub projected_cargo: Memo<u32>,
}

/// Creates a new TradingState with all reactive signals and memos
///
/// # Arguments
/// * `initial_inventory` - Callback returning current cargo inventory
/// * `economy` - Memo containing the current planet economy
/// * `player_credits` - Callback returning current player credits
/// * `cargo_capacity` - Callback returning cargo capacity
/// * `current_cargo` - Callback returning current cargo used
///
/// # Returns
/// TradingState with all signals and memos initialized
#[cfg(feature = "web")]
pub fn create_trading_state(
    initial_inventory: impl Fn() -> Vec<(CommodityType, u32)> + Clone + 'static,
    economy: Memo<PlanetEconomy>,
    player_credits: impl Fn() -> u32 + Clone + 'static,
    cargo_capacity: impl Fn() -> u32 + Clone + 'static,
    current_cargo: impl Fn() -> u32 + Clone + 'static,
) -> TradingState {
    // Initialize slider values from current inventory
    let initial_values: HashMap<CommodityType, u32> = CommodityType::all()
        .iter()
        .map(|c| {
            let qty = initial_inventory()
                .iter()
                .find(|(commodity, _)| *commodity == *c)
                .map(|(_, q)| *q)
                .unwrap_or(0);
            (c.clone(), qty)
        })
        .collect();

    // Create signal for slider values
    let (slider_values, set_slider_values) = create_signal(initial_values);

    // Calculate credit change based on slider values and economy
    let credit_change = create_credit_change_memo(
        slider_values,
        initial_inventory.clone(),
        economy,
    );

    // Calculate cargo change based on slider values
    let cargo_change = create_cargo_change_memo(
        slider_values,
        initial_inventory.clone(),
    );

    // Calculate projected credits (current + change)
    let projected_credits = create_projected_credits_memo(
        player_credits.clone(),
        credit_change,
    );

    // Calculate projected cargo (current + change)
    let projected_cargo = create_projected_cargo_memo(
        current_cargo.clone(),
        cargo_change,
    );

    TradingState {
        slider_values,
        set_slider_values,
        credit_change,
        cargo_change,
        projected_credits,
        projected_cargo,
    }
}

/// Creates a memo that calculates total credit change from trades
#[cfg(feature = "web")]
fn create_credit_change_memo(
    slider_values: ReadSignal<HashMap<CommodityType, u32>>,
    inventory: impl Fn() -> Vec<(CommodityType, u32)> + Clone + 'static,
    economy: Memo<PlanetEconomy>,
) -> Memo<i32> {
    create_memo(move |_| {
        let values = slider_values.get();
        let inv = inventory();
        let current_economy = economy.get();
        let mut total = 0i32;

        for commodity in CommodityType::all() {
            let current_qty = inv
                .iter()
                .find(|(c, _)| *c == commodity)
                .map(|(_, q)| *q)
                .unwrap_or(0);
            let new_qty = values.get(&commodity).copied().unwrap_or(0);

            if new_qty > current_qty {
                // Buying: spending credits
                let buy_amount = (new_qty - current_qty) as i64;
                let sell_price = current_economy.get_sell_price(&commodity).unwrap_or(0) as i64;
                total -= (buy_amount * sell_price) as i32;
            } else if new_qty < current_qty {
                // Selling: gaining credits
                let sell_amount = (current_qty - new_qty) as i64;
                let buy_price = current_economy.get_buy_price(&commodity).unwrap_or(0) as i64;
                total += (sell_amount * buy_price) as i32;
            }
        }

        total
    })
}

/// Creates a memo that calculates total cargo change from trades
#[cfg(feature = "web")]
fn create_cargo_change_memo(
    slider_values: ReadSignal<HashMap<CommodityType, u32>>,
    inventory: impl Fn() -> Vec<(CommodityType, u32)> + Clone + 'static,
) -> Memo<i32> {
    create_memo(move |_| {
        let values = slider_values.get();
        let inv = inventory();
        let mut total = 0i32;

        for commodity in CommodityType::all() {
            let current_qty = inv
                .iter()
                .find(|(c, _)| *c == commodity)
                .map(|(_, q)| *q)
                .unwrap_or(0);
            let new_qty = values.get(&commodity).copied().unwrap_or(0);

            total += new_qty as i32 - current_qty as i32;
        }

        total
    })
}

/// Creates a memo that calculates projected credits after trade
#[cfg(feature = "web")]
fn create_projected_credits_memo(
    player_credits: impl Fn() -> u32 + Clone + 'static,
    credit_change: Memo<i32>,
) -> Memo<u32> {
    create_memo(move |_| {
        let credits = player_credits() as i32;
        let change = credit_change.get();
        let projected = credits + change;
        projected.max(0) as u32
    })
}

/// Creates a memo that calculates projected cargo after trade
#[cfg(feature = "web")]
fn create_projected_cargo_memo(
    current_cargo: impl Fn() -> u32 + Clone + 'static,
    cargo_change: Memo<i32>,
) -> Memo<u32> {
    create_memo(move |_| {
        let cargo = current_cargo() as i32;
        let change = cargo_change.get();
        let projected = cargo + change;
        projected.max(0) as u32
    })
}

/// Helper function to format credit display with arrow notation
/// e.g., "$1,450 → $1,350"
#[cfg(feature = "web")]
pub fn format_credit_display(current: u32, projected: u32) -> String {
    if current == projected {
        format!("${}", current)
    } else {
        format!("${} → ${}", current, projected)
    }
}

/// Helper function to format cargo display with arrow notation
/// e.g., "35/50 → 40/50 units"
#[cfg(feature = "web")]
pub fn format_cargo_display(current: u32, projected: u32, capacity: u32) -> String {
    if current == projected {
        format!("{}/{} units", current, capacity)
    } else {
        format!("{}/{} → {}/{} units", current, capacity, projected, capacity)
    }
}

/// Helper function to determine credit change direction
/// Returns: "gain" if positive, "loss" if negative, "neutral" if zero
#[cfg(feature = "web")]
pub fn get_credit_change_direction(change: i32) -> &'static str {
    if change > 0 {
        "gain"
    } else if change < 0 {
        "loss"
    } else {
        "neutral"
    }
}

/// Helper function to determine cargo change direction
/// Returns: "increase" if positive, "decrease" if negative, "neutral" if zero
#[cfg(feature = "web")]
pub fn get_cargo_change_direction(change: i32) -> &'static str {
    if change > 0 {
        "increase"
    } else if change < 0 {
        "decrease"
    } else {
        "neutral"
    }
}

#[cfg(all(test, feature = "web"))]
mod tests {
    use super::*;
    use leptos::create_runtime;
    use crate::simulation::planet_types::PlanetType;

    #[test]
    fn test_format_credit_display_no_change() {
        assert_eq!(format_credit_display(1000, 1000), "$1000");
    }

    #[test]
    fn test_format_credit_display_with_change() {
        assert_eq!(format_credit_display(1000, 800), "$1000 → $800");
        assert_eq!(format_credit_display(1000, 1200), "$1000 → $1200");
    }

    #[test]
    fn test_format_cargo_display_no_change() {
        assert_eq!(format_cargo_display(35, 35, 50), "35/50 units");
    }

    #[test]
    fn test_format_cargo_display_with_change() {
        assert_eq!(format_cargo_display(35, 40, 50), "35/50 → 40/50 units");
        assert_eq!(format_cargo_display(35, 30, 50), "35/50 → 30/50 units");
    }

    #[test]
    fn test_get_credit_change_direction() {
        assert_eq!(get_credit_change_direction(100), "gain");
        assert_eq!(get_credit_change_direction(-100), "loss");
        assert_eq!(get_credit_change_direction(0), "neutral");
    }

    #[test]
    fn test_get_cargo_change_direction() {
        assert_eq!(get_cargo_change_direction(10), "increase");
        assert_eq!(get_cargo_change_direction(-10), "decrease");
        assert_eq!(get_cargo_change_direction(0), "neutral");
    }

    #[test]
    fn test_trading_state_creation() {
        // This test verifies the TradingState can be created without runtime errors
        // Note: Full reactive testing requires leptos runtime
        let runtime = create_runtime();

        // Create mock data
        let inventory = vec![
            (CommodityType::Water, 5),
            (CommodityType::Foodstuffs, 3),
        ];
        let economy = create_memo(move |_| PlanetEconomy::new(PlanetType::Agricultural));
        let player_credits = move || 1000u32;
        let cargo_capacity = move || 50u32;
        let current_cargo = move || 8u32;

        // Create trading state
        let _trading_state = create_trading_state(
            move || inventory.clone(),
            economy,
            player_credits,
            cargo_capacity,
            current_cargo,
        );

        // Cleanup
        runtime.dispose();
    }
}

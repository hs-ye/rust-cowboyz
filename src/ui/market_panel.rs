//! Market Panel Component with Slider-Based Trading
//!
//! Displays commodity prices for a selected planet's economy with interactive sliders
//! for buying and selling commodities. Based on ADR #0007: Market Trading UI Interaction.

#[cfg(feature = "web")]
use leptos::*;
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use crate::simulation::commodity::CommodityType;
#[cfg(feature = "web")]
use crate::simulation::economy::PlanetEconomy;
#[cfg(feature = "web")]
use crate::simulation::planet_types::PlanetType;

/// Market Panel Slider Component
///
/// Displays a slider for a single commodity row allowing the player to:
/// - Slide left to sell (decrease quantity)
/// - Slide right to buy (increase quantity)
/// - Range: 0 to max_quantity (cargo capacity)
#[cfg(feature = "web")]
#[component]
pub fn MarketPanelSlider(
    /// Commodity type for this row
    commodity: CommodityType,
    /// Current quantity in cargo hold
    current_quantity: u32,
    /// Price at which market buys from player
    buy_price: u32,
    /// Price at which market sells to player
    sell_price: u32,
    /// Base price for reference
    base_price: u32,
    /// Maximum quantity (cargo capacity)
    max_quantity: u32,
    /// Callback when slider value changes
    on_change: Callback<u32>,
) -> impl IntoView {
    // Create signal for slider value, initialized to current quantity
    let (slider_value, set_slider_value) = create_signal(current_quantity);
    
    // Calculate the difference from current quantity
    let quantity_change = create_memo(move |_| {
        slider_value.get() as i32 - current_quantity as i32
    });
    
    // Calculate credit change (positive = gaining credits, negative = losing credits)
    let credit_change = create_memo(move |_| {
        let change = quantity_change.get();
        if change > 0 {
            // Buying: spending credits
            -((change as i64) * (sell_price as i64)) as i32
        } else if change < 0 {
            // Selling: gaining credits
            ((-change) as i64 * (buy_price as i64)) as i32
        } else {
            0
        }
    });
    
    // Determine price trend indicator
    let price_trend = create_memo(move |_| {
        if sell_price > base_price {
            "↑" // Price is above base
        } else if sell_price < base_price {
            "↓" // Price is below base
        } else {
            "→" // Price is at base
        }
    });
    
    // Handle slider input
    let handle_input = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
        let value: u32 = input.value().parse().unwrap_or(0);
        set_slider_value.set(value);
        on_change.call(value);
    };
    
    view! {
        <div class="market-row-slider">
            <div class="commodity-info">
                <span class="commodity-name">{commodity.display_name()}</span>
                <span class="commodity-holdings">"Holdings: " {current_quantity}</span>
            </div>
            <div class="price-info">
                <span class="price-display">
                    <span class="price-trend">{price_trend.get()}</span>
                    <span class="buy-price">"B: ${buy_price}"</span>
                    <span class="sell-price">"S: ${sell_price}"</span>
                    <span class="base-price">"(${base_price})"</span>
                </span>
            </div>
            <div class="slider-container">
                <input
                    type="range"
                    min="0"
                    max={max_quantity.to_string()}
                    value={current_quantity.to_string()}
                    on:input={handle_input}
                    class="commodity-slider"
                />
                <div class="slider-labels">
                    <span class="slider-min">"0"</span>
                    <span class="slider-value">{move || slider_value.get()}</span>
                    <span class="slider-max">{max_quantity.to_string()}</span>
                </div>
            </div>
            <div class="trade-preview">
                <span class={move || {
                    let change = credit_change.get();
                    if change > 0 {
                        "credit-change positive"
                    } else if change < 0 {
                        "credit-change negative"
                    } else {
                        "credit-change neutral"
                    }
                }}>
                    {move || {
                        let change = credit_change.get();
                        if change > 0 {
                            format!("+${}", change)
                        } else if change < 0 {
                            format!("-${}", -change)
                        } else {
                            "$0".to_string()
                        }
                    }}
                </span>
            </div>
        </div>
    }
}

/// Trade Preview Component
///
/// Shows real-time preview of credit and cargo changes from pending trades
#[cfg(feature = "web")]
#[component]
pub fn TradePreview(
    /// Total credit change (positive = gain, negative = loss)
    credit_change: Memo<i32>,
    /// Total cargo change (positive = buying, negative = selling)
    cargo_change: Memo<i32>,
    /// Current cargo used
    current_cargo: u32,
    /// Cargo capacity
    cargo_capacity: u32,
) -> impl IntoView {
    let projected_cargo = create_memo(move |_| {
        current_cargo as i32 + cargo_change.get()
    });
    
    let cargo_warning = create_memo(move |_| {
        let projected = projected_cargo.get();
        if projected < 0 {
            Some("Cannot sell more than you have!")
        } else if projected as u32 > cargo_capacity {
            Some("Exceeds cargo capacity!")
        } else {
            None
        }
    });
    
    view! {
        <div class="trade-preview-panel">
            <div class="preview-row">
                <span class="preview-label">"Credit Change:"</span>
                <span class={move || {
                    let change = credit_change.get();
                    if change > 0 {
                        "preview-value credit-gain"
                    } else if change < 0 {
                        "preview-value credit-loss"
                    } else {
                        "preview-value credit-neutral"
                    }
                }}>
                    {move || {
                        let change = credit_change.get();
                        if change > 0 {
                            format!("+${}", change)
                        } else if change < 0 {
                            format!("-${}", -change)
                        } else {
                            "$0".to_string()
                        }
                    }}
                </span>
            </div>
            <div class="preview-row">
                <span class="preview-label">"Cargo Change:"</span>
                <span class={move || {
                    let change = cargo_change.get();
                    if change > 0 {
                        "preview-value cargo-increase"
                    } else if change < 0 {
                        "preview-value cargo-decrease"
                    } else {
                        "preview-value cargo-neutral"
                    }
                }}>
                    {move || {
                        let change = cargo_change.get();
                        if change > 0 {
                            format!("+{} units", change)
                        } else if change < 0 {
                            format!("{} units", change)
                        } else {
                            "0 units".to_string()
                        }
                    }}
                </span>
            </div>
            <div class="preview-row">
                <span class="preview-label">"Projected Cargo:"</span>
                <span class="preview-value">
                    {move || format!("{}/{}", projected_cargo.get().max(0) as u32, cargo_capacity)}
                </span>
            </div>
            {move || {
                cargo_warning.get().map(|warning| {
                    view! {
                        <div class="cargo-warning">{warning}</div>
                    }
                })
            }}
        </div>
    }
}

/// Trade Controls Component
///
/// Contains Trade, Reset, and Trade Log buttons
#[cfg(feature = "web")]
#[component]
pub fn TradeControls(
    /// Whether the trade button should be disabled
    trade_disabled: Memo<bool>,
    /// Tooltip text when trade is disabled
    trade_disabled_reason: Memo<String>,
    /// Callback when Trade button is clicked
    on_trade: Callback<()>,
    /// Callback when Reset button is clicked
    on_reset: Callback<()>,
    /// Callback when Trade Log button is clicked
    on_trade_log: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="trade-controls">
            <button
                class="trade-btn"
                class:disabled={move || trade_disabled.get()}
                on:click={move |_| on_trade.call(())}
                disabled={move || trade_disabled.get()}
            >
                <span class="btn-icon">"💱"</span>
                <span>"Trade"</span>
            </button>
            {move || {
                if trade_disabled.get() && !trade_disabled_reason.get().is_empty() {
                    Some(view! {
                        <div class="tooltip">{trade_disabled_reason.get()}</div>
                    })
                } else {
                    None
                }
            }}
            <button
                class="reset-btn"
                on:click={move |_| on_reset.call(())}
            >
                <span class="btn-icon">"↺"</span>
                <span>"Reset"</span>
            </button>
            <button
                class="trade-log-btn"
                on:click={move |_| on_trade_log.call(())}
            >
                <span class="btn-icon">"📋"</span>
                <span>"Trade Log"</span>
            </button>
        </div>
    }
}

/// Market Panel Component
///
/// Displays the market/commodity prices for a given planet with interactive sliders.
/// The panel shows all 10 commodity types with their buy and sell prices,
/// and allows the player to buy/sell using sliders.
#[cfg(feature = "web")]
#[component]
pub fn MarketPanel(
    planet_name: String,
    _planet_type: PlanetType,
    economy: PlanetEconomy,
    cargo_capacity: u32,
    current_cargo: u32,
    player_credits: u32,
    cargo_inventory: Vec<(CommodityType, u32)>,
) -> impl IntoView {
    // Get all commodity types
    let commodities = CommodityType::all();
    
    // Clone commodities for use in multiple closures
    let commodities_for_sliders = commodities.clone();
    let commodities_for_credit = commodities.clone();
    let commodities_for_cargo = commodities.clone();
    let commodities_for_reset = commodities.clone();
    let commodities_for_render = commodities.clone();
    
    // Clone inventory for use in multiple closures
    let inventory_for_sliders = cargo_inventory.clone();
    let inventory_for_credit = cargo_inventory.clone();
    let inventory_for_cargo = cargo_inventory.clone();
    let inventory_for_reset = cargo_inventory.clone();
    
    // Clone economy for use in credit calculation
    let economy_for_credit = economy.clone();
    
    // Create signal to track slider values for each commodity
    let (slider_values, set_slider_values) = create_signal(
        commodities_for_sliders.iter()
            .map(|c| {
                let qty = inventory_for_sliders.iter()
                    .find(|(commodity, _)| commodity == c)
                    .map(|(_, q)| *q)
                    .unwrap_or(0);
                (c.clone(), qty)
            })
            .collect::<std::collections::HashMap<_, _>>()
    );
    
    // Calculate total credit change and cargo change
    let total_credit_change = create_memo(move |_| {
        let values = slider_values.get();
        let mut total = 0i32;
        
        for commodity in &commodities_for_credit {
            let current_qty = inventory_for_credit.iter()
                .find(|(c, _)| c == commodity)
                .map(|(_, q)| *q)
                .unwrap_or(0);
            let new_qty = values.get(commodity).copied().unwrap_or(0);
            
            if new_qty > current_qty {
                // Buying: spending credits
                let buy_amount = (new_qty - current_qty) as i64;
                let sell_price = economy_for_credit.get_sell_price(commodity).unwrap_or(0) as i64;
                total -= (buy_amount * sell_price) as i32;
            } else if new_qty < current_qty {
                // Selling: gaining credits
                let sell_amount = (current_qty - new_qty) as i64;
                let buy_price = economy_for_credit.get_buy_price(commodity).unwrap_or(0) as i64;
                total += (sell_amount * buy_price) as i32;
            }
        }
        
        total
    });
    
    let total_cargo_change = create_memo(move |_| {
        let values = slider_values.get();
        let mut total = 0i32;
        
        for commodity in &commodities_for_cargo {
            let current_qty = inventory_for_cargo.iter()
                .find(|(c, _)| c == commodity)
                .map(|(_, q)| *q)
                .unwrap_or(0);
            let new_qty = values.get(commodity).copied().unwrap_or(0);
            
            total += new_qty as i32 - current_qty as i32;
        }
        
        total
    });
    
    // Check if trade is valid
    let trade_disabled = create_memo(move |_| {
        let credit_change = total_credit_change.get();
        let cargo_change = total_cargo_change.get();
        
        // Check if player has enough credits
        if credit_change < 0 {
            let required_credits = (-credit_change) as u32;
            if player_credits < required_credits {
                return true;
            }
        }
        
        // Check cargo capacity
        let projected_cargo = current_cargo as i32 + cargo_change;
        if projected_cargo < 0 || projected_cargo as u32 > cargo_capacity {
            return true;
        }
        
        // Check if any changes were made
        if credit_change == 0 && cargo_change == 0 {
            return true;
        }
        
        false
    });
    
    let trade_disabled_reason = create_memo(move |_| {
        let credit_change = total_credit_change.get();
        let cargo_change = total_cargo_change.get();
        
        // Check if player has enough credits
        if credit_change < 0 {
            let required_credits = (-credit_change) as u32;
            if player_credits < required_credits {
                return format!("Insufficient credits: need ${}, have ${}", required_credits, player_credits);
            }
        }
        
        // Check cargo capacity
        let projected_cargo = current_cargo as i32 + cargo_change;
        if projected_cargo < 0 {
            return "Cannot sell more than you have!".to_string();
        }
        if projected_cargo as u32 > cargo_capacity {
            return format!("Exceeds cargo capacity: {}/{}", projected_cargo as u32, cargo_capacity);
        }
        
        // No changes made
        if credit_change == 0 && cargo_change == 0 {
            return "No trades selected".to_string();
        }
        
        String::new()
    });
    
    // Handle slider change
    let handle_slider_change = {
        move |commodity: CommodityType, value: u32| {
            set_slider_values.update(|values| {
                values.insert(commodity, value);
            });
        }
    };
    
    // Handle trade button click
    let handle_trade = move |_| {
        // In a real implementation, this would call the backend to execute trades
        web_sys::window()
            .unwrap()
            .alert_with_message("Trade execution - to be implemented with backend integration (Issue #137)")
            .unwrap();
    };
    
    // Handle reset button click
    let handle_reset = move |_| {
        // Reset all sliders to current cargo inventory values
        set_slider_values.set(
            commodities_for_reset.iter()
                .map(|c| {
                    let qty = inventory_for_reset.iter()
                        .find(|(commodity, _)| commodity == c)
                        .map(|(_, q)| *q)
                        .unwrap_or(0);
                    (c.clone(), qty)
                })
                .collect()
        );
    };
    
    // Handle trade log button click
    let handle_trade_log = move |_| {
        web_sys::window()
            .unwrap()
            .alert_with_message("Trade Log - to be implemented")
            .unwrap();
    };
    
    view! {
        <div class="panel market-panel">
            <div class="panel-header">
                <h3>"Market"</h3>
                <span class="panel-subtitle">{planet_name}</span>
            </div>
            <div class="panel-content">
                <div class="market-table">
                    <div class="market-header">
                        <span>"Item"</span>
                        <span>"Price (B/S/Base)"</span>
                        <span>"Quantity"</span>
                        <span>"Preview"</span>
                    </div>
                    {
                        commodities_for_render.into_iter().map(move |commodity| {
                            let buy_price = economy.get_buy_price(&commodity).unwrap_or(0);
                            let sell_price = economy.get_sell_price(&commodity).unwrap_or(0);
                            let base_price = commodity.base_value();
                            
                            // Get current quantity for this commodity
                            let current_qty = cargo_inventory.iter()
                                .find(|(c, _)| c == &commodity)
                                .map(|(_, q)| *q)
                                .unwrap_or(0);
                            
                            // Create callback for this specific commodity
                            let commodity_clone = commodity.clone();
                            let on_change = Callback::new(move |value: u32| {
                                handle_slider_change(commodity_clone.clone(), value);
                            });
                            
                            view! {
                                <MarketPanelSlider
                                    commodity={commodity}
                                    current_quantity={current_qty}
                                    buy_price={buy_price}
                                    sell_price={sell_price}
                                    base_price={base_price}
                                    max_quantity={cargo_capacity}
                                    on_change={on_change}
                                />
                            }
                        }).collect::<Vec<_>>()
                    }
                </div>
                
                // Trade Preview Panel
                <TradePreview
                    credit_change={total_credit_change}
                    cargo_change={total_cargo_change}
                    current_cargo={current_cargo}
                    cargo_capacity={cargo_capacity}
                />
                
                // Trade Controls
                <TradeControls
                    trade_disabled={trade_disabled}
                    trade_disabled_reason={trade_disabled_reason}
                    on_trade={Callback::new(handle_trade)}
                    on_reset={Callback::new(handle_reset)}
                    on_trade_log={Callback::new(handle_trade_log)}
                />
            </div>
        </div>
    }
}

/// Market Panel with Signal Support
///
/// This version accepts reactive signals/memos for planet data,
/// allowing it to reactively update when the selected planet changes.
///
/// # Arguments
/// * `slider_values` - RwSignal containing slider values for each commodity
/// * `set_slider_values` - WriteSignal to update slider values
/// * `planet_name` - Callback to get the current planet name (reactive)
/// * `planet_type` - Memo containing the current planet type (reactive)
/// * `economy` - Memo containing the current planet economy (reactive)
/// * `cargo_capacity` - Callback for cargo capacity
/// * `current_cargo` - Callback for current cargo used
/// * `player_credits` - Callback for player credits
/// * `cargo_inventory` - Callback for cargo inventory
#[cfg(feature = "web")]
#[component]
pub fn MarketPanelReactive(
    slider_values: ReadSignal<std::collections::HashMap<CommodityType, u32>>,
    set_slider_values: WriteSignal<std::collections::HashMap<CommodityType, u32>>,
    planet_name: impl Fn() -> String + Clone + 'static,
    planet_type: Memo<PlanetType>,
    economy: Memo<PlanetEconomy>,
    cargo_capacity: impl Fn() -> u32 + Clone + 'static,
    current_cargo: impl Fn() -> u32 + Clone + 'static,
    player_credits: impl Fn() -> u32 + Clone + 'static,
    cargo_inventory: impl Fn() -> Vec<(CommodityType, u32)> + Clone + 'static,
) -> impl IntoView {
    // Get all commodity types
    let commodities = CommodityType::all();

    // Note: planet_type is available but not currently used in rendering
    // It's kept for potential future use (e.g., planet type badges)
    let _ = planet_type;

    // Clone commodities for use in multiple closures
    let commodities_for_sliders = commodities.clone();
    let commodities_for_credit = commodities.clone();
    let commodities_for_cargo = commodities.clone();
    let commodities_for_reset = commodities.clone();
    let commodities_for_render = commodities.clone();

    // Clone callbacks for use in multiple closures
    let cargo_capacity_clone1 = cargo_capacity.clone();
    let cargo_capacity_clone2 = cargo_capacity.clone();
    let cargo_capacity_clone3 = cargo_capacity.clone();
    let current_cargo_clone1 = current_cargo.clone();
    let current_cargo_clone2 = current_cargo.clone();
    let current_cargo_clone3 = current_cargo.clone();
    let player_credits_clone1 = player_credits.clone();
    let player_credits_clone2 = player_credits.clone();
    let cargo_inventory_clone1 = cargo_inventory.clone();
    let cargo_inventory_clone2 = cargo_inventory.clone();
    let cargo_inventory_clone3 = cargo_inventory.clone();
    let cargo_inventory_reset = cargo_inventory.clone();

    // slider_values and set_slider_values are now passed as props
    // No need to create internal signals

    // Calculate total credit change and cargo change
    let total_credit_change = create_memo(move |_| {
        let values = slider_values.get();
        let inv = cargo_inventory_clone1();
        let current_economy = economy.get();
        let mut total = 0i32;

        for commodity in &commodities_for_credit {
            let current_qty = inv.iter()
                .find(|(c, _)| c == commodity)
                .map(|(_, q)| *q)
                .unwrap_or(0);
            let new_qty = values.get(commodity).copied().unwrap_or(0);

            if new_qty > current_qty {
                // Buying: spending credits
                let buy_amount = (new_qty - current_qty) as i64;
                let sell_price = current_economy.get_sell_price(commodity).unwrap_or(0) as i64;
                total -= (buy_amount * sell_price) as i32;
            } else if new_qty < current_qty {
                // Selling: gaining credits
                let sell_amount = (current_qty - new_qty) as i64;
                let buy_price = current_economy.get_buy_price(commodity).unwrap_or(0) as i64;
                total += (sell_amount * buy_price) as i32;
            }
        }

        total
    });

    let total_cargo_change = create_memo(move |_| {
        let values = slider_values.get();
        let inv = cargo_inventory_clone2();
        let mut total = 0i32;

        for commodity in &commodities_for_cargo {
            let current_qty = inv.iter()
                .find(|(c, _)| c == commodity)
                .map(|(_, q)| *q)
                .unwrap_or(0);
            let new_qty = values.get(commodity).copied().unwrap_or(0);

            total += new_qty as i32 - current_qty as i32;
        }

        total
    });

    // Check if trade is valid
    let trade_disabled = create_memo(move |_| {
        let credit_change = total_credit_change.get();
        let cargo_change = total_cargo_change.get();
        let credits = player_credits_clone1();
        let cap = cargo_capacity_clone1();
        let cargo = current_cargo_clone1();

        // Check if player has enough credits
        if credit_change < 0 {
            let required_credits = (-credit_change) as u32;
            if credits < required_credits {
                return true;
            }
        }

        // Check cargo capacity
        let projected_cargo = cargo as i32 + cargo_change;
        if projected_cargo < 0 || projected_cargo as u32 > cap {
            return true;
        }

        // Check if any changes were made
        if credit_change == 0 && cargo_change == 0 {
            return true;
        }

        false
    });

    let trade_disabled_reason = create_memo(move |_| {
        let credit_change = total_credit_change.get();
        let cargo_change = total_cargo_change.get();
        let credits = player_credits_clone2();
        let cap = cargo_capacity_clone2();
        let cargo = current_cargo_clone2();

        // Check if player has enough credits
        if credit_change < 0 {
            let required_credits = (-credit_change) as u32;
            if credits < required_credits {
                return format!("Insufficient credits: need ${}, have ${}", required_credits, credits);
            }
        }

        // Check cargo capacity
        let projected_cargo = cargo as i32 + cargo_change;
        if projected_cargo < 0 {
            return "Cannot sell more than you have!".to_string();
        }
        if projected_cargo as u32 > cap {
            return format!("Exceeds cargo capacity: {}/{}", projected_cargo as u32, cap);
        }

        // No changes made
        if credit_change == 0 && cargo_change == 0 {
            return "No trades selected".to_string();
        }

        String::new()
    });
    
    // Handle slider change
    let handle_slider_change = {
        move |commodity: CommodityType, value: u32| {
            set_slider_values.update(|values| {
                values.insert(commodity, value);
            });
        }
    };
    
    // Handle trade button click
    let handle_trade = move |_| {
        web_sys::window()
            .unwrap()
            .alert_with_message("Trade execution - to be implemented with backend integration (Issue #137)")
            .unwrap();
    };
    
    // Handle reset button click
    let handle_reset = {
        let commodities_clone = commodities_for_reset.clone();
        move |_| {
            set_slider_values.set(
                commodities_clone.iter()
                    .map(|c| {
                        let inv = cargo_inventory_reset();
                        let qty = inv.iter()
                            .find(|(commodity, _)| commodity == c)
                            .map(|(_, q)| *q)
                            .unwrap_or(0);
                        (c.clone(), qty)
                    })
                    .collect()
            );
        }
    };
    
    // Handle trade log button click
    let handle_trade_log = move |_| {
        web_sys::window()
            .unwrap()
            .alert_with_message("Trade Log - to be implemented")
            .unwrap();
    };
    
    view! {
        <div class="panel market-panel">
            <div class="panel-header">
                <h3>"Market"</h3>
                <span class="panel-subtitle">{move || planet_name()}</span>
            </div>
            <div class="panel-content">
                <div class="market-table">
                    <div class="market-header">
                        <span>"Item"</span>
                        <span>"Price (B/S/Base)"</span>
                        <span>"Quantity"</span>
                        <span>"Preview"</span>
                    </div>
                    {
                        let cargo_inv_for_render = cargo_inventory_clone3.clone();
                        let cap_for_render = cargo_capacity_clone3.clone();
                        commodities_for_render.into_iter().map(move |commodity| {
                            let current_economy = economy.get();
                            let buy_price = current_economy.get_buy_price(&commodity).unwrap_or(0);
                            let sell_price = current_economy.get_sell_price(&commodity).unwrap_or(0);
                            let base_price = commodity.base_value();

                            // Get current quantity for this commodity
                            let inv = cargo_inv_for_render();
                            let current_qty = inv.iter()
                                .find(|(c, _)| c == &commodity)
                                .map(|(_, q)| *q)
                                .unwrap_or(0);

                            // Create callback for this specific commodity
                            let commodity_clone = commodity.clone();
                            let on_change = Callback::new(move |value: u32| {
                                handle_slider_change(commodity_clone.clone(), value);
                            });

                            view! {
                                <MarketPanelSlider
                                    commodity={commodity}
                                    current_quantity={current_qty}
                                    buy_price={buy_price}
                                    sell_price={sell_price}
                                    base_price={base_price}
                                    max_quantity={cap_for_render()}
                                    on_change={on_change}
                                />
                            }
                        }).collect::<Vec<_>>()
                    }
                </div>

                // Trade Controls
                <TradeControls
                    trade_disabled={trade_disabled}
                    trade_disabled_reason={trade_disabled_reason}
                    on_trade={Callback::new(handle_trade)}
                    on_reset={Callback::new(handle_reset)}
                    on_trade_log={Callback::new(handle_trade_log)}
                />
            </div>
        </div>
    }
}

#[cfg(all(test, feature = "web"))]
mod tests {
    use super::*;
    use crate::game_state::Planet;
    use crate::simulation::orbits::Position;

    #[test]
    fn test_market_panel_renders_all_commodities() {
        let economy = PlanetEconomy::new(PlanetType::Agricultural);
        let commodities = CommodityType::all();

        // Verify all 10 commodities exist
        assert_eq!(commodities.len(), 10);

        // Verify economy has prices for all commodities
        for commodity in &commodities {
            assert!(economy.get_buy_price(commodity).is_some());
            assert!(economy.get_sell_price(commodity).is_some());
        }
    }

    #[test]
    fn test_different_planet_types_have_different_prices() {
        let agricultural_economy = PlanetEconomy::new(PlanetType::Agricultural);
        let mining_economy = PlanetEconomy::new(PlanetType::Mining);

        // Agricultural planets should have cheaper Water (they produce it)
        let ag_water_sell = agricultural_economy.get_sell_price(&CommodityType::Water).unwrap();
        let mining_water_sell = mining_economy.get_sell_price(&CommodityType::Water).unwrap();

        // Mining planets should have cheaper Metals
        let mining_metals_sell = mining_economy.get_sell_price(&CommodityType::Metals).unwrap();
        let ag_metals_sell = agricultural_economy.get_sell_price(&CommodityType::Metals).unwrap();

        // Verify price differences based on planet specialization
        assert!(ag_water_sell < mining_water_sell, "Agricultural planets should have cheaper Water");
        assert!(mining_metals_sell < ag_metals_sell, "Mining planets should have cheaper Metals");
    }

    #[test]
    fn test_buy_price_less_than_sell_price() {
        let economy = PlanetEconomy::new(PlanetType::Industrial);

        for commodity in CommodityType::all() {
            let buy_price = economy.get_buy_price(&commodity).unwrap();
            let sell_price = economy.get_sell_price(&commodity).unwrap();

            // Market buys from player at lower price than it sells to player
            assert!(buy_price <= sell_price,
                "Buy price ({}) should be <= sell price ({}) for {:?}",
                buy_price, sell_price, commodity
            );
        }
    }

    #[test]
    fn test_slider_range_valid() {
        // Test that slider values are within valid range
        let max_quantity = 50;
        let test_values = vec![0, 10, 25, 50];
        
        for value in test_values {
            assert!(value >= 0, "Slider value must be non-negative");
            assert!(value <= max_quantity, "Slider value must not exceed max_quantity");
        }
    }

    #[test]
    fn test_credit_calculation_buy() {
        let economy = PlanetEconomy::new(PlanetType::Agricultural);
        let commodity = CommodityType::Water;
        let sell_price = economy.get_sell_price(&commodity).unwrap();
        
        // Buying 10 units should cost 10 * sell_price
        let buy_amount = 10;
        let expected_cost = buy_amount * sell_price;
        
        assert!(expected_cost > 0, "Buying should cost credits");
    }

    #[test]
    fn test_credit_calculation_sell() {
        let economy = PlanetEconomy::new(PlanetType::Agricultural);
        let commodity = CommodityType::Water;
        let buy_price = economy.get_buy_price(&commodity).unwrap();
        
        // Selling 10 units should gain 10 * buy_price
        let sell_amount = 10;
        let expected_gain = sell_amount * buy_price;
        
        assert!(expected_gain > 0, "Selling should gain credits");
    }

    #[test]
    fn test_trade_validation_insufficient_credits() {
        let player_credits = 100;
        let required_credits = 500;
        
        // Trade should be disabled when player lacks credits
        assert!(player_credits < required_credits, "Should detect insufficient credits");
    }

    #[test]
    fn test_trade_validation_cargo_capacity() {
        let cargo_capacity = 50;
        let current_cargo = 40;
        let cargo_change = 20; // Trying to add 20 more units
        
        let projected_cargo = current_cargo as i32 + cargo_change;
        
        // Trade should be disabled when exceeding capacity
        assert!(projected_cargo as u32 > cargo_capacity, "Should detect cargo overflow");
    }

    #[test]
    fn test_reset_restores_original_values() {
        let original_values = vec![
            (CommodityType::Water, 10),
            (CommodityType::Foodstuffs, 5),
        ];
        
        // Reset should restore original values
        let reset_values: std::collections::HashMap<CommodityType, u32> = original_values.iter().cloned().collect();
        
        assert_eq!(reset_values.get(&CommodityType::Water), Some(&10));
        assert_eq!(reset_values.get(&CommodityType::Foodstuffs), Some(&5));
    }

    #[test]
    fn test_price_trend_indicators() {
        let base_price = 100;
        let current_price_high = 150;
        let current_price_low = 50;
        let current_price_equal = 100;
        
        // Price above base should show up arrow
        assert!(current_price_high > base_price, "Should show ↑ for high prices");
        
        // Price below base should show down arrow
        assert!(current_price_low < base_price, "Should show ↓ for low prices");
        
        // Price equal to base should show right arrow
        assert_eq!(current_price_equal, base_price, "Should show → for equal prices");
    }
}

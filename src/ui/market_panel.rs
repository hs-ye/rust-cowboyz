//! Market Panel Component
//!
//! Displays market information for the selected planet with buy/sell functionality.

#[cfg(feature = "web")]
use leptos::view;
#[cfg(feature = "web")]
use leptos::IntoView;
#[cfg(feature = "web")]
use leptos::component;
#[cfg(feature = "web")]
use leptos::Callback;
#[cfg(feature = "web")]
use leptos::RwSignal;
#[cfg(feature = "web")]
use leptos::SignalGet;
#[cfg(feature = "web")]
use leptos::Signal;
#[cfg(feature = "web")]
use leptos::SignalSet;
#[cfg(feature = "web")]
use leptos::NodeRef;
#[cfg(feature = "web")]
use leptos::on_mount;
#[cfg(feature = "web")]
use leptos::on_cleanup;

#[cfg(feature = "web")]
use crate::game_state::{GameState, Transaction, TransactionType};
#[cfg(feature = "web")]
use crate::simulation::commodity::CommodityType;
#[cfg(feature = "web")]
use crate::simulation::economy::MarketGood;
#[cfg(feature = "web")]
use crate::simulation::planet_types::PlanetType;

#[cfg(feature = "web")]
use std::collections::HashMap;

/// Market entry for display in the UI
#[cfg(feature = "web")]
#[derive(Clone, Debug)]
pub struct MarketEntry {
    pub commodity_type: CommodityType,
    pub name: String,
    pub buy_price: u32,
    pub sell_price: u32,
    pub player_quantity: u32,
    pub is_produced: bool,
    pub is_demanded: bool,
}

/// Get market entries for a specific planet
#[cfg(feature = "web")]
pub fn get_market_entries(
    game_state: &GameState,
    planet_id: &str,
) -> Vec<MarketEntry> {
    let player = &game_state.player;
    
    // Get the planet's economy
    let planet = game_state.solar_system.get_planet(planet_id);
    if planet.is_none() {
        return Vec::new();
    }
    let planet = planet.unwrap();
    let economy = &planet.economy;
    
    // Build market entries for all commodities
    let mut entries = Vec::new();
    
    for commodity_type in CommodityType::all() {
        if let Some(market_good) = economy.get_commodity(&commodity_type) {
            let player_quantity = player.cargo.get_commodity_quantity(&commodity_type);
            
            entries.push(MarketEntry {
                commodity_type: commodity_type.clone(),
                name: commodity_type.display_name().to_string(),
                buy_price: market_good.buy_price,
                sell_price: market_good.sell_price,
                player_quantity,
                is_produced: market_good.is_produced,
                is_demanded: market_good.is_demanded,
            });
        }
    }
    
    entries
}

/// Check if player is at the given planet (can trade)
#[cfg(feature = "web")]
pub fn is_at_planet(game_state: &GameState, planet_id: &str) -> bool {
    game_state.player.location == planet_id
}

/// Buy commodity handler
#[cfg(feature = "web")]
pub fn buy_commodity(
    game_state: &mut GameState,
    commodity_type: &CommodityType,
    quantity: u32,
    planet_id: &str,
) -> Result<(), String> {
    // Verify player is at the planet
    if game_state.player.location != planet_id {
        return Err("You must be at the planet to trade".to_string());
    }
    
    // Get the planet's economy
    let planet = game_state.solar_system.get_planet_mut(planet_id)
        .ok_or("Planet not found")?;
    
    // Get the buy price (price player pays to buy from market)
    let buy_price = planet.economy.get_sell_price(commodity_type)
        .ok_or("Commodity not available")?;
    
    let total_cost = buy_price * quantity;
    
    // Check if player has enough money
    if game_state.player.money < total_cost {
        return Err(format!("Not enough money. Need ${}, have ${}", total_cost, game_state.player.money));
    }
    
    // Check if player has enough cargo space
    let cargo_space_needed = quantity;
    let remaining_space = game_state.player.cargo.remaining_capacity();
    if remaining_space < cargo_space_needed {
        return Err(format!("Not enough cargo space. Need {}, have {}", cargo_space_needed, remaining_space));
    }
    
    // Deduct money
    game_state.player.money -= total_cost;
    
    // Add to cargo
    game_state.player.cargo.add_commodity(commodity_type.clone(), quantity)
        .map_err(|e| e.to_string())?;
    
    // Update market (supply decreases when player buys)
    planet.economy.process_trade(commodity_type, -(quantity as i32))
        .map_err(|e| e.to_string())?;
    
    // Record transaction
    game_state.record_transaction(Transaction {
        turn: game_state.game_clock.current_turn,
        planet_id: planet_id.to_string(),
        commodity: commodity_type.clone(),
        quantity,
        price_per_unit: buy_price,
        transaction_type: TransactionType::Buy,
    });
    
    // Update player stats
    game_state.player.total_trades += 1;
    
    Ok(())
}

/// Sell commodity handler
#[cfg(feature = "web")]
pub fn sell_commodity(
    game_state: &mut GameState,
    commodity_type: &CommodityType,
    quantity: u32,
    planet_id: &str,
) -> Result<(), String> {
    // Verify player is at the planet
    if game_state.player.location != planet_id {
        return Err("You must be at the planet to trade".to_string());
    }
    
    // Get the planet's economy
    let planet = game_state.solar_system.get_planet_mut(planet_id)
        .ok_or("Planet not found")?;
    
    // Get the sell price (price player receives when selling to market)
    let sell_price = planet.economy.get_buy_price(commodity_type)
        .ok_or("Commodity not available")?;
    
    // Check if player has the commodity
    let player_quantity = game_state.player.cargo.get_commodity_quantity(commodity_type);
    if player_quantity < quantity {
        return Err(format!("Not enough {}. Have {}, trying to sell {}", 
            commodity_type.display_name(), player_quantity, quantity));
    }
    
    let total_earnings = sell_price * quantity;
    
    // Remove from cargo
    game_state.player.cargo.remove_commodity(commodity_type.clone(), quantity)
        .map_err(|e| e.to_string())?;
    
    // Add money
    game_state.player.money += total_earnings;
    
    // Update market (supply increases when player sells)
    planet.economy.process_trade(commodity_type, quantity as i32)
        .map_err(|e| e.to_string())?;
    
    // Record transaction
    game_state.record_transaction(Transaction {
        turn: game_state.game_clock.current_turn,
        planet_id: planet_id.to_string(),
        commodity: commodity_type.clone(),
        quantity,
        price_per_unit: sell_price,
        transaction_type: TransactionType::Sell,
    });
    
    // Update player stats
    game_state.player.total_trades += 1;
    game_state.player.total_earnings += total_earnings;
    
    Ok(())
}

/// Get planet type for display
#[cfg(feature = "web")]
pub fn get_planet_type(game_state: &GameState, planet_id: &str) -> Option<PlanetType> {
    game_state.solar_system.get_planet(planet_id).map(|p| p.planet_type.clone())
}

/// Calculate total cargo value at current market prices
#[cfg(feature = "web")]
pub fn calculate_cargo_value(game_state: &GameState) -> u32 {
    let player = &game_state.player;
    let planet = game_state.solar_system.get_planet(&player.location);
    
    if planet.is_none() {
        return 0;
    }
    let planet = planet.unwrap();
    
    let mut total_value = 0u32;
    
    for (commodity_type, &quantity) in player.cargo.get_commodities_list() {
        if let Some(market_good) = planet.economy.get_commodity(commodity_type) {
            total_value += market_good.buy_price * quantity;
        }
    }
    
    total_value
}

/// Market Panel Component for Leptos
#[cfg(feature = "web")]
#[component]
pub fn MarketPanel(
    game_state: RwSignal<GameState>,
    selected_planet: Signal<Option<String>>,
) -> impl IntoView {
    // Get the current planet ID (either selected or current location)
    let current_planet_id = Signal::derive(move || {
        selected_planet.get().unwrap_or_else(|| game_state.get().player.location.clone())
    });
    
    // Check if player is at the selected planet
    let is_at_selected_planet = Signal::derive(move || {
        let planet_id = current_planet_id.get();
        if let Some(id) = planet_id {
            is_at_planet(&game_state.get(), &id)
        } else {
            false
        }
    });
    
    // Get market entries for the current planet
    let market_entries = Signal::derive(move || {
        let planet_id = current_planet_id.get();
        if let Some(id) = planet_id {
            get_market_entries(&game_state.get(), &id)
        } else {
            Vec::new()
        }
    });
    
    // Get planet info
    let planet_info = Signal::derive(move || {
        let planet_id = current_planet_id.get();
        if let Some(id) = planet_id {
            game_state.get().solar_system.get_planet(&id).map(|p| {
                (p.name.clone(), p.planet_type.clone())
            })
        } else {
            None
        }
    });
    
    // Trading state for feedback
    let (trade_message, set_trade_message) = signal(Option::<(bool, String)>::None);
    
    // Clear message after delay
    let clear_message = move || {
        let set_msg = set_trade_message;
        set_timeout(move || set_msg.set(None), 2000);
    };
    
    // Buy handler
    let on_buy = move |(commodity_idx, quantity): (usize, u32)| {
        let planet_id = match current_planet_id.get() {
            Some(id) => id,
            None => return,
        };
        
        let entries = market_entries.get();
        if commodity_idx >= entries.len() {
            return;
        }
        
        let commodity_type = entries[commodity_idx].commodity_type.clone();
        
        match buy_commodity(&mut game_state.write().unwrap(), &commodity_type, quantity, &planet_id) {
            Ok(()) => {
                set_trade_message.set(Some((true, format!("Bought {}x {}", quantity, commodity_type.display_name()))));
                clear_message();
            }
            Err(e) => {
                set_trade_message.set(Some((false, e)));
                clear_message();
            }
        }
    };
    
    // Sell handler
    let on_sell = move |(commodity_idx, quantity): (usize, u32)| {
        let planet_id = match current_planet_id.get() {
            Some(id) => id,
            None => return,
        };
        
        let entries = market_entries.get();
        if commodity_idx >= entries.len() {
            return;
        }
        
        let commodity_type = entries[commodity_idx].commodity_type.clone();
        
        match sell_commodity(&mut game_state.write().unwrap(), &commodity_type, quantity, &planet_id) {
            Ok(()) => {
                set_trade_message.set(Some((true, format!("Sold {}x {}", quantity, commodity_type.display_name()))));
                clear_message();
            }
            Err(e) => {
                set_trade_message.set(Some((false, e)));
                clear_message();
            }
        }
    };
    
    view! {
        <div class="panel market-panel">
            <div class="panel-header">
                <h3>"市场" </h3>
                <span class="panel-subtitle">
                    {move || {
                        planet_info.get().map(|(name, planet_type)| {
                            format!("{} - {}", name, planet_type.display_name())
                        }).unwrap_or_else(|| "Market".to_string())
                    }}
                </span>
            </div>
            <div class="panel-content">
                // Trade message feedback
                {move || {
                    trade_message.get().map(|(success, message)| {
                        view! {
                            <div class={if success { "trade-message success" } else { "trade-message error" }}>
                                {message}
                            </div>
                        }
                    })
                }}
                
                // Market preview notice for distant planets
                {move || {
                    if !is_at_selected_planet.get() {
                        Some(view! {
                            <div class="market-preview-notice">
                                "📡 Market Preview (not at planet)"
                            </div>
                        })
                    } else {
                        None
                    }
                }}
                
                <div class="market-table">
                    <div class="market-header">
                        <span>"商品 Item"</span>
                        <span>"买入 Buy"</span>
                        <span>"卖出 Sell"</span>
                    </div>
                    {move || {
                        market_entries.get().into_iter().enumerate().map(|(idx, entry)| {
                            view! {
                                <div class="market-row">
                                    <div class="commodity-info">
                                        <span class="commodity-name">{entry.name}</span>
                                        {move || {
                                            if entry.player_quantity > 0 {
                                                Some(view! {
                                                    <span class="player-qty">{"(You: ".to_string() + &entry.player_quantity.to_string() + ")"}
                                                })
                                            } else {
                                                None
                                            }
                                        }}
                                        <div class="commodity-tags">
                                            {if entry.is_produced {
                                                Some(view! { <span class="tag produced">"Local"</span> })
                                            } else { None }}
                                            {if entry.is_demanded {
                                                Some(view! { <span class="tag demanded">"Wanted"</span> })
                                            } else { None }}
                                        </div>
                                    </div>
                                    <span class="buy-price">{"$".to_string() + &entry.buy_price.to_string()}</span>
                                    <span class="sell-price">{"$".to_string() + &entry.sell_price.to_string()}</span>
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>
                
                // Trading controls (only when at planet)
                {move || {
                    if is_at_selected_planet.get() {
                        Some(view! {
                            <div class="trading-controls">
                                <div class="trade-instructions">
                                    "Click commodity to trade"
                                </div>
                            </div>
                        })
                    } else {
                        None
                    }
                }}
            </div>
        </div>
    }
}

/// Inventory Panel Component for Leptos
#[cfg(feature = "web")]
#[component]
pub fn InventoryPanel(
    game_state: RwSignal<GameState>,
) -> impl IntoView {
    // Get inventory items
    let inventory_items = Signal::derive(move || {
        let player = &game_state.get().player;
        let planet = game_state.get().solar_system.get_planet(&player.location);
        
        let mut items: Vec<(String, u32, u32)> = Vec::new();
        
        for (commodity_type, &quantity) in player.cargo.get_commodities_list() {
            let value = if let Some(p) = planet {
                p.economy.get_buy_price(commodity_type).unwrap_or(0) * quantity
            } else {
                commodity_type.base_value() * quantity
            };
            
            items.push((commodity_type.display_name().to_string(), quantity, value));
        }
        
        items
    });
    
    // Get cargo stats
    let cargo_used = Signal::derive(move || game_state.get().player.cargo.current_load());
    let cargo_capacity = Signal::derive(move || game_state.get().player.cargo.capacity);
    let cargo_value = Signal::derive(move || {
        calculate_cargo_value(&game_state.get())
    });
    
    view! {
        <div class="panel inventory-panel">
            <div class="panel-header">
                <h3>"库存" </h3>
                <span class="panel-subtitle">"Inventory"</span>
            </div>
            <div class="panel-content">
                // Cargo space indicator
                <div class="cargo-stats">
                    <div class="stat-row">
                        <span class="stat-label">"货舱 Cargo:"</span>
                        <span class="stat-value"> {cargo_used()} "/ " {cargo_capacity()}</span>
                    </div>
                    <div class="progress-bar">
                        <div class="progress-fill cargo-fill" style={move || format!("width: {}%", 
                            if cargo_capacity() > 0 { (cargo_used() as f64 / cargo_capacity() as f64) * 100.0 } 
                            else { 0.0 }
                        )}></div>
                    </div>
                    <div class="stat-row">
                        <span class="stat-label">"价值 Value:"</span>
                        <span class="stat-value value">{"$".to_string() + &cargo_value().to_string()}</span>
                    </div>
                </div>
                
                // Inventory list
                {move || {
                    let items = inventory_items.get();
                    if items.is_empty() {
                        view! {
                            <div class="inventory-empty">
                                <p>"货舱为空"</p>
                                <p class="hint">"Cargo hold is empty"</p>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="inventory-list">
                                {items.into_iter().map(|(name, qty, value)| {
                                    view! {
                                        <div class="inventory-item">
                                            <span class="item-name">{name}</span>
                                            <span class="item-qty">{"x".to_string() + &qty.to_string()}</span>
                                            <span class="item-value">{"$".to_string() + &value.to_string()}</span>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        }
                    }
                }}
            </div>
        </div>
    }
}
//! Web entry point for Rust Cowboyz
//!
//! This is the entry point for the web application compiled to WASM.

use leptos::prelude::*;
use leptos_meta::{Title, Meta};

/// Main application component with 60/40 split-screen layout
#[component]
fn App() -> impl IntoView {
    // Create reactive game state
    let (money, set_money) = signal(1000);
    let (location, set_location) = signal("Earth".to_string());
    let (turn, set_turn) = signal(1);
    let (fuel, set_fuel) = signal(100);
    let (cargo_capacity, set_cargo_capacity) = signal(50);
    let (cargo_used, set_cargo_used) = signal(0);

    view! {
        <Title text="太空牛仔 - Rust Cowboyz" />
        <Meta name="description" content="A space-western trading game built with Rust and Leptos" />

        <div class="app-container">
            <header class="app-header">
                <h1>"太空牛仔" </h1>
                <span class="subtitle">"Space-Western Trading Game"</span>
            </header>

            <div class="split-layout">
                // Left side (60%): Solar System Map
                <div class="map-panel">
                    <div class="panel-header">
                        <h2>"太阳系地图" </h2>
                        <span class="panel-subtitle">"Solar System Map"</span>
                    </div>
                    <div class="map-viewport">
                        <div class="map-placeholder">
                            <div class="sun"></div>
                            <p>"太阳系地图将在这里显示"</p>
                            <p class="hint">"Solar system map will be displayed here"</p>
                        </div>
                    </div>
                </div>

                // Right side (40%): Information Panels
                <div class="info-panels">
                    // Player Status Panel
                    <div class="panel player-panel">
                        <div class="panel-header">
                            <h3>"玩家状态" </h3>
                            <span class="panel-subtitle">"Player Status"</span>
                        </div>
                        <div class="panel-content">
                            <div class="stat-row">
                                <span class="stat-label">"资金 Credits:"</span>
                                <span class="stat-value credits"> {move || format!("${}", money())}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"位置 Location:"</span>
                                <span class="stat-value location">{location}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"回合 Turn:"</span>
                                <span class="stat-value turn">{turn}</span>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"声望 Reputation:"</span>
                                <span class="stat-value">"新秀 Rookie"</span>
                            </div>
                        </div>
                    </div>

                    // Ship Status Panel
                    <div class="panel ship-panel">
                        <div class="panel-header">
                            <h3>"飞船状态" </h3>
                            <span class="panel-subtitle">"Ship Status"</span>
                        </div>
                        <div class="panel-content">
                            <div class="stat-row">
                                <span class="stat-label">"燃料 Fuel:"</span>
                                <span class="stat-value fuel"> {fuel()} "/ 100"</span>
                            </div>
                            <div class="progress-bar">
                                <div class="progress-fill fuel-fill" style={move || format!("width: {}%", fuel())}></div>
                            </div>
                            <div class="stat-row">
                                <span class="stat-label">"货舱 Cargo:"</span>
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

            // Action buttons
            <div class="actions">
                <button class="action-btn" on:click={move |_| set_money.update(|m| *m += 100)}>
                    <span class="btn-icon">"💰"</span>
                    <span>"测试: 增加资金"</span>
                </button>
                <button class="action-btn" on:click={move |_| set_turn.update(|t| *t += 1)}>
                    <span class="btn-icon">"⏱"</span>
                    <span>"下一回合"</span>
                </button>
                <button class="action-btn">
                    <span class="btn-icon">"⚙"</span>
                    <span>"新游戏"</span>
                </button>
            </div>
        </div>
    }
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
//! Web UI components for Rust Cowboyz

use leptos::*;

/// Main application component with 60/40 split-screen layout
#[component]
pub fn App() -> impl IntoView {
    // Create reactive game state
    let (money, set_money) = create_signal(1000);
    let (location, _set_location) = create_signal("Earth".to_string());
    let (turn, set_turn) = create_signal(1);
    let (fuel, _set_fuel) = create_signal(100);
    let (cargo_capacity, _set_cargo_capacity) = create_signal(50);
    let (cargo_used, _set_cargo_used) = create_signal(0);

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
                        <div class="map-placeholder">
                            <div class="sun"></div>
                            <p>"Solar system map will be displayed here"</p>
                        </div>
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

//! Market Panel Component
//!
//! Displays commodity prices for a selected planet's economy.
//! Uses reactive Leptos patterns to update when planet selection changes.

#[cfg(feature = "web")]
use leptos::*;
#[cfg(feature = "web")]
use crate::simulation::commodity::CommodityType;
#[cfg(feature = "web")]
use crate::simulation::economy::PlanetEconomy;
#[cfg(feature = "web")]
use crate::simulation::planet_types::PlanetType;

/// Market Panel Component
///
/// Displays the market/commodity prices for a given planet.
/// The panel shows all 10 commodity types with their buy and sell prices.
///
/// # Arguments
/// * `planet_name` - Name of the planet to display
/// * `_planet_type` - Type of the planet (determines economy) - currently unused but kept for API consistency
/// * `economy` - The planet's economy data containing commodity prices
#[cfg(feature = "web")]
#[component]
pub fn MarketPanel(
    planet_name: String,
    _planet_type: PlanetType,
    economy: PlanetEconomy,
) -> impl IntoView {
    // Get all commodity types
    let commodities = CommodityType::all();

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
                        <span>"Buy"</span>
                        <span>"Sell"</span>
                    </div>
                    {
                        commodities.into_iter().map(move |commodity| {
                            let commodity_name = commodity.display_name();
                            let buy_price = economy.get_buy_price(&commodity).unwrap_or(0);
                            let sell_price = economy.get_sell_price(&commodity).unwrap_or(0);

                            view! {
                                <div class="market-row">
                                    <span>{commodity_name}</span>
                                    <span class="buy-price">{format!("${}", buy_price)}</span>
                                    <span class="sell-price">{format!("${}", sell_price)}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }
                </div>
            </div>
        </div>
    }
}

/// Market Panel with Signal Support
///
/// This version accepts callbacks that return planet data,
/// allowing it to reactively update when the selected planet changes.
///
/// # Arguments
/// * `get_planet_name` - Callback to get the current planet name
/// * `get_planet_type` - Callback to get the current planet type (currently unused but accepted for API consistency)
/// * `get_economy` - Callback to get the current planet economy
#[cfg(feature = "web")]
#[component]
pub fn MarketPanelReactive(
    get_planet_name: Box<dyn Fn() -> String>,
    get_planet_type: Box<dyn Fn() -> PlanetType>,
    get_economy: Box<dyn Fn() -> PlanetEconomy>,
) -> impl IntoView {
    // Get all commodity types
    let commodities = CommodityType::all();

    // Note: get_planet_type is available but not currently used in rendering
    // It's kept for potential future use (e.g., planet type badges)
    let _ = get_planet_type;

    view! {
        <div class="panel market-panel">
            <div class="panel-header">
                <h3>"Market"</h3>
                <span class="panel-subtitle">{move || get_planet_name()}</span>
            </div>
            <div class="panel-content">
                <div class="market-table">
                    <div class="market-header">
                        <span>"Item"</span>
                        <span>"Buy"</span>
                        <span>"Sell"</span>
                    </div>
                    {
                        commodities.into_iter().map(move |commodity| {
                            let commodity_name = commodity.display_name();
                            let economy = get_economy();
                            let buy_price = economy.get_buy_price(&commodity).unwrap_or(0);
                            let sell_price = economy.get_sell_price(&commodity).unwrap_or(0);

                            view! {
                                <div class="market-row">
                                    <span>{commodity_name}</span>
                                    <span class="buy-price">{format!("${}", buy_price)}</span>
                                    <span class="sell-price">{format!("${}", sell_price)}</span>
                                </div>
                            }
                        }).collect::<Vec<_>>()
                    }
                </div>
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
        let mega_city_economy = PlanetEconomy::new(PlanetType::MegaCity);

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
}

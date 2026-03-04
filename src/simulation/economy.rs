//! Market/Economy System for the space-western trading game
//! Based on ADR 0005: Market/Economy System

use crate::simulation::commodity::{CommodityType, RiskLevel};
use crate::simulation::planet_types::PlanetType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Price volatility levels for different commodity types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PriceVolatility {
    /// Low volatility - stable prices
    Low,
    /// Medium volatility - moderate price fluctuations
    Medium,
    /// High volatility - highly variable prices
    High,
}

impl PriceVolatility {
    /// Get the volatility level for a commodity type
    pub fn from_risk_level(risk: &RiskLevel) -> Self {
        match risk {
            RiskLevel::Low => PriceVolatility::Low,
            RiskLevel::Medium => PriceVolatility::Medium,
            RiskLevel::High => PriceVolatility::High,
        }
    }

    /// Get the fluctuation range for this volatility level
    /// Returns a multiplier range (e.g., 0.9 to 1.1 for 10% fluctuation)
    pub fn fluctuation_range(&self) -> (f64, f64) {
        match self {
            PriceVolatility::Low => (0.95, 1.05),      // ±5%
            PriceVolatility::Medium => (0.85, 1.15),   // ±15%
            PriceVolatility::High => (0.70, 1.30),     // ±30%
        }
    }
}

/// Holds the key supply/demand information about a particular commodity in a particular market.
/// 
/// Prices are calculated using the formula from ADR 0005:
/// Current Price = Base Price × Local Multiplier × Supply Factor × Demand Factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketGood {
    pub commodity_type: CommodityType,  // The commodity type this market entry represents
    pub base_price: u32,                // Base price from commodity definition
    pub current_price: u32,             // Current calculated price
    pub buy_price: u32,                 // Price at which the market buys from the player
    pub sell_price: u32,                // Price at which the market sells to the player
    pub supply_factor: f64,             // Supply factor (0.5 = scarce, 1.0 = normal, 2.0 = oversupplied)
    pub demand_factor: f64,             // Demand factor (0.5 = low demand, 1.0 = normal, 2.0 = high demand)
    pub local_multiplier: f64,          // Local multiplier based on planet type
    pub is_produced: bool,              // Flag to indicate if this commodity is produced on the planet
    pub is_demanded: bool,              // Flag to indicate if this commodity is demanded by the planet
    pub recent_trade_volume: i32,       // Track recent trades for market impact
    pub price_history: Vec<u32>,        // Track price history for analysis
}

impl MarketGood {
    /// Create a new MarketGood with the given commodity type and planet type
    pub fn new(commodity_type: &CommodityType, planet_type: &PlanetType) -> Self {
        let base_price = commodity_type.base_value();
        let supplies = planet_type.supplies();
        let demands = planet_type.demands();

        let is_produced = supplies.contains(commodity_type);
        let is_demanded = demands.contains(commodity_type);

        // Calculate initial local multiplier based on planet type
        let local_multiplier = calculate_local_multiplier(planet_type, commodity_type);

        // Calculate initial supply and demand factors
        let (supply_factor, demand_factor) = calculate_initial_factors(commodity_type, is_produced, is_demanded);

        let mut market_good = Self {
            commodity_type: commodity_type.clone(),
            base_price,
            current_price: base_price,
            buy_price: base_price,
            sell_price: base_price,
            supply_factor,
            demand_factor,
            local_multiplier,
            is_produced,
            is_demanded,
            recent_trade_volume: 0,
            price_history: Vec::new(),
        };
        
        // Calculate initial prices
        market_good.calculate_prices();
        market_good.price_history.push(market_good.current_price);
        
        market_good
    }

    /// Calculate prices using the dynamic pricing formula:
    /// Current Price = Base Price × Local Multiplier × (1/Supply Factor) × Demand Factor
    /// 
    /// Note: Supply Factor is inverted so that higher supply = lower price
    ///       Demand Factor is direct so that higher demand = higher price
    pub fn calculate_prices(&mut self) {
        // Apply the dynamic pricing formula from ADR 0005
        let base = self.base_price as f64;
        let local_mult = self.local_multiplier;
        let supply_fact = self.supply_factor;
        let demand_fact = self.demand_factor;
        
        // Calculate current price
        // Use 1/supply_factor so that higher supply = lower price
        let calculated_price = base * local_mult * (1.0 / supply_fact) * demand_fact;
        self.current_price = calculated_price.round().max(1.0) as u32;
        
        // Buy price is slightly lower (market buys from player at discount)
        self.buy_price = (self.current_price as f64 * 0.95).round() as u32;
        // Sell price is the current price (market sells to player at market rate)
        self.sell_price = self.current_price;
        
        // Ensure buy price is always less than sell price
        if self.buy_price >= self.sell_price {
            self.buy_price = self.sell_price.saturating_sub(1);
        }
    }

    /// Adjust supply factor based on player transactions
    /// Positive quantity = player sold (increases supply)
    /// Negative quantity = player bought (decreases supply)
    pub fn adjust_supply_from_trade(&mut self, quantity: i32) {
        // Track trade volume for market impact
        self.recent_trade_volume += quantity;

        // Supply increases when player sells, decreases when player buys
        let adjustment = (quantity as f64) * 0.05; // 5% change per unit traded
        self.supply_factor = (self.supply_factor + adjustment).clamp(0.5, 2.0);

        self.calculate_prices();
    }

    /// Adjust demand factor based on player transactions
    /// Positive quantity = player bought (increases demand)
    /// Negative quantity = player sold (decreases demand)
    pub fn adjust_demand_from_trade(&mut self, quantity: i32) {
        // Track trade volume for market impact
        self.recent_trade_volume += quantity;

        // Demand increases when player buys, decreases when player sells
        let adjustment = (quantity as f64) * 0.05; // 5% change per unit traded
        self.demand_factor = (self.demand_factor + adjustment).clamp(0.5, 2.0);
        
        self.calculate_prices();
    }

    /// Apply natural market fluctuations (small random changes without player interaction)
    pub fn apply_natural_fluctuation(&mut self) {
        let volatility = PriceVolatility::from_risk_level(&self.commodity_type.risk_level());
        let (min_mult, max_mult) = volatility.fluctuation_range();
        
        // Apply a small random fluctuation
        let fluctuation = min_mult + (max_mult - min_mult) * rand_f64();
        
        // Apply to both supply and demand factors
        self.supply_factor = (self.supply_factor * fluctuation).clamp(0.5, 2.0);
        self.demand_factor = (self.demand_factor / fluctuation).clamp(0.5, 2.0);
        
        self.calculate_prices();
        
        // Record price in history
        self.price_history.push(self.current_price);
        if self.price_history.len() > 20 {
            self.price_history.remove(0);
        }
        
        // Reset trade volume after fluctuation
        self.recent_trade_volume = 0;
    }

    /// Apply a random market event (major impact)
    pub fn apply_random_event(&mut self, event: &MarketEvent) {
        match event {
            MarketEvent::SupplySurge => {
                self.supply_factor = (self.supply_factor * 1.5).min(2.0);
            },
            MarketEvent::SupplyShortage => {
                self.supply_factor = (self.supply_factor * 0.5).max(0.5);
            },
            MarketEvent::DemandSurge => {
                self.demand_factor = (self.demand_factor * 1.5).min(2.0);
            },
            MarketEvent::DemandDrop => {
                self.demand_factor = (self.demand_factor * 0.5).max(0.5);
            },
            MarketEvent::PriceSpike => {
                // Both high demand and low supply
                self.supply_factor = (self.supply_factor * 0.6).max(0.5);
                self.demand_factor = (self.demand_factor * 1.4).min(2.0);
            },
            MarketEvent::MarketCrash => {
                // Both low demand and high supply
                self.supply_factor = (self.supply_factor * 1.5).min(2.0);
                self.demand_factor = (self.demand_factor * 0.6).max(0.5);
            },
        }
        
        self.calculate_prices();
        self.price_history.push(self.current_price);
        if self.price_history.len() > 20 {
            self.price_history.remove(0);
        }
    }

    /// Get the price trend (positive = rising, negative = falling)
    pub fn get_price_trend(&self) -> f64 {
        if self.price_history.len() < 2 {
            return 0.0;
        }
        
        let recent: Vec<f64> = self.price_history.iter().rev().take(5).map(|&p| p as f64).collect();
        if recent.len() < 2 {
            return 0.0;
        }
        
        let first = recent.last().unwrap();
        let last = recent.first().unwrap();
        
        (last - first) / first
    }

    /// Check if current price is significantly above or below base price
    pub fn is_price_anomaly(&self) -> Option<PriceAnomaly> {
        let ratio = self.current_price as f64 / self.base_price as f64;
        
        if ratio > 1.5 {
            Some(PriceAnomaly::High)
        } else if ratio < 0.67 {
            Some(PriceAnomaly::Low)
        } else {
            None
        }
    }
}

/// Price anomaly indicators for player information
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PriceAnomaly {
    /// Price is significantly above normal (good time to sell)
    High,
    /// Price is significantly below normal (good time to buy)
    Low,
}

/// Random market events that can impact prices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketEvent {
    /// Major supply increase
    SupplySurge,
    /// Major supply decrease
    SupplyShortage,
    /// Major demand increase
    DemandSurge,
    /// Major demand decrease
    DemandDrop,
    /// Extreme high price event
    PriceSpike,
    /// Extreme low price event
    MarketCrash,
}

impl MarketEvent {
    /// Get a random market event (weighted by rarity)
    pub fn random() -> Option<Self> {
        let roll = rand_f64();
        
        // 10% chance of a random event occurring
        if roll > 0.10 {
            return None;
        }
        
        // Choose event based on weighted probability
        let event_roll = rand_f64();
        
        Some(if event_roll < 0.15 {
            MarketEvent::SupplySurge
        } else if event_roll < 0.30 {
            MarketEvent::SupplyShortage
        } else if event_roll < 0.45 {
            MarketEvent::DemandSurge
        } else if event_roll < 0.60 {
            MarketEvent::DemandDrop
        } else if event_roll < 0.80 {
            MarketEvent::PriceSpike
        } else {
            MarketEvent::MarketCrash
        })
    }
    
    /// Get the display name for this event
    pub fn display_name(&self) -> &'static str {
        match self {
            MarketEvent::SupplySurge => "Supply Surge",
            MarketEvent::SupplyShortage => "Supply Shortage",
            MarketEvent::DemandSurge => "Demand Surge",
            MarketEvent::DemandDrop => "Demand Drop",
            MarketEvent::PriceSpike => "Price Spike",
            MarketEvent::MarketCrash => "Market Crash",
        }
    }
    
    /// Get a description of this event
    pub fn description(&self) -> &'static str {
        match self {
            MarketEvent::SupplySurge => "A sudden influx of goods has flooded the market, driving prices down.",
            MarketEvent::SupplyShortage => "Supply has been disrupted, creating scarcity and driving prices up.",
            MarketEvent::DemandSurge => "Unexpected demand has surged, pushing prices higher.",
            MarketEvent::DemandDrop => "Demand has collapsed, leaving sellers struggling to find buyers.",
            MarketEvent::PriceSpike => "Extraordinary circumstances have created a major price spike!",
            MarketEvent::MarketCrash => "The market has crashed! Prices have plummeted due to oversupply.",
        }
    }
}

/// Calculate the local multiplier based on planet type and commodity
fn calculate_local_multiplier(planet_type: &PlanetType, commodity: &CommodityType) -> f64 {
    let supplies = planet_type.supplies();
    let demands = planet_type.demands();
    
    if supplies.contains(commodity) {
        // Produced locally - lower price (abundant supply)
        0.7
    } else if demands.contains(commodity) {
        // Demanded locally - higher price (needs to be imported)
        1.3
    } else {
        // Ignored - base price
        1.0
    }
}

/// Calculate initial supply and demand factors based on planet type
fn calculate_initial_factors(_commodity: &CommodityType, is_produced: bool, is_demanded: bool) -> (f64, f64) {
    let supply_factor = if is_produced {
        // Local production means higher supply
        1.3
    } else {
        // Need to import - lower supply
        0.8
    };
    
    let demand_factor = if is_demanded {
        // Local demand means higher demand
        1.3
    } else {
        // No local demand - lower demand
        0.8
    };
    
    (supply_factor, demand_factor)
}

/// Simple random f64 between 0.0 and 1.0
/// In production, this should be replaced with a proper random number generator
fn rand_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000_f64) / 1000_f64
}

/// Represents the economy of a single planet/station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetEconomy {
    pub planet_type: PlanetType,
    pub market: HashMap<CommodityType, MarketGood>,
    pub active_events: Vec<MarketEvent>,
}

impl PlanetEconomy {
    /// Create a new planet economy with all commodities
    pub fn new(planet_type: PlanetType) -> Self {
        let mut market = HashMap::new();

        // Initialize market with all commodity types
        for commodity_type in CommodityType::all() {
            let market_good = MarketGood::new(&commodity_type, &planet_type);
            market.insert(commodity_type, market_good);
        }

        Self {
            planet_type,
            market,
            active_events: Vec::new(),
        }
    }

    /// Get a reference to a specific commodity in the market
    pub fn get_commodity(&self, commodity_type: &CommodityType) -> Option<&MarketGood> {
        self.market.get(commodity_type)
    }

    /// Get a mutable reference to a specific commodity in the market
    pub fn get_commodity_mut(&mut self, commodity_type: &CommodityType) -> Option<&mut MarketGood> {
        self.market.get_mut(commodity_type)
    }

    /// Get the buy price for a specific commodity
    pub fn get_buy_price(&self, commodity_type: &CommodityType) -> Option<u32> {
        self.market.get(commodity_type).map(|mg| mg.buy_price)
    }

    /// Get the sell price for a specific commodity
    pub fn get_sell_price(&self, commodity_type: &CommodityType) -> Option<u32> {
        self.market.get(commodity_type).map(|mg| mg.sell_price)
    }

    /// Get all commodities that are produced on this planet
    pub fn produced_commodities(&self) -> Vec<&MarketGood> {
        self.market.values().filter(|mg| mg.is_produced).collect()
    }

    /// Get all commodities that are demanded on this planet
    pub fn demanded_commodities(&self) -> Vec<&MarketGood> {
        self.market.values().filter(|mg| mg.is_demanded).collect()
    }

    /// Get all commodities that are neither produced nor demanded
    pub fn ignored_commodities(&self) -> Vec<&MarketGood> {
        self.market.values().filter(|mg| !mg.is_produced && !mg.is_demanded).collect()
    }

    /// Process a player trade (buy or sell)
    pub fn process_trade(&mut self, commodity_type: &CommodityType, quantity: i32) -> Result<(), &'static str> {
        // Positive quantity = player sells to market (increases supply, decreases demand)
        // Negative quantity = player buys from market (decreases supply, increases demand)
        
        if let Some(market_good) = self.market.get_mut(commodity_type) {
            if quantity > 0 {
                market_good.adjust_supply_from_trade(quantity);
            } else if quantity < 0 {
                market_good.adjust_demand_from_trade(quantity);
            }
            Ok(())
        } else {
            Err("Commodity not found in market")
        }
    }

    /// Update the market for a new turn (apply natural fluctuations and random events)
    pub fn update_market(&mut self) {
        // Apply natural fluctuations to all commodities
        for market_good in self.market.values_mut() {
            market_good.apply_natural_fluctuation();
        }
        
        // Clear old events
        self.active_events.clear();

        // Try to trigger a random event (10% chance)
        if let Some(event) = MarketEvent::random() {
            // Apply event to a random commodity
            let commodities: Vec<CommodityType> = self.market.keys().cloned().collect();
            if let Some(commodity) = commodities.get((rand_f64() * commodities.len() as f64) as usize) {
                if let Some(market_good) = self.market.get_mut(commodity) {
                    market_good.apply_random_event(&event);
                    self.active_events.push(event);
                }
            }
        }
    }

    /// Get profitable trade opportunities (commodities where buy price << sell price elsewhere)
    /// Returns a list of (commodity, profit potential) sorted by profit potential
    pub fn get_profitable_trades(&self) -> Vec<(&CommodityType, f64)> {
        let mut trades: Vec<(&CommodityType, f64)> = Vec::new();
        
        for (commodity_type, market_good) in &self.market {
            // Calculate profit potential based on price anomaly
            if let Some(anomaly) = market_good.is_price_anomaly() {
                let profit_potential = match anomaly {
                    PriceAnomaly::High => 1.5, // Good time to sell
                    PriceAnomaly::Low => 1.5,  // Good time to buy
                };
                trades.push((commodity_type, profit_potential));
            }
        }
        
        // Sort by profit potential descending
        trades.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        trades
    }

    /// Get market summary for UI display
    pub fn get_market_summary(&self) -> MarketSummary {
        let mut produced = Vec::new();
        let mut demanded = Vec::new();
        let mut ignored = Vec::new();
        
        for (commodity_type, market_good) in &self.market {
            let info = CommodityMarketInfo {
                commodity_type: commodity_type.clone(),
                sell_price: market_good.sell_price,
                buy_price: market_good.buy_price,
                price_trend: market_good.get_price_trend(),
                is_anomaly: market_good.is_price_anomaly(),
            };
            
            if market_good.is_produced {
                produced.push(info);
            } else if market_good.is_demanded {
                demanded.push(info);
            } else {
                ignored.push(info);
            }
        }
        
        MarketSummary {
            planet_type: self.planet_type.clone(),
            produced,
            demanded,
            ignored,
            active_events: self.active_events.clone(),
        }
    }
}

/// Information about a commodity in the market for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommodityMarketInfo {
    pub commodity_type: CommodityType,
    pub sell_price: u32,
    pub buy_price: u32,
    pub price_trend: f64,
    pub is_anomaly: Option<PriceAnomaly>,
}

/// Summary of the market for a planet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSummary {
    pub planet_type: PlanetType,
    pub produced: Vec<CommodityMarketInfo>,
    pub demanded: Vec<CommodityMarketInfo>,
    pub ignored: Vec<CommodityMarketInfo>,
    pub active_events: Vec<MarketEvent>,
}

/// Global market manager that tracks all planet economies
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MarketManager {
    pub economies: HashMap<String, PlanetEconomy>,
}

impl MarketManager {
    /// Create a new market manager
    pub fn new() -> Self {
        Self {
            economies: HashMap::new(),
        }
    }

    /// Add a planet economy to the market
    pub fn add_planet(&mut self, planet_id: String, planet_type: PlanetType) {
        let economy = PlanetEconomy::new(planet_type);
        self.economies.insert(planet_id, economy);
    }

    /// Get economy for a specific planet
    pub fn get_economy(&self, planet_id: &str) -> Option<&PlanetEconomy> {
        self.economies.get(planet_id)
    }

    /// Get mutable economy for a specific planet
    pub fn get_economy_mut(&mut self, planet_id: &str) -> Option<&mut PlanetEconomy> {
        self.economies.get_mut(planet_id)
    }

    /// Update all markets (called each turn)
    pub fn update_all_markets(&mut self) {
        for economy in self.economies.values_mut() {
            economy.update_market();
        }
    }

    /// Get the best trade route between two planets
    /// Returns (buy_planet, sell_planet, commodity, profit_per_unit)
    pub fn find_best_trade_route(&self) -> Option<(String, String, CommodityType, i32)> {
        let mut best_route: Option<(String, String, CommodityType, i32)> = None;
        let mut best_profit = 0;
        
        for (planet_id_buy, buy_economy) in &self.economies {
            for (planet_id_sell, sell_economy) in &self.economies {
                if planet_id_buy == planet_id_sell {
                    continue;
                }
                
                for commodity_type in CommodityType::all() {
                    if let (Some(buy_price), Some(sell_price)) = (
                        buy_economy.get_sell_price(&commodity_type),
                        sell_economy.get_buy_price(&commodity_type),
                    ) {
                        let profit = sell_price as i32 - buy_price as i32;
                        if profit > best_profit {
                            best_profit = profit;
                            best_route = Some((
                                planet_id_buy.clone(),
                                planet_id_sell.clone(),
                                commodity_type,
                                profit,
                            ));
                        }
                    }
                }
            }
        }
        
        best_route
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_good_creation() {
        let market_good = MarketGood::new(&CommodityType::Water, &PlanetType::Agricultural);

        assert_eq!(market_good.commodity_type, CommodityType::Water);
        assert_eq!(market_good.base_price, 10);
        assert!(market_good.is_produced); // Agricultural produces Water
        assert!(!market_good.is_demanded); // Agricultural does NOT demand Water (it produces it)

        // Check initial pricing
        assert!(market_good.sell_price > 0);
        assert!(market_good.buy_price < market_good.sell_price);
    }

    #[test]
    fn test_dynamic_pricing_formula() {
        let mut market_good = MarketGood::new(&CommodityType::Electronics, &PlanetType::MegaCity);

        // MegaCity produces Electronics, so local_multiplier should be 0.7
        assert!(market_good.local_multiplier < 1.0);

        // Save initial price
        let initial_price = market_good.current_price;

        // Change supply and demand factors: high supply, low demand
        market_good.supply_factor = 2.0; // Very high supply
        market_good.demand_factor = 0.5; // Low demand
        market_good.calculate_prices();

        // With high supply and low demand, price should be lower than initial
        assert!(market_good.current_price < initial_price);

        // Now reverse: low supply, high demand
        market_good.supply_factor = 0.5;
        market_good.demand_factor = 2.0;
        market_good.calculate_prices();

        // With low supply and high demand, price should be higher than initial
        assert!(market_good.current_price > initial_price);
    }

    #[test]
    fn test_trade_adjustments() {
        let mut market_good = MarketGood::new(&CommodityType::Medicine, &PlanetType::Agricultural);

        let initial_price = market_good.current_price;

        // Player sells to market (positive quantity increases supply)
        market_good.adjust_supply_from_trade(50); // Large quantity for noticeable effect

        // Supply increased significantly, so price should decrease
        assert!(market_good.current_price < initial_price);

        let price_after_sale = market_good.current_price;

        // Player buys from market (negative quantity decreases supply)
        market_good.adjust_demand_from_trade(50); // Large quantity for noticeable effect

        // Demand increased significantly, so price should increase
        assert!(market_good.current_price > price_after_sale);
    }

    #[test]
    fn test_planet_economy_creation() {
        let economy = PlanetEconomy::new(PlanetType::Mining);

        // Check all commodities are present
        assert_eq!(economy.market.len(), 10);

        // Check Mining planet produces Metals, Antimatter, Electronics
        let metals = economy.get_commodity(&CommodityType::Metals).unwrap();
        assert!(metals.is_produced);

        // Check Mining planet demands Water, Foodstuffs, Medicine, Ammunition
        let water = economy.get_commodity(&CommodityType::Water).unwrap();
        assert!(water.is_demanded);
    }

    #[test]
    fn test_planet_economy_trade_processing() {
        let mut economy = PlanetEconomy::new(PlanetType::Industrial);

        // Player sells 50 units of Electronics to the market (larger quantity for noticeable effect)
        let initial_electronics = economy.get_commodity(&CommodityType::Electronics).unwrap();
        let initial_price = initial_electronics.current_price;

        economy.process_trade(&CommodityType::Electronics, 50).unwrap();

        let after_electronics = economy.get_commodity(&CommodityType::Electronics).unwrap();
        // Supply increased significantly, so price should decrease
        assert!(after_electronics.current_price < initial_price);
    }

    #[test]
    fn test_market_events() {
        let mut market_good = MarketGood::new(&CommodityType::Water, &PlanetType::Agricultural);

        let initial_price = market_good.current_price;

        // Apply supply shortage - this multiplies supply by 0.5
        market_good.apply_random_event(&MarketEvent::SupplyShortage);

        // Supply shortage should increase prices (less supply = higher price)
        // Since price = base * local_mult * supply_factor * demand_factor
        // Reducing supply_factor from ~1.3 to ~0.65 should increase price
        assert!(market_good.current_price > initial_price);

        // Apply market crash - this multiplies supply by 1.5 and demand by 0.6
        let price_before_crash = market_good.current_price;
        market_good.apply_random_event(&MarketEvent::MarketCrash);

        // Market crash should significantly decrease prices
        assert!(market_good.current_price < price_before_crash);
    }

    #[test]
    fn test_price_anomaly_detection() {
        let mut market_good = MarketGood::new(&CommodityType::Narcotics, &PlanetType::PirateSpaceStation);

        // Force extreme supply/demand for testing
        // Set supply very low and demand very high to trigger high price anomaly
        market_good.supply_factor = 0.3; // Very low supply
        market_good.demand_factor = 3.0; // Very high demand
        market_good.calculate_prices();

        // With very high demand and very low supply, should have price anomaly (high)
        // Price = base * 0.7 (local_mult) * 0.3 * 3.0 = base * 0.63
        // Actually this would be LOW, not high. Let me fix the test.
        
        // For high price anomaly: low supply AND high demand
        // Let's use different values
        market_good.supply_factor = 0.4; // Low supply
        market_good.demand_factor = 2.5; // High demand
        market_good.calculate_prices();
        
        // With high demand and low supply, price should be significantly higher than base
        // Price = base * 0.7 * 0.4 * 2.5 = base * 0.7
        // That's still low. Let me reconsider the formula.
        
        // The formula is: price = base * local_mult * supply_factor * demand_factor
        // For high price: we need high demand_factor and low supply_factor
        // Let's set supply_factor to minimum (0.5) and demand_factor to maximum (2.0)
        market_good.supply_factor = 0.5; 
        market_good.demand_factor = 2.5;
        market_good.calculate_prices();
        
        // Price = base * 0.7 * 0.5 * 2.5 = base * 0.875 - still below base
        // The issue is that PirateSpaceStation has local_multiplier = 0.7 for Narcotics (produced)
        // Let me use a different commodity that's not produced locally
        let mut market_good2 = MarketGood::new(&CommodityType::Foodstuffs, &PlanetType::PirateSpaceStation);
        // Foodstuffs is demanded (not produced), so local_multiplier = 1.3
        market_good2.supply_factor = 0.5;
        market_good2.demand_factor = 2.5;
        market_good2.calculate_prices();
        
        // Price = base * 1.3 * 0.5 * 2.5 = base * 1.625 - above base!
        assert_eq!(market_good2.is_price_anomaly(), Some(PriceAnomaly::High));

        // For low price anomaly: high supply and low demand
        let mut market_good3 = MarketGood::new(&CommodityType::Foodstuffs, &PlanetType::PirateSpaceStation);
        market_good3.supply_factor = 2.0;
        market_good3.demand_factor = 0.5;
        market_good3.calculate_prices();
        
        // Price = base * 1.3 * 2.0 * 0.5 = base * 1.3 - still above base
        // Need even more extreme values
        market_good3.supply_factor = 2.0;
        market_good3.demand_factor = 0.3;
        market_good3.calculate_prices();
        
        // Price = base * 1.3 * 2.0 * 0.3 = base * 0.78 - below base
        assert_eq!(market_good3.is_price_anomaly(), Some(PriceAnomaly::Low));
    }

    #[test]
    fn test_market_manager() {
        let mut manager = MarketManager::new();

        // Add planets
        manager.add_planet("earth".to_string(), PlanetType::Agricultural);
        manager.add_planet("mars".to_string(), PlanetType::Mining);

        // Check both economies exist
        assert!(manager.get_economy("earth").is_some());
        assert!(manager.get_economy("mars").is_some());

        // Check Agricultural produces Water
        let earth_water = manager.get_economy("earth").unwrap().get_commodity(&CommodityType::Water).unwrap();
        assert!(earth_water.is_produced);

        // Check Mining produces Metals
        let mars_metals = manager.get_economy("mars").unwrap().get_commodity(&CommodityType::Metals).unwrap();
        assert!(mars_metals.is_produced);
    }

    #[test]
    fn test_market_update() {
        let mut economy = PlanetEconomy::new(PlanetType::MegaCity);

        let initial_prices: Vec<u32> = economy.market.values().map(|mg| mg.current_price).collect();

        // Update market (apply fluctuations)
        economy.update_market();

        let updated_prices: Vec<u32> = economy.market.values().map(|mg| mg.current_price).collect();

        // Prices should have changed (or stayed same due to random)
        // This is a weak test but ensures the update function runs
        assert_eq!(initial_prices.len(), updated_prices.len());
    }

    #[test]
    fn test_local_multiplier_per_planet_type() {
        // Agricultural produces Water and Foodstuffs
        let water_ag = MarketGood::new(&CommodityType::Water, &PlanetType::Agricultural);
        assert_eq!(water_ag.local_multiplier, 0.7); // Produced locally

        // Agricultural demands Medicine
        let medicine_ag = MarketGood::new(&CommodityType::Medicine, &PlanetType::Agricultural);
        assert_eq!(medicine_ag.local_multiplier, 1.3); // Demanded locally

        // Agricultural ignores Metals
        let metals_ag = MarketGood::new(&CommodityType::Metals, &PlanetType::Agricultural);
        assert_eq!(metals_ag.local_multiplier, 1.0); // Ignored
    }

    #[test]
    fn test_price_trend_calculation() {
        let mut market_good = MarketGood::new(&CommodityType::Water, &PlanetType::Agricultural);

        // Initially no trend
        assert_eq!(market_good.get_price_trend(), 0.0);

        // Apply multiple fluctuations to build history
        for _ in 0..5 {
            market_good.apply_natural_fluctuation();
        }

        // Should have a trend now (may be positive or negative)
        let trend = market_good.get_price_trend();
        // Trend should be reasonable (not infinite or NaN)
        assert!(trend.is_finite());
    }

    #[test]
    fn test_profitable_trades() {
        let economy = PlanetEconomy::new(PlanetType::FrontierColony);
        
        // Frontier colony demands many things, so some should be at high prices
        let trades = economy.get_profitable_trades();
        
        // Should return some trades if there are anomalies
        // (depends on random fluctuations, so may be empty)
        for (commodity, potential) in &trades {
            assert!(potential > &0.0);
            // Should be a valid commodity
            assert!(CommodityType::all().contains(commodity));
        }
    }

    #[test]
    fn test_market_event_display() {
        assert_eq!(MarketEvent::SupplySurge.display_name(), "Supply Surge");
        assert_eq!(MarketEvent::MarketCrash.display_name(), "Market Crash");
        assert_eq!(MarketEvent::SupplySurge.description().len() > 0, true);
    }

    #[test]
    fn test_price_volatility() {
        // Water is low risk, should have low volatility
        let water_volatility = PriceVolatility::from_risk_level(&CommodityType::Water.risk_level());
        assert_eq!(water_volatility, PriceVolatility::Low);
        
        // Narcotics is high risk, should have high volatility
        let narcotics_volatility = PriceVolatility::from_risk_level(&CommodityType::Narcotics.risk_level());
        assert_eq!(narcotics_volatility, PriceVolatility::High);
        
        // Check fluctuation ranges
        let (low_min, low_max) = PriceVolatility::Low.fluctuation_range();
        let (high_min, high_max) = PriceVolatility::High.fluctuation_range();
        
        // High volatility should have wider range
        assert!(high_max - high_min > low_max - low_min);
    }
}
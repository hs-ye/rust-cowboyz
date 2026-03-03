// src/simulation/economy.rs

use crate::simulation::commodity::CommodityType;

/// Holds the key supply/demand information about a particular commodity in a particular market.
/// 
/// The variables (such as `is_produced`, `is_demanded` and the supply/demand levels) which are then used to calculate the `buy_price` and `sell_price`.
/// do not set `buy_price` and `sell_price` directly, it should be determined by supply and demand formula
#[derive(Debug, Clone)]
pub struct MarketCommodity {
    pub commodity_type: CommodityType,  // The commodity type this market entry represents
    pub buy_price: u32,               // Price at which the market buys from the player
    pub sell_price: u32,              // Price at which the market sells to the player  
    pub supply: f64,                  // Supply level (0.0-2.0)
    pub demand: f64,                  // Demand level (0.0-2.0)
    pub is_produced: bool,            // Flag to indicate if this commodity is produced on the planet
    pub is_demanded: bool,            // Flag to indicate if this commodity is demanded by the planet
}

impl MarketCommodity {
    /// Create a new MarketCommodity with the given commodity type and base pricing
    pub fn new(commodity_type: CommodityType, base_price: u32) -> Self {
        let mut market_commodity = Self {
            commodity_type,
            buy_price: base_price,
            sell_price: base_price,
            supply: 1.0,          // Neutral starting point
            demand: 1.0,          // Neutral starting point
            is_produced: false,
            is_demanded: false,
        };
        // Update prices to ensure proper buy/sell price relationship
        market_commodity.update_prices();
        market_commodity
    }

    /// Update prices based on current supply and demand
    pub fn update_prices(&mut self) {
        // Price calculation based on supply and demand
        // Higher demand increases price, higher supply decreases price
        let price_multiplier = self.demand / self.supply.max(f64::EPSILON);
        let base_price = self.commodity_type.base_value() as f64;
        let calculated_price = (base_price * price_multiplier).round() as u32;

        // Ensure buy price is slightly lower than sell price to allow for profit
        self.buy_price = calculated_price.saturating_sub(1);
        self.sell_price = calculated_price;
    }

    /// Adjust supply based on transactions
    pub fn adjust_supply(&mut self, quantity: i32) {
        let new_supply = self.supply + (quantity as f64 * 0.01); // Small adjustment factor
        self.supply = new_supply.clamp(0.1, 2.0); // Keep within reasonable bounds
        self.update_prices();
    }

    /// Adjust demand based on external factors
    pub fn adjust_demand(&mut self, change: f64) {
        let new_demand = self.demand + change;
        self.demand = new_demand.clamp(0.1, 2.0); // Keep within reasonable bounds
        self.update_prices();
    }
}

#[derive(Debug, Clone)]
pub struct PlanetEconomy {
    pub market: Vec<MarketCommodity>,
}

/// Filtering helpers for the `PlanetEconomy` struct, to extract produced and demanded commodities
impl PlanetEconomy {
    pub fn produced_commodities(&self) -> Vec<&MarketCommodity> {
        self.market.iter().filter(|mc| mc.is_produced).collect()
    }

    pub fn demanded_commodities(&self) -> Vec<&MarketCommodity> {
        self.market.iter().filter(|mc| mc.is_demanded).collect()
    }

    /// Find a specific commodity in the market
    pub fn find_commodity(&self, commodity_type: &CommodityType) -> Option<&MarketCommodity> {
        self.market.iter().find(|mc| &mc.commodity_type == commodity_type)
    }

    /// Find a mutable reference to a specific commodity in the market
    pub fn find_commodity_mut(&mut self, commodity_type: &CommodityType) -> Option<&mut MarketCommodity> {
        self.market.iter_mut().find(|mc| &mc.commodity_type == commodity_type)
    }

    /// Add a commodity to the market
    pub fn add_commodity(&mut self, market_commodity: MarketCommodity) {
        // If commodity already exists, replace it
        if let Some(existing) = self.find_commodity_mut(&market_commodity.commodity_type) {
            *existing = market_commodity;
        } else {
            self.market.push(market_commodity);
        }
    }

    /// Get the buy price for a specific commodity
    pub fn get_buy_price(&self, commodity_type: &CommodityType) -> Option<u32> {
        self.find_commodity(commodity_type).map(|mc| mc.buy_price)
    }

    /// Get the sell price for a specific commodity
    pub fn get_sell_price(&self, commodity_type: &CommodityType) -> Option<u32> {
        self.find_commodity(commodity_type).map(|mc| mc.sell_price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::commodity::CommodityType;

    #[test]
    fn test_market_commodity_creation() {
        let water_mc = MarketCommodity::new(CommodityType::Water, 10);
        assert_eq!(water_mc.commodity_type, CommodityType::Water);
        assert_eq!(water_mc.buy_price, 9);  // Base price - 1 after update_prices()
        assert_eq!(water_mc.sell_price, 10);  // Base price after update_prices()
        assert_eq!(water_mc.supply, 1.0);
        assert_eq!(water_mc.demand, 1.0);
    }

    #[test]
    fn test_market_commodity_price_updates() {
        let mut electronics_mc = MarketCommodity::new(CommodityType::Electronics, 120);
        let original_sell_price = electronics_mc.sell_price;
        
        // Increase demand, price should go up
        electronics_mc.adjust_demand(0.5); // demand becomes 1.5
        assert!(electronics_mc.sell_price > original_sell_price);
        
        // Decrease supply, price should go up (this should increase the price since supply/demand ratio changes)
        let previous_price = electronics_mc.sell_price;
        electronics_mc.adjust_supply(-50); // decrease supply
        assert!(electronics_mc.sell_price > previous_price);
        
        // Increase supply, price should go down
        let previous_price = electronics_mc.sell_price;
        electronics_mc.adjust_supply(100); // increase supply significantly
        assert!(electronics_mc.sell_price < previous_price);
    }

    #[test]
    fn test_planet_economy_operations() {
        let mut economy = PlanetEconomy { market: vec![] };
        
        let water_mc = MarketCommodity::new(CommodityType::Water, 10);
        economy.add_commodity(water_mc);
        
        // Check that we can retrieve the commodity
        let retrieved_mc = economy.find_commodity(&CommodityType::Water).unwrap();
        assert_eq!(retrieved_mc.buy_price, 9); // 10 - 1
        assert_eq!(retrieved_mc.sell_price, 10);
        assert_eq!(economy.get_buy_price(&CommodityType::Water), Some(9));
        assert_eq!(economy.get_sell_price(&CommodityType::Water), Some(10));
        
        // Check that non-existent commodity returns None
        assert!(economy.get_buy_price(&CommodityType::Medicine).is_none());
    }

    #[test]
    fn test_produced_and_demanded_filters() {
        let mut economy = PlanetEconomy { market: vec![] };
        
        let mut water_mc = MarketCommodity::new(CommodityType::Water, 10);
        water_mc.is_produced = true;
        economy.add_commodity(water_mc);
        
        let mut medicine_mc = MarketCommodity::new(CommodityType::Medicine, 100);
        medicine_mc.is_demanded = true;
        economy.add_commodity(medicine_mc);
        
        // Check filtering works
        assert_eq!(economy.produced_commodities().len(), 1);
        assert_eq!(economy.produced_commodities()[0].commodity_type, CommodityType::Water);
        
        assert_eq!(economy.demanded_commodities().len(), 1);
        assert_eq!(economy.demanded_commodities()[0].commodity_type, CommodityType::Medicine);
    }
}

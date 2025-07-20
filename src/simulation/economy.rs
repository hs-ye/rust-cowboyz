// src/simulation/economy.rs



#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct Good {
    pub id: String,
    pub base_value: u32,
}

/// Holds the key supply/demand information about a particular good in a particular market.
///
/// The variables (such as `is_produced`, `is_demanded` and the supply/demand levels) which are then used to calculate the `buy_price` and `sell_price`.
/// do not set `buy_price` and `sell_price` directly, it should be determined by supply and demand formula
#[derive(Debug, Clone)]
pub struct MarketGood {
    pub good: Good,         // The good this market entry represents
    pub buy_price: u32,   
    pub sell_price: u32,  
    pub supply: f64,      // Supply level (0.0-2.0)
    pub demand: f64,      // Demand level (0.0-2.0)
    pub is_produced: bool,  // Flag to indicate if this good is produced on the planet
    pub is_demanded: bool,  // Flag to indicate if this good is demanded by the planet
}

#[derive(Debug, Clone)]
pub struct PlanetEconomy {
    pub market: Vec<MarketGood>,
}

/// Filtering helpers for the `PlanetEconomy` struct, to extract produced and demanded goods
impl PlanetEconomy {
    pub fn produced_goods(&self) -> Vec<&MarketGood> {
        self.market.iter().filter(|mg| mg.is_produced).collect()
    }

    pub fn demanded_goods(&self) -> Vec<&MarketGood> {
        self.market.iter().filter(|mg| mg.is_demanded).collect()
    }
}

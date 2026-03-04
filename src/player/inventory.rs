use crate::simulation::commodity::{CommodityType, CommodityInventory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoHold {
    pub capacity: u32,
    pub commodities: CommodityInventory, // Using the new CommodityInventory system
}

impl CargoHold {
    pub fn new(capacity: u32) -> Self {
        CargoHold {
            capacity,
            commodities: CommodityInventory::new(capacity),
        }
    }

    pub fn add_commodity(&mut self, commodity_type: CommodityType, quantity: u32) -> Result<(), String> {
        match self.commodities.add_commodity(commodity_type, quantity) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn remove_commodity(&mut self, commodity_type: CommodityType, quantity: u32) -> Result<(), String> {
        match self.commodities.remove_commodity(&commodity_type, quantity) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_commodities_list(&self) -> Vec<(&CommodityType, &u32)> {
        // Convert HashMap<CommodityType, u32> to Vec<(&CommodityType, &u32)>
        self.commodities.commodities.iter().collect()
    }

    pub fn current_load(&self) -> u32 {
        self.commodities.total_cargo_space_used()
    }

    pub fn remaining_capacity(&self) -> u32 {
        self.commodities.remaining_cargo_space()
    }

    pub fn has_commodity(&self, commodity_type: &CommodityType) -> bool {
        self.commodities.contains(commodity_type)
    }

    pub fn get_commodity_quantity(&self, commodity_type: &CommodityType) -> u32 {
        self.commodities.get_quantity(commodity_type)
    }
}
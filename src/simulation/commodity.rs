//! Commodity types for the space-western trading game
//! Based on ADR 0005: Market/Economy System

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Enum representing all commodity types as defined in ADR 0005
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommodityType {
    /// Essential for survival, low value but high demand on dry worlds
    Water,
    /// Combination of grain, meat, and spices - staple nutrition sources
    Foodstuffs,
    /// Medical supplies, high value, essential for health
    Medicine,
    /// Weapons for protection/offense, high value on dangerous worlds
    Firearms,
    /// Weapon accessories, consumed regularly, moderate value
    Ammunition,
    /// Raw materials for construction and manufacturing
    Metals,
    /// Advanced raw material for energy production, high value
    Antimatter,
    /// High-tech components, essential for advanced civilizations
    Electronics,
    /// Illegal substances, high value but risky to trade
    Narcotics,
    /// Rare and mysterious items from ancient civilizations, extremely high value and risky to trade
    AlienArtefacts,
}

impl CommodityType {
    /// Get all commodity types
    pub fn all() -> Vec<CommodityType> {
        vec![
            CommodityType::Water,
            CommodityType::Foodstuffs,
            CommodityType::Medicine,
            CommodityType::Firearms,
            CommodityType::Ammunition,
            CommodityType::Metals,
            CommodityType::Antimatter,
            CommodityType::Electronics,
            CommodityType::Narcotics,
            CommodityType::AlienArtefacts,
        ]
    }

    /// Get the display name of the commodity
    pub fn display_name(&self) -> &'static str {
        match self {
            CommodityType::Water => "Water",
            CommodityType::Foodstuffs => "Foodstuffs",
            CommodityType::Medicine => "Medicine",
            CommodityType::Firearms => "Firearms",
            CommodityType::Ammunition => "Ammunition",
            CommodityType::Metals => "Metals",
            CommodityType::Antimatter => "Antimatter",
            CommodityType::Electronics => "Electronics",
            CommodityType::Narcotics => "Narcotics",
            CommodityType::AlienArtefacts => "Alien Artefacts",
        }
    }

    /// Get a brief description of the commodity
    pub fn description(&self) -> &'static str {
        match self {
            CommodityType::Water => "Essential for survival, low value but high demand on dry worlds",
            CommodityType::Foodstuffs => "Combination of grain, meat, and spices - staple nutrition sources",
            CommodityType::Medicine => "Medical supplies, high value, essential for health",
            CommodityType::Firearms => "Weapons for protection/offense, high value on dangerous worlds",
            CommodityType::Ammunition => "Weapon accessories, consumed regularly, moderate value",
            CommodityType::Metals => "Raw materials for construction and manufacturing",
            CommodityType::Antimatter => "Advanced raw material for energy production, high value",
            CommodityType::Electronics => "High-tech components, essential for advanced civilizations",
            CommodityType::Narcotics => "Illegal substances, high value but risky to trade",
            CommodityType::AlienArtefacts => "Rare and mysterious items from ancient civilizations, extremely high value and risky to trade",
        }
    }

    /// Get the base value of the commodity (used for pricing calculations)
    pub fn base_value(&self) -> u32 {
        match self {
            // Low-value essentials
            CommodityType::Water => 10,
            CommodityType::Foodstuffs => 20,
            
            // Moderate-value items
            CommodityType::Ammunition => 50,
            CommodityType::Metals => 60,
            
            // Higher-value items
            CommodityType::Medicine => 100,
            CommodityType::Electronics => 120,
            CommodityType::Firearms => 150,
            
            // High-value items
            CommodityType::Antimatter => 300,
            CommodityType::Narcotics => 400,
            CommodityType::AlienArtefacts => 800,
        }
    }

    /// Determine the risk level of trading this commodity
    pub fn risk_level(&self) -> RiskLevel {
        match self {
            CommodityType::Water | CommodityType::Foodstuffs => RiskLevel::Low,
            CommodityType::Ammunition | CommodityType::Metals | CommodityType::Medicine | 
            CommodityType::Electronics | CommodityType::Firearms => RiskLevel::Medium,
            CommodityType::Antimatter | CommodityType::Narcotics | CommodityType::AlienArtefacts => RiskLevel::High,
        }
    }
}

/// Risk level classification for commodities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk - stable prices, less volatile
    Low,
    /// Medium risk - moderate price fluctuations
    Medium,
    /// High risk - highly volatile prices, potentially high rewards
    High,
}

/// Represents a single unit of a commodity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commodity {
    /// The type of commodity
    pub commodity_type: CommodityType,
    /// Quantity of this commodity
    pub quantity: u32,
    /// Value per unit (may vary based on market conditions)
    pub value_per_unit: u32,
}

impl Commodity {
    /// Create a new commodity with specified type and quantity
    pub fn new(commodity_type: CommodityType, quantity: u32) -> Self {
        let base_value = commodity_type.base_value();
        Self {
            commodity_type,
            quantity,
            value_per_unit: base_value,
        }
    }

    /// Calculate total value of this commodity stack
    pub fn total_value(&self) -> u32 {
        self.quantity * self.value_per_unit
    }

    /// Update the value per unit (typically due to market fluctuations)
    pub fn update_value(&mut self, new_value_per_unit: u32) {
        self.value_per_unit = new_value_per_unit;
    }

    /// Add more units of this commodity
    pub fn add_quantity(&mut self, amount: u32) {
        self.quantity += amount;
    }

    /// Remove some units of this commodity
    pub fn remove_quantity(&mut self, amount: u32) -> Result<u32, &'static str> {
        if amount > self.quantity {
            return Err("Not enough quantity to remove");
        }
        self.quantity -= amount;
        Ok(amount)
    }
}

/// Commodity inventory that tracks multiple types of commodities
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CommodityInventory {
    /// Map of commodity types to their quantities
    pub commodities: HashMap<CommodityType, u32>,
    /// Maximum capacity of the inventory
    pub max_capacity: u32,
    /// Size per commodity unit (all commodities take 1 unit of space per ADR 0005)
    pub unit_size: u32,
}

impl CommodityInventory {
    /// Create a new commodity inventory with specified capacity
    pub fn new(max_capacity: u32) -> Self {
        Self {
            commodities: HashMap::new(),
            max_capacity,
            unit_size: 1, // Per ADR 0005: All commodities take a single unit of cargo space
        }
    }

    /// Add a commodity to inventory
    pub fn add_commodity(&mut self, commodity_type: CommodityType, quantity: u32) -> Result<(), &'static str> {
        let current_total = self.total_cargo_space_used();
        let additional_space_needed = quantity * self.unit_size;
        
        if current_total + additional_space_needed > self.max_capacity {
            return Err("Not enough cargo space");
        }

        *self.commodities.entry(commodity_type).or_insert(0) += quantity;
        Ok(())
    }

    /// Remove a commodity from inventory
    pub fn remove_commodity(&mut self, commodity_type: &CommodityType, quantity: u32) -> Result<u32, &'static str> {
        match self.commodities.get_mut(commodity_type) {
            Some(current_qty) => {
                if *current_qty < quantity {
                    return Err("Not enough of this commodity in inventory");
                }
                
                if *current_qty == quantity {
                    self.commodities.remove(commodity_type);
                } else {
                    *current_qty -= quantity;
                }
                
                Ok(quantity)
            },
            None => Err("Commodity not found in inventory"),
        }
    }

    /// Get the quantity of a specific commodity
    pub fn get_quantity(&self, commodity_type: &CommodityType) -> u32 {
        *self.commodities.get(commodity_type).unwrap_or(&0)
    }

    /// Check if inventory contains a specific commodity
    pub fn contains(&self, commodity_type: &CommodityType) -> bool {
        self.commodities.contains_key(commodity_type)
    }

    /// Calculate total cargo space currently used
    pub fn total_cargo_space_used(&self) -> u32 {
        self.commodities.values().map(|&qty| qty * self.unit_size).sum()
    }

    /// Calculate remaining cargo space
    pub fn remaining_cargo_space(&self) -> u32 {
        self.max_capacity.saturating_sub(self.total_cargo_space_used())
    }

    /// List all commodity types currently in inventory
    pub fn list_commodity_types(&self) -> Vec<CommodityType> {
        self.commodities.keys().cloned().collect()
    }

    /// Clear all commodities from inventory
    pub fn clear(&mut self) {
        self.commodities.clear();
    }

    /// Check if inventory is empty
    pub fn is_empty(&self) -> bool {
        self.commodities.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commodity_type_properties() {
        assert_eq!(CommodityType::Water.display_name(), "Water");
        assert_eq!(CommodityType::Water.description(), "Essential for survival, low value but high demand on dry worlds");
        assert_eq!(CommodityType::Water.base_value(), 10);
        assert_eq!(CommodityType::Water.risk_level(), RiskLevel::Low);

        assert_eq!(CommodityType::AlienArtefacts.display_name(), "Alien Artefacts");
        assert_eq!(CommodityType::AlienArtefacts.base_value(), 800);
        assert_eq!(CommodityType::AlienArtefacts.risk_level(), RiskLevel::High);
    }

    #[test]
    fn test_commodity_creation_and_value() {
        let water = Commodity::new(CommodityType::Water, 5);
        assert_eq!(water.commodity_type, CommodityType::Water);
        assert_eq!(water.quantity, 5);
        assert_eq!(water.value_per_unit, 10); // base value for Water
        assert_eq!(water.total_value(), 50); // 5 * 10
    }

    #[test]
    fn test_commodity_quantity_operations() {
        let mut medicine = Commodity::new(CommodityType::Medicine, 10);
        
        // Add quantity
        medicine.add_quantity(5);
        assert_eq!(medicine.quantity, 15);
        
        // Remove quantity
        assert_eq!(medicine.remove_quantity(7).unwrap(), 7);
        assert_eq!(medicine.quantity, 8);
        
        // Try to remove more than available
        assert!(medicine.remove_quantity(10).is_err());
    }

    #[test]
    fn test_commodity_inventory_basic_operations() {
        let mut inventory = CommodityInventory::new(100);
        
        // Add commodities
        assert!(inventory.add_commodity(CommodityType::Water, 10).is_ok());
        assert!(inventory.add_commodity(CommodityType::Foodstuffs, 5).is_ok());
        
        // Check quantities
        assert_eq!(inventory.get_quantity(&CommodityType::Water), 10);
        assert_eq!(inventory.get_quantity(&CommodityType::Foodstuffs), 5);
        
        // Check cargo space
        assert_eq!(inventory.total_cargo_space_used(), 15); // 10 + 5
        assert_eq!(inventory.remaining_cargo_space(), 85); // 100 - 15
    }

    #[test]
    fn test_commodity_inventory_overflow() {
        let mut inventory = CommodityInventory::new(10);
        
        // Adding more than capacity should fail
        assert!(inventory.add_commodity(CommodityType::Water, 15).is_err());
        
        // Adding up to capacity should work
        assert!(inventory.add_commodity(CommodityType::Water, 10).is_ok());
        assert_eq!(inventory.total_cargo_space_used(), 10);
        
        // Adding more should fail
        assert!(inventory.add_commodity(CommodityType::Foodstuffs, 1).is_err());
    }

    #[test]
    fn test_commodity_inventory_remove_operations() {
        let mut inventory = CommodityInventory::new(100);
        
        // Add commodities
        inventory.add_commodity(CommodityType::Medicine, 10).unwrap();
        
        // Remove some
        assert_eq!(inventory.remove_commodity(&CommodityType::Medicine, 3).unwrap(), 3);
        assert_eq!(inventory.get_quantity(&CommodityType::Medicine), 7);
        
        // Remove all remaining
        assert_eq!(inventory.remove_commodity(&CommodityType::Medicine, 7).unwrap(), 7);
        assert!(!inventory.contains(&CommodityType::Medicine));
        
        // Try to remove from empty
        assert!(inventory.remove_commodity(&CommodityType::Medicine, 1).is_err());
    }

    #[test]
    fn test_all_commodity_types_exist() {
        let all_types = CommodityType::all();
        assert_eq!(all_types.len(), 10);
        assert!(all_types.contains(&CommodityType::Water));
        assert!(all_types.contains(&CommodityType::Foodstuffs));
        assert!(all_types.contains(&CommodityType::Medicine));
        assert!(all_types.contains(&CommodityType::Firearms));
        assert!(all_types.contains(&CommodityType::Ammunition));
        assert!(all_types.contains(&CommodityType::Metals));
        assert!(all_types.contains(&CommodityType::Antimatter));
        assert!(all_types.contains(&CommodityType::Electronics));
        assert!(all_types.contains(&CommodityType::Narcotics));
        assert!(all_types.contains(&CommodityType::AlienArtefacts));
    }
}
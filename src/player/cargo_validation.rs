//! Cargo validation service for trading operations
//! Based on ADR 0008: Cargo Hold Management and Capacity Constraints
//!
//! This module provides stateless validation methods for cargo operations,
//! ensuring all trades comply with cargo capacity and inventory constraints.

use crate::player::inventory::CargoHold;
use crate::simulation::commodity::{CommodityInventory, CommodityType};

/// Errors that can occur during cargo validation
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Requested cargo space exceeds available capacity
    InsufficientCargoSpace {
        /// Amount of cargo space requested
        requested: u32,
        /// Amount of cargo space actually available
        available: u32,
    },
    /// Attempting to sell more than owned
    InsufficientInventory {
        /// The commodity type being sold
        commodity: CommodityType,
        /// Amount requested to sell
        requested: u32,
        /// Amount actually available in inventory
        available: u32,
    },
    /// Player cannot afford the transaction
    InsufficientCredits {
        /// Credits required for transaction
        required: u32,
        /// Credits actually available
        available: u32,
    },
    /// Trade request is invalid for other reasons
    InvalidTrade {
        /// Human-readable explanation of the issue
        reason: String,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InsufficientCargoSpace {
                requested,
                available,
            } => write!(
                f,
                "Insufficient cargo space: requested {} units, but only {} available",
                requested, available
            ),
            ValidationError::InsufficientInventory {
                commodity,
                requested,
                available,
            } => write!(
                f,
                "Insufficient inventory for {:?}: requested {} units, but only {} available",
                commodity, requested, available
            ),
            ValidationError::InsufficientCredits {
                required,
                available,
            } => write!(
                f,
                "Insufficient credits: required {} credits, but only {} available",
                required, available
            ),
            ValidationError::InvalidTrade { reason } => {
                write!(f, "Invalid trade: {}", reason)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Represents a single trade request (buy or sell)
#[derive(Debug, Clone)]
pub struct TradeRequest {
    /// The commodity being traded
    pub commodity: CommodityType,
    /// Quantity to trade (positive for both buy and sell)
    pub quantity: u32,
    /// Price per unit
    pub price_per_unit: u32,
    /// True if buying, false if selling
    pub is_buy: bool,
}

impl TradeRequest {
    /// Create a new buy trade request
    pub fn buy(commodity: CommodityType, quantity: u32, price_per_unit: u32) -> Self {
        TradeRequest {
            commodity,
            quantity,
            price_per_unit,
            is_buy: true,
        }
    }

    /// Create a new sell trade request
    pub fn sell(commodity: CommodityType, quantity: u32, price_per_unit: u32) -> Self {
        TradeRequest {
            commodity,
            quantity,
            price_per_unit,
            is_buy: false,
        }
    }

    /// Calculate the total value of this trade
    pub fn total_value(&self) -> u32 {
        self.quantity * self.price_per_unit
    }
}

/// Player data needed for trade validation
/// This is a simplified view of player state for validation purposes
#[derive(Debug, Clone)]
pub struct PlayerTradeView {
    /// Current money/credits
    pub money: u32,
    /// Current cargo hold
    pub cargo: CargoHold,
}

impl PlayerTradeView {
    /// Create a new player trade view
    pub fn new(money: u32, cargo: CargoHold) -> Self {
        PlayerTradeView { money, cargo }
    }
}

/// Stateless cargo validation service
///
/// This service provides validation methods for cargo operations without
/// maintaining any internal state. All validation is performed based on
/// the parameters passed to each method.
pub struct CargoValidationService;

impl CargoValidationService {
    /// Check if cargo can be added to the cargo hold
    ///
    /// # Arguments
    /// * `current_load` - Current amount of cargo space used
    /// * `capacity` - Total cargo capacity
    /// * `quantity` - Amount of cargo to add
    ///
    /// # Returns
    /// * `true` if the cargo can be added without exceeding capacity
    /// * `false` if adding would exceed capacity
    ///
    /// # Examples
    /// ```
    /// use cowboyz::player::cargo_validation::CargoValidationService;
    ///
    /// let can_add = CargoValidationService::can_add_cargo(50, 100, 30);
    /// assert!(can_add); // 50 + 30 = 80 <= 100
    ///
    /// let cannot_add = CargoValidationService::can_add_cargo(50, 100, 60);
    /// assert!(!cannot_add); // 50 + 60 = 110 > 100
    /// ```
    pub fn can_add_cargo(current_load: u32, capacity: u32, quantity: u32) -> bool {
        current_load.saturating_add(quantity) <= capacity
    }

    /// Check if cargo can be removed from inventory
    ///
    /// # Arguments
    /// * `inventory` - Current commodity inventory
    /// * `commodity` - Type of commodity to remove
    /// * `quantity` - Amount to remove
    ///
    /// # Returns
    /// * `true` if the commodity exists and quantity is available
    /// * `false` if commodity doesn't exist or insufficient quantity
    ///
    /// # Examples
    /// ```
    /// use cowboyz::player::cargo_validation::CargoValidationService;
    /// use cowboyz::simulation::commodity::{CommodityInventory, CommodityType};
    ///
    /// let mut inventory = CommodityInventory::new(100);
    /// inventory.add_commodity(CommodityType::Water, 10).unwrap();
    ///
    /// let can_remove = CargoValidationService::can_remove_cargo(&inventory, &CommodityType::Water, 5);
    /// assert!(can_remove);
    ///
    /// let cannot_remove = CargoValidationService::can_remove_cargo(&inventory, &CommodityType::Water, 15);
    /// assert!(!cannot_remove);
    /// ```
    pub fn can_remove_cargo(
        inventory: &CommodityInventory,
        commodity: &CommodityType,
        quantity: u32,
    ) -> bool {
        inventory.get_quantity(commodity) >= quantity
    }

    /// Validate a complete trade basket (multiple trade requests)
    ///
    /// This method validates all trades in the basket against the player's
    /// current state, checking:
    /// - Cargo capacity for all buy operations
    /// - Inventory availability for all sell operations
    /// - Credit availability for all buy operations
    /// - Validity of each trade request
    ///
    /// # Arguments
    /// * `trades` - Slice of trade requests to validate
    /// * `player` - Player's current state
    ///
    /// # Returns
    /// * `Ok(())` if all trades are valid
    /// * `Err(ValidationError)` with the first validation error encountered
    ///
    /// # Examples
    /// ```
    /// use cowboyz::player::cargo_validation::{CargoValidationService, PlayerTradeView, TradeRequest};
    /// use cowboyz::player::inventory::CargoHold;
    /// use cowboyz::simulation::commodity::CommodityType;
    ///
    /// let mut cargo = CargoHold::new(100);
    /// cargo.add_commodity(CommodityType::Water, 10).unwrap();
    /// let player = PlayerTradeView::new(1000, cargo);
    ///
    /// let trades = vec![
    ///     TradeRequest::buy(CommodityType::Foodstuffs, 5, 20),
    ///     TradeRequest::sell(CommodityType::Water, 5, 15),
    /// ];
    ///
    /// let result = CargoValidationService::validate_trade_basket(&trades, &player);
    /// assert!(result.is_ok());
    /// ```
    pub fn validate_trade_basket(
        trades: &[TradeRequest],
        player: &PlayerTradeView,
    ) -> Result<(), ValidationError> {
        // First pass: validate individual trades for basic validity and inventory/credits
        // (but NOT cargo space for buys - that's checked in combination)
        for trade in trades {
            Self::validate_single_trade_for_basket(trade, player)?;
        }

        // Second pass: validate combined effect on cargo and credits
        let mut net_cargo_change: i32 = 0;
        let mut net_credit_change: i32 = 0;

        for trade in trades {
            if trade.is_buy {
                net_cargo_change += trade.quantity as i32;
                net_credit_change -= trade.total_value() as i32;
            } else {
                net_cargo_change -= trade.quantity as i32;
                net_credit_change += trade.total_value() as i32;
            }
        }

        // Check combined cargo capacity
        let current_load = player.cargo.current_load();
        let capacity = player.cargo.capacity;

        let projected_load = (current_load as i32) + net_cargo_change;
        if projected_load > capacity as i32 {
            // Calculate how much cargo space the buys would need beyond what sells free up
            let total_buy_quantity: u32 = trades.iter().filter(|t| t.is_buy).map(|t| t.quantity).sum();
            let available = capacity.saturating_sub(current_load);
            return Err(ValidationError::InsufficientCargoSpace {
                requested: total_buy_quantity,
                available,
            });
        }

        // Check combined credits (only matters if net credit change is negative)
        if net_credit_change < 0 {
            let required = (-net_credit_change) as u32;
            if required > player.money {
                return Err(ValidationError::InsufficientCredits {
                    required,
                    available: player.money,
                });
            }
        }

        Ok(())
    }

    /// Calculate remaining cargo capacity for a player
    ///
    /// # Arguments
    /// * `player` - Player whose cargo hold to check
    ///
    /// # Returns
    /// * Amount of unused cargo capacity
    ///
    /// # Examples
    /// ```
    /// use cowboyz::player::cargo_validation::{CargoValidationService, PlayerTradeView};
    /// use cowboyz::player::inventory::CargoHold;
    /// use cowboyz::simulation::commodity::CommodityType;
    ///
    /// let mut cargo = CargoHold::new(100);
    /// cargo.add_commodity(CommodityType::Water, 30).unwrap();
    /// let player = PlayerTradeView::new(1000, cargo);
    ///
    /// let remaining = CargoValidationService::calculate_remaining_capacity(&player);
    /// assert_eq!(remaining, 70);
    /// ```
    pub fn calculate_remaining_capacity(player: &PlayerTradeView) -> u32 {
        player.cargo.remaining_capacity()
    }

    /// Validate a single trade request (for standalone validation)
    ///
    /// # Arguments
    /// * `trade` - Trade request to validate
    /// * `player` - Player's current state
    ///
    /// # Returns
    /// * `Ok(())` if the trade is valid
    /// * `Err(ValidationError)` with the specific error
    #[allow(dead_code)] // Reserved for future standalone trade validation
    fn validate_single_trade(
        trade: &TradeRequest,
        player: &PlayerTradeView,
    ) -> Result<(), ValidationError> {
        // Validate quantity is positive
        if trade.quantity == 0 {
            return Err(ValidationError::InvalidTrade {
                reason: "Trade quantity must be greater than zero".to_string(),
            });
        }

        // Validate price is positive
        if trade.price_per_unit == 0 {
            return Err(ValidationError::InvalidTrade {
                reason: "Trade price must be greater than zero".to_string(),
            });
        }

        if trade.is_buy {
            // Check cargo space for buy
            let current_load = player.cargo.current_load();
            let capacity = player.cargo.capacity;
            if !Self::can_add_cargo(current_load, capacity, trade.quantity) {
                let available = capacity.saturating_sub(current_load);
                return Err(ValidationError::InsufficientCargoSpace {
                    requested: trade.quantity,
                    available,
                });
            }

            // Check credits for buy
            let total_cost = trade.total_value();
            if total_cost > player.money {
                return Err(ValidationError::InsufficientCredits {
                    required: total_cost,
                    available: player.money,
                });
            }
        } else {
            // Check inventory for sell
            if !Self::can_remove_cargo(&player.cargo.commodities, &trade.commodity, trade.quantity)
            {
                let available = player.cargo.commodities.get_quantity(&trade.commodity);
                return Err(ValidationError::InsufficientInventory {
                    commodity: trade.commodity.clone(),
                    requested: trade.quantity,
                    available,
                });
            }
        }

        Ok(())
    }

    /// Validate a single trade request for basket validation
    /// This version does NOT check cargo space or credits (those are done in combination)
    /// but DOES check inventory for sells and basic validity
    ///
    /// # Arguments
    /// * `trade` - Trade request to validate
    /// * `player` - Player's current state
    ///
    /// # Returns
    /// * `Ok(())` if the trade is valid for basket inclusion
    /// * `Err(ValidationError)` with the specific error
    fn validate_single_trade_for_basket(
        trade: &TradeRequest,
        player: &PlayerTradeView,
    ) -> Result<(), ValidationError> {
        // Validate quantity is positive
        if trade.quantity == 0 {
            return Err(ValidationError::InvalidTrade {
                reason: "Trade quantity must be greater than zero".to_string(),
            });
        }

        // Validate price is positive
        if trade.price_per_unit == 0 {
            return Err(ValidationError::InvalidTrade {
                reason: "Trade price must be greater than zero".to_string(),
            });
        }

        if !trade.is_buy {
            // Check inventory for sell (cargo/credits checked in combination)
            if !Self::can_remove_cargo(&player.cargo.commodities, &trade.commodity, trade.quantity)
            {
                let available = player.cargo.commodities.get_quantity(&trade.commodity);
                return Err(ValidationError::InsufficientInventory {
                    commodity: trade.commodity.clone(),
                    requested: trade.quantity,
                    available,
                });
            }
        }
        // For buys: cargo space and credits are checked in combination

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Capacity Validation Tests
    // ========================================================================

    mod capacity_validation {
        use super::*;

        #[test]
        fn test_can_add_cargo_under_capacity() {
            // Current load: 50, Capacity: 100, Adding: 30 -> Should succeed (80 <= 100)
            assert!(CargoValidationService::can_add_cargo(50, 100, 30));
        }

        #[test]
        fn test_can_add_cargo_at_exact_capacity() {
            // Current load: 50, Capacity: 100, Adding: 50 -> Should succeed (100 <= 100)
            assert!(CargoValidationService::can_add_cargo(50, 100, 50));
        }

        #[test]
        fn test_can_add_cargo_over_capacity() {
            // Current load: 50, Capacity: 100, Adding: 60 -> Should fail (110 > 100)
            assert!(!CargoValidationService::can_add_cargo(50, 100, 60));
        }

        #[test]
        fn test_can_add_cargo_empty_hold() {
            // Current load: 0, Capacity: 100, Adding: 100 -> Should succeed
            assert!(CargoValidationService::can_add_cargo(0, 100, 100));
        }

        #[test]
        fn test_can_add_cargo_full_hold() {
            // Current load: 100, Capacity: 100, Adding: 1 -> Should fail
            assert!(!CargoValidationService::can_add_cargo(100, 100, 1));
        }

        #[test]
        fn test_can_add_cargo_zero_quantity() {
            // Adding 0 should always succeed
            assert!(CargoValidationService::can_add_cargo(50, 100, 0));
            assert!(CargoValidationService::can_add_cargo(100, 100, 0));
        }

        #[test]
        fn test_can_add_cargo_overflow_protection() {
            // Test with values near u32::MAX to ensure no overflow
            // Using saturating_add, so max + 1 should saturate to max, which equals capacity
            let max = u32::MAX;
            // max.saturating_add(1) = max, and max <= max is true
            assert!(CargoValidationService::can_add_cargo(max, max, 1));
            assert!(CargoValidationService::can_add_cargo(0, max, max));
        }
    }

    // ========================================================================
    // Inventory Validation Tests
    // ========================================================================

    mod inventory_validation {
        use super::*;

        #[test]
        fn test_can_remove_cargo_partial_amount() {
            let mut inventory = CommodityInventory::new(100);
            inventory
                .add_commodity(CommodityType::Water, 10)
                .unwrap();

            // Have 10, removing 5 -> Should succeed
            assert!(CargoValidationService::can_remove_cargo(
                &inventory,
                &CommodityType::Water,
                5
            ));
        }

        #[test]
        fn test_can_remove_cargo_exact_amount() {
            let mut inventory = CommodityInventory::new(100);
            inventory
                .add_commodity(CommodityType::Water, 10)
                .unwrap();

            // Have 10, removing 10 -> Should succeed
            assert!(CargoValidationService::can_remove_cargo(
                &inventory,
                &CommodityType::Water,
                10
            ));
        }

        #[test]
        fn test_can_remove_cargo_more_than_owned() {
            let mut inventory = CommodityInventory::new(100);
            inventory
                .add_commodity(CommodityType::Water, 10)
                .unwrap();

            // Have 10, removing 15 -> Should fail
            assert!(!CargoValidationService::can_remove_cargo(
                &inventory,
                &CommodityType::Water,
                15
            ));
        }

        #[test]
        fn test_can_remove_cargo_not_in_inventory() {
            let inventory = CommodityInventory::new(100);

            // Don't have Water, trying to remove 5 -> Should fail
            assert!(!CargoValidationService::can_remove_cargo(
                &inventory,
                &CommodityType::Water,
                5
            ));
        }

        #[test]
        fn test_can_remove_cargo_zero_quantity() {
            let mut inventory = CommodityInventory::new(100);
            inventory
                .add_commodity(CommodityType::Water, 10)
                .unwrap();

            // Removing 0 should always succeed
            assert!(CargoValidationService::can_remove_cargo(
                &inventory,
                &CommodityType::Water,
                0
            ));
        }

        #[test]
        fn test_can_remove_cargo_multiple_commodities() {
            let mut inventory = CommodityInventory::new(100);
            inventory.add_commodity(CommodityType::Water, 10).unwrap();
            inventory
                .add_commodity(CommodityType::Foodstuffs, 5)
                .unwrap();
            inventory.add_commodity(CommodityType::Medicine, 3).unwrap();

            // Can remove from each independently
            assert!(CargoValidationService::can_remove_cargo(
                &inventory,
                &CommodityType::Water,
                5
            ));
            assert!(CargoValidationService::can_remove_cargo(
                &inventory,
                &CommodityType::Foodstuffs,
                3
            ));
            assert!(CargoValidationService::can_remove_cargo(
                &inventory,
                &CommodityType::Medicine,
                3
            ));

            // Cannot remove more than available
            assert!(!CargoValidationService::can_remove_cargo(
                &inventory,
                &CommodityType::Medicine,
                5
            ));
        }
    }

    // ========================================================================
    // Remaining Capacity Tests
    // ========================================================================

    mod remaining_capacity {
        use super::*;

        #[test]
        fn test_calculate_remaining_capacity_empty() {
            let cargo = CargoHold::new(100);
            let player = PlayerTradeView::new(1000, cargo);

            assert_eq!(
                CargoValidationService::calculate_remaining_capacity(&player),
                100
            );
        }

        #[test]
        fn test_calculate_remaining_capacity_partial() {
            let mut cargo = CargoHold::new(100);
            cargo.add_commodity(CommodityType::Water, 30).unwrap();
            cargo.add_commodity(CommodityType::Foodstuffs, 20).unwrap();
            let player = PlayerTradeView::new(1000, cargo);

            assert_eq!(
                CargoValidationService::calculate_remaining_capacity(&player),
                50
            );
        }

        #[test]
        fn test_calculate_remaining_capacity_full() {
            let mut cargo = CargoHold::new(50);
            cargo.add_commodity(CommodityType::Water, 50).unwrap();
            let player = PlayerTradeView::new(1000, cargo);

            assert_eq!(
                CargoValidationService::calculate_remaining_capacity(&player),
                0
            );
        }
    }

    // ========================================================================
    // Trade Basket Validation Tests
    // ========================================================================

    mod trade_basket_validation {
        use super::*;

        fn create_player_with_cargo(
            money: u32,
            capacity: u32,
            initial_cargo: Vec<(CommodityType, u32)>,
        ) -> PlayerTradeView {
            let mut cargo = CargoHold::new(capacity);
            for (commodity, quantity) in initial_cargo {
                cargo.add_commodity(commodity, quantity).unwrap();
            }
            PlayerTradeView::new(money, cargo)
        }

        #[test]
        fn test_validate_single_buy_trade_success() {
            let player = create_player_with_cargo(1000, 100, vec![]);
            let trade = TradeRequest::buy(CommodityType::Water, 10, 20);

            let result = CargoValidationService::validate_trade_basket(&[trade], &player);
            assert!(result.is_ok());
        }

        #[test]
        fn test_validate_single_buy_trade_insufficient_cargo_space() {
            let player = create_player_with_cargo(1000, 10, vec![(CommodityType::Water, 8)]);
            let trade = TradeRequest::buy(CommodityType::Foodstuffs, 5, 20);

            let result = CargoValidationService::validate_trade_basket(&[trade], &player);
            assert!(result.is_err());
            match result.unwrap_err() {
                ValidationError::InsufficientCargoSpace {
                    requested,
                    available,
                } => {
                    assert_eq!(requested, 5);
                    assert_eq!(available, 2);
                }
                _ => panic!("Expected InsufficientCargoSpace error"),
            }
        }

        #[test]
        fn test_validate_single_buy_trade_insufficient_credits() {
            let player = create_player_with_cargo(100, 100, vec![]);
            let trade = TradeRequest::buy(CommodityType::Medicine, 10, 50); // Costs 500

            let result = CargoValidationService::validate_trade_basket(&[trade], &player);
            assert!(result.is_err());
            match result.unwrap_err() {
                ValidationError::InsufficientCredits {
                    required,
                    available,
                } => {
                    assert_eq!(required, 500);
                    assert_eq!(available, 100);
                }
                _ => panic!("Expected InsufficientCredits error"),
            }
        }

        #[test]
        fn test_validate_single_sell_trade_success() {
            let player = create_player_with_cargo(
                1000,
                100,
                vec![(CommodityType::Water, 20), (CommodityType::Foodstuffs, 10)],
            );
            let trade = TradeRequest::sell(CommodityType::Water, 10, 15);

            let result = CargoValidationService::validate_trade_basket(&[trade], &player);
            assert!(result.is_ok());
        }

        #[test]
        fn test_validate_single_sell_trade_insufficient_inventory() {
            let player = create_player_with_cargo(1000, 100, vec![(CommodityType::Water, 5)]);
            let trade = TradeRequest::sell(CommodityType::Water, 10, 15);

            let result = CargoValidationService::validate_trade_basket(&[trade], &player);
            assert!(result.is_err());
            match result.unwrap_err() {
                ValidationError::InsufficientInventory {
                    commodity,
                    requested,
                    available,
                } => {
                    assert_eq!(commodity, CommodityType::Water);
                    assert_eq!(requested, 10);
                    assert_eq!(available, 5);
                }
                _ => panic!("Expected InsufficientInventory error"),
            }
        }

        #[test]
        fn test_validate_single_sell_trade_commodity_not_owned() {
            let player = create_player_with_cargo(1000, 100, vec![]);
            let trade = TradeRequest::sell(CommodityType::Medicine, 5, 100);

            let result = CargoValidationService::validate_trade_basket(&[trade], &player);
            assert!(result.is_err());
            match result.unwrap_err() {
                ValidationError::InsufficientInventory {
                    commodity,
                    requested,
                    available,
                } => {
                    assert_eq!(commodity, CommodityType::Medicine);
                    assert_eq!(requested, 5);
                    assert_eq!(available, 0);
                }
                _ => panic!("Expected InsufficientInventory error"),
            }
        }

        #[test]
        fn test_validate_mixed_basket_success() {
            // Player has 100 credits, 100 capacity, 20 Water
            let player = create_player_with_cargo(100, 100, vec![(CommodityType::Water, 20)]);

            // Sell 10 Water @ 15 = +150 credits, -10 cargo
            // Buy 15 Foodstuffs @ 5 = -75 credits, +15 cargo
            // Net: +75 credits, +5 cargo
            let trades = vec![
                TradeRequest::sell(CommodityType::Water, 10, 15),
                TradeRequest::buy(CommodityType::Foodstuffs, 15, 5),
            ];

            let result = CargoValidationService::validate_trade_basket(&trades, &player);
            assert!(result.is_ok());
        }

        #[test]
        fn test_validate_mixed_basket_combined_cargo_exceeds() {
            // Player has 100 credits, 20 capacity, 10 Water
            let player = create_player_with_cargo(100, 20, vec![(CommodityType::Water, 10)]);

            // Buy 15 Foodstuffs = +15 cargo (total would be 25 > 20)
            let trades = vec![TradeRequest::buy(CommodityType::Foodstuffs, 15, 5)];

            let result = CargoValidationService::validate_trade_basket(&trades, &player);
            assert!(result.is_err());
            match result.unwrap_err() {
                ValidationError::InsufficientCargoSpace { .. } => {}
                _ => panic!("Expected InsufficientCargoSpace error"),
            }
        }

        #[test]
        fn test_validate_mixed_basket_sell_first_then_buy() {
            // Player has 50 credits, 20 capacity, 10 Water
            let player = create_player_with_cargo(50, 20, vec![(CommodityType::Water, 10)]);

            // Sell 10 Water @ 10 = +100 credits, -10 cargo (now 0 cargo, 150 credits)
            // Buy 15 Foodstuffs @ 5 = -75 credits, +15 cargo (now 15 cargo, 75 credits)
            // Net: +5 cargo (from 10 to 15), which is within 20 capacity
            let trades = vec![
                TradeRequest::sell(CommodityType::Water, 10, 10),
                TradeRequest::buy(CommodityType::Foodstuffs, 15, 5),
            ];

            let result = CargoValidationService::validate_trade_basket(&trades, &player);
            // Net cargo change: -10 + 15 = +5, starting from 10 = 15 <= 20 (OK)
            // Net credit change: +100 - 75 = +25, starting from 50 = 75 (OK, no credit check needed)
            if result.is_err() {
                eprintln!("Error: {:?}", result.as_ref().unwrap_err());
            }
            assert!(result.is_ok());
        }

        #[test]
        fn test_validate_basket_invalid_zero_quantity() {
            let player = create_player_with_cargo(1000, 100, vec![]);
            let trade = TradeRequest::buy(CommodityType::Water, 0, 10);

            let result = CargoValidationService::validate_trade_basket(&[trade], &player);
            assert!(result.is_err());
            match result.unwrap_err() {
                ValidationError::InvalidTrade { reason } => {
                    assert!(reason.contains("quantity"));
                }
                _ => panic!("Expected InvalidTrade error"),
            }
        }

        #[test]
        fn test_validate_basket_invalid_zero_price() {
            let player = create_player_with_cargo(1000, 100, vec![]);
            let trade = TradeRequest::buy(CommodityType::Water, 10, 0);

            let result = CargoValidationService::validate_trade_basket(&[trade], &player);
            assert!(result.is_err());
            match result.unwrap_err() {
                ValidationError::InvalidTrade { reason } => {
                    assert!(reason.contains("price"));
                }
                _ => panic!("Expected InvalidTrade error"),
            }
        }

        #[test]
        fn test_validate_basket_multiple_buys_exceeds_credits() {
            // Player has 100 credits
            let player = create_player_with_cargo(100, 100, vec![]);

            // Buy 5 Water @ 10 = 50 credits
            // Buy 5 Foodstuffs @ 10 = 50 credits
            // Total: 100 credits (should succeed)
            let trades = vec![
                TradeRequest::buy(CommodityType::Water, 5, 10),
                TradeRequest::buy(CommodityType::Foodstuffs, 5, 10),
            ];

            let result = CargoValidationService::validate_trade_basket(&trades, &player);
            assert!(result.is_ok());

            // Add one more buy that exceeds credits
            let trades = vec![
                TradeRequest::buy(CommodityType::Water, 5, 10),
                TradeRequest::buy(CommodityType::Foodstuffs, 5, 10),
                TradeRequest::buy(CommodityType::Medicine, 1, 10),
            ];

            let result = CargoValidationService::validate_trade_basket(&trades, &player);
            assert!(result.is_err());
            match result.unwrap_err() {
                ValidationError::InsufficientCredits {
                    required,
                    available,
                } => {
                    assert_eq!(required, 110);
                    assert_eq!(available, 100);
                }
                _ => panic!("Expected InsufficientCredits error"),
            }
        }

        #[test]
        fn test_validate_basket_at_exact_capacity_and_credits() {
            // Player has exactly enough for one full trade
            let player = create_player_with_cargo(100, 10, vec![]);

            // Buy 10 Water @ 10 = 100 credits, 10 cargo (exact match)
            let trade = TradeRequest::buy(CommodityType::Water, 10, 10);

            let result = CargoValidationService::validate_trade_basket(&[trade], &player);
            assert!(result.is_ok());
        }
    }

    // ========================================================================
    // ValidationError Display Tests
    // ========================================================================

    mod validation_error_display {
        use super::*;

        #[test]
        fn test_display_insufficient_cargo_space() {
            let error = ValidationError::InsufficientCargoSpace {
                requested: 50,
                available: 30,
            };
            let msg = format!("{}", error);
            assert!(msg.contains("50"));
            assert!(msg.contains("30"));
            assert!(msg.contains("cargo space"));
        }

        #[test]
        fn test_display_insufficient_inventory() {
            let error = ValidationError::InsufficientInventory {
                commodity: CommodityType::Water,
                requested: 20,
                available: 10,
            };
            let msg = format!("{}", error);
            assert!(msg.contains("Water"));
            assert!(msg.contains("20"));
            assert!(msg.contains("10"));
            assert!(msg.contains("inventory"));
        }

        #[test]
        fn test_display_insufficient_credits() {
            let error = ValidationError::InsufficientCredits {
                required: 500,
                available: 300,
            };
            let msg = format!("{}", error);
            assert!(msg.contains("500"));
            assert!(msg.contains("300"));
            assert!(msg.contains("credits"));
        }

        #[test]
        fn test_display_invalid_trade() {
            let error = ValidationError::InvalidTrade {
                reason: "Quantity must be positive".to_string(),
            };
            let msg = format!("{}", error);
            assert!(msg.contains("Invalid trade"));
            assert!(msg.contains("Quantity must be positive"));
        }
    }

    // ========================================================================
    // Integration Tests with CargoHold
    // ========================================================================

    mod integration_with_cargo_hold {
        use super::*;

        #[test]
        fn test_validation_with_actual_cargo_hold_operations() {
            // Create a cargo hold and perform operations that should validate correctly
            let mut cargo = CargoHold::new(50);

            // Add some initial cargo
            cargo.add_commodity(CommodityType::Water, 20).unwrap();
            cargo.add_commodity(CommodityType::Foodstuffs, 10).unwrap();

            let player = PlayerTradeView::new(500, cargo);

            // Validate we can add more (should have 20 space remaining)
            assert!(CargoValidationService::can_add_cargo(30, 50, 20));
            assert!(!CargoValidationService::can_add_cargo(30, 50, 21));

            // Validate we can remove cargo we have
            assert!(CargoValidationService::can_remove_cargo(
                &player.cargo.commodities,
                &CommodityType::Water,
                15
            ));

            // Validate we cannot remove more than we have
            assert!(!CargoValidationService::can_remove_cargo(
                &player.cargo.commodities,
                &CommodityType::Water,
                25
            ));

            // Validate remaining capacity calculation
            assert_eq!(
                CargoValidationService::calculate_remaining_capacity(&player),
                20
            );
        }

        #[test]
        fn test_validation_prevents_invalid_state() {
            let mut cargo = CargoHold::new(30);
            cargo.add_commodity(CommodityType::Water, 25).unwrap();

            let player = PlayerTradeView::new(200, cargo);

            // Try to buy 10 more units (would exceed capacity of 30)
            let buy_trade = TradeRequest::buy(CommodityType::Foodstuffs, 10, 5);
            let result = CargoValidationService::validate_trade_basket(&[buy_trade], &player);

            // Should fail validation
            assert!(result.is_err());

            // Cargo hold should still have only 25 units (unchanged)
            assert_eq!(player.cargo.current_load(), 25);
        }

        #[test]
        fn test_validation_allows_valid_state() {
            let mut cargo = CargoHold::new(50);
            cargo.add_commodity(CommodityType::Water, 20).unwrap();

            let player = PlayerTradeView::new(500, cargo);

            // Sell water, buy foodstuffs - valid operation
            let trades = vec![
                TradeRequest::sell(CommodityType::Water, 10, 15),
                TradeRequest::buy(CommodityType::Foodstuffs, 15, 10),
            ];

            let result = CargoValidationService::validate_trade_basket(&trades, &player);
            assert!(result.is_ok());
        }
    }
}

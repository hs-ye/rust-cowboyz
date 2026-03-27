## 📋 Assignment: software-engineer

**Priority**: High - Part of Epic #131 (Cargo Hold Management)

### 📖 Implementation Plan Reference
- **ADR #0008**: Cargo Hold Management - Capacity Constraints section
- **ADR #0006**: Data Models - CargoHold structure
- **ADR #0005**: Commodities - All commodities take 1 cargo unit

### ✅ Acceptance Criteria
- [x] CargoValidationService created with all validation methods
- [x] ValidationError enum covers all error cases
- [x] Unit tests for capacity validation
- [x] Unit tests for inventory validation
- [x] Unit tests for combined trade basket validation
- [x] Integration with existing CargoHold struct verified
- [x] All cargo tests pass with cargo test

### 🔗 Dependencies
- **Blocks**: #133 (Trading Transaction Service), #135 (Cargo Status Panel)
- **Blocked by**: None (CargoHold already exists)

### ⚙️ Technical Considerations
- Create module: src/player/cargo_validation.rs
- Starting capacity: 20 units (MVP default)
- Validation must be atomic - either all trades pass or none do
- Service should be stateless, taking player state as input

### 📁 Files to Create/Modify
- **Create**: src/player/cargo_validation.rs
- **Reference**: src/player/inventory.rs (CargoHold)
- **Reference**: src/simulation/commodity.rs (CommodityInventory)
- **Reference**: src/game_state.rs (Player struct)

---

## Implementation Summary

**Status**: ✅ COMPLETE - Ready for Review

**Branch**: `feat/132-cargo-hold-validation`

### What Was Implemented

1. **Created `src/player/cargo_validation.rs`** with:
   - `CargoValidationService` - Stateless validation service
   - `ValidationError` enum with 4 error variants:
     - `InsufficientCargoSpace { requested, available }`
     - `InsufficientInventory { commodity, requested, available }`
     - `InsufficientCredits { required, available }`
     - `InvalidTrade { reason }`
   - `TradeRequest` struct for representing buy/sell operations
   - `PlayerTradeView` struct for player state in validation

2. **Validation Methods**:
   - `can_add_cargo(current_load, capacity, quantity) -> bool`
   - `can_remove_cargo(inventory, commodity, quantity) -> bool`
   - `validate_trade_basket(trades, player) -> Result<(), ValidationError>`
   - `calculate_remaining_capacity(player) -> u32`

3. **Updated `src/player/mod.rs`** to export the new module

4. **Comprehensive Test Suite** (36 unit tests + 4 doc tests):
   - Capacity validation: 7 tests (under/at/over capacity, edge cases)
   - Inventory validation: 6 tests (partial/exact/insufficient amounts)
   - Remaining capacity: 3 tests
   - Trade basket validation: 13 tests (single trades, mixed baskets, error cases)
   - ValidationError display: 4 tests
   - Integration with CargoHold: 3 tests

### Test Results
```
test result: ok. 36 passed; 0 failed; 0 ignored
Doc-tests: ok. 4 passed; 0 failed
Full suite: ok. 201 passed; 0 failed
```

### Key Design Decisions

- **Basket Validation**: The `validate_trade_basket` method validates trades atomically, allowing sells to free up cargo space and credits for buys in the same basket
- **Stateless Design**: All validation methods are stateless, taking player state as input parameters
- **Overflow Protection**: Uses `saturating_add` to prevent integer overflow in capacity calculations

---
**Status**: Ready for tech-lead review. PR to be raised.

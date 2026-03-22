# 0010: Cargo Hold Management and Capacity Constraints

## Status
Proposed

## Date
2026-03-21

## Deciders
- User
- game-designer

## Context
Cargo management is a critical constraint in the trading gameplay loop. Per ADR #0005 (Market/Economy System), "all commodities take up a single unit of cargo space (simplifying assumption)" and "Cargo Limits: Ship capacity creates meaningful decisions about which goods to transport." ADR #0006 (Data Models/Schema) defines `Ship.cargo_capacity` and `Player.cargo_inventory` as core data structures.

Players need to:
- Understand their current cargo status at a glance
- Make meaningful decisions about what to buy/sell given capacity limits
- Navigate cargo constraints without frustration
- Plan trade routes based on available space

Current design gap: While cargo capacity is mentioned in ADRs #0005 and #0006, the specific mechanics of cargo management, capacity visualization, constraint handling, and player interaction with cargo hold are not defined.

## Decision
We will implement a cargo hold management system with the following mechanics and UI:

### Game Design Objectives
- Make cargo capacity a meaningful strategic constraint without being frustrating
- Provide clear, at-a-glance cargo status information
- Enable quick cargo management decisions during trading
- Support strategic planning for multi-planet trade routes
- Maintain simplicity for MVP while allowing future depth

### Cargo Capacity Fundamentals

#### Basic Mechanics
- **Unit-Based System**: All commodities occupy exactly 1 cargo unit each (per ADR #0005)
- **Ship Capacity**: Base cargo capacity defined in ship data model (ADR #0006)
  - Starting ship: 20 cargo units (MVP default)
  - Upgradable via ship equipment system (future enhancement)
- **No Stacking**: Each unit is tracked individually for simplicity
- **No Commodity Restrictions**: Any commodity can be stored in any ship (no special holds)

#### Cargo State Representation
```
Cargo State = {
  current_used: integer (0 to capacity),
  capacity: integer (fixed per ship),
  inventory: {
    commodity_id: quantity,
    ...
  }
}
```

### Cargo UI Display

#### 1. Cargo Status Panel (Always Visible)
Located in the ship status section of the information panel (per ADR #0003):

**Visual Elements:**
- **Capacity Bar**: Horizontal progress bar showing used/total capacity
  - Green zone: 0-50% full (plenty of space)
  - Yellow zone: 51-80% full (getting tight)
  - Red zone: 81-100% full (critical/near full)
  - Numeric overlay: "35/50 units"
- **Expandable Detail View**: Click to expand full cargo manifest

**Capacity Bar States:**
- **Empty (0%)**: Grey bar, "Cargo Empty" label
- **Partial (1-99%)**: Colored bar based on fill percentage
- **Full (100%)**: Pulsing red bar, "CARGO FULL" warning label

#### 2. Cargo Manifest (Expanded View)
When player clicks cargo panel, shows detailed breakdown:

**Manifest Contents:**
- **Commodity List**: Each commodity with quantity > 0
  - Icon + Name + Quantity + Current Value (at current planet)
  - Example: "💊 Medicine × 8 (¢1,160 at current prices)"
- **Total Value**: Sum of all cargo at current planet's sell prices
- **Empty Space**: "15 units empty" or "Cargo Full"
- **Actions**: 
  - [Sort by Quantity] - Arrange by amount held
  - [Sort by Value] - Arrange by total value
  - [Dump Cargo] - Emergency jettison (future enhancement, with penalty)

#### 3. Market Panel Integration
In the market trading panel (ADR #0009), cargo status is shown per-commodity:
- "You own: 8 units" displayed on each row
- Visual indicator if selling would free up space
- Visual indicator if buying would exceed capacity

### Cargo Capacity Constraints

#### Buying Constraints
**Hard Limit:** Cannot buy more than remaining cargo space
- Formula: `max_buy_quantity = MIN(market_available, ship_capacity - current_used, player_credits / unit_price)`
- UI Enforcement: Quantity input capped at max_buy_quantity
- Error Prevention: BUY button disabled if max_buy_quantity = 0

**Soft Warning:** Buying to >80% capacity shows warning
- Modal displays: "Warning: This will fill your cargo to 90% capacity"
- Player must confirm understanding (checkbox: "I understand this will fill my cargo")
- Can be dismissed for session (don't show again this planet visit)

#### Selling Constraints
**Ownership Limit:** Cannot sell more than owned
- Formula: `max_sell_quantity = player_inventory[commodity_id]`
- UI Enforcement: Quantity input capped at max_sell_quantity
- Error Prevention: SELL button disabled if quantity = 0

**No Negative Inventory:** Atomic transactions prevent overselling
- Validation check before transaction commit
- Rollback with error if race condition detected

#### Full Cargo Behavior
When cargo reaches 100% capacity:
- All BUY buttons automatically disabled
- Tooltip on disabled BUY: "Cargo full - sell commodities to free up space"
- Visual indicator on cargo panel: "CARGO FULL" with red highlight
- SELL buttons remain enabled
- "Emergency Dump" option appears (if implemented)

### Cargo Management Interactions

#### 1. Quick Sell from Cargo Manifest
From expanded cargo manifest, players can:
- Click commodity row to quick-sell at current planet
- Opens sell modal pre-filled with commodity data
- Same flow as market panel sell (ADR #0009)
- Useful when market panel is scrolled away from desired commodity

#### 2. Cargo Comparison (Future Enhancement)
When considering a purchase:
- Show "Opportunity Cost" of cargo space used
- Example: "Buying 10 Firearms (¢1,500) uses space that could hold 10 Medicine (¢2,000)"
- Requires price data from destination planets
- Deferred to post-MVP

#### 3. Cargo Sorting and Filtering
**MVP:** Simple list sorted by quantity (descending)

**Future Enhancement:**
- Sort by: Quantity, Value, Base Price, Profit Margin
- Filter by: Commodity type, Profitable at current planet, etc.
- Search by commodity name

### Transaction State Updates

#### Atomic Transaction Flow
```
1. Player confirms trade
2. System validates:
   - Cargo space available (for buy)
   - Inventory owned (for sell)
   - Credits available (for buy)
   - Market supply available (for buy)
3. If validation passes:
   - Update Player.credits
   - Update Player.cargo_inventory
   - Update Market.supply_factor or demand_factor
   - Persist to localStorage
   - Update UI
4. If validation fails:
   - Rollback any partial changes
   - Display specific error message
   - Return to modal for adjustment
```

#### State Persistence
Per ADR #0006, cargo state is persisted in browser localStorage:
- Saved after every transaction
- Saved on planet departure
- Saved on turn end
- Restored on page reload
- Version-checked for migration compatibility

### Error Scenarios and Handling

#### Insufficient Cargo Space
**Scenario:** Player tries to buy more than ship can hold

**Prevention:**
- BUY button disabled if cargo full
- Quantity input capped at remaining space
- Real-time calculation as quantity changes

**Error Message (if race condition):**
"Cargo capacity exceeded. You have X units free, tried to buy Y units."

**Resolution:**
- Auto-adjust quantity to maximum available space
- Player can confirm reduced quantity or cancel

#### Inventory Desync
**Scenario:** UI shows different quantity than actual state (rare, bug condition)

**Detection:**
- Validation check on transaction commit
- Compare expected vs actual inventory

**Resolution:**
- Refresh UI from authoritative state
- Display: "Cargo data updated. Please review and try again."
- Log error for debugging

#### Transaction Interrupted
**Scenario:** Page reload/close during transaction

**Prevention:**
- Transaction is atomic (all-or-nothing)
- State persisted before UI update

**Recovery:**
- On reload, state reflects last committed transaction
- If transaction was mid-commit, rollback to pre-transaction state
- Display: "Previous transaction may not have completed. Cargo restored to last known state."

### Strategic Depth Considerations

#### Meaningful Choices
Cargo capacity creates strategic decisions:
- **Diversification vs Specialization**: Many commodities (lower risk) vs few commodities (higher risk/reward)
- **Space Allocation**: How much space for high-value vs low-value goods
- **Route Planning**: Buy goods that fit capacity for intended route
- **Opportunity Cost**: Space used by commodity A cannot hold commodity B

#### Difficulty Balancing
**Starting Capacity (20 units):**
- Enough for meaningful early trades
- Constrains new players gently
- Encourages ship upgrade pursuit

**Capacity Upgrades (Future):**
- Increase capacity via ship equipment purchases
- Cost scales with capacity increase
- Creates progression goal for players

#### Risk/Reward Mechanics
**High-Value Commodities (Medicine, Electronics, Artefacts):**
- Worth using limited cargo space
- But may have limited markets
- Risk: Arrive at destination, no demand

**Low-Value Commodities (Water, Foodstuffs):**
- Use same cargo space but lower profit
- But consistent demand across many planets
- Risk: Low margins, need volume

### MVP Simplicity Decisions

**What's Included (MVP):**
- Single cargo hold (no compartments)
- All commodities treated identically (1 unit each)
- No cargo degradation or spoilage
- No theft or cargo loss mechanics
- Simple capacity bar visualization
- Basic manifest with quantity and value

**What's Deferred (Future Enhancement):**
- Multiple cargo holds (secured, refrigerated, etc.)
- Commodity-specific storage requirements
- Cargo insurance against loss/theft
- Perishable goods with time limits
- Cargo scanning (see what other traders carry)
- Cargo jettison in emergencies
- Smuggling hidden compartments

### Integration with Other Systems

#### Market System (ADR #0005)
- Cargo limits affect supply/demand calculations
- Large player purchases deplete local supply
- Large player sales increase local supply
- Price slippage for very large trades (future)

#### Ship System (ADR #0006, #0012, #0013)
- Base cargo capacity from ship type
- Upgrade slots can increase capacity
- Different ships have different base capacities

#### Turn System
- Cargo state persists between turns
- No cargo changes during transit (no random events affecting cargo in MVP)
- Cargo value fluctuates with market prices each turn

#### Save/Load System (ADR #0006)
- Cargo state included in game state serialization
- Validated on load for data integrity
- Migration support for schema changes

## Consequences

### Positive
- Clear cargo visualization supports strategic decision-making
- Hard constraints create meaningful gameplay choices
- Simple unit-based system easy to understand
- Integration with market system creates emergent complexity
- Upgrade path provides progression goal
- Error handling prevents frustrating loss of progress

### Negative
- Uniform cargo size reduces commodity differentiation
- No spoilage/perishables reduces realism and urgency
- Capacity constraints may frustrate new players
- Limited cargo management depth may feel shallow for experienced players
- No cargo loss mechanics reduces risk element

## References
- [Market/Economy System ADR #0005](./0005-market-economy-system.md)
- [Data Models/Schema ADR #0006](./0006-data-models-schema.md)
- [Ship Types System ADR #0012](./0012-ship-types-system.md)
- [Ship Types Technical Implementation ADR #0013](./0013-ship-types-technical-implementation.md)
- [Web UI View ADR #0003](./0003-web-ui-view.md)
- [Project Principles ADR #0000](./0000-project-principles.md)

# 0008: Cargo Hold Management and Capacity Constraints

## Status
Accepted

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

**Capacity Bar States:**
- **Empty (0%)**: Grey bar, "Cargo Empty" label
- **Partial (1-99%)**: Colored bar based on fill percentage
- **Full (100%)**: Red bar, "CARGO FULL" warning label

#### 2. Cargo Manifest (Expanded View)

This will be a future enhancement. In MVP players should look at the market panel for the sliders as per ADR #0007 for an overview of what they are carrying

#### 3. Market Panel Integration
In the market trading panel (ADR #0007), cargo status is shown per-commodity:
- Cargo slider displayed on each row
- Visual indicator on cargo bar if selling would free up space / buying would take up more capacity

#### 4. Cargo upgrades
If ship cargo upgrades are purchased by the player, will need to re-fresh / re-calculate Cargo status panel and the Market Panel integrations.

Exact mechanics will be addressed by future ADR

### Cargo Capacity Constraints

#### Buying Constraints
**Hard Limit:** Cannot buy more than remaining cargo space - if the player has selected more than their cargo space, 'trade' button is disabled / greyed out.
- There should be a check on the 'trade' logic anyway, in case some edge case where a user has set their sliders but somehow the button is not disabled and clicked.

#### Selling Constraints
**Ownership Limit:** Cannot sell more than owned
- UI Enforcement: Cannot slide commodity slider < 0
- Error Prevention: trade button checks if executing the trade would reuslt in quantity < 0

**No Negative Inventory:** Atomic transactions prevent overselling
- Validation check before transaction commit


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

**Capacity Upgrades (Future ADR):**
- Increase capacity via ship equipment purchases
- Cost scales with capacity increase
- Creates progression goal for players, enables more profit per turn

#### Risk/Reward Mechanics
**High-Value Commodities (Medicine, Electronics, Artefacts):**
- Worth using limited cargo space
- But may have limited markets
- Risk: Arrive at destination, no demand

**Low-Value Commodities (Water, Foodstuffs):**
- Use same cargo space but lower profit
- But consistent demand across many planets
- Risk: Low margins, need volume


### Integration with Other Systems

#### Market System (ADR #0005, #0007)
- Cargo limits affect supply/demand calculations
- Cargo display integrated into market panels
- Large player purchases deplete local supply
- Large player sales increase local supply
- Price slippage for very large trades (future)

#### Ship System (ADR #0006, future ADRs)
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
- No spoilage/perishables reduces realism and urgency (future features)
- Limited cargo management depth may feel shallow for experienced players
- Need upgrades progression/variety in ship types - to be addressed by future update

## References
- [Market/Trading UI ADR #0007](./0007-market-trading-ui-interaction)
- [Market/Economy System ADR #0005](./0005-market-economy-system.md)
- [Data Models/Schema ADR #0006](./0006-data-models-schema.md)
- [Web UI View ADR #0003](./0003-web-ui-view.md)
- [Project Principles ADR #0000](./0000-project-principles.md)

# 0009: Market Trading UI Interaction for Buy/Sell Mechanics

## Status
Proposed

## Date
2026-03-21

## Deciders
- User
- game-designer

## Context
Players need to interact with planetary markets to buy and sell commodities as the core gameplay loop. Following ADR #0005 (Market/Economy System), each planet has dynamic pricing for commodities based on supply/demand. ADR #0006 (Data Models/Schema) defines player inventory, ship cargo capacity, and market data structures. ADR #0003 (Web UI View) specifies a market panel showing buy/sell prices with interaction capabilities.

The trading interaction must be:
- Simple and intuitive for MVP (per ADR #0000 project principles)
- Integrated with the dynamic pricing system from ADR #0005
- Respect cargo capacity limits from ADR #0006
- Work within the turn-based game structure
- Accessible without complex menus or tutorials

Current design gap: While ADR #0003 mentions "interaction on each panel for the relevant action (e.g. buy/sell for market)", the specific UI interaction flow, quantity selection, confirmation mechanics, and error handling are not defined.

## Decision
We will implement a streamlined market trading UI with the following interaction patterns:

### Game Design Objectives
- Enable quick, intuitive commodity trading as the core gameplay action
- Minimize clicks and menu navigation for common trading actions
- Provide clear feedback on trade outcomes and constraints
- Support strategic decision-making with accessible market information
- Maintain turn-based pacing without breaking game flow

### Trading UI Interaction Flow

#### 1. Market Panel Display
When player is landed at a planet, the market panel (right side of screen per ADR #0003) displays:
- **Commodity List**: All 10 commodities from ADR #0005 in a scrollable table
- **Current Price**: Display current buy/sell price at this planet
- **Base Price Reference**: Show base price in parentheses for comparison (e.g., "¢145 (¢100)")
- **Price Trend Indicator**: Simple arrow icon (↑↓) showing if price is above/below base
- **Player Holdings**: Quantity of each commodity currently in cargo
- **Available Actions**: Buy/Sell buttons for each commodity row

#### 2. Buy/Sell Button Interaction
Each commodity row has two buttons:
- **[BUY]** button - Green themed, enabled when player has credits
- **[SELL]** button - Orange themed, enabled when player has that commodity in cargo

**Button States:**
- **Enabled**: Player meets basic requirements (credits for buy, inventory for sell)
- **Disabled**: Player lacks requirements, with tooltip explaining why
- **Hidden**: Commodity not traded at this planet type (per ADR #0005 "Ignores" list)

#### 3. Quantity Selection Modal
Clicking BUY or SELL opens a lightweight modal overlay:

**Modal Contents:**
- **Commodity Name & Icon**: Clear identification
- **Current Price**: "Price: ¢145 per unit"
- **Max Available/Space**: 
  - For BUY: "Available: 50 units | Your cargo space: 15 units"
  - For SELL: "Your cargo: 20 units | Market demand: unlimited"
- **Quantity Input**: 
  - Numeric input field with +/- stepper buttons
  - Quick-select buttons: [25%] [50%] [75%] [MAX]
  - Keyboard input support for direct number entry
- **Live Calculation Display**:
  - "Quantity: 10 units"
  - "Total: ¢1,450"
  - "Remaining cargo: 5/50 units" (for buy) or "Remaining: 10 units" (for sell)
  - "Credits after: ¢3,550" (for buy) or "Credits after: ¢6,450" (for sell)
- **Action Buttons**: [Confirm Trade] [Cancel]

#### 4. Confirmation and Execution
**Confirm Trade button triggers:**
1. **Validation Check**: Verify constraints still met (race condition protection)
2. **Transaction Processing**: 
   - Update player credits
   - Update cargo inventory
   - Update market supply/demand (per ADR #0005 player impact mechanics)
3. **Visual Feedback**: 
   - Brief animation on commodity row
   - Toast notification: "Bought 10 Firearms for ¢1,450"
   - Update all affected UI panels (cargo, credits, market)
4. **Modal Close**: Return to market panel

**Cancel button:**
- Closes modal with no changes
- No confirmation needed for cancellation

#### 5. Quick Trade Shortcuts
For experienced players, implement keyboard shortcuts:
- **Click + Shift**: Buy/Sell max available in one action (with confirmation)
- **Double-click**: Buy/Sell 1 unit instantly (no modal)
- **Number keys 1-9**: When modal open, select preset quantities

### Price Calculation at Point of Trade

**Integration with ADR #0005 Dynamic Pricing:**
- Price is calculated at moment of trade confirmation (not modal open)
- Formula: `Current Price = Base Price × Local Multiplier × Supply Factor × Demand Factor`
- Price displayed in modal is "locked" for 3 seconds after modal opens
- If modal stays open longer, show countdown timer and refresh price
- Large trades (>10 units) may trigger price slippage (future enhancement)

### Cargo Capacity Validation

**Pre-Trade Validation (Button Enable/Disable):**
- BUY button disabled if: cargo is full OR player lacks credits for 1 unit
- SELL button disabled if: player has 0 units of commodity

**During Trade Validation (Modal):**
- Quantity input capped at: `MIN(available_market_units, remaining_cargo_space)` for BUY
- Quantity input capped at: `player_inventory_units` for SELL
- Real-time validation feedback if player manually enters invalid quantity

**Post-Trade Validation:**
- Atomic transaction: all updates succeed or all fail
- Rollback with error message if validation fails on confirm
- Error messages are specific:
  - "Insufficient cargo space. Need 5 more units."
  - "Insufficient credits. Need ¢500 more."
  - "Market supply depleted. Only 3 units available."

### Error Handling

**Error Types and Player Feedback:**

| Error | When | Player Message | UI Response |
|-------|------|----------------|-------------|
| Insufficient Credits | Buy costs more than player has | "Insufficient credits. Need ¢X more." | Disable BUY, show credit balance in red |
| Insufficient Cargo Space | Buy exceeds ship capacity | "Insufficient cargo space. Need X more units." | Cap quantity at max space, show warning |
| Market Out of Stock | Trying to buy more than available | "Only X units available at this market." | Cap quantity at available, show "Limited supply" badge |
| No Inventory | Trying to sell commodity not owned | Button disabled | SELL button disabled with tooltip "You don't have this commodity" |
| Price Changed | Price updated while modal open | "Price updated: ¢X → ¢Y" | Refresh display, require re-confirmation |
| Not Landed at Planet | Trying to trade while in transit | "Cannot trade while in transit. Land at a planet first." | Disable entire market panel |

### Transaction Recording

**Per ADR #0006 Data Model Integration:**
- Update `Player.credits` immediately
- Update `Player.cargo_inventory[commodity_id]` immediately
- Update `Market.supply_factor` or `Market.demand_factor` based on trade direction
- Record transaction in `Transaction.history` (if implemented per ADR #0006)
- Trigger state persistence to localStorage

**Trade Log (Future Enhancement):**
- Maintain list of last 10 trades in UI
- Show commodity, quantity, price, location, turn number
- Accessible via expandable panel

### Turn-Based Integration

**Trading and Turn Progression:**
- Trading does NOT consume a turn (exploration/action phase)
- Unlimited trades allowed per planet visit
- Turn only advances on "Depart Planet" action
- Market prices may change between turns (per ADR #0005 fluctuations)
- Visual indicator if prices changed since last turn: "Prices updated!" badge

### Accessibility Considerations

**Visual Accessibility:**
- High contrast buttons with clear labels
- Color-blind friendly indicators (icons + color)
- Tooltips on all interactive elements
- Minimum button size 44x44px for touch targets

**Interaction Accessibility:**
- Full keyboard navigation support
- Screen reader compatible labels
- No time pressure on decisions (turn-based)
- Undo last trade option (single trade reversal within same planet visit)

### MVP Simplicity Decisions

**What's Included (MVP):**
- Single-commodity trades only (no bulk multi-commodity)
- Manual quantity selection (no auto-optimize)
- Current prices only (no price history graphs)
- Basic validation (no advanced market analysis)

**What's Deferred (Future Enhancement):**
- Multi-commodity bulk trading
- Price trend graphs and analytics
- Automated trading suggestions
- Futures contracts or pre-orders
- Haggling/negotiation mechanics
- Black market trading (illegal commodities)

## Consequences

### Positive
- Streamlined trading flow supports core gameplay loop
- Clear visual feedback reduces player confusion
- Quantity presets speed up common actions
- Validation prevents frustrating errors
- Turn-based integration maintains game pacing
- Accessibility features broaden player base

### Negative
- Manual quantity selection may feel tedious for large trades
- No bulk trading may slow down experienced players
- Price locking mechanism adds complexity
- Modal-based interaction interrupts visual flow
- Limited market information may frustrate strategic players

## References
- [Market/Economy System ADR #0005](./0005-market-economy-system.md)
- [Data Models/Schema ADR #0006](./0006-data-models-schema.md)
- [Web UI View ADR #0003](./0003-web-ui-view.md)
- [Project Principles ADR #0000](./0000-project-principles.md)

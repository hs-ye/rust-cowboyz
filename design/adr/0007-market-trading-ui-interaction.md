# 0007: Market Trading UI Interaction for Buy/Sell Mechanics

## Status
Accepted

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
- **Available Actions**: See buy/sell interaction below

#### 2. Buy/Sell Interaction
To keep gameplay fast paced, this has to be intuitive and straightforward as possible and minimise friction for the user in buying/selling goods since this is somewhat of a 'chore' mechanic - players would already have thought of what to buy or sell when they were previously travelling from another planet. Upon landing, they want to make their trades as quickly as possible then move on to the next planet.

**Components**
- Each commodity row has A slider which goes between 0 to the maximum of the cargo hold space of the current ship 
- `trade button`: which executes the trades, a single button for the entire market
- `reset button`: resets slider positions to current state of the cargo hold
- Separately, add previews:
  - credits component: show a preview of the change in credits as a result of player actions
  - cargo hold: show a preview change to the total amount of cargo

**Player Interactions**
- Position of the slider shows the number of each good type currently in the ship's cargo hold, one slider for each commodity
- Players slide this left to sell, or right to buy more. 
- When the player operates the slider, the the 'money' credit display panel should show a preview of their actions. 
- Preview of the credit amount is Green with a `+` infront if player is gaining credits, red with `-` infront if losing credits. This should be the change in credits, 
- e.g. if a player is gaining 1000 from selling 1 unit worth 1000 it should say `+1000` in green
- If they do not have enough money to make that trade, disable `trade` button
- If they do not have enough total cargo to make the trade, also disable `trade` button
- Pressing 'reset' button causes all sliders to reset to the original value in the hold

Note that separate to this ADR which will be addressed by the Cargo Hold ADR #0008, increasing (or decreasing) the size of the cargo hold will also have an effect on the position of sliders which needs to be updated if the max number of cargo holds increases.

**Other**
Slider Disabled: tooltip explaining why. E.g. "commodity not traded at this planet" (per ADR #0005 "Ignores" list)

#### 3. Confirmation and Execution
**Trade button triggers:**
Upon pressing 'trade' button to execute the trade:
1. **Validation Check**: Verify constraints still met (race condition protection)
2. **Transaction Processing**: 
   - Update player credits
   - Update cargo inventory
   - Update market supply/demand (per ADR #0005 player impact mechanics)
3. **Visual Feedback**: 
   - Brief animation on commodity row
   - Toast notification summarising trade: "Bought 10 Firearms, 5 Foodstuffs for ¢1,450"
   - Update all affected UI panels (cargo, credits, market)

**Reset button:**
- Reset slider to original position to match 
- Remove credit/cargo hold space preview
- No confirmation needed for reset

### Price Calculation at Point of Trade

**Integration with ADR #0005 Dynamic Pricing:**
- Price is calculated at moment of trade confirmation
- Formula: `Current Price = Base Price × Local Multiplier × Supply Factor × Demand Factor`
- For simplicity, do not change the local price until the `trade` button is clicked. Prices update after the player has made their trade, regardless of the sign of the trade

#### Future enhancements
- May look into modelling market depth so that not all units are given the same price when bought/sold

### Cargo Capacity Validation

**Pre-Trade Validation (Button Enable/Disable):**
- Trade button disabled if: not enough cargo space, player lacks credits to make the trade

### Transaction Recording

**Per ADR #0006 Data Model Integration:**
- Update `Player.credits` immediately
- Update `Player.cargo_inventory[commodity_id]` immediately
- Update `Market.supply_factor` or `Market.demand_factor` based on trade direction
- Record transaction in `Transaction.history` (if implemented per ADR #0006)
- Trigger state persistence to localStorage

**Trade Log:**
- Modal pop-up, accessed via a `trade log` button underneath markets panel
- Maintain list of all trades in UI
- Show commodity, quantity, price, location, turn number
- Player and show/hide this at will. 

### Turn-Based Integration

**Trading and Turn Progression:**
- Trading does NOT consume a turn (exploration/action phase)
- Unlimited trades allowed per planet visit
- Turn only advances on "Depart Planet" action
- Market prices may change between turns (per ADR #0005 fluctuations)

### MVP Simplicity Decisions

**What's Deferred (Future Enhancement):**
- Price trend graphs and analytics
- Automated trading suggestions
- Futures contracts or pre-orders
- Haggling/negotiation mechanics
- Black market trading (illegal commodities) - assume all trades are 'legal', although some planet types wlil 'ignore' commodities as per ADR #0005

## Consequences

### Positive
- Streamlined trading flow supports core gameplay loop
- Slick quantity selection supports quick execution
- Clear visual feedback reduces player confusion
- Validation prevents frustrating errors
- Turn-based integration maintains game pacing

### Negative
- Not really realistic simulation of how trading works, a bit too abstract 'spreadsheet' style management
- Limited simulation of 'true' market mechanics
- Limited market information may frustrate strategic players

## References
- [Market/Economy System ADR #0005](./0005-market-economy-system.md)
- [Data Models/Schema ADR #0006](./0006-data-models-schema.md)
- [Web UI View ADR #0003](./0003-web-ui-view.md)
- [Project Principles ADR #0000](./0000-project-principles.md)

# 0005: Market/Economy System for Space-Western Trading Game

## Status
Accepted

## Date
2026-03-01

## Deciders
- User
- game-designer

## Context
The core gameplay mechanic of the space-western trading game revolves around trading commodities between different planets/stations. Players need meaningful economic decisions that create engaging gameplay loops. The market system must balance predictability (so players can plan routes and strategies) with volatility (to create risk/reward decisions and replayability), which is tied into the movement system of planets that create dynamic trade routes over time.

Traditional static pricing would make the game too predictable, while completely random prices would remove strategic depth. We need a semi-dynamic economy that simulates supply and demand while remaining comprehensible to players.

## Decision
We will implement a dynamic market economy system with the following components:

### Game Design Objectives
- Create meaningful trading decisions that require player analysis and route planning
- Balance risk and reward in commodity trading. Some commodities are high-risk, high reward, others are more stable but low margin. 
- Combined with the movement system, the game rewards players who identify short-trips with guaranteed profit margins, but also enable risky, long travel trades that may be more profitable
- Simulate realistic supply and demand mechanics within a space-western setting
- Add random shocks to randomise gameplay and create unpredictability to prevent stale gameplay on repeat playthroughs

### Market Structure
- **Planets/Stations**: Each location has unique economic characteristics, determined by an underlying Planet type (e.g. could be a station that orbits the sun, which has entirely different supply/demand patterns as appropriate)
- **Commodity**: Create a small number of commodities so as to not overwhelm the player for a small indie game. See List below
- **Base Prices**: Each commodity has a base price determined by planet type
- **Local Multipliers**: Planets modify base prices based on local supply/demand factors

#### Commodity types:
1. **Water** - Essential for survival, low value but high demand on dry worlds
2. **Foodstuffs** - Combination of grain, meat, and spices - staple nutrition sources
3. **Medicine** - Medical supplies, high value, essential for health
4. **Firearms** - Weapons for protection/offense, high value on dangerous worlds
5. **Ammunition** - Weapon accessories, consumed regularly, moderate value
6. **Metals** - Raw materials for construction and manufacturing
7. **Antimatter** - Advanced raw material for energy production, high value
8. **Electronics** - High-tech components, essential for advanced civilizations
9. **Narcotics** - Illegal substances, high value but risky to trade
10. **Alien Artefacts** - Rare and mysterious items from ancient civilizations, extremely high value and risky to trade

All commodities take up a single unit of cargo space (simplifying assumption).

#### Planet Types
A small fixed number of planet types (appropriate for small indie game), which have base supply/demanded commodities. Ignores are commodities that are neither supplied or demanded by the planet and hovers near the base price of the commodity.

- **Agricultural Planet**
  - Supplies: Water, Foodstuffs
  - Demands: Medicine, Firearms, Ammunition, Electronics
  - Ignores: Metals, Antimatter, Narcotics, Alien Artefacts

- **Mega City Planet**
  - Supplies: Electronics, Medicine, Narcotics
  - Demands: Water, Foodstuffs, Firearms, Ammunition
  - Ignores: Metals, Antimatter, Alien Artefacts

- **Mining Planet**
  - Supplies: Metals, Antimatter, Electronics
  - Demands: Water, Foodstuffs, Medicine, Ammunition
  - Ignores: Narcotics, Alien Artefacts

- **Pirate Space Station**
  - Supplies: Narcotics, Ammunition
  - Demands: Foodstuffs, Firearms, Medicine
  - Ignores: Water, Metals, Antimatter, Electronics, Alien Artefacts

- **Research Outpost**
  - Supplies: Electronics, Medicine, Alien Artefacts
  - Demands: Water, Foodstuffs
  - Ignores: Firearms, Ammunition, Metals, Antimatter, Narcotics

- **Industrial Planet**
  - Supplies: Electronics, Metals, Ammunition, Antimatter
  - Demands: Water, Foodstuffs, Medicine
  - Ignores: Narcotics, Alien Artefacts

- **Frontier Colony**
  - Supplies: Water, Foodstuffs
  - Demands: Medicine, Firearms, Ammunition, Electronics, Metals, Antimatter, Alien Artefacts
  - Ignores: Narcotics

### Supply and Demand Mechanics
No formula values are provided in this ADR, that should be determined by playtesting to ensure correct balance
- **Dynamic Pricing Formula**: Current Price = Base Price × Local Multiplier × Supply Factor × Demand Factor
- **Supply Factor**: Ranges from (extremely scarce) to (oversupplied), affected by:
  - Recent player purchases/sales volume
  - Local production/consumption rates
  - Random events (mining booms, crop failures, etc.)
- **Demand Factor**: Ranges from (low demand) to (high demand), affected by:
  - Local population needs
  - Technological advancement level
  - Political situation
  - Random events (war, peace, festivals, etc.)


### Price Fluctuation System
- **Predictable Trends**: Markets can have small fluctuations without player interaction. Some goods fluctuate more than others (high risk - high reward).
- **Random Events**: major events has some % of happening each turn, can significantly impact 1 market at a time
- **Player Impact**: Large-scale player trading activities cause local markets events, such as gluts or shortages (temporary)

Later features:
- **Seasonal Cycles**: Some commodities follow predictable seasonal patterns

### Trading Mechanics
- **High-Risk Opportunities**: part of random events: extreme price disparities create high-profit opportunities, or removes the ca
- **Cargo Limits**: Ship capacity creates meaningful decisions about which goods to transport
- **Travel Time**: Longer routes have higher opportunity cost and risk of market changes

### Market Information System
- **Market Info**: Players see current prices at all locations
- **Full Transparency Cost**: market may be outdated by the time of arrival

Later features:
- **Risk Assessment**: Market volatility indicators help players assess risk levels, some goods are subject to change

### Player Experience Considerations
- **Intuitiveness**: Clear visual cues for profitable trades (if a good is way above/below base price)
- **Accessibility**: Simple core mechanics with deeper strategic layers
- **Replayability**: Dynamic markets ensure no two playthroughs are identical


## Consequences

### Positive
- Strategic depth through dynamic market conditions
- Meaningful player choices regarding risk tolerance and route planning
- Replayability due to changing market conditions
- Immersive simulation of interplanetary commerce

### Negative
- Market predictability balancing requires careful tuning
- Potential for player frustration during bad luck streaks
- Will need playtesting to achieve good balance of fun vs easy, not too frustrating or boring

## References
- Core gameplay loop ADR [#0001](0001-general-gameplay-scenario.md)
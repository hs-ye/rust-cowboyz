# 0006: Resource/Commodity Types for Space-Western Trading Game

## Status
proposed

## Date
2026-03-01

## Deciders
- User
- game-designer

## Context
The space-western trading game requires a diverse set of resources/commodities that fit the thematic setting while providing meaningful trading opportunities. These resources need to align with the space-western theme, have logical supply/demand relationships across different planet types, and create engaging economic gameplay. The resource system must support the dynamic market economy described in ADR 0001 while maintaining thematic consistency with the space-western setting. Each resource category should have distinct characteristics, rarity levels, and economic properties that influence trading strategies and player decisions.

## Decision

### Game Design Objectives
- Create resource types that authentically represent the space-western theme
- Establish logical supply/demand relationships based on planet types and locations
- Provide diverse trading opportunities with varying risk/reward profiles
- Support multiple viable trading strategies and play styles
- Create immersive economic simulation that enhances the space-western atmosphere

### Resource Categories and Types

#### 1. Basic Resources (Common)
These are fundamental materials needed for survival and basic construction:

**Raw Minerals**
- Iron Ore: Common metal, low value, found on most mining worlds
- Copper: Essential for electronics, moderate availability
- Rare Earth Elements: Higher value, limited to specific planet types
- Precious Metals (Gold/Silver): High value, rare deposits, consistent demand

**Energy Sources**
- Fusion Fuel: Standard energy commodity, stable demand
- Solar Cells: Technology component, varies by planet development
- Battery Cells: Medium-tier energy storage, steady market

#### 2. Agricultural Products (Regional Specialties)
These represent farming outputs that vary significantly by planet environment:

**Foodstuffs**
- Grain: Staple food, high volume, low value per unit
- Exotic Spices: High value, specific climate requirements, luxury status
- Preserved Meats: Medium value, requires processing facilities
- Fresh Produce: Perishable, high value, limited shelf life

**Plant-Based Materials**
- Cotton/Hemp: Textile materials, moderate value, specific growing conditions
- Wood (Exotic): Construction material, varies by planet ecosystem
- Medicinal Herbs: High value, specific planetary conditions needed

#### 3. Livestock and Animal Products (Perishables)
Reflecting the western cattle culture in space:

**Live Animals**
- Cattle (Space-Bred): High initial investment, significant space requirements
- Poultry: Moderate investment, faster turnover
- Exotic Pets: Very high value, niche markets, strict regulations

**Animal Derivatives**
- Leather: Processed goods, moderate value, fashion/craft applications
- Dairy Products: Perishable, high maintenance, premium pricing

#### 4. Manufactured Goods (Processed)
Items that require industrial processing:

**Consumer Goods**
- Clothing: Basic necessities, moderate value, steady demand
- Tools: Industrial/repair equipment, essential for frontier worlds
- Electronics: Communication devices, entertainment, high-tech worlds
- Furniture: Luxury items for developed settlements

**Industrial Equipment**
- Machinery Parts: High value, specialized markets
- Construction Materials: Building supplies, varies by development level
- Mining Equipment: Specialized tools for resource extraction

#### 5. Luxury Items (High Value)
Premium goods for wealthy populations:

**Entertainment**
- Music/Video Media: Digital content, high margins
- Gaming Systems: Tech luxury, specific market demand
- Artwork: Unique items, collector value

**Luxury Consumables**
- Fine Spirits: Premium alcohol, high value, specific production areas
- Tobacco Alternatives: Recreational substances, regulated markets
- Perfumes/Cosmetics: Personal luxury items

**Jewelry and Decorative Items**
- Gemstones: Natural crystals, high value, rare availability
- Crafted Jewelry: Artisan goods, cultural significance
- Antiques: Historical items, collector markets

#### 6. Technology Components (Specialized)
Advanced items for developed worlds:

**Computing**
- Processors: High value, sensitive to market changes
- Memory Chips: Essential tech component, stable demand
- Quantum Storage: Advanced tech, limited production

**Biotechnology**
- Medical Supplies: Critical items, steady high demand
- Genetic Samples: Research materials, heavily regulated
- Prosthetics: Advanced medical devices, premium pricing

**Space Technology**
- Navigation Systems: Essential for traders, consistent demand
- Hull Plating: Ship maintenance, regular consumption
- Propulsion Parts: Engine components, specialized markets

### Resource Properties Framework

#### Scarcity Tiers
- **Abundant (Tier 1)**: Available on 70%+ of planets, low value fluctuation
- **Common (Tier 2)**: Available on 40-70% of planets, moderate fluctuations
- **Limited (Tier 3)**: Available on 20-40% of planets, high fluctuations
- **Rare (Tier 4)**: Available on 5-20% of planets, extreme fluctuations
- **Unique (Tier 5)**: Only available on 1-2 specific planets, highest value

#### Perishability Factors
- **Non-perishable**: No degradation over time
- **Slow-deteriorating**: Gradual value loss over extended periods
- **Moderate**: Noticeable degradation after medium travel times
- **Fast-deteriorating**: Significant value loss during long journeys
- **Highly perishable**: Must be sold quickly or becomes worthless

#### Volume-to-Value Ratios
- **Bulk Low-Value**: High volume, low profit per unit (grain, raw minerals)
- **Medium Density**: Balanced volume and value (manufactured goods)
- **Compact High-Value**: Low volume, high profit per unit (luxury items, tech)

### Thematic Integration with Space-Western Setting

#### Western Elements in Space
- **Mining Towns**: Raw mineral resources from frontier mining colonies
- **Ranch Worlds**: Livestock and agricultural products from pastoral planets
- **Trading Posts**: Remote stations where resources are bought/sold
- **Outlaw Markets**: Black market goods with higher profits and risks
- **Saloon Culture**: Luxury items like spirits and entertainment for frontier populations

#### Space Elements
- **Terraformed Worlds**: Unique agricultural products not possible on Earth
- **Asteroid Mining**: Rare minerals and elements from space rocks
- **Research Stations**: Biotechnology and advanced materials
- **Colony Ships**: Consumer goods and manufactured items for new settlements
- **Trade Hubs**: Centralized markets with diverse resource availability

### Market Behavior Patterns

#### Predictable Relationships
- Agricultural worlds typically undersupply manufactured goods
- Industrial worlds typically undersupply food products
- Wealthy worlds have high demand for luxury items
- Frontier worlds have high demand for basic resources and tools

#### Volatility Triggers
- **Natural Disasters**: Crop failures, mine collapses, affecting supply
- **Political Events**: Wars disrupting trade routes, affecting demand
- **Technological Advances**: New production methods affecting supply
- **Population Shifts**: Colony establishment or evacuation affecting demand

### Player Experience Considerations
- **Intuitiveness**: Resource names and categories should be immediately understandable
- **Engagement**: Different resources should appeal to different play styles
- **Accessibility**: Entry-level resources available for new players
- **Replayability**: Resource availability and profitability should vary between playthroughs

### Risk Mitigation Strategies
- **Diversification**: Multiple resource categories prevent total portfolio collapse
- **Insurance Options**: Ability to hedge against resource-specific risks
- **Market Intelligence**: Information systems to forecast resource trends
- **Flexible Cargo**: Upgradeable ship compartments for different resource types

## Consequences

### Positive
- Rich thematic experience combining space and western elements
- Diverse trading opportunities supporting multiple strategies
- Logical supply/distribution patterns enhancing immersion
- Meaningful risk/reward decisions for players
- Scalable complexity for different player skill levels

### Negative
- Complex resource categorization may confuse new players
- Balancing resource availability and pricing requires ongoing attention
- Too many resource types could lead to analysis paralysis
- Perishability mechanics may frustrate players who miscalculate travel times

## References
- Market/Economy System ADR 0001
- General Gameplay Scenario ADR 0001
- Data Models Schema ADR 0005
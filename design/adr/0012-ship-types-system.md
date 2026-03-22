# 0012: Ship Types System

## Status
Proposed

## Date
2026-03-21

## Deciders
- User
- game-designer

## Context
Currently, every player starts with the same default ship (speed 10.0, cargo capacity 10, acceleration 1). This provides no strategic differentiation at game start and misses an important design opportunity: the ship choice is one of the most impactful strategic decisions a trader can make. A player who prioritises high-volume, low-margin trading needs a different ship than one who hunts rare, high-value commodities on fast routes.

The existing Ship struct already tracks `acceleration` (which feeds directly into the Brachistochrone travel-time formula) and `cargo_capacity`. The market economy (ADR #0005) makes cargo capacity a meaningful constraint — all commodities consume one unit of cargo space, so a ship with 20 cargo slots can carry twice the profit per trip compared to a 10-slot ship, at the cost of other trade-offs.

Introducing distinct ship types at game start gives players an immediate, consequential choice that shapes the rest of the game without adding ongoing complexity.

## Decision

### Ship Types

We define three ship archetypes suited to the space-western setting. Each represents a distinct trading philosophy.

#### 1. Rustbucket Sloop
> *"Fast and lean. If the cargo bay is small, the margins had better be fat."*

| Stat | Value |
|------|-------|
| Acceleration | 3 units/turn² |
| Cargo Capacity | 8 units |
| Purchase Cost | 200 gold |

**Strategic profile:** The Sloop excels at short-hop, high-margin runs. Its superior acceleration dramatically reduces travel time on nearby routes (travel_turns = 2 * sqrt(distance / 3) vs sqrt(distance / 1)), letting the player complete more trading cycles in the limited turn budget. The small cargo hold demands careful commodity selection — players should focus on high-value, low-volume goods (Antimatter, Alien Artefacts, Narcotics, Electronics) rather than bulk staples.

#### 2. Prairie Brigantine
> *"The working trader's ship. Does everything well enough."*

| Stat | Value |
|------|-------|
| Acceleration | 1 unit/turn² |
| Cargo Capacity | 15 units |
| Purchase Cost | 0 gold (starting ship) |

**Strategic profile:** The Brigantine is the default starting option — no extra cost, balanced capabilities. It suits players learning the game or those who want flexibility. With 15 cargo units it can carry meaningful bulk loads while still reaching all planets in reasonable time. It is the reference point against which other ships are measured.

#### 3. Iron Galleon
> *"Slow as a comet, rich as a king — if you time it right."*

| Stat | Value |
|------|-------|
| Acceleration | 1 unit/turn² |
| Cargo Capacity | 30 units |
| Purchase Cost | 400 gold |

**Strategic profile:** The Galleon sacrifices nothing in acceleration over the Brigantine but offers dramatically more cargo space at the cost of starting capital. A player who spends 400 gold on a Galleon starts with only 600 gold (vs 1,000) but can haul three times as much per trip. This ship rewards long-range bulk traders who identify stable, predictable price differentials between planet types (e.g. Agricultural Planet → Mega City Planet for Water/Foodstuffs).

### Why These Three Types

Three archetypes follows the principle from ADR #0001: "small number of choices, not overwhelming for a small indie game". Three options is the classic "Fast / Balanced / Slow-but-capacious" triad that is immediately legible to players without a tutorial. More ship types can be added in future iterations once the system is validated.

### Ship Selection Mechanic

**When:** The player chooses their ship type at game start, on a dedicated selection screen presented before the first turn. The choice is made once and cannot be changed mid-game (no ship trading or mid-game upgrade).

**How:** A simple selection UI presents three cards, each showing:
- Ship name and flavour description
- Visual icon (ASCII art or simple sprite)
- Acceleration, Cargo Capacity, and Purchase Cost stats
- Starting gold remaining after purchase (1,000 − cost)

The player clicks/selects a ship and confirms. The game then initialises with the chosen ship's stats and deducts the purchase cost from starting gold.

**Design note:** Locking the choice at start-of-game creates commitment and prevents the ship from being a trivial mid-game upgrade. It also means the difficulty of the game scales naturally with ship choice — the Sloop is harder (small cargo, requires precision targeting) while the Galleon is a high-risk-high-reward gamble on starting capital.

### Gameplay Impact

**Travel time:** Acceleration is the dominant factor in the Brachistochrone formula. A Sloop with acceleration 3 travels between two planets at orbital radii 5 and 12 (distance = 7) in ceil(2 * sqrt(7/3)) = 4 turns, versus 6 turns for a Brigantine or Galleon. Over a 10-turn Normal difficulty game, this difference is enormous.

**Cargo strategy:** The Galleon's 30-unit hold enables mass-loading of cheap bulk goods (Water, Foodstuffs, Metals) for high absolute profit per trip, but its 400-gold cost means less starting capital for initial purchases. The Sloop's 8-unit hold forces players into the riskier high-value commodity markets.

**Economic tension:** The Sloop costs 200 gold upfront, reducing starting trade capital to 800 gold. The Galleon costs 400 gold, reducing it to 600 gold. The Brigantine starts free at 1,000 gold. This creates a genuine trade-off: speed/capacity vs liquidity.

**Replayability:** Different ship types encourage different play styles and route choices, making repeat playthroughs feel distinct.

### No Mid-Game Ship Changes (MVP Scope)

For MVP, ships are permanent. A future enhancement could allow ship trading at specific port types (e.g. Industrial Planets), but this is explicitly out of scope for this feature and should be tracked as a separate backlog item.

## Consequences

### Positive
- Adds a meaningful strategic decision at game start without adding ongoing complexity
- Creates natural difficulty variance — the Sloop is harder to play than the Brigantine
- Increases replayability by encouraging different strategies across runs
- Slots cleanly into the existing Ship struct (acceleration + cargo_capacity already exist)
- The three-archetype design is immediately legible to players

### Negative
- The ship selection screen requires new UI work before the game begins
- Balancing three ship types requires playtesting; initial values are estimates
- Locking the ship choice at start may frustrate players who realise mid-game they picked wrong — this is acceptable given the short turn budget (10 turns Normal)
- The Galleon's high cost risks making it feel unviable if market conditions are unfavourable; balance tuning will be needed

## References
- [General Gameplay Scenario ADR #0001](./0001-general-gameplay-scenario.md)
- [Movement Mechanics System ADR #0002](./0002-movement-mechanics-system.md)
- [Market/Economy System ADR #0005](./0005-market-economy-system.md)
- [Data Models/Schema ADR #0006](./0006-data-models-schema.md)
- [Cargo Hold Management ADR #0010](./0010-cargo-hold-management.md)
- [Travel/Preview Feature ADR #0011](./0011-travel-preview-feature.md)

# 0003: Web UI View for Space-Western Trading Game

## Status
Accepted

## Date
2026-03-01

## Deciders
- User
- game-designer

## Context
The space-western trading game needs a comprehensive web interface that allows players to visualize the solar system with planets in their actual positions for the current turn. Players need quick access to critical information such as their ship status, inventory, market conditions, and planet details to make informed trading and travel decisions. The UI must balance visual appeal with information density while maintaining intuitive navigation.

Current constraints:
- Need to support real-time visualization of celestial bodies
- Must display multiple information panels simultaneously
- Should allow players to see distant planet market data without losing current position context
- Initial implementation should be a single-page application
- Game setup configuration needs to be accessible

## Decision
We will implement a web-based single-page application (SPA) with a split-screen layout featuring:

**Main Map View 60% of screen:**
Left Side in a landscape / top in a portrait viewport
- Solar system map showing planets in their actual orbital positions for the current turn
- Interactive visualization: selection of planets should change the display on the information panel
- Clear representation of travel routes between planets
- Visual indicators for player ship position and movement trajectory
- Different visual styles for various planet types (mining worlds, agricultural, industrial, etc.)
- Real-time updates of planet positions as turns progress

**Information Panels 40% of screen:**
Right Side in a landscape / bottom in a portrait viewport
- Player profile panel showing credits, reputation, and achievements
- Ship status panel displaying fuel levels, cargo capacity and other equipment
- Inventory panel showing current cargo with quantities and values
- Planet market panel showing buy/sell prices for goods - should work for both current and distance planets (as a preview)
- Interaction on each panel for the relevant action (e.g. buy/sell for market, buy/sell/upgrade equipment on ship)

**Configuration Modal:**
Minimal config modal for starting a new game
- Accessible from the main screen for new game setup. 
- Warning should be provided to player that starting new game will overwrite current game
- Options for difficulty level (i.e. number of turns), starting conditions (ship, credits etc), and game rules

**UI Design Elements:**
- Sci-fi themed visual design with space-western aesthetic (Lived-in world, exposed mechanical designs)
- Responsive layout that adapts to different screen sizes
- Consistent color scheme and typography supporting the theme
- Intuitive iconography for different resource types and actions
- Tooltips and contextual help for new players

**Technical Implementation:**
- Built using modern web technologies (HTML5, CSS3, JavaScript/TypeScript)
- Canvas or SVG-based rendering for the solar system map
- Component-based architecture for information panels
- Local storage for UI preferences and game settings

## Consequences

### Positive
- Players get an immersive, visually appealing way to navigate the game world
- All critical information is visible at once, reducing need for menu navigation
- Planet market previews enable strategic planning for trading routes
- Single-page application provides smooth, responsive user experience
- Configuration modal keeps setup options organized without cluttering gameplay

### Negative
- Designing for single page app limits extension of gameplay elements in future
- Performance optimization needed for smooth rendering of solar system visualization
- Multiple information panels might feel cluttered on smaller screens
- Higher development complexity compared to simpler interfaces
- May require significant art assets to achieve desired western-space aesthetic

## References
- [Game concept and mechanics documentation](../game-concept.md)
- [Solar system positioning algorithm specifications](../game-mechanics/orbital-mechanics.md)
# Playtest Scenario: Game Loads to Opening Screen

## Objective
Verify that the game successfully loads and displays the opening screen when navigating to the local development server.

## Preconditions
- Development server is running (`trunk serve`)
- Browser is available to access http://localhost:8080

## Test Steps
1. Open a web browser and navigate to http://localhost:8080
2. Wait for the page to fully load (WASM compilation and rendering)
3. Observe the displayed content

## Expected Results
- The game UI should display the solar system map (left panel)
- Player status panel should show credits, location, turn number
- Ship status panel should show fuel and cargo capacity
- Market panel should display commodities with buy/sell prices
- Action buttons should be visible (Test: Add Money, Next Turn, New Game)

## Actual Results
- (To be filled during testing)

## Pass/Fail Criteria
- [ ] Page loads without showing a blank screen
- [ ] Solar system map panel is visible
- [ ] Player status panel shows $1000 credits, Earth location, Turn 1
- [ ] Ship status panel shows fuel and cargo capacity
- [ ] Market panel displays commodities with prices
- [ ] No console errors related to WASM or JavaScript
# Travel Flow Playwright Tests

This directory contains comprehensive end-to-end Playwright tests for the travel system in Rust Cowboyz (Epic #93).

## Test Coverage

### Destination Selection Flow
- ✅ Planet list renders correctly with all planets
- ✅ Current planet is marked and disabled for selection
- ✅ Planet selection works and highlights selected planet
- ✅ Planet info displays correctly with type and orbit details

### Travel Information Display
- ✅ Travel time calculates correctly based on distance
- ✅ Fuel cost displays accurately
- ✅ Information updates when selection changes
- ✅ Fuel indicator shows visual fuel bar

### Travel Confirmation
- ✅ Confirm button enabled with valid selection and sufficient fuel
- ✅ Fuel validation prevents travel if insufficient fuel
- ✅ Travel initiates correctly and shows in-transit status
- ✅ Cancel button returns to destination selection

### Travel Feedback
- ✅ Turn counter decrements correctly during travel
- ✅ Progress indicator updates during travel
- ✅ Travel completes and updates location after all turns
- ✅ Arrival notification is displayed

### Solar System Map
- ✅ Map renders with all planets
- ✅ Map legend shows all planet types
- ✅ Click-to-select works on solar map

### Next Turn Button
- ✅ Next Turn button advances game time
- ✅ Planet positions update after next turn
- ✅ Next Turn works independently of travel

### Edge Cases
- ✅ Same destination selection shows appropriate message
- ✅ Travel button is disabled while in transit
- ✅ Multiple travel operations work correctly
- ✅ Fuel is deducted correctly after travel
- ✅ New Game resets travel state

### Full Integration
- ✅ Complete travel journey: selection to arrival
- ✅ Travel to all planet types

## Running the Tests

### Prerequisites

1. Install Playwright:
```bash
npm install -D @playwright/test
npx playwright install
```

2. Build the web application:
```bash
trunk build
```

### Run All Tests

```bash
npx playwright test
```

### Run Specific Test File

```bash
npx playwright test tests/travel-flow.spec.js
```

### Run Tests in Headed Mode (Visible Browser)

```bash
npx playwright test --headed
```

### Run Tests with Debug Mode

```bash
npx playwright test --debug
```

### Run Tests on Specific Browser

```bash
npx playwright test --project=chromium
npx playwright test --project=firefox
npx playwright test --project=webkit
```

### Generate HTML Report

```bash
npx playwright test --reporter=html
npx playwright show-report
```

## Test Structure

```
tests/
├── travel-flow.spec.js    # Main travel flow tests
├── web_integration_tests.rs # Rust integration tests
└── README.md              # This file
```

## Test Data

The tests use the default game configuration with 7 planets:
- Earth (Agricultural) - Starting location
- Mars (Mining)
- Jupiter (Industrial)
- Titan Station (Research Outpost)
- Pirate Haven (Pirate Space Station)
- New Eden (Frontier Colony)
- Mega City One (Mega City)

## CI Integration

The tests are configured to run in CI with:
- Automatic retries (2 retries on CI)
- Screenshot on failure
- Video recording on failure
- JSON and HTML reports

## Environment Variables

- `TEST_BASE_URL`: Base URL for the game (default: http://localhost:8080)
- `CI`: Set to true for CI mode (enables retries, disables parallel execution)

## Troubleshooting

### Tests fail with timeout
- Ensure the game server is running: `trunk serve --port 8080`
- Increase timeout in `playwright.config.js`

### WASM not loading
- Wait time is set to 3 seconds for WASM initialization
- Increase `waitForTimeout` values if needed

### Flaky tests
- Tests run sequentially to avoid state conflicts
- Each test starts with a fresh game state via "New Game" button
